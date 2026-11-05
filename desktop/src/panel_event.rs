//! Requested panel presentation, never physical segment/ACK evidence.
//! Adapts PCController's per-source generation retirement, monotonic receipt
//! deadlines and no-replay semantics from hostbridge/buzzer_mirror.go. The
//! donor's motion/relay/menu-page IDs do not describe this board and are not
//! imported as capability authority (Project/FrontPanelModel.h).
use crate::protocol_opcodes::pakar as wire;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub const USB_OPCODE: u8 = wire::PANEL_EVENT;
const KINDS: [&str; 13] = [
    "rest", "cell", "mode", "effect", "pixel", "network", "progress", "fault", "complete",
    "output", "menu", "scan", "state",
];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Presentation {
    pub text: String,
    pub lifetime_ms: u16,
    pub blink: String,
    pub marquee: String,
    pub priority: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Observation {
    pub board_id: String,
    pub boot_id: String,
    pub event_id: u32,
    pub uptime_ms: u32,
    pub kind: String,
    pub value: u16,
    pub text: String,
    pub presentation: Presentation,
    pub hardware_confirmed: bool,
}
fn ascii(value: &str, maximum: usize) -> bool {
    value.len() <= maximum && value.bytes().all(|b| (0x20..=0x7e).contains(&b))
}
impl Observation {
    pub fn parse(value: serde_json::Value) -> Option<Self> {
        if value["type"] != "panel-event" {
            return None;
        }
        let event: Self = serde_json::from_value(value).ok()?;
        event.valid().then_some(event)
    }
    fn valid(&self) -> bool {
        !self.hardware_confirmed
            && !self.board_id.is_empty()
            && self.board_id.len() <= 128
            && !self.boot_id.is_empty()
            && self.boot_id.len() <= 128
            && KINDS.contains(&self.kind.as_str())
            && ascii(&self.text, 40)
            && self.presentation.text.len() == 4
            && ascii(&self.presentation.text, 4)
            && matches!(
                self.presentation.blink.as_str(),
                "steady" | "slow" | "double-fast"
            )
            && matches!(self.presentation.marquee.as_str(), "once" | "none")
    }
    /// The compact event has no peer identity: bind only after an already
    /// verified snapshot on this exact physical USB session. Never bootstrap.
    pub fn usb(payload: &[u8], status: &crate::DeviceStatus) -> Option<Self> {
        crate::peer_http::Pin::from_status(status)?;
        if !(16..=48).contains(&payload.len())
            || payload[15] > 32
            || payload.len() != 16 + usize::from(payload[15])
        {
            return None;
        }
        let text = std::str::from_utf8(&payload[16..16 + usize::from(payload[15])])
            .ok()?
            .to_owned();
        if !ascii(&text, 32) {
            return None;
        }
        let mut preview: String = text.chars().take(4).collect();
        while preview.len() < 4 {
            preview.push(' ');
        }
        let event = Self {
            board_id: status.board_id.clone(),
            boot_id: status.boot_id.clone(),
            event_id: u32::from_le_bytes(payload[0..4].try_into().ok()?),
            uptime_ms: u32::from_le_bytes(payload[4..8].try_into().ok()?),
            kind: (*KINDS.get(usize::from(payload[8]))?).into(),
            value: u16::from_le_bytes(payload[13..15].try_into().ok()?),
            text,
            presentation: Presentation {
                text: preview,
                lifetime_ms: u16::from_le_bytes(payload[11..13].try_into().ok()?),
                blink: (*["steady", "slow", "double-fast"].get(usize::from(payload[9]))?).into(),
                marquee: if payload[15] > 4 { "once" } else { "none" }.into(),
                priority: payload[10],
            },
            hardware_confirmed: false,
        };
        event.valid().then_some(event)
    }
}
#[derive(Default)]
pub struct State {
    peer: Option<crate::peer_http::Pin>,
    last_id: Option<u32>,
    pub history: VecDeque<Observation>,
    at: Option<Instant>,
}
impl State {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    pub fn observe(&mut self, event: Observation, peer: &crate::peer_http::Pin) -> bool {
        if !event.valid() || !peer.matches_identity(&event.board_id, &event.boot_id) {
            return false;
        }
        if self.peer.as_ref() != Some(peer) {
            self.clear();
            self.peer = Some(peer.clone());
        }
        if self.last_id.is_some_and(|id| {
            let delta = event.event_id.wrapping_sub(id);
            delta == 0 || delta > 0x7fffffff
        }) {
            return false;
        }
        self.last_id = Some(event.event_id);
        if self.history.len() == 32 {
            self.history.pop_front();
        }
        self.history.push_back(event);
        self.at = Some(Instant::now());
        true
    }
    pub fn current(&self, peer: &crate::peer_http::Pin) -> Option<&Observation> {
        if self.peer.as_ref() != Some(peer) {
            return None;
        }
        let event = self.history.back()?;
        (event.presentation.lifetime_ms > 0
            && self.at?.elapsed()
                < Duration::from_millis(u64::from(event.presentation.lifetime_ms)))
        .then_some(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn status() -> crate::DeviceStatus {
        crate::DeviceStatus {
            board_id: "board-a".into(),
            boot_id: "a".repeat(32),
            http_peer_fencing: true,
            ..Default::default()
        }
    }
    fn peer() -> crate::peer_http::Pin {
        crate::peer_http::Pin::from_status(&status()).unwrap()
    }
    fn event(id: u32) -> Observation {
        Observation::parse(serde_json::json!({"type":"panel-event","board_id":"board-a","boot_id":"a".repeat(32),"event_id":id,"uptime_ms":123,"kind":"cell","value":80,"text":"Cell 80 red","presentation":{"text":"  80","lifetime_ms":1000,"blink":"steady","marquee":"none","priority":1},"hardware_confirmed":false})).unwrap()
    }
    #[test]
    fn semantic_events_are_peer_bound_deduplicated_bounded_and_not_hardware_evidence() {
        let mut state = State::default();
        let peer = peer();
        assert!(state.observe(event(u32::MAX), &peer));
        assert!(state.observe(event(0), &peer));
        assert!(!state.observe(event(0), &peer));
        assert!(!state.observe(event(u32::MAX), &peer));
        for id in 1..70 {
            assert!(state.observe(event(id), &peer));
        }
        assert_eq!(state.history.len(), 32);
        assert_eq!(state.current(&peer).unwrap().value, 80);
        let mut wrong = event(70);
        wrong.boot_id = "b".repeat(32);
        assert!(!state.observe(wrong, &peer));
        let mut physical = event(70);
        physical.hardware_confirmed = true;
        assert!(!state.observe(physical, &peer));
        state.at = Some(Instant::now() - Duration::from_secs(2));
        assert!(state.current(&peer).is_none());
        state.clear();
        assert!(state.history.is_empty());
        assert!(state.observe(event(1), &peer));
    }
    #[test]
    fn usb_requires_exact_payload_and_preserves_requested_only_semantics() {
        let mut payload = [0_u8; 48];
        payload[..4].copy_from_slice(&1_u32.to_le_bytes());
        payload[8] = 1;
        payload[10] = 2;
        payload[11..13].copy_from_slice(&1000_u16.to_le_bytes());
        payload[13] = 80;
        payload[15] = 11;
        payload[16..27].copy_from_slice(b"Cell 80 red");
        let event = Observation::usb(&payload[..27], &status()).unwrap();
        assert_eq!(event.text, "Cell 80 red");
        assert_eq!(event.presentation.text, "Cell");
        assert_eq!(event.presentation.marquee, "once");
        assert!(!event.hardware_confirmed);
        assert!(Observation::usb(&payload[..26], &status()).is_none());
        assert!(Observation::usb(&payload, &status()).is_none());
        payload[15] = 33;
        assert!(Observation::usb(&payload, &status()).is_none());
        payload[15] = 11;
        payload[9] = 3;
        assert!(Observation::usb(&payload[..27], &status()).is_none());
    }
    #[test]
    fn malformed_optional_event_is_ignored_not_a_disconnect() {
        let raw = serde_json::json!({"type":"panel-event","hardware_confirmed":true});
        // Observation deliberately exposes no status/capability conversion.
        assert!(Observation::parse(raw).is_none());
    }
}
