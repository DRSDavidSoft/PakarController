#include <Arduino.h>
#include <stdexcept>
using TaskHandle_t=void*;
constexpr int pdPASS=1,pdTRUE=1;constexpr uint32_t portMAX_DELAY=UINT32_MAX;
void (*testWorker)(void*)=nullptr;void *testOwner=nullptr;bool taken=false;
int xTaskCreate(void(*fn)(void*),const char*,unsigned,void *owner,int,TaskHandle_t*out){testWorker=fn;testOwner=owner;*out=owner;return pdPASS;}
void xTaskNotifyGive(TaskHandle_t){}
unsigned ulTaskNotifyTake(int,uint32_t){if(taken)throw std::runtime_error("idle");taken=true;return 1;}
void vTaskDelay(unsigned ticks);
#include "../firmware/esp/src/PeerCatalogue.h"
#include <Update.h>
uint32_t testNow=1000;fs::FS LittleFS;TestEsp ESP;TestUpdater Update;
std::map<std::string,TestResponse>testResponses;std::vector<TestRequest>testRequests;
void vTaskDelay(unsigned ticks){testNow+=ticks;}
String encode(const JsonDocument&d){String out;serializeJson(d,out);return out;}
JsonDocument decode(const String&s){JsonDocument d;assert(!deserializeJson(d,s));return d;}
int main(){
  const char *boot="0123456789abcdef0123456789abcdef";
  PeerCatalogue catalogue;catalogue.begin("self-board");
  assert(decode(catalogue.configure("{\"automatic_check\":false,\"accept_offers\":true,\"sharing\":false}"))["accepted"]==true);
  assert(!catalogue.automaticChecks()&&!catalogue.sharing());
  PeerCatalogue restored;restored.begin("self-board");assert(!restored.automaticChecks()&&!restored.sharing());
  JsonDocument d;d["source"]["hostname"]="neighbor.local";d["source"]["address"]="192.168.1.21";
  d["source"]["board_id"]="neighbor";d["source"]["boot_id"]=boot;
  auto a=d["source"]["artifact"].to<JsonObject>();a["family"]=PakarPeerArtifact::key();a["version"]="0.2.9";a["bytes"]=4096;
  a["sha256"]="1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";a["path"]="/firmware/download";
  assert(decode(catalogue.offer(encode(d),"192.168.1.22"))["accepted"]==false);
  assert(decode(catalogue.offer(encode(d),"192.168.1.21"))["accepted"]==true); // checks OFF still answers
  assert(decode(catalogue.snapshot())["offer_verified"]==false&&OtaFixture::begins==0);
  testNow+=60001;assert(decode(catalogue.snapshot())["offer"].isNull());
  JsonDocument identity;identity["product"]="PakarController";identity["kind"]="desktop-app";identity["instance_id"]="standalone-app";identity["version"]="99.0.0";
  const String url="http://neighbor.local:8789/api/peer/info";testResponses[url].bytes=std::vector<uint8_t>();
  const String info=encode(identity);testResponses[url].bytes.assign(info.begin(),info.end());
  mdns_ip_addr_t address;address.addr.u_addr.ip4.addr=0x1501a8c0;
  mdns_result_t result;result.hostname="NeIgHbOr";result.port=8789;result.addr=&address;testMdnsResults=&result;
  assert(catalogue.discover());taken=false;try{testWorker(testOwner);}catch(const std::runtime_error&e){assert(std::string(e.what())=="idle");}
  auto state=decode(catalogue.snapshot());assert(!catalogue.busy()&&state["peers"].size()==1);
  assert(state["peers"][0]["info"]["kind"]=="desktop-app");assert(OtaFixture::begins==0);
  assert(decode(catalogue.configure("{\"automatic_check\":false,\"accept_offers\":true,\"sharing\":true}"))["accepted"]==true);
  JsonDocument local;local["product"]="PakarController";local["kind"]="board";local["board_id"]="self-board";local["boot_id"]=boot;local["hostname"]="self-board";
  auto ourImage=local["artifacts"].to<JsonArray>().add<JsonObject>();ourImage.set(a);ourImage["version"]="0.2.8";
  const String localInfo=encode(local);catalogue.setLocalInfo([localInfo](){return localInfo;});
  identity.clear();identity["product"]="PakarController";identity["kind"]="board";identity["board_id"]="older-board";identity["boot_id"]=boot;
  auto oldImage=identity["artifacts"].to<JsonArray>().add<JsonObject>();oldImage.set(a);oldImage["version"]="0.2.7";
  result.port=80;const String boardUrl="http://neighbor.local:80/api/peer/info",offerUrl="http://neighbor.local:80/api/peer/offer";
  const String older=encode(identity);testResponses[boardUrl].bytes.assign(older.begin(),older.end());testResponses[boardUrl].board="older-board";testResponses[boardUrl].boot=boot;
  testResponses[offerUrl].bytes=std::vector<uint8_t>{'{','}'};testRequests.clear();
  assert(catalogue.discover());taken=false;try{testWorker(testOwner);}catch(const std::runtime_error&){}
  bool offered=false;for(const auto &request:testRequests)if(request.method=="POST"&&request.url==offerUrl){
    offered=true;const auto suggestion=decode(request.body);assert(suggestion["source"]["board_id"]=="self-board");
    assert(suggestion["source"]["artifact"]["version"]=="0.2.8"&&suggestion["source"]["address"]=="192.168.1.20");
  }
  assert(offered&&!catalogue.automaticChecks()&&OtaFixture::begins==0);
  PeerCatalogue bounded;bounded.begin("self-board");
  mdns_result_t nodes[8];std::string names[8];
  for(unsigned i=0;i<8;++i){
    names[i]="node-"+std::to_string(i);nodes[i].hostname=names[i].c_str();nodes[i].port=8789;nodes[i].addr=&address;nodes[i].next=i<7?&nodes[i+1]:nullptr;
    JsonDocument app;app["product"]="PakarController";app["kind"]="desktop-app";app["instance_id"]=names[i];app["version"]="0.2.9";
    app["client"]["hardware_report"]=String(1800,'x');app["exchange"]["future_optional_data"]=String(900,'x');
    const String packet=encode(app),url=String("http://")+names[i].c_str()+".local:8789/api/peer/info";
    testResponses[url].bytes.assign(packet.begin(),packet.end());
  }
  testMdnsResults=nodes;assert(bounded.discover());taken=false;try{testWorker(testOwner);}catch(const std::runtime_error&){}
  unsigned found=0,offset=0,pages=0;
  do {const String packet=bounded.snapshot(offset);assert(packet.length()<8192);const auto page=decode(packet);++pages;
    for(const auto row:page["peers"].as<JsonArrayConst>()){++found;assert(row["info"]["metadata_compact"]==true&&row["info"]["client"].isNull());}
    if(page["next_offset"].isNull())break;
    const unsigned next=page["next_offset"];assert(next>offset);offset=next;
  }while(pages<8);
  assert(found==8&&pages>1&&OtaFixture::begins==0);
  puts("Actual peer catalogue: passive offers with checks off, source binding, expiry, persistent policy, standalone app discovery, board-owned newer-image offer without apps and no implicit OTA passed (SDK doubles).");
}
