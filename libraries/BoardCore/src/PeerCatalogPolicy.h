#pragma once
#include "PeerUpdatePolicy.h"

// Shared discovery rules, not control admission. A peer may be inspected even
// when its release is outside the range admitted for hardware commands.
namespace BoardCore { namespace PeerCatalogPolicy {
constexpr unsigned capacity=8;
constexpr uint32_t staleAfterMs=120000, offerLifetimeMs=60000;
inline bool product(const char *value) {return value&&!strcmp(value,"PakarController");}
inline bool kind(const char *value) {
  return value&&(!strcmp(value,"board")||!strcmp(value,"android-app")||!strcmp(value,"desktop-app")||!strcmp(value,"bridge"));
}
inline bool canOffer(const PeerUpdatePolicy::Artifact &local,const PeerUpdatePolicy::Artifact &remote,bool sharing) {
  return sharing&&PeerUpdatePolicy::compare(local,remote)==PeerUpdatePolicy::Relation::LocalNewer&&
      PeerUpdatePolicy::hash(local.sha256)&&local.bytes>=24;
}
// An offer is advisory only. No receipt implies download/apply permission.
inline bool currentOffer(uint32_t now,uint32_t observed) {return uint32_t(now-observed)<offerLifetimeMs;}
} }
