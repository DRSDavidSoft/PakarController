//! One board-owned atomic selection mutation, never a fan-out/replay queue.
//! Reuses PCController Session's intent admission/ACK/no-late-write rule through
//! the existing HTTP Pin and USB Admission; the board owns transition timing.
use crate::protocol_opcodes::pakar as wire;
use crate::{DeviceStatus, Endpoint, peer_http};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub const OPCODE: u8 = wire::CELLS_ACTION;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub ids: Vec<u8>,
    pub state: String,
    pub effect: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Correlation {
    id: u64,
    pub request: Request,
    pub peer: peer_http::Pin,
    pub route: crate::Transport,
}
impl Correlation {
    pub fn new(
        request: Request,
        peer: peer_http::Pin,
        route: crate::Transport,
    ) -> Result<Self, String> {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let id = NEXT
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
                value.checked_add(1)
            })
            .map_err(|_| "Batch correlation IDs exhausted; restart before another operation")?;
        Ok(Self {
            id,
            request,
            peer,
            route,
        })
    }
}
impl Request {
    pub fn new(ids: &[u8], state: &str, effect: &str) -> Result<Self, String> {
        if ids.is_empty()
            || ids.len() > 80
            || ids.iter().any(|id| !(1..=80).contains(id))
            || !matches!(state, "idle" | "lost" | "won" | "white" | "red" | "off")
            || !matches!(effect, "immediate" | "fade" | "pulse")
        {
            return Err("Batch requires cells 1–80, idle/lost/won/off and immediate/fade/pulse".into());
        }
        let ids = ids
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        Ok(Self {
            ids,
            state: crate::cell_assignments::state(state)?,
            effect: effect.into(),
        })
    }
    pub fn supported(s: &DeviceStatus) -> bool {
        !s.programming_latch
            && s.capabilities
                .as_ref()
                .is_some_and(|m| m.table() && m.has("cell-batch"))
    }
    pub fn payload(&self) -> Vec<u8> {
        let mut payload = vec![
            match self.state.as_str() {
                "white" => 0,
                "red" => 1,
                "won" => 3,
                _ => 2,
            },
            match self.effect.as_str() {
                "immediate" => 0,
                "fade" => 1,
                _ => 2,
            },
        ];
        payload.resize(12, 0);
        for id in &self.ids {
            let index = usize::from(*id - 1);
            payload[2 + index / 8] |= 1 << (index % 8);
        }
        payload
    }
    pub fn from_payload(payload: &[u8]) -> Option<Self> {
        if payload.len() != 12 {
            return None;
        }
        let state = *["white", "red", "off", "won"].get(payload[0] as usize)?;
        let effect = *["immediate", "fade", "pulse"].get(payload[1] as usize)?;
        let ids: Vec<_> = (1_u8..=80)
            .filter(|id| {
                let i = usize::from(*id - 1);
                payload[2 + i / 8] & (1 << (i % 8)) != 0
            })
            .collect();
        Self::new(&ids, state, effect).ok()
    }
    pub fn form(&self) -> Vec<(String, String)> {
        vec![
            (
                "ids".into(),
                self.ids
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            ("state".into(), self.state.clone()),
            ("effect".into(), self.effect.clone()),
        ]
    }
    pub fn verify(&self, s: &DeviceStatus) -> bool {
        self.ids.iter().all(|id| {
            s.cells
                .get(usize::from(*id - 1))
                .is_some_and(|value| crate::cell_assignments::same_state(value, &self.state))
        })
    }
}
pub fn send(
    client: &reqwest::blocking::Client,
    endpoint: &Endpoint,
    request: &Request,
    expected: Option<&peer_http::Pin>,
    stop: Option<&AtomicBool>,
    admit: Option<&dyn Fn() -> Result<(), String>>,
) -> Result<String, String> {
    let current = crate::blocking_status(client, endpoint)?;
    send_verified(&current, endpoint, request, expected, stop, admit)
}
/// Reuse the worker's fresh, same-scope preflight; never do a second status
/// transaction before the one atomic POST. Admission is still checked at write.
pub fn send_verified(
    current: &DeviceStatus,
    endpoint: &Endpoint,
    request: &Request,
    expected: Option<&peer_http::Pin>,
    stop: Option<&AtomicBool>,
    admit: Option<&dyn Fn() -> Result<(), String>>,
) -> Result<String, String> {
    let pin = peer_http::Pin::from_status(current)
        .ok_or("Batch requires verified board/boot identity")?;
    if expected.is_some_and(|expected| expected != &pin) || !Request::supported(current) {
        return Err(
            "Atomic cell batch unavailable for this peer; nothing sent (no per-cell fallback)"
                .into(),
        );
    }
    let stopped = || stop.is_some_and(|stop| stop.load(Ordering::Acquire));
    if stopped() {
        return Err("Stopped before atomic batch dispatch".into());
    }
    let pinned = pin.client()?;
    if let Some(admit) = admit {
        admit()?;
    }
    if stopped() {
        return Err("Stopped before atomic batch dispatch".into());
    }
    let response = pinned
        .post(format!("{}/api/cells", endpoint.base.trim_end_matches('/')))
        .basic_auth(&endpoint.user, Some(&endpoint.password))
        .form(&request.form())
        .send()
        .map_err(|_| "Atomic batch outcome unknown; nothing replayed".to_string())?;
    if !response.status().is_success() {
        return Err(format!("Atomic batch rejected: HTTP {}", response.status()));
    }
    let receipt: serde_json::Value = response
        .json()
        .map_err(|_| "Atomic batch outcome unknown: invalid receipt; nothing replayed")?;
    if receipt["accepted"].as_bool() != Some(true)
        || receipt["count"].as_u64() != Some(request.ids.len() as u64)
    {
        return Err("Atomic batch not acknowledged with expected count; nothing replayed".into());
    }
    if stopped() {
        return Err("Atomic batch accepted; readback cancelled. Inspect before retrying".into());
    }
    let observed = crate::blocking_status(&pinned, endpoint)
        .map_err(|_| "Atomic batch accepted; readback unavailable. Nothing replayed".to_string())?;
    if peer_http::Pin::from_status(&observed).as_ref() != Some(&pin) || !request.verify(&observed) {
        return Err("Atomic batch accepted but same-board readback did not confirm all cells; nothing replayed".into());
    }
    Ok(format!(
        "Atomic batch acknowledged; {} cell states confirmed on this board",
        request.ids.len()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn wire_status(boot: char, batch: bool, state: &str) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({"version":crate::peer_identity::VERSION,"hostname":"ledboard","board_id":"esp32s3-fixture",
            "boot_id":boot.to_string().repeat(32),"http_peer_fencing":true,"cells":vec![state;80],
            "capabilities":{"board_kind":"table","acknowledged_features":if batch{vec!["table","cell-batch"]}else{vec!["table"]},
                "table":{"columns":10,"rows":8,"pixels_per_cell":6}}})).unwrap()
    }
    #[test]
    fn bitmap_is_exact_bounded_and_all_selected_cells_need_readback() {
        let request = Request::new(&[80, 1, 9, 8, 1], "red", "fade").unwrap();
        assert_eq!(request.ids, [1, 8, 9, 80]);
        assert_eq!(
            request.payload(),
            [1, 1, 0x81, 1, 0, 0, 0, 0, 0, 0, 0, 0x80]
        );
        let decoded = Request::from_payload(&request.payload()).unwrap();
        assert_eq!(decoded.ids, request.ids);
        let mut status = DeviceStatus {
            cells: vec!["white".into(); 80],
            ..Default::default()
        };
        assert!(!request.verify(&status));
        for id in &request.ids {
            status.cells[*id as usize - 1] = "red".into();
        }
        assert!(request.verify(&status));
        status.cells[79] = "off".into();
        assert!(!request.verify(&status));
        assert!(Request::new(&[], "red", "fade").is_err());
        assert!(Request::new(&[81], "red", "fade").is_err());
        assert!(Request::from_payload(&[0; 12]).is_none());
        assert!(Request::from_payload(&[255; 12]).is_none());
        assert_eq!(
            Request::new(&(1..=80).collect::<Vec<_>>(), "white", "immediate")
                .unwrap()
                .payload()[2..],
            [255; 10]
        );
    }
    #[test]
    fn one_post_has_matched_boot_fence_ack_and_full_selection_readback() {
        let (endpoint, server) = crate::tests::mock_device(vec![
            (200, wire_status('a', true, "white")),
            (202, br#"{"accepted":true,"count":2}"#.to_vec()),
            (200, wire_status('a', true, "red")),
        ]);
        let client = reqwest::blocking::Client::builder()
            .no_proxy()
            .build()
            .unwrap();
        let request = Request::new(&[1, 80], "red", "fade").unwrap();
        assert!(
            send(&client, &endpoint, &request, None, None, None)
                .unwrap()
                .contains("2 cell states confirmed")
        );
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 3);
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.starts_with("POST "))
                .count(),
            1
        );
        assert!(requests[1].starts_with("POST /api/cells "));
        assert!(requests[1].contains("ids=1%2C80&state=red&effect=fade"));
        assert!(
            requests[1]
                .to_lowercase()
                .contains(&format!("x-pakar-boot: {}", "a".repeat(32)))
        );
    }
    #[test]
    fn verified_worker_preflight_needs_only_one_post_and_same_boot_readback() {
        let current: DeviceStatus =
            serde_json::from_slice(&wire_status('a', true, "white")).unwrap();
        let pin = peer_http::Pin::from_status(&current).unwrap();
        let (endpoint, server) = crate::tests::mock_device(vec![
            (202, br#"{"accepted":true,"count":2}"#.to_vec()),
            (200, wire_status('a', true, "red")),
        ]);
        let request = Request::new(&[1, 80], "red", "immediate").unwrap();
        assert!(
            send_verified(
                &current,
                &endpoint,
                &request,
                Some(&pin),
                None,
                Some(&|| Ok(()))
            )
            .is_ok()
        );
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 2);
        assert!(requests[0].starts_with("POST /api/cells "));
        assert!(requests[1].starts_with("GET /api/status "));
        let stopped = AtomicBool::new(true);
        assert!(
            send_verified(
                &current,
                &endpoint,
                &request,
                Some(&pin),
                Some(&stopped),
                None
            )
            .is_err()
        );
        assert!(
            send_verified(
                &current,
                &endpoint,
                &request,
                Some(&pin),
                None,
                Some(&|| Err("revoked".into()))
            )
            .is_err()
        );
    }
    #[test]
    fn missing_capability_revoked_admission_conflict_or_new_boot_never_fans_out() {
        let client = reqwest::blocking::Client::builder()
            .no_proxy()
            .build()
            .unwrap();
        let request = Request::new(&[1, 80], "red", "fade").unwrap();
        let (endpoint, server) =
            crate::tests::mock_device(vec![(200, wire_status('a', false, "white"))]);
        assert!(send(&client, &endpoint, &request, None, None, None).is_err());
        assert_eq!(server.join().unwrap().len(), 1);
        let (endpoint, server) =
            crate::tests::mock_device(vec![(200, wire_status('a', true, "white"))]);
        assert!(
            send(
                &client,
                &endpoint,
                &request,
                None,
                None,
                Some(&|| Err("revoked before dispatch".into()))
            )
            .is_err()
        );
        assert_eq!(server.join().unwrap().len(), 1);
        let (endpoint, server) = crate::tests::mock_device(vec![
            (200, wire_status('a', true, "white")),
            (409, b"{}".to_vec()),
        ]);
        assert!(send(&client, &endpoint, &request, None, None, None).is_err());
        assert_eq!(server.join().unwrap().len(), 2);
        let (endpoint, server) = crate::tests::mock_device(vec![
            (200, wire_status('a', true, "white")),
            (202, br#"{"accepted":true,"count":2}"#.to_vec()),
            (200, wire_status('b', true, "red")),
        ]);
        assert!(send(&client, &endpoint, &request, None, None, None).is_err());
        assert_eq!(server.join().unwrap().len(), 3);
    }
}
