#pragma once
#include <stdint.h>

// Adapted from Orbit WebAssetPolicy/EventStoragePolicy at 6e92d963.
// Keep fresh write admission separate from cached filesystem telemetry. The
// reserve includes metadata/settings headroom; never reclaim unrelated files.
namespace PakarUploadSpace {
constexpr uint32_t ReserveBytes=65536, BlockBytes=4096;
inline uint64_t rounded(uint32_t bytes){return (uint64_t(bytes)+BlockBytes-1)/BlockBytes*BlockBytes;}
inline bool begin(uint32_t free,uint32_t size){return uint64_t(free)>=ReserveBytes+rounded(size)+BlockBytes;}
inline bool write(uint32_t free,uint32_t offset,uint32_t count){
  if(!count||count>UINT32_MAX-offset)return false;
  return uint64_t(free)>=ReserveBytes+BlockBytes+rounded(offset+count)-rounded(offset);
}
}
