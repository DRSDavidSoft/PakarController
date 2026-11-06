//! Read-only standalone discovery. Existing bridge/device owners retain all
//! control and update admission. One sweep, bounded origins, no detached poller.
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    io::Read,
    sync::mpsc,
    time::{Duration, Instant},
};

#[derive(Clone, Debug)]
pub struct Peer {
    pub address: Option<std::net::SocketAddr>,
    pub endpoint: String,
    pub identity: Value,
    pub observed: Instant,
}
impl Peer {
    pub fn key(&self) -> String {
        let board = self.identity["board_id"].as_str().unwrap_or("");
        let id = self.identity["instance_id"]
            .as_str()
            .or_else(|| self.identity["id"].as_str())
            .unwrap_or("");
        if !board.is_empty() {
            format!("board:{board}")
        } else {
            format!("app:{id}")
        }
    }
    pub fn name(&self) -> &str {
        self.identity["name"]
            .as_str()
            .or_else(|| self.identity["hostname"].as_str())
            .unwrap_or("PakarController")
    }
    pub fn board(&self) -> bool {
        self.identity["kind"].as_str().unwrap_or("board") == "board"
    }
}
pub fn parse_identity(
    endpoint: &str,
    value: Value,
    board_header: Option<&str>,
    boot_header: Option<&str>,
) -> Option<Peer> {
    let product = value["product"]
        .as_str()
        .or_else(|| value["application"].as_str());
    if product != Some("PakarController") {
        return None;
    }
    let kind = value["kind"].as_str().unwrap_or("board");
    let valid_text = |v: Option<&str>| {
        v.is_some_and(|s| {
            !s.is_empty() && s.len() <= 80 && s.bytes().all(|b| (33..=126).contains(&b))
        })
    };
    match kind {
        "board" => {
            let board = value["board_id"].as_str();
            let boot = value["boot_id"].as_str();
            if !board.zip(boot).is_some_and(|(board, boot)| {
                crate::peer_http::Pin::from_identity(board, boot).is_some()
            }) || board_header != board
                || boot_header != boot
            {
                return None;
            }
        }
        "bridge" | "desktop-app" | "android-app" => {
            if !valid_text(
                value["instance_id"]
                    .as_str()
                    .or_else(|| value["id"].as_str()),
            ) {
                return None;
            }
        }
        _ => return None,
    }
    Some(Peer {
        address: None,
        endpoint: endpoint.into(),
        identity: value,
        observed: Instant::now(),
    })
}
pub fn probe(endpoint: &str) -> Option<Peer> {
    let url = reqwest::Url::parse(endpoint).ok()?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !matches!(url.path(), "" | "/")
    {
        return None;
    }
    let client = reqwest::blocking::Client::builder()
        .user_agent(format!(
            "PakarController/{} (peer-discovery)",
            crate::peer_identity::VERSION
        ))
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_millis(1500))
        .build()
        .ok()?;
    // Public metadata must not be version-gated; the normal command client
    // supplies its release only after the selected peer admits that session.
    let response = client
        .get(format!("{}/api/peer/info", endpoint.trim_end_matches('/')))
        .send()
        .ok()?;
    if !response.status().is_success() {
        return None;
    }
    let address = response.remote_addr();
    let board = response
        .headers()
        .get("x-pakar-board")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    let boot = response
        .headers()
        .get("x-pakar-boot")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    let mut body = Vec::new();
    response.take(16385).read_to_end(&mut body).ok()?;
    if body.len() > 16384 {
        return None;
    }
    let mut peer = parse_identity(
        endpoint,
        serde_json::from_slice(&body).ok()?,
        board.as_deref(),
        boot.as_deref(),
    )?;
    peer.address = address;
    Some(peer)
}
pub fn board_source(peer: &Peer, role: &str, version: &str) -> Option<Value> {
    let address = peer.address?;
    if !peer.board()
        || peer.identity["role"] != role
        || address.port() != 80
        || !matches!(address.ip(),std::net::IpAddr::V4(ip)if ip.is_private())
    {
        return None;
    }
    let a = peer.identity["artifacts"]
        .as_array()?
        .iter()
        .find(|a| a["path"] == "/firmware/download")?;
    if !crate::peer_identity::release_order(a["version"].as_str()?, version)?.is_gt()
        || !crate::maintenance::valid_hash(a["sha256"].as_str()?)
        || a["bytes"].as_u64()? < 24
        || a["family"].as_str()?.is_empty()
    {
        return None;
    }
    let mut hostname = peer.identity["hostname"].as_str()?.to_owned();
    if !hostname.ends_with(".local") {
        hostname.push_str(".local");
    }
    let name = hostname.strip_suffix(".local")?;
    if crate::identity::normalize_name(name).ok().as_deref() != Some(name) {
        return None;
    }
    Some(
        json!({"hostname":hostname,"address":address.ip().to_string(),"board_id":peer.identity["board_id"],"boot_id":peer.identity["boot_id"],"artifact":a}),
    )
}
#[derive(Default)]
pub struct Workspace {
    pub started: bool,
    pub peers: BTreeMap<String, Peer>,
    pub endpoint: String,
    pub error: String,
    receiver: Option<mpsc::Receiver<Vec<Peer>>>,
}
impl Workspace {
    pub fn busy(&self) -> bool {
        self.receiver.is_some()
    }
    pub fn discover(&mut self, manual: Option<String>) {
        if self.busy() {
            return;
        }
        self.error.clear();
        self.started = true;
        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        std::thread::spawn(move || {
            let origins = manual.map(|p| vec![p]).unwrap_or_else(|| {
                crate::platform::discovery::board_locations(Duration::from_millis(1400))
            });
            let queue = std::sync::Mutex::new(
                origins
                    .into_iter()
                    .take(16)
                    .collect::<std::collections::VecDeque<_>>(),
            );
            let rows = std::sync::Mutex::new(Vec::new());
            let deadline = Instant::now() + Duration::from_secs(9);
            std::thread::scope(|scope| {
                for _ in 0..4 {
                    let queue = &queue;
                    let rows = &rows;
                    scope.spawn(move || {
                        while Instant::now() < deadline {
                            let Some(origin) = queue.lock().unwrap().pop_front() else {
                                break;
                            };
                            if let Some(peer) = probe(&origin) {
                                rows.lock().unwrap().push(peer);
                            }
                        }
                    });
                }
            });
            let peers = rows.into_inner().unwrap();
            let _ = tx.send(peers);
        });
    }
    pub fn service(&mut self) {
        let Some(receiver) = &self.receiver else {
            return;
        };
        match receiver.try_recv() {
            Ok(peers) => {
                if peers.is_empty() {
                    self.error = "No peers reached in this sweep".into();
                }
                for peer in peers {
                    self.peers.insert(peer.key(), peer);
                }
                self.receiver = None;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                self.error = "Peer discovery stopped".into();
                self.receiver = None;
            }
            Err(mpsc::TryRecvError::Empty) => {}
        }
        while self.peers.len() > 128 {
            let key = self
                .peers
                .iter()
                .min_by_key(|(_, p)| p.observed)
                .map(|(k, _)| k.clone())
                .unwrap();
            self.peers.remove(&key);
        }
    }
}
pub fn public_info(instance: &str, hostname: &str, bridge_enabled: bool) -> Value {
    json!({"product":"PakarController","kind":"desktop-app","role":"controller","id":instance,"instance_id":instance,
        "session_id":crate::peer_identity::session_id(),"version":crate::peer_identity::VERSION,"hostname":hostname,
        "compatibility":crate::peer_identity::advertised_policy(),"exchange":{"passive_identity":true,"bridge_enabled":bridge_enabled,"automatic_apply":false,"native_package_sharing":false},
        "artifacts":[],"client":crate::client_telemetry::Reporter::new().value_for_transport("bridge")})
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn app_peer_needs_no_board_and_release_gate_does_not_hide_identity() {
        let value = json!({"product":"PakarController","kind":"desktop-app","instance_id":"test-app","version":"99.0.0","exchange":{"automatic_check":false}});
        let peer = parse_identity("http://127.0.0.1:8789", value, None, None).unwrap();
        assert!(!peer.board());
        assert_eq!(peer.key(), "app:test-app");
    }
    #[test]
    fn board_requires_matching_body_and_transport_identity() {
        let boot = "0123456789abcdef0123456789abcdef";
        let value = json!({"product":"PakarController","kind":"board","board_id":"board-one","boot_id":boot});
        assert!(
            parse_identity(
                "http://board.local",
                value.clone(),
                Some("board-one"),
                Some(boot)
            )
            .is_some()
        );
        assert!(
            parse_identity("http://board.local", value, Some("board-two"), Some(boot)).is_none()
        );
    }
    #[test]
    fn native_passive_service_does_not_claim_an_unavailable_installer() {
        let info = public_info("app", "host", false);
        assert_eq!(info["artifacts"], json!([]));
        assert_eq!(info["exchange"]["bridge_enabled"], false);
    }
    #[test]
    fn board_transfer_uses_observed_private_address_and_newer_same_role_only() {
        let boot = "0123456789abcdef0123456789abcdef";
        let value = json!({"product":"PakarController","kind":"board","board_id":"source-board","boot_id":boot,
            "role":"table","hostname":"source-board","artifacts":[{"family":"same-pin-build","version":"0.2.9","sha256":"a".repeat(64),"bytes":4096,"path":"/firmware/download"}]});
        let mut peer = parse_identity(
            "http://source-board.local",
            value,
            Some("source-board"),
            Some(boot),
        )
        .unwrap();
        assert!(board_source(&peer, "table", "0.2.8").is_none()); // An advert IP is not a dialed identity.
        peer.address = Some("192.168.2.3:80".parse().unwrap());
        let source = board_source(&peer, "table", "0.2.8").unwrap();
        assert_eq!(source["address"], "192.168.2.3");
        assert_eq!(source["hostname"], "source-board.local");
        for (role, version) in [
            ("first-wins", "0.2.8"),
            ("table", "0.2.9"),
            ("table", "0.2.10"),
        ] {
            assert!(board_source(&peer, role, version).is_none());
        }
        for address in ["8.8.8.8:80", "127.0.0.1:80", "192.168.2.3:8789"] {
            peer.address = Some(address.parse().unwrap());
            assert!(board_source(&peer, "table", "0.2.8").is_none());
        }
    }
}
