//! Descriptive peer identity and minimum supported releases. API paths and
//! optional fields remain one living contract, never version-selected adapters.
use crate::protocol_opcodes::pakar as wire;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PRODUCT: &str = "pakarcontroller-native";
pub const USB_REPORT: u8 = wire::PEER_REPORT;
const POLICY: &str = include_str!("../../interfaces/peer-policy.json");
pub fn session_id() -> &'static str {
    static ID: OnceLock<String> = OnceLock::new();
    ID.get_or_init(|| uuid::Uuid::new_v4().to_string())
}
pub fn process_started() -> std::time::Instant {
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    *START.get_or_init(std::time::Instant::now)
}
pub fn reported_settings() -> serde_json::Value {
    let config = crate::config::live();
    serde_json::json!({"theme":format!("{:?}",config.theme),"language":format!("{:?}",config.language)})
}
pub fn enforced() -> bool {
    !matches!(
        option_env!("PAKAR_ENFORCE_PEER_VERSIONS"),
        Some("false" | "0")
    ) && policy().enforced
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Policy {
    #[serde(alias = "enforce")]
    pub enforced: bool,
    pub minimum_app: String,
    pub minimum_board: String,
    pub maximum_app: Option<String>,
    pub maximum_board: Option<String>,
}
pub fn policy() -> Policy {
    static VALUE: OnceLock<Policy> = OnceLock::new();
    VALUE
        .get_or_init(|| serde_json::from_str(POLICY).expect("checked shared peer support policy"))
        .clone()
}
pub fn advertised_policy() -> Policy {
    let mut value = policy();
    value.enforced = enforced();
    value
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Release {
    core: [u32; 3],
    pre: Vec<String>,
}
impl Release {
    fn parse(raw: &str) -> Option<Self> {
        if raw.is_empty() || raw.len() > 63 || !raw.is_ascii() {
            return None;
        }
        let (version, build) = match raw.split_once('+') {
            Some((a, b)) => (a, Some(b)),
            None => (raw, None),
        };
        let ids = |value: &str| {
            value.split('.').all(|id| {
                !id.is_empty() && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            })
        };
        if build.is_some_and(|b| !ids(b)) {
            return None;
        }
        let (core, pre) = match version.split_once('-') {
            Some((a, b)) => (a, b),
            None => (version, ""),
        };
        if version.contains('-')
            && (!ids(pre)
                || pre.split('.').any(|p| {
                    p.bytes().all(|b| b.is_ascii_digit()) && p.len() > 1 && p.starts_with('0')
                }))
        {
            return None;
        }
        let numbers: Vec<_> = core
            .split('.')
            .map(|part| {
                if part.is_empty()
                    || !part.bytes().all(|b| b.is_ascii_digit())
                    || part.len() > 1 && part.starts_with('0')
                {
                    None
                } else {
                    part.parse::<u32>().ok()
                }
            })
            .collect::<Option<_>>()?;
        Some(Self {
            core: numbers.try_into().ok()?,
            pre: if pre.is_empty() {
                vec![]
            } else {
                pre.split('.').map(str::to_owned).collect()
            },
        })
    }
    fn compare(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let core = self.core.cmp(&other.core);
        if core != Ordering::Equal {
            return core;
        }
        match (self.pre.is_empty(), other.pre.is_empty()) {
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            _ => {}
        }
        for (a, b) in self.pre.iter().zip(&other.pre) {
            let numeric = |v: &str| v.bytes().all(|b| b.is_ascii_digit());
            let order = match (numeric(a), numeric(b)) {
                (true, true) => a.len().cmp(&b.len()).then(a.cmp(b)),
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.cmp(b),
            };
            if order != Ordering::Equal {
                return order;
            }
        }
        self.pre.len().cmp(&other.pre.len())
    }
}
fn range(version: &str, minimum: &str, maximum: Option<&str>, role: &str) -> Result<(), String> {
    let current=Release::parse(version).ok_or_else(||format!("Peer compatibility: {role} omitted a valid software version; update that peer or explicitly disable version matching in its build"))?;
    let lower =
        Release::parse(minimum).ok_or("Peer compatibility: invalid minimum release policy")?;
    if current.compare(&lower).is_lt() {
        return Err(format!(
            "Peer compatibility: {role} {version} is unsupported; minimum {minimum}. Update that peer"
        ));
    }
    if let Some(maximum) = maximum {
        let upper =
            Release::parse(maximum).ok_or("Peer compatibility: invalid maximum release policy")?;
        if !current.compare(&upper).is_lt() {
            return Err(format!(
                "Peer compatibility: {role} {version} is unsupported; maximum {maximum} (exclusive). Update the receiving peer"
            ));
        }
    }
    Ok(())
}
pub fn unsupported(error: &str) -> bool {
    error.starts_with("Peer compatibility:") || error.starts_with("HTTP 426")
}
pub fn validate_board(status: &crate::DeviceStatus) -> Result<(), String> {
    if enforced() {
        let p = policy();
        range(
            &status.version,
            &p.minimum_board,
            p.maximum_board.as_deref(),
            "board",
        )?;
    }
    if let Some(p) = &status.compatibility
        && p.enforced
    {
        // The local override disables only our incoming version gate; it does
        // not claim that the other peer has accepted an unsupported app.
        range(
            VERSION,
            &p.minimum_app,
            p.maximum_app.as_deref(),
            "this application",
        )?;
    }
    if let Some(bridge) = &status.bridge
        && bridge.get("version").is_some()
        && enforced()
    {
        let p = policy();
        range(
            bridge["version"].as_str().unwrap_or_default(),
            &p.minimum_app,
            p.maximum_app.as_deref(),
            "bridge application",
        )?;
    }
    Ok(())
}
pub fn validate_client(version: Option<&str>) -> Result<(), String> {
    if !enforced() {
        return Ok(());
    }
    let p = policy();
    range(
        version.unwrap_or_default(),
        &p.minimum_app,
        p.maximum_app.as_deref(),
        "client",
    )
}
pub fn user_agent() -> String {
    format!(
        "PakarController/{VERSION} ({}; {}; native)",
        std::env::consts::OS,
        std::env::consts::ARCH
    )
}
pub fn build_identity() -> String {
    option_env!("PAKAR_BUILD_ID")
        .filter(|v| !v.is_empty() && v.len() <= 32 && v.bytes().all(|b| (33..=126).contains(&b)))
        .map(str::to_owned)
        .unwrap_or_else(|| crate::client_management::build().to_string())
}
pub fn headers() -> reqwest::header::HeaderMap {
    let mut result = reqwest::header::HeaderMap::new();
    for (key, value) in [
        ("user-agent", user_agent()),
        ("x-pakar-client", PRODUCT.into()),
        ("x-pakar-version", VERSION.into()),
        ("x-pakar-client-id", session_id().into()),
    ] {
        result.insert(key, value.parse().expect("constant safe peer header"));
    }
    result.insert("x-pakar-build", build_identity().parse().unwrap());
    result
}
pub fn client_builder() -> reqwest::blocking::ClientBuilder {
    reqwest::blocking::Client::builder().default_headers(headers())
}
pub fn websocket_headers(request: &mut tungstenite::handshake::client::Request) {
    request.headers_mut().extend(headers());
}
pub fn validate_response_headers(headers: &reqwest::header::HeaderMap) -> Result<(), String> {
    if !enforced() {
        return Ok(());
    }
    let p = policy();
    if let Some(version) = headers.get("x-pakar-version") {
        range(
            version.to_str().unwrap_or_default(),
            &p.minimum_app,
            p.maximum_app.as_deref(),
            "HTTP server",
        )?;
    }
    if let Some(version) = headers.get("x-pakar-board-version") {
        range(
            version.to_str().unwrap_or_default(),
            &p.minimum_board,
            p.maximum_board.as_deref(),
            "HTTP board",
        )?;
    }
    if let Some(server) = headers
        .get("server")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("PakarController/"))
    {
        range(
            server.split_ascii_whitespace().next().unwrap_or_default(),
            &p.minimum_app,
            p.maximum_app.as_deref(),
            "HTTP server",
        )?;
    }
    Ok(())
}
pub fn server_headers() -> String {
    format!(
        "Server: PakarController/{VERSION}\r\nX-Pakar-Version: {VERSION}\r\nX-Pakar-Build: {}\r\nX-Pakar-Client-Id: {}\r\nX-Pakar-Version-Check: {}\r\nX-Pakar-Min-App-Version: {}\r\n",
        build_identity(),
        session_id(),
        enforced(),
        policy().minimum_app,
    )
}
pub fn hello() -> Vec<u8> {
    let release = Release::parse(VERSION).expect("package semver");
    let mut payload = vec![3];
    for n in release.core {
        payload.push(u8::try_from(n).expect("USB compact package release"));
    }
    payload
}
pub fn validate_usb_hello(payload: &[u8]) -> Result<String, String> {
    if payload.len() < 9 || payload[1] != 2 {
        return Err("Peer compatibility: USB HELLO is not an ESP board".into());
    }
    let version = format!("{}.{}.{}", payload[6], payload[7], payload[8]);
    if enforced() {
        let p = policy();
        range(
            &version,
            &p.minimum_board,
            p.maximum_board.as_deref(),
            "USB board",
        )?;
    }
    Ok(version)
}
/// Optional telemetry support belongs to one physical USB session. Only an
/// explicit, correlated Unsupported response can retire this additive feature;
/// an absent response, auth failure or malformed packet never grants admission.
#[derive(Default)]
pub struct UsbReportSupport {
    unavailable: bool,
}
impl UsbReportSupport {
    pub fn supported(&self) -> bool {
        !self.unavailable
    }
    pub fn observe_unsupported(
        &mut self,
        request: (u8, u8),
        response: (u8, u8),
        payload: &[u8],
    ) -> bool {
        self.observe_with_policy(enforced(), request, response, payload)
    }
    fn observe_with_policy(
        &mut self,
        enforce: bool,
        request: (u8, u8),
        response: (u8, u8),
        payload: &[u8],
    ) -> bool {
        if !enforce
            && request.0 == USB_REPORT
            && response == (wire::ERROR_RESPONSE, request.1)
            && payload == [USB_REPORT, 2]
        {
            self.unavailable = true;
            return true;
        }
        false
    }
}
pub fn report_chunks(
    value: &serde_json::Value,
) -> Result<std::collections::VecDeque<Vec<u8>>, String> {
    let bytes = serde_json::to_vec(value).map_err(|_| "Cannot serialize peer introduction")?;
    if bytes.is_empty() || bytes.len() > 1536 {
        return Err("Peer introduction exceeds bounded report size".into());
    }
    Ok(bytes
        .chunks(44)
        .enumerate()
        .map(|(n, data)| {
            let mut p = (bytes.len() as u16).to_le_bytes().to_vec();
            p.extend(((n * 44) as u16).to_le_bytes());
            p.extend(data);
            p
        })
        .collect())
}
/// Preserve the device's actual error as PCController Session.RemoteError does.
/// Rejection is not necessarily a release-policy mismatch: report validation,
/// capacity, maintenance and malformed responses must remain distinguishable.
pub fn usb_introduction_error(request: u8, response: u8, payload: &[u8]) -> String {
    if response == wire::ERROR_RESPONSE && payload.len() == 2 && payload[0] == request {
        let reason = match payload[1] {
            1 => "bad envelope",
            2 => "unsupported caller or introduction",
            3 if request == USB_REPORT => "invalid report or unavailable peer-registry capacity",
            3 => "invalid introduction payload",
            4 => "hardware unavailable",
            5 => "board busy",
            6 => "request rejected by board safety fence",
            _ => "unknown board error",
        };
        let category = if payload[1] == 2 {
            "Peer compatibility:"
        } else {
            "USB introduction:"
        };
        return format!(
            "{category} board rejected request 0x{request:02X}: {reason} (code 0x{:02X}); no control command sent or retried",
            payload[1]
        );
    }
    let detail = payload
        .iter()
        .take(8)
        .map(|value| format!("{value:02X}"))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "USB introduction: unexpected response 0x{response:02X} to request 0x{request:02X} (payload {detail}, {} bytes); no control command sent or retried",
        payload.len()
    )
}
/// Peer reports are observations, not authority. Keep only bounded nonsecret
/// operational values; a report cannot change app settings or board ownership.
pub fn sanitize_report(
    value: &serde_json::Value,
    id: &str,
    version: &str,
    ip: &str,
) -> Result<serde_json::Value, String> {
    if value.to_string().len() > 1536
        || value["id"].as_str() != Some(id)
        || value["version"].as_str() != Some(version)
        || uuid::Uuid::parse_str(id).is_err()
    {
        return Err("Peer report/header identity differs or exceeds limit".into());
    }
    validate_client(Some(version))?;
    let mut out = serde_json::json!({"id":id,"version":version,"observed_ip":ip});
    for key in [
        "session_id",
        "application",
        "os",
        "architecture",
        "hostname",
        "transport",
        "device_model",
        "reported_ip",
        "build",
    ] {
        if let Some(v) = value[key]
            .as_str()
            .filter(|v| v.len() <= 96 && !v.chars().any(char::is_control))
        {
            out[key] = v.into();
        }
    }
    for key in [
        "process_id",
        "uptime_seconds",
        "cpu_count",
        "memory_mb",
        "memory_available_mb",
        "screen_width",
        "screen_height",
        "monitor_count",
    ] {
        if let Some(v) = value[key].as_u64().filter(|v| *v <= u32::MAX as u64) {
            out[key] = v.into();
        }
    }
    for (object, keys) in [
        (
            "hardware",
            &[
                "model",
                "manufacturer",
                "cpu",
                "os_version",
                "sdk",
                "memory_mb",
                "cpu_count",
                "screen_width",
                "screen_height",
            ][..],
        ),
        (
            "settings",
            &[
                "transport",
                "theme",
                "language",
                "silent_mode",
                "telemetry_enabled",
                "version_check",
            ][..],
        ),
    ] {
        for key in keys {
            let v = &value[object][*key];
            if v.is_boolean()
                || v.as_u64().is_some_and(|n| n <= u32::MAX as u64)
                || v.as_str()
                    .is_some_and(|s| s.len() <= 96 && !s.chars().any(char::is_control))
            {
                if out.get(object).is_none() {
                    out[object] = serde_json::json!({});
                }
                out[object][*key] = v.clone();
            }
        }
    }
    if let Some(caps) = value["capabilities"].as_array() {
        out["capabilities"] = caps
            .iter()
            .take(12)
            .filter_map(|v| v.as_str())
            .filter(|s| {
                !s.is_empty()
                    && s.len() <= 32
                    && s.bytes()
                        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
            })
            .map(|s| serde_json::json!(s))
            .collect::<Vec<_>>()
            .into();
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn usb_introduction_error_preserves_actual_board_reason_without_suggesting_policy_bypass() {
        for (code, expected) in [
            (2, "unsupported caller"),
            (3, "invalid report"),
            (5, "board busy"),
            (6, "safety fence"),
            (255, "unknown board error"),
        ] {
            let message = usb_introduction_error(USB_REPORT, 0x82, &[USB_REPORT, code]);
            assert!(message.contains(expected), "{message}");
            assert!(message.contains("0x61"));
            assert_eq!(unsupported(&message), code == 2);
            assert!(!message.contains("override"));
        }
        for (response, payload) in [
            (0x80, vec![USB_REPORT, 3]),
            (0x82, vec![0x55, 2]),
            (0x82, vec![USB_REPORT]),
            (0x82, vec![USB_REPORT, 2, 0]),
        ] {
            let message = usb_introduction_error(USB_REPORT, response, &payload);
            assert!(message.contains("unexpected response"));
            assert!(!unsupported(&message));
        }
        assert!(
            usb_introduction_error(0x01, 0x82, &[0x01, 3]).contains("invalid introduction payload")
        );
    }
    #[test]
    fn semver_minimum_not_exact_build_selectors() {
        for value in ["0.2.7", "0.2.8", "1.0.0+source.hash", "0.2.8-alpha.2"] {
            assert!(range(value, "0.2.7", None, "board").is_ok());
        }
        for value in [
            "",
            "0.2.6",
            "0.2.7-alpha",
            "v0.2.7",
            "0.02.7",
            "0.2",
            "0.2.7+",
            "0.2.7-01",
            "0.2.7+bad+bad",
            " 0.2.7",
        ] {
            assert!(range(value, "0.2.7", None, "board").is_err(), "{value}");
        }
        assert!(range("0.3.0", "0.2.7", Some("0.3.0"), "board").is_err());
        assert!(range("0.3.0-alpha", "0.2.7", Some("0.3.0"), "board").is_ok());
    }
    #[test]
    fn one_session_across_headers_reports_and_usb() {
        let headers = headers();
        assert_eq!(headers["x-pakar-version"], VERSION);
        assert_eq!(headers["x-pakar-client-id"], session_id());
        assert!(headers["user-agent"].to_str().unwrap().contains(VERSION));
        let report = crate::client_telemetry::Reporter::new().value();
        assert_eq!(report["id"], session_id());
        let chunks = report_chunks(&report).unwrap();
        let mut joined = vec![];
        for chunk in chunks {
            assert!(chunk.len() <= 48);
            assert_eq!(
                u16::from_le_bytes([chunk[2], chunk[3]]) as usize,
                joined.len()
            );
            joined.extend_from_slice(&chunk[4..]);
        }
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&joined).unwrap(),
            report
        );
    }
    #[test]
    fn usb_report_optional_only_correlated_explicit_unsupported_can_disable_reports() {
        let headers = server_headers();
        assert!(headers.contains(&format!("X-Pakar-Version-Check: {}\r\n", enforced())));
        assert!(headers.contains(&format!(
            "X-Pakar-Min-App-Version: {}\r\n",
            policy().minimum_app
        )));
        assert!(!headers.contains("X-Pakar-Version-Enforced:"));
        for enforce in [false, true] {
            for (request, response, payload) in [
                ((USB_REPORT, 9), (0x82, 8), vec![USB_REPORT, 2]),
                ((0x55, 9), (0x82, 9), vec![USB_REPORT, 2]),
                ((USB_REPORT, 9), (0x80, 9), vec![USB_REPORT, 2]),
                ((USB_REPORT, 9), (0x82, 9), vec![0x55, 2]),
                ((USB_REPORT, 9), (0x82, 9), vec![USB_REPORT, 6]),
                ((USB_REPORT, 9), (0x82, 9), vec![USB_REPORT]),
                ((USB_REPORT, 9), (0x82, 9), vec![]),
                ((USB_REPORT, 9), (0x82, 9), vec![USB_REPORT, 2, 0]),
            ] {
                let mut support = UsbReportSupport::default();
                assert!(!support.observe_with_policy(enforce, request, response, &payload));
                assert!(support.supported());
            }
            for code in (0..=255_u8).filter(|code| *code != 2) {
                let mut support = UsbReportSupport::default();
                assert!(!support.observe_with_policy(
                    enforce,
                    (USB_REPORT, 9),
                    (0x82, 9),
                    &[USB_REPORT, code],
                ));
                assert!(support.supported());
            }
            let mut support = UsbReportSupport::default();
            assert_eq!(
                support.observe_with_policy(enforce, (USB_REPORT, 9), (0x82, 9), &[USB_REPORT, 2],),
                !enforce
            );
            assert_eq!(support.supported(), enforce);
            assert!(
                UsbReportSupport::default().supported(),
                "A new physical session probes again"
            );
        }
    }
    #[test]
    fn additive_identity_and_other_peers_policy() {
        let s:crate::DeviceStatus=serde_json::from_value(serde_json::json!({"version":"0.2.8+source", "future":{"optional":true},"compatibility":{"enforced":true,"minimum_app":"99.0.0","minimum_board":"0.2.7"}})).unwrap();
        assert!(validate_board(&s).unwrap_err().contains("this application"));
        assert!(validate_client(Some(VERSION)).is_ok());
    }
    #[test]
    fn reports_drop_secrets_and_never_replace_observed_ip() {
        let id = uuid::Uuid::new_v4().to_string();
        let input = serde_json::json!({"id":id,"version":VERSION,"observed_ip":"forged","reported_ip":"192.0.2.9","password":"secret","settings":{"transport":"usb","password":"secret"},"hardware":{"cpu_count":4,"token":"secret"},"capabilities":["cell-batch","bad\nfield"]});
        let value = sanitize_report(&input, &id, VERSION, "127.0.0.1").unwrap();
        assert_eq!(value["observed_ip"], "127.0.0.1");
        assert_eq!(value["reported_ip"], "192.0.2.9");
        assert_eq!(value["capabilities"], serde_json::json!(["cell-batch"]));
        assert!(!value.to_string().contains("secret"));
        assert!(
            sanitize_report(
                &input,
                &uuid::Uuid::new_v4().to_string(),
                VERSION,
                "127.0.0.1"
            )
            .is_err()
        );
    }
    #[test]
    fn server_header_gate_is_additive_but_never_substitutes_for_body_identity() {
        let mut headers = reqwest::header::HeaderMap::new();
        assert!(validate_response_headers(&headers).is_ok());
        headers.insert("server", "PakarController/0.2.6".parse().unwrap());
        assert!(validate_response_headers(&headers).is_err());
        headers.insert("server", "PakarController/0.2.8+new".parse().unwrap());
        assert!(validate_response_headers(&headers).is_ok());
        let missing = crate::DeviceStatus::default();
        assert!(validate_board(&missing).is_err());
        let mut hello = vec![0, 2, 0, 0, 80, 9];
        hello.extend_from_slice(&super::hello()[1..]);
        assert_eq!(validate_usb_hello(&hello).unwrap(), VERSION);
        hello[8] = 6;
        assert!(validate_usb_hello(&hello).is_err());
        hello[8] = 7;
        hello[1] = 1;
        assert!(validate_usb_hello(&hello).is_err());
    }
}
