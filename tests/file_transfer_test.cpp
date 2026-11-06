#include "UserFileStore.h"
#include "TransferFiles.h"
#include "../firmware/esp32s3/src/ScpUpload.h"
#include "../firmware/esp32s3/src/SftpSession.h"
#include "../firmware/esp32s3/src/FtpSession.h"
#include <cassert>
#include <cstdio>
#include <vector>
#include <string>
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif
struct Digest {uint32_t value=0;void begin(){value=0;}void add(const void* bytes,size_t n){auto p=static_cast<const uint8_t*>(bytes);for(size_t i=0;i<n;++i)value=value*33+p[i];}String hex()const{char s[65];snprintf(s,sizeof(s),"%064x",value);return s;}};
fs::FS volume;UserFileStore<Digest> store;uint32_t sequence=1,security=1;
String request(JsonObjectConst body,bool stream){return store.request(body["action"]|"",body,1,security,++sequence,stream);}
using Files=TransferFiles<decltype(&request)>;
void put(std::vector<uint8_t>& p,uint32_t n){p.push_back(n>>24);p.push_back(n>>16);p.push_back(n>>8);p.push_back(n);}
void text(std::vector<uint8_t>& p,const std::string& s){put(p,s.size());p.insert(p.end(),s.begin(),s.end());}
std::vector<uint8_t> packet(uint8_t type,uint32_t id){std::vector<uint8_t> p{0,0,0,0,type};put(p,id);return p;}
void seal(std::vector<uint8_t>& p){auto n=p.size()-4;p[0]=n>>24;p[1]=n>>16;p[2]=n>>8;p[3]=n;}
int main(int argc,char** argv){
  store.begin(volume,true,[]()->uint32_t{return 1024*1024;});Files files(request);
  auto wire=[](const uint8_t* p,size_t n){const bool ok=fwrite(p,1,n,stdout)==n;fflush(stdout);return ok;};
  if(argc>1){
#ifdef _WIN32
    _setmode(_fileno(stdin),_O_BINARY);_setmode(_fileno(stdout),_O_BINARY);
#endif
    if(std::string(argv[1])=="--sftp"){
      PakarS3::SftpSession<Files> session(files);assert(session.ready());
      int c;while((c=getchar())!=EOF){while(!session.input(uint8_t(c))){session.service(wire);if(session.failed())return 2;}for(unsigned i=0;i<300;++i)session.service(wire);if(session.failed())return 3;}files.cancel();return 0;
    }
    for(int i=1;i<argc;++i)if(std::string(argv[i])=="scp -t /user/test.bin"||std::string(argv[i])=="scp -t /user/empty.bin"){
      const char* name=strstr(argv[i],"/user/")+6;PakarS3::ScpUpload upload;upload.start(name);upload.service(files,wire);
      int c;while((c=getchar())!=EOF){while(!upload.input(uint8_t(c),files))upload.service(files,wire);for(unsigned j=0;j<4;++j)upload.service(files,wire);if(upload.phase()==PakarS3::ScpUpload::Done){Files::Info info;assert(files.stat(name,info));const unsigned size=!strcmp(name,"test.bin")?513:0;assert(info.size==size);assert(files.open(name,false));uint8_t bytes[512];uint32_t offset=0;while(offset<size){size_t n=sizeof(bytes);assert(files.read(offset,bytes,n));for(size_t k=0;k<n;++k)assert(bytes[k]==uint8_t(offset+k));offset+=n;}assert(files.close());return 0;}if(upload.reported())return 4;}return 5;
    }return 6;
  }
  assert(!files.open("/settings.json",true));assert(!files.open("/user/../secret",true));
  assert(files.open("/user/hello.bin",true));uint8_t payload[]={0,255,13,10,4};assert(files.write(0,payload,sizeof(payload)));assert(!files.write(6,payload,1));assert(files.close());
  Files::Info info;assert(files.stat("hello.bin",info)&&info.size==5);Digest digest;digest.add(payload,5);assert(std::string(info.hash)==digest.hex());
  assert(files.open("hello.bin",true));assert(files.write(0,payload,1));files.cancel();assert(files.stat("hello.bin",info)&&info.size==5); // Interrupted replacement preserved.
  assert(!files.open("hello.bin",true,true));assert(files.open("hello.bin",true));++security;assert(!files.write(0,payload,1));files.cancel();
  JsonDocument q,r;q["name"]="no-digest.bin";q["size"]=0;q["revision"]=1;q["modified_ms"]=uint64_t(0);q["trustedStream"]=true;
  deserializeJson(r,store.request("begin",q.as<JsonObjectConst>(),1,security,88));assert(r["accepted"]!=true); // JSON cannot grant streaming authority.
  PakarS3::SftpSession<Files> sftp(files);assert(sftp.ready());std::vector<uint8_t> output;
  bool blocked=false;auto emit=[&](const uint8_t* p,size_t n){if(blocked)return false;output.insert(output.end(),p,p+n);return true;};
  auto send=[&](std::vector<uint8_t> p){seal(p);output.clear();for(auto c:p){unsigned budget=0;while(!sftp.input(c)){sftp.service(emit);assert(++budget<1000);}}for(unsigned i=0;i<400;++i)sftp.service(emit);assert(!sftp.failed());};
  send(packet(1,3));assert(output[4]==2); // VERSION 3.
  auto open=packet(3,1);text(open,"/user/big.bin");put(open,26);put(open,0);send(open);assert(output[4]==102);
  auto write=packet(6,2);text(write,"f");put(write,0);put(write,0);put(write,32768);for(unsigned i=0;i<32768;++i)write.push_back(uint8_t(i));send(write);assert(output[4]==101&&output[12]==0);
  auto attr=packet(10,3);text(attr,"f");put(attr,8);put(attr,1);put(attr,1700000000);send(attr);assert(output[12]==0);
  auto close=packet(4,4);text(close,"f");send(close);assert(output[12]==0);assert(files.stat("big.bin",info)&&info.size==32768&&info.modified==1700000000000ULL);
  auto readOpen=packet(3,5);text(readOpen,"/user/big.bin");put(readOpen,1);put(readOpen,0);send(readOpen);assert(output[4]==102);
  auto read=packet(5,6);text(read,"f");put(read,0);put(read,256);put(read,32768);send(read);assert(output[4]==103&&output.size()==525);for(unsigned i=0;i<512;++i)assert(output[13+i]==uint8_t(256+i));
  send(close);assert(output[12]==0);
  auto denied=packet(3,7);text(denied,"/access.bin");put(denied,26);put(denied,0);send(denied);assert(output[4]==101&&output[12]!=0);
  auto realpath=packet(16,8);text(realpath,".");seal(realpath);output.clear();for(auto c:realpath)assert(sftp.input(c));blocked=true;sftp.service(emit);assert(output.empty()&&!sftp.input(0));blocked=false;for(unsigned i=0;i<30;++i)sftp.service(emit);assert(output[4]==104);
  for(const char* header:{"C0644 -1 bad.bin\n","C0644 4294967296 bad.bin\n","C0644 131073 bad.bin\n","C0644 0 other.bin\n"}){
    PakarS3::ScpUpload bad;bad.start("bad.bin");bad.service(files,emit);for(const char* c=header;*c;++c)bad.input(*c,files);assert(bad.phase()==PakarS3::ScpUpload::Failed&&!files.opened());
  }
  PakarS3::FtpSession<Files> ftp(files);auto command=[&](const char* value){char line[256];snprintf(line,sizeof(line),"%s",value);ftp.command(line,"user","password123");return std::string(ftp.response());};
  assert(command("STOR hi.bin").find("530")==0);assert(command("USER user").find("331")==0);assert(command("PASS wrong").find("530")==0);assert(command("PASS password123").find("230")==0);
  assert(command("TYPE A").find("504")==0);assert(command("PORT 1,2,3,4,5,6").find("502")==0);assert(command("STOR ../private").find("550")==0);
  command("EPSV");assert(ftp.action==decltype(ftp)::Passive&&ftp.extended);command("STOR /user/a.bin");assert(ftp.action==decltype(ftp)::Store);assert(command("SIZE big.bin")=="213 32768\r\n");
  puts("Actual managed-store transfer adapters: atomic/interrupted upload, SHA publication, private isolation, security revocation, full 32KiB SFTP writes/short reads/timestamps, FTP auth/passive-only policy passed (FS/network doubles).");
}
