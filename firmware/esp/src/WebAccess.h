#pragma once
#include "AccessConfig.h"
#include <ApiPolicy.h>
#include <PeerFence.h>
#include <DiagnosticText.h>
#include <ESPAsyncWebServer.h>
#include "SystemPlatform.h"
#include "PeerIdentity.h"
#if !defined(ESP8266) && !defined(ARDUINO_ARCH_ESP8266)
#include <mutex>
#endif

namespace PakarAccessDetail {
#if defined(ESP8266) || defined(ARDUINO_ARCH_ESP8266)
// ESP8266 callbacks and loop cooperate. Never yield while touching admission.
struct Mutex { void lock() {} void unlock() {} };
#else
using Mutex = std::mutex;
#endif
struct Guard {
  Mutex &mutex;
  explicit Guard(Mutex &value) : mutex(value) { mutex.lock(); }
  ~Guard() { mutex.unlock(); }
  Guard(const Guard &) = delete;
  Guard &operator=(const Guard &) = delete;
};
}

// PU850 Authentication/Sessions capability with complete authorization,
// expiring sessions and no password/token diagnostic dumps.
class WebAccess {
 public:
  WebAccess() : middleware_([this](AsyncWebServerRequest *request, ArMiddlewareNext next) {
    const String path = request->url();
    // Before login, uploads, catch-all handlers or dispatch: no aliases.
    if (BoardCore::ApiPolicy::numberedRoute(path.c_str())) {
      request->send(404, "application/json", F("{\"error\":\"not_found\"}"));
      return;
    }
    const bool publicRead = (request->method() == HTTP_GET || request->method() == HTTP_HEAD) &&
        (path == F("/") || path == F("/basic") || path == F("/api/auth/info") || path == F("/description.xml") ||
         path == F("/api/peer/info") ||
         path == F("/board-icon.png") || path == F("/app-icon.png") ||
         path == F("/assets/fonts/Vazirmatn-Regular.ttf") || path == F("/assets/fonts/Vazirmatn-OFL.txt"));
    const bool login = path == F("/login") && request->method() == HTTP_POST;
    // An offer stores one bounded, expiring suggestion, never a transfer or
    // control permission. The handler binds its origin to the actual LAN TCP
    // source. Keep this passive listener independent of release/check policy.
    const bool peerOffer=path==F("/api/peer/offer")&&request->method()==HTTP_POST&&!request->hasHeader("Origin");
    if (publicRead || login || peerOffer) { next(); return; }
    String unavailable;
    if (!ready(unavailable)) { request->send(503, "text/plain", unavailable); return; }
    if (!sameOrigin(request)) { request->send(403, "text/plain", F("Cross-origin request rejected")); return; }
    if (!allowed(request)) {
      request->send(401, "application/json", F("{\"error\":\"login_required\",\"login\":\"/login\"}"));
      return; // no browser Basic-auth dialog; API clients may still send Basic
    }
    if (!versionAllowed(request)) {
      request->send(426,"application/json",PakarPeer::admissionError());return;
    }
    if (!peerAllowed(request)) {
      request->send(409, "application/json", F("{\"error\":\"peer_changed\",\"accepted\":false}")); return;
    }
    next();
  }) {}
  void refreshCredentials() {
    PakarAccessDetail::Guard lock(mutex_);
    refreshCredentialsLocked();
  }
  AsyncMiddleware *middleware() { return &middleware_; }
  void setPeerInfoDecorator(std::function<String(const String&)> decorator){peerInfoDecorator_=std::move(decorator);}
  static String clientVersion(AsyncWebServerRequest *request) {
    const auto *header=request->getHeader("X-Pakar-Version");
    const String query=request->url()==F("/ws")?request->arg("version"):String();
    if(header && query.length() && header->value()!=query)return String();
    return header?header->value():query;
  }
  static bool versionAllowed(AsyncWebServerRequest *request) {
    if(!BoardCore::PeerCompatibility::enforced)return true;
    const String version=clientVersion(request);
    // Discovery/bootstrap reads can inspect why a peer is unsupported, but
    // a supplied incompatible version never admits an API/control session.
    // Generic HTTP/WS clients may omit product-version metadata. An explicit
    // incompatible/empty/conflicting declaration still fails admission.
    if(!request->hasHeader("X-Pakar-Version") &&
        !(request->url()==F("/ws") && request->hasArg("version")))return true;
    return BoardCore::PeerCompatibility::appAllowed(version.c_str());
  }
  void configurePeerFence(const String &board, const String &boot,const String &hostname=String()) {
    PakarAccessDetail::Guard lock(mutex_);
    boardId_ = board; bootId_ = boot; hostname_=hostname;
  }
  bool peerFenceReady() const {
    PakarAccessDetail::Guard lock(mutex_);
    return BoardCore::PeerFence::ready(boardId_.c_str(), bootId_.c_str());
  }
  bool peerAllowed(AsyncWebServerRequest *request) const {
    // Method is not an ownership exemption: ISP probe is a hardware-affecting
    // GET, and pinned readback must not silently read a different boot either.
    // Absent headers retain explicitly unfenced compatibility in PeerFence.
    const auto *board = request->getHeader("X-Pakar-Board");
    const auto *boot = request->getHeader("X-Pakar-Boot");
    PakarAccessDetail::Guard lock(mutex_);
    return BoardCore::PeerFence::accepts(boardId_.c_str(), bootId_.c_str(),
        board ? board->value().c_str() : nullptr, boot ? boot->value().c_str() : nullptr);
  }
  bool allowed(AsyncWebServerRequest *request) {
    PakarAccessDetail::Guard lock(mutex_);
    if (!accessConfig.ready()) return false;
    if (!accessConfig.enabled()) return true;
    if (basic_.allowed(request)) return true;
    const String token = cookieToken(request);
    return sessions_.validate(token.c_str(), millis());
  }
  static bool sameOrigin(AsyncWebServerRequest *request) {
    if (!request->hasHeader("Origin")) return true; // native clients
    return request->getHeader("Origin")->value() == String(F("http://")) + request->host();
  }
  void revokeAll() {
    PakarAccessDetail::Guard lock(mutex_);
    sessions_.clear();
    for (auto &slot : sockets_) slot = Socket{};
    refreshCredentialsLocked();
  }
  uint8_t sessionCount() const {
    PakarAccessDetail::Guard lock(mutex_); return sessions_.count(millis());
  }
  bool bindSocket(uint32_t id, AsyncWebServerRequest *request, const String &address = String()) {
    if (!id || !request || !sameOrigin(request) || !versionAllowed(request)) return false;
    const String token = cookieToken(request);
    PakarAccessDetail::Guard lock(mutex_);
    if (!accessConfig.ready()) return false;
    const bool open = !accessConfig.enabled();
    const bool basic = basic_.allowed(request);
    if (!open && !basic && !sessions_.validate(token.c_str(), millis())) return false;
    for (const auto &slot : sockets_) if (slot.id == id) return false;
    for (auto &slot : sockets_) if (slot.id == 0) {
      slot.id = id; slot.basic = basic; slot.open = open;
      // The upgrade has already transferred request->client() to the socket.
      // Only headers/auth remain owned by request during WS_EVT_CONNECT.
      slot.address=address;slot.version=clientVersion(request);
      const auto *agent=request->getHeader("User-Agent");
      slot.agent=agent?agent->value():String();
      if(slot.agent.length()>192)slot.agent=String();
      token.toCharArray(slot.token, sizeof(slot.token)); return true;
    }
    return false;
  }
  String socketAddress(uint32_t id) const {
    PakarAccessDetail::Guard lock(mutex_);for(const auto &slot:sockets_)if(slot.id==id)return slot.address;return String();
  }
  String socketAgent(uint32_t id) const {
    PakarAccessDetail::Guard lock(mutex_);for(const auto &slot:sockets_)if(slot.id==id)return slot.agent;return String();
  }
  String socketVersion(uint32_t id) const {
    PakarAccessDetail::Guard lock(mutex_);for(const auto &slot:sockets_)if(slot.id==id)return slot.version;return String();
  }
  bool socketAllowed(uint32_t id) {
    PakarAccessDetail::Guard lock(mutex_);
    for (auto &slot : sockets_) if (slot.id == id)
      return accessConfig.ready() && (slot.open ? !accessConfig.enabled() :
          (slot.basic || sessions_.validate(slot.token, millis())));
    return false;
  }
  void forgetSocket(uint32_t id) {
    PakarAccessDetail::Guard lock(mutex_);
    for (auto &slot : sockets_) if (slot.id == id) slot = Socket{};
  }
  void expireSockets(AsyncWebSocket &ws) {
    uint32_t expired[8]{}; size_t count = 0;
    {
      PakarAccessDetail::Guard lock(mutex_);
      for (auto &slot : sockets_) if (slot.id && (!accessConfig.ready() ||
          (slot.open ? accessConfig.enabled() : (!slot.basic && !sessions_.validate(slot.token, millis(), false))))) {
        expired[count++] = slot.id; slot = Socket{};
      }
    }
    // SDK close may synchronously call forgetSocket while holding its client
    // mutex. Never invert that SDK-client -> access-state lock order.
    for (size_t i = 0; i < count; ++i) ws.close(expired[i], 1008, "Session expired");
  }
  void registerRoutes(AsyncWebServer &server, std::function<void()> credentialsChanged,
      std::function<bool()> busy) {
    // SDK defaults apply even to middleware rejections, errors, upgrades and
    // downloads, not just selected JSON routes. Configure once before begin().
    DefaultHeaders::Instance().addHeader("Server",PakarPeer::product());
    DefaultHeaders::Instance().addHeader("X-Pakar-Version",BoardCore::BuildInfo::kVersion);
    DefaultHeaders::Instance().addHeader("X-Pakar-Board-Version",BoardCore::BuildInfo::kVersion);
    DefaultHeaders::Instance().addHeader("X-Pakar-Board",boardId_);
    DefaultHeaders::Instance().addHeader("X-Pakar-Boot",bootId_);
    DefaultHeaders::Instance().addHeader("X-Pakar-Client","pakarcontroller-firmware");
    DefaultHeaders::Instance().addHeader("X-Pakar-Application","PakarController");
    DefaultHeaders::Instance().addHeader("X-Pakar-Min-App-Version",BoardCore::PeerCompatibility::minimumApp);
    DefaultHeaders::Instance().addHeader("X-Pakar-Version-Check",BoardCore::PeerCompatibility::enforced?"true":"false");
    refreshCredentials(); server.addMiddleware(&middleware_);
    server.on("/api/peer/info",HTTP_GET,[this](AsyncWebServerRequest *request){
      String board,boot,hostname;
      {PakarAccessDetail::Guard lock(mutex_);board=boardId_;boot=bootId_;hostname=hostname_;}
      String body=PakarPeer::info(board,boot,hostname);
      if(peerInfoDecorator_)body=peerInfoDecorator_(body);
      auto *response=request->beginResponse(200,"application/json",body);
      response->addHeader("Cache-Control","no-store");request->send(response);
    });
    server.on("/api/auth/info", HTTP_GET, [this](AsyncWebServerRequest *request) {
      String body;
      {
        PakarAccessDetail::Guard lock(mutex_);
        body = F("{\"ready\":"); body += accessConfig.ready() ? F("true") : F("false");
        body += F(",\"enabled\":"); body += accessConfig.enabled() ? F("true") : F("false");
        body += F(",\"username\":\""); body += accessConfig.username();
        body += F("\",\"default_credentials\":"); body += accessConfig.enabled() && accessConfig.defaultsActive() ? F("true") : F("false");
        body += F(",\"login\":\"/login\",\"settings\":\"/api/auth\"}");
      }
      auto response = request->beginResponse(200, "application/json", body);
      response->addHeader("Cache-Control", "no-store"); request->send(response);
    });
    server.on("/login", HTTP_POST, [this](AsyncWebServerRequest *request) {
      if (!sameOrigin(request)) { request->send(403, "text/plain", F("Cross-origin login rejected")); return; }
      {
        PakarAccessDetail::Guard lock(mutex_);
        if (accessConfig.ready() && !accessConfig.enabled()) {
          request->send(200, "application/json", F("{\"authenticated\":true,\"enabled\":false}")); return;
        }
      }
      char token[33]{}; String error;
      const int status = createSession(request, token, error);
      if (status != 200) { request->send(status, "text/plain", error); return; }
      auto response = request->beginResponse(200, "application/json", "{\"authenticated\":true}");
      response->addHeader("Set-Cookie", String(F("PakarSession=")) + token + F("; Path=/; HttpOnly; SameSite=Strict; Max-Age=1800"));
      response->addHeader("Cache-Control", "no-store"); request->send(response);
    });
    server.on("/logout", HTTP_POST, [this, credentialsChanged](AsyncWebServerRequest *request) {
      {
        PakarAccessDetail::Guard lock(mutex_);
        sessions_.revoke(cookieToken(request).c_str());
        // All transports are closed below; revoke their queued admission first.
        for (auto &slot : sockets_) slot = Socket{};
      }
      credentialsChanged(); // close established transports; other sessions may reconnect
      auto response = request->beginResponse(200, "application/json", "{\"signed_out\":true}");
      response->addHeader("Set-Cookie", "PakarSession=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0");
      response->addHeader("Cache-Control", "no-store"); request->send(response);
    });
    server.on("/api/auth", HTTP_GET, [this](AsyncWebServerRequest *request) {
      String body;
      { PakarAccessDetail::Guard lock(mutex_);
        body = String(F("{\"username\":\"")) + accessConfig.username() + F("\",\"password_set\":true,\"enabled\":") +
            (accessConfig.enabled() ? F("true}") : F("false}")); }
      request->send(200, "application/json", body);
    });
    server.on("/api/auth", HTTP_POST, [this, credentialsChanged, busy](AsyncWebServerRequest *request) {
      if (busy()) { request->send(409, "text/plain", F("Finish the current firmware operation before changing credentials")); return; }
      String error;
      bool enabled = false;
      {
        PakarAccessDetail::Guard lock(mutex_);
        const String flag = request->arg("enabled");
        if (request->hasArg("enabled") && flag != "true" && flag != "false") error = F("enabled must be true or false");
        else if (!accessConfig.configure(request->arg("current_password"),
            request->hasArg("enabled") ? flag == "true" : accessConfig.enabled(),
            request->arg("username"), request->arg("password"))) error = accessConfig.error();
        else {
          enabled = accessConfig.enabled();
          sessions_.clear(); for (auto &slot : sockets_) slot = Socket{};
          refreshCredentialsLocked();
        }
      }
      if (error.length()) { request->send(400, "text/plain", error); return; }
      credentialsChanged();
      request->send(200, "application/json", enabled ?
          F("{\"updated\":true,\"enabled\":true,\"reauthenticate\":true}") :
          F("{\"updated\":true,\"enabled\":false,\"reauthenticate\":false}"));
    });
  }
 private:
  bool ready(String &error) const {
    PakarAccessDetail::Guard lock(mutex_);
    if (!accessConfig.ready()) { error = accessConfig.error(); return false; }
    return true;
  }
  void refreshCredentialsLocked() {
    basic_.setAuthType(AsyncAuthType::AUTH_BASIC);
    basic_.setUsername(accessConfig.username()); basic_.setPassword(accessConfig.password());
    basic_.setRealm("PakarController");
  }
  int createSession(AsyncWebServerRequest *request, char (&token)[33], String &error) {
    PakarAccessDetail::Guard lock(mutex_);
    if (!accessConfig.ready()) { error = accessConfig.error(); return 503; }
    const uint32_t now = millis();
    if (failures_ >= 5 && static_cast<uint32_t>(now - failedAt_) < 15000UL) {
      error = F("Too many login attempts; retry in 15 seconds"); return 429;
    }
    if (!accessConfig.verify(request->arg("username").c_str(), request->arg("password").c_str())) {
      if (static_cast<uint32_t>(now - failedAt_) >= 15000UL) failures_ = 0;
      ++failures_; failedAt_ = now; error = F("Incorrect username or password"); return 401;
    }
    failures_ = 0;
    uint8_t bytes[16];
    if (SystemPlatform::randomBytes(bytes, sizeof(bytes)) != 0) { error = F("Session entropy unavailable"); return 503; }
    if (!BoardCore::DiagnosticText::encodeHex(bytes, sizeof(bytes), token, sizeof(token))) {
      error = F("Session encoding failed"); return 503;
    }
    if (!sessions_.add(token, now)) { error = F("Session limit reached; sign out another client"); return 503; }
    return 200;
  }
  static String cookieToken(AsyncWebServerRequest *request) {
    if (!request->hasHeader("Cookie")) return String();
    String cookies = request->getHeader("Cookie")->value();
    while (cookies.length()) {
      const int separator = cookies.indexOf(';');
      String item = separator < 0 ? cookies : cookies.substring(0, separator); item.trim();
      if (item.startsWith(F("PakarSession="))) return item.substring(13);
      if (separator < 0) break;
      cookies.remove(0, separator + 1);
    }
    return String();
  }
  AsyncAuthenticationMiddleware basic_;
  std::function<String(const String&)> peerInfoDecorator_;
  mutable PakarAccessDetail::Mutex mutex_;
  String boardId_, bootId_,hostname_;
  AsyncMiddlewareFunction middleware_;
  BoardCore::Access::Sessions sessions_;
  uint8_t failures_ = 0;
  uint32_t failedAt_ = 0;
  struct Socket { uint32_t id = 0; bool basic = false; bool open = false; char token[33] = {};String address,agent,version; } sockets_[8];
};
