// Orbit EspLoad/storage-telemetry presentation adapted to Pakar's peer owner.
// Uses the existing API; samples are session-only and never persisted as live.
(function(root){
  'use strict';
  function view(value){
    const e=value?.esp||{},c=e.cpu_usage||{},number=n=>Number.isFinite(n)&&n>=0?n:null;
    const percent=n=>number(n)!==null&&n<=100?n:null;
    return {board:value?.board_id,boot:value?.boot_id,cpuAvailable:c.available===true,
      core0:c.valid===true?percent(c.core0_busy_pct):null,core1:c.valid===true?percent(c.core1_busy_pct):null,
      windowMs:number(c.window_ms),free:number(e.free_heap_bytes),largest:number(e.internal_heap_largest_block_bytes),
      psram:number(e.free_psram_bytes),uptime:number(e.uptime_ms)};
  }
  function create({body,node,button,api,pin,snapshot,peer,translate=t=>t}){
    let owner='',epoch=0,disposed=false,busy=false,timer=null,history=[];
    const key=()=>{const s=snapshot();return s?.board_id&&s?.boot_id?s.board_id+':'+s.boot_id:'';};
    const tools=node('div','tools row'),live=node('input'),label=node('label'),status=node('p','status-note'),rows=node('dl','runtime-metrics');
    live.type='checkbox';label.append(live,node('span','',translate('Live · every 5 seconds')));
    status.setAttribute('role','status');
    const read=button('Read metrics',()=>void sample(),'refresh'),download=button('Export session',()=>{
      if(!history.length)return;const url=URL.createObjectURL(new Blob([JSON.stringify({board:owner,samples:history},null,2)],{type:'application/json'}));
      const a=node('a');a.href=url;a.download='device-resources.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);
    },'download');download.disabled=true;
    function stop(){if(timer!==null)clearTimeout(timer);timer=null;}
    function schedule(){stop();if(!disposed&&live.checked&&document.visibilityState!=='hidden'&&peer().has('telemetry'))timer=setTimeout(()=>void sample(),5000);}
    function draw(v){rows.replaceChildren();const bytes=n=>n===null?'—':(n/1024).toLocaleString(undefined,{maximumFractionDigits:1})+' KiB';
      for(const [name,value,pct]of [['Core 0 load',v.core0===null?'—':v.core0.toFixed(1)+'%',v.core0],['Core 1 load',v.core1===null?'—':v.core1.toFixed(1)+'%',v.core1],['Free heap',bytes(v.free)],['Largest internal block',bytes(v.largest)],['Free PSRAM',bytes(v.psram)]]){
        const dt=node('dt','muted',translate(name)),dd=node('dd','',value);if(pct!==undefined&&pct!==null){const meter=node('meter');meter.min=0;meter.max=100;meter.value=pct;meter.setAttribute('aria-label',translate(name));dd.append(meter);}rows.append(dt,dd);
      }
    }
    async function sample(){
      if(disposed||busy||!key()||!peer().has('telemetry'))return;
      stop();busy=true;read.disabled=true;const scope=key(),generation=epoch;
      try{const result=await api('/api/telemetry',{peer:pin()}),v=view(result);
        if(disposed||scope!==key()||generation!==epoch)return;
        const s=snapshot();if(v.board!==s.board_id||v.boot!==s.boot_id)throw Error('Device session changed');
        draw(v);history.push({received_at:new Date().toISOString(),...v});history=history.slice(-60);download.disabled=false;
        status.textContent=translate(!v.cpuAvailable?'CPU counters are not included in this build':v.core0===null?'CPU sampling · next sample in 5 seconds':'Connected');
      }catch(e){if(!disposed&&scope===key()&&generation===epoch)status.textContent=e.message;}
      finally{busy=false;if(!disposed){read.disabled=!key();schedule();}}
    }
    const visibility=()=>{stop();if(document.visibilityState!=='hidden')schedule();};
    live.onchange=()=>{stop();if(live.checked)void sample();};document.addEventListener('visibilitychange',visibility);
    tools.append(read,label,download);body.append(tools,status,rows);
    return {update(){const next=key();if(next!==owner){owner=next;epoch++;stop();history=[];rows.replaceChildren();download.disabled=true;status.textContent='';}read.disabled=busy||!next;},dispose(){disposed=true;epoch++;stop();document.removeEventListener('visibilitychange',visibility);}};
  }
  root.RuntimeMonitor={view,create};
})(typeof window==='undefined'?globalThis:window);
