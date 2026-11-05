//! Actual PCController Session acquireWrite/requestAdmission before/after
//! admission and closing checks, adapted to a revocable native HTTP queue.
//! This is not import of its serial acquisition, ACK routing or reset policy.
use crate::{DeviceStatus, Event, HttpCommand};
use reqwest::blocking::Client;
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpStream},
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
        mpsc::{SendError, Sender},
    },
    time::{Duration, Instant},
};

const ADDRESS_CACHE_LIMIT: usize = 32;
const WEBSOCKET_TIMEOUT: Duration = Duration::from_secs(4);

/// Routing hints only, never board/boot ownership or permission to replay.
/// Adapted from PCController lanresolver's successful-TCP-address cache.
#[derive(Default)]
struct AddressCache {
    entries: VecDeque<(String, IpAddr)>,
    relookup: HashSet<String>,
    verified_routes: BTreeMap<String, SavedRoute>,
    restored_routes: BTreeMap<String, SavedRoute>,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SavedRoute {
    pub endpoint: String,
    pub numeric_origin: String,
    pub board_id: String,
}
impl SavedRoute {
    pub fn valid(&self) -> bool {
        let (Some(logical), Some(numeric)) = (
            device_origin(&self.endpoint),
            device_origin(&self.numeric_origin),
        ) else {
            return false;
        };
        numeric.host_str().is_some_and(|host| {
            host.trim_matches(['[', ']'])
                .parse::<IpAddr>()
                .is_ok_and(|ip| !ip.is_unspecified() && !ip.is_multicast())
        }) && logical.scheme() == numeric.scheme()
            && logical.port_or_known_default() == numeric.port_or_known_default()
            && Pin::from_identity(&self.board_id, &"a".repeat(32)).is_some()
    }
}
impl AddressCache {
    fn remember(&mut self, domain: String, address: IpAddr) {
        self.entries.retain(|(known, _)| *known != domain);
        self.relookup.remove(&domain);
        if self.entries.len() >= ADDRESS_CACHE_LIMIT
            && let Some((evicted, _)) = self.entries.pop_front()
        {
            self.relookup.remove(&evicted);
        }
        self.entries.push_back((domain, address));
    }
    fn invalidate(&mut self, domain: &str) {
        self.entries.retain(|(known, _)| known != domain);
        self.relookup.remove(domain);
        self.verified_routes
            .retain(|origin, _| named_origin(origin).is_none_or(|(name, _)| name != domain));
        self.restored_routes
            .retain(|origin, _| named_origin(origin).is_none_or(|(name, _)| name != domain));
    }
    fn request_relookup(&mut self, domain: &str) {
        if self.entries.iter().any(|(known, _)| known == domain) {
            self.relookup.insert(domain.to_owned());
        }
    }
}
fn address_cache() -> &'static Mutex<AddressCache> {
    static CACHE: OnceLock<Mutex<AddressCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(AddressCache::default()))
}

fn device_origin(origin: &str) -> Option<reqwest::Url> {
    let url = reqwest::Url::parse(origin).ok()?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !matches!(url.path(), "" | "/")
    {
        return None;
    }
    url.host_str()?;
    Some(url)
}
fn local_origin(origin: &str) -> Option<(String, String, u16)> {
    let url = device_origin(origin)?;
    let domain = url.host_str()?.to_ascii_lowercase();
    let expected = domain.strip_suffix(".local")?;
    if crate::identity::normalize_name(expected).ok().as_deref() != Some(expected) {
        return None;
    }
    Some((
        domain.clone(),
        expected.to_owned(),
        url.port_or_known_default()?,
    ))
}
fn named_origin(origin: &str) -> Option<(String, u16)> {
    let url = device_origin(origin)?;
    let domain = url.host_str()?.to_ascii_lowercase();
    if domain.len() > 253
        || domain.parse::<IpAddr>().is_ok()
        || !domain.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && label
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'-')
                && label.as_bytes()[0].is_ascii_alphanumeric()
                && label.as_bytes()[label.len() - 1].is_ascii_alphanumeric()
        })
    {
        return None;
    }
    Some((domain, url.port_or_known_default()?))
}

fn verified_address(
    origin: &str,
    status: &DeviceStatus,
    remote: Option<SocketAddr>,
) -> Option<(String, IpAddr)> {
    let (domain, port) = named_origin(origin)?;
    let remote = remote?;
    if remote.port() != port
        || remote.ip().is_unspecified()
        || remote.ip().is_multicast()
        || Pin::from_status(status).is_none()
    {
        return None;
    }
    // DNS aliases need not equal the configured board name. A .local board
    // name must still match; learning an alias never relaxes that contract.
    if domain.ends_with(".local") {
        let (_, expected, _) = local_origin(origin)?;
        if !crate::bridge::matches_hostname(status, &expected) {
            return None;
        }
    }
    Some((domain, remote.ip()))
}

/// Call only with remote_addr() from the actual successfully decoded response.
/// Status IP/SSDP metadata is deliberately never consulted. No TTL expiration:
/// HTTP/auth/body-timeout failures do not evict a previously verified route.
pub fn remember_address(origin: &str, status: &DeviceStatus, remote: Option<SocketAddr>) {
    if let Some((domain, address)) = verified_address(origin, status, remote)
        && let Ok(mut cache) = address_cache().lock()
    {
        cache.remember(domain, address);
        let mut numeric = device_origin(origin).unwrap();
        if numeric.set_host(Some(&address.to_string())).is_ok() {
            if cache.verified_routes.len() >= 32
                && !cache
                    .verified_routes
                    .contains_key(origin.trim_end_matches('/'))
            {
                if let Some(first) = cache.verified_routes.keys().next().cloned() {
                    cache.verified_routes.remove(&first);
                }
            }
            cache.verified_routes.insert(
                origin.trim_end_matches('/').into(),
                SavedRoute {
                    endpoint: origin.trim_end_matches('/').into(),
                    numeric_origin: numeric.to_string().trim_end_matches('/').into(),
                    board_id: status.board_id.clone(),
                },
            );
        }
    }
    // An IP-only fallback learns the board's actual configured hostname from
    // this same verified HTTP/TCP response, never from a claimed status IP or
    // multicast packet. Future HTTP and WS requests retain Host/SNI ownership.
    if let Some(preferred) = configured_origin(origin, status)
        && preferred.trim_end_matches('/') != origin.trim_end_matches('/')
        && let Some((domain, address)) = verified_address(&preferred, status, remote)
        && let Ok(mut cache) = address_cache().lock()
    {
        cache.remember(domain, address);
        let mut numeric = device_origin(&preferred).unwrap();
        if numeric.set_host(Some(&address.to_string())).is_ok() {
            if cache.verified_routes.len() >= 32 && !cache.verified_routes.contains_key(&preferred)
            {
                if let Some(first) = cache.verified_routes.keys().next().cloned() {
                    cache.verified_routes.remove(&first);
                }
            }
            cache.verified_routes.insert(
                preferred.clone(),
                SavedRoute {
                    endpoint: preferred,
                    numeric_origin: numeric.to_string().trim_end_matches('/').into(),
                    board_id: status.board_id.clone(),
                },
            );
        }
    }
}
pub fn configured_origin(origin: &str, status: &DeviceStatus) -> Option<String> {
    let mut url = device_origin(origin)?;
    let name = if let Some(bridge) = &status.bridge {
        bridge["hostname"].as_str()?
    } else {
        status.hostname.as_str()
    };
    let name = crate::identity::normalize_name(name).ok()?;
    // A remote DNS alias is deliberately retained. Only literal LAN addresses
    // and existing local names become the configured mDNS board name.
    let host = url.host_str()?;
    if host
        .trim_matches(['[', ']'])
        .parse::<IpAddr>()
        .is_ok_and(|ip| ip.is_loopback())
    {
        return Some(origin.trim_end_matches('/').into());
    }
    if host.trim_matches(['[', ']']).parse::<IpAddr>().is_err() && !host.ends_with(".local") {
        return Some(origin.trim_end_matches('/').into());
    }
    url.set_host(Some(&format!("{name}.local"))).ok()?;
    Some(url.to_string().trim_end_matches('/').into())
}
/// A fresh same-peer WS status may report a completed rename before HTTP can
/// accept the old Host header. Reuse only the selected session's real TCP hint.
pub fn rebind_verified_name(origin: &str, previous: &DeviceStatus, status: &DeviceStatus) {
    if Pin::from_status(previous).is_none()
        || Pin::from_status(previous) != Pin::from_status(status)
    {
        return;
    }
    let Some((domain, port)) = named_origin(origin) else {
        return;
    };
    let remote = address_cache().lock().ok().and_then(|cache| {
        cache
            .entries
            .iter()
            .find(|(name, _)| name == &domain)
            .map(|(_, ip)| SocketAddr::new(*ip, port))
    });
    if let Some(preferred) = configured_origin(origin, status) {
        remember_address(&preferred, status, remote);
    }
}
/// Read-only recovery candidate, never a direct mutation destination. Even a
/// cached address must pass a fresh public identity/status probe after failure.
pub fn cached_origin(origin: &str) -> Option<String> {
    let cache = address_cache().lock().ok()?;
    cache
        .verified_routes
        .get(origin.trim_end_matches('/'))
        .or_else(|| cache.restored_routes.get(origin.trim_end_matches('/')))
        .map(|route| route.numeric_origin.clone())
}
pub fn restore_route(route: &SavedRoute, endpoint: &str) {
    if !route.valid() || route.endpoint != endpoint.trim_end_matches('/') {
        return;
    }
    if let Ok(mut cache) = address_cache().lock() {
        if cache.restored_routes.len() >= 32 && !cache.restored_routes.contains_key(&route.endpoint)
        {
            if let Some(first) = cache.restored_routes.keys().next().cloned() {
                cache.restored_routes.remove(&first);
            }
        }
        cache
            .restored_routes
            .insert(route.endpoint.clone(), route.clone());
    }
}
pub fn recovery_matches(origin: &str, status: &DeviceStatus) -> bool {
    address_cache()
        .lock()
        .ok()
        .and_then(|cache| {
            cache
                .verified_routes
                .get(origin.trim_end_matches('/'))
                .or_else(|| cache.restored_routes.get(origin.trim_end_matches('/')))
                .cloned()
        })
        .is_some_and(|route| route.board_id == status.board_id)
}
pub fn remembered_board(origin: &str) -> Option<String> {
    address_cache().lock().ok().and_then(|cache| {
        cache
            .verified_routes
            .get(origin.trim_end_matches('/'))
            .or_else(|| cache.restored_routes.get(origin.trim_end_matches('/')))
            .map(|route| route.board_id.clone())
    })
}
pub fn verified_route(origin: &str, status: &DeviceStatus) -> Option<SavedRoute> {
    address_cache()
        .lock()
        .ok()?
        .verified_routes
        .get(origin.trim_end_matches('/'))
        .filter(|route| route.board_id == status.board_id)
        .cloned()
}

/// A confirmed resolution failure can explicitly remove the obsolete hint.
pub fn invalidate_address(origin: &str) {
    if let Some((domain, _)) = named_origin(origin)
        && let Ok(mut cache) = address_cache().lock()
    {
        cache.invalidate(&domain);
    }
}

/// A failed CONNECT may request a fresh lookup on the next client without
/// discarding the last verified address. No command or body read is replayed.
/// Marks exist only for cached keys, so they share the same 32-entry bound.
pub fn request_relookup(origin: &str) {
    if let Some((domain, _)) = named_origin(origin)
        && let Ok(mut cache) = address_cache().lock()
    {
        cache.request_relookup(&domain);
    }
}

/// Preserve the URL hostname/Host header and HTTPS SNI/certificate checks while
/// bypassing repeated slow OS DNS/mDNS lookup for previously verified peers.
/// Build a fresh client after invalidation; never automatically resend a write.
pub fn device_client_builder() -> reqwest::blocking::ClientBuilder {
    let mut builder = crate::peer_identity::client_builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(4))
        .connect_timeout(Duration::from_secs(4))
        .pool_max_idle_per_host(0);
    if let Ok(cache) = address_cache().lock() {
        for (domain, address) in &cache.entries {
            if !cache.relookup.contains(domain) {
                builder = builder.resolve(domain, SocketAddr::new(*address, 0));
            }
        }
    }
    builder
}

/// TCP route only. Numeric endpoints need no bootstrap. Named endpoints wait
/// for the existing selected HTTP worker to learn the actual TCP peer; this
/// worker performs no HTTP or OS DNS work of its own. None is normal startup,
/// not a connection failure. Discovery metadata never chooses this dial.
fn websocket_address(
    endpoint: &crate::Endpoint,
    stamp: &crate::connection::Stamp,
) -> Result<Option<SocketAddr>, String> {
    if !stamp.current() {
        return Err("WebSocket connection retired before resolution".into());
    }
    let url = device_origin(&endpoint.base).ok_or("Invalid WebSocket device origin")?;
    let port = url
        .port_or_known_default()
        .ok_or("Missing WebSocket port")?;
    // host_str() includes IPv6 brackets on some URL implementations; trim only
    // those delimiters before parsing a numeric literal, never a DNS alias.
    let host = url.host_str().ok_or("Missing WebSocket host")?;
    if let Ok(address) = host
        .trim_start_matches('[')
        .trim_end_matches(']')
        .parse::<IpAddr>()
    {
        if address.is_unspecified() || address.is_multicast() {
            return Err("Invalid WebSocket TCP address".into());
        }
        return Ok(Some(SocketAddr::new(address, port)));
    }
    let (domain, _) = named_origin(&endpoint.base).ok_or("Invalid WebSocket hostname")?;
    if let Ok(cache) = address_cache().lock()
        && !cache.relookup.contains(&domain)
        && let Some((_, address)) = cache.entries.iter().find(|(known, _)| known == &domain)
    {
        return Ok(Some(SocketAddr::new(*address, port)));
    }
    Ok(None)
}

/// One owned socket, no resolver thread or leaked timeout worker. The absolute
/// handshake deadline also covers a peer dripping HTTP/TLS bytes just often
/// enough to evade a per-read timeout. Runtime reads remain briefly bounded.
pub struct WebSocketTransport {
    socket: TcpStream,
    stamp: crate::connection::Stamp,
    handshake_deadline: Option<Instant>,
}
impl WebSocketTransport {
    fn prepare(&self, reading: bool) -> io::Result<()> {
        if !self.stamp.current() {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "WebSocket route retired",
            ));
        }
        if let Some(deadline) = self.handshake_deadline {
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .filter(|d| !d.is_zero())
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::TimedOut,
                        "WebSocket handshake deadline elapsed",
                    )
                })?;
            if reading {
                self.socket.set_read_timeout(Some(remaining))?;
            } else {
                self.socket.set_write_timeout(Some(remaining))?;
            }
        }
        Ok(())
    }
    fn established(&mut self) -> io::Result<()> {
        self.prepare(true)?;
        self.handshake_deadline = None;
        self.socket
            .set_read_timeout(Some(Duration::from_millis(40)))?;
        self.socket.set_write_timeout(Some(WEBSOCKET_TIMEOUT))
    }
}
impl Read for WebSocketTransport {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.prepare(true)?;
        self.socket.read(bytes)
    }
}
impl Write for WebSocketTransport {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.prepare(false)?;
        self.socket.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.prepare(false)?;
        self.socket.flush()
    }
}
pub type DeviceWebSocket =
    tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<WebSocketTransport>>;

/// The original request retains its URL/Host/TLS SNI. The learned IP is used
/// only by connect_timeout; upgrade success never grants board authority.
pub fn connect_websocket(
    endpoint: &crate::Endpoint,
    mut request: tungstenite::handshake::client::Request,
    stamp: crate::connection::Stamp,
) -> Result<Option<DeviceWebSocket>, String> {
    if request.uri().to_string() != crate::websocket_url(&endpoint.base)? {
        return Err("WebSocket request does not match the selected device origin".into());
    }
    let Some(address) = websocket_address(endpoint, &stamp)? else {
        return Ok(None);
    };
    if !stamp.current() {
        return Err("WebSocket route retired before TCP connection".into());
    }
    let socket = TcpStream::connect_timeout(&address, WEBSOCKET_TIMEOUT).map_err(|error| {
        request_relookup(&endpoint.base);
        crate::diagnostics::describe_error(error)
    })?;
    socket
        .set_nodelay(true)
        .map_err(crate::diagnostics::describe_error)?;
    socket
        .set_read_timeout(Some(WEBSOCKET_TIMEOUT))
        .map_err(crate::diagnostics::describe_error)?;
    socket
        .set_write_timeout(Some(WEBSOCKET_TIMEOUT))
        .map_err(crate::diagnostics::describe_error)?;
    let transport = WebSocketTransport {
        socket,
        stamp,
        handshake_deadline: Some(Instant::now() + WEBSOCKET_TIMEOUT),
    };
    transport
        .prepare(true)
        .map_err(crate::diagnostics::describe_error)?;
    crate::peer_identity::websocket_headers(&mut request);
    let (mut socket, response) = tungstenite::client_tls(request, transport).map_err(|error|{
        if matches!(&error,tungstenite::HandshakeError::Failure(tungstenite::Error::Http(response)) if response.status().as_u16()==426){"Peer compatibility: server rejected this app version; update it or explicitly disable its version gate".into()}
        else{crate::diagnostics::describe_error(error)}
    })?;
    crate::peer_identity::validate_response_headers(response.headers())?;
    match socket.get_mut() {
        tungstenite::stream::MaybeTlsStream::Plain(stream) => stream.established(),
        tungstenite::stream::MaybeTlsStream::Rustls(stream) => stream.sock.established(),
        _ => return Err("Unsupported WebSocket TLS transport".into()),
    }
    .map_err(crate::diagnostics::describe_error)?;
    Ok(Some(socket))
}
pub fn websocket_read_timeout(
    socket: &mut DeviceWebSocket,
    timeout: Duration,
) -> Result<(), String> {
    let stream = match socket.get_mut() {
        tungstenite::stream::MaybeTlsStream::Plain(stream) => stream,
        tungstenite::stream::MaybeTlsStream::Rustls(stream) => &mut stream.sock,
        _ => return Err("Unsupported WebSocket TLS transport".into()),
    };
    stream
        .socket
        .set_read_timeout(Some(timeout))
        .map_err(crate::diagnostics::describe_error)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pin {
    board: String,
    boot: String,
}
impl Pin {
    pub fn board_id(&self) -> &str {
        &self.board
    }
    pub fn matches_identity(&self, board: &str, boot: &str) -> bool {
        self.board == board && self.boot == boot
    }
    pub fn from_status(s: &DeviceStatus) -> Option<Self> {
        if !s.http_peer_fencing {
            return None;
        }
        Self::from_identity(&s.board_id, &s.boot_id)
    }
    /// Board/boot syntax is shared by all transports. HTTP separately requires
    /// its advertised fencing feature; USB must not invent that capability.
    pub fn from_identity(board: &str, boot: &str) -> Option<Self> {
        (!board.is_empty()
            && board.len() <= 64
            && board
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-' || c == b'_')
            && boot.len() == 32
            && boot
                .bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
            && boot.bytes().any(|c| c != b'0'))
        .then(|| Self {
            board: board.to_owned(),
            boot: boot.to_owned(),
        })
    }
    pub fn client(&self) -> Result<Client, String> {
        let mut headers = crate::peer_identity::headers();
        headers.insert(
            "x-pakar-board",
            self.board.parse().map_err(|_| "Invalid board identity")?,
        );
        headers.insert(
            "x-pakar-boot",
            self.boot.parse().map_err(|_| "Invalid boot identity")?,
        );
        device_client_builder()
            .default_headers(headers)
            .build()
            .map_err(crate::diagnostics::describe_error)
    }
}
pub fn pinned_client(status: &DeviceStatus) -> Result<Client, String> {
    Pin::from_status(status)
        .ok_or("Read-only: peer has not advertised valid board/boot identity and HTTP fencing")?
        .client()
}
#[derive(Clone)]
pub struct Scope {
    pub auto_discovery: bool,
    pub pin: Option<Pin>,
    generation: u64,
    clock: Arc<AtomicU64>,
    queued_at: Instant,
}
impl Scope {
    pub fn current(&self) -> bool {
        self.clock.load(Ordering::Acquire) == self.generation
    }
    pub fn admit(&self) -> Result<(), String> {
        self.admit_with_limit(Duration::from_secs(8))
    }
    pub fn admit_read(&self) -> Result<(), String> {
        self.admit_with_limit(Duration::from_secs(24))
    }
    fn admit_with_limit(&self, limit: Duration) -> Result<(), String> {
        if !self.current() {
            return Err("Board session revoked before dispatch; nothing was sent".into());
        }
        if self.queued_at.elapsed() >= limit {
            return Err(
                "Queued request expired before dispatch; refresh instead of replaying".into(),
            );
        }
        Ok(())
    }
}
pub struct Queue {
    inner: Sender<HttpCommand>,
    clock: Arc<AtomicU64>,
    pin: Option<Pin>,
    auto_discovery: bool,
}
impl Queue {
    pub fn new(inner: Sender<HttpCommand>) -> Self {
        Self {
            inner,
            clock: Arc::new(AtomicU64::new(0)),
            pin: None,
            auto_discovery: true,
        }
    }
    pub fn send(&self, command: HttpCommand) -> Result<(), SendError<HttpCommand>> {
        self.inner
            .send(HttpCommand::Scoped(self.capture(), Box::new(command)))
    }
    pub fn capture(&self) -> Scope {
        Scope {
            auto_discovery: self.auto_discovery,
            pin: self.pin.clone(),
            generation: self.clock.load(Ordering::Acquire),
            clock: self.clock.clone(),
            queued_at: Instant::now(),
        }
    }
    pub fn clear(&mut self) {
        self.pin = None;
        self.clock.fetch_add(1, Ordering::AcqRel);
    }
    pub fn set_auto_discovery(&mut self, enabled: bool) {
        self.auto_discovery = enabled;
    }
    pub fn observe(&mut self, status: &DeviceStatus) -> bool {
        let next = Pin::from_status(status);
        if next == self.pin {
            return false;
        }
        self.clear();
        self.pin = next;
        true
    }
    pub fn writable(&self) -> bool {
        self.pin.is_some()
    }
}

/// A retired result stays in Events, never another peer's drafts/credentials.
/// Payload/error bodies are deliberately not included in this receipt.
pub fn retired_receipt(event: &Event) -> Option<String> {
    let kind = match event {
        Event::Accepted(_) => "Command acceptance",
        Event::CellBatchConfirmed(_) | Event::SerialBatchConfirmed(..) => {
            "Atomic cell batch readback"
        }
        Event::OperationFailed(_) => "Command failure",
        Event::SettingsFinished(..) => "Settings result",
        Event::MaintenanceFinished(..) => "Maintenance result",
        Event::PixelModeFinished(..) | Event::PixelSent(..) | Event::PixelReadbackFinished(..) => {
            "Pixel diagnostic result"
        }
        Event::CredentialsUpdated(_) => "Credential result",
        Event::NameUpdated(..) => "Identity result",
        _ => return None,
    };
    Some(format!(
        "{kind} belongs to a retired board session; not applied to this peer. Inspect the original device before retrying."
    ))
}

/// Preserve generic retired outcomes even when the outer endpoint changed.
/// Never carry its response body or credentials into the new peer.
pub fn retired_envelope(event: &Event) -> Option<String> {
    match event {
        Event::ForConnection(stamp, inner) if !stamp.current() => {
            retired_envelope(inner).or_else(|| retired_receipt(inner))
        }
        Event::ForConnection(_, inner) => retired_envelope(inner),
        Event::ForEndpoint(_, inner) => retired_envelope(inner),
        Event::ForScope(scope, inner) if !scope.current() => retired_receipt(inner),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn status(boot: char) -> DeviceStatus {
        DeviceStatus {
            board_id: "esp8266-aabbccddeeff".into(),
            boot_id: boot.to_string().repeat(32),
            http_peer_fencing: true,
            ..Default::default()
        }
    }
    #[test]
    fn websocket_bootstrap_preserves_hostname_and_uses_only_verified_tcp_routes() {
        use tungstenite::client::IntoClientRequest;
        for (host, seeded) in [
            ("ws-route-fixture.local", true),
            ("localhost", true),
            ("127.0.0.1", false),
        ] {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            listener.set_nonblocking(true).unwrap();
            let address = listener.local_addr().unwrap();
            let origin = format!("http://{host}:{}", address.port());
            let mut peer = status('a');
            peer.hostname = "ws-route-fixture".into();
            peer.ip = "203.0.113.99".into(); // Never a dial target.
            let body = serde_json::to_vec(&peer).unwrap();
            let server = std::thread::spawn(move || {
                let until = Instant::now() + Duration::from_secs(6);
                let accept = || loop {
                    match listener.accept() {
                        Ok((stream, _)) => {
                            // Winsock may inherit the listener's nonblocking mode.
                            // Only accept is polled; fixture HTTP/WS I/O must wait
                            // for bytes within the bounded socket timeouts below.
                            stream.set_nonblocking(false).unwrap();
                            stream
                                .set_read_timeout(Some(Duration::from_secs(2)))
                                .unwrap();
                            stream
                                .set_write_timeout(Some(Duration::from_secs(2)))
                                .unwrap();
                            break stream;
                        }
                        Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                            assert!(Instant::now() < until, "Bounded fixture accept deadline");
                            std::thread::sleep(Duration::from_millis(2));
                        }
                        Err(error) => panic!("{error}"),
                    }
                };
                if seeded {
                    let mut stream = accept();
                    let mut request = Vec::new();
                    while !request.windows(4).any(|v| v == b"\r\n\r\n") {
                        let mut bytes = [0; 1024];
                        let count = stream.read(&mut bytes).unwrap();
                        assert!(count > 0 && request.len() + count <= 8192);
                        request.extend_from_slice(&bytes[..count]);
                    }
                    assert!(request.starts_with(b"GET /api/status "));
                    write!(
                        stream,
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    )
                    .unwrap();
                    stream.write_all(&body).unwrap();
                }
                let mut observed = String::new();
                let socket = tungstenite::accept_hdr(
                    accept(),
                    |request: &tungstenite::handshake::server::Request,
                     response: tungstenite::handshake::server::Response| {
                        assert_eq!(request.uri().path(), "/ws");
                        observed = request.headers()["Host"].to_str().unwrap().to_owned();
                        Ok(response)
                    },
                )
                .unwrap();
                drop(socket);
                observed
            });
            if seeded {
                // Only a real decoded HTTP response/remote_addr seeds the cache;
                // this isolated fixture substitutes DNS, not board authority.
                let response = Client::builder()
                    .no_proxy()
                    .timeout(Duration::from_secs(2))
                    .resolve(host, address)
                    .build()
                    .unwrap()
                    .get(format!("{origin}/api/status"))
                    .send()
                    .unwrap();
                let remote = response.remote_addr();
                let observed: DeviceStatus = response.json().unwrap();
                remember_address(&origin, &observed, remote);
            }
            let endpoint = crate::Endpoint {
                base: origin.clone(),
                user: String::new(),
                password: String::new(),
            };
            let gate = crate::connection::Gate::default();
            let epoch = gate.select(Some(crate::connection::Method::WiFi));
            let request = crate::websocket_url(&origin)
                .unwrap()
                .into_client_request()
                .unwrap();
            let mut socket = connect_websocket(
                &endpoint,
                request,
                gate.stamp(crate::connection::Method::WiFi, epoch),
            )
            .unwrap()
            .unwrap();
            websocket_read_timeout(&mut socket, Duration::from_millis(7)).unwrap();
            if let tungstenite::stream::MaybeTlsStream::Plain(stream) = socket.get_mut() {
                assert_eq!(
                    stream.socket.read_timeout().unwrap(),
                    Some(Duration::from_millis(7))
                );
                assert!(stream.handshake_deadline.is_none());
            } else {
                panic!("Plain loopback fixture unexpectedly selected TLS");
            }
            drop(socket);
            assert_eq!(server.join().unwrap(), format!("{host}:{}", address.port()));
            if host == "localhost" {
                assert!(
                    address_cache()
                        .lock()
                        .unwrap()
                        .entries
                        .iter()
                        .any(|(key, _)| key == host)
                );
            }
            invalidate_address(&origin);
        }
    }
    #[test]
    fn websocket_unlearned_or_relookup_named_route_waits_without_opening_socket() {
        use tungstenite::client::IntoClientRequest;
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        for host in [
            "unlearned-ws-route-fixture.local",
            "unlearned-ws-route-fixture.example",
        ] {
            let origin = format!("http://{host}:{}", address.port());
            invalidate_address(&origin);
            let endpoint = crate::Endpoint {
                base: origin.clone(),
                user: String::new(),
                password: String::new(),
            };
            let gate = crate::connection::Gate::default();
            let epoch = gate.select(Some(crate::connection::Method::WiFi));
            let request = || {
                crate::websocket_url(&origin)
                    .unwrap()
                    .into_client_request()
                    .unwrap()
            };
            assert!(
                connect_websocket(
                    &endpoint,
                    request(),
                    gate.stamp(crate::connection::Method::WiFi, epoch)
                )
                .unwrap()
                .is_none()
            );
            // A real prior hint marked for relookup must wait, not dial its old
            // address or start another resolver in this worker.
            let mut peer = status('a');
            peer.hostname = "unlearned-ws-route-fixture".into();
            remember_address(&origin, &peer, Some(address));
            request_relookup(&origin);
            assert!(
                connect_websocket(
                    &endpoint,
                    request(),
                    gate.stamp(crate::connection::Method::WiFi, epoch)
                )
                .unwrap()
                .is_none()
            );
            assert_eq!(
                listener.accept().unwrap_err().kind(),
                io::ErrorKind::WouldBlock
            );
            invalidate_address(&origin);
        }
    }
    #[test]
    fn websocket_route_revocation_and_request_mismatch_never_open_a_socket() {
        use tungstenite::client::IntoClientRequest;
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let endpoint = crate::Endpoint {
            base: format!("http://{}", listener.local_addr().unwrap()),
            user: String::new(),
            password: String::new(),
        };
        let gate = crate::connection::Gate::default();
        let epoch = gate.select(Some(crate::connection::Method::WiFi));
        let mismatched = "ws://127.0.0.1:9/ws".into_client_request().unwrap();
        assert!(
            connect_websocket(
                &endpoint,
                mismatched,
                gate.stamp(crate::connection::Method::WiFi, epoch)
            )
            .is_err()
        );
        gate.select(None);
        let request = crate::websocket_url(&endpoint.base)
            .unwrap()
            .into_client_request()
            .unwrap();
        assert!(
            connect_websocket(
                &endpoint,
                request,
                gate.stamp(crate::connection::Method::WiFi, epoch)
            )
            .is_err()
        );
        assert_eq!(
            listener.accept().unwrap_err().kind(),
            io::ErrorKind::WouldBlock
        );
    }
    #[test]
    fn websocket_handshake_deadline_and_runtime_route_check_are_independent() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let socket =
            TcpStream::connect_timeout(&listener.local_addr().unwrap(), Duration::from_secs(1))
                .unwrap();
        let (_peer, _) = listener.accept().unwrap();
        let gate = crate::connection::Gate::default();
        let epoch = gate.select(Some(crate::connection::Method::WiFi));
        let mut stream = WebSocketTransport {
            socket,
            stamp: gate.stamp(crate::connection::Method::WiFi, epoch),
            handshake_deadline: Some(Instant::now() - Duration::from_millis(1)),
        };
        assert_eq!(
            stream.read(&mut [0]).unwrap_err().kind(),
            io::ErrorKind::TimedOut
        );
        stream.handshake_deadline = Some(Instant::now() + WEBSOCKET_TIMEOUT);
        stream.established().unwrap();
        assert!(stream.handshake_deadline.is_none());
        gate.select(None);
        assert_eq!(
            stream.read(&mut [0]).unwrap_err().kind(),
            io::ErrorKind::ConnectionAborted
        );
        assert_eq!(
            stream.write(&[1]).unwrap_err().kind(),
            io::ErrorKind::ConnectionAborted
        );
    }
    #[test]
    fn local_address_cache_accepts_only_verified_tcp_peer_not_advertised_ip() {
        let mut peer = status('a');
        peer.hostname = "ledboard".into();
        peer.ip = "203.0.113.99".into();
        let actual: SocketAddr = "192.0.2.10:80".parse().unwrap();
        let accepted = verified_address("http://LEDBOARD.local", &peer, Some(actual)).unwrap();
        assert_eq!(accepted, ("ledboard.local".into(), actual.ip()));
        assert_eq!(
            verified_address("http://ledboard.example", &peer, Some(actual)),
            Some(("ledboard.example".into(), actual.ip()))
        );
        assert_eq!(
            verified_address("http://localhost", &peer, Some(actual)),
            Some(("localhost".into(), actual.ip()))
        );
        assert!(verified_address("http://ledboard.local", &peer, None).is_none());
        peer.hostname = "foreign".into();
        assert!(verified_address("http://ledboard.local", &peer, Some(actual)).is_none());
        peer.hostname = "ledboard".into();
        peer.http_peer_fencing = false;
        assert!(verified_address("http://ledboard.local", &peer, Some(actual)).is_none());
        peer.http_peer_fencing = true;
        for remote in ["192.0.2.10:81", "0.0.0.0:80", "239.1.1.1:80"] {
            assert!(
                verified_address(
                    "http://ledboard.local",
                    &peer,
                    Some(remote.parse().unwrap())
                )
                .is_none()
            );
        }
        for origin in [
            "http://127.0.0.1",
            "http://[::1]",
            "http://bad_name.example",
            "http://user@ledboard.local",
            "http://ledboard.local/api",
            "http://ledboard.local?secret=x",
            "http://ledboard.local#part",
        ] {
            assert!(verified_address(origin, &peer, Some(actual)).is_none());
        }
    }
    #[test]
    fn discovery_numeric_fallback_and_same_peer_rename_share_actual_session_tcp_cache() {
        let mut observed = status('a');
        observed.hostname = "discovery-cache-old".into();
        observed.ip = "203.0.113.99".into();
        let source = "http://192.0.2.55:8080";
        let remote: SocketAddr = "192.0.2.55:8080".parse().unwrap();
        remember_address(source, &observed, Some(remote));
        let old = "http://discovery-cache-old.local:8080";
        assert_eq!(cached_origin(old).as_deref(), Some(source));
        let mut renamed = observed.clone();
        renamed.hostname = "discovery-cache-new".into();
        rebind_verified_name(old, &observed, &renamed);
        let new = "http://discovery-cache-new.local:8080";
        assert_eq!(cached_origin(new).as_deref(), Some(source));
        let mut foreign = renamed.clone();
        foreign.hostname = "discovery-cache-foreign".into();
        foreign.board_id = "different-board".into();
        rebind_verified_name(old, &observed, &foreign);
        assert!(cached_origin("http://discovery-cache-foreign.local:8080").is_none());
        assert_eq!(
            configured_origin("https://remote.example:8443", &renamed).as_deref(),
            Some("https://remote.example:8443")
        );
        for alias in [
            "http://127.0.0.1:18082",
            "http://[::1]:18082",
            "http://localhost:18082",
        ] {
            assert_eq!(
                configured_origin(alias, &renamed).as_deref(),
                Some(alias),
                "A forwarded port is not a board LAN port"
            );
        }
        let mut forwarded = observed.clone();
        forwarded.hostname = "forwarded-not-dns".into();
        remember_address(
            "http://127.0.0.1:18082",
            &forwarded,
            Some("127.0.0.1:18082".parse().unwrap()),
        );
        assert!(cached_origin("http://forwarded-not-dns.local:18082").is_none());
        invalidate_address(old);
        invalidate_address(new);
        assert!(cached_origin(old).is_none());
        assert!(cached_origin(new).is_none());
    }
    #[test]
    fn discovery_saved_route_roundtrip_stays_readonly_endpoint_board_and_port_bound() {
        let route = SavedRoute {
            endpoint: "http://saved-discovery.local:8081".into(),
            numeric_origin: "http://192.0.2.44:8081".into(),
            board_id: "saved-board".into(),
        };
        assert!(route.valid());
        let mut config = crate::config::AppConfig::default();
        config.endpoint = route.endpoint.clone();
        config.last_network_route = Some(route.clone());
        let decoded: crate::config::AppConfig =
            serde_json::from_slice(&serde_json::to_vec(&config).unwrap()).unwrap();
        assert_eq!(decoded.last_network_route, Some(route.clone()));
        invalidate_address(&route.endpoint);
        restore_route(&route, "http://different-discovery.local:8081");
        assert!(cached_origin(&route.endpoint).is_none());
        restore_route(&route, &route.endpoint);
        assert_eq!(
            cached_origin(&route.endpoint),
            Some(route.numeric_origin.clone())
        );
        let mut observed = status('a');
        observed.board_id = route.board_id.clone();
        observed.hostname = "saved-discovery".into();
        assert!(verified_route(&route.endpoint, &observed).is_none());
        assert!(recovery_matches(&route.endpoint, &observed));
        assert!(
            websocket_address(
                &crate::Endpoint {
                    base: route.endpoint.clone(),
                    user: String::new(),
                    password: String::new()
                },
                &{
                    let gate = crate::connection::Gate::default();
                    let epoch = gate.select(Some(crate::Transport::WiFi));
                    gate.stamp(crate::Transport::WiFi, epoch)
                }
            )
            .unwrap()
            .is_none(),
            "A saved profile is not an admitted WS dial hint"
        );
        observed.board_id = "another-board".into();
        assert!(!recovery_matches(&route.endpoint, &observed));
        let mut invalid = route.clone();
        invalid.numeric_origin = "https://192.0.2.44:8081".into();
        assert!(!invalid.valid());
        invalid = route.clone();
        invalid.numeric_origin = "http://192.0.2.44:8082".into();
        assert!(!invalid.valid());
        invalid = route.clone();
        invalid.numeric_origin = "http://unverified-discovery.local:8081".into();
        assert!(!invalid.valid());
        invalidate_address(&route.endpoint);
        assert!(cached_origin(&route.endpoint).is_none());
    }
    #[test]
    fn local_address_cache_bounds_replaces_and_invalidates_without_ttl_or_authority() {
        let mut cache = AddressCache::default();
        let first: IpAddr = "192.0.2.10".parse().unwrap();
        let replacement: IpAddr = "192.0.2.11".parse().unwrap();
        for n in 0..ADDRESS_CACHE_LIMIT + 4 {
            cache.remember(format!("board-{n}.local"), first);
        }
        assert_eq!(cache.entries.len(), ADDRESS_CACHE_LIMIT);
        assert!(
            !cache
                .entries
                .iter()
                .any(|(name, _)| name == "board-0.local")
        );
        cache.remember("board-4.local".into(), replacement);
        assert_eq!(cache.entries.len(), ADDRESS_CACHE_LIMIT);
        assert_eq!(
            cache.entries.back(),
            Some(&("board-4.local".into(), replacement))
        );
        assert_eq!(
            cache
                .entries
                .iter()
                .filter(|(name, _)| name == "board-4.local")
                .count(),
            1
        );
        cache.invalidate("board-4.local");
        assert!(
            !cache
                .entries
                .iter()
                .any(|(name, _)| name == "board-4.local")
        );
        let mut peer = status('a');
        peer.hostname = "new-address".into();
        let (_, address) = verified_address(
            "http://new-address.local:8787",
            &peer,
            Some(SocketAddr::new(replacement, 8787)),
        )
        .unwrap();
        cache.remember("new-address.local".into(), address);
        assert_eq!(cache.entries.back().unwrap().1, replacement);
    }
    #[test]
    fn local_relookup_marks_retain_verified_hints_until_success_or_explicit_invalidation() {
        let mut cache = AddressCache::default();
        let old: IpAddr = "192.0.2.10".parse().unwrap();
        let fresh: IpAddr = "192.0.2.11".parse().unwrap();
        cache.remember("ledboard.local".into(), old);
        cache.request_relookup("ledboard.local");
        assert_eq!(cache.entries.back(), Some(&("ledboard.local".into(), old)));
        assert!(cache.relookup.contains("ledboard.local"));
        let mut status = status('a');
        status.hostname = "ledboard".into();
        let (domain, address) = verified_address(
            "http://ledboard.local",
            &status,
            Some(SocketAddr::new(fresh, 80)),
        )
        .unwrap();
        cache.remember(domain, address);
        assert_eq!(
            cache.entries.back(),
            Some(&("ledboard.local".into(), fresh))
        );
        assert!(!cache.relookup.contains("ledboard.local"));
        cache.request_relookup("ledboard.local");
        cache.invalidate("ledboard.local");
        assert!(cache.entries.is_empty() && cache.relookup.is_empty());
        for n in 0..ADDRESS_CACHE_LIMIT * 2 {
            let domain = format!("board-{n}.local");
            cache.remember(domain.clone(), old);
            cache.request_relookup(&domain);
            cache.request_relookup(&format!("unknown-{n}.local"));
        }
        assert_eq!(cache.entries.len(), ADDRESS_CACHE_LIMIT);
        assert_eq!(cache.relookup.len(), ADDRESS_CACHE_LIMIT);
        assert!(
            cache
                .relookup
                .iter()
                .all(|name| cache.entries.iter().any(|(key, _)| key == name))
        );
        assert!(!cache.relookup.contains("board-0.local"));
    }
    #[test]
    fn local_verified_address_override_preserves_http_hostname_and_explicit_port() {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let origin = format!("http://cache-route-fixture.local:{}", address.port());
        let mut peer = status('a');
        peer.hostname = "cache-route-fixture".into();
        let body = serde_json::to_vec(&peer).unwrap();
        let server = std::thread::spawn(move || {
            let mut requests = Vec::new();
            let started = Instant::now();
            while requests.len() < 2 {
                let (mut socket, _) = match listener.accept() {
                    Ok(connection) => connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(
                            started.elapsed() < Duration::from_secs(6),
                            "Expected two bounded loopback requests"
                        );
                        std::thread::sleep(Duration::from_millis(10));
                        continue;
                    }
                    Err(error) => panic!("{error}"),
                };
                // Do not inherit the polled listener's mode on Windows.
                socket.set_nonblocking(false).unwrap();
                socket
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut request = Vec::new();
                while !request.windows(4).any(|v| v == b"\r\n\r\n") {
                    let mut chunk = [0; 1024];
                    let count = socket.read(&mut chunk).unwrap();
                    assert!(count > 0 && request.len() + count <= 8192);
                    request.extend_from_slice(&chunk[..count]);
                }
                requests.push(String::from_utf8(request).unwrap());
                write!(
                    socket,
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .unwrap();
                socket.write_all(&body).unwrap();
            }
            requests
        });
        // Bootstrap an explicit simulated name resolution. Only the actual TCP
        // response below, not this fixture override or status.ip, seeds cache.
        let response = Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(2))
            .resolve("cache-route-fixture.local", address)
            .build()
            .unwrap()
            .get(format!("{origin}/api/status"))
            .send()
            .unwrap();
        let remote = response.remote_addr();
        let observed: DeviceStatus = response.json().unwrap();
        remember_address(&origin, &observed, remote);
        let again: DeviceStatus = device_client_builder()
            .build()
            .unwrap()
            .get(format!("{origin}/api/status"))
            .send()
            .unwrap()
            .json()
            .unwrap();
        invalidate_address(&origin);
        assert_eq!(again.hostname, "cache-route-fixture");
        for request in server.join().unwrap() {
            assert!(request.to_ascii_lowercase().contains(&format!(
                "host: cache-route-fixture.local:{}\r\n",
                address.port()
            )));
        }
    }
    #[test]
    fn shared_pin_validates_fence_and_complete_identity() {
        assert!(Pin::from_status(&status('a')).is_some());
        for mut s in [status('0'), status('A'), status('x')] {
            assert!(Pin::from_status(&s).is_none());
            s.boot_id.clear();
            assert!(Pin::from_status(&s).is_none());
        }
        for board in ["", "UPPER", "space name", "bad\n", &"a".repeat(65)] {
            let mut s = status('a');
            s.board_id = board.into();
            assert!(Pin::from_status(&s).is_none());
        }
        let mut s = status('a');
        s.http_peer_fencing = false;
        assert!(pinned_client(&s).is_err());
    }
    #[test]
    fn usb_report_optional_identity_validation_does_not_require_http_fencing() {
        let mut s = status('a');
        s.http_peer_fencing = false;
        assert!(Pin::from_identity(&s.board_id, &s.boot_id).is_some());
        assert!(
            Pin::from_status(&s).is_none(),
            "Do not invent HTTP fence support for USB"
        );
        for board in ["", "UPPER", "space name", &"a".repeat(65)] {
            assert!(Pin::from_identity(board, &s.boot_id).is_none());
        }
        for boot in [
            "",
            &"0".repeat(32),
            &"A".repeat(32),
            &"a".repeat(31),
            &"g".repeat(32),
        ] {
            assert!(Pin::from_identity(&s.board_id, boot).is_none());
        }
    }
    #[test]
    fn queued_admission_is_revoked_after_boot_or_endpoint_change() {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut queue = Queue::new(tx);
        assert!(queue.observe(&status('a')));
        assert!(!queue.observe(&status('a')));
        let endpoint = crate::Endpoint {
            base: "http://ledboard.local".into(),
            user: "fixture".into(),
            password: "private".into(),
        };
        queue.send(HttpCommand::Status(endpoint.clone())).unwrap();
        let HttpCommand::Scoped(old, _) = rx.recv().unwrap() else {
            panic!()
        };
        assert!(old.admit().is_ok());
        assert!(queue.observe(&status('b')));
        assert!(!old.current());
        assert!(old.admit().is_err());
        queue.send(HttpCommand::Status(endpoint)).unwrap();
        let HttpCommand::Scoped(new, _) = rx.recv().unwrap() else {
            panic!()
        };
        assert!(new.admit().is_ok());
        queue.clear();
        assert!(!new.current());
        assert!(!queue.writable());
    }
    #[test]
    fn stale_queued_work_expires_but_inflight_receipts_do_not_expire() {
        let mut scope = Scope {
            auto_discovery: true,
            pin: Pin::from_status(&status('a')),
            generation: 1,
            clock: Arc::new(AtomicU64::new(1)),
            queued_at: Instant::now() - Duration::from_secs(9),
        };
        assert!(scope.admit().is_err());
        assert!(scope.admit_read().is_ok()); // A queued read is not a stale mutation.
        assert!(scope.current());
        scope.queued_at = Instant::now() - Duration::from_secs(25);
        assert!(scope.admit_read().is_err());
        scope.queued_at = Instant::now();
        assert!(scope.admit().is_ok());
        scope.clock.store(2, Ordering::Release);
        assert!(scope.admit().is_err());
        assert!(scope.admit_read().is_err());
    }
    #[test]
    fn retired_receipts_never_expose_another_peers_secrets() {
        let receipt = retired_receipt(&Event::OperationFailed(
            "private password error body".into(),
        ))
        .unwrap();
        assert!(!receipt.contains("password"));
        assert!(
            retired_receipt(&Event::Configuration(
                "/api/cloud",
                serde_json::json!({"private":"secret"})
            ))
            .is_none()
        );
    }
    #[test]
    fn retired_endpoint_receipts_are_kept_without_payload_or_credentials() {
        let scope = scope();
        let endpoint = crate::Endpoint {
            base: "http://previous.local".into(),
            user: "private-user".into(),
            password: "private-password".into(),
        };
        let event = Event::ForEndpoint(
            endpoint,
            Box::new(Event::ForScope(
                scope.clone(),
                Box::new(Event::OperationFailed("private response".into())),
            )),
        );
        assert!(retired_envelope(&event).is_none());
        scope.clock.fetch_add(1, Ordering::AcqRel);
        let receipt = retired_envelope(&event).unwrap();
        assert!(receipt.contains("Command failure"));
        for secret in [
            "previous.local",
            "private-user",
            "private-password",
            "private response",
        ] {
            assert!(!receipt.contains(secret));
        }
    }
    fn scope() -> Scope {
        Scope {
            auto_discovery: true,
            pin: Pin::from_status(&status('a')),
            generation: 0,
            clock: Arc::new(AtomicU64::new(0)),
            queued_at: Instant::now(),
        }
    }
    fn wire_status(boot: char, features: &[&str]) -> Vec<u8> {
        serde_json::to_vec(
            &serde_json::json!({"version":crate::peer_identity::VERSION,"hostname":"ledboard","board_id":"esp8266-aabbccddeeff",
            "boot_id":boot.to_string().repeat(32),"http_peer_fencing":true,
            "capabilities":{"board_kind":"table","acknowledged_features":features}}),
        )
        .unwrap()
    }
    fn client() -> Client {
        Client::builder()
            .no_proxy()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap()
    }
    fn wire_mapping(identity: Option<(&str, char)>) -> Vec<u8> {
        let mut value = serde_json::json!({
            "physical":(1_u8..=80).collect::<Vec<_>>(),
            "names":(1..=80).map(|id|id.to_string()).collect::<Vec<_>>()
        });
        if let Some((board, boot)) = identity {
            value["board_id"] = board.into();
            value["boot_id"] = boot.to_string().repeat(32).into();
        }
        serde_json::to_vec(&value).unwrap()
    }
    #[test]
    fn gui_mapping_enveloped_read_needs_no_http_status_preflight() {
        let (endpoint, server) = crate::tests::mock_device(vec![(
            200,
            wire_mapping(Some(("esp8266-aabbccddeeff", 'a'))),
        )]);
        let result =
            crate::scoped_http_request(&client(), &scope(), &HttpCommand::Mapping(endpoint, 73))
                .unwrap();
        assert!(matches!(result, Event::Mapping(73, cells, Some(_)) if cells.len()==80));
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].starts_with("GET /api/cells/mapping "));
        let request = requests[0].to_lowercase();
        assert!(request.contains("x-pakar-board: esp8266-aabbccddeeff"));
        assert!(request.contains(&format!("x-pakar-boot: {}", "a".repeat(32))));
    }
    #[test]
    fn gui_mapping_foreign_or_previous_boot_response_is_never_adopted() {
        for identity in [("other-board", 'a'), ("esp8266-aabbccddeeff", 'b')] {
            let (endpoint, server) =
                crate::tests::mock_device(vec![(200, wire_mapping(Some(identity)))]);
            let error = crate::scoped_http_request(
                &client(),
                &scope(),
                &HttpCommand::Mapping(endpoint, 74),
            )
            .err()
            .unwrap();
            assert!(error.contains("different board/boot"));
            let requests = server.join().unwrap();
            assert_eq!(
                requests.len(),
                1,
                "Foreign mapping cannot trigger an optimistic fallback"
            );
            assert!(requests[0].starts_with("GET /api/cells/mapping "));
        }
    }
    #[test]
    fn gui_mapping_incomplete_or_invalid_envelope_is_not_legacy() {
        for invalid in [
            serde_json::json!({"board_id":"esp8266-aabbccddeeff"}),
            serde_json::json!({"boot_id":"a".repeat(32)}),
        ] {
            let mut value: serde_json::Value = serde_json::from_slice(&wire_mapping(None)).unwrap();
            for (key, value_part) in invalid.as_object().unwrap() {
                value[key] = value_part.clone();
            }
            let (endpoint, server) =
                crate::tests::mock_device(vec![(200, serde_json::to_vec(&value).unwrap())]);
            let error = crate::scoped_http_request(
                &client(),
                &scope(),
                &HttpCommand::Mapping(endpoint, 75),
            )
            .err()
            .unwrap();
            assert!(error.contains("incomplete board/boot"));
            assert_eq!(server.join().unwrap().len(), 1);
        }
    }
    #[test]
    fn gui_mapping_revoked_inflight_result_is_not_applied() {
        let scope = scope();
        let clock = scope.clock.clone();
        let (endpoint, server) = crate::tests::mock_device_with_hook(
            vec![(200, wire_mapping(Some(("esp8266-aabbccddeeff", 'a'))))],
            Box::new(move |_| {
                clock.fetch_add(1, Ordering::AcqRel);
            }),
        );
        let error =
            crate::scoped_http_request(&client(), &scope, &HttpCommand::Mapping(endpoint, 76))
                .err()
                .unwrap();
        assert!(error.contains("revoked"));
        assert_eq!(server.join().unwrap().len(), 1);
    }
    #[test]
    fn gui_mapping_expired_queue_does_not_contact_device() {
        let mut scope = scope();
        scope.queued_at = Instant::now() - Duration::from_secs(25);
        let (endpoint, server) = crate::tests::mock_device(vec![]);
        assert!(
            crate::scoped_http_request(&client(), &scope, &HttpCommand::Mapping(endpoint, 77))
                .err()
                .unwrap()
                .contains("expired")
        );
        assert!(server.join().unwrap().is_empty());
    }
    #[test]
    fn gui_mapping_without_envelope_retains_both_legacy_status_fences() {
        for after in ['a', 'b'] {
            let (endpoint, server) = crate::tests::mock_device(vec![
                (200, wire_mapping(None)), // Unbound optimistic read is discarded.
                (200, wire_status('a', &["table"])),
                (200, wire_mapping(None)),
                (200, wire_status(after, &["table"])),
            ]);
            let result = crate::scoped_http_request(
                &client(),
                &scope(),
                &HttpCommand::Mapping(endpoint, 78),
            );
            if after == 'a' {
                assert!(matches!(result, Ok(Event::Mapping(78, cells, None)) if cells.len()==80));
            } else {
                assert!(result.err().unwrap().contains("changed during readback"));
            }
            let requests = server.join().unwrap();
            assert_eq!(requests.len(), 4);
            assert!(requests[0].starts_with("GET /api/cells/mapping "));
            assert!(requests[1].starts_with("GET /api/status "));
            assert!(requests[2].starts_with("GET /api/cells/mapping "));
            assert!(requests[3].starts_with("GET /api/status "));
        }
    }
    #[test]
    fn gui_settings_pin_every_nested_request_and_verify_same_boot() {
        let network = crate::tests::fixture_network();
        let s = wire_status('a', &["network"]);
        let (endpoint, server) = crate::tests::mock_device(vec![
            (200, s.clone()),
            (202, br#"{"accepted":true}"#.to_vec()),
            (200, serde_json::to_vec(&network).unwrap()),
            (200, s),
        ]);
        let write =
            crate::device_settings::Write::network(41, &network, "fixture-private", false).unwrap();
        let result = crate::scoped_http_request(
            &client(),
            &scope(),
            &HttpCommand::Settings(endpoint, write),
        )
        .unwrap();
        assert!(matches!(result, Event::SettingsFinished(41, _, Ok(_))));
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 4);
        assert!(requests[1].starts_with("POST /api/network "));
        assert!(requests[2].starts_with("GET /api/network "));
        for request in &requests[1..3] {
            let request = request.to_lowercase();
            assert!(request.contains("x-pakar-board: esp8266-aabbccddeeff"));
            assert!(request.contains(&format!("x-pakar-boot: {}", "a".repeat(32))));
        }
    }
    #[test]
    fn gui_preflight_reboot_or_capability_loss_never_dispatches_mutation() {
        for response in [wire_status('b', &["leds"]), wire_status('a', &[])] {
            let (endpoint, server) = crate::tests::mock_device(vec![(200, response)]);
            assert!(
                crate::scoped_http_request(
                    &client(),
                    &scope(),
                    &HttpCommand::Post(endpoint, "/api/brightness", vec![])
                )
                .is_err()
            );
            let requests = server.join().unwrap();
            assert_eq!(requests.len(), 1);
            assert!(requests[0].starts_with("GET /api/status "));
        }
    }
    #[test]
    fn gui_server_boot_conflict_is_not_replayed() {
        let (endpoint, server) = crate::tests::mock_device(vec![
            (200, wire_status('a', &["leds"])),
            (409, br#"{"error":"peer_changed"}"#.to_vec()),
        ]);
        assert!(
            crate::scoped_http_request(
                &client(),
                &scope(),
                &HttpCommand::Post(endpoint, "/api/brightness", vec![])
            )
            .is_err()
        );
        assert_eq!(server.join().unwrap().len(), 2);
    }
    #[test]
    fn gui_readback_from_new_boot_is_not_applied() {
        let (endpoint, server) = crate::tests::mock_device(vec![
            (200, wire_status('a', &[])),
            (200, b"{}".to_vec()),
            (200, wire_status('b', &[])),
        ]);
        let error = crate::scoped_http_request(
            &client(),
            &scope(),
            &HttpCommand::Configuration(endpoint, "/api/network"),
        )
        .err()
        .unwrap();
        assert!(error.contains("changed during readback"));
        assert_eq!(server.join().unwrap().len(), 3);
    }
    #[test]
    fn gui_revocation_during_preflight_stops_before_write() {
        let scope = scope();
        let clock = scope.clock.clone();
        let (endpoint, server) = crate::tests::mock_device_with_hook(
            vec![(200, wire_status('a', &["leds"]))],
            Box::new(move |_| {
                clock.fetch_add(1, Ordering::AcqRel);
            }),
        );
        let error = crate::scoped_http_request(
            &client(),
            &scope,
            &HttpCommand::Post(endpoint, "/api/brightness", vec![]),
        )
        .err()
        .unwrap();
        assert!(error.contains("before dispatch"));
        assert_eq!(server.join().unwrap().len(), 1);
    }
    #[test]
    fn gui_revocation_after_dispatch_reports_uncertainty_not_success() {
        let scope = scope();
        let clock = scope.clock.clone();
        let (endpoint, server) = crate::tests::mock_device_with_hook(
            vec![(200, wire_status('a', &["leds"])), (202, b"{}".to_vec())],
            Box::new(move |n| {
                if n == 2 {
                    clock.fetch_add(1, Ordering::AcqRel);
                }
            }),
        );
        let error = crate::scoped_http_request(
            &client(),
            &scope,
            &HttpCommand::Post(endpoint, "/api/brightness", vec![]),
        )
        .err()
        .unwrap();
        assert!(error.contains("outcome unknown"));
        assert_eq!(server.join().unwrap().len(), 2);
    }
    #[test]
    fn gui_unfenced_mutation_fails_before_network_and_preserves_correlation() {
        let mut scope = scope();
        scope.pin = None;
        let endpoint = crate::Endpoint {
            base: "http://127.0.0.1:1".into(),
            user: "fixture".into(),
            password: "private".into(),
        };
        let write = crate::device_settings::Write::network(
            41,
            &crate::tests::fixture_network(),
            "fixture-private",
            false,
        )
        .unwrap();
        let command = HttpCommand::Scoped(scope, Box::new(HttpCommand::Settings(endpoint, write)));
        let error = crate::http_request(&client(), command.clone())
            .err()
            .unwrap();
        assert!(error.starts_with("Read-only:"));
        let Event::ForScope(_, inner) = command.failure(error) else {
            panic!()
        };
        assert!(matches!(*inner, Event::SettingsFinished(41, _, Err(_))));
        assert!(!inner.http_succeeded());
    }
}
