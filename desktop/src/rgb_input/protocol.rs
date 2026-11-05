//! Adapted from PCController internal/rgbinput/{protocol,network_protocols}.go.
//! Complete RGB snapshots only; never controller commands or partial commits.
use super::Config;

pub const PROTOCOLS: &[&str] = &[
    "openrgb",
    "ddp",
    "tpm2net",
    "adalight",
    "adalight-openrgb",
    "tpm2",
    "artnet",
    "sacn",
    "wled",
    "hyperion",
];
pub fn endpoint(protocol: &str) -> String {
    format!(
        "127.0.0.1:{}",
        match protocol {
            "ddp" => 4048,
            "artnet" => 6454,
            "sacn" => 5568,
            "wled" => 21324,
            "hyperion" => 19446,
            _ => 65506,
        }
    )
}
pub fn serial_supported(protocol: &str) -> bool {
    matches!(
        protocol,
        "openrgb" | "adalight" | "adalight-openrgb" | "tpm2"
    )
}
#[derive(Debug, Default)]
pub struct Packet {
    pub rgb: Vec<u8>,
    identity: Vec<u8>,
    sequence: u8,
    modulo: u16,
    pub ended: bool,
}
fn be(p: &[u8], i: usize) -> usize {
    usize::from(u16::from_be_bytes([p[i], p[i + 1]]))
}
fn pdu(p: &[u8], i: usize) -> bool {
    let v = be(p, i);
    v & 0xf000 == 0x7000 && v & 0xfff == p.len() - i
}

pub fn decode(c: &Config, p: &[u8]) -> Option<Packet> {
    if !(1..=480).contains(&c.pixels) {
        return None;
    }
    let n = c.pixels * 3;
    let mut out = Packet::default();
    let data = match c.protocol.as_str() {
        "openrgb" => {
            if p.len() != n + 3 || p[0] != 0xaa {
                return None;
            }
            let sum = p[..n + 1]
                .iter()
                .fold(0u16, |s, b| s.wrapping_add(u16::from(*b)));
            if be(p, n + 1) != usize::from(sum) {
                return None;
            }
            &p[1..n + 1]
        }
        "adalight" | "adalight-openrgb" => {
            let count = if c.protocol == "adalight" {
                c.pixels - 1
            } else {
                c.pixels
            };
            if p.len() != n + 6
                || &p[..3] != b"Ada"
                || be(p, 3) != count
                || p[5] != p[3] ^ p[4] ^ 0x55
            {
                return None;
            }
            &p[6..]
        }
        "tpm2" => {
            if p.len() != n + 5 || p[..2] != [0xc9, 0xda] || be(p, 2) != n || p[n + 4] != 0x36 {
                return None;
            }
            &p[4..n + 4]
        }
        "tpm2net" => {
            if p.len() != n + 7
                || p[..2] != [0x9c, 0xda]
                || be(p, 2) != n
                || p[4..6] != [1, 1]
                || p[n + 6] != 0x36
            {
                return None;
            }
            &p[6..n + 6]
        }
        "ddp" => {
            if p.len() != n + 10
                || p[0] != 0x41
                || p[1] & 0xf0 != 0
                || !matches!(p[2], 0 | 1 | 0x0b)
                || p[3] != 1
                || p[4..8] != [0; 4]
                || be(p, 8) != n
            {
                return None;
            }
            out.sequence = p[1];
            out.modulo = if p[1] == 0 { 0 } else { 16 };
            &p[10..]
        }
        "artnet" => {
            if n > 512
                || c.channel == 0
                || c.channel > 513 - n
                || c.universe > 32767
                || p.len() < 20
                || &p[..8] != b"Art-Net\0"
                || p[8..10] != [0, 0x50]
                || be(p, 10) < 14
                || u16::from_le_bytes([p[14], p[15]]) != c.universe
            {
                return None;
            }
            let len = be(p, 16);
            let start = c.channel - 1;
            if !(2..=512).contains(&len) || len % 2 != 0 || p.len() != 18 + len || start + n > len {
                return None;
            }
            out.identity = vec![p[13]];
            out.sequence = p[12];
            out.modulo = if p[12] == 0 { 0 } else { 256 };
            &p[18 + start..18 + start + n]
        }
        "sacn" => {
            if n > 512
                || c.channel == 0
                || c.channel > 513 - n
                || !(1..=63999).contains(&c.universe)
                || p.len() < 126
                || be(p, 0) != 16
                || be(p, 2) != 0
                || &p[4..16] != b"ASC-E1.17\0\0\0"
                || !pdu(p, 16)
                || !pdu(p, 38)
                || !pdu(p, 115)
                || p[18..22] != [0, 0, 0, 4]
                || p[40..44] != [0, 0, 0, 2]
                || p[108] > 200
                || be(p, 109) != 0
                || p[112] & !0x40 != 0
                || be(p, 113) != usize::from(c.universe)
                || p[117] != 2
                || p[118] != 0xa1
                || be(p, 119) != 0
                || be(p, 121) != 1
                || p[125] != 0
            {
                return None;
            }
            let count = be(p, 123);
            let start = c.channel - 1;
            if !(1..=513).contains(&count) || p.len() != 125 + count {
                return None;
            }
            out.identity = p[22..38].to_vec();
            out.sequence = p[111];
            out.modulo = 256;
            out.ended = p[112] & 0x40 != 0;
            if out.ended {
                return Some(out);
            }
            if start + n > count - 1 {
                return None;
            }
            &p[126 + start..126 + start + n]
        }
        "wled" => {
            if p.len() < 2 || p[1] == 0 {
                return None;
            }
            match p[0] {
                2 if p.len() == n + 2 => &p[2..],
                4 if p.len() == n + 4 && be(p, 2) == 0 => &p[4..],
                1 if c.pixels <= 256 && p.len() == 2 + c.pixels * 4 => {
                    let mut seen = vec![false; c.pixels];
                    out.rgb = vec![0; n];
                    for item in p[2..].chunks_exact(4) {
                        let index = usize::from(item[0]);
                        if index >= c.pixels || seen[index] {
                            return None;
                        }
                        seen[index] = true;
                        out.rgb[index * 3..index * 3 + 3].copy_from_slice(&item[1..]);
                    }
                    return Some(out);
                }
                _ => return None,
            }
        }
        "hyperion" if p.len() == n => p,
        _ => return None,
    };
    out.rgb = data.to_vec();
    Some(out)
}

#[derive(Default)]
pub struct Order {
    last: Option<(Vec<u8>, u8, u16)>,
}
impl Order {
    pub fn accept(&mut self, p: &Packet) -> bool {
        if let Some((id, seq, modulo)) = &self.last {
            if id != &p.identity {
                return false;
            }
            if p.modulo != 0 && *modulo == p.modulo {
                let delta = (u16::from(p.sequence) + modulo - u16::from(*seq)) % modulo;
                if delta == 0 || delta > modulo / 2 {
                    return false;
                }
            }
        }
        self.last = Some((p.identity.clone(), p.sequence, p.modulo));
        true
    }
}

/// Never allocates from an untrusted wire length. Resynchronizes after noise.
pub struct Framer {
    pending: Vec<u8>,
    size: usize,
}
impl Framer {
    pub fn new(c: &Config) -> Self {
        let size = c.pixels * 3
            + match c.protocol.as_str() {
                "openrgb" => 3,
                "tpm2" => 5,
                _ => 6,
            };
        Self {
            pending: Vec::with_capacity(size),
            size,
        }
    }
    pub fn clear(&mut self) {
        self.pending.clear();
    }
    pub fn push(&mut self, c: &Config, b: u8) -> Option<Packet> {
        self.pending.push(b);
        if self.pending.len() < self.size {
            return None;
        }
        if let Some(p) = decode(c, &self.pending) {
            self.pending.clear();
            Some(p)
        } else {
            self.pending.remove(0);
            None
        }
    }
}
