import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
class Element {
 constructor(tag){this.tagName=tag;this.children=[];this.attributes={};this.textContent='';}
 append(...n){this.children.push(...n);}replaceChildren(...n){this.children=[...n];}setAttribute(k,v){this.attributes[k]=v;}click(){}
}
const listeners=new Map(),timers=new Map();let id=0;
const document={visibilityState:'visible',addEventListener:(n,f)=>listeners.set(n,f),removeEventListener:n=>listeners.delete(n)};
const context=vm.createContext({document,Date,Number,Blob,URL,setTimeout:(f,ms)=>{timers.set(++id,{f,ms});return id;},clearTimeout:id=>timers.delete(id)});
const code=readFileSync(new URL('../assets/web/runtime-monitor.js',import.meta.url),'utf8');vm.runInContext(code,context);
const {view,create}=context.RuntimeMonitor;
assert.equal(view({esp:{cpu_mhz:240}}).core0,null,'MHz is not load');
assert.equal(view({esp:{cpu_usage:{valid:false,core0_busy_pct:80}}}).core0,null);
assert.equal(view({esp:{cpu_usage:{valid:true,core0_busy_pct:101}}}).core0,null);
let state={board_id:'a',boot_id:'one'},calls=[],answer={...state,esp:{cpu_usage:{available:true,valid:true,core0_busy_pct:25,core1_busy_pct:75,window_ms:5000},free_heap_bytes:65536}},resolve;
const body=new Element('section'),node=(tag,cls,text)=>{const n=new Element(tag);n.className=cls;if(text!==undefined)n.textContent=text;return n;};
const buttons=[];const monitor=create({body,node,button:(label,run)=>{const b=node('button','',label);b.onclick=run;buttons.push(b);return b;},api:async(path,opts)=>{calls.push({path,opts});return answer;},pin:()=>({...state}),snapshot:()=>state,peer:()=>({has:()=>true})});
monitor.update();assert.equal(calls.length,0,'Opening the panel does not start monitoring');
const live=body.children[0].children[1].children[0];
buttons[0].onclick();await new Promise(setImmediate);
assert.equal(calls[0].path,'/api/telemetry');assert.equal(calls[0].opts.peer.board_id,'a');
assert.equal(body.children[2].children[1].textContent,'25.0%');assert.equal(buttons[1].disabled,false);
live.checked=true;live.onchange();await new Promise(setImmediate);assert.equal([...timers.values()].filter(t=>t.ms===5000).length,1);
document.visibilityState='hidden';listeners.get('visibilitychange')();assert.equal(timers.size,0,'Background monitoring is paused');
document.visibilityState='visible';listeners.get('visibilitychange')();assert.equal(timers.size,1);
answer=new Promise(r=>resolve=r);buttons[0].onclick();state={board_id:'b',boot_id:'two'};monitor.update();resolve({board_id:'a',boot_id:'one',esp:{}});await new Promise(setImmediate);
assert.equal(body.children[2].children.length,0,'Old board response does not repaint new board');
assert.equal(buttons[1].disabled,true,'History and export reset on peer change');monitor.dispose();assert.equal(timers.size,0);assert.equal(listeners.size,0);
const main=readFileSync(new URL('../firmware/esp/application.ino',import.meta.url),'utf8'),routes=readFileSync(new URL('../firmware/esp/src/MaintenanceRoutes.h',import.meta.url),'utf8');
assert.match(main.slice(main.indexOf('void loop()')),/runtimeCpu.poll\(\)/);
assert.match(routes,/runtimeCpu.append\(e\["cpu_usage"\].to<JsonObject>\(\),now\)/);
console.log('Runtime monitor: measured vs unavailable CPU, peer-bound read, opt-in sampling, visibility pause, history and retirement passed');
