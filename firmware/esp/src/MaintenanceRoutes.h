#pragma once
#include "FilesystemTelemetry.h"
#include <DiagnosticText.h>
#if defined(ARDUINO_ARCH_ESP32)
#include <esp_littlefs.h>
#endif
// Included inside the application's namespace after peripheral/transport code.
// PU850 route adaptation owns no duplicate UART/peripheral implementation.
ClientTelemetry clientTelemetry;
bool recordClientReport(const String &report,const String &peer,const String &agent,const String &version) {
  return clientTelemetry.report(report,peer,millis(),agent,version);
}
bool bootIntegrity=false;
PakarFilesystem::Observation filesystemTelemetry;
bool readFilesystemTelemetry(uint32_t &total,uint32_t &used) {
#if defined(ARDUINO_ARCH_ESP32)
  // Exactly one paired query of the partition mounted by LittleFS.begin(false).
  // Unlike separate totalBytes()/usedBytes(), this never duplicates that query.
  size_t capacity=0,occupied=0;
  if(esp_littlefs_info("spiffs",&capacity,&occupied)!=ESP_OK||
      capacity>UINT32_MAX||occupied>UINT32_MAX)return false;
  total=static_cast<uint32_t>(capacity);used=static_cast<uint32_t>(occupied);return true;
#else
  FSInfo info;if(!LittleFS.info(info))return false;
  total=info.totalBytes;used=info.usedBytes;return true;
#endif
}
void captureBootFilesystemTelemetry(bool mounted) {
  // Setup calls this BEFORE grid.begin: no normal renderer/network service may
  // inherit a full-filesystem traversal. Failed mount/probe stays unknown.
  filesystemTelemetry.capture(mounted,true,commandClock,readFilesystemTelemetry);
}
bool refreshFilesystemTelemetry(String &reason) {
  // Optional EXPLICIT maintenance call only. Never automatically called by
  // telemetry/status, and never scan even a static enabled output's repair loop.
  const PakarFilesystem::RefreshSafety safety{grid.outputEnabled(),otaActive,
      otaRebootAt!=0,remoteFirmware.busy(),artifactReaders!=0,bool(uploadFile),
      avrUploadOwner!=0,urbootBusy.get(),networkApplyPending,cloudApplyPending,cloudServices.ioActive()};
  const bool accepted=filesystemTelemetry.capture(filesystemTelemetry.mounted(),safety.idle(),
      commandClock,readFilesystemTelemetry);
  const auto state=filesystemTelemetry.snapshot(commandClock());
  if(accepted)reason=F("Filesystem observation refreshed");
  else reason=PakarFilesystem::code(state.error);
  return accepted;
}
// Running-image identity is immutable until reboot. Capture once before cloud
// servicing, not a whole-image verification/LED flash hold on every telemetry
// interval. Update admission still performs its independent fresh checks.
struct BootTelemetryImage {
  uint32_t size=0,flashHz=0,capturedAt=0;
  int flashMode=0;
  String md5;
} bootTelemetryImage;
void captureBootTelemetryImage() {
  bootTelemetryImage.size=FirmwarePlatform::sketchSize();
  bootTelemetryImage.md5=FirmwarePlatform::sketchMd5();
  bootTelemetryImage.flashHz=FirmwarePlatform::flashChipSpeed();
  bootTelemetryImage.flashMode=static_cast<int>(FirmwarePlatform::flashChipMode());
  bootTelemetryImage.capturedAt=millis();
}
String displayLocale="en";
String avrVersionText() {
  return String(avrVersion[0])+'.'+String(avrVersion[1])+'.'+String(avrVersion[2]);
}
uint16_t telemetry16(unsigned at) {return avrTelemetry[at]|static_cast<uint16_t>(avrTelemetry[at+1])<<8;}
uint32_t telemetry32(unsigned at) {
  return telemetry16(at)|static_cast<uint32_t>(telemetry16(at+2))<<16;
}
String encodedJson(const JsonDocument &d) {
  String s;const size_t expected=measureJson(d);
  return !d.overflowed() && serializeJson(d,s)==expected && s.length()==expected?s:String();
}
String clockJson() {
  JsonDocument d;d[F("epoch")]=static_cast<uint32_t>(time(nullptr));
  d["utc_epoch"]=time(nullptr)>1700000000?static_cast<uint32_t>(time(nullptr)):0;
  d["synced"]=time(nullptr)>1700000000;d["uptime_ms"]=millis();
  d[F("valid")]=time(nullptr)>1700000000;d[F("source")]=clockSource;
  d[F("ntp_configured")]=ntpConfigured;d[F("ntp_observed")]=ntpObserved;
  d[F("last_set_uptime_ms")]=lastClockSetAt;d[F("timezone")]=F("UTC");d[F("rtc_present")]=false;
  d["ntp_enabled"]=networkToolsSettings.ntpEnabled;d["ntp_dhcp"]=networkToolsSettings.ntpDhcp;
  d["ntp_server"]=networkToolsSettings.ntpServer;d["ntp_authenticated"]=false;
  return encodedJson(d);
}
String networkTelemetry() {
  JsonDocument d;
  const bool connected=networkConfig.stationConnected();
  // Route readiness is local IPv4 availability, never verified Internet access.
  d[F("uplink")][F("interface")]=NetworkPath::interfaceName();
  d[F("uplink")][F("ipv4_ready")]=NetworkPath::ready();
#if defined(ARDUINO_ARCH_ESP32)
  const auto &ethernet=NetworkPath::ethernet();
  const bool ethernetStarted=ethernet.started();
  d[F("ethernet")][F("configured")]=ethernet.configured();
  d[F("ethernet")][F("started")]=ethernetStarted;
  d[F("ethernet")][F("link_up")]=ethernet.linkUp();
  d[F("ethernet")][F("ipv4_ready")]=ethernet.connected();
  if(ethernetStarted) {
    d[F("ethernet")][F("ip")]=ethernet.localIP().toString();
    d[F("ethernet")][F("mac")]=ethernet.macAddress();
    d[F("ethernet")][F("gateway")]=ethernet.gatewayIP().toString();
    d[F("ethernet")][F("netmask")]=ethernet.subnetMask().toString();
    d[F("ethernet")][F("dns0")]=ethernet.dnsIP(0).toString();
    d[F("ethernet")][F("dns1")]=ethernet.dnsIP(1).toString();
  } else {
    for(const char *field:{"ip","mac","gateway","netmask","dns0","dns1"})
      d[F("ethernet")][field]=nullptr;
  }
  d[F("ethernet")][F("last_error")]=ethernet.lastError();
#else
  for(const char *field:{"configured","started","link_up","ipv4_ready"})
    d[F("ethernet")][field]=false;
  for(const char *field:{"ip","mac","gateway","netmask","dns0","dns1"})
    d[F("ethernet")][field]=nullptr;
  d[F("ethernet")][F("last_error")]=F("");
#endif
  d[F("mode")]=static_cast<int>(WiFi.getMode());
  d[F("station")][F("configured")]=networkConfig.stationEnabled();
  d[F("station")][F("configured_ssid")]=networkConfig.stationSsid();
  d[F("station")][F("connected")]=connected;d[F("station")][F("status")]=static_cast<int>(WiFi.status());
  d[F("station")][F("ssid")]=connected?WiFi.SSID():String();
  d[F("station")][F("mac")]=WiFi.macAddress();d[F("station")][F("ip")]=WiFi.localIP().toString();
  d[F("station")][F("netmask")]=WiFi.subnetMask().toString();d[F("station")][F("gateway")]=WiFi.gatewayIP().toString();
  d[F("station")][F("dns0")]=WiFi.dnsIP(0).toString();d[F("station")][F("dns1")]=WiFi.dnsIP(1).toString();
  if(connected) {d[F("station")][F("rssi_dbm")]=WiFi.RSSI();d[F("station")][F("channel")]=WiFi.channel();}
  else {d[F("station")][F("rssi_dbm")]=nullptr;d[F("station")][F("channel")]=nullptr;}
#if defined(ARDUINO_ARCH_ESP32)
  d[F("station")][F("dhcp")]=networkConfig.dhcpEnabled();
#else
  d[F("station")][F("dhcp")]=wifi_station_dhcpc_status()==DHCP_STARTED;
#endif
  d[F("ap")][F("enabled")]=apStarted;d[F("ap")][F("configured")]=networkConfig.accessPointEnabled();
  d[F("ap")][F("ssid")]=WiFi.softAPSSID();d[F("ap")][F("mac")]=WiFi.softAPmacAddress();
  d[F("ap")][F("ip")]=WiFi.softAPIP().toString();d[F("ap")][F("clients")]=WiFi.softAPgetStationNum();
#if defined(ARDUINO_ARCH_ESP32)
  d[F("ap")][F("netmask")]=WiFi.softAPSubnetMask().toString();
#else
  struct ip_info ip;
  if(wifi_get_ip_info(SOFTAP_IF,&ip)) d[F("ap")][F("netmask")]=IPAddress(ip.netmask.addr).toString();
#endif
  d[F("passwords")]=F("redacted");return encodedJson(d);
}
String buildTelemetryJson() {
  JsonDocument d;
  const uint32_t now=millis();
  d[F("type")]=F("telemetry");d[F("product")]=F("PakarController");
  d[F("hostname")]=hostname;d[F("fqdn")]=hostname+".local";
  d[F("board_id")]=pcBoardId;d[F("boot_id")]=pcBootId;
  d[F("display_name")]=deviceIdentity.displayName();
  auto application=d[F("application")].to<JsonObject>();
  application[F("id")]=APP_ID;application[F("name")]=APP_NAME;application[F("title")]=APP_TITLE;
  d[F("version")]=BoardCore::BuildInfo::kVersion;
  PakarPeer::compatibility(d["compatibility"].to<JsonObject>());
  auto e=d[F("esp")].to<JsonObject>();
  runtimeCpu.append(e["cpu_usage"].to<JsonObject>(),now);
  e[F("model")]=FirmwarePlatform::model();e[F("chip_id")]=SystemPlatform::chipId();
#if defined(ARDUINO_ARCH_ESP32)
  e[F("chip_model")]=ESP.getChipModel();e[F("chip_revision")]=ESP.getChipRevision();e[F("chip_cores")]=ESP.getChipCores();
  e[F("flash_chip_id")]=nullptr;e[F("core_version")]=ESP_ARDUINO_VERSION_STR;
  e[F("sdk_version")]=ESP.getSdkVersion();e[F("full_version")]=ESP_ARDUINO_VERSION_STR;
  e[F("flash_real_bytes")]=ESP.getFlashChipSize();
  e[F("largest_heap_block_bytes")]=ESP.getMaxAllocHeap();e[F("heap_fragmentation_percent")]=nullptr;
  const uint32_t internalCaps=MALLOC_CAP_INTERNAL|MALLOC_CAP_8BIT;
  e[F("internal_heap_free_bytes")]=heap_caps_get_free_size(internalCaps);
  e[F("internal_heap_largest_block_bytes")]=heap_caps_get_largest_free_block(internalCaps);
  e[F("internal_heap_minimum_free_bytes")]=heap_caps_get_minimum_free_size(internalCaps);
  e[F("reset_reason")]=SystemPlatform::resetReason();e[F("reset_info")]=nullptr;
  e[F("psram_bytes")]=ESP.getPsramSize();e[F("free_psram_bytes")]=ESP.getFreePsram();
  const auto *runningPartition=esp_ota_get_running_partition();
  const auto *bootPartition=esp_ota_get_boot_partition();
  if(runningPartition)e[F("ota")][F("running_offset")]=runningPartition->address;
  else e[F("ota")][F("running_offset")]=nullptr;
  if(bootPartition)e[F("ota")][F("boot_offset")]=bootPartition->address;
  else e[F("ota")][F("boot_offset")]=nullptr;
  esp_ota_img_states_t imageState;
  if(runningPartition && esp_ota_get_state_partition(runningPartition,&imageState)==ESP_OK)
    e[F("ota")][F("running_state_code")]=static_cast<uint32_t>(imageState);
  else e[F("ota")][F("running_state_code")]=nullptr;
#else
  e[F("flash_chip_id")]=ESP.getFlashChipId();
  e[F("core_version")]=ESP.getCoreVersion();e[F("sdk_version")]=ESP.getSdkVersion();e[F("full_version")]=ESP.getFullVersion();
  e[F("flash_real_bytes")]=ESP.getFlashChipRealSize();
  e[F("largest_heap_block_bytes")]=ESP.getMaxFreeBlockSize();e[F("heap_fragmentation_percent")]=ESP.getHeapFragmentation();
  e[F("reset_reason")]=ESP.getResetReason();e[F("reset_info")]=ESP.getResetInfo();
#endif
  PakarFilesystem::append(e[F("filesystem")].to<JsonObject>(),filesystemTelemetry.snapshot(commandClock()));
  e[F("cpu_mhz")]=ESP.getCpuFreqMHz();e[F("flash_bytes")]=ESP.getFlashChipSize();
  e[F("flash_hz")]=bootTelemetryImage.flashHz;
  e[F("flash_mode")]=bootTelemetryImage.flashMode;
  e[F("sketch_bytes")]=bootTelemetryImage.size;
  if(bootTelemetryImage.md5.length())e[F("sketch_md5")]=bootTelemetryImage.md5;
  else e[F("sketch_md5")]=nullptr;
  e[F("image_identity_scope")]=F("running_boot_image");
  e[F("image_identity_captured_uptime_ms")]=bootTelemetryImage.capturedAt;
  e[F("boot_integrity_verified")]=bootIntegrity;e[F("ota_free_bytes")]=ESP.getFreeSketchSpace();
  e[F("free_heap_bytes")]=ESP.getFreeHeap();e[F("uptime_ms")]=now;
  char uptimeText[40], heapText[32];
  if(BoardCore::DiagnosticText::formatUptime(now/1000U,uptimeText,sizeof(uptimeText)))e[F("uptime_text")]=uptimeText;
  else e[F("uptime_text")]=nullptr;
  if(BoardCore::DiagnosticText::formatBytes(e[F("free_heap_bytes")].as<uint32_t>(),heapText,sizeof(heapText)))e[F("free_heap_text")]=heapText;
  else e[F("free_heap_text")]=nullptr;
  e[F("uart_errors")]=link.errors();e[F("usb_errors")]=hostLink.errors();
#if defined(ARDUINO_ARCH_ESP32)
  e[F("hardware")][F("ws2811_pin")]=PAKAR_S3_LED_DATA;
  e[F("hardware")][F("avr_rx")]=PAKAR_S3_AVR_RX;e[F("hardware")][F("avr_tx")]=PAKAR_S3_AVR_TX;
  e[F("hardware")][F("uart_baud")]=38400;e[F("hardware")][F("busy_pin")]=nullptr;
  e[F("hardware")][F("isp_reset")]=PAKAR_S3_ISP_RESET;e[F("hardware")][F("isp_spi")]=F("GPIO11,GPIO13,GPIO12");
  e[F("hardware")][F("isp_available")]=static_cast<bool>(PAKAR_S3_ISP_ELECTRICAL_VERIFIED);
#else
  e[F("hardware")][F("ws2811_pin")]=F("D2/GPIO4");
  e[F("hardware")][F("avr_rx")]=F("D4/GPIO2");e[F("hardware")][F("avr_tx")]=F("D0/GPIO16");
  e[F("hardware")][F("uart_baud")]=38400;e[F("hardware")][F("busy_pin")]=F("D3/GPIO0");
  e[F("hardware")][F("busy_wire")]=static_cast<bool>(PAKAR_USE_BUSY_WIRE);
  e[F("hardware")][F("isp_reset")]=F("D1/GPIO5");e[F("hardware")][F("isp_spi")]=F("D5,D6,D7");
#endif
  e[F("hardware")][F("pixels")]=480;e[F("hardware")][F("cells")]=80;
  e[F("hardware")][F("busy_wire")]=static_cast<bool>(PAKAR_USE_BUSY_WIRE);
  e[F("hardware")][F("i2c_present")]=false;
  auto a=d[F("avr")].to<JsonObject>();a[F("online")]=avrOnline;a[F("version_known")]=avrVersionKnown;
  if(avrVersionKnown) a[F("version")]=avrVersionText();else a[F("version")]=nullptr;
  a[F("model")]=F("atmega328p");a[F("flash_capacity_bytes")]=32768;a[F("sram_capacity_bytes")]=2048;
  a[F("eeprom_capacity_bytes")]=1024;a[F("features")]=avrPcFeatures;
  a[F("programming_latched")]=programmingLatched;a[F("last_frame_age_ms")]=static_cast<uint32_t>(now-lastAvrFrameAt);
  a[F("telemetry_supported")]=avrTelemetryKnown;
  const bool recent=avrOnline && avrTelemetryKnown && static_cast<uint32_t>(now-avrTelemetryAt)<=12000U;
  a[F("telemetry_recent")]=recent;
  const bool resetRecent=avrOnline && avrResetTelemetryKnown &&
      static_cast<uint32_t>(now-avrResetTelemetryAt)<=12000U;
  a[F("reset_telemetry_known")]=resetRecent;
  if(resetRecent) {
    a[F("reset")][F("cause_raw")]=avrResetCause;
    a[F("reset")][F("power_on")]=static_cast<bool>(avrResetCause&1U);
    a[F("reset")][F("external")]=static_cast<bool>(avrResetCause&2U);
    a[F("reset")][F("brownout")]=static_cast<bool>(avrResetCause&4U);
    a[F("reset")][F("watchdog")]=static_cast<bool>(avrResetCause&8U);
    a[F("reset")][F("boot_count")]=avrBootCount;
    a[F("reset")][F("boot_count_persisted")]=avrBootCountPersisted;
  } else a[F("reset")]=nullptr;
  if(avrTelemetryKnown) a[F("telemetry_age_ms")]=static_cast<uint32_t>(now-avrTelemetryAt);
  else a[F("telemetry_age_ms")]=nullptr;
  if(recent) {
    a[F("uptime_ms")]=telemetry32(4);a[F("free_sram_bytes")]=telemetry16(8);a[F("adc6_raw")]=telemetry16(10);
    a[F("buzzer")][F("frequency_hz")]=telemetry16(12);a[F("buzzer")][F("duration_ms")]=telemetry16(14);
    a[F("buzzer")][F("muted")]=static_cast<bool>(avrTelemetry[16]);a[F("uart_errors")]=telemetry16(42);
    a[F("lost_events")]=telemetry16(44);
    auto s=a[F("settings")].to<JsonObject>();s[F("storage_layout")]=avrTelemetry[17];s[F("flags")]=avrTelemetry[18];
    s[F("display_brightness")]=avrTelemetry[19];s[F("stream_period_ms")]=telemetry16(20);
    s[F("ninth_threshold")]=telemetry16(22);s[F("default_page")]=avrTelemetry[24];
    s[F("visible_pages")]=avrTelemetry[25];s[F("persisted")]=static_cast<bool>(avrTelemetry[38]);
    auto order=s[F("packed_order")].to<JsonArray>();for(unsigned i=26;i<29;++i) order.add(avrTelemetry[i]);
    const unsigned len=min<unsigned>(avrTelemetry[29],8);char name[9]={};memcpy(name,avrTelemetry+30,len);
    s[F("name")]=name;
  } else {
    a[F("uptime_ms")]=nullptr;a[F("free_sram_bytes")]=nullptr;a[F("adc6_raw")]=nullptr;
    a[F("buzzer")]=nullptr;a[F("settings")]=nullptr;a[F("lost_events")]=nullptr;a[F("uart_errors")]=nullptr;
  }
  a[F("hardware")][F("tm1637")]=F("A4/DIO,A5/CLK");a[F("hardware")][F("buzzer")]=F("D9/Timer1");
  a[F("hardware")][F("keys")]=9;a[F("hardware")][F("shift_outputs")]=8;a[F("hardware")][F("i2c_present")]=false;
  d[F("discovery")][F("mdns")]=mdnsStarted;d[F("discovery")][F("ssdp")]=ssdpStarted;d[F("discovery")][F("netbios")]=netbiosStarted;
  d[F("cloud")][F("mqtt_connected")]=cloudServices.mqttConnected();d[F("cloud")][F("mqtt_state")]=cloudServices.mqttState();
  d[F("cloud")][F("websocket_connected")]=cloudServices.webSocketConnected();
  d[F("cloud")][F("mqtt_messages_received")]=cloudServices.mqttMessagesReceived();
  d[F("cloud")][F("websocket_messages_received")]=cloudServices.webSocketMessagesReceived();
  d[F("cloud")][F("mqtt_password_set")]=cloudConfig.mqttPasswordSet();d[F("cloud")][F("websocket_token_set")]=cloudConfig.webSocketTokenSet();
  d[F("cloud")][F("telemetry_seconds")]=cloudConfig.telemetrySeconds();d[F("cloud")][F("error")]=cloudServices.lastError();
  d[F("auth")][F("ready")]=accessConfig.ready();d[F("auth")][F("default_credentials")]=accessConfig.defaultsActive();
  d[F("locale")]=displayLocale;d[F("locale_translation_available")]=false;
  d[F("client_capacity")]=ClientTelemetry::kCapacity;d[F("client_ttl_ms")]=120000;
  // No credentials, arbitrary filesystem paths, synthetic voltage/temperature.
  if(d.overflowed()) return "{\"type\":\"telemetry-error\",\"error\":\"insufficient_heap\"}";
  String out=encodedJson(d);d.clear();
  if(out.length()<2)return "{\"type\":\"telemetry-error\",\"error\":\"insufficient_heap\"}";
  out.remove(out.length()-1);
#if defined(ARDUINO_ARCH_ESP32)
  // Reuse the compact live snapshot below instead of duplicating all detail
  // endpoints inside telemetry. Raising this allocation beyond 24 KiB fails
  // on otherwise healthy devices with a smaller contiguous internal heap.
  constexpr size_t telemetryLimit=24576;
  if(!out.reserve(telemetryLimit))return "{\"type\":\"telemetry-error\",\"error\":\"insufficient_heap\"}";
#else
  constexpr size_t telemetryLimit=16384;
#endif
  // Append serialized sections, releasing each tree before creating the next.
  // No deserialize-and-copy of 80 cells and eight client records into a second
  // aggregate tree. Failed String growth is detected, never partial JSON.
  auto add=[&](const char *key,const String &value) {
    if(value.isEmpty() || out.length()+strlen(key)+value.length()+5>telemetryLimit)return false;
    const size_t expected=out.length()+strlen(key)+value.length()+4;
    out+=F(",\"");out+=key;out+=F("\":");out+=value;
    return out.length()==expected;
  };
  if(!add("network",networkTelemetry()) || !add("clock",clockJson()) ||
     !add("network_tools",networkToolsJson()) ||
     !add("remote_firmware",remoteFirmware.json()) || !add("board",buildStatusJson(true)) ||
     !add("clients",clientTelemetry.json(now)))
    return "{\"type\":\"telemetry-error\",\"error\":\"insufficient_heap_or_snapshot_limit\"}";
  const size_t expected=out.length()+1;out+='}';
  return out.length()==expected?out:String("{\"type\":\"telemetry-error\",\"error\":\"insufficient_heap\"}");
}
bool prepareRemotePair(const RemoteFirmware::Plan &plan,String &reason) {
  if(artifactReaders || otaActive || otaRebootAt || avrUploadOwner || networkApplyPending || cloudApplyPending) {reason=F("Another maintenance/network operation is active");return false;}
  if(!flushUpgradeScene()) {reason=F("Scene checkpoint failed; update not applied");return false;}
  // An explicitly ESP-only update owns no AVR mutation, regardless of whether
  // UART support was compiled in or a companion is currently reachable. A
  // known recovery latch still requires all existing paired recovery gates.
  if(plan.targets==RemotePolicy::Esp && !programmingLatched) return true;
  if(!waitForAvr() || !avrVersionKnown || !avrPcSupported) {reason=F("AVR UART/profile unavailable; no erase allowed");return false;}
  if(plan.targets==1 && plan.esp.version!=avrVersionText()) {
    reason=F("ESP-only update would produce mismatched board versions; use target all");return false;
  }
  if(plan.targets==2 && plan.avr.version!=BoardCore::BuildInfo::kVersion) {
    reason=F("AVR-only update would produce mismatched board versions; use target all");return false;
  }
  if((plan.targets&2) && !plan.allowDowngrade &&
      RemotePolicy::compare(plan.avr.version.c_str(),avrVersionText().c_str())<0) {
    reason=F("AVR downgrade refused before erase");return false;
  }
  const bool retainRecovery=programmingLatched;
  // Audio is best effort, never a firmware/erase safety acknowledgement. The
  // loop already emitted the one start notice; Prog/ISP must remain silent.
  const uint8_t armed[]={1};
  if(!requestAndWaitForAck(kProgrammingState,armed,1)) {reason=F("AVR durable Prog arm not acknowledged; erase refused");return false;}
  programmingLatched=true;
  if(retainRecovery) {
    // An explicit new pinned repair must not overwrite the original known-good
    // flash with a half-updated application's contents. ISP still compares the
    // retained EEPROM to the actual target before any erase.
    File flash=LittleFS.open(String(kBackupDirectory)+"/flash.bin","r");
    File eeprom=LittleFS.open(String(kBackupDirectory)+"/eeprom.bin","r");
    File metadata=LittleFS.open(String(kBackupDirectory)+"/metadata.txt","r");
    const bool complete=flash && flash.size()==32768 && eeprom && eeprom.size()==1024 &&
        metadata && metadata.size()>0 && metadata.size()<=128;
    if(flash)flash.close();if(eeprom)eeprom.close();if(metadata)metadata.close();
    if(!complete){reason=F("Retained recovery snapshot missing; physical recovery required");return false;}
  } else {
    if(!avrIsp.backup(LittleFS,kBackupDirectory)) {reason=String("Recovery backup failed: ")+avrIsp.lastError();return false;}
    if(!waitForAvr()) {reason=F("AVR did not return from backup; Prog remains armed");return false;}
  }
  return true;
}
bool programRemoteAvr(const char *path,const String &version,String &reason) {
  File image=LittleFS.open(path,"r"),eeprom=LittleFS.open(String(kBackupDirectory)+"/eeprom.bin","r");
  const bool ok=avrIsp.programFlash(image,eeprom);image.close();eeprom.close();
  if(!ok) {reason=avrIsp.lastError();return false;}
  if(!waitForAvr() || !avrVersionKnown || avrVersionText()!=version) {
    reason=F("AVR flash verified but UART version did not match; Prog remains armed");return false;
  }
  const uint8_t armed[]={1};
  if(!requestAndWaitForAck(kProgrammingState,armed,1)) {reason=F("New AVR Prog acknowledgement missing");return false;}
  programmingLatched=true;return true;
}
int8_t finishRemotePair(const RemoteFirmware::Plan &plan,bool commit,String &reason) {
  if(plan.targets==RemotePolicy::Esp && !programmingLatched) {
    // RemoteFirmware already checked actual running-image SHA/integrity/version.
    // Do not wait for an optional AVR, release its Prog or emit a melody.
    if(plan.esp.version!=BoardCore::BuildInfo::kVersion) {
      reason=F("Actual ESP version differs from the staged update");return -1;
    }
    return 1;
  }
  if(!avrOnline || !avrVersionKnown || !avrPcSupported) {reason=F("Waiting for actual AVR version/profile");return 0;}
  if(avrVersionText()!=BoardCore::BuildInfo::kVersion ||
     ((plan.targets&2) && avrVersionText()!=plan.avr.version) ||
     ((plan.targets&1) && plan.esp.version!=BoardCore::BuildInfo::kVersion)) {
    reason=F("Actual board versions differ; Prog remains armed");return -1;
  }
  if(!commit) return 1;
  const uint8_t released[]={0};
  if(!requestAndWaitForAck(kProgrammingState,released,1)) {reason=F("Prog release outcome unknown; not retried");return -1;}
  programmingLatched=false;return 1; // Verified terminal phase owns the one cue.
}
bool strictU32(const String &s,uint32_t &out) {
  if(!s.length() || s.length()>10) return false;
  uint64_t v=0;for(size_t i=0;i<s.length();++i) {
    if(s[i]<'0'||s[i]>'9') return false;v=v*10+s[i]-'0';if(v>UINT32_MAX) return false;
  } out=v;return true;
}
bool remoteCommand(const JsonDocument &d,String &result) {
  const String command=d[F("command")]|"";
  bool ok=false;
  if(command=="firmware-status") {result=remoteFirmware.json();return true;}
  if(command=="telemetry") {result=buildTelemetryJson();return true;}
  if(command=="firmware-fetch") {
    if(artifactReaders || otaActive || otaRebootAt || avrUploadOwner || networkApplyPending || cloudApplyPending) {
      result="Another operation is active or recovery is required";return false;
    }
    if(!d[F("url")].is<const char*>() || !d[F("sha256")].is<const char*>() ||
       !d[F("targets")].is<unsigned>() || (!d[F("apply")].isNull()&&!d[F("apply")].is<bool>()) ||
       (!d[F("allow_downgrade")].isNull()&&!d[F("allow_downgrade")].is<bool>()) ||
       (!d[F("bearer")].isNull()&&!d[F("bearer")].is<const char*>())) {result="Invalid typed firmware request";return false;}
    const unsigned targets=d[F("targets")];if(targets<1 || targets>3) {result="Invalid target";return false;}
    if(programmingLatched && (targets!=3 || (d[F("apply")]|false))) {
      result=F("Recovery requires a new pinned both-board stage and separate explicit apply");return false;
    }
    ok=remoteFirmware.queue(d[F("url")].as<String>(),d[F("sha256")].as<String>(),targets,
        d[F("apply")]|false,d[F("allow_downgrade")]|false,d[F("bearer")]|"");
  } else if(command=="firmware-apply" || command=="firmware-cancel") {
    if(artifactReaders) {result=F("Artifact download owns storage; retry after completion");return false;}
    if(command=="firmware-apply" && (otaActive || otaRebootAt || avrUploadOwner ||
        networkApplyPending || cloudApplyPending)) {
      result=F("Another maintenance/configuration operation is active");return false;
    }
    if(!d[F("job_id")].is<uint32_t>() || !d[F("boot_id")].is<const char*>() ||
       (command=="firmware-apply"&&!d[F("sha256")].is<const char*>())) {result="Job identity required";return false;}
    ok=command=="firmware-apply"?remoteFirmware.apply(d[F("job_id")],d[F("boot_id")].as<String>(),d[F("sha256")].as<String>()):
        remoteFirmware.cancel(d[F("job_id")],d[F("boot_id")].as<String>());
  } else {result="Unknown firmware operation";return false;}
  result=ok?remoteFirmware.json():remoteFirmware.error();return ok;
}
void sendMaintenanceJson(const String &body,int code=200) {
  if(auto *request=server.request()) {
    auto *response=server.beginResponse(code,"application/json",body);
    response->addHeader("Cache-Control","no-store");server.send(response);
  }
}
bool handleMaintenanceShell(const String &line,String &result) {
  if(line=="info" || line=="telemetry") {result=buildTelemetryJson();return true;}
  String parts[6],rest=line;unsigned n=0;
  while(rest.length() && n<6) {
    const int space=rest.indexOf(' ');parts[n++]=space<0?rest:rest.substring(0,space);
    if(space<0){rest=String();break;}rest=rest.substring(space+1);rest.trim();
  }
  JsonDocument d;
  if(n==2 && parts[1]=="status") d[F("command")]=F("firmware-status");
  else if((n==4 || n==5) && parts[1]=="fetch") {
    d[F("command")]=F("firmware-fetch");d[F("url")]=parts[2];d[F("sha256")]=parts[3];
    d[F("targets")]=n==4 || parts[4]=="all"?3:parts[4]=="esp"?1:parts[4]=="avr"?2:0;
  } else if((n==5 && parts[1]=="apply") || (n==4 && parts[1]=="cancel")) {
    uint32_t id;if(!strictU32(parts[2],id)){result=F("ERR invalid job ID");return false;}
    d[F("command")]=parts[1]=="apply"?"firmware-apply":"firmware-cancel";
    d[F("job_id")]=id;d[F("boot_id")]=parts[3];d[F("sha256")]=parts[4];
  } else {result=F("ERR firmware status | fetch URL SHA [all|esp|avr] | apply JOB BOOT SHA | cancel JOB BOOT");return false;}
  return remoteCommand(d,result);
}
void registerMaintenanceRoutes() {
  bootIntegrity=FirmwarePlatform::runningIntegrity();
  captureBootTelemetryImage();
  localOtaReceipt.begin(LittleFS,accessConfig.ready(),pcBootId.c_str(),runningImageSize(),
    runningImageMd5().c_str(),bootIntegrity);
  // A post-boot receipt and the next upload must never share an operation ID.
  otaNextOwner=localOtaReceipt.uploadId();
  localBootNoticePending=localOtaReceipt.known() && !localOtaReceipt.pending() &&
    !localOtaReceipt.noticeConsumed() && localOtaReceipt.installedBoot()==pcBootId;
  remoteFirmware.setServiceCallback(upgradeBackground);
  remoteFirmware.setRebootReady(upgradeRebootReady);
  avrIsp.setServiceCallback(upgradeBackground);
  avrIsp.setProgressCallback([](const char *stage,uint32_t done,uint32_t total){
    if(urbootBusy.get()){urbootPhase=stage;urbootDone=done;urbootTotal=total;}
    else if(remoteFirmware.busy())remoteFirmware.reportProgress(stage,done,total);
  });
  remoteFirmware.begin(pcBootId,prepareRemotePair,programRemoteAvr,finishRemotePair);
  server.on("/api/update/lifecycle",HTTP_GET,[](){if(authenticate())sendMaintenanceJson(upgradeJson());});
  File locale=LittleFS.open("/locale.cfg","r");
  if(locale && locale.size()==2) {char p[3]={};locale.read(reinterpret_cast<uint8_t*>(p),2);
    if(!strcmp(p,"en")||!strcmp(p,"fa")||!strcmp(p,"ar")||!strcmp(p,"ru")) displayLocale=p;
  }
  if(locale)locale.close();
  server.on("/api/telemetry",HTTP_GET,[](){if(authenticate())sendMaintenanceJson(buildTelemetryJson());});
  server.on("/api/clients/report",HTTP_POST,[](){
    if(!authenticate())return;
    const auto *agent=server.request()->getHeader("User-Agent");
    const bool ok=recordClientReport(server.arg("report"),server.remoteAddress(),agent?agent->value():String(),WebAccess::clientVersion(server.request()));
    sendMaintenanceJson(ok?"{\"accepted\":true}":"{\"error\":\"invalid_report_or_client_limit\"}",ok?200:400);
  });
  server.on("/api/clients",HTTP_GET,[](){if(authenticate())sendMaintenanceJson(clientTelemetry.json(millis()));});
  server.on("/api/firmware/remote",HTTP_GET,[](){if(authenticate())sendMaintenanceJson(remoteFirmware.json());});
  const char *paths[]={"/api/firmware/fetch","/api/firmware/apply","/api/firmware/cancel","/file/cancel"};
  for(unsigned i=0;i<4;++i) {
    const unsigned op=i;
    server.on(paths[i],HTTP_POST,[op](){
      if(!authenticate())return;
      JsonDocument d;d[F("command")]=op==0?"firmware-fetch":op==1?"firmware-apply":"firmware-cancel";
      if(op==0) {
        uint32_t target=3;const String requested=server.arg("targets");
        if(requested.length() && !strictU32(requested,target)) {server.send(400,"text/plain",F("Invalid targets"));return;}
        d[F("targets")]=target;d[F("url")]=server.arg("url");d[F("sha256")]=server.arg("sha256");
        if((server.hasArg("apply") && server.arg("apply")!="true" && server.arg("apply")!="false") ||
           (server.hasArg("allow_downgrade") && server.arg("allow_downgrade")!="true" && server.arg("allow_downgrade")!="false")) {
          server.send(400,"text/plain",F("Boolean values must be true or false"));return;
        }
        d[F("apply")]=server.arg("apply")=="true";d[F("allow_downgrade")]=server.arg("allow_downgrade")=="true";
        d[F("bearer")]=server.arg("bearer");
      } else {
        uint32_t id;if(!strictU32(server.arg("job_id"),id)) {server.send(400,"text/plain",F("Job ID required"));return;}
        d[F("job_id")]=id;d[F("boot_id")]=server.arg("boot_id");d[F("sha256")]=server.arg("sha256");
      }
      String result;const bool ok=remoteCommand(d,result);
      if(ok)sendMaintenanceJson(result,op==0?202:200);else server.send(409,"text/plain",result);
    });
  }
  for(const char *path:{"/datetime/get","/datetime/active","/ntp/get"})
    server.on(path,HTTP_GET,[](){if(authenticate())sendMaintenanceJson(clockJson());});
  for(const char *path:{"/datetime/parse","/datetime/set"}) {
    const bool apply=!strcmp(path,"/datetime/set");
    server.on(path,HTTP_POST,[apply](){
      if(!authenticate())return;
      uint32_t epoch;if(!RemotePolicy::utc(server.arg("val").c_str(),epoch)) {
        server.send(400,"text/plain",F("Use valid UTC YYYY-MM-DDTHH:MM:SSZ"));return;
      }
      if(apply) {JsonDocument request;request["utc_epoch"]=epoch;String result;
        if(!setBoardClock(request["utc_epoch"],result)) {server.send(409,"text/plain",result);return;}}
      JsonDocument d;d[F("epoch")]=epoch;d[F("applied")]=apply;sendMaintenanceJson(encodedJson(d));
    });
  }
  for(const char *path:{"/wifi/status","/config/ap","/config/st"})
    server.on(path,HTTP_GET,[](){if(authenticate())sendMaintenanceJson(networkTelemetry());});
  server.on("/wifi/signal",HTTP_GET,[](){
    if(!authenticate())return;JsonDocument d;d[F("connected")]=networkConfig.stationConnected();
    if(networkConfig.stationConnected())d[F("rssi_dbm")]=WiFi.RSSI();else d[F("rssi_dbm")]=nullptr;
    sendMaintenanceJson(encodedJson(d));
  });
  server.on("/wifi/country",HTTP_GET,[](){
    if(!authenticate())return;wifi_country_t country{};JsonDocument d;
#if defined(ARDUINO_ARCH_ESP32)
    if(esp_wifi_get_country(&country)==ESP_OK) {char cc[3]={country.cc[0],country.cc[1],0};d[F("country")]=cc;d[F("start_channel")]=country.schan;d[F("channels")]=country.nchan;}
#else
    if(wifi_get_country(&country)) {char cc[3]={country.cc[0],country.cc[1],0};d[F("country")]=cc;d[F("start_channel")]=country.schan;d[F("channels")]=country.nchan;}
#endif
    else d[F("country")]=nullptr;
    sendMaintenanceJson(encodedJson(d));
  });
  server.on("/hostname/check",HTTP_GET|HTTP_POST,[](){
    if(!authenticate())return;const String name=server.arg("value");
    if(!RemotePolicy::host(name.c_str(),name.length())) {server.send(400,"text/plain",F("Invalid DNS hostname"));return;}
    if(remoteFirmware.busy()) {server.send(409,"text/plain",F("Firmware operation active"));return;}
    IPAddress ip;
#if defined(ARDUINO_ARCH_ESP32)
    const bool ok=WiFi.hostByName(name.c_str(),ip)==1;
#else
    const bool ok=WiFi.hostByName(name.c_str(),ip,3000)==1;
#endif
    JsonDocument d;d[F("hostname")]=name;d[F("resolved")]=ok;if(ok)d[F("ip")]=ip.toString();else d[F("ip")]=nullptr;
    sendMaintenanceJson(encodedJson(d),ok?200:404);
  });
  for(const char *path:{"/auth_dump","/sessions_dump","/auth","/username"})
    server.on(path,HTTP_GET,[](){
      if(!authenticate())return;JsonDocument d;d[F("username")]=accessConfig.username();
      d[F("ready")]=accessConfig.ready();d[F("secrets")]=F("redacted");d[F("sessions")]=webAccess.sessionCount();
      d[F("role")]=F("device-owner");d[F("settings")]=F("/api/auth");sendMaintenanceJson(encodedJson(d));
    });
  server.on("/pass",HTTP_POST,[](){
    if(!authenticate())return;
    if(programmingLatched || remoteFirmware.busy()) {server.send(409,"text/plain",F("Maintenance active"));return;}
    if(!accessConfig.change(server.arg("current_password"),server.arg("username"),server.arg("password"))) {
      server.send(400,"text/plain",accessConfig.error());return;
    }
    webAccess.revokeAll();webSocket.closeAll(1008,"Credentials changed");
    closeDeviceShells();
    sendMaintenanceJson(F("{\"updated\":true,\"reauthenticate\":true}"));
  });
  server.on("/random",HTTP_GET,[](){
    if(!authenticate())return;uint8_t bytes[32];char hex[65];
    if(SystemPlatform::randomBytes(bytes,sizeof(bytes))) {server.send(503,"text/plain",F("Entropy unavailable"));return;}
    if(!BoardCore::DiagnosticText::encodeHex(bytes,sizeof(bytes),hex,sizeof(hex))) {server.send(503,"text/plain",F("Random encoding failed"));return;}
    JsonDocument d;d[F("random_hex")]=hex;sendMaintenanceJson(encodedJson(d));
  });
  server.on("/lang",HTTP_GET,[](){if(authenticate()){JsonDocument d;d[F("locale")]=displayLocale;d[F("translated")]=false;sendMaintenanceJson(encodedJson(d));}});
  server.on("/lang",HTTP_POST,[](){
    if(!authenticate())return;String lang=server.arg("lang");
    if(lang=="E")lang="en";else if(lang=="F")lang="fa";else if(lang=="A")lang="ar";else if(lang=="R")lang="ru";
    if(lang!="en" && lang!="fa" && lang!="ar" && lang!="ru") {server.send(400,"text/plain",F("Unsupported locale"));return;}
    File f=LittleFS.open("/locale.pending","w");
    const bool written=f && f.write(reinterpret_cast<const uint8_t*>(lang.c_str()),2)==2;
    if(f) {f.flush();f.close();}
    if(!written || !LittleFS.rename("/locale.pending","/locale.cfg")) {server.send(500,"text/plain",F("Locale storage failed"));return;}
    displayLocale=lang;sendMaintenanceJson(F("{\"updated\":true,\"translated\":false}"));
  });
  server.on("/beep",HTTP_POST,[](){
    if(!authenticate())return;uint32_t count=1;
    if((server.hasArg("count")&&!strictU32(server.arg("count"),count)) || count<1 || count>8 || programmingLatched) {
      server.send(400,"text/plain",F("Count must be 1..8 and Prog must be off"));return;}
    const uint8_t tone[]={0x10,0x03,100,0};bool ok=true;
    for(unsigned i=0;i<count && ok;++i){ok=requestAndWaitForAck(kBuzzer,tone,4);if(ok)serviceAvrFor(160);}
    sendMaintenanceJson(ok?"{\"acknowledged\":true}":"{\"error\":\"avr_ack_missing\"}",ok?200:503);
  });
  server.on("/message/send",HTTP_POST,[](){
    if(!authenticate())return;const String text=server.arg("text");
    if(!text.length() || text.length()>40 || programmingLatched) {server.send(400,"text/plain",F("Text must be 1..40 characters; Prog must be off"));return;}
    uint8_t p[44]={0,4,1,static_cast<uint8_t>(text.length())}; // 260 ms, single pass
    for(size_t i=0;i<text.length();++i) {if(text[i]<32 || text[i]>126){server.send(400,"text/plain",F("ASCII display text required"));return;}p[4+i]=text[i];}
    // Legacy Pakar kDisplayText accepts flags/scroll/brightness/length.
    const bool ok=requestAndWaitForAck(kDisplayText,p,text.length()+4);
    sendMaintenanceJson(ok?"{\"acknowledged\":true}":"{\"error\":\"avr_ack_missing\"}",ok?200:503);
  });
  server.on("/crash",HTTP_GET|HTTP_POST,[](){if(authenticate())server.send(410,"text/plain",F("Unsafe memory corruption endpoint removed; diagnostics remain available"));});
  for(const char *path:{"/debug/info","/debug/data","/eProcessState","/eResponseStatus"})
    server.on(path,HTTP_GET,[](){
      if(!authenticate())return;JsonDocument d;
      d[F("remote_phase")]=remoteFirmware.phase();d[F("remote_error")]=remoteFirmware.error();d[F("manual_ota_phase")]=PakarUpdate::name(otaProgress.snapshot().phase);
      d[F("avr_online")]=avrOnline;d[F("prog")]=programmingLatched;d[F("uart_errors")]=link.errors();d[F("usb_errors")]=hostLink.errors();
      d[F("last_reply_seen")]=awaitedReplySeen;
      if(awaitedReplySeen) {d[F("request_opcode")]=awaitedOpcode;d[F("acknowledged")]=awaitedReplyOk;} else d[F("request_opcode")]=nullptr;
      d[F("reset_reason")]=SystemPlatform::resetReason();d[F("raw_uart_logging")]=false;
      sendMaintenanceJson(encodedJson(d));
    });
  server.on("/debug/clear",HTTP_POST,[](){
    if(!authenticate())return;awaitedReplySeen=false;awaitedReplyOk=false;
    sendMaintenanceJson(F("{\"cleared\":true,\"persistent_update_journal_preserved\":true,\"error_counters_reset\":false}"));
  });
}
