//! Selected-owner API sharing. PCController's bridge correlation, no-replay,
//! public discovery metadata and lifecycle rules are adapted, not its media RPC.
use crate::{DeviceStatus, Endpoint, SerialCommand, Transport, connection, peer_http, usb_table};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};
#[path = "bridge_events.rs"]
mod events;
#[path = "bridge_transfer.rs"]
mod transfer;
#[path = "bridge_pixels.rs"]
mod pixels;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub enabled: bool,
    pub peer_discovery: bool,
    pub peer_package_sharing: bool,
    pub listen: String,
    pub port: u16,
    pub hostname: String,
    pub instance_id: String,
    pub mdns: bool,
    pub ssdp: bool,
    pub mndp: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            peer_discovery: true,
            peer_package_sharing: true,
            listen: "0.0.0.0".into(),
            port: 8789,
            hostname: std::env::var("COMPUTERNAME")
                .or_else(|_| std::env::var("HOSTNAME"))
                .unwrap_or_else(|_| "pakarcontroller".into())
                .to_ascii_lowercase(),
            instance_id: uuid::Uuid::new_v4().to_string(),
            mdns: true,
            ssdp: true,
            mndp: true,
        }
    }
}
impl Settings {
    pub fn validate(&self) -> Result<SocketAddr, String> {
        let ip: IpAddr = self
            .listen
            .parse()
            .map_err(|_| "Choose a numeric bridge listen address")?;
        if ip.is_multicast()
            || self.port == 0
            || crate::identity::normalize_name(&self.hostname).as_deref()
                != Ok(self.hostname.as_str())
            || uuid::Uuid::parse_str(&self.instance_id).is_err()
        {
            return Err("Invalid bridge hostname, instance identity or port".into());
        }
        Ok(SocketAddr::new(ip, self.port))
    }
}
#[derive(Clone)]
pub enum Route {
    Usb {
        port: String,
        admission: usb_table::Admission,
        sender: mpsc::Sender<SerialCommand>,
    },
    Network {
        endpoint: Endpoint,
        stamp: connection::Stamp,
        gate: connection::Gate,
    },
}
impl Route {
    fn current(&self) -> bool {
        match self {
            Self::Usb { admission, .. } => admission.stamp.current(),
            Self::Network { stamp, .. } => stamp.current(),
        }
    }
}
pub struct Observation {
    pub status: DeviceStatus,
    pub mapping: Value,
    pub route: Route,
    pub at: Instant,
}
struct Shared {
    demand: crate::usb_demand::Watchers,
    settings: Settings,
    observation: Mutex<Option<Observation>>,
    stop: Arc<AtomicBool>,
    clients: AtomicUsize,
    receipts: Mutex<BTreeMap<String, (Instant, Value, Option<Value>)>>,
    last_input: Mutex<Value>,
    peers: Mutex<BTreeMap<String, (Instant, Value)>>,
    events: Mutex<events::Journal>,
    active_commands: AtomicUsize,
}
pub struct Runtime {
    settings: Option<Settings>,
    shared: Option<Arc<Shared>>,
    discovery: Option<crate::platform::discovery::Advertiser>,
    pub error: String,
}
impl Default for Runtime {
    fn default() -> Self {
        Self {
            settings: None,
            shared: None,
            discovery: None,
            error: String::new(),
        }
    }
}
impl Runtime {
    pub fn demand(&self) -> crate::usb_demand::Demand {
        self.shared
            .as_ref()
            .map(|s| s.demand.demand())
            .unwrap_or_default()
    }
    pub fn reconcile(&mut self, settings: &Settings, fixture: bool) {
        if self.settings.as_ref() == Some(settings) {
            return;
        }
        self.stop();
        self.settings = Some(settings.clone());
        self.error.clear();
        if fixture || (!settings.enabled&&!settings.peer_discovery) {
            return;
        }
        if settings.peer_package_sharing{crate::peer_package::prime();}
        match settings
            .validate()
            .and_then(|address| TcpListener::bind(address).map_err(|e| e.to_string()))
        {
            Ok(listener) => {
                let _ = listener.set_nonblocking(true);
                let shared = Arc::new(Shared {
                    demand: Default::default(),
                    settings: settings.clone(),
                    observation: Mutex::new(None),
                    stop: Arc::new(AtomicBool::new(false)),
                    clients: AtomicUsize::new(0),
                    receipts: Mutex::new(BTreeMap::new()),
                    last_input: Mutex::new(Value::Null),
                    peers: Mutex::new(BTreeMap::new()),
                    events: Mutex::new(events::Journal::default()),
                    active_commands: AtomicUsize::new(0),
                });
                let owner = shared.clone();
                thread::spawn(move || {
                    while !owner.stop.load(Ordering::Acquire) {
                        match listener.accept() {
                            Ok((socket, _)) => {
                                if owner.clients.fetch_add(1, Ordering::AcqRel) >= 16 {
                                    owner.clients.fetch_sub(1, Ordering::AcqRel);
                                    continue;
                                }
                                let shared = owner.clone();
                                thread::spawn(move || {
                                    let _ = serve(socket, &shared);
                                    shared.clients.fetch_sub(1, Ordering::AcqRel);
                                });
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                thread::sleep(Duration::from_millis(20))
                            }
                            Err(_) => break,
                        }
                    }
                });
                self.discovery = Some(crate::platform::discovery::Advertiser::start(settings));
                self.shared = Some(shared);
            }
            Err(error) => self.error = error,
        }
    }
    pub fn observe(&self, observation: Option<Observation>) {
        if let Some(shared) = &self.shared {
            let value = observation
                .as_ref()
                .map(|o| json!({"status":o.status,"mapping":o.mapping}))
                .unwrap_or(Value::Null);
            let mut previous = shared.last_input.lock().unwrap();
            if *previous != value || observation.is_none() {
                *previous = value;
                *shared.observation.lock().unwrap() = observation;
            }
        }
    }
    pub fn snapshot(&self) -> Value {
        json!({"enabled":self.settings.as_ref().is_some_and(|s|s.enabled),"listening":self.shared.is_some(),
        "version":crate::peer_identity::VERSION,"compatibility":crate::peer_identity::advertised_policy(),
        "tracked_peers":self.shared.as_ref().and_then(|s|s.peers.lock().ok().map(|v|v.len())).unwrap_or(0),
        "error":self.error,"discovery":self.discovery.as_ref().map(|d|d.snapshot())})
    }
    /// Only call after the selected source's typed event admission succeeds.
    pub fn publish_event(&self, event: Value) {
        if let Some(shared) = &self.shared {
            publish_event(shared, event);
        }
    }
    pub fn stop(&mut self) {
        if let Some(shared) = self.shared.take() {
            shared.stop.store(true, Ordering::Release);
        }
        self.discovery = None;
    }
}
fn publish_event(shared: &Shared, event: Value) {
    if let Ok(o) = route(shared) {
        if event["board_id"] == o.status.board_id && event["boot_id"] == o.status.boot_id {
            shared.events.lock().unwrap().push(event);
        }
    }
}
impl Drop for Runtime {
    fn drop(&mut self) {
        self.stop();
    }
}

fn route(shared: &Shared) -> Result<Observation, String> {
    if shared.stop.load(Ordering::Acquire) {
        return Err("Bridge stopped".into());
    }
    let guard = shared
        .observation
        .lock()
        .map_err(|_| "Bridge owner unavailable")?;
    let o = guard
        .as_ref()
        .filter(|o| o.at.elapsed() < Duration::from_secs(8) && o.route.current())
        .ok_or("Selected board unavailable")?;
    if let Some(path) = o.status.bridge.as_ref().and_then(|v| v["path"].as_array()) {
        if path.is_empty()
            || path.len() >= 4
            || path.iter().any(|v| {
                v.as_str() == Some(&shared.settings.instance_id)
                    || !v.as_str().is_some_and(|s| uuid::Uuid::parse_str(s).is_ok())
            })
            || path
                .iter()
                .enumerate()
                .any(|(i, id)| path[..i].contains(id))
        {
            return Err("Cyclic bridge observation rejected".into());
        }
    }
    Ok(Observation {
        status: o.status.clone(),
        mapping: o.mapping.clone(),
        route: o.route.clone(),
        at: o.at,
    })
}
fn status_value(shared: &Shared, o: &Observation) -> Value {
    let mut status = o.status.clone();
    if let Some(c) = &mut status.capabilities {
        // HTTP firmware/AVR transfers preserve upstream bytes and receipts.
        // Binary pixel streaming is not implemented; do not advertise it.
        c.acknowledged_features.retain(|s| s != "streaming");
    }
    if matches!(o.route, Route::Usb { .. }) {
        // Advertise only operations implemented by this route, not every
        // capability reachable through a separate, unselected Wi-Fi connection.
        if let Some(c) = &mut status.capabilities {
            let usb_key_settings = c
                .acknowledged_features
                .iter()
                .any(|feature| feature == "usb-key-settings");
            let usb_pixel = c
                .acknowledged_features
                .iter()
                .any(|feature| feature == "usb-pixel-diagnostic")
                && c.acknowledged_features
                    .iter()
                    .any(|feature| feature == "usb-demand");
            let usb_groups = c.has("usb-api") && c.has(crate::pixel_groups::FEATURE);
            c.acknowledged_features.retain(|s| {
                matches!(
                    s.as_str(),
                    "table"
                        | "leds"
                        | "cell-batch"
                        | "command-ack"
                        | "cell-assignments"
                        | "cell-lock"
                        | "inputs"
                        | "outputs"
                        | "usb-key-settings"
                        | "reset-ack"
                        | "usb-pixel-diagnostic"
                        | "usb-demand"
                ) || matches!(s.as_str(), "key-settings" | "game-melodies") && usb_key_settings
                    || s == "pixel-diagnostic" && usb_pixel
                    || s == crate::pixel_groups::FEATURE && usb_groups
            });
            c.modes.retain(|m| m.id == "cell" || m.id == "first-wins");
        }
    }
    let mut v = serde_json::to_value(status).unwrap_or(Value::Null);
    let mut path = o
        .status
        .bridge
        .as_ref()
        .and_then(|v| v["path"].as_array())
        .cloned()
        .unwrap_or_default();
    path.push(json!(shared.settings.instance_id));
    v["bridge"] = json!({"instance_id":shared.settings.instance_id,"hostname":shared.settings.hostname,"product":"PakarController","version":crate::peer_identity::VERSION,"session_id":crate::peer_identity::session_id(),"compatibility":crate::peer_identity::advertised_policy(),"path":path,
        "transport":if matches!(o.route,Route::Usb{..}){"usb"}else{"network"},"max_hops":4,"events":"/api/events","websocket":"/ws","http_command":"/api/command"});
    v["command_clock_ms"] = clock(o).map_or(Value::Null, |v| json!(v));
    v
}
pub fn matches_hostname(status: &DeviceStatus, expected: &str) -> bool {
    if status.hostname == expected {
        return true;
    }
    let Some(b) = &status.bridge else {
        return false;
    };
    let Some(path) = b["path"].as_array() else {
        return false;
    };
    b["product"] == "PakarController"
        && b["hostname"] == expected
        && !path.is_empty()
        && path.len() <= 4
        && path.last() == Some(&b["instance_id"])
        && path.iter().all(|id| {
            id.as_str()
                .is_some_and(|s| uuid::Uuid::parse_str(s).is_ok())
        })
        && path
            .iter()
            .enumerate()
            .all(|(i, id)| !path[..i].contains(id))
}
fn clock(o: &Observation) -> Result<u64, String> {
    o.status
        .command_clock_ms
        .and_then(|v| v.checked_add(o.at.elapsed().as_millis() as u64))
        .ok_or("Board command clock unavailable".into())
}
fn trace(value: Option<&str>, id: &str) -> Result<String, String> {
    let parts: Vec<_> = value
        .unwrap_or("")
        .split(',')
        .filter(|v| !v.is_empty())
        .collect();
    if parts.len() >= 4
        || parts
            .iter()
            .any(|v| uuid::Uuid::parse_str(v).is_err() || *v == id)
    {
        return Err("Bridge forwarding loop or hop limit".into());
    }
    Ok(parts
        .into_iter()
        .chain(Some(id))
        .collect::<Vec<_>>()
        .join(","))
}
fn require_pin(o: &Observation, headers: &BTreeMap<String, String>) -> Result<(), String> {
    if headers.get("x-pakar-board").map(String::as_str) != Some(o.status.board_id.as_str())
        || headers.get("x-pakar-boot").map(String::as_str) != Some(o.status.boot_id.as_str())
    {
        return Err("Board/boot identity required or changed".into());
    }
    Ok(())
}
pub struct Reply {
    sender: mpsc::Sender<Result<DeviceStatus, String>>,
    stop: Arc<AtomicBool>,
}
impl Reply {
    pub fn new(sender: mpsc::Sender<Result<DeviceStatus, String>>, stop: Arc<AtomicBool>) -> Self {
        Self { sender, stop }
    }
    pub fn current(&self) -> bool {
        !self.stop.load(Ordering::Acquire)
    }
    pub fn send(
        &self,
        result: Result<DeviceStatus, String>,
    ) -> Result<(), mpsc::SendError<Result<DeviceStatus, String>>> {
        self.sender.send(if self.current() {
            result
        } else {
            Err("Sharing stopped; outcome not replayed".into())
        })
    }
}
pub(crate) fn usb_request(
    path: &str,
    form: &[(String, String)],
    status: &DeviceStatus,
) -> Result<(u8, Vec<u8>, Option<crate::cell_batch::Correlation>), String> {
    if !status
        .capabilities
        .as_ref()
        .is_some_and(|c| c.allows(path, form))
    {
        return Err("Capability unavailable".into());
    }
    let get = |key: &str| {
        form.iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .ok_or_else(|| format!("Missing {key}"))
    };
    Ok(match path {
        "/api/game/reset" => (0x4d, Vec::new(), None),
        "/api/game/settings" if form.iter().any(|(name, _)| name == "presentation") => {
            if form.len() != 1 { return Err("USB presentation must be edited separately".into()); }
            (0x69, crate::game_presentation::payload(get("presentation")?, &status.key_game["presentation"])?, None)
        }
        "/api/game/settings" => (0x67, crate::usb_table::key_settings_patch(form)?, None),
        "/api/game/press" => (0x68, vec![get("key")?.parse::<u8>().ok().filter(|key| (1..=9).contains(key)).ok_or("Invalid key")?], None),
        "/api/led/diagnostic" => {
            if !crate::pixel_diagnostic::usb_supported(status) {
                return Err("USB pixel diagnostics unavailable".into());
            }
            let (probe,off)=crate::pixel_diagnostic::Probe::from_form(status,form)
                .ok_or("Invalid or stale physical pixel group")?;
            (
                0x65,
                probe.usb(off).ok_or("Invalid pixel diagnostic")?,
                None,
            )
        }
        "/api/cells/assignments" => {
            let doc = crate::cell_assignments::Document::new(
                serde_json::from_str(get("assignments")?).map_err(|_| "Invalid assignments")?,
            )?;
            (0x62, doc.payload(), None)
        }
        "/api/cells/lock" => {
            let enabled = match get("enabled")? {
                "true" => 1,
                "false" => 0,
                _ => return Err("Lock requires a boolean".into()),
            };
            (0x63, vec![enabled], None)
        }
        "/api/cell" => {
            let id = get("id")?.parse().map_err(|_| "Invalid cell")?;
            (
                0x4c,
                crate::ui::shell::led_cell_payload(
                    id,
                    get("state")?,
                    get("effect").unwrap_or("immediate"),
                )
                .ok_or("Invalid cell command")?,
                None,
            )
        }
        "/api/cells" => {
            let batch = crate::cell_batch::Request::new(
                &crate::ui::cell_search::parse(get("ids")?)?,
                get("state")?,
                get("effect").unwrap_or("immediate"),
            )?;
            let correlation = crate::cell_batch::Correlation::new(
                batch,
                peer_http::Pin::from_status(status).ok_or("Peer unavailable")?,
                Transport::Serial,
            )?;
            (
                crate::cell_batch::OPCODE,
                correlation.request.payload(),
                Some(correlation),
            )
        }
        "/api/brightness" => {
            let value: u8 = get("value")?.parse().map_err(|_| "Invalid brightness")?;
            if value == 0 {
                return Err("Invalid brightness".into());
            }
            (0x51, vec![value], None)
        }
        "/api/effect" => (
            0x52,
            vec![crate::ui::shell::led_effect_code(get("value")?).ok_or("Unsupported effect")?],
            None,
        ),
        "/api/mode" => (
            0x4b,
            vec![crate::ui::shell::led_mode_code(get("value")?).ok_or("Unsupported mode")?],
            None,
        ),
        "/api/led/output" => {
            if let Some((_, backend)) = form.iter().find(|(key, _)| key == "backend") {
                if form.len() != 1 || !crate::led_backend::allowed(status, backend) {
                    return Err("LED backend unavailable or switching".into());
                }
                return Ok((0x5c, vec![3, crate::led_backend::code(backend).ok_or("Invalid LED backend")?], None));
            }
            let value = match form
                .iter()
                .find(|(k, _)| k == "enabled")
                .map(|(_, v)| v.as_str())
            {
                Some("true") => "on",
                Some("false") => "off",
                Some(_) => return Err("Invalid enabled value".into()),
                None => get("mode")?,
            };
            if form.len() != 1 {
                return Err("Choose enabled OR mode in one operation".into());
            }
            (
                0x5c,
                crate::ui::shell::led_output_request(value)
                    .ok_or("Invalid LED output")?
                    .1,
                None,
            )
        }
        _ => return Err("This API is not implemented on the selected USB route".into()),
    })
}
fn mutate(
    shared: &Shared,
    o: &Observation,
    path: &str,
    form: Vec<(String, String)>,
    headers: &BTreeMap<String, String>,
    body: &[u8],
    lifetime: Duration,
) -> Result<Value, String> {
    if shared.stop.load(Ordering::Acquire) || !o.route.current() {
        return Err("Sharing or selected owner retired; nothing sent".into());
    }
    require_pin(o, headers)?;
    if o.status.programming_latch
        || !o
            .status
            .capabilities
            .as_ref()
            .is_some_and(|c| c.allows(path, &form))
    {
        return Err("Capability unavailable or board programming active".into());
    }
    if path=="/api/led/diagnostic" && matches!(o.route,Route::Usb{..})
        && crate::pixel_groups::advertised(&o.status) {
        let (probe,off)=crate::pixel_diagnostic::Probe::from_form(&o.status,&form)
            .ok_or("Invalid or stale physical pixel group; nothing sent")?;
        if form.iter().any(|(key,_)| !matches!(key.as_str(),"index"|"count"|"rgb"|"off"|"group_layout_revision")) {
            return Err("Advanced diagnostics are not advertised by this USB bridge".into());
        }
        let (_,mut request)=crate::pixel_diagnostic::api_command(&o.status,probe,off)?;
        request["expires_at_ms"]=json!(clock(o)?+lifetime.min(Duration::from_secs(8)).as_millis() as u64);
        return pixels::diagnostic(shared,o,&request,headers,lifetime);
    }
    let forwarding = trace(
        headers.get("x-pakar-route").map(String::as_str),
        &shared.settings.instance_id,
    )?;
    match &o.route {
        Route::Usb {
            port,
            admission,
            sender,
        } => {
            let (op, payload, batch) = usb_request(path, &form, &o.status)?;
            let (tx, rx) = mpsc::channel();
            let mut admission = admission.clone();
            admission.at =
                Instant::now() - (Duration::from_secs(10) - lifetime.min(Duration::from_secs(8)));
            admission.batch = batch;
            sender
                .send(SerialCommand::Bridge(
                    port.clone(),
                    admission,
                    op,
                    payload,
                    Reply::new(tx, shared.stop.clone()),
                ))
                .map_err(|_| "USB owner closed")?;
            let status = rx
                .recv_timeout(Duration::from_secs(8))
                .map_err(|_| "USB outcome uncertain; not replayed")??;
            if shared.stop.load(Ordering::Acquire)
                || !o.route.current()
                || peer_http::Pin::from_status(&status) != peer_http::Pin::from_status(&o.status)
            {
                return Err("Peer retired after dispatch; no replay".into());
            }
            let next = Observation {
                status,
                mapping: o.mapping.clone(),
                route: o.route.clone(),
                at: o.at,
            };
            let count = if path == "/api/cells/assignments" {
                form.iter()
                    .find(|(k, _)| k == "assignments")
                    .and_then(|(_, v)| {
                        serde_json::from_str::<Vec<crate::cell_assignments::Assignment>>(v).ok()
                    })
                    .map(|v| v.len())
            } else {
                form.iter()
                    .find(|(k, _)| k == "ids")
                    .and_then(|(_, v)| crate::ui::cell_search::parse(v).ok())
                    .map(|s| s.len())
            };
            let assignments = if matches!(path, "/api/cells/assignments" | "/api/cells/lock") {
                crate::cell_assignments::Document::from_status(&next.status)
                    .ok()
                    .map(|d| d.assignments)
            } else {
                None
            };
            let result = json!({"accepted":true,"count":count,"assignments":assignments,"red_lock":next.status.red_lock,
                "avr_acknowledged":matches!(path,"/api/game/reset"|"/api/game/press"),"status":status_value(shared,&next)});
            *shared.observation.lock().unwrap() = Some(next);
            Ok(result)
        }
        Route::Network {
            endpoint,
            stamp,
            gate,
        } => {
            let _lease = gate
                .acquire(Transport::WiFi, stamp.epoch)
                .ok_or("Network owner revoked")?;
            let client = peer_http::Pin::from_status(&o.status)
                .ok_or("Peer unavailable")?
                .client()?;
            let mut request = client
                .post(format!("{}{}", endpoint.base.trim_end_matches('/'), path))
                .timeout(lifetime.min(Duration::from_secs(4)))
                .basic_auth(&endpoint.user, Some(&endpoint.password))
                .header("X-Pakar-Route", forwarding);
            request = if path == "/api/command" {
                request
                    .header("Content-Type", "application/json")
                    .body(body.to_vec())
            } else {
                request.form(&form)
            };
            if !stamp.current() || shared.stop.load(Ordering::Acquire) {
                return Err("Network owner revoked".into());
            }
            let response = request
                .send()
                .map_err(|e| format!("Network outcome uncertain; not replayed: {e}"))?;
            if !stamp.current() || shared.stop.load(Ordering::Acquire) {
                return Err("Network owner changed after dispatch; not replayed".into());
            }
            if !response.status().is_success() {
                return Err(format!("Upstream rejected command ({})", response.status()));
            }
            let mut result: Value = response
                .json()
                .map_err(|_| "Upstream result unavailable; not replayed".to_string())?;
            if result["accepted"] != true {
                return Err("Upstream did not acknowledge command".into());
            }
            let status = crate::blocking_status(&client, endpoint)?;
            if shared.stop.load(Ordering::Acquire)
                || !stamp.current()
                || peer_http::Pin::from_status(&status) != peer_http::Pin::from_status(&o.status)
            {
                return Err("Upstream peer changed after dispatch; no replay".into());
            }
            let status = if path == "/api/led/output"
                && let Some((_, backend)) = form.iter().find(|(key, _)| key == "backend") {
                crate::led_backend::finish_network(endpoint, status, backend, || {
                    if !stamp.current() || shared.stop.load(Ordering::Acquire) {
                        Err("Bridge owner retired; nothing replayed".into())
                    } else { Ok(()) }
                })?
            } else { status };
            let next = Observation {
                status,
                mapping: o.mapping.clone(),
                route: o.route.clone(),
                at: Instant::now(),
            };
            if path == "/api/cells/assignments" {
                let raw = form
                    .iter()
                    .find(|(k, _)| k == "assignments")
                    .ok_or("Missing assignments")?;
                let doc = crate::cell_assignments::Document::new(
                    serde_json::from_str(&raw.1).map_err(|_| "Invalid assignments")?,
                )?;
                if !doc.verify(&next.status) {
                    return Err("Assignment ACK differs from board readback; no replay".into());
                }
            } else if path == "/api/cells/lock" {
                let enabled = form
                    .iter()
                    .find(|(k, _)| k == "enabled")
                    .ok_or("Missing lock")?
                    .1
                    == "true";
                if next.status.red_lock != Some(enabled) {
                    return Err("Lock ACK differs from board readback; no replay".into());
                }
            }
            result["status"] = status_value(shared, &next);
            *shared.observation.lock().unwrap() = Some(next);
            Ok(result)
        }
    }
}
fn command(shared: &Shared, request: Value, headers: &BTreeMap<String, String>) -> Value {
    let id = request["request_id"].as_str().unwrap_or("").to_string();
    let mut owns_receipt = false;
    let mut reply = json!({"type":"command-result","request_id":id,"board_id":request["board_id"],"boot_id":request["boot_id"],"accepted":false});
    let result = (|| -> Result<Value, String> {
        let o = route(shared)?;
        if id.is_empty()
            || id.len() > 128
            || !id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
            || request["type"] != "command"
            || request["board_id"] != o.status.board_id
            || request["boot_id"] != o.status.boot_id
        {
            return Err("Invalid command identity".into());
        }
        let now = clock(&o)?;
        let deadline = request["expires_at_ms"]
            .as_u64()
            .ok_or("Command deadline required")?;
        {
            let mut receipts = shared.receipts.lock().unwrap();
            receipts.retain(|_, (at, _, _)| at.elapsed() < Duration::from_secs(60));
            if let Some((_, previous, value)) = receipts.get(&id) {
                return if previous == &request {
                    value
                        .clone()
                        .ok_or("Command already dispatched; outcome pending, not replayed".into())
                } else {
                    Err("Request ID reused for another intent".into())
                };
            }
            if deadline <= now || deadline > now + 8000 {
                return Err("Command deadline expired or invalid".into());
            }
            if receipts.len() >= 1024 {
                return Err("Command receipt budget full".into());
            }
            receipts.insert(id.clone(), (Instant::now(), request.clone(), None));
            owns_receipt = true;
        }
        let path = match request["command"].as_str() {
            Some("reset") => "/api/game/reset",
            Some("game-settings") => "/api/game/settings",
            Some("game-press") => "/api/game/press",
            Some("cell") => "/api/cell",
            Some("cells") => "/api/cells",
            Some("cell-assignments") => "/api/cells/assignments",
            Some("cell-lock") => "/api/cells/lock",
            Some("effect") => "/api/effect",
            Some("mode") => "/api/mode",
            Some("brightness") => "/api/brightness",
            Some("led-output") => "/api/led/output",
            Some("pixel-diagnostic") => "/api/led/diagnostic",
            _ => return Err("Unsupported command on bridge".into()),
        };
        let mut form = Vec::new();
        for key in [
            "key",
            "presentation",
            "id",
            "ids",
            "state",
            "effect",
            "value",
            "enabled",
            "mode",
            "assignments",
            "index",
            "count",
            "rgb",
            "off",
            "group_layout_revision",
            "enabled_mask",
            "idle_rgb",
            "win_rgb",
            "lose_rgb",
            "extended_effects",
            "auto_reset_ms",
            "chase_period_ms",
            "chase_neighbors",
            "chase_invert",
            "winner_cue",
            "reset_cue",
            "backend",
        ] {
            if let Some(v) = request.get(key) {
                form.push((
                    key.into(),
                    if let Some(s) = v.as_str() {
                        s.into()
                    } else {
                        v.to_string()
                    },
                ));
            }
        }
        // Preserve the original intent across every network hop. Unwrapping it
        // into a raw mutation loses the receiver's same-ID deduplication.
        if let Route::Network {
            endpoint,
            stamp,
            gate,
        } = &o.route
        {
            if o.status.programming_latch
                || !o
                    .status
                    .capabilities
                    .as_ref()
                    .is_some_and(|c| c.allows(path, &form))
            {
                return Err("Capability unavailable or board programming active".into());
            }
            let _lease = gate
                .acquire(Transport::WiFi, stamp.epoch)
                .ok_or("Network owner revoked")?;
            let pin = peer_http::Pin::from_status(&o.status).ok_or("Peer unavailable")?;
            let response = pin
                .client()?
                .post(format!(
                    "{}/api/command",
                    endpoint.base.trim_end_matches('/')
                ))
                .timeout(Duration::from_millis(deadline - now).min(Duration::from_secs(4)))
                .basic_auth(&endpoint.user, Some(&endpoint.password))
                .header(
                    "X-Pakar-Route",
                    trace(
                        headers.get("x-pakar-route").map(String::as_str),
                        &shared.settings.instance_id,
                    )?,
                )
                .form(&[("packet", request.to_string())])
                .send()
                .map_err(|_| "Network outcome uncertain; not replayed")?;
            if !stamp.current() || shared.stop.load(Ordering::Acquire) {
                return Err("Network owner retired; not replayed".into());
            }
            if !response.status().is_success() {
                return Err(format!("Upstream rejected command ({})", response.status()));
            }
            if response.content_length().is_some_and(|n| n > 65536) {
                return Err("Upstream receipt exceeds limit; not replayed".into());
            }
            let mut bytes = Vec::new();
            response
                .take(65537)
                .read_to_end(&mut bytes)
                .map_err(|_| "Upstream receipt unavailable; not replayed")?;
            if bytes.len() > 65536 {
                return Err("Upstream receipt exceeds limit; not replayed".into());
            }
            let mut result: Value = serde_json::from_slice(&bytes)
                .map_err(|_| "Invalid upstream receipt; not replayed")?;
            if !stamp.current() || shared.stop.load(Ordering::Acquire) {
                return Err("Network owner retired while receiving receipt; not replayed".into());
            }
            if result["type"] != "command-result"
                || result["request_id"] != id
                || result["board_id"] != o.status.board_id
                || result["boot_id"] != o.status.boot_id
            {
                return Err("Foreign upstream receipt; not replayed".into());
            }
            // Actual PCController decodePeerResponse rejects ambiguous ACKs
            // while tolerating unrelated future metadata. Apply that rule to
            // this living command-result shape, not its JSON-RPC selector.
            let accepted = result["accepted"]
                .as_bool()
                .ok_or("Ambiguous upstream receipt: accepted boolean required; not replayed")?;
            if accepted && result.get("error").is_some()
                || !accepted
                    && !result["error"]
                        .as_str()
                        .is_some_and(|error| !error.trim().is_empty())
            {
                return Err("Ambiguous upstream result/error; not replayed".into());
            }
            if accepted {
                let mut status: DeviceStatus = serde_json::from_value(result["status"].clone())
                    .map_err(|_| "Upstream receipt has no readback")?;
                if peer_http::Pin::from_status(&status).as_ref() != Some(&pin) {
                    return Err("Upstream readback peer changed; not replayed".into());
                }
                if status.compatibility.is_some() {
                    crate::peer_identity::validate_board(&status)?;
                }
                if request["command"] == "led-output"
                    && let Some(backend) = request["backend"].as_str() {
                    status = crate::led_backend::finish_network(endpoint, status, backend, || {
                        if !stamp.current() || shared.stop.load(Ordering::Acquire) {
                            Err("Bridge owner retired; nothing replayed".into())
                        } else { Ok(()) }
                    })?;
                }
                if !command_readback(&request, &result, &status)? {
                    return Err("Upstream ACK differs from command readback; not replayed".into());
                }
                let next = Observation {
                    status,
                    mapping: o.mapping.clone(),
                    route: o.route.clone(),
                    at: Instant::now(),
                };
                result["status"] = status_value(shared, &next);
                *shared.observation.lock().unwrap() = Some(next);
            }
            return Ok(result);
        }
        let mut pin = headers.clone();
        pin.insert("x-pakar-board".into(), o.status.board_id.clone());
        pin.insert("x-pakar-boot".into(), o.status.boot_id.clone());
        if path=="/api/led/diagnostic" && crate::pixel_groups::advertised(&o.status) {
            return pixels::diagnostic(shared,&o,&request,&pin,Duration::from_millis(deadline-now));
        }
        let mut result = mutate(
            shared,
            &o,
            path,
            form,
            &pin,
            &[],
            Duration::from_millis(deadline - now),
        )?;
        result["type"] = json!("command-result");
        result["request_id"] = json!(id);
        result["board_id"] = json!(o.status.board_id);
        result["boot_id"] = json!(o.status.boot_id);
        Ok(result)
    })();
    match result {
        Ok(value) => reply = value,
        Err(error) => reply["error"] = json!(error),
    }
    if owns_receipt {
        if let Some(entry) = shared.receipts.lock().unwrap().get_mut(&id) {
            if entry.1 == request {
                entry.2 = Some(reply.clone());
            }
        }
        // Other observers receive the outcome, not another command execution.
        // The complete authoritative readback remains in the private receipt.
        publish_event(
            shared,
            json!({"type":"command-observed","request_id":id,"board_id":reply["board_id"],"boot_id":reply["boot_id"],"accepted":reply["accepted"],"error":reply["error"],"command":request["command"]}),
        );
    }
    reply
}
fn command_readback(request: &Value, reply: &Value, status: &DeviceStatus) -> Result<bool, String> {
    let text = |key: &str| request[key].as_str().unwrap_or("");
    Ok(match text("command") {
        "game-press" => reply["avr_acknowledged"] == true,
        "reset" => {
            reply["avr_acknowledged"] == true
                && status.winner == 0
                && status.cells.iter().all(|cell| cell == "white")
        }
        "game-settings" => {
            let mut selected = Vec::new();
            for key in [
                "presentation",
                "enabled_mask",
                "idle_rgb",
                "win_rgb",
                "lose_rgb",
                "extended_effects",
                "auto_reset_ms",
                "chase_period_ms",
                "chase_neighbors",
                "chase_invert",
                "winner_cue",
                "reset_cue",
            ] {
                if let Some(value) = request.get(key) {
                    selected.push((
                        key.to_owned(),
                        value
                            .as_str()
                            .map(str::to_owned)
                            .unwrap_or_else(|| value.to_string()),
                    ));
                }
            }
            crate::usb_table::key_settings_network_verify(status, &selected)
        }
        "cell" => request["id"]
            .as_u64()
            .filter(|v| (1..=80).contains(v))
            .is_some_and(|id| {
                status
                    .cells
                    .get(id as usize - 1)
                    .is_some_and(|s| s == text("state"))
            }),
        "cells" => {
            let batch = crate::cell_batch::Request::new(
                &crate::ui::cell_search::parse(text("ids"))?,
                text("state"),
                request["effect"].as_str().unwrap_or("immediate"),
            )?;
            reply["count"].as_u64() == Some(batch.ids.len() as u64) && batch.verify(status)
        }
        "cell-assignments" => {
            let doc = crate::cell_assignments::Document::new(
                serde_json::from_value(request["assignments"].clone())
                    .map_err(|_| "Invalid assignments")?,
            )?;
            reply["count"].as_u64() == Some(doc.assignments.len() as u64) && doc.verify(status)
        }
        "cell-lock" => request["enabled"]
            .as_bool()
            .is_some_and(|v| status.red_lock == Some(v)),
        "mode" => status.mode == text("value"),
        "effect" => status.effect == text("value"),
        "brightness" => request["value"].as_u64() == Some(u64::from(status.brightness)),
        "led-output" => {
            (!request.get("enabled").is_some()
                || request["enabled"]
                    .as_bool()
                    .is_some_and(|v| status.led_output["enabled"] == v))
                && (!request.get("mode").is_some() || status.led_output["mode"] == request["mode"])
                && (request.get("backend").is_none() || request["backend"].as_str()
                    .is_some_and(|backend| crate::led_backend::confirmed(status, backend)))
        }
        "pixel-diagnostic" => {
            pixels::diagnostic_matches(request,status)?
        }
        _ => false,
    })
}
struct Request {
    method: String,
    path: String,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}
fn read_request(socket: &mut TcpStream) -> Result<Request, String> {
    socket
        .set_read_timeout(Some(Duration::from_millis(250)))
        .map_err(|e| e.to_string())?;
    let mut deadline = Instant::now() + Duration::from_secs(4);
    let mut bytes = Vec::new();
    loop {
        let mut headers = [httparse::EMPTY_HEADER; 40];
        if Instant::now() >= deadline || bytes.len() > 16384 {
            return Err("HTTP header limit/deadline".into());
        }
        let mut parser = httparse::Request::new(&mut headers);
        if let httparse::Status::Complete(offset) =
            parser.parse(&bytes).map_err(|e| e.to_string())?
        {
            let mut map = BTreeMap::new();
            for header in parser.headers.iter() {
                let key = header.name.to_ascii_lowercase();
                if map
                    .insert(
                        key,
                        std::str::from_utf8(header.value)
                            .map_err(|_| "Invalid header")?
                            .to_owned(),
                    )
                    .is_some()
                {
                    return Err("Duplicate HTTP header".into());
                }
            }
            if map.contains_key("transfer-encoding") || map.contains_key("origin") {
                return Err("Chunked bodies and browser-origin commands unavailable".into());
            }
            let length = map
                .get("content-length")
                .map(|v| v.parse::<usize>())
                .transpose()
                .map_err(|_| "Invalid body length")?
                .unwrap_or(0);
            let upload = parser.method == Some("POST")
                && matches!(parser.path, Some("/update" | "/api/avr/image"));
            let limit = if upload {
                transfer::MAX_IMAGE_BODY
            } else {
                8192
            };
            if length > limit {
                return Err("Request body exceeds limit".into());
            }
            if upload {
                deadline = Instant::now() + Duration::from_secs(60);
            }
            let method = parser.method.ok_or("Missing HTTP method")?.to_string();
            let path = parser.path.ok_or("Missing HTTP path")?.to_string();
            while bytes.len() < offset + length {
                if Instant::now() >= deadline {
                    return Err("Body deadline".into());
                }
                let mut chunk = [0; 2048];
                match socket.read(&mut chunk) {
                    Ok(0) => return Err("Truncated body".into()),
                    Ok(n) => bytes.extend_from_slice(&chunk[..n]),
                    Err(e)
                        if matches!(
                            e.kind(),
                            std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                        ) => {}
                    Err(e) => return Err(e.to_string()),
                }
            }
            return Ok(Request {
                method,
                path,
                headers: map,
                body: bytes[offset..offset + length].to_vec(),
            });
        }
        let mut chunk = [0; 2048];
        match socket.read(&mut chunk) {
            Ok(0) => return Err("Closed request".into()),
            Ok(n) => bytes.extend_from_slice(&chunk[..n]),
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                ) => {}
            Err(e) => return Err(e.to_string()),
        }
    }
}
fn write_response(
    socket: &mut TcpStream,
    code: u16,
    kind: &str,
    body: &[u8],
) -> Result<(), String> {
    socket
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| e.to_string())?;
    write!(socket,"HTTP/1.1 {code} Result\r\n{}Content-Type: {kind}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",crate::peer_identity::server_headers(),body.len()).map_err(|e|e.to_string())?;
    socket.write_all(body).map_err(|e| e.to_string())
}
fn remember_peer(shared: &Shared, value: Value) {
    let Some(id) = value["id"].as_str().map(str::to_owned) else {
        return;
    };
    let mut peers = shared.peers.lock().unwrap();
    peers.retain(|_, (at, _)| at.elapsed() < Duration::from_secs(120));
    if peers.contains_key(&id) || peers.len() < 32 {
        peers.insert(id, (Instant::now(), value));
    }
}
fn serve(mut socket: TcpStream, shared: &Arc<Shared>) -> Result<(), String> {
    // Peek leaves the complete handshake for tungstenite's reviewed parser.
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| e.to_string())?;
    socket
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| e.to_string())?;
    let mut peek = [0; 16384];
    let started = Instant::now();
    let count = loop {
        let count = socket.peek(&mut peek).map_err(|e| e.to_string())?;
        if count == 0 {
            return Ok(());
        }
        if peek[..count].windows(4).any(|v| v == b"\r\n\r\n") {
            break count;
        }
        if count == peek.len() || started.elapsed() > Duration::from_secs(3) {
            return Err("Incomplete/oversized HTTP header".into());
        }
        thread::sleep(Duration::from_millis(2));
    };
    if std::str::from_utf8(&peek[..count])
        .is_ok_and(|s| s.to_ascii_lowercase().contains("upgrade: websocket"))
    {
        if !shared.settings.enabled{return write_response(&mut socket,403,"application/json",b"{\"error\":\"bridge_disabled\"}");}
        let mut limits = tungstenite::protocol::WebSocketConfig::default();
        limits.max_message_size = Some(8192);
        limits.max_frame_size = Some(8192);
        let observed_ip = socket
            .peer_addr()
            .map(|a| a.ip().to_string())
            .unwrap_or_default();
        let mut ws_peer = None;
        let mut ws_forwarding = BTreeMap::new();
        let mut ws = tungstenite::accept_hdr_with_config(
            socket,
            |r: &tungstenite::handshake::server::Request,
             mut response: tungstenite::handshake::server::Response| {
                let compatible = crate::peer_identity::validate_client(
                    r.headers()
                        .get("x-pakar-version")
                        .and_then(|v| v.to_str().ok()),
                );
                let allowed = r.uri().path() == "/ws"
                    && !r.headers().contains_key("origin")
                    && trace(
                        r.headers()
                            .get("x-pakar-route")
                            .and_then(|v| v.to_str().ok()),
                        &shared.settings.instance_id,
                    )
                    .is_ok()
                    && compatible.is_ok();
                if allowed {
                    if let Some(value) = r
                        .headers()
                        .get("x-pakar-route")
                        .and_then(|v| v.to_str().ok())
                    {
                        ws_forwarding.insert("x-pakar-route".to_owned(), value.to_owned());
                    }
                    ws_peer = Some((
                        r.headers()
                            .get("x-pakar-client-id")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or_default()
                            .to_owned(),
                        r.headers()
                            .get("x-pakar-version")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or_default()
                            .to_owned(),
                    ));
                    response.headers_mut().insert(
                        "server",
                        format!("PakarController/{}", crate::peer_identity::VERSION)
                            .parse()
                            .unwrap(),
                    );
                    response.headers_mut().insert(
                        "x-pakar-version",
                        crate::peer_identity::VERSION.parse().unwrap(),
                    );
                    response.headers_mut().insert(
                        "x-pakar-build",
                        crate::peer_identity::build_identity().parse().unwrap(),
                    );
                    response.headers_mut().insert(
                        "x-pakar-client-id",
                        crate::peer_identity::session_id().parse().unwrap(),
                    );
                    Ok(response)
                } else {
                    Err(tungstenite::http::Response::builder()
                        .status(if compatible.is_err() { 426 } else { 403 })
                        .header(
                            "Server",
                            format!("PakarController/{}", crate::peer_identity::VERSION),
                        )
                        .header("X-Pakar-Version", crate::peer_identity::VERSION)
                        .header("X-Pakar-Build", crate::peer_identity::build_identity())
                        .body(Some(compatible.err().unwrap_or_else(|| {
                            "Bridge request rejected: check origin and route".into()
                        })))
                        .unwrap())
                }
            },
            Some(limits),
        )
        .map_err(|e| e.to_string())?;
        ws.get_mut()
            .set_read_timeout(Some(Duration::from_millis(40)))
            .map_err(|e| e.to_string())?;
        let mut previous = Value::Null;
        let mut pushed = Instant::now() - Duration::from_secs(3);
        let mut cursor = shared.events.lock().unwrap().cursor();
        let (results_tx, results_rx) = mpsc::sync_channel(1);
        let mut pending = false;
        let mut pixels = false;
        while !shared.stop.load(Ordering::Acquire) {
            shared.demand.touch(crate::usb_demand::Demand {
                flags: crate::usb_demand::STATUS
                    | crate::usb_demand::PANEL
                    | crate::usb_demand::PERIPHERAL
                    | if pixels { crate::usb_demand::PIXELS } else { 0 },
                table: true,
            });
            if let Ok(result) = results_rx.try_recv() {
                pending = false;
                ws.send(tungstenite::Message::Text(Value::to_string(&result).into()))
                    .map_err(|e| e.to_string())?;
            }
            if let Ok(o) = route(shared) {
                let journal = shared.events.lock().unwrap().read(
                    Some(&cursor),
                    &o.status.board_id,
                    &o.status.boot_id,
                )?;
                cursor = journal["cursor"].as_str().unwrap().to_owned();
                if journal["gap"] == true {
                    ws.send(tungstenite::Message::Text(
                        json!({"type":"event-gap","cursor":cursor})
                            .to_string()
                            .into(),
                    ))
                    .map_err(|e| e.to_string())?;
                }
                for entry in journal["events"].as_array().unwrap() {
                    let mut event = entry["event"].clone();
                    event["bridge_cursor"] = entry["cursor"].clone();
                    ws.send(tungstenite::Message::Text(event.to_string().into()))
                        .map_err(|e| e.to_string())?;
                }
            }
            let value=route(shared).map(|o|status_value(shared,&o)).unwrap_or_else(|_|json!({"type":"bridge-offline","bridge":{"instance_id":shared.settings.instance_id}}));
            let mut comparison = value.clone();
            comparison["command_clock_ms"] = Value::Null;
            if comparison != previous || pushed.elapsed() >= Duration::from_secs(2) {
                ws.send(tungstenite::Message::Text(value.to_string().into()))
                    .map_err(|e| e.to_string())?;
                previous = comparison;
                pushed = Instant::now();
            }
            match ws.read() {
                Ok(tungstenite::Message::Text(text)) if text.len() <= 8192 => {
                    let request: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
                    if request["type"] == "peer-introduction" {
                        let accepted = ws_peer
                            .as_ref()
                            .ok_or("Missing peer identity".to_owned())
                            .and_then(|(id, version)| {
                                crate::peer_identity::sanitize_report(
                                    &request["report"],
                                    id,
                                    version,
                                    &observed_ip,
                                )
                            });
                        let response = match accepted {
                            Ok(value) => {
                                remember_peer(shared, value);
                                json!({"type":"peer-introduction","accepted":true,"version":crate::peer_identity::VERSION,"compatibility":crate::peer_identity::advertised_policy()})
                            }
                            Err(error) => {
                                json!({"type":"peer-introduction","accepted":false,"error":error})
                            }
                        };
                        ws.send(tungstenite::Message::Text(response.to_string().into()))
                            .map_err(|e| e.to_string())?;
                    } else if request["type"] == "command" {
                        if pending
                            || shared
                                .active_commands
                                .fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
                                    (n < 16).then_some(n + 1)
                                })
                                .is_err()
                        {
                            ws.send(tungstenite::Message::Text(json!({"type":"command-result","request_id":request["request_id"],"board_id":request["board_id"],"boot_id":request["boot_id"],"accepted":false,"error":"Bridge command busy; nothing sent"}).to_string().into())).map_err(|e|e.to_string())?;
                        } else {
                            pending = true;
                            let owner = shared.clone();
                            let tx = results_tx.clone();
                            let forwarding = ws_forwarding.clone();
                            thread::spawn(move || {
                                let result = command(&owner, request, &forwarding);
                                owner.active_commands.fetch_sub(1, Ordering::AcqRel);
                                let _ = tx.try_send(result);
                            });
                        }
                    } else if request["type"] == "pixel-subscribe" {
                        pixels = request["enabled"].as_bool().ok_or("Invalid pixel subscription")?;
                        previous = Value::Null;
                    } else if request["type"] == "status" {
                        if let Some(value) = request["pixels"].as_bool() {
                            pixels = value;
                        }
                        previous = Value::Null;
                    }
                }
                Ok(tungstenite::Message::Close(_)) => break,
                Err(tungstenite::Error::Io(e))
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) => {}
                Err(_) => break,
                _ => {}
            }
        }
        return Ok(());
    }
    let request = match read_request(&mut socket) {
        Ok(r) => r,
        Err(e) => {
            return write_response(
                &mut socket,
                400,
                "application/json",
                json!({"error":e}).to_string().as_bytes(),
            );
        }
    };
    let path = request.path.split('?').next().unwrap_or("");
    if shared.settings.enabled && request.method == "GET"
        && matches!(path, "/api/status" | "/api/cells/mapping" | "/api/events")
    {
        let pixels = request.path.split_once('?').is_some_and(|(_, query)| {
            query
                .split('&')
                .any(|v| matches!(v, "pixels=true" | "pixels=1"))
        });
        shared.demand.touch(crate::usb_demand::Demand {
            flags: crate::usb_demand::STATUS | if pixels { crate::usb_demand::PIXELS } else { 0 },
            table: true,
        });
    }
    // Public identity/status bootstrap stays discoverable before admission;
    // all control/config/export paths require a supported app release.
    let bootstrap = request.method == "GET"
        && matches!(
            path,
            "/description.xml"
                | "/upnp/device.xml"
                | "/api/discovery"
                | "/upnp/public.json"
                | "/healthz"
                | "/api/status"
                | "/api/auth/info"
                | "/api/peer/info"
        );
    let peer_package=matches!(request.method.as_str(),"GET"|"HEAD")&&path==crate::peer_package::PATH&&shared.settings.peer_package_sharing;
    if !shared.settings.enabled&&!peer_package&&!(request.method=="GET"&&matches!(path,"/api/peer/info"|"/description.xml"|"/upnp/device.xml"|"/api/discovery"|"/upnp/public.json"|"/healthz"|"/api/auth/info")) {
        return write_response(&mut socket,403,"application/json",b"{\"error\":\"bridge_disabled\"}");
    }
    if !peer_package&&(!bootstrap || (path!="/api/peer/info"&&request.headers.contains_key("x-pakar-version")))
        && let Err(error) = crate::peer_identity::validate_client(
            request.headers.get("x-pakar-version").map(String::as_str),
        )
    {
        return write_response(&mut socket,426,"application/json",json!({"accepted":false,"error":error,"compatibility":crate::peer_identity::advertised_policy()}).to_string().as_bytes());
    }
    if let Some(id) = request
        .headers
        .get("x-pakar-client-id")
        .filter(|v| uuid::Uuid::parse_str(v).is_ok())
    {
        let mut peers = shared.peers.lock().unwrap();
        peers.retain(|_, (at, _)| at.elapsed() < Duration::from_secs(120));
        if peers.contains_key(id) || peers.len() < 32 {
            let mut value = peers
                .get(id)
                .map(|(_, v)| v.clone())
                .unwrap_or_else(|| json!({"id":id}));
            value["version"] = request
                .headers
                .get("x-pakar-version")
                .cloned()
                .unwrap_or_default()
                .into();
            if let Some(agent) = request
                .headers
                .get("user-agent")
                .filter(|v| v.len() <= 192 && !v.chars().any(char::is_control))
            {
                value["user_agent"] = agent.clone().into();
            }
            if let Ok(address) = socket.peer_addr() {
                value["observed_ip"] = address.ip().to_string().into();
            }
            peers.insert(id.clone(), (Instant::now(), value));
        }
    }
    if trace(
        request.headers.get("x-pakar-route").map(String::as_str),
        &shared.settings.instance_id,
    )
    .is_err()
    {
        return write_response(
            &mut socket,
            508,
            "application/json",
            b"{\"error\":\"Bridge loop or hop limit\"}",
        );
    }
    if request.method == "GET" && path == "/api/auth/info" {
        // This describes this listener, not an upstream board's login.
        return write_response(
            &mut socket,
            200,
            "application/json",
            br#"{"enabled":false,"ready":true,"username":"","default_credentials":false}"#,
        );
    }
    if request.method == "GET" && path == "/api/peer/info" {
        let mut value = crate::peer_catalogue::public_info(&shared.settings.instance_id,&shared.settings.hostname,shared.settings.enabled);
        value["build"]=crate::client_management::build().into();
        value["exchange"]["native_package_state"]=if shared.settings.peer_package_sharing{crate::peer_package::state()}else{"disabled"}.into();
        if shared.settings.peer_package_sharing&&let Some(artifact)=crate::peer_package::artifact(){
            value["artifacts"]=json!([artifact]);value["exchange"]["native_package_sharing"]=true.into();
        }
        return write_response(
            &mut socket,
            200,
            "application/json",
            value.to_string().as_bytes(),
        );
    }
    if peer_package {
        if crate::peer_package::artifact().is_none(){return write_response(&mut socket,503,"application/json",b"{\"error\":\"peer_package_unavailable\"}");}
        return crate::peer_package::serve(&mut socket,&shared.settings.instance_id,request.method=="HEAD");
    }
    if path==crate::peer_package::PATH{return write_response(&mut socket,403,"application/json",b"{\"error\":\"peer_package_sharing_disabled\"}");}
    if request.method == "GET" && path == "/api/clients" {
        let mut peers = shared.peers.lock().unwrap();
        peers.retain(|_, (at, _)| at.elapsed() < Duration::from_secs(120));
        let value = json!({"clients":peers.values().map(|(at,v)|{let mut v=v.clone();v["age_ms"]=(at.elapsed().as_millis() as u64).into();v}).collect::<Vec<_>>()});
        return write_response(
            &mut socket,
            200,
            "application/json",
            value.to_string().as_bytes(),
        );
    }
    if request.method == "POST" && path == "/api/clients/report" {
        let body = std::str::from_utf8(&request.body).map_err(|_| "Invalid report form")?;
        let parsed = reqwest::Url::parse(&format!("http://bridge/?{body}"))
            .map_err(|_| "Invalid report form")?;
        let fields = parsed.query_pairs().collect::<Vec<_>>();
        let report = if fields.len() == 1 && fields[0].0 == "report" {
            serde_json::from_str::<Value>(&fields[0].1).map_err(|_| "Invalid peer report")?
        } else {
            return write_response(
                &mut socket,
                400,
                "application/json",
                br#"{"accepted":false,"error":"Expected one report field"}"#,
            );
        };
        let ip = socket
            .peer_addr()
            .map(|a| a.ip().to_string())
            .unwrap_or_default();
        let result = crate::peer_identity::sanitize_report(
            &report,
            request
                .headers
                .get("x-pakar-client-id")
                .map_or("", String::as_str),
            request
                .headers
                .get("x-pakar-version")
                .map_or("", String::as_str),
            &ip,
        );
        return match result {
            Ok(value) => {
                remember_peer(shared, value);
                write_response(
                    &mut socket,
                    200,
                    "application/json",
                    br#"{"accepted":true}"#,
                )
            }
            Err(error) => write_response(
                &mut socket,
                400,
                "application/json",
                json!({"accepted":false,"error":error})
                    .to_string()
                    .as_bytes(),
            ),
        };
    }
    if request.method == "GET" && matches!(path, "/description.xml" | "/upnp/device.xml") {
        let xml = format!(
            "<root><device><deviceType>urn:schemas-upnp-org:device:Basic:1</deviceType><friendlyName>{}</friendlyName><manufacturer>AtomicDeploy</manufacturer><modelName>PakarController</modelName><modelNumber>{}</modelNumber><modelDescription>Controller bridge</modelDescription><modelURL>https://github.com/atomicdeploy/PakarController</modelURL><UDN>uuid:{}</UDN></device></root>",
            shared.settings.hostname,
            crate::peer_identity::VERSION,
            shared.settings.instance_id
        );
        return write_response(&mut socket, 200, "text/xml", xml.as_bytes());
    }
    if request.method == "GET"
        && matches!(path, "/api/discovery" | "/upnp/public.json" | "/healthz")
    {
        let public = json!({"product":"PakarController","version":crate::peer_identity::VERSION,"compatibility":crate::peer_identity::advertised_policy(),"status":"ok","kind":"bridge","instance_id":shared.settings.instance_id,"hostname":shared.settings.hostname,"api":"/api","ws":"/ws","auth_required":false,"connected":route(shared).is_ok()});
        return write_response(
            &mut socket,
            200,
            "application/json",
            public.to_string().as_bytes(),
        );
    }
    if transfer::handles(&request) {
        return match transfer::open(shared, &request) {
            Ok(response) => response.write(&mut socket),
            Err(e) => write_response(
                &mut socket,
                409,
                "application/json",
                json!({"accepted":false,"error":e,"replayed":false})
                    .to_string()
                    .as_bytes(),
            ),
        };
    }
    let result = (|| -> Result<Value, String> {
        let o = route(shared)?;
        if pixels::handles(&request.method,path) {return pixels::handle(shared,&o,&request);}
        match (request.method.as_str(), path) {
            ("GET", "/api/events") => {
                let query = reqwest::Url::parse(&format!("http://bridge{}", request.path))
                    .map_err(|_| "Invalid event query")?;
                let values: Vec<_> = query.query_pairs().collect();
                if values.len() > 1 || values.iter().any(|(key, _)| key != "after") {
                    return Err("Expected only after event cursor".into());
                }
                shared.events.lock().unwrap().read(
                    values.first().map(|(_, v)| v.as_ref()),
                    &o.status.board_id,
                    &o.status.boot_id,
                )
            }
            ("GET", "/api/status") => Ok(status_value(shared, &o)),
            ("GET", "/api/cells/assignments") => {
                let doc = crate::cell_assignments::Document::from_status(&o.status)?;
                Ok(
                    json!({"role":doc.role,"assignments":doc.assignments,"red_lock":o.status.red_lock,"board_id":o.status.board_id,"boot_id":o.status.boot_id}),
                )
            }
            ("GET", "/api/cells/mapping") => {
                if o.mapping.is_null() {
                    Err("Current peer mappings unavailable".into())
                } else {
                    Ok(o.mapping)
                }
            }
            ("GET", path) if path.starts_with("/api/") && !path.contains("..") => match &o.route {
                Route::Network {
                    endpoint,
                    stamp,
                    gate,
                } => {
                    let _lease = gate
                        .acquire(Transport::WiFi, stamp.epoch)
                        .ok_or("Owner retired")?;
                    let client = peer_http::Pin::from_status(&o.status)
                        .ok_or("Peer unavailable")?
                        .client()?;
                    let response = client
                        .get(format!(
                            "{}{}",
                            endpoint.base.trim_end_matches('/'),
                            request.path
                        ))
                        .basic_auth(&endpoint.user, Some(&endpoint.password))
                        .header(
                            "X-Pakar-Route",
                            trace(
                                request.headers.get("x-pakar-route").map(String::as_str),
                                &shared.settings.instance_id,
                            )?,
                        )
                        .send()
                        .map_err(|e| e.to_string())?;
                    if !stamp.current() || !response.status().is_success() {
                        return Err("Upstream read unavailable or peer retired".into());
                    }
                    response
                        .json()
                        .map_err(|_| "Upstream did not return JSON".into())
                }
                _ => Err("API unavailable on the selected USB route".into()),
            },
            ("POST", "/api/command") => {
                Ok(command(shared, command_packet(&request)?, &request.headers))
            }
            ("POST", path) if path.starts_with("/api/") => {
                if matches!(path, "/api/cells/assignments" | "/api/cells/lock")
                    && request
                        .headers
                        .get("content-type")
                        .is_some_and(|v| v.starts_with("application/json"))
                {
                    let value: Value = serde_json::from_slice(&request.body)
                        .map_err(|_| "Invalid assignment JSON")?;
                    let form = if path == "/api/cells/assignments" {
                        crate::cell_assignments::Document::new(
                            serde_json::from_value(value["assignments"].clone())
                                .map_err(|_| "Invalid assignments")?,
                        )?
                        .form()
                    } else {
                        vec![(
                            "enabled".into(),
                            value["enabled"]
                                .as_bool()
                                .ok_or("Lock requires enabled boolean")?
                                .to_string(),
                        )]
                    };
                    return mutate(
                        shared,
                        &o,
                        path,
                        form,
                        &request.headers,
                        &request.body,
                        Duration::from_secs(8),
                    );
                }
                let body =
                    std::str::from_utf8(&request.body).map_err(|_| "Expected URL-encoded form")?;
                let url = reqwest::Url::parse(&format!("http://bridge/?{body}"))
                    .map_err(|_| "Invalid form")?;
                let form: Vec<_> = url
                    .query_pairs()
                    .map(|(k, v)| (k.into_owned(), v.into_owned()))
                    .collect();
                if form.len() > 16
                    || form.iter().enumerate().any(|(i, (k, v))| {
                        v.len() > (if k == "assignments" { 4096 } else { 512 })
                            || form[..i].iter().any(|(key, _)| key == k)
                    })
                {
                    return Err("Invalid or duplicate form field".into());
                }
                mutate(
                    shared,
                    &o,
                    path,
                    form,
                    &request.headers,
                    &request.body,
                    Duration::from_secs(8),
                )
            }
            _ => Err("API not implemented on this route".into()),
        }
    })();
    let (code, value) = match result {
        Ok(v) => (200, v),
        Err(e) => (409, json!({"accepted":false,"error":e,"replayed":false})),
    };
    write_response(
        &mut socket,
        code,
        "application/json",
        value.to_string().as_bytes(),
    )
}

fn command_packet(request: &Request) -> Result<Value, String> {
    if request
        .headers
        .get("content-type")
        .is_some_and(|v| v.split(';').next() == Some("application/json"))
    {
        return serde_json::from_slice(&request.body).map_err(|_| "Invalid command JSON".into());
    }
    let body = std::str::from_utf8(&request.body).map_err(|_| "Invalid command form")?;
    let url = reqwest::Url::parse(&format!("http://bridge/?{body}"))
        .map_err(|_| "Invalid command form")?;
    let fields = url.query_pairs().collect::<Vec<_>>();
    if fields.len() != 1 || fields[0].0 != "packet" {
        return Err("Expected one packet field".into());
    }
    serde_json::from_str(&fields[0].1).map_err(|_| "Invalid command packet".into())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cell_assignments_bridge_http_json_export_lock_and_correlated_no_replay() {
        let (shared, rx, _gate) = fixture();
        {
            let mut owned = shared.observation.lock().unwrap();
            let status = &mut owned.as_mut().unwrap().status;
            status.red_lock = Some(false);
            let caps = status.capabilities.as_mut().unwrap();
            caps.role = "table-selection".into();
            caps.acknowledged_features
                .extend(["cell-assignments".into(), "cell-lock".into()]);
        }
        let before = route(&shared).unwrap().status;
        let mut actual = before.clone();
        let worker = thread::spawn(move || {
            for expected in [0x62, 0x63, 0x62] {
                match rx.recv_timeout(Duration::from_secs(4)).unwrap() {
                    SerialCommand::Bridge(_, admission, opcode, payload, reply) => {
                        assert_eq!(opcode, expected);
                        assert!(admission.current(1, Some(&actual)));
                        if opcode == 0x62 {
                            let doc =
                                crate::cell_assignments::Document::from_payload(&payload).unwrap();
                            for row in doc.assignments {
                                actual.cells[row.cell as usize - 1] = row.state;
                            }
                        } else {
                            actual.red_lock = Some(payload == [1]);
                        }
                        reply.send(Ok(actual.clone())).unwrap();
                    }
                    _ => panic!("Only selected USB owner may receive assignments"),
                }
            }
            assert!(
                rx.recv_timeout(Duration::from_millis(100)).is_err(),
                "No foreign-pin or duplicate command may reach USB"
            );
        });
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let owner = shared.clone();
        let server = thread::spawn(move || {
            for _ in 0..4 {
                let (socket, _) = listener.accept().unwrap();
                serve(socket, &owner).unwrap();
            }
        });
        let client = peer_http::Pin::from_status(&before)
            .unwrap()
            .client()
            .unwrap();
        let exported: Value = client
            .get(format!("{base}/api/cells/assignments"))
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(exported["assignments"].as_array().unwrap().len(), 80);
        assert_eq!(exported["red_lock"], false);
        let body = json!({"assignments":[{"cell":1,"state":"red","future_optional":true},{"cell":80,"state":"off"}]});
        assert_eq!(
            client
                .post(format!("{base}/api/cells/assignments"))
                .header("X-Pakar-Boot", "b".repeat(32))
                .json(&body)
                .send()
                .unwrap()
                .status(),
            409
        );
        let applied: Value = client
            .post(format!("{base}/api/cells/assignments"))
            .json(&body)
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(applied["accepted"], true);
        assert_eq!(applied["count"], 2);
        assert_eq!(applied["status"]["cells"][0], "red");
        assert_eq!(applied["status"]["cells"][79], "off");
        let locked: Value = client
            .post(format!("{base}/api/cells/lock"))
            .json(&json!({"enabled":true}))
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(locked["red_lock"], true);
        assert_eq!(locked["status"]["red_lock"], true);
        server.join().unwrap();
        let packet = json!({"type":"command","request_id":"assignment-one-intent","board_id":before.board_id,"boot_id":before.boot_id,
            "expires_at_ms":9000,"command":"cell-assignments","assignments":[{"cell":2,"state":"red"}]});
        let first = command(&shared, packet.clone(), &BTreeMap::new());
        assert_eq!(first["accepted"], true);
        assert_eq!(first["count"], 1);
        assert_eq!(command(&shared, packet, &BTreeMap::new()), first);
        worker.join().unwrap();
    }
    pub(super) fn fixture() -> (Arc<Shared>, mpsc::Receiver<SerialCommand>, connection::Gate) {
        let settings = Settings {
            enabled: true,
            hostname: "fixture-host".into(),
            listen: "127.0.0.1".into(),
            ..Default::default()
        };
        let gate = connection::Gate::default();
        let epoch = gate.select(Some(Transport::Serial));
        let (tx, rx) = mpsc::channel();
        let status:DeviceStatus=serde_json::from_value(json!({"version":crate::peer_identity::VERSION,"hostname":"ledboard","board_id":"fixture-board","boot_id":"a".repeat(32),
            "http_peer_fencing":true,"command_clock_ms":1000,"cells":vec!["white";80],"pixels":480,"mode":"cell",
            "capabilities":{"board_kind":"table","modes":[{"id":"cell","code":1,"kind":"presentation"}],"acknowledged_features":["table","leds","cell-batch","command-ack","ota"],"table":{"columns":10,"rows":8,"pixels_per_cell":6},
                "led_output":{"backend":"gpio-static","rmt":false,"dma":false}}})).unwrap();
        let admission = usb_table::Admission {
            stamp: gate.stamp(Transport::Serial, epoch),
            session: 1,
            peer: peer_http::Pin::from_status(&status),
            at: Instant::now(),
            batch: None,
        };
        (
            Arc::new(Shared {
                demand: Default::default(),
                settings,
                observation: Mutex::new(Some(Observation {
                    status,
                    mapping: json!({"physical":(1..=80).collect::<Vec<_>>()}),
                    route: Route::Usb {
                        port: "fixture".into(),
                        admission,
                        sender: tx,
                    },
                    at: Instant::now(),
                })),
                stop: Arc::new(AtomicBool::new(false)),
                clients: AtomicUsize::new(0),
                receipts: Mutex::new(BTreeMap::new()),
                last_input: Mutex::new(Value::Null),
                peers: Mutex::new(BTreeMap::new()),
                events: Mutex::new(events::Journal::default()),
                active_commands: AtomicUsize::new(0),
            }),
            rx,
            gate,
        )
    }
    #[test]
    fn bridge_stop_revokes_queued_requests_without_releasing_primary_usb_owner() {
        let (shared, _, _) = fixture();
        let o = route(&shared).unwrap();
        let Route::Usb { admission, .. } = o.route else {
            unreachable!()
        };
        let (tx, rx) = mpsc::channel();
        let intent = usb_table::Intent {
            opcode: 0x4c,
            payload: vec![1, 1, 0],
            admission: admission.clone(),
            reply: Some(Reply::new(tx, shared.stop.clone())),
        };
        assert!(intent.current(1, Some(&o.status)));
        shared.stop.store(true, Ordering::Release);
        assert!(admission.stamp.current()); // Local board remains selected.
        assert!(!intent.current(1, Some(&o.status)));
        intent.reply(Ok(o.status));
        assert!(rx.recv().unwrap().is_err());
    }
    #[test]
    fn usb_demand_bridge_listener_is_not_a_reader_and_http_rgb_is_explicit() {
        let (shared, _rx, _gate) = fixture();
        assert_eq!(shared.demand.demand(), crate::usb_demand::Demand::default());
        shared.clients.store(4, Ordering::Release);
        assert_eq!(
            shared.demand.demand(),
            crate::usb_demand::Demand::default(),
            "Open sockets alone are not subscribers"
        );
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let owner = shared.clone();
        let server = thread::spawn(move || {
            for _ in 0..2 {
                let (socket, _) = listener.accept().unwrap();
                serve(socket, &owner).unwrap();
            }
        });
        // This fixture serves one request per socket and closes it; consume
        // each response and never race a pooled Windows socket's final close.
        let client = reqwest::blocking::Client::builder()
            .pool_max_idle_per_host(0)
            .build()
            .unwrap();
        let _: Value = client
            .get(format!("{base}/api/status"))
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(
            shared.demand.demand(),
            crate::usb_demand::Demand::metadata()
        );
        let _: Value = client
            .get(format!("{base}/api/status?pixels=1"))
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(
            shared.demand.demand().flags,
            crate::usb_demand::STATUS | crate::usb_demand::PIXELS
        );
        server.join().unwrap();
    }
    #[test]
    fn bridge_http_recovery_uses_existing_packet_form_contract() {
        let packet = json!({"type":"command","request_id":"recovery-id"});
        let mut url = reqwest::Url::parse("http://fixture/").unwrap();
        url.query_pairs_mut()
            .append_pair("packet", &packet.to_string());
        let mut request = Request {
            method: "POST".into(),
            path: "/api/command".into(),
            headers: BTreeMap::new(),
            body: url.query().unwrap().as_bytes().to_vec(),
        };
        assert_eq!(command_packet(&request).unwrap(), packet);
        request.body.extend(b"&packet=%7B%7D");
        assert!(command_packet(&request).is_err());
        request.body = packet.to_string().into_bytes();
        request
            .headers
            .insert("content-type".into(), "application/json".into());
        assert_eq!(command_packet(&request).unwrap(), packet);
    }
    #[test]
    fn bridge_trace_and_observation_loops_fail_closed() {
        let a = uuid::Uuid::new_v4().to_string();
        let b = uuid::Uuid::new_v4().to_string();
        assert_eq!(trace(None, &a).unwrap(), a);
        assert!(trace(Some(&a), &a).is_err());
        assert!(trace(Some("not-an-instance"), &a).is_err());
        assert_eq!(trace(Some(&a), &b).unwrap(), format!("{a},{b}"));
        let (shared, _, _) = fixture();
        let mut o = shared.observation.lock().unwrap();
        o.as_mut().unwrap().status.bridge = Some(json!({"path":[shared.settings.instance_id]}));
        drop(o);
        assert!(route(&shared).is_err());
    }
    #[test]
    fn bridge_status_keeps_board_identity_and_only_available_route_capabilities() {
        let (shared, _, gate) = fixture();
        let mut o = route(&shared).unwrap();
        let v = status_value(&shared, &o);
        assert_eq!(v["board_id"], "fixture-board");
        assert_eq!(v["hostname"], "ledboard");
        assert_eq!(v["bridge"]["hostname"], "fixture-host");
        assert_eq!(
            v["capabilities"]["led_output"],
            json!({"backend":"gpio-static","rmt":false,"dma":false,
                "available_backends":[],"runtime_switch":false})
        );
        assert!(
            !v["capabilities"]["acknowledged_features"]
                .as_array()
                .unwrap()
                .contains(&json!("ota"))
        );
        assert_eq!(v["bridge"]["path"], json!([shared.settings.instance_id]));
        o.status.pixels = 216;
        o.status.mode = "first-wins".into();
        {
            let c = o.status.capabilities.as_mut().unwrap();
            c.role = "first-wins".into();
            c.modes = vec![crate::capabilities::Mode {
                id: "first-wins".into(),
                code: 2,
                label: "First wins".into(),
                kind: "game".into(),
            }];
            c.acknowledged_features.extend([
                "inputs".into(),
                "outputs".into(),
                "key-settings".into(),
                "pixel-diagnostic".into(),
            ]);
        }
        let without_usb = status_value(&shared, &o);
        let features = without_usb["capabilities"]["acknowledged_features"]
            .as_array()
            .unwrap();
        assert!(
            !features.contains(&json!("key-settings"))
                && !features.contains(&json!("pixel-diagnostic"))
        );
        o.status
            .capabilities
            .as_mut()
            .unwrap()
            .acknowledged_features
            .extend([
                "usb-key-settings".into(),
                "usb-pixel-diagnostic".into(),
                "usb-demand".into(),
            ]);
        let with_usb = status_value(&shared, &o);
        let features = with_usb["capabilities"]["acknowledged_features"]
            .as_array()
            .unwrap();
        assert!(
            features.contains(&json!("key-settings"))
                && features.contains(&json!("pixel-diagnostic"))
        );
        assert_eq!(with_usb["capabilities"]["modes"][0]["id"], "first-wins");
        gate.select(None);
        assert!(route(&shared).is_err());
    }
    #[test]
    fn bridge_same_command_id_returns_correlated_readback_without_second_usb_write() {
        let (shared, rx, _) = fixture();
        let before = route(&shared).unwrap().status;
        let worker = thread::spawn(move || {
            if let SerialCommand::Bridge(_, admission, op, payload, reply) =
                rx.recv_timeout(Duration::from_secs(2)).unwrap()
            {
                assert_eq!(op, 0x4c);
                assert_eq!(payload, [1, 1, 0]);
                assert!(admission.current(1, Some(&before)));
                let mut actual = before;
                actual.cells[0] = "red".into();
                reply.send(Ok(actual)).unwrap();
            } else {
                panic!("Only the selected USB worker may receive this command")
            };
            assert!(rx.recv_timeout(Duration::from_millis(80)).is_err());
        });
        let request = json!({"type":"command","request_id":"one-intent","board_id":"fixture-board","boot_id":"a".repeat(32),
            "expires_at_ms":9000,"command":"cell","id":1,"state":"red","effect":"immediate"});
        let first = command(&shared, request.clone(), &BTreeMap::new());
        assert_eq!(first["accepted"], true);
        assert_eq!(first["status"]["cells"][0], "red");
        let second = command(&shared, request.clone(), &BTreeMap::new());
        assert_eq!(first, second);
        let mut foreign = request;
        foreign["id"] = json!(2);
        assert_eq!(
            command(&shared, foreign, &BTreeMap::new())["accepted"],
            false
        );
        worker.join().unwrap();
    }
    #[test]
    fn bridge_rejects_expired_wrong_peer_and_raw_unfenced_mutations_before_usb() {
        let (shared, rx, _) = fixture();
        let o = route(&shared).unwrap();
        assert!(
            mutate(
                &shared,
                &o,
                "/api/cell",
                vec![],
                &BTreeMap::new(),
                &[],
                Duration::from_secs(8)
            )
            .is_err()
        );
        for request in [
            json!({"type":"command","request_id":"old","board_id":"fixture-board","boot_id":"a".repeat(32),"expires_at_ms":1}),
            json!({"type":"command","request_id":"foreign","board_id":"other","boot_id":"a".repeat(32),"expires_at_ms":9000}),
        ] {
            assert_eq!(
                command(&shared, request, &BTreeMap::new())["accepted"],
                false
            );
        }
        assert!(rx.try_recv().is_err());
        assert!(usb_request("/api/firmware/apply", &[], &o.status).is_err());
    }
    #[test]
    fn bridge_hostname_matches_bridge_without_renaming_its_downstream_board() {
        let (shared, _, _) = fixture();
        let o = route(&shared).unwrap();
        let mut s: DeviceStatus = serde_json::from_value(status_value(&shared, &o)).unwrap();
        assert!(matches_hostname(&s, "fixture-host"));
        assert!(matches_hostname(&s, "ledboard"));
        assert!(!matches_hostname(&s, "foreign"));
        s.bridge.as_mut().unwrap()["path"] =
            json!([shared.settings.instance_id, shared.settings.instance_id]);
        assert!(!matches_hostname(&s, "fixture-host"));
    }
    #[test]
    fn bridge_network_repeater_confirms_upstream_usb_readback_and_preserves_both_hops() {
        let (upstream, rx, _) = fixture();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let peer = upstream.clone();
        let server = thread::spawn(move || {
            for _ in 0..1 {
                serve(listener.accept().unwrap().0, &peer).unwrap();
            }
        });
        let usb = thread::spawn(move || {
            let SerialCommand::Bridge(_, _, op, payload, reply) =
                rx.recv_timeout(Duration::from_secs(2)).unwrap()
            else {
                panic!("Not the selected USB owner");
            };
            assert_eq!((op, payload), (0x4c, vec![1, 1, 0]));
            let mut s = route(&upstream).unwrap().status;
            s.cells[0] = "red".into();
            reply.send(Ok(s)).unwrap();
        });
        let (downstream, _, _) = fixture();
        let gate = connection::Gate::default();
        let epoch = gate.select(Some(Transport::WiFi));
        {
            let mut o = downstream.observation.lock().unwrap();
            o.as_mut().unwrap().route = Route::Network {
                endpoint: Endpoint {
                    base: format!("http://{address}"),
                    user: String::new(),
                    password: String::new(),
                },
                stamp: gate.stamp(Transport::WiFi, epoch),
                gate: gate.clone(),
            };
        }
        let request = json!({"type":"command","request_id":"repeater-command","board_id":"fixture-board","boot_id":"a".repeat(32),"expires_at_ms":7500,"command":"cell","id":1,"state":"red","effect":"immediate"});
        let result = command(&downstream, request.clone(), &BTreeMap::new());
        assert_eq!(result["accepted"], true, "{result}");
        assert_eq!(result["status"]["cells"][0], "red");
        assert_eq!(
            result["status"]["bridge"]["path"].as_array().unwrap().len(),
            2
        );
        assert_eq!(command(&downstream, request, &BTreeMap::new()), result);
        gate.select(None);
        assert!(route(&downstream).is_err());
        usb.join().unwrap();
        server.join().unwrap();
    }
    #[test]
    fn bridge_duplex_events_continue_while_usb_ack_pending_and_http_recovers_cursor() {
        use tungstenite::client::IntoClientRequest;
        let (shared, rx, _gate) = fixture();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let owner = shared.clone();
        let server = thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let first = owner.clone();
            let ws = thread::spawn(move || serve(stream, &first).unwrap());
            let (stream, _) = listener.accept().unwrap();
            serve(stream, &owner).unwrap();
            ws.join().unwrap();
        });
        let mut handshake = format!("ws://{address}/ws").into_client_request().unwrap();
        crate::peer_identity::websocket_headers(&mut handshake);
        let (mut ws, _) = tungstenite::connect(handshake).unwrap();
        let _: Value = serde_json::from_str(ws.read().unwrap().to_text().unwrap()).unwrap();
        let before = shared.events.lock().unwrap().cursor();
        let packet = json!({"type":"command","request_id":"duplex-command","board_id":"fixture-board","boot_id":"a".repeat(32),"expires_at_ms":7500,"command":"cell","id":1,"state":"red","effect":"immediate"});
        ws.send(tungstenite::Message::Text(
            packet.clone().to_string().into(),
        ))
        .unwrap();
        let SerialCommand::Bridge(_, _, _, _, reply) =
            rx.recv_timeout(Duration::from_secs(2)).unwrap()
        else {
            panic!("Expected USB dispatch")
        };
        publish_event(
            &shared,
            json!({"type":"panel-event","board_id":"fixture-board","boot_id":"b".repeat(32),"event_id":9}),
        );
        publish_event(
            &shared,
            json!({"type":"panel-event","board_id":"fixture-board","boot_id":"a".repeat(32),"event_id":10,"text":"working"}),
        );
        let started = Instant::now();
        loop {
            let event: Value = serde_json::from_str(ws.read().unwrap().to_text().unwrap()).unwrap();
            if event["type"] == "panel-event" {
                assert_eq!(event["event_id"], 10);
                assert!(event["bridge_cursor"].is_string());
                break;
            }
        }
        assert!(
            started.elapsed() < Duration::from_millis(500),
            "Event must not wait for command ACK"
        );
        let mut actual = route(&shared).unwrap().status;
        actual.cells[0] = "red".into();
        reply.send(Ok(actual)).unwrap();
        let receipt = loop {
            let value: Value = serde_json::from_str(ws.read().unwrap().to_text().unwrap()).unwrap();
            if value["type"] == "command-result" {
                break value;
            }
        };
        assert_eq!(receipt["accepted"], true);
        assert_eq!(command(&shared, packet, &BTreeMap::new()), receipt);
        assert!(rx.try_recv().is_err());
        let client = crate::peer_identity::client_builder()
            .no_proxy()
            .build()
            .unwrap();
        let journal: Value = client
            .get(format!("http://{address}/api/events"))
            .query(&[("after", before)])
            .send()
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(journal["events"].as_array().unwrap().len(), 2);
        assert_eq!(journal["gap"], false);
        assert_eq!(journal["events"][0]["event"]["event_id"], 10);
        assert_eq!(journal["events"][1]["event"]["type"], "command-observed");
        ws.close(None).unwrap();
        drop(ws);
        server.join().unwrap();
    }
    #[test]
    fn bridge_network_correlation_rejects_foreign_or_inconsistent_ack_without_extra_get() {
        for bad in [
            "id",
            "boot",
            "readback",
            "missing-accepted",
            "null-accepted",
            "string-accepted",
            "success-error",
            "success-null-error",
            "rejected-no-error",
            "rejected-empty-error",
            "rejected-object-error",
            "valid-rejection",
        ] {
            let (shared, _, _) = fixture();
            let old = route(&shared).unwrap().status;
            let mut status = old.clone();
            status.cells[0] = "red".into();
            let mut result = json!({"type":"command-result","request_id":"forwarded","board_id":old.board_id,"boot_id":old.boot_id,"accepted":true,"status":status});
            match bad {
                "id" => result["request_id"] = "other".into(),
                "boot" => result["status"]["boot_id"] = "b".repeat(32).into(),
                "readback" => result["status"]["cells"][0] = "white".into(),
                "missing-accepted" => {
                    result.as_object_mut().unwrap().remove("accepted");
                }
                "null-accepted" => result["accepted"] = Value::Null,
                "string-accepted" => result["accepted"] = "true".into(),
                "success-error" => result["error"] = "denied".into(),
                "success-null-error" => result["error"] = Value::Null,
                "rejected-no-error" => result["accepted"] = false.into(),
                "rejected-empty-error" => {
                    result["accepted"] = false.into();
                    result["error"] = " \t".into();
                }
                "rejected-object-error" => {
                    result["accepted"] = false.into();
                    result["error"] = json!({"message":"denied"});
                }
                _ => {
                    result["accepted"] = false.into();
                    result["error"] = "denied".into();
                }
            };
            let (endpoint, server) =
                crate::tests::mock_device(vec![(200, serde_json::to_vec(&result).unwrap())]);
            let gate = connection::Gate::default();
            let epoch = gate.select(Some(Transport::WiFi));
            shared.observation.lock().unwrap().as_mut().unwrap().route = Route::Network {
                endpoint,
                stamp: gate.stamp(Transport::WiFi, epoch),
                gate: gate.clone(),
            };
            let packet = json!({"type":"command","request_id":"forwarded","board_id":"fixture-board","boot_id":"a".repeat(32),"expires_at_ms":7500,"command":"cell","id":1,"state":"red","effect":"immediate"});
            let receipt = command(&shared, packet.clone(), &BTreeMap::new());
            assert_eq!(receipt["accepted"], false, "{bad}");
            if bad == "valid-rejection" {
                assert_eq!(receipt["error"], "denied");
            }
            assert_eq!(
                route(&shared).unwrap().status.cells[0],
                "white",
                "Rejected/ambiguous ACK must not update observed state"
            );
            assert_eq!(command(&shared, packet.clone(), &BTreeMap::new()), receipt);
            let requests = server.join().unwrap();
            assert_eq!(requests.len(), 1);
            assert!(requests[0].starts_with("POST /api/command "));
            let form = reqwest::Url::parse(&format!(
                "http://bridge/?{}",
                requests[0].split_once("\r\n\r\n").unwrap().1
            ))
            .unwrap();
            let fields: Vec<_> = form.query_pairs().collect();
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].0, "packet");
            assert_eq!(serde_json::from_str::<Value>(&fields[0].1).unwrap(), packet);
        }
    }
    #[test]
    fn bridge_readback_covers_pixel_groups_and_optional_batch_effect() {
        let (shared, _, _) = fixture();
        let mut status = route(&shared).unwrap().status;
        status.cells[0] = "red".into();
        assert!(
            command_readback(
                &json!({"command":"cells","ids":"1","state":"red"}),
                &json!({"count":1}),
                &status
            )
            .unwrap()
        );
        status.effect = "none".into();
        status.led_output = json!({"enabled":true,"mode":"effects"});
        status
            .capabilities
            .as_mut()
            .unwrap()
            .acknowledged_features
            .push("pixel-diagnostic".into());
        for count in [1, 6] {
            let probe = crate::pixel_diagnostic::Probe {
                index: 474,
                group: count == 6,
                rgb: 0x12ab34,
                ..Default::default()
            };
            status.pixel_frame=Some(serde_json::from_value(json!({"revision":1,"pending":false,"brightness":255,"physical":(1..=80).collect::<Vec<_>>(),"rgb":probe.frame(false).unwrap().iter().map(|v|format!("{v:02x}")).collect::<String>()})).unwrap());
            let packet = json!({"command":"pixel-diagnostic","index":474,"count":count,"rgb":probe.rgb,"off":false});
            assert!(
                !command_readback(&packet, &json!({}), &status).unwrap(),
                "A matching RGB frame alone does not confirm the diagnostic owner/state"
            );
            status.led_output["diagnostic"] = json!({
                "active":true,"index":474,"count":count,"rgb":probe.rgb,"off":false
            });
            assert!(command_readback(&packet, &json!({}), &status).unwrap());
            status.pixel_frame.as_mut().unwrap().pending = true;
            assert!(
                !command_readback(&packet, &json!({}), &status).unwrap(),
                "Pending frames are not confirmed output"
            );
            status.led_output.as_object_mut().unwrap().remove("diagnostic");
        }
        status.pixel_frame = None;
        assert!(!command_readback(&json!({"command":"pixel-diagnostic","index":474,"count":6,"rgb":0x12ab34,"off":false}),&json!({}),&status).unwrap());
    }
    #[test]
    fn bridge_real_http_and_websocket_share_the_same_board_snapshot() {
        let (shared, _, _) = fixture();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let owner = shared.clone();
        let worker = thread::spawn(move || {
            for _ in 0..3 {
                let (socket, _) = listener.accept().unwrap();
                serve(socket, &owner).unwrap();
            }
        });
        let client = crate::peer_identity::client_builder()
            .no_proxy()
            .build()
            .unwrap();
        let value: Value = client
            .get(format!("http://{address}/api/status"))
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(value["board_id"], "fixture-board");
        let auth: Value = client
            .get(format!("http://{address}/api/auth/info"))
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(auth["enabled"], false);
        use tungstenite::client::IntoClientRequest;
        let mut request = format!("ws://{address}/ws").into_client_request().unwrap();
        crate::peer_identity::websocket_headers(&mut request);
        let (mut ws, _) = tungstenite::connect(request).unwrap();
        let value: Value = serde_json::from_str(ws.read().unwrap().to_text().unwrap()).unwrap();
        assert_eq!(value["board_id"], "fixture-board");
        ws.close(None).unwrap();
        drop(ws);
        worker.join().unwrap();
    }
    #[test]
    fn bridge_parser_rejects_duplicate_length_and_browser_origin() {
        for headers in [
            "Content-Length: 0\r\nContent-Length: 0\r\n",
            "Origin: http://foreign\r\n",
            "Transfer-Encoding: chunked\r\n",
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let address = listener.local_addr().unwrap();
            let task = thread::spawn(move || read_request(&mut listener.accept().unwrap().0));
            let mut socket = TcpStream::connect(address).unwrap();
            write!(
                socket,
                "POST /api/cell HTTP/1.1\r\nHost: fixture\r\n{headers}\r\n"
            )
            .unwrap();
            assert!(task.join().unwrap().is_err());
        }
    }
    #[test]
    fn bridge_live_headers_and_peer_registry_reject_unsupported_controls() {
        let (shared, rx, _) = fixture();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let owner = shared.clone();
        let worker = thread::spawn(move || {
            for _ in 0..4 {
                serve(listener.accept().unwrap().0, &owner).unwrap();
            }
        });
        let anonymous = reqwest::blocking::Client::builder()
            .no_proxy()
            .build()
            .unwrap();
        let rejected = anonymous
            .post(format!("http://{address}/api/reset"))
            .send()
            .unwrap();
        assert_eq!(rejected.status().as_u16(), 426);
        assert!(rx.try_recv().is_err());
        assert_eq!(
            rejected.headers()["x-pakar-version"],
            crate::peer_identity::VERSION
        );
        assert_eq!(
            rejected.headers()["server"],
            format!("PakarController/{}", crate::peer_identity::VERSION)
        );
        let client = crate::peer_identity::client_builder()
            .no_proxy()
            .build()
            .unwrap();
        let mut report = crate::client_telemetry::Reporter::new().value();
        report["password"] = "never retain".into();
        let accepted = client
            .post(format!("http://{address}/api/clients/report"))
            .form(&[("report", report.to_string())])
            .send()
            .unwrap();
        assert!(accepted.status().is_success());
        let peers: Value = client
            .get(format!("http://{address}/api/clients"))
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(peers["clients"].as_array().unwrap().len(), 1);
        assert_eq!(peers["clients"][0]["observed_ip"], "127.0.0.1");
        assert!(!peers.to_string().contains("never retain"));
        let public: Value = anonymous
            .get(format!("http://{address}/api/peer/info"))
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(public["version"], crate::peer_identity::VERSION);
        assert_eq!(public["compatibility"]["enforced"], true);
        worker.join().unwrap();
    }
}

impl crate::PakarControllerApp {
    pub(crate) fn bridge_tick(&mut self) {
        self.bridge
            .reconcile(&self.config.bridge, self.screenshot_harness);
        let observation = self.live_status().cloned().and_then(|status| {
            let route = match self.command_route()? {
                Transport::Serial => Route::Usb {
                    port: self.serial_port.clone(),
                    admission: self.serial_admission()?,
                    sender: self.serial_tx.clone(),
                },
                Transport::WiFi => Route::Network {
                    endpoint: self.endpoint(),
                    stamp: self
                        .connection_gate
                        .stamp(Transport::WiFi, self.connection_gate.epoch()),
                    gate: self.connection_gate.clone(),
                },
            };
            let at = match self.transport {
                Transport::Serial => self.last_serial_status_at?,
                Transport::WiFi => self.last_status_at?,
            };
            let mapping = if self.mapping_peer == peer_http::Pin::from_status(&status)
                && self.mappings.len() == 80
            {
                json!({"board_id":status.board_id,"boot_id":status.boot_id,
                    "physical":self.mappings.iter().map(|c|c.physical_cell).collect::<Vec<_>>(),
                    "names":self.mappings.iter().map(|c|c.name.clone()).collect::<Vec<_>>(),
                    "pixel_groups":status.pixel_groups})
            } else {
                Value::Null
            };
            Some(Observation {
                status,
                mapping,
                route,
                at,
            })
        });
        self.bridge.observe(observation);
    }
}
