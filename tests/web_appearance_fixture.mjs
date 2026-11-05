// Serve the actual embedded HTML against a visibly labeled local test peer.
// Read-only unless --interactive is explicitly requested. Even then all
// state is memory-only; there is no board connection, proxy or disk log.
import http from 'node:http';
import {createHash} from 'node:crypto';
import {readFile} from 'node:fs/promises';
import {toolRelease,toolPolicy} from '../tools/peer-identity.mjs';
const webSourceUrl=new URL('../web/index.html',import.meta.url);
// --app selects richer simulated telemetry, never another UI implementation.
const appFixture=process.argv.includes('--app')||process.argv.includes('--interactive');
const interactiveFixture=process.argv.includes('--interactive');
const started=Date.now();
const status={hostname:'SIMULATED WEB FIXTURE — NOT A BOARD',canonical_url:'loopback visual test only',version:'0.2.8',
  mode:'cell',brightness:255,effect:'none',pixels:480,avr_online:false,tm1637_ack:false,tm1637_segments:[6,91,79,102],
  tm1637_host_active:false,link_mode:'busy-wire',websocket_clients:0,station_enabled:false,station_connected:false,
  ap_enabled:false,ap_started:false,mdns_started:false,ssdp_started:false,netbios_started:false,ntp_synced:false,
  esp_uart_errors:0,avr_uart_errors:0,usb_serial_errors:0,mqtt_enabled:false,cloud_websocket_enabled:false,
  active_outputs:0,audio_cues:[],cells:Array.from({length:80},(_,i)=>i===11?'red':i===25?'off':'white')};
const values={
  '/api/auth/info':{ready:true,enabled:false,username:'fixture',default_credentials:false},
  '/api/status':status,
  '/api/cells/mapping':{physical:Array.from({length:80},(_,i)=>i+1),names:Array(80).fill('')},
  '/api/device/name':{hostname:'simulated-web-fixture'},
  '/api/network':{station_enabled:false,ap_enabled:true,station_ssid:'Fixture only'},
  '/api/cloud':{mqtt_enabled:false,mqtt_tls:true,mqtt_host:'mqtt.digitalogic.ir',mqtt_port:8883,mqtt_user:'fixture',mqtt_base_topic:'fixture',mqtt_password_set:false,websocket_enabled:false,websocket_tls:true,websocket_host:'',websocket_port:443,websocket_path:'/',websocket_token_set:false,telemetry_seconds:30},
  '/api/firmware/remote':{job_id:0,boot_id:'SIMULATED',targets:3,manifest_sha256:'',phase:'idle',busy:false,cancellable:false},
};
const contextProfile=process.argv.includes('--controller')?'controller':process.argv.includes('--unacknowledged')?'unknown':'table';
const icon=(await readFile(new URL('../assets/branding/app-icon.png.base64',import.meta.url),'utf8')).trim();
const application={id:'simulated.'+contextProfile,name:contextProfile==='table'?'Pakar Table':'Pakar Keys',title:contextProfile==='table'?'Table controller':'Keypad controller',icon_png_base64:icon};
values['/api/device/name']={hostname:'simulated-web-fixture',display_name:'SIMULATED '+contextProfile.toUpperCase()+' — NOT A BOARD',application,persisted:true,schema:1};
if(contextProfile!=='unknown')status.capabilities={name:'SIMULATED '+contextProfile.toUpperCase()+' — NOT A BOARD',board_kind:contextProfile,
  hostname:'simulated-web-fixture',application,
  acknowledged_features:contextProfile==='table'?['network','telemetry','identity','table','leds','streaming']:['network','telemetry','identity','buzzer'],
  ...(contextProfile==='table'?{table:{columns:10,rows:8,pixels_per_cell:6},modes:[{id:'cell',code:1,label:'Cell selection',kind:'presentation'},{id:'snake',code:3,label:'Snake',kind:'presentation'}]}:{})};
if(contextProfile==='controller'){status.avr_online=true;status.audio_cues=['welcome'];}
if(appFixture){
  Object.assign(status,{board_id:'simulated-web-fixture-'+contextProfile,boot_id:'a'.repeat(32),http_peer_fencing:true,command_clock_ms:100,
    simulated:true,fixture_read_only:!interactiveFixture,free_heap:178240,uptime:60,winner:0,
    led_output:{enabled:true,mode:'static',diagnostic:{revision:1,active:false,index:0,count:1,rgb:65280,off:false}},
    key_game:{enabled_mask:15,idle_rgb:16777215,win_rgb:65280,lose_rgb:16711680,extended_effects:true,auto_reset_ms:0}});
  if(status.capabilities){
    status.capabilities.role=contextProfile==='table'?'table-selection':'first-wins';
    status.capabilities.acknowledged_features.push('pixel-diagnostic','pixel-corrections','state-presentation');
    if(contextProfile==='controller'){status.pixels=216;status.capabilities.acknowledged_features.push('leds','game-press','key-settings','reset-ack');}
    if(interactiveFixture)status.capabilities.acknowledged_features.push('command-ack',...(contextProfile==='table'?['cell-batch','cell-lock']:[]));
  }
  status.state_presentation={enabled:false,fade_ms:350,pulse_ms:1200,ring_ms:800,chase_ms:1200,states:{idle:{rgb:16777215,motion:0},won:{rgb:65280,motion:2},lost:{rgb:16711680,motion:1},off:{rgb:0,motion:0}}};
  if(contextProfile==='table')status.state_presentation.fire_rotation=0;
  if(status.capabilities&&process.argv.includes('--peripherals')){
    status.capabilities.acknowledged_features.push('buzzer','display');
    Object.assign(status,{avr_online:true,tm1637_ack:true,tm1637_brightness:5,audio_cues:['welcome','winner','attention']});
  }
  values['/api/pixels/corrections']={revision:0,enabled:true,pixels:status.pixels,max_rules:32,index_base:0,rules:[],sources:Array.from({length:status.pixels},(_,index)=>index)};
  values['/api/peer/info']={product:'PakarController',board_id:status.board_id,boot_id:status.boot_id,version:status.version,hostname:status.hostname,application,capabilities:status.capabilities,simulated:true};
  values['/api/telemetry']={type:'telemetry',product:'PakarController',board_id:status.board_id,boot_id:status.boot_id,hostname:status.hostname,version:status.version,simulated:true,
    esp:{model:'SIMULATED ESP32-S3',cpu_mhz:240,flash_bytes:16777216,free_heap_bytes:178240,uptime_ms:60000,uart_errors:0,usb_errors:0},avr:{online:status.avr_online}};
}
// Extend the existing no-write visual harness, rather than a second package or
// inferred live-board snapshot. Physical cell 80 is deliberately remapped to 1.
const pixelFixture=process.argv.includes('--pixels');
const diagnosticFixture=process.argv.includes('--diagnostic');
if(diagnosticFixture){
  const {enforce,...range}=toolPolicy;
  Object.assign(status,{board_id:'simulated-pixel-diagnostics',boot_id:'a'.repeat(32),http_peer_fencing:true,
    command_clock_ms:1000,compatibility:{...range,enforced:enforce},version:toolRelease.version,
    led_output:{enabled:true,mode:'effects',diagnostic:{revision:1,active:true,index:474,count:6,rgb:0x1234ab,off:false}}});
  status.capabilities?.acknowledged_features.push('command-ack','pixel-diagnostic');
}
const toastFixture=process.argv.includes('--toasts');
if(pixelFixture){values['/api/cells/mapping'].physical[0]=80;values['/api/cells/mapping'].physical[79]=1;}
const receipts=new Map();
function fixtureCommand(packet){
  if(packet?.board_id!==status.board_id||packet?.boot_id!==status.boot_id)throw Error('peer_changed');
  if(!/^[a-f0-9]{32}$/.test(packet.request_id||''))throw Error('invalid_request_id');
  const feature={cells:'cell-batch','game-press':'game-press',reset:'reset-ack','cell-lock':'cell-lock','pixel-diagnostic':'pixel-diagnostic','state-presentation':'state-presentation',effect:'leds'}[packet.command];
  if(!feature||!status.capabilities?.acknowledged_features.includes(feature))throw Error('fixture_command_unavailable');
  const prior=receipts.get(packet.request_id);if(prior){if(prior.packet!==JSON.stringify(packet))throw Error('request_id_conflict');return prior.receipt;}
  const now=Date.now()-started+100;if(!Number.isSafeInteger(packet.expires_at_ms)||packet.expires_at_ms<now||packet.expires_at_ms>now+10000)throw Error('command_expired');
  let count;
  switch(packet.command){
    case 'cells': {
      if(contextProfile!=='table')throw Error('capability_unavailable');
      if(!['idle','lost','won','off'].includes(packet.state))throw Error('invalid_state');
      const ids=new Set();for(const entry of String(packet.ids||'').split(',')){const match=/^\s*(\d+)(?:\s*-\s*(\d+))?\s*$/.exec(entry);if(!match)throw Error('invalid_selection');const first=+match[1],last=+(match[2]||match[1]);if(first<1||last>80||first>last)throw Error('invalid_selection');for(let id=first;id<=last;id++)ids.add(id);}
      status.cell_states||=status.cells.map(value=>({white:'idle',red:'lost',off:'off'})[value]||'unknown');
      if(status.red_lock&&[...ids].some(id=>status.cell_states[id-1]==='lost'&&packet.state!=='lost'))throw Error('cell_locked');
      for(const id of ids)status.cell_states[id-1]=packet.state;count=ids.size;break;
    }
    case 'game-press':if(contextProfile!=='controller'||!Number.isInteger(packet.key)||packet.key<1||packet.key>9||!(status.key_game.enabled_mask&(1<<(packet.key-1))))throw Error('invalid_key');if(!status.winner)status.winner=packet.key;break;
    case 'reset':if(contextProfile!=='controller')throw Error('capability_unavailable');status.winner=0;break;
    case 'cell-lock':status.red_lock=packet.enabled===true;break;
    case 'pixel-diagnostic': {
      if(!Number.isInteger(packet.index)||!Number.isInteger(packet.count)||packet.index<0||packet.count<1||packet.index+packet.count>status.pixels||!Number.isInteger(packet.rgb)||packet.rgb<0||packet.rgb>0xffffff)throw Error('invalid_pixel');
      status.led_output.diagnostic={revision:status.led_output.diagnostic.revision+1,active:true,index:packet.index,count:packet.count,rgb:packet.rgb,off:packet.off===true};break;
    }
    case 'state-presentation': {
      const settings=packet.settings;
      if(!settings||typeof settings!=='object')throw Error('invalid_settings');
      if('fire_rotation' in settings){
        if(contextProfile!=='table'||![0,90,180,270].includes(settings.fire_rotation))throw Error('invalid_rotation');
        status.state_presentation.fire_rotation=settings.fire_rotation;
      }
      for(const key of ['fade_ms','pulse_ms','ring_ms','chase_ms'])if(key in settings){if(!Number.isInteger(settings[key])||settings[key]<0||settings[key]>65535)throw Error('invalid_timing');status.state_presentation[key]=settings[key];}
      if('enabled' in settings)status.state_presentation.enabled=settings.enabled===true;
      for(const [key,value] of Object.entries(settings.states||{})){
        if(!['idle','won','lost','off'].includes(key)||!value||typeof value!=='object')throw Error('invalid_state');
        if('motion' in value&&(!Number.isInteger(value.motion)||value.motion<0||value.motion>3))throw Error('invalid_motion');
        if('rgb' in value&&(!Number.isInteger(value.rgb)||value.rgb<0||value.rgb>0xffffff))throw Error('invalid_rgb');
        Object.assign(status.state_presentation.states[key],value);
      }
      break;
    }
    case 'effect':if(!['rainbow','none'].includes(packet.value))throw Error('invalid_effect');status.effect=packet.value;break;
    default:throw Error('fixture_command_unavailable');
  }
  status.command_clock_ms=now;
  const receipt={type:'command-result',request_id:packet.request_id,board_id:status.board_id,boot_id:status.boot_id,accepted:true,simulated:true,avr_acknowledged:['reset','game-press'].includes(packet.command),...(count===undefined?{}:{count}),status:structuredClone(status)};
  if(receipts.size>=64)receipts.delete(receipts.keys().next().value);receipts.set(packet.request_id,{packet:JSON.stringify(packet),receipt});return receipt;
}
const server=http.createServer(async(req,res)=>{
  res.setHeader('Cache-Control','no-store');
  res.setHeader('X-Pakar-Simulated','true');
  const url=new URL(req.url,'http://localhost');
  const json=(code,value)=>{res.writeHead(code,{'Content-Type':'application/json'});res.end(JSON.stringify(value));};
  status.command_clock_ms=Date.now()-started+100;
  if(req.method==='GET'){
    const assets={'/manifest.webmanifest':'manifest.webmanifest','/sw.js':'sw.js'};
    const file=assets[url.pathname];if(file){try{const bytes=await readFile(new URL('../build/web/full/'+file,import.meta.url));res.writeHead(200,{'Content-Type':file.endsWith('.js')?'text/javascript':'application/manifest+json'});res.end(bytes);}catch{json(404,{error:'fixture_asset_not_built',file});}return;}
  }
  if(req.method==='GET'&&req.url==='/app-icon.png'){res.writeHead(200,{'Content-Type':'image/png'});res.end(await readFile(new URL('../assets/branding/app-icon-256.png',import.meta.url)));return;}
  if(req.method==='GET'&&req.url==='/assets/fonts/Vazirmatn-Regular.ttf'){res.writeHead(200,{'Content-Type':'font/ttf'});res.end(await readFile(new URL('../assets/fonts/Vazirmatn-Regular.ttf',import.meta.url)));return;}
  if(req.method!=='GET'){
    if(!interactiveFixture||req.method!=='POST'||url.pathname!=='/api/command'){json(405,{error:'Read-only SIMULATED fixture; no device mutation'});return;}
    if(req.headers['x-pakar-board']!==status.board_id||req.headers['x-pakar-boot']!==status.boot_id){json(409,{error:'peer_changed'});return;}
    let body='';try{for await(const bytes of req){body+=bytes;if(body.length>8192)throw Error('fixture_body_too_large');}const packet=JSON.parse(new URLSearchParams(body).get('packet'));json(200,fixtureCommand(packet));}catch(error){json(400,{error:error.message,simulated:true});}return;
  }
  if(req.url==='/'){let html=await readFile(webSourceUrl,'utf8');if(toastFixture)html=html.replace('</body>','<button type="button" id="fixtureToast" style="position:fixed;top:4px;left:4px;z-index:101" onclick="webToasts??=createToastController({root:notice,translate:uiTr});webToasts.showToast(\'info\',\'SIMULATED notification\',\'Read-only fixture — not a device event\',60000);webToasts.showToast(\'success\',\'SIMULATED success\',\'No board command was sent\',60000);webToasts.showToast(\'error\',\'SIMULATED error\',\'Keyboard Escape dismisses the focused notification\',60000)">SIMULATED · Test notifications</button></body>');res.writeHead(200,{'Content-Type':'text/html;charset=utf-8'});res.end(html);return;}
  const value=values[url.pathname];
  const response=pixelFixture&&url.pathname==='/api/status'&&url.searchParams.get('pixels')==='1'?{...status,pixel_preview_requested:true,pixel_frame:{revision:42,brightness:255,physical:values['/api/cells/mapping'].physical,rgb:['1234ab','ff0000','00ff00','0000ff','ffffff','000000'].join('').repeat(80)}}:value;
  res.writeHead(response?200:404,{'Content-Type':'application/json'});res.end(JSON.stringify(response||{error:'Fixture has no such endpoint'}));
});
server.on('upgrade',(req,socket)=>{
  if(!diagnosticFixture || new URL(req.url,'http://localhost').pathname!=='/ws' || !req.headers['sec-websocket-key']){socket.destroy();return;}
  const accept=createHash('sha1').update(req.headers['sec-websocket-key']+'258EAFA5-E914-47DA-95CA-C5AB0DC85B11').digest('base64');
  socket.write('HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: '+accept+'\r\nSec-WebSocket-Protocol: arduino\r\n\r\n');
  // Read-only simulated pushes. Never execute any received command or forward
  // client data to hardware; HTTP mutations remain405 above.
  const push=()=>{const payload=Buffer.from(JSON.stringify(status)),header=Buffer.alloc(4);
    header[0]=0x81;header[1]=126;header.writeUInt16BE(payload.length,2);socket.write(Buffer.concat([header,payload]));};
  socket.on('error',()=>socket.destroy());
  socket.on('data',bytes=>{if((bytes[0]&15)===8)socket.end();});
  const timer=setInterval(()=>{if(!socket.destroyed)push();},1000);
  socket.once('close',()=>clearInterval(timer));push();
});
server.listen(Number(process.env.PAKAR_FIXTURE_PORT)||0,'127.0.0.1',()=>console.log(JSON.stringify({scope:interactiveFixture?'SIMULATED interactive memory-only UI; no hardware connection':'SIMULATED read-only actual Web UI, not hardware acceptance',url:'http://localhost:'+server.address().port+'/',pid:process.pid})));
process.on('SIGINT',()=>server.close(()=>process.exit(0)));
