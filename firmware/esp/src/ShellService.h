#pragma once
// Included in the application's loop-owner namespace, after maintenance APIs.
// PU850 ShellSession/output routing and DeviceParameters are adapted here;
// terminal editing, TUI state and binary ownership remain independently tested.
#ifndef PAKAR_TELNET_SESSIONS
#define PAKAR_TELNET_SESSIONS 2
#endif
#ifndef PAKAR_SERIAL_SHELL_ENABLED
#define PAKAR_SERIAL_SHELL_ENABLED 1
#endif
#ifndef PAKAR_TELNET_ENABLED
#define PAKAR_TELNET_ENABLED 1
#endif
#ifndef PAKAR_REMOTE_QUERY_UDP_ENABLED
#define PAKAR_REMOTE_QUERY_UDP_ENABLED 0
#endif
#if PAKAR_REMOTE_QUERY_UDP_ENABLED
WiFiUDP shellQuerySocket;
PakarShell::RemoteQuery shellRemoteQuery;
void serviceShellRemoteQuery(){
  // Rebind when either interface address changes, including W5500 selection.
  const uint32_t primary=NetworkPath::ready()?uint32_t(NetworkPath::localIP()):0;
  const uint32_t accessPoint=apStarted?uint32_t(WiFi.softAPIP()):0;
  static uint32_t previousPrimary=0,previousAp=0,epoch=1;
  if(primary!=previousPrimary||accessPoint!=previousAp){previousPrimary=primary;previousAp=accessPoint;if(!++epoch)epoch=1;}
  shellRemoteQuery.service(shellQuerySocket,millis(),(primary||accessPoint)?epoch:0,accessConfig.ready()&&!accessConfig.enabled()&&shellSettings.enabled(),[](char* value,size_t capacity){
    snprintf(value,capacity,"hostname=%s boot_id=%s version=%s uptime=%lu avr=%s",deviceIdentity.hostname(),pcBootId.c_str(),BoardCore::BuildInfo::kVersion,static_cast<unsigned long>(millis()/1000U),avrOnline?"online":"offline");
  });
}
#endif
static_assert(PAKAR_TELNET_SESSIONS>=1&&PAKAR_TELNET_SESSIONS<=4,"Shell session count outside1..4");
struct DeviceShellSession : public Print {
  PakarShell::TelnetParser parser;
  PakarShell::LineEditor editor;
  PakarShell::ShellTui tui;
  PakarShell::OutputWindow serialOutput;
  String output;
  uint32_t lastInput=0,lastDrain=0,escapeAt=0;
  uint32_t accessRevision=0;
  bool authenticated=false,closing=false,fault=false,color=true,watch=false,upper=false,unicode=true,alternateScreen=false;
  bool commandOnly=false,terminal=true;uint8_t exitStatus=0;
  size_t write(uint8_t byte) override{return write(&byte,1);}
  size_t write(const uint8_t *bytes,size_t size) override {
    if(fault||output.length()+size>PS_DEVICE_VIEW_FRAME_BYTES+2048U){fault=true;return 0;}
    const size_t before=output.length();output.concat(reinterpret_cast<const char*>(bytes),size);
    if(!before&&size)lastDrain=millis();
    if(output.length()!=before+size){fault=true;return 0;}return size;
  }
  using Print::write;
};
std::unique_ptr<DeviceShellSession> telnetSessions[PAKAR_TELNET_SESSIONS],serialShell;
#if PAKAR_SSH_ENABLED
PakarS3::SshTransport deviceSsh;
std::unique_ptr<DeviceShellSession> sshShell;
uint32_t sshShellToken=0;
uint16_t sshAnnouncedPort=0;
bool updateSshConfiguration(JsonObjectConst patch,String &result);
#endif
WiFiClient shellPeers[PAKAR_TELNET_SESSIONS];
PakarShell::SerialChannel shellChannel;
uint32_t shellTokenCounter=0,shellRebindAt=0,lastShellObserve=0;
uint16_t shellListeningPort=0;
bool shellListening=false;
String shellObserved;
#if defined(ARDUINO_ARCH_ESP32)
std::atomic<bool> shellCloseRequested{false};
#else
bool shellCloseRequested=false;
#endif

#include "ShellProperties.h"
String shellGet(const ShellProperty &p){
  const String key(p.key);
  if(p.secret)return F("<write-only>");
  // PU850 Commands.cpp Firmware_Hash/Build_Date read-only parameter getters.
  if(key=="firmware.hash")return ESP.getSketchMD5();
  if(key=="build.date")return F(__DATE__ " " __TIME__);
  if(key=="hostname")return deviceIdentity.hostname();if(key=="label")return deviceIdentity.displayName();
  if(key=="clock.epoch")return String(static_cast<uint32_t>(time(nullptr)));
  if(key=="version")return BoardCore::BuildInfo::kVersion;if(key=="boot_id")return pcBootId;
  if(key=="uptime")return String(millis()/1000U);if(key=="heap")return String(ESP.getFreeHeap());
  if(key=="avr.online")return avrOnline?F("true"):F("false");
  if(key=="display.ack")return tm1637DetectionKnown()?(tm1637Ack?F("true"):F("false")):F("unknown");
  if(key=="firmware.phase")return PakarUpdate::name(otaProgress.snapshot().phase);
  if(key=="network.mode")return networkConfig.accessPointEnabled()?(networkConfig.stationEnabled()?F("both"):F("ap")):F("station");
  if(key=="network.ssid")return networkConfig.stationSsid();
  if(key=="network.station_enabled")return networkConfig.stationEnabled()?F("true"):F("false");
  if(key=="network.ap_enabled")return networkConfig.accessPointEnabled()?F("true"):F("false");
  if(key=="network.auto_switch")return networkConfig.autoSwitch()?F("true"):F("false");
  if(key=="network.ap_ssid")return String(deviceIdentity.hostname()).substring(0,32);
  if(key=="led.enabled")return grid.outputEnabled()?F("true"):F("false");
  if(key=="led.mode")return grid.staticMode()?F("static"):F("effects");
  if(key=="led.brightness")return String(grid.brightness());if(key=="led.effect")return grid.patternName();
  if(key=="shell.enabled")return shellSettings.enabled()?F("true"):F("false");
  if(key=="shell.port")return String(shellSettings.port());if(key=="shell.color")return shellSettings.color()?F("true"):F("false");
#if PAKAR_SSH_ENABLED
  if(key=="ssh.enabled")return deviceSsh.enabled()?F("true"):F("false");
  if(key=="ssh.port")return String(deviceSsh.port());
#endif
  JsonDocument d;
  if(String(p.domain)=="network-tools"){
    networkToolsSettings.append(d.to<JsonObject>());
    const auto value=d[p.field];if(value.is<bool>())return value.as<bool>()?F("true"):F("false");
    if(value.is<const char*>())return value.as<String>();return String(value.as<uint32_t>());
  }
#if PAKAR_FIRST_WINS_ENABLED
  if(String(p.domain)=="game"){
    if(deserializeJson(d,keySettings.json()))return F("unknown");
    const auto value=d[p.field];if(value.is<bool>())return value.as<bool>()?F("true"):F("false");
    if(value.is<const char*>())return value.as<String>();return String(value.as<uint32_t>());
  }
#endif
  const String json=String(p.domain)=="driver"?grid.outputCapabilitiesJson():String(p.domain)=="network"?networkConfigurationJson():cloudConfigurationJson();
  if(deserializeJson(d,json))return F("unknown");
  const auto v=d[p.field];if(v.is<bool>())return v.as<bool>()?F("true"):F("false");
  if(v.is<const char*>())return v.as<String>();if(v.is<uint32_t>())return String(v.as<uint32_t>());return F("unknown");
}
bool shellBusy(){return urbootBusy.get()||remoteFirmware.busy()||otaActive||otaRebootAt||avrUploadOwner||programmingLatched||cloudServices.ioActive();}
bool shellSet(const String &requestedKey,const String &value,String &result){
  ShellProperty property;if(!shellProperty(requestedKey.c_str(),property)||!property.writable){result=F("Unknown or read-only property");return false;}
  const String key(property.key);
  const auto *p=&property;
  if(shellBusy()){result=F("Maintenance active; nothing changed");return false;}
  uint32_t number=0;bool boolean=false;
  if(p->type=='b'){
    if(value!="true"&&value!="false"&&value!="on"&&value!="off"&&value!="1"&&value!="0"){result=F("Use true or false");return false;}
    boolean=value=="true"||value=="on"||value=="1";
  }
  if(p->type=='n'&&!PakarShell::unsignedNumber(value.c_str(),UINT32_MAX,number)){result=F("Invalid bounded integer");return false;}
#if PAKAR_SSH_ENABLED
  if(String(p->domain)=="ssh"){
    JsonDocument patch;if(p->type=='b')patch[p->field]=boolean;else patch[p->field]=number;
    return updateSshConfiguration(patch.as<JsonObjectConst>(),result);
  }
#endif
  if(String(p->domain)=="shell"){
    JsonDocument patch;if(p->type=='b')patch[p->field]=boolean;else patch[p->field]=number;
    return updateShellConfiguration(patch.as<JsonObjectConst>(),result);
  }
  if(String(p->domain)=="network-tools"){
    JsonDocument patch;
    if(p->type=='b')patch[p->field]=boolean;else if(p->type=='n')patch[p->field]=number;else patch[p->field]=value;
    return updateNetworkTools(patch.as<JsonObjectConst>(),result);
  }
  if(String(p->domain)=="clock"){
    JsonDocument patch;patch["utc_epoch"]=number;return setBoardClock(patch["utc_epoch"],result);
  }
  JsonDocument d;
  if(String(p->domain)=="device"){
    d["command"]="device-identity";d["hostname"]=key=="hostname"?value:String(deviceIdentity.hostname());
    d["display_name"]=key=="label"?value:String(deviceIdentity.displayName());
  }else if(String(p->domain)=="led"){
    if(key=="led.brightness"){d["command"]="brightness";d["value"]=number;}
    else if(key=="led.effect"){d["command"]="effect";d["value"]=value;}
    else {d["command"]="led-output";if(p->type=='b')d[p->field]=boolean;else d[p->field]=value;}
  }else if(String(p->domain)=="game"){
    d["command"]="game-settings";
    if(p->type=='b')d[p->field]=boolean;else if(p->type=='n')d[p->field]=number;else d[p->field]=value;
  }else{
    d["command"]=String(p->domain)=="network"?"configure-network":"configure-cloud";
    auto settings=d["settings"].to<JsonObject>();
    if(key=="network.mode"){
      if(value!="ap"&&value!="station"&&value!="both"){result=F("Use ap, station, or both");return false;}
      settings["ap_enabled"]=value!="station";settings["station_enabled"]=value!="ap";
    }else if(p->type=='b')settings[p->field]=boolean;
    else if(p->type=='n')settings[p->field]=number;else settings[p->field]=value;
  }
  String json;serializeJson(d,json);return handleCloudCommand(json,result);
}
void shellPrompt(DeviceShellSession &s){
  if(s.commandOnly)return;
  s.editor.setTheme(s.tui.theme(),s.tui.trueColor());
  s.editor.setPrompt(deviceIdentity.hostname(),s.color&&s.parser.ansi());
  s.editor.prompt([&](const uint8_t *p,size_t n){s.write(p,n);});
}
void shellSafePrint(DeviceShellSession &s,const String &message){
  char safe[513];for(size_t at=0;at<message.length();at+=512){
    PakarShell::terminalText(safe,sizeof(safe),message.c_str()+at,min<size_t>(512,message.length()-at));s.print(safe);
  }
}
void shellResult(DeviceShellSession &s,bool accepted,const String &message){
  if(!accepted)s.exitStatus=1;
  if(message.length()+s.output.length()+128U>8192U){
    s.exitStatus=1;
    s.println(F("ERR Response exceeds terminal window; use the JSON API for the full snapshot"));return;
  }
  if(s.color)s.print(accepted?F("\x1b[32m"):F("\x1b[31m"));
  s.print(accepted?F("OK "):F("ERR "));shellSafePrint(s,message);
  if(s.color)s.print(F("\x1b[0m"));s.println();
}
void shellTuiDraw(DeviceShellSession &s,bool force=false);
void shellTuiInput(DeviceShellSession &s,uint8_t ch,bool flushEscape=false);
String shellWifiScanJson(){
  return wifiScanJson(false);
}

void executeDeviceShell(DeviceShellSession &s,String line){
  s.exitStatus=0;
  if(!s.authenticated||!accessConfig.ready()||s.accessRevision!=accessConfig.securityRevision()){s.exitStatus=1;s.closing=true;return;}
  line.trim();if(!line.length()){shellPrompt(s);return;}
  if(s.output.length()>1024U){shellResult(s,false,F("Terminal busy; drain output before another command"));shellPrompt(s);return;}
  const int space=line.indexOf(' ');String verb=space<0?line:line.substring(0,space);
  verb.toLowerCase();String args=space<0?String():line.substring(space+1);args.trim();
  const auto *command=PakarShell::findCommand(verb.c_str());
  if(!command||!PakarShell::commandAvailable(*command,PAKAR_FIRST_WINS_ENABLED)){shellResult(s,false,F("Unknown or unavailable command; use help"));shellPrompt(s);return;}
  verb=command->canonical;
  const bool query=command->readOnly||((verb=="firmware"||verb=="game")&&args=="status");
  if(shellBusy()&&!query){shellResult(s,false,F("Maintenance active; command not dispatched"));shellPrompt(s);return;}
  if(verb=="exit"||verb=="quit"){s.println(F("Goodbye."));s.closing=true;return;}
  if(verb=="tui"){
    if(s.commandOnly&&!s.terminal){shellResult(s,false,F("TUI requires a terminal; use ssh -t HOST tui"));return;}
    s.alternateScreen=s.color&&s.parser.ansi();if(s.alternateScreen)s.print(F("\x1b[?1049h"));s.tui.begin(s.color&&s.parser.ansi());shellTuiDraw(s,true);return;
  }
  if(verb=="help"||verb=="?"){
    PakarShell::renderHelp([&](const uint8_t* p,size_t n){s.write(p,n);},PAKAR_FIRST_WINS_ENABLED,s.color&&s.parser.ansi(),s.unicode,s.parser.columns());
  }else if(verb=="properties"||verb=="get"){
    if(args.length()){
      ShellProperty property;if(shellProperty(args.c_str(),property)){shellSafePrint(s,shellGet(property));s.println();}else shellResult(s,false,F("Unknown property"));
    }else for(size_t index=0;index<shellPropertyCount;++index){const auto p=shellPropertyAt(index);s.print(p.writable?F("rw "):F("ro "));s.print(p.key);s.print(F(" = "));shellSafePrint(s,shellGet(p));s.println();}
  }else if(verb=="terminal"){
    if(args=="unicode on"||args=="unicode off")s.unicode=args.endsWith("on");
    else if(args=="color on"||args=="color off")s.color=args.endsWith("on");
    else if(args=="truecolor on"||args=="truecolor off")s.tui.setTrueColor(args.endsWith("on"));
    else if(args.startsWith("theme ")){
      const String theme=args.substring(6);using T=PakarShell::ShellTui;
      if(theme!="default"&&theme!="violet"&&theme!="red"&&theme!="cyan"&&theme!="onedark"){
        shellResult(s,false,F("Use terminal theme default|violet|red|cyan|onedark"));shellPrompt(s);return;}
      s.tui.setTheme(theme=="default"?T::Default:theme=="red"?T::Red:theme=="cyan"?T::Cyan:theme=="onedark"?T::OneDark:T::Violet);
    }
    else if(args.startsWith("size ")){
      const int split=args.indexOf(' ',5);uint32_t w=0,h=0;
      if(split<0||!PakarShell::unsignedNumber(args.substring(5,split).c_str(),240,w)||
         !PakarShell::unsignedNumber(args.substring(split+1).c_str(),100,h)||!s.parser.size(w,h)){
        shellResult(s,false,F("Use terminal size 40..240 16..100"));shellPrompt(s);return;}
    }else if(args.length()){shellResult(s,false,F("Use terminal unicode|color|truecolor on|off, terminal theme default|violet|red|cyan|onedark, terminal size COLS ROWS"));shellPrompt(s);return;}
    s.print(F("Terminal "));s.print(s.parser.terminal());s.print(F(" | "));s.print(s.parser.columns());s.print(s.unicode?F(" \xc3\x97 "):F(" x "));s.print(s.parser.rows());
    s.print(F(" | Unicode "));s.print(s.unicode?F("on"):F("off"));s.print(F(" | ANSI "));s.println(s.color&&s.parser.ansi()?F("on"):F("off"));
  }else if(verb=="game"){
    String result;JsonDocument d;bool valid=PAKAR_FIRST_WINS_ENABLED;
    if(args=="status"){
#if PAKAR_FIRST_WINS_ENABLED
      s.print(F("Winner "));s.println(winner);s.println(keySettings.json());
#else
      shellResult(s,false,F("This board has no First-wins game"));
#endif
    }else{
      if(args=="reset")d["command"]="reset";
      else if(args.startsWith("press ")){uint32_t key=0;valid=valid&&PakarShell::unsignedNumber(args.substring(6).c_str(),9,key)&&key>0;d["command"]="game-press";d["key"]=key;}
      else valid=false;
      if(!valid)shellResult(s,false,F("Use game status|press 1..9|reset on a First-wins board"));
      else{String json;serializeJson(d,json);const bool accepted=handleCloudCommand(json,result);shellResult(s,accepted,result);}
    }
  }else if(verb=="set"){
    const int at=args.indexOf(' ');String result;
    if(at<1)shellResult(s,false,F("Use set KEY VALUE"));
    else{String value=args.substring(at+1);value.trim();if(value.length()>=2&&value[0]=='"'&&value[value.length()-1]=='"')value=value.substring(1,value.length()-1);
      shellResult(s,shellSet(args.substring(0,at),value,result),result);}
  }else if(verb=="clear"||verb=="cls")PakarShell::Terminal::clear([&](const uint8_t *p,size_t n){s.write(p,n);},s.color);
  else if(verb=="echo")s.println(args);
  else if(verb=="history")s.editor.printHistory([&](const uint8_t *p,size_t n){s.write(p,n);});
  else if(verb=="upper"||verb=="noupper"){s.upper=verb=="upper";s.editor.setUpper(s.upper);}
  else if(verb=="color"||verb=="watch"){
    if(args!="on"&&args!="off")shellResult(s,false,F("Use on or off"));
    else{if(verb=="color")s.color=args=="on";else s.watch=args=="on";shellResult(s,true,F("Session setting changed"));}
  }else if(verb=="hex"){
    uint32_t number;if(!PakarShell::unsignedNumber(args.c_str(),UINT32_MAX,number))shellResult(s,false,F("Invalid unsigned integer"));
    else {s.print(F("0x"));s.println(number,HEX);}
  }else if(verb=="display"&&args.startsWith("text ")){
    String rest=args.substring(5),fields[5];bool valid=true;uint32_t values[4]{};
    for(unsigned i=0;i<5;++i){const int at=rest.indexOf(' ');if(at<1){valid=false;break;}fields[i]=rest.substring(0,at);rest=rest.substring(at+1);}
    for(unsigned i=0;i<4;++i)valid=valid&&PakarShell::unsignedNumber(fields[i+1].c_str(),i<2?65535U:i==2?255U:1U,values[i]);
    String result;const bool ok=valid&&submitDisplayText(rest,values[0],values[1],fields[0],values[3]!=0,values[2],result);
    shellResult(s,ok,result.length()?result:String("Use display text once|loop|interval SPEED HOLD PAUSE SCROLL TEXT"));
  }else if(verb=="status"){
    s.print(F("Host "));s.print(hostname);s.print(F(" | STA "));s.print(networkConfig.stationConnected()?F("up"):F("down"));
    s.print(F(" | AP "));s.print(apStarted?F("up"):F("down"));s.print(F(" | AVR "));s.println(avrOnline?F("up"):F("down"));
    s.println(grid.outputJson());
  }else if(verb=="info"||verb=="telemetry"||verb=="firmware"){
    String result;const bool ok=handleMaintenanceShell(verb+(args.length()?" "+args:String()),result);shellResult(s,ok,result);
  }else if(verb=="clock"){
    s.println(networkToolsJson());
  }else if(verb=="dns"||verb=="ping"||verb=="public-ip"||verb=="http-date"){
    String result;const bool accepted=submitNetworkProbe(verb,args,result);shellResult(s,accepted,result);
  }else if(verb=="network"){
    const auto code=args=="scan"?espPanelSettings.scan():args=="reconnect"?espPanelSettings.reconnect():kUnsupported;
    if(args=="scan-results")s.println(shellWifiScanJson());
    else shellResult(s,code==kNoError,code==kNoError?F("Queued; use network scan-results for completed scan"):F("Unknown action or device busy"));
  }else if(verb=="device"&&args=="identity")s.println(deviceNameJson());
  else if(verb=="device"&&(args.startsWith("rename ")||args.startsWith("label "))){
    String result;const bool rename=args.startsWith("rename ");shellResult(s,shellSet(rename?"hostname":"label",args.substring(rename?7:6),result),result);
  }else{
    JsonDocument d;d["command"]=verb;
    bool valid=true;String result;
    if(verb=="led-output"){
      if(args=="on"||args=="off")d["enabled"]=args=="on";
      else if(args=="static"||args=="effects")d["mode"]=args;else valid=false;
    }else if(verb=="brightness"||verb=="display"){
      uint32_t number;if(verb=="display"&&args=="stop")d["release"]=true;
      else if(verb=="display"&&args.startsWith("-")){
        if(!PakarShell::unsignedNumber(args.substring(1).c_str(),999,number))valid=false;else d["value"]=-int32_t(number);
      }else if(!PakarShell::unsignedNumber(args.c_str(),verb=="display"?9999U:255U,number))valid=false;
      else d["value"]=number;
    }else if(verb=="mode"||verb=="effect")d["value"]=args;
    else if(verb=="audio")d["cue"]=args;
    else if(verb=="beep"){
      PakarShell::ToneCommand tone;valid=PakarShell::toneCommand(args.c_str(),tone);
      d["command"]="buzzer";d["frequency"]=tone.frequency;d["duration_ms"]=tone.duration;
      if(!valid){shellResult(s,false,F("Use beep [20..12000 Hz [1..5000 ms]]"));shellPrompt(s);return;}
    }
    else if(verb=="cell"||verb=="cells"){
      const int a=args.indexOf(' '),b=a<0?-1:args.indexOf(' ',a+1);
      if(a<1)valid=false;else{
        if(verb=="cell"){uint32_t id;if(!PakarShell::unsignedNumber(args.substring(0,a).c_str(),80,id)||!id)valid=false;else d["id"]=id;}
        else d["ids"]=args.substring(0,a);
        d["state"]=b<0?args.substring(a+1):args.substring(a+1,b);d["effect"]=b<0?String("immediate"):args.substring(b+1);
      }
    }else if(verb=="pixel"){
      PakarShell::PixelCommand pixel;valid=PakarShell::pixelCommand(args.c_str(),grid.groupLayout(),pixel);
      d["command"]="pixel-diagnostic";d["off"]=pixel.off;d["index"]=pixel.index;d["count"]=pixel.count;d["rgb"]=pixel.rgb;
      d["group_layout_revision"]=pixel.revision;
      if(!valid){shellResult(s,false,F("Use a zero-based pixel or an exact physical group: pixel group 1 FF0000. COUNT must be 1 or the full current group."));shellPrompt(s);return;}
    }else if(verb!="reset")valid=false;
    if(!valid)shellResult(s,false,F("Invalid command/arguments; use help"));
    else{String json;serializeJson(d,json);const bool ok=handleCloudCommand(json,result);shellResult(s,ok,result);}
  }
  if(!s.closing&&!s.tui.active())shellPrompt(s);
}
void initializeDeviceShell(DeviceShellSession &s,bool telnet,bool transportAuthenticated=false,bool commandOnly=false){
  s.commandOnly=commandOnly;
  s.accessRevision=accessConfig.securityRevision();
  s.authenticated=accessConfig.ready()&&(transportAuthenticated||!accessConfig.enabled());s.color=shellSettings.color();s.lastInput=s.lastDrain=millis();
  if(telnet)s.parser.greet([&](const uint8_t *p,size_t n){s.write(p,n);});
  if(commandOnly)return; // SSH exec has no banner, password prompt or editor echo.
  if(s.color)s.print(F("\x1b[1;32m"));s.print(APP_NAME);s.print(F(" @ "));s.print(deviceIdentity.hostname());
  if(s.color)s.print(F("\x1b[0m"));s.println();
  s.editor.setEcho(s.authenticated);if(s.authenticated)shellPrompt(s);else s.print(F("Password: "));
}
void inputDeviceShell(DeviceShellSession &s,uint8_t ch){
  if(!accessConfig.ready()||s.accessRevision!=accessConfig.securityRevision()){s.closing=true;return;}
  if(s.closing||s.fault)return;s.lastInput=millis();
  if(s.tui.active()){shellTuiInput(s,ch);s.escapeAt=millis();return;}
  const auto event=s.editor.feed(ch,[&](const uint8_t *p,size_t n){s.write(p,n);});
  if(event==PakarShell::LineEditor::None){
    if(!s.authenticated)return;
    const char* line=s.editor.line();const char* space=strchr(line,' ');
    auto write=[&](const uint8_t* p,size_t n){s.write(p,n);};
    if(!space)s.editor.suggest(0,PakarShell::commandCount,[](size_t i)->const char*{
      const auto& command=PakarShell::commands[i];
      return PakarShell::commandAvailable(command,PAKAR_FIRST_WINS_ENABLED)?command.name:nullptr;
    },write);
    else if(PakarShell::namePrefix(line,"get ",4)||PakarShell::namePrefix(line,"set ",4)||PakarShell::namePrefix(line,"properties ",11)){
      const size_t start=space-line+1;
      if(!strchr(line+start,' '))s.editor.suggest(start,shellPropertyCount,[&](size_t i){
        const auto p=shellPropertyAt(i);return (PakarShell::namePrefix(line,"set ",4)&&!p.writable)?nullptr:p.key;
      },write);
    }
    return;
  }
  if(event==PakarShell::LineEditor::Complete){
    if(!s.authenticated)return;
    auto write=[&](const uint8_t *p,size_t n){s.write(p,n);};
    const char* line=s.editor.line();const char* space=strchr(line,' ');
    if(!space)s.editor.complete(0,PakarShell::commandCount,[](size_t i)->const char*{
      const auto& command=PakarShell::commands[i];
      return PakarShell::commandAvailable(command,PAKAR_FIRST_WINS_ENABLED)?command.name:nullptr;
    },write);
    else if(PakarShell::namePrefix(line,"get ",4)||PakarShell::namePrefix(line,"set ",4)||PakarShell::namePrefix(line,"properties ",11)){
      const size_t start=space-line+1;
      if(!strchr(line+start,' '))s.editor.complete(start,shellPropertyCount,[&](size_t i){
        const auto p=shellPropertyAt(i);return (PakarShell::namePrefix(line,"set ",4)&&!p.writable)?nullptr:p.key;
      },write);
    }
    return;
  }
  if(event==PakarShell::LineEditor::Close){s.closing=true;return;}
  if(event==PakarShell::LineEditor::Cancel||event==PakarShell::LineEditor::TooLong){
    if(event==PakarShell::LineEditor::TooLong)shellResult(s,false,F("Line too long; discarded"));s.editor.clear();
    if(s.authenticated)shellPrompt(s);else s.print(F("Password: "));return;
  }
  if(!s.authenticated){
    const bool valid=accessConfig.ready()&&BoardCore::Access::equal(s.editor.line(),accessConfig.password());s.editor.clear();
    if(!valid){s.println(F("Authentication failed."));s.closing=true;return;}
    s.authenticated=true;s.editor.setEcho(true);shellPrompt(s);return;
  }
  String line(s.editor.line());line.trim();bool historySafe=true;
  if(PakarShell::namePrefix(line.c_str(),"set ",4)){const int at=line.indexOf(' ',4);ShellProperty property;historySafe=at>4&&shellProperty(line.substring(4,at).c_str(),property)&&!property.secret;}
  if(line.indexOf("://")>=0)historySafe=false;
  s.editor.remember(historySafe);s.editor.clear();executeDeviceShell(s,line);
}
bool ftpPortReserved(uint16_t port);
#include "FileTransferBinding.h"
#include "SshShellBinding.h"
#include "FtpBinding.h"
bool ftpPortReserved(uint16_t port){
#if PAKAR_FTP_ENABLED
  return port==DeviceFtp::settings.port||port==DeviceFtp::settings.passivePort;
#else
  (void)port;return false;
#endif
}
void closeDeviceShells(){
  // Auth callbacks can run on AsyncTCP's task. Only the main loop destroys
  // editor/socket owners; input is independently fenced by accessRevision.
  shellCloseRequested=true;
}
void retireDeviceShells(){
#if PAKAR_SSH_ENABLED
  deviceSsh.invalidate();sshShell.reset();sshShellToken=0;
#if PAKAR_SCP_ENABLED || PAKAR_SFTP_ENABLED
  resetSshTransfer();
#endif
#if PAKAR_SCP_ENABLED
  sshScp=SshScpSession{};
#endif
#endif
  for(unsigned i=0;i<PAKAR_TELNET_SESSIONS;++i){shellPeers[i].stop();telnetSessions[i].reset();}
  serialShell.reset();shellChannel.close();
}
unsigned shellConnectionCount(){unsigned n=0;for(unsigned i=0;i<PAKAR_TELNET_SESSIONS;++i)if(telnetSessions[i]&&shellPeers[i].connected())++n;return n;}
uint16_t shellAdvertisedPort(){return shellListening?shellListeningPort:0;}
String shellStatusJson(){
  JsonDocument d;d["enabled"]=bool(PAKAR_TELNET_ENABLED)&&shellSettings.enabled();d["port"]=shellSettings.port();
  d["listening"]=shellListening;d["sessions"]=shellConnectionCount();d["session_limit"]=PAKAR_TELNET_SESSIONS;
  d["color"]=shellSettings.color();d["history_limit"]=PS_EDITOR_HISTORY;d["serial_encapsulated"]=bool(PAKAR_SERIAL_SHELL_ENABLED);
  d["udp_query_built"]=bool(PAKAR_REMOTE_QUERY_UDP_ENABLED);d["udp_query_read_only"]=true;
#if PAKAR_REMOTE_QUERY_UDP_ENABLED
  d["udp_query_listening"]=shellRemoteQuery.listening();d["udp_query_port"]=PakarShell::RemoteQuery::port;
#endif
  d["serial_open"]=shellChannel.active();d["tui"]=true;
#if PAKAR_SSH_ENABLED
  JsonDocument ssh;deserializeJson(ssh,sshStatusJson());d["ssh"]=ssh.as<JsonVariantConst>();
#else
  d["ssh"]["built"]=false;
#endif
#if PAKAR_FTP_ENABLED
  JsonDocument ftp;deserializeJson(ftp,DeviceFtp::status());d["ftp"]=ftp.as<JsonVariantConst>();
#else
  d["ftp"]["built"]=false;
#endif
  String json;serializeJson(d,json);return json;
}
void beginDeviceShells(){shellSettings.begin(LittleFS);shellRebindAt=millis();
#if PAKAR_FTP_ENABLED
  DeviceFtp::begin();
#endif
#if PAKAR_SSH_ENABLED
  deviceSsh.begin();
#endif
}
void serviceDeviceShells(){
  const uint32_t now=millis();
#if PAKAR_SCP_ENABLED || PAKAR_SFTP_ENABLED || PAKAR_FTP_ENABLED
  serviceFileTransferCleanup();
#endif
#if PAKAR_FTP_ENABLED
  DeviceFtp::service();
#endif
#if PAKAR_REMOTE_QUERY_UDP_ENABLED
  serviceShellRemoteQuery();
#endif
#if defined(ARDUINO_ARCH_ESP32)
  if(shellCloseRequested.exchange(false))retireDeviceShells();
#else
  if(shellCloseRequested){shellCloseRequested=false;retireDeviceShells();}
#endif
#if PAKAR_SSH_ENABLED
  serviceSshShell();
#endif
  if(shellRebindAt&&int32_t(now-shellRebindAt)>=0){
    shellRebindAt=0;retireDeviceShells();telnetServer.stop();shellListening=false;shellListeningPort=0;
    if(PAKAR_TELNET_ENABLED&&shellSettings.enabled()&&accessConfig.ready()){
      telnetServer.begin(shellSettings.port());telnetServer.setNoDelay(true);
#if defined(ARDUINO_ARCH_ESP32)
      shellListening=bool(telnetServer);
#else
      shellListening=telnetServer.status()==LISTEN;
#endif
      if(shellListening)shellListeningPort=shellSettings.port();
    }
    startDiscovery();
  }
  if(shellListening&&telnetServer.hasClient()){
    WiFiClient candidate=telnetServer.accept();bool admitted=false;
    for(unsigned i=0;i<PAKAR_TELNET_SESSIONS;++i)if(!telnetSessions[i]){
      auto *session=new(std::nothrow) DeviceShellSession;if(!session)break;
      shellPeers[i]=candidate;shellPeers[i].setNoDelay(true);telnetSessions[i].reset(session);initializeDeviceShell(*session,true);admitted=true;break;
    }
    if(!admitted)candidate.stop();
  }
  for(unsigned i=0;i<PAKAR_TELNET_SESSIONS;++i){
    if(!telnetSessions[i])continue;auto &s=*telnetSessions[i];auto &peer=shellPeers[i];
    // Accept/greeting and earlier session work can stamp a time later than the
    // loop-entry snapshot. Sample AFTER them; unsigned stale-now subtraction
    // otherwise looks like a 49-day idle lease and immediately drops a peer.
    const uint32_t leaseNow=millis();
    if(!peer.connected()||s.fault||!accessConfig.ready()||s.accessRevision!=accessConfig.securityRevision()||uint32_t(leaseNow-s.lastInput)>(s.authenticated?1800000U:60000U)||
        (s.output.length()&&uint32_t(leaseNow-s.lastDrain)>30000U)){peer.stop();telnetSessions[i].reset();continue;}
    if(s.output.length()){
      const size_t count=min<size_t>(192,s.output.length());int sent=0;
#if defined(ARDUINO_ARCH_ESP32)
      // One nonblocking write. Slow terminals cannot hold up UART/LED/API work.
      sent=::send(peer.fd(),s.output.c_str(),count,MSG_DONTWAIT);
      if(sent<0&&errno!=EWOULDBLOCK&&errno!=EAGAIN){s.fault=true;continue;}
#else
      const size_t available=peer.availableForWrite();if(available)sent=peer.write(reinterpret_cast<const uint8_t*>(s.output.c_str()),min(count,available));
#endif
      if(sent>0){s.output.remove(0,sent);s.lastDrain=millis();}
    }
    if(s.closing){if(!s.output.length()){peer.stop();telnetSessions[i].reset();}continue;}
    // Backpressure before consuming more commands; no truncation or replay.
    unsigned budget=s.output.length()<1024?64:0;bool terminalInput=false;
    while(budget--&&peer.available()&&s.output.length()<1024U){
      const int byte=peer.read();if(byte<0)break;
      terminalInput=true;
      s.parser.feed(uint8_t(byte),[&](const uint8_t *p,size_t n){s.write(p,n);},[&](uint8_t ch){inputDeviceShell(s,ch);});
      if(s.closing||s.fault)break;
    }
    if(s.tui.active()&&s.tui.escapePending()&&uint32_t(millis()-s.escapeAt)>350){shellTuiInput(s,0,true);terminalInput=true;}
    if(terminalInput&&s.tui.active()&&s.output.length()<4096U)shellTuiDraw(s);
  }
  const uint32_t serialNow=millis();
  if(serialShell&&(!accessConfig.ready()||serialShell->accessRevision!=accessConfig.securityRevision()||uint32_t(serialNow-serialShell->lastInput)>30000U)){serialShell.reset();shellChannel.close();}
  if(uint32_t(now-lastShellObserve)>=500U){
    lastShellObserve=now;
    String observed=String("event STA ")+(networkConfig.stationConnected()?"up":"down")+" AP "+(apStarted?"up":"down")+" AVR "+(avrOnline?"up":"down")+" update "+(remoteFirmware.busy()?String(remoteFirmware.phase()):String(PakarUpdate::name(otaProgress.snapshot().phase)));
    const auto progress=remoteFirmware.busy()?remoteFirmware.progress():otaProgress.snapshot();
    if(progress.known())observed+=" "+String(progress.percent())+"%";
#if PAKAR_FIRST_WINS_ENABLED
    observed+=" | winner "+String(winner)+" enabled_mask "+String(keySettings.enabledMask);
#else
    unsigned red=0,off=0;for(uint8_t id=1;id<=80;++id){red+=grid.state(id)==kCellRed;off+=grid.state(id)==kCellOff;}
    observed+=" | cells red "+String(red)+" off "+String(off);
#endif
    observed+=" | output "+String(grid.outputEnabled()?"on":"off");
    if(observed!=shellObserved){shellObserved=observed;
      for(auto &slot:telnetSessions)if(slot&&slot->authenticated&&slot->watch&&!slot->tui.active()&&slot->output.length()<1024U)
        slot->editor.redrawEvent(observed.c_str(),[&](const uint8_t *p,size_t n){slot->write(p,n);});
      if(serialShell&&serialShell->authenticated&&serialShell->watch&&!serialShell->tui.active()&&serialShell->output.length()<1024U)
        serialShell->editor.redrawEvent(observed.c_str(),[&](const uint8_t *p,size_t n){serialShell->write(p,n);});
#if PAKAR_SSH_ENABLED
      if(sshShell&&!sshShell->closing&&sshShell->watch&&!sshShell->tui.active()&&sshShell->output.length()<1024U)
        sshShell->editor.redrawEvent(observed.c_str(),[&](const uint8_t *p,size_t n){sshShell->write(p,n);});
#endif
    }
    for(auto &slot:telnetSessions)if(slot&&slot->authenticated&&slot->tui.active()&&slot->output.length()<1024U)shellTuiDraw(*slot);
    if(serialShell&&serialShell->authenticated&&serialShell->tui.active()&&serialShell->output.length()<1024U)shellTuiDraw(*serialShell);
#if PAKAR_SSH_ENABLED
    if(sshShell&&!sshShell->closing&&sshShell->tui.active()&&sshShell->output.length()<1024U)shellTuiDraw(*sshShell);
#endif
  }
}
bool updateShellConfiguration(JsonObjectConst patch,String &result){
  if(shellBusy()||networkApplyPending||cloudApplyPending){result=F("Device busy; shell settings unchanged");return false;}
  if(patch.isNull()||!patch.size()||patch.size()>3){result=F("Supply enabled, port and/or color");return false;}
  bool enabled=shellSettings.enabled(),color=shellSettings.color();uint16_t port=shellSettings.port();
  for(JsonPairConst pair:patch){const String key=pair.key().c_str();const auto value=pair.value();
    if(key=="enabled"||key=="color"){
      if(!value.is<bool>()){result=F("Shell enabled/color must be boolean");return false;}
      if(key=="enabled")enabled=value.as<bool>();else color=value.as<bool>();
    }else if(key=="port"){
      if(!value.is<uint16_t>()||!value.as<uint16_t>()){result=F("Invalid shell TCP port");return false;}port=value.as<uint16_t>();
    }else{result=F("Unknown shell setting");return false;}
  }
#if PAKAR_SSH_ENABLED
  if(port==deviceSsh.port()){result=F("Telnet and SSH require distinct ports");return false;}
#endif
  if(ftpPortReserved(port)||!shellSettings.configure(enabled,port,color)){result=F("Invalid/reserved port or durable shell save failed");return false;}
  shellRebindAt=millis()+1000U;result=F("Shell settings saved; listener applies after response");return true;
}
void registerShellApi(){
#if PAKAR_FTP_ENABLED
  DeviceFtp::api();
#endif
#if PAKAR_SSH_ENABLED
  registerSshApi();
#endif
  server.on("/api/shell",HTTP_GET,[](){if(authenticate())server.send(200,"application/json",shellStatusJson());});
  server.on("/api/shell",HTTP_POST,[](){
    if(!authenticate())return;
    JsonDocument patch;
    for(const char *key:{"enabled","color"})if(server.hasArg(key)){
      const String value=server.arg(key);if(value!="true"&&value!="false"){server.send(400,"text/plain","Use true or false");return;}patch[key]=value=="true";
    }
    if(server.hasArg("port")){uint32_t number;if(!PakarShell::unsignedNumber(server.arg("port").c_str(),65535,number)||!number){server.send(400,"text/plain","Invalid port");return;}patch["port"]=number;}
    String result;const bool ok=updateShellConfiguration(patch.as<JsonObjectConst>(),result);server.send(ok?200:409,"application/json",String("{\"accepted\":")+(ok?"true":"false")+",\"shell\":"+shellStatusJson()+"}");
  });
}

bool handleShellFrame(const BoardCore::Protocol::Frame &frame){
  if(frame.opcode!=kShellRequest)return false;
#if !PAKAR_SERIAL_SHELL_ENABLED
  hostLink.error(frame.sequence,frame.opcode,kUnsupported);return true;
#else
  const auto *p=frame.payload;const uint8_t n=frame.payloadLength;
  if(!n){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
  if(p[0]==PakarShell::SerialChannel::Open){
    uint8_t boot[16];if(n!=21||pcBootId.length()!=32){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
    for(unsigned i=0;i<16;++i){char hex[3]={pcBootId[i*2],pcBootId[i*2+1],0};boot[i]=strtoul(hex,nullptr,16);}
    if(memcmp(p+1,boot,16)){hostLink.error(frame.sequence,frame.opcode,kUnsafe);return true;}
    if(serialShell){hostLink.error(frame.sequence,frame.opcode,kBusy);return true;}
    if(shellTokenCounter==UINT32_MAX){hostLink.error(frame.sequence,frame.opcode,kUnsafe);return true;}
    auto *session=new(std::nothrow) DeviceShellSession;if(!session){hostLink.error(frame.sequence,frame.opcode,kBusy);return true;}
    serialShell.reset(session);++shellTokenCounter;shellChannel.open(shellTokenCounter);initializeDeviceShell(*session,false);
    uint8_t response[13]={0};memcpy(response+1,&shellTokenCounter,4);memcpy(response+9,p+17,4);
    hostLink.send(kShellResponse,frame.sequence,response,sizeof(response));return true;
  }
  if(n<5){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
  uint32_t token;memcpy(&token,p+1,4);
  if(!serialShell||token!=shellChannel.token()||!accessConfig.ready()||serialShell->accessRevision!=accessConfig.securityRevision()){hostLink.error(frame.sequence,frame.opcode,kUnsafe);return true;}
  auto &s=*serialShell;s.lastInput=millis();
  if(p[0]==PakarShell::SerialChannel::Input){
    if(n<8){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
    const uint16_t seq=p[5]|uint16_t(p[6])<<8;
    if(s.output.length()>1024U){hostLink.error(frame.sequence,frame.opcode,kBusy);return true;}
    const auto admission=shellChannel.accept(token,seq,p+7,n-7);
    if(admission!=PakarShell::SerialChannel::Accepted&&admission!=PakarShell::SerialChannel::Duplicate){hostLink.error(frame.sequence,frame.opcode,kUnsafe);return true;}
    if(admission==PakarShell::SerialChannel::Accepted)for(unsigned i=7;i<n;++i)inputDeviceShell(s,p[i]);
    if(s.tui.active()&&s.output.length()<4096U)shellTuiDraw(s);
    hostLink.ack(frame.sequence,frame.opcode);return true;
  }
  if(p[0]==PakarShell::SerialChannel::Close&&n==5){serialShell.reset();shellChannel.close();hostLink.ack(frame.sequence,frame.opcode);return true;}
  if(p[0]!=PakarShell::SerialChannel::Poll||n!=7){hostLink.error(frame.sequence,frame.opcode,kBadPayload);return true;}
  const uint16_t ack=p[5]|uint16_t(p[6])<<8;size_t consumed=0;
  if(!s.serialOutput.acknowledge(ack,consumed)){hostLink.error(frame.sequence,frame.opcode,kUnsafe);return true;}
  if(consumed)s.output.remove(0,consumed);
  if(s.tui.active()&&s.tui.escapePending()&&uint32_t(millis()-s.escapeAt)>350)shellTuiInput(s,0,true);
  uint8_t response[48]={3};memcpy(response+1,&token,4);const uint16_t next=shellChannel.next();memcpy(response+5,&next,2);
  if(!s.serialOutput.stage(reinterpret_cast<const uint8_t*>(s.output.c_str()),s.output.length())){s.fault=true;s.closing=true;}
  response[7]=uint8_t(s.closing)|(s.fault?2:0);
  const uint16_t outSequence=s.serialOutput.sequence();memcpy(response+8,&outSequence,2);
  const uint8_t count=s.serialOutput.size();memcpy(response+10,s.serialOutput.bytes(),count);
  hostLink.send(kShellResponse,frame.sequence,response,count+10); // Only a later explicit ACK retires bytes.
  return true;
#endif
}
