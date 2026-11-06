#pragma once
#include "FirmwarePlatform.h"
#include <BuildInfo.h>
#include <PeerUpdatePolicy.h>
#include <GameRole.h>
#ifndef PAKAR_PEER_IMAGE_FAMILY
#define PAKAR_PEER_IMAGE_FAMILY ""
#endif

namespace PakarPeerArtifact {
// Also retained inside the image for post-download role/profile verification.
inline const char *family(){return "PAKAR-PEER-FAMILY:" PAKAR_PEER_IMAGE_FAMILY ":END";}
inline const char *key(){return PAKAR_PEER_IMAGE_FAMILY;}
inline bool transferable(){return *key()!=0;}
}
