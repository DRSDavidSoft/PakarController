//! Runtime-only external lighting input. Adapted from PCController's bounded
//! receiver/serial owner; output stays in Pakar's existing WebSocket worker.
pub mod protocol;
#[cfg(test)]
mod tests;
pub mod ui;
use crate::{CellMapping, DeviceStatus, peer_http::Pin, pixel_groups::Layout};
use std::{
    io::Read,
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct Config {
    pub protocol: String,
    pub listen: String,
    pub source: String,
    pub serial: String,
    pub baud: u32,
    pub pixels: usize,
    pub universe: u16,
    pub channel: usize,
    pub timeout_secs: u64,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            protocol: "ddp".into(),
            listen: protocol::endpoint("ddp"),
            source: String::new(),
            serial: String::new(),
            baud: 115200,
            pixels: 480,
            universe: 1,
            channel: 1,
            timeout_secs: 5,
        }
    }
}
impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if !protocol::PROTOCOLS.contains(&self.protocol.as_str())
            || !(1..=480).contains(&self.pixels)
            || !(1..=60).contains(&self.timeout_secs)
        {
            return Err("Choose a supported protocol, 1–480 pixels and a 1–60s timeout".into());
        }
        if matches!(self.protocol.as_str(), "artnet" | "sacn")
            && (self.pixels > 170
                || self.channel == 0
                || self.channel > 513 - self.pixels * 3
                || if self.protocol == "sacn" {
                    !(1..=63999).contains(&self.universe)
                } else {
                    self.universe > 32767
                })
        {
            return Err(
                "DMX needs one universe: select 80 logical cells and a fitting channel window"
                    .into(),
            );
        }
        if !self.serial.is_empty()
            && (!protocol::serial_supported(&self.protocol)
                || !(1200..=2_000_000).contains(&self.baud)
                || !self.listen.is_empty()
                || !self.source.is_empty())
        {
            return Err(
                "Serial requires OpenRGB, Adalight or TPM2; no UDP endpoint; baud 1200–2000000"
                    .into(),
            );
        }
        Ok(())
    }
}
#[derive(Clone, Default)]
pub struct Status {
    pub running: bool,
    pub pending: bool,
    pub received: u64,
    pub rejected: u64,
    pub coalesced: u64,
    pub sent: u64,
    pub sender: String,
    pub error: String,
}
#[derive(Default)]
pub struct Shared {
    pub status: Status,
    latest: Option<(Instant, Vec<u8>)>,
    last: Option<Instant>,
    failure: Option<String>,
}
pub type View = Arc<Mutex<Shared>>;
pub fn snapshot(view: &View) -> Status {
    view.lock().unwrap().status.clone()
}
pub fn reject(request: Request, message: &str) {
    let mut s = request.view.lock().unwrap();
    s.status.pending = false;
    s.status.running = false;
    s.status.error = message.into();
}

pub struct Request {
    pub config: Config,
    pub peer: Pin,
    pub view: View,
    pub at: Instant,
}
pub fn ready(status: &DeviceStatus) -> bool {
    status.pixels == 480
        && crate::pixel_diagnostic::ready(status)
        && status
            .capabilities
            .as_ref()
            .is_some_and(|c| c.table() && c.has("streaming"))
}
pub struct Input {
    pub peer: Pin,
    pub config: Config,
    view: View,
    stop: Arc<AtomicBool>,
    reader: Option<thread::JoinHandle<()>>,
    started: Instant,
    mapping: Vec<CellMapping>,
    layout: Option<Layout>,
}
enum Source {
    Udp(UdpSocket, Option<IpAddr>),
    Serial(Box<dyn serialport::SerialPort>),
}
fn valid_ip(ip: IpAddr) -> bool {
    !ip.is_unspecified() && !ip.is_multicast() && ip != IpAddr::V4(std::net::Ipv4Addr::BROADCAST)
}
impl Input {
    pub fn open(
        request: Request,
        mapping: &[CellMapping],
        layout: Option<&Layout>,
    ) -> Result<Self, String> {
        request.config.validate()?;
        if request.config.pixels != 480
            && (request.config.pixels != 80
                || !crate::streaming::valid_mapping(mapping)
                || layout.is_none())
        {
            return Err(
                "80-cell input needs this board's complete mapping and physical layout".into(),
            );
        }
        if let Some(layout) = layout {
            layout.validate()?;
        }
        let config = request.config;
        let source = if config.serial.is_empty() {
            let address: SocketAddr = config
                .listen
                .parse()
                .map_err(|_| "Listen must be a specific local IP:port")?;
            if address.port() == 0 || !valid_ip(address.ip()) {
                return Err("Wildcard/multicast listen addresses are not permitted".into());
            }
            let allowed = if config.source.is_empty() {
                None
            } else {
                Some(
                    config
                        .source
                        .parse::<IpAddr>()
                        .map_err(|_| "Trusted sender must be an IP address")?,
                )
            };
            if allowed.is_some_and(|ip| !valid_ip(ip))
                || (!address.ip().is_loopback() && allowed.is_none())
            {
                return Err("LAN input requires one explicit trusted unicast sender IP".into());
            }
            let socket = UdpSocket::bind(address).map_err(|e| e.to_string())?;
            socket
                .set_read_timeout(Some(Duration::from_millis(50)))
                .map_err(|e| e.to_string())?;
            Source::Udp(socket, allowed)
        } else {
            let port = serialport::new(&config.serial, config.baud)
                .timeout(Duration::from_millis(50))
                .dtr_on_open(false)
                .open()
                .map_err(|e| e.to_string())?;
            Source::Serial(port)
        };
        let view = request.view;
        let stop = Arc::new(AtomicBool::new(false));
        let (worker_view, worker_stop, worker_config) =
            (view.clone(), stop.clone(), config.clone());
        let reader = thread::Builder::new()
            .name("rgb-input".into())
            .spawn(move || read(source, worker_config, worker_view, worker_stop))
            .map_err(|e| e.to_string())?;
        {
            let mut s = view.lock().unwrap();
            s.status.pending = false;
            s.status.running = true;
        }
        Ok(Self {
            peer: request.peer,
            config,
            view,
            stop,
            reader: Some(reader),
            started: Instant::now(),
            mapping: mapping.to_vec(),
            layout: layout.cloned(),
        })
    }
    pub fn topology_current(&self, mapping: &[CellMapping], layout: Option<&Layout>) -> bool {
        self.config.pixels == 480
            || (layout == self.layout.as_ref()
                && mapping.len() == self.mapping.len()
                && mapping.iter().zip(&self.mapping).all(|(a, b)| {
                    a.logical_id == b.logical_id && a.physical_cell == b.physical_cell
                }))
    }
    pub fn poll(&self, take: bool) -> Result<Option<Vec<u8>>, String> {
        let mut state = self.view.lock().unwrap();
        if let Some(error) = state.failure.as_ref() {
            return Err(error.clone());
        }
        if state
            .last
            .is_some_and(|at| at.elapsed() > Duration::from_secs(self.config.timeout_secs))
        {
            return Err("RGB sender timed out; input stopped without replay".into());
        }
        if state.last.is_none() && self.started.elapsed() > Duration::from_secs(60) {
            return Err("No valid RGB frame received in 60s".into());
        }
        if !take {
            return Ok(None);
        }
        let Some((at, rgb)) = state.latest.take() else {
            return Ok(None);
        };
        if at.elapsed() > Duration::from_secs(self.config.timeout_secs) {
            return Ok(None);
        }
        drop(state);
        if self.config.pixels == 480 {
            return Ok(Some(rgb));
        }
        let layout = self.layout.as_ref().ok_or("Physical layout unavailable")?;
        let mut output = vec![0; 1440];
        for cell in &self.mapping {
            let (start, count) = layout
                .span(cell.physical_cell)
                .ok_or("Physical cell not in layout")?;
            let color = usize::from(cell.logical_id - 1) * 3;
            for pixel in usize::from(start)..usize::from(start) + usize::from(count) {
                output[pixel * 3..pixel * 3 + 3].copy_from_slice(&rgb[color..color + 3]);
            }
        }
        Ok(Some(output))
    }
    pub fn fail(&self, error: impl Into<String>) {
        self.view.lock().unwrap().status.error = error.into();
    }
    pub fn sent(&self) {
        self.view.lock().unwrap().status.sent += 1;
    }
}
impl Drop for Input {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
        let mut state = self.view.lock().unwrap();
        state.status.running = false;
        state.status.pending = false;
        state.latest = None;
    }
}
fn read(mut source: Source, c: Config, view: View, stop: Arc<AtomicBool>) {
    let mut buffer = vec![0; 65536];
    let mut owner = None;
    let mut order = protocol::Order::default();
    let mut framer = protocol::Framer::new(&c);
    while !stop.load(Ordering::Acquire) {
        let incoming = match &mut source {
            Source::Udp(socket, allowed) => socket.recv_from(&mut buffer).map(|(n, peer)| {
                (
                    n,
                    peer.to_string(),
                    allowed.map_or(peer.ip().is_loopback(), |ip| ip == peer.ip()),
                    false,
                )
            }),
            Source::Serial(port) => port
                .read(&mut buffer[..512])
                .map(|n| (n, c.serial.clone(), true, true)),
        };
        let (n, peer, permitted, serial) = match incoming {
            Ok(value) => value,
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                ) =>
            {
                framer.clear();
                continue;
            }
            Err(e) => {
                view.lock().unwrap().failure = Some(e.to_string());
                return;
            }
        };
        if n == 0 {
            framer.clear();
            continue;
        }
        if !permitted || owner.as_ref().is_some_and(|value| value != &peer) {
            view.lock().unwrap().status.rejected += 1;
            continue;
        }
        let mut accept = |packet: Option<protocol::Packet>| -> bool {
            let mut s = view.lock().unwrap();
            let Some(packet) = packet else {
                s.status.rejected += 1;
                return true;
            };
            if (packet.ended && owner.is_none()) || !order.accept(&packet) {
                s.status.rejected += 1;
                return true;
            }
            if packet.ended {
                s.failure = Some("sACN sender ended the stream".into());
                return false;
            }
            owner = Some(peer.clone());
            s.status.sender = peer.clone();
            s.status.received += 1;
            let at = Instant::now();
            s.last = Some(at);
            if s.latest.replace((at, packet.rgb)).is_some() {
                s.status.coalesced += 1;
            }
            true
        };
        if serial {
            for b in &buffer[..n] {
                if let Some(packet) = framer.push(&c, *b) {
                    if !accept(Some(packet)) {
                        return;
                    }
                }
            }
        } else if !accept(protocol::decode(&c, &buffer[..n])) {
            return;
        }
    }
}

pub fn same_serial(a: &str, b: &str) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }
    #[cfg(windows)]
    {
        let normalize = |p: &str| {
            p.trim()
                .trim_start_matches("\\\\.\\")
                .trim_start_matches("\\\\?\\")
                .trim_end_matches(':')
                .to_ascii_lowercase()
        };
        normalize(a) == normalize(b)
    }
    #[cfg(not(windows))]
    {
        std::fs::canonicalize(a)
            .ok()
            .zip(std::fs::canonicalize(b).ok())
            .is_some_and(|(a, b)| a == b)
            || a == b
    }
}
