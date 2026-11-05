// Compile the actual application binding. Observations and hardware owners are
// deterministic doubles; navigation/rendering must never invoke those owners.
#include <cassert>
#include <cstdio>
#include <cstring>
#include <string>
#include <set>
#include "ShellTui.h"
#include "ShellProperties.h"
#include "../libraries/BoardCore/src/StateAppearance.h"
#define F(x) x
struct String:std::string {
  using std::string::string;using std::string::operator=;
  String()=default;String(const std::string &v):std::string(v){}
  explicit String(unsigned v):std::string(std::to_string(v)){}
  bool startsWith(const char *s)const{return rfind(s,0)==0;}
};
struct DeviceShellSession {
  PakarShell::TelnetParser parser;PakarShell::ShellTui tui;
  bool unicode=true,color=true,alternateScreen=false,commandOnly=false,closing=false;String output;
  void write(const uint8_t *p,size_t n){output.append((const char*)p,n);}
  void print(const char *s){output+=s;}
};
struct Identity {const char *hostname()const{return "workbench";}} deviceIdentity;
namespace BoardCore {struct Context {enum {Table=1,Leds=2};};}
namespace LedGrid {constexpr unsigned kCells=80,kPixels=480;}
unsigned currentFeatures(){return 3;}
struct Layout {unsigned used()const{return 480;}unsigned revision()const{return 7;}
  uint8_t count(unsigned)const{return 6;}uint16_t start(unsigned id)const{return uint16_t((id-1)*6);}};
struct Grid {
  Layout layout;BoardCore::StateAppearance styles;
  Grid(){styles.states[0].rgb=0xffeecc;styles.states[1].rgb=0xd82855;styles.states[3].rgb=0x33dd99;}
  const Layout &groupLayout()const{return layout;}
  const BoardCore::StateAppearance &appearance()const{return styles;}
  uint8_t state(unsigned id)const{return uint8_t((id-1)%4);}
} grid;
#if PAKAR_FIRST_WINS_ENABLED
bool avrOnline=true,avrRemotePress=true,keyMaskApplied=true;
struct KeySettings {unsigned enabledMask=511;} keySettings;
unsigned winner=0;
#endif
bool shellBusy(){return false;}
unsigned writes=0,commands=0;String lastKey,lastValue;
String shellGet(const ShellProperty &p){
  if(p.secret)return "hidden-fixture-secret";
  if(!strcmp(p.key,"hostname"))return "workbench";
  if(!strcmp(p.key,"label"))return "Studio controller";
  if(!strcmp(p.key,"led.brightness"))return "128";
  if(!strcmp(p.key,"led.mode"))return "effects";
  if(!strcmp(p.key,"network.mode"))return "both";
  if(!strcmp(p.key,"network.ssid"))return "Studio";
  if(!strcmp(p.key,"version"))return "source fixture";
  if(!strcmp(p.key,"uptime"))return "86400";
  if(!strcmp(p.key,"heap"))return "120000";
  return p.type=='b'?"true":p.type=='n'?"10":"Observed";
}
bool shellSet(const char *k,const char *v,String &result){++writes;lastKey=k;lastValue=v;result="Owner accepted";return true;}
struct JsonDocument {struct Value {template<class T>Value&operator=(T){return *this;}};
  Value operator[](const char*){return {};}};
void serializeJson(JsonDocument&,String &s){s="fixture";}
bool handleCloudCommand(const String&,String &result){++commands;result="Owner accepted";return true;}
void shellPrompt(DeviceShellSession&){}
#include "ShellTuiBinding.h"
#ifdef PS_RENDER_FIXTURE
static void capture(DeviceShellSession &s,const char *prefix,const char *name){
  s.output.clear();shellTuiDraw(s,true);
  const auto n=s.output.size();const std::string path=std::string(prefix)+name+".ansi";
  FILE *f=fopen(path.c_str(),"wb");assert(f);assert(fwrite(s.output.data(),1,n,f)==n);assert(!fclose(f));
}
#endif
int main(int argc,char **argv){
  (void)argc;(void)argv;using V=PakarShell::ShellTui;
  DeviceShellSession s;s.parser.size(96,24);s.tui.begin();
  auto input=[&](const char *keys){s.output.clear();for(;*keys;++keys)shellTuiInput(s,uint8_t(*keys),false);};
  std::set<unsigned> visited;
  for(unsigned i=0;i<V::PageCount+2;++i){
    ShellTuiSnapshot snapshot(s);visited.insert(s.tui.page());
    for(unsigned j=0;j<snapshot.view.rowCount;++j){const auto &r=snapshot.rows[j];assert(r.label&&r.key&&r.value);
      if(r.secret)assert(r.kind==V::Text);}
    s.output.clear();shellTuiDraw(s,true);assert(s.output.size()<PS_DEVICE_VIEW_FRAME_BYTES);
    assert(s.output.find("hidden-fixture-secret")==std::string::npos);
#ifdef PS_RENDER_FIXTURE
    if(argc==2){char page[20];snprintf(page,sizeof(page),"page-%u",unsigned(s.tui.page()));capture(s,argv[1],page);}
#endif
    input("\t");assert(writes==0&&commands==0);
  }
  assert(visited.count(V::Device)&&visited.count(V::Network)&&visited.count(V::Led)&&visited.count(V::Pixel)&&visited.count(V::TerminalPage)&&visited.count(V::Cloud)&&visited.count(V::Notifications)&&visited.count(V::Clock));
#if PAKAR_FIRST_WINS_ENABLED
  assert(visited.count(V::Game));
#endif
  while(s.tui.page()!=V::Led)input("\t");
  {ShellTuiSnapshot matrix(s);assert(matrix.view.cellStaggered);
    for(unsigned i=0;i<80;++i)assert(matrix.cellColors[i]==grid.appearance().states[grid.state(i+1)].rgb);}
  input("f ");assert(writes==1&&lastKey=="led.enabled"&&lastValue=="false"); // Actual typed row and routed Action.
  input("\x1b[B\x1b[B\r\x15" "200");assert(writes==1&&s.tui.editing());
  s.output.clear();shellTuiDraw(s,true);assert(s.output.find("\x1b[10;34H")!=std::string::npos);
  assert(s.output.find("LIGHTING")!=std::string::npos&&s.output.find("LIGHTING")==s.output.rfind("LIGHTING"));
  assert(s.output.find("Tab pages")==std::string::npos); // Inline editor alone owns keys.
#ifdef PS_RENDER_FIXTURE
  if(argc==2)capture(s,argv[1],"inline");
#endif
  input("\r");assert(writes==2&&lastKey=="led.brightness"&&lastValue=="200");
  input("\x1bOQ");assert(s.tui.editing());
#ifdef PS_RENDER_FIXTURE
  if(argc==2)capture(s,argv[1],"dialog");
#endif
  input("\x1b");shellTuiInput(s,0,true);assert(writes==2);
  while(s.tui.page()!=V::TerminalPage)input("\t");
  input("\x1b[B\x1b[B\x1b[B\x1b[B\x1b[B");
  input(" ");assert(s.tui.theme()==V::Violet&&writes==2);
  input(" ");assert(s.tui.theme()==V::Red);
#ifdef PS_RENDER_FIXTURE
  if(argc==2){s.tui.setTheme(V::Cyan);capture(s,argv[1],"cyan");s.tui.setTheme(V::OneDark);capture(s,argv[1],"onedark");}
#endif
  s.output.clear();shellTuiDraw(s,true);const auto frame=s.output;
  shellTuiDraw(s,true);assert(s.output==frame); // Backpressure defers a whole frame.
  s.alternateScreen=true;s.output.clear();input("\x1b");shellTuiInput(s,0,true);
  assert(!s.tui.active()&&!s.alternateScreen);
  assert(s.output.find("\x1b[?1002l\x1b[?1006l")<s.output.find("\x1b[?1049l"));
  s.tui.begin();s.output.clear();s.color=false;s.unicode=false;shellTuiDraw(s,true);
  assert(s.output.find('\x1b')==std::string::npos);
  std::printf("Actual workspace binding: %u pages, inline/typed/theme routes; view=%u, row=%u bytes; fixture only\n",unsigned(visited.size()),unsigned(sizeof(V)),unsigned(sizeof(V::Row)));
}
