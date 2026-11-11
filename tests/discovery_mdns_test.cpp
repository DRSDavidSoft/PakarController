#include "../firmware/esp32s3/src/DiscoveryMdns.h"
#include <cassert>
#include <cstdio>
#include <string>
#include <vector>

struct Responder {
  bool initialized=false,failBegin=false;
  unsigned registrations=0,failAt=0,starts=0,stops=0;
  std::string hostname;
  std::vector<std::string> entries;
  bool begin(const char *name) {
    ++starts;if(initialized)return false;
    initialized=true;hostname=name;return !failBegin; // Actual SDK partial-init failure.
  }
  void end(){++stops;initialized=false;entries.clear();}
  bool registration(const std::string &value) {
    ++registrations;if(registrations==failAt)return false;
    entries.push_back(value);return true;
  }
  bool addService(const char *name,const char *protocol,uint16_t port) {
    return registration(std::string(name)+"."+protocol+":"+std::to_string(port));
  }
  bool addServiceTxt(char *name,char *protocol,char *key,char *value) {
    return registration(std::string(name)+"."+protocol+" "+key+"="+value);
  }
  void addServiceTxt(const char *,const char *,const char *,const char *) {
    assert(false&&"SDK convenience overload loses registration failure");
  }
};
static bool start(Responder &responder) {
  return PakarS3::startDiscoveryMdns(responder,"ledboard","PakarController","pakarcontroller","0.2.40",7890);
}
int main() {
  Responder good;assert(start(good));
  assert(good.initialized&&good.hostname=="ledboard"&&good.registrations==18&&good.entries.size()==18);
  assert(good.entries[5]=="pakarcontroller.tcp:80"&&good.entries[16]=="pakar-stream.udp:7890");
  assert(good.entries[6]=="lan-peer.tcp:80"&&good.entries[7]=="lan-peer.tcp product=pakarcontroller"&&
    good.entries[8]=="lan-peer.tcp health=/healthz");
  assert(good.entries[12]==std::string("pakarcontroller.tcp role=")+BoardCore::GameRole::name);
  good.end();assert(start(good));
  Responder noShell;assert(PakarS3::startDiscoveryMdns(noShell,"ledboard","PakarController","pakarcontroller","0.2.7",7777,0));
  assert(noShell.registrations==17);
  Responder moved;assert(PakarS3::startDiscoveryMdns(moved,"ledboard","PakarController","pakarcontroller","0.2.7",7777,2323));
  assert(moved.entries.back()=="telnet.tcp:2323");
  Responder ssh;assert(PakarS3::startDiscoveryMdns(ssh,"keys","Pakar Keys","pakarcontroller","0.2.8",7777,23,22));
  assert(ssh.entries.back()=="ssh.tcp:22"&&ssh.registrations==19);
  Responder failedSsh;failedSsh.failAt=19;
  assert(!PakarS3::startDiscoveryMdns(failedSsh,"keys","Pakar Keys","pakarcontroller","0.2.8",7777,23,22));
  assert(!failedSsh.initialized&&failedSsh.entries.empty());
  Responder partial;partial.failBegin=true;
  assert(!start(partial)&&!partial.initialized&&partial.stops==1&&partial.registrations==0);
  partial.failBegin=false;assert(start(partial));
  for(unsigned at=1;at<=18;++at) {
    Responder failed;failed.failAt=at;
    assert(!start(failed)&&!failed.initialized&&failed.stops==1&&failed.entries.empty());
    failed.failAt=0;failed.registrations=0;assert(start(failed));
  }
  std::puts("Checked S3 mDNS/DNS-SD registration and partial-start cleanup fixture passed; not multicast/RF acceptance");
}
