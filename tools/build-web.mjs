// A single application is packaged for hosted and onboard delivery. Both
// profiles retain all capabilities by default; size flags must be explicit.
import {readFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import {readWebSource,webHeader,webHeaderUrl,writeUnchanged} from './web-source.mjs';
import {portableGzip} from './portable-gzip.mjs';
import {generateWebCompressed} from './generate-web-compressed.mjs';
const root=new URL('../',import.meta.url),check=process.argv.includes('--check');
const profile=process.argv.find(v=>v.startsWith('--profile='))?.split('=')[1]||'full';
if(!['full','onboard'].includes(profile))throw Error('Use --profile=full or --profile=onboard');
for(const tool of ['generate-web-appearance.mjs','generate-interface-contract.mjs','generate-branding.mjs','generate-font-assets.mjs']){
 const result=spawnSync(process.execPath,[fileURLToPath(new URL(tool,import.meta.url)),...(check?['--check']:[])],{stdio:'inherit',windowsHide:true});
 if(result.error)throw result.error;if(result.status!==0)process.exit(result.status||1);
}
const html=await readWebSource();
await writeUnchanged(webHeaderUrl,webHeader(html),{check});
await generateWebCompressed({check});
const out=new URL('build/web/'+profile+'/',root);if(!check)await mkdir(out,{recursive:true});
const files=new Map([['index.html',Buffer.from(html)]]);
files.set('app-icon.png',await readFile(new URL('assets/branding/app-icon-256.png',root)));
if(!check)await mkdir(new URL('assets/fonts/',out),{recursive:true});
files.set('assets/fonts/Vazirmatn-Regular.ttf',await readFile(new URL('assets/fonts/Vazirmatn-Regular.ttf',root)));
const revision=createHash('sha256').update(html).digest('hex');
const manifest=JSON.parse(await readFile(new URL('web/manifest.webmanifest',root),'utf8'));
files.set('manifest.webmanifest',Buffer.from(JSON.stringify(manifest)));
files.set('sw.js',Buffer.from((await readFile(new URL('web/sw.js',root),'utf8')).replace('__WEB_ASSET_REVISION__',revision.slice(0,16))));
const receipt={application:'PakarController Web',source:'web/index.html',profile,disabled_features:[],revision,files:[]};
for(const[name,bytes]of files){const zipped=portableGzip(bytes);if(!check){await writeUnchanged(new URL(name,out),bytes);await writeUnchanged(new URL(name+'.gz',out),zipped);}receipt.files.push({name,bytes:bytes.length,gzip:zipped.length,sha256:createHash('sha256').update(bytes).digest('hex')});}
const worker=files.get('sw.js').toString(),manifestText=files.get('manifest.webmanifest').toString();
await writeUnchanged(new URL('firmware/esp/src/WebAppAssets.h',root),'// Generated from the same Web application manifest and worker.\n#pragma once\n#include <Arduino.h>\n#include "WebInstallArtwork.h"\nstatic const char kWebManifest[] PROGMEM = R"JSON('+manifestText+')JSON";\nstatic const char kWebWorker[] PROGMEM = R"JS('+worker+')JS";\n',{check});
// Strong validators identify the actual bytes of each wire representation.
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
const basic=(await readFile(new URL('firmware/esp/src/BasicUi.h',root),'utf8')).replaceAll('\r\n','\n').match(/R"BASIC\(([\s\S]*)\)BASIC"/)[1];
const uiBytes=(await readFile(webHeaderUrl,'utf8')).replaceAll('\r\n','\n').match(/R"HTML\(([\s\S]*)\)HTML"/)[1];
const tags={Ui:portableGzip(Buffer.from(uiBytes)),Font:portableGzip(files.get('assets/fonts/Vazirmatn-Regular.ttf')),License:(await readFile(new URL('desktop/licenses/Vazirmatn-OFL.txt',root),'utf8')).replaceAll('\r\n','\n'),Basic:basic,Manifest:files.get('manifest.webmanifest'),Worker:files.get('sw.js')};
let validators='// Generated from exact static wire bytes; no boot-time hashing.\n#pragma once\n';
for(const[name,bytes]of Object.entries(tags))validators+='static const char kStatic'+name+'Etag[] = "\\"'+digest(bytes)+'\\"";\n';
validators+='#if PAKAR_GAME_MODE == 2\n';
for(const[at,role]of ['keys','table'].entries())validators+=(at?'#else\n':'')+'static const char kStaticIconEtag[] = "\\"'+digest(await readFile(new URL('assets/branding/'+role+'-256.png',root)))+'\\"";\n';
await writeUnchanged(new URL('firmware/esp/src/StaticAssetValidators.h',root),validators+'#endif\n',{check});
if(!check)await writeUnchanged(new URL('build-manifest.json',out),JSON.stringify(receipt,null,2)+'\n');
console.log(JSON.stringify({output:'build/web/'+profile,...receipt}));
