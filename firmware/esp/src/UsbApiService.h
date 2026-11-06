#pragma once
// Included in application namespace. Reuses commandReply, ConfigurationManagement,
// FirmwarePlatform and the existing loop-owned HTTP server; no second renderer,
// OTA engine or raw partition writer is introduced here.
BoardCore::UsbApiRequest<> usbApiRequest;
String usbApiResponse;
uint32_t usbApiSecurityRevision=0;
BoardCore::FilesystemInitialization filesystemInitialization;
uint32_t filesystemInitializationToken=0;
bool filesystemInitializationComplete=false;
uint32_t filesystemInitializationRebootAt=0;

void serviceFilesystemInitialization() {
  if(filesystemInitializationRebootAt&&int32_t(millis()-filesystemInitializationRebootAt)>=0)ESP.restart();
}

void resetUsbApi(){usbApiRequest.clear();usbApiResponse=String();}
bool maintenanceApiIdle() {
  return !programmingLatched&&!urbootBusy.get()&&!otaActive&&!otaRebootAt&&
      !remoteFirmware.busy()&&!avrUploadOwner&&!artifactReaders&&!uploadFile&&
      !networkApplyPending&&!cloudApplyPending&&!cloudServices.ioActive()&&!passiveCaptureActive();
}
String maintenanceApiError(const char *error) {
  JsonDocument reply;reply["accepted"]=false;reply["error"]=error;return encodedJson(reply);
}
// Only a separate public user-files namespace may be read by this API. Settings,
// credentials, journals, firmware stage and backup files are deliberately absent.
bool maintenanceUserPath(const String &path) {
  if(!path.startsWith("/user/")||path.length()<=6||path.length()>70)return false;
  for(size_t i=6;i<path.length();++i){const char c=path[i];
    if(!((c>='a'&&c<='z')||(c>='A'&&c<='Z')||(c>='0'&&c<='9')||c=='-'||c=='_'||c=='.'))return false;
  }
  return path.indexOf("..")<0;
}
String maintenanceApi(const String &message,bool physicalUsb) {
  JsonDocument request,reply;
  if(message.length()>4096||deserializeJson(request,message)||!request.is<JsonObject>())return maintenanceApiError("invalid_request");
  const String method=request["method"]|"",path=request["path"]|"";
  const JsonObjectConst body=request["body"].as<JsonObjectConst>();
  // Read-only recovery identity works even when the filesystem cannot mount.
  if(method=="GET"&&path=="/api/maintenance") {
    reply["board_id"]=pcBoardId;reply["boot_id"]=pcBootId;
    reply["filesystem_mounted"]=filesystemTelemetry.mounted();
    reply["filesystem_initialized_reboot_required"]=filesystemInitializationComplete;
    reply["firmware_bytes"]=runningImageSize();reply["firmware_md5"]=runningImageMd5();
    reply["usb_api"]=true;reply["usb_firmware_read"]=true;
    reply["usb_firmware_upload"]=false;reply["file_write"]=true;reply["cloud_configuration_sync"]=false;
    reply["managed_files"]=true;reply["file_dates"]=true;reply["private_file_access"]=false;
    reply["microsd_built"]=bool(PAKAR_MICROSD_ENABLED);
    return encodedJson(reply);
  }
  if(request["board_id"].as<String>()!=pcBoardId||request["boot_id"].as<String>()!=pcBootId)return maintenanceApiError("peer_changed");
#if defined(ARDUINO_ARCH_ESP32) && PAKAR_NETWORK_ENABLED
  if(method=="GET"&&path=="/api/peer/info")return peerArtifactInfo.decorate(PakarPeer::info(pcBoardId,pcBootId,deviceIdentity.hostname()),peerCatalogue.sharing(),peerCatalogue.automaticChecks());
  if(method=="GET"&&path=="/api/peers/discovered")return peerCatalogue.snapshot(body["offset"]|0U);
  if(method=="GET"&&path=="/api/firmware/remote")return remoteFirmware.json();
  if(method=="POST"&&(path=="/api/peers/discover"||path=="/api/peers/policy"||path=="/api/peers/update"||path=="/api/firmware/apply"||path=="/api/firmware/cancel")) {
    if(filesystemInitializationComplete)return maintenanceApiError("initialization_requires_reboot");
    if(path=="/api/firmware/cancel"){
      if(!remoteFirmware.cancel(body["job_id"]|0U,body["boot_id"]|""))return maintenanceApiError(remoteFirmware.error().c_str());
    }else{
      if(!maintenanceApiIdle())return maintenanceApiError("maintenance_busy");
      if(path=="/api/peers/discover"){peerCatalogue.discover();return peerCatalogue.snapshot();}
      if(path=="/api/peers/policy"){String data;serializeJson(body,data);return peerCatalogue.configure(data);}
      if(path=="/api/peers/update"){
        if(peerCatalogue.busy())return maintenanceApiError("maintenance_busy");
        if(!remoteFirmware.stagePeer(body["source"].as<JsonObjectConst>(),body["authorization"]|""))return maintenanceApiError(remoteFirmware.error().c_str());
      }
      if(path=="/api/firmware/apply"&&!remoteFirmware.apply(body["job_id"]|0U,body["boot_id"]|"",body["sha256"]|""))return maintenanceApiError(remoteFirmware.error().c_str());
    }return remoteFirmware.json();
  }
#endif
  if(method=="GET"&&path=="/api/network/tools")return networkToolsJson();
  if(method=="GET"&&path=="/api/time")return clockJson();
  if(method=="GET"&&path=="/api/wifi/scan")return wifiScanJson(body["results"]!=true);
  if(method=="GET"&&path=="/api/network")return networkConfigurationJson();
#if defined(ARDUINO_ARCH_ESP32) && PAKAR_NETWORK_ENABLED
  if(method=="GET"&&path=="/api/peers")return peerRelay.directory();
  if(method=="GET"&&path=="/api/peers/result")return peerRelay.snapshot(body["request_id"]|"");
  if(method=="POST"&&path=="/api/peers/request") {
    String outgoing;serializeJson(body,outgoing);return submitPeerRelay(outgoing);
  }
  if(method=="POST"&&path=="/api/peers/routes"){
    if(!maintenanceApiIdle())return maintenanceApiError("maintenance_busy");
    String outgoing;serializeJson(body,outgoing);return peerRelay.saveRoute(outgoing);
  }
#endif
  if(physicalUsb&&filesystemInitializationComplete&&method=="POST"&&path=="/api/filesystem/reboot"&&
      body["confirmation"].as<String>()=="REBOOT INITIALIZED FILESYSTEM") {
    // Explicit second operation, never an automatic reboot after formatting.
    if(!filesystemInitializationRebootAt)filesystemInitializationRebootAt=millis()+3000U;
    reply["accepted"]=true;reply["reboot_in_ms"]=3000;return encodedJson(reply);
  }
  if(filesystemInitializationComplete)return maintenanceApiError("initialization_requires_reboot");
  if(!maintenanceApiIdle())return maintenanceApiError("maintenance_busy");
  if(method=="POST"&&path=="/api/time"){
    String result;if(!setBoardClock(body["utc_epoch"],result))return maintenanceApiError(result.c_str());return result;
  }
  if(method=="POST"&&(path=="/api/network/tools"||path=="/api/network/probe"||path=="/api/network")){
    String result;const bool ok=path=="/api/network/tools"?updateNetworkTools(body,result):
      path=="/api/network"?updateNetworkConfiguration(body,result):submitNetworkProbe(body["kind"]|"",body["target"]|"",result);
    if(!ok)return maintenanceApiError(result.c_str());return result;
  }
  if(method=="GET"&&path=="/api/led/output")return grid.outputJson();
  if(method=="POST"&&path=="/api/led/output"){
    if(body.size()!=1||!body["backend"].is<const char*>())return maintenanceApiError("backend_must_be_requested_separately");
    String error;if(!selectLedBackend(body["backend"].as<String>(),error))return maintenanceApiError(error.c_str());
    String receipt=grid.outputJson();receipt.remove(0,1);
    return String(F("{\"accepted\":true,"))+receipt;
  }
  if(method=="POST"&&(path=="/api/filesystem/prepare"||path=="/api/filesystem/initialize")) {
#if defined(ARDUINO_ARCH_ESP32)
    // Failed credentials storage must not turn into anonymous remote erase.
    // Recovery of an unmounted FS requires the physically connected USB path.
    if(!physicalUsb||filesystemTelemetry.mounted())return maintenanceApiError("requires_usb_and_unmounted_filesystem");
    if(path=="/api/filesystem/prepare"&&body["disable_led_output"].is<bool>()&&body["disable_led_output"].as<bool>())grid.setOutputEnabled(false);
    if(grid.outputEnabled())return maintenanceApiError("disable_led_output_before_initialization");
    if(path=="/api/filesystem/prepare") {
      filesystemInitialization.clear();filesystemInitializationToken=0;
      const auto *partition=esp_partition_find_first(ESP_PARTITION_TYPE_DATA,ESP_PARTITION_SUBTYPE_DATA_SPIFFS,"spiffs");
      if(!partition||!filesystemInitialization.begin(partition->size,false,true))return maintenanceApiError("filesystem_partition_missing");
      PakarS3::ScopedLedFlashPause pause;if(!pause)return maintenanceApiError("led_flash_guard_busy");
      uint8_t block[512];
      for(uint32_t offset=0;offset<partition->size;) {
        const size_t count=min<size_t>(sizeof(block),partition->size-offset);
        if(esp_partition_read(partition,offset,block,count)!=ESP_OK||!filesystemInitialization.inspect(offset,block,count)) {
          filesystemInitialization.clear();return maintenanceApiError("partition_not_erased_or_read_failed");
        }
        offset+=count;yield();
      }
      do{SystemPlatform::randomBytes(&filesystemInitializationToken,sizeof(filesystemInitializationToken));}while(!filesystemInitializationToken);
      if(!filesystemInitialization.arm(filesystemInitializationToken,millis()))return maintenanceApiError("initialization_guard_failed");
      reply["token"]=filesystemInitializationToken;reply["expires_in_ms"]=30000;
      reply["confirmation"]="INITIALIZE EMPTY FILESYSTEM";reply["verified_erased_bytes"]=partition->size;
      return encodedJson(reply);
    }
    if(!body["token"].is<uint32_t>()||!filesystemInitialization.consume(body["token"],millis(),false,true,body["confirmation"]|""))return maintenanceApiError("initialization_confirmation_rejected");
    PakarS3::ScopedLedFlashPause pause;if(!pause)return maintenanceApiError("led_flash_guard_busy");
    // Durable intent before format, same safety principle as SystemPlatform.
    nvs_handle_t handle;
    if(nvs_open("pakar-setup",NVS_READWRITE,&handle)!=ESP_OK)return maintenanceApiError("initialization_journal_failed");
    const bool journal=nvs_set_u8(handle,"fs-init",1)==ESP_OK&&nvs_commit(handle)==ESP_OK;
    nvs_close(handle);
    if(!journal)return maintenanceApiError("initialization_journal_failed");
    if(!LittleFS.format()||!LittleFS.begin(false))return maintenanceApiError("initialization_failed_no_automatic_retry");
    filesystemInitializationComplete=true;
    reply["accepted"]=true;reply["reboot_required"]=true;
    return encodedJson(reply);
#else
    return maintenanceApiError("unsupported_platform");
#endif
  }
  if(!accessConfig.ready()&&!(physicalUsb&&method=="GET"&&path=="/firmware/download"))return maintenanceApiError("settings_unavailable");
  if(method=="POST"&&path=="/api/storage/files")return userFileRequest(body);
  // HTTP was authenticated by its route. USB is physical authority, matching
  // existing USB network/access provisioning, not a network auth bypass.
  if(method=="POST"&&path=="/api/command") {
    String command;serializeJson(body,command);return commandReply(command,physicalUsb);
  }
  if(path=="/api/presentation"&&(method=="GET"||method=="POST")){
    if(method=="GET")return presentationJson(true);
    JsonDocument request;request.set(body);request["command"]="presentation";
    String command;serializeJson(request,command);return commandReply(command,physicalUsb);
  }
  if(path=="/api/pixels/groups"&&(method=="GET"||method=="POST")){
    const char *error=method=="POST"?savePixelGroups(body):nullptr;
    return error?maintenanceApiError(error):pixelGroupsJson();
  }
  if(path=="/api/pixels/frame"&&method=="GET"){
    const bool fenced=body.containsKey("group_layout_revision");
    if(fenced&&!body["group_layout_revision"].is<uint32_t>())return maintenanceApiError("invalid_group_layout_revision");
    return pixelFrameResponse(fenced,body["group_layout_revision"]|uint32_t(0));
  }
#if PAKAR_PIXEL_CORRECTIONS
  if((method=="GET"&&path=="/api/pixels/corrections") ||
     (method=="POST"&&(path=="/api/pixels/corrections"||path=="/api/pixels/corrections/preview"))) {
    pixelCorrections.service(millis());
    const char *error=nullptr;
    if(method=="POST")error=path=="/api/pixels/corrections/preview"?
      applyCorrectionPreview(body):pixelCorrections.save(body);
    grid.setCorrections(&pixelCorrections.mapping);
    keyObservationPending=true;
    return error?maintenanceApiError(error):pixelCorrections.json();
  }
#endif
  if(method=="GET"&&path=="/api/configuration/export") {
    reply["board_id"]=pcBoardId;reply["secrets_included"]=false;
    JsonDocument section;
    if(deserializeJson(section,pixelGroups.json()))return maintenanceApiError("configuration_read_failed");
    reply["pixel_groups"]=section.as<JsonObjectConst>();section.clear();
    if(deserializeJson(section,networkConfigurationJson()))return maintenanceApiError("configuration_read_failed");
    reply["network"]=section.as<JsonObjectConst>();
    section.clear();networkToolsSettings.append(section.to<JsonObject>());
    reply["network_tools"]=section.as<JsonObjectConst>();
    section.clear();if(deserializeJson(section,cloudConfigurationJson()))return maintenanceApiError("configuration_read_failed");
    reply["cloud"]=section.as<JsonObjectConst>();
    section.clear();if(deserializeJson(section,ethernetConfigurationJson()))return maintenanceApiError("configuration_read_failed");
    reply["ethernet"]=section.as<JsonObjectConst>();
#if PAKAR_PIXEL_CORRECTIONS
    section.clear();if(deserializeJson(section,pixelCorrections.json()))return maintenanceApiError("configuration_read_failed");
    section.remove("preview");section.remove("active_sources");section.remove("active_revision");
    reply["pixel_corrections"]=section.as<JsonObjectConst>();
#endif
    return encodedJson(reply);
  }
  if(method=="POST"&&path=="/api/configuration/import") {
    const String section=body["section"]|"";const auto settings=body["settings"].as<JsonObjectConst>();
    String result;bool ok=false;
    if(section=="network")ok=updateNetworkConfiguration(settings,result);
    else if(section=="network_tools")ok=updateNetworkTools(settings,result);
    else if(section=="pixel_groups"){
      const char *error=savePixelGroups(settings);ok=!error;result=error?String(error):pixelGroupsJson();
    }
    else if(section=="cloud")ok=updateCloudConfiguration(settings,result);
    else if(section=="ethernet")ok=updateEthernetConfiguration(settings,result);
#if PAKAR_PIXEL_CORRECTIONS
    else if(section=="pixel_corrections"){
      const char *error=pixelCorrections.save(settings);grid.setCorrections(&pixelCorrections.mapping);
      keyObservationPending=true;
      ok=!error;result=error?String(error):pixelCorrections.json();
    }
#endif
    else return maintenanceApiError("unsupported_configuration_section");
    reply["accepted"]=ok;reply["result"]=result;return encodedJson(reply);
  }
  if(method=="GET"&&(path=="/firmware/download"||path=="/api/files/read")) {
    if(!body["offset"].is<uint32_t>()||!body["count"].is<uint16_t>())return maintenanceApiError("invalid_range");
    const uint32_t offset=body["offset"];const uint16_t count=body["count"];
    if(!count||count>512)return maintenanceApiError("invalid_range");
    uint8_t bytes[512];uint32_t total=0;
#if defined(ARDUINO_ARCH_ESP32)
    PakarS3::ScopedLedFlashPause pause;if(!pause)return maintenanceApiError("led_flash_guard_busy");
#endif
    if(path=="/firmware/download") {
      total=runningImageSize();
      if(!total||offset>total||count>total-offset)return maintenanceApiError("invalid_range");
      if(!FirmwarePlatform::readRunning(offset,bytes,count))return maintenanceApiError("firmware_read_failed");
      reply["firmware_md5"]=runningImageMd5();
    } else {
      const String filename=body["file"]|"";if(!maintenanceUserPath(filename))return maintenanceApiError("protected_or_invalid_path");
      File file=LittleFS.open(filename,"r");if(!file||file.isDirectory())return maintenanceApiError("file_not_found");
      total=file.size();
      if(offset>total||count>total-offset||!file.seek(offset)||file.read(bytes,count)!=count)return maintenanceApiError("file_read_failed");
    }
    FirmwarePlatform::Sha256 digest;digest.add(bytes,count);char hex[1025];
    for(size_t i=0;i<count;++i)snprintf(hex+2*i,3,"%02x",bytes[i]);
    reply["offset"]=offset;reply["count"]=count;reply["total"]=total;reply["hex"]=hex;reply["sha256"]=digest.hex();reply["integrity_scope"]="chunk";
    return encodedJson(reply);
  }
  return maintenanceApiError("unsupported_api_operation");
}
bool handleUsbApiFrame(const BoardCore::Protocol::Frame &frame) {
  if(frame.opcode!=kUsbApi)return false;
  const auto *p=frame.payload;const uint8_t n=frame.payloadLength;
  if(!n){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
  const uint8_t action=p[0];uint32_t token=0;
  if(action==0) {
    if(n!=19||pcBootId.length()!=32){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
    char boot[33];for(unsigned i=0;i<16;++i)snprintf(boot+i*2,3,"%02x",p[i+1]);
    if(pcBootId!=boot){hostLink.error(frame.sequence,frame.opcode,kUnsafe);return true;}
    if(usbApiRequest.live(usbApiRequest.token(),millis())){hostLink.error(frame.sequence,frame.opcode,kBusy);return true;}
    do{SystemPlatform::randomBytes(&token,sizeof(token));}while(!token);
    if(!usbApiRequest.open(token,uint16_t(p[17])|(uint16_t(p[18])<<8),millis())){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
    usbApiResponse=String();usbApiSecurityRevision=accessConfig.securityRevision();
  } else {
    if(n<5){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
    token=uint32_t(p[1])|(uint32_t(p[2])<<8)|(uint32_t(p[3])<<16)|(uint32_t(p[4])<<24);
    if(!usbApiRequest.live(token,millis())||usbApiSecurityRevision!=accessConfig.securityRevision()) {resetUsbApi();hostLink.error(frame.sequence,frame.opcode,kUnsafe);return true;}
    if(action==1) {
      if(n<8){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
      const auto accepted=usbApiRequest.append(token,uint16_t(p[5])|(uint16_t(p[6])<<8),p+7,n-7,millis());
      if(accepted!=BoardCore::UsbApiRequest<>::Result::Accepted&&accepted!=BoardCore::UsbApiRequest<>::Result::Duplicate){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
    } else if(action==2&&n==5) {
      if(usbApiRequest.claim(token,millis())) {
        usbApiResponse=maintenanceApi(String(usbApiRequest.text()),true);
        if(!usbApiResponse.length()||usbApiResponse.length()>8192)usbApiResponse=maintenanceApiError("response_unavailable_do_not_replay");
      } else if(!usbApiRequest.claimed()){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
    } else if(action==4&&n==5) {resetUsbApi();hostLink.ack(frame.sequence,frame.opcode);return true;}
    else if(action!=3||n!=7||!usbApiRequest.claimed()){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
  }
  const uint16_t offset=action==3?uint16_t(p[5])|(uint16_t(p[6])<<8):0;
  const uint16_t total=usbApiResponse.length();
  if(offset>total){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
  uint8_t response[48]={action,uint8_t(token),uint8_t(token>>8),uint8_t(token>>16),uint8_t(token>>24),uint8_t(total),uint8_t(total>>8),uint8_t(offset),uint8_t(offset>>8)};
  const uint8_t count=action==3?min<size_t>(39,total-offset):0;
  if(count)memcpy(response+9,usbApiResponse.c_str()+offset,count);
  hostLink.send(kUsbApiResponse,frame.sequence,response,9+count);return true;
}
void registerUsbApiRoutes() {
  server.on("/api/maintenance/request",HTTP_POST,[](){
    if(!authenticate())return;
    const String result=maintenanceApi(server.jsonBody(),false);
    server.send(200,"application/json",result);
  });
}
