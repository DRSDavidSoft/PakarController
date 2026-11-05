#include "UserFileStore.h"
#include "../firmware/esp32s3/src/ScpDownload.h"
#include "../firmware/esp32s3/src/SshPolicy.h"
#include <cassert>
#include <cstdio>
#define PAKAR_SCP_ENABLED 1
namespace FirmwarePlatform {
struct Sha256 {uint32_t value=0;void begin(){value=0;}void add(const void* bytes,size_t count){const auto*p=static_cast<const uint8_t*>(bytes);for(size_t i=0;i<count;++i)value=value*33+p[i];}String hex()const{char s[65];snprintf(s,sizeof(s),"%064x",value);return s;}};
}
struct {uint32_t revision=7;uint32_t securityRevision(){return revision;}} accessConfig;
namespace SystemPlatform {void randomBytes(void* out,size_t count){memset(out,1,count);}}
#include "../firmware/esp/src/UserFileService.h"
struct {
  uint32_t token=9;bool blocked=false,closed=false,finished=false;unsigned exit=99;std::string wire;
  uint32_t active(){return token;}
  bool output(uint32_t t,uint32_t r,const uint8_t* bytes,size_t count){if(blocked||!PakarS3::SshPolicy::admitted(t,token,r,accessConfig.revision))return false;wire.append(reinterpret_cast<const char*>(bytes),count);return true;}
  void close(uint32_t){closed=true;}
  void finish(uint32_t,unsigned status=0){finished=true;exit=status;}
} deviceSsh;
#include "../firmware/esp/src/SshScpBinding.h"
int main(){
  fs::FS fs;userFiles.begin(fs,true,[]()->uint32_t{return 1024*1024;});
  FirmwarePlatform::Sha256 digest;const uint8_t bytes[]={0,1,255,13,10};digest.add(bytes,sizeof(bytes));
  JsonDocument body;body["action"]="begin";body["name"]="example.bin";body["size"]=5;body["sha256"]=digest.hex();body["modified_ms"]=123ULL;body["revision"]=0;
  JsonDocument response;assert(!deserializeJson(response,userFileRequest(body.as<JsonObjectConst>()))&&response["accepted"]==true);
  const uint32_t token=response["token"];body.clear();body["action"]="write";body["token"]=token;body["offset"]=0;body["hex"]="0001ff0d0a";
  assert(!deserializeJson(response,userFileRequest(body.as<JsonObjectConst>()))&&response["accepted"]==true);
  body["action"]="commit";assert(!deserializeJson(response,userFileRequest(body.as<JsonObjectConst>()))&&response["committed"]==true);
  body["action"]="cancel";userFileRequest(body.as<JsonObjectConst>());
  const unsigned writes=fs.writes;
  sshScp.token=9;sshScp.revision=7;snprintf(sshScp.name,sizeof(sshScp.name),"example.bin");
  serviceSshScp();assert(sshScp.ready&&deviceSsh.wire.empty());
  sshScp.download.input(0);serviceSshScp();assert(deviceSsh.wire=="C0644 5 example.bin\n");
  sshScp.download.input(0);deviceSsh.blocked=true;serviceSshScp();assert(sshScp.download.sent()==0);
  deviceSsh.blocked=false;serviceSshScp();serviceSshScp();sshScp.download.input(0);serviceSshScp();
  const std::string expected=std::string("C0644 5 example.bin\n")+std::string(reinterpret_cast<const char*>(bytes),5)+std::string(1,0);
  assert(deviceSsh.wire==expected&&deviceSsh.finished&&deviceSsh.exit==0&&!deviceSsh.closed&&fs.writes==writes);
  sshScp=SshScpSession{};sshScp.token=9;sshScp.revision=7;snprintf(sshScp.name,sizeof(sshScp.name),"example.bin");serviceSshScp();assert(sshScp.ready);
  sshScp.hash=String(64,'0');sshScp.download.input(0);serviceSshScp();sshScp.download.input(0);serviceSshScp();assert(deviceSsh.closed); // Changed file cannot become a successful download.
  sshScp=SshScpSession{};sshScp.token=9;sshScp.revision=7;snprintf(sshScp.name,sizeof(sshScp.name),".settings");serviceSshScp();assert(sshScp.failed);
  serviceSshScp();assert(deviceSsh.exit==1&&fs.writes==writes);
  sshScp.token=9;sshScp.revision=7;accessConfig.revision=8;const auto before=deviceSsh.wire;serviceSshScp();assert(!sshScp.token&&deviceSsh.wire==before);
  puts("Actual SCP binding + public file store: binary bytes, unchanged-file fencing, private isolation, auth revocation, backpressure and zero-write downloads passed (transport/FS doubles).");
}
