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
  mdns_result_t result;result.hostname="neighbor";result.port=8789;result.addr=&address;testMdnsResults=&result;
  assert(catalogue.discover());taken=false;try{testWorker(testOwner);}catch(const std::runtime_error&e){assert(std::string(e.what())=="idle");}
  auto state=decode(catalogue.snapshot());assert(!catalogue.busy()&&state["peers"].size()==1);
  assert(state["peers"][0]["info"]["kind"]=="desktop-app");assert(OtaFixture::begins==0);
  puts("Actual peer catalogue: passive offers with checks off, source binding, expiry, persistent policy, standalone app discovery and no implicit OTA passed (SDK doubles).");
}
