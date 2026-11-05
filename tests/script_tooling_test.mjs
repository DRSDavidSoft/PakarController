// Offline and child-process fixtures only. No compiler, device or SDK install.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync,mkdirSync,mkdtempSync,existsSync,unlinkSync,symlinkSync,realpathSync,openSync,ftruncateSync,closeSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {resolve} from 'node:path';
import {spawnSync} from 'node:child_process';
import {createLogger,sanitizeText,runChild} from '../tools/script-console.mjs';
import {main,portableArguments,legacyBuildArguments,doctor,scriptPath} from '../tools/pakar.mjs';
import {compareVersions,installationPlan,validatePins,checkDependencies,applyUpdates,dependencyMarkdown,fetchIndex} from '../tools/firmware-dependencies.mjs';
import {collectArtifacts,readArtifact,expectedArtifacts,legacyArtifacts,unchangedWrite,artifactMarkdown,validateAvrHex} from '../tools/firmware-report.mjs';
import {encodeHex} from '../tools/prepare-avr-image.mjs';
import {trustedBuild,verifiedArtifact,deployArtifact} from '../tools/deploy-artifact.mjs';
const read=path=>readFileSync(new URL('../'+path,import.meta.url),'utf8');
const memory=()=>({value:'',write(text){this.value+=text;}});
const out=memory(),err=memory(),logger=createLogger({out,err,json:true});
logger.step('Firmware');logger.error('\x1b[31munsafe\x07');
assert.equal(JSON.parse(out.value).kind,'step');assert.equal(JSON.parse(err.value).message,'unsafe');
assert.equal(sanitizeText('\x1b]0;title\x07valid'),'valid');
assert.ok(!out.value.includes('\x1b'));const quietOut=memory(),quietErr=memory();
const redirectedErr=memory(),ttyOut={...memory(),isTTY:true};
createLogger({out:ttyOut,err:redirectedErr,env:{}}).error('plain redirected error');assert.ok(!redirectedErr.value.includes('\x1b'));
const redirectedOut=memory(),ttyErr={...memory(),isTTY:true};
const independent=createLogger({out:redirectedOut,err:ttyErr,env:{}});
independent.info('plain redirected info');independent.error('interactive error');
assert.ok(!redirectedOut.value.includes('\x1b'));assert.ok(ttyErr.value.includes('\x1b[91m'));
for(const env of [{NO_COLOR:''},{TERM:'dumb'}]){
  const stderr={...memory(),isTTY:true};createLogger({out:memory(),err:stderr,env}).error('plain by policy');assert.ok(!stderr.value.includes('\x1b'));
}
const quiet=createLogger({out:quietOut,err:quietErr,quiet:true});quiet.info('hidden');quiet.error('retained');assert.equal(quietOut.value,'');assert.match(quietErr.value,/retained/);
assert.deepEqual(portableArguments(['--s3-flash-mb','16','--link-mode=buffered','-Plan']),['-S3FlashMb','16','-LinkMode','buffered','-Plan']);
assert.deepEqual(legacyBuildArguments(['buffered','esp32s3','--plan']),['-LinkMode','buffered','-Target','esp32s3','-Plan']);
assert.deepEqual(legacyBuildArguments(['polling','--plan']),['-LinkMode','polling','-Target','both','-Plan']);
assert.throws(()=>legacyBuildArguments(['invalid']));assert.throws(()=>scriptPath('../outside'));
assert.throws(()=>scriptPath('missing-command'));
let calls=[];const runner=async(file,args,options)=>{calls.push({file,args,options});return {exit_code:7,elapsed_ms:12};};
assert.equal(await main(['--json','firmware','--target','esp32s3','--plan'],{runner,out:memory(),err:memory()}),7);
assert.equal(calls.length,1);assert.equal(calls[0].file,'pwsh');assert.ok(calls[0].args.includes('-Plan'));assert.ok(!calls[0].args.includes('-Clean'));
assert.equal(await main(['firmware','--help'],{runner,out:memory(),err:memory()}),0);assert.equal(calls.length,1);
assert.equal(await main(['no-such-command'],{runner,out:memory(),err:memory()}),2);
const inventory=doctor({probe:()=>({status:0,stdout:'fixture tool\n'})});assert.equal(inventory.device_contacted,false);assert.equal(inventory.installed,false);
assert.ok(inventory.tools.every(row=>row.available));
const child=await runChild(process.execPath,['-e','console.error("fixture diagnostic");process.exit(7)'],{json:true,logger:createLogger({json:true,out:memory(),err:memory()})});assert.equal(child.exit_code,7);
await assert.rejects(runChild('pakar-certainly-missing-executable',[],{logger}),/Required command/);
assert.equal(compareVersions('1.10.0','1.9.9'),1);assert.equal(compareVersions('2.8','2.8.0'),0);
assert.throws(()=>compareVersions('1.2.0-rc1','1.2.0'));
const pins=validatePins(JSON.parse(read('tools/firmware-dependencies.json')));
const plan=installationPlan(pins);assert.ok(plan.some(args=>args.includes('esp32:esp32@3.3.12')));
assert.ok(plan.every(args=>!args.includes('--clean')));
assert.throws(()=>validatePins({...pins,libraries:[...pins.libraries,pins.libraries[0]]}));
assert.throws(()=>validatePins({...pins,library_index:'http://insecure.invalid/index'}));
let proxyCalls=0;
const proxyEnv={HTTPS_PROXY:'http://operator:secret@localhost:8118'};
assert.deepEqual(await fetchIndex('https://registry.example/index',{env:proxyEnv,curl:(file,args,options)=>{
  proxyCalls++;assert.equal(file,'curl');assert.ok(!args.join(' ').includes('secret'));assert.equal(options.env,proxyEnv);assert.equal(options.shell,false);
  return {status:0,stdout:Buffer.from('{"packages":[]}')};
}}),{packages:[]});assert.equal(proxyCalls,1);
await assert.rejects(fetchIndex('https://registry.example/index',{env:proxyEnv,curl:()=>({status:5,stderr:'secret'})}),error=>error.exitCode===4&&!error.message.includes('secret'));
await assert.rejects(fetchIndex('http://registry.example/index'),/direct HTTPS/);
assert.deepEqual(await fetchIndex('https://registry.example/index',{env:{},fetcher:async()=>new Response('{"libraries":[]}')}),{libraries:[]});
const selected={platforms:[{name:'fixture:avr',version:'1.9.9',index:'https://registry.example/platform.json'}],library_index:'https://registry.example/libs.json',libraries:[{name:'Example lib',version:'2.8.0'},{name:'Ahead',version:'9.0.0'}]};
let reads=0;
const rows=await checkDependencies(selected,{readIndex:async url=>{reads++;return url.endsWith('platform.json')?{packages:[{name:'fixture',platforms:[{architecture:'avr',version:'1.9.9'},{architecture:'avr',version:'1.10.0'},{architecture:'avr',version:'2.0.0-beta'}]}]}:{libraries:[{name:'Example lib',version:'2.8'},{name:'Ahead',version:'8.0.0'}]};}});
assert.equal(reads,2);assert.equal(rows[0].latest,'1.10.0');assert.equal(rows[0].update,true);assert.equal(rows[1].update,false);assert.equal(rows[2].update,false);
assert.match(dependencyMarkdown(rows),/Pinned ahead; no downgrade/);
const fixture=resolve('build/native/script-tooling-fixture');mkdirSync(fixture,{recursive:true});
const pinPath=resolve(fixture,'pins.json'),before=JSON.stringify(selected);writeFileSync(pinPath,before);
if(existsSync(pinPath+'.pending'))unlinkSync(pinPath+'.pending'); // Only this fixture's known tiny file.
assert.equal(applyUpdates(pinPath,before,selected,rows),true);assert.equal(JSON.parse(readFileSync(pinPath)).platforms[0].version,'1.10.0');
assert.throws(()=>applyUpdates(pinPath,before,selected,rows),/changed/);
const artifacts=mkdtempSync(resolve(tmpdir(),'PakarController-artifacts-test-'));mkdirSync(resolve(artifacts,'esp32s3-buffered'),{recursive:true});
const image=Buffer.alloc(96);image[0]=0xe9;image[1]=1;image.writeUInt16LE(9,12);image[23]=1;
const imagePath=resolve(artifacts,'esp32s3-buffered/esp32s3.ino.bin');writeFileSync(imagePath,image);
// Adapt actual PU CLI subprocess/no-contact fixture, but keep Pakar raw images
// and the current JSON acknowledgement/new-boot contract, not ASA gzip/ok!.
const cliPlan=spawnSync(process.execPath,['tools/update-firmware.mjs','--image',imagePath,'--origin','http://localhost:1'],{encoding:'utf8',timeout:10000,windowsHide:true});
assert.equal(cliPlan.status,0,cliPlan.stderr);assert.equal(JSON.parse(cliPlan.stdout).device_contacted,false);
const invalidCli=spawnSync(process.execPath,['tools/update-firmware.mjs','--image',imagePath,'--mode','unsupported','--run','--origin','http://localhost:1'],{encoding:'utf8',timeout:10000,windowsHide:true});
assert.notEqual(invalidCli.status,0);assert.match(invalidCli.stderr,/Mode must be/);assert.doesNotMatch(invalidCli.stderr,/ECONNREFUSED/);
const observed=collectArtifacts(artifacts);assert.equal(observed.length,1);assert.equal(observed[0].format,'raw');assert.ok(!existsSync(imagePath+'.gz'));
assert.throws(()=>collectArtifacts(artifacts,{required:true}),/Missing expected/);
assert.equal(expectedArtifacts.length,10);assert(expectedArtifacts.every(name=>!name.startsWith('esp8266')));
assert.throws(()=>validateAvrHex(Buffer.from(':00000001FE\n')),/checksum/);
assert.throws(()=>validateAvrHex(Buffer.from([0xff])),/Non-ASCII/);
assert.equal(validateAvrHex(Buffer.from(encodeHex(Buffer.alloc(32)))),32);
for(const name of expectedArtifacts.filter(name=>name.startsWith('avr'))){
  const path=resolve(artifacts,name);mkdirSync(resolve(path,'..'),{recursive:true});const data=Buffer.alloc(32768);writeFileSync(path,name.endsWith('.hex')?encodeHex(data):data);
}
assert.equal(collectArtifacts(artifacts,{required:true}).length,10);
assert.throws(()=>collectArtifacts(artifacts,{required:true,includeLegacy:true}),/Missing expected.*esp8266/);
for(const name of legacyArtifacts){
  const path=resolve(artifacts,name);mkdirSync(resolve(path,'..'),{recursive:true});writeFileSync(path,image);
}
assert.equal(collectArtifacts(artifacts,{required:true}).length,10,'Old cached legacy artifacts are not primary release artifacts');
assert.equal(collectArtifacts(artifacts,{required:true,includeLegacy:true,compress:true}).length,14,'Explicit legacy raw/gzip collection is preserved');
const badHex=resolve(artifacts,'avr/avr.ino.with_bootloader.hex');
writeFileSync(badHex,encodeHex(Buffer.alloc(32768,1)));
assert.throws(()=>collectArtifacts(artifacts,{required:true}),/HEX differs/);
writeFileSync(badHex,encodeHex(Buffer.alloc(32768)));
assert.deepEqual(readArtifact(imagePath,realpathSync(artifacts)),image,'Actual descriptor read retains original application bytes');
const emptyPath=resolve(artifacts,'empty.bin'),oversizedPath=resolve(artifacts,'oversized.bin'),aliasPath=resolve(artifacts,'alias.bin');
writeFileSync(emptyPath,Buffer.alloc(0),{flag:'wx'});
assert.throws(()=>readArtifact(emptyPath,realpathSync(artifacts)),/size/);
const oversizedFd=openSync(oversizedPath,'wx');try{ftruncateSync(oversizedFd,4*1024*1024+1);}finally{closeSync(oversizedFd);}
assert.throws(()=>readArtifact(oversizedPath,realpathSync(artifacts)),/size/,'Reject before allocating/reading oversized data');
assert.throws(()=>readArtifact(artifacts,realpathSync(artifacts)),/nonregular/);
assert.throws(()=>readArtifact(imagePath,realpathSync(resolve(artifacts,'avr'))),/Escaping/);
let linked=false;try{symlinkSync(imagePath,aliasPath);linked=true;}catch(error){if(!['EPERM','EACCES'].includes(error.code))throw error;}
if(linked){assert.throws(()=>readArtifact(aliasPath,realpathSync(artifacts)));unlinkSync(aliasPath);}
unlinkSync(emptyPath);unlinkSync(oversizedPath); // Only the exact fixtures exclusively created above.
const sha='a'.repeat(40),report={product:'PakarController',source_sha:sha,source_dirty:false,artifacts:observed};
const reportPath=resolve(artifacts,'firmware-report.json');writeFileSync(reportPath,JSON.stringify(report));
assert.equal(unchangedWrite(reportPath,JSON.stringify(report)),false);
assert.match(artifactMarkdown(report),/do not prove installation/);
assert.equal(verifiedArtifact(artifacts,sha).identity.sha256,observed[0].sha256);
const run={id:123,repository:{full_name:'atomicdeploy/PakarController'},head_repository:{full_name:'atomicdeploy/PakarController'},head_sha:sha,head_branch:'main',path:'.github/workflows/build.yml',event:'push',status:'completed',conclusion:'success'};
assert.equal(trustedBuild(run,{repository:'atomicdeploy/PakarController',expectedSha:sha,runId:'123'}),sha);
for(const change of [{conclusion:'failure'},{event:'pull_request'},{head_branch:'feature'},{path:'.github/workflows/untrusted.yml'},{head_repository:{full_name:'other/fork'}}])assert.throws(()=>trustedBuild({...run,...change},{repository:'atomicdeploy/PakarController',expectedSha:sha,runId:'123'}));
let mutations=0;await deployArtifact({directory:artifacts,expectedSha:sha,origin:'http://ledboard.local',runId:'123',run,updater:async options=>{mutations++;assert.equal(options.mode,'auto');assert.equal(options.run,true);return {simulated:true};}});assert.equal(mutations,1);
writeFileSync(imagePath,Buffer.alloc(96));await assert.rejects(deployArtifact({directory:artifacts,expectedSha:sha,runId:'123',run,updater:async()=>{mutations++;}}));assert.equal(mutations,1);
for(const entry of ['build.sh','pakar.sh','upgrade.sh']){
  const syntax=spawnSync('bash',['-n',entry],{encoding:'utf8',timeout:10000});
  if(!syntax.error){
    assert.equal(syntax.status,0,syntax.stderr);
    const result=spawnSync('bash',[resolve(entry),'--help'],{cwd:fixture,encoding:'utf8',timeout:15000});
    assert.equal(result.status,0,result.stderr);assert.match(result.stdout,/PakarController/);
  }
}
const workflow=read('.github/workflows/build.yml');assert.match(workflow,/os: \[windows-2025, ubuntu-24.04, macos-15\]/);
const quality=workflow.split('  quality:')[1].split('  tooling:')[0];
assert.match(quality,/setup-arduino-cli@[a-f0-9]{40}/);
assert.match(quality,/Get-Content tools\/firmware-dependencies\.json/);
assert.match(quality,/foreach \(\$taskName in @\('ArduinoJson','LibSSH-ESP32'\)\)/);
assert.match(quality,/Where-Object name -eq \$taskName/);
assert.match(quality,/\$taskLibrary\.Count -ne 1/);
assert.match(quality,/arduino-cli lib install \(\$taskName \+ '@' \+ \$taskLibrary\[0\]\.version\)/);
assert(quality.indexOf('arduino-cli lib install')<quality.indexOf('node tools/ci-checks.mjs quality'));
assert.doesNotMatch(quality,/core install/); // Header-only native fixtures need no board SDK.
for(const name of ['build','update-dependencies','upload'])for(const action of read(`.github/workflows/${name}.yml`).matchAll(/uses: ([^\s#]+)/g))assert.match(action[1],/@[a-f0-9]{40}$/);
assert.match(read('.github/workflows/upload.yml'),/PAKAR_ENABLE_DEVICE_DEPLOY == 'true'/);assert.match(read('.github/workflows/upload.yml'),/cancel-in-progress: false/);
assert.doesNotMatch(read('build.sh'),/arduino-cli compile|--clean/);assert.doesNotMatch(read('build.cmd'),/test-protocol/);
console.log('Script tooling: logger/quiet/JSON, argv/exit propagation, offline plans, numeric registry updates/no downgrade/concurrent edit, raw artifacts/hash/provenance and opt-in deployment guards passed. No SDK build/install/device contact.');
