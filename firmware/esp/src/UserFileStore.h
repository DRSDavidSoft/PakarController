#pragma once
#include <ArduinoJson.h>
#include <FS.h>
#include "UploadSpacePolicy.h"

// Loop-owned, bounded public file store. Logical names never become filesystem
// paths. Immutable content blobs + one atomic index publish bytes and timestamps
// together; private settings are not part of this namespace, even with auth off.
// Reuses PixelGroupStore's staged/readback/rename publication discipline.
template<class Digest> class UserFileStore {
 public:
  static constexpr uint32_t maxBytes=128*1024, chunkBytes=512;
  static constexpr unsigned maxFiles=32;
  using FreeSpace=uint32_t(*)();
  void begin(fs::FS &fs,bool mounted,FreeSpace probe){fs_=mounted?&fs:nullptr;freeSpace_=probe;}
  void service(uint32_t now,uint32_t security){
    if(token_&&(uint32_t(now-touched_)>60000||security!=security_))clear();
  }
  static bool nameAllowed(const String &name){
    if(!name.length()||name.length()>64||name[0]=='.')return false;
    for(size_t i=0;i<name.length();++i){const char c=name[i];
      if(!((c>='a'&&c<='z')||(c>='A'&&c<='Z')||(c>='0'&&c<='9')||c=='-'||c=='_'||c=='.'))return false;
    }
    return strstr(name.c_str(),"..")==nullptr;
  }
  static bool hashAllowed(const String &hash){
    if(hash.length()!=64)return false;
    for(size_t i=0;i<64;++i)if(!((hash[i]>='0'&&hash[i]<='9')||(hash[i]>='a'&&hash[i]<='f')))return false;
    return true;
  }
  // trustedStream is a C++ transport capability, NEVER a JSON/API option. SCP,
  // SFTP and FTP cannot supply a digest before receiving the file. They still
  // use the same readback-verified chunks and atomic index publication.
  String request(const String &action,JsonObjectConst body,uint32_t now,uint32_t security,uint32_t newToken,bool trustedStream=false){
    service(now,security);
    if(!fs_)return error("volume_unavailable");
    JsonDocument index;const char *failure=load(index);if(failure)return error(failure);
    if(action=="list"){
      JsonDocument out;out["accepted"]=true;out["revision"]=index["revision"];
      out["max_file_bytes"]=maxBytes;out["max_files"]=maxFiles;out["chunk_bytes"]=chunkBytes;
      out["private_files_accessible"]=false;out["namespace"]="user";
      auto rows=out["files"].to<JsonArray>();
      if(!body["offset"].isNull()&&!body["offset"].is<unsigned>())return error("invalid_offset");
      const unsigned offset=body["offset"]|0U;auto entries=index["files"].as<JsonArrayConst>();
      if(offset>entries.size())return error("invalid_offset");
      unsigned i=0;for(auto row:entries){if(i>=offset&&i<offset+8)rows.add(row);++i;}
      out["next_offset"]=offset+rows.size();out["more"]=offset+rows.size()<entries.size();return encode(out);
    }
    if(action=="read"){
      auto row=find(index,body["name"]|"");if(row.isNull())return error("file_not_found");
      if(!body["sha256"].is<const char*>()||body["sha256"].as<String>()!=row["sha256"].as<String>())return error("file_changed");
      if(!body["offset"].is<uint32_t>()||!body["count"].is<uint16_t>())return error("invalid_range");
      const uint32_t offset=body["offset"],count=body["count"],size=row["size"];
      if(count>chunkBytes||offset>size||count>size-offset)return error("invalid_range");
      auto file=fs_->open(blob(row["sha256"].as<String>(),row["object"].as<uint32_t>()).c_str(),"r");uint8_t bytes[chunkBytes];
      if(!file||file.size()!=size||!file.seek(offset)||file.read(bytes,count)!=count)return error("file_read_failed");
      JsonDocument out;out.set(row);out["accepted"]=true;out["offset"]=offset;out["hex"]=hex(bytes,count);return encode(out);
    }
    if(action=="begin"){
      if(token_)return error("file_transfer_busy");
      const String name=body["name"]|"",hash=trustedStream?"0000000000000000000000000000000000000000000000000000000000000000":body["sha256"]|"";
      if(!nameAllowed(name)||!hashAllowed(hash)||!body["size"].is<uint32_t>()||body["size"].as<uint32_t>()>maxBytes||
          !body["modified_ms"].is<uint64_t>()||body["modified_ms"].as<uint64_t>()>8640000000000000ULL||
          !body["revision"].is<uint32_t>()||body["revision"].as<uint32_t>()!=index["revision"].as<uint32_t>())return error("invalid_file_or_stale_index");
      auto old=find(index,name);
      if(!old.isNull()&&(!body["replace_sha256"].is<const char*>()||body["replace_sha256"].as<String>()!=old["sha256"].as<String>()))return error("explicit_replace_required");
      if(old.isNull()&&index["files"].size()>=maxFiles)return error("file_limit");
      if(!newToken)return error("token_unavailable");
      if(fs_->exists(blob(hash,newToken).c_str()))return error("token_collision_retry_begin");
      if(!freeSpace_||!PakarUploadSpace::begin(freeSpace_(),body["size"].as<uint32_t>()))return error("storage_reserved_space_required");
      if(!fs_->exists("/.user-files")&&!fs_->mkdir("/.user-files"))return error("storage_unavailable");
      auto file=fs_->open(stage(),"w");if(!file)return error("file_stage_failed");file.close();
      name_=name;hash_=hash;size_=body["size"];modified_=body["modified_ms"];revision_=body["revision"];
      token_=newToken;security_=security;touched_=now;received_=0;committed_=false;streaming_=trustedStream;digest_.begin();return progress();
    }
    if(!body["token"].is<uint32_t>()||!token_||body["token"].as<uint32_t>()!=token_)return error("transfer_expired_or_not_owned");
    touched_=now;
    if(action=="cancel"){clear();return accepted();}
    if(action=="status")return progress();
    if(action=="write"){
      if(committed_)return error("transfer_already_committed");
      const String value=body["hex"]|"";
      if(!body["offset"].is<uint32_t>()||!value.length()||value.length()>chunkBytes*2||value.length()%2)return error("invalid_chunk");
      uint8_t bytes[chunkBytes],check[chunkBytes];const size_t count=value.length()/2;const uint32_t offset=body["offset"];
      for(size_t i=0;i<count;++i){int hi=nibble(value[i*2]),lo=nibble(value[i*2+1]);if(hi<0||lo<0)return error("invalid_hex");bytes[i]=uint8_t(hi*16+lo);}
      if(offset>received_||offset>size_||count>size_-offset)return error("invalid_chunk_offset");
      auto file=fs_->open(stage(),"r+");if(!file||!file.seek(offset))return error("file_stage_failed");
      if(offset<received_){if(count>received_-offset||file.read(check,count)!=count||memcmp(bytes,check,count))return error("duplicate_chunk_mismatch");return progress();}
      if(!freeSpace_||!PakarUploadSpace::write(freeSpace_(),offset,count))return error("storage_reserved_space_required");
      if(file.write(bytes,count)!=count){file.close();clear();return error("file_write_failed");}file.flush();
      if(!file.seek(offset)||file.read(check,count)!=count||memcmp(bytes,check,count)){file.close();clear();return error("file_verify_failed");}
      digest_.add(bytes,count);received_+=count;return progress();
    }
    if(action=="commit"){
      if(committed_)return progress();
      if(streaming_){size_=received_;hash_=digest_.hex();if(body["modified_ms"].is<uint64_t>()&&body["modified_ms"].as<uint64_t>()<=8640000000000000ULL)modified_=body["modified_ms"];}
      if(received_!=size_||!hashAllowed(hash_)||digest_.hex()!=hash_)return error("file_integrity_mismatch");
      if(index["revision"].as<uint32_t>()!=revision_||revision_==UINT32_MAX)return error("file_index_changed");
      // Unique immutable object: FAT and LittleFS need not replace an open file.
      if(!blobPublished_){if(fs_->exists(blob(hash_,token_).c_str())||!fs_->rename(stage(),blob(hash_,token_).c_str()))return error("file_publish_failed");blobPublished_=true;}
      auto row=find(index,name_);String oldHash=row["sha256"]|"";uint32_t oldObject=row["object"]|0U;
      if(row.isNull())row=index["files"].as<JsonArray>().add<JsonObject>();
      row["name"]=name_;row["size"]=size_;row["sha256"]=hash_;row["modified_ms"]=modified_;row["object"]=token_;
      index["revision"]=revision_+1;
      Digest indexHash;String bytes=encode(index);if(!bytes.length())return error("index_memory_unavailable");
      indexHash.add(bytes.c_str(),bytes.length());index["checksum"]=indexHash.hex();bytes=encode(index);
      if(!bytes.length())return error("index_memory_unavailable");
      auto file=fs_->open("/.user-index.pending","w");
      if(!file||file.print(bytes)!=bytes.length())return error("index_stage_failed");
      file.close();
      file=fs_->open("/.user-index.pending","r");
      if(!file||file.size()!=bytes.length()||file.readString()!=bytes)return error("index_verify_failed");
      file.close();
      // Replace only the inactive generation. The previous slot remains valid
      // across power loss, including FAT's lack of rename-over-existing support.
      const char *slot=(revision_+1)%2?"/.user-index.1.json":"/.user-index.0.json";
      JsonDocument retired;bool retiredValid=false;
      if(fs_->exists(slot)){auto old=fs_->open(slot,"r");if(old&&old.size()<=12000&&!deserializeJson(retired,old)){
        const String checksum=retired["checksum"]|"";retired.remove("checksum");const String encoded=encode(retired);Digest digest;digest.add(encoded.c_str(),encoded.length());
        retiredValid=hashAllowed(checksum)&&digest.hex()==checksum&&retired["files"].is<JsonArray>()&&retired["files"].size()<=maxFiles;
      }}
      if(fs_->exists(slot)&&!fs_->remove(slot))return error("index_commit_failed");
      if(!fs_->rename("/.user-index.pending",slot))return error("index_commit_failed");
      committed_=true;
      // Reclaim only objects from the retired, verified generation that neither
      // the new nor previous generation references. No broad filesystem sweep.
      if(retiredValid)for(auto entry:retired["files"].as<JsonArray>()){
        const String hash=entry["sha256"]|"";const uint32_t object=entry["object"]|0U;
        if(!hashAllowed(hash)||!object)continue;
        bool used=hash==oldHash&&object==oldObject;
        for(auto live:index["files"].as<JsonArray>())if(live["sha256"].as<String>()==hash&&live["object"].as<uint32_t>()==object)used=true;
        if(!used)fs_->remove(blob(hash,object).c_str());
      }
      return progress();
    }
    return error("unsupported_file_action");
  }
 private:
  fs::FS *fs_=nullptr;FreeSpace freeSpace_=nullptr;Digest digest_;String name_,hash_;uint64_t modified_=0;
  uint32_t token_=0,security_=0,touched_=0,size_=0,received_=0,revision_=0;bool committed_=false,blobPublished_=false,streaming_=false;
  static const char *stage(){return "/.user-files/upload.pending";}
  static String blob(const String &hash,uint32_t object){char id[9];snprintf(id,sizeof(id),"%08lx",static_cast<unsigned long>(object));return String("/.user-files/")+hash+"."+id+".bin";}
  static int nibble(char c){return c>='0'&&c<='9'?c-'0':c>='a'&&c<='f'?c-'a'+10:-1;}
  static String hex(const uint8_t *bytes,size_t count){String result;result.reserve(count*2);const char *digits="0123456789abcdef";for(size_t i=0;i<count;++i){result+=digits[bytes[i]>>4];result+=digits[bytes[i]&15];}return result;}
  static String encode(const JsonDocument &value){String bytes;if(value.overflowed())return bytes;const size_t size=measureJson(value);serializeJson(value,bytes);if(bytes.length()!=size)bytes=String();return bytes;}
  static String error(const char *text){JsonDocument out;out["accepted"]=false;out["error"]=text;return encode(out);}
  static String accepted(){return String("{\"accepted\":true}");}
  String progress()const{JsonDocument out;out["accepted"]=true;out["token"]=token_;out["received"]=received_;out["size"]=size_;out["committed"]=committed_;out["sha256"]=hash_;out["modified_ms"]=modified_;out["revision"]=revision_+(committed_?1:0);return encode(out);}
  void clear(){
    if(fs_&&token_&&!committed_){if(blobPublished_)fs_->remove(blob(hash_,token_).c_str());else fs_->remove(stage());}
    token_=0;name_=String();hash_=String();received_=0;committed_=false;blobPublished_=false;
  }
  static JsonObject find(JsonDocument &index,const String &name){for(auto row:index["files"].as<JsonArray>())if(row["name"].as<String>()==name)return row.as<JsonObject>();return JsonObject();}
  const char *load(JsonDocument &index){
    String best;uint32_t revision=0;bool found=false;
    for(const char *slot:{"/.user-index.0.json","/.user-index.1.json"}){
      if(!fs_->exists(slot))continue;
      found=true;index.clear();auto file=fs_->open(slot,"r");
      if(!file||file.size()>12000||deserializeJson(index,file)||!index["revision"].is<uint32_t>())continue;
      const String expected=index["checksum"]|"";index.remove("checksum");const String bytes=encode(index);Digest digest;digest.add(bytes.c_str(),bytes.length());
      if(!hashAllowed(expected)||digest.hex()!=expected)continue;
      if(best.length()==0||index["revision"].as<uint32_t>()>revision){best=bytes;revision=index["revision"];}
    }
    index.clear();
    if(!found){index["revision"]=0;index["files"].to<JsonArray>();return nullptr;}
    if(!best.length()||deserializeJson(index,best)||!index["files"].is<JsonArray>()||index["files"].size()>maxFiles)return "file_index_damaged";
    auto rows=index["files"].as<JsonArray>();
    for(size_t i=0;i<rows.size();++i){auto row=rows[i];if(!row.is<JsonObject>()||!nameAllowed(row["name"]|"")||!hashAllowed(row["sha256"]|"")||!row["object"].is<uint32_t>()||row["object"].as<uint32_t>()==0||!row["size"].is<uint32_t>()||row["size"].as<uint32_t>()>maxBytes||!row["modified_ms"].is<uint64_t>()||row["modified_ms"].as<uint64_t>()>8640000000000000ULL)return "file_index_damaged";
      for(size_t j=0;j<i;++j)if(rows[j]["name"].as<String>()==row["name"].as<String>())return "file_index_damaged";
    }
    return nullptr;
  }
};
