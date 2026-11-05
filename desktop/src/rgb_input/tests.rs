//! Actual donor packet/truncation/order/framer fixtures adapted to 480 pixels.
use super::{protocol::*, *};
fn config(protocol: &str, pixels: usize) -> Config {
    Config {
        protocol: protocol.into(),
        pixels,
        channel: 7,
        ..Config::default()
    }
}
fn fixture(c: &Config) -> Vec<u8> {
    let rgb = vec![17, 128, 250].repeat(c.pixels);
    let n = rgb.len();
    let mut p = match c.protocol.as_str() {
        "openrgb" => vec![0xaa],
        "adalight" | "adalight-openrgb" => {
            let count = if c.protocol == "adalight" {
                c.pixels - 1
            } else {
                c.pixels
            };
            vec![
                b'A',
                b'd',
                b'a',
                (count >> 8) as u8,
                count as u8,
                (count >> 8) as u8 ^ count as u8 ^ 0x55,
            ]
        }
        "tpm2" => vec![0xc9, 0xda, (n >> 8) as u8, n as u8],
        "tpm2net" => vec![0x9c, 0xda, (n >> 8) as u8, n as u8, 1, 1],
        "ddp" => vec![0x41, 1, 0x0b, 1, 0, 0, 0, 0, (n >> 8) as u8, n as u8],
        "wled" => vec![2, 2],
        "hyperion" => vec![],
        "artnet" => {
            let len = (c.channel - 1 + n + 1) & !1;
            let mut p = vec![0; 18 + len];
            p[..8].copy_from_slice(b"Art-Net\0");
            p[9] = 0x50;
            p[11] = 14;
            p[12] = 1;
            p[14] = 1;
            p[16..18].copy_from_slice(&(len as u16).to_be_bytes());
            p[18 + c.channel - 1..18 + c.channel - 1 + n].copy_from_slice(&rgb);
            return p;
        }
        "sacn" => {
            let mut p = vec![0; 126 + c.channel - 1 + n];
            p[1] = 16;
            p[4..16].copy_from_slice(b"ASC-E1.17\0\0\0");
            for i in [16, 38, 115] {
                let len = p.len() - i;
                p[i..i + 2].copy_from_slice(&(0x7000 | len as u16).to_be_bytes());
            }
            p[21] = 4;
            p[22] = 7;
            p[43] = 2;
            p[108] = 100;
            p[111] = 1;
            p[114] = 1;
            p[117] = 2;
            p[118] = 0xa1;
            p[122] = 1;
            let count = p.len() - 125;
            p[123..125].copy_from_slice(&(count as u16).to_be_bytes());
            p[126 + c.channel - 1..].copy_from_slice(&rgb);
            return p;
        }
        _ => panic!(),
    };
    p.extend(rgb);
    if c.protocol == "openrgb" {
        let sum = p.iter().fold(0u16, |s, b| s.wrapping_add(u16::from(*b)));
        p.extend(sum.to_be_bytes());
    }
    if matches!(c.protocol.as_str(), "tpm2" | "tpm2net") {
        p.push(0x36);
    }
    p
}
#[test]
fn complete_frames_and_every_truncation() {
    for protocol in PROTOCOLS {
        for pixels in [1, 80, 480] {
            if pixels == 480 && matches!(*protocol, "artnet" | "sacn") {
                continue;
            }
            let c = config(protocol, pixels);
            let p = fixture(&c);
            assert_eq!(
                decode(&c, &p).unwrap().rgb,
                vec![17, 128, 250].repeat(pixels)
            );
            for end in 0..p.len() {
                assert!(decode(&c, &p[..end]).is_none(), "{protocol}/{pixels}/{end}");
            }
            let mut extra = p.clone();
            extra.push(0);
            assert!(decode(&c, &extra).is_none());
        }
    }
}
#[test]
fn commands_fragments_checksums_and_universes_rejected() {
    for protocol in ["openrgb", "adalight", "tpm2net", "ddp", "artnet", "sacn"] {
        let c = config(protocol, 80);
        let mut p = fixture(&c);
        match protocol {
            "openrgb" => {
                let last = p.len() - 1;
                p[last] ^= 1;
            }
            "adalight" => p[5] ^= 1,
            "tpm2net" => p[5] = 2,
            "ddp" => p[0] = 0x40,
            "artnet" => p[14] = 2,
            "sacn" => p[114] = 2,
            _ => {}
        }
        assert!(decode(&c, &p).is_none());
    }
    for protocol in ["artnet", "sacn"] {
        assert!(config(protocol, 480).validate().is_err());
        let mut c = config(protocol, 80);
        c.channel = usize::MAX;
        assert!(decode(&c, &[]).is_none());
    }
}
#[test]
fn serial_noise_concatenation_and_count_variants() {
    for protocol in ["openrgb", "adalight", "adalight-openrgb", "tpm2"] {
        let c = config(protocol, 480);
        let p = fixture(&c);
        let mut stream = vec![0, 255, 65, 0];
        stream.extend(&p);
        stream.extend(&p);
        let mut framer = Framer::new(&c);
        let mut count = 0;
        for b in stream {
            if let Some(frame) = framer.push(&c, b) {
                assert_eq!(frame.rgb.len(), 1440);
                count += 1;
            }
        }
        assert_eq!(count, 2);
    }
    assert!(
        decode(
            &config("adalight-openrgb", 80),
            &fixture(&config("adalight", 80))
        )
        .is_none()
    );
}
#[test]
fn sequenced_frames_are_pinned_and_termination_is_not_color() {
    let c = config("ddp", 480);
    let mut p = fixture(&c);
    p[1] = 15;
    let mut order = Order::default();
    assert!(order.accept(&decode(&c, &p).unwrap()));
    p[1] = 1;
    assert!(order.accept(&decode(&c, &p).unwrap()));
    assert!(!order.accept(&decode(&c, &p).unwrap()));
    p[1] = 14;
    assert!(!order.accept(&decode(&c, &p).unwrap()));
    let c = config("sacn", 80);
    let mut p = fixture(&c);
    let mut order = Order::default();
    assert!(order.accept(&decode(&c, &p).unwrap()));
    p[22] ^= 1;
    p[111] = 2;
    assert!(!order.accept(&decode(&c, &p).unwrap()));
    p[112] = 0x40;
    let ended = decode(&c, &p).unwrap();
    assert!(ended.ended && ended.rgb.is_empty());
}
#[test]
fn bounded_latest_frame_and_logical_mapping() {
    let mapping: Vec<_> = (1..=80)
        .map(|id| CellMapping {
            logical_id: id,
            physical_cell: 81 - id,
            name: String::new(),
        })
        .collect();
    let view = View::default();
    let mut input = Input {
        peer: Pin::from_identity("table", &"a".repeat(32)).unwrap(),
        config: config("ddp", 80),
        view: view.clone(),
        stop: Arc::new(AtomicBool::new(false)),
        reader: None,
        started: Instant::now(),
        mapping: mapping.clone(),
        layout: Layout::uniform(480, 6),
    };
    let mut rgb = vec![0; 240];
    rgb[..3].copy_from_slice(&[1, 2, 3]);
    {
        let mut s = view.lock().unwrap();
        s.last = Some(Instant::now());
        s.latest = Some((Instant::now(), rgb));
    }
    let out = input.poll(true).unwrap().unwrap();
    assert_eq!(&out[474 * 3..477 * 3], &[1, 2, 3, 1, 2, 3, 1, 2, 3]);
    assert!(out[..474 * 3].iter().all(|v| *v == 0));
    assert!(input.poll(true).unwrap().is_none());
    input.layout.as_mut().unwrap().revision += 1;
    assert!(!input.topology_current(&mapping, Layout::uniform(480, 6).as_ref()));
    view.lock().unwrap().last = Some(Instant::now() - Duration::from_secs(6));
    assert!(input.poll(false).is_err());
}
