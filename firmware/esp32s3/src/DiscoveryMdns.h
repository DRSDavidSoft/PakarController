#pragma once
#include <stdint.h>
#include <PeerCompatibility.h>
#include <GameRole.h>

namespace PakarS3 {
// PU850's one hostname/service owner, adapted to checked Arduino-ESP32 startup.
// In3.3.12 the const-char addServiceTxt convenience overload discards its bool;
// select the actual checked overload just as that SDK wrapper does internally.
template<class Responder>
bool discoveryTxt(Responder &responder,const char *service,const char *protocol,const char *key,const char *value) {
  return responder.addServiceTxt(const_cast<char*>(service),const_cast<char*>(protocol),
      const_cast<char*>(key),const_cast<char*>(value));
}
template<class Responder>
bool startDiscoveryMdns(Responder &responder,const char *hostname,const char *appName,
    const char *appId,const char *version,uint16_t streamPort,uint16_t shellPort=23,uint16_t sshPort=0) {
  // SDK begin() can fail AFTER mdns_init succeeds (hostname allocation/set).
  // Always retire that partial owner before the existing bounded retry path.
  if(!responder.begin(hostname)){responder.end();return false;}
  const bool complete=responder.addService("http","tcp",80) &&
    discoveryTxt(responder,"http","tcp","path","/") &&
    discoveryTxt(responder,"http","tcp","app",appName) &&
    discoveryTxt(responder,"http","tcp","app_id",appId) &&
    discoveryTxt(responder,"http","tcp","version",version) &&
    responder.addService("pakarcontroller","tcp",80) &&
    responder.addService("lan-peer","tcp",80) &&
    discoveryTxt(responder,"lan-peer","tcp","product","pakarcontroller") &&
    discoveryTxt(responder,"lan-peer","tcp","health","/healthz") &&
    discoveryTxt(responder,"pakarcontroller","tcp","api","/api/status") &&
    discoveryTxt(responder,"pakarcontroller","tcp","websocket","/ws") &&
    discoveryTxt(responder,"pakarcontroller","tcp","version",version) &&
    discoveryTxt(responder,"pakarcontroller","tcp","role",BoardCore::GameRole::name) &&
    discoveryTxt(responder,"pakarcontroller","tcp","minimum_app",BoardCore::PeerCompatibility::minimumApp) &&
    discoveryTxt(responder,"pakarcontroller","tcp","minimum_board",BoardCore::PeerCompatibility::minimumBoard) &&
    discoveryTxt(responder,"pakarcontroller","tcp","version_check",BoardCore::PeerCompatibility::enforced?"true":"false") &&
    responder.addService("pakar-stream","udp",streamPort) &&
    (!shellPort||responder.addService("telnet","tcp",shellPort)) &&
    (!sshPort||responder.addService("ssh","tcp",sshPort));
  if(!complete)responder.end();
  return complete;
}
} // namespace PakarS3
