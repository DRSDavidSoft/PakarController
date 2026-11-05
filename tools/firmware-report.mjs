// PU build.sh binary-size/MD5/compression and build.yml download summaries,
// adapted without renaming gzip to .bin, sourcing shell metadata or rewriting
// timestamps. S3 application bytes always stay raw; compression is explicit.
import {readFileSync,writeFileSync,realpathSync,lstatSync,openSync,fstatSync,readSync,closeSync,constants} from 'node:fs';
import {resolve,relative,sep} from 'node:path';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {portableGzip} from './portable-gzip.mjs';
import {createFirmwareManifest} from './firmware-manifest.mjs';
import {parseHex} from './prepare-avr-image.mjs';
export const expectedArtifacts=[
  ...['','-busy-wire','-buffered'].flatMap(mode=>['avr.ino.with_bootloader.bin','avr.ino.hex','avr.ino.with_bootloader.hex'].map(file=>`avr${mode}/${file}`)),
  'esp32s3-buffered/esp32s3.ino.bin',
];
// Preserved for explicit legacy builds; these are not promised release outputs.
export const legacyArtifacts=['','-busy-wire'].map(mode=>`esp8266${mode}/esp8266.ino.bin`);
const digest=(algorithm,bytes)=>createHash(algorithm).update(bytes).digest('hex');
export function validateAvrHex(bytes,binary=null){
  if(bytes.some(byte=>byte>127))throw Error('Non-ASCII Intel HEX');
  const memory=parseHex(bytes.toString('ascii'));
  if(!memory.has(0)||!memory.has(1))throw Error('AVR HEX lacks reset vector');
  if(binary){
    if(binary.length!==32768||memory.size!==32768||
       [...memory].some(([at,value])=>binary[at]!==value))throw Error('Merged AVR HEX differs from verified full-flash binary');
  }
  return memory.size;
}
export function unchangedWrite(path,bytes) {
  // A missing file is an open result, not a pathname check followed by reopen.
  try{if(readFileSync(path).equals(Buffer.from(bytes)))return false;}
  catch(error){if(error.code!=='ENOENT')throw error;}
  // Reports are derived metadata, not original firmware/settings/recovery data.
  writeFileSync(path,bytes);return true;
}
// Adapt the existing running-image verifier's descriptor ownership: validate
// and read the SAME opened file. Never reopen a pathname checked by stat.
export function readArtifact(path,root) {
  const fd=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0));
  try{
    const file=fstatSync(fd),entry=lstatSync(path);
    if(!file.isFile()||!entry.isFile()||entry.isSymbolicLink()||file.dev!==entry.dev||file.ino!==entry.ino||!realpathSync(path).startsWith(root+sep))throw Error('Escaping or nonregular firmware artifact');
    if(file.size<1||file.size>4*1024*1024)throw Error('Invalid firmware artifact size');
    const bytes=Buffer.alloc(file.size);let offset=0;
    while(offset<bytes.length){const count=readSync(fd,bytes,offset,bytes.length-offset,offset);if(!count)throw Error('Firmware artifact changed while reading');offset+=count;}
    const after=fstatSync(fd);
    if(after.size!==file.size||after.mtimeMs!==file.mtimeMs||readSync(fd,Buffer.alloc(1),0,1,file.size)!==0)throw Error('Firmware artifact changed while reading');
    return bytes;
  }finally{closeSync(fd);}
}
export function collectArtifacts(directory,{required=false,compress=false,includeLegacy=false}={}) {
  const root=realpathSync(directory),artifacts=[];
  for(const name of [...expectedArtifacts,...(includeLegacy?legacyArtifacts:[])]){
    const path=resolve(root,name);
    let bytes;
    try{bytes=readArtifact(path,root);}
    catch(error){if(error.code!=='ENOENT')throw error;if(required)throw Error('Missing expected firmware artifact: '+name);continue;}
    const hex=name.endsWith('.hex');
    if(hex)validateAvrHex(bytes,name.endsWith('.with_bootloader.hex')?readArtifact(path.replace(/\.hex$/,'.bin'),root):null);
    else if(name.startsWith('avr')&&bytes.length!==32768)throw Error('AVR image must include full flash/Urboot');
    if(name.startsWith('esp')&&bytes[0]!==0xe9)throw Error('ESP application must remain raw; do not rename gzip to .bin');
    artifacts.push({path:name,size:bytes.length,sha256:digest('sha256',bytes),md5:digest('md5',bytes),format:hex?'intel-hex':'raw'});
    if(compress&&name.startsWith('esp8266')){
      const gz=portableGzip(bytes),manifest=createFirmwareManifest(bytes,gz,name.split('/').at(-1)+'.gz');
      unchangedWrite(path+'.gz',gz);unchangedWrite(path+'.gz.md5',manifest.content);
      artifacts.push({path:name+'.gz',size:gz.length,sha256:digest('sha256',gz),md5:manifest.identity.compressed_md5,format:'gzip',raw_sha256:digest('sha256',bytes),raw_md5:manifest.identity.raw_md5,raw_size:manifest.identity.raw_size,manifest:name+'.gz.md5'});
    }
  }return artifacts;
}
export function artifactMarkdown(report,url='') {
  const link=/^https:\/\/github\.com\/[\w.-]+\/[\w.-]+\/actions\/runs\/\d+(?:\/artifacts\/\d+)?$/.test(url)?`\n[Download build artifacts](${url})\n`:'';
  return '# PakarController firmware artifacts\n\n'+`Source: \`${report.source_sha}\` · source tree ${report.source_dirty?'dirty / not a release':'clean'}\n`+link+
    '\n| Artifact | Bytes | SHA-256 |\n|---|---:|---|\n'+report.artifacts.map(row=>`| ${row.path} | ${row.size.toLocaleString('en-US')} | \`${row.sha256}\` |`).join('\n')+
    '\n\nMD5 and format/size metadata are in firmware-report.json. Build artifacts do not prove installation, electrical/optical operation or physical acceptance.\n';
}
export function main(args=process.argv.slice(2)) {
  const options=new Set(args);for(const arg of options)if(!['--write','--require','--compress','--legacy','--help'].includes(arg))throw Error('Unknown report option');
  if(options.has('--help')){console.log('PakarController artifacts: read-only by default. --write emits stable metadata; --require needs three AVR profiles and ESP32-S3. --legacy additionally collects unfinished ESP8266 artifacts; --compress adds separate legacy .gz only.');return;}
  const root=fileURLToPath(new URL('../',import.meta.url)),directory=resolve(root,'build');
  const source_sha=execFileSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).trim();
  const source_dirty=!!execFileSync('git',['status','--porcelain'],{cwd:root,encoding:'utf8'}).trim();
  const report={product:'PakarController',source_sha,source_dirty,release_targets:['esp32s3','atmega328p'],legacy_esp8266:'unfinished; resource-budget failures; explicit opt-in artifacts only',scope:'existing artifact bytes, not embedded-source or hardware proof',artifacts:collectArtifacts(directory,{required:options.has('--require'),compress:options.has('--compress'),includeLegacy:options.has('--legacy')})};
  if(options.has('--write')){
    unchangedWrite(resolve(directory,'firmware-report.json'),JSON.stringify(report,null,2)+'\n');
    unchangedWrite(resolve(directory,'SHA256SUMS'),report.artifacts.map(row=>`${row.sha256}  ${row.path}`).join('\n')+'\n');
  }
  if(process.env.GITHUB_STEP_SUMMARY)writeFileSync(process.env.GITHUB_STEP_SUMMARY,artifactMarkdown(report,process.env.PAKAR_ARTIFACT_URL??''),{flag:'a'});
  console.log(JSON.stringify(report,null,2));return report;
}
if(process.argv[1]&&import.meta.url===pathToFileURL(resolve(process.argv[1])).href){try{main();}catch(error){console.error(error.message);process.exitCode=1;}}
