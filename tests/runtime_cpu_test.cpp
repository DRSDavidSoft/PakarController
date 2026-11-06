#include "../firmware/esp/src/CpuLoadWindow.h"
#include <cassert>
#include <cmath>
int main() {
  PakarCpu::Window w;uint32_t idle[2]={100,200};
  assert(!w.capture(0,0,idle,true,4).valid);
  idle[0]+=2500000;idle[1]+=1250000;
  auto s=w.capture(5000000,5000,idle,true,5);
  assert(s.valid&&s.windowMs==5000&&s.tasks==5);
  assert(std::abs(s.busy[0]-50)<0.001&&std::abs(s.busy[1]-75)<0.001);
  assert(!w.capture(10000000,10000,idle,false,0).valid);
  assert(!w.capture(15000000,15000,idle,true,5).valid);
  assert(!w.capture(80000000,80000,idle,true,5).valid);
  assert(!w.capture(10,1,idle,true,5).valid);
  PakarCpu::Window wrap;uint32_t before[2]={0xffffff00,0xffffff00};
  wrap.capture(1000000,1000,before,true,4);
  uint32_t after[2]={uint32_t(before[0]+2500000),uint32_t(before[1]+5000000)};
  s=wrap.capture(6000000,6000,after,true,4);
  assert(s.valid&&s.busy[0]==50&&s.busy[1]==0);
  after[0]-=1;assert(!wrap.capture(11000000,11000,after,true,4).valid);
}
