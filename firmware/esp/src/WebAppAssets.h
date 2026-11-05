// Generated from the same Web application manifest and worker.
#pragma once
#include <Arduino.h>
#include "WebInstallArtwork.h"
static const char kWebManifest[] PROGMEM = R"JSON({"id":"/","name":"PakarController","short_name":"PakarController","description":"PakarController control workspace","start_url":"/","scope":"/","display":"standalone","background_color":"#11161e","theme_color":"#11161e","icons":[{"src":"/app-icon.png","sizes":"256x256","type":"image/png","purpose":"any"}]})JSON";
static const char kWebWorker[] PROGMEM = R"JS(// One versioned app shell. Never cache API, authentication, firmware or commands.
// No skipWaiting/clients.claim: installing an update must not interrupt a draft.
const shell='pakar-web-shell-bc05318e412594e7';
const assets=['/','/app-icon.png','/assets/fonts/Vazirmatn-Regular.ttf'];
self.addEventListener('install',event=>event.waitUntil(caches.open(shell).then(cache=>cache.addAll(assets))));
self.addEventListener('activate',event=>event.waitUntil(caches.keys().then(keys=>Promise.all(keys.filter(key=>key.startsWith('pakar-web-shell-')&&key!==shell).map(key=>caches.delete(key))))));
self.addEventListener('fetch',event=>{
 const url=new URL(event.request.url);
 if(event.request.method!=='GET'||url.origin!==self.location.origin||url.search||!assets.includes(url.pathname))return;
 event.respondWith(fetch(event.request).catch(()=>caches.match(event.request).then(response=>response||Response.error())));
});
)JS";
