#pragma once
#include <stdint.h>

// Adapted from Orbit EspLoad.h: use elapsed microseconds and idle runtime,
// never MHz as an estimate of utilization. Pure policy for portable host tests.
namespace PakarCpu {
struct Sample {
  bool valid=false;
  uint32_t at=0,windowMs=0,tasks=0;
  float busy[2]{};
};
class Window {
 public:
  Sample capture(uint64_t us,uint32_t ms,const uint32_t idle[2],bool complete,uint32_t count) {
    Sample next;next.at=ms;next.tasks=count;
    const uint64_t elapsed=us>=previous_?us-previous_:0;
    // Rebaseline after a missing task, clock reset or long unsampled interval.
    // A 32-bit runtime counter wraps; bounded unsigned deltas remain exact.
    next.valid=ready_&&complete&&elapsed>=1000&&elapsed<=60000000;
    if(next.valid) {
      next.windowMs=uint32_t(elapsed/1000);
      for(unsigned i=0;i<2;i++) {
        const uint32_t delta=idle[i]-idle_[i];
        if(delta>elapsed){next.valid=false;break;}
        next.busy[i]=float(100.0-100.0*double(delta)/double(elapsed));
      }
    }
    ready_=complete;previous_=us;idle_[0]=idle[0];idle_[1]=idle[1];return next;
  }
 private:
  bool ready_=false;uint64_t previous_=0;uint32_t idle_[2]{};
};
} // namespace PakarCpu
