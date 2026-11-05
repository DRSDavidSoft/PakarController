#include "UserFileStore.h"
#include <cassert>
#include <cstdio>
// Deterministic digest double exercises transaction integrity branches. The Web
// SHA-256 is separately compared against node:crypto; firmware uses mbedTLS.
struct Digest {uint32_t value=0;void begin(){value=0;}void add(const void *bytes,size_t n){auto p=static_cast<const uint8_t*>(bytes);for(size_t i=0;i<n;++i)value=value*33+p[i];}String hex()const{char s[65];snprintf(s,sizeof(s),"%064x",value);return s;}};
using Store=UserFileStore<Digest>;
static uint32_t available=1024*1024;
static uint32_t freeSpace(){return available;}
int main(){
 assert(!PakarUploadSpace::begin(UINT32_MAX,UINT32_MAX));
 assert(PakarUploadSpace::begin(65536+8192,4096));
 assert(!PakarUploadSpace::begin(65536+8191,4096));
 assert(PakarUploadSpace::write(65536+4096,1,1));
 assert(!PakarUploadSpace::write(UINT32_MAX,UINT32_MAX,1));
 assert(!PakarUploadSpace::write(UINT32_MAX,0,0));
 fs::FS fs;Store store;store.begin(fs,true,freeSpace);uint32_t time=1,security=7,token=123;
 auto call=[&](const char *action,JsonDocument &body){JsonDocument result;assert(!deserializeJson(result,store.request(action,body.as<JsonObjectConst>(),time,security,token)));return result;};
 JsonDocument body;auto listed=call("list",body);assert(listed["accepted"]==true&&listed["files"].size()==0&&fs.writes==0);
 for(const char *path:{"../access.bin","/access.bin",".hidden","a/b","a\\b","a%2fb","x..y",""})assert(!Store::nameAllowed(path));
 Digest digest;const uint8_t bytes[]={0,1,255,13,10};digest.add(bytes,sizeof(bytes));
 body["name"]="example.bin";body["size"]=5;body["sha256"]=digest.hex();body["modified_ms"]=1720000000123ULL;body["revision"]=0;
 const auto beforeSpace=fs.writes;
 available=65536;assert(call("begin",body)["error"]=="storage_reserved_space_required"&&fs.writes==beforeSpace);
 available=1024*1024;assert(call("begin",body)["accepted"]==true);
 JsonDocument chunk;chunk["token"]=123;chunk["offset"]=0;chunk["hex"]="0001ff0d0a";
 available=65536;assert(call("write",chunk)["error"]=="storage_reserved_space_required");
 available=1024*1024;assert(call("write",chunk)["received"]==5);assert(call("write",chunk)["received"]==5);
 chunk["hex"]="0000ff0d0a";assert(call("write",chunk)["error"]=="duplicate_chunk_mismatch");
 auto committed=call("commit",chunk);assert(committed["committed"]==true&&committed["modified_ms"]==1720000000123ULL);
 const auto writes=fs.writes;assert(call("commit",chunk)["committed"]==true&&fs.writes==writes);
 call("cancel",chunk);Store reboot;reboot.begin(fs,true,freeSpace);JsonDocument empty;
 JsonDocument reopened;assert(!deserializeJson(reopened,reboot.request("list",empty.as<JsonObjectConst>(),time,security,456)));
 assert(reopened["files"][0]["modified_ms"]==1720000000123ULL&&fs.writes==writes);
 JsonDocument read;read["name"]="example.bin";read["sha256"]=digest.hex();read["offset"]=0;read["count"]=5;
 assert(call("read",read)["hex"]=="0001ff0d0a");read["sha256"]="bad";assert(call("read",read)["error"]=="file_changed");
 body["revision"]=1;assert(call("begin",body)["error"]=="explicit_replace_required");body["replace_sha256"]=digest.hex();
 token=124;chunk["token"]=token;assert(call("begin",body)["accepted"]==true);chunk["hex"]="0001ff0d0a";assert(call("write",chunk)["received"]==5);
 fs.failRename=true;assert(call("commit",chunk)["error"]=="file_publish_failed");fs.failRename=false;
 assert(call("list",empty)["revision"]==1);assert(call("commit",chunk)["committed"]==true);call("cancel",chunk);
 token=125;chunk["token"]=token;body["revision"]=2;assert(call("begin",body)["accepted"]==true);security++;assert(call("write",chunk)["error"]=="transfer_expired_or_not_owned");
 assert(call("begin",body)["accepted"]==true);time+=60001;assert(call("commit",chunk)["error"]=="transfer_expired_or_not_owned");
 // Incomplete data never publishes and unrelated private files remain untouched.
 fs.files["/access.bin"]=std::make_shared<std::string>("private-secret");assert(call("begin",body)["accepted"]==true);assert(call("commit",chunk)["error"]=="file_integrity_mismatch");
 assert(*fs.files["/access.bin"]=="private-secret");read["name"]="/access.bin";assert(call("read",read)["error"]=="file_not_found");
 *fs.files["/.user-index.0.json"]="{broken";auto before=fs.writes;assert(call("list",empty)["revision"]==1&&fs.writes==before);
 *fs.files["/.user-index.1.json"]="{broken";assert(call("list",empty)["error"]=="file_index_damaged"&&fs.writes==before);
 puts("Managed files: byte/date persistence, explicit replacement, private isolation, no-write reads, chunk replay, corruption, failure and lease fences passed.");
}
