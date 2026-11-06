#pragma once
#if defined(ARDUINO_ARCH_ESP32)
#include <HTTPClient.h>
#include <WiFi.h>
#include <mdns.h>
#include <ArduinoJson.h>
#include <PeerCatalogPolicy.h>
#include <BuildInfo.h>
#include <mutex>
#include <atomic>
#include <functional>
#include <LittleFS.h>
#include "PeerArtifact.h"

// Independent of an attached app/AVR. Reuse PeerRelay's bounded, numeric-dial /
// logical-Host pattern (adapted from PCController lanresolver). Advertisements
// are hints: only a bounded public identity read populates this catalogue.
// No discovery result grants a control session or changes a network/scene.
class PeerCatalogue {
 public:
  void begin(const String &self) {self_=self;
    File f=LittleFS.open("/peer-exchange.json","r");JsonDocument d;
    if(f&&f.size()<=256&&!deserializeJson(d,f)){
      automatic_=d["automatic_check"]|true;acceptOffers_=d["accept_offers"]|true;sharing_=d["sharing"]|true;
    }
    if(f)f.close();
  }
  void setLocalInfo(std::function<String()> read){localInfo_=std::move(read);}
  bool automaticChecks()const{return automatic_;}
  bool sharing()const{return sharing_;}
  String configure(const String &body){
    std::lock_guard<std::mutex> lock(mutex_);JsonDocument d;
    if(busy()||body.length()>256||deserializeJson(d,body)||!d["automatic_check"].is<bool>()||!d["accept_offers"].is<bool>()||!d["sharing"].is<bool>())return "{\"accepted\":false,\"error\":\"invalid_or_busy_peer_policy\"}";
    File f=LittleFS.open("/peer-exchange.pending","w");const size_t size=measureJson(d);
    if(!f)return "{\"accepted\":false,\"error\":\"storage_unavailable\"}";
    const size_t written=serializeJson(d,f);f.flush();f.close();
    File check=LittleFS.open("/peer-exchange.pending","r");JsonDocument verified;
    const bool ok=written==size&&check&&check.size()==size&&!deserializeJson(verified,check)&&verified.as<JsonVariantConst>()==d.as<JsonVariantConst>();
    if(check)check.close();
    if(!ok||!LittleFS.rename("/peer-exchange.pending","/peer-exchange.json"))return "{\"accepted\":false,\"error\":\"policy_write_failed\"}";
    automatic_=d["automatic_check"].as<bool>();acceptOffers_=d["accept_offers"].as<bool>();sharing_=d["sharing"].as<bool>();
    return "{\"accepted\":true}";
  }
  String offer(const String &body,const String &observedAddress){
    using namespace BoardCore::PeerRoutePolicy;
    if(!acceptOffers_)return "{\"accepted\":false,\"error\":\"peer_offers_disabled\"}";
    JsonDocument d;
    if(body.length()>2048||deserializeJson(d,body))return "{\"accepted\":false,\"error\":\"invalid_offer\"}";
    auto source=d["source"].as<JsonObjectConst>();
    const char *board=source["board_id"]|"",*boot=source["boot_id"]|"",*host=source["hostname"]|"",*address=source["address"]|"";
    if(!boardId(board)||self_==board||!hexId(boot)||!hostname(host)||!privateIpv4(address)||observedAddress!=address)return "{\"accepted\":false,\"error\":\"invalid_offer_source\"}";
    const auto a=source["artifact"].as<JsonObjectConst>();
    BoardCore::PeerUpdatePolicy::Artifact local{PakarPeerArtifact::key(),BoardCore::BuildInfo::kVersion,"",0};
    BoardCore::PeerUpdatePolicy::Artifact remote{a["family"]|"",a["version"]|"",a["sha256"]|"",a["bytes"]|0U};
    if(!BoardCore::PeerUpdatePolicy::canStage(local,remote,FirmwarePlatform::otaCapacity())||!BoardCore::PeerUpdatePolicy::peerPath(a["path"]|""))return "{\"accepted\":false,\"error\":\"incompatible_offer\"}";
    std::lock_guard<std::mutex> lock(mutex_);offered_=body;offeredAt_=millis();
    return "{\"accepted\":true,\"download_started\":false,\"apply_started\":false}";
  }
  bool busy()const{return busy_;}
  bool discover() {
    if(busy_.exchange(true))return false;
    if(ESP.getFreeHeap()<(worker_?32768U:49152U)){busy_=false;return false;}
    if(!worker_&&xTaskCreate(worker,"pakar-catalogue",6144,this,1,&worker_)!=pdPASS){busy_=false;return false;}
    xTaskNotifyGive(worker_);return true;
  }
  String snapshot(unsigned offset=0) {
    std::lock_guard<std::mutex> lock(mutex_);JsonDocument d;
    d["busy"]=busy();d["automatic_authority"]=false;d["error"]=error_;
    d["policy"]["automatic_check"]=automatic_.load();d["policy"]["accept_offers"]=acceptOffers_.load();d["policy"]["sharing"]=sharing_.load();
    if(offered_.length()&&BoardCore::PeerCatalogPolicy::currentOffer(millis(),offeredAt_)){
      JsonDocument offer;if(!deserializeJson(offer,offered_))d["offer"]=offer.as<JsonObjectConst>();
      d["offer_verified"]=false; // Suggestion only; staging verifies source/hash.
    }
    auto rows=d["peers"].to<JsonArray>();
    unsigned index=0;d["offset"]=offset;d["next_offset"]=nullptr;
    for(const auto &r:rows_){const unsigned row=index++;if(row<offset||!r.identity.length())continue;
      JsonDocument peer;if(deserializeJson(peer,r.identity))continue;
      // Same bounded result fits HTTP, WS and the existing 8 KiB USB API.
      // A continuation retains every peer, rather than silently dropping rows.
      if(measureJson(d)+r.identity.length()+256>7600){d["next_offset"]=row;break;}
      auto o=rows.add<JsonObject>();o["info"]=peer.as<JsonObjectConst>();o["endpoint"]=r.origin;
      o["observed_address"]=r.address;o["observation_age_ms"]=uint32_t(millis()-r.at);
      o["stale"]=uint32_t(millis()-r.at)>=BoardCore::PeerCatalogPolicy::staleAfterMs;
      o["verified"]=true;
    }
    String out;if(d.overflowed()||!out.reserve(measureJson(d)+1))return "{\"accepted\":false,\"error\":\"catalogue_memory_unavailable\"}";
    serializeJson(d,out);return out;
  }
 private:
  struct Row {String key,origin,address,identity;uint32_t at=0;} rows_[BoardCore::PeerCatalogPolicy::capacity];
  String self_,error_,offered_;uint32_t offeredAt_=0;std::mutex mutex_;std::atomic<bool> busy_{false},automatic_{true},acceptOffers_{true},sharing_{true};TaskHandle_t worker_=nullptr;
  std::function<String()> localInfo_;
  static void worker(void *owner){auto &s=*static_cast<PeerCatalogue*>(owner);for(;;){ulTaskNotifyTake(pdTRUE,portMAX_DELAY);s.scan();s.busy_=false;}}
  class PinnedClient final:public NetworkClient {
   public:explicit PinnedClient(IPAddress ip):ip_(ip){}
    int connect(const char*,uint16_t port)override{return NetworkClient::connect(ip_,port,700);}
    int connect(const char*,uint16_t port,int32_t ms)override{return NetworkClient::connect(ip_,port,ms);}
   private:IPAddress ip_;
  };
  void probe(const String &host,uint16_t port,IPAddress ip) {
    using namespace BoardCore::PeerRoutePolicy;
    if(!hostname(host.c_str())||!port||!privateAddress(ip[0],ip[1],ip[2],ip[3])||ip==WiFi.localIP()||ip==WiFi.softAPIP())return;
    PinnedClient client(ip);HTTPClient http;
    String origin=String("http://")+host+":";origin+=String(port);
    if(!http.begin(client,origin+"/api/peer/info"))return;
    http.setConnectTimeout(700);http.setTimeout(900);http.setReuse(false);http.setFollowRedirects(HTTPC_DISABLE_FOLLOW_REDIRECTS);
    http.setUserAgent(String("PakarController/")+BoardCore::BuildInfo::kVersion+" (peer-discovery)");
    // Deliberately omit control-version/target headers for public bootstrap.
    const char *headers[]={"X-Pakar-Board","X-Pakar-Boot"};http.collectHeaders(headers,2);
    const int code=http.GET(),size=http.getSize();String body;
    if(code==200&&size>0&&size<=4096&&body.reserve(size)) {
      const uint32_t at=millis();uint8_t block[256];auto *stream=http.getStreamPtr();
      while(body.length()<static_cast<size_t>(size)&&uint32_t(millis()-at)<1200) {
        const size_t n=min<size_t>(sizeof(block),min<size_t>(stream->available(),size-body.length()));
        if(!n){if(!http.connected())break;vTaskDelay(1);continue;}
        const int got=stream->read(block,n);if(got<=0)break;body.concat(reinterpret_cast<const char*>(block),got);
      }
    }
    JsonDocument d;String key;
    if(body.length()!=static_cast<size_t>(size)||deserializeJson(d,body)){http.end();return;}
    const char *product=d["product"]|"";if(!*product)product=d["application"]|"";
    const char *kind=d["kind"]|"board";
    bool valid=BoardCore::PeerCatalogPolicy::product(product)&&BoardCore::PeerCatalogPolicy::kind(kind);
    if(valid&&!strcmp(kind,"board")) {
      key=d["board_id"]|"";
      valid=boardId(key.c_str())&&key!=self_&&hexId(d["boot_id"]|"")&&
          http.header("X-Pakar-Board")==key&&http.header("X-Pakar-Boot")==d["boot_id"].as<String>();
      if(valid)key=String("board:")+key;
    }else if(valid){key=d["instance_id"]|"";if(key.isEmpty())key=d["id"]|"";valid=key.length()>0&&key.length()<=80;
      for(size_t i=0;i<key.length();++i)if(key[i]<33||key[i]>126)valid=false;
      if(valid)key=String("app:")+key;}
    const IPAddress ourAddress=client.localIP();http.end();if(!valid)return;
    // Offer the receiver an available newer image. This endpoint only stores
    // an expiring suggestion; it cannot stage, apply, change policy or reboot.
    if(localInfo_&&sharing_&&!strcmp(kind,"board")) {
      JsonDocument local;if(!deserializeJson(local,localInfo_())) {
        const auto a=local["artifacts"][0].as<JsonObjectConst>(),b=d["artifacts"][0].as<JsonObjectConst>();
        const BoardCore::PeerUpdatePolicy::Artifact ours{a["family"]|"",a["version"]|"",a["sha256"]|"",a["bytes"]|0U};
        const BoardCore::PeerUpdatePolicy::Artifact theirs{b["family"]|"",b["version"]|"",b["sha256"]|"",b["bytes"]|0U};
        if(BoardCore::PeerCatalogPolicy::canOffer(ours,theirs,true)&&http.begin(client,origin+"/api/peer/offer")) {
          JsonDocument offer;auto source=offer["source"].to<JsonObject>();source["board_id"]=local["board_id"];source["boot_id"]=local["boot_id"];
          String ownHost=local["hostname"]|"";if(!ownHost.endsWith(".local"))ownHost+=".local";
          source["hostname"]=ownHost;source["address"]=ourAddress.toString();source["artifact"]=a;
          String packet;serializeJson(offer,packet);http.addHeader("Content-Type","application/json");http.POST(packet);http.end();
        }
      }
    }
    // An index retains identity/update metadata, not each app's full hardware
    // report. Eight 4 KiB reports plus a JSON snapshot could exhaust S3 heap.
    // The original endpoint remains available for complete information.
    JsonDocument compact;
    for(const char *field:{"product","application","kind","id","instance_id","session_id","board_id","boot_id","hostname","name","role","version","build","compatibility","exchange","artifacts"})
      if(!d[field].isNull())compact[field]=d[field];
    compact["metadata_compact"]=true;
    const size_t compactSize=measureJson(compact);if(compact.overflowed()||compactSize>1536)return;
    String retained;if(!retained.reserve(compactSize+1))return;serializeJson(compact,retained);if(retained.length()!=compactSize)return;
    std::lock_guard<std::mutex> lock(mutex_);Row *slot=nullptr;
    for(auto &r:rows_)if(r.key==key){slot=&r;break;}
    if(!slot)for(auto &r:rows_)if(r.key.isEmpty()){slot=&r;break;}
    if(!slot){slot=&rows_[0];for(auto &r:rows_)if(uint32_t(millis()-r.at)>uint32_t(millis()-slot->at))slot=&r;}
    *slot={key,origin,ip.toString(),retained,millis()};
  }
  void scan() {
    {std::lock_guard<std::mutex> lock(mutex_);error_="";}
    for(const char *service:{"_pakarcontroller","_pakar-update"}) {
      mdns_result_t *results=nullptr;
      if(mdns_query_ptr(service,"_tcp",700,BoardCore::PeerCatalogPolicy::capacity,&results)!=ESP_OK) {
        std::lock_guard<std::mutex> lock(mutex_);error_="mdns_query_failed";continue;
      }
      for(auto *r=results;r;r=r->next)if(r->hostname&&r->port) {
        for(auto *a=r->addr;a;a=a->next)if(a->addr.type==ESP_IPADDR_TYPE_V4) {
          IPAddress ip(a->addr.u_addr.ip4.addr);probe(String(r->hostname)+".local",r->port,ip);break;
        }
      }
      mdns_query_results_free(results);
    }
  }
};
#endif
