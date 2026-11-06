#pragma once
#include "CpuLoadWindow.h"
#ifndef PAKAR_RUNTIME_CPU_STATS
#define PAKAR_RUNTIME_CPU_STATS 1
#endif
#if defined(ARDUINO_ARCH_ESP32) && PAKAR_RUNTIME_CPU_STATS
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <freertos/idf_additions.h>
#include <esp_timer.h>
#endif

// Orbit EspLoad.h's five-second FreeRTOS sampling, adapted to Pakar's loop
// owner and cached telemetry. No worker, per-request task scan or heap allocation.
class RuntimeCpu {
 public:
  void poll() {
#if defined(ARDUINO_ARCH_ESP32) && PAKAR_RUNTIME_CPU_STATS && configUSE_TRACE_FACILITY && configGENERATE_RUN_TIME_STATS && CONFIG_FREERTOS_RUN_TIME_STATS_USING_ESP_TIMER
    const uint64_t us=esp_timer_get_time();
    if(attempted_&&us-lastAttempt_<5000000)return;
    attempted_=true;lastAttempt_=us;
    uint32_t total=0,idle[2]{};bool found[2]{};
    const UBaseType_t count=uxTaskGetSystemState(tasks_,40,&total);
    for(UBaseType_t i=0;i<count;i++)for(unsigned core=0;core<2;core++)
      if(tasks_[i].xHandle==xTaskGetIdleTaskHandleForCore(core)) {
        idle[core]=tasks_[i].ulRunTimeCounter;found[core]=true;
      }
    const auto next=window_.capture(us,millis(),idle,found[0]&&found[1],count);
    portENTER_CRITICAL(&mux_);cached_=next;portEXIT_CRITICAL(&mux_);
#endif
  }
  template<class Object> void append(Object out,uint32_t now) {
#if defined(ARDUINO_ARCH_ESP32) && PAKAR_RUNTIME_CPU_STATS && configUSE_TRACE_FACILITY && configGENERATE_RUN_TIME_STATS && CONFIG_FREERTOS_RUN_TIME_STATS_USING_ESP_TIMER
    PakarCpu::Sample s;portENTER_CRITICAL(&mux_);s=cached_;portEXIT_CRITICAL(&mux_);
    const bool valid=s.valid&&uint32_t(now-s.at)<20000;
    out["available"]=true;out["valid"]=valid;
    out["source"]="FreeRTOS idle runtime / esp_timer";
    out["sampled_ms"]=s.at;out["window_ms"]=s.windowMs;out["task_count"]=s.tasks;
    if(valid){out["core0_busy_pct"]=s.busy[0];out["core1_busy_pct"]=s.busy[1];}
    else {out["core0_busy_pct"]=nullptr;out["core1_busy_pct"]=nullptr;}
#else
    (void)now;out["available"]=false;out["valid"]=false;
    out["reason"]="runtime_counters_not_in_build";
#endif
  }
 private:
#if defined(ARDUINO_ARCH_ESP32) && PAKAR_RUNTIME_CPU_STATS && configUSE_TRACE_FACILITY && configGENERATE_RUN_TIME_STATS && CONFIG_FREERTOS_RUN_TIME_STATS_USING_ESP_TIMER
  TaskStatus_t tasks_[40]{};
  PakarCpu::Window window_;PakarCpu::Sample cached_;
  uint64_t lastAttempt_=0;bool attempted_=false;
  portMUX_TYPE mux_=portMUX_INITIALIZER_UNLOCKED;
#endif
};
