//! Current device configuration contracts. No fabricated defaults or secrets on disk.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Network,
    Cloud,
    StatusRgb,
    Ethernet,
}
impl Group {
    pub fn path(self) -> &'static str {
        match self {
            Self::Network => "/api/network",
            Self::Cloud => "/api/cloud",
            Self::StatusRgb => "/api/status-rgb",
            Self::Ethernet => "/api/network/ethernet",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Network {
    pub station_enabled: bool,
    pub ap_enabled: bool,
    pub station_ssid: String,
    #[serde(flatten)]
    pub addressing: Addressing,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Ethernet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lldp_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lldp_receive: Option<bool>,
    #[serde(flatten)]
    pub addressing: Addressing,
}
impl Ethernet {
    fn read(value: serde_json::Value) -> Result<Self, String> {
        if value["configured"] != true {
            return Err("Ethernet is not supported by this firmware".into());
        }
        let config: Self = serde_json::from_value(value)
            .map_err(|e| format!("Invalid Ethernet configuration: {e}"))?;
        if !config.addressing.observed() {
            return Err("Complete Ethernet addressing settings are required".into());
        }
        config.addressing.validate()?;
        Ok(config)
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Addressing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dhcp: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns2: Option<String>,
}
impl Addressing {
    pub fn observed(&self) -> bool {
        self.dhcp.is_some()
            && [
                &self.address,
                &self.gateway,
                &self.subnet,
                &self.dns1,
                &self.dns2,
            ]
            .iter()
            .all(|v| v.is_some())
    }
    fn validate(&self) -> Result<(), String> {
        if *self == Self::default() {
            return Ok(());
        }
        if !self.observed() {
            return Err("Incomplete device addressing settings; refresh before editing".into());
        }
        let mut values = [0u32; 5];
        for (index, text) in [
            &self.address,
            &self.gateway,
            &self.subnet,
            &self.dns1,
            &self.dns2,
        ]
        .iter()
        .enumerate()
        {
            let text = text.as_deref().unwrap_or_default();
            if index >= 3 && text.is_empty() {
                continue;
            }
            values[index] = u32::from(
                text.parse::<std::net::Ipv4Addr>()
                    .map_err(|_| "Enter valid IPv4 address, gateway, subnet and DNS values")?,
            );
        }
        if self.dhcp == Some(true) {
            return Ok(());
        }
        let [address, gateway, subnet, dns1, dns2] = values;
        let host = !subnet;
        if host < 3
            || host == u32::MAX
            || host & host.wrapping_add(1) != 0
            || address >> 24 == 0
            || address >> 24 >= 224
            || address & host == 0
            || address & host == host
            || gateway >> 24 == 0
            || gateway >> 24 >= 224
            || gateway & host == 0
            || gateway & host == host
            || address & subnet != gateway & subnet
            || address == gateway
            || dns1 >> 24 >= 224
            || dns2 >> 24 >= 224
        {
            return Err("Invalid static subnet, host, gateway or DNS address".into());
        }
        Ok(())
    }
    fn form(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        if let Some(dhcp) = self.dhcp {
            result.push(("dhcp".into(), dhcp.to_string()));
        }
        for (name, value) in [
            ("address", &self.address),
            ("gateway", &self.gateway),
            ("subnet", &self.subnet),
            ("dns1", &self.dns1),
            ("dns2", &self.dns2),
        ] {
            if let Some(value) = value {
                result.push((name.into(), value.clone()));
            }
        }
        result
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Cloud {
    pub mqtt_enabled: bool,
    pub mqtt_tls: bool,
    pub mqtt_host: String,
    pub mqtt_port: u16,
    pub mqtt_user: String,
    pub mqtt_base_topic: String,
    pub mqtt_password_set: bool,
    pub websocket_enabled: bool,
    pub websocket_tls: bool,
    pub websocket_host: String,
    pub websocket_port: u16,
    pub websocket_path: String,
    pub websocket_token_set: bool,
    pub telemetry_seconds: u16,
    pub ntfy_enabled: Option<bool>,
    pub ntfy_host: Option<String>,
    pub ntfy_topic: Option<String>,
    pub ntfy_token_set: Option<bool>,
}
fn bounded(value: &str, max: usize, label: &str) -> Result<(), String> {
    if value.len() > max || value.chars().any(char::is_control) {
        return Err(format!(
            "{label} must be at most {max} UTF-8 bytes, without control characters"
        ));
    }
    Ok(())
}
fn host(value: &str, enabled: bool, label: &str) -> Result<(), String> {
    bounded(value, 63, label)?;
    if (enabled && value.is_empty())
        || value
            .bytes()
            .any(|c| !(c.is_ascii_alphanumeric() || b".-".contains(&c)))
    {
        return Err(format!(
            "{label} must be a hostname or IPv4 address, without a scheme or path"
        ));
    }
    Ok(())
}
impl Network {
    pub fn validate(&self) -> Result<(), String> {
        self.validate_with_ethernet(false)
    }
    fn validate_with_ethernet(&self, ethernet_ready: bool) -> Result<(), String> {
        bounded(&self.station_ssid, 32, "SSID")?;
        if !self.station_enabled && !self.ap_enabled && !ethernet_ready {
            return Err(
                "Keep station or recovery AP enabled until Ethernet has a working IPv4 link".into(),
            );
        }
        if self.station_enabled && self.station_ssid.is_empty() {
            return Err("Select or enter the station SSID".into());
        }
        self.addressing.validate()
    }
}
impl Cloud {
    pub fn validate(&self) -> Result<(), String> {
        if self.ntfy_enabled.is_some()
            || self.ntfy_host.is_some()
            || self.ntfy_topic.is_some()
            || self.ntfy_token_set.is_some()
        {
            let (Some(enabled), Some(server), Some(topic), Some(_)) = (
                self.ntfy_enabled,
                self.ntfy_host.as_ref(),
                self.ntfy_topic.as_ref(),
                self.ntfy_token_set,
            ) else {
                return Err("Read complete ntfy settings first".into());
            };
            host(server, true, "ntfy server")?;
            bounded(topic, 64, "ntfy topic")?;
            if (enabled && topic.is_empty())
                || !topic
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"_-".contains(&c))
            {
                return Err("ntfy needs a single topic using letters, numbers, - or _".into());
            }
        }
        host(&self.mqtt_host, self.mqtt_enabled, "MQTT broker")?;
        host(
            &self.websocket_host,
            self.websocket_enabled,
            "WebSocket host",
        )?;
        bounded(&self.mqtt_user, 32, "MQTT username")?;
        bounded(&self.mqtt_base_topic, 64, "MQTT topic")?;
        bounded(&self.websocket_path, 96, "WebSocket path")?;
        // Current firmware's public JSON strings are not yet universally escaped.
        for text in [&self.mqtt_user, &self.mqtt_base_topic, &self.websocket_path] {
            if text.contains(['"', '\\']) {
                return Err("Current firmware does not support quotes or backslashes in these public settings".into());
            }
        }
        if self.mqtt_base_topic.is_empty() || self.mqtt_base_topic.contains(['+', '#']) {
            return Err("Use a nonempty MQTT topic without wildcard characters".into());
        }
        if self.mqtt_port == 0 || self.websocket_port == 0 {
            return Err("Ports must be 1–65535".into());
        }
        if !self.websocket_path.starts_with('/') {
            return Err("WebSocket path must start with /".into());
        }
        if !(1..=3600).contains(&self.telemetry_seconds) {
            return Err("Telemetry interval must be 1–3600 seconds".into());
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SecretMode {
    #[default]
    Keep,
    Replace,
    Clear,
}
#[derive(Clone, Default, PartialEq, Eq)]
pub struct SecretEdit {
    pub mode: SecretMode,
    pub value: String,
}
impl SecretEdit {
    pub fn form_value(&self) -> Result<Option<String>, String> {
        match self.mode {
            SecretMode::Keep => Ok(None),
            SecretMode::Clear => Ok(Some(String::new())),
            SecretMode::Replace => {
                bounded(&self.value, 64, "Secret")?;
                if self.value.is_empty() {
                    return Err("Enter a replacement secret, or explicitly choose Clear".into());
                }
                Ok(Some(self.value.clone()))
            }
        }
    }
    pub fn expected_presence(&self, current: bool) -> bool {
        match self.mode {
            SecretMode::Keep => current,
            SecretMode::Replace => true,
            SecretMode::Clear => false,
        }
    }
}
#[derive(Clone)]
pub enum Expected {
    Network(Network),
    Cloud(Cloud),
    StatusRgb(crate::status_rgb::Configuration),
    Ethernet(Ethernet),
}
#[derive(Clone)]
pub struct Write {
    pub id: u64,
    pub group: Group,
    pub form: Vec<(String, String)>,
    pub expected: Expected,
}
impl Write {
    pub fn ethernet(id: u64, config: &Ethernet) -> Result<Self, String> {
        if !config.addressing.observed() {
            return Err("Read complete Ethernet settings first".into());
        }
        config.addressing.validate()?;
        let mut expected = config.clone();
        if expected.addressing.dhcp == Some(false) {
            for value in [&mut expected.addressing.dns1, &mut expected.addressing.dns2] {
                if value.as_deref() == Some("") {
                    *value = Some("0.0.0.0".into());
                }
            }
        }
        let mut form = expected.addressing.form();
        if let Some(enabled) = expected.enabled {
            form.push(("enabled".into(), enabled.to_string()));
        }
        if let Some(enabled) = expected.lldp_enabled {
            form.push(("lldp_enabled".into(), enabled.to_string()));
        }
        if let Some(enabled) = expected.lldp_receive {
            form.push(("lldp_receive".into(), enabled.to_string()));
        }
        Ok(Self {
            id,
            group: Group::Ethernet,
            form,
            expected: Expected::Ethernet(expected),
        })
    }
    pub fn status_rgb(id: u64, config: &crate::status_rgb::Configuration) -> Result<Self, String> {
        Ok(Self {
            id,
            group: Group::StatusRgb,
            form: config.form()?,
            expected: Expected::StatusRgb(config.clone()),
        })
    }
    pub fn network(id: u64, config: &Network, password: &str, open: bool) -> Result<Self, String> {
        Self::network_with_ethernet(id, config, password, open, false)
    }
    fn network_with_ethernet(
        id: u64,
        config: &Network,
        password: &str,
        open: bool,
        ethernet_ready: bool,
    ) -> Result<Self, String> {
        config.validate_with_ethernet(ethernet_ready)?;
        if !password.is_empty() && !open && !(8..=63).contains(&password.len()) {
            return Err("Enter an 8–63 byte replacement Wi-Fi password, or keep the existing secret by leaving this blank.".into());
        }
        bounded(password, 63, "Wi-Fi password")?;
        let mut expected = config.clone();
        if expected.addressing.dhcp == Some(false) {
            for value in [&mut expected.addressing.dns1, &mut expected.addressing.dns2] {
                if value.as_deref() == Some("") {
                    *value = Some("0.0.0.0".into());
                }
            }
        }
        let mut form = vec![
            ("station_enabled".into(), config.station_enabled.to_string()),
            ("ap_enabled".into(), config.ap_enabled.to_string()),
            ("ssid".into(), config.station_ssid.clone()),
        ];
        form.extend(expected.addressing.form());
        if open || !password.is_empty() {
            form.push((
                "password".into(),
                if open { String::new() } else { password.into() },
            ));
        }
        Ok(Self {
            id,
            group: Group::Network,
            expected: Expected::Network(expected),
            form,
        })
    }
    pub fn cloud(
        id: u64,
        config: &Cloud,
        password: &SecretEdit,
        token: &SecretEdit,
        ntfy_token: &SecretEdit,
    ) -> Result<Self, String> {
        config.validate()?;
        let mut expected = config.clone();
        expected.mqtt_password_set = password.expected_presence(config.mqtt_password_set);
        expected.websocket_token_set = token.expected_presence(config.websocket_token_set);
        if let Some(present) = config.ntfy_token_set {
            expected.ntfy_token_set = Some(ntfy_token.expected_presence(present));
        }
        let mut form = vec![
            ("mqtt_enabled".into(), config.mqtt_enabled.to_string()),
            ("mqtt_tls".into(), config.mqtt_tls.to_string()),
            ("mqtt_host".into(), config.mqtt_host.clone()),
            ("mqtt_port".into(), config.mqtt_port.to_string()),
            ("mqtt_user".into(), config.mqtt_user.clone()),
            ("mqtt_base_topic".into(), config.mqtt_base_topic.clone()),
            (
                "websocket_enabled".into(),
                config.websocket_enabled.to_string(),
            ),
            ("websocket_tls".into(), config.websocket_tls.to_string()),
            ("websocket_host".into(), config.websocket_host.clone()),
            ("websocket_port".into(), config.websocket_port.to_string()),
            ("websocket_path".into(), config.websocket_path.clone()),
            (
                "telemetry_seconds".into(),
                config.telemetry_seconds.to_string(),
            ),
        ];
        if let Some(value) = password.form_value()? {
            form.push(("mqtt_password".into(), value));
        }
        if let Some(value) = token.form_value()? {
            form.push(("websocket_token".into(), value));
        }
        if let Some(enabled) = config.ntfy_enabled {
            form.push(("ntfy_enabled".into(), enabled.to_string()));
            form.push((
                "ntfy_host".into(),
                config.ntfy_host.clone().unwrap_or_default(),
            ));
            form.push((
                "ntfy_topic".into(),
                config.ntfy_topic.clone().unwrap_or_default(),
            ));
            if let Some(value) = ntfy_token.form_value()? {
                form.push(("ntfy_token".into(), value));
            }
        }
        Ok(Self {
            id,
            group: Group::Cloud,
            form,
            expected: Expected::Cloud(expected),
        })
    }
    pub fn verify_ack(&self, value: &serde_json::Value) -> Result<(), String> {
        let field = match self.group {
            Group::Network | Group::Ethernet => "accepted",
            Group::Cloud => "updated",
            Group::StatusRgb => "saved",
        };
        if value[field].as_bool() != Some(true) {
            return Err("Device did not confirm a saved configuration write".into());
        }
        Ok(())
    }
    pub fn verify_readback(&self, value: &serde_json::Value) -> Result<(), String> {
        let matches = match &self.expected {
            Expected::Network(expected) => serde_json::from_value::<Network>(value.clone())
                .is_ok_and(|actual| actual == *expected),
            Expected::Cloud(expected) => serde_json::from_value::<Cloud>(value.clone())
                .is_ok_and(|actual| actual == *expected),
            Expected::StatusRgb(expected) => {
                crate::status_rgb::Configuration::read(value.clone())
                    .is_ok_and(|actual| actual == *expected)
                    && value["persisted"] == true
            }
            Expected::Ethernet(expected) => {
                Ethernet::read(value.clone()).is_ok_and(|actual| actual == *expected)
            }
        };
        if matches {
            Ok(())
        } else {
            Err("Write was accepted, but saved settings could not be confirmed. Refresh before retrying; no write was replayed.".into())
        }
    }
}
#[derive(Clone)]
pub struct Draft<T> {
    pub saved: T,
    pub value: T,
    pub conflict: bool,
}
impl<T: Clone + PartialEq> Draft<T> {
    pub fn new(value: T) -> Self {
        Self {
            saved: value.clone(),
            value,
            conflict: false,
        }
    }
    pub fn dirty(&self) -> bool {
        self.value != self.saved
    }
    pub fn observe(&mut self, value: T) {
        if !self.dirty() {
            *self = Self::new(value);
        } else if self.saved != value {
            self.conflict = true;
        }
    }
    pub fn discard(&mut self) {
        self.value = self.saved.clone();
        self.conflict = false;
    }
}
#[derive(Default)]
pub struct Editor {
    pub open: bool,
    pub page: usize,
    pub search: String,
    pub network: Option<Draft<Network>>,
    pub ethernet: Option<Draft<Ethernet>>,
    pub ethernet_ready: bool,
    pub cloud: Option<Draft<Cloud>>,
    pub status_rgb: Option<Draft<crate::status_rgb::Configuration>>,
    pub avr: Option<Draft<crate::board_silent::Parameters>>,
    pub avr_owner: Option<(crate::peer_http::Pin, crate::Transport)>,
    pub avr_pending: Option<u64>,
    pub wifi_password: String,
    pub open_network: bool,
    pub mqtt_password: SecretEdit,
    pub websocket_token: SecretEdit,
    pub ntfy_token: SecretEdit,
    pub pending: Option<(u64, Group)>,
    pub serial: u64,
    pub message: String,
    pub network_confirmation: bool,
    pub close_confirmation: bool,
    pub ssids: Vec<String>,
    pub scan_pending: bool,
    pub scan_retry_at: Option<std::time::Instant>,
    pub scan_deadline: Option<std::time::Instant>,
    pub usb_request: Option<Network>,
    pub maintenance_confirmation: Option<crate::maintenance::Operation>,
    pub peer_source_user:String,
    pub peer_source_password:String,
    pub maintenance_pending: Option<u64>,
    pub maintenance_read: bool,
    pub firmware_url: String,
    pub firmware_sha256: String,
    pub firmware_bearer: String,
    pub firmware_targets: u8,
    pub image_url: String,
    pub image_sha256: String,
    pub image_avr: bool,
    pub feed_token: String,
    pub firmware_feed: Option<Draft<crate::firmware_feed::Settings>>,
    pub remote_job: Option<crate::maintenance::RemoteJob>,
    pub telemetry: Option<serde_json::Value>,
}
impl Editor {
    pub fn network_write(&self, id: u64) -> Result<Write, String> {
        let draft = self
            .network
            .as_ref()
            .ok_or("Network settings unavailable")?;
        if draft.value.station_enabled
            && draft.value.station_ssid != draft.saved.station_ssid
            && self.wifi_password.is_empty()
            && !self.open_network
        {
            return Err(
                "Enter a password or explicitly select Open when changing the station SSID".into(),
            );
        }
        let mut config = draft.value.clone();
        if config.addressing.dhcp == Some(true) {
            config.addressing = draft.saved.addressing.clone();
            config.addressing.dhcp = Some(true);
        }
        Write::network_with_ethernet(
            id,
            &config,
            &self.wifi_password,
            self.open_network,
            self.ethernet_ready,
        )
    }
    pub fn dirty(&self) -> bool {
        self.usb_request.is_some()
            || self.status_rgb.as_ref().is_some_and(Draft::dirty)
            || self.firmware_feed.as_ref().is_some_and(Draft::dirty)
            || !self.feed_token.is_empty()
            || self.avr.as_ref().is_some_and(Draft::dirty)
            || self.network.as_ref().is_some_and(Draft::dirty)
            || self.ethernet.as_ref().is_some_and(Draft::dirty)
            || self.cloud.as_ref().is_some_and(Draft::dirty)
            || !self.wifi_password.is_empty()
            || self.open_network
            || self.mqtt_password.mode != SecretMode::Keep
            || self.websocket_token.mode != SecretMode::Keep
            || self.ntfy_token.mode != SecretMode::Keep
    }
    pub fn clear_secrets(&mut self) {
        self.peer_source_password.clear();
        self.feed_token.clear();
        self.firmware_bearer.clear();
        self.wifi_password.clear();
        self.open_network = false;
        self.mqtt_password = SecretEdit::default();
        self.websocket_token = SecretEdit::default();
        self.ntfy_token = SecretEdit::default();
    }
    pub fn discard(&mut self) {
        if let Some(d) = &mut self.status_rgb {
            d.discard();
        }
        if let Some(d) = &mut self.firmware_feed {
            d.discard();
        }
        self.usb_request = None;
        if let Some(d) = &mut self.avr {
            d.discard();
        }
        if let Some(d) = &mut self.network {
            d.discard();
        }
        if let Some(d) = &mut self.ethernet {
            d.discard();
        }
        if let Some(d) = &mut self.cloud {
            d.discard();
        }
        self.clear_secrets();
    }
    pub fn invalidate_peer(&mut self) {
        let open = self.open;
        let page = self.page;
        let search = self.search.clone();
        let serial = self.serial;
        *self = Self {
            open,
            page,
            search,
            serial,
            ..Self::default()
        };
    }
    pub fn finish_avr(
        &mut self,
        id: u64,
        peer: Option<&crate::peer_http::Pin>,
        route: Option<crate::Transport>,
        result: &Result<crate::pc_contract::Settings, String>,
    ) {
        if self.avr_pending != Some(id) {
            return;
        }
        self.avr_pending = None;
        if self
            .avr_owner
            .as_ref()
            .is_none_or(|(p, r)| Some(p) != peer || Some(*r) != route)
        {
            self.avr = None;
            self.avr_owner = None;
            self.message = "Board changed; settings result was not applied to this editor.".into();
            return;
        }
        match result {
            Ok(settings) => {
                self.avr = Some(Draft::new(crate::board_silent::Parameters::from_settings(
                    settings,
                )));
                self.message = "AVR settings verified · saved in EEPROM".into();
            }
            Err(error) => {
                if let Some(d) = &mut self.avr {
                    d.conflict = true;
                }
                self.message = error.clone();
            }
        }
    }
    pub fn observe(&mut self, path: &str, value: serde_json::Value) -> Result<(), String> {
        if path == Group::Ethernet.path() {
            let config = Ethernet::read(value)?;
            if let Some(d) = &mut self.ethernet {
                d.observe(config);
            } else {
                self.ethernet = Some(Draft::new(config));
            }
            return Ok(());
        }
        if path == Group::StatusRgb.path() {
            let config = crate::status_rgb::Configuration::read(value)?;
            if let Some(d) = &mut self.status_rgb {
                d.observe(config);
            } else {
                self.status_rgb = Some(Draft::new(config));
            }
            return Ok(());
        }
        if path == "/api/firmware/remote" {
            self.remote_job = Some(crate::maintenance::parse_job(value)?);
            return Ok(());
        }
        if path == "/api/telemetry" {
            if value["type"] != "telemetry"
                || value["product"] != "PakarController"
                || !value["boot_id"]
                    .as_str()
                    .is_some_and(|s| !s.is_empty() && s.len() <= 80)
            {
                return Err("Complete telemetry unavailable".into());
            }
            self.telemetry = Some(value);
            return Ok(());
        }
        if matches!(path, "/api/wifi/scan" | "/api/wifi/scan?results=true") {
            if value["running"] == true {
                if self.scan_pending {
                    self.scan_retry_at =
                        Some(std::time::Instant::now() + std::time::Duration::from_millis(500));
                }
                return Ok(()); // Keep the previous visible list while scanning.
            }
            self.scan_pending = false;
            self.scan_retry_at = None;
            self.scan_deadline = None;
            if let Some(error) = value["error"].as_str() {
                return Err(format!("Wi-Fi scan failed: {error}"));
            }
            self.ssids = value["networks"]
                .as_array()
                .ok_or("Invalid Wi-Fi scan response")?
                .iter()
                .filter_map(|network| network["ssid"].as_str())
                .filter(|ssid| {
                    !ssid.is_empty() && ssid.len() <= 32 && !ssid.chars().any(char::is_control)
                })
                .map(str::to_owned)
                .collect();
            self.ssids.sort();
            self.ssids.dedup();
        }
        if path == Group::Network.path() {
            self.ethernet_ready = value["ethernet_ready"] == true;
            let config: Network = serde_json::from_value(value)
                .map_err(|e| format!("Invalid network response: {e}"))?;
            config.validate_with_ethernet(self.ethernet_ready)?;
            if let Some(d) = &mut self.network {
                d.observe(config);
            } else {
                self.network = Some(Draft::new(config));
            }
        } else if path == Group::Cloud.path() {
            let config: Cloud = serde_json::from_value(value)
                .map_err(|e| format!("Invalid cloud response: {e}"))?;
            config.validate()?;
            if let Some(d) = &mut self.cloud {
                d.observe(config);
            } else {
                self.cloud = Some(Draft::new(config));
            }
        }
        Ok(())
    }
    pub fn scan_poll_due(&mut self, now: std::time::Instant) -> bool {
        if !self.scan_pending {
            return false;
        }
        if self.scan_deadline.is_some_and(|deadline| now >= deadline) {
            self.scan_pending = false;
            self.scan_retry_at = None;
            self.scan_deadline = None;
            self.message = "Wi-Fi scan timed out; no scan was replayed".into();
            return false;
        }
        if self.scan_retry_at.is_some_and(|due| now >= due) {
            self.scan_retry_at = None; // At most one result read outstanding.
            return true;
        }
        false
    }
    pub fn next_id(&mut self) -> u64 {
        self.serial = self.serial.wrapping_add(1);
        self.serial
    }
    pub fn finish(&mut self, id: u64, group: Group, result: Result<serde_json::Value, String>) {
        if self.pending != Some((id, group)) {
            return;
        }
        self.pending = None;
        match result {
            Ok(value) => {
                let loaded = match group {
                    Group::Ethernet => {
                        self.ethernet = None;
                        self.observe(group.path(), value)
                    }
                    Group::StatusRgb => {
                        self.status_rgb = None;
                        self.observe(group.path(), value)
                    }
                    Group::Network => {
                        self.network = None;
                        self.wifi_password.clear();
                        self.open_network = false;
                        self.observe(group.path(), value)
                    }
                    Group::Cloud => {
                        self.cloud = None;
                        self.mqtt_password = SecretEdit::default();
                        self.websocket_token = SecretEdit::default();
                        self.ntfy_token = SecretEdit::default();
                        self.observe(group.path(), value)
                    }
                };
                self.message = loaded.map(|_| "Saved public settings confirmed by device readback. Secret values remain write-only.".into()).unwrap_or_else(|e| e);
            }
            Err(error) => self.message = error,
        }
    }
}
pub fn usb_payloads(config: &Network, password: &str, open: bool) -> Result<Vec<Vec<u8>>, String> {
    if config.station_enabled && !open && !(8..=63).contains(&password.len()) {
        return Err("USB provisioning requires an explicit password or Open choice; saved secrets cannot be read over this transport".into());
    }
    if config.addressing.dhcp == Some(false) {
        return Err(
            "Static addressing must be configured over the authenticated network API".into(),
        );
    }
    config.validate()?;
    bounded(password, 63, "Wi-Fi password")?;
    let password = if open {
        b"".as_slice()
    } else {
        password.as_bytes()
    };
    let mut payloads = vec![vec![0]];
    for (kind, bytes) in [(1, config.station_ssid.as_bytes()), (2, password)] {
        for chunk in bytes.chunks(47) {
            let mut payload = vec![kind];
            payload.extend_from_slice(chunk);
            payloads.push(payload);
        }
    }
    payloads.push(vec![
        3,
        u8::from(config.station_enabled),
        u8::from(config.ap_enabled),
    ]);
    Ok(payloads)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn async_wifi_scan_preserves_list_yields_between_reads_and_never_restarts_scan() {
        let now = std::time::Instant::now();
        let mut editor = Editor::default();
        editor.ssids = vec!["Previous".into()];
        editor.scan_pending = true;
        editor.scan_deadline = Some(now + std::time::Duration::from_secs(15));
        editor
            .observe(
                "/api/wifi/scan",
                serde_json::json!({"running":true,"networks":[]}),
            )
            .unwrap();
        assert_eq!(editor.ssids, ["Previous"]);
        assert!(!editor.scan_poll_due(now));
        assert!(editor.scan_poll_due(now + std::time::Duration::from_secs(1)));
        assert!(!editor.scan_poll_due(now + std::time::Duration::from_secs(2)));
        editor
            .observe(
                "/api/wifi/scan?results=true",
                serde_json::json!({"running":false,"networks":[{"ssid":"New"},{"ssid":"New"}]}),
            )
            .unwrap();
        assert_eq!(editor.ssids, ["New"]);
        assert!(!editor.scan_pending);
        editor.scan_pending = true;
        editor.scan_deadline = Some(now);
        editor.scan_retry_at = Some(now);
        assert!(!editor.scan_poll_due(now));
        assert!(!editor.scan_pending && editor.scan_retry_at.is_none());
    }
    #[test]
    fn ethernet_settings_require_actual_support_validate_addressing_and_confirm_readback() {
        let raw = serde_json::json!({"configured":true,"enabled":true,"lldp_enabled":true,"lldp_receive":false,
            "dhcp":false,"address":"192.168.9.2","gateway":"192.168.9.1","subnet":"255.255.255.0","dns1":"0.0.0.0","dns2":"0.0.0.0"});
        let config = Ethernet::read(raw.clone()).unwrap();
        let write = Write::ethernet(1, &config).unwrap();
        assert_eq!(write.group.path(), "/api/network/ethernet");
        assert!(
            write
                .verify_ack(&serde_json::json!({"accepted":true,"apply_pending":true}))
                .is_ok()
        );
        assert!(write.verify_readback(&raw).is_ok());
        let mut foreign = raw.clone();
        foreign["address"] = serde_json::json!("192.168.9.3");
        assert!(write.verify_readback(&foreign).is_err());
        foreign = raw.clone();
        foreign["configured"] = false.into();
        assert!(Ethernet::read(foreign).is_err());
        foreign = raw.clone();
        foreign["subnet"] = "255.0.255.0".into();
        assert!(Ethernet::read(foreign).is_err());
        assert!(
            !write
                .form
                .iter()
                .any(|(key, _)| ["configured", "ipv4_ready", "apply_pending"]
                    .contains(&key.as_str()))
        );
        let mut editor = Editor::default();
        editor.observe(Group::Ethernet.path(), raw).unwrap();
        editor.ethernet.as_mut().unwrap().value.enabled = Some(false);
        assert!(editor.dirty());
        editor.invalidate_peer();
        assert!(editor.ethernet.is_none());
    }
    #[test]
    fn wifi_off_requires_observed_ethernet_ready_not_configured_or_cached_peer() {
        let mut raw = serde_json::json!({"station_enabled":true,"ap_enabled":true,"station_ssid":"Fixture","ethernet_ready":false});
        let mut editor = Editor::default();
        editor.observe(Group::Network.path(), raw.clone()).unwrap();
        let network = &mut editor.network.as_mut().unwrap().value;
        network.station_enabled = false;
        network.ap_enabled = false;
        assert!(editor.network_write(1).is_err());
        raw["ethernet_ready"] = true.into();
        editor.observe(Group::Network.path(), raw).unwrap();
        assert!(editor.network_write(1).is_ok());
        assert!(
            !editor
                .network_write(1)
                .unwrap()
                .form
                .iter()
                .any(|(key, _)| key == "ethernet_ready")
        );
        editor.invalidate_peer();
        assert!(!editor.ethernet_ready);
    }
    #[test]
    fn avr_editor_owns_dirty_draft_pending_receipt_and_same_peer_readback() {
        let peer = |boot: char| {
            crate::peer_http::Pin::from_status(&crate::DeviceStatus {
                board_id: "esp32s3-settings-fixture".into(),
                boot_id: boot.to_string().repeat(32),
                http_peer_fencing: true,
                ..Default::default()
            })
            .unwrap()
        };
        let a = peer('a');
        let b = peer('b');
        let mut bytes = vec![0x50, 2, 5, 0, 0, 0, 2, 0, 63, 0x10, 0x32, 0x54, 0];
        bytes.extend_from_slice(&[0; 8]);
        bytes.push(1);
        let settings =
            crate::pc_contract::Settings::parse(&crate::pc_contract::encode_hex(&bytes)).unwrap();
        let mut e = Editor {
            avr: Some(Draft::new(crate::board_silent::Parameters::from_settings(
                &settings,
            ))),
            avr_owner: Some((a.clone(), crate::Transport::Serial)),
            ..Default::default()
        };
        assert!(!e.dirty());
        e.avr.as_mut().unwrap().value.threshold = 700;
        assert!(e.dirty());
        e.avr_pending = Some(3);
        e.finish_avr(
            2,
            Some(&a),
            Some(crate::Transport::Serial),
            &Ok(settings.clone()),
        );
        assert!(e.dirty());
        assert_eq!(e.avr_pending, Some(3));
        e.finish_avr(
            3,
            Some(&a),
            Some(crate::Transport::Serial),
            &Err("ACK lost".into()),
        );
        assert!(e.dirty() && e.avr.as_ref().unwrap().conflict);
        assert!(e.avr_pending.is_none());
        e.discard();
        assert!(!e.dirty());
        e.avr_pending = Some(4);
        e.finish_avr(4, Some(&b), Some(crate::Transport::Serial), &Ok(settings));
        assert!(e.avr.is_none() && e.avr_owner.is_none());
    }
    fn network() -> Network {
        Network {
            station_enabled: true,
            ap_enabled: true,
            station_ssid: "Fixture network".into(),
            addressing: Addressing::default(),
        }
    }
    #[test]
    fn usb_provisioning_preserves_staging_protocol_and_chunk_bounds() {
        let payloads = usb_payloads(&network(), &"p".repeat(63), false).unwrap();
        assert_eq!(payloads.first().unwrap(), &[0]);
        assert_eq!(payloads.last().unwrap(), &[3, 1, 1]);
        assert!(payloads.iter().all(|p| p.len() <= 48));
        assert_eq!(
            payloads
                .iter()
                .filter(|p| p[0] == 2)
                .flat_map(|p| p[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![b'p'; 63]
        );
        assert!(
            !usb_payloads(&network(), "", true)
                .unwrap()
                .iter()
                .any(|p| p[0] == 2)
        );
        assert!(usb_payloads(&network(), "", false).is_err());
    }
    fn cloud() -> Cloud {
        Cloud {
            mqtt_enabled: true,
            mqtt_tls: true,
            mqtt_host: "mqtt.example.invalid".into(),
            mqtt_port: 8883,
            mqtt_user: "fixture".into(),
            mqtt_base_topic: "pakar/fixture".into(),
            mqtt_password_set: true,
            websocket_enabled: false,
            websocket_tls: true,
            websocket_host: String::new(),
            websocket_port: 443,
            websocket_path: "/controller".into(),
            websocket_token_set: false,
            telemetry_seconds: 30,
            ntfy_enabled: None,
            ntfy_host: None,
            ntfy_topic: None,
            ntfy_token_set: None,
        }
    }
    #[test]
    fn ntfy_opt_in_validation_secret_presence_and_optional_support() {
        let mut c = cloud();
        let keep = SecretEdit::default();
        assert!(
            !Write::cloud(1, &c, &keep, &keep, &keep)
                .unwrap()
                .form
                .iter()
                .any(|(k, _)| k.starts_with("ntfy"))
        );
        c.ntfy_enabled = Some(false);
        c.ntfy_host = Some("ntfy.sh".into());
        c.ntfy_topic = Some("fixture-private".into());
        c.ntfy_token_set = Some(false);
        c.validate().unwrap();
        c.ntfy_enabled = Some(true);
        let token = SecretEdit {
            mode: SecretMode::Replace,
            value: "tk_fixture".into(),
        };
        let w = Write::cloud(1, &c, &keep, &keep, &token).unwrap();
        assert!(w.form.contains(&("ntfy_enabled".into(), "true".into())));
        assert!(w.form.contains(&("ntfy_token".into(), "tk_fixture".into())));
        let Expected::Cloud(saved) = w.expected else {
            panic!()
        };
        assert_eq!(saved.ntfy_token_set, Some(true));
        for topic in ["", "bad/path", "bad\r\nheader"] {
            c.ntfy_topic = Some(topic.into());
            assert!(c.validate().is_err());
        }
    }
    #[test]
    fn missing_required_fields_are_not_defaults() {
        let mut editor = Editor::default();
        assert!(
            editor
                .observe(
                    Group::Network.path(),
                    serde_json::json!({"station_enabled":true})
                )
                .is_err()
        );
        assert!(editor.network.is_none());
        assert!(
            editor
                .observe(
                    Group::Cloud.path(),
                    serde_json::json!({"mqtt_enabled":false})
                )
                .is_err()
        );
        assert!(editor.cloud.is_none());
    }
    #[test]
    fn network_bounds_and_explicit_open_choice() {
        let mut n = network();
        assert!(Write::network(1, &n, "", false).is_ok());
        assert!(
            !Write::network(1, &n, "", false)
                .unwrap()
                .form
                .iter()
                .any(|(k, _)| k == "password")
        );
        assert!(Write::network(1, &n, "", true).is_ok());
        assert!(Write::network(1, &n, "fixture-pass", false).is_ok());
        n.ap_enabled = false;
        n.station_enabled = false;
        assert!(n.validate().is_err());
        n.station_enabled = true;
        n.station_ssid = "é".repeat(17);
        assert!(n.validate().is_err());
    }
    #[test]
    fn optional_addressing_preserves_unknown_and_validates_static_profile() {
        let old: Network = serde_json::from_value(
            serde_json::json!({"station_enabled":true,"ap_enabled":true,"station_ssid":"Fixture"}),
        )
        .unwrap();
        assert!(!old.addressing.observed());
        assert!(old.validate().is_ok());
        assert!(
            !serde_json::to_value(&old)
                .unwrap()
                .as_object()
                .unwrap()
                .contains_key("dhcp")
        );
        let mut current = network();
        current.addressing = Addressing {
            dhcp: Some(false),
            address: Some("192.168.7.8".into()),
            gateway: Some("192.168.7.1".into()),
            subnet: Some("255.255.255.0".into()),
            dns1: Some(String::new()),
            dns2: Some(String::new()),
        };
        let write = Write::network(1, &current, "", false).unwrap();
        assert!(write.form.contains(&("dns1".into(), "0.0.0.0".into())));
        current.addressing.subnet = Some("255.0.255.0".into());
        assert!(current.validate().is_err());
    }
    #[test]
    fn secret_keep_never_silently_turns_a_new_ssid_into_open_network() {
        let mut editor = Editor {
            network: Some(Draft::new(network())),
            ..Default::default()
        };
        assert!(editor.network_write(1).is_ok());
        editor.network.as_mut().unwrap().value.station_ssid = "Other network".into();
        assert!(editor.network_write(1).is_err());
        editor.open_network = true;
        assert!(
            editor
                .network_write(1)
                .unwrap()
                .form
                .contains(&("password".into(), String::new()))
        );
    }
    #[test]
    fn public_readback_cannot_confirm_different_configuration() {
        let write = Write::network(1, &network(), "fixture-pass", false).unwrap();
        assert!(
            write
                .verify_ack(&serde_json::json!({"accepted":true}))
                .is_ok()
        );
        assert!(
            write
                .verify_ack(&serde_json::json!({"accepted":false}))
                .is_err()
        );
        assert!(
            write
                .verify_readback(&serde_json::to_value(network()).unwrap())
                .is_ok()
        );
        let mut changed = network();
        changed.ap_enabled = false;
        assert!(
            write
                .verify_readback(&serde_json::to_value(changed).unwrap())
                .is_err()
        );
    }
    #[test]
    fn all_cloud_fields_and_secret_presence_are_verified() {
        let write = Write::cloud(
            2,
            &cloud(),
            &SecretEdit::default(),
            &SecretEdit::default(),
            &SecretEdit::default(),
        )
        .unwrap();
        assert!(
            write
                .verify_readback(&serde_json::to_value(cloud()).unwrap())
                .is_ok()
        );
        let mut changed = cloud();
        changed.mqtt_password_set = false;
        assert!(
            write
                .verify_readback(&serde_json::to_value(changed).unwrap())
                .is_err()
        );
        changed = cloud();
        changed.telemetry_seconds += 1;
        assert!(
            write
                .verify_readback(&serde_json::to_value(changed).unwrap())
                .is_err()
        );
    }
    #[test]
    fn keep_replace_clear_secrets_have_distinct_wire_semantics() {
        let keep = SecretEdit::default();
        let replace = SecretEdit {
            mode: SecretMode::Replace,
            value: "fixture-secret".into(),
        };
        let clear = SecretEdit {
            mode: SecretMode::Clear,
            value: String::new(),
        };
        let write = Write::cloud(1, &cloud(), &keep, &keep, &keep).unwrap();
        assert!(
            write
                .form
                .iter()
                .all(|(k, _)| k != "mqtt_password" && k != "websocket_token")
        );
        let write = Write::cloud(2, &cloud(), &replace, &clear, &keep).unwrap();
        assert!(
            write
                .form
                .contains(&("mqtt_password".into(), "fixture-secret".into()))
        );
        assert!(
            write
                .form
                .contains(&("websocket_token".into(), String::new()))
        );
        assert!(
            SecretEdit {
                mode: SecretMode::Replace,
                value: String::new()
            }
            .form_value()
            .is_err()
        );
    }
    #[test]
    fn invalid_cloud_values_block_writes() {
        let mut c = cloud();
        c.mqtt_host = "https://example.invalid".into();
        assert!(c.validate().is_err());
        c = cloud();
        c.mqtt_port = 0;
        assert!(c.validate().is_err());
        c = cloud();
        c.mqtt_base_topic = "pakar/#".into();
        assert!(c.validate().is_err());
        c = cloud();
        c.websocket_path = "path".into();
        assert!(c.validate().is_err());
        c = cloud();
        c.telemetry_seconds = 3601;
        assert!(c.validate().is_err());
    }
    #[test]
    fn refresh_does_not_overwrite_dirty_edits() {
        let mut d = Draft::new(network());
        d.value.station_ssid = "Edited".into();
        let mut remote = network();
        remote.station_ssid = "Remote".into();
        d.observe(remote);
        assert_eq!(d.value.station_ssid, "Edited");
        assert!(d.conflict);
        d.discard();
        assert_eq!(d.value, network());
    }
    #[test]
    fn peer_switch_clears_secret_and_pending_state() {
        let mut e = Editor {
            open: true,
            page: 2,
            wifi_password: "fixture-pass".into(),
            pending: Some((4, Group::Network)),
            ..Default::default()
        };
        e.network = Some(Draft::new(network()));
        e.invalidate_peer();
        assert!(e.open && e.network.is_none() && e.pending.is_none() && e.wifi_password.is_empty());
        assert_eq!(e.page, 2);
    }
    #[test]
    fn only_matching_request_can_complete_and_failures_keep_drafts() {
        let mut e = Editor::default();
        e.network = Some(Draft::new(network()));
        e.pending = Some((9, Group::Network));
        e.finish(
            8,
            Group::Network,
            Ok(serde_json::to_value(network()).unwrap()),
        );
        assert!(e.pending.is_some());
        e.finish(9, Group::Network, Err("Unknown outcome".into()));
        assert!(e.network.is_some() && e.pending.is_none());
        assert_eq!(e.message, "Unknown outcome");
    }
}
