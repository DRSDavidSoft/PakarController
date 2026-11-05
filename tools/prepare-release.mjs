// Pealayer ci-cd.yml packaging, curated notes, manifest and checksum policy
// adapted for the ESP32-S3/AVR pair plus Windows and Android preview.
// Explicit legacy packaging remains available; unfinished ESP8266 is not a
// default release target and is never substituted for the S3 application.
import {readFileSync,writeFileSync,copyFileSync,mkdirSync,existsSync,readdirSync} from 'node:fs';
import {resolve,join} from 'node:path';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {manifest} from './create-remote-manifest.mjs';
import {validateAvrHex} from './firmware-report.mjs';
export const sha256=data=>createHash('sha256').update(data).digest('hex');
export function releaseIdentity(repository,tag,commit,version){
  if(!/^[\w.-]+\/[\w.-]+$/.test(repository))throw Error('Invalid repository');
  if(!/^v\d+\.\d+\.\d+(?:-[A-Za-z0-9.-]+)?$/.test(tag)||tag.split('-')[0]!==`v${version}`)throw Error('Release tag must match source version');
  if(!/^[0-9a-f]{40}$/.test(commit))throw Error('Exact source commit required');
  return {repository,tag,commit,version,draft:true,prerelease:true};
}
export function firmwareManifest(esp,avr,identity,mode){
  const base=`https://github.com/${identity.repository}/releases/download/${identity.tag}/`;
  const value=manifest(esp,avr,base,identity.version,mode,mode==='buffered'?'esp32s3':'esp8266');
  value.esp.url=new URL(`esp-${mode}.bin`,base).href;
  value.avr.url=new URL(`avr-${mode}.bin`,base).href;
  return value;
}
export function checksumList(files){
  return files.sort((a,b)=>a.name.localeCompare(b.name,'en')).map(({name,data})=>{
    if(!/^[A-Za-z0-9_.-]+$/.test(name))throw Error('Unsafe release asset name');
    return `${sha256(data)}  ${name}\n`;
  }).join('');
}
function prepare(){
  const [incoming,out,tag,commit,repository,run,legacy]=process.argv.slice(2);
  if(!run||!/^https:\/\/github.com\/[\w.-]+\/[\w.-]+\/actions\/runs\/\d+$/.test(run)||(legacy!==undefined&&legacy!=='--legacy')||process.argv.length>9)throw Error('Usage: prepare-release INCOMING OUTPUT TAG SHA REPO RUN_URL [--legacy]');
  const source=readFileSync('libraries/BoardCore/src/BuildInfo.h','utf8');
  const version=/kVersion\[\]\s*=\s*"(\d+\.\d+\.\d+)"/.exec(source)?.[1];
  const identity=releaseIdentity(repository,tag,commit,version);
  const output=resolve(out);
  if(existsSync(output)&&readdirSync(output).length)throw Error('Refuse to overwrite existing release assets');
  mkdirSync(output,{recursive:true});
  const input=resolve(incoming),notesPath=existsSync(`docs/releases/${tag}.md`)?`docs/releases/${tag}.md`:'docs/releases/next.md';
  const notes=readFileSync(notesPath,'utf8');
  if(!notes.trim())throw Error('Curated release notes required');
  const emit=(name,data)=>writeFileSync(join(output,name),data,{flag:'wx'});
  const zip=(name,files)=>{
    const stage=join(output,'.staging',name.replace(/\.zip$/,''));
    mkdirSync(stage,{recursive:true});
    for(const [name,data] of files)writeFileSync(join(stage,name),data,{flag:'wx'});
    const result=spawnSync('zip',['-q',join(output,name),...files.map(([name])=>name)],{cwd:stage,stdio:'inherit'});
    if(result.error)throw result.error;
    if(result.status!==0)throw Error('Package archive failed');
    const verify=spawnSync('zip',['-T',join(output,name)],{stdio:'inherit'});
    if(verify.error)throw verify.error;
    if(verify.status!==0)throw Error('Archive integrity test failed');
  };
  const docs=[['README.md',readFileSync('README.md')],['LICENSE',readFileSync('LICENSE')],['Pealayer-MIT.txt',readFileSync('docs/licenses/Pealayer-MIT.txt')],['RELEASE-NOTES.md',Buffer.from(notes)]];
  for(const mode of ['buffered',...(legacy?['polling','busy-wire']:[])]){
    const suffix=mode==='polling'?'':`-${mode}`;
    const platform=mode==='buffered'?'esp32s3':'esp8266';
    const esp=readFileSync(join(input,'firmware',`${platform}${suffix}`,`${platform}.ino.bin`));
    const avr=readFileSync(join(input,'firmware',`avr${suffix}`,'avr.ino.with_bootloader.bin'));
    const appHex=readFileSync(join(input,'firmware',`avr${suffix}`,'avr.ino.hex'));
    const fullHex=readFileSync(join(input,'firmware',`avr${suffix}`,'avr.ino.with_bootloader.hex'));
    validateAvrHex(appHex);validateAvrHex(fullHex,avr);
    const value=firmwareManifest(esp,avr,identity,mode);
    const json=Buffer.from(JSON.stringify(value,null,2)+'\n');
    emit(`esp-${mode}.bin`,esp);emit(`avr-${mode}.bin`,avr);emit(`manifest-${mode}.json`,json);
    emit(`avr-${mode}.hex`,appHex);emit(`avr-${mode}-with-bootloader.hex`,fullHex);
    zip(`PakarController-firmware-${mode}.zip`,[...docs,[`esp-${mode}.bin`,esp],[`avr-${mode}.bin`,avr],[`avr-${mode}.hex`,appHex],[`avr-${mode}-with-bootloader.hex`,fullHex],[`manifest-${mode}.json`,json]]);
  }
  const native=readFileSync(join(input,'windows','pakarcontroller.exe'));
  if(native.length<64||native.subarray(0,2).toString()!=='MZ')throw Error('Missing Windows PE executable');
  zip('PakarController-Windows.zip',[...docs,['pakarcontroller.exe',native]]);
  const apk=readFileSync(join(input,'android','app-release.apk'));
  if(apk.length<64||apk.subarray(0,4).toString('hex')!=='504b0304')throw Error('Missing Android preview APK');
  emit('PakarController-Android-preview.apk',apk);
  const assets=()=>readdirSync(output).filter(name=>!name.startsWith('.')).map(name=>({name,data:readFileSync(join(output,name))}));
  emit('release-manifest.json',JSON.stringify({product:'PakarController',...identity,run,physicalAcceptance:false,
    releaseTargets:['esp32s3','atmega328p'],legacyEsp8266:legacy?'explicit unfinished legacy artifacts included':'source/tooling preserved; resource-budget failures; not packaged',
    androidSigning:'preview',assets:assets().map(({name,data})=>({name,size:data.length,sha256:sha256(data)}))},null,2)+'\n');
  emit('RELEASE-NOTES.md',notes+`\n\nDraft prerelease; no automatic board flashing or physical acceptance.\n\n[Exact build](${run})\n\nVerify with \`sha256sum --check SHA256SUMS.txt\` and \`gh attestation verify ASSET --repo ${repository}\`. Firmware manifests pin both board images and the link mode; distribute the manifest hash through a separately trusted channel.\n`);
  emit('SHA256SUMS.txt',checksumList(assets()));
  console.log(JSON.stringify({tag,draft:true,prerelease:true,assets:assets().map(({name})=>name)},null,2));
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url))prepare();
