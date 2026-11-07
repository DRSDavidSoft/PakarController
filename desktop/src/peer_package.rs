//! Peer sources for the existing Windows managed installer. Only the running
//! executable is shared; never a caller-selected path or the settings folder.
use serde_json::{Value, json};
use std::{
    fs::File,
    io::{Read, Write},
    net::{IpAddr, TcpStream},
    path::PathBuf,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
};
pub const FAMILY: &str = "pakar-native-windows-x64";
pub const PATH: &str = "/api/peer/package";
const MAX: u64 = 256 * 1024 * 1024;
struct Package {
    path: PathBuf,
    bytes: u64,
    sha: String,
}
static PACKAGE: OnceLock<Result<Package, String>> = OnceLock::new();
static STARTED: AtomicBool = AtomicBool::new(false);
pub fn prime() {
    if !cfg!(windows) || STARTED.swap(true, Ordering::AcqRel) {
        return;
    }
    std::thread::spawn(|| {
        let value = (|| {
            let path = std::env::current_exe()
                .map_err(|_| "Executable unavailable")?
                .canonicalize()
                .map_err(|_| "Executable unavailable")?;
            crate::client_install::regular(&path)?;
            crate::client_install::inspect_pe(&path)?;
            let bytes = std::fs::metadata(&path)
                .map_err(|_| "Executable unavailable")?
                .len();
            if !(1..=MAX).contains(&bytes) {
                return Err("Executable exceeds sharing limit".into());
            }
            Ok(Package {
                sha: crate::client_install::hash(&path)?,
                bytes,
                path,
            })
        })();
        let _ = PACKAGE.set(value);
    });
}
pub fn artifact() -> Option<Value> {
    let p = PACKAGE.get()?.as_ref().ok()?;
    Some(
        json!({"family":FAMILY,"version":crate::peer_identity::VERSION,"build":crate::client_management::build(),"bytes":p.bytes,"sha256":p.sha,"path":PATH,"platform":"windows-x64"}),
    )
}
pub fn state() -> &'static str {
    if !cfg!(windows) {
        "unsupported-platform"
    } else {
        match PACKAGE.get() {
            Some(Ok(_)) => "ready",
            Some(Err(_)) => "unavailable",
            None => "preparing",
        }
    }
}
pub fn serve(socket: &mut TcpStream, instance: &str, head: bool) -> Result<(), String> {
    let p = PACKAGE
        .get()
        .and_then(|p| p.as_ref().ok())
        .ok_or("Peer package is not ready")?;
    crate::client_install::regular(&p.path)?;
    let mut file = File::open(&p.path).map_err(|_| "Peer package unavailable")?;
    if file
        .metadata()
        .map_err(|_| "Peer package unavailable")?
        .len()
        != p.bytes
    {
        return Err("Peer package changed; restart sharing".into());
    }
    socket
        .set_write_timeout(Some(std::time::Duration::from_secs(5)))
        .map_err(|e| e.to_string())?;
    write!(socket,"HTTP/1.1 200 OK\r\n{}Content-Type: application/octet-stream\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Pakar-Peer: {}\r\nX-Pakar-SHA256: {}\r\nConnection: close\r\n\r\n",crate::peer_identity::server_headers(),p.bytes,instance,p.sha).map_err(|e|e.to_string())?;
    if head {
        return Ok(());
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    let mut remaining = p.bytes;
    let mut buffer = [0u8; 16384];
    while remaining > 0 {
        if std::time::Instant::now() > deadline {
            return Err("Peer export deadline elapsed".into());
        }
        let n = file
            .read(&mut buffer[..remaining.min(16384) as usize])
            .map_err(|e| e.to_string())?;
        if n == 0 {
            return Err("Peer executable changed during export".into());
        }
        socket.write_all(&buffer[..n]).map_err(|e| e.to_string())?;
        remaining -= n as u64;
    }
    Ok(())
}
pub fn release(
    info: &Value,
    endpoint: &str,
    current_version: &str,
    current_build: u64,
) -> Result<crate::client_management::Release, String> {
    if !matches!(info["kind"].as_str(), Some("desktop-app" | "bridge"))
        || !info["instance_id"]
            .as_str()
            .is_some_and(|s| uuid::Uuid::parse_str(s).is_ok())
        || !info["session_id"]
            .as_str()
            .is_some_and(|s| uuid::Uuid::parse_str(s).is_ok())
    {
        return Err("Native peer identity/session missing".into());
    }
    let a = info["artifacts"]
        .as_array()
        .and_then(|a| a.iter().find(|a| a["family"] == FAMILY))
        .ok_or("No compatible native package advertised")?;
    let version = a["version"].as_str().ok_or("Peer release missing")?;
    let build = a["build"].as_u64().ok_or("Peer build missing")?;
    let bytes = a["bytes"].as_u64().unwrap_or(0);
    let sha = a["sha256"].as_str().unwrap_or("");
    if a["path"] != PATH
        || a["platform"] != "windows-x64"
        || build <= current_build
        || !(1..=MAX).contains(&bytes)
        || !crate::maintenance::valid_hash(sha)
        || crate::peer_identity::release_order(version, current_version).is_none_or(|o| o.is_lt())
    {
        return Err("Peer package is not a newer compatible native release".into());
    }
    Ok(crate::client_management::Release {
        product: "PakarController".into(),
        platform: "windows-x64".into(),
        version: version.into(),
        build,
        bytes,
        sha256: sha.into(),
        url: format!("{}{PATH}", endpoint.trim_end_matches('/')),
    })
}
pub fn download(
    selected: &crate::peer_catalogue::Peer,
    progress: impl FnMut(u64, u64) -> Result<(), String>,
) -> Result<(crate::client_management::Release, PathBuf), String> {
    download_at(selected, None, progress)
}
fn download_at(
    selected: &crate::peer_catalogue::Peer,
    root: Option<PathBuf>,
    mut progress: impl FnMut(u64, u64) -> Result<(), String>,
) -> Result<(crate::client_management::Release, PathBuf), String> {
    if !cfg!(windows) {
        return Err("Managed native peer upgrades are supported on Windows x64 only".into());
    }
    // Fresh probe prevents using an old discovery row after DHCP/session change.
    let fresh = crate::peer_catalogue::probe(&selected.endpoint)
        .ok_or("Peer no longer answers identity")?;
    if fresh.key() != selected.key()
        || fresh.identity["session_id"] != selected.identity["session_id"]
    {
        return Err("Peer session changed; inspect it again".into());
    }
    let release = release(
        &fresh.identity,
        &fresh.endpoint,
        crate::peer_identity::VERSION,
        crate::client_management::build(),
    )?;
    if release
        != self::release(
            &selected.identity,
            &selected.endpoint,
            crate::peer_identity::VERSION,
            crate::client_management::build(),
        )?
    {
        return Err("Peer artifact changed; inspect it again".into());
    }
    let address = fresh.address.ok_or("Peer TCP route unavailable")?;
    let url = reqwest::Url::parse(&release.url).map_err(|_| "Invalid peer endpoint")?;
    let private = matches!(address.ip(),IpAddr::V4(ip) if ip.is_private()||ip.is_loopback());
    if !private && url.scheme() != "https" {
        return Err("Public peer downloads require HTTPS".into());
    }
    let path = root
        .map(Ok)
        .unwrap_or_else(crate::client_install::cache)?
        .join(format!("{}.pending.exe", release.sha256));
    if path.exists() {
        crate::client_install::inspect_pe(&path)?;
        if std::fs::metadata(&path)
            .map_err(|_| "Candidate unavailable")?
            .len()
            != release.bytes
            || crate::client_install::hash(&path)? != release.sha256
        {
            return Err("Existing candidate differs; preserved for inspection".into());
        }
        progress(release.bytes, release.bytes)?;
        return Ok((release, path));
    }
    let client = crate::peer_identity::client_builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .resolve(url.host_str().ok_or("Peer host missing")?, address)
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;
    let mut response = client
        .get(url)
        .header("Accept-Encoding", "identity")
        .send()
        .map_err(|_| "Peer download unavailable")?;
    if response.status() != reqwest::StatusCode::OK
        || response.content_length() != Some(release.bytes)
        || response.remote_addr() != Some(address)
        || response.headers().contains_key("content-encoding")
        || response.headers().contains_key("transfer-encoding")
        || response
            .headers()
            .get("x-pakar-peer")
            .and_then(|v| v.to_str().ok())
            != fresh.identity["instance_id"].as_str()
        || response
            .headers()
            .get("x-pakar-client-id")
            .and_then(|v| v.to_str().ok())
            != fresh.identity["session_id"].as_str()
        || response
            .headers()
            .get("x-pakar-sha256")
            .and_then(|v| v.to_str().ok())
            != Some(release.sha256.as_str())
    {
        return Err("Peer package identity/session/size changed".into());
    }
    let partial = path.with_extension("partial");
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&partial)
        .map_err(|_| "Peer staging path unavailable; existing file preserved")?;
    let result: Result<(), String> = (|| {
        use sha2::{Digest, Sha256};
        let mut digest = Sha256::new();
        let mut done = 0u64;
        let mut buffer = [0u8; 16384];
        loop {
            progress(done, release.bytes)?;
            let n = response
                .read(&mut buffer)
                .map_err(|_| "Peer transfer interrupted")?;
            if n == 0 {
                break;
            }
            done = done.checked_add(n as u64).ok_or("Peer transfer overflow")?;
            if done > release.bytes {
                return Err("Peer package exceeds declared size".into());
            }
            file.write_all(&buffer[..n])
                .map_err(|_| "Cannot stage peer bytes")?;
            digest.update(&buffer[..n]);
        }
        file.sync_all().map_err(|_| "Cannot persist peer package")?;
        let sha = digest
            .finalize()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        if done != release.bytes || sha != release.sha256 {
            return Err("Peer package integrity check failed".into());
        }
        progress(done, release.bytes)?;
        Ok(())
    })();
    drop(file);
    result?; // An interrupted owned partial remains inspectable, never applied.
    crate::client_install::inspect_pe(&partial)?;
    std::fs::rename(&partial, &path).map_err(|_| "Cannot publish peer candidate")?;
    crate::client_install::write_new(&path.with_extension("release.json"), &release)?;
    Ok((release, path))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_release_is_platform_and_build_bound() {
        let mut info = json!({"kind":"desktop-app","instance_id":uuid::Uuid::new_v4().to_string(),"session_id":uuid::Uuid::new_v4().to_string(),"artifacts":[{"family":FAMILY,"platform":"windows-x64","path":PATH,"version":"0.2.8","build":24,"bytes":1000,"sha256":"a".repeat(64)}]});
        assert!(release(&info, "http://peer.local:8789", "0.2.8", 23).is_ok());
        assert!(release(&info, "http://peer.local:8789", "0.2.8", 24).is_err());
        info["artifacts"][0]["family"] = "pakar-android".into();
        assert!(release(&info, "http://peer.local", "0.2.8", 23).is_err());
    }
    #[cfg(windows)]
    #[test]
    fn standalone_peer_download_pins_session_hash_and_reuses_verified_candidate() {
        use sha2::{Digest, Sha256};
        use std::{
            net::TcpListener,
            time::{Duration, Instant},
        };
        let mut bytes = vec![0u8; 512];
        bytes[..2].copy_from_slice(b"MZ");
        bytes[60..64].copy_from_slice(&128u32.to_le_bytes());
        bytes[128..132].copy_from_slice(b"PE\0\0");
        bytes[132..134].copy_from_slice(&0x8664u16.to_le_bytes());
        let sha = Sha256::digest(&bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let id = uuid::Uuid::new_v4().to_string();
        let session = uuid::Uuid::new_v4().to_string();
        let info = json!({"product":"PakarController","kind":"desktop-app","instance_id":id,"session_id":session,"artifacts":[{"family":FAMILY,"platform":"windows-x64","path":PATH,"version":crate::peer_identity::VERSION,"build":crate::client_management::build()+1,"bytes":bytes.len(),"sha256":sha}]});
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        listener.set_nonblocking(true).unwrap();
        let selected =
            crate::peer_catalogue::parse_identity(&origin, info.clone(), None, None).unwrap();
        let worker = std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(15);
            let mut exports = 0;
            for _ in 0..3 {
                let mut socket = loop {
                    match listener.accept() {
                        Ok((s, _)) => break s,
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            assert!(Instant::now() < deadline, "bounded peer fixture accept");
                            std::thread::sleep(Duration::from_millis(5));
                        }
                        Err(e) => panic!("{e}"),
                    }
                };
                // Accepted sockets can inherit a polled listener's mode on
                // Windows. Reuse the peer_http fixture's bounded blocking IO.
                socket.set_nonblocking(false).unwrap();
                socket
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                socket
                    .set_write_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut request = Vec::new();
                let mut block = [0u8; 512];
                while !request.windows(4).any(|w| w == b"\r\n\r\n") {
                    let n = socket.read(&mut block).unwrap();
                    assert!(n > 0 && request.len() + n <= 8192);
                    request.extend_from_slice(&block[..n]);
                }
                let is_image =
                    String::from_utf8_lossy(&request).starts_with(&format!("GET {PATH} "));
                let body = if is_image {
                    exports += 1;
                    bytes.clone()
                } else {
                    serde_json::to_vec(&info).unwrap()
                };
                write!(socket,"HTTP/1.1 200 OK\r\nContent-Length: {}\r\nX-Pakar-Peer: {}\r\nX-Pakar-Client-Id: {}\r\nX-Pakar-SHA256: {}\r\nConnection: close\r\n\r\n",body.len(),id,session,sha).unwrap();
                socket.write_all(&body).unwrap();
            }
            assert_eq!(exports, 1); // second receive probes identity but does NOT re-stream the EXE
        });
        let root =
            std::env::temp_dir().join(format!("pakar-peer-fixture-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&root).unwrap();
        let mut progress = 0;
        let (release, path) = download_at(&selected, Some(root.clone()), |done, total| {
            assert!(done <= total);
            progress += 1;
            Ok(())
        })
        .unwrap();
        assert!(progress > 1);
        assert_eq!(crate::client_install::hash(&path).unwrap(), release.sha256);
        crate::client_install::inspect_pe(&path).unwrap();
        let (_, reused) = download_at(&selected, Some(root.clone()), |_, _| Ok(())).unwrap();
        assert_eq!(reused, path);
        worker.join().unwrap();
        // Only the exact two fixture files are removed; no recursive cleanup.
        std::fs::remove_file(path.with_extension("release.json")).unwrap();
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}
