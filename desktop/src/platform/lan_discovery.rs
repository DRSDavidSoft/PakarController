//! Actual Pealayer server/discovery.rs sockets, DNS-SD and bounded parsers;
//! PCController discovery.go/mndp.go metadata and lifetime rules. No authority.
use mdns_sd::{DaemonEvent, ServiceDaemon, ServiceEvent, ServiceInfo};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};
const SSDP_GROUP: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const TYPE: &str = "urn:schemas-upnp-org:device:Basic:1";
#[derive(Default)]
struct Registry {
    active: BTreeSet<String>,
    errors: BTreeMap<String, String>,
    peers: BTreeMap<String, (Instant, Value)>,
    attempted: BTreeMap<String, Instant>,
}
pub struct Advertiser {
    stop: Arc<AtomicBool>,
    daemon: Option<ServiceDaemon>,
    registry: Arc<Mutex<Registry>>,
}
impl Advertiser {
    pub fn start(settings: &crate::bridge::Settings) -> Self {
        let mut result = Self {
            stop: Arc::new(AtomicBool::new(false)),
            daemon: None,
            registry: Arc::new(Mutex::new(Registry::default())),
        };
        let Ok(bound) = settings.validate() else {
            return result;
        };
        // Donor native adapter enumeration: advertise only bound LAN interfaces.
        if bound.ip().is_loopback() {
            return result;
        }
        let addresses: Vec<Ipv4Addr> = NetworkInterface::show()
            .unwrap_or_default()
            .into_iter()
            .flat_map(|n| n.addr)
            .filter_map(|a| match a.ip() {
                IpAddr::V4(ip)
                    if !ip.is_loopback()
                        && !ip.is_unspecified()
                        && !ip.is_multicast()
                        && (bound.ip().is_unspecified() || bound.ip() == IpAddr::V4(ip)) =>
                {
                    Some(ip)
                }
                _ => None,
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        if addresses.is_empty() {
            result.fault("interfaces", "No bound LAN interface");
            return result;
        }
        let (tx, rx) = mpsc::sync_channel(64);
        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..4 {
            let rx = rx.clone();
            let registry = result.registry.clone();
            let stop = result.stop.clone();
            thread::spawn(move || {
                while !stop.load(Ordering::Acquire) {
                    let candidate = rx.lock().unwrap().recv_timeout(Duration::from_millis(250));
                    if let Ok(candidate) = candidate {
                        verify(candidate, &registry);
                    } else if matches!(candidate, Err(mpsc::RecvTimeoutError::Disconnected)) {
                        break;
                    }
                }
            });
        }
        if settings.mdns {
            match ServiceDaemon::new() {
                Ok(daemon) => {
                    let peer_policy = crate::peer_identity::advertised_policy();
                    let service = ServiceInfo::new(
                        "_pakarcontroller._tcp.local.",
                        &format!("PakarController on {}", settings.hostname),
                        &format!("{}.local.", settings.hostname),
                        addresses
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(","),
                        settings.port,
                        [
                            ("product", "pakarcontroller"),
                            ("kind", "bridge"),
                            ("api", "/api"),
                            ("ws", "/ws"),
                            ("hostname", settings.hostname.as_str()),
                            ("instance", settings.instance_id.as_str()),
                            ("version", crate::peer_identity::VERSION),
                            ("minimum_app", peer_policy.minimum_app.as_str()),
                            ("minimum_board", peer_policy.minimum_board.as_str()),
                            (
                                "version_check",
                                if crate::peer_identity::enforced() {
                                    "true"
                                } else {
                                    "false"
                                },
                            ),
                        ]
                        .as_slice(),
                    );
                    match service.and_then(|s| daemon.register(s)) {
                        Ok(()) => {}
                        Err(e) => result.fault("mdns", e),
                    }
                    if let Ok(monitor) = daemon.monitor() {
                        let stop = result.stop.clone();
                        let registry = result.registry.clone();
                        let monitor_daemon = daemon.clone();
                        thread::spawn(move || {
                            while !stop.load(Ordering::Acquire) {
                                match monitor.recv_timeout(Duration::from_millis(250)) {
                                    Ok(DaemonEvent::Announce(..)) => {
                                        let mut r = registry.lock().unwrap();
                                        r.active.insert("mdns".into());
                                        r.active.insert("dns-sd".into());
                                    }
                                    Ok(DaemonEvent::Error(e)) => {
                                        let mut r = registry.lock().unwrap();
                                        r.active.remove("mdns");
                                        r.active.remove("dns-sd");
                                        r.errors.insert("mdns".into(), e.to_string());
                                    }
                                    Ok(DaemonEvent::NameChange(change)) => {
                                        let mut r = registry.lock().unwrap();
                                        r.active.remove("mdns");
                                        r.active.remove("dns-sd");
                                        r.errors.insert("mdns".into(), format!("Name conflict for {}; choose another hostname. Automatic suffix rejected.", change.original));
                                        let _ = monitor_daemon.shutdown();
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                        });
                    }
                    for kind in ["_pakarcontroller._tcp.local.", "_http._tcp.local."] {
                        match daemon.browse(kind) {
                            Ok(events) => {
                                let stop = result.stop.clone();
                                let tx = tx.clone();
                                let registry = result.registry.clone();
                                let own = settings.instance_id.clone();
                                thread::spawn(move || {
                                    while !stop.load(Ordering::Acquire) {
                                        if let Ok(ServiceEvent::ServiceResolved(service)) =
                                            events.recv_timeout(Duration::from_millis(250))
                                        {
                                            if service.get_property_val_str("instance")
                                                == Some(&own)
                                            {
                                                continue;
                                            }
                                            for ip in service.get_addresses_v4() {
                                                offer(
                                                    &tx,
                                                    &registry,
                                                    Candidate {
                                                        origin: format!(
                                                            "http://{ip}:{}",
                                                            service.get_port()
                                                        ),
                                                        hostname: service
                                                            .get_hostname()
                                                            .trim_end_matches('.')
                                                            .into(),
                                                        protocol: "dns-sd",
                                                    },
                                                );
                                            }
                                        }
                                    }
                                });
                            }
                            Err(e) => result.fault("dns-sd", e),
                        }
                    }
                    result.daemon = Some(daemon);
                }
                Err(e) => result.fault("mdns", e),
            }
        }
        for (enabled, mndp) in [(settings.ssdp, false), (settings.mndp, true)] {
            if !enabled {
                continue;
            }
            let group = if mndp {
                Ipv4Addr::new(224, 0, 0, 88)
            } else {
                SSDP_GROUP
            };
            let port = if mndp { 5678 } else { 1900 };
            match udp_socket(port, group, &addresses) {
                Ok(socket) => {
                    result
                        .registry
                        .lock()
                        .unwrap()
                        .active
                        .insert(if mndp { "mndp" } else { "ssdp" }.into());
                    let stop = result.stop.clone();
                    let registry = result.registry.clone();
                    let settings = settings.clone();
                    let addresses = addresses.clone();
                    let tx = tx.clone();
                    thread::spawn(move || {
                        let mut last = Instant::now() - Duration::from_secs(31);
                        let mut sequence = 0u16;
                        let mut replies = BTreeMap::new();
                        let mut bytes = [0; 4097];
                        while !stop.load(Ordering::Acquire) {
                            if last.elapsed() >= Duration::from_secs(30) {
                                if mndp {
                                    let packet = encode_mndp(
                                        &settings.hostname,
                                        &settings.instance_id,
                                        settings.port,
                                        sequence,
                                    );
                                    sequence = sequence.wrapping_add(1);
                                    let _ = socket.send_to(&packet, (Ipv4Addr::BROADCAST, port));
                                    let _ = socket.send_to(&[0, 0, 0, 0], (group, port));
                                } else {
                                    for ip in &addresses {
                                        let _ = socket.send_to(
                                            ssdp(&settings, *ip, "ssdp:alive").as_bytes(),
                                            (group, port),
                                        );
                                    }
                                    let _ = socket.send_to(SEARCH, (group, port));
                                }
                                last = Instant::now();
                            }
                            let (length, source) = match socket.recv_from(&mut bytes) {
                                Ok(v) => v,
                                Err(e)
                                    if matches!(
                                        e.kind(),
                                        std::io::ErrorKind::WouldBlock
                                            | std::io::ErrorKind::TimedOut
                                    ) =>
                                {
                                    continue;
                                }
                                Err(e) => {
                                    let mut r = registry.lock().unwrap();
                                    r.active.remove(if mndp { "mndp" } else { "ssdp" });
                                    r.errors.insert(
                                        if mndp { "mndp" } else { "ssdp" }.into(),
                                        e.to_string(),
                                    );
                                    break;
                                }
                            };
                            if !local_source(source) || length > 4096 {
                                continue;
                            }
                            let request = &bytes[..length];
                            let query = if mndp {
                                request == [0, 0, 0, 0]
                            } else {
                                search_request(request)
                            };
                            if query {
                                replies.retain(|_, at: &mut Instant| {
                                    at.elapsed() < Duration::from_secs(1)
                                });
                                if replies.len() >= 128
                                    || replies.insert(source.ip(), Instant::now()).is_some()
                                {
                                    continue;
                                }
                                let response = if mndp {
                                    encode_mndp(
                                        &settings.hostname,
                                        &settings.instance_id,
                                        settings.port,
                                        sequence,
                                    )
                                } else {
                                    // Route selection, not a guessed /24, chooses the real outbound NIC.
                                    let probe = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok();
                                    let ip = probe
                                        .and_then(|s| {
                                            s.connect(source).ok()?;
                                            s.local_addr().ok()
                                        })
                                        .map(|a| a.ip())
                                        .and_then(|ip| match ip {
                                            IpAddr::V4(v) => Some(v),
                                            _ => None,
                                        })
                                        .filter(|ip| addresses.contains(ip))
                                        .unwrap_or(addresses[0]);
                                    ssdp(&settings, ip, "").into_bytes()
                                };
                                let _ = socket.send_to(&response, source);
                            } else if let Some(candidate) = if mndp {
                                parse_mndp(request, source)
                            } else {
                                parse_ssdp(request, source)
                            } {
                                if !addresses.iter().any(|ip| {
                                    candidate.origin == format!("http://{ip}:{}", settings.port)
                                }) {
                                    offer(&tx, &registry, candidate);
                                }
                            }
                        }
                        if !mndp {
                            for ip in &addresses {
                                let _ = socket.send_to(
                                    ssdp(&settings, *ip, "ssdp:byebye").as_bytes(),
                                    (group, port),
                                );
                            }
                        }
                    });
                }
                Err(e) => result.fault(if mndp { "mndp" } else { "ssdp" }, e),
            }
        }
        result
    }
    fn fault(&self, protocol: &str, error: impl std::fmt::Display) {
        self.registry
            .lock()
            .unwrap()
            .errors
            .insert(protocol.into(), error.to_string());
    }
    pub fn snapshot(&self) -> Value {
        let mut r = self.registry.lock().unwrap();
        r.peers
            .retain(|_, (at, _)| at.elapsed() < Duration::from_secs(120));
        json!({"active_protocols":r.active,"errors":r.errors,"peers":r.peers.values().map(|(_,v)|v).collect::<Vec<_>>(),"automatic_authority":false})
    }
}
impl Drop for Advertiser {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(d) = self.daemon.take() {
            let _ = d.shutdown();
        }
    }
}
#[derive(Clone)]
struct Candidate {
    origin: String,
    hostname: String,
    protocol: &'static str,
}
fn local_source(address: SocketAddr) -> bool {
    matches!(address.ip(),IpAddr::V4(ip) if !ip.is_unspecified()&&!ip.is_multicast()&&!ip.is_broadcast()&&address.port()!=0)
}
fn offer(tx: &mpsc::SyncSender<Candidate>, registry: &Mutex<Registry>, candidate: Candidate) {
    let mut r = registry.lock().unwrap();
    r.attempted
        .retain(|_, at| at.elapsed() < Duration::from_secs(30));
    if r.attempted.len() >= 128 || r.attempted.contains_key(&candidate.origin) {
        return;
    }
    r.attempted.insert(candidate.origin.clone(), Instant::now());
    drop(r);
    let _ = tx.try_send(candidate);
}
fn verify(candidate: Candidate, registry: &Mutex<Registry>) {
    let Ok(client) = crate::peer_identity::client_builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_millis(900))
        .build()
    else {
        return;
    };
    let read = |path: &str, limit: u64| -> Option<Vec<u8>> {
        let response = client
            .get(format!("{}{path}", candidate.origin))
            .send()
            .ok()?;
        if !response.status().is_success() {
            return None;
        }
        let mut bytes = Vec::new();
        response.take(limit + 1).read_to_end(&mut bytes).ok()?;
        (bytes.len() <= limit as usize).then_some(bytes)
    };
    let public = read("/api/peer/info", 8192)
        .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        .filter(|v| v["product"] == "PakarController"||v["application"]=="PakarController")
        .or_else(||read("/api/discovery",8192).and_then(|b|serde_json::from_slice::<Value>(&b).ok()).filter(|v|v["product"]=="PakarController"));
    let status = read("/api/status", 32768)
        .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        .filter(valid_status);
    if public.is_none() && status.is_none() {
        let Some(xml) = read("/description.xml", 8192).and_then(|b| String::from_utf8(b).ok())
        else {
            return;
        };
        if !xml.contains("PakarController")
            || !xml.contains("<manufacturer>AtomicDeploy</manufacturer>")
        {
            return;
        }
    }
    let instance = public
        .as_ref()
        .and_then(|v| v["instance_id"].as_str())
        .unwrap_or("");
    let board = status
        .as_ref()
        .and_then(|v| v["board_id"].as_str())
        .unwrap_or("");
    let key = if !board.is_empty() {
        format!("board:{board}")
    } else if !instance.is_empty() {
        format!("instance:{instance}")
    } else {
        candidate.origin.clone()
    };
    let mut r = registry.lock().unwrap();
    if r.peers.len() >= 128 && !r.peers.contains_key(&key) {
        return;
    }
    let mut protocols = r
        .peers
        .get(&key)
        .and_then(|(_, v)| v["protocols"].as_array())
        .cloned()
        .unwrap_or_default();
    if !protocols.contains(&json!(candidate.protocol)) {
        protocols.push(json!(candidate.protocol));
    }
    let mut aliases = r
        .peers
        .get(&key)
        .and_then(|(_, v)| v["aliases"].as_array())
        .cloned()
        .unwrap_or_default();
    if !aliases.contains(&json!(candidate.origin)) {
        aliases.push(json!(candidate.origin));
    }
    let hostname = public
        .as_ref()
        .and_then(|v| v["hostname"].as_str())
        .filter(|v| crate::identity::normalize_name(v).is_ok())
        .map(|v| format!("{v}.local"))
        .unwrap_or(candidate.hostname.clone());
    let preferred = if !hostname.is_empty() {
        reqwest::Url::parse(&candidate.origin)
            .ok()
            .and_then(|mut u| {
                u.set_host(Some(&hostname)).ok()?;
                Some(u.to_string().trim_end_matches('/').to_owned())
            })
            .unwrap_or(candidate.origin.clone())
    } else {
        candidate.origin
    };
    r.peers.insert(key,(Instant::now(),json!({"endpoint":preferred,"aliases":aliases,"board_id":board,"instance_id":instance,"protocols":protocols,"verified":true,"connected":false,
        "version":public.as_ref().and_then(|v|v.get("version")).or_else(||status.as_ref().and_then(|v|v.get("version"))),
        "board_version":status.as_ref().and_then(|v|v.get("version")),
        "compatibility":public.as_ref().and_then(|v|v.get("compatibility")).or_else(||status.as_ref().and_then(|v|v.get("compatibility"))),
        "supported":status.as_ref().and_then(|v|serde_json::from_value::<crate::DeviceStatus>(v.clone()).ok()).map(|s|crate::peer_identity::validate_board(&s).is_ok())})));
}
fn valid_status(v: &Value) -> bool {
    serde_json::from_value::<crate::DeviceStatus>(v.clone())
        .ok()
        .is_some_and(|s| {
            crate::peer_http::Pin::from_status(&s).is_some()
                && s.capabilities.as_ref().is_some_and(|c| c.known())
        })
}
pub(super) fn verified_origin(origin: &str) -> bool {
    let registry = Mutex::new(Registry::default());
    verify(
        Candidate {
            origin: origin.into(),
            hostname: String::new(),
            protocol: "ssdp",
        },
        &registry,
    );
    !registry.lock().unwrap().peers.is_empty()
}
pub(super) fn ssdp_origin(packet: &[u8], source: SocketAddr) -> Option<String> {
    parse_ssdp(packet, source).map(|c| c.origin)
}
fn udp_socket(port: u16, group: Ipv4Addr, addresses: &[Ipv4Addr]) -> std::io::Result<UdpSocket> {
    let socket = socket2::Socket::new(
        socket2::Domain::IPV4,
        socket2::Type::DGRAM,
        Some(socket2::Protocol::UDP),
    )?;
    socket.set_reuse_address(true)?;
    socket.bind(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)).into())?;
    let socket: UdpSocket = socket.into();
    for ip in addresses {
        socket.join_multicast_v4(&group, ip)?;
    }
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(Duration::from_millis(250)))?;
    Ok(socket)
}
/// Read-only discovery uses the same source-bound parsers as the advertiser,
/// without enabling a bridge listener or granting authority to advertisements.
pub(super) fn scan_origins(timeout: Duration) -> Vec<String> {
    let addresses: Vec<Ipv4Addr> = NetworkInterface::show()
        .unwrap_or_default()
        .into_iter()
        .flat_map(|n| n.addr)
        .filter_map(|a| match a.ip() {
            IpAddr::V4(ip) if !ip.is_loopback() && !ip.is_unspecified() && !ip.is_multicast() => {
                Some(ip)
            }
            _ => None,
        })
        .collect();
    let ssdp = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).ok();
    let mndp = udp_socket(5678, Ipv4Addr::new(224, 0, 0, 88), &addresses).ok();
    if let Some(socket) = &ssdp {
        let _ = socket.set_nonblocking(true);
        let _ = socket.send_to(SEARCH, (SSDP_GROUP, 1900));
    }
    if let Some(socket) = &mndp {
        let _ = socket.set_nonblocking(true);
        let _ = socket.send_to(&[0, 0, 0, 0], (Ipv4Addr::BROADCAST, 5678));
        let _ = socket.send_to(&[0, 0, 0, 0], (Ipv4Addr::new(224, 0, 0, 88), 5678));
    }
    let daemon = ServiceDaemon::new().ok();
    let browsers: Vec<_> = daemon
        .as_ref()
        .into_iter()
        .flat_map(|d| {
            ["_pakarcontroller._tcp.local.", "_pakar-update._tcp.local.", "_http._tcp.local."]
                .into_iter()
                .filter_map(|kind| d.browse(kind).ok())
        })
        .collect();
    let deadline = Instant::now() + timeout;
    let mut origins = BTreeSet::new();
    let mut packet = [0; 4097];
    let mut overflow = false;
    while Instant::now() < deadline {
        for (socket, is_mndp) in [(&ssdp, false), (&mndp, true)] {
            if let Some(socket) = socket {
                for _ in 0..32 {
                    match socket.recv_from(&mut packet) {
                        Ok((count, source)) => {
                            if let Some(candidate) = if is_mndp {
                                parse_mndp(&packet[..count], source)
                            } else {
                                parse_ssdp(&packet[..count], source)
                            } {
                                if origins.len() < 16 || origins.contains(&candidate.origin) {
                                    origins.insert(candidate.origin);
                                } else {
                                    overflow = true;
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
            }
        }
        for browser in &browsers {
            for _ in 0..32 {
                match browser.try_recv() {
                    Ok(ServiceEvent::ServiceResolved(service)) => {
                        for ip in service.get_addresses_v4() {
                            if !ip.is_unspecified() && !ip.is_multicast() {
                                let origin = format!("http://{ip}:{}", service.get_port());
                                if origins.len() < 16 || origins.contains(&origin) {
                                    origins.insert(origin);
                                } else {
                                    overflow = true;
                                }
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
    if let Some(daemon) = daemon {
        for kind in ["_pakarcontroller._tcp.local.", "_http._tcp.local."] {
            let _ = daemon.stop_browse(kind);
        }
        let _ = daemon.shutdown();
    }
    if overflow {
        Vec::new()
    } else {
        origins.into_iter().collect()
    }
}
const SEARCH:&[u8]=concat!("M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 1\r\nST: ssdp:all\r\nUSER-AGENT: PakarController/",env!("CARGO_PKG_VERSION")," native\r\n\r\n").as_bytes();
fn search_request(packet: &[u8]) -> bool {
    std::str::from_utf8(packet).is_ok_and(|s| {
        s.starts_with("M-SEARCH * HTTP/1.1\r\n")
            && s.to_ascii_lowercase().contains("man: \"ssdp:discover\"")
            && (s.to_ascii_lowercase().contains("st: ssdp:all")
                || s.contains(TYPE)
                || s.to_ascii_lowercase().contains("st: upnp:rootdevice"))
    })
}
fn ssdp(s: &crate::bridge::Settings, ip: Ipv4Addr, nts: &str) -> String {
    let first = if nts.is_empty() {
        "HTTP/1.1 200 OK\r\nEXT:\r\n".to_string()
    } else {
        format!("NOTIFY * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nNTS: {nts}\r\nNT: {TYPE}\r\n")
    };
    format!(
        "{first}CACHE-CONTROL: max-age=60\r\n{}ST: {TYPE}\r\nUSN: uuid:{}::{TYPE}\r\nLOCATION: http://{ip}:{}/description.xml\r\nX-Digitalogic-Product: pakarcontroller\r\nX-PCController-Name: {}\r\n\r\n",
        crate::peer_identity::server_headers(),
        s.instance_id,
        s.port,
        s.hostname
    )
}
// Donor big-endian vendor product/API-port TLVs, never an invented port80 app.
fn encode_mndp(name: &str, id: &str, port: u16, seq: u16) -> Vec<u8> {
    let uuid = uuid::Uuid::parse_str(id).unwrap_or_default();
    let mut mac = uuid.as_bytes()[..6].to_vec();
    mac[0] = (mac[0] | 2) & !1;
    let mut packet = vec![0, 0];
    packet.extend(seq.to_be_bytes());
    for (kind, data) in [
        (1u16, mac),
        (5, name.as_bytes().to_vec()),
        (7, crate::peer_identity::VERSION.as_bytes().to_vec()),
        (8, b"Digitalogic".to_vec()),
        (11, id.as_bytes().to_vec()),
        (12, b"Application endpoint".to_vec()),
        (16, b"application".to_vec()),
        (0xff01, b"pakarcontroller".to_vec()),
        (0xff02, port.to_be_bytes().to_vec()),
    ] {
        packet.extend(kind.to_be_bytes());
        packet.extend((data.len() as u16).to_be_bytes());
        packet.extend(data);
    }
    packet
}
fn parse_mndp(packet: &[u8], source: SocketAddr) -> Option<Candidate> {
    if !(14..=2048).contains(&packet.len()) || packet[..2] != [0, 0] || !local_source(source) {
        return None;
    }
    let mut fields = BTreeMap::new();
    let mut at = 4;
    while at < packet.len() {
        if at + 4 > packet.len() {
            return None;
        }
        let kind = u16::from_be_bytes(packet[at..at + 2].try_into().ok()?);
        let length = u16::from_be_bytes(packet[at + 2..at + 4].try_into().ok()?) as usize;
        at += 4;
        if at + length > packet.len() || fields.insert(kind, &packet[at..at + length]).is_some() {
            return None;
        }
        at += length;
    }
    if fields.get(&1)?.len() != 6 {
        return None;
    }
    let name = std::str::from_utf8(fields.get(&5)?).ok()?;
    if name.is_empty() || name.len() > 96 || name.chars().any(char::is_control) {
        return None;
    }
    let port = if fields
        .get(&0xff01)
        .is_some_and(|v| *v == b"pakarcontroller")
    {
        u16::from_be_bytes((*fields.get(&0xff02)?).try_into().ok()?)
    } else if fields.get(&12).is_some_and(|v| *v == b"PakarController") {
        80
    } else {
        return None;
    };
    if port == 0 {
        return None;
    }
    Some(Candidate {
        origin: format!("http://{}:{port}", source.ip()),
        hostname: String::new(),
        protocol: "mndp",
    })
}

fn parse_ssdp(packet: &[u8], source: SocketAddr) -> Option<Candidate> {
    if packet.len() > 4096 || !local_source(source) {
        return None;
    }
    let text = std::str::from_utf8(packet).ok()?;
    let mut lines = text.lines();
    if !lines.next()?.starts_with("HTTP/1.1 200 ") {
        return None;
    }
    let mut headers = BTreeMap::new();
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            if headers
                .insert(k.trim().to_ascii_lowercase(), v.trim())
                .is_some()
            {
                return None;
            }
        }
    }
    let url = reqwest::Url::parse(headers.get("location")?).ok()?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.host_str()?.parse::<IpAddr>().ok()? != source.ip()
    {
        return None;
    }
    Some(Candidate {
        origin: format!(
            "{}://{}:{}",
            url.scheme(),
            source.ip(),
            url.port_or_known_default()?
        ),
        hostname: String::new(),
        protocol: "ssdp",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bridge_mndp_uses_explicit_application_port_not_guessed_port80() {
        let bytes = encode_mndp(
            "controller-host",
            &uuid::Uuid::new_v4().to_string(),
            8789,
            1,
        );
        let source = "192.0.2.1:5678".parse().unwrap();
        assert_eq!(
            parse_mndp(&bytes, source).unwrap().origin,
            "http://192.0.2.1:8789"
        );
        assert!(parse_mndp(&bytes[..bytes.len() - 1], source).is_none());
        let mut duplicate = bytes.clone();
        duplicate.extend(&bytes[4..14]);
        assert!(parse_mndp(&duplicate, source).is_none());
        assert!(parse_mndp(&[0, 0, 0, 0], source).is_none());
    }
    #[test]
    fn bridge_ssdp_requires_source_location_binding_and_unique_headers() {
        let source = "192.0.2.1:1900".parse().unwrap();
        let packet = b"HTTP/1.1 200 OK\r\nLOCATION: http://192.0.2.1:8789/description.xml\r\n\r\n";
        assert_eq!(
            parse_ssdp(packet, source).unwrap().origin,
            "http://192.0.2.1:8789"
        );
        assert!(parse_ssdp(packet, "192.0.2.2:1900".parse().unwrap()).is_none());
        assert!(
            parse_ssdp(
                b"HTTP/1.1 200 OK\r\nLOCATION: http://192.0.2.1\r\nLOCATION: http://192.0.2.1\r\n",
                source
            )
            .is_none()
        );
    }
    #[test]
    fn bridge_loopback_registration_starts_no_discovery_transports() {
        let settings = crate::bridge::Settings {
            listen: "127.0.0.1".into(),
            ..Default::default()
        };
        let owner = Advertiser::start(&settings);
        assert_eq!(owner.snapshot()["active_protocols"], json!([]));
        assert_eq!(owner.snapshot()["peers"], json!([]));
    }
}
