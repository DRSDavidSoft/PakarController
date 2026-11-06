#pragma once
#include "PeerCompatibility.h"
#include "PeerRoutePolicy.h"

// Artifact compatibility is independent of either party's optional release
// admission/automatic-check policy. Never infer a newer build from a Git hash.
namespace BoardCore { namespace PeerUpdatePolicy {
enum class Relation { Unknown, Same, RemoteNewer, LocalNewer, Incompatible };
struct Artifact {
  const char *family="",*version="",*sha256="";
  uint32_t bytes=0;
};
inline bool hash(const char *value) {
  if(!value||strlen(value)!=64)return false;
  for(unsigned i=0;i<64;++i)if(!((value[i]>='0'&&value[i]<='9')||(value[i]>='a'&&value[i]<='f')))return false;
  return true;
}
inline Relation compare(const Artifact &local,const Artifact &remote) {
  if(!local.family||!remote.family||!*local.family||strcmp(local.family,remote.family))return Relation::Incompatible;
  PeerCompatibility::Version a,b;
  if(!PeerCompatibility::parse(local.version,a)||!PeerCompatibility::parse(remote.version,b))return Relation::Unknown;
  const int order=PeerCompatibility::compare(a,b);
  return order<0?Relation::RemoteNewer:order>0?Relation::LocalNewer:Relation::Same;
}
inline const char *name(Relation value) {
  switch(value){case Relation::RemoteNewer:return "remote-newer";case Relation::LocalNewer:return "local-newer";
    case Relation::Same:return "same-release";case Relation::Incompatible:return "incompatible-family";default:return "unknown-release";}
}
// This special LAN source is NOT an exemption for ordinary remote firmware
// URLs. Its exact origin, endpoint and current peer/boot headers are pinned by
// the transfer owner. Redirects, alternate paths and public IPs are forbidden.
inline bool peerPath(const char *path){return path&&!strcmp(path,"/firmware/download");}
inline bool canStage(const Artifact &local,const Artifact &remote,uint32_t capacity) {
  return compare(local,remote)==Relation::RemoteNewer && hash(remote.sha256) && remote.bytes>=24 && remote.bytes<capacity;
}
} }
