#include "AsyncHttpService.h"
#include "StaticAssetService.h"
#include <assert.h>
#include <stdio.h>
#include <thread>
#include <atomic>
#include <vector>
// Extracted failing FINAL-FILL/SHORT-ADD branch from installed SDK3.12.1
// WebResponses.cpp (SHA12055aa687cd0f6f719b65057c7a1d203baf6f0ef88e96b3fb4909ac68d0e6f8).
// Executable SDK-algorithm model, not compilation of the complete LwIP stack.
class InstalledFinalFillResponse final : public AsyncWebServerResponse {
 public:
  size_t pending=32;
  InstalledFinalFillResponse(){_code=200;_contentLength=32;}
  void _respond(AsyncWebServerRequest *request) override {
    _sentLength+=pending; // SDK counts FILLED bytes before TCP admission.
    if(_sentLength==_contentLength)_state=RESPONSE_END;
    const String bytes(32,'A');const size_t added=request->client()->add(bytes.c_str(),pending);
    pending-=added;request->client()->send();
  }
};
int main() {
  {
    auto old=std::make_shared<AsyncWebServerRequest>();old->connection.addLimits={1};
    {AsyncSdkFixture::Actor actor;old->send(new InstalledFinalFillResponse());}
    auto *response=static_cast<InstalledFinalFillResponse*>(old->getResponse());
    assert(response->_finished()&&response->pending==31&&old->connection.sentWire.length()==1);
    old->sdkAck(1);
    assert(!old->connection.connected()&&response->pending==31);
    puts("Installed SDK final-fill trace reproduced:32 filled,1 admitted,31 pending, first ACK closed response.");
  }
  {
    AsyncHttpService fenced(80);unsigned effects=0;
    fenced.on("/mutate",HTTP_POST,[&](){++effects;fenced.send(200,"text/plain","done");});
    auto enqueue=[&](){auto r=std::make_shared<AsyncWebServerRequest>();r->path="/mutate";r->requestMethod=HTTP_POST;
      assert(fenced.native().dispatch(r.get()));return r;};
    auto before=enqueue();const auto epoch=fenced.ingressEpoch();
    fenced.setMaintenanceFence(true);assert(fenced.ingressEpoch()!=epoch);
    fenced.service();before->connection.poll();assert(before->status==409 && !effects);
    auto during=enqueue();fenced.setMaintenanceFence(false);
    fenced.service();during->connection.poll();assert(during->status==409 && !effects);
    auto after=enqueue();fenced.service();after->connection.poll();assert(after->status==200 && effects==1);
  }
  // Reproduce installed SDK3.12.1's actual bare-string/first-registration
  // collision, rather than a map lookup that always selects the exact key.
  {
    AsyncWebServer sdk(80);
    sdk.on("/api/avr/contracts",HTTP_GET,[](AsyncWebServerRequest*r){r->send(200,"text/plain","metadata");});
    sdk.on("/api/avr/contracts/result",HTTP_GET,[](AsyncWebServerRequest*r){r->send(200,"text/plain","receipt");});
    auto request=std::make_shared<AsyncWebServerRequest>();request->path="/api/avr/contracts/result";
    request->arguments["id"]="2";
    assert(sdk.dispatch(request.get()) && request->body=="metadata");
  }
  {
    AsyncHttpService exact(80);unsigned parentReads=0,parentWrites=0,childReads=0,childDeletes=0,rootReads=0;
    // Parents intentionally registered before descendants, as production is.
    exact.on("/",HTTP_GET,[&](){++rootReads;exact.send(200,"text/plain","root");});
    exact.on("/api/avr/contracts",HTTP_GET,[&](){++parentReads;exact.send(200,"text/plain","metadata");});
    exact.on("/api/avr/contracts",HTTP_POST,[&](){++parentWrites;exact.send(202,"text/plain","enqueued");});
    exact.on("/api/avr/contracts/result",HTTP_GET,[&](){++childReads;assert(exact.arg("id")=="2");exact.send(200,"text/plain","receipt2");});
    exact.on("/api/avr/contracts/result",HTTP_DELETE,[&](){++childDeletes;exact.send(200,"text/plain","deleted2");});
    auto dispatch=[&](const char*path,int method,const char*body,int status=200){
      auto request=std::make_shared<AsyncWebServerRequest>();request->path=path;request->requestMethod=method;request->arguments["id"]="2";
      assert(exact.native().dispatch(request.get()));exact.service();request->connection.poll();
      assert(request->status==status && request->body==body && request->responseStarts==1);
    };
    dispatch("/api/avr/contracts/result",HTTP_GET,"receipt2");
    dispatch("/api/avr/contracts",HTTP_GET,"metadata");
    dispatch("/api/avr/contracts",HTTP_POST,"enqueued",202);
    dispatch("/api/avr/contracts/result",HTTP_DELETE,"deleted2");
    dispatch("/",HTTP_GET,"root");
    for(const char*path:{"/api/avr/contracts/","/api/avr/contracts/result/extra","/api/avr/contracts2"}){
      auto request=std::make_shared<AsyncWebServerRequest>();request->path=path;
      assert(!exact.native().dispatch(request.get()) && request->status==404);
    }
    auto unsupported=std::make_shared<AsyncWebServerRequest>();unsupported->path="/api/avr/contracts/result";unsupported->requestMethod=HTTP_POST;
    assert(!exact.native().dispatch(unsupported.get()) && unsupported->status==404);
    assert(parentReads==1 && parentWrites==1 && childReads==1 && childDeletes==1 && rootReads==1);
  }
  AsyncHttpService server(80); int called = 0;
  server.on("/hardware", HTTP_GET, [&]() {
    ++called; server.service(); // a hardware handler may yield; don't replace current request
    server.send(200, "text/plain", "OK");
  });
  std::shared_ptr<AsyncWebServerRequest> requests[9];
  for (auto &request : requests) {
    request = std::make_shared<AsyncWebServerRequest>(); server.native().handlers["/hardware"](request.get());
  }
  assert(requests[8]->status == 503 && called == 0);
  requests[2].reset(); // disconnected queued continuation expires
  requests[3]->disconnect();
  for (int i = 0; i < 8; ++i){server.service();for(const auto&r:requests)if(r)r->connection.poll();}
  assert(called == 6 && !server.processing());
  assert(requests[0]->status == 200 && requests[3]->status == 0);
  server.on("/disconnect", HTTP_GET, [&]() {
    server.request()->disconnect(); server.send(200, "text/plain", "not sent");
  });
  auto disconnected = std::make_shared<AsyncWebServerRequest>();
  server.native().handlers["/disconnect"](disconnected.get()); server.service();
  assert(disconnected->status == 0 && !server.processing());
  auto request = std::make_shared<AsyncWebServerRequest>();
  server.native().handlers["/hardware"](request.get()); server.service();assert(request->status==0);request->connection.poll();assert(called == 7 && request->status==200);
  server.on("/", HTTP_GET, [&]() { server.send_P(200, "text/html", "embedded UI"); });
  auto root = std::make_shared<AsyncWebServerRequest>();
  server.native().handlers["/"](root.get()); server.service();assert(root->status==0);root->connection.poll();assert(root->status == 200 && root->body=="embedded UI");
  // Deterministic old failure: service ran after callback assignment but
  // before pause returned/published its weak request, clearing that callback.
  unsigned publication=0;
  server.on("/publish",HTTP_GET,[&](){++publication;server.send(200,"text/plain","OK");});
  auto publishing=std::make_shared<AsyncWebServerRequest>();publishing->onPause=[&](){server.service();};
  server.native().handlers.at("/publish")(publishing.get());server.service();publishing->connection.poll();
  assert(publication==1 && publishing->status==200);
  // Loop response publication occurs while the actual SDK-model middleware
  // callback has not returned. Its outer _send tail must still see paused.
  // The previous implementation called request.send from this other thread,
  // violating the strict SDK actor assertion and racing the outer _send tail.
  auto reentry=std::make_shared<AsyncWebServerRequest>();
  reentry->afterHandler=[&](){std::thread loop([&](){server.service();});loop.join();assert(reentry->responseStarts==0 && reentry->paused);};
  server.native().handlers.at("/publish")(reentry.get());
  assert(reentry->status==0 && reentry->responseStarts==0);
  reentry->connection.poll();assert(reentry->status==200 && reentry->responseStarts==1);
  auto *active=reentry->getResponse();assert(active);
  const auto wireBefore=reentry->connection.wire;
  reentry->connection.poll();assert(reentry->connection.wire==wireBefore);
  reentry->sdkAck(4);
  assert(!active->_finished()&&reentry->connection.alive&&reentry->connection.wire==wireBefore&&reentry->responseStarts==1);
  // Disconnect after loop publication destroys the unsent response exactly
  // once and prevents actor-side send; response construction alone is no ACK.
  auto cancelled=std::make_shared<AsyncWebServerRequest>();
  server.native().handlers.at("/publish")(cancelled.get());
  const auto beforeResponses=AsyncSdkFixture::liveResponses.load();server.service();
  assert(AsyncSdkFixture::liveResponses==beforeResponses+1 && cancelled->status==0);
  cancelled->disconnect();assert(AsyncSdkFixture::liveResponses==beforeResponses);
  cancelled->connection.poll();assert(cancelled->status==0 && cancelled->responseStarts==0);
  server.on("/duplicate",HTTP_GET,[&](){assert(server.remoteAddress()=="192.0.2.10");server.send(200,"text/plain","first");server.send(new AsyncWebServerResponse(201,"second"));});
  auto duplicate=std::make_shared<AsyncWebServerRequest>();server.native().handlers.at("/duplicate")(duplicate.get());
  server.service();duplicate->connection.poll();assert(duplicate->status==200 && duplicate->body=="first" && duplicate->responseStarts==1);
  // Larger than the installed S3 TCP send window (5744 bytes): use actual
  // admitted-byte offsets and immutable content across real add/send/ACK/poll
  // double boundaries; this is not an LwIP hardware test.
  String large;for(unsigned i=0;i<16384;++i)large+=static_cast<char>('A'+i%26);
  const String expectedLarge=large;
  server.on("/large",HTTP_GET,[&](){auto *response=server.beginResponse(206,"application/json",large);response->addHeader("Cache-Control","no-store");server.send(response);});
  auto largeRequest=std::make_shared<AsyncWebServerRequest>();
  server.native().handlers.at("/large")(largeRequest.get());server.service();large="changed after snapshot";
  assert(largeRequest->status==0);largeRequest->connection.poll();
  assert(largeRequest->status==206&&!largeRequest->body.empty()&&largeRequest->body.size()<expectedLarge.size());
  assert(largeRequest->responseHeaders.at("Cache-Control")=="no-store");
  const auto firstLarge=largeRequest->body;
  largeRequest->connection.writable=false;largeRequest->connection.poll();assert(largeRequest->body==firstLarge);
  largeRequest->sdkAck(1024);assert(largeRequest->body==firstLarge);
  largeRequest->connection.writable=true;largeRequest->connection.chunkLimit=333;
  while(largeRequest->body.size()<expectedLarge.size()){
    largeRequest->connection.poll();
    largeRequest->sdkAck(largeRequest->connection.inFlight);
  }
  assert(largeRequest->body==expectedLarge && largeRequest->responseStarts==1);
  const auto completed=largeRequest->body;largeRequest->connection.poll();assert(largeRequest->body==completed);
  largeRequest->sdkAck(largeRequest->connection.inFlight);
  assert(largeRequest->getResponse()->_finished()&&!largeRequest->connection.connected());
  const auto drain=[](const std::shared_ptr<AsyncWebServerRequest> &r,const String &expected){
    for(unsigned turns=0;r->connection.alive&&turns<40000;++turns){r->connection.poll();r->sdkAck(r->connection.inFlight);}
    assert(!r->connection.alive&&r->getResponse()->_finished()&&r->body==expected);
    assert(r->responseStarts==1);
  };
  server.on("/short",HTTP_GET,[&](){server.send(200,"text/plain","partial-and-zero-add-safe");});
  auto shortRequest=std::make_shared<AsyncWebServerRequest>();shortRequest->connection.addLimits={0,3,0,1,0,7};
  server.native().handlers.at("/short")(shortRequest.get());server.service();shortRequest->connection.poll();
  assert(shortRequest->connection.wire.empty()&&!shortRequest->getResponse()->_finished());
  drain(shortRequest,"partial-and-zero-add-safe");
  assert(shortRequest->connection.wire.find("HTTP/1.1")==0&&shortRequest->connection.wire.find("HTTP/1.1",1)==String::npos);
  for(size_t finalLimit:{size_t(0),size_t(3)}) {
    auto tail=std::make_shared<AsyncWebServerRequest>();tail->connection.addLimits={SIZE_MAX,finalLimit};
    server.native().handlers.at("/short")(tail.get());server.service();tail->connection.poll();
    assert(tail->body==String("partial-and-zero-add-safe").substr(0,finalLimit));
    assert(!tail->getResponse()->_finished());
    tail->sdkAck(tail->connection.inFlight); // Header/partial-body ACK is NOT completion.
    assert(tail->connection.alive&&!tail->getResponse()->_finished()&&tail->body=="partial-and-zero-add-safe");
    drain(tail,"partial-and-zero-add-safe");
  }
  // Failed send with a full TCP window: poll must retry output without adding
  // the same header/body bytes again or falsely reporting completion.
  auto sendFailure=std::make_shared<AsyncWebServerRequest>();sendFailure->connection.windowCapacity=32;
  sendFailure->connection.sendResults={false,false,true};
  server.native().handlers.at("/short")(sendFailure.get());server.service();sendFailure->connection.poll();
  assert(sendFailure->connection.inFlight==32&&sendFailure->connection.sentWire.empty());
  const auto heldWire=sendFailure->connection.wire;const auto heldAdds=sendFailure->connection.addCalls;
  assert(!sendFailure->connection.canSend());
  sendFailure->connection.poll();assert(sendFailure->connection.sentWire.empty());
  sendFailure->connection.poll();assert(sendFailure->connection.sentWire==heldWire);
  assert(sendFailure->connection.wire==heldWire&&sendFailure->connection.addCalls==heldAdds);
  assert(sendFailure->connection.sendCalls==3&&!sendFailure->getResponse()->_finished());
  drain(sendFailure,"partial-and-zero-add-safe");
  server.on("/empty",HTTP_GET,[&](){server.send(204,"application/json","");});
  auto empty=std::make_shared<AsyncWebServerRequest>();server.native().handlers.at("/empty")(empty.get());server.service();empty->connection.poll();
  const auto emptySent=empty->connection.inFlight,emptySends=empty->connection.sendCalls;
  assert(emptySent>1&&!empty->getResponse()->_finished()&&empty->body.empty());
  empty->sdkAck(emptySent-1);assert(empty->connection.alive&&!empty->getResponse()->_finished());
  empty->connection.poll();assert(empty->connection.sendCalls==emptySends);
  empty->sdkAck(1);assert(!empty->connection.alive&&empty->getResponse()->_finished()&&empty->body.empty());
  auto zeroWindow=std::make_shared<AsyncWebServerRequest>();zeroWindow->connection.windowCapacity=0;
  server.native().handlers.at("/short")(zeroWindow.get());server.service();zeroWindow->connection.poll();
  assert(zeroWindow->connection.wire.empty()&&!zeroWindow->getResponse()->_finished());
  zeroWindow->connection.windowCapacity=17;drain(zeroWindow,"partial-and-zero-add-safe");
  String flashUi;for(unsigned i=0;i<224*1024;++i)flashUi+=static_cast<char>('!'+i%90);
  server.on("/flash",HTTP_GET,[&](){server.send_P(200,"text/html",flashUi.c_str());});
  auto flash=std::make_shared<AsyncWebServerRequest>();flash->connection.addLimits={1,0,9,0,13};
  server.native().handlers.at("/flash")(flash.get());server.service();drain(flash,flashUi);
  // Compressed flash assets are binary and can contain NULs. They retain the
  // same admitted-byte/ACK lifecycle, including zero/short adds and failed send.
  std::vector<uint8_t> compressed(66056);for(size_t i=0;i<compressed.size();++i)compressed[i]=static_cast<uint8_t>(i);
  const String expectedCompressed(std::string(reinterpret_cast<const char*>(compressed.data()),compressed.size()));
  server.on("/gzip",HTTP_GET,[&](){server.send_P(200,"text/html",compressed.data(),compressed.size(),"gzip");});
  auto gzip=std::make_shared<AsyncWebServerRequest>();gzip->connection.addLimits={1,0,3,0,9};
  gzip->connection.sendResults={false,true};
  server.native().handlers.at("/gzip")(gzip.get());server.service();drain(gzip,expectedCompressed);
  assert(gzip->responseHeaders.at("Content-Encoding")=="gzip");
  assert(gzip->connection.sentWire.find("Content-Length: 66056\r\n")!=String::npos);
  server.on("/head",HTTP_HEAD,[&](){server.send(200,"text/plain","not-a-head-body");});
  auto head=std::make_shared<AsyncWebServerRequest>();head->requestMethod=HTTP_HEAD;
  server.native().handlers.at("/head")(head.get());server.service();drain(head,"");
  assert(head->connection.sentWire.find("Content-Length: 15\r\n")!=String::npos);
  {
    using namespace PakarStatic;
    assert(accepts(nullptr,"gzip")&&!accepts("","gzip"));
    assert(accepts("br, GZip; q=0.125","gzip"));
    assert(!accepts("gzip;q=0, *;q=1","gzip"));
    assert(!accepts("gzip;q=garbage","gzip")&&!accepts("gzip;q=1.1","gzip"));
    assert(accepts("*;q=0.2","gzip")&&!accepts("br","gzip"));
    assert(accepts("","identity")&&accepts("gzip","identity"));
    assert(!accepts("*;q=0","identity")&&accepts("*;q=0, identity;q=1","identity"));
    assert(notModified("W/\"other\", W/\"abc\"","\"abc\""));
    assert(notModified("*","\"abc\"")&&!notModified("\"abcd\"","\"abc\""));
    assert(!notModified("\"abc\"garbage","\"abc\""));
    assert(!notModified("\"abc\", malformed","\"abc\""));
    assert(!notModified("\"abc\", ","\"abc\""));
    assert(!notModified("\"bad\x01\", \"abc\"","\"abc\""));
    auto asset=[&](int method,const char *enc,const char *tag,int status,const String &expected){
      auto r=std::make_shared<AsyncWebServerRequest>();r->requestMethod=method;
      if(enc)r->headers["Accept-Encoding"].text=enc;
      if(tag)r->headers["If-None-Match"].text=tag;
      r->connection.addLimits={1,0,3,1,0,13};r->connection.sendResults={false,true};
      {AsyncSdkFixture::Actor actor;serveStaticAsset(r.get(),"text/html",compressed.data(),compressed.size(),"\"abc\"",true);}
      drain(r,expected);assert(r->status==status);
      assert(r->responseHeaders.at("Vary")=="Accept-Encoding");
      assert(r->responseHeaders.at("X-Content-Type-Options")=="nosniff");
      if(status==304)assert(r->connection.sentWire.find("Content-Length:")==String::npos);
      if(status==200)assert(r->connection.sentWire.find("Content-Length: 66056\r\n")!=String::npos);
      if(status==406)assert(!r->responseHeaders.count("Content-Encoding")&&!r->responseHeaders.count("ETag"));
    };
    asset(HTTP_GET,"gzip",nullptr,200,expectedCompressed);
    asset(HTTP_HEAD,"gzip",nullptr,200,"");
    asset(HTTP_GET,"gzip","W/\"abc\"",304,"");
    asset(HTTP_GET,"identity","\"abc\"",406,"");
    asset(HTTP_GET,"gzip;q=0",nullptr,406,"");
    puts("Static assets: negotiation, validators, HEAD/304/406 and binary short-write/ACK integrity passed.");
  }
  auto midway=std::make_shared<AsyncWebServerRequest>();midway->connection.windowCapacity=127;
  server.native().handlers.at("/flash")(midway.get());server.service();midway->connection.poll();
  assert(!midway->getResponse()->_finished());const auto midwayWire=midway->connection.wire;
  midway->disconnect();midway->connection.poll();assert(midway->connection.wire==midwayWire);
  // Deterministic starvation regression: refill the first free slot before
  // every dispatch while seven older requests are already waiting. A fixed
  // slot-zero scan serves newcomers forever; round-robin must serve the old
  // requests within the eight-slot bound, preserving weak ownership.
  {
    AsyncHttpService fair(80);
    std::vector<AsyncWebServerRequest*> order;
    fair.on("/fair",HTTP_GET,[&](){order.push_back(fair.request());fair.send(200,"text/plain","OK");});
    std::vector<std::shared_ptr<AsyncWebServerRequest>> older,newer;
    for(unsigned i=0;i<8;++i){older.push_back(std::make_shared<AsyncWebServerRequest>());fair.native().handlers.at("/fair")(older.back().get());}
    fair.service();older[0]->connection.poll();assert(order.size()==1 && order[0]==older[0].get());
    for(unsigned i=1;i<8;++i){
      newer.push_back(std::make_shared<AsyncWebServerRequest>());
      fair.native().handlers.at("/fair")(newer.back().get());
      fair.service();older[i]->connection.poll();assert(order.size()==i+1 && order[i]==older[i].get());
    }
    for(const auto&r:older)assert(r->status==200);
    for(unsigned i=0;i<8;++i)fair.service();
    for(const auto&r:newer)r->connection.poll();
    for(const auto&r:newer)assert(r->status==200);
    assert(order.size()==15 && !fair.processing());
  }
  constexpr unsigned count=20000;unsigned served=0;
  server.on("/parallel",HTTP_GET,[&](){++served;assert(server.processing());server.service();server.send(200,"text/plain","OK");});
  std::vector<std::shared_ptr<AsyncWebServerRequest>> parallel;
  for(unsigned i=0;i<count;++i)parallel.push_back(std::make_shared<AsyncWebServerRequest>());
  std::atomic<bool> done{false};
  std::thread producer([&](){for(const auto&r:parallel){server.native().handlers.at("/parallel")(r.get());r->connection.poll();std::this_thread::yield();}done.store(true,std::memory_order_release);});
  while(!done.load(std::memory_order_acquire)){server.service();std::this_thread::yield();}
  producer.join();for(unsigned i=0;i<8;++i)server.service();for(const auto&r:parallel)r->connection.poll();
  unsigned accepted=0,rejected=0;
  for(const auto&r:parallel){if(r->status==200)++accepted;else if(r->status==503)++rejected;else assert(false);}
  assert(accepted==served && accepted+rejected==count && !server.processing());
  puts("Production async queue: SDK final-fill short-add loss and first-match collision reproduced; admitted-byte snapshots, final TCP ACK, zero/partial adds, failed-send/full-window retry, empty/HEAD/large/flash responses, disconnect, actor-only ownership, fairness and20000 publications passed (transport doubles, not hardware soak or OTA-ingress proof).");
}
