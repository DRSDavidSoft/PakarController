#pragma once
#if defined(ARDUINO_ARCH_ESP32)
#include <HTTPClient.h>
#include <WiFi.h>
#include <ArduinoJson.h>
#include <PeerRoutePolicy.h>
#include <PeerCompatibility.h>
#include <BuildInfo.h>
#include <mbedtls/sha256.h>
#include <esp_heap_caps.h>
#include "PeerLanResolver.h"
#include "PeerRouteStore.h"
#include <mutex>
#include <atomic>

// One bounded outgoing job, never network I/O on the game/HTTP owner. Explicit
// target board+boot fencing, no redirects/retries/credential forwarding. The
// target retains ownership of command expiry, deduplication and actual ACKs.
class PeerRelay {
 public:
  bool busy()const{return busy_.load();}
  void begin(fs::FS &fs,bool mounted){routes_.begin(fs,mounted);}
  String directory() {
    std::lock_guard<std::mutex> lock(mutex_);JsonDocument d;
    d["transports"].to<JsonArray>().add("http-lan");d["max_hops"]=1;d["explicit_target"]=true;
    d["automatic_retry"]=false;d["discovery"]=false;d["event_subscription"]=false;
    d["firmware_transfer"]=false;d["file_transfer"]=false;d["persistent_routes"]=true;
    d["verified_address_cache"]=true;d["resolution_budget_ms"]=900;d["request_budget_ms"]=6000;
    auto memory=d["memory"].to<JsonObject>();
    memory["internal_free_bytes"]=heap_caps_get_free_size(MALLOC_CAP_INTERNAL|MALLOC_CAP_8BIT);
    memory["internal_largest_block_bytes"]=heap_caps_get_largest_free_block(MALLOC_CAP_INTERNAL|MALLOC_CAP_8BIT);
    memory["internal_minimum_free_bytes"]=heap_caps_get_minimum_free_size(MALLOC_CAP_INTERNAL|MALLOC_CAP_8BIT);
    memory["minimum_is_historical"]=true;memory["worker_started"]=worker_!=nullptr;
    auto rows=d["routes"].to<JsonArray>();
    for(unsigned i=0;i<PeerRouteStore::capacity;++i){
      const auto &saved=routes_.at(i);const auto &seen=cache_[i];
      if(saved.board.length())appendRoute(rows,saved,findCache(saved.board,saved.hostname));
      if(seen.route.board.length()){
        bool exists=false;for(unsigned j=0;j<PeerRouteStore::capacity;++j)
          if(routes_.at(j).board==seen.route.board&&routes_.at(j).hostname==seen.route.hostname)exists=true;
        if(!exists)appendRoute(rows,seen.route,&seen);
      }
    }
    String body;serializeJson(d,body);return body;
  }
  String saveRoute(const String &raw) {
    std::lock_guard<std::mutex> lock(mutex_);
    if(busy())return error("peer_relay_busy");
    if(raw.length()>512)return error("request_too_large");
    JsonDocument d;if(deserializeJson(d,raw)||!d.is<JsonObject>())return error("invalid_request");
    const String action=d["action"]|"",board=d["board_id"]|"",host=d["hostname"]|"";
    PeerRouteStore::Route selected;const auto *seen=findCache(board,host);
    if(action=="save"){
      if(!seen||!seen->verified)return error("peer_route_not_verified");
      selected=seen->route;
    }else if(action=="remove"){
      for(unsigned i=0;i<PeerRouteStore::capacity;++i)if(routes_.at(i).board==board&&routes_.at(i).hostname==host)selected=routes_.at(i);
      if(selected.board.isEmpty())return error("peer_route_not_found");
    }else return error("unsupported_peer_route_action");
    const char *failure=routes_.update(selected,action=="remove");if(failure)return error(failure);
    if(action=="remove")for(auto &entry:cache_)if(entry.route.board==board&&entry.route.hostname==host)entry={};
    return F("{\"accepted\":true}");
  }
  String submit(const String &raw,const String &self) {
    using namespace BoardCore::PeerRoutePolicy;
    std::lock_guard<std::mutex> lock(mutex_);
    if(raw.length()>4096)return error("request_too_large");
    JsonDocument d;if(deserializeJson(d,raw)||!d.is<JsonObject>())return error("invalid_request");
    const String id=d["request_id"]|"",host=d["hostname"]|"",board=d["board_id"]|"",boot=d["boot_id"]|"",path=d["path"]|"",method=d["method"]|"",hint=d["address"]|"";
    const bool bootstrap=method=="GET"&&path=="/api/peer/info"&&boot.isEmpty();
    if(!hexId(id.c_str())||!hostname(host.c_str())||!boardId(board.c_str())||(!bootstrap&&!hexId(boot.c_str()))||board==self)return error("invalid_peer_identity");
    const bool command=method=="POST"&&path=="/api/command";
    if(!(method=="GET"&&readPath(path.c_str()))&&!command)return error("unsupported_peer_route");
    if(hint.length()&&!privateIpv4(hint.c_str()))return error("invalid_peer_address");
    const String packet=d["packet"]|"";
    if(command) {
      JsonDocument p;
      if(packet.length()>3072||deserializeJson(p,packet)||p["type"]!="command"||p["request_id"]!=id||p["board_id"]!=board||p["boot_id"]!=boot||!p["expires_at_ms"].is<uint64_t>())return error("invalid_command_envelope");
    } else if(packet.length())return error("unexpected_packet");
    uint8_t digest[32];if(mbedtls_sha256(reinterpret_cast<const uint8_t*>(raw.c_str()),raw.length(),digest,0))return error("request_hash_failed");
    for(const auto &old:history_)if(old.id==id&&uint32_t(millis()-old.at)<60000U) {
      if(memcmp(old.digest,digest,sizeof(digest)))return error("request_id_conflict");
      if(id==id_)return snapshotLocked();
      return error("receipt_retired_do_not_replay");
    }
    if(busy())return error("peer_relay_busy");
    History *slot=nullptr;for(auto &h:history_)if(!h.id.length()||uint32_t(millis()-h.at)>=60000U){slot=&h;break;}
    if(!slot)return error("peer_receipts_full");
    // Free internal RAM and its largest block, not PSRAM/global historical
    // minimum, determine whether HTTP response + worker can be admitted now.
    // The first stack allocation can split a 31KB block into ~21KB. Requiring
    // the *maximum* 24KB response block forever would then lock out ordinary
    // 10KB status reads. Admission reserves minimum working space; the actual
    // Content-Length allocation is checked before reading any response bytes.
    const size_t minimumBlock=worker_?4096:12288;
    if(heap_caps_get_free_size(MALLOC_CAP_INTERNAL|MALLOC_CAP_8BIT)+response_.length()<70000||
       heap_caps_get_largest_free_block(MALLOC_CAP_INTERNAL|MALLOC_CAP_8BIT)<minimumBlock)return error("peer_relay_low_memory");
    if(!worker_&&xTaskCreate(worker,"pakar-peer",10240,this,1,&worker_)!=pdPASS)return error("worker_unavailable");
    slot->id=id;memcpy(slot->digest,digest,sizeof(digest));slot->at=millis();
    id_=id;host_=host;board_=board;boot_=boot;path_=path;packet_=packet;self_=self;hint_=hint;
    // Arduino String's empty-string assignment retains its large allocation.
    // A null assignment uses its checked invalidate path, releasing the retired
    // receipt buffer we credited above before starting another network read.
    command_=command;response_=static_cast<const char*>(nullptr);failure_="";code_=0;phase_="queued";verified_=false;started_=millis();
    elapsed_=resolutionMs_=httpMs_=0;address_="";routeSource_="";commandDispatched_=false;
    busy_=true;
    xTaskNotifyGive(worker_);
    return snapshotLocked();
  }
  String snapshot(const String &id=String()) {
    std::lock_guard<std::mutex> lock(mutex_);
    if(id.length()&&id!=id_)return error("unknown_peer_request");
    return snapshotLocked();
  }
 private:
  struct History {String id;uint8_t digest[32]{};uint32_t at=0;} history_[8];
  struct Cached {PeerRouteStore::Route route;String boot;uint32_t at=0;bool verified=false;} cache_[PeerRouteStore::capacity];
  PeerRouteStore routes_;PeerLanResolver resolver_;TaskHandle_t worker_=nullptr;
  std::mutex mutex_;std::atomic<bool> busy_{false};
  String id_,host_,board_,boot_,path_,packet_,self_,response_,failure_,phase_="idle",hint_,address_,routeSource_;
  bool command_=false,verified_=false;int code_=0;uint32_t started_=0;
  bool commandDispatched_=false;uint32_t elapsed_=0,resolutionMs_=0,httpMs_=0;
  Cached *findCache(const String &board,const String &host){for(auto &r:cache_)if(r.route.board==board&&r.route.hostname==host)return &r;return nullptr;}
  void appendRoute(JsonArray rows,const PeerRouteStore::Route &route,const Cached *seen){
    auto o=rows.add<JsonObject>();o["board_id"]=route.board;o["hostname"]=route.hostname;
    o["address"]=seen?seen->route.address:route.address;
    o["saved"]=routes_.saved(route.board,route.hostname,route.address);
    o["verified_this_boot"]=seen&&seen->verified;
    if(seen&&seen->verified){o["boot_id"]=seen->boot;o["observation_age_ms"]=uint32_t(millis()-seen->at);}
    else{o["boot_id"]=nullptr;o["observation_age_ms"]=nullptr;}
  }
  void remember(const IPAddress &ip,const String &boot){
    std::lock_guard<std::mutex> lock(mutex_);Cached *slot=findCache(board_,host_);
    if(!slot)for(auto &r:cache_)if(r.route.board.isEmpty()){slot=&r;break;}
    if(!slot){slot=&cache_[0];for(auto &r:cache_)if(uint32_t(millis()-r.at)>uint32_t(millis()-slot->at))slot=&r;}
    *slot={{host_,board_,ip.toString()},boot,millis(),true};
  }
  static String error(const char *e){JsonDocument d;d["accepted"]=false;d["error"]=e;String s;serializeJson(d,s);return s;}
  String snapshotLocked() {
    JsonDocument d;d["request_id"]=id_;d["phase"]=phase_;d["busy"]=busy();d["target_board_id"]=board_;d["target_boot_id"]=boot_;
    d["hostname"]=host_;d["via_board_id"]=self_;d["transport"]="http-lan";d["hop_count"]=1;d["automatic_retry"]=false;
    d["response_verified"]=verified_;d["http_status"]=code_;d["error"]=failure_;
    // Serialization finishes under mutex_: link the immutable response rather
    // than allocating a second full status string in ArduinoJson's pool.
    d["response"]=JsonString(response_.length()?response_.c_str():"",response_.length(),true);
    d["address"]=address_;d["route_source"]=routeSource_;d["elapsed_ms"]=busy()?uint32_t(millis()-started_):elapsed_;
    d["resolution_ms"]=resolutionMs_;d["http_ms"]=httpMs_;d["command_dispatched"]=commandDispatched_;
    d["accepted"]=id_.length()>0&&failure_.isEmpty();String s;const size_t bytes=measureJson(d);
    if(!s.reserve(bytes)||serializeJson(d,s)!=bytes||s.length()!=bytes)return error("peer_receipt_memory_unavailable");
    return s;
  }
  static void worker(void *p){auto &s=*static_cast<PeerRelay*>(p);for(;;){ulTaskNotifyTake(pdTRUE,portMAX_DELAY);s.run();}}
  class PinnedClient final:public NetworkClient {
   public:
    explicit PinnedClient(const IPAddress &ip):ip_(ip){}
    int connect(const char *,uint16_t port)override{return NetworkClient::connect(ip_,port,700);}
    int connect(const char *,uint16_t port,int32_t timeout)override{return NetworkClient::connect(ip_,port,timeout);}
   private:IPAddress ip_;
  };
  String request(const IPAddress &ip,const char *path,bool post,int &code,bool &fenced) {
    PinnedClient client(ip);HTTPClient http;String result;
    code=0;fenced=false;
    http.setConnectTimeout(700);http.setTimeout(1000);http.setFollowRedirects(HTTPC_DISABLE_FOLLOW_REDIRECTS);http.setReuse(false);
    const String url=String("http://")+host_+path;
    if(!http.begin(client,url))return String();
    // SDK HTTPClient ignores addHeader(Host/User-Agent). Retain its logical
    // URL host and override ONLY the dial, then use its dedicated UA setter.
    http.setUserAgent(String("PakarController/")+BoardCore::BuildInfo::kVersion+" (peer-relay)");
    http.addHeader("X-Pakar-Version",BoardCore::BuildInfo::kVersion);
    http.addHeader("X-Pakar-Board",board_);if(boot_.length())http.addHeader("X-Pakar-Boot",boot_);
    http.addHeader("X-Pakar-Via",self_); // Metadata only, never downstream authority.
    const char *headers[]={"X-Pakar-Board","X-Pakar-Boot"};http.collectHeaders(headers,2);
    if(post) {
      String form;if(!form.reserve(7+packet_.length()*3)){http.end();return String();}
      form="packet=";const char hex[]="0123456789ABCDEF";
      for(size_t i=0;i<packet_.length();++i){uint8_t c=packet_[i];form+='%';form+=hex[c>>4];form+=hex[c&15];}
      http.addHeader("Content-Type","application/x-www-form-urlencoded");
      {std::lock_guard<std::mutex> lock(mutex_);commandDispatched_=true;}
      code=http.POST(form);
    }else code=http.GET();
    fenced=http.header("X-Pakar-Board")==board_&&(boot_.length()?http.header("X-Pakar-Boot")==boot_:BoardCore::PeerRoutePolicy::hexId(http.header("X-Pakar-Boot").c_str()));
    const int size=http.getSize();
    if(code==200&&fenced&&size>0&&size<=24576) {
      if(!result.reserve(size)){http.end();return String();}auto *stream=http.getStreamPtr();uint8_t block[512];
      while(result.length()<static_cast<size_t>(size)&&uint32_t(millis()-started_)<6000U) {
        const size_t available=stream->available();
        if(!available){if(!http.connected())break;vTaskDelay(1);continue;}
        const size_t count=min<size_t>(sizeof(block),min<size_t>(available,size-result.length()));
        const int n=stream->read(block,count);if(n<=0)break;
        result.concat(reinterpret_cast<const char*>(block),n);
      }
      if(result.length()!=static_cast<size_t>(size))result="";
    }
    http.end();return result;
  }
  void run() {
    IPAddress ip;String errorText,body;bool fenced=false,verified=false;int code=0;
    String source,cached,cachedBoot;uint32_t resolution=0,httpTime=0;
    {
      std::lock_guard<std::mutex> lock(mutex_);
      if(const auto *entry=findCache(board_,host_)){cached=entry->route.address;cachedBoot=entry->boot;source="verified-cache";}
      if(cached.isEmpty())for(unsigned i=0;i<PeerRouteStore::capacity;++i){const auto &r=routes_.at(i);
        if(r.board==board_&&r.hostname==host_){cached=r.address;source="saved-route";break;}}
      if(cached.isEmpty()&&hint_.length()){cached=hint_;source="address-hint";}
    }
    auto allowed=[&](const IPAddress &address){return address.type()==IPv4&&
      BoardCore::PeerRoutePolicy::privateAddress(address[0],address[1],address[2],address[3])&&
      address!=WiFi.localIP()&&address!=WiFi.softAPIP();};
    auto fetch=[&](const char *path,bool post){
      const uint32_t at=millis();String value=request(ip,path,post,code,fenced);httpTime+=uint32_t(millis()-at);return value;
    };
    auto resolve=[&](){
      const uint32_t at=millis();const bool ok=resolver_.lookup(host_,ip);resolution+=uint32_t(millis()-at);
      source="hostname";return ok&&allowed(ip);
    };
    // Filter only the identity/receipt envelope, not a second in-RAM copy of
    // an entire telemetry/status tree. JSON syntax and correlation still count.
    JsonDocument filter;for(const char *key:{"application","board_id","boot_id","version","type","request_id","accepted"})filter[key]=true;
    JsonDocument reply;
    bool candidate=cached.length()&&ip.fromString(cached)&&allowed(ip);
    if(!candidate){cachedBoot="";candidate=resolve();}
    if(!candidate)errorText="peer_lan_resolution_failed";
    else {
      const bool directRead=!command_&&path_!="/api/peer/info"&&cachedBoot==boot_&&boot_.length();
      body=fetch(directRead?path_.c_str():"/api/peer/info",false);
      // Only a failed READ connect may try hostname recovery. Never switch
      // destinations on HTTP rejection, identity mismatch or after a POST.
      if((code==HTTPC_ERROR_CONNECTION_REFUSED||code==HTTPC_ERROR_NOT_CONNECTED||code==HTTPC_ERROR_CONNECTION_LOST)&&
          cached.length()&&uint32_t(millis()-started_)<3500U){
        bool found=resolve();
        // A caller may have observed a new DHCP address while mDNS is down.
        // It is only an identity-probe candidate, never POST replay authority.
        if(!found&&hint_.length()&&hint_!=cached&&ip.fromString(hint_)&&allowed(ip)){
          found=true;source="address-hint";
        }
        if(found)body=fetch("/api/peer/info",false);
        else errorText="peer_lan_resolution_failed";
      }
      const bool identityRead=!directRead||source!="verified-cache";
      if(errorText.isEmpty()){
        const bool parsed=!deserializeJson(reply,body,DeserializationOption::Filter(filter))&&reply.is<JsonObject>();
        if(!fenced||!parsed)errorText=code==401||code==403?"peer_authentication_required":"peer_response_unavailable";
        else if(identityRead&&(reply["application"]!="PakarController"||reply["board_id"]!=board_||
            !BoardCore::PeerRoutePolicy::hexId(reply["boot_id"]|"")||(boot_.length()&&reply["boot_id"]!=boot_)||
            (path_!="/api/peer/info"&&BoardCore::PeerCompatibility::enforced&&!BoardCore::PeerCompatibility::boardAllowed(reply["version"]|""))))errorText="peer_identity_not_verified";
        else if(!identityRead){verified=true;remember(ip,boot_);}
        else {
          // Metadata visibility does not grant a supported control route.
          if(!BoardCore::PeerCompatibility::enforced||BoardCore::PeerCompatibility::boardAllowed(reply["version"]|""))remember(ip,reply["boot_id"].as<String>());
          if(path_=="/api/peer/info")verified=true;
          else if(uint32_t(millis()-started_)>=4000U)errorText="peer_deadline_elapsed";
          else {
            body=fetch(path_.c_str(),command_);
            if(!fenced||body.isEmpty()||deserializeJson(reply,body,DeserializationOption::Filter(filter))||!reply.is<JsonObject>())
              errorText=command_?"command_outcome_uncertain_do_not_replay":"peer_response_unavailable";
            else if(command_&&(reply["type"]!="command-result"||reply["request_id"]!=id_||reply["board_id"]!=board_||
                reply["boot_id"]!=boot_||!reply["accepted"].is<bool>()))errorText="uncorrelated_peer_receipt";
            else verified=true; // Transport receipt, not physical execution.
          }
        }
      }
    }
    std::lock_guard<std::mutex> lock(mutex_);
    if(verified)response_=std::move(body);else response_=static_cast<const char*>(nullptr);
    packet_=static_cast<const char*>(nullptr);
    failure_=errorText;code_=code;verified_=verified;phase_=verified?"complete":"failed";
    elapsed_=uint32_t(millis()-started_);resolutionMs_=resolution;httpMs_=httpTime;address_=ip.toString();routeSource_=source;
    busy_=false;
  }
};
#endif
