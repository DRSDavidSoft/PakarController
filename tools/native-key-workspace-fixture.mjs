// Isolated native visual peer, adapted from tests/remote_ui_fixture.mjs and
// tests/cli_peer_fixture.mjs. Never forwards to a physical board or production IPC.
import assert from 'node:assert/strict';
import http from 'node:http';
import {createHash} from 'node:crypto';
import {spawn} from 'node:child_process';
import {mkdir, readFile, writeFile, access} from 'node:fs/promises';
import {resolve, join} from 'node:path';
import {cliStatus, cliHeaders, cliBoard, cliBoot} from '../tests/cli_peer_fixture.mjs';

const [exeArg, profileArg, scenario = 'winner', screenshotArg, panel = 'Keys', ...options] = process.argv.slice(2);
const scenarios = ['ready', 'winner', 'pressed', 'all', 'connected', 'offline', 'warning', 'progress', 'status-rgb'];
assert(exeArg && profileArg && scenarios.includes(scenario),
  'Usage: node tools/native-key-workspace-fixture.mjs <built.exe> <isolated-profile-dir> [ready|winner|pressed|all|connected|offline|warning|progress|status-rgb] [unused-startup.png] [panel] [--once] [--width=900 --height=600]');
assert(['Keys', 'Hardware', 'Network', 'EffectControls', 'FrontPanel', 'PixelCorrections', 'Table', 'Preferences'].includes(panel), 'Explicit supported panel required');
const tableFixture = ['Table', 'Preferences'].includes(panel);
assert(options.every(value => value === '--once' || /^--(?:width|height)=\d+$/.test(value)
  || /^--workspace-ui=(?:egui_dock|craft_ui)$/.test(value)
  || /^--preferences-page=(?:application|appearance)$/.test(value)), 'Unsupported capture option');
assert(new Set(options.map(value => value.split('=')[0])).size === options.length, 'Capture options must not be repeated');
const once = options.includes('--once');
const workspaceUi = options.find(value => value.startsWith('--workspace-ui='))?.split('=')[1] ?? 'egui_dock';
const preferencesPage = options.find(value => value.startsWith('--preferences-page='))?.split('=')[1] ?? 'application';
assert(panel === 'Preferences' || !options.some(value => value.startsWith('--preferences-page=')), 'Preferences page requires the Preferences panel');
const dimension = name => {
  const value = options.find(value => value.startsWith(`--${name}=`));
  return value === undefined ? undefined : Number(value.split('=')[1]);
};
const width = dimension('width'), height = dimension('height');
assert((width === undefined) === (height === undefined), 'Capture width and height must be supplied together');
assert(width === undefined || (Number.isInteger(width) && width >= 900 && width <= 3840
  && Number.isInteger(height) && height >= 600 && height <= 2160), 'Capture viewport must be within 900x600 and 3840x2160');
const statusScenario = ['connected', 'offline', 'warning', 'progress', 'status-rgb'].includes(scenario);
const exe = resolve(exeArg), profile = resolve(profileArg), config = join(profile, 'preferences.json');
await mkdir(profile, {recursive: true});
// Existing fixture preferences (including saved box positions) are never replaced.
// The exclusive write below is the atomic authority, not a prior exists check.
const sockets = new Set();
const requests = [];
const winner = scenario === 'ready' || statusScenario ? 0 : 2;
const mask = scenario === 'all' ? 511 : 15;
let origin;
let statusRevision = 0;
const colors = Array.from({length: 9}, (_, i) =>
  !(mask & (1 << i)) ? '000000' : winner === 0 ? 'e8e5df' : i + 1 === winner ? '30cf80' : 'e74c64');
// Native Frame::key_group consumes 18 one-based physical groups of 12 RGB
// pixels, not 9 cells and not the table's 80 groups of 6. Both group samples
// carry the same synthetic box color; no output state is inferred by the app.
const pixelFrame = {
  rgb: colors.flatMap(color => [color.repeat(12), color.repeat(12)]).join(''),
  physical: Array.from({length: 18}, (_, i) => i + 1),
  revision: 1, brightness: 255, pending: false,
};
assert.equal(pixelFrame.rgb.length, 216 * 6);
assert.match(pixelFrame.rgb, /^[0-9a-f]{1296}$/i);
assert.equal(new Set(pixelFrame.physical).size, 18);
const tableFrame={rgb:'ffffff'.repeat(480),physical:Array.from({length:80},(_,i)=>i+1),revision:1,brightness:255,pending:false};
function requestedProtocol(header) {
  if (header === undefined) return null;
  if (typeof header !== 'string') throw new Error('Invalid WebSocket subprotocol header');
  const protocols = header.split(',').map(protocol => protocol.trim());
  if (protocols.some(protocol => !/^[\w!#$%&'*+.^|~\x60-]+$/.test(protocol))
      || !protocols.includes('arduino')) throw new Error('Unsupported WebSocket subprotocol');
  // Echo only an offered supported protocol. Native tungstenite rejects an
  // otherwise valid upgrade if the requested arduino protocol is omitted.
  return 'arduino';
}
function status() {
  const value = cliStatus({hostname: 'simulated-keycontroller', mode: 'first-wins', winner,
    canonical_url: origin, ip: '127.0.0.1', uplink: 'wifi', uplink_ipv4_ready: true,
    avr_online: true, programming_latch: false, station_connected: true,
    station_enabled: true, station_ssid: 'SIMULATED VISUAL PEER', pixels: 216,
    brightness: 255, effect: 'none', active_inputs: scenario === 'pressed' ? 2 : 0,
    station_ip: '192.168.1.42', station_rssi: -46, ap_enabled: true, ap_started: true,
    ap_ip: '192.168.4.1', mdns_started: true, ssdp_started: true, netbios_started: true,
    ntp_synced: true, avr_version: 'SIMULATED', link_mode: 'hardware-uart',
    tm1637_ack: true, tm1637_brightness: 7, tm1637_segments: [0x3f, 0x06, 0x5b, 0x4f], audio_cues: ['welcome', 'winner', 'reset'],
    state_presentation: {enabled: true, states: {idle:{rgb:0xe8e5df,motion:0},won:{rgb:0x30cf80,motion:2},lost:{rgb:0xe74c64,motion:1},off:{rgb:0,motion:0}},
      fade_ms:250,pulse_ms:900,ring_ms:900,chase_ms:1800,neighbors:true,invert:false},
    key_mask_applied: true, game_audio_applied: true,
    key_game: {enabled_mask: mask, idle_rgb: 0xe8e5df, win_rgb: 0x30cf80,
      lose_rgb: 0xe74c64, auto_reset_ms: 0, extended_effects: true,
      winner_cue: 'silent', reset_cue: 'silent', chase_period_ms: 900,
      chase_neighbors: true, chase_invert: false},
    led_output: {enabled: true, mode: 'effects', driver: {ready: true, backend: 'rmt-dma', rmt: true, dma: true}},
    pixel_preview_requested: true,
    pixel_frame: pixelFrame,
    capabilities: {board_kind: 'keys', role: 'first-wins', name: 'SIMULATED key workspace', hostname: 'simulated-keycontroller',
      acknowledged_features: ['inputs', 'outputs', 'leds', 'game-press', 'key-settings', 'network', 'telemetry', 'pixel-events', 'pixel-corrections', 'state-presentation', 'display', 'text', 'buzzer'],
      led_output: {backend: 'rmt-dma', rmt: true, dma: true},
      modes: [{id: 'first-wins', code: 2, label: 'First press wins', kind: 'game'}]}});
  if(tableFixture){
    Object.assign(value,{hostname:'SIMULATED table workspace',mode:'cell',pixels:480,winner:0,cells:Array(80).fill('white'),cell_states:Array(80).fill('idle'),pixel_frame:tableFrame});
    value.capabilities={board_kind:'table',role:'table-selection',name:'SIMULATED TABLE',hostname:'simulated-table',
      acknowledged_features:['table','network','leds','cell-batch','cell-lock','pixel-events','pixel-diagnostic','pixel-corrections','state-presentation'],
      table:{columns:10,rows:8,pixels_per_cell:6},modes:[{id:'cell',code:1,label:'Cell selection',kind:'presentation'}]};
  }
  if (statusScenario) {
    // These are peer-bound observations through the normal status parser, not
    // app-caption overrides or hardware evidence. Keep original scenarios intact.
    value.status_rgb = {available:true,have_output:true,enabled:true,rgb:[56,210,122],
      brightness:255,state:'ready',revision:1,pin:48,pin_confirmed:true,persisted:true,effect:0};
    if (scenario === 'warning') {
      value.avr_online = false;
      value.tm1637_ack = false;
      value.status_rgb = {...value.status_rgb,rgb:[255,186,66],state:'avr-offline',effect:2};
    }
    if (scenario === 'progress') {
      value.capabilities.acknowledged_features.push('ota');
      value.upgrade = {board_id:cliBoard,boot_id:cliBoot,source_boot_id:cliBoot,source:'visual-fixture',
        operation_id:1,revision:++statusRevision,phase:'receiving',active:true,received_bytes:388096,
        expected_bytes:1048576,progress_known:true,progress_percent:37,completion_verified:false,error:''};
      value.status_rgb = {...value.status_rgb,rgb:[62,170,255],state:'updating',effect:5,progress_percent:37};
    }
    if (scenario === 'status-rgb') {
      // Already emitted color; brightness must not be applied a second time.
      value.status_rgb = {...value.status_rgb,rgb:[24,105,61],brightness:128,state:'ready',effect:1};
    }
  }
  return value;
}
function frame(value, opcode = 1) {
  const payload = Buffer.isBuffer(value) ? value : Buffer.from(JSON.stringify(value));
  assert(payload.length <= 65535);
  const header = Buffer.alloc(payload.length < 126 ? 2 : 4);
  header[0] = 0x80 | opcode;
  header[1] = payload.length < 126 ? payload.length : 126;
  if (header.length === 4) header.writeUInt16BE(payload.length, 2);
  return Buffer.concat([header, payload]);
}
const server = http.createServer(async (req, res) => {
  cliHeaders(res);
  requests.push({method: req.method, path: req.url});
  if (requests.length > 200) requests.shift();
  // Client introductions are accepted locally; no credentials/telemetry escape.
  if (req.method === 'POST' && req.url === '/api/clients/report') {
    for await (const _ of req) { /* drain only */ }
    res.end('{"accepted":true}'); return;
  }
  if (req.method !== 'GET') {
    res.writeHead(405); res.end('{"error":"Read-only visual fixture: no hardware commands"}'); return;
  }
  const path = new URL(req.url, 'http://127.0.0.1').pathname;
  const values = {
    '/api/status': status(),
    '/api/auth/info': {ready: true, enabled: false, default_credentials: false},
    '/api/network': {station_enabled: true, ap_enabled: false, station_ssid: 'SIMULATED VISUAL PEER'},
    '/api/cloud': {mqtt_enabled: false, websocket_enabled: false, telemetry_seconds: 30},
    '/api/game/settings': status().key_game,
    '/api/cells/mapping': {physical:Array.from({length:80},(_,i)=>i+1),names:Array(80).fill('')},
    '/api/telemetry': {product: 'PakarController', esp: {model: 'SIMULATED ONLY'}, avr: {model: 'SIMULATED ONLY'}, clients: []},
    '/api/pixels/corrections': {revision:2,enabled:true,pixels:216,rules:[{kind:'shift',pixel:120,offset:-1}],sources:Array.from({length:216},(_,i)=>i)},
  };
  res.writeHead(path in values ? 200 : 404);
  res.end(JSON.stringify(values[path] ?? {error: 'Not provided by visual fixture'}));
});
server.on('upgrade', (req, socket) => {
  if (req.url?.split('?')[0] !== '/ws' || typeof req.headers['sec-websocket-key'] !== 'string') {socket.destroy(); return;}
  let protocol;
  try {protocol = requestedProtocol(req.headers['sec-websocket-protocol']);}
  catch {socket.end('HTTP/1.1 400 Bad Request\r\nConnection: close\r\nContent-Length: 0\r\n\r\n'); return;}
  const accept = createHash('sha1').update(req.headers['sec-websocket-key'] + '258EAFA5-E914-47DA-95CA-C5AB0DC85B11').digest('base64');
  socket.write('HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: ' + accept
    + (protocol ? '\r\nSec-WebSocket-Protocol: ' + protocol : '') + '\r\n\r\n');
  sockets.add(socket);
  socket.on('error', () => {});
  socket.on('close', () => sockets.delete(socket));
  let pending = Buffer.alloc(0);
  socket.on('data', chunk => {
    pending = Buffer.concat([pending, chunk]);
    if (pending.length > 65536) {socket.destroy(); return;}
    while (pending.length >= 2) {
      let length = pending[1] & 127, at = 2;
      const opcode = pending[0] & 15, masked = !!(pending[1] & 128);
      if (!(pending[0] & 128) || !masked || length === 127) {socket.destroy(); return;}
      if (length === 126) {if (pending.length < 4) return; length = pending.readUInt16BE(2); at = 4;}
      if (pending.length < at + 4 + length) return;
      const mask = pending.subarray(at, at + 4); at += 4;
      const data = Buffer.from(pending.subarray(at, at + length));
      for (let i = 0; i < data.length; i++) data[i] ^= mask[i % 4];
      pending = pending.subarray(at + length);
      if (opcode === 8) {socket.end(frame(data, 8)); return;}
      if (opcode === 9) {socket.write(frame(data, 10)); continue;}
      if (opcode !== 1) continue;
      try {
        const value = JSON.parse(data.toString());
        if (['status', 'subscribe', 'hello', 'client-report', 'ping'].includes(value.type)) socket.write(frame(status()));
        // Intentionally do not ACK or execute any actuation intent.
      } catch {socket.destroy(); return;}
    }
  });
  socket.write(frame(status()));
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
origin = 'http://127.0.0.1:' + server.address().port;
// A startup --panel request may precede capability discovery and is correctly
// rejected then. Seed only this simulated peer's ordinary saved presentation;
// never inject capabilities into the production app or enable its IPC owner.
const panels = ['Table','PixelDiagnostics','PixelCorrections','Keys','EffectControls','FrontPanel','Hardware','Network','Streaming','Events'];
const zeroRect = {min:{x:0,y:0},max:{x:0,y:0}};
const leaf = tab => ({Leaf:{rect:zeroRect,viewport:zeroRect,tabs:[tab],active:0,scroll:0,collapsed:false,tab_bar_hidden:false}});
const panelWorkspace = tableFixture ? {native_workspaces:{entries:[{
  identity:{board:cliBoard,application:'board-kind:table'},
  snapshot:{layout_revision:1,hidden_panels:panels.filter(item=>item!=='Table'),layout:JSON.stringify({surfaces:[{Main:{nodes:[leaf('Table')],focused_node:0,collapsed:false,collapsed_leaf_count:0}}],focused_surface:0})}
}]}} : panel === 'Keys' ? {} : {native_workspaces:{entries:[{
  identity:{board:cliBoard,application:'board-kind:keys'},
  snapshot:{layout_revision:1,hidden_panels:panels.filter(item=>item!=='Keys' && item!==panel),
    layout:JSON.stringify({surfaces:[{Main:{nodes:[
      {Horizontal:{rect:zeroRect,fraction:0.58,fully_collapsed:false,collapsed_leaf_count:0}},leaf('Keys'),leaf(panel)
    ],focused_node:2,collapsed:false,collapsed_leaf_count:0}}],focused_surface:0})}
}]}};
await writeFile(config, JSON.stringify({endpoint: origin, user: '', single_instance: false,
  native_dialogs: false, theme: 'Dark', language: 'English', close_to_tray: false,
  workspace_ui: workspaceUi,
  // Offline means actually disconnected in this isolated profile, not a live
  // transport with a fabricated "offline" caption or stale physical sample.
  connection_method: 'WiFi', connection_enabled: scenario !== 'offline', auto_connect: scenario !== 'offline',
  client_management:{base:'https://office.digitalogic.ir',auto_check:false,auto_download:false,auto_apply:false,auto_cleanup:false,telemetry:false,remote_commands:false,allow_remote_cleanup:false,allow_remote_uninstall:false},
  ...panelWorkspace,
  notifications: {system: 'Off', in_app: false, sound: false}}, null, 2), {flag: 'wx'});
const screenshot = screenshotArg ? resolve(screenshotArg) : join(profile, 'startup.png');
await assert.rejects(access(screenshot), 'Startup screenshot must not already exist');
const child = spawn(exe, panel==='Preferences'?['--preferences-page',preferencesPage]:['--panel', panel], {env: {...process.env, PAKAR_CONFIG_FILE: config,
  PAKAR_ENDPOINT: origin, PAKAR_USER: '', PAKAR_PASSWORD: '', PAKAR_SCREENSHOT_PATH: screenshot,
  // The native app also enforces config + screenshot + single_instance=false.
  // Empty defaults prevent inheriting unrelated fixture size overrides.
  PAKAR_SCREENSHOT_WIDTH:width === undefined ? '' : String(width),
  PAKAR_SCREENSHOT_HEIGHT:height === undefined ? '' : String(height)},
  windowsHide: true, stdio: ['ignore', 'pipe', 'pipe']});
child.stdout.resume(); child.stderr.on('data', data => process.stderr.write(data));
const receipt = {scope: 'SIMULATED loopback native visual fixture; not board or physical acceptance',
  process_id: child.pid, executable: exe, executable_sha256: createHash('sha256').update(await readFile(exe)).digest('hex'),
  endpoint: origin, isolated_profile: config, scenario, panel, workspace_ui:workspaceUi,
  preferences_page:panel === 'Preferences' ? preferencesPage : null, startup_screenshot: screenshot,
  requested_viewport:width === undefined ? null : {width,height},
  connection_enabled:scenario !== 'offline',simulated_update_progress:scenario === 'progress'};
await writeFile(join(profile, 'fixture.json'), JSON.stringify(receipt, null, 2), {flag: 'wx'});
console.log(JSON.stringify(receipt));
const timer = setInterval(() => {for (const socket of sockets) if (!socket.destroyed) socket.write(frame(status()));}, 1000);
// Use the real pixel-only event envelope after the peer introduction. A first
// workspace restoration may legitimately revoke its previous RGB sample; the
// next independently identified observation supplies the fixture's live dots.
const pixelTimer = setInterval(() => {
  for (const socket of sockets) if (!socket.destroyed) socket.write(frame({
    type: 'pixel-frame', board_id: cliBoard, boot_id: cliBoot,
    pixel_preview_requested: true, pixel_frame: tableFixture?tableFrame:pixelFrame,
  }));
}, 100);
// Only this process's spawned, IPC-isolated fixture is retired on Ctrl+C.
// No executable-name scan, shared owner, board endpoint or production task is used.
const stop = () => {
  clearInterval(timer);
  clearInterval(pixelTimer);
  clearInterval(captureTimer);
  for (const socket of sockets) socket.destroy();
  server.close();
  if (child.exitCode === null && child.signalCode === null) child.kill();
};
process.once('SIGINT', stop);
process.once('SIGTERM', stop);
const captureStarted = Date.now();
const captureTimer = once ? setInterval(async () => {
  try {
    const pixels = await readFile(screenshot);
    if (pixels.length > 1024 && pixels.subarray(0, 8).equals(Buffer.from([137,80,78,71,13,10,26,10]))
        && pixels.subarray(-12).equals(Buffer.from('0000000049454e44ae426082', 'hex'))) stop();
  } catch { /* The application owns its bounded framebuffer request. */ }
  if (Date.now() - captureStarted > 20000) { console.error('No valid framebuffer capture within deadline'); process.exitCode = 1; stop(); }
}, 250) : undefined;
child.once('exit', async () => {
  clearInterval(timer);
  clearInterval(pixelTimer);
  clearInterval(captureTimer);
  for (const socket of sockets) socket.destroy();
  await writeFile(join(profile, 'requests.json'), JSON.stringify(requests, null, 2), {flag: 'wx'});
  server.close();
});
child.once('error', error => {console.error(error); clearInterval(timer); clearInterval(pixelTimer); server.close(); process.exitCode = 1;});
