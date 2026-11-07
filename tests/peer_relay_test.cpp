#define ARDUINO_ARCH_ESP32
#include "../firmware/esp/src/PeerRelay.h"
#include <cassert>
#include <cstdio>

const String board="esp32s3-b81f3fbb0684",boot="1234567890abcdef1234567890abcdef",self="esp32s3-907069eaaf34";
unsigned nextId=1;
String id(){char value[33];snprintf(value,sizeof(value),"%032x",nextId++);return value;}
JsonDocument json(const String &raw){JsonDocument d;assert(!deserializeJson(d,raw));return d;}
String encode(const JsonDocument &d){String raw;serializeJson(d,raw);return raw;}
String envelope(const char *path="/api/peer/info",const char *hint="",bool command=false){
  JsonDocument d;d["request_id"]=id();d["hostname"]="keycontroller.local";d["board_id"]=board;
  d["boot_id"]=String(path)=="/api/peer/info"?"":boot;d["method"]=command?"POST":"GET";d["path"]=path;
  if(*hint)d["address"]=hint;
  if(command){JsonDocument p;p["type"]="command";p["request_id"]=d["request_id"];p["board_id"]=board;p["boot_id"]=boot;p["expires_at_ms"]=uint64_t(999999);d["packet"]=encode(p);}
  return encode(d);
}
String info(){JsonDocument d;d["application"]="PakarController";d["version"]="0.2.8";d["board_id"]=board;d["boot_id"]=boot;return encode(d);}
void add(const String &body,int code=200){TestReply reply;reply.body=body;reply.code=code;replies.push_back(reply);}
JsonDocument run(PeerRelay &relay,const String &raw){
  const unsigned released=stringNullReleases;auto receipt=json(relay.submit(raw,self));assert(receipt["accepted"]==true);
  assert(stringNullReleases==released+1&&receipt["response"].is<const char*>()&&receipt["response"]=="");
  runRelayWorker();assert(replies.empty());return json(relay.snapshot());
}
void clean(){requests.clear();replies.clear();mdnsCalls=0;mdnsAddress=0xc901a8c0;freeInternal=100000;largestInternal=40000;}
void stickyAndPersistent(){
  clean();fs::FS fs;PeerRelay relay;relay.begin(fs,true);const unsigned priorTasks=taskCreates;
  add(info());auto first=run(relay,envelope());assert(first["response_verified"]==true&&mdnsCalls==1);
  assert(mdnsLabel=="keycontroller"&&mdnsBudget==900&&requests.size()==1);
  assert(fs.writes==0); // Never silently persist discovery or observation.
  mdnsAddress=0;add("{\"board_id\":\"esp32s3-b81f3fbb0684\",\"boot_id\":\"1234567890abcdef1234567890abcdef\",\"large\":[1,2,3]}");
  auto second=run(relay,envelope("/api/status"));
  assert(second["response_verified"]==true&&second["route_source"]=="verified-cache");
  assert(mdnsCalls==1&&requests.size()==2&&requests.back().url=="http://keycontroller.local/api/status");
  assert(requests.back().address=="192.168.1.201");
  assert(requests.back().headers["Host"]=="keycontroller.local");
  assert(requests.back().headers["User-Agent"].find("PakarController/")==0);
  assert(taskCreates==priorTasks+1); // One worker, not one task allocation/read.
  const String save="{\"action\":\"save\",\"board_id\":\"esp32s3-b81f3fbb0684\",\"hostname\":\"keycontroller.local\"}";
  assert(json(relay.saveRoute(save))["accepted"]==true&&fs.writes==1);
  assert(json(relay.saveRoute(save))["accepted"]==true&&fs.writes==1);
  auto directory=json(relay.directory());assert(directory["routes"].size()==1&&directory["routes"][0]["saved"]==true);
  PeerRelay restored;restored.begin(fs,true);
  assert(json(restored.directory())["routes"][0]["verified_this_boot"]==false);
  add(info());add("{}");auto saved=run(restored,envelope("/api/status"));
  assert(saved["response_verified"]==true&&saved["route_source"]=="saved-route");
  assert(mdnsCalls==1); // Saved hint is reverified, not rediscovered/auto-trusted.
}
void hintsAndIdentity(){
  clean();PeerRelay relay;fs::FS fs;relay.begin(fs,true);mdnsAddress=0;
  add(info());auto hinted=run(relay,envelope("/api/peer/info","192.168.1.201"));
  assert(hinted["response_verified"]==true&&hinted["route_source"]=="address-hint"&&mdnsCalls==0);
  assert(json(relay.submit(envelope("/api/status","169.254.169.254"),self))["error"]=="invalid_peer_address");
  TestReply wrong;wrong.body=info();wrong.board="other-board";replies.push_back(wrong);
  auto failed=run(relay,envelope("/api/status"));assert(failed["response_verified"]==false&&requests.size()==2&&mdnsCalls==0);
  // A stale boot/header cannot promote another address or trigger discovery.
  assert(json(relay.saveRoute("{\"action\":\"save\",\"board_id\":\"missing\",\"hostname\":\"unknown.local\"}"))["error"]=="peer_route_not_verified");
  PeerRelay selfRelay;auto rejected=run(selfRelay,envelope("/api/peer/info","192.168.1.218"));
  assert(rejected["error"]=="peer_lan_resolution_failed");
}
void connectRecoveryAndNoReplay(){
  clean();PeerRelay relay;add(info());run(relay,envelope());
  const unsigned before=requests.size();mdnsAddress=0xca01a8c0; // Changed to .202.
  add("",-1);add(info());add("{}");auto recovered=run(relay,envelope("/api/status"));
  assert(recovered["response_verified"]==true&&recovered["address"]=="192.168.1.202");
  assert(requests.size()==before+3&&mdnsCalls==2);
  const String raw=envelope("/api/command","",true);add(info());add("",-1);
  auto uncertain=run(relay,raw);assert(uncertain["error"]=="command_outcome_uncertain_do_not_replay");
  assert(uncertain["command_dispatched"]==true&&requests.back().method=="POST");
  const unsigned sent=requests.size();auto repeated=json(relay.submit(raw,self));
  assert(repeated["error"]=="command_outcome_uncertain_do_not_replay"&&requests.size()==sent&&!relay.busy());
  auto changed=json(raw);changed["address"]="192.168.1.203";
  assert(json(relay.submit(encode(changed),self))["error"]=="request_id_conflict");
  // A stale/malformed receipt never becomes accepted execution.
  add(info());add("{\"type\":\"command-result\",\"accepted\":true}");
  auto malformed=run(relay,envelope("/api/command","",true));assert(malformed["error"]=="uncorrelated_peer_receipt");
}
void memoryAndExpiry(){
  clean();PeerRelay relay;freeInternal=69999;
  assert(json(relay.submit(envelope(),self))["error"]=="peer_relay_low_memory");
  freeInternal=100000;largestInternal=12000;
  assert(json(relay.submit(envelope(),self))["error"]=="peer_relay_low_memory");
  largestInternal=31732; // Actual incident's largest block is sufficient.
  for(unsigned i=0;i<8;++i){add(info());assert(run(relay,envelope())["response_verified"]==true);}
  assert(json(relay.submit(envelope(),self))["error"]=="peer_receipts_full");
  testNow+=60000;add(info());assert(run(relay,envelope())["response_verified"]==true);
}
void errorsAndLargeReceipt(){
  clean();PeerRelay relay;add(info());run(relay,envelope());
  JsonDocument status;status["board_id"]=board;status["boot_id"]=boot;
  status["payload"]=String(18000,'x')+"\"\\\n";const String large=encode(status);add(large);
  auto complete=run(relay,envelope("/api/status"));assert(complete["response"].as<String>()==large);
  auto usage=json(relay.directory());assert(usage["memory"]["internal_minimum_free_bytes"]==216);
  assert(usage["memory"]["internal_free_bytes"]==100000&&usage["memory"]["minimum_is_historical"]==true);
  const unsigned lookups=mdnsCalls,sent=requests.size();
  add("{}",401);auto auth=run(relay,envelope("/api/status"));
  assert(auth["error"]=="peer_authentication_required"&&mdnsCalls==lookups&&requests.size()==sent+1);
  TestReply stale;stale.body=info();stale.boot="abcdef1234567890abcdef1234567890";replies.push_back(stale);
  auto failed=run(relay,envelope("/api/command","",true));
  assert(failed["response_verified"]==false&&failed["command_dispatched"]==false&&requests.back().method=="GET");
  assert(mdnsCalls==lookups); // No stale-boot retargeting or POST.
  add("",-11);auto timeout=run(relay,envelope("/api/status"));
  assert(timeout["response_verified"]==false&&mdnsCalls==lookups); // HTTP timeout isn't failed DNS.
}
void resolverLateCallback(){
  PeerLanResolver resolver;IPAddress ip;dnsDeferred=true;const auto start=millis();
  assert(!resolver.lookup("keycontroller",ip)&&uint32_t(millis()-start)==900);
  assert(!resolver.lookup("another-host",ip)&&dnsHost=="keycontroller"); // No overwrite/UAF.
  ip_addr_t late{};dnsCallback("keycontroller",&late,dnsArg);dnsDeferred=false;
  assert(resolver.lookup("another-host",ip)&&dnsHost=="another-host"&&ip.toString()=="192.168.1.201");
}
void changedHintDuringDnsOutage(){
  clean();PeerRelay relay;add(info());run(relay,envelope());mdnsAddress=0;
  add("",-1);add(info());add("{}");
  auto next=run(relay,envelope("/api/status","192.168.1.203"));
  assert(next["response_verified"]==true&&next["route_source"]=="address-hint"&&next["address"]=="192.168.1.203");
  assert(requests[requests.size()-2].url=="http://keycontroller.local/api/peer/info");
  assert(requests.back().address=="192.168.1.203"&&requests.back().method=="GET");
}
void stackFragmentationDoesNotLockOutReads(){
  clean();largestInternal=31732;simulateStackAllocation=true;
  PeerRelay relay;add(info());assert(run(relay,envelope())["response_verified"]==true);
  assert(largestInternal==21492);add("{}");assert(run(relay,envelope("/api/status"))["response_verified"]==true);
  largestInternal=4095;assert(json(relay.submit(envelope(),self))["error"]=="peer_relay_low_memory");
  simulateStackAllocation=false;
}
void persistenceFaults(){
  fs::FS fs;PeerRouteStore store;store.begin(fs,true);
  PeerRouteStore::Route route{"keycontroller.local",board,"192.168.1.201"};
  assert(store.update(route,false)==nullptr);
  const String saved=*fs.files["/peer-routes.json"];fs.failRename=true;route.address="192.168.1.202";
  assert(String(store.update(route,false))=="route_publish_failed");
  assert(*fs.files["/peer-routes.json"]==saved&&store.at(0).address=="192.168.1.201");
  fs.failRename=false;assert(store.update(route,false)==nullptr);
  PeerRouteStore restored;restored.begin(fs,true);assert(restored.at(0).address==route.address);
  *fs.files["/peer-routes.json"]="{\"routes\":[{\"hostname\":\"keycontroller.local\",\"board_id\":\"peer\",\"address\":\"8.8.8.8\"}]}";
  PeerRouteStore corrupt;corrupt.begin(fs,true);assert(corrupt.at(0).board.isEmpty());
  PeerRouteStore unavailable;unavailable.begin(fs,false);assert(String(unavailable.update(route,false))=="volume_unavailable");
}
void metadataIsNotControlAdmission(){
  clean();PeerRelay relay;auto old=json(info());old["version"]="0.1.0";add(encode(old));
  assert(run(relay,envelope())["response_verified"]==true);
  assert(json(relay.directory())["routes"].size()==0);
  add(encode(old));auto denied=run(relay,envelope("/api/status"));assert(denied["response_verified"]==false);
  assert(denied["error"]=="peer_identity_not_verified"&&requests.back().url=="http://keycontroller.local/api/peer/info");
}
int main(){stickyAndPersistent();hintsAndIdentity();connectRecoveryAndNoReplay();memoryAndExpiry();errorsAndLargeReceipt();resolverLateCallback();changedHintDuringDnsOutage();stackFragmentationDoesNotLockOutReads();persistenceFaults();metadataIsNotControlAdmission();puts("Actual ESP peer relay/cache/route-store/DNS fixtures passed; not LAN or physical acceptance");}
