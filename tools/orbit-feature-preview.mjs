// Isolated, loopback-only component previews. No board endpoints or forwarding.
import http from 'node:http';
import {readFileSync} from 'node:fs';
const root=new URL('../',import.meta.url),read=p=>readFileSync(new URL(p,root),'utf8');
const orbit=new URL('../repos/orbit/',root),orbitRead=p=>readFileSync(new URL(p,orbit),'utf8');
const style=read('web/index.html').match(/<style>([\s\S]*?)<\/style>/)[1];
const base=(title,css,script)=>`<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>${title} · isolated preview</title><style>${css}body{display:block;padding:24px}main{display:block;width:100%;max-width:920px;margin:auto}h1{font-size:24px}.card{padding:20px;margin-top:20px}.actions,.tools{display:flex;gap:12px;flex-wrap:wrap;align-items:center}label{display:inline-flex;gap:8px;align-items:center}</style><body class="studio"><main><p>ISOLATED COMPONENT PREVIEW · SAMPLE DATA · NO BOARD</p><h1>${title}</h1><section id="preview" class="card"></section></main><script>${script}</script></body></html>`;
const shared=`const node=(tag,cls,text)=>{const n=document.createElement(tag);if(cls)n.className=cls;if(text!==undefined)n.textContent=text;return n;};`;
const cpu=base('CPU & memory',style,read('web/modules/components.js')+'\n'+read('assets/web/runtime-monitor.js')+'\n'+shared+`
const state={board_id:'fixture',boot_id:'fixture'},answer={...state,esp:{cpu_usage:{available:true,valid:true,core0_busy_pct:23.7,core1_busy_pct:62.4,window_ms:5000},free_heap_bytes:75124,internal_heap_largest_block_bytes:30720,free_psram_bytes:8316928}};
const monitor=RuntimeMonitor.create({body:document.getElementById('preview'),node,button:(title,fn,icon)=>WebUI.button(title,fn,{icon}),api:async()=>answer,pin:()=>state,snapshot:()=>state,peer:()=>({has:()=>true})});monitor.update();document.querySelector('#preview button').click();`);
const remote=orbitRead('firmware/web/remote-tools.js').split('// END PAKAR FILE BROWSER')[0];
const orbitCss=[...orbitRead('firmware/modem_bridge/WebUi.h').matchAll(/<style>([\s\S]*?)<\/style>/g)].map(m=>m[1]).join('\n');
const files=base('Orbit · Files',orbitCss,remote+'\n'+shared+`
const host=document.getElementById('preview'),toolbar=node('div','actions'),refresh=node('button','','Reload'),upload=node('button','','Upload'),picker=node('input');picker.type='file';toolbar.append(refresh,picker,upload);
OrbitFileBrowser.create({document,host,toolbar,scope:()=> 'fixture',locale:()=>document.documentElement.lang,list:async()=>({files:[{name:'commissioning-notes.txt',bytes:1832,modified_unix:1793850000},{name:'network-report.json',bytes:21402,modified_unix:1793840000}]}),picker,uploadButton:upload,upload:async()=>false,download:async()=>{},remove:async()=>{}});refresh.click();`);
const server=http.createServer((req,res)=>{const html=req.url==='/orbit'?files:req.url==='/pakar'?cpu:null;if(!html){res.writeHead(404).end();return;}res.writeHead(200,{'Content-Type':'text/html; charset=utf-8','Cache-Control':'no-store'}).end(html);});
server.listen(0,'127.0.0.1',()=>console.log('Isolated feature previews: http://127.0.0.1:'+server.address().port+'/{orbit,pakar}'));
