//! Explicit, non-replayed maintenance operations against the integrated firmware.
use crate::Endpoint;
use reqwest::blocking::{Client, multipart};
use std::{
    io::{Read, Write},
    path::PathBuf,
    time::Duration,
};

#[derive(Clone)]
pub enum Operation {
    ReconnectCloud,
    Probe,
    Backup,
    Download {
        eeprom: bool,
        destination: PathBuf,
    },
    StageAvr(Vec<u8>),
    ProgramAvr,
    UpdateEsp(Vec<u8>),
    SourceImage {
        source: crate::firmware_source::Source,
        esp: bool,
        progress: crate::firmware_progress::Transfer,
    },
    RemoteStage {
        url: String,
        sha256: String,
        targets: u8,
        bearer: String,
    },
    RemoteApply(RemoteJob),
    RemoteCancel(RemoteJob),
    RemoteStatus,
    PeerStage {
        source: serde_json::Value,
        authorization: String,
        target: crate::peer_http::Pin,
    },
}
impl Operation {
    pub fn warning(&self) -> &'static str {
        match self {
            Self::PeerStage { .. } => {
                "Download the selected peer's newer compatible ESP image into the inactive slot. This does not activate it or reboot. Source credentials are used for this transfer only; inspect progress before explicitly applying."
            }
            Self::RemoteStage { .. } => {
                "Download and SHA-256-verify the pinned manifest and selected complete firmware images into ESP storage. This stages only: neither board is programmed. Endpoint credentials are not saved by this app."
            }
            Self::RemoteApply(_) => {
                "Apply exactly this advertised job identity. Durable AVR Prog safety, full recovery backup, EEPROM preservation/readback and matching pair versions are required. Power loss is not an atomic two-board rollback. An uncertain outcome must not be retried blindly."
            }
            Self::RemoteCancel(_) => {
                "Cancel this exact download/staged job. Applying jobs cannot be cancelled. Staged files remain for inspection; no automatic rollback occurs."
            }
            Self::RemoteStatus => {
                "Read the observed remote firmware transaction. No hardware write."
            }
            Self::ReconnectCloud => {
                "Reconnect MQTT and WebSocket sessions using the stored configuration. Unsaved changes are not applied. Acceptance does not prove a successful broker connection."
            }
            Self::Probe => {
                "ISP probe resets and temporarily interrupts the AVR. Confirm wiring and power before proceeding."
            }
            Self::Backup => {
                "Backup resets the AVR and reads flash (including bootloader) and EEPROM into ESP storage. Existing on-device backups will be replaced."
            }
            Self::ProgramAvr => {
                "Program the CURRENT shared staged image, which another client may have replaced. This firmware backs up first, then erases/programs/verifies the AVR. The API does not expose a staged-image hash. Confirm image provenance and electrical readiness; never retry an uncertain result blindly."
            }
            Self::StageAvr(_) => {
                "Store this complete 32 KiB AVR binary (including the intended bootloader). Staging replaces the shared image but does not program the AVR."
            }
            Self::UpdateEsp(_) => {
                "Replace ESP firmware and reboot. Confirm this binary is built for this exact ESP board/module and preserves its pin mapping. A lost connection does not prove failure; reconnect before retrying."
            }
            Self::SourceImage { esp: true, .. } => {
                "Verify this exact local/downloaded ESP binary, upload once to the selected board and reboot it. Transfer progress is not flash completion. Reconnect and inspect running firmware; no uncertain write is replayed."
            }
            Self::SourceImage { .. } => {
                "Verify and upload a complete 32768-byte AVR image to ESP storage. It is NOT programmed automatically. Existing staged image is replaced; final programming remains explicit."
            }
            Self::Download { .. } => {
                "Export the existing backup. No board write is performed and existing PC files will not be overwritten."
            }
        }
    }
}
// Adapt the actual create-remote-manifest raw-image policy and SDK
// FirmwarePlatform::otaCapacity boundary. These are application-slot bounds,
// not the module's total flash size. The device still verifies image integrity,
// chip revision and its actual inactive partition before committing an update.
const S3_OTA_SLOT: u64 = 0x330000;
const ESP8266_IMAGE_LIMIT: usize = 0x100000;
pub fn maximum_image_bytes(esp: bool) -> u64 {
    if esp { S3_OTA_SLOT - 1 } else { 32768 }
}
fn validate_esp_image(bytes: &[u8], expected_s3: Option<bool>) -> Result<(), String> {
    if bytes.len() < 24 || bytes[0] != 0xe9 || !(1..=16).contains(&bytes[1]) {
        return Err("Select a raw uncompressed ESP application .bin with a valid E9/segment header; merged flash images are not supported".into());
    }
    let chip = u16::from_le_bytes([bytes[12], bytes[13]]);
    // An ESP8266 header has no chip/hash fields. Without a selected board,
    // recognizable S3/large candidates must meet the S3 policy; the
    // actual pinned board below remains authoritative before any upload.
    let s3 = expected_s3.unwrap_or(chip == 9 || bytes.len() >= ESP8266_IMAGE_LIMIT);
    if s3 {
        if bytes.len() as u64 >= S3_OTA_SLOT || chip != 9 || bytes[23] != 1 {
            return Err("ESP32-S3 requires a hashed raw application image (chip9, hash-appended flag1) smaller than its supported0x330000-byte OTA slot".into());
        }
    } else if bytes.len() >= ESP8266_IMAGE_LIMIT || chip == 9 {
        return Err("ESP8266 requires its own raw application image smaller than1MiB; an ESP32-S3 image cannot be uploaded to it".into());
    }
    Ok(())
}
pub fn validate_image(esp: bool, bytes: &[u8]) -> Result<(), String> {
    if esp {
        validate_esp_image(bytes, None)
    } else if bytes.len() != 32768 {
        Err("AVR staging requires exactly 32768 bytes; Intel HEX and partial application images are not accepted".into())
    } else {
        Ok(())
    }
}
pub fn validate_image_for_board(
    esp: bool,
    bytes: &[u8],
    board: &crate::DeviceStatus,
) -> Result<(), String> {
    if !esp {
        return validate_image(false, bytes);
    }
    let s3 = if board.board_id.starts_with("esp32s3-") {
        Some(true)
    } else if board.board_id.starts_with("esp8266-") {
        Some(false)
    } else {
        None
    };
    validate_esp_image(bytes, s3)?;
    if !board.ota_capacity_bytes.is_null() {
        let capacity = board
            .ota_capacity_bytes
            .as_u64()
            .filter(|n| *n > 0)
            .ok_or("Board advertised an invalid OTA capacity; no firmware sent")?;
        if bytes.len() as u64 >= capacity {
            return Err("Firmware does not fit the board's advertised inactive OTA partition; no firmware sent".into());
        }
    }
    Ok(())
}
pub fn read_image_file(path: &std::path::Path, esp: bool) -> Result<Vec<u8>, String> {
    let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    let maximum = maximum_image_bytes(esp);
    if !metadata.is_file() || metadata.len() > maximum {
        return Err("Firmware file exceeds the supported image-slot bound".into());
    }
    let mut bytes = Vec::new();
    file.take(maximum + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    validate_image(esp, &bytes)?;
    Ok(bytes)
}
pub fn run(client: &Client, endpoint: &Endpoint, operation: Operation) -> Result<String, String> {
    if let Operation::SourceImage {
        source,
        esp,
        progress,
    } = operation
    {
        return crate::firmware_source::run(client, endpoint, source, esp, &progress);
    }
    if matches!(
        operation,
        Operation::RemoteStage { .. }
            | Operation::RemoteApply(_)
            | Operation::RemoteCancel(_)
            | Operation::RemoteStatus
            | Operation::PeerStage { .. }
    ) {
        return remote_run(client, endpoint, operation);
    }
    let (path, post) = match &operation {
        Operation::ReconnectCloud => ("/api/cloud/reconnect", true),
        Operation::Probe => ("/api/avr/probe", false),
        Operation::Backup => ("/api/avr/backup", true),
        Operation::Download { eeprom: true, .. } => ("/api/avr/backup/eeprom", false),
        Operation::Download { .. } => ("/api/avr/backup/flash", false),
        Operation::StageAvr(_) => ("/api/avr/image", true),
        Operation::ProgramAvr => ("/api/avr/program", true),
        Operation::UpdateEsp(_) => ("/update", true),
        _ => unreachable!(),
    };
    let url = format!("{}{path}", endpoint.base.trim_end_matches('/'));
    let request = if post {
        client.post(url)
    } else {
        client.get(url)
    };
    let mut request = request
        .basic_auth(&endpoint.user, Some(&endpoint.password))
        .timeout(Duration::from_secs(180));
    match &operation {
        Operation::StageAvr(bytes) | Operation::UpdateEsp(bytes) => {
            let esp = matches!(operation, Operation::UpdateEsp(_));
            validate_image(esp, bytes)?;
            if esp {
                let before = crate::blocking_status(client, endpoint)?;
                validate_image_for_board(true, bytes, &before)?;
            }
            let part = multipart::Part::bytes(bytes.clone()).file_name(if esp {
                "esp.bin"
            } else {
                "avr.bin"
            });
            request = request.multipart(
                multipart::Form::new().part(if esp { "firmware" } else { "image" }, part),
            );
        }
        _ => {}
    }
    let response = request.send().map_err(|_| "Maintenance outcome unknown: transport failed. Nothing was replayed; reconnect and inspect before any retry.".to_string())?;
    if !response.status().is_success() {
        return Err(format!(
            "Device rejected maintenance (HTTP {}). Inspect device state before retrying.",
            response.status()
        ));
    }
    if let Operation::Download {
        eeprom,
        destination,
    } = operation
    {
        let expected = if eeprom { 1024 } else { 32768 };
        let bytes = response.take(expected + 1).bytes();
        // Bound the read even if the peer sends an unexpected body.
        let bytes = bytes
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Backup download failed".to_string())?;
        if bytes.len() != expected as usize {
            return Err(format!(
                "Backup length mismatch: expected {expected} bytes; no file was written"
            ));
        }
        let mut file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&destination)
            .map_err(|e| format!("Cannot create export (existing files are preserved): {e}"))?;
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|e| format!("Export write failed; file may be incomplete: {e}"))?;
        return Ok(format!(
            "Exported {expected} verified-length bytes to {}",
            destination.display()
        ));
    }
    if matches!(operation, Operation::StageAvr(_)) {
        if response
            .text()
            .map_err(|_| "Staging acknowledgment unreadable")?
            .trim()
            != "AVR image stored"
        {
            return Err("Unexpected staging acknowledgment; image state unconfirmed".into());
        }
        return Ok("AVR image stored, NOT programmed. Shared staging has no advertised hash; confirm provenance before programming.".into());
    }
    let value: serde_json::Value = response
        .json()
        .map_err(|_| "Maintenance acknowledgment unreadable; outcome unconfirmed")?;
    match operation {
        Operation::ReconnectCloud => (value["accepted"].as_bool() == Some(true)).then(|| "Cloud reconnect accepted. Check live MQTT/WebSocket status to verify the connection.".into()),
        Operation::Probe => value["signature"]
            .as_str()
            .filter(|s| *s == "1E950F")
            .map(|s| format!("ATmega328P ISP signature confirmed: {s}")),
        Operation::Backup => (value["backed_up"].as_bool() == Some(true))
            .then(|| "Device confirmed flash + EEPROM backup in ESP storage".into()),
        Operation::ProgramAvr => (value["programmed"].as_bool() == Some(true)
            && value["verified"].as_bool() == Some(true))
        .then(|| "Device confirmed AVR programming and verification".into()),
        Operation::UpdateEsp(_) => (value["updated"].as_bool() == Some(true)
            && value["rebooting"].as_bool() == Some(true))
        .then(|| {
            "ESP accepted firmware and scheduled reboot. Reconnect to verify running version."
                .into()
        }),
        _ => None,
    }
    .ok_or("Device did not confirm the expected maintenance result".into())
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RemoteJob {
    pub job_id: u32,
    pub boot_id: String,
    pub phase: String,
    pub targets: u8,
    pub manifest_sha256: String,
    pub busy: bool,
    pub cancellable: bool,
    pub received_bytes: u32,
    pub expected_bytes: u32,
    pub error: String,
}
pub fn valid_hash(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn validate_remote(url: &str, sha: &str, targets: u8) -> Result<(), String> {
    let parsed = reqwest::Url::parse(url).map_err(|_| "Invalid firmware URL")?;
    if url.len() > 383
        || parsed.scheme() != "https"
        || parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.fragment().is_some()
        || url.bytes().any(|b| b <= 32 || b >= 127 || b == b'\\')
    {
        return Err(
            "Use a direct HTTPS hostname URL without embedded credentials or fragment".into(),
        );
    }
    if !valid_hash(sha) || !(1..=3).contains(&targets) {
        return Err(
            "A lowercase 64-digit manifest SHA-256 and target 1, 2 or 3 are required".into(),
        );
    }
    Ok(())
}
pub fn parse_job(value: serde_json::Value) -> Result<RemoteJob, String> {
    let job: RemoteJob =
        serde_json::from_value(value).map_err(|_| "Malformed remote job response")?;
    if !(1..=3).contains(&job.targets)
        || job.boot_id.is_empty()
        || job.boot_id.len() > 80
        || job.boot_id.bytes().any(|b| !(33..=126).contains(&b))
        || job.error.len() > 1024
        || job
            .error
            .chars()
            .any(|c| c.is_control() && c != '\n' && c != '\t')
        || job.received_bytes > job.expected_bytes
        || (job.job_id > 0 && !valid_hash(&job.manifest_sha256))
        || ![
            "idle",
            "manifest",
            "download_esp",
            "download_avr",
            "ready",
            "prepare",
            "program_avr",
            "program_esp",
            "awaiting_reboot",
            "verify_pair",
            "release_pair",
            "complete",
            "failed",
            "cancelled",
        ]
        .contains(&job.phase.as_str())
    {
        return Err("Invalid remote transaction identity/state".into());
    }
    Ok(job)
}
pub fn result_severity(result: &Result<String, String>) -> crate::messaging::Severity {
    use crate::messaging::Severity;
    let Ok(text) = result else {
        return Severity::Error;
    };
    if text.starts_with("ESP accepted firmware")
        || text.starts_with("AVR image uploaded and stored")
    {
        return Severity::Info;
    }
    let job = serde_json::from_str(text)
        .ok()
        .and_then(|v| parse_job(v).ok());
    match job.as_ref().map(|j| j.phase.as_str()) {
        Some("complete") => Severity::Success,
        Some("failed") => Severity::Error,
        Some(_) => Severity::Info, // Accepted/staged/applying is NOT completion.
        None => Severity::Success, // Legacy run() already validated positive ACK.
    }
}
fn remote_run(
    client: &Client,
    endpoint: &Endpoint,
    operation: Operation,
) -> Result<String, String> {
    let peer_client = if let Operation::PeerStage { target, .. } = &operation {
        Some(target.client()?)
    } else {
        None
    };
    let client = peer_client.as_ref().unwrap_or(client);
    let (path, form) = remote_request(&operation)?;
    let url = format!("{}{path}", endpoint.base.trim_end_matches('/'));
    let request = if let Some(form) = form {
        client.post(url).form(&form)
    } else {
        client.get(url)
    };
    let response = request
        .basic_auth(&endpoint.user, Some(&endpoint.password))
        .timeout(Duration::from_secs(15))
        .send()
        .map_err(|_| "Remote firmware request outcome unknown. No mutation was replayed.")?;
    if !response.status().is_success() {
        return Err(format!(
            "Remote firmware rejected (HTTP {})",
            response.status()
        ));
    }
    let mut bytes = Vec::new();
    response
        .take(8193)
        .read_to_end(&mut bytes)
        .map_err(|_| "Remote acknowledgment unreadable")?;
    if bytes.len() > 8192 {
        return Err("Oversized remote acknowledgment".into());
    }
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|_| "Invalid remote acknowledgment JSON")?;
    verify_remote_receipt(&operation, value)
}
type RemoteForm = Option<Vec<(&'static str, String)>>;
fn remote_request(operation: &Operation) -> Result<(&'static str, RemoteForm), String> {
    let request = match operation {
        Operation::PeerStage {
            source,
            authorization,
            ..
        } => {
            if !source.is_object()
                || serde_json::to_vec(source).map_err(|e| e.to_string())?.len() > 3072
                || authorization.len() > 512
                || authorization.bytes().any(|b| !(32..=126).contains(&b))
            {
                return Err("Invalid peer staging source".into());
            }
            (
                "/api/peers/update",
                Some(vec![
                    ("source", source.to_string()),
                    ("authorization", authorization.clone()),
                ]),
            )
        }
        Operation::RemoteStatus => ("/api/firmware/remote", None),
        Operation::RemoteStage {
            url,
            sha256,
            targets,
            bearer,
        } => {
            validate_remote(url, sha256, *targets)?;
            if bearer.len() > 256 || bearer.bytes().any(|b| !(33..=126).contains(&b)) {
                return Err("Invalid firmware endpoint bearer token".into());
            }
            (
                "/api/firmware/fetch",
                Some(vec![
                    ("url", url.clone()),
                    ("sha256", sha256.clone()),
                    ("targets", targets.to_string()),
                    ("apply", "false".into()),
                    ("bearer", bearer.clone()),
                ]),
            )
        }
        Operation::RemoteApply(job) | Operation::RemoteCancel(job) => {
            parse_job(serde_json::to_value(job).map_err(|e| e.to_string())?)?;
            let apply = matches!(operation, Operation::RemoteApply(_));
            if (apply && job.phase != "ready")
                || (!apply && !job.cancellable && job.phase != "ready")
            {
                return Err(
                    "Observed job is not in the required state; refresh before proceeding".into(),
                );
            }
            (
                if apply {
                    "/api/firmware/apply"
                } else {
                    "/api/firmware/cancel"
                },
                Some(vec![
                    ("job_id", job.job_id.to_string()),
                    ("boot_id", job.boot_id.clone()),
                    ("sha256", job.manifest_sha256.clone()),
                ]),
            )
        }
        _ => return Err("Not a remote operation".into()),
    };
    Ok(request)
}
pub fn usb_remote_request(
    operation: &Operation,
) -> Result<(&'static str, &'static str, serde_json::Value), String> {
    if !matches!(
        operation,
        Operation::PeerStage { .. }
            | Operation::RemoteStatus
            | Operation::RemoteApply(_)
            | Operation::RemoteCancel(_)
    ) {
        return Err("This maintenance source requires Wi-Fi; nothing sent over USB".into());
    }
    let (path, form) = remote_request(operation)?;
    let method = if form.is_some() { "POST" } else { "GET" };
    let mut body = serde_json::json!({});
    for (key, value) in form.unwrap_or_default() {
        body[key] = match key {
            "source" => serde_json::from_str(&value).map_err(|_| "Invalid peer source JSON")?,
            "job_id" => value
                .parse::<u32>()
                .map_err(|_| "Invalid firmware job")?
                .into(),
            _ => value.into(),
        };
    }
    Ok((method, path, body))
}
pub fn verify_remote_receipt(
    operation: &Operation,
    value: serde_json::Value,
) -> Result<String, String> {
    if value["accepted"] == false {
        return Err(value["error"]
            .as_str()
            .unwrap_or("Maintenance rejected")
            .into());
    }
    let job = parse_job(value.clone())?;
    match operation {
        Operation::PeerStage { source, target, .. } => {
            if job.job_id == 0
                || job.targets != 1
                || job.phase != "download_esp"
                || !target.matches_identity(target.board_id(), &job.boot_id)
                || value["esp"]["sha256"] != source["artifact"]["sha256"]
            {
                return Err("Peer staging receipt differs from the selected source/destination; inspect before retrying".into());
            }
        }
        Operation::RemoteStage {
            sha256, targets, ..
        } if job.job_id == 0
            || job.manifest_sha256 != *sha256
            || job.targets != *targets
            || job.phase != "manifest" =>
        {
            return Err("Peer did not acknowledge the pinned staging request".into());
        }
        Operation::RemoteApply(expected) | Operation::RemoteCancel(expected) => {
            if job.job_id != expected.job_id
                || job.boot_id != expected.boot_id
                || job.manifest_sha256 != expected.manifest_sha256
                || (matches!(operation, Operation::RemoteApply(_)) && job.phase != "prepare")
                || (matches!(operation, Operation::RemoteCancel(_)) && job.phase != "cancelled")
            {
                return Err("Remote mutation acknowledgment did not match this exact job".into());
            }
        }
        _ => {}
    }
    serde_json::to_string_pretty(&value).map_err(|e| e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_untrusted_urls_hashes_and_unobserved_jobs() {
        let sha = "a".repeat(64);
        assert!(
            validate_remote(
                "https://firmware.example/PakarController/manifest.json",
                &sha,
                3
            )
            .is_ok()
        );
        for url in [
            "http://firmware.example/file",
            "https://user:pass@firmware.example/file",
            "https://firmware.example/file#fragment",
            "https://firmware.example/ bad",
        ] {
            assert!(validate_remote(url, &sha, 3).is_err());
        }
        assert!(validate_remote("https://host/file", &"A".repeat(64), 3).is_err());
        assert!(validate_remote("https://host/file", &sha, 4).is_err());
        assert!(parse_job(serde_json::json!({})).is_err());
    }
    #[test]
    fn remote_requests_pin_identity_and_never_replay_lost_or_wrong_ack() {
        use std::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let sha = "b".repeat(64);
        let ready = RemoteJob {
            job_id: 7,
            boot_id: "observed-boot".into(),
            phase: "ready".into(),
            targets: 3,
            manifest_sha256: sha.clone(),
            busy: false,
            cancellable: false,
            received_bytes: 32768,
            expected_bytes: 32768,
            error: String::new(),
        };
        let job = ready.clone();
        let server = std::thread::spawn(move || {
            for (i, stream) in listener.incoming().take(2).enumerate() {
                let mut stream = stream.unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut request = Vec::new();
                let mut chunk = [0; 1024];
                loop {
                    let n = stream.read(&mut chunk).unwrap();
                    assert!(n > 0);
                    request.extend_from_slice(&chunk[..n]);
                    if let Some(end) = request.windows(4).position(|p| p == b"\r\n\r\n") {
                        let header = String::from_utf8_lossy(&request[..end]).to_lowercase();
                        let length = header
                            .lines()
                            .find_map(|s| {
                                s.strip_prefix("content-length: ")
                                    .and_then(|v| v.parse::<usize>().ok())
                            })
                            .unwrap_or(0);
                        if request.len() >= end + 4 + length {
                            break;
                        }
                    }
                }
                let text = String::from_utf8(request).unwrap();
                assert!(text.starts_with(if i == 0 {
                    "POST /api/firmware/fetch"
                } else {
                    "POST /api/firmware/apply"
                }));
                assert!(text.to_lowercase().contains("authorization: basic "));
                let body = text.split("\r\n\r\n").nth(1).unwrap();
                let form: std::collections::HashMap<_, _> =
                    reqwest::Url::parse(&format!("http://fixture/?{body}"))
                        .unwrap()
                        .query_pairs()
                        .into_owned()
                        .collect();
                if i == 0 {
                    assert_eq!(form.get("apply").unwrap(), "false");
                    assert_eq!(form.get("sha256").unwrap(), &job.manifest_sha256);
                } else {
                    assert_eq!(form.get("job_id").unwrap(), "7");
                    assert_eq!(form.get("boot_id").unwrap(), "observed-boot");
                }
                let mut reply = job.clone();
                reply.phase = if i == 0 { "manifest" } else { "prepare" }.into();
                if i == 1 {
                    reply.boot_id = "wrong-boot".into();
                }
                let bytes = serde_json::to_vec(&reply).unwrap();
                write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",bytes.len()).unwrap();
                stream.write_all(&bytes).unwrap();
            }
        });
        let endpoint = Endpoint {
            base: format!("http://{address}"),
            user: "fixture".into(),
            password: "fixture-private".into(),
        };
        let client = Client::builder().no_proxy().build().unwrap();
        assert!(
            run(
                &client,
                &endpoint,
                Operation::RemoteStage {
                    url: "https://firmware.example/manifest.json".into(),
                    sha256: sha,
                    targets: 3,
                    bearer: String::new()
                }
            )
            .is_ok()
        );
        assert!(
            run(&client, &endpoint, Operation::RemoteApply(ready))
                .unwrap_err()
                .contains("exact job")
        );
        server.join().unwrap();
    }
    #[test]
    fn accepted_update_is_not_reported_as_completion() {
        use crate::messaging::Severity;
        let mut value = serde_json::json!({"job_id":1,"boot_id":"observed",
            "phase":"prepare","targets":3,"manifest_sha256":"a".repeat(64),
            "busy":true,"cancellable":false,"received_bytes":0,"expected_bytes":0,"error":""});
        assert_eq!(result_severity(&Ok(value.to_string())), Severity::Info);
        value["phase"] = "complete".into();
        assert_eq!(result_severity(&Ok(value.to_string())), Severity::Success);
        value["phase"] = "failed".into();
        assert_eq!(result_severity(&Ok(value.to_string())), Severity::Error);
        assert_eq!(result_severity(&Err("ACK lost".into())), Severity::Error);
    }
    #[test]
    fn peer_stage_has_destination_fence_separate_source_credentials_and_exact_receipt() {
        use std::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let boot = "0123456789abcdef0123456789abcdef";
        let sha = "b".repeat(64);
        let image_hash = sha.clone();
        let server = std::thread::spawn(move || {
            for (i, stream) in listener.incoming().take(2).enumerate() {
                let mut stream = stream.unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut bytes = Vec::new();
                let mut block = [0; 1024];
                loop {
                    let n = stream.read(&mut block).unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&block[..n]);
                    if let Some(end) = bytes.windows(4).position(|p| p == b"\r\n\r\n") {
                        let header = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
                        let size = header
                            .lines()
                            .find_map(|l| l.strip_prefix("content-length: ")?.parse::<usize>().ok())
                            .unwrap_or(0);
                        if bytes.len() >= end + 4 + size {
                            break;
                        }
                    }
                }
                let text = String::from_utf8(bytes).unwrap();
                let lower = text.to_lowercase();
                assert!(text.starts_with("POST /api/peers/update"));
                assert!(lower.contains("x-pakar-board: destination-board"));
                assert!(lower.contains(&format!("x-pakar-boot: {boot}")));
                assert!(
                    lower.contains(
                        "authorization: basic ZGVzdGluYXRpb246ZGVzdC1zZWNyZXQ="
                            .to_lowercase()
                            .as_str()
                    )
                );
                let form: std::collections::HashMap<_, _> = reqwest::Url::parse(&format!(
                    "http://fixture/?{}",
                    text.split("\r\n\r\n").nth(1).unwrap()
                ))
                .unwrap()
                .query_pairs()
                .into_owned()
                .collect();
                assert_eq!(form["authorization"], "Bearer source-only");
                let source: serde_json::Value = serde_json::from_str(&form["source"]).unwrap();
                assert_eq!(source["artifact"]["sha256"], image_hash);
                let reply=serde_json::json!({"job_id":1,"boot_id":if i==0{boot}else{"abcdef0123456789abcdef0123456789ab"},"phase":"download_esp","targets":1,"manifest_sha256":"a".repeat(64),"busy":true,"cancellable":true,"received_bytes":0,"expected_bytes":4096,"error":"","esp":{"sha256":image_hash}}).to_string();
                write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{reply}",reply.len()).unwrap();
            }
        });
        let endpoint = Endpoint {
            base: format!("http://{address}"),
            user: "destination".into(),
            password: "dest-secret".into(),
        };
        let client = Client::builder().no_proxy().build().unwrap();
        let target = crate::peer_http::Pin::from_identity("destination-board", boot).unwrap();
        let operation = Operation::PeerStage {
            source: serde_json::json!({"artifact":{"sha256":sha}}),
            authorization: "Bearer source-only".into(),
            target,
        };
        assert!(run(&client, &endpoint, operation.clone()).is_ok());
        assert!(
            run(&client, &endpoint, operation)
                .unwrap_err()
                .contains("differs")
        );
        server.join().unwrap();
    }
    #[test]
    fn rejects_wrong_target_or_partial_images() {
        assert!(validate_image(false, &vec![0; 32768]).is_ok());
        assert!(validate_image(false, &vec![0; 32767]).is_err());
        assert!(validate_image(true, &vec![0; 32768]).is_err());
        let mut esp = vec![0; 1024];
        esp[0] = 0xe9;
        esp[1] = 1;
        assert!(validate_image(true, &esp).is_ok());
        assert!(validate_image(true, &[0xe9]).is_err());
    }
    #[test]
    fn s3_image_policy_accepts_actual_large_raw_image_but_not_slot_or_header_mismatch() {
        let mut image = vec![0; 2_220_352];
        image[0] = 0xe9;
        image[1] = 4;
        image[12] = 9;
        image[23] = 1;
        assert!(image.len() > 2 * 1024 * 1024);
        assert!(validate_image(true, &image).is_ok());
        let board = crate::DeviceStatus {
            board_id: "esp32s3-fixture".into(),
            ..Default::default()
        };
        assert!(validate_image_for_board(true, &image, &board).is_ok());
        for size in [S3_OTA_SLOT, S3_OTA_SLOT + 1] {
            let mut oversized = image.clone();
            oversized.resize(size as usize, 0);
            assert!(validate_image(true, &oversized).is_err());
        }
        for (offset, value) in [
            (0, 0),
            (1, 0),
            (1, 17),
            (12, 0),
            (12, 5),
            (13, 1),
            (23, 0),
            (23, 2),
        ] {
            let mut invalid = image.clone();
            invalid[offset] = value;
            assert!(
                validate_image_for_board(true, &invalid, &board).is_err(),
                "Invalid S3 header byte{offset}={value}"
            );
        }
        let mut wrong_small = image[..1024].to_vec();
        wrong_small[12] = 5;
        assert!(validate_image_for_board(true, &wrong_small, &board).is_err());
        assert!(
            validate_image_for_board(
                true,
                &image,
                &crate::DeviceStatus {
                    board_id: "esp8266-fixture".into(),
                    ..Default::default()
                }
            )
            .is_err()
        );
        assert!(validate_image(false, &image).is_err());
        assert!(validate_image(false, &vec![0; 32768]).is_ok());
        for size in [0, 32767, 32769] {
            assert!(validate_image(false, &vec![0; size]).is_err());
        }
        assert_eq!(maximum_image_bytes(true), S3_OTA_SLOT - 1);
        assert_eq!(maximum_image_bytes(false), 32768);
    }
    #[test]
    fn s3_image_policy_respects_smaller_advertised_capacity_without_inventing_it() {
        let mut image = vec![0; 2_220_352];
        image[0] = 0xe9;
        image[1] = 1;
        image[12] = 9;
        image[23] = 1;
        let mut board = crate::DeviceStatus {
            board_id: "esp32s3-fixture".into(),
            ..Default::default()
        };
        assert!(validate_image_for_board(true, &image, &board).is_ok());
        board.ota_capacity_bytes = serde_json::json!(image.len());
        assert!(validate_image_for_board(true, &image, &board).is_err());
        board.ota_capacity_bytes = serde_json::json!(image.len() + 1);
        assert!(validate_image_for_board(true, &image, &board).is_ok());
        for capacity in [
            serde_json::json!(0),
            serde_json::json!(-1),
            serde_json::json!("bad"),
        ] {
            board.ota_capacity_bytes = capacity;
            assert!(validate_image_for_board(true, &image, &board).is_err());
        }
        let mut legacy = vec![0; 1024];
        legacy[0] = 0xe9;
        legacy[1] = 1;
        legacy[23] = 0xfe;
        assert!(
            validate_image(true, &legacy).is_ok(),
            "ESP8266 body byte23 is not an ESP32 hash flag"
        );
        legacy.resize(ESP8266_IMAGE_LIMIT, 0);
        assert!(validate_image(true, &legacy).is_err());
    }
}
