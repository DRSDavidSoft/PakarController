#pragma once
#include "FirmwarePlatform.h"
#include <BuildInfo.h>
#include <PeerUpdatePolicy.h>
#include <GameRole.h>
#include <stddef.h>
#ifndef PAKAR_PEER_IMAGE_FAMILY
#define PAKAR_PEER_IMAGE_FAMILY ""
#endif

namespace PakarPeerArtifact {
// Also retained inside the image for post-download role/profile verification.
inline const char *family(){return "PAKAR-PEER-FAMILY:" PAKAR_PEER_IMAGE_FAMILY ":END";}
inline const char *key(){return PAKAR_PEER_IMAGE_FAMILY;}
inline bool transferable(){return *key()!=0;}
struct ImageReleaseMarker {
  char bytes[96]{};
  static_assert(sizeof(BoardCore::BuildInfo::kVersion)+sizeof("PAKAR-PEER-RELEASE:")+sizeof(":END")-2<=sizeof(bytes),"Peer release marker exceeds bounded image scan");
  constexpr ImageReleaseMarker(){
    const char prefix[]="PAKAR-PEER-RELEASE:",suffix[]=":END";size_t i=0;
    for(size_t n=0;n<sizeof(prefix)-1;++n)bytes[i++]=prefix[n];
    for(size_t n=0;n<sizeof(BoardCore::BuildInfo::kVersion)-1;++n)bytes[i++]=BoardCore::BuildInfo::kVersion[n];
    for(size_t n=0;n<sizeof(suffix)-1;++n)bytes[i++]=suffix[n];
  }
};
inline const char *releaseMarker(){static constexpr ImageReleaseMarker marker{};return marker.bytes;}
}
