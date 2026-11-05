// Actual Pealayer src/main.rs entrypoint policy: Explorer launches are GUI-only.
#![cfg_attr(windows, windows_subsystem = "windows")]

use std::{
    io::{ErrorKind, Read, Write},
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    thread,
    time::{Duration, Instant},
};

mod access;
mod application_identity;
mod application_shortcuts;
mod audio;
mod board_silent;
mod brand_image;
mod branding;
mod bridge;
mod capabilities;
mod cli_output;
mod client_install;
mod client_management;
mod client_secrets;
mod client_telemetry;
mod panel_event;
mod peripheral_events;
mod status_rgb;
mod winner_notice;
#[macro_use]
mod cli_console;
mod cell_assignments;
mod cell_batch;
mod config;
mod configuration_reads;
mod connection;
mod device_settings;
mod diagnostics;
mod display;
mod firmware_feed;
mod firmware_progress;
mod firmware_source;
mod identity;
mod maintenance;
mod led_backend;
mod messaging;
mod native_workspace;
mod pc_contract;
mod protocol_opcodes;
mod peer_http;
mod peer_identity;
mod pixel_diagnostic;
mod pixel_corrections;
mod pixel_groups;
mod usb_api;
mod platform;
mod realtime;
mod game_presentation;
mod minigame;
mod state_appearance;
mod streaming;
mod terminal_ui;
mod ui;
mod usb_demand;
mod usb_table;
mod workspace_layout;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use connection::Method as Transport;
use protocol_opcodes::{pakar as wire, controller as pc_wire};
use eframe::egui;
use reqwest::blocking::Client;
use serde::Deserialize;
use tungstenite::{Message, client::IntoClientRequest};

const OP_GET_STATUS: u8 = wire::GET_STATUS;
const OP_BUZZER: u8 = wire::BUZZER;
const OP_AUDIO: u8 = wire::AUDIO_CUE;
const OP_DISPLAY: u8 = wire::DISPLAY_TEXT;
const OP_SET_MODE: u8 = wire::SET_MODE;
const OP_CELL_ACTION: u8 = wire::CELL_ACTION;
const OP_RESET: u8 = wire::GAME_RESET;
const OP_OUTPUT_MASK: u8 = wire::OUTPUT_MASK;
const OP_BRIGHTNESS: u8 = wire::SET_BRIGHTNESS;
const OP_PATTERN: u8 = wire::SET_LED_PATTERN;
const OP_LED_OUTPUT: u8 = wire::SET_LED_OUTPUT;
const OP_NETWORK_CONFIG: u8 = wire::NETWORK_CONFIG;
const OP_STATUS: u8 = wire::STATUS_RESPONSE;

#[derive(Clone, PartialEq)]
struct Endpoint {
    base: String,
    user: String,
    password: String,
}

#[derive(Debug, Clone, Default, Deserialize, serde::Serialize)]
struct DeviceStatus {
    #[serde(default)]
    bridge: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "ui::status_bar::optional_status_rgb")]
    status_rgb: Option<ui::status_bar::PhysicalStatusRgb>,
    #[serde(default)]
    board_id: String,
    #[serde(default)]
    boot_id: String,
    #[serde(default)]
    http_peer_fencing: bool,
    #[serde(default)]
    command_clock_ms: Option<u64>,
    #[serde(default)]
    pixel_corrections_revision: Option<u32>,
    #[serde(default)]
    pixel_groups: serde_json::Value,
    #[serde(default)]
    capabilities: Option<capabilities::Manifest>,
    #[serde(default)]
    version: String,
    #[serde(default)]
    compatibility: Option<peer_identity::Policy>,
    #[serde(default)]
    avr_version: Option<String>,
    #[serde(default)]
    audio_cues: Vec<String>,
    #[serde(default)]
    hostname: String,
    #[serde(default)]
    ip: String,
    #[serde(default)]
    uplink: String,
    #[serde(default)]
    uplink_ipv4_ready: Option<bool>,
    #[serde(default)]
    ethernet: serde_json::Value,
    #[serde(default)]
    mode: String,
    #[serde(default)]
    avr_online: bool,
    #[serde(default)]
    ap_started: bool,
    #[serde(default)]
    ap_enabled: bool,
    #[serde(default)]
    ap_ip: String,
    #[serde(default)]
    station_enabled: bool,
    #[serde(default)]
    station_connected: bool,
    #[serde(default)]
    station_ssid: String,
    #[serde(default)]
    station_ip: String,
    #[serde(default)]
    station_rssi: i16,
    #[serde(default)]
    mdns_started: bool,
    #[serde(default)]
    ssdp_started: bool,
    #[serde(default)]
    netbios_started: bool,
    #[serde(default)]
    canonical_url: String,
    #[serde(default)]
    tm1637_ack: bool,
    #[serde(default, deserialize_with = "display::optional_lcd")]
    lcd: Option<display::LcdObservation>,
    #[serde(default)]
    tm1637_segments: Vec<u8>,
    #[serde(default)]
    tm1637_brightness: u8,
    #[serde(default)]
    tm1637_host_active: bool,
    #[serde(default)]
    mirroring: serde_json::Value,
    #[serde(default)]
    programming_latch: bool,
    #[serde(default)]
    winner: u8,
    #[serde(default)]
    key_game: serde_json::Value,
    #[serde(default)]
    minigame: serde_json::Value,
    #[serde(default)]
    key_mask_applied: Option<bool>,
    #[serde(default)]
    game_audio_applied: bool,
    #[serde(default)]
    selected_cell: u8,
    #[serde(default)]
    brightness: u8,
    #[serde(default)]
    led_output: serde_json::Value,
    #[serde(default)]
    update: serde_json::Value,
    #[serde(default)]
    ota_capacity_bytes: serde_json::Value,
    #[serde(default, deserialize_with = "firmware_progress::optional_upgrade")]
    upgrade: Option<firmware_progress::Upgrade>,
    #[serde(default)]
    remote_firmware: serde_json::Value,
    #[serde(default)]
    avr_maintenance: serde_json::Value,
    #[serde(default)]
    effect: String,
    #[serde(default)]
    pixels: u16,
    #[serde(default)]
    active_inputs: u8,
    #[serde(default)]
    input_9: bool,
    #[serde(default)]
    active_outputs: u8,
    #[serde(default)]
    esp_uart_errors: u16,
    #[serde(default)]
    avr_uart_errors: u16,
    #[serde(default)]
    usb_serial_errors: u16,
    #[serde(default)]
    ntp_synced: bool,
    #[serde(default)]
    utc_epoch: u64,
    #[serde(default)]
    websocket_clients: u8,
    #[serde(default)]
    mqtt_enabled: bool,
    #[serde(default)]
    mqtt_connected: bool,
    #[serde(default)]
    mqtt_state: i16,
    #[serde(default)]
    cloud_websocket_enabled: bool,
    #[serde(default)]
    cloud_websocket_connected: bool,
    #[serde(default)]
    cloud_error: String,
    #[serde(default)]
    link_mode: String,
    #[serde(default)]
    cells: Vec<String>,
    #[serde(default)]
    state_presentation: serde_json::Value,
    #[serde(default)]
    color_preview: serde_json::Value,
    #[serde(default)]
    red_lock: Option<bool>,
    #[serde(default)]
    pixel_preview_requested: bool,
    #[serde(default)]
    pixel_frame: Option<ui::cell_pixels::Frame>,
    #[serde(default, skip_serializing)]
    pixel_frame_partial: serde_json::Value,
}

fn led_output_mode(status: &DeviceStatus) -> Option<&str> {
    status
        .led_output
        .get("mode")
        .and_then(|v| v.as_str())
        .filter(|mode| matches!(*mode, "static" | "effects"))
}

#[derive(Clone)]
enum HttpCommand {
    Scoped(peer_http::Scope, Box<HttpCommand>),
    Status(Endpoint),
    Mapping(Endpoint, u64),
    Configuration(Endpoint, &'static str),
    CorrectionPreview(Endpoint, pixel_corrections::PreviewRequest),
    BoardSilent(Endpoint, u64, board_silent::Change),
    Post(Endpoint, &'static str, Vec<(String, String)>),
    CellBatch(Endpoint, cell_batch::Correlation),
    AccessChange(Endpoint, Endpoint, bool),
    NameChange(Endpoint, String, Option<String>),
    Settings(Endpoint, device_settings::Write),
    Maintenance(Endpoint, u64, maintenance::Operation),
    PixelReadback(Endpoint, u64, u16, peer_http::Pin),
}

impl HttpCommand {
    // Adapted from PCController Session.requestAdmission.grantNextLocked:
    // priority only reorders unsent work, never preempts/replays a wire request.
    fn queue_priority(&self) -> u8 {
        match self {
            Self::Scoped(_, inner) => inner.queue_priority(),
            Self::Configuration(..) => 0,
            Self::Status(_) | Self::Mapping(..) => 1,
            _ => 2,
        }
    }
    fn is_status(&self) -> bool {
        match self {
            Self::Scoped(_, command) => command.is_status(),
            Self::Status(_) => true,
            _ => false,
        }
    }
    fn discoverable_read(&self) -> bool {
        match self {
            Self::Scoped(_, command) => command.discoverable_read(),
            Self::Status(_) | Self::Mapping(..) | Self::Configuration(..) => true,
            _ => false,
        }
    }
    fn failure(&self, error: String) -> Event {
        match self {
            Self::Scoped(scope, command) => {
                Event::ForScope(scope.clone(), Box::new(command.failure(error)))
            }
            Self::BoardSilent(_, id, _) => Event::BoardSilentFinished(*id, Err(error)),
            Self::Settings(_, write) => Event::SettingsFinished(write.id, write.group, Err(error)),
            Self::Maintenance(_, id, _) => Event::MaintenanceFinished(*id, Err(error)),
            Self::PixelReadback(_, id, ..) => Event::PixelReadbackFinished(*id, Err(error)),
            Self::Mapping(_, id) => Event::MappingFailed(*id, error),
            Self::CorrectionPreview(_, request) => Event::CorrectionPreviewFinished(request.id, Err(error)),
            _ if self.diagnostic_source().is_some() => {
                Event::DiagnosticFailure(self.diagnostic_source().unwrap(), error)
            }
            _ => Event::OperationFailed(error),
        }
    }
    fn diagnostic_source(&self) -> Option<String> {
        match self {
            Self::Scoped(_, command) => command.diagnostic_source(),
            Self::Status(_) => Some("HTTP GET /api/status".into()),
            Self::Mapping(..) => Some("HTTP GET /api/cells/mapping".into()),
            Self::Configuration(_, path) => Some(format!("HTTP GET {path}")),
            _ => None, // Explicit operations keep their own correlated outcomes.
        }
    }
    fn endpoint(&self) -> &Endpoint {
        match self {
            Self::Scoped(_, command) => command.endpoint(),
            Self::Status(endpoint)
            | Self::Mapping(endpoint, ..)
            | Self::BoardSilent(endpoint, ..)
            | Self::Configuration(endpoint, ..)
            | Self::CorrectionPreview(endpoint, ..)
            | Self::AccessChange(endpoint, ..)
            | Self::NameChange(endpoint, ..)
            | Self::Settings(endpoint, ..)
            | Self::Maintenance(endpoint, ..)
            | Self::PixelReadback(endpoint, ..)
            | Self::Post(endpoint, ..) => endpoint,
            Self::CellBatch(endpoint, ..) => endpoint,
        }
    }

    fn with_base(&self, base: String) -> Self {
        let replace = |endpoint: &Endpoint| Endpoint {
            base: base.clone(),
            user: endpoint.user.clone(),
            password: endpoint.password.clone(),
        };
        match self {
            Self::Scoped(scope, command) => {
                Self::Scoped(scope.clone(), Box::new(command.with_base(base.clone())))
            }
            Self::Status(endpoint) => Self::Status(replace(endpoint)),
            Self::Mapping(endpoint, id) => Self::Mapping(replace(endpoint), *id),
            Self::Configuration(endpoint, path) => Self::Configuration(replace(endpoint), path),
            Self::CorrectionPreview(endpoint, request) => Self::CorrectionPreview(replace(endpoint), request.clone()),
            Self::BoardSilent(endpoint, id, desired) => {
                Self::BoardSilent(replace(endpoint), *id, *desired)
            }
            Self::Post(endpoint, path, form) => Self::Post(replace(endpoint), path, form.clone()),
            Self::CellBatch(endpoint, correlation) => {
                Self::CellBatch(replace(endpoint), correlation.clone())
            }
            Self::AccessChange(endpoint, next, enabled) => {
                Self::AccessChange(replace(endpoint), replace(next), *enabled)
            }
            Self::NameChange(endpoint, name, label) => {
                Self::NameChange(replace(endpoint), name.clone(), label.clone())
            }
            Self::Settings(endpoint, write) => Self::Settings(replace(endpoint), write.clone()),
            Self::Maintenance(endpoint, id, operation) => {
                Self::Maintenance(replace(endpoint), *id, operation.clone())
            }
            Self::PixelReadback(endpoint, id, index, peer) => {
                Self::PixelReadback(replace(endpoint), *id, *index, peer.clone())
            }
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct CellMapping {
    logical_id: u8,
    physical_cell: u8,
    name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct CellMappingResponse {
    #[serde(default)]
    pixel_groups: Option<serde_json::Value>,
    #[serde(default)]
    board_id: Option<String>,
    #[serde(default)]
    boot_id: Option<String>,
    #[serde(default)]
    physical: Vec<u8>,
    #[serde(default)]
    names: Vec<String>,
}
impl CellMappingResponse {
    fn decode(self) -> Result<Vec<CellMapping>, String> {
        if let Some(groups) = &self.pixel_groups { pixel_groups::Layout::parse(groups)?; }
        let mappings: Vec<_> = self
            .physical
            .into_iter()
            .enumerate()
            .map(|(index, physical_cell)| CellMapping {
                logical_id: u8::try_from(index + 1).unwrap_or(0),
                physical_cell,
                name: self
                    .names
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| (index + 1).to_string()),
            })
            .collect();
        if !streaming::valid_mapping(&mappings) {
            return Err("Board mapping is incomplete or invalid; previous same-peer mapping retained, nothing written".into());
        }
        Ok(mappings)
    }
}
struct MappingPending {
    id: u64,
    peer: peer_http::Pin,
    started: Instant,
}
fn status_observation_fresh(last: Option<Instant>) -> bool {
    last.is_some_and(|at| at.elapsed() < Duration::from_secs(8))
}
fn websocket_status_ready(status: &DeviceStatus) -> bool {
    !status.hostname.is_empty()
        && status
            .capabilities
            .as_ref()
            .is_some_and(|manifest| manifest.known())
}
fn websocket_status(text: &str) -> Option<DeviceStatus> {
    let status = serde_json::from_str::<DeviceStatus>(text).ok()?;
    websocket_status_ready(&status).then_some(status)
}
enum WsCommand {
    Control(realtime::Intent),
    Connect(Endpoint, u64),
    Disconnect,
    Rate(u16),
    Mapping(Vec<CellMapping>),
    Stream(bool),
    Sample(bool),
    PixelSubscribe(bool),
    Pixel(pixel_diagnostic::Request),
    ArmPixel(pixel_diagnostic::Request),
}
enum SerialCommand {
    Api(String, usb_api::Work),
    Demand(String, u64, usb_demand::Demand),
    Bridge(String, usb_table::Admission, u8, Vec<u8>, bridge::Reply),
    Port(String, u64),
    Send(String, usb_table::Admission, u8, Vec<u8>),
    Identity(String, usb_table::Admission, String, String),
    BoardSilent(String, usb_table::Admission, u64, board_silent::Change),
}
enum Event {
    ForConnection(connection::Stamp, Box<Event>),
    ForScope(peer_http::Scope, Box<Event>),
    ForEndpoint(Endpoint, Box<Event>),
    StreamStats(f64),
    Status(DeviceStatus),
    PixelSample(peer_http::Pin, ui::cell_pixels::Frame),
    UpgradeProgress(firmware_progress::Upgrade),
    StatusRgb(status_rgb::Observation),
    PanelEvent(panel_event::Observation),
    SerialPanelEvent(String, u64, panel_event::Observation),
    PeripheralEvent(peripheral_events::Observation),
    SerialSegments(String, u64, peripheral_events::Segments),
    Mapping(u64, Vec<CellMapping>, Option<(String, String)>),
    MappingFailed(u64, String),
    Configuration(&'static str, serde_json::Value),
    CorrectionPreviewFinished(u64, Result<pixel_corrections::Configuration, String>),
    Accepted(String),
    BoardSilentFinished(u64, Result<pc_contract::Settings, String>),
    CredentialsUpdated(Endpoint),
    NameUpdated(Endpoint, bool),
    DiscoveredBoard(Endpoint, DeviceStatus),
    ConfigurationChanged(String, String),
    SerialIdentity(String, serde_json::Value),
    SerialIdentitySaved(String, Result<(), String>),
    SerialSnapshot(String, usb_table::Snapshot),
    SerialOperation(String, Result<String, String>),
    SerialTiming(String, usb_table::Timing),
    SerialCapture(String, usb_table::CaptureProgress),
    SerialCommandState(String, DeviceStatus),
    CellBatchConfirmed(cell_batch::Correlation),
    SerialBatchConfirmed(String, cell_batch::Correlation),
    SerialQueueDepth(String, usize),
    SerialOwner(String, u64),
    DiagnosticFailure(String, String),
    DiagnosticRecovered(String),
    SerialFailure(String, String),
    OperationFailed(String),
    SettingsFinished(
        u64,
        device_settings::Group,
        Result<serde_json::Value, String>,
    ),
    MaintenanceFinished(u64, Result<String, String>),
    PixelModeFinished(u64, Result<DeviceStatus, String>),
    PixelSent(pixel_diagnostic::Request, Result<(), String>),
    PixelReadbackFinished(u64, Result<String, String>),
    PixelCommandFinished(u64, Result<DeviceStatus, String>),
    WebSocket(bool),
    Serial(String, bool, Option<DeviceStatus>),
}

impl Event {
    fn http_status(&self) -> Option<&DeviceStatus> {
        match self {
            Self::ForScope(_, inner) => inner.http_status(),
            Self::Status(status) => Some(status),
            _ => None,
        }
    }
    fn http_succeeded(&self) -> bool {
        match self {
            Self::ForScope(_, inner) => inner.http_succeeded(),
            Self::MappingFailed(..)
            | Self::DiagnosticFailure(..)
            | Self::OperationFailed(_)
            | Self::BoardSilentFinished(_, Err(_))
            | Self::SettingsFinished(_, _, Err(_))
            | Self::MaintenanceFinished(_, Err(_))
            | Self::PixelModeFinished(_, Err(_))
            | Self::PixelReadbackFinished(_, Err(_))
            | Self::CorrectionPreviewFinished(_, Err(_))
            | Self::PixelCommandFinished(_, Err(_)) => false,
            _ => true,
        }
    }
}

fn for_endpoint(endpoint: &Endpoint, event: Event) -> Event {
    Event::ForEndpoint(endpoint.clone(), Box::new(event))
}

struct SessionEvents {
    sender: Sender<Event>,
    stamp: connection::Stamp,
}
impl SessionEvents {
    fn send(&self, event: Event) -> Result<(), mpsc::SendError<Event>> {
        self.sender
            .send(Event::ForConnection(self.stamp.clone(), Box::new(event)))
    }
}

fn current_event(event: Event, endpoint: &Endpoint, serial_port: &str) -> Option<Event> {
    match event {
        Event::ForEndpoint(origin, inner) if origin == *endpoint => Some(*inner),
        Event::ForEndpoint(..) => None,
        Event::Serial(ref port, ..) if port != serial_port => None,
        Event::SerialFailure(ref port, ..) if port != serial_port => None,
        Event::SerialPanelEvent(ref port, ..) if port != serial_port => None,
        Event::SerialSegments(ref port, ..) if port != serial_port => None,
        Event::SerialSnapshot(ref port, ..)
        | Event::SerialOperation(ref port, ..)
        | Event::SerialTiming(ref port, ..)
        | Event::SerialCapture(ref port, ..)
        | Event::SerialCommandState(ref port, ..)
        | Event::SerialBatchConfirmed(ref port, ..)
        | Event::SerialQueueDepth(ref port, ..)
        | Event::SerialOwner(ref port, ..)
            if port != serial_port =>
        {
            None
        }
        other => Some(other),
    }
}

fn http_request(client: &Client, command: HttpCommand) -> Result<Event, String> {
    if let HttpCommand::Scoped(scope, command) = command {
        return scoped_http_request(client, &scope, &command)
            .map(|result| Event::ForScope(scope, Box::new(result)));
    }
    if matches!(command, HttpCommand::CorrectionPreview(..)) {
        return Err("Correction preview requires a scoped board owner".into());
    }
    let mapping_id = match &command {
        HttpCommand::Mapping(_, id) => *id,
        _ => 0,
    };
    if let HttpCommand::PixelReadback(endpoint, id, index, peer) = &command {
        return Ok(Event::PixelReadbackFinished(
            *id,
            pixel_diagnostic::verify_frame(client, endpoint, peer, *index),
        ));
    }
    if let HttpCommand::Maintenance(endpoint, id, operation) = &command {
        return Ok(Event::MaintenanceFinished(
            *id,
            maintenance::run(client, endpoint, operation.clone()),
        ));
    }
    if let HttpCommand::Settings(endpoint, write) = &command {
        // Exactly one mutation. Neither discovery nor readback failure replays it.
        let result = (|| -> Result<serde_json::Value, String> {
            let response = client.post(format!("{}{}", endpoint.base.trim_end_matches('/'), write.group.path()))
                .basic_auth(&endpoint.user, Some(&endpoint.password)).form(&write.form).send()
                .map_err(|_| "Configuration write outcome is unknown. Refresh before retrying; no write was replayed.".to_string())?
                .error_for_status().map_err(|e| format!("Configuration rejected: {e}"))?;
            let accepted = response.json::<serde_json::Value>().map_err(|_| {
                "Write response could not be validated; refresh before retrying".to_string()
            })?;
            write.verify_ack(&accepted)?;
            let readback = client.get(format!("{}{}", endpoint.base.trim_end_matches('/'), write.group.path()))
                .basic_auth(&endpoint.user, Some(&endpoint.password)).send()
                .map_err(|_| "Write accepted; readback unavailable, possibly after Wi-Fi reassociation. Reconnect and refresh; no write was replayed.".to_string())?
                .error_for_status().map_err(|e| format!("Write accepted; readback failed: {e}"))?
                .json::<serde_json::Value>().map_err(|_| "Write accepted; malformed readback. Refresh before retrying.".to_string())?;
            write.verify_readback(&readback)?;
            Ok(readback)
        })();
        return Ok(Event::SettingsFinished(write.id, write.group, result));
    }
    if let HttpCommand::NameChange(endpoint, input, label) = &command {
        let name = identity::normalize_name(input)?;
        if label
            .as_ref()
            .is_some_and(|s| !identity::valid_display_name(s))
        {
            return Err("Invalid board display name".into());
        }
        let mut form = vec![("hostname", name.as_str())];
        if let Some(label) = label {
            form.push(("display_name", label.as_str()));
        }
        let response = client
            .post(format!(
                "{}/api/device/name",
                endpoint.base.trim_end_matches('/')
            ))
            .basic_auth(&endpoint.user, Some(&endpoint.password))
            .form(&form)
            .send()
            .map_err(diagnostics::describe_error)?;
        if !response.status().is_success() {
            return Err(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().unwrap_or_default()
            ));
        }
        let value: serde_json::Value = response.json().map_err(diagnostics::describe_error)?;
        if value.get("accepted").and_then(|v| v.as_bool()) != Some(true)
            || value.get("persisted").and_then(|v| v.as_bool()) != Some(true)
            || value.get("hostname").and_then(|v| v.as_str()) != Some(name.as_str())
            || label
                .as_ref()
                .is_some_and(|s| value["display_name"].as_str() != Some(s.as_str()))
        {
            return Err(
                "Device did not confirm the saved name; no retry or endpoint switch was attempted"
                    .into(),
            );
        }
        let mut next = endpoint.clone();
        next.base = format!("http://{name}.local");
        let pending = value["apply_pending"].as_bool().unwrap_or(true);
        if !pending {
            let readback: serde_json::Value = client
                .get(format!(
                    "{}/api/device/name",
                    endpoint.base.trim_end_matches('/')
                ))
                .basic_auth(&endpoint.user, Some(&endpoint.password))
                .send()
                .map_err(|_| "Identity saved but readback unavailable; refresh before retrying")?
                .error_for_status()
                .map_err(|e| e.to_string())?
                .json()
                .map_err(|e| e.to_string())?;
            if readback["hostname"] != value["hostname"]
                || readback["display_name"] != value["display_name"]
            {
                return Err("Identity saved but readback mismatch; no retry attempted".into());
            }
        }
        return Ok(Event::NameUpdated(next, pending));
    }
    if let HttpCommand::AccessChange(endpoint, next, enabled) = &command {
        let response = client
            .post(format!("{}/api/auth", endpoint.base.trim_end_matches('/')))
            .basic_auth(&endpoint.user, Some(&endpoint.password))
            .form(&[
                ("enabled", &enabled.to_string()),
                ("current_password", &endpoint.password),
                ("username", &next.user),
                ("password", &next.password),
            ])
            .send()
            .map_err(diagnostics::describe_error)?;
        if !response.status().is_success() {
            return Err(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().unwrap_or_default()
            ));
        }
        let value: serde_json::Value = response.json().map_err(diagnostics::describe_error)?;
        if value.get("updated").and_then(|v| v.as_bool()) != Some(true) {
            return Err("Device did not confirm a saved credential change".into());
        }
        if value.get("enabled").and_then(|v| v.as_bool()) != Some(*enabled) {
            return Err("Device did not confirm the authentication setting".into());
        }
        let mut active = next.clone();
        if !enabled {
            active.user.clear();
            active.password.clear();
        }
        return Ok(Event::CredentialsUpdated(active));
    }
    if let HttpCommand::Configuration(endpoint, path) = &command {
        let response = client
            .get(format!("{}{}", endpoint.base.trim_end_matches('/'), path))
            .basic_auth(&endpoint.user, Some(&endpoint.password))
            .send()
            .map_err(diagnostics::describe_error)?
            .error_for_status()
            .map_err(diagnostics::describe_error)?;
        return response
            .json::<serde_json::Value>()
            .map(|value| Event::Configuration(path, value))
            .map_err(diagnostics::describe_error);
    }
    if let HttpCommand::Post(endpoint, path, form) = &command
        && matches!(*path, pixel_corrections::PATH | pixel_groups::PATH)
    {
        let request = client.post(format!("{}{}",endpoint.base.trim_end_matches('/'),path))
            .basic_auth(&endpoint.user,Some(&endpoint.password));
        let request = if *path == pixel_groups::PATH {
            let json: serde_json::Value = serde_json::from_str(form.iter().find(|(key,_)| key == "configuration")
                .map(|(_,v)| v.as_str()).ok_or("Missing physical group configuration")?).map_err(|_| "Invalid group configuration")?;
            request.json(&json)
        } else { request.form(form) };
        request.send()
            .map_err(diagnostics::describe_error)?.error_for_status()
            .map_err(diagnostics::describe_error)?;
        return client.get(format!("{}{}",endpoint.base.trim_end_matches('/'),path))
            .basic_auth(&endpoint.user,Some(&endpoint.password)).send()
            .map_err(diagnostics::describe_error)?.error_for_status()
            .map_err(diagnostics::describe_error)?.json::<serde_json::Value>()
            .map(|value|Event::Configuration(path,value)).map_err(diagnostics::describe_error);
    }
    let (endpoint, request, response_kind) = match command {
        HttpCommand::Scoped(..) => unreachable!(),
        HttpCommand::Status(endpoint) => {
            let url = format!("{}/api/status", endpoint.base.trim_end_matches('/'));
            let request = client.get(url).query(&[(
                "pixels",
                if config::live().cell_shape != ui::cell_pixels::Shape::Rounded {
                    "1"
                } else {
                    "0"
                },
            )]);
            (endpoint, request, 1_u8)
        }
        HttpCommand::Mapping(endpoint, _) => {
            let url = format!("{}/api/cells/mapping", endpoint.base.trim_end_matches('/'));
            let request = client.get(url);
            (endpoint, request, 2_u8)
        }
        HttpCommand::Configuration(..)
        | HttpCommand::CorrectionPreview(..)
        | HttpCommand::AccessChange(..)
        | HttpCommand::BoardSilent(..)
        | HttpCommand::NameChange(..) => unreachable!(),
        HttpCommand::Settings(..) => unreachable!(),
        HttpCommand::Maintenance(..) => unreachable!(),
        HttpCommand::PixelReadback(..) => unreachable!(),
        HttpCommand::CellBatch(..) => unreachable!("Cell batch requires scoped admission"),
        HttpCommand::Post(endpoint, path, form) => {
            let url = format!("{}{}", endpoint.base.trim_end_matches('/'), path);
            let request = client.post(url).form(&form);
            (endpoint, request, 0_u8)
        }
    };
    let response = request
        .basic_auth(endpoint.user, Some(endpoint.password))
        .send()
        .map_err(|error| {
            if error.is_connect() {
                peer_http::request_relookup(&endpoint.base);
            }
            diagnostics::describe_error(error)
        })?;
    let code = response.status();
    let remote = response.remote_addr();
    let response_headers = response.headers().clone();
    if !code.is_success() {
        return Err(format!(
            "HTTP {code}: {}",
            response.text().unwrap_or_default()
        ));
    }
    match response_kind {
        1 => {
            peer_identity::validate_response_headers(&response_headers)?;
            let status = response
                .json::<DeviceStatus>()
                .map_err(diagnostics::describe_error)?;
            peer_identity::validate_board(&status)?;
            peer_http::remember_address(&endpoint.base, &status, remote);
            Ok(Event::Status(status))
        }
        2 => response
            .json::<CellMappingResponse>()
            .map_err(diagnostics::describe_error)
            .and_then(|value| {
                let identity = match (&value.board_id, &value.boot_id) {
                    (None, None) => None,
                    (Some(board), Some(boot)) => Some((board.clone(), boot.clone())),
                    _ => {
                        return Err(
                            "Mapping response has incomplete board/boot identity; not applied"
                                .into(),
                        );
                    }
                };
                value
                    .decode()
                    .map(|cells| Event::Mapping(mapping_id, cells, identity))
            }),
        _ => Ok(Event::Accepted(format!(
            "Device accepted the command ({code})"
        ))),
    }
}

fn scoped_http_request(
    client: &Client,
    scope: &peer_http::Scope,
    command: &HttpCommand,
) -> Result<Event, String> {
    // Adapt donor acquireWrite's checks before AND after waiting/admission.
    let mutating = !matches!(
        command,
        HttpCommand::Status(_)
            | HttpCommand::Mapping(..)
            | HttpCommand::Configuration(..)
            | HttpCommand::Maintenance(
                _,
                _,
                maintenance::Operation::RemoteStatus | maintenance::Operation::Download { .. }
            )
            | HttpCommand::PixelReadback(..)
    );
    let admit = || {
        if mutating {
            scope.admit()
        } else {
            scope.admit_read()
        }
    };
    admit()?;
    if matches!(command, HttpCommand::Status(_)) {
        return http_request(client, command.clone());
    }
    if let HttpCommand::PixelReadback(_, _, _, peer) = command {
        if scope.pin.as_ref() != Some(peer) {
            return Err("Pixel readback peer was revoked".into());
        }
        let result = http_request(&peer.client()?, command.clone())?;
        if !scope.current() {
            return Err("Pixel readback belongs to a retired board; not applied".into());
        }
        return Ok(result);
    }
    if matches!(command, HttpCommand::Mapping(..))
        && let Some(peer) = &scope.pin
    {
        // The mapping response identifies its own producer. A fresh selected
        // WS peer must not depend on an unrelated HTTP status preflight before
        // this small read is even sent. Headers are routing fences, not proof:
        // validate the actual response identity and the still-current lease.
        let result = http_request(&peer.client()?, command.clone())?;
        scope.admit_read()?;
        if let Event::Mapping(_, _, Some((board, boot))) = &result {
            if !peer.matches_identity(board, boot) {
                return Err("Mapping belongs to a different board/boot; not applied".into());
            }
            return Ok(result);
        }
        // Older firmware has no producer envelope. Discard this unbound read
        // and perform the original status -> mapping -> status fences below;
        // a status observed only after a mapping cannot establish its owner.
    }
    if mutating && scope.pin.is_none() {
        return Err(
            "Read-only: peer has not advertised valid board/boot identity and HTTP fencing".into(),
        );
    }
    let before = blocking_status(client, command.endpoint())?;
    let observed = peer_http::Pin::from_status(&before);
    if scope.pin != observed {
        return Err("Board/boot identity changed before dispatch; nothing was sent".into());
    }
    admit()?;
    if mutating {
        let allowed = before.capabilities.as_ref().is_some_and(|c| match command {
            HttpCommand::Post(_, path, form) => c.allows(path, form),
            HttpCommand::CorrectionPreview(..) => c.has(pixel_corrections::PREVIEW_FEATURE),
            HttpCommand::CellBatch(_, correlation) => {
                c.allows("/api/cells", &correlation.request.form())
            }
            HttpCommand::Settings(_, write) => c.allows(write.group.path(), &write.form),
            HttpCommand::BoardSilent(..) => board_silent::settings_supported(&before),
            HttpCommand::NameChange(..) => c.has("identity"),
            HttpCommand::AccessChange(..) => c.has("network"),
            HttpCommand::Maintenance(_, _, operation) => c.has(match operation {
                maintenance::Operation::ReconnectCloud => "network",
                maintenance::Operation::Probe
                | maintenance::Operation::Backup
                | maintenance::Operation::StageAvr(_)
                | maintenance::Operation::ProgramAvr
                | maintenance::Operation::SourceImage { esp: false, .. } => "isp",
                _ => "ota",
            }),
            _ => false,
        });
        if !allowed {
            return Err(
                "This board no longer acknowledges the requested capability; nothing was sent"
                    .into(),
            );
        }
    }
    let pinned = scope.pin.as_ref().map(peer_http::Pin::client).transpose()?;
    if let HttpCommand::CorrectionPreview(endpoint, request) = command {
        let peer = scope.pin.as_ref().ok_or("Correction preview requires a board owner")?;
        let client = pinned.as_ref().ok_or("Correction preview requires a fenced client")?;
        let value = pixel_corrections::send_http_preview(client, endpoint, peer, request, || scope.admit())?;
        return Ok(Event::CorrectionPreviewFinished(request.id, Ok(value)));
    }
    if let HttpCommand::BoardSilent(endpoint, id, desired) = command {
        if !desired.allowed(&before) {
            return Err("Board settings are unavailable or programming safety is active".into());
        }
        let client = pinned
            .as_ref()
            .ok_or("Board settings require a verified peer")?;
        let mut transaction = board_silent::Transaction::new(*desired);
        loop {
            scope.admit()?;
            let (opcode, payload) = transaction.request();
            let receipt = pc_contract::request(
                client,
                &endpoint.base,
                &endpoint.user,
                &endpoint.password,
                opcode,
                &payload,
            )?;
            let response = receipt["response_opcode"]
                .as_u64()
                .ok_or("Missing board response opcode")?;
            let payload = pc_contract::decode_hex(
                receipt["payload_hex"]
                    .as_str()
                    .ok_or("Missing board response")?,
            )?;
            scope.admit()?;
            if let Some(settings) = transaction.accept(
                u8::try_from(response).map_err(|_| "Invalid response opcode")?,
                &payload,
            )? {
                let after = blocking_status(client, endpoint)?;
                if scope.pin != peer_http::Pin::from_status(&after) || !scope.current() {
                    return Err(
                        "Board/session changed during settings readback; result not applied".into(),
                    );
                }
                return Ok(Event::BoardSilentFinished(*id, Ok(settings)));
            }
        }
    }
    if let HttpCommand::CellBatch(endpoint, correlation) = command {
        if scope.pin.as_ref() != Some(&correlation.peer) || correlation.route != Transport::WiFi {
            return Err("Atomic batch admission does not belong to this peer/route".into());
        }
        cell_batch::send_verified(
            &before,
            endpoint,
            &correlation.request,
            Some(&correlation.peer),
            None,
            Some(&|| scope.admit()),
        )?;
        if !scope.current() {
            return Err(
                "Batch belongs to a retired board; inspect original device, no replay".into(),
            );
        }
        return Ok(Event::CellBatchConfirmed(correlation.clone()));
    }
    if let HttpCommand::Post(endpoint, "/api/cells", form) = command {
        let get = |name: &str| {
            form.iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value.as_str())
                .ok_or_else(|| format!("Missing batch {name}"))
        };
        let request = cell_batch::Request::new(
            &ui::cell_search::parse(get("ids")?)?,
            get("state")?,
            get("effect")?,
        )?;
        let receipt = cell_batch::send_verified(
            &before,
            endpoint,
            &request,
            scope.pin.as_ref(),
            None,
            Some(&|| scope.admit()),
        )?;
        if !scope.current() {
            return Err(
                "Batch belongs to a retired board; inspect original device, no replay".into(),
            );
        }
        return Ok(Event::Accepted(receipt)); // No UI intent correlation: never complete another batch.
    }
    if let HttpCommand::Post(endpoint, path, form) = command
        && matches!(*path, "/api/cells/assignments" | "/api/cells/lock")
    {
        let pinned = pinned
            .as_ref()
            .ok_or("Assignments require a verified board")?;
        scope.admit()?;
        let response = pinned
            .post(format!("{}{}", endpoint.base.trim_end_matches('/'), path))
            .basic_auth(&endpoint.user, Some(&endpoint.password))
            .form(form)
            .send()
            .map_err(|_| "Cell operation outcome unknown; nothing replayed")?;
        if !response.status().is_success() {
            let code = response.status();
            let body = response.json::<serde_json::Value>().ok();
            return Err(body
                .and_then(|v| v["error"].as_str().map(cell_assignments::rejection))
                .unwrap_or_else(|| format!("Cell operation rejected: HTTP {code}")));
        }
        let receipt: serde_json::Value = response
            .json()
            .map_err(|_| "Invalid cell operation receipt; nothing replayed")?;
        if receipt["accepted"] != true {
            return Err("Cell operation was not acknowledged".into());
        }
        scope.admit()?;
        let after = blocking_status(pinned, endpoint)?;
        if scope.pin != peer_http::Pin::from_status(&after) {
            return Err("Cell operation peer changed; nothing replayed".into());
        }
        let matches = if *path == "/api/cells/lock" {
            form.iter()
                .find(|(k, _)| k == "enabled")
                .is_some_and(|(_, v)| after.red_lock == Some(v == "true"))
        } else {
            let value = form
                .iter()
                .find(|(k, _)| k == "assignments")
                .ok_or("Missing assignments")?;
            let doc = cell_assignments::Document::new(
                serde_json::from_str(&value.1).map_err(|_| "Invalid assignments")?,
            )?;
            receipt["count"].as_u64() == Some(doc.assignments.len() as u64) && doc.verify(&after)
        };
        if !matches {
            return Err("Cell operation ACK differs from observed state; nothing replayed".into());
        }
        return Ok(Event::Status(after));
    }
    if let HttpCommand::Post(endpoint, "/api/led/output", form) = command
        && let Some((_, backend)) = form.iter().find(|(key, _)| key == "backend") {
        let client = pinned.as_ref().unwrap_or(client);
        let before = blocking_status(client, endpoint)?;
        if scope.pin != peer_http::Pin::from_status(&before) || !led_backend::allowed(&before, backend) {
            return Err("LED backend peer/capability unavailable; nothing sent".into());
        }
        scope.admit()?;
        let receipt: serde_json::Value = client.post(format!("{}/api/led/output", endpoint.base.trim_end_matches('/')))
            .basic_auth(&endpoint.user, Some(&endpoint.password)).form(form).send()
            .and_then(|response| response.error_for_status()).and_then(|response| response.json())
            .map_err(|_| "LED backend request outcome unknown; nothing replayed")?;
        if receipt["accepted"] != true {
            return Err(format!("LED backend rejected: {}", receipt["error"]));
        }
        let after = blocking_status(client, endpoint)?;
        if scope.pin != peer_http::Pin::from_status(&after) {
            return Err("LED backend peer changed; nothing replayed".into());
        }
        return led_backend::finish_network(endpoint, after, backend, || scope.admit()).map(Event::Status);
    }
    let result = http_request(pinned.as_ref().unwrap_or(client), command.clone())?;
    // Credential/hostname/self-update results may intentionally retire the endpoint.
    // Settings already performed readback; validate its observed epoch separately.
    // New mapping responses identify their own producer. Validate that exact
    // response rather than discarding valid mapping when an unrelated trailing
    // status request fails. Older firmware retains both status fences.
    let mapping_identified = if let Event::Mapping(_, _, Some((board, boot))) = &result {
        if !scope
            .pin
            .as_ref()
            .is_some_and(|pin| pin.matches_identity(board, boot))
        {
            return Err("Mapping belongs to a different board/boot; not applied".into());
        }
        true
    } else {
        false
    };
    let verify_read = (matches!(command, HttpCommand::Mapping(..)) && !mapping_identified)
        || matches!(command, HttpCommand::Post(_, pixel_corrections::PATH | pixel_groups::PATH, _))
        || matches!(command, HttpCommand::Configuration(..))
        || matches!(
            &result,
            Event::SettingsFinished(_, _, Ok(_)) | Event::NameUpdated(_, false)
        );
    if verify_read {
        let after = blocking_status(client, command.endpoint()).map_err(|_| {
            "Result received but board/boot readback unavailable; inspect before retrying"
                .to_string()
        })?;
        if scope.pin != peer_http::Pin::from_status(&after) {
            return Err(
                "Board/boot changed during readback; result not applied to this peer".into(),
            );
        }
    }
    if !scope.current() {
        return Err("Board session revoked after dispatch; outcome unknown, no replay".into());
    }
    Ok(result)
}

fn http_worker(
    commands: Receiver<HttpCommand>,
    events: Sender<Event>,
    context: egui::Context,
    gate: connection::Gate,
) {
    http_worker_with_discovery(
        commands,
        events,
        context,
        gate,
        platform::discovery::board_locations,
    )
}
fn http_worker_with_discovery(
    commands: Receiver<HttpCommand>,
    events: Sender<Event>,
    context: egui::Context,
    gate: connection::Gate,
    discover: impl Fn(Duration) -> Vec<String>,
) {
    let mut reporter = client_telemetry::Reporter::new();
    let mut discovered_base: Option<String> = None;
    let mut requested_base = String::new();
    let mut auth_probe: Option<(u64, String)> = None;
    let mut waiting = std::collections::VecDeque::new();
    let mut rejected_peer: Option<(u64, String, String)> = None;
    let mut last_discovery = Instant::now() - Duration::from_secs(30);
    loop {
        if waiting.is_empty() {
            match commands.recv() {
                Ok(command) => waiting.push_back(command),
                Err(_) => break,
            }
        }
        while waiting.len() < 64 {
            match commands.try_recv() {
                Ok(command) => waiting.push_back(command),
                Err(_) => break,
            }
        }
        // Strictly greater retains FIFO within a priority, like the donor.
        let mut best = 0;
        for index in 1..waiting.len() {
            if waiting[index].queue_priority() > waiting[best].queue_priority() {
                best = index;
            }
        }
        let command = waiting.remove(best).expect("nonempty HTTP queue");
        let stamp = gate.stamp(Transport::WiFi, gate.epoch());
        let lease = loop {
            if !stamp.current()
                || matches!(&command, HttpCommand::Scoped(scope, _) if !scope.current())
            {
                break None;
            }
            if let Some(lease) = gate.acquire(Transport::WiFi, stamp.epoch) {
                break Some(lease);
            }
            thread::sleep(Duration::from_millis(20));
        };
        let Some(lease) = lease else {
            continue;
        };
        // A verified TCP address is a routing hint, never ownership. Rebuild
        // this small client from the latest cache rather than repeating a slow
        // Windows .local lookup for each pinned/status/configuration request.
        // No idle socket survives release to the selected USB-only method.
        let client = peer_http::device_client_builder()
            .build()
            .expect("HTTP client");
        let events = SessionEvents {
            sender: events.clone(),
            stamp: lease.stamp(),
        };
        if requested_base != command.endpoint().base {
            discovered_base = None;
            requested_base = command.endpoint().base.clone();
        }
        let first = discovered_base
            .as_ref()
            .map(|base| command.with_base(base.clone()))
            .unwrap_or_else(|| command.clone());
        let key = (stamp.epoch, command.endpoint().base.clone());
        let mut result = if let Some((epoch, base, error)) = &rejected_peer
            && (*epoch, base.as_str()) == (key.0, key.1.as_str())
        {
            Err(error.clone())
        } else {
            http_request(&client, first)
        };
        if let Some(error) = result
            .as_ref()
            .err()
            .filter(|e| peer_identity::unsupported(e))
        {
            rejected_peer = Some((key.0, key.1, error.clone()));
        }
        if result.is_err()
            && command.is_status()
            && lease.current()
            && !result
                .as_ref()
                .err()
                .is_some_and(|e| peer_identity::unsupported(e) || e.starts_with("HTTP 401"))
            && let Some(origin) = peer_http::cached_origin(&command.endpoint().base)
            && let Some(board) =
                platform::discovery::probe_recovery(&command.endpoint().base, &origin)
            && lease.current()
            && match &command {
                HttpCommand::Scoped(scope, _) => scope.current(),
                _ => true,
            }
            && peer_http::recovery_matches(&command.endpoint().base, &board.status)
        {
            // A saved route is already bound to the exact logical endpoint
            // and board ID. Requiring its DNS alias to equal the configured
            // board label would wrongly reject localhost/Internet aliases
            // and completed renames on first open (where no boot pin exists).
            board.remember_route();
            discovered_base = Some(board.endpoint.base);
            let event = Event::Status(board.status);
            result = Ok(match &command {
                HttpCommand::Scoped(scope, _) => Event::ForScope(scope.clone(), Box::new(event)),
                _ => event,
            });
        }
        if result.is_err()
            && !result
                .as_ref()
                .err()
                .is_some_and(|e| peer_identity::unsupported(e))
            && lease.current()
            && command.discoverable_read()
            && last_discovery.elapsed() >= Duration::from_secs(5)
            && !result
                .as_ref()
                .err()
                .is_some_and(|e| e.starts_with("HTTP 401"))
        {
            last_discovery = Instant::now();
            discovered_base = None;
            let origins = discover(Duration::from_millis(1200));
            let available = platform::discovery::probe_boards(origins, &lease.stamp());
            let expected = match &command {
                HttpCommand::Scoped(scope, _) => {
                    scope.pin.as_ref().map(|pin| pin.board_id().to_owned())
                }
                _ => None,
            }
            .or_else(|| peer_http::remembered_board(&command.endpoint().base));
            if lease.current()
                && match &command {
                    HttpCommand::Scoped(scope, _) => scope.current(),
                    _ => true,
                }
                && let Some(choice) = platform::discovery::choose_board(
                    available,
                    expected.as_deref(),
                    auto_discovery_allowed(&command),
                )
            {
                use platform::discovery::BoardChoice;
                match choice {
                    BoardChoice::Known(board) => {
                        board.remember_route();
                        let preferred = board.endpoint.base.clone();
                        let event = if command.is_status() {
                            let event = Event::Status(board.status);
                            Ok(match &command {
                                HttpCommand::Scoped(scope, _) => {
                                    Event::ForScope(scope.clone(), Box::new(event))
                                }
                                _ => event,
                            })
                        } else {
                            let verified = peer_http::device_client_builder()
                                .build()
                                .expect("Verified fallback HTTP client");
                            http_request(&verified, command.with_base(preferred.clone()))
                        };
                        if let Ok(event) = event {
                            discovered_base = Some(preferred);
                            result = Ok(event);
                        }
                    }
                    BoardChoice::New(board) => {
                        board.remember_route();
                        let event = Event::DiscoveredBoard(board.endpoint, board.status);
                        result = Ok(match &command {
                            HttpCommand::Scoped(scope, _) => {
                                Event::ForScope(scope.clone(), Box::new(event))
                            }
                            _ => event,
                        });
                    }
                }
            }
        }
        if lease.current()
            && command.is_status()
            && reporter.due()
            && let Some(status) = result.as_ref().ok().and_then(Event::http_status)
            && status
                .capabilities
                .as_ref()
                .is_some_and(|c| c.has("telemetry"))
            && let Ok(pinned) = peer_http::pinned_client(status)
            && match &command {
                HttpCommand::Scoped(scope, _) => scope.current(),
                _ => false,
            }
        {
            let mut endpoint = command.endpoint().clone();
            if let Some(base) = &discovered_base {
                endpoint.base = base.clone();
            }
            reporter.report(&pinned, &endpoint);
        }
        if command.is_status()
            && lease.current()
            && !result
                .as_ref()
                .err()
                .is_some_and(|e| peer_identity::unsupported(e))
        {
            let key = (stamp.epoch, command.endpoint().base.clone());
            let unauthorized = result
                .as_ref()
                .err()
                .is_some_and(|error| error.starts_with("HTTP 401"));
            if auth_probe.as_ref() != Some(&key) || unauthorized {
                let mut endpoint = command.endpoint().clone();
                if let Some(base) = &discovered_base {
                    endpoint.base = base.clone();
                }
                let refreshed = peer_http::device_client_builder()
                    .build()
                    .expect("Current route HTTP client");
                if let Ok(info) = http_request(
                    &refreshed,
                    HttpCommand::Configuration(endpoint, "/api/auth/info"),
                ) {
                    auth_probe = Some(key);
                    let _ = events.send(for_endpoint(command.endpoint(), info));
                }
            }
        }
        let source = command.diagnostic_source();
        if result.as_ref().is_ok_and(Event::http_succeeded) {
            if let Some(source) = &source {
                let recovered = Event::DiagnosticRecovered(source.clone());
                let recovered = match &command {
                    HttpCommand::Scoped(scope, _) => {
                        Event::ForScope(scope.clone(), Box::new(recovered))
                    }
                    _ => recovered,
                };
                let _ = events.send(for_endpoint(command.endpoint(), recovered));
            }
        }
        let event = result.unwrap_or_else(|e| command.failure(e));
        if events
            .send(for_endpoint(command.endpoint(), event))
            .is_err()
        {
            break;
        }
        context.request_repaint();
    }
}
fn auto_discovery_allowed(command: &HttpCommand) -> bool {
    command.is_status()
        && matches!(command,HttpCommand::Scoped(scope,_) if scope.current()&&scope.pin.is_none()&&scope.auto_discovery)
}

fn websocket_url(base: &str) -> Result<String, String> {
    let parsed = reqwest::Url::parse(base).map_err(|e| e.to_string())?;
    let scheme = if parsed.scheme() == "https" {
        "wss"
    } else {
        "ws"
    };
    let host = parsed.host_str().ok_or("endpoint has no host")?;
    let authority = match parsed.port() {
        Some(p) => format!("{host}:{p}"),
        None => host.into(),
    };
    Ok(format!("{scheme}://{authority}/ws"))
}
fn configuration_event_notice(
    value: &serde_json::Value,
    pin: &peer_http::Pin,
) -> Option<(String, String, u64, String)> {
    if value["type"] != "configuration-changed" {
        return None;
    }
    let board = value["board_id"].as_str()?;
    let boot = value["boot_id"].as_str()?;
    let phase = value["phase"].as_str()?;
    let revision = value["revision"].as_u64()?;
    (pin.matches_identity(board, boot) && matches!(phase, "saved" | "applied" | "failed"))
        .then(|| (board.into(), boot.into(), revision, phase.into()))
}

fn rainbow_packet(offset: u16, count: u8, commit: bool, hue: u16) -> Vec<u8> {
    let mut packet = vec![0_u8; 10 + count as usize * 3];
    packet[0..4].copy_from_slice(b"PKAR");
    // Bytes 4 and 5 stay reserved zero, never an API generation.
    packet[6] = offset as u8;
    packet[7] = (offset >> 8) as u8;
    packet[8] = count;
    packet[9] = u8::from(commit);
    for i in 0..count as usize {
        let h = ((hue as usize + (offset as usize + i) * 3) % 360) as f32;
        let x = (1.0 - ((h / 60.0) % 2.0 - 1.0).abs()) * 255.0;
        let (r, g, b) = match (h / 60.0) as u8 {
            0 => (255, x as u8, 0),
            1 => (x as u8, 255, 0),
            2 => (0, 255, x as u8),
            3 => (0, x as u8, 255),
            4 => (x as u8, 0, 255),
            _ => (255, 0, x as u8),
        };
        let at = 10 + i * 3;
        packet[at..at + 3].copy_from_slice(&[r, g, b]);
    }
    packet
}

fn pixel_packet(pixels: &[u8], offset: u16, count: u8, commit: bool) -> Vec<u8> {
    let mut packet = vec![0_u8; 10 + count as usize * 3];
    packet[0..4].copy_from_slice(b"PKAR");
    // Bytes 4 and 5 stay reserved zero, never an API generation.
    packet[6] = offset as u8;
    packet[7] = (offset >> 8) as u8;
    packet[8] = count;
    packet[9] = u8::from(commit);
    let start = offset as usize * 3;
    let end = start + count as usize * 3;
    packet[10..].copy_from_slice(&pixels[start..end]);
    packet
}

fn sampled_screen_packets(mapping: &[CellMapping], layout: &pixel_groups::Layout) -> Result<(Vec<u8>, Vec<u8>), String> {
    let monitors = xcap::Monitor::all().map_err(|error| error.to_string())?;
    let monitor = monitors
        .iter()
        .find(|monitor| monitor.is_primary().unwrap_or(false))
        .or_else(|| monitors.first())
        .ok_or("no screen is available")?;
    let image = monitor.capture_image().map_err(|error| error.to_string())?;
    let pixels = streaming::sample_pixels(image.width(), image.height(), mapping, layout, |x, y| {
        let color = image.get_pixel(x, y).0;
        [color[0], color[1], color[2]]
    })?;
    Ok((
        pixel_packet(&pixels, 0, 240, false),
        pixel_packet(&pixels, 240, 240, true),
    ))
}

fn reject_pixel(events: &Sender<Event>, request: pixel_diagnostic::Request) {
    let endpoint = request.endpoint.clone();
    let _ = events.send(for_endpoint(
        &endpoint,
        Event::PixelSent(
            request,
            Err("WebSocket is not ready; diagnostic frame was not queued or replayed".into()),
        ),
    ));
}
fn recover_realtime(request: realtime::Intent, events: &Sender<Event>, gate: &connection::Gate) {
    let result = if let Some(_lease) = gate.acquire(Transport::WiFi, request.stamp.epoch) {
        request.http()
    } else {
        Err("Realtime connection retired; no fallback sent".into())
    };
    request.emit(events, result);
}
fn ws_worker(
    commands: Receiver<WsCommand>,
    events: Sender<Event>,
    context: egui::Context,
    gate: connection::Gate,
) {
    let mut retry = diagnostics::Backoff::new(Duration::from_millis(600), Duration::from_secs(15));
    let mut endpoint: Option<Endpoint> = None;
    let mut generation = 0;
    let mut streaming = false;
    let mut sampling = false;
    let mut pixel_subscription = false;
    let mut fps = streaming::DEFAULT_FPS;
    let mut mapping = Vec::new();
    loop {
        if endpoint.is_none() {
            match commands.recv() {
                Ok(WsCommand::Connect(e, epoch)) => {
                    endpoint = Some(e);
                    generation = epoch;
                }
                Ok(WsCommand::Disconnect) => {
                    endpoint = None;
                    streaming = false;
                    sampling = false;
                    mapping.clear();
                }
                Ok(WsCommand::Rate(value)) => fps = value.min(streaming::MAX_FPS),
                Ok(WsCommand::Mapping(value)) => mapping = value,
                Ok(WsCommand::Stream(v)) => streaming = v,
                Ok(WsCommand::Sample(v)) => sampling = v,
                Ok(WsCommand::PixelSubscribe(v)) => pixel_subscription = v,
                Ok(WsCommand::Pixel(request) | WsCommand::ArmPixel(request)) => {
                    reject_pixel(&events, request)
                }
                Ok(WsCommand::Control(request)) => {
                    recover_realtime(request, &events, &gate);
                    context.request_repaint();
                }
                Err(_) => return,
            }
        }
        loop {
            let command = match commands.try_recv() {
                Ok(command) => command,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            };
            match command {
                WsCommand::Connect(e, epoch) => {
                    if endpoint.as_ref() != Some(&e) {
                        mapping.clear();
                        sampling = false;
                        streaming = false;
                    }
                    endpoint = Some(e);
                    generation = epoch;
                }
                WsCommand::Disconnect => {
                    endpoint = None;
                    streaming = false;
                    sampling = false;
                    mapping.clear();
                }
                WsCommand::Rate(value) => fps = value.min(streaming::MAX_FPS),
                WsCommand::Mapping(value) => mapping = value,
                WsCommand::Stream(v) => streaming = v,
                WsCommand::Sample(v) => sampling = v,
                WsCommand::PixelSubscribe(v) => pixel_subscription = v,
                WsCommand::Pixel(request) | WsCommand::ArmPixel(request) => {
                    reject_pixel(&events, request)
                }
                WsCommand::Control(request) => {
                    recover_realtime(request, &events, &gate);
                    context.request_repaint();
                }
            }
        }
        let Some(current) = endpoint.clone() else {
            continue;
        };
        let Some(lease) = gate.acquire(Transport::WiFi, generation) else {
            thread::sleep(Duration::from_millis(20));
            continue;
        };
        let events = SessionEvents {
            sender: events.clone(),
            stamp: lease.stamp(),
        };
        let mut controls: Vec<(realtime::Intent, Instant)> = Vec::new();
        let mut awaiting_route = false;
        let result = (|| -> Result<(), String> {
            let mut request = websocket_url(&current.base)?
                .into_client_request()
                .map_err(|e| e.to_string())?;
            let auth = format!(
                "Basic {}",
                BASE64.encode(format!("{}:{}", current.user, current.password))
            );
            request.headers_mut().insert(
                "Authorization",
                auth.parse().map_err(|_| "invalid authorization")?,
            );
            request
                .headers_mut()
                .insert("Sec-WebSocket-Protocol", "arduino".parse().unwrap());
            let Some(mut socket) = peer_http::connect_websocket(&current, request, lease.stamp())?
            else {
                awaiting_route = true;
                return Ok(());
            };
            let mut peer_reporter = client_telemetry::Reporter::new();
            if let Some(packet) = peer_reporter.websocket_introduction() {
                socket
                    .send(Message::Text(packet.into()))
                    .map_err(|e| e.to_string())?;
            }
            // A successful TCP/HTTP upgrade is not recovery if the service then
            // immediately closes or never acknowledges an actual board status.
            let mut confirmed = false;
            let mut sent_pixel_subscription = None;
            let mut pixel_events_supported = false;
            let mut last_received = Instant::now();
            let mut last_status = Instant::now() - Duration::from_secs(2);
            let mut last_status_request = Instant::now() - Duration::from_secs(2);
            let mut observed_peer: Option<(peer_http::Pin, Instant, bool, bool)> = None;
            let mut sample_layout: Option<pixel_groups::Layout> = None;
            let mut configuration_notice: Option<(String, String, u64, String)> = None;
            let mut pending_arm: Option<(pixel_diagnostic::Request, bool)> = None;
            context.request_repaint();
            let mut hue = 0_u16;
            let mut last_stream = Instant::now() - Duration::from_millis(50);
            let mut stats_at = Instant::now();
            let mut sent_frames = 0_u32;
            loop {
                if !lease.current() {
                    break;
                }
                if confirmed && pixel_events_supported && sent_pixel_subscription != Some(pixel_subscription) {
                    socket.send(Message::Text(serde_json::json!({"type":"pixel-subscribe","enabled":pixel_subscription}).to_string().into()))
                        .map_err(|e| e.to_string())?;
                    sent_pixel_subscription = Some(pixel_subscription);
                }
                if let Some(packet) = peer_reporter.websocket_introduction() {
                    socket
                        .send(Message::Text(packet.into()))
                        .map_err(|e| e.to_string())?;
                }
                // A connected TCP handle is not a live WS channel. Probe only
                // when authoritative pushes go quiet; retire a silent socket
                // so later commands do not pay an800ms fallback on every click.
                if last_received.elapsed() >= Duration::from_secs(8) {
                    return Err("WebSocket heartbeat timed out; reconnecting read-only".into());
                }
                if last_status.elapsed() >= Duration::from_secs(2)
                    && last_status_request.elapsed() >= Duration::from_secs(2)
                {
                    socket
                        .send(Message::Text(r#"{"type":"status"}"#.into()))
                        .map_err(|e| e.to_string())?;
                    last_status_request = Instant::now();
                }
                if pending_arm
                    .as_ref()
                    .is_some_and(|(request, _)| request.at.elapsed() >= Duration::from_secs(8))
                {
                    if let Some((request, _)) = pending_arm.take() {
                        let _ = events.send(for_endpoint(&current, Event::PixelModeFinished(request.id,
                            Err("Effects-mode ACK/readback timed out; no frame sent or mode write replayed".into()))));
                    }
                }
                match commands.try_recv() {
                    Ok(WsCommand::Control(request)) => {
                        if let Err(error) = request.admit() {
                            request.emit(&events.sender, Err(error));
                        } else if controls.len() >= 8 {
                            request.emit(
                                &events.sender,
                                Err("Realtime command queue full; nothing sent".into()),
                            );
                        } else {
                            let sent = socket
                                .send(Message::Text(request.packet.clone().into()))
                                .is_ok();
                            if sent {
                                controls.push((request, Instant::now()));
                            } else {
                                recover_realtime(request, &events.sender, &gate);
                            }
                        }
                        context.request_repaint();
                    }
                    Ok(WsCommand::Connect(new_endpoint, epoch)) => {
                        mapping.clear();
                        sampling = false;
                        streaming = false;
                        endpoint = Some(new_endpoint);
                        generation = epoch;
                        break;
                    }
                    Ok(WsCommand::Disconnect) => {
                        endpoint = None;
                        mapping.clear();
                        sampling = false;
                        streaming = false;
                        break;
                    }
                    Ok(WsCommand::Rate(value)) => fps = value.min(streaming::MAX_FPS),
                    Ok(WsCommand::PixelSubscribe(value)) => pixel_subscription = value,
                    Ok(WsCommand::Mapping(value)) => {
                        mapping = value;
                        if !streaming::valid_mapping(&mapping) {
                            sampling = false;
                        }
                    }
                    Ok(WsCommand::Stream(value)) => {
                        streaming = value;
                        if value {
                            sampling = false;
                        }
                    }
                    Ok(WsCommand::Sample(value)) => {
                        sampling = value && streaming::valid_mapping(&mapping)
                            && sample_layout.is_some()
                            && observed_peer.as_ref().is_some_and(|(_, at, _, _)| at.elapsed() < Duration::from_secs(8));
                        if value && !sampling {
                            let _ = events.send(for_endpoint(&current, Event::DiagnosticFailure("Screen sampler".into(),
                                "Current board mapping or physical group layout unavailable; no sampled frame sent".into())));
                        }
                        if value {
                            streaming = false;
                        }
                    }
                    Ok(WsCommand::ArmPixel(request)) => {
                        if request.endpoint != current
                            || request.at.elapsed() >= Duration::from_secs(3)
                            || pending_arm.is_some()
                            || streaming
                            || sampling
                            || !observed_peer
                                .as_ref()
                                .is_some_and(|(peer, at, _, can_arm)| {
                                    peer == &request.peer
                                        && *can_arm
                                        && at.elapsed() < Duration::from_secs(8)
                                })
                        {
                            let _ = events.send(for_endpoint(&current, Event::PixelModeFinished(request.id,
                                Err("Stop streams and select the same enabled Cell board with no global effect before diagnostics".into()))));
                        } else {
                            socket
                                .send(Message::Text(
                                    r#"{"command":"led-output","mode":"effects"}"#.into(),
                                ))
                                .map_err(|_| "Mode write outcome unknown; no replay")?;
                            pending_arm = Some((request, false));
                        }
                        context.request_repaint();
                    }
                    Ok(WsCommand::Pixel(request)) => {
                        let result = (|| -> Result<(), String> {
                            if request.endpoint != current
                                || request.at.elapsed() >= Duration::from_secs(3)
                                || !lease.current()
                                || streaming
                                || sampling
                                || pending_arm.is_some()
                                || !observed_peer.as_ref().is_some_and(|(peer, at, ready, _)| {
                                    peer == &request.peer
                                        && *ready
                                        && at.elapsed() < Duration::from_secs(8)
                                })
                            {
                                return Err("Diagnostic peer/mode expired or another stream is active; frame not sent".into());
                            }
                            let pixels = pixel_diagnostic::pixels(request.index)
                                .ok_or("Pixel index outside 0..479")?;
                            socket
                                .send(Message::Binary(pixel_packet(&pixels, 0, 240, false).into()))
                                .map_err(
                                    |_| "Diagnostic write outcome unknown; no frame was replayed",
                                )?;
                            if !lease.current() {
                                return Err(
                                    "Connection released after first chunk; frame not committed"
                                        .into(),
                                );
                            }
                            socket
                                .send(Message::Binary(
                                    pixel_packet(&pixels, 240, 240, true).into(),
                                ))
                                .map_err(
                                    |_| "Diagnostic commit outcome unknown; no frame was replayed",
                                )?;
                            Ok(())
                        })();
                        let _ =
                            events.send(for_endpoint(&current, Event::PixelSent(request, result)));
                        context.request_repaint();
                    }
                    Err(TryRecvError::Disconnected) => return Ok(()),
                    Err(TryRecvError::Empty) => {}
                }
                if streaming && last_stream.elapsed() >= streaming::interval(fps) {
                    let frame_at = Instant::now();
                    socket
                        .send(Message::Binary(rainbow_packet(0, 240, false, hue).into()))
                        .map_err(|e| e.to_string())?;
                    socket
                        .send(Message::Binary(rainbow_packet(240, 240, true, hue).into()))
                        .map_err(|e| e.to_string())?;
                    hue = (hue + 4) % 360;
                    last_stream = frame_at;
                    sent_frames += 1;
                }
                if sampling && (sample_layout.is_none()
                    || !observed_peer.as_ref().is_some_and(|(_, at, _, _)| at.elapsed() < Duration::from_secs(8))) {
                    sampling = false;
                    let _ = events.send(for_endpoint(&current, Event::DiagnosticFailure("Screen sampler".into(),
                        "Board topology or observation expired; sampling stopped without replay".into())));
                }
                if sampling && last_stream.elapsed() >= streaming::interval(fps) {
                    let frame_at = Instant::now();
                    let (first, second) = sampled_screen_packets(&mapping, sample_layout.as_ref().unwrap())?;
                    socket
                        .send(Message::Binary(first.into()))
                        .map_err(|e| e.to_string())?;
                    socket
                        .send(Message::Binary(second.into()))
                        .map_err(|e| e.to_string())?;
                    last_stream = frame_at;
                    sent_frames += 1;
                }
                if stats_at.elapsed() >= Duration::from_secs(1) {
                    let measured = f64::from(sent_frames) / stats_at.elapsed().as_secs_f64();
                    let _ = events.send(for_endpoint(&current, Event::StreamStats(measured)));
                    sent_frames = 0;
                    stats_at = Instant::now();
                    context.request_repaint();
                }
                peer_http::websocket_read_timeout(
                    &mut socket,
                    streaming::read_budget(streaming || sampling, fps, last_stream.elapsed()),
                )?;
                match socket.read() {
                    Ok(Message::Text(text)) => {
                        last_received = Instant::now();
                        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text)
                            && value["type"] == "panel-event"
                        {
                            if let Some(event) = panel_event::Observation::parse(value) {
                                let _ =
                                    events.send(for_endpoint(&current, Event::PanelEvent(event)));
                                context.request_repaint();
                            }
                            continue; // Requested presentation cannot become a status/capability packet.
                        }
                        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text)
                            && value["type"] == "pc-contract-event"
                        {
                            if let Some(event) = peripheral_events::Observation::parse(value) {
                                let _ = events
                                    .send(for_endpoint(&current, Event::PeripheralEvent(event)));
                                context.request_repaint();
                            }
                            continue;
                        }
                        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text)
                            && value["type"] == "status-rgb"
                        {
                            if let Some(rgb) = status_rgb::Observation::parse(value) {
                                let _ = events.send(for_endpoint(&current, Event::StatusRgb(rgb)));
                                context.request_repaint();
                            }
                            continue; // Optional RGB cannot become a partial full-status packet.
                        }
                        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text)
                            && value["type"] == "configuration-changed"
                        {
                            if let Some((pin, _, _, _)) = &observed_peer
                                && let Some(notice) = configuration_event_notice(&value, pin)
                                && configuration_notice.as_ref() != Some(&notice)
                            {
                                configuration_notice = Some(notice);
                                // Ask on the existing socket, not the possibly
                                // retired HTTP Host name. Never seed an address
                                // or actuation directly from event metadata.
                                socket
                                    .send(Message::Text(r#"{"type":"status"}"#.into()))
                                    .map_err(|e| e.to_string())?;
                                let _ = events.send(for_endpoint(
                                    &current,
                                    Event::ConfigurationChanged(
                                        value["board_id"].as_str().unwrap_or_default().into(),
                                        value["boot_id"].as_str().unwrap_or_default().into(),
                                    ),
                                ));
                                context.request_repaint();
                            }
                            continue;
                        }
                        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text)
                            && value["type"] == "command-result"
                            && let Some(index) = controls
                                .iter()
                                .position(|(request, _)| value["request_id"] == request.id)
                        {
                            let (request, _) = controls.remove(index);
                            let result = request.verify(value);
                            if result.is_ok() {
                                last_status = Instant::now();
                            }
                            request.emit(&events.sender, result);
                            context.request_repaint();
                            continue;
                        }
                        if let Some(upgrade) = firmware_progress::websocket_upgrade(&text) {
                            let _ = events
                                .send(for_endpoint(&current, Event::UpgradeProgress(upgrade)));
                            context.request_repaint();
                            continue;
                        }
                        if pending_arm.as_ref().is_some_and(|(_, acked)| !acked) {
                            if let Some(ack) = pixel_diagnostic::mode_ack(&text) {
                                match ack {
                                    Ok(()) => {
                                        if let Some((_, acked)) = pending_arm.as_mut() {
                                            *acked = true;
                                        }
                                        socket
                                            .send(Message::Text(r#"{"type":"status"}"#.into()))
                                            .map_err(|e| e.to_string())?;
                                    }
                                    Err(error) => {
                                        if let Some((request, _)) = pending_arm.take() {
                                            let _ = events.send(for_endpoint(
                                                &current,
                                                Event::PixelModeFinished(request.id, Err(error)),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        if let Some((peer, at, _, _)) = observed_peer.as_ref()
                            && pixel_subscription && at.elapsed() < Duration::from_secs(8)
                            && let Some(frame) = ui::cell_pixels::websocket_sample(&text, peer) {
                            let _ = events.send(for_endpoint(&current, Event::PixelSample(peer.clone(), frame)));
                            context.request_repaint();
                            continue;
                        }
                        if let Some(status) = websocket_status(&text) {
                            peer_identity::validate_board(&status)?;
                            sample_layout = streaming::sampling_layout(&status);
                            pixel_events_supported = status.capabilities.as_ref().is_some_and(|c| c.has("pixel-events"));
                            last_status = Instant::now();
                            observed_peer = peer_http::Pin::from_status(&status).map(|pin| {
                                (
                                    pin,
                                    Instant::now(),
                                    pixel_diagnostic::ready(&status),
                                    pixel_diagnostic::can_arm(&status),
                                )
                            });
                            if pending_arm.as_ref().is_some_and(|(request, acked)| {
                                *acked
                                    && peer_http::Pin::from_status(&status).as_ref()
                                        == Some(&request.peer)
                                    && pixel_diagnostic::ready(&status)
                            }) {
                                if let Some((request, _)) = pending_arm.take() {
                                    let _ = events.send(for_endpoint(
                                        &current,
                                        Event::PixelModeFinished(request.id, Ok(status.clone())),
                                    ));
                                }
                            }
                            if !confirmed && websocket_status_ready(&status) {
                                confirmed = true;
                                retry.reset();
                                let _ = events.send(for_endpoint(&current, Event::WebSocket(true)));
                                let _ = events.send(for_endpoint(
                                    &current,
                                    Event::DiagnosticRecovered("WebSocket".into()),
                                ));
                            }
                            let _ = events.send(for_endpoint(&current, Event::Status(status)));
                            context.request_repaint();
                        }
                    }
                    Ok(Message::Close(_)) => return Err("WebSocket closed".into()),
                    Ok(_) => {
                        last_received = Instant::now();
                    }
                    Err(tungstenite::Error::Io(e))
                        if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
                    Err(e) => return Err(e.to_string()),
                }
                let mut index = 0;
                while index < controls.len() {
                    if controls[index].1.elapsed() >= Duration::from_millis(800) {
                        let (request, _) = controls.remove(index);
                        recover_realtime(request, &events.sender, &gate);
                        context.request_repaint();
                    } else {
                        index += 1;
                    }
                }
            }
            Ok(())
        })();
        if !awaiting_route {
            let _ = events.send(for_endpoint(&current, Event::WebSocket(false)));
            context.request_repaint();
        }
        drop(lease); // The closure dropped the socket before another method opens.
        for (request, _) in controls {
            recover_realtime(request, &events.sender, &gate);
        }
        if result.is_err() || awaiting_route {
            if let Err(error) = result {
                if peer_identity::unsupported(&error) {
                    endpoint = None;
                } // Explicit reconnect after update/override, no endless incompatible handshake loop.
                streaming = false;
                sampling = false; // Reconnect must not silently restart a stream.
                let _ = events.send(for_endpoint(
                    &current,
                    Event::DiagnosticFailure("WebSocket".into(), error),
                ));
            }
            // Interruptible: changing peer or stopping streaming need not wait
            // for the retry deadline. Mutations are never replayed here.
            let delay = if awaiting_route {
                Duration::from_millis(250)
            } else {
                let delay = retry.delay();
                retry.failed();
                delay
            };
            match commands.recv_timeout(delay) {
                Ok(WsCommand::Connect(e, epoch)) => {
                    endpoint = Some(e);
                    generation = epoch;
                    mapping.clear();
                    streaming = false;
                    sampling = false;
                    retry.reset();
                }
                Ok(WsCommand::Disconnect) => {
                    endpoint = None;
                    mapping.clear();
                    streaming = false;
                    sampling = false;
                    retry.reset();
                }
                Ok(WsCommand::Rate(value)) => fps = value.min(streaming::MAX_FPS),
                Ok(WsCommand::Mapping(value)) => mapping = value,
                Ok(WsCommand::Stream(value)) => streaming = value,
                Ok(WsCommand::Sample(value)) => sampling = value,
                Ok(WsCommand::PixelSubscribe(value)) => pixel_subscription = value,
                Ok(WsCommand::Pixel(request) | WsCommand::ArmPixel(request)) => {
                    reject_pixel(&events.sender, request)
                }
                Ok(WsCommand::Control(request)) => {
                    recover_realtime(request, &events.sender, &gate);
                    context.request_repaint();
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => return,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
    }
}

fn crc8(data: &[u8]) -> u8 {
    let mut crc = 0_u8;
    for byte in data {
        crc ^= *byte;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 {
                (crc << 1) ^ 0x07
            } else {
                crc << 1
            };
        }
    }
    crc
}
fn cobs_encode(input: &[u8]) -> Vec<u8> {
    let mut out = vec![0_u8];
    let mut code_at = 0;
    let mut code = 1_u8;
    for byte in input {
        if *byte == 0 {
            out[code_at] = code;
            code_at = out.len();
            out.push(0);
            code = 1;
        } else {
            out.push(*byte);
            code += 1;
            if code == 0xff {
                out[code_at] = code;
                code_at = out.len();
                out.push(0);
                code = 1;
            }
        }
    }
    out[code_at] = code;
    out.push(0);
    out
}
fn cobs_decode(input: &[u8]) -> Option<Vec<u8>> {
    if input.is_empty() {
        return None;
    }
    let mut out = Vec::new();
    let mut at = 0;
    while at < input.len() {
        let code = *input.get(at)? as usize;
        if code == 0 || at + code > input.len() + 1 {
            return None;
        }
        at += 1;
        for _ in 1..code {
            let value = *input.get(at)?;
            if value == 0 {
                return None;
            }
            out.push(value);
            at += 1;
        }
        if code != 0xff && at < input.len() {
            out.push(0);
        }
    }
    Some(out)
}
fn wire_frame(opcode: u8, sequence: u8, payload: &[u8]) -> Vec<u8> {
    let mut raw = vec![wire::MAGIC, wire::RESERVED, opcode, sequence, payload.len() as u8];
    raw.extend_from_slice(payload);
    raw.push(crc8(&raw));
    cobs_encode(&raw)
}
fn decode_frame(encoded: &[u8]) -> Option<(u8, Vec<u8>)> {
    decode_envelope(encoded).map(|(op, _, payload)| (op, payload))
}
fn decode_envelope(encoded: &[u8]) -> Option<(u8, u8, Vec<u8>)> {
    if encoded.len() > wire::MAXIMUM_PAYLOAD + 7 { return None; }
    let raw = cobs_decode(encoded)?;
    if raw.len() < 6
        || raw[0] != wire::MAGIC
        || raw[1] != wire::RESERVED
        || usize::from(raw[4]) > wire::MAXIMUM_PAYLOAD
        || raw.len() != raw[4] as usize + 6
        || crc8(&raw[..raw.len() - 1]) != raw[raw.len() - 1]
    {
        return None;
    }
    Some((raw[2], raw[3], raw[5..raw.len() - 1].to_vec()))
}
fn serial_status(payload: &[u8]) -> Option<DeviceStatus> {
    if payload.len() < 19 {
        return None;
    }
    Some(DeviceStatus {
        capabilities: capabilities::Manifest::from_usb(payload),
        mode: ["unknown", "cell", "first-wins", "snake"]
            .get(payload[0] as usize)
            .unwrap_or(&"unknown")
            .to_string(),
        winner: payload[1],
        selected_cell: payload[2],
        brightness: payload[3],
        led_output: payload
            .get(42)
            .filter(|v| **v <= 1)
            .map(|v| {
                let mut state = serde_json::json!({"enabled": *v != 0});
                if let Some(mode) = payload
                    .get(43)
                    .and_then(|v| ["static", "effects"].get(*v as usize))
                {
                    state["mode"] = (*mode).into();
                }
                state
            })
            .unwrap_or_default(),
        effect: ["none", "rainbow", "fire"]
            .get(payload[4] as usize)
            .unwrap_or(&"unknown")
            .to_string(),
        avr_online: payload[5] != 0,
        tm1637_ack: payload[6] != 0,
        tm1637_segments: payload.get(19..23).unwrap_or_default().to_vec(),
        tm1637_brightness: payload.get(23).copied().unwrap_or_default(),
        tm1637_host_active: payload.get(24).is_some_and(|value| *value != 0),
        avr_version: if payload.len() >= 29 && payload[5] != 0 && payload[28] > 0 {
            Some(format!("{}.{}.{}", payload[25], payload[26], payload[27]))
        } else {
            None
        },
        audio_cues: payload
            .get(28)
            .filter(|count| payload[5] != 0 && **count as usize <= audio::CUES.len())
            .map(|count| {
                audio::CUES[..*count as usize]
                    .iter()
                    .map(|(name, _)| (*name).to_owned())
                    .collect()
            })
            .unwrap_or_default(),
        programming_latch: payload[7] != 0,
        active_inputs: payload[8],
        input_9: payload[9] != 0,
        active_outputs: payload[10],
        esp_uart_errors: u16::from_le_bytes([payload[11], payload[12]]),
        avr_uart_errors: u16::from_le_bytes([payload[13], payload[14]]),
        station_connected: payload[15] != 0,
        ap_started: payload[16] != 0,
        ntp_synced: payload[17] != 0,
        link_mode: if payload[18] != 0 {
            "busy-wire"
        } else {
            "polling"
        }
        .into(),
        ..Default::default()
    })
}

fn serial_worker(
    commands: Receiver<SerialCommand>,
    events: Sender<Event>,
    context: egui::Context,
    gate: connection::Gate,
) {
    serial_worker_with(commands, events, context, gate, |name| {
        serialport::new(name, 115_200)
            .dtr_on_open(false)
            .timeout(Duration::from_millis(10))
            .open()
            .map_err(|error| error.to_string())
    });
}
// Inject the byte transport, not the scheduler/decoder, for deterministic tests
// of the actual production worker (PCController Session fakePort pattern).
fn serial_worker_with<P: Read + Write, F: FnMut(&str) -> Result<P, String>>(
    commands: Receiver<SerialCommand>,
    events: Sender<Event>,
    context: egui::Context,
    gate: connection::Gate,
    mut open: F,
) {
    let mut port_name = String::new();
    let mut generation = 0;
    let mut sequence = 1_u8;
    let mut nonce = 0_u8;
    let mut physical_session = 0_u64;
    let mut demand = usb_demand::Demand::default();
    loop {
        if port_name.is_empty() {
            match commands.recv() {
                Ok(SerialCommand::Port(p, epoch)) => {
                    port_name = p;
                    generation = epoch;
                }
                Ok(_) => {}
                Err(_) => return,
            }
        }
        if port_name.is_empty() {
            continue;
        }
        loop {
            let c = match commands.try_recv() {
                Ok(command) => command,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            };
            match c {
                SerialCommand::Port(p, epoch) => {
                    port_name = p;
                    generation = epoch;
                    demand = Default::default();
                }
                SerialCommand::Demand(p, epoch, next) if p == port_name && epoch == generation => {
                    demand = next
                }
                _ => {}
            }
        }
        if port_name.is_empty() {
            continue;
        }
        let Some(lease) = gate.acquire(Transport::Serial, generation) else {
            thread::sleep(Duration::from_millis(20));
            continue;
        };
        let events = SessionEvents {
            sender: events.clone(),
            stamp: lease.stamp(),
        };
        let mut port = match open(&port_name) {
            Ok(port) => port,
            Err(error) => {
                let _ = events.send(Event::SerialFailure(port_name.clone(), error.to_string()));
                let _ = events.send(Event::Serial(port_name.clone(), false, None));
                context.request_repaint();
                thread::sleep(Duration::from_secs(1));
                while let Ok(c) = commands.try_recv() {
                    if let SerialCommand::Port(p, epoch) = c {
                        port_name = p;
                        generation = epoch;
                    }
                }
                continue;
            }
        };
        let connected_port = port_name.clone();
        let Some(next_session) = physical_session.checked_add(1) else {
            return;
        };
        physical_session = next_session;
        let _ = events.send(Event::SerialOwner(connected_port.clone(), physical_session));
        // An opened handle is not a compatible, admitted board connection.
        context.request_repaint();
        let mut encoded = Vec::new();
        let mut last_poll = Instant::now() - Duration::from_secs(1);
        let mut last_identity = Instant::now() - Duration::from_secs(20);
        let mut stamp = None;
        let mut identity: Option<serde_json::Value> = None;
        let mut read: Option<identity::UsbRead> = None;
        let mut read_wait: Option<(u8, Instant)> = None;
        let mut packets = std::collections::VecDeque::new();
        let mut write_wait: Option<(u8, Instant)> = None;
        let mut expected: Option<(String, String)> = None;
        let mut identity_admission: Option<usb_table::Admission> = None;
        let mut table_supported = false;
        let mut compact_supported = false;
        let mut demand_supported = false;
        let mut demand_lease = usb_demand::Lease::default();
        let mut table_read: Option<usb_table::Reader> = None;
        let mut paused_table: Option<usb_table::Reader> = None;
        let mut last_command_state: Option<(Vec<u8>, Instant)> = None;
        let mut table_wait: Option<(u8, Instant)> = None;
        let mut table_following = std::collections::VecDeque::<(u8, Instant)>::new();
        let mut table_started = Instant::now();
        let mut progress_at = Instant::now();
        let mut last_table = Instant::now() - Duration::from_secs(2);
        let mut table_snapshot: Option<DeviceStatus> = None;
        let mut table_observed = Instant::now() - Duration::from_secs(20);
        let mut intents = std::collections::VecDeque::<usb_table::Intent>::new();
        let mut operation: Option<usb_table::Operation> = None;
        let mut api: Option<usb_api::Work> = None;
        let mut silent: Option<(
            u64,
            usb_table::Admission,
            board_silent::Transaction,
            Option<(u8, Instant)>,
        )> = None;
        let mut reported_depth = 0;
        let mut intro_wait = Some((wire::HELLO, sequence, Instant::now()));
        let mut intro_chunks = std::collections::VecDeque::<Vec<u8>>::new();
        let mut peer_reports = peer_identity::UsbReportSupport::default();
        let mut introduced = false;
        let mut reintroductions = 0_u8;
        let mut last_peer_report = Instant::now();
        if port
            .write_all(&wire_frame(wire::HELLO, sequence, &peer_identity::hello()))
            .is_err()
        {
            continue;
        }
        sequence = sequence.wrapping_add(1);
        'connected: loop {
            if !lease.current() {
                break;
            }
            while let Ok(c) = commands.try_recv() {
                match c {
                    SerialCommand::Api(target, work) if target == connected_port && lease.current() => {
                        if api.is_some() || operation.is_some() || !intents.is_empty() || silent.is_some() || expected.is_some()
                            || !work.current(physical_session, table_snapshot.as_ref()) {
                            work.fail("USB API busy or peer retired; nothing sent");
                        } else { api = Some(work); }
                    }
                    SerialCommand::Demand(target, epoch, next)
                        if target == connected_port && epoch == generation =>
                    {
                        demand = next;
                        demand_lease.desired(next.flags);
                    }
                    SerialCommand::Port(p, epoch) => {
                        port_name = p;
                        generation = epoch;
                        break 'connected;
                    }
                    SerialCommand::Send(target, admission, op, payload)
                        if target == connected_port && lease.current() =>
                    {
                        if !admission.current(physical_session, table_snapshot.as_ref()) {
                            let _=events.send(Event::SerialOperation(connected_port.clone(),Err("USB command belongs to a retired UI/session/peer; nothing sent".into())));
                        } else if payload.len() > 48 || intents.len() >= 32 {
                            let _ = events.send(Event::SerialOperation(
                                connected_port.clone(),
                                Err("USB command queue full or payload too large; nothing sent"
                                    .into()),
                            ));
                        } else if usb_table::table_command(op) && table_snapshot.is_none() {
                            let _ = events.send(Event::SerialOperation(
                                connected_port.clone(),
                                Err("USB table snapshot not verified; nothing sent".into()),
                            ));
                        } else {
                            intents.push_back(usb_table::Intent {
                                opcode: op,
                                payload,
                                admission,
                                reply: None,
                            });
                        }
                    }
                    SerialCommand::Bridge(target, admission, op, payload, reply)
                        if target == connected_port && lease.current() =>
                    {
                        let intent = usb_table::Intent {
                            opcode: op,
                            payload,
                            admission,
                            reply: Some(reply),
                        };
                        if !intent.current(physical_session, table_snapshot.as_ref())
                            || intents.len() >= 32
                            || !usb_table::table_command(op)
                        {
                            intent.reply(Err("USB bridge request expired, unsupported or owner unavailable; nothing sent".into()));
                        } else {
                            intents.push_back(intent);
                        }
                    }
                    SerialCommand::BoardSilent(target, admission, id, desired)
                        if target == connected_port =>
                    {
                        if silent.is_some()
                            || !admission.current(physical_session, table_snapshot.as_ref())
                            || !table_snapshot.as_ref().is_some_and(|s| desired.allowed(s))
                        {
                            let _ = events.send(Event::BoardSilentFinished(id, Err("USB board settings busy, unavailable or peer retired; nothing sent".into())));
                        } else {
                            silent = Some((
                                id,
                                admission,
                                board_silent::Transaction::new(desired),
                                None,
                            ));
                        }
                    }
                    SerialCommand::Identity(target, admission, host, label)
                        if target == connected_port && lease.current() =>
                    {
                        if admission.peer.is_none()
                            || !admission.current(physical_session, table_snapshot.as_ref())
                        {
                            let _=events.send(Event::SerialIdentitySaved(connected_port.clone(),Err("USB identity request belongs to an unverified or retired board; nothing sent".into())));
                        } else if expected.is_some() {
                            let _ = events.send(Event::SerialIdentitySaved(
                                connected_port.clone(),
                                Err("An identity write is already pending".into()),
                            ));
                        } else {
                            match identity::usb_write(&host, &label) {
                                Ok(next) => {
                                    packets = next;
                                    identity_admission = Some(admission);
                                    expected =
                                        Some((identity::normalize_name(&host).unwrap(), label));
                                    read = None;
                                    read_wait = None;
                                }
                                Err(error) => {
                                    let _ = events.send(Event::SerialIdentitySaved(
                                        connected_port.clone(),
                                        Err(error),
                                    ));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            if intro_wait.is_some_and(|(_, _, at)| at.elapsed() >= Duration::from_secs(4)) {
                let _=events.send(Event::SerialFailure(connected_port.clone(),"Peer compatibility: USB introduction timed out; explicit reconnect after updating the board".into()));
                port_name.clear();
                break 'connected;
            }
            if api.is_none() && introduced
                && peer_reports.supported()
                && intro_wait.is_none()
                && intro_chunks.is_empty()
                && last_peer_report.elapsed() >= Duration::from_secs(30)
                && operation.is_none()
                && table_read.is_none()
                && write_wait.is_none()
                && read_wait.is_none()
            {
                intro_chunks = peer_identity::report_chunks(
                    &client_telemetry::Reporter::new().value_for_transport("usb"),
                )
                .unwrap_or_default();
                last_peer_report = Instant::now();
            }
            if intro_wait.is_none() && !intro_chunks.is_empty() {
                let payload = intro_chunks.pop_front().unwrap();
                if port
                    .write_all(&wire_frame(peer_identity::USB_REPORT, sequence, &payload))
                    .is_err()
                {
                    break 'connected;
                }
                intro_wait = Some((peer_identity::USB_REPORT, sequence, Instant::now()));
                sequence = sequence.wrapping_add(1);
            }
            if operation
                .as_ref()
                .is_some_and(|op| op.ack.is_none() && op.sent.elapsed() > Duration::from_secs(2))
            {
                let _ = events.send(Event::SerialOperation(
                    connected_port.clone(),
                    Err("USB command ACK timed out; outcome unknown, nothing replayed".into()),
                ));
                break 'connected;
            }
            // READ windows share one absolute capture deadline, not a shorter
            // per-fragment timer. Commands retain their separate2s ACK bound.
            // A delayed read can finish only while its ORIGINAL8s age is valid.
            if (table_read.is_some() || paused_table.is_some())
                && table_started.elapsed() >= usb_table::MAX_SNAPSHOT_AGE
            {
                if let Some(reader) = table_read.as_ref().or(paused_table.as_ref()) {
                    let _ = events.send(Event::SerialCapture(
                        connected_port.clone(),
                        reader.progress("expired"),
                    ));
                }
                table_read = None;
                paused_table = None;
                table_wait = None;
                table_following.clear();
                last_table = Instant::now();
                let error =
                    "USB table snapshot timed out; no state assumed and no command replayed"
                        .to_string();
                let _ = events.send(Event::SerialFailure(connected_port.clone(), error.clone()));
                if let Some(active) = operation.take() {
                    active.intent.reply(Err(error.clone()));
                    let _ = events.send(Event::SerialOperation(connected_port.clone(), Err(error)));
                }
            }
            // Yield only BETWEEN correlated chunks. Never preempt an in-flight
            // request or a post-ACK verification. A compact-capable peer can
            // resume immutable metadata after control; overlay newer actual
            // command readback and invalidate pre-command RGB before publishing.
            // This lets full identity/peripheral reads progress during bursts.
            if silent.is_none()
                && usb_table::yield_snapshot(
                    !intents.is_empty() || api.is_some(),
                    table_snapshot.is_some(),
                    table_read.is_some(),
                    table_wait.is_some(),
                    operation.is_some(),
                )
            {
                if compact_supported {
                    paused_table = table_read.take();
                } else {
                    table_read = None;
                }
                last_table = Instant::now();
            }
            if api.is_none() && silent.is_none()
                && operation.is_none()
                && table_read.is_none()
                && write_wait.is_none()
                && expected.is_none()
                && let Some(intent) = intents.front()
            {
                let is_table = usb_table::table_command(intent.opcode);
                if !intent
                    .admission
                    .current(physical_session, table_snapshot.as_ref())
                    || (is_table && !intent.current(physical_session, table_snapshot.as_ref()))
                {
                    if let Some(intent) = intents.pop_front() {
                        intent.reply(Err(
                            "Queued USB command expired or peer changed; nothing sent".into(),
                        ));
                    }
                    let _ = events.send(Event::SerialOperation(
                        connected_port.clone(),
                        Err("Queued USB command expired or peer changed; nothing sent".into()),
                    ));
                } else if !is_table || table_observed.elapsed() < usb_table::MAX_SNAPSHOT_AGE {
                    let intent = intents.pop_front().unwrap();
                    let packet = if is_table {
                        usb_table::fenced(
                            table_snapshot.as_ref().unwrap(),
                            intent.opcode,
                            &intent.payload,
                        )
                    } else {
                        Ok(intent.payload.clone())
                    };
                    match packet {
                        Ok(payload) => {
                            if !lease.current()
                                || !intent
                                    .admission
                                    .current(physical_session, table_snapshot.as_ref())
                            {
                                let _ = events.send(Event::SerialOperation(
                                    connected_port.clone(),
                                    Err("USB command admission revoked before write; nothing sent"
                                        .into()),
                                ));
                                continue 'connected;
                            }
                            let sent = Instant::now();
                            if let Err(error) = port.write_all(&wire_frame(
                                if is_table {
                                    usb_table::COMMAND
                                } else {
                                    intent.opcode
                                },
                                sequence,
                                &payload,
                            )) {
                                let _ = events.send(Event::SerialOperation(
                                    connected_port.clone(),
                                    Err(format!(
                                        "USB write failed; outcome unknown, no replay: {error}"
                                    )),
                                ));
                                break 'connected;
                            }
                            operation = Some(usb_table::Operation {
                                intent,
                                sequence,
                                sent,
                                ack: None,
                            });
                            sequence = sequence.wrapping_add(1);
                        }
                        Err(error) => {
                            intent.reply(Err(error.clone()));
                            let _ = events
                                .send(Event::SerialOperation(connected_port.clone(), Err(error)));
                        }
                    }
                }
            }
            if operation.as_ref().is_some_and(|op| {
                op.ack
                    .is_some_and(|at| at.elapsed() >= Duration::from_millis(100))
            }) {
                paused_table = None; // Missing compact companion: require a NEW post-ACK capture.
            }
            if api.is_none() && silent.is_none() && operation.is_none() && table_read.is_none() && intents.is_empty()
            {
                table_read = paused_table.take();
            }
            if api.is_none() && silent.is_none()
                && table_supported
                && table_read.is_none()
                && paused_table.is_none()
                && write_wait.is_none()
                && expected.is_none()
                && (demand.table
                    || operation.as_ref().is_some_and(|op| op.ack.is_some())
                    || !intents.is_empty())
                && usb_table::snapshot_admitted(
                    !intents.is_empty() && table_observed.elapsed() < usb_table::MAX_SNAPSHOT_AGE,
                    operation.as_ref(),
                    compact_supported,
                )
                && last_table.elapsed() >= Duration::from_secs(1)
            {
                nonce = nonce.wrapping_add(1);
                table_read = Some(usb_table::Reader::new(nonce));
                table_started = Instant::now();
                last_table = Instant::now();
            }
            if let Some((id, admission, transaction, wait)) = &silent {
                if !admission.current(physical_session, table_snapshot.as_ref())
                    || !table_snapshot
                        .as_ref()
                        .is_some_and(|s| transaction.allowed(s))
                    || wait.is_some_and(|(_, at)| at.elapsed() > Duration::from_secs(2))
                {
                    let _ = events.send(Event::BoardSilentFinished(
                        *id,
                        Err(
                            "USB settings timed out or peer retired; outcome unknown, no replay"
                                .into(),
                        ),
                    ));
                    silent = None;
                }
            }
            if api.is_none() && operation.is_none()
                && table_read.is_none()
                && write_wait.is_none()
                && expected.is_none()
                && let Some((id, admission, transaction, wait)) = &mut silent
                && wait.is_none()
            {
                let (opcode, mut payload) = transaction.request();
                payload.insert(0, opcode);
                if !lease.current()
                    || !admission.current(physical_session, table_snapshot.as_ref())
                    || port
                        .write_all(&wire_frame(wire::PC_CONTRACT, sequence, &payload))
                        .is_err()
                {
                    let _ = events.send(Event::BoardSilentFinished(
                        *id,
                        Err("USB settings write unavailable; outcome unknown, no replay".into()),
                    ));
                    break 'connected;
                }
                *wait = Some((sequence, Instant::now()));
                sequence = sequence.wrapping_add(1);
            }
            if api.is_none() && lease.current()
                && let Some(reader) = &table_read
                && table_wait.is_none()
            {
                if port
                    .write_all(&wire_frame(usb_table::READ, sequence, &reader.request()))
                    .is_err()
                {
                    break 'connected;
                }
                table_wait = Some((sequence, Instant::now()));
                sequence = sequence.wrapping_add(1);
                for request in reader.following_requests() {
                    if port
                        .write_all(&wire_frame(usb_table::READ, sequence, &request))
                        .is_err()
                    {
                        break 'connected;
                    }
                    table_following.push_back((sequence, Instant::now()));
                    sequence = sequence.wrapping_add(1);
                }
            }
            if progress_at.elapsed() >= Duration::from_millis(500)
                && let Some(reader) = table_read.as_ref().or(paused_table.as_ref())
            {
                let _ = events.send(Event::SerialCapture(
                    connected_port.clone(),
                    reader.progress("reading"),
                ));
                progress_at = Instant::now();
            }
            if write_wait.is_some_and(|(_, at)| at.elapsed() > Duration::from_secs(2))
                || read_wait.is_some_and(|(_, at)| at.elapsed() > Duration::from_secs(2))
                || (read.is_some() && last_identity.elapsed() > Duration::from_secs(8))
            {
                packets.clear();
                write_wait = None;
                read_wait = None;
                read = None;
                identity = None;
                if expected.take().is_some() {
                    let _=events.send(Event::SerialIdentitySaved(connected_port.clone(),Err("Identity response timed out. Refresh before retrying; no mutation was replayed.".into())));
                }
                last_identity = Instant::now(); // Bounded backoff for a read-only refresh.
            }
            if api.is_none() && lease.current() && write_wait.is_none() {
                if let Some(payload) = packets.pop_front() {
                    if !identity_admission.as_ref().is_some_and(|admission| {
                        admission.current(physical_session, table_snapshot.as_ref())
                    }) {
                        packets.clear();
                        expected = None;
                        identity_admission = None;
                        let _ = events.send(Event::SerialIdentitySaved(
                            connected_port.clone(),
                            Err("USB identity admission revoked before write; no replay".into()),
                        ));
                        continue 'connected;
                    }
                    if port
                        .write_all(&wire_frame(wire::DEVICE_NAME, sequence, &payload))
                        .is_err()
                    {
                        break;
                    }
                    write_wait = Some((sequence, Instant::now()));
                    sequence = sequence.wrapping_add(1);
                } else if let Some(reader) = &read {
                    if read_wait.is_none() {
                        if port
                            .write_all(&wire_frame(wire::GET_IDENTITY, sequence, &reader.request()))
                            .is_err()
                        {
                            break;
                        }
                        read_wait = Some((sequence, Instant::now()));
                        sequence = sequence.wrapping_add(1);
                    }
                }
            }
            if introduced && lease.current() && last_poll.elapsed() >= Duration::from_millis(750) {
                if port
                    .write_all(&wire_frame(OP_GET_STATUS, sequence, &[]))
                    .is_err()
                {
                    break 'connected;
                }
                sequence = sequence.wrapping_add(1);
                last_poll = Instant::now();
            }
            // Never serialize lease ACKs ahead of commands or snapshot chunks.
            // One small best-effort packet uses the same physical writer/sequence.
            if introduced
                && demand_supported
                && lease.current()
                && intro_wait.is_none()
                && write_wait.is_none()
            {
                let pixel_command = operation
                    .as_ref()
                    .is_some_and(|op| op.intent.opcode == wire::PIXEL_DIAGNOSTIC)
                    || intents.iter().any(|op| op.opcode == wire::PIXEL_DIAGNOSTIC);
                demand_lease
                    .desired(demand.flags | if pixel_command { usb_demand::PIXELS } else { 0 });
                if let Some(payload) = demand_lease.request(sequence) {
                    if port
                        .write_all(&wire_frame(usb_demand::REQUEST, sequence, &payload))
                        .is_err()
                    {
                        break 'connected;
                    }
                    sequence = sequence.wrapping_add(1);
                }
            }
            let depth = intents.len() + usize::from(operation.is_some());
            if depth != reported_depth {
                reported_depth = depth;
                let _ = events.send(Event::SerialQueueDepth(connected_port.clone(), depth));
                context.request_repaint();
            }
            let mut bytes = [0_u8; 128];
            if api.as_ref().is_some_and(|work| !work.current(physical_session, table_snapshot.as_ref())
                || work.wait.is_some_and(|(_, at)| at.elapsed() >= Duration::from_secs(2))) {
                if let Some(work) = api.take() {
                    if lease.current() && let Some(cleanup) = work.transaction.cleanup() {
                        let _ = port.write_all(&wire_frame(usb_api::OPCODE, sequence, &cleanup));
                        sequence = sequence.wrapping_add(1);
                    }
                    work.fail("USB API timed out or peer retired; outcome unknown, no replay");
                }
            }
            if operation.is_none() && silent.is_none() && table_read.is_none()
                && table_wait.is_none() && write_wait.is_none() && read_wait.is_none()
                && intro_wait.is_none() && expected.is_none()
                && let Some(work) = &mut api && work.wait.is_none() {
                match work.transaction.request() {
                    Ok(payload) if work.current(physical_session, table_snapshot.as_ref()) => {
                        if port.write_all(&wire_frame(usb_api::OPCODE, sequence, &payload)).is_err() { break 'connected; }
                        work.wait = Some((sequence, Instant::now())); sequence = sequence.wrapping_add(1);
                    }
                    _ => { api.take().unwrap().fail("USB API admission expired; no replay"); }
                }
            }
            match port.read(&mut bytes) {
                Ok(n) => {
                    for byte in &bytes[..n] {
                        if *byte == 0 {
                            if let Some((op, response_sequence, payload)) =
                                decode_envelope(&encoded)
                            {
                                if api.as_ref().is_some_and(|work| work.wait.is_some_and(|(seq,_)| seq == response_sequence))
                                    && matches!(op, usb_api::RESPONSE | wire::ACK | wire::ERROR_RESPONSE) {
                                    let work = api.as_mut().unwrap(); work.wait = None;
                                    match work.transaction.accept(op, &payload) {
                                        Ok(Some(value)) => { let work = api.take().unwrap(); let _ = work.reply.send(Ok(value));
                                            paused_table = None; last_table = Instant::now() - Duration::from_secs(2); }
                                        Ok(None) => {}
                                        Err(error) => {
                                            let work = api.take().unwrap();
                                            if let Some(cleanup) = work.transaction.cleanup() {
                                                let _ = port.write_all(&wire_frame(usb_api::OPCODE, sequence, &cleanup)); sequence=sequence.wrapping_add(1);
                                            }
                                            work.fail(error);
                                        }
                                    }
                                    context.request_repaint();
                                }
                                if intro_wait.is_some_and(|(_, seq, _)| seq == response_sequence)
                                    && matches!(op, wire::ACK | wire::HELLO_RESPONSE | wire::ERROR_RESPONSE)
                                {
                                    let (expected, expected_sequence, _) =
                                        intro_wait.take().unwrap();
                                    let result = if expected == wire::HELLO && op == wire::HELLO_RESPONSE {
                                        peer_identity::validate_usb_hello(&payload).and_then(|_| {
                                            if peer_reports.supported() {
                                                intro_chunks = peer_identity::report_chunks(
                                                    &client_telemetry::Reporter::new()
                                                        .value_for_transport("usb"),
                                                )?;
                                            } else {
                                                introduced = true;
                                                last_peer_report = Instant::now();
                                            }
                                            Ok(())
                                        })
                                    } else if expected == peer_identity::USB_REPORT
                                        && op == wire::ACK
                                        && payload.as_slice() == [peer_identity::USB_REPORT, 0]
                                    {
                                        if intro_chunks.is_empty() {
                                            introduced = true;
                                            last_peer_report = Instant::now();
                                        }
                                        Ok(())
                                    } else if peer_reports.observe_unsupported(
                                        (expected, expected_sequence),
                                        (op, response_sequence),
                                        &payload,
                                    ) {
                                        // Explicit build override may omit an unsupported additive
                                        // report, never HELLO/full identity or command fences. Keep
                                        // this capability result only for the current physical handle.
                                        intro_chunks.clear();
                                        introduced = true;
                                        last_peer_report = Instant::now();
                                        Ok(())
                                    } else {
                                        Err(peer_identity::usb_introduction_error(
                                            expected, op, &payload,
                                        ))
                                    };
                                    if let Err(error) = result {
                                        let _ = events.send(Event::SerialFailure(
                                            connected_port.clone(),
                                            error,
                                        ));
                                        port_name.clear();
                                        break 'connected;
                                    }
                                } else if !introduced {
                                    // Stale boot/status data cannot bypass the correlated introduction.
                                } else if op == panel_event::USB_OPCODE {
                                    if table_observed.elapsed() < Duration::from_secs(8)
                                        && let Some(status) = table_snapshot.as_ref()
                                        && let Some(event) =
                                            panel_event::Observation::usb(&payload, status)
                                    {
                                        let _ = events.send(Event::SerialPanelEvent(
                                            connected_port.clone(),
                                            physical_session,
                                            event,
                                        ));
                                    } // No full snapshot/command ACK lease is refreshed by this event.
                                } else if op == wire::PC_CONTRACT_RESPONSE
                                    && matches!(response_sequence, 0 | 0xfe)
                                    && payload.first() == Some(&pc_wire::SEGMENT_CHANGED)
                                {
                                    if table_observed.elapsed() < Duration::from_secs(8)
                                        && let Some(status) = table_snapshot.as_mut()
                                        && let Some(frame) =
                                            peripheral_events::Segments::usb(&payload, status)
                                    {
                                        frame.clone().apply(status);
                                        let _ = events.send(Event::SerialSegments(
                                            connected_port.clone(),
                                            physical_session,
                                            frame,
                                        ));
                                    }
                                } else if (op == wire::PC_CONTRACT_RESPONSE || op == wire::ERROR_RESPONSE)
                                    && silent.as_ref().is_some_and(|(_, _, _, wait)| {
                                        wait.is_some_and(|(s, _)| s == response_sequence)
                                    })
                                {
                                    let (id, _, transaction, wait) = silent.as_mut().unwrap();
                                    *wait = None;
                                    let result = if op == wire::PC_CONTRACT_RESPONSE && !payload.is_empty() {
                                        transaction.accept(payload[0], &payload[1..])
                                    } else {
                                        Err("ESP rejected board settings; refresh before retrying"
                                            .into())
                                    };
                                    match result {
                                        Ok(None) => {}
                                        Ok(Some(settings)) => {
                                            let id = *id;
                                            silent = None;
                                            let _ = events
                                                .send(Event::BoardSilentFinished(id, Ok(settings)));
                                        }
                                        Err(error) => {
                                            let id = *id;
                                            silent = None;
                                            let _ = events
                                                .send(Event::BoardSilentFinished(id, Err(error)));
                                        }
                                    }
                                } else if op == usb_demand::RESPONSE {
                                    let _ = demand_lease.accept(response_sequence, &payload);
                                } else if op == OP_STATUS {
                                    if serial_status(&payload).is_none() {
                                        encoded.clear();
                                        continue;
                                    }
                                    if matches!(response_sequence, 0 | 0xfe) {
                                        last_table = Instant::now() - Duration::from_secs(2);
                                    }
                                    let next_table_supported = payload.get(44) == Some(&1);
                                    if table_supported && !next_table_supported {
                                        table_snapshot = None;
                                        table_read = None;
                                        paused_table = None;
                                        table_wait = None;
                                        table_following.clear();
                                        intents.clear();
                                        if operation.take().is_some() {
                                            let _=events.send(Event::SerialOperation(connected_port.clone(),Err("USB table capability disappeared; outcome unverified, no replay".into())));
                                        }
                                    }
                                    table_supported = next_table_supported;
                                    compact_supported =
                                        table_supported && payload.get(45) == Some(&1);
                                    demand_supported = payload.get(46) == Some(&1);
                                    if !table_supported
                                        && payload.len() >= 38
                                        && payload[31] & (1 << 5) != 0
                                        && packets.is_empty()
                                        && write_wait.is_none()
                                        && read.is_none()
                                        && (stamp != Some(payload[37])
                                            || last_identity.elapsed() > Duration::from_secs(10))
                                    {
                                        if stamp != Some(payload[37]) {
                                            identity = None;
                                        }
                                        stamp = Some(payload[37]);
                                        nonce = nonce.wrapping_add(1);
                                        read = Some(identity::UsbRead {
                                            nonce,
                                            ..Default::default()
                                        });
                                        last_identity = Instant::now();
                                    }
                                    if !table_supported
                                        && identity.is_some()
                                        && let Some(mut status) = serial_status(&payload)
                                    {
                                        let value = identity.as_ref().unwrap();
                                        status.version =
                                            value["version"].as_str().unwrap_or_default().into();
                                        status.compatibility = match serde_json::from_value::<
                                            Option<peer_identity::Policy>,
                                        >(
                                            value["compatibility"].clone(),
                                        ) {
                                            Ok(policy) => policy,
                                            Err(_) => {
                                                let _=events.send(Event::SerialFailure(connected_port.clone(),"Peer compatibility: malformed USB release policy".into()));
                                                port_name.clear();
                                                break 'connected;
                                            }
                                        };
                                        status.board_id =
                                            value["board_id"].as_str().unwrap_or_default().into();
                                        status.boot_id =
                                            value["boot_id"].as_str().unwrap_or_default().into();
                                        if peer_http::Pin::from_identity(
                                            &status.board_id,
                                            &status.boot_id,
                                        )
                                        .is_none()
                                        {
                                            let _=events.send(Event::SerialFailure(connected_port.clone(),"USB board/boot identity missing or invalid; explicit reconnect after board recovery".into()));
                                            port_name.clear();
                                            break 'connected;
                                        }
                                        if let Err(error) = peer_identity::validate_board(&status) {
                                            let _ = events.send(Event::SerialFailure(
                                                connected_port.clone(),
                                                error,
                                            ));
                                            port_name.clear();
                                            break 'connected;
                                        }
                                        if let (Some(value), Some(c)) =
                                            (&identity, &mut status.capabilities)
                                        {
                                            c.name = value["display_name"]
                                                .as_str()
                                                .unwrap_or_default()
                                                .into();
                                            c.hostname = value["hostname"]
                                                .as_str()
                                                .unwrap_or_default()
                                                .into();
                                            c.application = serde_json::from_value(
                                                value["application"].clone(),
                                            )
                                            .ok();
                                            status.hostname = c.hostname.clone();
                                        }
                                        let _ = events.send(Event::Serial(
                                            connected_port.clone(),
                                            true,
                                            Some(status),
                                        ));
                                    }
                                } else if op == usb_table::RESPONSE
                                    && table_wait.is_some_and(|(s, _)| s == response_sequence)
                                {
                                    table_wait = table_following.pop_front();
                                    match table_read.as_mut().unwrap().accept(&payload) {
                                        Ok(Some(mut snapshot)) => {
                                            snapshot.retain_icon(table_snapshot.as_ref());
                                            if let Err(error) =
                                                peer_identity::validate_board(&snapshot.status)
                                            {
                                                let _ = events.send(Event::SerialFailure(
                                                    connected_port.clone(),
                                                    error,
                                                ));
                                                port_name.clear();
                                                break 'connected;
                                            }
                                            reintroductions = 0;
                                            let _ = events.send(Event::SerialCapture(
                                                connected_port.clone(),
                                                table_read.as_ref().unwrap().progress("complete"),
                                            ));
                                            table_read = None;
                                            table_observed = snapshot.observed_at;
                                            if let Some((packet, at)) = &last_command_state
                                                && *at > snapshot.observed_at
                                                && peer_http::Pin::from_status(&snapshot.status)
                                                    == table_snapshot
                                                        .as_ref()
                                                        .and_then(peer_http::Pin::from_status)
                                            {
                                                // Background data may predate control. Never roll cells back
                                                // or present its old RGB as the newly applied pixel frame.
                                                if let Ok(status) = usb_table::command_state(
                                                    &snapshot.status,
                                                    packet[0],
                                                    packet,
                                                ) {
                                                    snapshot.status = status;
                                                }
                                            }
                                            let pin = peer_http::Pin::from_status(&snapshot.status);
                                            let backend_pending = operation.as_ref().is_some_and(|active| {
                                                active.ack.is_some() && active.sent.elapsed() < Duration::from_secs(4)
                                                    && active.intent.admission.peer == pin
                                                    && active.intent.opcode == OP_LED_OUTPUT
                                                    && matches!(active.intent.payload.as_slice(), [3, _])
                                                    && led_backend::name(active.intent.payload[1])
                                                        .is_some_and(|backend| led_backend::pending(&snapshot.status, backend))
                                            });
                                            if let Some(active) =
                                                operation.as_ref().filter(|op| op.ack.is_some() && !backend_pending)
                                            {
                                                let intent = &active.intent;
                                                let pixel_revision=intent.opcode!=wire::PIXEL_DIAGNOSTIC || (snapshot.status.led_output["diagnostic"]["revision"].as_u64().unwrap_or(0)>table_snapshot.as_ref().and_then(|s|s.led_output["diagnostic"]["revision"].as_u64()).unwrap_or(0)
                                                && snapshot.status.pixel_frame.as_ref().is_some_and(|frame|frame.revision>table_snapshot.as_ref().and_then(|s|s.pixel_frame.as_ref()).map_or(0,|f|f.revision)));
                                                let result = if intent.admission.peer == pin
                                                    && intent.verify(&snapshot.status)
                                                    && pixel_revision
                                                {
                                                    Ok("USB command ACK and same-board snapshot readback verified".into())
                                                } else {
                                                    Err("USB ACK received but board/boot or applied values differ; no replay attempted".into())
                                                };
                                                if intent.opcode == cell_batch::OPCODE
                                                    && result.is_ok()
                                                {
                                                    if let Some(correlation) =
                                                        intent.admission.batch.as_ref()
                                                    {
                                                        let _ = events.send(
                                                            Event::SerialBatchConfirmed(
                                                                connected_port.clone(),
                                                                correlation.clone(),
                                                            ),
                                                        );
                                                    }
                                                }
                                                active.intent.reply(
                                                    result
                                                        .as_ref()
                                                        .map(|_| snapshot.status.clone())
                                                        .map_err(Clone::clone),
                                                );
                                                if result.is_ok() {
                                                    let _ = events.send(Event::SerialTiming(
                                                        connected_port.clone(),
                                                        active.timing(
                                                            physical_session,
                                                            "full snapshot",
                                                        ),
                                                    ));
                                                }
                                                let _ = events.send(Event::SerialOperation(
                                                    connected_port.clone(),
                                                    result,
                                                ));
                                                operation = None;
                                            }
                                            if backend_pending {
                                                // Re-observe only. An accepted backend request is not
                                                // ready/persisted success and must never be resent.
                                                last_table = Instant::now() - Duration::from_secs(1);
                                            }
                                            table_snapshot = Some(snapshot.status.clone());
                                            let _ = events.send(Event::SerialSnapshot(
                                                connected_port.clone(),
                                                snapshot,
                                            ));
                                        }
                                        Ok(None) => {}
                                        Err(error) => {
                                            table_read = None;
                                            table_wait = None;
                                            table_following.clear();
                                            last_table = Instant::now();
                                            let _ = events.send(Event::SerialFailure(
                                                connected_port.clone(),
                                                error.clone(),
                                            ));
                                            if let Some(active) = operation.take() {
                                                active.intent.reply(Err(error.clone()));
                                                let _ = events.send(Event::SerialOperation(
                                                    connected_port.clone(),
                                                    Err(error),
                                                ));
                                            }
                                        }
                                    }
                                } else if op == usb_table::COMMAND_STATE
                                    && compact_supported
                                    && operation.as_ref().is_some_and(|active| {
                                        !active.intent.needs_full_snapshot()
                                            && active.sequence == response_sequence
                                            && active.ack.is_some()
                                            && payload.first() == Some(&active.intent.opcode)
                                    })
                                {
                                    let active = operation.as_ref().unwrap();
                                    let result=table_snapshot.as_ref().ok_or_else(||"USB table owner unavailable".to_string())
                                    .and_then(|status|usb_table::command_state(status,active.intent.opcode,&payload))
                                    .and_then(|status|if active.intent.admission.peer==peer_http::Pin::from_status(&status) && active.intent.verify(&status){Ok(status)}else{Err("USB ACK readback differs from this intent/peer; no replay".into())});
                                    match result {
                                        Ok(status) => {
                                            // Do not renew capability/peripheral freshness using this partial readback.
                                            table_snapshot = Some(status.clone());
                                            table_read = paused_table.take();
                                            table_wait = None;
                                            table_following.clear();
                                            last_table = Instant::now();
                                            last_command_state =
                                                Some((payload.clone(), Instant::now()));
                                            let _ = events.send(Event::SerialCommandState(
                                                connected_port.clone(),
                                                status.clone(),
                                            ));
                                            active.intent.reply(Ok(status));
                                            if let Some(correlation) =
                                                active.intent.admission.batch.as_ref()
                                            {
                                                let _ = events.send(Event::SerialBatchConfirmed(
                                                    connected_port.clone(),
                                                    correlation.clone(),
                                                ));
                                            }
                                            let timing = active
                                                .timing(physical_session, "compact board readback");
                                            let message = format!(
                                                "USB ACK + board readback · {:.1} ms (queue {:.1} ms)",
                                                timing.wire_confirm_ms, timing.queue_ms
                                            );
                                            let _ = events.send(Event::SerialTiming(
                                                connected_port.clone(),
                                                timing,
                                            ));
                                            let _ = events.send(Event::SerialOperation(
                                                connected_port.clone(),
                                                Ok(message),
                                            ));
                                        }
                                        Err(error) => {
                                            active.intent.reply(Err(error.clone()));
                                            table_read = paused_table.take();
                                            let _ = events.send(Event::SerialOperation(
                                                connected_port.clone(),
                                                Err(error),
                                            ));
                                        }
                                    }
                                    operation = None;
                                } else if operation.as_ref().is_some_and(|active| {
                                    active.matches(op, response_sequence, &payload)
                                }) {
                                    if op == wire::ACK && payload[1] == 0 {
                                        if usb_table::table_command(
                                            operation.as_ref().unwrap().intent.opcode,
                                        ) {
                                            operation.as_mut().unwrap().ack = Some(Instant::now());
                                            table_read = None;
                                            table_wait = None;
                                            table_following.clear();
                                            last_table = Instant::now() - Duration::from_secs(2);
                                        } else {
                                            operation = None;
                                            table_read = paused_table.take();
                                            let _=events.send(Event::SerialOperation(connected_port.clone(),Ok("USB device ACK received; no table state inferred".into())));
                                        }
                                    } else {
                                        if let Some(active) = operation.take() {
                                            active.intent.reply(Err(
                                                cell_assignments::usb_rejection(payload[1]),
                                            ));
                                        }
                                        table_read = paused_table.take();
                                        let _ = events.send(Event::SerialOperation(
                                            connected_port.clone(),
                                            Err(cell_assignments::usb_rejection(payload[1])),
                                        ));
                                    }
                                } else if op == wire::ERROR_RESPONSE
                                    && payload.first() == Some(&usb_table::READ)
                                    && table_wait.is_some_and(|(s, _)| s == response_sequence)
                                {
                                    table_read = None;
                                    table_wait = None;
                                    table_following.clear();
                                    last_table = Instant::now();
                                    if payload.get(1) == Some(&2) && reintroductions < 2 {
                                        // Firmware reboot can retain the USB-UART handle while
                                        // losing caller admission. Reintroduce metadata only;
                                        // retire every uncertain/unsent mutation, never replay it.
                                        reintroductions += 1;
                                        introduced = false;
                                        table_supported = false;
                                        compact_supported = false;
                                        demand_supported = false;
                                        demand_lease = Default::default();
                                        table_snapshot = None;
                                        paused_table = None;
                                        last_command_state = None;
                                        intro_chunks.clear();
                                        if let Some(active) = operation.take() {
                                            active.intent.reply(Err("USB board restarted; command outcome unknown, no replay".into()));
                                            let _=events.send(Event::SerialOperation(connected_port.clone(),Err("USB board restarted; command outcome unknown, no replay".into())));
                                        }
                                        for intent in intents.drain(..) {
                                            intent.reply(Err("USB board admission changed; queued command discarded, no replay".into()));
                                        }
                                        let _ = events.send(Event::Serial(
                                            connected_port.clone(),
                                            false,
                                            None,
                                        ));
                                        if port
                                            .write_all(&wire_frame(
                                                wire::HELLO,
                                                sequence,
                                                &peer_identity::hello(),
                                            ))
                                            .is_err()
                                        {
                                            break 'connected;
                                        }
                                        intro_wait = Some((wire::HELLO, sequence, Instant::now()));
                                        sequence = sequence.wrapping_add(1);
                                        encoded.clear();
                                        continue 'connected;
                                    }
                                    if payload.get(1) == Some(&2) {
                                        let _=events.send(Event::SerialFailure(connected_port.clone(),"Peer compatibility: USB admission repeatedly rejected after introduction; update/reconnect explicitly".into()));
                                        port_name.clear();
                                        break 'connected;
                                    }
                                    let error =
                                        "Board rejected USB table snapshot; no state assumed"
                                            .to_string();
                                    let _ = events.send(Event::SerialFailure(
                                        connected_port.clone(),
                                        error.clone(),
                                    ));
                                    if operation.take().is_some() {
                                        let _ = events.send(Event::SerialOperation(
                                            connected_port.clone(),
                                            Err(error),
                                        ));
                                    }
                                } else if op == wire::IDENTITY_RESPONSE
                                    && read_wait.is_some_and(|(s, _)| s == response_sequence)
                                {
                                    read_wait = None;
                                    let result = read.as_mut().unwrap().accept(&payload);
                                    match result {
                                        Ok(Some(value)) => {
                                            read = None;
                                            if let Some((host, label)) = expected.take() {
                                                let valid = value["hostname"].as_str()
                                                    == Some(host.as_str())
                                                    && value["display_name"].as_str()
                                                        == Some(label.as_str());
                                                let _=events.send(Event::SerialIdentitySaved(connected_port.clone(),if valid {Ok(())} else {Err("Identity ACK received but readback differs; no replay attempted".into())}));
                                            }
                                            let _ = events.send(Event::SerialIdentity(
                                                connected_port.clone(),
                                                value.clone(),
                                            ));
                                            identity = Some(value);
                                        }
                                        Ok(None) => {}
                                        Err(error) => {
                                            read = None;
                                            identity = None;
                                            if expected.take().is_some() {
                                                let _ = events.send(Event::SerialIdentitySaved(
                                                    connected_port.clone(),
                                                    Err(error),
                                                ));
                                            }
                                        }
                                    }
                                } else if write_wait.is_some_and(|(s, _)| s == response_sequence)
                                    && (op == wire::ACK || op == wire::ERROR_RESPONSE)
                                    && payload.first() == Some(&wire::DEVICE_NAME)
                                {
                                    write_wait = None;
                                    if op == wire::ERROR_RESPONSE {
                                        packets.clear();
                                        expected = None;
                                        let _=events.send(Event::SerialIdentitySaved(connected_port.clone(),Err(format!("Board rejected identity write ({:?}); no replay attempted",payload.get(1)))));
                                    } else if packets.is_empty() {
                                        identity = None;
                                        nonce = nonce.wrapping_add(1);
                                        read = Some(identity::UsbRead {
                                            nonce,
                                            ..Default::default()
                                        });
                                        last_identity = Instant::now();
                                    }
                                }
                                context.request_repaint();
                            }
                            encoded.clear();
                        } else if encoded.len() < 80 {
                            encoded.push(*byte)
                        } else {
                            encoded.clear()
                        }
                    }
                }
                Err(e) if matches!(e.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock) => {}
                Err(error) => {
                    let _ = events.send(Event::SerialFailure(
                        connected_port.clone(),
                        error.to_string(),
                    ));
                    break;
                }
            }
        }
        if introduced && demand_supported {
            let _ = port.write_all(&wire_frame(usb_demand::REQUEST, sequence, &[0, 0, 0, 0]));
            sequence = sequence.wrapping_add(1);
        }
        drop(port); // Release the real handle before the next method may acquire.
        if let Some(work) = api { work.fail("USB API session ended; outcome unknown, no replay"); }
        if let Some(active) = operation.as_ref() {
            active.intent.reply(Err(
                "USB session ended; command outcome unverified, no replay".into(),
            ));
        }
        for intent in &intents {
            intent.reply(Err("USB session ended; queued command not replayed".into()));
        }
        if let Some((id, ..)) = silent {
            let _ = events.send(Event::BoardSilentFinished(
                id,
                Err("USB disconnected during settings; outcome unknown, no replay".into()),
            ));
        }
        let _ = events.send(Event::SerialQueueDepth(connected_port.clone(), 0));
        if operation.is_some() || !intents.is_empty() {
            let _=events.send(Event::SerialOperation(connected_port.clone(),Err("USB session ended; pending commands were not replayed. Inspect device state before retrying".into())));
        }
        if expected.is_some() {
            let _ = events.send(Event::SerialIdentitySaved(
                connected_port.clone(),
                Err(
                    "USB disconnected during identity write; reconnect and read before retrying"
                        .into(),
                ),
            ));
        }
        let _ = events.send(Event::Serial(connected_port, false, None));
        context.request_repaint();
    }
}
// A selected method never falls back to another transport, including on error.
fn command_route(selected: Transport, wifi_fresh: bool, usb_fresh: bool) -> Option<Transport> {
    match selected {
        Transport::WiFi if wifi_fresh => Some(Transport::WiFi),
        Transport::Serial if usb_fresh => Some(Transport::Serial),
        _ => None,
    }
}
struct PakarControllerApp {
    firmware_feed: Option<firmware_feed::Session>,
    firmware_transfer: Option<firmware_progress::Transfer>,
    firmware_feed_message: String,
    bridge: bridge::Runtime,
    connection_gate: connection::Gate,
    connection_enabled: bool,
    active_context: Option<String>,
    config: config::AppConfig,
    workspace: native_workspace::Runtime,
    preferences: Option<ui::preferences::PreferencesDraft>,
    board_silent: board_silent::State,
    device_settings: device_settings::Editor,
    control: platform::instance::Control,
    dock: egui_dock::DockState<ui::layout::Panel>,
    reveal_panels: Vec<ui::layout::Panel>,
    close_panels: Vec<ui::layout::Panel>,
    open_panels: Vec<ui::layout::Panel>,
    show_connection: bool,
    show_mapping: bool,
    show_workspaces: bool,
    show_about: bool,
    confirm_reset: bool,
    exiting: bool,
    usb_ui_visible: bool,
    usb_ui_demand: (usb_demand::Demand, Option<Instant>),
    usb_sent_demand: Option<(String, u64, usb_demand::Demand)>,
    ws_pixel_demand: Option<(u64, bool)>,
    workspace_name: String,
    workspace_icon: String,
    selected_cell: u8,
    cell_search: String,
    table: ui::cell_table::TableState,
    pixel_diagnostic: pixel_diagnostic::State,
    pixel_corrections: pixel_corrections::Editor,
    pixel_groups: pixel_groups::Editor,
    pixel_api_reply: Option<(peer_http::Pin, pixel_groups::Purpose, Receiver<Result<serde_json::Value, String>>)>,
    pixel_usb_reply: Option<(
        u64,
        Receiver<Result<DeviceStatus, String>>,
        std::sync::Arc<std::sync::atomic::AtomicBool>,
    )>,
    capture: ui::screenshot::State,
    screenshot_harness: bool,
    window_startup: ui::window_geometry::Startup,
    focus_cell_search: bool,
    cell_effect: String,
    event_log: diagnostics::EventLog,
    panel_events: panel_event::State,
    peripheral_events: peripheral_events::State,
    status_request_pending: bool,
    status_retry: diagnostics::Backoff,
    last_status_at: Option<Instant>,
    last_serial_status_at: Option<Instant>,
    presentation: capabilities::Presentation,
    configuration_loaded: bool,
    configuration_reads: configuration_reads::Queue,
    tray: Option<platform::windows_tray::TrayController>,
    notifications: std::cell::RefCell<platform::notifications::Delivery>,
    connection_notice: platform::notifications::ConnectionNotice,
    winner_notice: winner_notice::WinnerNotice,
    #[cfg(windows)]
    windows_desktop: Option<platform::windows_desktop::Desktop>,
    base: String,
    user: String,
    password: String,
    access_user: String,
    access_password: String,
    access_confirm: String,
    access_info: Option<serde_json::Value>,
    access_enabled: bool,
    // Public requirement metadata survives Disconnect so a protected peer's
    // credentials can be edited offline. It never grants operational access.
    access_requirement: Option<(String, bool)>,
    serial_port: String,
    transport: Transport,
    status: Option<DeviceStatus>,
    serial_status: Option<DeviceStatus>,
    serial_operations_pending: usize,
    serial_timing: Option<usb_table::Timing>,
    serial_capture: Option<usb_table::CaptureProgress>,
    serial_session: u64,
    serial_stamp: Option<connection::Stamp>,
    brightness: u8,
    pattern: String,
    display_value: i16,
    display_text: String,
    buzzer_frequency: u16,
    buzzer_duration: u16,
    key_layout_edit: bool,
    key_selection: ui::cell_table::Selection,
    key_trigger_on_click: bool,
    key_positions: [[f32; 2]; 9],
    output_mask: u8,
    device_name: Option<String>,
    board_display_name: Option<String>,
    identity_pending: bool,
    identity_saved: Option<(String, String)>,
    message: String,
    http_tx: peer_http::Queue,
    ws_tx: Sender<WsCommand>,
    serial_tx: Sender<SerialCommand>,
    event_rx: Receiver<Event>,
    last_poll: Instant,
    last_ws_endpoint: Option<Endpoint>,
    last_serial_port: String,
    websocket_live: bool,
    serial_live: bool,
    streaming: bool,
    sampling: bool,
    stream_sent_fps: Option<(f64, Instant)>,
    palette: ui::palette::Palette,
    mappings: Vec<CellMapping>,
    mapping_logical: u8,
    mapping_physical: u8,
    mapping_name: String,
    last_mapping_poll: Instant,
    mapping_request_pending: Option<MappingPending>,
    mapping_request_id: u64,
    mapping_peer: Option<peer_http::Pin>,
    mapping_error: Option<String>,
    mapping_retry: diagnostics::Backoff,
}

impl Drop for PakarControllerApp {
    fn drop(&mut self) {
        // The sole owner sends the final zero lease before closing its handle.
        // A crashed process is bounded by the firmware's 3s expiry instead.
        let _ = self.serial_tx.send(SerialCommand::Port(
            String::new(),
            self.connection_gate.epoch(),
        ));
    }
}

impl PakarControllerApp {
    fn new(context: &eframe::CreationContext<'_>) -> Self {
        let config = config::AppConfig::load().unwrap_or_else(|error| {
            cli_eprintln!("{error}");
            config::AppConfig::default()
        });
        // apply resolves system/override locale and owns the initial font install.
        config::apply(&context.egui_ctx, &config);
        let dock = ui::layout::restore(&config.dock_layout);
        let screenshot_harness = std::env::var_os("PAKAR_SCREENSHOT_PATH").is_some()
            && std::env::var_os("PAKAR_CONFIG_FILE").is_some()
            && !config.single_instance;
        let (http_tx, http_rx) = mpsc::channel();
        let (ws_tx, ws_rx) = mpsc::channel();
        let (serial_tx, serial_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();
        let connection_gate = connection::Gate::default();
        let startup_enabled = config.startup_connection_enabled(screenshot_harness);
        if !screenshot_harness && let Some(route) = &config.last_network_route {
            peer_http::restore_route(route, &config.endpoint);
        }
        connection_gate.select(startup_enabled.then_some(config.connection_method));
        let ctx = context.egui_ctx.clone();
        let events = event_tx.clone();
        let gate = connection_gate.clone();
        thread::Builder::new()
            .name("pakar-http".into())
            .spawn(move || http_worker(http_rx, events, ctx, gate))
            .unwrap();
        let ctx = context.egui_ctx.clone();
        let events = event_tx.clone();
        let gate = connection_gate.clone();
        thread::Builder::new()
            .name("pakar-websocket".into())
            .spawn(move || ws_worker(ws_rx, events, ctx, gate))
            .unwrap();
        let ctx = context.egui_ctx.clone();
        let gate = connection_gate.clone();
        thread::Builder::new()
            .name("pakar-serial".into())
            .spawn(move || serial_worker(serial_rx, event_tx, ctx, gate))
            .unwrap();
        let tray = if screenshot_harness {
            Ok(None)
        } else {
            platform::windows_tray::TrayController::new().map(Some)
        };
        let mut event_log = diagnostics::EventLog::default();
        if let Err(error) = &tray {
            event_log.record("Windows tray unavailable", error);
        }
        #[cfg(windows)]
        let windows_desktop = if screenshot_harness {
            Ok(None)
        } else {
            platform::windows_desktop::Desktop::new(context).map(Some)
        };
        #[cfg(windows)]
        if let Err(error) = &windows_desktop {
            event_log.record("Windows desktop unavailable", error);
        }
        Self {
            firmware_feed: None,
            firmware_transfer: None,
            firmware_feed_message: String::new(),
            bridge: Default::default(),
            connection_gate,
            connection_enabled: startup_enabled,
            active_context: None,
            workspace: Default::default(),
            control: platform::instance::start(context.egui_ctx.clone(), !screenshot_harness),
            dock,
            reveal_panels: Vec::new(),
            close_panels: Vec::new(),
            open_panels: Vec::new(),
            preferences: None,
            board_silent: Default::default(),
            show_connection: false,
            show_mapping: false,
            device_settings: device_settings::Editor::default(),
            show_workspaces: false,
            show_about: false,
            confirm_reset: false,
            exiting: false,
            usb_ui_visible: true,
            usb_ui_demand: (Default::default(), None),
            usb_sent_demand: None,
            ws_pixel_demand: None,
            workspace_name: String::new(),
            workspace_icon: "hardware".into(),
            selected_cell: 1,
            cell_search: String::new(),
            table: Default::default(),
            pixel_diagnostic: Default::default(),
            pixel_corrections: Default::default(),
            pixel_groups: Default::default(),
            pixel_api_reply: None,
            pixel_usb_reply: None,
            capture: ui::screenshot::State::from_environment(),
            screenshot_harness,
            window_startup: Default::default(),
            focus_cell_search: false,
            cell_effect: "fade".into(),
            event_log,
            panel_events: Default::default(),
            peripheral_events: Default::default(),
            status_request_pending: false,
            status_retry: diagnostics::Backoff::new(
                Duration::from_secs(2),
                Duration::from_secs(30),
            ),
            last_status_at: None,
            last_serial_status_at: None,
            presentation: Default::default(),
            configuration_loaded: false,
            configuration_reads: Default::default(),
            tray: tray.ok().flatten(),
            notifications: Default::default(),
            connection_notice: Default::default(),
            winner_notice: Default::default(),
            #[cfg(windows)]
            windows_desktop: windows_desktop.ok().flatten(),
            base: std::env::var("PAKAR_ENDPOINT").unwrap_or_else(|_| config.endpoint.clone()),
            user: std::env::var("PAKAR_USER").unwrap_or_else(|_| config.user.clone()),
            password: std::env::var("PAKAR_PASSWORD").unwrap_or_default(),
            access_user: String::new(),
            access_password: String::new(),
            access_confirm: String::new(),
            access_info: None,
            access_enabled: false,
            access_requirement: None,
            serial_port: std::env::var("PAKAR_SERIAL")
                .unwrap_or_else(|_| config.serial_port.clone()),
            transport: config.connection_method,
            status: None,
            serial_status: None,
            serial_operations_pending: 0,
            serial_timing: None,
            serial_capture: None,
            serial_session: 0,
            serial_stamp: None,
            brightness: 255,
            pattern: "none".into(),
            display_value: 1,
            display_text: "PAKAR CONTROLLER".into(),
            buzzer_frequency: 1032,
            buzzer_duration: 100,
            key_layout_edit: false,
            key_selection: Default::default(),
            key_trigger_on_click: false,
            key_positions: native_workspace::default_key_positions(),
            output_mask: 0,
            device_name: None,
            board_display_name: None,
            identity_pending: false,
            identity_saved: None,
            message: startup_connection_message(startup_enabled, config.connection_method).into(),
            http_tx: peer_http::Queue::new(http_tx),
            ws_tx,
            serial_tx,
            event_rx,
            last_poll: Instant::now() - Duration::from_secs(2),
            last_ws_endpoint: None,
            last_serial_port: String::new(),
            websocket_live: false,
            serial_live: false,
            streaming: false,
            sampling: false,
            stream_sent_fps: None,
            palette: ui::palette::Palette::Studio,
            mappings: Vec::new(),
            mapping_logical: 1,
            mapping_physical: 73,
            mapping_name: "1".into(),
            last_mapping_poll: Instant::now() - Duration::from_secs(30),
            mapping_request_pending: None,
            mapping_request_id: 0,
            mapping_peer: None,
            mapping_error: None,
            mapping_retry: diagnostics::Backoff::new(
                Duration::from_secs(2),
                Duration::from_secs(15),
            ),
            config,
        }
    }
    fn endpoint(&self) -> Endpoint {
        Endpoint {
            base: self.base.clone(),
            user: self.user.clone(),
            password: self.password.clone(),
        }
    }
    fn http_writable(&self) -> bool {
        self.command_route() == Some(Transport::WiFi)
            && self.http_tx.writable()
            && self
                .status
                .as_ref()
                .and_then(peer_http::Pin::from_status)
                .is_some()
    }
    fn revoke_http_drafts(&mut self) {
        self.board_silent.revoke();
        self.status_request_pending = false;
        self.device_settings.invalidate_peer();
        self.configuration_loaded = false;
        self.table.stop_pending_edits();
        self.table.editing_order = false;
        self.table.manager_open = false;
        self.table.pixel_sample = None;
        self.pixel_diagnostic.revoke();
        self.confirm_reset = false;
        self.show_mapping = false;
        self.mappings.clear();
        self.mapping_name.clear();
        self.mapping_logical = 1;
        self.mapping_physical = 1;
        self.access_info = None;
        self.access_user.clear();
        self.access_password.clear();
        self.access_confirm.clear();
        self.device_name = None;
        self.board_display_name = None;
        self.identity_pending = false;
        self.identity_saved = None;
        self.streaming = false;
        self.sampling = false;
        self.stream_sent_fps = None;
        self.last_mapping_poll = Instant::now() - Duration::from_secs(16);
        self.mapping_request_pending = None;
        self.mapping_peer = None;
        self.mapping_error = None;
        self.mapping_retry.reset();
        let _ = self.ws_tx.send(WsCommand::Mapping(Vec::new()));
        let _ = self.ws_tx.send(WsCommand::Stream(false));
        let _ = self.ws_tx.send(WsCommand::Sample(false));
    }
    fn reject_incompatible_peer(&mut self, error: &str) {
        // Fail closed immediately, but preserve private drafts and the saved
        // workspace. They must never be thrown away merely to stop retries.
        self.connection_enabled = false;
        self.config.connection_enabled = false;
        self.connection_gate.select(None);
        self.http_tx.clear();
        self.last_status_at = None;
        self.last_serial_status_at = None;
        self.websocket_live = false;
        self.serial_live = false;
        self.streaming = false;
        self.sampling = false;
        self.table.stop_pending_edits();
        self.pixel_diagnostic.revoke();
        let _ = self.ws_tx.send(WsCommand::Disconnect);
        let _ = self.serial_tx.send(SerialCommand::Port(
            String::new(),
            self.connection_gate.epoch(),
        ));
        self.message = error.into();
    }
    fn request_board_silent(&mut self, desired: Option<bool>) {
        if self.live_status().is_some_and(board_silent::supported) {
            self.request_board_settings(desired.into());
        }
    }
    fn request_board_settings(&mut self, desired: board_silent::Change) -> Option<u64> {
        let status = self.live_status()?;
        if !desired.allowed(status) {
            return None;
        }
        let peer = peer_http::Pin::from_status(status)?;
        let route = self.command_route()?;
        let id = self.board_silent.begin(peer.clone(), route)?;
        let sent = match route {
            Transport::WiFi => self
                .http_tx
                .send(HttpCommand::BoardSilent(self.endpoint(), id, desired))
                .is_ok(),
            Transport::Serial => self.serial_admission().is_some_and(|admission| {
                self.serial_tx
                    .send(SerialCommand::BoardSilent(
                        self.serial_port.clone(),
                        admission,
                        id,
                        desired,
                    ))
                    .is_ok()
            }),
        };
        if !sent {
            self.board_silent.finish(
                id,
                Some(&peer),
                Some(route),
                Err("Settings worker unavailable; nothing sent".into()),
            );
        } else if !matches!(desired, board_silent::Change::Read) {
            self.board_silent.message = "Applying to board, then verifying EEPROM…".into();
        }
        sent.then_some(id)
    }
    fn post(&mut self, path: &'static str, form: Vec<(String, String)>) {
        if matches!(
            path,
            "/api/cells/assignments" | "/api/cells/lock" | "/api/game/settings" | "/api/game/press"
        ) && self.transport == Transport::Serial
        {
            let Some(status) = self.live_status() else {
                self.message = "No fresh selected USB board".into();
                return;
            };
            match bridge::usb_request(path, &form, status).and_then(|(opcode, payload, _)| {
                let admission = self
                    .serial_admission()
                    .ok_or("USB command owner unavailable")?;
                self.serial_tx
                    .send(SerialCommand::Send(
                        self.serial_port.clone(),
                        admission,
                        opcode,
                        payload,
                    ))
                    .map_err(|_| "USB worker unavailable".into())
            }) {
                Ok(()) => self.message = "Command queued; awaiting board ACK and readback".into(),
                Err(e) => self.message = e,
            }
            return;
        }
        if !self.http_writable() || !self.peer_context().is_some_and(|c| c.allows(path, &form)) {
            self.message = "This connected board has not acknowledged this capability".into();
            return;
        }
        if self.transport != Transport::Serial {
            if self
                .status
                .as_ref()
                .is_some_and(|s| realtime::supported(s, path))
            {
                if let Err(error) = self.queue_realtime(path, form, None) {
                    self.message = error;
                }
                return;
            }
            let _ = self
                .http_tx
                .send(HttpCommand::Post(self.endpoint(), path, form));
        }
    }
    fn queue_realtime(
        &self,
        path: &'static str,
        form: Vec<(String, String)>,
        batch: Option<cell_batch::Correlation>,
    ) -> Result<(), String> {
        let status = self.live_status().ok_or("No fresh board status")?;
        let request = realtime::Intent::new(
            self.endpoint(),
            status,
            path,
            form,
            self.http_tx.capture(),
            self.connection_gate
                .stamp(Transport::WiFi, self.connection_gate.epoch()),
            batch,
        )?;
        self.ws_tx
            .send(WsCommand::Control(request))
            .map_err(|_| "Realtime worker closed; nothing sent".into())
    }
    fn serial_admission(&self) -> Option<usb_table::Admission> {
        let stamp = self.serial_stamp.clone()?;
        (self.serial_session != 0 && stamp.current()).then(|| usb_table::Admission {
            stamp,
            session: self.serial_session,
            peer: self
                .serial_status
                .as_ref()
                .and_then(peer_http::Pin::from_status),
            at: Instant::now(),
            batch: None,
        })
    }
    fn dispatch_cell_batch(
        &mut self,
        batch: cell_batch::Request,
    ) -> Result<cell_batch::Correlation, String> {
        let route = self
            .command_route()
            .ok_or("No selected live transport for atomic batch")?;
        let status = self
            .live_status()
            .ok_or("No current board status for atomic batch")?;
        if !cell_batch::Request::supported(status) {
            return Err("Atomic cell-batch capability is unavailable; nothing queued".into());
        }
        let peer = peer_http::Pin::from_status(status)
            .ok_or("Atomic batch requires a verified current peer")?;
        let correlation = cell_batch::Correlation::new(batch, peer, route)?;
        match route {
            Transport::WiFi => {
                if realtime::supported(status, "/api/cells") {
                    self.queue_realtime(
                        "/api/cells",
                        correlation.request.form(),
                        Some(correlation.clone()),
                    )?;
                } else {
                    self.http_tx
                        .send(HttpCommand::CellBatch(self.endpoint(), correlation.clone()))
                        .map_err(|_| "Atomic batch queue closed; nothing sent".to_string())?;
                }
            }
            Transport::Serial => {
                let mut admission = self
                    .serial_admission()
                    .ok_or("USB owner/session is not verified")?;
                admission.batch = Some(correlation.clone());
                self.serial_tx
                    .send(SerialCommand::Send(
                        self.serial_port.clone(),
                        admission,
                        cell_batch::OPCODE,
                        correlation.request.payload(),
                    ))
                    .map_err(|_| "USB batch queue closed; nothing sent".to_string())?;
            }
        }
        Ok(correlation)
    }
    fn serial(&mut self, op: u8, payload: Vec<u8>) {
        if self.connection_enabled && self.command_route() == Some(Transport::Serial) {
            if let Some(admission) = self.serial_admission() {
                let _ = self.serial_tx.send(SerialCommand::Send(
                    self.serial_port.clone(),
                    admission,
                    op,
                    payload,
                ));
            } else {
                self.message = "USB owner/session is not verified; nothing queued".into();
            }
        }
    }
    fn action(
        &mut self,
        path: &'static str,
        form: Vec<(String, String)>,
        op: u8,
        payload: Vec<u8>,
    ) {
        if !self.peer_context().is_some_and(|c| c.allows(path, &form)) {
            self.message = "This connected board has not acknowledged this capability".into();
            return;
        }
        match self.command_route() {
            Some(Transport::WiFi) => self.post(path, form),
            Some(Transport::Serial) => self.serial(op, payload),
            _ => self.notify(
                "Device offline",
                "No fresh status from the selected transport. Reconnect before sending a command.",
                messaging::Severity::Warning,
            ),
        }
    }
    fn command_route(&self) -> Option<Transport> {
        if !self.connection_enabled {
            return None;
        }
        command_route(
            self.transport,
            self.status.is_some()
                && self.last_ws_endpoint.as_ref() == Some(&self.endpoint())
                && self
                    .last_status_at
                    .is_some_and(|at| at.elapsed() < Duration::from_secs(8)),
            self.serial_live
                && self.last_serial_port == self.serial_port
                && self.serial_status.is_some()
                && self
                    .last_serial_status_at
                    .is_some_and(|at| at.elapsed() < Duration::from_secs(8)),
        )
    }
    fn provision_usb_network(
        &mut self,
        config: &device_settings::Network,
        password: &str,
        open: bool,
    ) -> Result<(), String> {
        if self.command_route() != Some(Transport::Serial) {
            return Err("Reconnect to the selected USB peer before provisioning".into());
        }
        for payload in device_settings::usb_payloads(config, password, open)? {
            self.serial(OP_NETWORK_CONFIG, payload);
        }
        self.message = "USB provisioning queued; this serial protocol has no saved-settings readback. Verify via authenticated HTTP after reconnecting.".into();
        Ok(())
    }
    fn restart_connection(&mut self) {
        self.connection_gate
            .select(self.connection_enabled.then_some(self.transport));
        self.http_tx.clear();
        self.panel_events.clear();
        self.peripheral_events.clear();
        self.revoke_http_drafts();
        self.status = None;
        self.serial_status = None;
        self.last_status_at = None;
        self.last_serial_status_at = None;
        self.websocket_live = false;
        self.serial_live = false;
        self.serial_timing = None;
        self.serial_capture = None;
        self.status_retry.reset();
        self.connection_notice.reset();
        self.last_ws_endpoint = None;
        self.last_serial_port.clear();
        self.event_log.reset_sources("HTTP");
        self.event_log.reset_sources("WebSocket");
        self.event_log.reset_sources("USB");
        let _ = self.ws_tx.send(WsCommand::Disconnect);
        let _ = self.serial_tx.send(SerialCommand::Port(
            String::new(),
            self.connection_gate.epoch(),
        ));
        self.last_poll = Instant::now() - Duration::from_secs(30);
    }
    pub(crate) fn set_connection(&mut self, method: Transport, enabled: bool) {
        if self.connection_change_guard() {
            return;
        }
        if !enabled {
            self.presentation.update(None, None);
        }
        if !enabled || method != self.transport {
            peer_http::invalidate_address(&self.base);
            if let Some(origin) = self
                .status
                .as_ref()
                .and_then(|status| peer_http::configured_origin(&self.base, status))
            {
                peer_http::invalidate_address(&origin);
            }
        }
        if enabled && method == Transport::Serial && self.serial_port.trim().is_empty() {
            self.message = "Select a USB serial port before connecting".into();
            return;
        }
        self.transport = method;
        self.connection_enabled = enabled;
        self.config.connection_method = method;
        self.config.connection_enabled = enabled;
        self.config.endpoint = self.base.clone();
        self.config.user = self.user.clone();
        self.config.serial_port = self.serial_port.clone();
        if enabled
            && method == Transport::WiFi
            && let Some(route) = &self.config.last_network_route
        {
            peer_http::restore_route(route, &self.base);
        }
        self.restart_connection();
        self.save_workspace();
        self.message = if enabled {
            format!("Connecting through {}", method.label())
        } else {
            format!("{} disconnected", method.label())
        };
    }
    fn update_esp_from_file(&mut self, path: &str) -> Result<(), String> {
        if !self.http_writable() || !self.has_feature("ota") {
            return Err("Connect to the selected Wi-Fi board with OTA acknowledged first".into());
        }
        if self.connection_change_guard() {
            return Err("Save/discard drafts and finish pending operations before OTA".into());
        }
        let path = std::path::Path::new(path);
        if !path.is_absolute() {
            return Err("Select an absolute firmware file path".into());
        }
        let bytes = maintenance::read_image_file(path, true)?;
        let operation = self.source_operation(firmware_source::Source::Local(bytes), true)?;
        if let maintenance::Operation::SourceImage { progress, .. } = &operation {
            self.firmware_transfer = Some(progress.clone());
        }
        let id = self.device_settings.next_id();
        self.http_tx
            .send(HttpCommand::Maintenance(self.endpoint(), id, operation))
            .map_err(|_| "Firmware request queue closed")?;
        self.device_settings.maintenance_pending = Some(id);
        self.device_settings.message =
            "ESP Wi-Fi update queued once; do not disconnect its power".into();
        Ok(())
    }
    fn poll(&mut self) {
        self.poll_pixel_api();
        if let Some((id, receiver, cancel)) = &self.pixel_usb_reply {
            if !self.pixel_diagnostic.matches(*id) {
                cancel.store(true, std::sync::atomic::Ordering::Release);
                self.pixel_usb_reply = None;
            } else {
                match receiver.try_recv() {
                    Ok(result) => {
                        let id = *id;
                        self.pixel_diagnostic.finish(id,result.map(|_|"Device confirmed pixel colors via USB".into()).unwrap_or_else(|e|e));
                        self.pixel_usb_reply = None;
                    }
                    Err(TryRecvError::Disconnected) => {
                        let id = *id;
                        self.pixel_diagnostic.finish(
                            id,
                            "USB diagnostic ended without verified readback; nothing replayed."
                                .into(),
                        );
                        self.pixel_usb_reply = None;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        }
        self.http_tx.set_auto_discovery(self.config.auto_connect);
        let endpoint = self.endpoint();
        if self.connection_enabled
            && self.transport == Transport::WiFi
            && self.last_ws_endpoint.as_ref() != Some(&endpoint)
        {
            self.restart_connection();
            self.http_tx.clear();
            self.revoke_http_drafts();
            self.event_log.reset_sources("HTTP");
            self.event_log.reset_sources("WebSocket");
            self.status_request_pending = false;
            self.status_retry.reset();
            self.connection_notice.reset();
            self.device_settings.invalidate_peer();
            self.status = None;
            self.table.pixel_sample = None;
            self.last_status_at = None;
            self.websocket_live = false;
            self.stream_sent_fps = None;
            self.streaming = false;
            self.sampling = false;
            self.mappings.clear();
            self.access_info = None;
            self.device_name = None;
            self.board_display_name = None;
            self.identity_pending = false;
            self.identity_saved = None;
            let _ = self.ws_tx.send(WsCommand::Connect(
                endpoint.clone(),
                self.connection_gate.epoch(),
            ));
            let _ = self.ws_tx.send(WsCommand::Stream(false));
            let _ = self.ws_tx.send(WsCommand::Sample(false));
            let _ = self.ws_tx.send(WsCommand::Rate(self.config.stream_fps));
            self.last_ws_endpoint = Some(endpoint);
            self.configuration_loaded = false;
        }
        if self.connection_enabled
            && self.transport == Transport::Serial
            && self.last_serial_port != self.serial_port
        {
            self.restart_connection();
            self.event_log.reset_sources("USB");
            self.identity_pending = false;
            self.connection_notice.reset();
            self.serial_status = None;
            self.last_serial_status_at = None;
            self.serial_live = false;
            let _ = self.serial_tx.send(SerialCommand::Port(
                self.serial_port.clone(),
                self.connection_gate.epoch(),
            ));
            self.last_serial_port = self.serial_port.clone();
        }
        if self.connection_enabled
            && self.transport == Transport::WiFi
            && !self.status_request_pending
            && !(self.websocket_live
                && self
                    .last_status_at
                    .is_some_and(|at| at.elapsed() < Duration::from_secs(2)))
            && self.last_poll.elapsed() >= self.status_retry.delay()
        {
            self.last_poll = Instant::now();
            self.status_request_pending = self
                .http_tx
                .send(HttpCommand::Status(self.endpoint()))
                .is_ok();
            // Firmware progress already arrives in this same status/WS stream,
            // including external OTA. Do not queue a redundant job poll.
        }
        // A retired/expired queue entry may never produce an event. Its request
        // token must expire independently, and a late result cannot settle a
        // newer request. Preserve confirmed same-peer mapping while retrying.
        if self
            .mapping_request_pending
            .as_ref()
            .is_some_and(|pending| pending.started.elapsed() >= Duration::from_secs(24))
        {
            self.mapping_request_pending = None;
            self.mapping_retry.failed();
            self.last_mapping_poll = Instant::now();
            self.mapping_error = Some(
                "Mapping read timed out; retrying without changing the confirmed mapping".into(),
            );
        }
        let mapping_interval = if self.mapping_error.is_some() || self.mappings.is_empty() {
            self.mapping_retry.delay()
        } else {
            Duration::from_secs(15)
        };
        if self.command_route() == Some(Transport::WiFi)
            && self.has_table()
            && self.mapping_request_pending.is_none()
            && self.last_mapping_poll.elapsed() >= mapping_interval
            && let Some(peer) = self.status.as_ref().and_then(peer_http::Pin::from_status)
        {
            self.last_mapping_poll = Instant::now();
            self.mapping_request_id = self.mapping_request_id.wrapping_add(1);
            let id = self.mapping_request_id;
            if self
                .http_tx
                .send(HttpCommand::Mapping(self.endpoint(), id))
                .is_ok()
            {
                self.mapping_request_pending = Some(MappingPending {
                    id,
                    peer,
                    started: Instant::now(),
                });
            } else {
                self.mapping_retry.failed();
                self.mapping_error =
                    Some("Mapping request queue closed; reconnect to retry".into());
            }
        }
        // Load mapping before optional settings: the four settings preflights
        // used to queue ahead of the small mapping read on first connection.
        // Never flood an offline/bootstrap queue with settings preflights.
        // Read-only legacy peers can still load after a fresh Wi-Fi status.
        if !self.configuration_loaded {
            self.configuration_reads.reset_all(
                self.status
                    .as_ref()
                    .is_some_and(|s| s.ethernet["configured"] == true),
                self.has_feature("status-rgb"),
            );
            self.configuration_loaded = true;
        }
        // Auth is already probed by the HTTP worker. Optional settings must not
        // queue four status-fenced transactions ahead of interactive controls.
        // Keep one read outstanding, and require the table's initial mapping first.
        if self.command_route() == Some(Transport::WiFi)
            && (self.configuration_reads.urgent()
                || ((!self.has_table()
                    || self.mapping_peer
                        == self.live_status().and_then(peer_http::Pin::from_status))
                    && (self.device_settings.open
                        || self.show_connection
                        || self.dock.find_tab(&ui::layout::Panel::Network).is_some())))
            && let Some(path) = self.configuration_reads.take_due()
        {
            if self
                .http_tx
                .send(HttpCommand::Configuration(self.endpoint(), path))
                .is_err()
            {
                self.configuration_reads.finish(path, false);
            }
        }
        self.pixel_diagnostic.expire();
        while let Ok(event) = self.event_rx.try_recv() {
            let event_stamp = match &event {
                Event::ForConnection(stamp, _) if stamp.current() => Some(stamp.clone()),
                _ => None,
            };
            if let Some(receipt) = peer_http::retired_envelope(&event) {
                self.record_event("Previous board session", &receipt);
                continue;
            }
            let event = match event {
                Event::ForConnection(stamp, inner) if stamp.current() => *inner,
                Event::ForConnection(_, inner) => {
                    if let Some(receipt) = peer_http::retired_envelope(&inner)
                        .or_else(|| peer_http::retired_receipt(&inner))
                    {
                        self.record_event("Previous connection", &receipt);
                    }
                    continue;
                }
                other => other,
            };
            let Some(event) = current_event(event, &self.endpoint(), &self.serial_port) else {
                continue;
            };
            let event = match event {
                Event::ForScope(scope, inner) if scope.current() => *inner,
                Event::ForScope(_, inner) => {
                    if let Some(receipt) = peer_http::retired_receipt(&inner) {
                        self.record_event("Previous board session", &receipt);
                    }
                    continue;
                }
                other => other,
            };
            match event {
                Event::ForConnection(..) => {}
                Event::ForScope(..) => {} // Scope envelopes are unwrapped once.
                Event::ForEndpoint(..) => {} // Worker envelopes are unwrapped once.
                Event::StreamStats(value) => self.stream_sent_fps = Some((value, Instant::now())),
                Event::PixelSample(peer, frame) => {
                    if self.connection_enabled && self.transport == Transport::WiFi
                        && status_observation_fresh(self.last_status_at)
                        && let Some(status) = self.status.as_mut()
                        && peer_http::Pin::from_status(status).as_ref() == Some(&peer) {
                        ui::cell_pixels::Sample::observe(&mut self.table.pixel_sample, Some(frame.clone()));
                        status.pixel_preview_requested = true;
                        status.pixel_frame = Some(frame);
                    }
                }
                Event::PanelEvent(event) => {
                    if self.connection_enabled
                        && self.transport == Transport::WiFi
                        && status_observation_fresh(self.last_status_at)
                        && let Some(peer) =
                            self.status.as_ref().and_then(peer_http::Pin::from_status)
                    {
                        let mut value = serde_json::to_value(&event).unwrap_or_default();
                        value["type"] = "panel-event".into();
                        if self.panel_events.observe(event, &peer) {
                            self.bridge.publish_event(value);
                        }
                    }
                }
                Event::SerialPanelEvent(_, session, event) => {
                    if self.connection_enabled
                        && self.transport == Transport::Serial
                        && session == self.serial_session
                        && status_observation_fresh(self.last_serial_status_at)
                        && let Some(peer) = self
                            .serial_status
                            .as_ref()
                            .and_then(peer_http::Pin::from_status)
                    {
                        let mut value = serde_json::to_value(&event).unwrap_or_default();
                        value["type"] = "panel-event".into();
                        if self.panel_events.observe(event, &peer) {
                            self.bridge.publish_event(value);
                        }
                    }
                }
                Event::PeripheralEvent(event) => {
                    if self.connection_enabled
                        && self.transport == Transport::WiFi
                        && status_observation_fresh(self.last_status_at)
                        && let Some(status) = self.status.as_mut()
                    {
                        let mut value = serde_json::to_value(&event).unwrap_or_default();
                        value["type"] = "pc-contract-event".into();
                        if self.peripheral_events.observe(event, status) {
                            self.bridge.publish_event(value);
                        }
                    }
                }
                Event::SerialSegments(_, session, frame) => {
                    if self.connection_enabled
                        && self.transport == Transport::Serial
                        && session == self.serial_session
                        && status_observation_fresh(self.last_serial_status_at)
                        && let Some(status) = self.serial_status.as_mut()
                    {
                        frame.apply(status);
                    }
                }
                Event::StatusRgb(rgb) => {
                    if self.connection_enabled
                        && self.transport == Transport::WiFi
                        && status_observation_fresh(self.last_status_at)
                    {
                        if let Some(status) = self.status.as_mut() {
                            status_rgb::observe(status, rgb);
                        }
                    } // Never renew full status, capability, command-clock or workspace authority.
                }
                Event::UpgradeProgress(upgrade) => {
                    if self.connection_enabled && self.transport == Transport::WiFi {
                        if let Some(status) = self.status.as_mut() {
                            firmware_progress::observe_upgrade(status, upgrade);
                        }
                    } // Progress alone never renews command/status authority.
                }
                Event::Status(mut s) => {
                    if realtime::older_than(&s, self.status.as_ref()) {
                        continue;
                    }
                    firmware_progress::retain_upgrade(&mut s, self.status.as_ref());
                    status_rgb::retain(&mut s, self.status.as_ref());
                    if let Some(previous) = &self.status {
                        peer_http::rebind_verified_name(&self.base, previous, &s);
                    }
                    if let Some(origin) = peer_http::configured_origin(&self.base, &s)
                        && origin != self.base.trim_end_matches('/')
                        && self
                            .status
                            .as_ref()
                            .is_none_or(|previous| previous.board_id == s.board_id)
                    {
                        // Actual decoded status, not a pending saved rename or
                        // raw discovery hint, owns an active hostname change.
                        self.http_tx.clear();
                        self.configuration_reads.reset();
                        self.base = origin;
                        self.config.endpoint = self.base.clone();
                        let endpoint = self.endpoint();
                        // This is the SAME physical peer, not a manual target
                        // switch. Avoid poll's generic endpoint-reset path:
                        // retain user drafts/layout/mappings, retire only old
                        // request scopes and reopen the socket at the new name.
                        self.last_ws_endpoint = Some(endpoint.clone());
                        let _ = self
                            .ws_tx
                            .send(WsCommand::Connect(endpoint, self.connection_gate.epoch()));
                        let _ = self.ws_tx.send(WsCommand::Mapping(self.mappings.clone()));
                        self.configuration_loaded = false;
                        self.status_request_pending = false;
                        self.save_workspace();
                    }
                    if !s.hostname.is_empty() {
                        if ui::cell_pixels::semantic_changed(&s, self.status.as_ref()) {
                            self.table.pixel_sample = None; // Old RGB must not mask a new acknowledged semantic state.
                        }
                        if self.http_tx.observe(&s) {
                            // A resumed lease on this exact board/boot is not a
                            // new workspace. Queue generations still retire all
                            // old operations; only a genuine peer change may
                            // discard selection, text and settings drafts.
                            if self.status.as_ref().and_then(peer_http::Pin::from_status)
                                != peer_http::Pin::from_status(&s)
                            {
                                self.revoke_http_drafts();
                            }
                        }
                        self.device_settings.remote_job =
                            maintenance::parse_job(s.remote_firmware.clone()).ok();
                        if s.pixel_preview_requested {
                            ui::cell_pixels::Sample::observe(
                                &mut self.table.pixel_sample,
                                s.pixel_frame.clone(),
                            );
                        }
                        self.brightness = s.brightness;
                        self.pixel_diagnostic.observe(&s);
                        if !pixel_diagnostic::ready(&s) {
                            self.pixel_diagnostic.armed = None;
                        }
                        self.pattern = s.effect.clone();
                        self.output_mask = s.active_outputs;
                        self.observe_winner(&s);
                        self.status = Some(s);
                        if let Some(route) = self
                            .status
                            .as_ref()
                            .and_then(|s| peer_http::verified_route(&self.base, s))
                            && self.config.last_network_route.as_ref() != Some(&route)
                        {
                            self.config.last_network_route = Some(route);
                            self.save_workspace();
                        }
                        self.last_status_at = Some(Instant::now());
                        self.message.clear();
                    }
                }
                Event::DiscoveredBoard(endpoint, status) => {
                    if !self.config.auto_connect
                        || !self.connection_enabled
                        || self.transport != Transport::WiFi
                        || status_observation_fresh(self.last_status_at)
                    {
                        continue;
                    }
                    if self.connection_change_guard() {
                        continue;
                    }
                    self.base = endpoint.base;
                    self.user = endpoint.user;
                    self.password = endpoint.password;
                    self.config.endpoint = self.base.clone();
                    self.config.user = self.user.clone();
                    self.restart_connection();
                    self.save_workspace();
                    self.message = format!(
                        "Discovered {} — connecting at {}",
                        status.hostname, self.base
                    );
                }
                Event::ConfigurationChanged(board, boot) => {
                    if self.connection_enabled
                        && self.transport == Transport::WiFi
                        && self
                            .status
                            .as_ref()
                            .and_then(peer_http::Pin::from_status)
                            .is_some_and(|pin| pin.matches_identity(&board, &boot))
                    {
                        self.configuration_loaded = false;
                        self.last_poll = Instant::now() - Duration::from_secs(30);
                    }
                }
                Event::Mapping(id, mappings, _) => {
                    let current_peer = self.status.as_ref().and_then(peer_http::Pin::from_status);
                    if !self
                        .mapping_request_pending
                        .as_ref()
                        .is_some_and(|pending| {
                            pending.id == id && Some(&pending.peer) == current_peer.as_ref()
                        })
                    {
                        continue;
                    }
                    self.mapping_request_pending = None;
                    self.mapping_retry.reset();
                    self.mapping_peer = current_peer;
                    self.mapping_error = None;
                    self.last_mapping_poll = Instant::now();
                    if self.table.pixel_sample.as_ref().is_some_and(|sample| {
                        mappings.iter().any(|m| {
                            sample.frame.physical.get(usize::from(m.logical_id - 1))
                                != Some(&m.physical_cell)
                        })
                    }) {
                        self.table.pixel_sample = None;
                    }
                    self.mappings = mappings;
                    let _ = self.ws_tx.send(WsCommand::Mapping(self.mappings.clone()));
                    if !streaming::valid_mapping(&self.mappings) {
                        self.sampling = false;
                    }
                }
                Event::MappingFailed(id, error) => {
                    if !self
                        .mapping_request_pending
                        .as_ref()
                        .is_some_and(|pending| pending.id == id)
                    {
                        continue;
                    }
                    self.mapping_request_pending = None;
                    self.mapping_retry.failed();
                    self.last_mapping_poll = Instant::now();
                    self.mapping_error = Some(error.clone());
                    self.event_log
                        .failure("HTTP GET /api/cells/mapping", &error);
                }
                Event::Configuration("/api/auth/info", value) => {
                    self.configuration_reads.finish("/api/auth/info", true);
                    self.access_enabled =
                        value.get("enabled").and_then(|v| v.as_bool()) == Some(true);
                    self.access_requirement = value
                        .get("enabled")
                        .and_then(|v| v.as_bool())
                        .map(|enabled| (self.base.clone(), enabled));
                    self.access_user = value
                        .get("username")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .into();
                    self.access_info = Some(value);
                }
                Event::Configuration(path, value) => {
                    self.configuration_reads.finish(path, true);
                    if path != "/api/device/name" || self.command_route() != Some(Transport::Serial)
                    {
                        self.receive_configuration(path, value);
                    }
                }
                Event::CorrectionPreviewFinished(id, result) => {
                    self.pixel_corrections.live.finish(id, result, Instant::now());
                }
                Event::MaintenanceFinished(id, result) => {
                    if self.device_settings.maintenance_pending == Some(id) {
                        if let Some(progress) = self.firmware_transfer.as_ref() {
                            progress.finish(&result);
                        }
                        let severity = maintenance::result_severity(&result);
                        self.device_settings.maintenance_pending = None;
                        self.device_settings.message = result.unwrap_or_else(|e| e);
                        self.record_event("Maintenance", &self.device_settings.message.clone());
                        if severity == messaging::Severity::Error {
                            self.notify_detailed_id(
                                "pakar.maintenance",
                                "Maintenance result",
                                "Maintenance did not complete.",
                                &self.device_settings.message,
                                severity,
                            );
                        } else {
                            self.notify_id(
                                "pakar.maintenance",
                                "Maintenance result",
                                &self.device_settings.message,
                                severity,
                            );
                        }
                        self.configuration_loaded = false;
                    }
                }
                Event::SettingsFinished(id, group, result) => {
                    if self.device_settings.pending != Some((id, group)) {
                        continue;
                    }
                    let severity = if result.is_ok() {
                        messaging::Severity::Success
                    } else {
                        messaging::Severity::Error
                    };
                    self.device_settings.finish(id, group, result);
                    self.record_event("Settings", &self.device_settings.message.clone());
                    if severity == messaging::Severity::Error {
                        self.notify_detailed_id(
                            "pakar.settings",
                            "Device settings result",
                            "Device settings were not applied.",
                            &self.device_settings.message,
                            severity,
                        );
                    } else {
                        self.notify_id(
                            "pakar.settings",
                            "Device settings result",
                            &self.device_settings.message,
                            severity,
                        );
                    }
                }
                Event::CredentialsUpdated(endpoint) => {
                    self.access_requirement =
                        Some((endpoint.base.clone(), !endpoint.password.is_empty()));
                    self.user = endpoint.user;
                    self.password = endpoint.password;
                    self.config.user = self.user.clone();
                    self.access_password.clear();
                    self.access_confirm.clear();
                    self.configuration_loaded = false;
                    self.last_ws_endpoint = None;
                    self.notify("Credentials saved", "All device sessions were signed out. This client will reconnect with the new credentials.", messaging::Severity::Success);
                }
                Event::NameUpdated(endpoint, pending) => {
                    self.identity_pending = false;
                    self.identity_saved = None;
                    self.base = endpoint.base;
                    self.config.endpoint = self.base.clone();
                    self.configuration_loaded = false;
                    self.last_ws_endpoint = None;
                    self.notify("Board identity saved", &if pending {format!("Reconnect at {} after Wi-Fi reassociates. DNS caches may need time to expire.", self.base)} else {"Board name verified. Wi-Fi was not restarted.".into()}, messaging::Severity::Success);
                }
                Event::SerialIdentity(port, value) => {
                    if port == self.serial_port && self.command_route() == Some(Transport::Serial) {
                        self.receive_configuration("/api/device/name", value);
                    }
                }
                Event::SerialSnapshot(_, mut snapshot) => {
                    firmware_progress::retain_upgrade(
                        &mut snapshot.status,
                        self.serial_status.as_ref(),
                    );
                    self.device_settings.remote_job =
                        maintenance::parse_job(snapshot.status.remote_firmware.clone()).ok();
                    let peer = peer_http::Pin::from_status(&snapshot.status);
                    let previous = self
                        .serial_status
                        .as_ref()
                        .and_then(peer_http::Pin::from_status);
                    if peer != previous {
                        self.board_silent.revoke();
                        self.table.stop_pending_edits();
                        // Revoked wire ownership/boot must cancel operations,
                        // not discard this same physical board's user workspace.
                        // The presentation workspace owns selection/find across
                        // sessions; fresh peer admission below never replays it.
                        self.table.pixel_sample = None;
                        self.pixel_diagnostic.revoke();
                        self.show_mapping = false;
                        self.mapping_request_pending = None;
                    }
                    self.mappings = snapshot.mappings;
                    self.mapping_peer = peer;
                    self.mapping_error = None;
                    if ui::cell_pixels::semantic_changed(
                        &snapshot.status,
                        self.serial_status.as_ref(),
                    ) {
                        self.table.pixel_sample = None;
                    }
                    if self.table.pixel_sample.as_ref().is_some_and(|sample| {
                        self.mappings.iter().any(|mapping| {
                            sample
                                .frame
                                .physical
                                .get(usize::from(mapping.logical_id - 1))
                                != Some(&mapping.physical_cell)
                        })
                    }) {
                        self.table.pixel_sample = None;
                    }
                    ui::cell_pixels::Sample::observe_at(
                        &mut self.table.pixel_sample,
                        snapshot.status.pixel_frame.clone(),
                        snapshot.observed_at,
                    );
                    self.pattern = snapshot.status.effect.clone();
                    self.pixel_diagnostic.observe(&snapshot.status);
                    self.observe_winner(&snapshot.status);
                    self.serial_status = Some(snapshot.status);
                    self.last_serial_status_at = Some(snapshot.observed_at);
                    self.serial_live = true;
                    self.request_missing_usb_pixels();
                    if matches!(
                        self.message.as_str(),
                        "Connecting through USB" | "Connecting to PakarController"
                    ) {
                        self.message = "USB connected".into();
                    }
                    self.event_log.recover(&format!("USB {}", self.serial_port));
                }
                Event::CellBatchConfirmed(correlation)
                | Event::SerialBatchConfirmed(_, correlation) => {
                    if self.command_route() == Some(correlation.route)
                        && self
                            .live_status()
                            .and_then(peer_http::Pin::from_status)
                            .as_ref()
                            == Some(&correlation.peer)
                    {
                        self.table.confirm_batch_ack(&correlation);
                        self.record_event(
                            "Atomic cell batch",
                            &format!(
                                "Acknowledged and same-board readback verified for {} cells",
                                correlation.request.ids.len()
                            ),
                        );
                    } else {
                        self.record_event("Previous board session","Atomic batch result belongs to another peer/route; no current UI batch completed");
                    }
                }
                Event::SerialOperation(_, result) => match result {
                    Ok(message) => {
                        self.message = message.clone();
                        self.record_event("USB command", &message);
                    }
                    Err(error) => {
                        self.message = error.clone();
                        self.record_event("USB command unverified", &error);
                        self.table.fail_pending_edits(error);
                    }
                },
                Event::SerialCommandState(_, mut status) => {
                    firmware_progress::retain_upgrade(&mut status, self.serial_status.as_ref());
                    if peer_http::Pin::from_status(&status)
                        == self
                            .serial_status
                            .as_ref()
                            .and_then(peer_http::Pin::from_status)
                    {
                        self.pattern = status.effect.clone();
                        self.serial_status = Some(status);
                        self.table.pixel_sample = None;
                    }
                }
                Event::SerialTiming(_, timing) => self.serial_timing = Some(timing),
                Event::SerialCapture(_, progress) => self.serial_capture = Some(progress),
                Event::SerialQueueDepth(_, depth) => self.serial_operations_pending = depth,
                Event::SerialOwner(_, session) => {
                    self.panel_events.clear();
                    self.peripheral_events.clear();
                    self.board_silent.revoke();
                    self.serial_session = session;
                    self.serial_stamp = event_stamp.clone();
                    self.serial_status = None;
                    self.last_serial_status_at = None;
                    self.serial_operations_pending = 0;
                    self.serial_timing = None;
                    self.mappings.clear();
                    self.mapping_peer = None;
                    self.table.pixel_sample = None;
                    self.table.stop_pending_edits();
                }
                Event::BoardSilentFinished(id, result) => {
                    let peer = self.live_status().and_then(peer_http::Pin::from_status);
                    let route = self.command_route();
                    self.device_settings
                        .finish_avr(id, peer.as_ref(), route, &result);
                    self.board_silent.finish(id, peer.as_ref(), route, result);
                }
                Event::SerialIdentitySaved(port, result) => {
                    if port == self.serial_port {
                        self.identity_pending = false;
                        if result.is_ok() {
                            self.identity_saved = None;
                        }
                        match result {
                            Ok(()) => self.notify(
                                "Board identity saved",
                                "USB ACK and identity readback verified.",
                                messaging::Severity::Success,
                            ),
                            Err(error) => self.notify(
                                "Board identity unverified",
                                &error,
                                messaging::Severity::Warning,
                            ),
                        }
                    }
                }
                Event::PixelModeFinished(id, result) => {
                    if self.pixel_diagnostic.matches(id) {
                        match result {
                            Ok(status)
                                if peer_http::Pin::from_status(&status)
                                    == self.live_status().and_then(peer_http::Pin::from_status) =>
                            {
                                self.pixel_diagnostic.armed = peer_http::Pin::from_status(&status);
                                self.status = Some(status);
                                self.last_status_at = Some(Instant::now());
                                self.pixel_diagnostic.finish(id, "Effects mode acknowledged and read back. Select a physical pixel, then Show/Previous/Next.".into());
                            }
                            Ok(_) => self.pixel_diagnostic.finish(
                                id,
                                "Mode result belongs to another board; no frame sent".into(),
                            ),
                            Err(error) => self.pixel_diagnostic.finish(id, error),
                        }
                    }
                }
                Event::PixelSent(request, result) => {
                    if self.pixel_diagnostic.matches(request.id) {
                        match result {
                            Ok(()) if Some(request.peer.clone()) == self.live_status().and_then(peer_http::Pin::from_status) => {
                                self.pixel_diagnostic.message = format!("Pixel {} frame written once; checking RGB readback…", request.index + 1);
                                if self.http_tx.send(HttpCommand::PixelReadback(self.endpoint(), request.id, request.index, request.peer)).is_err() {
                                    self.pixel_diagnostic.finish(request.id, "Frame written; pixel readback unavailable. No replay.".into());
                                }
                            }
                            Ok(()) => self.pixel_diagnostic.finish(request.id, "Frame write completed for a retired peer; inspect it before retrying".into()),
                            Err(error) => self.pixel_diagnostic.finish(request.id, error),
                        }
                    }
                }
                Event::PixelReadbackFinished(id, result) => {
                    self.pixel_diagnostic.finish(id, result.unwrap_or_else(|error|
                        format!("Frame written; pixel readback unavailable: {error}. Nothing replayed.")));
                }
                Event::PixelCommandFinished(id, result) => {
                    if self.pixel_diagnostic.matches(id) {
                        self.pixel_diagnostic.finish(id,match result {
                            Ok(_)=>"Device confirmed pixel colors".into(),
                            Err(error)=>format!("{error} No new intent was replayed.")});
                    }
                }
                Event::Accepted(m) => {
                    self.record_event("Command", &m);
                    self.notify("Command accepted", &m, messaging::Severity::Success);
                }
                Event::OperationFailed(error) => {
                    self.identity_pending = false;
                    self.table.fail_pending_edits(error.clone());
                    self.record_event("Operation failed", &error);
                    self.notify_detailed_id(
                        "pakar.operation-failed",
                        "Operation failed",
                        "The requested operation did not complete.",
                        &error,
                        messaging::Severity::Error,
                    );
                    self.message = error;
                }
                Event::DiagnosticFailure(source, e) => {
                    if source == "Screen sampler" { self.sampling = false; }
                    if peer_identity::unsupported(&e) {
                        self.reject_incompatible_peer(&e);
                    }
                    self.configuration_reads.failed_source(&source);
                    if source == "HTTP GET /api/status" {
                        // A recoverable HTTP read failure is not a peer change.
                        // Fresh WS/HTTP observations retain the same bound view;
                        // writes still require fresh preflight and pinned headers.
                        if self.http_tx.writable() && !status_observation_fresh(self.last_status_at)
                        {
                            self.http_tx.clear();
                            self.table.stop_pending_edits();
                            self.pixel_diagnostic.revoke();
                            self.streaming = false;
                            self.sampling = false;
                            let _ = self.ws_tx.send(WsCommand::Stream(false));
                            let _ = self.ws_tx.send(WsCommand::Sample(false));
                            // Retain the old observation only as peer ownership
                            // context. last_status_at stays expired, so neither
                            // live_status nor command_route grants authority.
                            self.last_status_at = None;
                        }
                        self.status_request_pending = false;
                        self.status_retry.failed();
                        self.last_poll = Instant::now();
                    }
                    if self.device_settings.scan_pending {
                        self.device_settings.scan_pending = false;
                        self.device_settings.message =
                            "Network scan did not complete. Reconnect and scan again.".into();
                    }
                    self.event_log.failure(&source, &e);
                    self.message = e;
                }
                Event::DiagnosticRecovered(source) => {
                    if source == "HTTP GET /api/status" {
                        self.status_request_pending = false;
                        self.status_retry.reset();
                    }
                    self.event_log.recover(&source);
                }
                Event::SerialFailure(port, error) => {
                    if peer_identity::unsupported(&error) {
                        self.reject_incompatible_peer(&error);
                    }
                    self.event_log.failure(&format!("USB {port}"), &error);
                }
                Event::WebSocket(v) => {
                    if self.websocket_live && !v {
                        self.record_event(
                            "Connection",
                            "WebSocket connection lost; reconnecting with bounded backoff.",
                        );
                    }
                    self.websocket_live = v;
                    if !v {
                        self.serial_session = 0;
                        self.serial_stamp = None;
                        self.serial_operations_pending = 0;
                        self.serial_timing = None;
                        // Socket loss does not retire this board's fresh HTTP
                        // observation or its same-intent fallback receipt.
                        self.pixel_diagnostic.armed = None;
                        if !status_observation_fresh(self.last_status_at) {
                            self.pixel_diagnostic.revoke();
                        }
                        self.stream_sent_fps = None;
                        self.streaming = false;
                        self.sampling = false;
                    }
                }
                Event::Serial(_, v, s) => {
                    if self.serial_live && !v {
                        self.record_event("Connection", "USB connection lost; reconnecting.");
                    }
                    if v {
                        self.event_log.recover(&format!("USB {}", self.serial_port));
                    }
                    self.serial_live = v;
                    if !v {
                        self.serial_status = None;
                        self.last_serial_status_at = None;
                        if self.transport == Transport::Serial {
                            self.mappings.clear();
                            self.mapping_peer = None;
                            self.table.pixel_sample = None;
                            self.table.stop_pending_edits();
                        }
                    }
                    if let Some(s) = s {
                        if self
                            .serial_status
                            .as_ref()
                            .and_then(peer_http::Pin::from_status)
                            != peer_http::Pin::from_status(&s)
                        {
                            self.mappings.clear();
                            self.mapping_peer = None;
                            self.table.pixel_sample = None;
                            self.table.stop_pending_edits();
                            self.pixel_diagnostic.revoke();
                        }
                        self.observe_winner(&s);
                        self.serial_status = Some(s);
                        self.last_serial_status_at = Some(Instant::now());
                    }
                }
            }
        }
        self.poll_firmware_feed();
        let state = self.live_status().map(|s| s.avr_online);
        if let Some((message, severity)) = self.connection_notice.observe(state, Instant::now()) {
            self.record_event("Connection", message);
            self.notify_id("pakar.connection", "Connection changed", message, severity);
        }
        let tip = match self.live_status() {
            Some(s) => format!(
                "{} — AVR {}",
                s.capabilities
                    .as_ref()
                    .map(|c| c.title())
                    .unwrap_or_else(|| "PakarController".into()),
                if s.avr_online { "online" } else { "offline" }
            ),
            None => "PakarController — device offline".into(),
        };
        if let Some(tray) = &mut self.tray {
            match tray.refresh_tooltip(&tip, Instant::now()) {
                Ok(true) => self.event_log.recover("Windows tray tooltip"),
                Ok(false) => {}
                Err(error) => self.event_log.failure("Windows tray tooltip", &error),
            }
        }
    }
    fn deliver_notifications(&mut self) {
        loop {
            let request = self
                .notifications
                .borrow_mut()
                .next(&self.config.notifications, messaging::now_ms());
            let Some(request) = request else {
                break;
            };
            let result = self
                .tray
                .as_ref()
                .ok_or_else(|| "Native notification unavailable: tray registration failed".into())
                .and_then(|tray| tray.notify(&request, self.config.notifications.sound));
            if let Err(error) = &result {
                self.record_event("Windows notification", error);
            }
            self.notifications.borrow_mut().result(result);
        }
    }
    fn notification_snapshot(&self) -> serde_json::Value {
        let delivery = self.notifications.borrow();
        serde_json::json!({"backend":"Windows Shell", "policy":self.config.notifications.system, "sound":self.config.notifications.sound, "in_app":self.config.notifications.in_app, "accepted":delivery.accepted, "suppressed":delivery.suppressed, "failed":delivery.failed, "last_result":delivery.last_result, "shell_events":self.tray.as_ref().map(|tray| tray.notification_snapshot())})
    }
    #[cfg(test)]
    fn next_cell_state(s: &str) -> (&'static str, u8) {
        match s {
            "white" => ("red", 1),
            "red" => ("off", 2),
            _ => ("white", 0),
        }
    }
}

impl eframe::App for PakarControllerApp {
    fn logic(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Adapted from Pealayer: eframe runs logic while minimized/occluded.
        // IPC/tray, device results and status must not depend on painting.
        self.capture.receive(ctx);
        self.capture.tick(ctx);
        self.native_logic(ctx);
    }
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        // Bootstrap identity once; thereafter only actually rendered consumers
        // request full table data. A visible empty shell is not an RGB reader.
        self.usb_ui_demand = (
            usb_demand::Demand {
                flags: usb_demand::STATUS,
                table: self.serial_status.is_none(),
            },
            Some(Instant::now()),
        );
        self.draw_shell(ui);
        self.capture.after_frame(ui.ctx());
    }
    fn save(&mut self, _: &mut dyn eframe::Storage) {
        self.save_workspace();
    }
}

fn environment_endpoint() -> Endpoint {
    let saved = config::AppConfig::load().unwrap_or_default();
    Endpoint {
        base: std::env::var("PAKAR_ENDPOINT").unwrap_or(saved.endpoint),
        user: std::env::var("PAKAR_USER").unwrap_or(saved.user),
        password: std::env::var("PAKAR_PASSWORD").unwrap_or_default(),
    }
}

fn blocking_status(client: &Client, endpoint: &Endpoint) -> Result<DeviceStatus, String> {
    let response = client
        .get(format!(
            "{}/api/status",
            endpoint.base.trim_end_matches('/')
        ))
        .basic_auth(&endpoint.user, Some(&endpoint.password))
        .send()
        .map_err(|error| {
            // A body/status timeout does not prove that cached DNS is wrong.
            // Only a dial/resolution failure asks a later read to re-resolve;
            // the previous verified hint is retained until replacement.
            if error.is_connect() {
                peer_http::request_relookup(&endpoint.base);
            }
            diagnostics::describe_error(error)
        })?;
    if !response.status().is_success() {
        return Err(format!(
            "HTTP {}: {}",
            response.status(),
            response.text().unwrap_or_default()
        ));
    }
    let remote = response.remote_addr();
    peer_identity::validate_response_headers(response.headers())?;
    let status = response.json().map_err(diagnostics::describe_error)?;
    peer_identity::validate_board(&status)?;
    peer_http::remember_address(&endpoint.base, &status, remote);
    Ok(status)
}

fn format_text_status(status: &DeviceStatus, table: bool) -> String {
    let mut output = String::new();
    macro_rules! buffer_println { ($($arg:tt)*) => {{ use std::fmt::Write as _; let _ = writeln!(output, $($arg)*); }}; }
    macro_rules! buffer_print { ($($arg:tt)*) => {{ use std::fmt::Write as _; let _ = write!(output, $($arg)*); }}; }
    buffer_println!(
        "{} {} | AVR {} | TM1637 {} | {} | MQTT {} | NTP {} | brightness {} | effect {} | LED output {}",
        status.hostname,
        status.station_ip,
        if status.avr_online {
            "online"
        } else {
            "offline"
        },
        if status.tm1637_ack {
            "ready"
        } else {
            "missing"
        },
        status.mode,
        if status.mqtt_connected {
            "connected"
        } else {
            "offline"
        },
        if status.ntp_synced {
            "synced"
        } else {
            "waiting"
        },
        status.brightness,
        status.effect,
        match status.led_output.get("enabled").and_then(|v| v.as_bool()) {
            Some(true) => "On",
            Some(false) => "Off",
            None => "Unknown",
        }
    );
    buffer_println!(
        "inputs {:02X}+{} outputs {:02X} winner {} link {} UART {}/{}/{}",
        status.active_inputs,
        u8::from(status.input_9),
        status.active_outputs,
        status.winner,
        status.link_mode,
        status.esp_uart_errors,
        status.avr_uart_errors,
        status.usb_serial_errors
    );
    if !table || !status.capabilities.as_ref().is_some_and(|m| m.table()) {
        return output;
    }
    for row in 0..8_i32 {
        for column in 0..10_i32 {
            let id = row * 10 + column + 1;
            let mark = match status.cells.get((id - 1) as usize).map(String::as_str) {
                Some("white") => 'W',
                Some("red") => 'R',
                _ => '.',
            };
            buffer_print!("{id:02}{mark}  ");
        }
        buffer_println!();
    }
    output
}

fn text_command(client: &Client, endpoint: &Endpoint, words: &[&str]) -> Result<(), String> {
    let mut output = String::new();
    text_command_output(client, endpoint, words, &mut output)?;
    cli_print!("{output}");
    Ok(())
}

// One command parser for CLI and the TUI command palette. Capturing output must
// never replace a peer operation with a second invocation or stdout redirection.
fn text_command_output(
    client: &Client,
    endpoint: &Endpoint,
    words: &[&str],
    output: &mut String,
) -> Result<(), String> {
    macro_rules! buffer_println {
        ($($arg:tt)*) => {{
            use std::fmt::Write as _;
            writeln!(output, $($arg)*).map_err(|e| e.to_string())?;
        }};
    }
    if let ["cells", ids, state, rest @ ..] = words {
        let effect = match rest {
            [] => "immediate",
            [effect] => effect,
            _ => return Err("cells IDS white|red|off [immediate|fade|pulse]".into()),
        };
        let request = cell_batch::Request::new(&ui::cell_search::parse(ids)?, state, effect)?;
        buffer_println!(
            "{}",
            cell_batch::send(client, endpoint, &request, None, None, None)?
        );
        return Ok(());
    }
    if words.first() == Some(&"firmware") {
        use maintenance::Operation;
        let operation=match words {
            ["firmware","status"]=>Operation::RemoteStatus,
            ["firmware","fetch",url,sha] | ["firmware","fetch",url,sha,"all"]=>Operation::RemoteStage{url:(*url).into(),sha256:(*sha).into(),targets:3,bearer:std::env::var("PAKAR_FIRMWARE_TOKEN").unwrap_or_default()},
            ["firmware","fetch",url,sha,target]=>Operation::RemoteStage{url:(*url).into(),sha256:(*sha).into(),targets:match *target{"esp"=>1,"avr"=>2,_=>return Err("Target must be all, esp or avr".into())},bearer:std::env::var("PAKAR_FIRMWARE_TOKEN").unwrap_or_default()},
            ["firmware",action,id,boot,sha] if *action=="apply" || *action=="cancel"=>{
                // Read-only preflight. Never fetch replacement identity for an
                // unpinned mutation; the caller must supply the advertised triple.
                let current=maintenance::run(client,endpoint,Operation::RemoteStatus)?;
                let job=maintenance::parse_job(serde_json::from_str(&current).map_err(|_|"Invalid job JSON")?)?;
                if job.job_id!=id.parse::<u32>().map_err(|_|"Invalid job ID")? || job.boot_id!=*boot || job.manifest_sha256!=*sha {
                    return Err("Stale or incorrect remote job identity; nothing applied".into());
                }
                if *action=="apply"{Operation::RemoteApply(job)}else{Operation::RemoteCancel(job)}
            }
            _=>return Err("firmware status | fetch HTTPS_URL MANIFEST_SHA256 [all|esp|avr] | apply JOB BOOT SHA256 | cancel JOB BOOT SHA256".into()),
        };
        buffer_println!("{}", maintenance::run(client, endpoint, operation)?);
        return Ok(());
    }
    if words == ["info"] || words == ["telemetry"] {
        let value = pc_contract::get(
            client,
            &endpoint.base,
            &endpoint.user,
            &endpoint.password,
            "/api/telemetry",
        )?;
        buffer_println!(
            "{}",
            serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if words.first() == Some(&"avr") {
        let fetch = |op, payload: &[u8]| {
            pc_contract::request(
                client,
                &endpoint.base,
                &endpoint.user,
                &endpoint.password,
                op,
                payload,
            )
        };
        let value = match words {
            ["avr","contracts"] => pc_contract::get(client,&endpoint.base,&endpoint.user,&endpoint.password,"/api/avr/contracts")?,
            ["avr","settings"] => {
                let receipt=fetch(4,&[])?;
                pc_contract::Settings::parse(receipt["payload_hex"].as_str().ok_or("Missing settings payload")?)?.json()
            }
            ["avr","set",key,input] => {
                let current=fetch(4,&[])?;
                let mut next=pc_contract::Settings::parse(current["payload_hex"].as_str().ok_or("Missing settings payload")?)?;
                next.set(key,input)?;
                let saved=fetch(5,&next.bytes)?;
                if saved["response_opcode"].as_u64()!=Some(0x80) ||
                    saved["payload_hex"].as_str()!=Some("0500") {
                    return Err("AVR did not acknowledge settings; operation outcome is unknown".into());
                }
                let readback=fetch(4,&[])?;
                let verified=pc_contract::Settings::parse(readback["payload_hex"].as_str().ok_or("Missing settings readback")?)?;
                if !verified.persisted || verified.bytes!=next.bytes {
                    return Err("AVR settings readback did not confirm the requested persistent values".into());
                }
                verified.json()
            }
            ["avr","status"] => fetch(2,&[])?,
            ["avr","panel"] => fetch(0x3c,&[])?,
            ["avr","command",op] => fetch(pc_contract::opcode(op)?,&[])?,
            ["avr","command",op,payload] => fetch(pc_contract::opcode(op)?,&pc_contract::decode_hex(payload)?)?,
            _ => return Err("avr contracts | settings | set PARAMETER VALUE | status | panel | command OPCODE [HEX]".into())
        };
        buffer_println!(
            "{}",
            serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if words == ["device", "name"] || words == ["device", "identity"] {
        if let Event::Configuration(_, value) = http_request(
            client,
            HttpCommand::Configuration(endpoint.clone(), "/api/device/name"),
        )? {
            buffer_println!(
                "{}",
                serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?
            );
        }
        return Ok(());
    }
    if let ["device", "label", rest @ ..] = words {
        let label = rest.join(" ");
        if !identity::valid_display_name(&label) {
            return Err(
                "Board name must be 1..63 UTF-8 bytes without controls or edge spaces".into(),
            );
        }
        let Event::Configuration(_, value) = http_request(
            client,
            HttpCommand::Configuration(endpoint.clone(), "/api/device/name"),
        )?
        else {
            return Err("Identity unavailable".into());
        };
        let host = value["hostname"].as_str().ok_or("Hostname unavailable")?;
        let _ = http_request(
            client,
            HttpCommand::NameChange(endpoint.clone(), host.into(), Some(label)),
        )?;
        buffer_println!("Board display name saved and verified; Wi-Fi was not restarted.");
        return Ok(());
    }
    if let ["device", "configure", host, rest @ ..] = words {
        let label = rest.join(" ");
        if !identity::valid_display_name(&label) {
            return Err("Invalid board display name".into());
        }
        if let Event::NameUpdated(next, pending) = http_request(
            client,
            HttpCommand::NameChange(endpoint.clone(), (*host).into(), Some(label)),
        )? {
            buffer_println!(
                "Board identity saved. Endpoint: {}. Network apply pending: {pending}",
                next.base
            );
        }
        return Ok(());
    }
    if let ["device", "rename", name] = words {
        if let Event::NameUpdated(next, pending) = http_request(
            client,
            HttpCommand::NameChange(endpoint.clone(), (*name).into(), None),
        )? {
            buffer_println!(
                "Device confirmed persistent name saved. Reconnect at {} after Wi-Fi reassociates. Set PAKAR_ENDPOINT to this URL for subsequent CLI calls.",
                next.base
            );
            if !pending {
                buffer_println!("No Wi-Fi restart was required.");
            }
        }
        return Ok(());
    }
    if words == ["auth", "info"] {
        let value: serde_json::Value = client
            .get(format!(
                "{}/api/auth/info",
                endpoint.base.trim_end_matches('/')
            ))
            .send()
            .map_err(|e| e.to_string())?
            .error_for_status()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())?;
        buffer_println!(
            "{}",
            serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if let ["auth", "change", user] = words {
        let password = std::env::var("PAKAR_NEW_PASSWORD").map_err(
            |_| "Set PAKAR_NEW_PASSWORD privately; do not put passwords in command arguments",
        )?;
        access::validate(user, &password, &password)?;
        let next = Endpoint {
            base: endpoint.base.clone(),
            user: (*user).into(),
            password,
        };
        http_request(
            client,
            HttpCommand::AccessChange(endpoint.clone(), next, true),
        )?;
        buffer_println!(
            "Credentials saved. Update PAKAR_USER/PAKAR_PASSWORD before further commands."
        );
        return Ok(());
    }
    if words == ["auth", "disable"] {
        let next = Endpoint {
            base: endpoint.base.clone(),
            user: String::new(),
            password: String::new(),
        };
        http_request(
            client,
            HttpCommand::AccessChange(endpoint.clone(), next, false),
        )?;
        buffer_println!("Device confirmed authentication disabled and saved.");
        return Ok(());
    }
    if words.is_empty() || words[0] == "status" {
        let status = blocking_status(client, endpoint)?;
        output.push_str(&format_text_status(&status, true));
        return Ok(());
    }
    if matches!(words, ["display", "segments", ..] | ["display", "stop"]) {
        let form = if words == ["display", "stop"] {
            vec![("release".into(), "true".into())]
        } else {
            let (settings, text) = display::parse_command(&words[2..])?;
            settings.form(&text)
        };
        let response = client
            .post(format!(
                "{}/api/display",
                endpoint.base.trim_end_matches('/')
            ))
            .basic_auth(&endpoint.user, Some(&endpoint.password))
            .form(&form)
            .send()
            .map_err(|e| e.to_string())?;
        if !response.status().is_success() {
            return Err(format!(
                "HTTP {}: {}",
                response.status(),
                response.text().unwrap_or_default()
            ));
        }
        buffer_println!("accepted");
        return Ok(());
    }
    let (path, form): (&str, Vec<(&str, &str)>) =
        match words {
            ["mode", value] => ("/api/mode", vec![("value", value)]),
            ["cell", id, state] => (
                "/api/cell",
                vec![("id", id), ("state", state), ("effect", "fade")],
            ),
            ["cell", id, state, effect] => (
                "/api/cell",
                vec![("id", id), ("state", state), ("effect", effect)],
            ),
            ["brightness", value] => ("/api/brightness", vec![("value", value)]),
            ["led-output", "on"] => ("/api/led/output", vec![("enabled", "true")]),
            ["led-output", "off"] => ("/api/led/output", vec![("enabled", "false")]),
            ["led-output", mode @ ("static" | "effects")] => ("/api/led/output", vec![("mode", mode)]),
            ["led-output", backend @ ("gpio-static" | "rmt-dma")] => ("/api/led/output", vec![("backend", backend)]),
            ["effect", value] => ("/api/effect", vec![("value", value)]),
            ["display", value] => ("/api/display", vec![("value", value)]),
            ["audio", cue] => ("/api/audio", vec![("cue", cue)]),
            ["snake", value] => ("/api/snake/direction", vec![("value", value)]),
            ["reset"] => ("/api/game/reset", vec![]),
            _ => return Err(
                "commands: device identity, device rename HOST[.local], device label DISPLAY NAME, device configure HOST DISPLAY NAME, auth info, auth change USER, status, mode, cell, cells IDS STATE [EFFECT], brightness, led-output on|off|static|effects, effect, display, audio, reset"
                    .into(),
            ),
        };
    let backend_peer = if path == "/api/led/output"
        && let Some((_, backend)) = form.iter().find(|(key, _)| *key == "backend") {
        let before = blocking_status(client, endpoint)?;
        if !led_backend::allowed(&before, backend) {
            return Err("LED backend unavailable or switching; nothing sent".into());
        }
        Some(peer_http::Pin::from_status(&before).ok_or("LED backend peer unavailable; nothing sent")?)
    } else { None };
    let response = client
        .post(format!("{}{}", endpoint.base.trim_end_matches('/'), path))
        .basic_auth(&endpoint.user, Some(&endpoint.password))
        .form(&form)
        .send()
        .map_err(|error| error.to_string())?;
    if !response.status().is_success() {
        return Err(format!(
            "HTTP {}: {}",
            response.status(),
            response.text().unwrap_or_default()
        ));
    }
    if path == "/api/led/output" && let Some((_, backend)) = form.iter().find(|(key, _)| *key == "backend") {
        let receipt: serde_json::Value = response.json().map_err(|_| "LED backend receipt unavailable; nothing replayed")?;
        if receipt["accepted"] != true { return Err(format!("LED backend rejected: {}", receipt["error"])); }
        let after = blocking_status(client, endpoint)?;
        if peer_http::Pin::from_status(&after) != backend_peer {
            return Err("LED backend readback belongs to another board/boot; nothing replayed".into());
        }
        led_backend::finish_network(endpoint, after, backend, || Ok(()))?;
        buffer_println!("LED backend {backend} active and saved");
        return Ok(());
    }
    buffer_println!("accepted");
    Ok(())
}

fn run_text_interface(arguments: &[String]) -> Result<bool, String> {
    if !matches!(
        arguments.get(1).map(String::as_str),
        Some("--status" | "--command" | "--tui")
    ) {
        return Ok(false);
    }
    let endpoint = environment_endpoint();
    if arguments[1] == "--tui" {
        terminal_ui::run(endpoint)?;
        return Ok(true);
    }
    let client = peer_identity::client_builder()
        .no_proxy()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|error| error.to_string())?;
    match arguments[1].as_str() {
        "--status" => text_command(&client, &endpoint, &["status"]),
        "--command" => {
            let words = arguments[2..]
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>();
            if words.as_slice() == ["auth", "info"] {
                text_command(&client, &endpoint, &words)?;
                return Ok(true);
            }
            // CLI is a real client too: validate the live release and identity
            // before any command, not just before the table batch helper.
            let status = blocking_status(&client, &endpoint)?;
            let pinned = peer_http::pinned_client(&status)?;
            if status
                .capabilities
                .as_ref()
                .is_some_and(|c| c.has("telemetry"))
            {
                client_telemetry::Reporter::new().report(&pinned, &endpoint);
            }
            text_command(&pinned, &endpoint, &words)
        }
        _ => unreachable!(),
    }?;
    Ok(true)
}

fn main() -> eframe::Result {
    peer_identity::process_started();
    let arguments = std::env::args().collect::<Vec<_>>();
    cli_console::prepare(&arguments);
    if let Some(result) = client_management::cli(&arguments) {
        match result {
            Ok(()) => cli_println!("Client management operation completed"),
            Err(error) => {
                cli_eprintln!("PakarController: {error}");
                std::process::exit(2);
            }
        }
        return Ok(());
    }
    if arguments.get(1).is_some_and(|arg| {
        matches!(
            arg.as_str(),
            "--client-update-init" | "--apply-client-update"
        )
    }) {
        let result = match arguments.get(1).map(String::as_str) {
            Some("--client-update-init") if arguments.len() == 2 => client_install::initialize(),
            Some("--apply-client-update") if arguments.len() == 3 => {
                client_install::helper(std::path::Path::new(&arguments[2]))
            }
            _ => Err("Invalid managed app update arguments".into()),
        };
        match result {
            Ok(()) => cli_println!("Managed app update operation completed"),
            Err(error) => {
                cli_eprintln!("PakarController: {error}");
                std::process::exit(2);
            }
        }
        return Ok(());
    }
    if let Some(output) = cli_output::parse_cli_output(&arguments) {
        cli_println!("{output}");
        return Ok(());
    }
    match run_text_interface(&arguments) {
        Ok(true) => return Ok(()),
        Ok(false) => {}
        Err(error) => {
            cli_eprintln!("PakarController: {error}");
            std::process::exit(2);
        }
    }
    let config = config::AppConfig::load().unwrap_or_default();
    let command = ui::shell::launch_command(&arguments);
    let control_only = arguments.iter().any(|arg| {
        matches!(
            arg.as_str(),
            "--quit"
                | "--app-status"
                | "--wifi"
                | "--usb"
                | "--disconnect"
                | "--led-effect"
                | "--led-mode"
                | "--led-cell"
                | "--led-cells"
                | "--led-output"
                | "--cell-backgrounds"
                | "--update-esp"
                | "--pixel-arm"
                | "--pixel-show"
        )
    });
    if control_only {
        match platform::instance::send(&command, Duration::from_secs(2)) {
            Ok(reply) => {
                cli_println!("{reply}");
                return Ok(());
            }
            Err(error) => {
                cli_eprintln!("{error}");
                std::process::exit(3);
            }
        }
    }
    let _ownership = if config.single_instance {
        match platform::instance::acquire_gui_ownership() {
            Ok(platform::instance::GuiOwnership::Primary(owner)) => Some(owner),
            Ok(platform::instance::GuiOwnership::Existing) => {
                let deadline = Instant::now() + Duration::from_secs(5);
                loop {
                    if let Ok(reply) =
                        platform::instance::send(&command, Duration::from_millis(300))
                    {
                        if reply["accepted"] == true {
                            cli_println!("Forwarded to active PakarController");
                            return Ok(());
                        }
                    }
                    if Instant::now() >= deadline {
                        cli_eprintln!("The active instance did not accept the launch request");
                        std::process::exit(3);
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            }
            Err(error) => {
                cli_eprintln!("{error}");
                std::process::exit(3);
            }
        }
    } else {
        None
    };
    // Set the process/shell identity before eframe creates its first HWND.
    // Isolated framebuffer fixtures retain their no-production-shell policy.
    #[cfg(windows)]
    if !(std::env::var_os("PAKAR_SCREENSHOT_PATH").is_some()
        && std::env::var_os("PAKAR_CONFIG_FILE").is_some()
        && !config.single_instance)
        && let Err(error) = platform::windows_jump_list::install()
    {
        cli_eprintln!("Windows quick actions: {error}");
    }
    // Explicitly sized screenshot fixtures never change normal window policy.
    // All three isolation gates must be present before reading fixture geometry.
    let fixture_size = screenshot_fixture_size(
        std::env::var_os("PAKAR_SCREENSHOT_PATH").is_some()
            && std::env::var_os("PAKAR_CONFIG_FILE").is_some()
            && !config.single_instance,
        std::env::var("PAKAR_SCREENSHOT_WIDTH").ok().as_deref(),
        std::env::var("PAKAR_SCREENSHOT_HEIGHT").ok().as_deref(),
    );
    let options = eframe::NativeOptions {
        // Fixture window geometry/egui storage must not read or overwrite the
        // production app's separate eframe persistence directory.
        persistence_path: std::env::var_os("PAKAR_CONFIG_FILE")
            .filter(|_| !config.single_instance)
            .map(|path| std::path::PathBuf::from(path).with_extension("window-state")),
        viewport: egui::ViewportBuilder::default()
            .with_visible(true)
            .with_active(std::env::var_os("PAKAR_SCREENSHOT_PATH").is_none())
            .with_inner_size(fixture_size.unwrap_or([1180.0, 760.0]))
            .with_maximized(fixture_size.is_none())
            .with_icon(branding::window_icon())
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        application_identity::APP_NAME,
        options,
        Box::new(move |c| {
            let mut app = PakarControllerApp::new(c);
            app.dispatch(command, &c.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

fn screenshot_fixture_size(isolated: bool, width: Option<&str>, height: Option<&str>) -> Option<[f32; 2]> {
    if !isolated {
        return None;
    }
    let width: u32 = width?.parse().ok()?;
    let height: u32 = height?.parse().ok()?;
    ((900..=3840).contains(&width) && (600..=2160).contains(&height))
        .then_some([width as f32, height as f32])
}

fn startup_connection_message(enabled: bool, method: Transport) -> &'static str {
    if !enabled { "" }
    else if method == Transport::Serial { "Connecting through USB" }
    else { "Connecting to PakarController" }
}

#[cfg(test)]
mod tests {
    #[test]
    fn status_bar_startup_message_respects_disconnected_and_selected_route() {
        for method in [super::Transport::WiFi, super::Transport::Serial] {
            assert_eq!(super::startup_connection_message(false, method), "");
        }
        assert_eq!(super::startup_connection_message(true, super::Transport::Serial), "Connecting through USB");
        assert_eq!(super::startup_connection_message(true, super::Transport::WiFi), "Connecting to PakarController");
    }
    #[test]
    fn screenshot_fixture_geometry_requires_isolation_and_bounded_pair() {
        use super::screenshot_fixture_size as size;
        assert_eq!(size(true, Some("900"), Some("600")), Some([900.0, 600.0]));
        assert_eq!(size(true, Some("1280"), Some("800")), Some([1280.0, 800.0]));
        assert_eq!(size(false, Some("900"), Some("600")), None);
        for (width, height) in [(Some("899"), Some("600")), (Some("900"), None),
            (Some("NaN"), Some("600")), (Some("900"), Some("2161"))] {
            assert_eq!(size(true, width, height), None);
        }
    }

    #[test]
    fn cli_console_buffered_status_stays_in_memory() {
        let output = super::format_text_status(&super::DeviceStatus::default(), false);
        assert!(output.contains("LED output Unknown"));
        assert!(output.contains("inputs 00+0 outputs 00"));
    }
    pub(super) fn mock_device(
        responses: Vec<(u16, Vec<u8>)>,
    ) -> (super::Endpoint, std::thread::JoinHandle<Vec<String>>) {
        mock_device_with_hook(responses, Box::new(|_| {}))
    }
    pub(super) fn mock_device_with_hook(
        responses: Vec<(u16, Vec<u8>)>,
        mut on_request: Box<dyn FnMut(usize) + Send>,
    ) -> (super::Endpoint, std::thread::JoinHandle<Vec<String>>) {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = super::Endpoint {
            base: format!("http://{}", listener.local_addr().unwrap()),
            user: "fixture".into(),
            password: "fixture-secret".into(),
        };
        let worker = std::thread::spawn(move || {
            let mut requests = Vec::new();
            for (code, body) in responses {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                    .unwrap();
                let mut bytes = Vec::new();
                let mut buffer = [0; 4096];
                loop {
                    let read = stream.read(&mut buffer).unwrap();
                    assert!(read > 0);
                    bytes.extend_from_slice(&buffer[..read]);
                    if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                        let header = String::from_utf8_lossy(&bytes[..end]);
                        let length = header
                            .lines()
                            .find_map(|line| {
                                line.to_lowercase()
                                    .strip_prefix("content-length:")
                                    .and_then(|n| n.trim().parse::<usize>().ok())
                            })
                            .unwrap_or(0);
                        if bytes.len() >= end + 4 + length {
                            break;
                        }
                    }
                    assert!(bytes.len() < 3 * 1024 * 1024);
                }
                requests.push(String::from_utf8_lossy(&bytes).into_owned());
                on_request(requests.len());
                write!(stream,"HTTP/1.1 {code} Fixture\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",body.len()).unwrap();
                stream.write_all(&body).unwrap();
            }
            requests
        });
        (endpoint, worker)
    }
    pub(super) fn fixture_network() -> super::device_settings::Network {
        super::device_settings::Network {
            station_enabled: true,
            ap_enabled: true,
            station_ssid: "Fixture".into(),
            addressing: Default::default(),
        }
    }
    #[test]
    fn settings_worker_requires_ack_and_exact_readback() {
        let n = fixture_network();
        let (endpoint, server) = mock_device(vec![
            (202, br#"{"accepted":true}"#.to_vec()),
            (200, serde_json::to_vec(&n).unwrap()),
        ]);
        let client = super::Client::builder()
            .no_proxy()
            .timeout(std::time::Duration::from_secs(3))
            .build()
            .unwrap();
        let command =
            super::device_settings::Write::network(41, &n, "fixture-pass", false).unwrap();
        assert!(matches!(
            super::http_request(&client, super::HttpCommand::Settings(endpoint, command)).unwrap(),
            super::Event::SettingsFinished(41, _, Ok(_))
        ));
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 2);
        assert!(requests[0].starts_with("POST /api/network "));
        assert!(requests[1].starts_with("GET /api/network "));
        assert!(requests[0].to_lowercase().contains("authorization: basic "));
    }
    #[test]
    fn mismatched_readback_and_negative_ack_never_replay_write() {
        let client = super::Client::builder()
            .no_proxy()
            .timeout(std::time::Duration::from_secs(3))
            .build()
            .unwrap();
        for negative_ack in [false, true] {
            let mut changed = fixture_network();
            changed.ap_enabled = false;
            let responses = if negative_ack {
                vec![(200, br#"{"accepted":false}"#.to_vec())]
            } else {
                vec![
                    (202, br#"{"accepted":true}"#.to_vec()),
                    (200, serde_json::to_vec(&changed).unwrap()),
                ]
            };
            let (endpoint, server) = mock_device(responses);
            let write = super::device_settings::Write::network(
                9,
                &fixture_network(),
                "fixture-pass",
                false,
            )
            .unwrap();
            assert!(matches!(
                super::http_request(&client, super::HttpCommand::Settings(endpoint, write))
                    .unwrap(),
                super::Event::SettingsFinished(9, _, Err(_))
            ));
            let requests = server.join().unwrap();
            assert_eq!(
                requests.iter().filter(|r| r.starts_with("POST ")).count(),
                1
            );
        }
    }
    #[test]
    fn maintenance_checks_positive_device_confirmation() {
        let client = super::Client::builder().no_proxy().build().unwrap();
        let (endpoint, server) = mock_device(vec![(
            200,
            br#"{"programmed":true,"verified":false}"#.to_vec(),
        )]);
        assert!(
            super::maintenance::run(
                &client,
                &endpoint,
                super::maintenance::Operation::ProgramAvr
            )
            .is_err()
        );
        assert_eq!(server.join().unwrap().len(), 1);
        let (endpoint, server) = mock_device(vec![(201, b"AVR image stored".to_vec())]);
        assert!(
            super::maintenance::run(
                &client,
                &endpoint,
                super::maintenance::Operation::StageAvr(vec![0; 32768])
            )
            .unwrap()
            .contains("NOT programmed")
        );
        let requests = server.join().unwrap();
        assert!(requests[0].starts_with("POST /api/avr/image "));
        assert!(requests[0].contains("multipart/form-data"));
    }
    #[test]
    fn backup_export_rejects_wrong_length_and_preserves_existing_files() {
        let destination =
            std::env::temp_dir().join(format!("pakar-fixture-{}.bin", uuid::Uuid::new_v4()));
        let client = super::Client::builder().no_proxy().build().unwrap();
        let operation = super::maintenance::Operation::Download {
            eeprom: true,
            destination: destination.clone(),
        };
        let (endpoint, server) = mock_device(vec![(200, vec![0; 1023])]);
        assert!(super::maintenance::run(&client, &endpoint, operation.clone()).is_err());
        server.join().unwrap();
        assert!(!destination.exists());
        let (endpoint, server) = mock_device(vec![(200, vec![7; 1024])]);
        assert!(super::maintenance::run(&client, &endpoint, operation.clone()).is_ok());
        server.join().unwrap();
        let (endpoint, server) = mock_device(vec![(200, vec![8; 1024])]);
        assert!(super::maintenance::run(&client, &endpoint, operation).is_err());
        server.join().unwrap();
        assert_eq!(std::fs::read(&destination).unwrap(), vec![7; 1024]);
        std::fs::remove_file(destination).unwrap();
    }
    #[test]
    fn late_observations_are_bound_to_their_requested_peer() {
        let current = super::Endpoint {
            base: "http://ledboard.local".into(),
            user: "test".into(),
            password: "old".into(),
        };
        let mut other = current.clone();
        assert!(
            super::current_event(
                super::Event::SerialFailure("COM8".into(), "removed".into()),
                &current,
                "COM9"
            )
            .is_none()
        );
        other.base = "http://other.local".into();
        assert!(
            super::current_event(
                super::for_endpoint(&other, super::Event::Status(super::DeviceStatus::default())),
                &current,
                "COM9"
            )
            .is_none()
        );
        other = current.clone();
        other.password = "new".into();
        for event in [
            super::Event::DiagnosticFailure("HTTP".into(), "offline".into()),
            super::Event::DiagnosticRecovered("HTTP".into()),
        ] {
            assert!(
                super::current_event(super::for_endpoint(&other, event), &current, "COM9")
                    .is_none()
            );
        }
        assert!(
            super::current_event(
                super::for_endpoint(&other, super::Event::WebSocket(false)),
                &current,
                "COM9"
            )
            .is_none()
        );
        assert!(
            super::current_event(
                super::for_endpoint(&current, super::Event::WebSocket(true)),
                &current,
                "COM9"
            )
            .is_some()
        );
        assert!(
            super::current_event(
                super::Event::Serial("COM8".into(), true, None),
                &current,
                "COM9"
            )
            .is_none()
        );
        assert!(
            super::current_event(
                super::Event::Serial("COM9".into(), true, None),
                &current,
                "COM9"
            )
            .is_some()
        );
    }
    use super::*;
    #[test]
    fn diagnostic_sources_separate_reads_and_never_coalesce_explicit_writes() {
        let peer = Endpoint {
            base: "http://ledboard.local".into(),
            user: "fixture".into(),
            password: "secret".into(),
        };
        assert_eq!(
            HttpCommand::Status(peer.clone())
                .diagnostic_source()
                .as_deref(),
            Some("HTTP GET /api/status")
        );
        assert_ne!(
            HttpCommand::Mapping(peer.clone(), 1).diagnostic_source(),
            HttpCommand::Status(peer.clone()).diagnostic_source()
        );
        assert!(
            HttpCommand::Post(peer.clone(), "/api/cell", vec![])
                .diagnostic_source()
                .is_none()
        );
        assert!(
            HttpCommand::NameChange(peer, "board".into(), None)
                .diagnostic_source()
                .is_none()
        );
    }
    #[test]
    fn selected_transport_never_falls_back_to_the_other_method() {
        for wifi in [false, true] {
            for usb in [false, true] {
                assert_eq!(
                    command_route(Transport::WiFi, wifi, usb),
                    wifi.then_some(Transport::WiFi)
                );
                assert_eq!(
                    command_route(Transport::Serial, wifi, usb),
                    usb.then_some(Transport::Serial)
                );
            }
        }
    }
    #[test]
    fn usb_selected_http_worker_never_contacts_even_a_live_network_endpoint() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let endpoint = Endpoint {
            base: format!("http://{}", listener.local_addr().unwrap()),
            user: "fixture".into(),
            password: "fixture".into(),
        };
        let gate = connection::Gate::default();
        gate.select(Some(Transport::Serial));
        let (commands, rx) = mpsc::channel();
        let (events, output) = mpsc::channel();
        let worker = thread::spawn(move || http_worker(rx, events, egui::Context::default(), gate));
        commands.send(HttpCommand::Status(endpoint)).unwrap();
        drop(commands);
        worker.join().unwrap();
        assert!(matches!(listener.accept(), Err(error) if error.kind()==ErrorKind::WouldBlock));
        assert!(
            output.try_recv().is_err(),
            "inactive network must not emit failure noise"
        );
    }
    #[test]
    fn discovery_cached_read_fallback_is_credential_free_and_never_replays_a_mutation() {
        fn wire() -> Vec<u8> {
            serde_json::to_vec(&serde_json::json!({"version":peer_identity::VERSION,"hostname":"cached-discovery-fixture","board_id":"esp32s3-discovery","boot_id":"a".repeat(32),"http_peer_fencing":true,
            "capabilities":{"board_kind":"table","acknowledged_features":["table","leds"],"table":{"columns":10,"rows":8,"pixels_per_cell":6}}})).unwrap()
        }
        let responses = vec![
            (500, vec![]),
            (200, wire()),
            (200, br#"{"enabled":false}"#.to_vec()),
        ];
        let (numeric, server) = mock_device(responses);
        let socket = reqwest::Url::parse(&numeric.base)
            .unwrap()
            .socket_addrs(|| None)
            .unwrap()[0];
        let mut endpoint = numeric.clone();
        endpoint.base = format!("http://localhost:{}", socket.port());
        endpoint.user = "saved".into();
        endpoint.password = "private-fixture".into();
        let status: DeviceStatus = serde_json::from_slice(&wire()).unwrap();
        peer_http::invalidate_address(&endpoint.base);
        let saved = peer_http::SavedRoute {
            endpoint: endpoint.base.clone(),
            numeric_origin: numeric.base.clone(),
            board_id: status.board_id.clone(),
        };
        peer_http::restore_route(&saved, &endpoint.base);
        assert!(
            peer_http::verified_route(&endpoint.base, &status).is_none(),
            "Restoring a profile must not seed a verified DNS route"
        );
        let gate = connection::Gate::default();
        gate.select(Some(Transport::WiFi));
        let (commands, rx) = mpsc::channel();
        let (events, output) = mpsc::channel();
        let queue = peer_http::Queue::new(commands);
        queue.send(HttpCommand::Status(endpoint.clone())).unwrap();
        drop(queue);
        let worker = thread::spawn(move || http_worker(rx, events, egui::Context::default(), gate));
        worker.join().unwrap();
        fn unwrap(event: Event) -> Event {
            match event {
                Event::ForScope(_, inner)
                | Event::ForConnection(_, inner)
                | Event::ForEndpoint(_, inner) => unwrap(*inner),
                other => other,
            }
        }
        assert!(
            output
                .try_iter()
                .map(unwrap)
                .any(|event| matches!(event,Event::Status(s) if s.board_id=="esp32s3-discovery"))
        );
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 3);
        assert!(requests.iter().all(|r| r.starts_with("GET ")));
        assert!(
            !requests[1].to_ascii_lowercase().contains("authorization:"),
            "Public fallback must not send selected credentials to a candidate IP"
        );
        peer_http::invalidate_address(&endpoint.base);
        let (numeric, server) = mock_device(vec![(500, vec![])]);
        let socket = reqwest::Url::parse(&numeric.base)
            .unwrap()
            .socket_addrs(|| None)
            .unwrap()[0];
        endpoint.base = format!("http://cached-discovery-fixture.local:{}", socket.port());
        peer_http::remember_address(&endpoint.base, &status, Some(socket));
        let gate = connection::Gate::default();
        gate.select(Some(Transport::WiFi));
        let (commands, rx) = mpsc::channel();
        let (events, _) = mpsc::channel();
        commands
            .send(HttpCommand::Post(
                endpoint.clone(),
                "/api/cell",
                vec![("id".into(), "1".into()), ("state".into(), "red".into())],
            ))
            .unwrap();
        drop(commands);
        http_worker(rx, events, egui::Context::default(), gate);
        let requests = server.join().unwrap();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].starts_with("POST /api/cell "));
        peer_http::invalidate_address(&endpoint.base);
        // A reused saved IP with another compatible board is still rejected,
        // even though it serves the same hostname and a successful status.
        let mut foreign: serde_json::Value = serde_json::from_slice(&wire()).unwrap();
        foreign["board_id"] = "unexpected-board".into();
        let (numeric, server) = mock_device(vec![
            (500, vec![]),
            (200, serde_json::to_vec(&foreign).unwrap()),
            (200, br#"{"enabled":false}"#.to_vec()),
        ]);
        let socket = reqwest::Url::parse(&numeric.base)
            .unwrap()
            .socket_addrs(|| None)
            .unwrap()[0];
        endpoint.base = format!("http://localhost:{}", socket.port());
        peer_http::invalidate_address(&endpoint.base);
        peer_http::restore_route(
            &peer_http::SavedRoute {
                endpoint: endpoint.base.clone(),
                numeric_origin: numeric.base,
                board_id: status.board_id.clone(),
            },
            &endpoint.base,
        );
        let gate = connection::Gate::default();
        gate.select(Some(Transport::WiFi));
        let (commands, rx) = mpsc::channel();
        let (events, output) = mpsc::channel();
        let queue = peer_http::Queue::new(commands);
        queue.send(HttpCommand::Status(endpoint.clone())).unwrap();
        drop(queue);
        http_worker_with_discovery(rx, events, egui::Context::default(), gate, |_| Vec::new());
        assert!(
            !output
                .try_iter()
                .map(unwrap)
                .any(|event| matches!(event, Event::Status(_) | Event::DiscoveredBoard(..))),
            "A saved IP must not silently acquire another board"
        );
        assert!(peer_http::verified_route(&endpoint.base, &status).is_none());
        assert_eq!(server.join().unwrap().len(), 3);
        peer_http::invalidate_address(&endpoint.base);
    }
    #[test]
    fn discovery_startup_preference_is_separate_from_current_session_disconnect() {
        let mut config = config::AppConfig::default();
        config.connection_enabled = false;
        assert!(config.startup_connection_enabled(false));
        assert!(!config.startup_connection_enabled(true));
        config.auto_connect = false;
        assert!(!config.startup_connection_enabled(false));
        config.auto_connect = true;
        config.connection_method = Transport::Serial;
        assert!(!config.startup_connection_enabled(false));
        config.serial_port = "COM22".into();
        assert!(config.startup_connection_enabled(false));
        let (tx, _) = mpsc::channel();
        let mut queue = peer_http::Queue::new(tx);
        queue.set_auto_discovery(false);
        assert!(!queue.capture().auto_discovery);
        let before = queue.capture();
        queue.clear();
        assert!(
            !before.current(),
            "Explicit owner retirement must reject a late discovered board"
        );
    }
    #[test]
    fn discovery_new_target_policy_does_not_depend_on_previous_origin_spelling() {
        for origin in [
            "http://192.0.2.99",
            "http://localhost:18082",
            "https://previous.example",
        ] {
            let (tx, _) = mpsc::channel();
            let mut queue = peer_http::Queue::new(tx);
            let endpoint = Endpoint {
                base: origin.into(),
                user: String::new(),
                password: String::new(),
            };
            let read = HttpCommand::Scoped(
                queue.capture(),
                Box::new(HttpCommand::Status(endpoint.clone())),
            );
            assert!(auto_discovery_allowed(&read));
            queue.clear();
            assert!(
                !auto_discovery_allowed(&read),
                "A late scope is not permission to reconnect"
            );
            queue.set_auto_discovery(false);
            assert!(!auto_discovery_allowed(&HttpCommand::Scoped(
                queue.capture(),
                Box::new(HttpCommand::Status(endpoint.clone()))
            )));
            assert!(!auto_discovery_allowed(&HttpCommand::Scoped(
                queue.capture(),
                Box::new(HttpCommand::Post(endpoint, "/api/cell", vec![]))
            )));
        }
    }
    #[test]
    fn discovery_configuration_events_require_current_peer_and_complete_typed_notice() {
        let pin = peer_http::Pin::from_identity("board-a", &"a".repeat(32)).unwrap();
        let value = serde_json::json!({"type":"configuration-changed","board_id":"board-a","boot_id":"a".repeat(32),"revision":3,"phase":"saved","configured_hostname":"not-active-yet"});
        let saved = configuration_event_notice(&value, &pin).unwrap();
        assert_eq!(saved.2, 3);
        let mut applied = value.clone();
        applied["phase"] = "applied".into();
        assert_ne!(configuration_event_notice(&applied, &pin), Some(saved));
        for field in ["board_id", "boot_id", "revision", "phase"] {
            let mut bad = value.clone();
            bad[field] = serde_json::Value::Null;
            assert!(configuration_event_notice(&bad, &pin).is_none());
        }
        let mut foreign = value;
        foreign["boot_id"] = "b".repeat(32).into();
        assert!(configuration_event_notice(&foreign, &pin).is_none());
    }
    #[test]
    fn discovery_verified_rename_retires_pending_configuration_slot_without_waiting_for_timeout() {
        let (tx, _) = mpsc::channel();
        let mut owner = peer_http::Queue::new(tx);
        let mut reads = configuration_reads::Queue::default();
        let old_scope = owner.capture();
        assert_eq!(reads.take_due(), Some("/api/network"));
        // Same two owner retirements as the admitted active-rename branch.
        owner.clear();
        reads.reset();
        reads.reset_all(true, true);
        assert!(
            !old_scope.current(),
            "The old response cannot reach a new settings owner"
        );
        assert_eq!(
            reads.take_due(),
            Some("/api/auth/info"),
            "New-host refresh must not wait 24 seconds for the retired read"
        );
        reads.finish("/api/network", true);
        assert!(
            reads.take_due().is_none(),
            "An unrelated old result cannot release the current slot"
        );
        reads.finish("/api/auth/info", true);
        assert_eq!(reads.take_due(), Some("/api/network"));
    }
    #[test]
    fn offline_usb_preferences_do_not_need_a_valid_network_endpoint() {
        let config: config::AppConfig = serde_json::from_value(serde_json::json!({
            "connection_method":"Serial", "connection_enabled":true, "serial_port":"COM22",
            "endpoint":"not a network URL"}))
        .unwrap();
        assert_eq!(config.connection_method, Transport::Serial);
        assert!(config.connection_enabled);
        let roundtrip: config::AppConfig =
            serde_json::from_slice(&serde_json::to_vec(&config).unwrap()).unwrap();
        assert_eq!(roundtrip.connection_method, Transport::Serial);
        assert_eq!(roundtrip.serial_port, "COM22");
        assert_eq!(roundtrip.endpoint, "not a network URL");
    }
    #[test]
    fn wire_round_trip() {
        let encoded = wire_frame(OP_CELL_ACTION, 7, &[80, 2, 1]);
        let (op, payload) = decode_frame(&encoded[..encoded.len() - 1]).unwrap();
        assert_eq!(op, OP_CELL_ACTION);
        assert_eq!(payload, vec![80, 2, 1]);
    }
    #[test]
    fn opcode_namespace_rejects_foreign_frames_without_fallback() {
        for discriminator in 1..=255_u8 {
            let mut raw = vec![wire::MAGIC, discriminator, wire::AUDIO_CUE, 7, 1, 4];
            raw.push(crc8(&raw));
            let encoded = cobs_encode(&raw);
            assert!(decode_envelope(&encoded[..encoded.len()-1]).is_none());
        }
        let valid = wire_frame(wire::AUDIO_CUE, 7, &[4]);
        assert_eq!(decode_frame(&valid[..valid.len()-1]), Some((wire::AUDIO_CUE, vec![4])));
        let unknown = wire_frame(0xfc, 7, &[]);
        assert_eq!(decode_frame(&unknown[..unknown.len()-1]), Some((0xfc, vec![])));
    }
    #[test]
    fn shared_cobs_vectors() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("../../tests/fixtures/cobs.json")).unwrap();
        let bytes = |v: &serde_json::Value| v.as_array().unwrap().iter().map(|v| u8::try_from(v.as_u64().unwrap()).unwrap()).collect::<Vec<_>>();
        for value in vectors["valid"].as_array().unwrap() {
            let encoded=bytes(&value["encoded"]);let decoded=bytes(&value["decoded"]);
            assert_eq!(cobs_decode(&encoded),Some(decoded.clone()));
            let mut actual=cobs_encode(&decoded);assert_eq!(actual.pop(),Some(0));
            assert_eq!(actual,encoded);
        }
        for value in vectors["invalid"].as_array().unwrap() { assert!(cobs_decode(&bytes(value)).is_none()); }
        assert_eq!(crc8(&bytes(&vectors["crc8_atm"]["input"])),vectors["crc8_atm"]["checksum"].as_u64().unwrap() as u8);
    }
    #[test]
    fn streaming_packet_bounds() {
        let p = rainbow_packet(240, 240, true, 20);
        assert_eq!(p.len(), 730);
        assert_eq!(&p[..4], b"PKAR");
        assert_eq!(p[9], 1);
    }
    #[test]
    fn cell_states_cycle() {
        assert_eq!(PakarControllerApp::next_cell_state("white"), ("red", 1));
        assert_eq!(PakarControllerApp::next_cell_state("red"), ("off", 2));
    }
    #[test]
    fn serial_display_state_extension_preserves_old_status() {
        let mut payload = vec![0; 19];
        payload[0] = 3;
        let old = serial_status(&payload).unwrap();
        assert_eq!(old.mode, "snake");
        assert!(old.tm1637_segments.is_empty());
        payload.extend([1, 2, 4, 8, 5, 1]);
        let current = serial_status(&payload).unwrap();
        assert_eq!(current.tm1637_segments, [1, 2, 4, 8]);
        assert_eq!(current.tm1637_brightness, 5);
        assert!(current.tm1637_host_active);
        payload[24] = 0;
        assert!(!serial_status(&payload).unwrap().tm1637_host_active);
        assert!(serial_status(&payload[..18]).is_none());
        assert!(current.audio_cues.is_empty() && current.avr_version.is_none());
        payload.extend([0, 2, 5, 14]);
        payload[5] = 1;
        let catalog = serial_status(&payload).unwrap();
        assert_eq!(catalog.avr_version.as_deref(), Some("0.2.5"));
        assert_eq!(catalog.audio_cues.len(), 14);
        assert_eq!(catalog.audio_cues[13], "welcome-40led");
        payload[28] = 255;
        assert!(serial_status(&payload).unwrap().audio_cues.is_empty());
        payload[28] = 14;
        payload[5] = 0;
        assert!(serial_status(&payload).unwrap().audio_cues.is_empty());
        assert!(serial_status(&payload).unwrap().avr_version.is_none());
    }
    #[test]
    fn led_output_status_never_invents_enabled_for_missing_or_invalid_usb_extension() {
        let mut payload = vec![0; 42];
        assert!(serial_status(&payload).unwrap().led_output.is_null());
        for (byte, expected) in [(0, false), (1, true)] {
            payload.resize(43, byte);
            payload[42] = byte;
            assert_eq!(
                serial_status(&payload).unwrap().led_output["enabled"],
                expected
            );
        }
        payload[42] = 255;
        assert!(serial_status(&payload).unwrap().led_output.is_null());
        payload[42] = 1;
        assert!(led_output_mode(&serial_status(&payload).unwrap()).is_none());
        payload.push(0);
        assert_eq!(
            led_output_mode(&serial_status(&payload).unwrap()),
            Some("static")
        );
        payload[43] = 1;
        assert_eq!(
            led_output_mode(&serial_status(&payload).unwrap()),
            Some("effects")
        );
        payload[43] = 255;
        assert!(led_output_mode(&serial_status(&payload).unwrap()).is_none());
        let unknown: DeviceStatus =
            serde_json::from_value(serde_json::json!({"led_output":{"mode":"future"}})).unwrap();
        assert!(led_output_mode(&unknown).is_none());
        let status = DeviceStatus::default();
        assert!(format_text_status(&status, false).contains("LED output Unknown"));
    }
    #[test]
    fn a_transport_failure_does_not_retire_a_fresh_peer_observation() {
        assert!(status_observation_fresh(Some(Instant::now())));
        assert!(!status_observation_fresh(None));
        assert!(!status_observation_fresh(Some(
            Instant::now() - Duration::from_secs(9)
        )));
        assert!(!websocket_status_ready(&DeviceStatus::default()));
        let ready: DeviceStatus = serde_json::from_value(serde_json::json!({"hostname":"ledboard",
            "capabilities":{"board_kind":"table","acknowledged_features":["table"]}}))
        .unwrap();
        assert!(websocket_status_ready(&ready));
        for non_status in [
            r#"{"accepted":true,"message":"ok"}"#,
            r#"{"type":"pong"}"#,
            r#"{"error":"not a board status"}"#,
        ] {
            assert!(websocket_status(non_status).is_none());
        }
        assert!(websocket_status(r#"{"hostname":"ledboard","capabilities":{"board_kind":"table","acknowledged_features":["table"]}}"#).is_some());
    }
    #[test]
    fn mapping_requires_all_unique_cells_but_optional_labels_can_be_absent() {
        let valid: CellMappingResponse =
            serde_json::from_value(serde_json::json!({"physical":(1..=80).collect::<Vec<_>>() }))
                .unwrap();
        let decoded = valid.decode().unwrap();
        assert_eq!(decoded.len(), 80);
        assert_eq!(decoded[79].name, "80");
        let duplicate: CellMappingResponse =
            serde_json::from_value(serde_json::json!({"physical":vec![1;80]})).unwrap();
        assert!(duplicate.decode().is_err());
        assert!(CellMappingResponse::default().decode().is_err());
    }
}
