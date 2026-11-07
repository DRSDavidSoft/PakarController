#pragma once
#include <cstdint>
constexpr unsigned ESP_IPADDR_TYPE_V4=0;
struct mdns_ip_addr_t{struct {unsigned type=ESP_IPADDR_TYPE_V4;union {struct{uint32_t addr;}ip4;}u_addr;}addr{};mdns_ip_addr_t*next=nullptr;};
struct mdns_result_t{mdns_result_t*next=nullptr;const char*hostname=nullptr;uint16_t port=80;mdns_ip_addr_t*addr=nullptr;};
inline mdns_result_t *testMdnsResults=nullptr;
inline int mdns_query_ptr(const char*,const char*,uint32_t,unsigned,mdns_result_t**out){*out=testMdnsResults;return 0;}
inline void mdns_query_results_free(mdns_result_t*){}
