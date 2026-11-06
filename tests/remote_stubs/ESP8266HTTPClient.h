#pragma once
#include "ESP8266WiFi.h"
#include <map>
struct TestResponse{std::vector<uint8_t>bytes;int status=200,length=-1;String encoding,transfer,board,boot;};
struct TestRequest{String url,authorization,method="GET",body;};
extern std::map<std::string,TestResponse>testResponses;
extern std::vector<TestRequest>testRequests;
constexpr int HTTPC_DISABLE_FOLLOW_REDIRECTS=0;
class HTTPClient{
 public:
 ~HTTPClient(){if(client_){assert(WiFiClient::live.count(client_));client_->stop();}}
 void setTimeout(int){if(client_)assert(WiFiClient::live.count(client_));}
 void setConnectTimeout(int){}
 void setReuse(bool){}void useHTTP10(bool){}void setFollowRedirects(int){}
 void setUserAgent(const String &agent){userAgent_=agent;}
 bool begin(WiFiClient &c,const String&url){if(client_)assert(WiFiClient::live.count(client_));client_=&c;url_=url;auth_=String();return testResponses.count(url);}
 void collectHeaders(const char**,size_t){}
 void addHeader(const String&name,const String&value){if(name=="Authorization")auth_=value;}
 int GET(){testRequests.push_back({url_,auth_});r_=testResponses.at(url_);client_->bytes=r_.bytes;client_->at=0;return r_.status;}
 int POST(const String&body){const int result=GET();testRequests.back().method="POST";testRequests.back().body=body;return result;}
 int getSize()const{return r_.length<0?static_cast<int>(r_.bytes.size()):r_.length;}
 String header(const char*key)const{if(!strcmp(key,"X-Pakar-Board"))return r_.board;if(!strcmp(key,"X-Pakar-Boot"))return r_.boot;return strcmp(key,"Content-Encoding")==0?r_.encoding:r_.transfer;}
 WiFiClient*getStreamPtr(){return client_;}bool connected()const{return client_&&client_->at<client_->bytes.size();}
 // Actual S3 end() retains _client when the peer has already disconnected.
 // Deliberately model that branch, not a forgiving unconditional nullptr.
 void end(){if(client_){assert(WiFiClient::live.count(client_));client_->stop();}}
 private:WiFiClient*client_=nullptr;String url_,auth_,userAgent_;TestResponse r_;
};
