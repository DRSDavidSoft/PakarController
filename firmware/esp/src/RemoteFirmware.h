#pragma once
#include <Arduino.h>
#include <ArduinoJson.h>
#include <LittleFS.h>
#if defined(ARDUINO_ARCH_ESP32)
#include <HTTPClient.h>
#include <WiFiClient.h>
#include <WiFiClientSecure.h>
#include "../../esp32s3/src/OtaStage.h"
#else
#include <ESP8266HTTPClient.h>
#include <WiFiClientSecureBearSSL.h>
#endif
#include "FirmwarePlatform.h"
#include <memory>
#include "RemotePolicy.h"
#include "UpdateProgress.h"

#ifndef PAKAR_USE_BUSY_WIRE
#define PAKAR_USE_BUSY_WIRE 1
#endif
#ifndef PAKAR_ALLOW_HTTP_FIRMWARE
#define PAKAR_ALLOW_HTTP_FIRMWARE 0
#endif
static_assert(PAKAR_ALLOW_HTTP_FIRMWARE==0 || PAKAR_ALLOW_HTTP_FIRMWARE==1,
              "PAKAR_ALLOW_HTTP_FIRMWARE must be 0 or 1");
class RemoteFirmware {
 public:
  struct Component { String url,sha,version; uint32_t size=0; };
  struct Plan { Component esp,avr; uint8_t targets=3;bool allowDowngrade=false; };
  // prepare arms Prog; program verifies flash/EEPROM and leaves Prog armed.
  // finish verifies the actual pair and releases Prog, including after ESP reboot.
  using Prepare=bool (*)(const Plan&,String&);
  using Program=bool (*)(const char*,const String&,String&);
  // commit=false is read-only readiness; commit=true is ONE Prog release.
  // 0=wait, 1=verified, -1=uncertain/failed; never replay a lost release ACK.
  using Finish=int8_t (*)(const Plan&,bool,String&);
  void begin(const String &boot, Prepare prepare, Program program, Finish finish);
  bool queue(const String &url,const String &sha,uint8_t targets,bool apply,
             bool downgrade,const String &bearer);
#if defined(ARDUINO_ARCH_ESP32)
  // Explicit receiver-owned staging, NEVER implicit apply. Source credentials
  // are supplied for this source only and never journalled or advertised.
  bool stagePeer(JsonObjectConst source,const String &authorization);
#endif
  bool apply(uint32_t id,const String &boot,const String &sha);
  bool cancel(uint32_t id,const String &boot);
  void service();
  void setServiceCallback(void (*callback)()) {service_=callback;}
  void setRebootReady(bool (*callback)()) {rebootReady_=callback;}
  void reportProgress(const char *stage,uint32_t done,uint32_t total) {
    progressStage_=stage;progressDone_=done;progressTotal_=total;
  }
  const char *progressStage()const {
    return (phase_==PrepareApply || phase_==ProgramAvr || phase_==ProgramEsp || phase_==VerifyPair) && progressStage_?progressStage_:phase();
  }
  bool busy() const;
  bool needsDownloadConnection() const {
    return !opened_ && (phase_==Manifest || phase_==DownloadEsp || phase_==DownloadAvr);
  }
  bool applying() const;
  bool rebooting() const { return phase_==Rebooting; }
  bool completionVerified()const{return phase_==Complete && completedThisBoot_;}
  uint32_t jobId() const { return id_; }
  bool ownsEspStage() const { return phase_==Ready && (plan_.targets&RemotePolicy::Esp); }
#if defined(ARDUINO_ARCH_ESP32)
  uint32_t stagedEspSize() const {return ownsEspStage()?espStage_.size():0;}
  bool readStagedEsp(uint32_t offset,uint8_t *out,size_t bytes) const {
    return ownsEspStage() && espStage_.read(offset,out,bytes);
  }
#endif
  PakarUpdate::Observation progress() const {
    const bool transfer=phase_==DownloadEsp || phase_==DownloadAvr;
    if(transfer)return {PakarUpdate::Phase::Receiving,transfer_.received,transfer_.expected};
    if(phase_==PrepareApply || phase_==ProgramAvr || phase_==ProgramEsp || phase_==VerifyPair)
      return {PakarUpdate::Phase::Receiving,progressDone_,progressTotal_};
    return {};
  }
  String json() const;
  const String &error() const { return error_; }
  const char *phase() const;
 private:
  enum Phase : uint8_t { Idle,Manifest,DownloadEsp,DownloadAvr,Ready,PrepareApply,
      ProgramAvr,ProgramEsp,Rebooting,VerifyPair,ReleasePair,Complete,Failed,Cancelled };
  bool openDownload(const String&,uint32_t,const String&,const char*);
  int download();
  bool parseManifest();
  bool validateImage(const Component&,const char*,bool);
  bool save();
  void fail(const char*);
  void fail(const __FlashStringHelper *s) {const String reason(s);fail(reason.c_str());}
  void close();
  bool installEsp();
  void cooperate(){if(service_)service_();yield();}
  bool validateEspStage();
  void verifyBoot();
  static String digest(const FirmwarePlatform::Sha256&);
  Phase phase_=Idle;
  Plan plan_;
  String boot_,manifestSha_,bearer_,error_,credentialOrigin_;
#if defined(ARDUINO_ARCH_ESP32)
  String peerHost_,peerAddress_,peerBoard_,peerBoot_,peerAuthorization_;
  bool peerSource_=false;
#endif
  uint32_t id_=0,lastDataAt_=0,startedAt_=0,rebootAt_=0,verifyAt_=0;
  bool autoApply_=false,downgrade_=false,opened_=false,prepareDone_=false;
  bool completedThisBoot_=false;
#if defined(ARDUINO_ARCH_ESP32)
  S3OtaStage espStage_;
  bool downloadingEspStage_=false;
#endif
  RemotePolicy::Transfer transfer_;
  String expectedSha_;
  File file_;
  // HTTPClient's destructor may call stop on its last disconnected client.
  // Declaration order keeps that client alive until HTTPClient is destroyed.
  std::unique_ptr<WiFiClient> socket_;
  HTTPClient http_;
  FirmwarePlatform::Sha256 sha_;
  Prepare prepare_=nullptr; Program program_=nullptr; Finish finish_=nullptr;
  void (*service_)()=nullptr;bool (*rebootReady_)()=nullptr;
  const char *progressStage_=nullptr;uint32_t progressDone_=0,progressTotal_=0;
};
