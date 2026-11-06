// One Web application: these workspaces use the existing peer owner, API client,
// command transport and render lifecycle. No connection, polling or front-panel
// implementation is created here. Correction semantics are adapted from the
// actual Flutter PixelCorrectionsPage and PixelCorrectionStore firmware contract.
(function(root){
  'use strict';
  const correctionPath='/api/pixels/corrections';
  let installed=null;
  function parseCorrections(value){
    if(!value||!Number.isSafeInteger(value.revision)||value.revision<0||value.revision>0xffffffff||
       !Number.isInteger(value.pixels)||value.pixels<1||value.pixels>480||typeof value.enabled!=='boolean'||
       !Array.isArray(value.rules)||value.rules.length>32||(value.index_base!==undefined&&value.index_base!==0))throw Error('Invalid pixel correction response');
    const rules=value.rules.map(rule=>{
      if(!rule||!['black','remove','shift'].includes(rule.kind)||!Number.isInteger(rule.pixel)||rule.pixel<0||rule.pixel>=value.pixels||
         !Number.isInteger(rule.offset??0)||Math.abs(rule.offset??0)>value.pixels||(rule.kind!=='shift'&&(rule.offset??0)!==0)||
         (rule.enabled!==undefined&&typeof rule.enabled!=='boolean'))throw Error('Invalid pixel correction rule');
      return {kind:rule.kind,pixel:rule.pixel,offset:rule.offset??0,enabled:rule.enabled??true};
    });
    if(value.sources!==undefined&&(!Array.isArray(value.sources)||value.sources.length!==value.pixels||value.sources.some(n=>!Number.isInteger(n)||n< -1||n>=value.pixels)))throw Error('Invalid correction wire map');
    return {...value,rules};
  }
  function correctionsMatch(actual,wanted){return actual.enabled===wanted.enabled&&actual.rules.length===wanted.rules.length&&actual.rules.every((r,i)=>['kind','pixel','offset','enabled'].every(k=>r[k]===wanted.rules[i][k]));}
  // Port of Flutter pixelCorrectionSources / pixelCorrectionAddress: rules own
  // output slots, not raw pixels. This local projection never actuates a board.
  function correctionSources(pixels,enabled,rules){
    const checked=parseCorrections({revision:0,pixels,enabled,rules}),sources=Array.from({length:pixels},(_,i)=>i);
    if(!enabled)return sources;
    for(const rule of checked.rules){if(!rule.enabled)continue;const p=rule.pixel,o=rule.offset;
      if(rule.kind==='black')sources[p]=-1;
      else if(rule.kind==='remove'){for(let i=p;i+1<pixels;i++)sources[i]=sources[i+1];sources[pixels-1]=-1;}
      else if(o>0){for(let i=pixels-1;i>=p;i--)sources[i]=i-o>=p?sources[i-o]:-1;}
      else if(o<0){for(let i=Math.max(0,p+o);i<pixels+o;i++)sources[i]=sources[i-o];for(let i=Math.max(p,pixels+o);i<pixels;i++)sources[i]=-1;}
    }return sources;
  }
  // Flutter operationOption's six-slot window includes preceding rules and the
  // current rule, but not later edits. Enabled flags remain actual user choices.
  function correctionWindow(pixels,enabled,rules,index){
    if(!Array.isArray(rules)||!Number.isInteger(index)||index<0||index>=rules.length)throw Error('Invalid correction rule');
    const prefix=rules.slice(0,index+1),sources=correctionSources(pixels,enabled,prefix),focus=prefix[index].pixel;
    const start=Math.min(Math.max(0,focus-2),Math.max(0,pixels-6));return {start,focus,sources:sources.slice(start,Math.min(pixels,start+6))};
  }
  function ruleDisclosureKey(rule,index,rules){const signature=r=>[r.kind,r.pixel,r.offset??0,r.enabled===false?0:1].join(':'),base=signature(rule);return base+':'+rules.slice(0,index).filter(r=>signature(r)===base).length;}
  // Presentation only, scoped by the existing immutable board/application key.
  // A rule fingerprint is matched against freshly read configuration; it is
  // never decoded into rules, saved values, capabilities or an operation.
  function pixelToolsWorkspace(value){
    if(!value||typeof value!=='object'||Array.isArray(value))return null;
    const expandedRules=[];for(const key of Array.isArray(value.expandedRules)?value.expandedRules.slice(0,32):[]){
      if(typeof key!=='string'||key.length>48)continue;const [kind,pixel,offset,flag,ordinal,...extra]=key.split(':');
      if(extra.length||!['black','remove','shift'].includes(kind)||!/^\d{1,3}$/.test(pixel)||Number(pixel)>479||! /^-?\d{1,3}$/.test(offset)||Math.abs(Number(offset))>480||!['0','1'].includes(flag)||!/^\d{1,2}$/.test(ordinal)||Number(ordinal)>31)continue;
      if(kind!=='shift'&&Number(offset)!==0)continue;if(!expandedRules.includes(key))expandedRules.push(key);
    }
    return {mapView:value.mapView==='draft'?'draft':'applied',mapOpen:value.mapOpen===true,mapFilter:typeof value.mapFilter==='string'&&/^\d{0,3}$/.test(value.mapFilter)?value.mapFilter:'',advancedOpen:value.advancedOpen===true,expandedRules};
  }
  const correctionDescriptions={black:'Send black to this address. Later pixels keep their positions.',remove:'Skip this value. Following values move one address earlier; the last address is black.',shift:'Move this pixel and all following values. Positive moves later; negative moves earlier. Empty addresses are black.'};
  // Actual Flutter PixelCorrectionDiagram/observedRgb: published_ is the raw
  // scene before corrected(published_) in LedGrid. Never color an output slot
  // by its own index after a shift, or invent RGB when publication is pending.
  function correctionRgb(state,pixels,role,current=true){
    const frame=state?.pixel_frame;
    if(!current||state?.pixels!==pixels||!Number.isInteger(pixels)||pixels<1||pixels>480||!frame||
      !Number.isInteger(frame.revision)||frame.revision<1||frame.revision>0xffffffff||frame.pending!==false||
      !Number.isInteger(frame.brightness)||frame.brightness<0||frame.brightness>255||
      typeof frame.rgb!=='string'||frame.rgb.length!==pixels*6||!/^[a-f0-9]+$/i.test(frame.rgb))return null;
    const features=Array.isArray(state.capabilities)?state.capabilities:state.capabilities?.acknowledged_features;
    if(features?.includes('pixel-group-layout')){
      try{const observed=root.PixelGroups.frameLayout(frame,role),live=root.PixelGroups.fromState(state,role);
        if(!live||observed.revision!==live.revision||observed.counts.some((n,i)=>n!==live.counts[i]))return null;
      }catch{return null;}
    }
    return Array.from({length:pixels},(_,i)=>parseInt(frame.rgb.slice(i*6,i*6+6),16));
  }
  function correctionInk(rgb){const linear=v=>{v/=255;return v<=.04045?v/12.92:((v+.055)/1.055)**2.4;};return .2126*linear(rgb>>16)+.7152*linear((rgb>>8)&255)+.0722*linear(rgb&255)>.35?'#000000':'#ffffff';}
  function correctionAddress({wire,pixels,enabled,rules,state,role,mapping,current=true}){
    const result={wire,reason:'unavailable'};
    if(!current)return {...result,reason:'not-current'};
    const layout=root.PixelGroups?.fromState(state,role);
    if(!Number.isInteger(wire)||wire<0||wire>=pixels||!layout||layout.capacity!==pixels)return result;
    try{result.source=correctionSources(pixels,enabled,rules)[wire];}catch{return {...result,reason:'invalid-rules'};}
    if(result.source<0)return {...result,reason:'black'};
    const g=root.PixelGroups.groupAt(layout,result.source);if(g<0)return {...result,reason:'outside-groups'};
    Object.assign(result,{group:g+1,pixel:result.source-layout.starts[g]+1,count:layout.counts[g]});
    if(role==='first-wins')return {...result,box:Math.floor(g/2)+1,part:g%2?'indicator':'button',reason:null};
    const physical=mapping?.physical;
    if(role!=='table-selection'||!Array.isArray(physical)||physical.length!==80||new Set(physical).size!==80||physical.some(n=>!Number.isInteger(n)||n<1||n>80))return {...result,reason:'no-logical-map'};
    const features=Array.isArray(state?.capabilities)?state.capabilities:state?.capabilities?.acknowledged_features;
    if(features?.includes('pixel-group-layout')){try{const captured=root.PixelGroups.parse(mapping.pixel_groups,role);if(captured.revision!==layout.revision||captured.counts.some((n,i)=>n!==layout.counts[i]))return {...result,reason:'no-logical-map'};}catch{return {...result,reason:'no-logical-map'};}}
    result.logical=physical.indexOf(g+1)+1;const name=mapping.names?.[result.logical-1];
    return {...result,name:typeof name==='string'?name.slice(0,100):'',reason:null};
  }
  function diagnosticShape(state,role){
    const group=role==='first-wins'?12:6,pixels=role==='first-wins'?216:480;
    const layout=root.PixelGroups?.fromState(state,role);
    if(root.PixelGroups&&!layout)return null;
    return state?.pixels===pixels?{group,pixels,layout}:null;
  }
  const diagnosticDefaults=Object.freeze({effect:'solid',composite:'clear',period_ms:1000,duty_percent:50,min_brightness:0,max_brightness:255,enabled:true});
  const diagnosticLimits={period_ms:[100,10000],duty_percent:[1,99],min_brightness:[0,255],max_brightness:[0,255]};
  function diagnosticOptions(value={}){
    const options={...diagnosticDefaults};for(const key of Object.keys(options))if(Object.hasOwn(value,key))options[key]=value[key];
    if(!['solid','blink','breathe','rainbow'].includes(options.effect)||!['clear','overlay'].includes(options.composite)||typeof options.enabled!=='boolean')throw Error('Invalid diagnostic options');
    for(const[key,[min,max]]of Object.entries(diagnosticLimits))if(!Number.isInteger(options[key])||options[key]<min||options[key]>max)throw Error('Invalid diagnostic timing or brightness');
    if(options.min_brightness>options.max_brightness)throw Error('Minimum brightness exceeds maximum');return options;
  }
  // Adapted from Flutter PixelDiagnosticWorkspace.snapshot/restore and the
  // reviewed Pealayer stable-ID workspace ownership. User drafts only: never
  // persist an ACK, observation, revision, pending operation or command.
  function diagnosticWorkspace(value){
    if(!value||typeof value!=='object'||Array.isArray(value))return null;
    const bounded=(key,min,max,fallback)=>Number.isInteger(value[key])&&value[key]>=min&&value[key]<=max?value[key]:fallback;
    const draft={index:bounded('index',0,479,0),rgb:bounded('rgb',0,0xffffff,0xffffff),wholeGroup:value.wholeGroup===true,
      effect:['solid','blink','breathe','rainbow'].includes(value.effect)?value.effect:'solid',composite:value.composite==='overlay'?'overlay':'clear',enabled:value.enabled!==false};
    for(const[key,[min,max]]of Object.entries(diagnosticLimits))draft[key]=bounded(key,min,max,diagnosticDefaults[key]);
    if(draft.min_brightness>draft.max_brightness){draft.min_brightness=0;draft.max_brightness=255;}
    draft.position=typeof value.position==='string'&&value.position.length<=8?value.position:String(draft.index+1);
    draft.timingDrafts={};for(const key of Object.keys(diagnosticLimits)){const text=value.timingDrafts?.[key];if(typeof text==='string'&&text.length<=12)draft.timingDrafts[key]=text;}
    return draft;
  }
  function parseDiagnostic(state,role){
    const shape=diagnosticShape(state,role),v=state?.led_output?.diagnostic;
    if(!shape||!v||!Number.isInteger(v.revision)||v.revision<0||v.revision>0xffffffff||
       !Number.isInteger(v.index)||v.index<0||v.index>=shape.pixels||!Number.isInteger(v.count)||v.count<1||
       v.index+v.count>shape.pixels||
       !Number.isInteger(v.rgb)||v.rgb<0||v.rgb>0xffffff||typeof v.active!=='boolean'||typeof v.off!=='boolean'||
       (v.active&&v.revision===0))return null;
    const span=shape.layout?root.PixelGroups.probe(shape.layout,v.index,true):{index:Math.floor(v.index/shape.group)*shape.group,count:shape.group};
    if(shape.layout&&v.index+v.count>shape.layout.used_pixels)return null;
    if(v.count!==1&&(!span||v.index!==span.index||v.count!==span.count))return null;
    if(v.group_layout_revision!==undefined&&v.group_layout_revision!==shape.layout?.revision)return null;
    const extended=Object.keys(diagnosticDefaults).some(key=>Object.hasOwn(v,key));
    if(extended&&!Object.keys(diagnosticDefaults).every(key=>Object.hasOwn(v,key)))return null;
    try{return {...v,...shape,...diagnosticOptions(v),extended};}catch{return null;}
  }
  // Adapted from the reviewed Pealayer MelodySelect / former workspace: the
  // connected board's advertised catalogue is authoritative, not a baked list.
  function audioCues(state,query=''){
    if(state?.avr_online!==true||!Array.isArray(state.audio_cues))return [];
    const needle=String(query).trim().toLowerCase().slice(0,80);
    return [...new Set(state.audio_cues.filter(cue=>typeof cue==='string'&&/^[a-z0-9][a-z0-9-]{0,39}$/.test(cue)))].slice(0,64)
      .filter(cue=>!needle||cue.includes(needle)||cue.replaceAll('-',' ').includes(needle));
  }
  function redact(value,depth=0){
    if(depth>12)return '[depth limited]';
    if(Array.isArray(value))return value.slice(0,512).map(v=>redact(v,depth+1));
    if(value&&typeof value==='object')return Object.fromEntries(Object.entries(value).map(([key,v])=>[key,/password|secret|token|credential|private_key/i.test(key)&&!/_set$|_enabled$/.test(key)?'[redacted]':redact(v,depth+1)]));
    return value;
  }
  function create(options){
    if(installed)return installed;
    const {snapshot,peer,api,command,pin,report}=options,translate=options.translate||((value)=>value);
    const layout=options.layout||document.querySelector('#workspace .layout');
    if(!layout)throw Error('Web workspace is not initialized');
    const models=new Map();let owner='',model=null,disposed=false,inspectorPayload=null,inspectorBusy=false;
    const ui=()=>root.WebUI;
    const node=(tag,cls,text)=>{const n=document.createElement(tag);if(cls)n.className=cls;if(text!==undefined)n.textContent=translate(text);return n;};
    const input=(type,value='')=>{const n=node('input');n.type=type;n.value=value;return n;};
    const numeric=(value,min,max)=>{const n=input('number',value);n.min=min;n.max=max;n.step=1;return n;};
    const field=(label,control)=>{const n=node('label','field settings-field');n.append(node('span','',label),control);return n;};
    const select=items=>{const n=node('select');for(const[value,label]of items){const o=node('option','',label);o.value=value;n.append(o);}return n;};
    const identity=s=>s?.board_id&&s?.boot_id?s.board_id+':'+s.boot_id+':'+(s.capabilities?.application?.id||''):'';
    const current=expected=>!disposed&&owner===expected&&identity(snapshot())===expected&&peer().known;
    const requireCurrent=expected=>{if(!current(expected))throw Error('Device session changed. No command was replayed.');};
    const writable=()=>current(owner)&&peer().has('pixel-corrections')&&!model?.busy;
    function button(label,run,icon,variant){
      const action=async()=>{try{return await run();}catch(error){report(error);}};
      if(ui()?.button)return ui().button(translate(label),action,{icon,variant});
      const b=node('button','',label);b.type='button';b.onclick=action;return b;
    }
    function card(id,title,capability,icon){
      const c=node('section','card feature-workspace'),body=node('div','card-body');c.id=id;c.hidden=true;c.dataset.capability=capability;
      c.dataset.panelId=id;c.dataset.panelLabel=title;c.dataset.workspaceArea='advanced';
      const heading=node('h2','',title);if(ui()?.icon)heading.prepend(ui().icon(icon));c.append(heading,body);layout.append(c);
      options.registerPanel?.(c);return {card:c,body};
    }
    const corrections=card('pixelCorrectionsCard','Pixel corrections','pixel-corrections','corrections');
    const diagnostics=card('pixelDiagnosticsCard','Pixel diagnostics','pixel-diagnostic','diagnostics');
    const inspector=card('apiInspectorCard','API inspector','identity','api');
    const fileCard=root.UserFiles?card('userFilesCard','Files','managed-files','folder'):null;
    const userFiles=fileCard?root.UserFiles.create({...options,...fileCard,node,button,select,api,pin,snapshot,peer}):null;
    const runtimeCard=root.RuntimeMonitor?card('runtimeMonitorCard','CPU & memory','telemetry','activity'):null;
    const runtimeMonitor=runtimeCard?root.RuntimeMonitor.create({...options,...runtimeCard,node,button,api,pin,snapshot,peer}):null;
    function reveal(target){if(!target)return;if(options.reveal)options.reveal(target);else root.studioReveal?.(target);target.scrollIntoView({behavior:'smooth',block:'start'});}
    const groupCard=root.PixelGroups?card('pixelGroupsCard','Pixel groups','pixel-group-layout','mapping'):null;
    const groups=groupCard?root.PixelGroups.create({...options,...groupCard,node,field,numeric,button,select,snapshot,peer,api,command,pin,report,reveal,diagnostics:diagnostics.card,corrections:corrections.card}):null;
    const tabs=node('nav','feature-tabs page-tabs');tabs.setAttribute('aria-label',translate('Pixel tools'));
    const diagnosticLink=button('Pixel diagnostics',()=>reveal(peer().has('pixel-diagnostic')?diagnostics.card:document.getElementById('pixelDiagnostic')?.closest('.card')),'diagnostics');
    const correctionLabel=node('span','active','Corrections');correctionLabel.setAttribute('aria-current','page');tabs.append(diagnosticLink,correctionLabel);
    const groupLinks=[];function groupLink(nav){if(!groupCard)return;const link=button('Pixel groups',()=>reveal(groupCard.card),'mapping');groupLinks.push(link);nav.append(link);}groupLink(tabs);
    const enabled=input('checkbox'),notice=node('p','status-note'),rows=node('div','correction-rules'),tools=node('div','tools row'),mapping=node('div','correction-map');
    notice.setAttribute('role','status');mapping.setAttribute('aria-label',translate('Applied pixel mapping'));
    const revision=node('span','badge','Not loaded'),filter=input('search');filter.inputMode='numeric';filter.placeholder=translate('Find physical pixel');filter.setAttribute('aria-label',translate('Find physical pixel in applied map'));
    const mapDisclosure=node('details','mapping-disclosure'),mapTitle=node('summary','','Wire map'),mapTools=node('div','tools row'),mapNotice=node('p','status-note');mapTitle.append(revision);mapNotice.setAttribute('role','status');
    const showMap=view=>{if(!model?.saved||!current(owner))return;model.mappingView=view;drawMapping();rememberPresentation();};
    const mapApplied=button('Applied map',()=>showMap('applied'),'check'),mapDraft=button('Draft map',()=>showMap('draft'),'edit');mapTools.append(mapApplied,mapDraft);
    mapDisclosure.append(mapTitle,mapTools,mapNotice,filter,mapping);
    // Match the existing Flutter editor's fixed controls / scrolling rules split.
    // A long list must not carry Save, Reload or the global switch out of reach.
    const fixed=node('div','correction-toolbar');fixed.append(tabs,tools,field('Apply corrections',enabled),notice);
    rows.tabIndex=0;rows.setAttribute('role','region');rows.setAttribute('aria-label',translate('Correction rules'));
    corrections.body.append(fixed,rows,mapDisclosure);
    function message(text,error=false){notice.textContent=translate(text);notice.classList.toggle('field-error',error);}
    const livePreview=input('checkbox');livePreview.checked=true;
    const livePreviewField=field('Live preview',livePreview);fixed.append(livePreviewField);
    // Adapt Flutter's connectionRevision and foreground ownership. A board/boot
    // alone is not the connection which acquired a temporary preview lease.
    let pageLeaving=false;
    function previewOwner(){
      if(!current(owner))return null;
      try{const connection=pin?.();if(!connection)return null;
        return {key:JSON.stringify([owner,connection]),editor:owner,pin:Object.freeze({...connection})};
      }catch{return null;}
    }
    function requirePreviewOwner(identity){requireCurrent(identity.editor);if(previewOwner()?.key!==identity.key)throw Error('Device connection changed. No preview was replayed.');}
    const preview=root.createPixelCorrectionPreview?.({
      owner:previewOwner,
      allowed:()=>!pageLeaving&&document.visibilityState!=='hidden'&&current(owner)&&peer().has('pixel-corrections-preview')&&!!model?.saved&&!model.conflict&&validCorrectionDraft(model),
      configuration:()=>({revision:model.saved.revision,enabled:model.enabled,rules:model.rules}),
      pixels:()=>model.saved.pixels,project:correctionSources,
      // No named WS preview command exists. Use the shared fenced HTTP owner,
      // retaining the acquisition pin through its asynchronous preflight too.
      send:async(value,identity)=>{requirePreviewOwner(identity);const reply=await api(correctionPath+'/preview',{method:'POST',peer:identity.pin,body:new URLSearchParams({configuration:JSON.stringify(value)})});requirePreviewOwner(identity);return reply;},
      read:async identity=>{requirePreviewOwner(identity);const reply=await api(correctionPath,{peer:identity.pin});requirePreviewOwner(identity);return reply;},
      onChange:state=>{if(state.error)message(state.error,true);}
    });
    const previewVisibility=()=>{if(document.visibilityState==='hidden')void preview?.stop();};
    const previewPageHide=()=>{pageLeaving=true;void preview?.stop();};
    const previewPageShow=()=>{pageLeaving=false;}; // BFCache return never starts output.
    document.addEventListener?.('visibilitychange',previewVisibility);
    root.addEventListener('pagehide',previewPageHide);root.addEventListener('pageshow',previewPageShow);
    livePreview.onchange=()=>{if(!livePreview.checked)void preview?.stop();else if(model?.dirty)startPreview();};
    function startPreview(){if(!livePreview.checked||!preview||!validCorrectionDraft(model)||model.conflict)return;if(preview.snapshot().enabled)preview.edited();else void preview.start();}
    function changed(){if(!model)return;correctionEdits++;model.dirty=true;message(model.conflict?'Board changed elsewhere. Reload before saving.':'Unsaved changes',model.conflict);sync();if(model.mappingView==='draft')drawMapping();rememberPresentation();startPreview();}
    enabled.onchange=()=>{if(!writable()||!model?.saved)return;model.enabled=enabled.checked;changed();};
    const validNumericDraft=(text,min,max)=>text===undefined||text.trim()!==''&&Number.isSafeInteger(Number(text))&&Number(text)>=min&&Number(text)<=max;
    const validCorrectionDraft=record=>!!record?.saved&&record.rules.every(rule=>validNumericDraft(rule.pixelDraft,1,record.saved.pixels)&&(rule.kind!=='shift'||validNumericDraft(rule.offsetDraft,-record.saved.pixels,record.saved.pixels)));
    let mapStateKey='',mapColorKey='';
    function mappingKey(){return JSON.stringify([owner,model?.mappingView,model?.saved?.revision,correctionEdits,filter.value]);}
    function mappingColorKey(){const value=snapshot();return JSON.stringify([current(owner),peer().has('pixel-diagnostic'),model?.probe?.index,model?.probe?.position,model?.probe?.dirty,value?.pixels,value?.pixel_groups,value?.capabilities,value?.pixel_frame]);}
    function mappingPixel(index,source){
      const tile=node('span','mapping-pixel'),address=node('span','mapping-wire',String(index+1)),dot=node('span','mapping-dot',source<0?'×':String(source+1));
      tile.setAttribute('role','img');tile.dataset.wire=String(index);tile.dataset.source=String(source);
      address.setAttribute('aria-hidden','true');dot.setAttribute('aria-hidden','true');tile.append(address,dot);return tile;
    }
    function paintDiagram(container,rgb,focus){
      for(const tile of container.children){const index=Number(tile.dataset.wire),source=Number(tile.dataset.source),dot=tile.children[1],color=source<0?0:rgb?.[source];
        const label=translate('Wire')+' '+(index+1)+' ← '+(source<0?translate('Off'):translate('Raw')+' '+(source+1)),caption=label+' · '+(color===undefined?translate('RGB unavailable'):'#'+color.toString(16).padStart(6,'0').toUpperCase());
        tile.dataset.focused=String(index===focus);dot.dataset.known=String(color!==undefined);dot.dataset.black=String(source<0);
        const background=color===undefined?'':'#'+color.toString(16).padStart(6,'0'),ink=color===undefined?'':correctionInk(color);
        if(dot.dataset.rgb!==background){dot.dataset.rgb=background;dot.style.backgroundColor=background;dot.style.color=ink;}
        if(tile.getAttribute('aria-label')!==caption){tile.setAttribute('aria-label',caption);tile.title=caption;}
      }
    }
    function paintMapping(){mapColorKey=mappingColorKey();if(mapDisclosure.open&&model?.saved)paintDiagram(mapping,correctionRgb(snapshot(),model.saved.pixels,peer().role,current(owner)),selectedCorrectionCursor());}
    function drawMapping(){
      mapStateKey=mappingKey();
      const draft=model?.mappingView==='draft';mapping.replaceChildren();
      for(const [control,selected]of [[mapApplied,!draft],[mapDraft,draft]]){control.setAttribute('aria-pressed',String(selected));control.disabled=!model?.saved;}
      mapping.setAttribute('aria-label',translate(draft?'Draft pixel mapping':'Applied pixel mapping'));filter.setAttribute('aria-label',translate('Find physical pixel in wire map'));
      mapNotice.textContent=draft?translate('Local preview · not applied'):'';mapNotice.classList.toggle('field-error',false);
      if(!mapDisclosure.open||!model?.saved)return;
      if(draft&&!validCorrectionDraft(model)){mapNotice.textContent=translate('Complete invalid values to preview the draft');mapNotice.classList.toggle('field-error',true);return;}
      const sources=draft?correctionSources(model.saved.pixels,model.enabled,model.rules):model.saved.sources,match=filter.value.trim();
      if(!sources){mapNotice.textContent=translate('Applied mapping unavailable');return;}
      sources.forEach((source,index)=>{if(match&&!String(index+1).includes(match))return;
        mapping.append(mappingPixel(index,source));
      });
      paintMapping();
      if(!mapping.children.length)mapNotice.textContent=translate('No matching pixels');
    }
    function rememberPresentation(){if(!model||!current(owner))return;const next=pixelToolsWorkspace({mapView:model.mappingView,mapOpen:mapDisclosure.open,mapFilter:filter.value,advancedOpen:advancedProbe.open,expandedRules:model.saved?model.rules.flatMap((rule,index)=>rule.previewOpen?[ruleDisclosureKey(rule,index,model.rules)]:[]):model.presentation.expandedRules});
      if(JSON.stringify(next)===JSON.stringify(model.presentation))return;model.presentation=next;options.workspaceChanged?.();}
    filter.oninput=()=>{drawMapping();rememberPresentation();};mapDisclosure.ontoggle=()=>{drawMapping();rememberPresentation();};
    const rowRefreshers=[],cursorActions=[],rulePreviews=[];let correctionEdits=0,addressContextKey='';
    function refreshRulePreviews(){if(!rulePreviews.some(view=>view.open()))return;const key=mappingColorKey(),rgb=correctionRgb(snapshot(),model?.saved?.pixels,peer().role,current(owner));for(const view of rulePreviews)view.refresh(key,rgb);}
    // Diagnostics select RAW inputs. Rules select WIRE destinations. Resolve
    // through the saved map; never migrate existing rule coordinates.
    function selectedCorrectionCursor(){
      const p=model?.probe,s=diagnosticShape(snapshot(),peer().role);
      if(!current(owner)||!peer().has('pixel-diagnostic')||!s||!p||(!p.dirty&&!p.observation))return null;
      if(position.value===''||!position.checkValidity())return null;
      const index=Number(position.value)-1,used=s.layout?.used_pixels||s.pixels;
      if(!Number.isInteger(index)||index<0||index>=used||model.conflict||(model.dirty&&preview?.snapshot().enabled))return null;
      const sources=model.saved?.sources;
      if(!Array.isArray(sources)||sources.length!==model.saved.pixels)return null;
      const destinations=sources.flatMap((source,wire)=>source===index?[wire]:[]);
      return destinations.length===1?destinations[0]:null;
    }
    function refreshCursorActions(){const index=selectedCorrectionCursor();for(const refresh of cursorActions)refresh(index);}
    function drawRows(){
      rows.replaceChildren();rowRefreshers.length=0;cursorActions.length=0;rulePreviews.length=0;if(!model)return;
      if(!model.rules.length)rows.append(node('div','empty-state','Original pixel order · no correction rules'));
      model.rules.forEach((rule,index)=>{
        const row=node('div','correction-rule'),rowOwner=owner,rowModel=model,kind=select([['black','Black out · keep address'],['remove','Remove address'],['shift','Shift following pixels']]);kind.value=rule.kind;
        const editable=()=>current(rowOwner)&&model===rowModel&&[...rows.children].includes(row)&&rowModel.rules[index]===rule&&writable();
        const toggle=input('checkbox'),ruleLabel=node('label','correction-rule-toggle');
        toggle.checked=rule.enabled!==false;toggle.setAttribute('aria-label',translate('Enable rule')+' '+(index+1));
        ruleLabel.append(toggle,node('span','rule-number',String(index+1)));
        row.dataset.disabled=String(!toggle.checked);
        toggle.onchange=()=>{if(!editable())return;rule.enabled=toggle.checked;row.dataset.disabled=String(!toggle.checked);changed();};
        const pixels=model.saved?.pixels||480,pixel=numeric(rule.pixelDraft??rule.pixel+1,1,pixels),offset=numeric(rule.offsetDraft??rule.offset,-pixels,pixels);
        function stepped(label,control,read){
          const stepper=node('div','correction-number-stepper');control.setAttribute('aria-label',translate(label)+' · '+(index+1));
          const step=amount=>{if(!editable())return;const base=control.value!==''&&control.checkValidity()?Number(control.value):read();control.value=Math.max(Number(control.min),Math.min(Number(control.max),base+amount));control.oninput();};
          const decrease=label==='Pixel'?'Decrease pixel':'Decrease offset',increase=label==='Pixel'?'Increase pixel':'Increase offset';
          const less=button(decrease,()=>step(-1),'minus'),more=button(increase,()=>step(1),'plus');
          for(const b of [less,more]){b.classList.add('icon-button');b.setAttribute('aria-label',translate(b===less?decrease:increase)+' · '+(index+1));b.title=b.getAttribute('aria-label');const caption=b.querySelector('.button-label');if(caption)caption.hidden=true;}
          rowRefreshers.push(()=>{const valid=control.value!==''&&control.checkValidity(),value=valid?Number(control.value):read();control.setAttribute('aria-invalid',String(!valid));less.dataset.boundary=String(value<=Number(control.min));more.dataset.boundary=String(value>=Number(control.max));less.disabled=!writable()||less.dataset.boundary==='true';more.disabled=!writable()||more.dataset.boundary==='true';});
          stepper.append(less,control,more);return field(label,stepper);
        }
        const pixelField=stepped('Pixel',pixel,()=>rule.pixel+1),offsetField=stepped('Offset',offset,()=>rule.offset);offsetField.hidden=rule.kind!=='shift';
        kind.onchange=()=>{if(!editable())return;rule.kind=kind.value;rule.offset=0;rule.offsetDraft='0';offset.value=0;offsetField.hidden=rule.kind!=='shift';changed();};
        pixel.oninput=()=>{if(!editable())return;rule.pixelDraft=pixel.value;if(pixel.checkValidity()&&pixel.value!=='')rule.pixel=+pixel.value-1;changed();};
        offset.oninput=()=>{if(!editable())return;rule.offsetDraft=offset.value;if(offset.checkValidity()&&offset.value!=='')rule.offset=+offset.value;changed();};
        const useCursor=button('Use selected cursor',()=>{if(!editable())return;const selected=selectedCorrectionCursor();if(selected===null)return;pixel.value=selected+1;pixel.oninput();},'pointer');
        useCursor.classList.add('icon-button');const cursorCaption=useCursor.querySelector('.button-label');if(cursorCaption)cursorCaption.hidden=true;
        cursorActions.push(selected=>{const hidden=!peer().has('pixel-diagnostic');if(useCursor.hidden!==hidden)useCursor.hidden=hidden;useCursor.disabled=!writable()||selected===null;
          const title=selected===null?translate('Use selected cursor'):translate('Use selected cursor ({pixel})').replace('{pixel}',selected+1);
          useCursor.title=title;useCursor.setAttribute('aria-label',title+' · '+(index+1));});
        const move=direction=>{if(!editable())return;const target=index+direction;if(target<0||target>=model.rules.length)return;[model.rules[index],model.rules[target]]=[model.rules[target],model.rules[index]];changed();drawRows();};
        const up=button('Move up',()=>move(-1),'up'),down=button('Move down',()=>move(1),'down'),remove=button('Remove rule',()=>{if(!editable())return;model.rules.splice(index,1);changed();drawRows();},'trash');
        up.dataset.boundary=String(index===0);down.dataset.boundary=String(index===model.rules.length-1);
        for(const b of [up,down,remove]){b.classList.add('icon-button');b.setAttribute('aria-label',translate(b===up?'Move up':b===down?'Move down':'Remove rule')+' '+(index+1));b.title=b.getAttribute('aria-label');const label=b.querySelector('.button-label');if(label)label.hidden=true;}
        const address=node('p','correction-address');let lastAddressKey;
        rowRefreshers.push(()=>{
          if(lastAddressKey===addressContextKey)return;lastAddressKey=addressContextKey;
          if(pixel.value===''||!pixel.checkValidity()){address.textContent=translate('Complete the pixel address');return;}
          if([...rows.children].slice(0,index).some(r=>[...r.querySelectorAll('input')].some(n=>n.type==='number'&&(n.value===''||!n.checkValidity())))){address.textContent=translate('Complete prior rules to locate source');return;}
          const a=correctionAddress({wire:rule.pixel,pixels,enabled:model.enabled,rules:model.rules.slice(0,index),state:snapshot(),role:peer().role,mapping:options.tableMapping?.(),current:current(owner)});
          let copy=translate('Wire')+' '+(a.wire+1);
          if(a.source>=0)copy+=' · '+translate('Raw')+' '+(a.source+1);
          if(a.logical)copy+=' · ≈ '+translate('Cell')+' '+a.logical+(a.name&&a.name!==String(a.logical)?' ('+a.name+')':'');
          if(a.box)copy+=' · ≈ '+translate('Box')+' '+a.box+' '+translate(a.part==='button'?'Button':'Indicator');
          if(a.group)copy+=' · '+translate('Pixel')+' '+a.pixel+'/'+a.count+' · '+translate('Group')+' '+a.group;
          if(a.reason)copy+=' · '+translate({'black':'Black from prior rule · no logical source','outside-groups':'Outside assigned groups','invalid-rules':'Complete prior rules to locate source','not-current':'Logical address not current','no-logical-map':'Logical cell unavailable','unavailable':'Logical address unavailable'}[a.reason]);
          address.textContent=copy;
        });
        const preview=node('details','correction-rule-preview'),previewTitle=node('summary','','Rule preview'),previewDescription=node('p','status-note'),previewMap=node('div','correction-map correction-rule-map');
        preview.setAttribute('aria-label',translate('Rule preview')+' '+(index+1));preview.open=rule.previewOpen===true;
        previewMap.setAttribute('aria-label',translate('Local preview · not applied')+' · '+(index+1));preview.append(previewTitle,previewDescription,previewMap);
        let previewStructure=-1,previewVisual='',previewFocus=null;
        const attached=()=>!disposed&&model===rowModel&&owner===rowOwner&&[...rows.children].includes(row)&&rowModel.rules[index]===rule;
        const view={open:()=>attached()&&preview.open,refresh(key,rgb){
          if(!attached()||!preview.open)return;
          if(previewStructure!==correctionEdits){previewStructure=correctionEdits;previewVisual='';previewMap.replaceChildren();
            const prefix={...rowModel,rules:rowModel.rules.slice(0,index+1)};
            if(!validCorrectionDraft(prefix)){previewDescription.textContent=translate('Complete invalid values to preview the draft');previewDescription.classList.toggle('field-error',true);return;}
            const window=correctionWindow(pixels,rowModel.enabled,rowModel.rules,index);previewFocus=window.focus;
            previewDescription.textContent=translate(correctionDescriptions[rule.kind]);previewDescription.classList.toggle('field-error',false);
            window.sources.forEach((source,i)=>previewMap.append(mappingPixel(window.start+i,source)));
          }
          if(previewVisual!==key){previewVisual=key;paintDiagram(previewMap,rgb,previewFocus);}
        }};
        preview.ontoggle=()=>{if(!attached())return;rule.previewOpen=preview.open;refreshRulePreviews();rememberPresentation();};rulePreviews.push(view);
        const actions=node('div','tools row');actions.append(useCursor,up,down,remove);row.append(ruleLabel,field('Operation',kind),pixelField,offsetField,actions,address,preview);rows.append(row);
      });sync();
    }
    function readInput(n){if(n.value.trim()===''||!n.checkValidity()){n.reportValidity();throw Error('Check the highlighted correction value');}}
    async function load(save=false){
      const expected=owner,record=model;if(!record||record.busy)return;requireCurrent(expected);
      if(!peer().has('pixel-corrections'))throw Error('Pixel corrections were not acknowledged');
      if(save){if(!record.saved||record.conflict)throw Error('Reload corrections before saving');for(const n of rows.querySelectorAll('input'))if(n.type==='number')readInput(n);if(!validCorrectionDraft(record))throw Error('Check the highlighted correction value');}
      record.busy=true;record.attempted=true;sync();message(save?'Saving and verifying…':'Reading board…');
      try{
        const lease=pin?.();
        const wanted={revision:record.saved?.revision,enabled:record.enabled,rules:record.rules.map(({kind,pixel,offset,enabled})=>({kind,pixel,offset,enabled}))};
        // Retire RAM preview before explicit save/reload. A failed release
        // leaves the draft intact and never overwrites the stored rules.
        if(preview&&(preview.snapshot().enabled||preview.snapshot().lease!==null||preview.snapshot().pending)&&!await preview.stop())throw Error('Preview release not confirmed; retry after it expires');
        if(save)await command(correctionPath,{configuration:JSON.stringify(wanted)});
        requireCurrent(expected);const result=parseCorrections(await api(correctionPath,lease?{peer:lease}:{}));requireCurrent(expected);
        if(save&&(!correctionsMatch(result,wanted)||result.revision<wanted.revision))throw Error('Correction readback differs. Reload before editing; no write was replayed.');
        record.saved=result;record.rules=result.rules.map((r,index)=>({...r,previewOpen:record.presentation.expandedRules.includes(ruleDisclosureKey(r,index,result.rules))}));record.enabled=result.enabled;record.dirty=false;record.conflict=false;record.observedRevision=result.revision;
        record.presentation.expandedRules=record.rules.flatMap((rule,index)=>rule.previewOpen?[ruleDisclosureKey(rule,index,record.rules)]:[]);
        enabled.checked=record.enabled;revision.textContent=translate('Revision')+' '+result.revision;drawRows();drawMapping();message(result.error||(save?'Saved and verified':'Stored on this board'),!!result.error);
      }catch(error){if(save)record.conflict=true;if(current(expected))message(error.message,true);report(error);}
      finally{record.busy=false;if(current(expected))sync();}
    }
    const add=button('Add rule',()=>{if(!writable()||!model.saved||model.rules.length>=32)return;model.rules.push({kind:'black',pixel:0,offset:0,enabled:true});changed();drawRows();},'plus');
    const save=button('Save to board',()=>load(true),'save','primary');
    // Adapt Flutter PixelCorrectionsPage.reload: dismissal is Cancel, and the
    // decision belongs to the exact editor/peer which opened it. Reuse the
    // application's modal rather than freezing its event loop with confirm().
    let reloadPrompt=null;
    function dismissReload(){reloadPrompt?.close();}
    async function reloadCorrections(){
      if(reloadPrompt||!writable())return;
      const expected=owner,record=model;
      if(record.dirty){
        if(!ui()?.dialog)throw Error('Correction confirmation is unavailable. Edits were kept.');
        await new Promise(resolve=>{
          const prompt=ui().dialog({title:translate('Discard correction edits?'),closeLabel:translate('Close'),
            description:translate('Reload keeps the saved board configuration and discards only these edits.'),
            actions:[{label:translate('Cancel'),icon:'close',autofocus:true},
              {label:translate('Reload'),icon:'refresh',variant:'primary',action:async()=>{
                if(reloadPrompt!==prompt||!current(expected)||model!==record||!writable())return false;
                // Keep the modal's async owner until the read restores enabled
                // controls, so native close can return focus to Reload.
                await load();resolve(true);
              }}]});
          reloadPrompt=prompt;
          prompt.element.addEventListener('close',()=>{if(reloadPrompt===prompt)reloadPrompt=null;resolve(false);},{once:true});
        });
        return;
      }
      requireCurrent(expected);
      if(model!==record)throw Error('Device session changed. Edits were kept.');
      await load();
    }
    const reload=button('Reload',reloadCorrections,'refresh');tools.append(add,save,reload);
    function sync(){
      const ready=writable()&&!!model?.saved;
      livePreviewField.hidden=!preview||!peer().has('pixel-corrections-preview');livePreview.disabled=!ready;
      for(const control of rows.querySelectorAll('input,select,button'))control.disabled=!ready||control.dataset.boundary==='true';
      enabled.disabled=!ready;add.disabled=!ready||model.rules.length>=32;save.disabled=!ready||!model.dirty||model.conflict||!validCorrectionDraft(model);reload.disabled=!current(owner)||!peer().has('pixel-corrections')||model?.busy===true;
      diagnosticLink.disabled=!peer().has('pixel-diagnostic')&&!peer().has('streaming');
      // High-rate RGB/status pushes do not recompute every prefix projection.
      // Only edits, connection/context, spans or the existing cell map matter.
      const observed=snapshot();addressContextKey=JSON.stringify([correctionEdits,owner,current(owner),peer().role,observed?.pixel_groups,observed?.capabilities,options.tableMapping?.()]);
      for(const refresh of rowRefreshers)refresh();
      refreshCursorActions();
      refreshRulePreviews();
      if(mapDisclosure.open&&mappingColorKey()!==mapColorKey)paintMapping();
    }

    // PixelProbe / PixelDiagnosticObservation from the shared mobile contract.
    // The wire command publishes an entire immutable frame, including its black
    // tail; it is not a browser-side stream or a patch to a previous frame.
    const diagnosticTabs=node('nav','feature-tabs page-tabs');diagnosticTabs.setAttribute('aria-label',translate('Pixel tools'));
    const diagnosticLabel=node('span','active','Diagnostics');diagnosticLabel.setAttribute('aria-current','page');
    const correctionsLink=button('Pixel corrections',()=>reveal(corrections.card),'corrections');correctionsLink.dataset.openCorrections='true';diagnosticTabs.append(diagnosticLabel,correctionsLink);
    groupLink(diagnosticTabs);
    const position=numeric(1,1,480),grouping=select([['1','Single pixel'],['6','6-pixel group']]),color=input('color','#ffffff');
    position.inputMode='numeric';position.setAttribute('aria-label',translate('Physical pixel'));
    const probeFields=node('div','tools row'),probeActions=node('div','tools row'),live=node('p','status-note'),draftLabel=node('span','badge'),probeStatus=node('p','status-note');
    live.setAttribute('role','status');probeStatus.setAttribute('role','status');probeFields.append(field('Physical pixel',position),field('Step',grouping),field('Probe color',color),draftLabel);
    const probe=()=>model?.probe;
    function rememberProbe(){const p=probe();if(!p||!current(owner))return;model.diagnosticDraft=diagnosticWorkspace({...p,position:position.value});options.workspaceChanged?.();}
    const advancedProbe=node('details','diagnostic-advanced'),advancedTitle=node('summary','','Advanced probe'),advancedFields=node('div','tools row'),optionNotice=node('p','status-note field-error','\u00a0');optionNotice.setAttribute('role','status');advancedProbe.append(advancedTitle,advancedFields,optionNotice);
    advancedProbe.ontoggle=rememberPresentation;
    const probeEffect=select([['solid','Solid'],['blink','Flash'],['breathe','Breathing'],['rainbow','Rainbow']]),composition=select([['clear','Solo'],['overlay','Overlay']]);
    const effectField=field('Probe effect',probeEffect),compositionField=field('Composition',composition);advancedFields.append(effectField,compositionField);
    const timingLabels={period_ms:'Period · ms',duty_percent:'On time · %',min_brightness:'Minimum brightness',max_brightness:'Maximum brightness'},timingControls={},timingFields=[];
    for(const[key,[min,max]]of Object.entries(diagnosticLimits)){const control=numeric(diagnosticDefaults[key],min,max);control.setAttribute('aria-label',translate(timingLabels[key]));timingControls[key]=control;const holder=field(timingLabels[key],control);timingFields.push(holder);advancedFields.append(holder);}
    const optionKeys=Object.keys(diagnosticDefaults);
    function observeOptions(p,v){for(const key of optionKeys)p[key]=v[key];p.timingDrafts={};}
    function editOptions(){const p=probe();if(!p||!current(owner)||p.pending)return;p.dirty=true;p.effect=probeEffect.value;p.composite=composition.value;p.timingDrafts={};for(const[key,control]of Object.entries(timingControls)){p.timingDrafts[key]=control.value;if(control.value!==''&&control.checkValidity())p[key]=Number(control.value);}syncProbe();rememberProbe();}
    probeEffect.onchange=editOptions;composition.onchange=editOptions;for(const control of Object.values(timingControls))control.oninput=editOptions;
    function requestedOptions(p,off=false){
      const candidate={...diagnosticDefaults};if(!off){for(const key of optionKeys)if(p[key]!==undefined)candidate[key]=p[key];for(const[key,text]of Object.entries(p.timingDrafts||{})){if(text.trim()==='')throw Error('Complete diagnostic timing and brightness');candidate[key]=Number(text);}}
      const options=diagnosticOptions(candidate),effectKeys=optionKeys.filter(key=>key!=='composite');
      if(!peer().has('pixel-diagnostic-effects')&&effectKeys.some(key=>options[key]!==diagnosticDefaults[key]))throw Error('Diagnostic effects were not acknowledged');
      if(!peer().has('pixel-diagnostic-overlay')&&options.composite!=='clear')throw Error('Diagnostic overlay was not acknowledged');
      return {...(peer().has('pixel-diagnostic-effects')?Object.fromEntries(effectKeys.map(key=>[key,options[key]])):{}),...(peer().has('pixel-diagnostic-overlay')?{composite:options.composite}:{})};
    }
    const shape=()=>diagnosticShape(snapshot(),peer().role);
    const wholeGroup=()=>grouping.value!=='1';
    const spanAt=(s,index,whole,direction=0)=>s.layout?root.PixelGroups.probe(s.layout,index,whole,direction):{index:Math.floor(index/(whole?s.group:1))*(whole?s.group:1)+direction*(whole?s.group:1),count:whole?s.group:1};
    const diagnosticAllowed=()=>current(owner)&&peer().has('pixel-diagnostic')&&peer().has('command-ack')&&peer().has('leds')&&!!shape();
    function paintProbe(){const p=probe(),s=shape();if(!p||!s)return;position.max=s.layout?.used_pixels||s.pixels;position.value=p.position??String(p.index+1);color.value='#'+p.rgb.toString(16).padStart(6,'0');
      if(grouping.children[1]?.value!==String(s.group)){grouping.replaceChildren();for(const[value,label]of [['1','Single pixel'],[String(s.group),s.group+'-pixel group']]){const o=node('option','',label);o.value=value;grouping.append(o);}}
      grouping.value=p.wholeGroup||p.count!==1?String(s.group):'1';probeEffect.value=p.effect??'solid';composition.value=p.composite??'clear';for(const[key,control]of Object.entries(timingControls))control.value=p.timingDrafts?.[key]??p[key]??diagnosticDefaults[key];}
    function editedProbe(){const p=probe(),s=shape();if(!p||!s||!current(owner)||p.pending)return;p.dirty=true;p.position=position.value;
      if(position.value!==''&&position.checkValidity())p.index=Number(position.value)-1;
      p.wholeGroup=wholeGroup();p.count=spanAt(s,p.index,p.wholeGroup)?.count||1;
      if(/^#[a-f0-9]{6}$/i.test(color.value))p.rgb=parseInt(color.value.slice(1),16);
      syncProbe();rememberProbe();}
    position.oninput=editedProbe;color.oninput=editedProbe;
    grouping.onchange=()=>{editedProbe();const p=probe(),s=shape(),span=p&&s?spanAt(s,p.index,p.wholeGroup):null;if(span){p.index=span.index;p.position=String(p.index+1);p.count=span.count;paintProbe();syncProbe();rememberProbe();}};
    function observeProbe(value){const p=probe();if(!p)return;const observation=parseDiagnostic(value,peer().role);
      if(!observation||p.observation&&observation.revision<=p.observation.revision)return;
      p.observation=observation;
      if(observation.active&&!observation.off&&!p.dirty&&!p.pending){p.index=observation.index;p.position=String(p.index+1);p.count=observation.count;p.wholeGroup=observation.count!==1||p.wholeGroup===true;p.rgb=observation.rgb;observeOptions(p,observation);paintProbe();}
    }
    function syncProbe(){const p=probe(),s=shape(),ready=diagnosticAllowed()&&!p?.pending;
      if(p&&s&&grouping.children[1]){const span=spanAt(s,p.index,true);grouping.children[1].textContent=translate('Whole group')+(span?' · '+span.count:'');}
      for(const control of [position,grouping,color,show,allOff,rainbow])control.disabled=!ready;
      const effects=peer().has('pixel-diagnostic-effects'),overlay=peer().has('pixel-diagnostic-overlay');advancedProbe.hidden=!effects&&!overlay;effectField.hidden=!effects;compositionField.hidden=!overlay;for(const holder of timingFields)holder.hidden=!effects;
      for(const control of [probeEffect,composition,...Object.values(timingControls)])control.disabled=!ready;
      for(const control of Object.values(timingControls))control.setAttribute('aria-invalid',String(control.value===''||!control.checkValidity()));
      let validOptions=true;try{if(p)requestedOptions(p);optionNotice.textContent='\u00a0';}catch(error){validOptions=false;optionNotice.textContent=translate(error.message);}show.disabled=!ready||!validOptions;
      const before=p&&s?spanAt(s,p.index,wholeGroup(),-1):null,after=p&&s?spanAt(s,p.index,wholeGroup(),1):null;
      previous.disabled=!ready||!validOptions||!before||before.index<0;nextPixel.disabled=!ready||!validOptions||!after||after.index+after.count>s.pixels;
      follow.disabled=!ready||!p?.observation?.active||p.observation.off;
      correctionsLink.hidden=!peer().has('pixel-corrections');
      draftLabel.textContent=translate(p?.dirty?'Local draft':'Following board');
      const v=p?.observation;
      live.textContent=!v?translate('No diagnostic observation from this device'):!v.active?translate('Board diagnostic inactive'):v.off?translate(v.composite==='overlay'?'Board reports probe mask off':'Board reports all pixels off'):
        `${translate('Board reports')} ${v.index+1}${v.count===1?'':'–'+(v.index+v.count)} · #${v.rgb.toString(16).padStart(6,'0').toUpperCase()}${v.extended?' · '+translate({solid:'Solid',blink:'Flash',breathe:'Breathing',rainbow:'Rainbow'}[v.effect])+' · '+translate(v.composite==='overlay'?'Overlay':'Solo'):''} · ${translate('Revision')} ${v.revision}`;
      refreshCursorActions();
      if(mapDisclosure.open&&mappingColorKey()!==mapColorKey)paintMapping();
    }
    async function sendProbe(action,direction=0){
      const expected=owner,p=probe(),s=shape();requireCurrent(expected);
      if(!diagnosticAllowed()||!p||!s)throw Error('Pixel diagnostics were not acknowledged for this device');
      if(p.pending)return;const lease=pin?.(),startRevision=p.observation?.revision??-1;
      let wanted;
      if(action!=='rainbow'){
        if(action==='off')wanted={index:0,count:1,rgb:0,off:true};
        else{
          if(position.value===''||!position.checkValidity()||!/^#[a-f0-9]{6}$/i.test(color.value)){position.reportValidity();throw Error('Choose a valid physical pixel and color');}
          const span=spanAt(s,Number(position.value)-1,wholeGroup(),direction);if(!span)throw Error('End of pixel group range');const {index,count}=span;
          if(index<0||index+count>s.pixels)throw Error('End of strip');
          wanted={index,count,rgb:parseInt(color.value.slice(1),16),off:false};
        }
      }
      if(wanted&&peer().has('pixel-group-layout'))wanted.group_layout_revision=s.layout.revision;
      if(wanted)Object.assign(wanted,requestedOptions(p,action==='off'));
      p.pending=true;syncProbe();probeStatus.classList.toggle('field-error',false);probeStatus.textContent=translate('Waiting for device confirmation…');
      try{
        // /api/led/diagnostic is the semantic client route. The shared command
        // owner sends pixel-diagnostic over WS or /api/command, never a raw POST.
        const reply=await command(action==='rainbow'?'/api/effect':'/api/led/diagnostic',action==='rainbow'?{value:'rainbow'}:wanted);
        requireCurrent(expected);let result=reply?.status||await api('/api/status',lease?{peer:lease}:{});requireCurrent(expected);
        if(identity(result)!==expected)throw Error('Diagnostic readback belongs to a different device session');
        const extendedRequest=wanted&&optionKeys.some(key=>key in wanted);
        const framePublished=value=>Number.isSafeInteger(value.pixel_frame?.revision)&&value.pixel_frame.revision>=0&&value.pixel_frame.revision<=0xffffffff&&value.pixel_frame.pending===false;
        if(extendedRequest&&!framePublished(result)){
          // One on-demand published-frame read; never replay the mutation or
          // compare an animated overlay against a fabricated static RGB frame.
          result=await api('/api/status?pixels=1',lease?{peer:lease}:{});requireCurrent(expected);
          if(identity(result)!==expected)throw Error('Diagnostic readback belongs to a different device session');
        }
        const observation=parseDiagnostic(result,peer().role);
        if(action==='rainbow'){if(result.effect!=='rainbow')throw Error('Device did not confirm rainbow');}
        else if(!observation||!observation.active||observation.revision<=startRevision||['index','count','rgb','off',...optionKeys.filter(key=>key in wanted)].some(key=>observation[key]!==wanted[key])||(optionKeys.some(key=>key in wanted)&&!observation.extended))throw Error('Diagnostic result differs. Inspect live state before another action.');
        if(extendedRequest&&!framePublished(result))throw Error('Diagnostic frame publication is not confirmed');
        observeProbe(result);
        if(action!=='off'&&action!=='rainbow'){p.index=wanted.index;p.count=wanted.count;p.rgb=wanted.rgb;}
        p.dirty=false;const latest=p.observation;
        if(latest?.active&&!latest.off){p.index=latest.index;p.count=latest.count;p.rgb=latest.rgb;observeOptions(p,latest);}
        p.position=String(p.index+1);paintProbe();if(action!=='off'&&action!=='rainbow')rememberProbe();probeStatus.textContent=translate('Device confirmed');
      }catch(error){if(current(expected)){probeStatus.textContent=error.message;probeStatus.classList.add('field-error');}throw error;}
      finally{p.pending=false;if(current(expected))syncProbe();}
    }
    const previous=button('Previous',()=>sendProbe('probe',-1),'left'),show=button('Show pixel',()=>sendProbe('probe'),'play','primary'),nextPixel=button('Next',()=>sendProbe('probe',1),'right');
    const allOff=button('All off',()=>sendProbe('off'),'power'),rainbow=button('Rainbow',()=>sendProbe('rainbow'),'palette');
    const follow=button('Use live cursor',()=>{const p=probe(),v=p?.observation;if(v?.active&&!v.off&&current(owner)){p.index=v.index;p.position=String(p.index+1);p.count=v.count;p.wholeGroup=v.count!==1||p.wholeGroup===true;p.rgb=v.rgb;observeOptions(p,v);p.dirty=false;paintProbe();syncProbe();rememberProbe();}},'refresh');
    for(const b of [previous,show,nextPixel,allOff,rainbow])b.dataset.write='pixel-diagnostic';
    probeActions.append(previous,show,nextPixel,allOff,rainbow,follow);diagnostics.body.append(diagnosticTabs,probeFields,probeActions,advancedProbe,live,probeStatus);

    const endpoints=select([]),response=node('pre','code-view'),inspectorStatus=node('span','badge','Read-only'),inspectorTools=node('div','tools row');
    response.tabIndex=0;response.setAttribute('aria-label',translate('Device JSON response'));let routeKey='';
    async function read(){if(inspectorBusy)return;const expected=owner;requireCurrent(expected);const lease=pin?.();inspectorBusy=true;syncInspector();try{
      const result=await api(endpoints.value,lease?{peer:lease}:{});requireCurrent(expected);inspectorPayload=redact(result);response.textContent=JSON.stringify(inspectorPayload,null,2);inspectorStatus.textContent=translate('Received');
    }finally{inspectorBusy=false;syncInspector();}}
    const readButton=button('Read device',read,'refresh'),copyButton=button('Copy JSON',async()=>{
      if(!inspectorPayload)return;if(navigator.clipboard?.writeText){await navigator.clipboard.writeText(JSON.stringify(inspectorPayload,null,2));inspectorStatus.textContent=translate('Copied');}
      else{const range=document.createRange();range.selectNodeContents(response);const selected=document.getSelection();selected.removeAllRanges();selected.addRange(range);response.focus();inspectorStatus.textContent=translate('Selected · use Copy');}
    },'copy');inspectorTools.append(field('Resource',endpoints),readButton,copyButton,inspectorStatus);inspector.body.append(inspectorTools,response);
    function syncInspector(){readButton.disabled=inspectorBusy||!current(owner);copyButton.disabled=inspectorPayload===null;}
    function inspectRoutes(){const c=peer(),list=[['/api/status','Live state'],['/api/peer/info','Peer identity']];if(c.has('telemetry'))list.push(['/api/telemetry','Full telemetry']);if(c.has('network'))list.push(['/api/network','Network settings']);if(c.has('pixel-corrections'))list.push([correctionPath,'Pixel corrections']);const key=JSON.stringify(list);if(key===routeKey)return;routeKey=key;const selected=endpoints.value;endpoints.replaceChildren();for(const[value,label]of list){const o=node('option','',label);o.value=value;endpoints.append(o);}if(list.some(([value])=>value===selected))endpoints.value=selected;}
    function linkDiagnostics(){const diagnostic=document.getElementById('pixelDiagnostic');if(!diagnostic||diagnostic.querySelector('[data-open-corrections]'))return;
      const link=button('Pixel corrections',()=>reveal(corrections.card),'corrections');link.dataset.openCorrections='true';diagnostic.append(link);}
    function update(value=snapshot()){
      if(disposed)return;const next=identity(value),c=peer();
      preview?.observe();
      if(reloadPrompt&&(!c.known||!c.has('pixel-corrections')))dismissReload();
      if(next!==owner){dismissReload();owner=next;model=next?models.get(next):null;
        if(next&&!model){const scope=options.workspaceScope?.(value)||'',draft=scope?diagnosticWorkspace(options.readWorkspace?.(scope)):null,presentation=pixelToolsWorkspace(scope?options.readPresentation?.(scope):null)||pixelToolsWorkspace({});
          model={scope,diagnosticDraft:draft,presentation,mappingView:presentation.mapView,saved:null,rules:[],enabled:true,dirty:false,conflict:false,busy:false,attempted:false,observedRevision:-1,probe:{index:0,count:1,rgb:0xffffff,...draft,dirty:!!draft,pending:false,observation:null}};
          const s=shape();if(s){model.probe.index=Math.min(model.probe.index,(s.layout?.used_pixels||s.pixels)-1);model.probe.count=spanAt(s,model.probe.index,model.probe.wholeGroup)?.count||1;}}
        if(model){models.set(next,model);if(models.size>8)models.delete(models.keys().next().value);filter.value=model.presentation.mapFilter;mapDisclosure.open=model.presentation.mapOpen;advancedProbe.open=model.presentation.advancedOpen;enabled.checked=model.enabled;revision.textContent=model.saved?translate('Revision')+' '+model.saved.revision:translate('Not loaded');paintProbe();}
        probeStatus.textContent='';
        inspectorPayload=null;response.textContent='';message('');drawRows();drawMapping();}
      if(!c.has('pixel-corrections'))corrections.card.hidden=true;else if(!options.registerPanel)corrections.card.hidden=false;
      if(!c.known||!c.has('pixel-diagnostic'))diagnostics.card.hidden=true;else if(!options.registerPanel)diagnostics.card.hidden=false;
      if(!c.known||!c.has('identity'))inspector.card.hidden=true;else if(!options.registerPanel)inspector.card.hidden=false;
      if(model&&c.has('pixel-corrections')){
        linkDiagnostics();const observed=value.pixel_corrections_revision;
        if(model.saved&&Number.isInteger(observed)&&observed>model.saved.revision&&observed!==model.observedRevision){model.observedRevision=observed;if(model.dirty){model.conflict=true;message('Board changed elsewhere. Reload before saving.',true);}else if(!model.busy)void load();}
        if(!model.attempted&&!model.busy)void load();
      }
      for(const link of document.querySelectorAll('[data-open-corrections]'))link.hidden=!c.has('pixel-corrections');
      for(const link of groupLinks)link.hidden=!c.has('pixel-group-layout');groups?.update(value);userFiles?.update();runtimeMonitor?.update();
      observeProbe(value);inspectRoutes();sync();syncProbe();syncInspector();
      if(mapDisclosure.open&&mappingKey()!==mapStateKey)drawMapping();
      if(c.known&&endpoints.value==='/api/status'&&!inspectorBusy){inspectorPayload=redact(value);response.textContent=JSON.stringify(inspectorPayload,null,2);inspectorStatus.textContent=translate('Live snapshot');}
    }
    const beforeUnload=event=>{if(groups?.dirty()||[...models.values()].some(record=>record.dirty||record.busy)){event.preventDefault();event.returnValue='';}};
    root.addEventListener('beforeunload',beforeUnload);
    installed={update,workspaceSnapshot(scope){return model&&model.scope===scope?diagnosticWorkspace(model.diagnosticDraft):undefined;},presentationSnapshot(scope){return model&&model.scope===scope?pixelToolsWorkspace(model.presentation):undefined;},dispose(){void preview?.dispose();disposed=true;dismissReload();document.removeEventListener?.('visibilitychange',previewVisibility);root.removeEventListener('pagehide',previewPageHide);root.removeEventListener('pageshow',previewPageShow);root.removeEventListener('beforeunload',beforeUnload);groups?.dispose();userFiles?.dispose();runtimeMonitor?.dispose();runtimeCard?.card.remove();corrections.card.remove();diagnostics.card.remove();inspector.card.remove();installed=null;}};
    update();return installed;
  }
  root.createFeatureWorkspaces=create;
  root.WebFeatureContracts={parseCorrections,correctionsMatch,correctionSources,correctionWindow,ruleDisclosureKey,pixelToolsWorkspace,correctionRgb,correctionInk,correctionAddress,diagnosticShape,diagnosticOptions,diagnosticWorkspace,parseDiagnostic,audioCues,redact};
})(typeof window==='undefined'?globalThis:window);
