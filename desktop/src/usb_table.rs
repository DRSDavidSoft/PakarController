//! Frozen USB table reads reuse the existing identity chunk shape, not its
//! mutable identity revision. PCController Session's matched ACK/admission and
//! no-replay rules apply to one operation on the selected USB owner.
use crate::protocol_opcodes::pakar as wire;
use crate::{CellMapping, CellMappingResponse, DeviceStatus, peer_http};
use std::time::{Duration, Instant};
pub const READ: u8 = wire::GET_TABLE_SNAPSHOT;
pub const RESPONSE: u8 = wire::TABLE_SNAPSHOT_RESPONSE;
pub const COMMAND: u8 = wire::TABLE_COMMAND;
pub const COMMAND_STATE: u8 = wire::TABLE_COMMAND_STATE;
// A command can overtake the next unsent background chunk, never bytes already
// on wire. Adapted from PCController Session.grantNextLocked request priority.
pub fn yield_snapshot(
    pending: bool,
    verified: bool,
    reading: bool,
    waiting: bool,
    operating: bool,
) -> bool {
    pending && verified && reading && !waiting && !operating
}
/// Actual board readback, not request-derived values. Identity, mappings and
/// peripheral observations remain owned by the full capture; this must NOT
/// renew their freshness or fabricate published pixel RGB.
pub fn command_state(
    previous: &DeviceStatus,
    opcode: u8,
    p: &[u8],
) -> Result<DeviceStatus, String> {
    if !matches!(p.len(), 44 | 45)
        || p[0] != opcode
        || !table_command(opcode)
        || peer_http::Pin::from_status(previous).is_none()
    {
        return Err("Invalid USB command-state readback".into());
    }
    let boot = p[1..17]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    if boot != previous.boot_id
        || p[17] > 3
        || p[18] > 9
        || !(1..=80).contains(&p[19])
        || p[20] == 0
        || p[21] > 3
        || p[22] > 1
        || p[23] > 1
    {
        return Err("USB command-state peer/values invalid; no state assumed".into());
    }
    let cell_count = if previous
        .capabilities
        .as_ref()
        .is_some_and(|c| c.role == "first-wins")
    {
        18
    } else {
        80
    };
    let mut cells = Vec::with_capacity(cell_count);
    for i in 0..cell_count {
        cells.push(
            ["white", "red", "off"]
                .get(((p[24 + i / 4] >> ((i % 4) * 2)) & 3) as usize)
                .ok_or("Invalid USB cell state")?
                .to_string(),
        );
    }
    let mut status = previous.clone();
    status.cells = cells;
    status.red_lock = if p.len() == 45 {
        if p[44] > 1 {
            return Err("Invalid USB red-lock readback".into());
        }
        Some(p[44] != 0)
    } else {
        None
    };
    status.mode = ["unknown", "cell", "first-wins", "snake"][p[17] as usize].into();
    status.winner = p[18];
    status.selected_cell = p[19];
    status.brightness = p[20];
    status.effect = ["none", "rainbow", "fire", "cell-chase"][p[21] as usize].into();
    status.led_output["enabled"] = (p[22] != 0).into();
    status.led_output["mode"] = if p[23] == 0 { "static" } else { "effects" }.into();
    status.pixel_frame = None;
    status.pixel_preview_requested = false;
    Ok(status)
}
#[derive(Clone, serde::Serialize)]
pub struct Timing {
    pub session: u64,
    pub sequence: u8,
    pub opcode: u8,
    pub queue_ms: f64,
    pub wire_ack_ms: f64,
    pub wire_confirm_ms: f64,
    pub total_ms: f64,
    pub confirmation: &'static str,
}
pub struct Operation {
    pub intent: Intent,
    pub sequence: u8,
    pub sent: Instant,
    pub ack: Option<Instant>,
}
impl Operation {
    pub fn matches(&self, op: u8, seq: u8, p: &[u8]) -> bool {
        matches!(op, wire::ACK | wire::ERROR_RESPONSE)
            && self.sequence == seq
            && p.len() == 2
            && (p[0] == self.intent.opcode || (op == wire::ERROR_RESPONSE && p[0] == COMMAND))
    }
    pub fn timing(&self, session: u64, confirmation: &'static str) -> Timing {
        let now = Instant::now();
        Timing {
            session,
            sequence: self.sequence,
            opcode: self.intent.opcode,
            queue_ms: self
                .sent
                .saturating_duration_since(self.intent.admission.at)
                .as_secs_f64()
                * 1000.,
            wire_ack_ms: self
                .ack
                .unwrap_or(now)
                .saturating_duration_since(self.sent)
                .as_secs_f64()
                * 1000.,
            wire_confirm_ms: now.saturating_duration_since(self.sent).as_secs_f64() * 1000.,
            total_ms: now
                .saturating_duration_since(self.intent.admission.at)
                .as_secs_f64()
                * 1000.,
            confirmation,
        }
    }
}
/// Verification belongs to the active command, ahead of any queued command.
/// Otherwise a lost compact companion plus a second click would deadlock the
/// fallback: the next write needs verification, while verification waits for
/// an empty write queue. Never replay either write to recover that condition.
pub fn snapshot_admitted(queued: bool, operation: Option<&Operation>, compact: bool) -> bool {
    match operation {
        None => !queued,
        Some(active) => active.ack.is_some_and(|at| {
            !compact
                || active.intent.needs_full_snapshot()
                || at.elapsed() >= Duration::from_millis(100)
        }),
    }
}
// Match the native current-peer authority window; capture time, not the final
// chunk's arrival, owns freshness. Slow captures expire rather than masquerade
// as new state. Windows first-byte COM timeout is not a per-chunk sleep.
pub const MAX_SNAPSHOT_AGE: Duration = Duration::from_secs(8);
#[derive(Default)]
pub struct Reader {
    pub nonce: u8,
    token: Option<u32>,
    total: usize,
    bytes: Vec<u8>,
    started: Option<Instant>,
}
pub struct Snapshot {
    pub status: DeviceStatus,
    pub mappings: Vec<CellMapping>,
    pub observed_at: Instant,
    pub icon_omitted: bool,
}
impl Snapshot {
    pub fn retain_icon(&mut self, previous: Option<&DeviceStatus>) {
        let Some(previous)=previous else{return;};
        if !self.icon_omitted || peer_http::Pin::from_status(&self.status)!=peer_http::Pin::from_status(previous) {return;}
        let old=previous.capabilities.as_ref().and_then(|c|c.application.as_ref());
        let next=self.status.capabilities.as_mut().and_then(|c|c.application.as_mut());
        if let (Some(old),Some(next))=(old,next) && old.valid() && next.valid()
            && old.id==next.id && old.name==next.name && old.title==next.title
            && next.icon_png_base64.is_empty() && old.icon_bytes().is_some() {
            next.icon_png_base64=old.icon_png_base64.clone();
        }
    }
}
#[derive(Clone, serde::Serialize)]
pub struct CaptureProgress {
    pub received: usize,
    pub total: usize,
    pub age_ms: u64,
    pub state: &'static str,
}
impl Reader {
    pub fn new(nonce: u8) -> Self {
        Self {
            nonce,
            started: Some(Instant::now()),
            ..Default::default()
        }
    }
    pub fn request(&self) -> Vec<u8> {
        let mut out = vec![self.nonce];
        out.extend((self.bytes.len() as u16).to_le_bytes());
        out.extend(self.token.unwrap_or(0).to_le_bytes());
        out
    }
    /// After the first immutable token/total is verified, request up to three
    /// following chunks without another Windows COM read-timeout round trip.
    /// No speculative token, write or unbounded receive queue is introduced.
    pub fn following_requests(&self) -> Vec<Vec<u8>> {
        let Some(token) = self.token else {
            return Vec::new();
        };
        (1..4)
            .map(|n| self.bytes.len() + 39 * n)
            .take_while(|at| *at < self.total)
            .map(|at| {
                let mut p = vec![self.nonce];
                p.extend((at as u16).to_le_bytes());
                p.extend(token.to_le_bytes());
                p
            })
            .collect()
    }
    pub fn progress(&self, state: &'static str) -> CaptureProgress {
        CaptureProgress {
            received: self.bytes.len(),
            total: self.total,
            age_ms: self.started.map_or(0, |at| {
                at.elapsed().as_millis().min(u64::MAX as u128) as u64
            }),
            state,
        }
    }
    pub fn accept(&mut self, p: &[u8]) -> Result<Option<Snapshot>, String> {
        if self
            .started
            .is_some_and(|at| at.elapsed() >= MAX_SNAPSHOT_AGE)
        {
            return Err("USB snapshot capture expired".into());
        }
        if !(10..=48).contains(&p.len()) || p[0] != self.nonce {
            return Err("Invalid/stale USB table chunk".into());
        }
        let token = u32::from_le_bytes(p[1..5].try_into().unwrap());
        let total = u16::from_le_bytes(p[5..7].try_into().unwrap()) as usize;
        let offset = u16::from_le_bytes(p[7..9].try_into().unwrap()) as usize;
        if token == 0
            || total == 0
            || total > 16384
            || offset != self.bytes.len()
            || offset + p.len() - 9 > total
            || self
                .token
                .is_some_and(|old| old != token || self.total != total)
        {
            return Err("Mixed/out-of-order USB table capture".into());
        }
        self.token = Some(token);
        self.total = total;
        self.bytes.extend_from_slice(&p[9..]);
        if self.bytes.len() != total {
            return Ok(None);
        }
        let value: serde_json::Value =
            serde_json::from_slice(&self.bytes).map_err(|_| "Malformed USB table snapshot")?;
        let mut status: DeviceStatus = serde_json::from_value(value["status"].clone())
            .map_err(|_| "Invalid USB table status")?;
        let icon_omitted=value["status_partial"]==true
            && value["omitted_fields"]==serde_json::json!(["capabilities.application.icon_png_base64"])
            && value["status"]["capabilities"]["application"].as_object().is_some_and(|app|!app.contains_key("icon_png_base64"));
        let keys = status
            .capabilities
            .as_ref()
            .is_some_and(|c| c.known() && c.role == "first-wins" && c.has("leds"));
        let (pixels, groups) = if keys { (216, 18) } else { (480, 80) };
        if let Some(semantic) = value["status"].get("cell_states") {
            status.cells = semantic.as_array().filter(|items| items.len() == groups)
                .and_then(|items| items.iter().map(|v| v.as_str().filter(|s| matches!(*s,"idle"|"lost"|"won"|"off")).map(str::to_owned)).collect())
                .ok_or("Invalid USB semantic group states")?;
        }
        if peer_http::Pin::from_status(&status).is_none()
            || status.pixels != pixels
            || status.cells.len() != groups
            || !status
                .cells
                .iter()
                .all(|v| matches!(v.as_str(), "white" | "red" | "idle" | "lost" | "won" | "off"))
            || crate::pixel_groups::for_status(&status).is_none()
            || !(keys || status.capabilities.as_ref().is_some_and(|c| c.table()))
        {
            return Err("USB board identity/capabilities/state incomplete".into());
        }
        // Optional on older firmware. The frame is actual last-published RGB,
        // captured once with this token; never derive six pixels from cell state.
        status.pixel_preview_requested = value.get("pixels").is_some_and(|v| !v.is_null());
        status.pixel_frame_partial = if value["pixel_frame_partial"] == true && value["pixels"]["rgb_omitted"] == true {
            value["pixels"].clone()
        } else { serde_json::Value::Null };
        status.pixel_frame = match value.get("pixels").filter(|v| !v.is_null()) {
            Some(frame) if value["pixel_frame_partial"] == true && frame["rgb_omitted"] == true => None,
            Some(value) => {
                let frame: crate::ui::cell_pixels::Frame = serde_json::from_value(value.clone())
                    .map_err(|_| "Invalid USB pixel snapshot")?;
                if !frame.structurally_valid() || frame.physical.len() != groups {
                    return Err("Invalid USB pixel snapshot".into());
                }
                // dirty/staging is normal during animation. It cannot promote
                // RGB as a fresh presented sample, but must not discard the
                // independently valid board identity, state and 80 mappings.
                (!frame.pending).then_some(frame)
            }
            None => None,
        };
        if keys {
            let physical = value["physical"]
                .as_array()
                .ok_or("Missing key group mapping")?;
            if physical.len() != 18
                || !(1..=18)
                    .all(|id| physical.iter().filter(|p| p.as_u64() == Some(id)).count() == 1)
            {
                return Err("Invalid key group mapping".into());
            }
            return Ok(Some(Snapshot {
                status,
                mappings: Vec::new(),
                observed_at: self.started.unwrap_or_else(Instant::now),
                icon_omitted,
            }));
        }
        let mapping: CellMappingResponse =
            serde_json::from_value(value).map_err(|_| "Invalid USB table mapping")?;
        Ok(Some(Snapshot {
            status,
            mappings: mapping.decode()?,
            observed_at: self.started.unwrap_or_else(Instant::now),
            icon_omitted,
        }))
    }
}
pub fn table_command(op: u8) -> bool {
    matches!(
        op,
        wire::SET_MODE | wire::CELL_ACTION | wire::GAME_RESET | wire::SET_BRIGHTNESS | wire::SET_LED_PATTERN | wire::SET_LED_OUTPUT | wire::CELLS_ACTION | wire::CELL_ASSIGNMENTS | wire::CELL_LOCK | wire::PIXEL_DIAGNOSTIC | wire::KEY_SETTINGS | wire::GAME_PRESS | wire::KEY_PRESENTATION
    )
}
const KEY_FIELDS: [(&str, usize, usize, u32); 6] = [
    ("enabled_mask", 1, 2, 511),
    ("idle_rgb", 3, 3, 0xffffff),
    ("win_rgb", 6, 3, 0xffffff),
    ("lose_rgb", 9, 3, 0xffffff),
    ("extended_effects", 12, 1, 1),
    ("auto_reset_ms", 13, 4, 86400000),
];
pub fn game_cue_id(name: &str) -> Option<u8> {
    if name == "silent" {
        Some(255)
    } else {
        crate::audio::id(name)
    }
}
fn game_cue_name(id: u8) -> Option<&'static str> {
    if id == 255 {
        Some("silent")
    } else {
        crate::audio::CUES.get(id as usize).map(|v| v.0)
    }
}
pub fn key_settings_patch(form: &[(String, String)]) -> Result<Vec<u8>, String> {
    let mut p = vec![0; 17];
    for (name, text) in form {
        if matches!(name.as_str(), "winner_cue" | "reset_cue") {
            let (bit, at) = if name == "winner_cue" {
                (64, 17)
            } else {
                (128, 18)
            };
            if p[0] & bit != 0 {
                return Err("Duplicate key setting".into());
            }
            p.resize(19, 0);
            p[at] = game_cue_id(text).ok_or("Invalid game melody")?;
            p[0] |= bit;
            continue;
        }
        let Some((i, (_, at, n, max))) = KEY_FIELDS.iter().enumerate().find(|(_, f)| f.0 == name)
        else {
            return Err("Unknown key setting".into());
        };
        if p[0] & (1 << i) != 0 {
            return Err("Duplicate key setting".into());
        }
        let value = if i == 4 {
            match text.as_str() {
                "true" => 1,
                "false" => 0,
                _ => return Err("Invalid effects boolean".into()),
            }
        } else {
            text.parse::<u32>().map_err(|_| "Invalid key setting")?
        };
        if value > *max || (i == 0 && value == 0) {
            return Err("Key setting outside range".into());
        }
        p[0] |= 1 << i;
        for byte in 0..*n {
            p[at + byte] = (value >> (byte * 8)) as u8;
        }
    }
    if p[0] == 0 {
        return Err("Empty key settings".into());
    }
    Ok(p)
}
/// Network game settings may include ESP-owned motion controls absent from the
/// fixed AVR/USB mask. Keep the USB wire unchanged and verify every selected
/// field against the same board's acknowledged status after network delivery.
pub fn key_settings_network_verify(s: &DeviceStatus, form: &[(String, String)]) -> bool {
    if form.is_empty() {
        return false;
    }
    form.iter().all(|(name, text)| match name.as_str() {
        "presentation" => crate::game_presentation::parse(text).is_ok_and(|patch| crate::game_presentation::verify(&s.key_game["presentation"], &patch)),
        "winner_cue" | "reset_cue" => {
            game_cue_id(text).is_some()
                && s.game_audio_applied
                && s.key_game[name].as_str() == Some(text.as_str())
        }
        "extended_effects" | "chase_neighbors" | "chase_invert" => {
            matches!(text.as_str(), "true" | "false")
                && s.key_game[name].as_bool() == Some(text == "true")
        }
        "chase_period_ms" => text
            .parse::<u64>()
            .ok()
            .filter(|v| (400..=3000).contains(v))
            .is_some_and(|v| s.key_game[name].as_u64() == Some(v)),
        "enabled_mask" | "idle_rgb" | "win_rgb" | "lose_rgb" | "auto_reset_ms" => {
            let limit = if name == "enabled_mask" {
                511
            } else if name == "auto_reset_ms" {
                86400000
            } else {
                0xffffff
            };
            text.parse::<u64>()
                .ok()
                .filter(|v| *v <= limit && (name != "enabled_mask" || *v > 0))
                .is_some_and(|v| s.key_game[name].as_u64() == Some(v))
        }
        _ => false,
    })
}
fn key_settings_valid(p: &[u8]) -> bool {
    matches!(p.len(), 17 | 19)
        && p[0] != 0
        && (p.len() == 19 || p[0] & !63 == 0)
        && (0..2).all(|i| p[0] & (64 << i) == 0 || game_cue_name(p[17 + i]).is_some())
        && KEY_FIELDS.iter().enumerate().all(|(i, (_, at, n, max))| {
            if p[0] & (1 << i) == 0 {
                return true;
            }
            let value = (0..*n).fold(0u32, |v, b| v | (u32::from(p[at + b]) << (b * 8)));
            value <= *max && (i != 0 || value != 0)
        })
}
pub(crate) fn key_settings_verify(s: &DeviceStatus, p: &[u8]) -> bool {
    key_settings_valid(p)
        && ["winner_cue", "reset_cue"]
            .iter()
            .enumerate()
            .all(|(i, name)| {
                p[0] & (64 << i) == 0
                    || s.game_audio_applied
                        && s.key_game[*name].as_str() == game_cue_name(p[17 + i])
            })
        && KEY_FIELDS.iter().enumerate().all(|(i, (name, at, n, _))| {
            if p[0] & (1 << i) == 0 {
                return true;
            }
            let value = (0..*n).fold(0u32, |v, b| v | (u32::from(p[at + b]) << (b * 8)));
            if i == 4 {
                s.key_game[*name].as_bool() == Some(value != 0)
            } else {
                s.key_game[*name].as_u64() == Some(u64::from(value))
            }
        })
}
pub fn fenced(status: &DeviceStatus, op: u8, payload: &[u8]) -> Result<Vec<u8>, String> {
    if op == wire::SET_LED_OUTPUT && payload.first() == Some(&3)
        && !(payload.len() == 2 && crate::led_backend::name(payload[1])
            .is_some_and(|backend| crate::led_backend::allowed(status, backend))) {
        return Err("LED backend unavailable, switching or invalid; nothing sent".into());
    }
    if matches!(op, wire::GAME_PRESS | wire::KEY_PRESENTATION) {
        let feature = if op == wire::GAME_PRESS { "game-press" } else { "game-presentation" };
        let valid = if op == wire::GAME_PRESS { matches!(payload, [1..=9]) } else { crate::game_presentation::decode(payload).is_some() };
        if !valid || !status.capabilities.as_ref().is_some_and(|c| c.role == "first-wins" && c.has(feature)) {
            return Err("Game control unavailable or invalid".into());
        }
    }
    if op == wire::KEY_SETTINGS
        && payload.first().is_some_and(|mask| mask & 192 != 0)
        && !status
            .capabilities
            .as_ref()
            .is_some_and(|c| c.has("game-melodies"))
    {
        return Err("Game melodies unavailable".into());
    }
    if op == wire::KEY_SETTINGS
        && (!key_settings_valid(payload)
            || !status
                .capabilities
                .as_ref()
                .is_some_and(|c| c.role == "first-wins" && c.has("usb-key-settings")))
    {
        return Err("USB key settings unavailable or invalid".into());
    }
    if !table_command(op) || payload.len() > 31 || peer_http::Pin::from_status(status).is_none() {
        return Err("USB table command lacks a verified peer".into());
    }
    if op == crate::cell_batch::OPCODE
        && (!crate::cell_batch::Request::supported(status)
            || crate::cell_batch::Request::from_payload(payload).is_none())
    {
        return Err(
            "Atomic USB cell batch unavailable/invalid; nothing sent, no per-cell fallback".into(),
        );
    }
    if op == wire::CELL_ASSIGNMENTS
        && (!crate::cell_assignments::supported(status)
            || crate::cell_assignments::Document::from_payload(payload).is_err())
    {
        return Err("Invalid or unavailable atomic assignments".into());
    }
    if op == wire::CELL_LOCK
        && (!matches!(payload, [0] | [1])
            || !status
                .capabilities
                .as_ref()
                .is_some_and(|c| c.table_selection() && c.has("cell-lock")))
    {
        return Err("Invalid or unavailable red lock".into());
    }
    if op == wire::PIXEL_DIAGNOSTIC
        && (!crate::pixel_diagnostic::usb_supported(status)
            || crate::pixel_groups::advertised(status)
            || crate::pixel_diagnostic::profile(status).is_none_or(|(pixels, group)| {
                crate::pixel_diagnostic::Probe::from_usb_profile(payload, pixels, group).is_none()
            }))
    {
        return Err("Invalid or unavailable USB pixel diagnostic; nothing sent".into());
    }
    let mut out = Vec::with_capacity(17 + payload.len());
    for pair in status.boot_id.as_bytes().chunks_exact(2) {
        let nibble = |v: u8| {
            if v.is_ascii_digit() {
                v - b'0'
            } else {
                v - b'a' + 10
            }
        };
        out.push(nibble(pair[0]) * 16 + nibble(pair[1]));
    }
    out.push(op);
    out.extend_from_slice(payload);
    Ok(out)
}
#[derive(Clone)]
pub struct Admission {
    pub stamp: crate::connection::Stamp,
    pub session: u64,
    pub peer: Option<peer_http::Pin>,
    pub at: Instant,
    pub batch: Option<crate::cell_batch::Correlation>,
}
impl Admission {
    pub fn current(&self, session: u64, status: Option<&DeviceStatus>) -> bool {
        self.session != 0
            && self.session == session
            && self.stamp.current()
            && self.at.elapsed() < Duration::from_secs(10)
            && self.peer == status.and_then(peer_http::Pin::from_status)
            && self.batch.as_ref().is_none_or(|batch| {
                Some(&batch.peer) == self.peer.as_ref() && batch.route == crate::Transport::Serial
            })
    }
}
pub struct Intent {
    pub opcode: u8,
    pub payload: Vec<u8>,
    pub admission: Admission,
    pub reply: Option<crate::bridge::Reply>,
}
impl Intent {
    pub fn needs_full_snapshot(&self) -> bool {
        matches!(self.opcode, wire::PIXEL_DIAGNOSTIC | wire::KEY_SETTINGS | wire::KEY_PRESENTATION)
            || self.opcode == wire::SET_LED_OUTPUT && self.payload.first() == Some(&3)
    }
    pub fn reply(&self, result: Result<DeviceStatus, String>) {
        if let Some(reply) = &self.reply {
            let _ = reply.send(result);
        }
    }
    pub fn current(&self, session: u64, status: Option<&DeviceStatus>) -> bool {
        self.reply.as_ref().is_none_or(|r| r.current())
            && self.admission.peer.is_some()
            && self.admission.current(session, status)
            && (self.opcode != crate::cell_batch::OPCODE
                || self
                    .admission
                    .batch
                    .as_ref()
                    .is_some_and(|batch| batch.request.payload() == self.payload))
    }
    pub fn verify(&self, s: &DeviceStatus) -> bool {
        match (self.opcode, self.payload.as_slice()) {
            (wire::SET_LED_OUTPUT, [3, backend]) => crate::led_backend::name(*backend)
                .is_some_and(|name| crate::led_backend::confirmed(s, name)),
            (wire::KEY_SETTINGS, payload) => key_settings_verify(s, payload),
            (wire::GAME_PRESS, [1..=9]) => true, // Completion requires the correlated AVR ACK, then authoritative readback.
            (wire::KEY_PRESENTATION, payload) => crate::game_presentation::decode(payload).is_some_and(|patch| crate::game_presentation::verify(&s.key_game["presentation"], &patch)),
            (wire::PIXEL_DIAGNOSTIC, payload) => crate::pixel_diagnostic::usb_verify(s, payload),
            (wire::CELL_ASSIGNMENTS, payload) => {
                crate::cell_assignments::Document::from_payload(payload).is_ok_and(|d| d.verify(s))
            }
            (wire::CELL_LOCK, [value]) if *value <= 1 => s.red_lock == Some(*value != 0),
            (wire::CELLS_ACTION, payload) => self.admission.batch.as_ref().is_some_and(|batch| {
                batch.request.payload() == payload
                    && crate::peer_http::Pin::from_status(s).as_ref() == Some(&batch.peer)
                    && batch.request.verify(s)
            }),
            (wire::SET_MODE, [mode]) => ["unknown", "cell", "first-wins", "snake"]
                .get(*mode as usize)
                .is_some_and(|v| s.mode == *v),
            (wire::CELL_ACTION, [id, state, ..]) => {
                *id >= 1
                    && *id <= 80
                    && ["white", "red", "off"]
                        .get(*state as usize)
                        .is_some_and(|v| {
                            s.cells
                                .get(*id as usize - 1)
                                .is_some_and(|actual| actual.as_str() == *v)
                        })
            }
            (wire::GAME_RESET, []) => s.cells.iter().all(|v| v == "white") && s.winner == 0,
            (wire::SET_BRIGHTNESS, [brightness]) => s.brightness == *brightness,
            (wire::SET_LED_PATTERN, [pattern]) => ["none", "rainbow", "fire"]
                .get(*pattern as usize)
                .is_some_and(|v| s.effect == *v),
            (wire::SET_LED_OUTPUT, [enabled]) if *enabled <= 1 => {
                s.led_output["enabled"].as_bool() == Some(*enabled != 0)
            }
            (wire::SET_LED_OUTPUT, [2, mode]) if *mode <= 1 => {
                s.led_output["mode"].as_str() == Some(if *mode == 0 { "static" } else { "effects" })
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn key_settings_melodies_require_applied_receipt_and_valid_wire_ids() {
        let form = vec![
            ("winner_cue".into(), "attention".into()),
            ("reset_cue".into(), "silent".into()),
        ];
        let p = key_settings_patch(&form).unwrap();
        assert_eq!(p.len(), 19);
        assert_eq!(p[0], 192);
        assert_eq!(&p[17..], &[14, 255]);
        let mut s = read(&fixture()).status;
        s.key_game = serde_json::json!({"winner_cue":"attention","reset_cue":"silent"});
        assert!(!key_settings_verify(&s, &p));
        assert!(!key_settings_network_verify(&s, &form));
        s.game_audio_applied = true;
        assert!(key_settings_verify(&s, &p));
        assert!(key_settings_network_verify(&s, &form));
        let mut invalid = p.clone();
        invalid[17] = 15;
        assert!(!key_settings_valid(&invalid));
        assert!(!key_settings_valid(&p[..17]));
        assert!(key_settings_patch(&[("winner_cue".into(), "invalid".into())]).is_err());
        assert!(
            key_settings_patch(&[
                ("winner_cue".into(), "winner".into()),
                ("winner_cue".into(), "silent".into())
            ])
            .is_err()
        );
    }
    #[test]
    fn key_settings_patch_preserves_partial_fields_and_checks_readback() {
        let p = key_settings_patch(&[
            ("enabled_mask".into(), "3".into()),
            ("win_rgb".into(), "65280".into()),
            ("auto_reset_ms".into(), "10000".into()),
        ])
        .unwrap();
        assert_eq!(p.len(), 17);
        assert_eq!(p[0], 37);
        assert_eq!(&p[6..9], &[0, 255, 0]);
        assert!(key_settings_valid(&p));
        let mut s = read(&fixture()).status;
        s.key_game = serde_json::json!({"enabled_mask":3,"win_rgb":65280,"auto_reset_ms":10000});
        assert!(key_settings_verify(&s, &p));
        s.key_game["enabled_mask"] = 15.into();
        assert!(!key_settings_verify(&s, &p));
        assert!(key_settings_patch(&[("enabled_mask".into(), "0".into())]).is_err());
        assert!(key_settings_patch(&[("extended_effects".into(), "yes".into())]).is_err());
        assert!(key_settings_patch(&[("auto_reset_ms".into(), "86400001".into())]).is_err());
    }
    fn fixture() -> serde_json::Value {
        serde_json::json!({"status":{"version":crate::peer_identity::VERSION,"hostname":"ledboard","board_id":"esp32s3-fixture","boot_id":"a".repeat(32),
            "http_peer_fencing":true,"pixels":480,"mode":"cell","cells":vec!["white";80],
            "led_output":{"enabled":true,"mode":"static"},
            "capabilities":{"board_kind":"table","acknowledged_features":["table","leds"],
                "table":{"columns":10,"rows":8,"pixels_per_cell":6},
                "led_output":{"backend":"gpio-static","rmt":false,"dma":false}}},
            "physical":(1..=80).collect::<Vec<_>>(),"names":vec!["cell";80]})
    }
    fn state_packet(opcode: u8) -> Vec<u8> {
        let mut p = vec![opcode];
        p.extend([0xaa; 16]);
        p.extend([1, 0, 1, 255, 0, 1, 0]);
        p.extend([0; 20]);
        p
    }
    #[test]
    fn compact_usb_readback_validates_all_values_without_renewing_full_observations() {
        let mut previous = read(&fixture()).status;
        previous.pixel_frame=Some(serde_json::from_value(serde_json::json!({"rgb":"000000".repeat(480),"physical":(1..=80).collect::<Vec<_>>(),"revision":9,"brightness":255})).unwrap());
        let mut p = state_packet(0x4c);
        p[24] = 1;
        p[43] = 2 << 6;
        let next = command_state(&previous, 0x4c, &p).unwrap();
        assert!(
            !next
                .capabilities
                .as_ref()
                .unwrap()
                .led_output()
                .unwrap()
                .dma
        );
        assert_eq!(next.cells[0], "red");
        assert_eq!(next.cells[79], "off");
        assert_eq!(next.cells[1], "white");
        assert!(next.pixel_frame.is_none());
        assert_eq!(
            serde_json::to_value(&next.capabilities).unwrap(),
            serde_json::to_value(&previous.capabilities).unwrap()
        );
        assert_eq!(next.board_id, previous.board_id);
        assert_eq!(previous.cells[0], "white");
        for (index, value) in [
            (0, 0x4d),
            (1, 0xbb),
            (17, 4),
            (18, 10),
            (19, 81),
            (20, 0),
            (21, 4),
            (22, 2),
            (23, 2),
            (24, 3),
        ] {
            let mut bad = p.clone();
            bad[index] = value;
            assert!(command_state(&previous, 0x4c, &bad).is_err(), "{index}");
        }
        assert!(command_state(&previous, 0x4c, &p[..43]).is_err());
        assert!(command_state(&previous, 0x55, &p).is_err());
        previous.capabilities.as_mut().unwrap().role = "first-wins".into();
        assert_eq!(command_state(&previous, 0x4c, &p).unwrap().cells.len(), 18);
    }
    #[test]
    fn usb_priority_yields_only_unsent_background_chunks_and_matches_ack_opcode_sequence() {
        assert!(yield_snapshot(true, true, true, false, false));
        assert!(!yield_snapshot(true, true, true, true, false)); // In flight.
        assert!(!yield_snapshot(true, true, true, false, true)); // Post-ACK verification.
        assert!(!yield_snapshot(true, false, true, false, false)); // Initial identity.
        assert!(!yield_snapshot(false, true, true, false, false));
        let status = read(&fixture()).status;
        let gate = crate::connection::Gate::default();
        let epoch = gate.select(Some(crate::connection::Method::Serial));
        let mut active = Operation {
            intent: Intent {
                opcode: 0x4c,
                payload: vec![1, 1, 0],
                reply: None,
                admission: Admission {
                    stamp: gate.stamp(crate::connection::Method::Serial, epoch),
                    session: 1,
                    peer: peer_http::Pin::from_status(&status),
                    at: Instant::now(),
                    batch: None,
                },
            },
            sequence: 13,
            sent: Instant::now(),
            ack: None,
        };
        assert!(active.matches(0x80, 13, &[0x4c, 0]));
        assert!(active.matches(0x82, 13, &[COMMAND, 6]));
        assert!(!active.matches(0x80, 12, &[0x4c, 0]));
        assert!(!active.matches(0x80, 13, &[0x4d, 0]));
        assert!(!active.matches(0x90, 13, &[0x4c, 0]));
        assert!(!active.matches(0x80, 13, &[COMMAND, 0]));
        assert!(snapshot_admitted(false, None, true));
        assert!(!snapshot_admitted(true, None, true));
        assert!(!snapshot_admitted(false, Some(&active), true));
        active.ack = Some(Instant::now());
        assert!(!snapshot_admitted(true, Some(&active), true));
        assert!(
            snapshot_admitted(true, Some(&active), false),
            "Legacy post-ACK readback cannot wait behind queued mutations"
        );
        active.ack = Some(Instant::now() - Duration::from_millis(101));
        assert!(
            snapshot_admitted(true, Some(&active), true),
            "Missing companion fallback must progress with another command queued"
        );
        active.ack = Some(Instant::now());
        active.intent.opcode = 0x67;
        assert!(
            snapshot_admitted(true, Some(&active), true),
            "Settings need full readback immediately after ACK"
        );
        active.intent.opcode = 0x5c;
        active.intent.payload = vec![3, 2];
        assert!(active.intent.needs_full_snapshot());
        assert!(snapshot_admitted(true, Some(&active), true),
            "Backend selection cannot be confirmed by the compact state-only companion");
    }
    #[test]
    fn actual_usb_worker_preempts_large_preview_and_confirms_compact_board_readback() {
        use std::{
            collections::VecDeque,
            io::{Read, Write},
            sync::{
                Arc,
                atomic::{AtomicUsize, Ordering},
                mpsc,
            },
            thread,
        };
        struct Port {
            pending: VecDeque<u8>,
            capture: Vec<u8>,
            captures: Arc<AtomicUsize>,
        }
        impl Write for Port {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                let (op, seq, p) = crate::decode_envelope(&bytes[..bytes.len() - 1]).unwrap();
                match op {
                    0x01 => {
                        assert_eq!(p, crate::peer_identity::hello());
                        let mut hello = vec![0, 2, 0, 0, 80, 9];
                        hello.extend_from_slice(&crate::peer_identity::hello()[1..]);
                        hello.extend([0xaa; 16]);
                        self.pending.extend(crate::wire_frame(0x81, seq, &hello));
                    }
                    crate::peer_identity::USB_REPORT => {
                        assert!(p.len() <= 48);
                        self.pending.extend(crate::wire_frame(
                            0x80,
                            seq,
                            &[crate::peer_identity::USB_REPORT, 0],
                        ));
                    }
                    crate::OP_GET_STATUS => {
                        let mut s = vec![0; 46];
                        s[44] = 1;
                        s[45] = 1;
                        self.pending
                            .extend(crate::wire_frame(crate::OP_STATUS, seq, &s));
                    }
                    READ => {
                        let offset = u16::from_le_bytes([p[1], p[2]]) as usize;
                        if offset == 0 {
                            self.captures.fetch_add(1, Ordering::SeqCst);
                        }
                        // A deliberately slow large preview, not a fake fast scheduler.
                        thread::sleep(Duration::from_millis(2));
                        self.pending.extend(crate::wire_frame(
                            RESPONSE,
                            seq,
                            &chunk(&self.capture, offset, p[0], 17),
                        ));
                    }
                    COMMAND => {
                        assert_eq!(&p[..16], &[0xaa; 16]);
                        assert_eq!(&p[16..], &[0x4c, 1, 1, 0]);
                        // Unexpected same-sequence status/error do not satisfy ACK.
                        self.pending
                            .extend(crate::wire_frame(0x82, seq, &[0x4d, 5]));
                        self.pending
                            .extend(crate::wire_frame(0x80, seq, &[0x4c, 0]));
                        let mut state = state_packet(0x4c);
                        state[24] = 1;
                        self.pending
                            .extend(crate::wire_frame(COMMAND_STATE, seq, &state));
                    }
                    _ => panic!("unexpected opcode {op:x}"),
                }
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        impl Read for Port {
            fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
                if self.pending.is_empty() {
                    thread::sleep(Duration::from_millis(1));
                    return Err(std::io::ErrorKind::TimedOut.into());
                }
                // Fragment packets; exercise the real framing/ACK decoder.
                let n = bytes.len().min(13).min(self.pending.len());
                for value in &mut bytes[..n] {
                    *value = self.pending.pop_front().unwrap();
                }
                Ok(n)
            }
        }
        let mut value = fixture();
        value["status"]["mode"] = "cell".into();
        value["padding"] = "x".repeat(6000).into();
        let captures = Arc::new(AtomicUsize::new(0));
        let mut port = Some(Port {
            pending: VecDeque::new(),
            capture: serde_json::to_vec(&value).unwrap(),
            captures: captures.clone(),
        });
        let (tx, rx) = mpsc::channel();
        let (events, received) = mpsc::channel();
        let gate = crate::connection::Gate::default();
        let epoch = gate.select(Some(crate::connection::Method::Serial));
        let owner = gate.clone();
        let worker = thread::spawn(move || {
            crate::serial_worker_with(rx, events, eframe::egui::Context::default(), owner, |_| {
                port.take().ok_or("unexpected reopen".into())
            })
        });
        tx.send(crate::SerialCommand::Port("fixture".into(), epoch))
            .unwrap();
        tx.send(crate::SerialCommand::Demand(
            "fixture".into(),
            epoch,
            crate::usb_demand::Demand::metadata(),
        ))
        .unwrap();
        let mut snapshot = None;
        let mut session = 0;
        while snapshot.is_none() {
            match received.recv_timeout(Duration::from_secs(4)).unwrap() {
                crate::Event::ForConnection(_, event) => match *event {
                    crate::Event::SerialSnapshot(_, s) => snapshot = Some(s.status),
                    crate::Event::SerialOwner(_, s) => session = s,
                    _ => {}
                },
                _ => {}
            }
        }
        let until = Instant::now() + Duration::from_secs(3);
        while captures.load(Ordering::SeqCst) < 2 {
            assert!(Instant::now() < until);
            thread::sleep(Duration::from_millis(1));
        }
        let before = Instant::now();
        let admission = Admission {
            stamp: gate.stamp(crate::connection::Method::Serial, epoch),
            session,
            peer: peer_http::Pin::from_status(snapshot.as_ref().unwrap()),
            at: before,
            batch: None,
        };
        let (reply, result) = mpsc::channel();
        tx.send(crate::SerialCommand::Bridge(
            "fixture".into(),
            admission,
            0x4c,
            vec![1, 1, 0],
            crate::bridge::Reply::new(
                reply,
                std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            ),
        ))
        .unwrap();
        let mut confirmed = false;
        loop {
            if let crate::Event::ForConnection(_, event) =
                received.recv_timeout(Duration::from_secs(2)).unwrap()
            {
                match *event {
                    crate::Event::SerialCommandState(_, s) => {
                        assert_eq!(s.cells[0], "red");
                        confirmed = true;
                    }
                    crate::Event::SerialTiming(_, timing) => {
                        assert!(confirmed);
                        assert_eq!(timing.confirmation, "compact board readback");
                        assert!(timing.total_ms < 250., "{}", timing.total_ms);
                        break;
                    }
                    crate::Event::SerialOperation(_, Err(error)) => panic!("{error}"),
                    _ => {}
                }
            }
        }
        assert_eq!(
            captures.load(Ordering::SeqCst),
            2,
            "No post-ACK full preview is required"
        );
        assert_eq!(
            result
                .recv_timeout(Duration::from_secs(1))
                .unwrap()
                .unwrap()
                .cells[0],
            "red",
            "Bridge receipt requires this real worker's sequence/ACK/readback"
        );
        loop {
            if let crate::Event::ForConnection(_, event) =
                received.recv_timeout(Duration::from_secs(2)).unwrap()
            {
                if let crate::Event::SerialSnapshot(_, s) = *event {
                    assert!(
                        s.observed_at < before,
                        "Resume the original immutable capture instead of starving full observations"
                    );
                    assert_eq!(
                        s.status.cells[0], "red",
                        "Old background data must not roll command state back"
                    );
                    assert!(s.status.pixel_frame.is_none());
                    break;
                }
            }
        }
        gate.select(None);
        drop(tx);
        worker.join().unwrap();
    }
    fn chunk(bytes: &[u8], offset: usize, nonce: u8, token: u32) -> Vec<u8> {
        let mut p = vec![nonce];
        p.extend(token.to_le_bytes());
        p.extend((bytes.len() as u16).to_le_bytes());
        p.extend((offset as u16).to_le_bytes());
        p.extend_from_slice(&bytes[offset..(offset + 39).min(bytes.len())]);
        p
    }
    #[test]
    fn usb_demand_actual_worker_has_no_full_capture_without_reader_and_releases_lease() {
        use std::{
            collections::VecDeque,
            io::{Read, Write},
            sync::{Arc, Mutex, mpsc},
            thread,
        };
        struct Port {
            pending: VecDeque<u8>,
            capture: Vec<u8>,
            writes: Arc<Mutex<Vec<(u8, Vec<u8>)>>>,
        }
        impl Write for Port {
            fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
                let (op, seq, p) = crate::decode_envelope(&b[..b.len() - 1]).unwrap();
                self.writes.lock().unwrap().push((op, p.clone()));
                match op {
                    1 => {
                        let mut hello = vec![0, 2, 0, 0, 80, 9];
                        hello.extend_from_slice(&crate::peer_identity::hello()[1..]);
                        hello.extend([0xaa; 16]);
                        self.pending.extend(crate::wire_frame(0x81, seq, &hello));
                    }
                    crate::peer_identity::USB_REPORT => self.pending.extend(crate::wire_frame(
                        0x80,
                        seq,
                        &[crate::peer_identity::USB_REPORT, 0],
                    )),
                    crate::OP_GET_STATUS => {
                        let mut status = vec![0; 47];
                        status[44..47].fill(1);
                        self.pending
                            .extend(crate::wire_frame(crate::OP_STATUS, seq, &status));
                    }
                    crate::usb_demand::REQUEST => {
                        self.pending
                            .extend(crate::wire_frame(crate::usb_demand::RESPONSE, seq, &p))
                    }
                    READ => {
                        let offset = u16::from_le_bytes([p[1], p[2]]) as usize;
                        self.pending.extend(crate::wire_frame(
                            RESPONSE,
                            seq,
                            &chunk(&self.capture, offset, p[0], 17),
                        ));
                    }
                    _ => panic!("unexpected {op:x}"),
                }
                Ok(b.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        impl Read for Port {
            fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
                if self.pending.is_empty() {
                    thread::sleep(Duration::from_millis(1));
                    return Err(std::io::ErrorKind::TimedOut.into());
                }
                let n = b.len().min(self.pending.len());
                for v in &mut b[..n] {
                    *v = self.pending.pop_front().unwrap();
                }
                Ok(n)
            }
        }
        let writes = Arc::new(Mutex::new(Vec::new()));
        let mut port = Some(Port {
            pending: VecDeque::new(),
            capture: serde_json::to_vec(&fixture()).unwrap(),
            writes: writes.clone(),
        });
        let (tx, rx) = mpsc::channel();
        let (events, received) = mpsc::channel();
        let gate = crate::connection::Gate::default();
        let epoch = gate.select(Some(crate::Transport::Serial));
        let owner = gate.clone();
        let worker = thread::spawn(move || {
            crate::serial_worker_with(rx, events, eframe::egui::Context::default(), owner, |_| {
                port.take().ok_or("unexpected reopen".into())
            })
        });
        tx.send(crate::SerialCommand::Port("fixture".into(), epoch))
            .unwrap();
        thread::sleep(Duration::from_millis(1100));
        assert!(
            !writes.lock().unwrap().iter().any(|(op, _)| *op == READ),
            "Connecting alone is not a high-bandwidth consumer"
        );
        tx.send(crate::SerialCommand::Demand(
            "fixture".into(),
            epoch,
            crate::usb_demand::Demand {
                flags: crate::usb_demand::STATUS | crate::usb_demand::PIXELS,
                table: true,
            },
        ))
        .unwrap();
        loop {
            if let crate::Event::ForConnection(_, event) =
                received.recv_timeout(Duration::from_secs(3)).unwrap()
            {
                if matches!(*event, crate::Event::SerialSnapshot(..)) {
                    break;
                }
            }
        }
        tx.send(crate::SerialCommand::Demand(
            "fixture".into(),
            epoch,
            Default::default(),
        ))
        .unwrap();
        thread::sleep(Duration::from_millis(100));
        let count = writes
            .lock()
            .unwrap()
            .iter()
            .filter(|(op, _)| *op == READ)
            .count();
        thread::sleep(Duration::from_millis(1150));
        assert_eq!(
            count,
            writes
                .lock()
                .unwrap()
                .iter()
                .filter(|(op, _)| *op == READ)
                .count(),
            "No cyclic capture after reader hides/closes"
        );
        assert!(
            writes
                .lock()
                .unwrap()
                .iter()
                .any(|(op, p)| *op == crate::usb_demand::REQUEST && p == &[9, 0, 0xb8, 0x0b])
        );
        assert!(
            writes
                .lock()
                .unwrap()
                .iter()
                .any(|(op, p)| *op == crate::usb_demand::REQUEST && p == &[0, 0, 0, 0])
        );
        gate.select(None);
        drop(tx);
        worker.join().unwrap();
    }
    fn read(value: &serde_json::Value) -> Snapshot {
        let bytes = serde_json::to_vec(value).unwrap();
        let mut reader = Reader::new(9);
        let mut result = None;
        for offset in (0..bytes.len()).step_by(39) {
            result = reader.accept(&chunk(&bytes, offset, 9, 17)).unwrap();
        }
        result.unwrap()
    }
    #[test]
    fn immutable_usb_snapshot_requires_complete_identity_states_and_unique_mapping() {
        let mut value = fixture();
        let snapshot = read(&value);
        assert_eq!(snapshot.status.cells.len(), 80);
        assert_eq!(snapshot.mappings.len(), 80);
        assert!(snapshot.status.pixel_frame.is_none());
        for field in ["boot_id", "board_id"] {
            let mut invalid = value.clone();
            invalid["status"][field] = serde_json::json!("");
            let bytes = serde_json::to_vec(&invalid).unwrap();
            let mut reader = Reader::new(9);
            let mut failed = false;
            for offset in (0..bytes.len()).step_by(39) {
                if reader.accept(&chunk(&bytes, offset, 9, 17)).is_err() {
                    failed = true;
                    break;
                }
            }
            assert!(failed);
        }
        value["physical"] = serde_json::json!(vec![1; 80]);
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut reader = Reader::new(9);
        assert!(
            (0..bytes.len())
                .step_by(39)
                .any(|offset| reader.accept(&chunk(&bytes, offset, 9, 17)).is_err())
        );
    }
    #[test]
    fn winner_board_usb_snapshot_accepts_eighteen_twelve_pixel_groups() {
        let mut value = fixture();
        value["status"]["capabilities"]["role"] = "first-wins".into();
        value["status"]["mode"] = "first-wins".into();
        value["status"]["pixels"] = 216.into();
        value["status"]["cells"] = serde_json::json!(vec!["white"; 18]);
        value["physical"] = serde_json::json!((1..=18).collect::<Vec<_>>());
        value["names"] = serde_json::json!(vec!["ring"; 18]);
        value["pixels"] = serde_json::json!({"revision":7,"brightness":255,"pending":false,
            "physical":(1..=18).collect::<Vec<_>>(),"rgb":format!("{}{}{}","ff0000".repeat(12),"00ff00".repeat(12),"0000ff".repeat(192))});
        let snapshot = read(&value);
        assert!(snapshot.mappings.is_empty());
        let frame = snapshot.status.pixel_frame.unwrap();
        assert_eq!(
            frame.key_group(1, 0).unwrap(),
            vec![eframe::egui::Color32::RED; 12]
        );
        assert_eq!(
            frame.key_group(1, 1).unwrap(),
            vec![eframe::egui::Color32::GREEN; 12]
        );
        assert!(frame.cell(1).is_none());
    }
    #[test]
    fn pixel_groups_usb_snapshot_preserves_compact_geometry_and_semantic_won_state() {
        let mut value=fixture();
        value["status"]["capabilities"]["acknowledged_features"].as_array_mut().unwrap().push(crate::pixel_groups::FEATURE.into());
        let mut layout=crate::pixel_groups::Layout::uniform(480,6).unwrap();
        layout.revision=3;layout.set_override(1,5);layout.set_override(80,7);
        layout.counts[0]=5;layout.counts[79]=7;
        let mut start=0;layout.starts=layout.counts.iter().map(|&count|{let next=start;start+=u16::from(count);next}).collect();
        value["status"]["pixel_groups"]=serde_json::to_value(&layout).unwrap();
        value["status"]["pixel_groups"]["partial"]=true.into();
        value["status"]["pixel_groups"].as_object_mut().unwrap().remove("overrides");
        value["status"]["pixel_groups"].as_object_mut().unwrap().remove("starts");
        value["status"]["cell_states"]=serde_json::json!(vec!["won";80]);
        value["pixels"]=serde_json::json!({"revision":9,"brightness":255,"pending":false,"rgb_omitted":true,
            "group_layout_revision":3,"group_counts":layout.counts,"group_starts":layout.starts});
        value["pixel_frame_partial"]=true.into();
        let snapshot=read(&value);assert_eq!(snapshot.status.cells[0],"won");
        assert!(snapshot.status.pixel_frame.is_none());
        assert_eq!(crate::pixel_groups::for_status(&snapshot.status).unwrap().span(80),Some((473,7)));
        assert_eq!(snapshot.status.pixel_frame_partial["revision"],9);
    }
    #[test]
    fn pixel_groups_partial_snapshot_preserves_only_explicit_same_peer_application_icon() {
        use base64::Engine;
        let mut png=vec![0;33];png[..8].copy_from_slice(b"\x89PNG\r\n\x1a\n");png[12..16].copy_from_slice(b"IHDR");
        png[16..20].copy_from_slice(&8_u32.to_be_bytes());png[20..24].copy_from_slice(&8_u32.to_be_bytes());
        let icon=base64::engine::general_purpose::STANDARD.encode(png);
        let mut value=fixture();value["status"]["capabilities"]["application"]=serde_json::json!({"id":"table","name":"Table","title":"Table","icon_png_base64":icon});
        let previous=read(&value).status;
        value["status"]["capabilities"]["application"].as_object_mut().unwrap().remove("icon_png_base64");
        value["status_partial"]=true.into();value["omitted_fields"]=serde_json::json!(["capabilities.application.icon_png_base64"]);
        let mut partial=read(&value);partial.retain_icon(Some(&previous));
        assert_eq!(partial.status.capabilities.unwrap().application.unwrap().icon_png_base64,icon);
        for mutation in [0,1,2] {
            let mut invalid=value.clone();
            if mutation==0 {invalid["status"]["boot_id"]="b".repeat(32).into();}
            if mutation==1 {invalid["status"]["capabilities"]["application"]["id"]="other".into();}
            if mutation==2 {invalid["omitted_fields"]=serde_json::json!(["capabilities"]);}
            let mut partial=read(&invalid);partial.retain_icon(Some(&previous));
            assert!(partial.status.capabilities.unwrap().application.unwrap().icon_png_base64.is_empty());
        }
    }
    #[test]
    fn frozen_usb_rgb_reuses_published_frame_parser_and_capture_time() {
        let mut value = fixture();
        value["pixels"] = serde_json::json!({"rgb":"112233".repeat(480),
            "physical":(1..=80).rev().collect::<Vec<_>>(),"revision":7,"brightness":203,"pending":true});
        // A larger coherent capture is supported without another transport.
        value["optional_metadata"] = serde_json::Value::String("x".repeat(8192));
        let bytes = serde_json::to_vec(&value).unwrap();
        assert!(bytes.len() > 8192 && bytes.len() < 16384);
        let mut reader = Reader::new(9);
        let captured_at = reader.started.unwrap();
        let mut snapshot = None;
        for offset in (0..bytes.len()).step_by(39) {
            snapshot = reader.accept(&chunk(&bytes, offset, 9, 17)).unwrap();
        }
        let snapshot = snapshot.unwrap();
        assert_eq!(snapshot.observed_at, captured_at);
        assert_eq!(snapshot.status.cells.len(), 80);
        assert_eq!(snapshot.mappings.len(), 80);
        assert!(peer_http::Pin::from_status(&snapshot.status).is_some());
        assert!(snapshot.status.pixel_preview_requested);
        assert!(snapshot.status.pixel_frame.is_none());
        value["pixels"]["pending"] = false.into();
        let published = read(&value);
        let frame = published.status.pixel_frame.unwrap();
        assert_eq!(frame.revision, 7);
        assert_eq!(frame.brightness, 203);
        let (first, colors) = frame.cell(1).unwrap();
        assert_eq!(first, 474);
        assert!(
            colors
                .iter()
                .all(|color| *color == eframe::egui::Color32::from_rgb(17, 34, 51))
        );
        // Pending cannot excuse malformed RGB or a corrupt physical mapping.
        value["pixels"]["pending"] = true.into();
        value["pixels"]["rgb"] = "zz0000".repeat(480).into();
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut reader = Reader::new(9);
        assert!(
            (0..bytes.len())
                .step_by(39)
                .any(|offset| reader.accept(&chunk(&bytes, offset, 9, 17)).is_err())
        );
        value["pixels"]["rgb"] = "112233".repeat(480).into();
        value["pixels"]["physical"] = serde_json::json!(vec![1; 80]);
        let bytes = serde_json::to_vec(&value).unwrap();
        let mut reader = Reader::new(9);
        assert!(
            (0..bytes.len())
                .step_by(39)
                .any(|offset| reader.accept(&chunk(&bytes, offset, 9, 17)).is_err())
        );
        let mut reader = Reader::new(9);
        let mut oversized = chunk(&bytes, 0, 9, 17);
        oversized[5..7].copy_from_slice(&16385_u16.to_le_bytes());
        assert!(reader.accept(&oversized).is_err());
    }
    #[test]
    fn chunks_reject_wrong_nonce_token_offset_and_total_without_promotion() {
        let bytes = serde_json::to_vec(&fixture()).unwrap();
        let mut reader = Reader::new(9);
        assert_eq!(reader.request(), vec![9, 0, 0, 0, 0, 0, 0]);
        assert!(reader.accept(&chunk(&bytes, 0, 8, 17)).is_err());
        assert!(reader.accept(&chunk(&bytes, 0, 9, 0)).is_err());
        assert!(reader.accept(&chunk(&bytes, 0, 9, 17)).unwrap().is_none());
        assert!(reader.accept(&chunk(&bytes, 39, 9, 18)).is_err());
        assert!(reader.accept(&chunk(&bytes, 0, 9, 17)).is_err());
        let mut wrong = chunk(&bytes, 39, 9, 17);
        wrong[5] ^= 1;
        assert!(reader.accept(&wrong).is_err());
    }
    #[test]
    fn read_window_is_token_bound_small_and_never_requests_past_capture() {
        let bytes = serde_json::to_vec(&fixture()).unwrap();
        let mut reader = Reader::new(9);
        assert!(reader.following_requests().is_empty());
        reader.accept(&chunk(&bytes, 0, 9, 17)).unwrap();
        let following = reader.following_requests();
        assert_eq!(following.len(), 3);
        for (i, p) in following.iter().enumerate() {
            assert_eq!(p[0], 9);
            assert_eq!(u16::from_le_bytes([p[1], p[2]]) as usize, 78 + 39 * i);
            assert_eq!(&p[3..], &17_u32.to_le_bytes());
        }
        for at in (39..bytes.len()).step_by(39) {
            for p in reader.following_requests() {
                assert!((u16::from_le_bytes([p[1], p[2]]) as usize) < bytes.len());
            }
            reader.accept(&chunk(&bytes, at, 9, 17)).unwrap();
        }
        assert!(reader.following_requests().is_empty());
    }
    #[test]
    fn actual_usb_worker_windows_read_latency_does_not_expire_large_capture() {
        use std::{
            collections::VecDeque,
            io::{Read, Write},
            sync::mpsc,
            thread,
        };
        struct Port {
            pending: VecDeque<u8>,
            capture: Vec<u8>,
            stall_until: Option<Instant>,
            delayed: bool,
        }
        impl Write for Port {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                let (op, seq, p) = crate::decode_envelope(&bytes[..bytes.len() - 1]).unwrap();
                match op {
                    0x01 => {
                        assert_eq!(p, crate::peer_identity::hello());
                        let mut hello = vec![0, 2, 0, 0, 80, 9];
                        hello.extend_from_slice(&crate::peer_identity::hello()[1..]);
                        hello.extend([0xaa; 16]);
                        self.pending.extend(crate::wire_frame(0x81, seq, &hello));
                    }
                    crate::peer_identity::USB_REPORT => {
                        assert!(p.len() <= 48);
                        self.pending.extend(crate::wire_frame(
                            0x80,
                            seq,
                            &[crate::peer_identity::USB_REPORT, 0],
                        ));
                    }
                    crate::OP_GET_STATUS => {
                        let mut s = vec![0; 46];
                        s[44] = 1;
                        s[45] = 1;
                        self.pending
                            .extend(crate::wire_frame(crate::OP_STATUS, seq, &s));
                    }
                    READ => {
                        let at = u16::from_le_bytes([p[1], p[2]]) as usize;
                        // Real main-loop work may pause a READ window for >2s.
                        // The complete capture still must fit the SAME8s age.
                        if at > 300 && !self.delayed {
                            self.delayed = true;
                            self.stall_until = Some(Instant::now() + Duration::from_millis(2200));
                        }
                        self.pending.extend(crate::wire_frame(
                            RESPONSE,
                            seq,
                            &chunk(&self.capture, at, p[0], 17),
                        ));
                    }
                    _ => panic!("Unexpected mutation in background-read test"),
                }
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        impl Read for Port {
            fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
                thread::sleep(Duration::from_millis(35));
                if self.stall_until.is_some_and(|until| Instant::now() < until) {
                    return Err(std::io::ErrorKind::TimedOut.into());
                }
                if self.pending.is_empty() {
                    return Err(std::io::ErrorKind::TimedOut.into());
                }
                let count = bytes.len().min(self.pending.len());
                for b in &mut bytes[..count] {
                    *b = self.pending.pop_front().unwrap();
                }
                Ok(count)
            }
        }
        let mut value = fixture();
        value["padding"] = "x".repeat(10000).into();
        let mut port = Some(Port {
            pending: VecDeque::new(),
            capture: serde_json::to_vec(&value).unwrap(),
            stall_until: None,
            delayed: false,
        });
        let (tx, rx) = mpsc::channel();
        let (events, received) = mpsc::channel();
        let gate = crate::connection::Gate::default();
        let epoch = gate.select(Some(crate::connection::Method::Serial));
        let owner = gate.clone();
        let worker = thread::spawn(move || {
            crate::serial_worker_with(rx, events, eframe::egui::Context::default(), owner, |_| {
                port.take().ok_or("Unexpected reopen".into())
            })
        });
        tx.send(crate::SerialCommand::Port("fixture".into(), epoch))
            .unwrap();
        tx.send(crate::SerialCommand::Demand(
            "fixture".into(),
            epoch,
            crate::usb_demand::Demand::metadata(),
        ))
        .unwrap();
        let start = Instant::now();
        loop {
            let crate::Event::ForConnection(_, event) =
                received.recv_timeout(Duration::from_secs(7)).unwrap()
            else {
                continue;
            };
            match *event {
                crate::Event::SerialSnapshot(_, s) => {
                    assert_eq!(s.status.cells.len(), 80);
                    // Capture admission starts AFTER HELLO/report, not when
                    // the serial handle opens. Keep the SAME8s capture lease.
                    assert!(s.observed_at.elapsed() < MAX_SNAPSHOT_AGE);
                    assert!(start.elapsed() < MAX_SNAPSHOT_AGE + Duration::from_secs(4));
                    break;
                }
                crate::Event::SerialFailure(_, e) => panic!("{e}"),
                _ => {}
            }
        }
        gate.select(None);
        drop(tx);
        worker.join().unwrap();
    }
    #[test]
    fn table_write_is_exact_boot_fenced_and_post_ack_values_are_not_assumed() {
        let snapshot = read(&fixture());
        let s = snapshot.status;
        let packet = fenced(&s, 0x4c, &[1, 1, 0]).unwrap();
        assert_eq!(&packet[..16], &[0xaa; 16]);
        assert_eq!(&packet[16..], &[0x4c, 1, 1, 0]);
        assert!(fenced(&s, 0x55, &[0]).is_err());
        assert!(fenced(&s, 0x4c, &[0; 32]).is_err());
        let gate = crate::connection::Gate::default();
        let epoch = gate.select(Some(crate::connection::Method::Serial));
        let admission = Admission {
            stamp: gate.stamp(crate::connection::Method::Serial, epoch),
            session: 1,
            peer: peer_http::Pin::from_status(&s),
            at: Instant::now(),
            batch: None,
        };
        let intent = Intent {
            opcode: 0x4c,
            payload: vec![1, 1, 0],
            admission,
            reply: None,
        };
        assert!(intent.current(1, Some(&s)));
        assert!(!intent.current(2, Some(&s)));
        assert!(!intent.verify(&s));
        let mut applied = s.clone();
        applied.cells[0] = "red".into();
        assert!(intent.verify(&applied));
        applied.boot_id = "b".repeat(32);
        assert!(!intent.current(1, Some(&applied)));
        gate.select(Some(crate::connection::Method::Serial));
        assert!(!intent.current(1, Some(&s)));
    }
    #[test]
    fn actual_usb_worker_rejects_unsupported_hello_without_live_status_or_retry() {
        use std::{
            collections::VecDeque,
            io::{Read, Write},
            sync::{
                Arc,
                atomic::{AtomicUsize, Ordering},
                mpsc,
            },
            thread,
        };
        struct Port {
            pending: VecDeque<u8>,
            writes: Arc<AtomicUsize>,
        }
        impl Write for Port {
            fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
                let (op, seq, p) = crate::decode_envelope(&b[..b.len() - 1]).unwrap();
                assert_eq!(op, 0x01);
                assert_eq!(p, crate::peer_identity::hello());
                self.writes.fetch_add(1, Ordering::SeqCst);
                self.pending
                    .extend(crate::wire_frame(0x81, seq, &[0, 2, 0, 0, 80, 9, 0, 2, 6]));
                Ok(b.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        impl Read for Port {
            fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
                if self.pending.is_empty() {
                    return Err(std::io::ErrorKind::TimedOut.into());
                }
                let n = b.len().min(self.pending.len());
                for v in &mut b[..n] {
                    *v = self.pending.pop_front().unwrap();
                }
                Ok(n)
            }
        }
        let writes = Arc::new(AtomicUsize::new(0));
        let mut port = Some(Port {
            pending: VecDeque::new(),
            writes: writes.clone(),
        });
        let (tx, rx) = mpsc::channel();
        let (events, received) = mpsc::channel();
        let gate = crate::connection::Gate::default();
        let epoch = gate.select(Some(crate::Transport::Serial));
        let owner = gate.clone();
        let task = thread::spawn(move || {
            crate::serial_worker_with(rx, events, eframe::egui::Context::default(), owner, |_| {
                port.take()
                    .ok_or("Must not retry an unsupported peer".into())
            })
        });
        tx.send(crate::SerialCommand::Port("fixture".into(), epoch))
            .unwrap();
        tx.send(crate::SerialCommand::Demand(
            "fixture".into(),
            epoch,
            crate::usb_demand::Demand::metadata(),
        ))
        .unwrap();
        loop {
            if let crate::Event::ForConnection(_, event) =
                received.recv_timeout(Duration::from_secs(2)).unwrap()
            {
                match *event {
                    crate::Event::SerialFailure(_, error) => {
                        assert!(crate::peer_identity::unsupported(&error));
                        break;
                    }
                    crate::Event::Serial(_, true, _) | crate::Event::SerialSnapshot(..) => {
                        panic!("Unsupported peer became live")
                    }
                    _ => {}
                }
            }
        }
        drop(tx);
        task.join().unwrap();
        assert_eq!(writes.load(Ordering::SeqCst), 1);
    }
    #[test]
    fn usb_report_optional_actual_worker_keeps_board_boot_and_error_admission() {
        use std::{
            collections::VecDeque,
            io::{Read, Write},
            sync::{
                Arc,
                atomic::{AtomicUsize, Ordering},
                mpsc,
            },
            thread,
        };
        struct Port {
            pending: VecDeque<u8>,
            capture: Vec<u8>,
            reply: Option<Vec<u8>>,
            hello_core: Vec<u8>,
            reports: Arc<AtomicUsize>,
        }
        impl Write for Port {
            fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
                let (op, seq, p) = crate::decode_envelope(&b[..b.len() - 1]).unwrap();
                match op {
                    0x01 => {
                        assert_eq!(p, crate::peer_identity::hello());
                        let mut hello = vec![0, 2, 0, 0, 80, 9];
                        hello.extend_from_slice(&self.hello_core);
                        hello.extend([0xaa; 16]);
                        self.pending.extend(crate::wire_frame(0x81, seq, &hello));
                    }
                    crate::peer_identity::USB_REPORT => {
                        assert!(p.len() <= 48);
                        self.reports.fetch_add(1, Ordering::SeqCst);
                        // A stale explicit Unsupported cannot grant introduction.
                        self.pending.extend(crate::wire_frame(
                            0x82,
                            seq.wrapping_add(91),
                            &[crate::peer_identity::USB_REPORT, 2],
                        ));
                        if let Some(reply) = &self.reply {
                            self.pending.extend(crate::wire_frame(0x82, seq, reply));
                        }
                    }
                    crate::OP_GET_STATUS => {
                        let mut status = vec![0; 46];
                        status[44] = 1;
                        status[45] = 1;
                        self.pending
                            .extend(crate::wire_frame(crate::OP_STATUS, seq, &status));
                    }
                    READ => {
                        let at = u16::from_le_bytes([p[1], p[2]]) as usize;
                        self.pending.extend(crate::wire_frame(
                            RESPONSE,
                            seq,
                            &chunk(&self.capture, at, p[0], 17),
                        ));
                    }
                    _ => panic!("Introduction may never send a mutation or fallback opcode {op:x}"),
                }
                Ok(b.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        impl Read for Port {
            fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
                if self.pending.is_empty() {
                    thread::sleep(Duration::from_millis(1));
                    return Err(std::io::ErrorKind::TimedOut.into());
                }
                let n = b.len().min(13).min(self.pending.len());
                for value in &mut b[..n] {
                    *value = self.pending.pop_front().unwrap();
                }
                Ok(n)
            }
        }
        for (reply, valid_identity, unsupported_report) in [
            (Some(vec![crate::peer_identity::USB_REPORT, 2]), true, true),
            (Some(vec![crate::peer_identity::USB_REPORT, 2]), false, true),
            (Some(vec![crate::peer_identity::USB_REPORT, 6]), true, false),
            (Some(vec![0x55, 2]), true, false),
            (None, true, false),
        ] {
            let mut value = fixture();
            let hello_core = if crate::peer_identity::enforced() {
                crate::peer_identity::hello()[1..].to_vec()
            } else {
                vec![0, 2, 7]
            };
            let version = hello_core
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(".");
            value["status"]["version"] = version.clone().into();
            if !valid_identity {
                value["status"]["board_id"] = "".into();
            }
            let reports = Arc::new(AtomicUsize::new(0));
            let mut port = Some(Port {
                pending: VecDeque::new(),
                capture: serde_json::to_vec(&value).unwrap(),
                reply,
                hello_core,
                reports: reports.clone(),
            });
            let (tx, rx) = mpsc::channel();
            let (events, received) = mpsc::channel();
            let gate = crate::connection::Gate::default();
            let epoch = gate.select(Some(crate::Transport::Serial));
            let owner = gate.clone();
            let worker = thread::spawn(move || {
                crate::serial_worker_with(
                    rx,
                    events,
                    eframe::egui::Context::default(),
                    owner,
                    |_| {
                        port.take()
                            .ok_or("No implicit reopen or retry after introduction failure".into())
                    },
                )
            });
            tx.send(crate::SerialCommand::Port("fixture".into(), epoch))
                .unwrap();
            tx.send(crate::SerialCommand::Demand(
                "fixture".into(),
                epoch,
                crate::usb_demand::Demand::metadata(),
            ))
            .unwrap();
            let may_connect =
                !crate::peer_identity::enforced() && unsupported_report && valid_identity;
            loop {
                let crate::Event::ForConnection(_, event) =
                    received.recv_timeout(Duration::from_secs(6)).unwrap()
                else {
                    continue;
                };
                match *event {
                    crate::Event::SerialSnapshot(_, snapshot) => {
                        assert!(
                            may_connect,
                            "Unsupported/error/invalid identity must not become live"
                        );
                        assert_eq!(snapshot.status.version, version);
                        assert!(peer_http::Pin::from_status(&snapshot.status).is_some());
                        assert_eq!(snapshot.status.cells.len(), 80);
                        break;
                    }
                    crate::Event::SerialFailure(_, error) => {
                        assert!(!may_connect, "{error}");
                        break;
                    }
                    crate::Event::Serial(_, true, _) => {
                        panic!("Never become live before the full identity capture")
                    }
                    _ => {}
                }
            }
            gate.select(None);
            drop(tx);
            worker.join().unwrap();
            assert_eq!(
                reports.load(Ordering::SeqCst),
                1,
                "Retire all remaining report chunks; never retry a rejection"
            );
        }
    }
    #[test]
    fn usb_report_optional_non_table_worker_requires_full_board_boot_identity() {
        use std::{
            collections::VecDeque,
            io::{Read, Write},
            sync::mpsc,
            thread,
        };
        struct Port {
            pending: VecDeque<u8>,
            identity: Vec<u8>,
            hello_core: Vec<u8>,
        }
        impl Write for Port {
            fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
                let (op, seq, p) = crate::decode_envelope(&b[..b.len() - 1]).unwrap();
                match op {
                    0x01 => {
                        assert_eq!(p, crate::peer_identity::hello());
                        let mut hello = vec![0, 2, 0, 0, 0, 0];
                        hello.extend_from_slice(&self.hello_core);
                        hello.extend([0xaa; 16]);
                        self.pending.extend(crate::wire_frame(0x81, seq, &hello));
                    }
                    crate::peer_identity::USB_REPORT => {
                        let (op, payload) = if crate::peer_identity::enforced() {
                            (0x80, vec![crate::peer_identity::USB_REPORT, 0])
                        } else {
                            (0x82, vec![crate::peer_identity::USB_REPORT, 2])
                        };
                        self.pending.extend(crate::wire_frame(op, seq, &payload));
                    }
                    crate::OP_GET_STATUS => {
                        let mut status = vec![0; 46];
                        status[31] = 1 << 5;
                        status[37] = 1;
                        self.pending
                            .extend(crate::wire_frame(crate::OP_STATUS, seq, &status));
                    }
                    0x59 => {
                        let at = u16::from_le_bytes([p[1], p[2]]) as usize;
                        self.pending.extend(crate::wire_frame(
                            0xb2,
                            seq,
                            &chunk(&self.identity, at, p[0], 1),
                        ));
                    }
                    _ => panic!("Identity bootstrap must remain read-only, not opcode {op:x}"),
                }
                Ok(b.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        impl Read for Port {
            fn read(&mut self, b: &mut [u8]) -> std::io::Result<usize> {
                if self.pending.is_empty() {
                    thread::sleep(Duration::from_millis(1));
                    return Err(std::io::ErrorKind::TimedOut.into());
                }
                let n = b.len().min(13).min(self.pending.len());
                for value in &mut b[..n] {
                    *value = self.pending.pop_front().unwrap();
                }
                Ok(n)
            }
        }
        for (field, bad) in [
            ("", None),
            ("board_id", None),
            ("boot_id", None),
            ("board_id", Some(serde_json::json!("UPPER"))),
            ("boot_id", Some(serde_json::json!("0".repeat(32)))),
            ("boot_id", Some(serde_json::json!("a".repeat(31)))),
            ("boot_id", Some(serde_json::json!("g".repeat(32)))),
        ] {
            let hello_core = if crate::peer_identity::enforced() {
                crate::peer_identity::hello()[1..].to_vec()
            } else {
                vec![0, 2, 7]
            };
            let version = hello_core
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(".");
            let mut identity = serde_json::json!({"revision":1,"hostname":"fixture","display_name":"Fixture controller",
                "version":version,"board_id":"esp32s3-fixture","boot_id":"a".repeat(32)});
            if !field.is_empty() {
                if let Some(value) = bad {
                    identity[field] = value;
                } else {
                    identity.as_object_mut().unwrap().remove(field);
                }
            }
            let mut port = Some(Port {
                pending: VecDeque::new(),
                identity: serde_json::to_vec(&identity).unwrap(),
                hello_core,
            });
            let (tx, rx) = mpsc::channel();
            let (events, received) = mpsc::channel();
            let gate = crate::connection::Gate::default();
            let epoch = gate.select(Some(crate::Transport::Serial));
            let owner = gate.clone();
            let worker = thread::spawn(move || {
                crate::serial_worker_with(
                    rx,
                    events,
                    eframe::egui::Context::default(),
                    owner,
                    |_| {
                        port.take().ok_or(
                            "Invalid identity must not implicitly reopen the physical port".into(),
                        )
                    },
                )
            });
            tx.send(crate::SerialCommand::Port("fixture".into(), epoch))
                .unwrap();
            tx.send(crate::SerialCommand::Demand(
                "fixture".into(),
                epoch,
                crate::usb_demand::Demand::metadata(),
            ))
            .unwrap();
            loop {
                let crate::Event::ForConnection(_, event) =
                    received.recv_timeout(Duration::from_secs(3)).unwrap()
                else {
                    continue;
                };
                match *event {
                    crate::Event::Serial(_, true, Some(status)) => {
                        assert!(field.is_empty(), "Missing/invalid {field} became live");
                        assert!(
                            !status.http_peer_fencing,
                            "USB identity must not invent HTTP-only support"
                        );
                        assert!(
                            peer_http::Pin::from_identity(&status.board_id, &status.boot_id)
                                .is_some()
                        );
                        assert_eq!(status.version, version);
                        break;
                    }
                    crate::Event::SerialFailure(_, error) => {
                        assert!(!field.is_empty(), "{error}");
                        assert!(error.contains("board/boot identity"), "{error}");
                        break;
                    }
                    crate::Event::SerialSnapshot(..) => panic!("This is a non-table controller"),
                    _ => {}
                }
            }
            gate.select(None);
            drop(tx);
            worker.join().unwrap();
        }
    }
    #[test]
    fn atomic_batch_uses_one_boot_fenced_bitmap_and_verifies_every_selected_state() {
        let mut status = read(&fixture()).status;
        let request = crate::cell_batch::Request::new(&[1, 80], "red", "fade").unwrap();
        assert!(fenced(&status, 0x5f, &request.payload()).is_err());
        status
            .capabilities
            .as_mut()
            .unwrap()
            .acknowledged_features
            .push("cell-batch".into());
        let packet = fenced(&status, 0x5f, &request.payload()).unwrap();
        assert_eq!(packet.len(), 29);
        assert_eq!(packet[16], 0x5f);
        let gate = crate::connection::Gate::default();
        let epoch = gate.select(Some(crate::connection::Method::Serial));
        let admission = Admission {
            stamp: gate.stamp(crate::connection::Method::Serial, epoch),
            session: 1,
            peer: peer_http::Pin::from_status(&status),
            at: Instant::now(),
            batch: Some(
                crate::cell_batch::Correlation::new(
                    request.clone(),
                    peer_http::Pin::from_status(&status).unwrap(),
                    crate::Transport::Serial,
                )
                .unwrap(),
            ),
        };
        let intent = Intent {
            opcode: 0x5f,
            payload: request.payload(),
            admission,
            reply: None,
        };
        assert!(intent.current(1, Some(&status)));
        let mut changed = Intent {
            opcode: 0x5f,
            payload: request.payload(),
            admission: intent.admission.clone(),
            reply: None,
        };
        changed.payload[2] ^= 2;
        assert!(!changed.current(1, Some(&status)));
        assert!(!intent.verify(&status));
        status.cells[0] = "red".into();
        assert!(!intent.verify(&status));
        status.cells[79] = "red".into();
        assert!(intent.verify(&status));
        status.boot_id = "b".repeat(32);
        assert!(!intent.current(1, Some(&status)));
    }
}
