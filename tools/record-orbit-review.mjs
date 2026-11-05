// Materialize THIS manually read, pinned checkpoint into the existing ledgers.
// Never discovers new "read" files or accepts changed source automatically.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {resolve,relative} from 'node:path';
import {fileURLToPath} from 'node:url';
const root=resolve(fileURLToPath(new URL('..',import.meta.url)));
const donor=resolve(root,'../repos/orbit');
const revision='da50c877a59cdc61f7a4381a05797886c9a7abf5';
const check=process.argv.includes('--check');
const hash=b=>createHash('sha256').update(b).digest('hex');
const lf=b=>b.toString('utf8').replaceAll('\r\n','\n');
const git=(cwd,...args)=>execFileSync('git',args,{cwd,maxBuffer:16*1024*1024});
const targetRevision='ef22c268fc896a56f71a8daf42a196b93f7761c0';
const expectedTargets={
 'tools/consult-reuse.ps1':'67c51b628d0c80ecb09db6e3fae507f4fd8f9c3dcc1544e4250a659b37cf6abf',
 'tools/generate-reuse-ledger.ps1':'40f1e32dd75e72529b5062caf662e3cf0b9311722151b8ddcf6e88d06ec1785f',
 'firmware/esp/application.ino':'2d000c3788d9427a362c2c4a4163bb1367c363ead57169281269433a9b6a3809',
 'firmware/esp/src/StaticAssetPolicy.h':'971f156e44313d2d751b4bd10e89337669885f28b219df1407f70451deb8bc2b',
 'firmware/esp/src/UserFileStore.h':'8291b62bf324fe6a75dfa1fce020a55dd29bf90a09a5316178763792bc0b9599',
 'tests/async_http_test.cpp':'00b94a5c0e929470d30095207036fb640b648fa9e26ea7706fa1d7a615fc0bbe',
 'tests/file_transfer_test.cpp':'f864f61e61cc3bdcc247651739439919a098dabd56d1fe7e271e706ee9af6af4',
 'tests/user_file_store_test.cpp':'a070c43cbc3906c65b83b2e895f3a5c3fe8824e5052e79bb5965e4c3daf1c528',
 'tests/reuse_preflight_test.ps1':'ac0d5bf6bc40f874344cc001f83e789fdd663246c8426241d8c0f3164af4be33',
 'tests/file_comparison_test.ps1':'8684745b447736922bfecacdc6116f8f665803b87f908867bd4dfce84d3dd45a',
 'tools/test-user-files.ps1':'22947b3a5d02c14955fb7781f010f23a8640c02f6ace1a46589fff9288c137c2',
 'tools/generate-file-comparison.ps1':'ce9339babbf311d1cf3c0d11d699701e22b96339d7f5820cc19af831f82d70db',
 'firmware/esp/src/UploadSpacePolicy.h':'b4096ba6b7855808c20924de0310e6fd6828f4df034efc2746a23e7b35289911',
 'docs/ORBIT-FIRMWARE-PARITY.md':'c8f52e5267290887db0c05134dc20519902f826ed2807f0bc7f680571a858eb7'
};
function csv(text){
 const rows=[];let row=[],field='',quoted=false;
 for(let i=0;i<text.length;i++){
  const c=text[i];
  if(c==='"'){if(quoted&&text[i+1]==='"'){field+='"';i++;}else quoted=!quoted;}
  else if(!quoted&&c===','){row.push(field);field='';}
  else if(!quoted&&c==='\n'){row.push(field.replace(/\r$/,''));if(row.some(Boolean))rows.push(row);row=[];field='';}
  else field+=c;
 }
 if(quoted)throw Error('Unclosed CSV field');
 if(field||row.length){row.push(field);rows.push(row);}
 const keys=rows.shift();return rows.map(r=>{if(r.length!==keys.length)throw Error('CSV width');return Object.fromEntries(keys.map((k,i)=>[k,r[i]]));});
}
function write(path,text){
 const full=resolve(root,path),before=readFileSync(full,'utf8');
 if(before===text)return;
 if(check)throw Error('Checkpoint metadata differs: '+path);
 writeFileSync(full,text);
}
const notes={
 'AtomicEventRecord.h':'Committed-file visibility, no overwrite, write/readback/rename failure states.',
 'JsonUtf8.h':'UTF-8 validity and JSON escaping; byte-count repair cap/allocation failure, not a parser replacement.',
 'WebAssetPolicy.h':'Aligned fresh-space reserve, chunk arithmetic, size cap and wrap-safe lease.',
 'FirmwareTlsMemory.h':'One installed allocator with task-scoped PSRAM preference; SDK allocator fallback; do not hot-swap callbacks.',
 'HttpDate.h':'IMF date grammar/calendar/weekday and non-mutating failure; paired stricter implementation.',
 'HttpFileValidators.h':'Modified-Since vs exact strong If-Range; date/tag admission.',
 'HttpRepresentationPolicy.h':'Paired encoding negotiation and full weak ETag-list parsing.',
 'VerifiedFileWrite.h':'Paired flushed-byte readback before progress/digest advancement.',
 'StorageFiles.h':'Sandbox/atomic upload, public HTTP serving, ranges and validators; no private path exposure.',
 'WebAsset.h':'Serialized asset upload, verified staging and atomic rename; old asset retained.',
 'FirmwareWifiRange.h':'Strict resumed response start/end/total and bounded download-only recovery.',
 'OtaReplyFence.h':'Generation plus fragment count CAS; old callbacks cannot decrement a new generation.',
 'OtaAdmissionPolicy.h':'OTA transport reply admission vs conflicting modem workflow ownership.',
 'CooperativeTlsClient.h':'Empty available() yield in the dedicated MQTT task, not global TLS weakening.',
 'TlsMemoryTasks.h':'Independent bounded task-role slots and compare/exchange release.',
 'NativePayloadProgress.h':'Stall deadline separated from hard transfer cap; queued-byte counters.',
 'OutboundQueuePolicy.h':'Bounded reply-priority queue; only expendable events may be evicted.',
 'HttpCountedHeader.h':'Counted AT header length bounds; selected peripheral exclusion remains pending exact comparison.',
 'MqttCommandAdmission.h':'Reply single-flight guard that does not let background telemetry starve controls.',
 'MqttDeliveryPolicy.h':'Identity-current vs fresh-session admission and SIM/epoch invalidation.',
 'MqttTelemetrySchedule.h':'Advance cadence only on queue acceptance and bound failed admission retries.',
 'SafeRestartPolicy.h':'Control-receipt delivery fence and bounded restart deadline, without interrupting UART ownership.',
 'EventStoragePolicy.h':'Payload+metadata byte reserve and bounded retry after storage exhaustion.',
 'HttpActionResult.h':'SIMCom AT result grammar; specific exclusion below.',
 'HttpBinaryFrames.h':'Counted SIMCom HTTPREAD frames and binary payload preservation; specific exclusion below.',
 'HttpExchange.h':'Redacted inspection headers and UTF-8/hex byte capture; mixed SIM/AT pieces need sectional comparison.',
 'DeviceCredentialPolicy.h':'UID-bound credential choice; corrupted stored record never silently bootstraps.',
 'DeviceSerialPolicy.h':'Read-only factory identity formatting and invalid all-zero/all-one MAC rejection.',
 'JsonStorage.h':'PSRAM-first ArduinoJson allocator with internal fallback; allocation lifecycle.',
 'CompactTelemetry.h':'Compact freshness/unknown projection; most location/cellular fields are product logic, not all-file exclusion.',
 'NetworkServices.h':'mDNS/SSDP/NetBIOS/NTP and mixed modem time; network/time sections must be compared separately.',
 'WifiRecovery.h':'STA link+IP readiness, AP-preserving reconnect/backoff, disconnect reason observations.',
 'EspHealth.h':'Actual capability heap/OTA/security observations, not build-time guesses.',
 'EspLoad.h':'Bounded FreeRTOS per-core/task runtime sample and stale sample validity.',
 'EspThermal.h':'Loop-owned temperature driver with validated range/status and locked freshness snapshot.',
 'RuntimePerf.h':'Windowed nested component wall-time counters, not additive CPU load.',
 'RuntimeTrace.h':'RTC-only watchdog breadcrumbs, no allocating or flash-writing ISR.',
 'DeviceIdentity.h':'Factory eFuse identity read, no eFuse writes; attestation not claimed.',
 'FirmwareVersion.h':'Descriptive firmware identity only; no API version adaptation.',
 'CatalogAsset.h':'Operation catalog embedded in gzip FEXTRA; bounded input and canonical asset linkage.',
 'HardwareProfile.h':'Saved fitted-capability profile with prerequisite validation; pin details remain board-specific.',
 'ConfigExtrasStore.h':'NVS migration, bounded atomic file staging/readback; retains previous settings on failure.',
 'DiagnosticProfiles.h':'Generated modem AT groups; specific exclusion below.',
 'LedProfile.h':'Small status RGB settings record validation, not pixel/game logic; compare generic status profile separately.',
 'UartCapture.h':'Opt-in bounded PSRAM capture, explicit stop/read/clear, not automatic streaming.'
};
const other={
 'firmware/bringup_01/bringup_01.ino':'Read bringup identity/heap/watchdog diagnostics and bounded USB wait; compare generic diagnostics, do not copy board pin setup.',
 'firmware/common/board_pins.h':'Read complete carrier pin declarations; specific peripheral exclusion.',
 'firmware/common/UartIsolationProbe.h':'Read both standard/custom branches, counters/capture and bounded command input; UART utility vs AT/pin sections still need comparison.',
 'firmware/dependency_check/dependency_check.ino':'Read compile-only dependency header sketch; generic compile smoke vs peripheral dependencies remains pending comparison.',
 'firmware/i2c_scanner/i2c_scanner.ino':'Read complete physical scanner sketch; specific peripheral exclusion.',
 'firmware/modem_console/modem_console.ino':'Read complete AT passthrough sketch; specific peripheral exclusion.',
 'firmware/uart_custom_probe/uart_custom_probe.ino':'Read complete custom probe flag/include; shared probe mapping remains pending.',
 'firmware/uart_standard_probe/uart_standard_probe.ino':'Read complete standard probe include; shared probe mapping remains pending.',
 'firmware/modem_bridge/modem_bridge.ino':'Read setup/loop watchdog, stack and ownership wiring; not excluded with modem product logic.',
 'firmware/web/sw.js':'Read shell-only caching/activation/push routes; compare worker source-client validation and cache policies next.'
};
const readPlan=new Map([...Object.entries(notes).map(([p,n])=>['firmware/modem_bridge/'+p,n]),...Object.entries(other)]);
const source=new Map();
for(const [path,note] of readPlan){
 const bytes=readFileSync(resolve(donor,path)),pinned=git(donor,'show',`${revision}:${path}`);
 if(!bytes.equals(pinned))throw Error('Manual reread required; source no longer equals pinned bytes: '+path);
 source.set(path,{sha256:hash(bytes),lines:lf(bytes).split('\n').length-(bytes.at(-1)===10?1:0),note});
}
function ref(path,anchor){
 const full=resolve(root,path);if(relative(root,full).startsWith('..'))throw Error('Escaping target');
 const text=lf(readFileSync(full));if(!text.includes(anchor))throw Error('Missing target anchor '+path+' '+anchor);
 const expected=expectedTargets[path]??hash(lf(git(root,'show',`${targetRevision}:${path}`)));
 if(hash(text)!==expected)throw Error('Target changed outside pinned review: '+path);
 return {path,sha256:hash(text),anchor};
}
const store=()=>ref('firmware/esp/src/UserFileStore.h','template<class Digest> class UserFileStore');
const storeProof=()=>ref('tests/user_file_store_test.cpp','const auto beforeSpace=fs.writes;');
const budget=()=>ref('firmware/esp/src/UploadSpacePolicy.h','namespace PakarUploadSpace');
const comparisons=[];
function comparison(name,verdict,preserved,targets,proof,omissions,remaining){
 const path='firmware/modem_bridge/'+name,s=source.get(path);if(!s)throw Error('Unread source '+path);
 comparisons.push({origin:'Orbit',path,sha256:s.sha256,source_read:'complete',source_revision:revision,source_lines:s.lines,read_ranges:[{first:1,last:s.lines}],verdict,preserved:[preserved],targets,proof,omissions,remaining});
}
const pending=(code,reason)=>({code,disposition:'pending',reason});
comparison('WebAssetPolicy.h','parity-gap','Adapted aligned reserve, bounded arithmetic and fresh begin/chunk admission into managed user files.',[budget(),store()],[storeProof()],[pending('Live replaceable Web application asset upload','Pakar serves its single compiled Web application. A separate, verified asset-only publication path is not implemented here.')],['Finish same-source asset-only update design without a second UI codebase; SDK/physical storage verification pending.']);
comparison('EventStoragePolicy.h','parity-gap','Retained 64 KiB plus metadata block admission and 64-bit allocation rounding.',[budget()],[storeProof()],[pending('Event spool byte retention and RetryGate','Managed files use explicit request failure, not an autonomous durable event spool. Do not falsely equate the two.')],['Compare persistent event retention/retry owner and implement generic missing behavior if applicable.']);
comparison('HttpDate.h','equivalent-behavior','Calendar, strict weekday, 2024..2099 UTC and invalid-input rejection align; Orbit upgraded from unchecked weekday/2039 cutoff.',[ref('firmware/esp/src/NetworkObservationPolicy.h','inline bool httpDate(')],[ref('tests/network_observation_test.cpp','uint32_t epoch=0;')],[],[]);
comparison('HttpRepresentationPolicy.h','equivalent-behavior','Actual Pakar encoding/ETag parser adapted into Orbit. Both reject a matching tag followed by malformed list data.',[ref('firmware/esp/src/StaticAssetPolicy.h','inline bool notModified(')],[ref('tests/async_http_test.cpp','assert(!notModified("\\\"abc\\\", malformed"')],[],[]);
comparison('VerifiedFileWrite.h','equivalent-behavior','Orbit now reuses flushed seek/readback verification before offset/hash progress, as Pakar managed chunks already do.',[store()],[storeProof()],[],[]);
comparison('HttpFileValidators.h','parity-gap','Shared representation-specific validator rules compared; strong If-Range exact-date bug fixed in Orbit.',[ref('firmware/esp/src/StaticAssetPolicy.h','inline bool notModified(')],[ref('tests/async_http_test.cpp','assert(notModified("*"')],[pending('HTTP date/If-Range for managed user files','Pakar exposes logical file reads over its guarded API; this is not equivalent to Orbit public-file HTTP date/range serving.')],['Add date/If-Range only to appropriately guarded logical-file HTTP representations; no arbitrary private filesystem routes.']);
comparison('StorageFiles.h','parity-gap','Sandboxed bytes/date metadata, guarded managed APIs and readback/atomic publication compared. Orbit gained readback and HTTP negotiation.',[store(),ref('firmware/esp/src/StaticAssetService.h','inline void serveStaticAsset(')],[storeProof()],[pending('HTTP public-file date/range serving and protocol parity','Pakar logical API/CAS/tokens differ from Orbit raw public serving. Keep each admission/ownership policy; do not call transport coverage identical.')],['Compare live HTTP/USB/MQTT cancellation/ownership and resource bounds; SDK and physical filesystem tests pending.']);
comparison('WebAsset.h','parity-gap','Serialized checked temporary publication compared with managed file publication; both retain previous bytes until publication.',[store(),ref('tools/build-web.mjs','const receipt={application:')],[storeProof()],[pending('Asset-only live Web replacement','Pakar compiled application delivery currently requires firmware; managed user files alone do not replace that application.')],['Implement canonical-source asset generation/publication after ownership/size/resource review; no duplicate Web UI.']);
comparison('FirmwareWifiRange.h','parity-gap','Compared strict response ranges against Pakar bounded request-range parser and fail-closed non-resuming remote downloader.',[ref('firmware/esp/src/FirmwareRange.h','inline bool parseRange('),ref('firmware/esp/src/RemoteFirmware.cpp','const int code=http_.GET(),length=http_.getSize();')],[ref('tests/firmware_range_test.cpp','FirmwareRange range{};')],[pending('Validated immutable-download-only resume','Pakar rejects interrupted downloads. Introducing safe GET resume is distinct from retrying apply/install, which remains forbidden.')],['Design pinned URL/hash/size/offset recovery with strict 206 validation, never repeat mutation or install.']);
comparison('AtomicEventRecord.h','parity-gap','Checked staging/readback/rename is retained by Pakar managed files; event identities are not relabelled file-transfer tokens.',[store()],[storeProof()],[pending('Durable queued event identity/no-overwrite spool','Pakar managed files are not an equivalent durable telemetry event queue.')],['Review actual event persistence and optional bounded spool requirements separately.']);
const exclusions={
 'firmware/common/board_pins.h':'Orbit carrier UART/DTR/NET/CAN/K-Line physical map conflicts with reviewed Pakar wiring.',
 'firmware/i2c_scanner/i2c_scanner.ino':'Physical I2C scanner peripheral sketch tied to that board map, outside requested ESP utility adaptation.',
 'firmware/modem_console/modem_console.ino':'A7670E AT UART passthrough, no generic OTA/Web/API implementation.',
 'firmware/modem_bridge/DiagnosticProfiles.h':'Generated modem AT groups and modem response-field invalidation only.',
 'firmware/modem_bridge/HttpActionResult.h':'SIMCom HTTPACTION/HTTPPOSTFILE AT grammar; not a Pakar carrier.',
 'firmware/modem_bridge/HttpBinaryFrames.h':'SIMCom counted HTTPREAD response envelope; not interchangeable with Pakar USB/COBS framing.'
};
for(const [path,reason]of Object.entries(exclusions)){
 const s=source.get(path);if(!s)throw Error('Unread exclusion '+path);
 comparisons.push({origin:'Orbit',path,sha256:s.sha256,source_read:'complete',source_revision:revision,source_lines:s.lines,read_ranges:[{first:1,last:s.lines}],verdict:'not-applicable',preserved:['Source remains preserved in Orbit; exclusion does not hide its inventory row.'],targets:[ref('docs/ORBIT-FIRMWARE-PARITY.md','`'+path+'`')],proof:[],omissions:[{code:path,disposition:'not-applicable',reason}],remaining:[]});
}
const manifest=JSON.parse(readFileSync(resolve(root,'docs/file-comparison-evidence.json'),'utf8'));
// Only refs affected by this task's reviewed diffs may change. Do not rebind
// concurrent donor/target edits, invent completion, or erase prior omissions.
for(const [path,expected]of Object.entries(expectedTargets))if(hash(lf(readFileSync(resolve(root,path))))!==expected)throw Error('Target changed outside pinned review: '+path);
const changed=new Set(Object.keys(expectedTargets));
for(const item of manifest.files)if(item.origin!=='Orbit')for(const r of [...item.targets,...item.proof])if(changed.has(r.path)){
 const fresh=ref(r.path,r.anchor),old=hash(lf(git(root,'show','HEAD:'+r.path)));
 if(r.sha256!==old&&r.sha256!==fresh.sha256)throw Error('Unowned target drift: '+r.path);
 r.sha256=fresh.sha256;
}
manifest.files=manifest.files.filter(f=>f.origin!=='Orbit').concat(comparisons);
write('docs/file-comparison-evidence.json',JSON.stringify(manifest,null,2)+'\n');
const reviews=csv(readFileSync(resolve(root,'docs/reuse-review-evidence.csv'),'utf8')).filter(f=>f.origin!=='Orbit');
for(const [path,s]of source)reviews.push({origin:'Orbit',path,sha256:s.sha256,review_status:exclusions[path]?'reviewed-excluded':comparisons.some(c=>c.path===path)?'compared':'complete-read-awaiting-comparison',reuse_target:'docs/ORBIT-FIRMWARE-PARITY.md',notes:`Read complete text lines 1-${s.lines} at ${revision}. ${s.note} Target comparison remains separate; no full SDK/hardware proof.`});
const keys=['origin','path','sha256','review_status','reuse_target','notes'];
const quote=s=>'"'+String(s).replaceAll('"','""')+'"';
write('docs/reuse-review-evidence.csv',keys.join(',')+'\n'+reviews.map(r=>keys.map(k=>quote(r[k])).join(',')).join('\n')+'\n');
console.log(JSON.stringify({revision,completely_read:source.size,compared:comparisons.length-Object.keys(exclusions).length,excluded:Object.keys(exclusions).length,meaning:'Pinned manual-read checkpoint, not complete parity or test execution'}));
