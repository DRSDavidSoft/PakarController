// README uses the same bytes as native/embedded branding, never a second logo.
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import assert from 'node:assert/strict';
const master=new URL('../assets/branding/app-icon-1024.png',import.meta.url);
const target=new URL('../docs/assets/app-icon.png',import.meta.url);
const png=readFileSync(master);
assert.equal(png.subarray(0,8).toString('hex'),'89504e470d0a1a0a');
let existing;
try{existing=readFileSync(target);}catch(error){if(error.code!=='ENOENT')throw error;}
if(!existing?.equals(png)){
  assert(!process.argv.includes('--check'),'README artwork is stale');
  mkdirSync(new URL('../docs/assets/',import.meta.url),{recursive:true});
  writeFileSync(target,png);
}
console.log('README/native/Web canonical icon bytes match.');
