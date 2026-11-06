#pragma once
#include <Arduino.h>
#include <ArduinoJson.h>
#include <BuildInfo.h>
#include <PeerCompatibility.h>

namespace PakarPeer {
inline void compatibility(JsonObject object) {
  using namespace BoardCore::PeerCompatibility;
  object["enforced"]=enforced;
  object["minimum_app"]=minimumApp;object["minimum_board"]=minimumBoard;
  if(*maximumApp)object["maximum_app"]=maximumApp;else object["maximum_app"]=nullptr;
  if(*maximumBoard)object["maximum_board"]=maximumBoard;else object["maximum_board"]=nullptr;
}
inline String policyJson() {
  JsonDocument document;compatibility(document.to<JsonObject>());
  String body;serializeJson(document,body);return body;
}
inline String product() { return String("PakarController/")+BoardCore::BuildInfo::kVersion; }
inline String admissionError() {
  String body="{\"error\":\"unsupported_app_version\",\"accepted\":false,\"version\":\"";
  body+=BoardCore::BuildInfo::kVersion;body+="\",\"compatibility\":";body+=policyJson();body+='}';return body;
}
// Version-independent, additive identity bootstrap. No secrets or settings dump.
inline String info(const String &board,const String &boot,const String &hostname) {
  JsonDocument document;
  document["application"]="PakarController";document["version"]=BoardCore::BuildInfo::kVersion;
  document["product"]="PakarController";document["kind"]="board";
  document["board_id"]=board;document["boot_id"]=boot;document["hostname"]=hostname;
  compatibility(document["compatibility"].to<JsonObject>());
  String body;serializeJson(document,body);return body;
}
} // namespace PakarPeer
