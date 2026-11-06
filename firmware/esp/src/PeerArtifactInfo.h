#pragma once
#if defined(ARDUINO_ARCH_ESP32)
#include "PeerArtifact.h"
#include <mutex>

// Loop-owned incremental hash; public HTTP readers see an immutable snapshot.
// No complete-image scan on every discovery or telemetry request.
class PeerArtifactInfo {
 public:
  void begin(uint32_t size){size_=size;sha_.begin();refresh();}
  void service() {
    if(!size_||done_>=size_||failed_)return;
    alignas(4) uint8_t bytes[512];const size_t n=min<uint32_t>(sizeof(bytes),size_-done_);
    if(!FirmwarePlatform::readRunning(done_,bytes,n)){failed_=true;refresh();return;}
    sha_.add(bytes,n);done_+=n;
    if(done_==size_){digest_=sha_.hex();refresh();}
  }
  String decorate(const String &identity,bool sharing=true,bool checking=true) {
    JsonDocument d;if(deserializeJson(d,identity))return identity;
    String snapshot;{std::lock_guard<std::mutex> lock(mutex_);snapshot=snapshot_;}
    JsonDocument info;if(!deserializeJson(info,snapshot)) {
      d["role"]=BoardCore::GameRole::name;
      d["exchange"]=info["exchange"];d["artifacts"]=info["artifacts"];
      d["exchange"]["automatic_check"]=checking;d["exchange"]["firmware_sharing"]=sharing&&PakarPeerArtifact::transferable();
      if(!sharing)d["artifacts"].to<JsonArray>();
    }
    String out;serializeJson(d,out);return out;
  }
 private:
  uint32_t size_=0,done_=0;bool failed_=false;String digest_,snapshot_;FirmwarePlatform::Sha256 sha_;std::mutex mutex_;
  void refresh(){JsonDocument d;auto e=d["exchange"].to<JsonObject>();
    e["catalogue"]="/api/peers/discovered";e["discover"]="/api/peers/discover";
    e["stage"]="/api/peers/update";e["apply"]="/api/firmware/apply";
    e["automatic_apply"]=false;e["passive_identity"]=true;
    e["firmware_sharing"]=PakarPeerArtifact::transferable();e["download_requires_auth"]=true;
    e["hash_state"]=failed_?"failed":done_==size_&&size_?"ready":"preparing";
    auto a=d["artifacts"].to<JsonArray>();if(digest_.length()&&PakarPeerArtifact::transferable()) {
      auto item=a.add<JsonObject>();item["family"]=PakarPeerArtifact::key();
      item["image_marker"]=PakarPeerArtifact::family();item["version"]=BoardCore::BuildInfo::kVersion;
      item["bytes"]=size_;item["sha256"]=digest_;item["path"]="/firmware/download";
      item["model"]=FirmwarePlatform::model();item["role"]=BoardCore::GameRole::name;
    }
    String out;serializeJson(d,out);std::lock_guard<std::mutex> lock(mutex_);snapshot_=std::move(out);
  }
};
#endif
