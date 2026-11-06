// Share actual Pealayer Vazirmatn bytes, not a CDN or an OS-dependent font.
// Like generate-branding: deterministic output and no identical-file writes.
import assert from 'node:assert/strict';
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {portableGzip} from './portable-gzip.mjs';
import {decodeHtmlSourceText} from './html-source-text.mjs';
const root=new URL('../',import.meta.url);
const font=await readFile(new URL('assets/fonts/Vazirmatn-Regular.ttf',root));
assert.equal(font.readUInt32BE(0),0x00010000,'Real TrueType font');
const compressed=portableGzip(font);
const lines=[];
for(let i=0;i<compressed.length;i+=24)lines.push('  '+[...compressed.subarray(i,i+24)].join(',')+',');
const license=await readFile(new URL('desktop/licenses/Vazirmatn-OFL.txt',root),'utf8');
const header='// Generated from assets/fonts/Vazirmatn-Regular.ttf; OFL in desktop/licenses.\n#pragma once\n#include <Arduino.h>\nstatic const uint8_t kVazirmatnGzip[] PROGMEM = {\n'+lines.join('\n')+'\n};\nstatic const char kVazirmatnLicense[] PROGMEM = '+JSON.stringify(license.replaceAll('\r\n','\n'))+';\n';
async function sync(path,bytes){const url=new URL(path,root);let old;try{old=await readFile(url);}catch(e){if(e.code!=='ENOENT')throw e;}
if(old?.equals(Buffer.from(bytes)))return;
assert(!process.argv.includes('--check'),'Font assets stale: '+path);
await mkdir(new URL('.',url),{recursive:true});await writeFile(url,bytes);}
await sync('firmware/esp/src/FontAssets.h',header);
// Install/shortcut artwork uses the same board masters as native/mobile.
let installArtwork='#pragma once\n#include <Arduino.h>\n';
for(const [i,role] of ['keys','table'].entries()){
 const png=await readFile(new URL('assets/branding/'+role+'-256.png',root));
 installArtwork+=(i?'#else\n':'#if PAKAR_GAME_MODE == 2\n')+'static const uint8_t kWebAppPng[] PROGMEM={'+[...png].join(',')+'};\n';
}
await sync('firmware/esp/src/WebInstallArtwork.h',installArtwork+'#endif\n');
await sync('mobile/assets/fonts/Vazirmatn-Regular.ttf',font);
await sync('mobile/assets/fonts/Vazirmatn-OFL.txt',await readFile(new URL('desktop/licenses/Vazirmatn-OFL.txt',root)));
// Import the actual native translation table into the standalone Web surface.
// Only keys occurring in this application are retained: no media/timeline UI.
const native=await readFile(new URL('desktop/src/ui/i18n.rs',root),'utf8');
let web=await readFile(new URL('web/index.html',root),'utf8');
const studioCss=await readFile(new URL('assets/web/studio.css',root),'utf8')+'\n'+await readFile(new URL('assets/web/application-utilities.css',root),'utf8');
const componentJs=await readFile(new URL('web/modules/components.js',root),'utf8');
const layoutJs=await readFile(new URL('web/modules/workspace.js',root),'utf8');
const tableLayoutJs=await readFile(new URL('web/modules/table-layout.js',root),'utf8');
const studioJs=await readFile(new URL('assets/web/workspace-studio.js',root),'utf8');
const commandJs=await readFile(new URL('assets/web/command-transport.js',root),'utf8');
const featuresJs=await readFile(new URL('assets/web/correction-preview.js',root),'utf8')+'\n'+await readFile(new URL('assets/web/pixel-groups.js',root),'utf8')+'\n'+await readFile(new URL('assets/web/user-files.js',root),'utf8')+'\n'+await readFile(new URL('assets/web/runtime-monitor.js',root),'utf8')+'\n'+await readFile(new URL('assets/web/feature-workspaces.js',root),'utf8');
const utilitiesJs=await readFile(new URL('assets/web/application-utilities.js',root),'utf8');
web=web.replace(/\/\* BEGIN STUDIO CSS \*\/[\s\S]*?\/\* END STUDIO CSS \*\//,'');
web=web.replace('</style>','/* BEGIN STUDIO CSS */\n'+studioCss+'\n/* END STUDIO CSS */</style>');
web=web.replace(/\/\/ BEGIN STUDIO MODULE[\s\S]*?\/\/ END STUDIO MODULE\r?\n/,'');
web=web.replace('<script>','<script>\n// BEGIN STUDIO MODULE\n'+commandJs+'\n'+componentJs+'\n'+layoutJs+'\n'+tableLayoutJs+'\n'+featuresJs+'\n'+utilitiesJs+'\n'+studioJs+'\n// END STUDIO MODULE');
const consoleModule=await readFile(new URL('assets/web/browser-console.js',root),'utf8');
const toastModule=await readFile(new URL('assets/web/toasts.js',root),'utf8');
const toastStyles=await readFile(new URL('assets/web/toasts.css',root),'utf8');
const retryModule=await readFile(new URL('assets/web/realtime-retry.js',root),'utf8');
// Mode captions arrive through capabilities, not as Web template literals.
const deviceContext=await readFile(new URL('libraries/BoardCore/src/DeviceContext.h',root),'utf8');
assert(!retryModule.includes('</script>')&&!retryModule.includes(')HTML"'),'Realtime retry cannot break the container');
assert(!toastModule.includes('</script>')&&!toastModule.includes(')HTML"'),'Toasts cannot break the container');
assert(!toastStyles.includes('</style>'),'Toast CSS cannot break the container');
assert(!consoleModule.includes('</script>')&&!consoleModule.includes(')HTML"'),'Browser diagnostics cannot break the container');
// Analyze the canonical module on the FIRST run too, never its previous embed.
// Otherwise adding a translated literal takes two generator passes to stabilize.
const withoutGenerated=decodeHtmlSourceText(web.replace(/\/\/ BEGIN SHARED LOCALIZATION[\s\S]*?\/\/ END SHARED LOCALIZATION/,'')
  .replace(/\/\/ BEGIN BROWSER DIAGNOSTICS[\s\S]*?\/\/ END BROWSER DIAGNOSTICS/,'')
  .replace(/\/\/ BEGIN RAYANLAMP TOASTS[\s\S]*?\/\/ END RAYANLAMP TOASTS/,'')
  .replace(/\/\/ BEGIN RAYANLAMP REALTIME RETRY[\s\S]*?\/\/ END RAYANLAMP REALTIME RETRY/,'')+'\n'+consoleModule+'\n'+toastModule+'\n'+retryModule+'\n'+deviceContext);
let js=await readFile(new URL('assets/web/localization.js',root),'utf8');
const dictionary={};
const controllerCopy=JSON.parse(await readFile(new URL('assets/web/controller-copy.json',root),'utf8'));
for(const match of native.matchAll(/((?:"[^"\n]+"\s*\|\s*)*"[^"\n]+")\s*=>\s*(?:\{\s*)?"([^"\n]+)"/g)){
  for(const key of match[1].matchAll(/"([^"\n]+)"/g))if(withoutGenerated.includes(key[1])||['Language','Direction','System language','Automatic','Left to right','Right to left','English','Persian'].includes(key[1]))dictionary[key[1]]=match[2];
}
for(const[key,value]of Object.entries(controllerCopy)){
  assert.equal(typeof value,'string');
  assert(withoutGenerated.includes(key),'Unused controller translation: '+key);
  dictionary[key]=value;
}
const dictionaryBegin='// BEGIN SHARED PERSIAN DICTIONARY',dictionaryEnd='// END SHARED PERSIAN DICTIONARY';
js=js.slice(0,js.indexOf(dictionaryBegin)+dictionaryBegin.length)+'\nconst PAKAR_FA='+JSON.stringify(dictionary)+';\n'+js.slice(js.indexOf(dictionaryEnd));
await sync('assets/web/localization.js',js);
const begin='// BEGIN SHARED LOCALIZATION',end='// END SHARED LOCALIZATION';
assert.equal(web.split(begin).length,2);assert.equal(web.split(end).length,2);
web=web.replace(/\/\/ BEGIN SHARED LOCALIZATION[\s\S]*?\/\/ END SHARED LOCALIZATION\r?\n/,'');
web=web.replace('<script>','<script>\n'+begin+'\n'+js+'\n'+end);
// Canonical source adaptation, shared by tests and embedded Web UI.
const rayanBegin='// BEGIN RAYANLAMP ADAPTATIONS',rayanEnd='// END RAYANLAMP ADAPTATIONS';
web=web.replace(/\/\/ BEGIN RAYANLAMP ADAPTATIONS[\s\S]*?\/\/ END RAYANLAMP ADAPTATIONS\r?\n/,'');
web=web.replace('<script>','<script>\n'+rayanBegin+'\n'+await readFile(new URL('assets/web/rayanlamp.js',root),'utf8')+'\n'+rayanEnd);
const consoleBegin='// BEGIN BROWSER DIAGNOSTICS',consoleEnd='// END BROWSER DIAGNOSTICS';
web=web.replace(/\/\/ BEGIN BROWSER DIAGNOSTICS[\s\S]*?\/\/ END BROWSER DIAGNOSTICS\r?\n/,'');
web=web.replace('<script>','<script>\n'+consoleBegin+'\n'+consoleModule+'\n'+consoleEnd);
web=web.replace(/\/\/ BEGIN RAYANLAMP TOASTS[\s\S]*?\/\/ END RAYANLAMP TOASTS\r?\n/,'');
web=web.replace('<script>','<script>\n// BEGIN RAYANLAMP TOASTS\n'+toastModule+'\n// END RAYANLAMP TOASTS');
web=web.replace(/\/\/ BEGIN RAYANLAMP REALTIME RETRY[\s\S]*?\/\/ END RAYANLAMP REALTIME RETRY\r?\n/,'');
web=web.replace('<script>','<script>\n// BEGIN RAYANLAMP REALTIME RETRY\n'+retryModule+'\n// END RAYANLAMP REALTIME RETRY');
web=web.replace(/\/\* BEGIN RAYANLAMP TOAST CSS \*\/[\s\S]*?\/\* END RAYANLAMP TOAST CSS \*\//,'');
web=web.replace('</style>','/* BEGIN RAYANLAMP TOAST CSS */\n'+toastStyles+'\n/* END RAYANLAMP TOAST CSS */</style>');
await sync('web/index.html',web);
console.log('Shared Vazirmatn assets verified:',font.length,'bytes; gzip',compressed.length);
