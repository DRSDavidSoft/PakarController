#pragma once
#include <stdint.h>
#include <stddef.h>
#include <string.h>

namespace BoardCore {
// Extends ShellWire's exact-duplicate admission to bounded API transactions.
// One request per random, boot-bound session. Execution is claimed BEFORE the
// handler runs. A lost reply can be read again, never executed again.
template<size_t Capacity=4096> class UsbApiRequest {
 public:
  enum class Result { Accepted, Duplicate, Invalid, Expired };
  bool open(uint32_t token,uint16_t total,uint32_t now) {
    if(!token||!total||total>Capacity)return false;
    clear();token_=token;total_=total;opened_=now;return true;
  }
  void clear() { memset(bytes_,0,sizeof(bytes_));token_=0;total_=used_=0;claimed_=false; }
  bool live(uint32_t token,uint32_t now)const {
    return token_ && token==token_ && uint32_t(now-opened_)<30000U;
  }
  Result append(uint32_t token,uint16_t offset,const uint8_t *data,size_t size,uint32_t now) {
    if(!live(token,now))return Result::Expired;
    if(claimed_||!data||!size||size>41||offset>total_||size>size_t(total_-offset))return Result::Invalid;
    if(offset<used_)return size<=size_t(used_-offset)&&!memcmp(bytes_+offset,data,size)?Result::Duplicate:Result::Invalid;
    if(offset!=used_||memchr(data,0,size))return Result::Invalid;
    memcpy(bytes_+used_,data,size);used_+=size;bytes_[used_]=0;return Result::Accepted;
  }
  bool claim(uint32_t token,uint32_t now) {
    if(!live(token,now)||claimed_||used_!=total_)return false;
    claimed_=true;return true;
  }
  bool claimed()const{return claimed_;}
  bool receiptPending(uint32_t now)const{return claimed_&&live(token_,now);}
  uint32_t token()const{return token_;}
  uint16_t used()const{return used_;}
  const char *text()const{return reinterpret_cast<const char*>(bytes_);}
 private:
  uint8_t bytes_[Capacity+1]{};
  uint32_t token_=0,opened_=0;
  uint16_t total_=0,used_=0;
  bool claimed_=false;
};
}
