//! PCController's MCU-owned Silent bit, separate from host notification audio.
//! Adapted from its SettingsStore and buzzer_runtime's known/pending/verified
//! distinction. Both transports run the same GET / SET / GET transaction.
use crate::{
    DeviceStatus, Transport,
    pc_contract::{self, Settings},
    peer_http::Pin,
};
use std::time::{Duration, Instant};

pub fn supported(status: &DeviceStatus) -> bool {
    settings_supported(status)
        && status
            .capabilities
            .as_ref()
            .is_some_and(|c| c.has("buzzer"))
}
pub fn settings_supported(status: &DeviceStatus) -> bool {
    status.avr_online
        && status
            .capabilities
            .as_ref()
            .is_some_and(|c| c.has("settings"))
}

#[derive(Clone, Copy)]
pub enum Change {
    Read,
    Silent(bool),
    Apply {
        baseline: [u8; 21],
        desired: [u8; 21],
    },
}
impl From<Option<bool>> for Change {
    fn from(value: Option<bool>) -> Self {
        value.map_or(Self::Read, Self::Silent)
    }
}
impl Change {
    pub fn allowed(self, status: &DeviceStatus) -> bool {
        if !settings_supported(status) || status.programming_latch {
            return false;
        }
        let has = |feature| status.capabilities.as_ref().is_some_and(|c| c.has(feature));
        match self {
            Self::Read => true,
            Self::Silent(_) => has("buzzer"),
            Self::Apply { baseline, desired } => {
                ((baseline[1] ^ desired[1]) & 3 == 0 || has("buzzer"))
                    && (baseline[2] == desired[2] || has("display"))
                    && (baseline[5..7] == desired[5..7] || has("inputs"))
            }
        }
    }
}

/// Editable MCU parameters. No PC-only preferences or fabricated hardware fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameters {
    pub silent: bool,
    pub welcome: bool,
    pub remember_page: bool,
    pub brightness: u8,
    pub stream_ms: u16,
    pub threshold: u16,
    pub default_page: u8,
    pub visible: u8,
    pub order: [u8; 6],
    pub name: String,
}
impl Parameters {
    pub fn from_settings(settings: &Settings) -> Self {
        let p = &settings.bytes;
        Self {
            silent: p[1] & 1 != 0,
            welcome: p[1] & 2 != 0,
            remember_page: p[1] & 4 != 0,
            brightness: p[2],
            stream_ms: u16::from_le_bytes([p[3], p[4]]),
            threshold: u16::from_le_bytes([p[5], p[6]]),
            default_page: p[7],
            visible: p[8],
            order: std::array::from_fn(|rank| (p[9 + rank / 2] >> (4 * (rank % 2))) & 15),
            name: String::from_utf8_lossy(&p[13..13 + p[12] as usize]).into_owned(),
        }
    }
    pub fn settings(&self) -> Result<Settings, String> {
        if self.stream_ms != 0 && self.stream_ms < 100 {
            return Err("Periodic telemetry must be 0 (changed-only), or at least 100 ms".into());
        }
        if self.default_page >= 6 || self.visible & (1 << self.default_page) == 0 {
            return Err(
                "The start page must remain visible. Choose another start page before hiding it."
                    .into(),
            );
        }
        if self.name.len() > 8 || !self.name.bytes().all(|b| (32..=126).contains(&b)) {
            return Err("AVR name must contain at most eight printable ASCII characters".into());
        }
        if self.order.iter().any(|page| *page >= 6) {
            return Err("Invalid menu order".into());
        }
        let mut p = [0u8; 21];
        p[0] = pc_contract::PROFILE;
        p[1] = u8::from(self.silent)
            | (u8::from(self.welcome) << 1)
            | (u8::from(self.remember_page) << 2);
        p[2] = self.brightness;
        p[3..5].copy_from_slice(&self.stream_ms.to_le_bytes());
        p[5..7].copy_from_slice(&self.threshold.to_le_bytes());
        p[7] = self.default_page;
        p[8] = self.visible;
        for (rank, page) in self.order.iter().enumerate() {
            p[9 + rank / 2] |= page << (4 * (rank % 2));
        }
        p[12] = self.name.len() as u8;
        p[13..13 + self.name.len()].copy_from_slice(self.name.as_bytes());
        let mut value = Settings {
            bytes: p,
            persisted: false,
        };
        value.validate()?;
        Ok(value)
    }
}

pub struct Transaction {
    desired: Change,
    next: Option<Settings>,
    phase: u8,
}
impl Transaction {
    pub fn new(desired: impl Into<Change>) -> Self {
        Self {
            desired: desired.into(),
            next: None,
            phase: 0,
        }
    }
    pub fn allowed(&self, status: &DeviceStatus) -> bool {
        self.desired.allowed(status)
    }
    pub fn request(&self) -> (u8, Vec<u8>) {
        if self.phase == 1 {
            (crate::protocol_opcodes::controller::SET_SETTINGS, self.next.as_ref().unwrap().bytes.to_vec())
        } else {
            (crate::protocol_opcodes::controller::GET_SETTINGS, vec![])
        }
    }
    pub fn accept(&mut self, opcode: u8, payload: &[u8]) -> Result<Option<Settings>, String> {
        if opcode == crate::protocol_opcodes::controller::ERROR_RESPONSE {
            return Err(format!(
                "Board rejected settings: {payload:02x?}; nothing retried"
            ));
        }
        if self.phase == 1 {
            if opcode != crate::protocol_opcodes::controller::ACK || payload != [crate::protocol_opcodes::controller::SET_SETTINGS, 0] {
                return Err("Missing AVR settings ACK; outcome unknown, no replay".into());
            }
            self.phase = 2;
            return Ok(None);
        }
        if opcode != crate::protocol_opcodes::controller::SETTINGS_RESPONSE {
            return Err("AVR settings response did not match; no state assumed".into());
        }
        let settings = Settings::parse(&pc_contract::encode_hex(payload))?;
        if self.phase == 2 {
            if !settings.persisted || self.next.as_ref().is_none_or(|s| s.bytes != settings.bytes) {
                return Err(
                    "AVR readback did not confirm persistent settings; refresh before retrying"
                        .into(),
                );
            }
            return Ok(Some(settings));
        }
        let mut next = settings.clone();
        match self.desired {
            Change::Read => return Ok(Some(settings)),
            Change::Silent(desired) => next.set("silent", if desired { "on" } else { "off" })?,
            Change::Apply { baseline, desired } => {
                if settings.bytes != baseline {
                    return Err("Board settings changed since editing began. Nothing written; refresh and review your changes.".into());
                }
                next.bytes = desired;
                next.validate()?;
            }
        }
        if settings.persisted && next.bytes == settings.bytes {
            return Ok(Some(settings));
        }
        self.next = Some(next);
        self.phase = 1;
        Ok(None)
    }
}

#[derive(Default)]
pub struct State {
    counter: u64,
    pub pending: Option<(u64, Pin, Transport, Instant)>,
    observed: Option<(Settings, Pin, Transport, Instant)>,
    pub message: String,
}
impl State {
    pub fn revoke(&mut self) {
        self.pending = None;
        self.observed = None;
        self.message.clear();
    }
    pub fn value(&self, peer: Option<&Pin>, route: Option<Transport>) -> Option<bool> {
        self.settings(peer, route).map(|s| s.bytes[1] & 1 != 0)
    }
    pub fn settings(&self, peer: Option<&Pin>, route: Option<Transport>) -> Option<&Settings> {
        self.observed
            .as_ref()
            .filter(|(_, p, r, at)| {
                Some(p) == peer && Some(*r) == route && at.elapsed() < Duration::from_secs(8)
            })
            .map(|(s, ..)| s)
    }
    pub fn persisted(&self) -> bool {
        self.observed.as_ref().is_some_and(|(s, ..)| s.persisted)
    }
    pub fn needs_refresh(&self) -> bool {
        self.pending.is_none()
            && (self.message.is_empty()
                || self
                    .observed
                    .as_ref()
                    .is_some_and(|(_, _, _, at)| at.elapsed() >= Duration::from_secs(4)))
    }
    pub fn begin(&mut self, peer: Pin, route: Transport) -> Option<u64> {
        if self.pending.is_some() {
            return None;
        }
        self.counter = self.counter.checked_add(1)?;
        self.pending = Some((self.counter, peer, route, Instant::now()));
        self.message = "Reading the board…".into();
        Some(self.counter)
    }
    pub fn finish(
        &mut self,
        id: u64,
        peer: Option<&Pin>,
        route: Option<Transport>,
        result: Result<Settings, String>,
    ) {
        let Some((expected, pin, method, _)) = &self.pending else {
            return;
        };
        if id != *expected {
            return;
        }
        if Some(pin) != peer || Some(*method) != route {
            self.revoke();
            return;
        }
        let (_, pin, method, _) = self.pending.take().unwrap();
        match result {
            Ok(settings) => {
                self.message = if settings.persisted {
                    "Board settings verified · saved in EEPROM"
                } else {
                    "Board settings read · not persisted"
                }
                .into();
                self.observed = Some((settings, pin, method, Instant::now()));
            }
            Err(error) => {
                self.observed = None;
                self.message = error;
            }
        }
    }
    pub fn expire(&mut self) {
        if self
            .pending
            .as_ref()
            .is_some_and(|(_, _, _, at)| at.elapsed() > Duration::from_secs(12))
        {
            self.pending = None;
            self.observed = None;
            self.message = "Board readback timed out; outcome unknown, nothing replayed".into();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn raw(flags: u8, persisted: u8) -> Vec<u8> {
        let mut p = vec![0x50, flags, 5, 0, 0, 0, 2, 0, 63, 0x10, 0x32, 0x54, 0];
        p.extend_from_slice(&[0; 8]);
        p.push(persisted);
        p
    }
    #[test]
    fn board_settings_require_each_changed_hardware_capability_and_no_programming_latch() {
        let mut status = DeviceStatus {
            avr_online: true,
            capabilities: Some(crate::capabilities::Manifest {
                board_kind: "controller".into(),
                acknowledged_features: vec!["settings".into()],
                ..Default::default()
            }),
            ..Default::default()
        };
        let base = Settings::parse(&pc_contract::encode_hex(&raw(2, 1))).unwrap();
        assert!(Change::Read.allowed(&status));
        assert!(!Change::Silent(true).allowed(&status));
        let mut next = base.bytes;
        next[2] = 7;
        assert!(
            !Change::Apply {
                baseline: base.bytes,
                desired: next
            }
            .allowed(&status)
        );
        status
            .capabilities
            .as_mut()
            .unwrap()
            .acknowledged_features
            .push("display".into());
        assert!(
            Change::Apply {
                baseline: base.bytes,
                desired: next
            }
            .allowed(&status)
        );
        next[5] = 1;
        assert!(
            !Change::Apply {
                baseline: base.bytes,
                desired: next
            }
            .allowed(&status)
        );
        status
            .capabilities
            .as_mut()
            .unwrap()
            .acknowledged_features
            .push("inputs".into());
        assert!(
            Change::Apply {
                baseline: base.bytes,
                desired: next
            }
            .allowed(&status)
        );
        status.programming_latch = true;
        assert!(!Change::Read.allowed(&status));
        status.programming_latch = false;
        status.avr_online = false;
        assert!(!Change::Read.allowed(&status));
    }
    #[test]
    fn board_settings_parameters_roundtrip_and_validate_every_editable_field() {
        let base = Settings::parse(&pc_contract::encode_hex(&raw(6, 1))).unwrap();
        let mut draft = Parameters::from_settings(&base);
        assert_eq!(draft.settings().unwrap().bytes, base.bytes);
        draft.name = "LED BOARD".into();
        assert!(draft.settings().is_err());
        draft.name = "TABLE".into();
        draft.silent = true;
        draft.welcome = false;
        draft.remember_page = true;
        draft.brightness = 7;
        draft.stream_ms = 125;
        draft.threshold = 731;
        draft.default_page = 4;
        draft.visible = 0b010101;
        draft.order = [5, 3, 1, 4, 2, 0];
        let changed = draft.settings().unwrap();
        assert_eq!(Parameters::from_settings(&changed), draft);
        for value in [1, 99] {
            let mut invalid = draft.clone();
            invalid.stream_ms = value;
            assert!(invalid.settings().is_err());
        }
        for value in [0, 1024] {
            let mut invalid = draft.clone();
            invalid.threshold = value;
            assert!(invalid.settings().is_err());
        }
        let mut invalid = draft.clone();
        invalid.order[0] = invalid.order[1];
        assert!(invalid.settings().is_err());
        let mut invalid = draft.clone();
        invalid.visible = 1;
        assert!(invalid.settings().is_err());
        let mut invalid = draft.clone();
        invalid.name = "نام".into();
        assert!(invalid.settings().is_err());
    }
    #[test]
    fn board_settings_full_apply_requires_fresh_baseline_ack_and_exact_persistent_readback() {
        let original = Settings::parse(&pc_contract::encode_hex(&raw(2, 1))).unwrap();
        let mut form = Parameters::from_settings(&original);
        form.name = "TABLE".into();
        form.threshold = 700;
        let next = form.settings().unwrap();
        let change = Change::Apply {
            baseline: original.bytes,
            desired: next.bytes,
        };
        let mut conflict = Transaction::new(change);
        assert!(
            conflict
                .accept(0x91, &raw(3, 1))
                .unwrap_err()
                .contains("Nothing written")
        );
        assert_eq!(conflict.request(), (4, vec![]));
        let mut tx = Transaction::new(change);
        assert!(tx.accept(0x91, &raw(2, 1)).unwrap().is_none());
        assert_eq!(tx.request(), (5, next.bytes.to_vec()));
        assert!(tx.accept(0x80, &[5, 0]).unwrap().is_none());
        assert!(tx.accept(0x91, &raw(2, 1)).is_err());
        let mut readback = next.bytes.to_vec();
        readback.push(0);
        assert!(tx.accept(0x91, &readback).is_err());
        readback[21] = 1;
        assert_eq!(
            tx.accept(0x91, &readback).unwrap().unwrap().bytes,
            next.bytes
        );
        let mut noop = Transaction::new(Change::Apply {
            baseline: original.bytes,
            desired: original.bytes,
        });
        assert!(noop.accept(0x91, &raw(2, 1)).unwrap().is_some());
        let mut bad = next.bytes;
        bad[2] = 8;
        let mut invalid = Transaction::new(Change::Apply {
            baseline: original.bytes,
            desired: bad,
        });
        assert!(invalid.accept(0x91, &raw(2, 1)).is_err());
    }
    #[test]
    fn board_silent_preserves_other_settings_and_requires_ack_persistent_readback() {
        for desired in [true, false] {
            let mut tx = Transaction::new(Some(desired));
            assert_eq!(tx.request(), (4, vec![]));
            assert!(
                tx.accept(0x91, &raw(if desired { 6 } else { 7 }, 1))
                    .unwrap()
                    .is_none()
            );
            let (op, bytes) = tx.request();
            assert_eq!(op, 5);
            assert_eq!(bytes[1], if desired { 7 } else { 6 });
            assert_eq!(&bytes[2..], &raw(6, 1)[2..21]);
            assert!(tx.accept(0x80, &[5, 0]).unwrap().is_none());
            assert_eq!(tx.request(), (4, vec![]));
            assert!(tx.accept(0x91, &raw(bytes[1], 0)).is_err());
            assert!(tx.accept(0x91, &raw(bytes[1] ^ 1, 1)).is_err());
            assert!(
                tx.accept(0x91, &raw(bytes[1], 1))
                    .unwrap()
                    .unwrap()
                    .persisted
            );
        }
    }
    #[test]
    fn board_silent_unknown_is_not_false_and_noop_does_not_write_eeprom() {
        let state = State::default();
        assert_eq!(state.value(None, None), None);
        let mut tx = Transaction::new(Some(true));
        assert!(tx.accept(0x91, &raw(3, 1)).unwrap().is_some());
        let mut tx = Transaction::new(None);
        assert!(tx.accept(0x91, &raw(2, 0)).unwrap().is_some());
        assert!(Transaction::new(Some(true)).accept(0x82, &[4, 6]).is_err());
    }
    #[test]
    fn board_silent_retires_foreign_and_failed_observations_without_clearing_id_counter() {
        let status = |boot: char| DeviceStatus {
            board_id: "esp32s3-fixture".into(),
            boot_id: boot.to_string().repeat(32),
            http_peer_fencing: true,
            ..Default::default()
        };
        let a = Pin::from_status(&status('a')).unwrap();
        let b = Pin::from_status(&status('b')).unwrap();
        let settings = Settings::parse(&pc_contract::encode_hex(&raw(3, 1))).unwrap();
        let mut state = State::default();
        let id = state.begin(a.clone(), Transport::Serial).unwrap();
        assert!(state.begin(a.clone(), Transport::Serial).is_none());
        state.finish(
            id + 1,
            Some(&a),
            Some(Transport::Serial),
            Ok(settings.clone()),
        );
        assert!(state.pending.is_some());
        state.finish(id, Some(&a), Some(Transport::Serial), Ok(settings.clone()));
        assert_eq!(state.value(Some(&a), Some(Transport::Serial)), Some(true));
        assert_eq!(state.value(Some(&a), Some(Transport::WiFi)), None);
        assert_eq!(state.value(Some(&b), Some(Transport::Serial)), None);
        let id = state.begin(a.clone(), Transport::Serial).unwrap();
        state.finish(
            id,
            Some(&a),
            Some(Transport::Serial),
            Err("Readback unavailable".into()),
        );
        assert_eq!(state.value(Some(&a), Some(Transport::Serial)), None);
        state.revoke();
        let next = state.begin(a, Transport::Serial).unwrap();
        assert!(next > id);
        state.finish(next, Some(&b), Some(Transport::Serial), Ok(settings));
        assert!(state.pending.is_none() && state.observed.is_none());
    }
}
