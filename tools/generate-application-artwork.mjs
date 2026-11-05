// Editable vector owner; external Sharp is needed only when deliberately rendering.
// Check mode validates source/output hashes without installing any image tool.
import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {createRequire} from 'node:module';
import assert from 'node:assert/strict';
const root=new URL('../',import.meta.url),render=process.argv.includes('--render');
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
const source='assets/branding/app-icon.svg',manifestPath='assets/branding/application-artwork.json';
const svg=await readFile(new URL(source,root));
async function put(path,bytes){bytes=Buffer.from(bytes);let old;try{old=await readFile(new URL(path,root));}catch(e){if(e.code!=='ENOENT')throw e;}if(!old?.equals(bytes))await writeFile(new URL(path,root),bytes);}
if(!render){
 const manifest=JSON.parse(await readFile(new URL(manifestPath,root),'utf8'));
 assert.equal(manifest.source,source);assert.equal(manifest.source_sha256,hash(svg),'Application SVG changed: render its derivatives');
 for(const [path,expected] of Object.entries(manifest.outputs))assert.equal(hash(await readFile(new URL(path,root))),expected,'Stale application artwork: '+path);
 assert.equal(Object.keys(manifest.outputs).length,8);
}else{
 const require=createRequire(import.meta.url),sharp=require(process.env.PAKAR_SHARP_MODULE||'sharp');
 const outputs={};
 for(const size of [24,32,64,128,256,512,1024]){
  const png=await sharp(svg).resize(size,size).png(size===64?{palette:true,colours:48,dither:0}:{compressionLevel:9}).toBuffer();
  const path='assets/branding/app-icon-'+size+'.png';await put(path,png);outputs[path]=hash(png);
  if(size===64){const encoded=png.toString('base64')+'\n';assert(encoded.trim().length<=4096,'Compact artwork exceeds peer identity budget');await put('assets/branding/app-icon.png.base64',encoded);outputs['assets/branding/app-icon.png.base64']=hash(encoded);}
 }
 await put(manifestPath,JSON.stringify({source,source_sha256:hash(svg),outputs},null,2)+'\n');
}
console.log('Scalable application artwork and bounded peer derivative agree; '+(!render?'no image tool or writes.':'Sharp rasterization complete.'));
