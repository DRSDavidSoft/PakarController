#pragma once
#include "Arduino.h"
#include <vector>
#include <set>
#include <cassert>
class IPAddress {
 public:bool fromString(const String &text){value_=text;return true;}
  IPAddress()=default;explicit IPAddress(uint32_t v){char out[16];snprintf(out,sizeof(out),"%u.%u.%u.%u",unsigned(v&255),unsigned((v>>8)&255),unsigned((v>>16)&255),unsigned(v>>24));value_=out;}
  uint8_t operator[](unsigned index)const{unsigned a[4]{};sscanf(value_.c_str(),"%u.%u.%u.%u",&a[0],&a[1],&a[2],&a[3]);return index<4?a[index]:0;}
  bool operator==(const IPAddress &other)const{return value_==other.value_;}
  String toString()const{return value_;}
 private:String value_;
};
class WiFiClient{
 public:
 inline static std::set<const WiFiClient*> live;
 WiFiClient(){live.insert(this);}
 virtual ~WiFiClient(){live.erase(this);}
 virtual int connect(const char*,uint16_t){return 1;}
 virtual int connect(const char*,uint16_t,int32_t){return 1;}
 virtual int connect(IPAddress,uint16_t,int32_t){return 1;}
 IPAddress localIP()const{IPAddress ip;ip.fromString("192.168.1.20");return ip;}
 virtual void stop(){assert(live.count(this));at=bytes.size();}
 std::vector<uint8_t>bytes;size_t at=0;
 int available()const{return static_cast<int>(min<size_t>(1024,bytes.size()-at));}
 int read(uint8_t*out,size_t n){n=min(n,bytes.size()-at);memcpy(out,bytes.data()+at,n);at+=n;return n;}
};
constexpr uint8_t FM_UNKNOWN=255;
class TestEsp{
 public:std::vector<uint8_t>flash;bool crc=true;unsigned restarts=0;uint32_t heap=50000;
 uint32_t getFlashChipSpeed()const{return 80000000;}
 uint32_t getFreeHeap()const{return heap;}uint32_t getMaxFreeBlockSize()const{return heap;}
 uint8_t getFlashChipMode()const{return 2;}uint32_t getFreeSketchSpace()const{return 1048576;}
 // Metadata-only SDK doubles. Actual flash-boundary interception is exercised
 // separately by test-s3-flash-guard.ps1, not this updater state-machine fixture.
 String getSketchMD5()const{return "00000000000000000000000000000000";}
 uint32_t getSketchSize()const{return static_cast<uint32_t>(flash.size());}
 uint16_t getChipRevision()const{return 2;}
 bool checkFlashCRC()const{return crc;}
 bool flashRead(uint32_t at,uint8_t*out,size_t n){if(at+n>flash.size())return false;memcpy(out,flash.data()+at,n);return true;}
 void restart(){++restarts;}
};
extern TestEsp ESP;
using NetworkClient=WiFiClient;
struct TestWiFi{IPAddress localIP()const{IPAddress ip;ip.fromString("192.168.1.20");return ip;}IPAddress softAPIP()const{IPAddress ip;ip.fromString("192.168.4.1");return ip;}};
inline TestWiFi WiFi;
