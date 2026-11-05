// One bundled artwork source for build/runtime native and pre-auth Web branding.
// Pealayer HeaderBar consumes resolved artwork; its WebPlatform binds favicon
// to that same resolved icon. Pakar embeds it for the standalone ESP surface.
import assert from 'node:assert/strict';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
await import('./generate-application-artwork.mjs');
await import('./generate-board-icon-asset.mjs');
const asset = (await readFile(new URL('../assets/branding/app-icon.png.base64', import.meta.url), 'utf8')).trim();
assert.match(asset, /^[A-Za-z0-9+/]+={0,2}$/);
const png = Buffer.from(asset, 'base64');
assert.equal(png.subarray(0,8).toString('hex'), '89504e470d0a1a0a');
assert.equal(png.readUInt32BE(16), 64);
assert.equal(png.readUInt32BE(20), 64);
const url = new URL('../web/index.html', import.meta.url);
let web = await readFile(url, 'utf8');
const original = web;
const data = `data:image/png;base64,${asset}`;
const blocks = {
  'FAVICON': `<link rel="icon" type="image/png" href="${data}"><link rel="apple-touch-icon" href="${data}">`,
  'HEADER': `<img class="app-logo" src="${data}" width="42" height="42" alt="PakarController application logo">`,
  'LOGIN': `<img class="app-logo" src="${data}" width="42" height="42" alt="PakarController application logo">`,
};
for (const [name, body] of Object.entries(blocks)) {
  const begin = `<!-- BEGIN SHARED BRAND ${name} -->`, end = `<!-- END SHARED BRAND ${name} -->`;
  assert.equal(web.split(begin).length, 2, `Exactly one ${name} begin marker`);
  assert.equal(web.split(end).length, 2, `Exactly one ${name} end marker`);
  const from = web.indexOf(begin)+begin.length, to = web.indexOf(end);
  assert(from <= to, 'Marker order (empty bootstrap block is valid)');
  web = web.slice(0,from)+body+web.slice(to);
}
if (web !== original) {
  assert(!process.argv.includes('--check'), 'Embedded branding is stale; run node tools/generate-branding.mjs');
  await writeFile(url, web);
  console.log('Updated shared Web branding only.');
} else console.log('Shared Web branding is current; no file writes.');
const applicationHeader = new URL('../firmware/esp/src/ApplicationArtwork.h', import.meta.url);
const artwork = `// Generated from the same cross-surface master; do not edit by hand.\n#pragma once\n#include <Arduino.h>\nstatic const char kApplicationIconPngBase64[] PROGMEM = "${asset}";\n`;
let prior;
try { prior = await readFile(applicationHeader, 'utf8'); } catch (error) { if (error.code !== 'ENOENT') throw error; }
if (prior !== artwork) {
  assert(!process.argv.includes('--check'), 'Firmware application artwork is stale; regenerate shared branding');
  await writeFile(applicationHeader, artwork);
}
// Flutter header uses the identical PNG master, not a substitute icon glyph.
const mobileArtwork = new URL('../mobile/lib/application_artwork.dart', import.meta.url);
const dart = '// Generated from assets/branding/app-icon.png.base64; same native/Web master.\nconst applicationArtworkBase64 = '+JSON.stringify(asset)+';\n';
let mobilePrior;
try { mobilePrior = await readFile(mobileArtwork, 'utf8'); } catch (error) { if (error.code !== 'ENOENT') throw error; }
if (mobilePrior !== dart) {
  assert(!process.argv.includes('--check'), 'Mobile application artwork is stale; regenerate shared branding');
  await writeFile(mobileArtwork, dart);
}
// Android's launcher is an OS resource, not the Flutter in-app header.
// Preserve the master bytes; Android scales/masks it through native drawables.
const launcher = new URL('../mobile/android/app/src/main/res/drawable-nodpi/pakar_application.png', import.meta.url);
const launcherPng = await readFile(new URL('../assets/branding/app-icon-1024.png', import.meta.url));
let launcherPrior;
try { launcherPrior=await readFile(launcher); } catch(error) { if(error.code!=='ENOENT')throw error; }
if(!launcherPrior?.equals(launcherPng)){
  assert(!process.argv.includes('--check'),'Android launcher master is stale; regenerate shared branding');
  await mkdir(new URL('.',launcher),{recursive:true});
  await writeFile(launcher,launcherPng);
}
