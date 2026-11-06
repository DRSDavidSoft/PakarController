#include "../firmware/esp/src/RemoteFirmware.h"
#include <Update.h>
#include <cassert>
#include <cstdio>
#include "../firmware/esp/src/PeerArtifact.h"
uint32_t testNow=1000;
fs::FS LittleFS;TestEsp ESP;TestUpdater Update;
std::map<std::string,TestResponse>testResponses;
std::vector<TestRequest>testRequests;
const char *pakarTrustAnchorPem(){return "fixture trust anchor";}
unsigned prepares=0,programs=0,releases=0;
const RemoteFirmware *observedJob=nullptr;
unsigned verifyProgressCalls=0;uint32_t lastVerifiedBytes=0;
bool allowReboot=false;
bool rebootReady(){return allowReboot;}
void observeProgress(){
  assert(observedJob);
  if(!strcmp(observedJob->progressStage(),"verify_esp")) {
    const auto p=observedJob->progress();assert(p.known() && p.received>lastVerifiedBytes);
    lastVerifiedBytes=p.received;++verifyProgressCalls;
  }
}
bool prepare(const RemoteFirmware::Plan&p,String&){assert(p.targets==1);++prepares;return true;}
bool program(const char*,const String&,String&){++programs;return false;}
int8_t finish(const RemoteFirmware::Plan&p,bool commit,String&){assert(p.targets==1);if(commit)++releases;return 1;}
String hash(const std::vector<uint8_t>&v){FirmwarePlatform::Sha256 c;c.add(v.data(),v.size());return c.hex();}
std::vector<uint8_t>bytes(const String&s){return {s.begin(),s.end()};}
struct Fixture {
 String url="https://updates.example.test/PakarController/manifest.json",imageUrl="https://updates.example.test/PakarController/esp.bin",sha;
 std::vector<uint8_t>image=std::vector<uint8_t>(1843552,0);
 Fixture(){
   OtaFixture::reset();LittleFS=fs::FS{};ESP=TestEsp{};Update=TestUpdater{};
   testResponses.clear();testRequests.clear();prepares=programs=releases=0;testNow=1000;
   image[0]=0xe9;image[1]=5;image[12]=9;image[23]=1;
   JsonDocument d;d["product"]="PakarController";d["profile"]="pakar-esp32s3-atmega328p";d["link_mode"]="buffered";
   auto e=d["esp"].to<JsonObject>();e["url"]=imageUrl;e["size"]=image.size();e["sha256"]=hash(image);e["version"]="0.2.8";e["model"]="esp32s3";
   String text;serializeJson(d,text);sha=hash(bytes(text));testResponses[url].bytes=bytes(text);testResponses[imageUrl].bytes=image;
 }
 void queue(RemoteFirmware&j){j.begin("boot-A",prepare,program,finish);assert(j.queue(url,sha,1,false,false,"secret-fixture"));}
 static void pump(RemoteFirmware&j,const char*phase){
   for(unsigned i=0;i<5000 && strcmp(j.phase(),phase) && j.busy();++i){++testNow;j.service();}
   assert(!strcmp(j.phase(),phase));
 }
 static void apply(RemoteFirmware&j,const String&boot="boot-A"){
   JsonDocument d;assert(!deserializeJson(d,j.json()));assert(j.apply(d["job_id"],boot,d["manifest_sha256"].as<String>()));
 }
};
int main(){
 {Fixture f;RemoteFirmware j;f.queue(j);testResponses[f.url].status=-1;j.service();
  assert(!strcmp(j.phase(),"failed") && j.error()=="Download HTTP status -1; TLS error 0");
  assert(j.error().find("secret-fixture")==String::npos && OtaFixture::selections==0);
 }
 {Fixture f;{
  RemoteFirmware j;f.queue(j);Fixture::pump(j,"ready");
  assert(j.cancel(1,"boot-A"));assert(j.queue(f.url,f.sha,1,false,false,"secret-fixture"));
  Fixture::pump(j,"ready");
 }assert(WiFiClient::live.empty());} // manifest→image, cancel→new job, HTTP-before-socket destruction
 {Fixture f;RemoteFirmware j;f.queue(j);Fixture::pump(j,"ready");
  assert(f.image.size()>LittleFS.totalBytes());assert(!LittleFS.files.count("/remote/esp.bin"));
  assert(OtaFixture::ends==1 && OtaFixture::selections==0 && OtaFixture::running==0 && prepares==0);
  assert(j.ownsEspStage() && j.stagedEspSize()==f.image.size());
  JsonDocument ready;assert(!deserializeJson(ready,j.json()) && ready["cancellable"]==true);
  uint8_t sample[24];assert(j.readStagedEsp(0,sample,sizeof(sample))&&sample[0]==0xe9);
  assert(!j.readStagedEsp(f.image.size()-1,sample,2));
  assert(j.json().find("secret-fixture")==String::npos);
  Fixture::apply(j);Fixture::pump(j,"awaiting_reboot");
  assert(OtaFixture::selections==1 && programs==0 && Update.commits==0 && prepares==1);
  j.setRebootReady(rebootReady);allowReboot=false;testNow+=2001;j.service();assert(ESP.restarts==0);
  allowReboot=true;j.service();assert(ESP.restarts==1);
  OtaFixture::reboot();RemoteFirmware after;observedJob=&after;verifyProgressCalls=lastVerifiedBytes=0;
  after.setServiceCallback(observeProgress);after.begin("boot-B",prepare,program,finish);
  assert(verifyProgressCalls==(f.image.size()+511)/512 && lastVerifiedBytes==f.image.size());
  assert(!after.progress().known() && !strcmp(after.progressStage(),"verify_pair"));
  assert(!strcmp(after.phase(),"verify_pair"));after.service();assert(!strcmp(after.phase(),"complete")&&releases==1);
  assert(after.completionVerified());
  RemoteFirmware historical;historical.begin("boot-C",prepare,program,finish);
  assert(!strcmp(historical.phase(),"complete") && !historical.completionVerified());
  observedJob=nullptr;
 }
 {Fixture f;RemoteFirmware j;f.queue(j);Fixture::pump(j,"ready");
  RemoteFirmware after;after.begin("boot-B",prepare,program,finish);assert(!strcmp(after.phase(),"ready"));
  assert(!after.apply(1,"boot-A",f.sha));Fixture::apply(after,"boot-B");Fixture::pump(after,"awaiting_reboot");assert(programs==0);
 }
 for(unsigned failure=0;failure<8;++failure){
  Fixture f;RemoteFirmware j;f.queue(j);
  if(failure==0)OtaFixture::failBegin=true;
  if(failure==1)OtaFixture::failWrite=true;
  if(failure==2)OtaFixture::failEnd=true;
  if(failure==3)testResponses[f.imageUrl].bytes[100]^=1;
  if(failure==4)testResponses[f.imageUrl].length=10;
  if(failure==5)testResponses[f.imageUrl].status=302;
  if(failure==6)testResponses[f.imageUrl].transfer="chunked";
  if(failure==7)testResponses[f.imageUrl].encoding="gzip";
  Fixture::pump(j,"failed");assert(OtaFixture::selections==0 && prepares==0 && programs==0 && !OtaFixture::active);
  if(failure==0)assert(j.error()=="Download inactive OTA stage could not begin");
  if(failure==4)assert(j.error()=="Download Content-Length missing or mismatched");
  if(failure==5)assert(j.error()=="Download HTTP status 302");
  if(failure==6)assert(j.error()=="Download Transfer-Encoding is not supported");
  if(failure==7)assert(j.error()=="Download Content-Encoding is not identity");
 }
 {Fixture f;RemoteFirmware j;f.queue(j);Fixture::pump(j,"ready");
  OtaFixture::images[0x340000][100]^=1;Fixture::apply(j);Fixture::pump(j,"failed");assert(OtaFixture::selections==0&&prepares==0);
 }
 {Fixture f;RemoteFirmware j;f.queue(j);Fixture::pump(j,"ready");
  OtaFixture::failSelect=true;Fixture::apply(j);Fixture::pump(j,"failed");assert(OtaFixture::selections==0&&programs==0);
 }
 {Fixture f;RemoteFirmware j;f.queue(j);Fixture::pump(j,"ready");
  LittleFS.failRename=true;JsonDocument d;deserializeJson(d,j.json());assert(!j.apply(d["job_id"],"boot-A",f.sha));assert(OtaFixture::selections==0);
 }
 {Fixture f;RemoteFirmware j;f.queue(j);j.service();assert(OtaFixture::begins==0);j.service();assert(OtaFixture::active);
  assert(j.cancel(1,"boot-A"));assert(!OtaFixture::active&&OtaFixture::aborts==1&&OtaFixture::selections==0);
 }
 {Fixture f;RemoteFirmware j;f.queue(j);Fixture::pump(j,"ready");Fixture::apply(j);
  RemoteFirmware after;after.begin("boot-B",prepare,program,finish);assert(!strcmp(after.phase(),"failed")&&OtaFixture::selections==0);
 }
 {Fixture f;RemoteFirmware j;f.queue(j);Fixture::pump(j,"ready");Fixture::apply(j);Fixture::pump(j,"awaiting_reboot");
  OtaFixture::reboot();OtaFixture::images[0x340000][100]^=1;RemoteFirmware after;after.begin("boot-B",prepare,program,finish);
  assert(!strcmp(after.phase(),"failed")&&releases==0);
 }
 {OtaFixture::reset();S3OtaStage stage;uint8_t v[24]{};
  assert(!stage.begin(0x330000));assert(!stage.begin(0));assert(stage.begin(24));assert(!stage.write(v,25));
  assert(!stage.finish());assert(OtaFixture::aborts==1&&OtaFixture::selections==0);
  OtaFixture::boot=1;assert(!stage.begin(24));OtaFixture::boot=0;
  assert(!stage.resume(0x10000,24));assert(!stage.selectForBoot());
 }
 {Fixture f;RemoteFirmware j;j.begin("boot-A",prepare,program,finish);
  const String url="http://source.local/firmware/download",boot="0123456789abcdef0123456789abcdef";
  const char *marker=PakarPeerArtifact::family();memcpy(f.image.data()+1020,marker,strlen(marker)); // crosses chunk boundary
  JsonDocument source;source["hostname"]="source.local";source["address"]="192.168.1.21";
  source["board_id"]="source-board";source["boot_id"]=boot;
  auto a=source["artifact"].to<JsonObject>();a["family"]=PakarPeerArtifact::key();a["version"]="0.2.9";
  a["path"]="/firmware/download";a["bytes"]=uint32_t(f.image.size());a["sha256"]=hash(f.image);
  testResponses[url].bytes=f.image;testResponses[url].board="source-board";testResponses[url].boot=boot;
  assert(j.stagePeer(source.as<JsonObjectConst>(),"Basic source-only-secret"));Fixture::pump(j,"ready");
  assert(OtaFixture::selections==0&&prepares==0&&testRequests.back().authorization=="Basic source-only-secret");
  RemoteFirmware resumed;resumed.begin("boot-B",prepare,program,finish);assert(!strcmp(resumed.phase(),"ready"));
  assert(j.cancel(1,"boot-A"));a["family"]="different-board-role";assert(!j.stagePeer(source.as<JsonObjectConst>(),""));
  a["family"]=PakarPeerArtifact::key();a["version"]="0.2.8+other-build";assert(!j.stagePeer(source.as<JsonObjectConst>(),""));
  a["version"]="0.2.9";source["address"]="8.8.8.8";assert(!j.stagePeer(source.as<JsonObjectConst>(),""));
  source["address"]="192.168.1.21";testResponses[url].boot="changed-boot";
  assert(j.stagePeer(source.as<JsonObjectConst>(),""));Fixture::pump(j,"failed");assert(OtaFixture::selections==0);
 }
 puts("Actual S3 RemoteFirmware + inactive-slot adapter: image larger than LittleFS, explicit apply, persisted-stage resume, SHA/HTTP/header failures, peer family/boot/address validation and no automatic apply passed. SDK/HTTP/flash doubles, not physical OTA or Internet proof.");
}
