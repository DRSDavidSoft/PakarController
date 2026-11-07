#pragma once
#include <algorithm>
#include <cstdint>
#include <cstring>
#include <string>
#include <cstdio>
#include <cctype>
struct __FlashStringHelper {};
#define F(x) x
using std::min;
extern uint32_t testNow;
inline uint32_t millis(){return testNow;}
inline void yield(){}
class String:public std::string{
 public:using std::string::string;String()=default;String(const std::string&s):std::string(s){}
 String(const __FlashStringHelper*p):std::string(reinterpret_cast<const char*>(p)){}
 bool isEmpty()const{return empty();}
 bool startsWith(const char*p)const{return rfind(p,0)==0;}
 bool endsWith(const char*p)const{const size_t n=strlen(p);return size()>=n&&compare(size()-n,n,p)==0;}
 bool reserve(size_t n){std::string::reserve(n);return true;}
 bool concat(const char*p,size_t n){append(p,n);return true;}
 explicit String(uint16_t value):std::string(std::to_string(value)){}
 int indexOf(const char*p,size_t start=0)const{auto n=find(p,start);return n==npos?-1:static_cast<int>(n);}
 int indexOf(char p,size_t start=0)const{auto n=find(p,start);return n==npos?-1:static_cast<int>(n);}
 String substring(size_t start,size_t end=npos)const{return substr(start,end==npos?npos:end-start);}
 void toLowerCase(){std::transform(begin(),end(),begin(),[](unsigned char c){return std::tolower(c);});}
 size_t write(uint8_t c){push_back(static_cast<char>(c));return 1;}
 size_t write(const uint8_t*p,size_t n){append(reinterpret_cast<const char*>(p),n);return n;}
};
