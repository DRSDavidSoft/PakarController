#define PAKAR_FTP_ENABLED 1
#include "UserFileStore.h"
#include <cassert>
#include <deque>
#include <map>
#include <vector>
#include <algorithm>
#include <cerrno>
#include <cstdio>
#include "../tests/ssh_stubs/Preferences.h"
#include "../firmware/esp32s3/src/FtpSession.h"
#include <sys/socket.h>
struct Digest {uint32_t value=0;void begin(){value=0;}void add(const void* b,size_t n){auto p=static_cast<const uint8_t*>(b);for(size_t i=0;i<n;++i)value=value*33+p[i];}String hex()const{char s[65];snprintf(s,sizeof(s),"%064x",value);return s;}};
struct Socket {std::deque<uint8_t> in;std::string out;bool eof=false,reset=false;unsigned ip=1;};
std::map<int,Socket> sockets;std::map<unsigned,std::deque<int>> accepts;
int send(int fd,const void* p,size_t n,int){if(!sockets.count(fd)){errno=EBADF;return -1;}n=std::min<size_t>(n,7);sockets[fd].out.append(static_cast<const char*>(p),n);return int(n);}
int recv(int fd,void* p,size_t n,int){if(!sockets.count(fd)){errno=EBADF;return -1;}auto&s=sockets[fd];if(s.reset){errno=ECONNRESET;return -1;}n=std::min(n,s.in.size());if(!n){if(s.eof)return 0;errno=EAGAIN;return -1;}for(size_t i=0;i<n;++i){static_cast<uint8_t*>(p)[i]=s.in.front();s.in.pop_front();}return int(n);}
struct IPAddress {unsigned value=1;unsigned operator[](unsigned i)const{return i==3?value:192;}bool operator==(const IPAddress&b)const{return value==b.value;}};
class WiFiClient {int id_=-1;public:WiFiClient()=default;explicit WiFiClient(int n):id_(n){};int fd()const{return id_;}explicit operator bool()const{return id_>=0;}void stop(){id_=-1;}void setNoDelay(bool){}IPAddress localIP()const{return {};}IPAddress remoteIP()const{return {sockets[id_].ip};}};
class WiFiServer {unsigned port_=0;bool open_=false;public:void begin(unsigned port){port_=port;open_=true;}void stop(){open_=false;}explicit operator bool()const{return open_;}WiFiClient accept(){if(!open_||accepts[port_].empty())return {};const int n=accepts[port_].front();accepts[port_].pop_front();return WiFiClient(n);}};
static std::map<std::string,std::vector<uint8_t>> nvs;
bool Preferences::begin(const char*,bool){return true;}void Preferences::end(){}bool Preferences::isKey(const char*k){return nvs.count(k);}size_t Preferences::getBytesLength(const char*k){return nvs[k].size();}
size_t Preferences::getBytes(const char*k,void*p,size_t n){if(nvs[k].size()!=n)return 0;memcpy(p,nvs[k].data(),n);return n;}
size_t Preferences::putBytes(const char*k,const void*p,size_t n){nvs[k]={static_cast<const uint8_t*>(p),static_cast<const uint8_t*>(p)+n};return n;}
fs::FS volume;UserFileStore<Digest> store;uint32_t token=0;unsigned artifactReaders=0;bool flashAllowed=true,maintenance=false;
struct Access {uint32_t revision=7;bool ready()const{return true;}uint32_t securityRevision()const{return revision;}const char* username()const{return "user";}const char* password()const{return "password123";}} accessConfig;
struct ShellSettings {uint16_t port()const{return 23;}}shellSettings;
namespace NetworkPath {bool ready(){return true;}}
bool apStarted=false;bool shellBusy(){return maintenance;}bool networkApplyPending=false,cloudApplyPending=false;
bool maintenanceApiIdle(){return !maintenance&&!artifactReaders;}
String maintenanceApiError(const char* text){return String("{\"accepted\":false,\"error\":\"")+text+"\"}";}
String managedUserFileRequest(JsonObjectConst q,bool stream){if(!flashAllowed)return maintenanceApiError("led_flash_guard_busy");return store.request(q["action"]|"",q,millis(),accessConfig.revision,++token,stream);}
namespace PakarShell {bool unsignedNumber(const char*,uint32_t,uint32_t&){return false;}}
namespace PakarConfiguration {enum {Shell};struct Changes {void saved(int){}};Changes& changes(){static Changes c;return c;}}
enum {HTTP_GET,HTTP_POST};
struct Server {template<class F>void on(const char*,int,F){}void send(int,const char*,const String&){}bool hasArg(const char*){return false;}String arg(const char*){return "";}}server;
bool authenticate(){return true;}
#include "FtpBinding.h"
void turn(unsigned n=1){for(unsigned i=0;i<n;++i){serviceFileTransferCleanup();DeviceFtp::service();}}
void input(int fd,const std::string& bytes){sockets[fd].in.insert(sockets[fd].in.end(),bytes.begin(),bytes.end());}
void command(const char* line){input(1,std::string(line)+"\r\n");turn(350);}
int main(){
  store.begin(volume,true,[]()->uint32_t{return 1024*1024;});DeviceFtp::begin();assert(DeviceFtp::ready&&!DeviceFtp::settings.enabled);
  assert(!DeviceFtp::configure(true,80,50009));assert(!DeviceFtp::configure(true,21,23));assert(DeviceFtp::configure(true,21,50009));
  sockets[1]={};accepts[21].push_back(1);turn(100);assert(sockets[1].out.find("220 ")==0);
  command("USER user");command("PASS password123");assert(sockets[1].out.find("230 Logged in")!=std::string::npos);
  command("EPSV");sockets[2]={};sockets[2].ip=2;accepts[50009].push_back(2);turn();assert(DeviceFtp::data.fd()<0); // Reject foreign data peer.
  sockets[3]={};accepts[50009].push_back(3);turn();command("STOR /user/probe.bin");assert(artifactReaders==1);
  const std::string payload("\0\xff\r\nhello",9);input(3,payload);turn(2);sockets[3].eof=true;turn(350);
  assert(!artifactReaders&&sockets[1].out.find("226 Upload committed")!=std::string::npos);
  ManagedTransferFiles reader(transferFileRequest);ManagedTransferFiles::Info info;assert(reader.stat("probe.bin",info)&&info.size==payload.size());
  command("EPSV");sockets[4]={};accepts[50009].push_back(4);turn();command("RETR /user/probe.bin");turn(350);assert(sockets[4].out==payload); // Seven-byte send fragmentation.
  command("EPSV");sockets[5]={};accepts[50009].push_back(5);turn();command("STOR /user/probe.bin");input(5,"bad");turn(2);sockets[5].reset=true;turn(350);assert(!artifactReaders&&reader.stat("probe.bin",info)&&info.size==payload.size());
  command("EPSV");sockets[8]={};accepts[50009].push_back(8);turn();command("STOR /user/probe.bin");command("NOOP");assert(DeviceFtp::transferring&&DeviceFtp::direction==DeviceFtp::Session::Store);input(8,"wrong");turn(2);command("ABOR");assert(!artifactReaders&&reader.stat("probe.bin",info)&&info.size==payload.size());
  command("EPSV");sockets[6]={};accepts[50009].push_back(6);turn();command("STOR /user/private.bin");assert(artifactReaders==1);
  flashAllowed=false;++accessConfig.revision;turn();assert(artifactReaders==1&&fileTransferCancelPending);flashAllowed=true;turn();assert(!artifactReaders); // Cleanup retry after auth revocation.
  DeviceFtp::stop();assert(DeviceFtp::settings.enabled);DeviceFtp::settings.enabled=0;DeviceFtp::begin();assert(DeviceFtp::settings.enabled); // Saved enable.
  assert(reader.open("probe.bin",false));uint8_t check[16];size_t count=sizeof(check);assert(reader.read(0,check,count)&&count==payload.size()&&!memcmp(check,payload.data(),count));assert(reader.close());
  sockets[1]={};accepts[21].push_back(1);turn(100);command("USER user");command("PASS password123");command("EPSV");pakarTestMillis=15001;turn(350);assert(sockets[1].out.find("425 Data connection timed out")!=std::string::npos);
  pakarTestMillis=75002;turn();assert(DeviceFtp::control.fd()<0);
  puts("Actual FTP socket binding: persistent opt-in, same-IP passive admission, binary upload/download, partial writes, reset abort, auth revocation and deferred flash-fence cleanup passed (socket/NVS/FS doubles). No live ESP network claim.");
}
