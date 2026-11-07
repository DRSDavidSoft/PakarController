import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';

// Source guard of the actual lifecycle owners, not an executed SDK/OTA test.
const source = readFileSync(new URL('../firmware/esp/src/MaintenanceRoutes.h', import.meta.url), 'utf8');
const remote = readFileSync(new URL('../firmware/esp/src/RemoteFirmware.cpp', import.meta.url), 'utf8');
const prepare = source.slice(source.indexOf('bool prepareRemotePair('), source.indexOf('bool programRemoteAvr('));
const finish = source.slice(source.indexOf('int8_t finishRemotePair('), source.indexOf('bool strictU32('));
for (const body of [prepare, finish]) {
  assert.doesNotMatch(body, /#if|#else|#endif|PAKAR_AVR_LINK_ENABLED/,
    'Explicit ESP-only lifecycle must not depend on AVR-link compile flags');
  assert.match(body, /if\(plan.targets==RemotePolicy::Esp && !programmingLatched\)/,
    'ESP-only bypass is forbidden for a known recovery latch or AVR/all targets');
}
const early = prepare.indexOf('if(plan.targets==RemotePolicy::Esp');
const maintenanceGuard = prepare.indexOf('artifactReaders || otaActive || otaRebootAt || avrUploadOwner || networkApplyPending || cloudApplyPending');
assert.ok(maintenanceGuard >= 0 && maintenanceGuard < early);
assert.ok(early < prepare.indexOf('waitForAvr()'), 'Optional AVR cannot block independent ESP OTA');
assert.match(prepare, /if\(plan.targets==RemotePolicy::Esp && !programmingLatched\) return true;/);
assert.match(prepare, /!avrVersionKnown \|\| !avrPcSupported/);
assert.match(prepare, /plan.targets==1 && plan.esp.version!=avrVersionText\(\)/);
assert.match(prepare, /plan.targets==2 && plan.avr.version!=BoardCore::BuildInfo::kVersion/);
assert.match(prepare, /RemotePolicy::compare\(plan.avr.version.c_str\(\),avrVersionText\(\).c_str\(\)\)<0/);
assert.match(prepare, /requestAndWaitForAck\(kProgrammingState,armed,1\)/);
assert.match(prepare, /avrIsp.backup\(LittleFS,kBackupDirectory\)/);
assert.match(prepare, /flash.size\(\)==32768[\s\S]*eeprom.size\(\)==1024/);
const independentFinish = finish.slice(0, finish.indexOf('if(!avrOnline'));
assert.match(independentFinish, /plan.esp.version!=BoardCore::BuildInfo::kVersion/);
assert.match(independentFinish, /return -1/);
assert.match(independentFinish, /return 1;/);
assert.doesNotMatch(independentFinish, /requestAndWaitForAck|playUpdate\w+Cue\(|programmingLatched\s*=|digitalWrite|pinMode|avrIsp\./,
  'Independent completion cannot manipulate AVR pins, Prog, ISP or audio');
assert.match(finish, /if\(!avrOnline \|\| !avrVersionKnown \|\| !avrPcSupported\)/);
assert.match(finish, /if\(!commit\) return 1;/);
assert.match(finish, /requestAndWaitForAck\(kProgrammingState,released,1\)/);
assert.match(finish, /programmingLatched=false;return 1;/);
assert.doesNotMatch(prepare,/playUpdateStartCue\(/,'A missing melody is never an erase-safety acknowledgement');
assert.match(remote, /plan_.esp.version!=BoardCore::BuildInfo::kVersion \|\| !FirmwarePlatform::runningIntegrity\(\)/);
assert.match(remote, /if\(digest\(c\)!=plan_.esp.sha\) fail/);
const network = source.slice(source.indexOf('String networkTelemetry()'), source.indexOf('String buildTelemetryJson()'));
assert.match(network, /d\[F\("uplink"\)\]\[F\("interface"\)\]=NetworkPath::interfaceName\(\)/);
assert.match(network, /d\[F\("uplink"\)\]\[F\("ipv4_ready"\)\]=NetworkPath::ready\(\)/);
for(const [field, getter] of [['configured','configured'],['started','started'],['link_up','linkUp'],['ipv4_ready','connected'],['ip','localIP'],['mac','macAddress'],['gateway','gatewayIP'],['netmask','subnetMask'],['dns0','dnsIP'],['dns1','dnsIP'],['last_error','lastError']]) {
  assert.ok(network.includes(`"${field}"`), `Ethernet ${field} must be exposed`);
  assert.ok(network.includes(`ethernet.${getter}(`), `Ethernet ${field} requires its actual getter`);
}
assert.match(network, /if\(ethernetStarted\)[\s\S]*else[\s\S]*\[field\]=nullptr/);
const telemetry = source.slice(source.indexOf('String buildTelemetryJson()'), source.indexOf('bool prepareRemotePair('));
assert.match(telemetry, /d\[F\("board_id"\)\]=pcBoardId;d\[F\("boot_id"\)\]=pcBootId/);
// Immutable running-image metadata is captured once on boot, not recomputed
// for every Web/USB/cloud telemetry snapshot. OTA admission above still uses
// RemoteFirmware's separate fresh integrity/hash validations.
const sketch=readFileSync(new URL('../firmware/esp/application.ino',import.meta.url),'utf8');
function checkBootImageCache(routes,main) {
  const first=routes.indexOf('String buildTelemetryJson()'),last=routes.indexOf('bool prepareRemotePair(');
  assert.ok(first>=0 && last>first);
  const body=routes.slice(first,last);
  assert.doesNotMatch(body,/FirmwarePlatform::(?:sketchSize|sketchMd5|flashChipSpeed|flashChipMode)\s*\(/);
  for(const field of ['flashHz','flashMode','size','md5','capturedAt'])
    assert.ok(body.includes(`bootTelemetryImage.${field}`),`${field} must use the boot observation`);
  assert.match(routes,/void captureBootTelemetryImage\(\) \{[\s\S]*?bootTelemetryImage.size=FirmwarePlatform::sketchSize\(\);[\s\S]*?bootTelemetryImage.md5=FirmwarePlatform::sketchMd5\(\);[\s\S]*?bootTelemetryImage.flashHz=FirmwarePlatform::flashChipSpeed\(\);[\s\S]*?bootTelemetryImage.flashMode=static_cast<int>\(FirmwarePlatform::flashChipMode\(\)\);/);
  assert.match(routes,/void registerMaintenanceRoutes\(\) \{\s+bootIntegrity=FirmwarePlatform::runningIntegrity\(\);\s+captureBootTelemetryImage\(\);/);
  assert.equal((routes.match(/captureBootTelemetryImage\(\);/g)||[]).length,1);
  const setup=main.indexOf('void setup()'),loop=main.indexOf('void loop()');
  assert.ok(setup>=0 && loop>setup);
  assert.ok(main.slice(setup,loop).includes('registerMaintenanceRoutes();'));
  assert.doesNotMatch(main.slice(setup,loop),/cloudServices.service\(/);
  assert.doesNotMatch(main.slice(loop),/captureBootTelemetryImage\(/);
}
checkBootImageCache(source,sketch);
assert.throws(()=>checkBootImageCache(source.replace('String buildTelemetryJson() {','String buildTelemetryJson() { FirmwarePlatform::sketchMd5();'),sketch));
assert.throws(()=>checkBootImageCache(source,sketch.replace(/^  registerMaintenanceRoutes\(\);/m,'')));
console.log('ESP-only maintenance source guards passed: compile-independent optional AVR, maintenance/latch gates and existing paired recovery retained. Not an SDK build, OTA or physical proof.');

// General USB chunks must drain after a stage/apply changes the job to busy.
// That carrier exception never grants another API mutation or raw AVR access.
const usbApi=readFileSync(new URL('../firmware/esp/src/UsbApiService.h',import.meta.url),'utf8');
function checkUsbMaintenance(main,api){
  assert.match(main,/const bool remoteApiDrain=frame.opcode==kUsbApi && remoteFirmware.busy\(\) &&\s*!urbootBusy.get\(\)&&!otaActive&&!otaRebootAt&&!avrUploadOwner/);
  assert.match(main,/!shellPoll && !remoteApiDrain/);
  const fence=api.indexOf('return maintenanceApiError("peer_changed")');
  const busy=api.indexOf('PeerUpdatePolicy::duringMaintenance(method.c_str(),path.c_str())');
  const dispatch=api.indexOf('remoteFirmware.stagePeer(');
  assert.ok(fence>=0&&busy>fence&&dispatch>busy);
  assert.match(main,/return !usbApiReceiptPending\(\)&&upgradeLifecycle.readyToReboot/);
  assert.match(api,/usbApiRequest.receiptPending\(now\)\|\|usbApiClosing/);
  assert.match(api,/usbApiClosedAt=millis\(\);usbApiClosing=true/);
}
checkUsbMaintenance(sketch,usbApi);
assert.throws(()=>checkUsbMaintenance(sketch.replace(' && !remoteApiDrain',''),usbApi));
assert.throws(()=>checkUsbMaintenance(sketch,usbApi.replace('PeerUpdatePolicy::duringMaintenance(method.c_str(),path.c_str())','true')));
assert.throws(()=>checkUsbMaintenance(sketch.replace('!usbApiReceiptPending()&&',''),usbApi));
