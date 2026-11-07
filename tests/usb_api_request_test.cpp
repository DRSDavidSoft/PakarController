#include <UsbApiRequest.h>
#include <FilesystemInitialization.h>
#include <cassert>
#include <cstdio>
int main(){
  using Request=BoardCore::UsbApiRequest<64>;
  Request request;const uint8_t data[]={'{','}'};
  assert(!request.open(0,2,0));assert(!request.open(1,65,0));
  assert(request.open(7,2,0));assert(!request.claim(7,0));
  assert(!request.receiptPending(0));
  assert(request.append(7,1,data,1,0)==Request::Result::Invalid);
  assert(request.append(7,0,data,2,0)==Request::Result::Accepted);
  assert(request.append(7,0,data,2,0)==Request::Result::Duplicate);
  assert(request.append(7,0,data+1,1,0)==Request::Result::Invalid);
  assert(request.claim(7,1));assert(!request.claim(7,2));
  assert(request.receiptPending(2));assert(!request.receiptPending(30000));
  assert(request.append(7,0,data,2,2)==Request::Result::Invalid);
  assert(!request.live(8,2));assert(!request.live(7,30000));
  assert(request.open(9,2,UINT32_MAX-10));assert(request.live(9,10));
  request.clear();assert(!request.live(9,10));
  assert(!request.receiptPending(10));
  assert(request.open(10,2,0));const uint8_t nul[]={0};
  assert(request.append(10,0,nul,1,0)==Request::Result::Invalid);
  BoardCore::FilesystemInitialization guard;uint8_t blank[4]={255,255,255,255};
  assert(!guard.begin(4,true,true));assert(!guard.begin(4,false,false));
  assert(guard.begin(4,false,true));assert(!guard.arm(1,0));
  assert(guard.inspect(0,blank,4));assert(guard.arm(1,0));
  assert(!guard.consume(1,0,false,true,"yes"));
  assert(!guard.consume(1,0,false,true,"INITIALIZE EMPTY FILESYSTEM"));
  assert(guard.begin(4,false,true));blank[3]=0;
  assert(!guard.inspect(0,blank,4));assert(!guard.arm(1,0));blank[3]=255;
  assert(guard.begin(4,false,true));assert(guard.inspect(0,blank,4));assert(guard.arm(2,0));
  assert(!guard.consume(2,30000,false,true,"INITIALIZE EMPTY FILESYSTEM"));
  assert(guard.begin(4,false,true));assert(guard.inspect(0,blank,4));assert(guard.arm(3,0));
  assert(guard.consume(3,1,false,true,"INITIALIZE EMPTY FILESYSTEM"));
  assert(!guard.consume(3,1,false,true,"INITIALIZE EMPTY FILESYSTEM"));
  puts("USB API replay, bounds, expiry and blank-filesystem guards passed");
}
