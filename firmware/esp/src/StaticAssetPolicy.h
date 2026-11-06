#pragma once
#include <string_view>

// RFC 9110 negotiation/validators, without per-token heap allocation.
namespace PakarStatic {
inline std::string_view trim(std::string_view s){
  while(!s.empty()&&(s.front()==' '||s.front()=='\t'))s.remove_prefix(1);
  while(!s.empty()&&(s.back()==' '||s.back()=='\t'))s.remove_suffix(1);
  return s;
}
inline bool equalToken(std::string_view a,std::string_view b){
  if(a.size()!=b.size())return false;
  for(size_t i=0;i<a.size();++i){const char c=a[i]>='A'&&a[i]<='Z'?a[i]+('a'-'A'):a[i];if(c!=b[i])return false;}
  return true;
}
inline int quality(std::string_view s){
  s=trim(s);if(s.empty()||(s[0]!='0'&&s[0]!='1'))return 0;
  const bool one=s[0]=='1';if(s.size()==1)return one?1000:0;
  if(s[1]!='.'||s.size()>5)return 0;
  int result=0,scale=100;
  for(size_t i=2;i<s.size();++i){if(s[i]<'0'||s[i]>'9'||(one&&s[i]!='0'))return 0;result+=(s[i]-'0')*scale;scale/=10;}
  return one?1000:result;
}
inline bool accepts(const char *header,const char *coding){
  if(!header)return true; // absent differs from an explicitly empty field
  std::string_view rest(header);int exact=-1,wildcard=-1;
  while(!rest.empty()){
    const size_t comma=rest.find(',');const auto item=trim(rest.substr(0,comma));
    const size_t semi=item.find(';');const auto token=trim(item.substr(0,semi));int q=1000;
    if(semi!=std::string_view::npos){const auto parameter=trim(item.substr(semi+1));const auto eq=parameter.find('=');q=eq!=std::string_view::npos&&equalToken(trim(parameter.substr(0,eq)),"q")?quality(parameter.substr(eq+1)):0;}
    if(equalToken(token,coding))exact=exact<0?q:(q<exact?q:exact);
    else if(token=="*")wildcard=wildcard<0?q:(q<wildcard?q:wildcard);
    if(comma==std::string_view::npos)break;
    rest.remove_prefix(comma+1);
  }
  if(exact>=0)return exact>0;
  if(equalToken(coding,"identity"))return wildcard!=0;
  return wildcard>0;
}
inline bool notModified(const char *header,const char *etag){
  if(!header)return false;
  auto rest=trim(header);if(rest=="*")return true;bool matched=false;
  while(!rest.empty()){
    rest=trim(rest);if(rest.substr(0,2)=="W/")rest.remove_prefix(2);
    if(rest.empty()||rest.front()!='"')return false;
    const auto end=rest.find('"',1);if(end==std::string_view::npos)return false;
    for(size_t i=1;i<end;++i)if(static_cast<unsigned char>(rest[i])<0x21||rest[i]==0x7f)return false;
    matched=matched||rest.substr(0,end+1)==etag;rest=trim(rest.substr(end+1));
    if(!rest.empty()&&rest.front()!=',')return false;
    if(rest.empty())break;
    rest.remove_prefix(1);
    if(trim(rest).empty())return false;
  }
  return matched;
}
}
