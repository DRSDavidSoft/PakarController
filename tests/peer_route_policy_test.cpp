#include <PeerRoutePolicy.h>
#include <PeerCatalogPolicy.h>
#include <cassert>
int main() {
  using namespace BoardCore::PeerUpdatePolicy;
  const char *sha="1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
  Artifact local{"table","0.2.8",sha,1024},newer{"table","0.2.9",sha,2048},key{"first-wins","0.2.9",sha,2048};
  assert(compare(local,newer)==Relation::RemoteNewer&&canStage(local,newer,4096));
  assert(compare(local,key)==Relation::Incompatible&&!canStage(local,key,4096));
  assert(BoardCore::PeerCatalogPolicy::canOffer(newer,local,true));
  assert(!BoardCore::PeerCatalogPolicy::canOffer(newer,local,false));
  assert(!canStage(local,newer,2048));newer.sha256="bad";assert(!canStage(local,newer,4096));
  Artifact same{"table","0.2.8+other-build",sha,1024};assert(compare(local,same)==Relation::Same);
  Artifact pre{"table","0.2.9-rc.1",sha,1024};assert(compare(local,pre)==Relation::RemoteNewer);
  assert(BoardCore::PeerCatalogPolicy::currentOffer(10,UINT32_MAX-10));
  assert(!BoardCore::PeerCatalogPolicy::currentOffer(100000,0));
  using namespace BoardCore::PeerRoutePolicy;
  for(const char *s:{"keycontroller.local","ledcontroller.local","david-asus"})assert(hostname(s));
  for(const char *s:{"localhost","127.0.0.1","a/../x","a:80","a@b","a\r\nx","-a","a-","a..b","a."})assert(!hostname(s));
  assert(privateAddress(192,168,4,1));assert(privateAddress(10,1,0,2));
  assert(!privateAddress(127,0,0,1));assert(!privateAddress(169,254,169,254));assert(!privateAddress(224,0,0,1));assert(!privateAddress(192,168,1,255));
  for(const char *s:{"192.168.1.201","10.0.0.2","172.16.1.99"})assert(privateIpv4(s));
  for(const char *s:{"192.168.1.201:80","192.168.01.2","192.168.1.256","192.168.1.0","192.168.1.255","127.0.0.1","169.254.169.254","8.8.8.8","1.2.3.4.5","192.168.1.2/",""})assert(!privateIpv4(s));
  assert(readPath("/api/status"));assert(!readPath("/api/peers/request"));assert(!readPath("/api/avr/probe"));assert(!readPath("/firmware/download"));
  assert(hexId("1234567890abcdef1234567890abcdef"));assert(!hexId("00000000000000000000000000000000"));
  assert(boardId("esp32s3-b81f3fbb0684"));assert(!boardId("a\r\nHost:evil"));
}
