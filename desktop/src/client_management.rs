//! App management is independent of board firmware. Adapt actual Pealayer
//! update.rs streaming/hash/size/PE gates and PCController's secure enrollment
//! backend. HTTPS origin pinning and durable pre-claim intent prevent replay.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
pub const DEFAULT_BASE: &str = "https://office.digitalogic.ir/pakar";
const PLATFORM: &str = "windows-x64";
const MAX_PACKAGE: u64 = 256 * 1024 * 1024;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub base: String,
    pub auto_check: bool,
    pub auto_download: bool,
    pub auto_apply: bool,
    pub auto_cleanup: bool,
    pub telemetry: bool,
    pub remote_commands: bool,
    pub allow_remote_cleanup: bool,
    pub allow_remote_uninstall: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            base: DEFAULT_BASE.into(),
            auto_check: true,
            auto_download: true,
            auto_apply: true,
            auto_cleanup: true,
            telemetry: false,
            remote_commands: false,
            allow_remote_cleanup: false,
            allow_remote_uninstall: false,
        }
    }
}
impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        base(&self.base).map(|_| ())
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Release {
    pub product: String,
    pub platform: String,
    pub version: String,
    pub build: u64,
    pub bytes: u64,
    pub sha256: String,
    pub url: String,
}
#[derive(Clone, Default, Serialize)]
pub struct Snapshot {
    pub state: String,
    pub detail: String,
    pub received_bytes: u64,
    pub expected_bytes: Option<u64>,
    pub release: Option<Release>,
    pub enrolled: bool,
    pub client_id: Option<String>,
    pub managed_installation: bool,
    pub applying: bool,
    pub package_verified: bool,
    pub peer_candidate: bool,
}
#[derive(Serialize, Deserialize)]
struct Enrollment {
    client_id: String,
    token: String,
    base: String,
}
#[derive(Default, Serialize, Deserialize)]
struct Commands {
    attempted: Vec<String>,
}
#[derive(Deserialize)]
struct RemoteCommand {
    id: String,
    action: String,
    expires_at_ms: u64,
}
pub enum Action {
    Check,
    Download,
    Apply,
    Cleanup,
    Enroll(String),
    Heartbeat,
}
struct Engine {
    snapshot: Mutex<Snapshot>,
    busy: AtomicBool,
    last_check: Mutex<Option<Instant>>,
    last_report: Mutex<Option<Instant>>,
    ready_quit: Mutex<Option<Instant>>,
    health_attempted: AtomicBool,
    apply_requested: Mutex<Option<String>>,
    started: Instant,
    settings: Mutex<Option<Settings>>,
    epoch: AtomicU64,
    credential_epoch: AtomicU64,
    staged: Mutex<Option<String>>,
}
fn engine() -> &'static Engine {
    static ENGINE: OnceLock<Engine> = OnceLock::new();
    ENGINE.get_or_init(|| Engine {
        snapshot: Mutex::new(Snapshot {
            state: "idle".into(),
            ..Default::default()
        }),
        busy: AtomicBool::new(false),
        last_check: Mutex::new(None),
        last_report: Mutex::new(None),
        ready_quit: Mutex::new(None),
        health_attempted: AtomicBool::new(false),
        apply_requested: Mutex::new(None),
        started: Instant::now(),
        settings: Mutex::new(None),
        epoch: AtomicU64::new(0),
        credential_epoch: AtomicU64::new(0),
        staged: Mutex::new(None),
    })
}
fn bind(settings: &Settings) -> u64 {
    let e = engine();
    if let Ok(mut old) = e.settings.lock() {
        if old.as_ref() != Some(settings) {
            *old = Some(settings.clone());
            e.epoch.fetch_add(1, Ordering::AcqRel);
            if let Ok(mut requested) = e.apply_requested.lock() {
                *requested = None;
            }
            if let Ok(mut ready) = e.ready_quit.lock() {
                *ready = None;
            }
            if let Ok(mut staged) = e.staged.lock() {
                *staged = None;
            }
            if let Ok(mut report) = e.last_report.lock() {
                *report = None;
            }
            if let Ok(mut value) = e.snapshot.lock() {
                value.release = None;
                value.enrolled = false;
                value.client_id = None;
                value.applying = false;
                value.package_verified = false;
                value.peer_candidate = false;
                value.state = "idle".into();
                value.detail.clear();
            }
        }
    }
    e.epoch.load(Ordering::Acquire)
}
fn guard(settings: &Settings, epoch: u64) -> Result<(), String> {
    if engine().settings.lock().is_ok_and(|s| {
        s.as_ref() == Some(settings) && engine().epoch.load(Ordering::Acquire) == epoch
    }) {
        Ok(())
    } else {
        Err("App management settings changed; old operation revoked without replay".into())
    }
}
pub fn snapshot() -> Snapshot {
    engine()
        .snapshot
        .lock()
        .map(|v| v.clone())
        .unwrap_or_default()
}
pub fn busy() -> bool {
    engine().busy.load(Ordering::Acquire)
}
fn state(name: &str, detail: &str) {
    if let Ok(mut value) = engine().snapshot.lock() {
        value.state = name.into();
        value.detail = detail.into();
    }
}
pub fn build() -> u64 {
    serde_json::from_str::<serde_json::Value>(include_str!("../../interfaces/client-release.json"))
        .ok()
        .and_then(|v| v["build"].as_u64())
        .unwrap_or(0)
}
fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u128::from(u64::MAX)) as u64
}
fn command_admissible(command: &RemoteCommand, now: u64) -> bool {
    command.id.len() == 32
        && command.id.bytes().all(|v| v.is_ascii_hexdigit())
        && command.expires_at_ms > now
        && command.expires_at_ms <= now.saturating_add(60_000)
        && matches!(
            command.action.as_str(),
            "check-update" | "cleanup" | "uninstall"
        )
}
fn base(value: &str) -> Result<reqwest::Url, String> {
    let value = reqwest::Url::parse(value).map_err(|_| "Choose a complete HTTPS management URL")?;
    if value.scheme() != "https"
        || value.host_str().is_none()
        || !value.username().is_empty()
        || value.password().is_some()
        || value.query().is_some()
        || value.fragment().is_some()
    {
        return Err("Management uses HTTPS without credentials, query or fragment".into());
    }
    Ok(value)
}
fn endpoint(settings: &Settings, path: &str) -> Result<String, String> {
    Ok(format!(
        "{}{}",
        base(&settings.base)?.as_str().trim_end_matches('/'),
        path
    ))
}
fn artifact(settings: &Settings, url: &str) -> Result<reqwest::Url, String> {
    let origin = base(&settings.base)?;
    let candidate = base(url)?;
    if candidate.origin() != origin.origin() {
        return Err("Release artifact must use the configured trusted HTTPS origin".into());
    }
    Ok(candidate)
}
fn client() -> Result<reqwest::blocking::Client, String> {
    crate::peer_identity::client_builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(Duration::from_secs(8))
        .timeout(Duration::from_secs(120))
        .build()
        .map_err(|_| "Cannot create management transport".into())
}
fn json(response: reqwest::blocking::Response) -> Result<serde_json::Value, String> {
    if !response.status().is_success() {
        return Err(format!(
            "Management request rejected: HTTP {}",
            response.status()
        ));
    }
    let mut bytes = Vec::new();
    response
        .take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Management response interrupted")?;
    if bytes.len() > 1024 * 1024 {
        return Err("Management response exceeds its bound".into());
    }
    serde_json::from_slice(&bytes).map_err(|_| "Invalid management JSON".into())
}
fn version(value: &str) -> Option<[u64; 3]> {
    let parts: Vec<_> = value.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    let mut out = [0; 3];
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty()
            || !part.bytes().all(|v| v.is_ascii_digit())
            || (part.len() > 1 && part.starts_with('0'))
        {
            return None;
        }
        out[i] = part.parse().ok()?;
    }
    Some(out)
}
pub fn newer(
    release: &Release,
    current_version: &str,
    current_build: u64,
    settings: &Settings,
) -> bool {
    release.product == "PakarController"
        && release.platform == PLATFORM
        && release.build > current_build
        && version(&release.version)
            .zip(version(current_version))
            .is_some_and(|(new, old)| new >= old)
        && (1..=MAX_PACKAGE).contains(&release.bytes)
        && crate::maintenance::valid_hash(&release.sha256)
        && artifact(settings, &release.url).is_ok()
}
fn check(settings: &Settings, epoch: u64) -> Result<(), String> {
    guard(settings, epoch)?;
    state("checking", "Checking app releases (not board firmware)");
    let response = client()?
        .get(endpoint(settings, "/api/releases")?)
        .query(&[("platform", PLATFORM)])
        .send()
        .map_err(|_| "App release check unavailable")?;
    let value = json(response)?;
    guard(settings, epoch)?;
    let releases: Vec<Release> = serde_json::from_value(value["releases"].clone())
        .map_err(|_| "Invalid app release inventory")?;
    let found = releases
        .into_iter()
        .filter(|r| newer(r, env!("CARGO_PKG_VERSION"), build(), settings))
        .max_by_key(|r| (version(&r.version), r.build));
    if let Ok(mut value) = engine().snapshot.lock() {
        value.release = found.clone();
        value.state = if found.is_some() {
            "available"
        } else {
            "current"
        }
        .into();
        value.detail = if found.is_some() {
            "A newer verified-manifest app package is available"
        } else {
            "No newer applicable stable app build is published"
        }
        .into();
    }
    *engine()
        .last_check
        .lock()
        .map_err(|_| "Update coordinator unavailable")? = Some(Instant::now());
    if found.is_some() && settings.auto_download {
        download(settings, epoch)?;
    }
    Ok(())
}
fn candidate(release: &Release) -> Result<PathBuf, String> {
    Ok(crate::client_install::cache()?.join(format!("{}.pending.exe", release.sha256)))
}
fn download(settings: &Settings, epoch: u64) -> Result<(), String> {
    guard(settings, epoch)?;
    if let Ok(mut value) = engine().snapshot.lock() {
        value.peer_candidate = false;
    }
    let release = snapshot().release.ok_or("Check for an app release first")?;
    if !newer(&release, env!("CARGO_PKG_VERSION"), build(), settings) {
        return Err("Release no longer passes the app upgrade policy".into());
    }
    let path = candidate(&release)?;
    if path.exists() {
        crate::client_install::regular(&path)?;
        if fs::metadata(&path)
            .map_err(|_| "Candidate unavailable")?
            .len()
            != release.bytes
            || crate::client_install::hash(&path)? != release.sha256
        {
            return Err(
                "Existing staged package identity differs; preserved for inspection".into(),
            );
        }
        crate::client_install::inspect_pe(&path)?;
        guard(settings, epoch)?;
        *engine()
            .staged
            .lock()
            .map_err(|_| "Update coordinator unavailable")? = Some(release.sha256.clone());
        if let Ok(mut value) = engine().snapshot.lock() {
            value.package_verified = true;
        }
        state(
            "staged",
            "Verified app package is ready; settings and rollback remain untouched",
        );
        return Ok(());
    }
    let partial = path.with_extension("partial");
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&partial)
        .map_err(|_| "Partial package already exists or staging is unavailable")?;
    state("downloading", "Downloading one bounded app package");
    let result = (|| -> Result<(), String> {
        let mut response = client()?
            .get(artifact(settings, &release.url)?)
            .send()
            .map_err(|_| "App download unavailable")?;
        if !response.status().is_success()
            || response
                .content_length()
                .is_some_and(|n| n != release.bytes)
        {
            return Err("App download response/size differs from the release".into());
        }
        let mut hasher = Sha256::new();
        let mut done = 0_u64;
        let mut buffer = [0_u8; 65536];
        loop {
            guard(settings, epoch)?;
            let n = response
                .read(&mut buffer)
                .map_err(|_| "App download interrupted; not applied")?;
            if n == 0 {
                break;
            }
            done = done.checked_add(n as u64).ok_or("App download overflow")?;
            if done > release.bytes {
                return Err("App download exceeds the declared bound".into());
            }
            file.write_all(&buffer[..n])
                .map_err(|_| "Cannot stage app package")?;
            hasher.update(&buffer[..n]);
            if let Ok(mut value) = engine().snapshot.lock() {
                value.received_bytes = done;
                value.expected_bytes = Some(release.bytes);
            }
        }
        file.flush()
            .and_then(|_| file.sync_all())
            .map_err(|_| "Cannot persist app package")?;
        if done != release.bytes
            || hasher
                .finalize()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>()
                != release.sha256
        {
            return Err("App package size/SHA-256 verification failed".into());
        }
        Ok(())
    })();
    drop(file);
    if let Err(error) = result {
        let _ = fs::remove_file(&partial);
        return Err(error);
    }
    guard(settings, epoch)?;
    crate::client_install::inspect_pe(&partial)?;
    fs::rename(&partial, &path).map_err(|_| "Cannot publish verified staged app")?;
    crate::client_install::write_new(&path.with_extension("release.json"), &release)?;
    *engine()
        .staged
        .lock()
        .map_err(|_| "Update coordinator unavailable")? = Some(release.sha256.clone());
    if let Ok(mut value) = engine().snapshot.lock() {
        value.package_verified = true;
    }
    state(
        "staged",
        "App package verified; managed graceful replacement is pending",
    );
    if settings.auto_cleanup {
        cleanup(Some(&release.sha256), settings, epoch)?;
    }
    Ok(())
}
fn cleanup(protected: Option<&str>, settings: &Settings, epoch: u64) -> Result<usize, String> {
    guard(settings, epoch)?;
    let directory = crate::client_install::cache()?;
    let transactions = crate::client_install::protected_candidates()?;
    let mut count = 0;
    for entry in fs::read_dir(&directory)
        .map_err(|_| "Cannot inspect owned app staging")?
        .flatten()
    {
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        let Some(sha) = name.strip_suffix(".pending.exe") else {
            continue;
        };
        if !crate::maintenance::valid_hash(sha) || protected == Some(sha) {
            continue;
        }
        let path = entry.path();
        let record = path.with_extension("release.json");
        if transactions.contains(&path) {
            continue;
        }
        crate::client_install::regular(&path)?;
        crate::client_install::regular(&record)?;
        if fs::metadata(&record)
            .map_err(|_| "Cannot inspect pending package record")?
            .len()
            > 32768
        {
            return Err("Oversized staging record preserved".into());
        }
        let release: Release = serde_json::from_slice(
            &fs::read(&record).map_err(|_| "Cannot read owned package record")?,
        )
        .map_err(|_| "Unknown staging record preserved")?;
        if release.sha256 != sha
            || fs::metadata(&path)
                .map_err(|_| "Cannot inspect old staged package")?
                .len()
                != release.bytes
            || crate::client_install::hash(&path)? != sha
        {
            continue;
        }
        // Only this exact verified obsolete pending pair. Never caches, active
        // executable, rollback, helper, journal, preferences or arbitrary files.
        guard(settings, epoch)?;
        fs::remove_file(&path).map_err(|_| "Cannot remove old owned pending package")?;
        fs::remove_file(&record).map_err(|_| "Cannot remove old pending record")?;
        count += 1;
    }
    Ok(count)
}
fn secret_key(settings: &Settings) -> Result<String, String> {
    Ok(format!(
        "PakarController/client-management/{}",
        Sha256::digest(
            base(&settings.base)?
                .as_str()
                .trim_end_matches('/')
                .as_bytes()
        )
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>()
    ))
}
fn enrollment(settings: &Settings) -> Result<Option<Enrollment>, String> {
    crate::client_secrets::get(&secret_key(settings)?)?
        .map(|value| {
            serde_json::from_str::<Enrollment>(&value)
                .map_err(|_| "Stored enrollment is invalid".to_string())
        })
        .transpose()
}
fn enroll(settings: &Settings, epoch: u64, mut token: String) -> Result<(), String> {
    guard(settings, epoch)?;
    if token.len() < 8 || token.len() > 512 || token.chars().any(char::is_control) {
        return Err("Enter a valid enrollment token".into());
    }
    let id = enrollment(settings)?
        .map(|e| e.client_id)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let response=client()?.post(endpoint(settings,"/api/clients/enroll")?).json(&serde_json::json!({"enrollment_token":token,"client_id":id,"platform":PLATFORM,"application":"PakarController"}))
        .send().map_err(|_|"Enrollment outcome unknown; request was not replayed")?;
    token.clear();
    let value = json(response)?;
    guard(settings, epoch)?;
    let returned = value["client_id"]
        .as_str()
        .ok_or("Enrollment omitted client identity")?;
    let secret = value["token"]
        .as_str()
        .filter(|t| t.len() >= 16 && t.len() <= 2048 && !t.chars().any(char::is_control))
        .ok_or("Enrollment omitted a valid credential")?;
    if returned != id {
        return Err("Enrollment returned another client identity; nothing saved".into());
    }
    let e = Enrollment {
        client_id: id.clone(),
        token: secret.into(),
        base: base(&settings.base)?.as_str().trim_end_matches('/').into(),
    };
    crate::client_secrets::set(
        &secret_key(settings)?,
        &serde_json::to_string(&e).map_err(|_| "Enrollment cannot be saved")?,
    )?;
    if let Ok(mut value) = engine().snapshot.lock() {
        value.enrolled = true;
        value.client_id = Some(id);
    }
    state(
        "enrolled",
        "Client enrolled; credential is held only in the current-user Windows store",
    );
    Ok(())
}
fn remember_command(id: &str) -> Result<bool, String> {
    let path = crate::client_install::cache()?.join("remote-commands.json");
    let mut log = if path.exists() {
        crate::client_install::regular(&path)?;
        let bytes = fs::read(&path).map_err(|_| "Command journal unavailable")?;
        if bytes.len() > 262144 {
            return Err("Command journal bound exceeded".into());
        }
        serde_json::from_slice::<Commands>(&bytes)
            .map_err(|_| "Command journal invalid; no remote action")?
    } else {
        Commands::default()
    };
    if log.attempted.iter().any(|old| old == id) {
        return Ok(false);
    }
    if log.attempted.len() >= 1024 {
        return Err("Command journal full; inspect before accepting more actions".into());
    }
    log.attempted.push(id.into());
    let pending = path.with_extension("pending");
    crate::client_install::write_new(&pending, &log)?;
    fs::rename(pending, path).map_err(|_| "Cannot commit at-most-once command intent")?;
    Ok(true)
}
fn heartbeat(settings: &Settings, epoch: u64) -> Result<(), String> {
    guard(settings, epoch)?;
    let Some(e) = enrollment(settings)? else {
        state(
            "not-enrolled",
            "Enroll this client before enabling management telemetry",
        );
        return Ok(());
    };
    if e.base != base(&settings.base)?.as_str().trim_end_matches('/') {
        return Err("Enrollment belongs to another management origin".into());
    }
    if let Ok(mut value) = engine().snapshot.lock() {
        value.enrolled = true;
        value.client_id = Some(e.client_id.clone());
    }
    let c = client()?;
    let current = snapshot();
    if settings.telemetry {
        json(c.post(endpoint(settings,"/api/clients/report")?).bearer_auth(&e.token).json(&serde_json::json!({
        "client_id":e.client_id,"application":"PakarController","platform":PLATFORM,"version":env!("CARGO_PKG_VERSION"),"build":build(),
        "uptime_seconds":engine().started.elapsed().as_secs(),"update_state":current.state,"managed_installation":current.managed_installation,
        "auto_check":settings.auto_check,"auto_update":settings.auto_download&&settings.auto_apply,"auto_cleanup":settings.auto_cleanup,"allow_remote_uninstall":false,
        "architecture":std::env::consts::ARCH,"os":std::env::consts::OS,"hostname":std::env::var("COMPUTERNAME").unwrap_or_default(),
        "received_bytes":current.received_bytes,"expected_bytes":current.expected_bytes.unwrap_or(0)}))
        .send().map_err(|_|"Client telemetry unavailable")?)?;
    }
    if settings.remote_commands {
        let value = json(
            c.get(endpoint(settings, "/api/client/commands")?)
                .bearer_auth(&e.token)
                .send()
                .map_err(|_| "Remote command list unavailable")?,
        )?;
        let commands: Vec<RemoteCommand> = serde_json::from_value(value["commands"].clone())
            .map_err(|_| "Invalid remote command list")?;
        for command in commands.into_iter().take(16) {
            if !command_admissible(&command, now_ms()) {
                continue;
            }
            guard(settings, epoch)?;
            if !remember_command(&command.id)? {
                continue;
            }
            let claim = json(
                c.post(endpoint(
                    settings,
                    &format!("/api/client/commands/{}/claim", command.id),
                )?)
                .bearer_auth(&e.token)
                .json(&serde_json::json!({}))
                .send()
                .map_err(|_| "Remote claim outcome unknown; action not executed or replayed")?,
            )?;
            if claim["claimed"] != true {
                continue;
            }
            let result=guard(settings,epoch).and_then(|_|if command.expires_at_ms<=now_ms(){Err("Claimed command expired before execution; not replayed".into())}else{Ok(())}).and_then(|_|match command.action.as_str(){
                "check-update"=>check(settings,epoch),
                "cleanup" if settings.allow_remote_cleanup=>cleanup(snapshot().release.as_ref().map(|r|r.sha256.as_str()),settings,epoch).map(|_|()),
                "uninstall"=>Err(if settings.allow_remote_uninstall{"Uninstall policy is opted in, but removal inventory still requires explicit local approval"}else{"Remote uninstall is disabled by local policy"}.into()),
                _=>Err("Action disabled by local policy".into())});
            let (state, detail) = match result {
                Ok(()) => ("completed", "Local policy action completed".to_string()),
                Err(detail) => ("blocked", detail),
            };
            json(
                c.post(endpoint(
                    settings,
                    &format!("/api/client/commands/{}/result", command.id),
                )?)
                .bearer_auth(&e.token)
                .json(&serde_json::json!({"state":state,"detail":detail}))
                .send()
                .map_err(|_| "Remote result delivery uncertain; action will not be replayed")?,
            )?;
        }
    }
    Ok(())
}
pub fn request(action: Action, settings: Settings, ctx: &eframe::egui::Context) {
    if !matches!(action, Action::Apply | Action::Cleanup)
        && let Err(error) = settings.validate()
    {
        state("blocked", &error);
        return;
    }
    let epoch = bind(&settings);
    if matches!(action, Action::Apply) {
        let current = snapshot();
        let Some(release) = current.release.filter(|_| current.package_verified) else {
            state("blocked", "No verified app package is ready to apply");
            return;
        };
        if let Ok(mut requested) = engine().apply_requested.lock() {
            *requested = Some(release.sha256);
        }
        state(
            "staged",
            "Waiting for all Preferences/device drafts and pending operations to close",
        );
        return;
    }
    if engine().busy.swap(true, Ordering::AcqRel) {
        return;
    }
    let ctx = ctx.clone();
    std::thread::spawn(move || {
        let result=match action{Action::Check=>check(&settings,epoch),Action::Download=>download(&settings,epoch),Action::Cleanup=>cleanup(snapshot().release.as_ref().map(|r|r.sha256.as_str()),&settings,epoch).map(|n|state("cleaned",&format!("Removed {n} obsolete owned pending packages; active/settings/rollback preserved"))),
            Action::Enroll(token)=>enroll(&settings,epoch,token),Action::Heartbeat=>heartbeat(&settings,epoch),Action::Apply=>unreachable!()};
        if guard(&settings, epoch).is_ok() {
            if let Err(error) = result {
                state("blocked", &error);
            }
        }
        engine().busy.store(false, Ordering::Release);
        ctx.request_repaint();
    });
}
/// Local explicit receive from a discovered app; no board or cloud enrollment
/// prerequisite. Reuses the SAME candidate/managed replacement lifecycle.
pub fn request_peer(
    peer: crate::peer_catalogue::Peer,
    settings: Settings,
    ctx: &eframe::egui::Context,
) {
    // LAN peer reception does not depend on cloud enrollment or a cloud URL.
    let epoch = bind(&settings);
    if engine().busy.swap(true, Ordering::AcqRel) {
        return;
    }
    if let Ok(mut requested) = engine().apply_requested.lock() {
        *requested = None;
    }
    if let Ok(mut staged) = engine().staged.lock() {
        *staged = None;
    }
    if let Ok(mut current) = engine().snapshot.lock() {
        current.package_verified = false;
        current.release = None;
        current.peer_candidate = true;
        current.received_bytes = 0;
        current.expected_bytes = None;
    }
    state(
        "downloading",
        "Receiving peer package; no restart has been requested",
    );
    if let Ok(mut last) = engine().last_check.lock() {
        *last = Some(Instant::now());
    }
    let ctx = ctx.clone();
    std::thread::spawn(move || {
        let result = crate::peer_package::download(&peer, |done, total| {
            guard(&settings, epoch)?;
            if let Ok(mut current) = engine().snapshot.lock() {
                current.received_bytes = done;
                current.expected_bytes = Some(total);
            }
            ctx.request_repaint();
            Ok(())
        });
        if guard(&settings, epoch).is_ok() {
            match result {
                Ok((release, _path)) => {
                    if let Ok(mut staged) = engine().staged.lock() {
                        *staged = Some(release.sha256.clone());
                    }
                    if let Ok(mut current) = engine().snapshot.lock() {
                        current.release = Some(release);
                        current.package_verified = true;
                        current.peer_candidate = true;
                    }
                    state(
                        "staged",
                        "Peer package verified; Apply and restart remains explicit",
                    );
                }
                Err(error) => state("blocked", &error),
            }
        }
        engine().busy.store(false, Ordering::Release);
        ctx.request_repaint();
    });
}
pub fn tick(settings: &Settings, ctx: &eframe::egui::Context, can_apply: bool) -> bool {
    let e = engine();
    if !e.health_attempted.swap(true, Ordering::AcqRel) {
        if let Err(error) = crate::client_install::acknowledge_health() {
            state("health-unverified", &error);
        }
        if let Ok(mut value) = e.snapshot.lock() {
            value.managed_installation = crate::client_install::managed().is_ok();
        }
    }
    let epoch = bind(settings);
    if e.credential_epoch.swap(epoch, Ordering::AcqRel) != epoch {
        if let Ok(Some(enrollment)) = enrollment(settings) {
            if let Ok(mut value) = e.snapshot.lock() {
                value.enrolled = true;
                value.client_id = Some(enrollment.client_id);
            }
        }
    }
    if let Ok(mut ready) = e.ready_quit.lock() {
        if let Some(at) = *ready {
            if at.elapsed() > Duration::from_secs(30) {
                *ready = None;
                state(
                    "blocked",
                    "Graceful restart deadline expired; helper will not force termination",
                );
            } else if can_apply {
                *ready = None;
                return true;
            }
        }
    }
    if e.busy.load(Ordering::Acquire) {
        return false;
    }
    let current = snapshot();
    if e.staged
        .lock()
        .is_ok_and(|sha| sha.as_ref() == current.release.as_ref().map(|r| &r.sha256))
        && current.release.is_some()
        && !current.applying
        && current.state != "apply-blocked"
        && ((settings.auto_apply && !current.peer_candidate)
            || e.apply_requested
                .lock()
                .is_ok_and(|sha| sha.as_ref() == current.release.as_ref().map(|r| &r.sha256)))
        && can_apply
    {
        if let Some(release) = current.release {
            if e.busy.swap(true, Ordering::AcqRel) {
                return false;
            }
            let ctx = ctx.clone();
            let settings = settings.clone();
            std::thread::spawn(move || {
                let result = guard(&settings, epoch)
                    .and_then(|_| candidate(&release))
                    .and_then(|path| crate::client_install::prepare(&path, &release.sha256));
                match result {
                    Ok(()) => {
                        if let Ok(policy) = engine().settings.lock() {
                            if policy.as_ref() == Some(&settings)
                                && engine().epoch.load(Ordering::Acquire) == epoch
                            {
                                if let Ok(mut value) = engine().snapshot.lock() {
                                    value.applying = true;
                                    value.state = "graceful-restart".into();
                                }
                                if let Ok(mut ready) = engine().ready_quit.lock() {
                                    *ready = Some(Instant::now());
                                }
                            }
                        }
                    }
                    Err(error) => state("apply-blocked", &error),
                }
                engine().busy.store(false, Ordering::Release);
                ctx.request_repaint();
            });
            return false;
        }
    }
    if (settings.telemetry || settings.remote_commands)
        && e.last_report
            .lock()
            .is_ok_and(|last| last.is_none_or(|at| at.elapsed() > Duration::from_secs(30)))
    {
        if let Ok(mut last) = e.last_report.lock() {
            *last = Some(Instant::now());
        }
        request(Action::Heartbeat, settings.clone(), ctx);
    } else if settings.auto_check
        && e.last_check
            .lock()
            .is_ok_and(|last| last.is_none_or(|at| at.elapsed() > Duration::from_secs(1800)))
    {
        if let Ok(mut last) = e.last_check.lock() {
            *last = Some(Instant::now());
        }
        request(Action::Check, settings.clone(), ctx);
    }
    ctx.request_repaint_after(Duration::from_secs(1));
    false
}
/// Enrollment material is accepted only on stdin, never command arguments or
/// saved preferences. Configuration requires exclusive GUI ownership so an
/// open Preferences draft cannot be silently overwritten by another process.
pub fn cli(arguments: &[String]) -> Option<Result<(), String>> {
    let option = arguments.get(1)?.as_str();
    if !matches!(option, "--client-enroll" | "--client-management-config") {
        return None;
    }
    Some((|| {
        let mut config = crate::config::AppConfig::load()?;
        if option == "--client-enroll" {
            if arguments.len() != 2 {
                return Err(
                    "Enrollment takes no command-line token; pipe the token on stdin".into(),
                );
            }
            let mut token = String::new();
            std::io::stdin()
                .take(514)
                .read_to_string(&mut token)
                .map_err(|_| "Could not read enrollment token from stdin")?;
            let token = token.trim_end_matches(['\r', '\n']).to_owned();
            let epoch = bind(&config.client_management);
            return enroll(&config.client_management, epoch, token);
        }
        if arguments.len() < 3 || (arguments.len() - 3) % 2 != 0 {
            return Err("Use --client-management-config HTTPS_BASE [--telemetry on|off] [--remote-commands on|off] [--remote-cleanup on|off]".into());
        }
        let _ownership=match crate::platform::instance::acquire_gui_ownership()? {
            crate::platform::instance::GuiOwnership::Primary(owner)=>owner,
            crate::platform::instance::GuiOwnership::Existing=>return Err("Close the app through its guarded graceful lifecycle before CLI preference changes".into()),
        };
        config.client_management.base = arguments[2].clone();
        for pair in arguments[3..].chunks_exact(2) {
            let value = match pair[1].as_str() {
                "on" => true,
                "off" => false,
                _ => return Err("Management flags require on or off".into()),
            };
            match pair[0].as_str() {
                "--telemetry" => config.client_management.telemetry = value,
                "--remote-commands" => config.client_management.remote_commands = value,
                "--remote-cleanup" => config.client_management.allow_remote_cleanup = value,
                _ => return Err("Unknown app management setting".into()),
            }
        }
        config.client_management.validate()?;
        config.save()
    })())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn release() -> Release {
        Release {
            product: "PakarController".into(),
            platform: PLATFORM.into(),
            version: "0.2.7".into(),
            build: 10,
            bytes: 1024,
            sha256: "a".repeat(64),
            url: format!("{DEFAULT_BASE}/packages/app.exe"),
        }
    }
    #[test]
    fn release_policy_checks_numeric_build_semver_platform_origin_and_integrity() {
        let settings = Settings::default();
        let mut r = release();
        assert!(newer(&r, "0.2.7", 9, &settings));
        r.build = 9;
        assert!(!newer(&r, "0.2.7", 9, &settings));
        r.build = 10;
        r.version = "0.2.6".into();
        assert!(!newer(&r, "0.2.7", 9, &settings));
        r.version = "0.2.8".into();
        assert!(newer(&r, "0.2.7", 9, &settings));
        for value in ["0.2.8-beta", "0.02.8", "0.2", "0.2.8.1"] {
            r.version = value.into();
            assert!(!newer(&r, "0.2.7", 9, &settings));
        }
        r = release();
        for url in [
            "http://office.digitalogic.ir/app.exe",
            "https://other.invalid/app.exe",
            "https://user@office.digitalogic.ir/app.exe",
            "https://office.digitalogic.ir/app.exe?token=secret",
        ] {
            r.url = url.into();
            assert!(!newer(&r, "0.2.7", 9, &settings));
        }
        r = release();
        r.platform = "android".into();
        assert!(!newer(&r, "0.2.7", 9, &settings));
        r = release();
        r.bytes = MAX_PACKAGE + 1;
        assert!(!newer(&r, "0.2.7", 9, &settings));
        r = release();
        r.sha256 = "not-a-hash".into();
        assert!(!newer(&r, "0.2.7", 9, &settings));
    }
    #[test]
    fn exported_settings_and_snapshot_never_contain_enrollment_credentials() {
        let settings = Settings::default();
        assert!(
            !settings.telemetry && !settings.remote_commands && !settings.allow_remote_uninstall
        );
        let value = serde_json::to_value(&settings).unwrap();
        assert!(value.get("token").is_none());
        assert!(value.get("enrollment_token").is_none());
        let status = serde_json::to_value(Snapshot {
            enrolled: true,
            client_id: Some("client-id".into()),
            ..Default::default()
        })
        .unwrap();
        assert!(status.get("token").is_none());
        assert!(status.get("enrollment_token").is_none());
    }
    #[test]
    fn server_settings_reject_credentials_queries_fragments_and_non_https() {
        for value in [
            "http://office.digitalogic.ir/pakar",
            "https://u:p@office.digitalogic.ir/pakar",
            "https://office.digitalogic.ir/pakar?x=1",
            "https://office.digitalogic.ir/pakar#secret",
        ] {
            assert!(
                Settings {
                    base: value.into(),
                    ..Default::default()
                }
                .validate()
                .is_err()
            );
        }
        assert!(Settings::default().validate().is_ok());
    }
    #[test]
    fn remote_command_allowlist_is_expiring_and_cannot_supply_paths_or_shell_actions() {
        let mut command = RemoteCommand {
            id: "a".repeat(32),
            action: "cleanup".into(),
            expires_at_ms: 101,
        };
        assert!(command_admissible(&command, 100));
        assert!(!command_admissible(&command, 101));
        command.expires_at_ms = 60_101;
        assert!(!command_admissible(&command, 100));
        command.expires_at_ms = 101;
        for action in ["execute", "delete", "install", "reboot", "../cleanup"] {
            command.action = action.into();
            assert!(!command_admissible(&command, 100));
        }
        command.action = "check-update".into();
        for id in ["", "../current.exe", "x?token=secret", "x/claim"] {
            command.id = id.into();
            assert!(!command_admissible(&command, 100));
        }
    }
}
