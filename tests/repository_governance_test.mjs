import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {releaseIdentity,firmwareManifest,checksumList,sha256} from '../tools/prepare-release.mjs';
import {runChecks} from '../tools/ci-checks.mjs';
const read=path=>readFileSync(new URL('../'+path,import.meta.url),'utf8').replaceAll('\r\n','\n');
const build=read('.github/workflows/build.yml'),security=read('.github/workflows/codeql.yml');
// CI actionlint invokes ShellCheck on Linux; Windows may lack that executable.
// Preserve the donor's grouped summary redirection instead of suppressing SC2129.
const securitySummary=security.split('        run: |')[1];
assert.equal((securitySummary.match(/>>\s*"\$GITHUB_STEP_SUMMARY"/g)||[]).length,1);
assert.match(securitySummary,/\{[\s\S]*Actions and JavaScript result:[\s\S]*\}\s*>>\s*"\$GITHUB_STEP_SUMMARY"/);
assert.match(securitySummary,/test "\$RESULT" = success/);
assert.doesNotMatch(security,/shellcheck disable/);
for(const workflow of [build,security]){
  assert.match(workflow,/concurrency:/);assert.match(workflow,/timeout-minutes:/);
  for(const match of workflow.matchAll(/uses: ([^\s#]+)/g))assert.match(match[1],/@[a-f0-9]{40}$/,'Actions must be pinned');
  assert(!workflow.includes('pull_request_target'),'Untrusted PR execution must not gain write credentials');
}
assert.match(build,/persist-credentials: false/);
assert.match(build,/needs: \[lint, quality, firmware, windows-desktop, android-preview\]/);
assert.match(build,/--draft --prerelease/);assert.match(build,/attest-build-provenance@/);
assert.match(build,/retention-days: 7/);assert.match(build,/workspaces: desktop -> target/);
assert.match(build,/node tools\/ci-checks.mjs (quality|desktop)/);
const firmwareJob=build.split('  firmware:')[1].split('  windows-desktop:')[0];
assert.match(firmwareJob,/name: "🔧 ESP32-S3 \+ AVR"/);
for(const mode of ['polling','busy-wire','buffered'])assert(firmwareJob.includes(`bash build.sh ${mode} avr`));
for(const mode of ['','-busy-wire','-buffered'])for(const file of ['avr.ino.hex','avr.ino.with_bootloader.hex'])
  assert(firmwareJob.includes(`build/avr${mode}/${file}`),'CI must publish both application and bootloader-inclusive HEX');
assert.doesNotMatch(firmwareJob,/build\/esp8266|bash build.sh (?:polling|busy-wire)\s*\n/,
  'Primary releases must not require the unfinished, resource-overflowing ESP8266 image');
assert(firmwareJob.indexOf('bash tools/test-protocol.sh')<firmwareJob.indexOf('bash build.sh polling avr'),
  'Host-contract setup must fail before expensive SDK compilation');
const canonicalS3Command='bash build.sh buffered esp32s3 --s3-flash-mb 16 --s3-psram opi --s3-led-gpio --s3-led-rmt-dma';
assert.equal(firmwareJob.split(canonicalS3Command).length-1,1,'Canonical S3 artifact must explicitly include both runtime LED drivers');
assert(firmwareJob.indexOf(canonicalS3Command)>firmwareJob.indexOf('-S3Ethernet -Jobs 2'));
assert(firmwareJob.indexOf(canonicalS3Command)<firmwareJob.indexOf('actions/upload-artifact@'));
const hostProtocol=read('tools/test-protocol.sh');
assert.match(hostProtocol,/mkdir -p "\$root\/build"/,'Early host tests must work on a fresh checkout');
for(const target of ['led_scheduler_test.cpp','led_worker_test.cpp']){
  const command=hostProtocol.split(/(?<=\n)(?=c\+\+|  c\+\+)/).find(command=>command.includes(target));
  assert(command?.includes('-I "$root/libraries/BoardCore/src"'),`${target} requires actual GameRole headers`);
}
const pcFlags=read('tools/test-pc-flags.sh');
assert.match(pcFlags,/for feature in I2C SENSORS PWM RELAYS STATUS_RGB AVR_LED; do/);
assert.match(pcFlags,/pwsh -NoProfile -File "\$root\/tools\/test-radio.ps1"/,
  'Implemented RF needs its actual positive adapter tests, not an obsolete expected compilation failure');
assert.match(pcFlags,/for value in -1 2; do/);
assert.match(read('libraries/BoardCore/src/BoardFeatures.h'),/implemented = Menu \| Macros \| Automations \| Rf/);
assert.match(firmwareJob,/actions\/setup-node@[a-f0-9]{40}[\s\S]*node-version: '24'/,
  'Firmware asset verification must use the same Node major as source quality');
const quietSdkCommand='pwsh -NoProfile -File tools/build-firmware.ps1 -Target esp32s3 -LinkMode buffered -S3FlashMB 16 -S3Psram opi -S3LedGpio -S3LedRmtDma -PauseAvrLink -Jobs 2';
const normalSdkCommand='pwsh -NoProfile -File tools/build-firmware.ps1 -Target esp32s3 -LinkMode buffered -S3FlashMB 16 -S3Psram opi -S3LedGpio -S3LedRmtDma -S3Ethernet -Jobs 2';
assert.equal(firmwareJob.split(quietSdkCommand).length-1,1,'Compile the quiet-only SDK adapter exactly once through the guarded wrapper');
assert.equal(firmwareJob.split(normalSdkCommand).length-1,1,'Keep one canonical normal S3 build');
assert(firmwareJob.indexOf(quietSdkCommand)<firmwareJob.indexOf(normalSdkCommand),
  'Normal S3 output must replace the quiet diagnostic image in the shared cache/output before upload');
assert(firmwareJob.indexOf(normalSdkCommand)<firmwareJob.indexOf('actions/upload-artifact@'),
  'Never publish the quiet diagnostic output as canonical firmware');
assert.doesNotMatch(firmwareJob,/--clean|\s-Clean\b/,'Variant coverage must reuse the warm cache');
assert.doesNotMatch(firmwareJob,/esp32s3-(quiet|passive)/,'Do not create a separate diagnostic cache/package');
const nativeTestStep=build.split('      - name: Test and build the mpv-free Windows client')[1].split('      - uses:')[0];
assert.match(nativeTestStep,/PAKAR_CONFIG_FILE: \$\{\{ runner.temp \}\}\/pakar-tests.json/,
  'Native tests must not access a user configuration');
assert.match(nativeTestStep,/RUST_BACKTRACE: '1'/,
  'Keep actual donor failure backtraces enabled in native CI');
let calls=[];
assert.throws(()=>runChecks([['first'],['never']],cmd=>{calls.push(cmd);return {status:7};}),error=>error.exitCode===7);
assert.deepEqual(calls,['first'],'Do not execute later commands after a failure');
calls=[];
assert.throws(()=>runChecks([['first'],['never']],cmd=>{calls.push(cmd);return {error:Error('spawn failed')};}),/spawn failed/);
assert.deepEqual(calls,['first']);
assert.throws(()=>runChecks([['terminated']],()=>({status:null})),error=>error.exitCode===1);
let passed=0;runChecks([['first'],['second']],()=>{passed++;return {status:0};});assert.equal(passed,2);
assert.match(read('.gitattributes'),/text=auto eol=lf/);
const settings=JSON.parse(read('.github/repository-settings.json'));
assert.equal(settings.repository,'atomicdeploy/PakarController');
assert.equal(settings.workflow_permissions.default_workflow_permissions,'read');
assert.equal(settings.workflow_permissions.can_approve_pull_request_reviews,false);
const labels=JSON.parse(read('.github/labels.json'));
assert.equal(new Set(labels.map(x=>x.name)).size,labels.length);
for(const label of labels){assert.match(label.color,/^[A-F0-9]{6}$/);assert(label.description);}
for(const name of ['domain: repository','domain: firmware','domain: desktop','domain: webui','domain: mobile','acceptance needed'])assert(labels.some(x=>x.name===name));
const identity=releaseIdentity('atomicdeploy/PakarController','v0.2.7-preview','a'.repeat(40),'0.2.7');
assert(identity.draft&&identity.prerelease);
for(const [tag,commit] of [['v0.2.8','a'.repeat(40)],['branch-main','a'.repeat(40)],['v0.2.7','main']])assert.throws(()=>releaseIdentity(identity.repository,tag,commit,'0.2.7'));
const esp=Buffer.alloc(64);esp[0]=0xe9;esp[1]=1;
const avr=Buffer.alloc(32768,0xff);avr[0]=0x0c;avr[1]=0x94;avr[32767]=0;
for(const mode of ['polling','busy-wire']){
  const value=firmwareManifest(esp,avr,identity,mode);
  assert.equal(value.link_mode,mode);assert.equal(value.avr.size,32768);assert.equal(value.avr.bootloader_included,true);
  assert.equal(value.esp.sha256,sha256(esp));assert.equal(value.avr.sha256,sha256(avr));
  assert(value.esp.url.endsWith(`/esp-${mode}.bin`));assert(value.avr.url.endsWith(`/avr-${mode}.bin`));
}
const s3=Buffer.from(esp);s3.writeUInt16LE(9,12);s3[23]=1;
const s3Value=firmwareManifest(s3,avr,identity,'buffered');
assert.equal(s3Value.profile,'pakar-esp32s3-atmega328p');
assert.equal(s3Value.esp.model,'esp32s3');assert.equal(s3Value.link_mode,'buffered');
assert.equal(s3Value.esp.sha256,sha256(s3));
assert.throws(()=>firmwareManifest(esp,avr,identity,'buffered'),/ESP32-S3/);
assert.throws(()=>firmwareManifest(esp,avr.subarray(0,32000),identity,'busy-wire'));
assert.throws(()=>firmwareManifest(esp,avr,identity,'mixed'));
assert.equal(checksumList([{name:'b.bin',data:esp},{name:'a.bin',data:avr}]),`${sha256(avr)}  a.bin\n${sha256(esp)}  b.bin\n`);
assert.throws(()=>checksumList([{name:'../escape',data:esp}]));
assert(read('README.md').includes('simulated device state'));
assert(read('SECURITY.md').includes('not authentication'));
assert(read('docs/REPOSITORY-GOVERNANCE.md').includes('not whole-project superiority'));
console.log('Repository governance, primary S3 and explicit legacy manifests, immutable release identity, malformed image/tag and checksum tests passed. Source tests are not publication or hardware proof.');
