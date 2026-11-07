//! Pealayer dialog/rail/card primitives, bound to actual Pakar device contracts.
use super::dropdown::DropdownUiExt;
use crate::{
    device_settings::{Editor, Group, SecretEdit, SecretMode, Write},
    ui::{dialog, icons},
    *,
};
use base64::Engine as _;
use eframe::egui;

include!("settings_catalog.rs");
pub fn page_index(name: &str) -> Option<usize> {
    PAGE_IDS
        .iter()
        .position(|page| page.eq_ignore_ascii_case(name))
}
fn matches(page: usize, search: &str) -> bool {
    let query = search.trim().to_lowercase();
    query.is_empty()
        || format!("{} {}", PAGES[page].0, PAGES[page].2)
            .to_lowercase()
            .contains(&query)
}
pub(crate) fn available(app: &PakarControllerApp, page: usize) -> bool {
    if page == 6 && app.has_feature("status-rgb") {
        return true;
    }
    PAGE_FEATURES.get(page).is_some_and(|features| {
        features.is_empty()
            || features.iter().any(|feature| {
                if *feature == "table" {
                    app.has_table()
                } else {
                    app.has_feature(feature)
                }
            })
    })
}
fn field(ui: &mut egui::Ui, icon: &str, label: &str, value: &mut String, secret: bool) {
    dialog::setting_row(ui, icon, label, None, |ui| {
        ui.add_sized(
            [ui.available_width(), 28.0],
            dialog::singleline_text_edit(ui, value).password(secret),
        );
    });
}
const AVR_PAGES: [&str; 6] = [
    "Cells",
    "Outputs",
    "Inputs",
    "Sound",
    "Display",
    "Run state",
];
fn avr_page(app: &mut PakarControllerApp, draft: &mut Editor, ui: &mut egui::Ui) {
    let status = app.live_status().cloned();
    let supported = status
        .as_ref()
        .is_some_and(crate::board_silent::settings_supported);
    let owner = status
        .as_ref()
        .and_then(crate::peer_http::Pin::from_status)
        .zip(app.command_route());
    if draft.avr_owner != owner {
        draft.avr = None;
        draft.avr_pending = None;
        draft.avr_owner = owner.clone();
    }
    if !supported {
        ui.weak("Connect an AVR that acknowledges settings to configure its saved parameters.");
        return;
    }
    app.board_silent.expire();
    if draft.avr_pending.is_some() && app.board_silent.pending.is_none() {
        draft.avr_pending = None;
        if let Some(value) = &mut draft.avr {
            value.conflict = true;
        }
        draft.message = app.board_silent.message.clone();
    }
    let ready = status.as_ref().is_some_and(|s| !s.programming_latch) && owner.is_some();
    if ready && app.board_silent.needs_refresh() && draft.avr.as_ref().is_none_or(|d| !d.dirty()) {
        app.request_board_settings(crate::board_silent::Change::Read);
    }
    if let Some((peer, route)) = &owner {
        if let Some(value) = app.board_silent.settings(Some(peer), Some(*route)) {
            let params = crate::board_silent::Parameters::from_settings(value);
            if let Some(editor) = &mut draft.avr {
                editor.observe(params);
            } else {
                draft.avr = Some(crate::device_settings::Draft::new(params));
            }
        }
    }
    let busy = app.board_silent.pending.is_some();
    dialog::section(ui, icons::GEAR, "Saved AVR settings", |ui| {
        ui.weak("Read from the connected board. Save writes its EEPROM only after you apply; application preferences are separate.");
        if !ready {
            ui.weak("Programming safety is active; editing is disabled.");
        }
        if busy {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.weak(&app.board_silent.message);
            });
        }
        if let Some(editor) = &mut draft.avr {
            if editor.conflict {
                ui.colored_label(ui.visuals().warn_fg_color,"The board changed or the result is uncertain. Discard and reload before saving.");
            }
            ui.add_enabled_ui(ready && !busy && !editor.conflict, |ui| {
                let v = &mut editor.value;
                field(ui, icons::CPU, "AVR name", &mut v.name, false);
                if app.has_feature("buzzer") {
                    ui.checkbox(&mut v.silent, super::i18n::text("Silent mode"));
                    ui.checkbox(&mut v.welcome, super::i18n::text("Play welcome melody"));
                }
                if app.has_feature("display") {
                    dialog::setting_row(ui, icons::GAUGE, "Seven-segment brightness", None, |ui| {
                        ui.add(egui::Slider::new(&mut v.brightness, 0..=7));
                    });
                }
                if app.has_feature("inputs") {
                    dialog::setting_row(
                        ui,
                        icons::HAND_TAP,
                        "Button 9 ADC threshold",
                        None,
                        |ui| {
                            ui.add(egui::DragValue::new(&mut v.threshold).range(1..=1023));
                        },
                    );
                }
                dialog::setting_row(
                    ui,
                    icons::BROADCAST,
                    "Periodic AVR telemetry",
                    Some("0 sends only changed state; otherwise 100–65535 ms."),
                    |ui| {
                        ui.add(
                            egui::DragValue::new(&mut v.stream_ms)
                                .range(0..=65535)
                                .suffix(" ms"),
                        );
                    },
                );
                ui.checkbox(
                    &mut v.remember_page,
                    super::i18n::text("Remember last page"),
                );
                dialog::setting_row(ui, icons::GAUGE, "Start page", None, |ui| {
                    super::dropdown::ComboBox::from_id_salt("avr-default-page")
                        .selected_text(super::i18n::text(AVR_PAGES[v.default_page as usize]))
                        .show_ui(ui, |ui| {
                            for (page, label) in AVR_PAGES.iter().enumerate() {
                                if v.visible & (1 << page) != 0 {
                                    ui.dropdown_value(
                                        &mut v.default_page,
                                        page as u8,
                                        super::i18n::text(label),
                                    );
                                }
                            }
                        });
                });
                ui.label(egui::RichText::new(super::i18n::text("Front-panel pages")).strong());
                for rank in 0..6 {
                    let page = v.order[rank];
                    ui.push_id(rank, |ui| {
                        ui.horizontal(|ui| {
                            let mut visible = v.visible & (1 << page) != 0;
                            if ui
                                .checkbox(&mut visible, super::i18n::text(AVR_PAGES[page as usize]))
                                .changed()
                            {
                                if visible {
                                    v.visible |= 1 << page;
                                } else {
                                    v.visible &= !(1 << page);
                                }
                            }
                            if ui
                                .add_enabled(rank > 0, egui::Button::new(icons::ARROW_UP))
                                .on_hover_text(super::i18n::text("Move page up"))
                                .clicked()
                            {
                                v.order.swap(rank, rank - 1);
                            }
                            if ui
                                .add_enabled(rank < 5, egui::Button::new(icons::ARROW_DOWN))
                                .on_hover_text(super::i18n::text("Move page down"))
                                .clicked()
                            {
                                v.order.swap(rank, rank + 1);
                            }
                        });
                    });
                }
            });
            if let Err(error) = editor.value.settings() {
                ui.colored_label(ui.visuals().warn_fg_color, error);
            }
        } else {
            ui.weak("Saved settings: Unknown");
        }
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    ready && !busy,
                    egui::Button::new(format!(
                        "{}  {}",
                        icons::ARROW_CLOCKWISE,
                        super::i18n::text("Read board")
                    )),
                )
                .clicked()
            {
                app.request_board_settings(crate::board_silent::Change::Read);
            }
            if app.board_silent.persisted() {
                ui.weak("Readback · stored in EEPROM");
            }
        });
        if !busy && !app.board_silent.message.is_empty() {
            ui.weak(&app.board_silent.message);
        }
    });
}
fn secret(ui: &mut egui::Ui, id: &str, label: &str, stored: bool, value: &mut SecretEdit) {
    ui.push_id(id, |ui| {
        dialog::setting_row(ui, icons::LOCK, label, None, |ui| {
            super::dropdown::ComboBox::from_id_salt("secret-mode")
                .selected_text(match value.mode {
                    SecretMode::Keep => {
                        if stored {
                            "Keep stored secret"
                        } else {
                            "Leave unset"
                        }
                    }
                    SecretMode::Replace => "Replace secret",
                    SecretMode::Clear => "Clear stored secret",
                })
                .show_ui(ui, |ui| {
                    ui.dropdown_value(&mut value.mode, SecretMode::Keep, "Keep stored value");
                    ui.dropdown_value(&mut value.mode, SecretMode::Replace, "Replace…");
                    ui.dropdown_value(&mut value.mode, SecretMode::Clear, "Clear");
                });
        });
        if value.mode == SecretMode::Replace {
            field(ui, icons::LOCK, "New value", &mut value.value, true);
        }
        if value.mode == SecretMode::Clear {
            ui.colored_label(
                ui.visuals().warn_fg_color,
                "The stored secret will be removed when you save cloud settings.",
            );
        }
    });
}
fn network_page(app: &mut PakarControllerApp, draft: &mut Editor, ui: &mut egui::Ui, ready: bool) {
    if draft.scan_pending && app.command_route() == Some(Transport::WiFi) {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(500));
        let now = std::time::Instant::now();
        if draft.scan_poll_due(now) {
            if ready {
                let _ = app.http_tx.send(HttpCommand::Configuration(
                    app.endpoint(),
                    "/api/wifi/scan?results=true",
                ));
            } else {
                draft.scan_retry_at = Some(now + std::time::Duration::from_millis(500));
            }
        }
    }
    if app.command_route() == Some(Transport::Serial) {
        dialog::section(ui, icons::PLUG, "USB network provisioning", |ui| {
            ui.label("USB can provision Wi-Fi, but cannot read saved network settings. This is a NEW provisioning request, not observed device configuration. Verify via HTTP afterward.");
            if draft.usb_request.is_none() && ui.button("Create provisioning request").clicked() {
                draft.usb_request = Some(crate::device_settings::Network {
                    station_enabled: true,
                    ap_enabled: true,
                    station_ssid: String::new(),
                    addressing: Default::default(),
                });
            }
            if let Some(request) = &mut draft.usb_request {
                ui.checkbox(
                    &mut request.station_enabled,
                    "Enable station in this request",
                );
                ui.checkbox(
                    &mut request.ap_enabled,
                    "Enable recovery AP in this request",
                );
                field(
                    ui,
                    icons::WIFI_HIGH,
                    "Requested SSID",
                    &mut request.station_ssid,
                    false,
                );
                ui.checkbox(
                    &mut draft.open_network,
                    "Open station network (no password)",
                );
                if !draft.open_network {
                    field(
                        ui,
                        icons::LOCK,
                        "Requested password",
                        &mut draft.wifi_password,
                        true,
                    );
                }
                let result = crate::device_settings::usb_payloads(
                    request,
                    &draft.wifi_password,
                    draft.open_network,
                );
                if let Err(error) = &result {
                    ui.label(
                        egui::RichText::new(error)
                            .small()
                            .color(ui.visuals().warn_fg_color),
                    );
                }
                if ui
                    .add_enabled(result.is_ok(), egui::Button::new("Provision via USB…"))
                    .clicked()
                {
                    draft.network_confirmation = true;
                }
            }
        });
        return;
    }
    let Some(network) = &mut draft.network else {
        ui.label("Connect and refresh to read the device's Wi-Fi policy.");
        return;
    };
    dialog::section(ui, icons::WIFI_HIGH, "Wireless network", |ui| {
        ui.add_enabled_ui(ready && draft.pending.is_none(), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut network.value.station_enabled, "Join a Wi-Fi network");
                ui.checkbox(
                    &mut network.value.ap_enabled,
                    "Enable recovery access point",
                );
            });
            field(
                ui,
                icons::WIFI_HIGH,
                "SSID",
                &mut network.value.station_ssid,
                false,
            );
            if ui
                .add_enabled(
                    !draft.scan_pending,
                    egui::Button::new(format!("{}  Scan networks", icons::MAGNIFYING_GLASS)),
                )
                .clicked()
            {
                draft.scan_pending = true;
                draft.scan_retry_at = None;
                draft.scan_deadline = Some(std::time::Instant::now() + std::time::Duration::from_secs(15));
                let _ = app.http_tx.send(HttpCommand::Configuration(
                    app.endpoint(),
                    "/api/wifi/scan",
                ));
            }
            if !draft.ssids.is_empty() {
                super::dropdown::ComboBox::from_id_salt("scanned-ssids")
                    .selected_text("Choose a discovered network…")
                    .show_ui(ui, |ui| {
                        for ssid in &draft.ssids {
                            ui.dropdown_value(
                                &mut network.value.station_ssid,
                                ssid.clone(),
                                ssid,
                            );
                        }
                    });
            }
            ui.checkbox(
                &mut draft.open_network,
                "This station network is open (no password)",
            );
            if !draft.open_network {
                field(
                    ui,
                    icons::LOCK,
                    "Replacement password (blank keeps stored)",
                    &mut draft.wifi_password,
                    true,
                );
            }
            if network.value.addressing.observed() {
                let address = &mut network.value.addressing;
                ui.separator();
                let dhcp = address.dhcp.as_mut().expect("observed addressing");
                ui.checkbox(dhcp, "Automatic station addressing (DHCP)");
                if !*dhcp {
                    for (label,value) in [("IPv4 address",&mut address.address),("Gateway",&mut address.gateway),
                        ("Subnet mask",&mut address.subnet),("Primary DNS",&mut address.dns1),("Secondary DNS",&mut address.dns2)] {
                        if let Some(value) = value { field(ui, icons::GLOBE,label,value,false); }
                    }
                }
            } else { ui.label(egui::RichText::new("Addressing is not reported by this firmware; no DHCP/static defaults are assumed.").small().weak()); }
        });
        ui.label(egui::RichText::new("Station credentials are write-only. Blank keeps the stored password for the unchanged SSID. Enter a password or explicitly choose Open when changing networks.").small().weak());
        if !network.value.ap_enabled {
            ui.colored_label(
                ui.visuals().warn_fg_color,
                "Recovery AP disabled: losing the station connection may require USB recovery.",
            );
        }
        if network.conflict {
            ui.colored_label(
                ui.visuals().warn_fg_color,
                "Device settings changed while you were editing. Discard and reload before saving.",
            );
        }
    });
}
fn ethernet_page(app: &mut PakarControllerApp, draft: &mut Editor, ui: &mut egui::Ui, ready: bool) {
    if app.command_route() == Some(Transport::Serial) {
        return;
    }
    let Some(observed) = app
        .live_status()
        .filter(|s| s.ethernet["configured"] == true)
    else {
        return;
    };
    let runtime = observed.ethernet.clone();
    dialog::section(ui, icons::PLUG, "Wired Ethernet", |ui| {
        let state = if runtime["ipv4_ready"] == true {
            "IPv4 ready"
        } else if runtime["link_up"] == true {
            "Link up · waiting for IPv4"
        } else {
            "Link down"
        };
        ui.label(format!(
            "{} · {}",
            state,
            runtime["ip"].as_str().unwrap_or("No address")
        ));
        if let Some(speed) = runtime["speed_mbps"].as_u64() {
            ui.weak(format!(
                "{speed} Mbps · {}",
                match runtime["full_duplex"].as_bool() {
                    Some(true) => "full duplex",
                    Some(false) => "half duplex",
                    None => "duplex unknown",
                }
            ));
        }
        if draft.ethernet.is_none() {
            if ui
                .add_enabled(
                    ready && draft.pending.is_none(),
                    egui::Button::new("Read Ethernet settings"),
                )
                .clicked()
            {
                let _ = app.http_tx.send(HttpCommand::Configuration(
                    app.endpoint(),
                    Group::Ethernet.path(),
                ));
            }
            return;
        }
        let d = draft.ethernet.as_mut().unwrap();
        ui.add_enabled_ui(ready && draft.pending.is_none() && !d.conflict, |ui| {
            if let Some(enabled) = &mut d.value.enabled {
                ui.checkbox(enabled, "Enable Ethernet");
            }
            let a = &mut d.value.addressing;
            if let Some(dhcp) = &mut a.dhcp {
                ui.checkbox(dhcp, "Automatic Ethernet addressing (DHCP)");
            }
            if a.dhcp == Some(false) {
                for (label, value) in [
                    ("IPv4 address", &mut a.address),
                    ("Gateway", &mut a.gateway),
                    ("Subnet mask", &mut a.subnet),
                    ("Primary DNS", &mut a.dns1),
                    ("Secondary DNS", &mut a.dns2),
                ] {
                    if let Some(value) = value {
                        field(ui, icons::GLOBE, label, value, false);
                    }
                }
            }
            if let Some(enabled) = &mut d.value.lldp_enabled {
                ui.checkbox(enabled, "Advertise Ethernet link identity (LLDP)");
            }
            if let Some(receive) = &mut d.value.lldp_receive {
                ui.checkbox(
                    receive,
                    "Advanced: receive LLDP neighbors (promiscuous Ethernet)",
                );
            }
        });
        if d.conflict {
            ui.colored_label(
                ui.visuals().warn_fg_color,
                "Ethernet settings changed remotely. Discard and reload before saving.",
            );
        }
        let valid = Write::ethernet(0, &d.value);
        if let Err(error) = &valid {
            ui.colored_label(ui.visuals().warn_fg_color, error);
        }
        if ui
            .add_enabled(
                ready && draft.pending.is_none() && !d.conflict && d.dirty() && valid.is_ok(),
                egui::Button::new("Save Ethernet and verify"),
            )
            .clicked()
        {
            submit(app, draft, Group::Ethernet);
        }
        ui.weak("Changing addressing may disconnect this route. No uncertain write is replayed or moved to another interface. Wi-Fi can be disabled only after Ethernet has a working IPv4 link.");
    });
}
fn cloud_page(draft: &mut Editor, ui: &mut egui::Ui, ready: bool, websocket: bool) {
    let Some(cloud) = &mut draft.cloud else {
        ui.label("Connect and refresh to read the device's cloud settings.");
        return;
    };
    dialog::section(
        ui,
        if websocket {
            icons::LINK
        } else {
            icons::BROADCAST
        },
        if websocket {
            "Outbound WebSocket"
        } else {
            "MQTT connection"
        },
        |ui| {
            ui.add_enabled_ui(ready && draft.pending.is_none(), |ui| {
                let config = &mut cloud.value;
                if websocket {
                    ui.horizontal_wrapped(|ui| {
                        ui.checkbox(&mut config.websocket_enabled, "Enabled");
                        ui.checkbox(&mut config.websocket_tls, "TLS encryption");
                    });
                    field(
                        ui,
                        icons::GLOBE,
                        "Server hostname",
                        &mut config.websocket_host,
                        false,
                    );
                    dialog::setting_row(ui, icons::PLUG, "Port", None, |ui| {
                        ui.add(egui::DragValue::new(&mut config.websocket_port).range(1..=65535));
                    });
                    field(
                        ui,
                        icons::LINK,
                        "Endpoint path",
                        &mut config.websocket_path,
                        false,
                    );
                    secret(
                        ui,
                        "ws-token",
                        "Bearer token",
                        config.websocket_token_set,
                        &mut draft.websocket_token,
                    );
                } else {
                    ui.horizontal_wrapped(|ui| {
                        ui.checkbox(&mut config.mqtt_enabled, "Enabled");
                        ui.checkbox(&mut config.mqtt_tls, "TLS encryption");
                    });
                    field(
                        ui,
                        icons::GLOBE,
                        "Broker hostname",
                        &mut config.mqtt_host,
                        false,
                    );
                    dialog::setting_row(ui, icons::PLUG, "Port", None, |ui| {
                        ui.add(egui::DragValue::new(&mut config.mqtt_port).range(1..=65535));
                    });
                    field(
                        ui,
                        icons::LIST,
                        "Base topic",
                        &mut config.mqtt_base_topic,
                        false,
                    );
                    field(ui, icons::LOCK, "Username", &mut config.mqtt_user, false);
                    secret(
                        ui,
                        "mqtt-password",
                        "Password",
                        config.mqtt_password_set,
                        &mut draft.mqtt_password,
                    );
                }
                dialog::setting_row(ui, icons::GAUGE, "Telemetry interval", None, |ui| {
                    ui.add(
                        egui::DragValue::new(&mut config.telemetry_seconds)
                            .range(1..=3600)
                            .suffix(" s"),
                    );
                });
            });
            ui.label(egui::RichText::new("MQTT and WebSocket settings are stored together. Save applies both drafts; passwords and tokens are never read back or exported.").small().weak());
            if ui
                .add_enabled(
                    ready && draft.pending.is_none(),
                    egui::Button::new("Reconnect stored cloud sessions…"),
                )
                .clicked()
            {
                draft.maintenance_confirmation =
                    Some(crate::maintenance::Operation::ReconnectCloud);
            }
            if cloud.conflict {
                ui.colored_label(
                    ui.visuals().warn_fg_color,
                    "Cloud configuration changed remotely. Discard and reload before saving.",
                );
            }
        },
    );
}
fn ntfy_page(app: &mut PakarControllerApp, draft: &mut Editor, ui: &mut egui::Ui, ready: bool) {
    let Some(cloud) = &mut draft.cloud else {
        return;
    };
    let config = &mut cloud.value;
    let (Some(enabled), Some(host), Some(topic), Some(token_set)) = (
        config.ntfy_enabled.as_mut(),
        config.ntfy_host.as_mut(),
        config.ntfy_topic.as_mut(),
        config.ntfy_token_set,
    ) else {
        return;
    };
    let mut save = false;
    dialog::section(
        ui,
        icons::BROADCAST,
        "Firmware notifications · ntfy",
        |ui| {
            ui.add_enabled_ui(ready && draft.pending.is_none() && !cloud.conflict,|ui| {
            ui.checkbox(enabled,"Send firmware-update notifications (opt in)");
            field(ui,icons::GLOBE,"HTTPS server",host,false);
            field(ui,icons::LIST,"Private topic",topic,true);
            secret(ui,"ntfy-token","Optional bearer token",token_set,&mut draft.ntfy_token);
            ui.weak("Disabled by default. Major measured stages and results only; flashing never waits for delivery. A topic is private information. No ntfy message can trigger an update.");
            save=ui.button("Save notifications and verify").clicked();
        });
        },
    );
    if save {
        submit(app, draft, Group::Cloud);
    }
}
fn submit(app: &mut PakarControllerApp, draft: &mut Editor, group: Group) {
    if draft.pending.is_some() || draft.maintenance_pending.is_some() {
        return;
    }
    if match group {
        Group::Network => draft.network.as_ref().is_some_and(|d| d.conflict),
        Group::Cloud => draft.cloud.as_ref().is_some_and(|d| d.conflict),
        Group::StatusRgb => draft.status_rgb.as_ref().is_some_and(|d| d.conflict),
        Group::Ethernet => draft.ethernet.as_ref().is_some_and(|d| d.conflict),
    } {
        draft.message = "Discard and refresh conflicting device settings before saving".into();
        return;
    }
    let id = draft.next_id();
    let result = match group {
        Group::Network => draft.network_write(id),
        Group::Cloud => draft
            .cloud
            .as_ref()
            .ok_or("Cloud settings unavailable".into())
            .and_then(|d| {
                Write::cloud(
                    id,
                    &d.value,
                    &draft.mqtt_password,
                    &draft.websocket_token,
                    &draft.ntfy_token,
                )
            }),
        Group::StatusRgb => draft
            .status_rgb
            .as_ref()
            .ok_or("RGB settings unavailable".into())
            .and_then(|d| Write::status_rgb(id, &d.value)),
        Group::Ethernet => draft
            .ethernet
            .as_ref()
            .ok_or("Ethernet settings unavailable".into())
            .and_then(|d| Write::ethernet(id, &d.value)),
    };
    match result {
        Ok(write) => {
            if app
                .http_tx
                .send(HttpCommand::Settings(app.endpoint(), write))
                .is_ok()
            {
                draft.pending = Some((id, group));
                draft.message = "Saving, then reading the stored configuration…".into();
            } else {
                draft.message = "Settings worker unavailable; nothing was sent".into();
            }
        }
        Err(error) => draft.message = error,
    }
}
fn status_rgb_page(
    app: &mut PakarControllerApp,
    draft: &mut Editor,
    ui: &mut egui::Ui,
    ready: bool,
) {
    if !app.has_feature("status-rgb") {
        return;
    }
    // Only a fresh current peer can seed the saved baseline. Optional missing
    // fields remain Unknown and cannot silently choose a GPIO or enabled state.
    let observed = app
        .live_status()
        .and_then(|s| s.status_rgb.as_ref())
        .and_then(|v| serde_json::to_value(v).ok())
        .and_then(|v| crate::status_rgb::Configuration::read(v).ok());
    if let Some(value) = observed {
        if let Some(d) = &mut draft.status_rgb {
            d.observe(value);
        } else {
            draft.status_rgb = Some(crate::device_settings::Draft::new(value));
        }
    }
    dialog::section(ui, icons::LIGHTBULB, "Onboard semantic status RGB", |ui| {
        ui.weak("Separate from the GPIO4 matrix. The board owns semantic colors, breathing and transitions; the status-bar dot mirrors its emitted RGB, not a local animation.");
        if app.command_route() == Some(Transport::Serial) {
            ui.weak("USB mirrors observed RGB. Saved RGB configuration is not supported by this USB contract; editing is unavailable.");
            return;
        }
        let Some(d) = &mut draft.status_rgb else {
            ui.weak("RGB settings: Unknown. Read the connected board before editing.");
            if ui
                .add_enabled(ready, egui::Button::new("Read RGB settings"))
                .clicked()
            {
                let _ = app.http_tx.send(HttpCommand::Configuration(
                    app.endpoint(),
                    Group::StatusRgb.path(),
                ));
            }
            return;
        };
        if d.conflict {
            ui.colored_label(
                ui.visuals().warn_fg_color,
                "RGB settings changed on the board. Discard and reload before saving.",
            );
        }
        ui.add_enabled_ui(ready && draft.pending.is_none() && !d.conflict, |ui| {
            dialog::setting_row(ui, icons::CPU, "Onboard pixel GPIO", None, |ui| {
                let previous = d.value.pin;
                super::dropdown::ComboBox::from_id_salt("onboard-status-pin")
                    .selected_text(if d.value.pin == -1 {
                        "Unassigned".into()
                    } else {
                        format!("GPIO{}", d.value.pin)
                    })
                    .show_ui(ui, |ui| {
                        ui.dropdown_value(&mut d.value.pin, -1, "Unassigned");
                        ui.dropdown_value(&mut d.value.pin, 48, "GPIO48 · common DevKit");
                        ui.dropdown_value(&mut d.value.pin, 38, "GPIO38 · alternate DevKit");
                    });
                if previous != d.value.pin {
                    d.value.pin_confirmed = false;
                    if d.value.pin == -1 {
                        d.value.enabled = false;
                    }
                }
            });
            ui.checkbox(
                &mut d.value.pin_confirmed,
                "Confirm this is the onboard addressable RGB pin",
            );
            ui.checkbox(&mut d.value.enabled, "Enable semantic status output");
            dialog::setting_row(ui, icons::GAUGE, "Status RGB brightness", None, |ui| {
                ui.add(egui::Slider::new(&mut d.value.brightness, 0..=255));
            });
        });
        if let Err(error) = d.value.validate() {
            ui.colored_label(ui.visuals().warn_fg_color, error);
        }
        let save = ready
            && draft.pending.is_none()
            && !d.conflict
            && d.dirty()
            && d.value.validate().is_ok();
        if ui
            .add_enabled(
                save,
                egui::Button::new(format!("{}  Save onboard RGB", icons::FLOPPY_DISK)),
            )
            .clicked()
        {
            submit(app, draft, Group::StatusRgb);
        }
    });
}
fn maintenance_page(
    app: &mut PakarControllerApp,
    draft: &mut Editor,
    ui: &mut egui::Ui,
    ready: bool,
) {
    use crate::maintenance::Operation;
    let isp = app.has_feature("isp");
    let ota = app.has_feature("ota");
    if draft.firmware_feed.is_none() {
        draft.firmware_feed = Some(crate::device_settings::Draft::new(
            app.config.firmware_feed.clone(),
        ));
    }
    if let Some(progress) = app.firmware_observation() {
        dialog::section(ui, icons::CLOCK, "Update progress", |ui| {
            crate::firmware_progress::draw(ui, &progress)
        });
    }
    dialog::section(ui, icons::BROADCAST, "Remote firmware subscription", |ui| {
        let running = app.firmware_feed.is_some();
        ui.add_enabled_ui(!running,|ui| {
            let settings=&mut draft.firmware_feed.as_mut().unwrap().value;
            field(ui,icons::LINK,"WebSocket endpoint",&mut settings.endpoint,false);
            super::dropdown::ComboBox::from_id_salt("firmware-feed-protocol")
                .selected_text(if settings.protocol==crate::firmware_feed::Protocol::SocketIo {"Socket.IO"}else{"WebSocket"})
                .show_ui(ui,|ui|{
                    ui.dropdown_value(&mut settings.protocol,crate::firmware_feed::Protocol::WebSocket,"WebSocket");
                    ui.dropdown_value(&mut settings.protocol,crate::firmware_feed::Protocol::SocketIo,"Socket.IO");
                });
            if settings.protocol==crate::firmware_feed::Protocol::SocketIo {
                field(ui,icons::LINK,"Namespace",&mut settings.namespace,false);
            }
            field(ui,icons::BROADCAST,"Firmware event",&mut settings.event,false);
            field(ui,icons::GLOBE,"Trusted HTTPS download origin",&mut settings.download_origin,false);
            field(ui,icons::LOCK,"Feed bearer · memory only",&mut draft.feed_token,true);
            ui.checkbox(&mut settings.deliver,"Arm delivery for this board session");
            if settings.deliver {ui.colored_label(ui.visuals().warn_fg_color,"Armed binary ESP offers can upload and reboot this board. Manifest/AVR offers stage only; programming remains explicit.");}
        });
        if running {
            if let Some(feed) = &app.firmware_feed {
                ui.label(&feed.state);
                ui.weak(format!("Session {} · {}", feed.session, feed.board));
                if let Some(offer) = feed.pending.as_ref() {
                    ui.label(format!(
                        "Offer {} · {} · {}",
                        offer.id, offer.target, offer.kind
                    ));
                }
            }
            ui.horizontal(|ui| {
                if ui.button("Stop listener").clicked() {
                    app.firmware_feed = None;
                }
                let pending = app.firmware_feed.as_ref().and_then(|f| f.pending.clone());
                if ui
                    .add_enabled(
                        pending.is_some(),
                        egui::Button::new("Deliver pending offer"),
                    )
                    .clicked()
                {
                    if let Some(offer) = app.firmware_feed.as_mut().and_then(|f| f.pending.take()) {
                        app.firmware_feed_message = app
                            .deliver_firmware_offer(offer)
                            .err()
                            .unwrap_or_else(|| "Offer dispatched once".into());
                    }
                }
                if ui.button("Discard offer").clicked() {
                    if let Some(feed) = app.firmware_feed.as_mut() {
                        feed.pending = None;
                    }
                }
            });
        } else if ui
            .add_enabled(
                app.live_status().is_some(),
                egui::Button::new("Start listener"),
            )
            .clicked()
        {
            let settings = draft.firmware_feed.as_ref().unwrap().value.clone();
            match app.start_firmware_feed(settings.clone(), std::mem::take(&mut draft.feed_token)) {
                Ok(()) => draft.firmware_feed = Some(crate::device_settings::Draft::new(settings)),
                Err(e) => app.firmware_feed_message = e,
            }
        }
        if !app.firmware_feed_message.is_empty() {
            ui.label(&app.firmware_feed_message);
        }
    });
    dialog::section(ui, icons::DOWNLOAD_SIMPLE, "Firmware file URL", |ui| {
        field(
            ui,
            icons::LINK,
            "HTTP(S) binary URL",
            &mut draft.image_url,
            false,
        );
        field(
            ui,
            icons::LOCK,
            "File SHA-256",
            &mut draft.image_sha256,
            false,
        );
        field(
            ui,
            icons::LOCK,
            "Download bearer · memory only",
            &mut draft.firmware_bearer,
            true,
        );
        if isp && ota {
            ui.checkbox(&mut draft.image_avr, "AVR image · stage only");
        }
        let esp = ota && !draft.image_avr;
        if ui
            .add_enabled(
                ready && draft.maintenance_pending.is_none(),
                egui::Button::new("Download and upload…"),
            )
            .clicked()
        {
            let result = crate::firmware_source::validate_remote(
                &draft.image_url,
                &draft.image_sha256,
                &draft.firmware_bearer,
            )
            .and_then(|_| {
                app.source_operation(
                    crate::firmware_source::Source::Http {
                        url: draft.image_url.clone(),
                        sha256: draft.image_sha256.clone(),
                        bearer: std::mem::take(&mut draft.firmware_bearer),
                    },
                    esp,
                )
            });
            match result {
                Ok(op) => draft.maintenance_confirmation = Some(op),
                Err(e) => draft.message = e,
            }
        }
    });
    if ota {
        dialog::section(ui, icons::GLOBE, "Verified remote firmware", |ui| {
            field(
                ui,
                icons::LINK,
                "Manifest HTTPS URL",
                &mut draft.firmware_url,
                false,
            );
            field(
                ui,
                icons::LOCK,
                "Pinned manifest SHA-256",
                &mut draft.firmware_sha256,
                false,
            );
            field(
                ui,
                icons::LOCK,
                "Endpoint bearer (optional)",
                &mut draft.firmware_bearer,
                true,
            );
            if !(1..=3).contains(&draft.firmware_targets) || (!isp && draft.firmware_targets != 1) {
                draft.firmware_targets = if isp { 3 } else { 1 };
            }
            dialog::setting_row(ui, icons::CPU, "Update target", None, |ui| {
                super::dropdown::ComboBox::from_id_salt("remote-target")
                    .selected_text(match draft.firmware_targets {
                        1 => "ESP",
                        2 => "AVR",
                        _ => "Both boards",
                    })
                    .show_ui(ui, |ui| {
                        if isp {
                            ui.dropdown_value(&mut draft.firmware_targets, 3, "Both boards");
                        }
                        ui.dropdown_value(
                            &mut draft.firmware_targets,
                            1,
                            "ESP (same-version repair)",
                        );
                        if isp {
                            ui.dropdown_value(
                                &mut draft.firmware_targets,
                                2,
                                "AVR (same-version repair)",
                            );
                        }
                    });
            });
            let available =
                (app.command_route() == Some(Transport::WiFi)||app.peer_maintenance_ready()) && draft.maintenance_pending.is_none();
            let mut refresh=false; // Live progress comes from ordinary peer-bound status/snapshots.
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(available, egui::Button::new("Refresh update & telemetry"))
                    .clicked()
                {
                    refresh=true;
                    if app.command_route()==Some(Transport::WiFi){
                        let _=app.http_tx.send(HttpCommand::Configuration(app.endpoint(),"/api/telemetry"));
                    }
                }
                let valid = crate::maintenance::validate_remote(
                    &draft.firmware_url,
                    &draft.firmware_sha256,
                    draft.firmware_targets,
                );
                let busy = draft.remote_job.as_ref().is_some_and(|j| j.busy);
                if ui
                    .add_enabled(
                        available
                            && app.command_route()==Some(Transport::WiFi)
                            && draft.remote_job.is_some()
                            && !busy
                            && valid.is_ok()
                            && (ready || draft.firmware_targets == 3),
                        egui::Button::new("Download & verify…"),
                    )
                    .clicked()
                {
                    draft.maintenance_confirmation = Some(Operation::RemoteStage {
                        url: draft.firmware_url.clone(),
                        sha256: draft.firmware_sha256.clone(),
                        targets: draft.firmware_targets,
                        bearer: std::mem::take(&mut draft.firmware_bearer),
                    });
                }
            });
            if refresh {
                let id=draft.next_id();
                match app.submit_peer_maintenance(id,Operation::RemoteStatus){
                    Ok(())=>{draft.maintenance_pending=Some(id);draft.maintenance_read=true;},
                    Err(error)=>draft.message=error,
                }
            }
            if let Some(job) = draft.remote_job.clone() {
                ui.label(format!(
                    "Job {} · {} · {}/{} bytes",
                    job.job_id, job.phase, job.received_bytes, job.expected_bytes
                ));
                if job.expected_bytes>0 {
                    ui.add(egui::ProgressBar::new((job.received_bytes as f64/job.expected_bytes as f64).clamp(0.0,1.0) as f32).show_percentage());
                }
                if !job.error.is_empty() {
                    ui.colored_label(ui.visuals().warn_fg_color, &job.error);
                }
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add_enabled(
                            available && job.phase == "ready",
                            egui::Button::new("Apply this verified job…"),
                        )
                        .clicked()
                    {
                        draft.maintenance_confirmation = Some(Operation::RemoteApply(job.clone()));
                    }
                    if ui
                        .add_enabled(
                            available && (job.cancellable || job.phase == "ready"),
                            egui::Button::new("Cancel this job…"),
                        )
                        .clicked()
                    {
                        draft.maintenance_confirmation = Some(Operation::RemoteCancel(job));
                    }
                });
            } else {
                ui.label("No remote transaction has been read from this device.");
            }
            ui.label(egui::RichText::new("Downloads never program automatically. Apply validates the exact job, boot identity and manifest digest. Matching board versions, durable Prog and recovery EEPROM readback are required. Credentials are memory-only.").small().weak());
            if let Some(value) = &draft.telemetry {
                ui.collapsing("Complete device & client telemetry", |ui| {
                    for key in [
                        "esp",
                        "avr",
                        "network",
                        "clock",
                        "discovery",
                        "cloud",
                        "clients",
                    ] {
                        ui.collapsing(key, |ui| {
                            let text =
                                serde_json::to_string_pretty(&value[key]).unwrap_or_default();
                            ui.add(
                                egui::Label::new(egui::RichText::new(text).monospace())
                                    .selectable(true)
                                    .wrap(),
                            );
                        });
                    }
                });
            }
        });
    }
    let recovery = isp
        && app.command_route() == Some(Transport::WiFi)
        && app.status.as_ref().is_some_and(|s| s.programming_latch)
        && draft.pending.is_none()
        && draft.maintenance_pending.is_none();
    if recovery {
        ui.colored_label(ui.visuals().warn_fg_color,"AVR programming latch is active. Normal device operations remain blocked; recovery uses the shared stored AVR image.");
        if ui.button("Resume stored AVR programming…").clicked() {
            draft.maintenance_confirmation = Some(Operation::ProgramAvr);
        }
    }
    dialog::section(ui, icons::FLOPPY_DISK, "Firmware and backup tools", |ui| {
        if isp {
            ui.label("Maintenance is explicit and never automatically retried. ISP operations interrupt the AVR. Confirm electrical readiness before programming.");
        }
        ui.add_enabled_ui(
            ready && draft.pending.is_none() && draft.maintenance_pending.is_none(),
            |ui| {
                if isp {
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Probe AVR ISP…").clicked() {
                            draft.maintenance_confirmation = Some(Operation::Probe);
                        }
                        if ui.button("Back up AVR…").clicked() {
                            draft.maintenance_confirmation = Some(Operation::Backup);
                        }
                    });
                }
                if isp {
                    ui.horizontal_wrapped(|ui| {
                        for (eeprom, label, name) in [
                            (false, "Export flash backup…", "avr-flash.bin"),
                            (true, "Export EEPROM backup…", "avr-eeprom.bin"),
                        ] {
                            if ui.button(label).clicked() {
                                if let Some(destination) =
                                    rfd::FileDialog::new().set_file_name(name).save_file()
                                {
                                    let id = draft.next_id();
                                    if app
                                        .http_tx
                                        .send(HttpCommand::Maintenance(
                                            app.endpoint(),
                                            id,
                                            Operation::Download {
                                                eeprom,
                                                destination,
                                            },
                                        ))
                                        .is_ok()
                                    {
                                        draft.maintenance_pending = Some(id);
                                        draft.message = "Downloading existing backup…".into();
                                    }
                                }
                            }
                        }
                    });
                }
                ui.horizontal_wrapped(|ui| {
                    for (esp, label) in
                        [(false, "Select AVR image…"), (true, "Update ESP firmware…")]
                    {
                        if (esp && !ota) || (!esp && !isp) {
                            continue;
                        }
                        if ui.button(label).clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Firmware binary", &["bin"])
                                .pick_file()
                            {
                                let result = (|| {
                                    let bytes = crate::maintenance::read_image_file(&path, esp)?;
                                    app.source_operation(
                                        crate::firmware_source::Source::Local(bytes),
                                        esp,
                                    )
                                })();
                                match result {
                                    Ok(operation) => {
                                        draft.message = format!("Selected {}", path.display());
                                        draft.maintenance_confirmation = Some(operation);
                                    }
                                    Err(error) => draft.message = error,
                                }
                            }
                        }
                    }
                    if isp && ui.button("Program staged AVR…").clicked() {
                        draft.maintenance_confirmation = Some(Operation::ProgramAvr);
                    }
                });
            },
        );
        if isp {
            ui.label(egui::RichText::new("Flash export includes Urboot if present. EEPROM restore and fuse writes are not exposed by this firmware; no nonfunctional controls are shown. Header checks do not establish firmware authenticity.").small().weak());
        }
    });
}
fn page(
    app: &mut PakarControllerApp,
    draft: &mut Editor,
    ui: &mut egui::Ui,
    index: usize,
    ready: bool,
) {
    if !available(app, index) {
        return;
    }
    ui.push_id(index, |ui| {
        ui.heading(PAGES[index].0); ui.add_space(12.0);
        match index {
            0 => {
                dialog::section(ui, icons::CIRCUITRY, "Connected controller", |ui| {
                    if let Some(status) = app.live_status().cloned() {
                        let mut values = vec![("Hostname",status.hostname.clone()),("Firmware",status.version.clone())];
                        if ["display", "text", "buzzer", "inputs", "outputs", "settings"].iter().any(|v| app.has_feature(v)) { values.push(("UART mode",status.link_mode.clone())); }
                        if app.has_feature("leds") { values.push(("LED pixels",status.pixels.to_string())); }
                        for (name,value) in values {
                            dialog::setting_row(ui, icons::INFO, name, None, |ui| { ui.label(&value); });
                        }
                        if app.has_feature("network") { ui.label(if status.ntp_synced { "Device clock synchronized by NTP" } else { "Device clock is not synchronized" }); }
                    } else { ui.label("Device state unavailable. Connect to read the controller's configuration."); }
                    if dialog::action_button(ui, icons::PLUG, "Connection…").clicked() && return_to_workspace(draft,ui) { app.show_connection = true; }
                });
                ui.add_space(12.0);
                dialog::section(ui, icons::GEAR, "Application preferences", |ui| {
                    ui.label("Appearance, language, window behavior, USB connection defaults and workspaces are saved on this PC, not on the boards.");
                    if dialog::action_button(ui, icons::GEAR, "Preferences…").clicked() && return_to_workspace(draft,ui) { app.open_preferences(); }
                });
            }
            1 => { ui.add_enabled_ui((ready || app.command_route()==Some(Transport::Serial)) && !app.identity_pending, |ui| { app.draw_device_identity(ui); }); }
            2 => {network_page(app, draft, ui, ready);ethernet_page(app,draft,ui,ready);},
            3 | 4 => cloud_page(draft, ui, ready, index == 4),
            5 => { ui.add_enabled_ui(ready, |ui| { app.draw_device_access(ui); }); }
            6 => {
                status_rgb_page(app,draft,ui,ready);
                ui.add_space(12.0);
                app.draw_led_settings(ui);
                ui.add_space(12.0);
                if app.has_table() { dialog::section(ui, icons::GRID_FOUR, "Table and LED controls", |ui| {
                    ui.label("Logical cell numbers are row-major. Physical addresses and names are separate; odd columns have a half-cell stagger.");
                    if dialog::action_button(ui, icons::GRID_FOUR, "Cell mapping…").clicked() && return_to_workspace(draft,ui) { app.show_mapping = true; }
                    if dialog::action_button(ui, icons::LIGHTBULB, "Effect controls").clicked() && return_to_workspace(draft,ui) { app.reveal_panels.push(ui::layout::Panel::EffectControls); }
                    ui.label(egui::RichText::new("Brightness and effects are live device controls. Chipset/channel order and pixel count are currently build-only; no unsupported runtime editor is shown.").small().weak());
                }); }
            }
            7 => app.draw_front_panel(ui),
            8 => {
                avr_page(app, draft, ui);
                ui.add_space(12.0);
                app.draw_hardware(ui,false);
                ui.add_space(12.0);
                ui.label(egui::RichText::new("Clock, fuse, pin ownership, debounce and busy-wire/polling selection are build-time settings in the current firmware—not remotely writable parameters.").small().weak());
            }
            9 => { ntfy_page(app,draft,ui,ready); maintenance_page(app, draft, ui, ready); },
            _ => {}
        }
        ui.add_space(16.0);
    });
}
fn return_to_workspace(draft: &mut Editor, ui: &egui::Ui) -> bool {
    if draft.dirty()
        || draft.pending.is_some()
        || draft.avr_pending.is_some()
        || draft.maintenance_pending.is_some()
    {
        draft.message="Save or discard device edits before opening another editor. In-flight writes cannot be cancelled.".into();
        return false;
    }
    draft.open = false;
    draft.clear_secrets();
    ui.ctx()
        .send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::Focus);
    true
}
fn editor(app: &mut PakarControllerApp, draft: &mut Editor, ui: &mut egui::Ui) {
    if !available(app, draft.page) {
        draft.page = 0;
    }
    let ready = app.http_writable()
        && app.transport != Transport::Serial
        && draft.maintenance_pending.is_none()
        && app.status.as_ref().is_some_and(|s| !s.programming_latch);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(icons::CIRCUITRY)
                .size(30.0)
                .color(ui.visuals().selection.bg_fill),
        );
        ui.vertical(|ui| {
            ui.heading("Device settings");
            ui.label(
                egui::RichText::new(if app.transport == Transport::Serial {
                    format!("USB · {}", app.serial_port)
                } else {
                    app.base.clone()
                })
                .small()
                .weak(),
            );
        });
    });
    ui.add_space(10.0);
    if app.command_route() == Some(Transport::WiFi) && !app.http_writable() {
        ui.label(
            egui::RichText::new(crate::ui::i18n::text(
                "Read-only: firmware must advertise board/boot identity and HTTP fencing",
            ))
            .small()
            .weak(),
        );
    }
    ui.horizontal(|ui| {
        ui.label(icons::MAGNIFYING_GLASS);
        ui.add_sized(
            [ui.available_width().max(120.0) - 112.0, 28.0],
            dialog::singleline_text_edit(ui, &mut draft.search).hint_text("Search device settings"),
        );
        if dialog::action_button(ui, icons::ARROW_CLOCKWISE, "Refresh").clicked() {
            app.configuration_loaded = false;
            draft.message = "Reading device configuration…".into();
        }
    });
    if !ready && matches!(draft.page, 2..=5 | 9) && !(draft.page==9&&app.peer_maintenance_ready()) {
        ui.label(egui::RichText::new("Wi-Fi configuration is read-only until the device is connected and not programming. USB AVR controls remain available when advertised.").small().color(ui.visuals().warn_fg_color));
    }
    ui.add_space(10.0);
    let wide = ui.available_width() >= 620.0;
    if !wide {
        ui.horizontal_wrapped(|ui| {
            for (index, (title, icon, _)) in PAGES.iter().enumerate() {
                if available(app, index)
                    && matches(index, &draft.search)
                    && ui
                        .selectable_label(index == draft.page, format!("{icon} {title}"))
                        .clicked()
                {
                    draft.page = index;
                    draft.search.clear();
                }
            }
        });
    }
    let height = (ui.available_height() - 105.0).max(80.0);
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), height),
        egui::Layout::left_to_right(egui::Align::Min),
        |ui| {
            if wide {
                ui.allocate_ui_with_layout(
                    egui::vec2(172.0, height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        for (index, (title, icon, _)) in PAGES.iter().enumerate() {
                            if !available(app, index) {
                                continue;
                            }
                            if app.config.preferences_grouped_navigation && index == 0 {
                                ui.label(egui::RichText::new("DEVICE").small().weak());
                            }
                            if app.config.preferences_grouped_navigation && index == 2 {
                                ui.add_space(12.0);
                                ui.label(egui::RichText::new("CONNECTIONS").small().weak());
                            }
                            if app.config.preferences_grouped_navigation && index == 6 {
                                ui.add_space(12.0);
                                ui.label(egui::RichText::new("CONTROLLERS").small().weak());
                            }
                            if available(app, index)
                                && matches(index, &draft.search)
                                && dialog::navigation_button(
                                    ui,
                                    draft.page == index,
                                    icon,
                                    title,
                                    168.0,
                                )
                                .clicked()
                            {
                                draft.page = index;
                                draft.search.clear();
                            }
                        }
                    },
                );
                ui.separator();
                ui.add_space(10.0);
            }
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    dialog::scroll_column(ui, "device-settings-body", Some(height), |ui| {
                        let results: Vec<_> = (0..PAGES.len())
                            .filter(|&index| available(app, index) && matches(index, &draft.search))
                            .collect();
                        if results.is_empty() {
                            ui.label("No settings match this search.");
                        } else if draft.search.trim().is_empty() {
                            page(app, draft, ui, draft.page, ready);
                        } else {
                            for index in results {
                                page(app, draft, ui, index, ready);
                            }
                        }
                    });
                },
            );
        },
    );
    ui.separator();
    if !draft.message.is_empty() {
        ui.label(egui::RichText::new(&draft.message).small());
    }
    let validation = match draft.page {
        2 => draft
            .network
            .as_ref()
            .and_then(|_| draft.network_write(0).err()),
        3 | 4 => draft.cloud.as_ref().and_then(|d| {
            Write::cloud(
                0,
                &d.value,
                &draft.mqtt_password,
                &draft.websocket_token,
                &draft.ntfy_token,
            )
            .err()
        }),
        _ => None,
    };
    if let Some(error) = validation {
        ui.label(
            egui::RichText::new(error)
                .small()
                .color(ui.visuals().warn_fg_color),
        );
    }
    let footer_status = if draft.maintenance_pending.is_some() {
        "Maintenance in progress"
    } else if draft.pending.is_some() || draft.avr_pending.is_some() {
        "Awaiting device readback"
    } else if draft.dirty() {
        "Unsaved device changes"
    } else {
        "Device configuration"
    };
    dialog::action_bar(
        ui,
        false,
        |ui| {
            ui.label(egui::RichText::new(footer_status).small().weak());
        },
        |ui| {
            let group = match draft.page {
                2 => Some(Group::Network),
                3 | 4 => Some(Group::Cloud),
                _ => None,
            };
            if let Some(group) = group {
                let valid = match group {
                    Group::StatusRgb => false, // RGB owns its contextual section save action.
                    Group::Ethernet => false,  // Ethernet has its own explicit save action.
                    Group::Network => draft
                        .network
                        .as_ref()
                        .is_some_and(|d| !d.conflict && draft.network_write(0).is_ok()),
                    Group::Cloud => draft.cloud.as_ref().is_some_and(|d| {
                        !d.conflict
                            && Write::cloud(
                                0,
                                &d.value,
                                &draft.mqtt_password,
                                &draft.websocket_token,
                                &draft.ntfy_token,
                            )
                            .is_ok()
                    }),
                };
                if ui
                    .add_enabled(
                        ready && draft.pending.is_none() && valid && draft.dirty(),
                        egui::Button::new(format!(
                            "{}  Save {}",
                            icons::CHECK,
                            if group == Group::Network {
                                "network"
                            } else {
                                "cloud"
                            }
                        )),
                    )
                    .clicked()
                {
                    if group == Group::Network {
                        draft.network_confirmation = true;
                    } else {
                        submit(app, draft, group);
                    }
                }
            }
            if draft.page == 8 {
                let request = draft
                    .avr
                    .as_ref()
                    .filter(|d| d.dirty() && !d.conflict)
                    .and_then(|d| {
                        Some(crate::board_silent::Change::Apply {
                            baseline: d.saved.settings().ok()?.bytes,
                            desired: d.value.settings().ok()?.bytes,
                        })
                    });
                let available = app.live_status().is_some_and(|s| {
                    crate::board_silent::settings_supported(s) && !s.programming_latch
                });
                if ui
                    .add_enabled(
                        available && app.board_silent.pending.is_none() && request.is_some(),
                        egui::Button::new(format!(
                            "{}  {}",
                            icons::FLOPPY_DISK,
                            super::i18n::text("Save to AVR")
                        )),
                    )
                    .clicked()
                {
                    draft.avr_pending = app.request_board_settings(request.unwrap());
                }
            }
            if ui
                .add_enabled(
                    draft.pending.is_none()
                        && draft.avr_pending.is_none()
                        && draft.maintenance_pending.is_none()
                        && draft.dirty(),
                    egui::Button::new("Discard & reload"),
                )
                .clicked()
            {
                draft.discard();
                if draft.page == 8 {
                    draft.avr = None;
                    app.request_board_settings(crate::board_silent::Change::Read);
                }
                app.configuration_loaded = false;
            }
        },
    );
    if draft.network_confirmation {
        egui::Modal::new(egui::Id::new("network-save-confirm")).show(ui.ctx(),|ui| {
            ui.set_width(390.0); ui.heading("Apply network changes?");
            ui.label("The ESP will reassociate Wi-Fi. This connection may drop; reconnect and refresh if readback is unavailable. A blank password preserves the stored secret; Open explicitly clears it.");
            dialog::action_row(ui,false,|ui| {
                let usb_ready=app.command_route()==Some(Transport::Serial) && draft.usb_request.is_some();
                if ui.add_enabled(ready || usb_ready,egui::Button::new("Apply network")).clicked() {
                    draft.network_confirmation=false;
                    if usb_ready {
                        let result=app.provision_usb_network(draft.usb_request.as_ref().unwrap(),&draft.wifi_password,draft.open_network);
                        match result { Ok(())=> { draft.usb_request=None; draft.clear_secrets(); draft.message=app.message.clone(); }, Err(error)=>draft.message=error }
                    } else { submit(app,draft,Group::Network); }
                }
                if ui.button("Keep editing").clicked() { draft.network_confirmation=false; }
            });
    });
    }
    if let Some(mut operation) = draft.maintenance_confirmation.clone() {
        egui::Modal::new(egui::Id::new("maintenance-confirm")).show(ui.ctx(), |ui| {
            ui.set_width(420.0);
            ui.heading("Confirm device maintenance");
            ui.label(operation.warning());
            if matches!(operation,crate::maintenance::Operation::PeerStage{..}) {
                dialog::setting_row(ui,icons::APP_WINDOW,"Source username",None,|ui|{ui.add(dialog::singleline_text_edit(ui,&mut draft.peer_source_user));});
                dialog::setting_row(ui,icons::LOCK,"Source password",None,|ui|{ui.add(dialog::singleline_text_edit(ui,&mut draft.peer_source_password).password(true));});
            }
            if !draft.message.is_empty() {
                ui.label(&draft.message);
            }
            dialog::action_row(ui, false, |ui| {
                if ui
                    .add_enabled(
                        (ready
                            || (matches!(
                                operation,
                                crate::maintenance::Operation::PeerStage { .. }
                                    | crate::maintenance::Operation::RemoteApply(_)
                                    | crate::maintenance::Operation::RemoteCancel(_)
                            ) && app.peer_maintenance_ready()
                                && draft.maintenance_pending.is_none())
                            || (matches!(operation, crate::maintenance::Operation::ProgramAvr)
                                && app.http_writable()
                                && app.status.as_ref().is_some_and(|s| s.programming_latch)
                                && draft.maintenance_pending.is_none()))
                            && draft.pending.is_none(),
                        egui::Button::new("Proceed"),
                    )
                    .clicked()
                {
                    let id = draft.next_id();
                    if let crate::maintenance::Operation::PeerStage{authorization,..}=&mut operation {
                        *authorization=if draft.peer_source_password.is_empty(){String::new()}else{format!("Basic {}",base64::engine::general_purpose::STANDARD.encode(format!("{}:{}",draft.peer_source_user,draft.peer_source_password)))};
                        draft.peer_source_password.clear();
                    }
                    let transfer =
                        if let crate::maintenance::Operation::SourceImage { progress, .. } =
                            &operation
                        {
                            Some(progress.clone())
                        } else {
                            None
                        };
                    match app.submit_peer_maintenance(id,operation){Ok(())=>{
                        if let Some(transfer) = transfer {
                            app.firmware_transfer = Some(transfer);
                        }
                        draft.maintenance_pending = Some(id);
                        draft.maintenance_read = false;
                        draft.message =
                            "Maintenance submitted; waiting for the device result…".into();
                    },Err(error)=>draft.message=error}
                    draft.maintenance_confirmation = None;
                }
                if ui.button("Cancel").clicked() {
                    draft.peer_source_password.clear();
                    draft.maintenance_confirmation = None;
                }
            });
        });
    }
}
fn request_close(draft: &mut Editor) {
    if draft.dirty()
        || draft.pending.is_some()
        || draft.avr_pending.is_some()
        || draft.maintenance_pending.is_some()
    {
        draft.close_confirmation = true;
    } else {
        draft.open = false;
        draft.clear_secrets();
        draft.maintenance_confirmation = None;
        draft.network_confirmation = false;
    }
}
fn close_modal(draft: &mut Editor, ctx: &egui::Context) {
    if !draft.close_confirmation {
        return;
    }
    egui::Modal::new(egui::Id::new("device-settings-close")).show(ctx,|ui| {
        let pending=draft.pending.is_some() || draft.avr_pending.is_some() || draft.maintenance_pending.is_some();
        ui.set_width(370.0); ui.heading(if pending { "Device operation in progress" } else { "Discard device changes?" });
        ui.label(if pending { "Wait for the result before closing. An in-flight device write cannot be safely cancelled." } else { "Unsaved edits and secret values will be cleared." });
        if !pending && ui.button("Discard and close").clicked() { draft.discard(); draft.open=false; draft.close_confirmation=false; draft.maintenance_confirmation=None; }
        if ui.button("Keep open").clicked() { draft.close_confirmation=false; }
    });
}
pub fn draw(app: &mut PakarControllerApp, ctx: &egui::Context) {
    if !app.device_settings.open {
        return;
    }
    let mut draft = std::mem::take(&mut app.device_settings);
    if app.config.native_dialogs && !ctx.embed_viewports() {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("pakar-device-settings"),
            egui::ViewportBuilder::default()
                .with_title("Device settings — PakarController")
                .with_inner_size([940.0, 760.0])
                .with_min_inner_size([640.0, 500.0]),
            |child, _| {
                config::apply(child, &app.config);
                if child.input(|i| i.viewport().close_requested()) {
                    request_close(&mut draft);
                    if draft.open {
                        child.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                    }
                }
                egui::CentralPanel::default().show(child, |ui| {
                    egui::Frame::new()
                        .inner_margin(18)
                        .show(ui, |ui| editor(app, &mut draft, ui));
                });
                if dialog::escape_pressed(child) {
                    request_close(&mut draft);
                }
                close_modal(&mut draft, child);
            },
        );
    } else {
        let mut open = true;
        let g = dialog::bounded_geometry(
            ctx.content_rect(),
            16.0,
            egui::vec2(940.0, 760.0),
            egui::vec2(640.0, 500.0),
            egui::vec2(1100.0, 900.0),
        );
        egui::Window::new("Device settings")
            .id(egui::Id::new("pakar-device-settings"))
            .open(&mut open)
            .default_rect(g.default_rect)
            .min_size(g.min_size)
            .max_size(g.max_size)
            .constrain_to(g.bounds)
            .frame(dialog::opaque_window_frame_from_context(ctx))
            .collapsible(false)
            .show(ctx, |ui| editor(app, &mut draft, ui));
        if !open || dialog::escape_pressed(ctx) {
            request_close(&mut draft);
        }
        close_modal(&mut draft, ctx);
    }
    app.device_settings = draft;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn close_guard_preserves_drafts_and_inflight_writes() {
        let mut draft = Editor {
            open: true,
            wifi_password: "fixture-pass".into(),
            ..Default::default()
        };
        request_close(&mut draft);
        assert!(draft.open && draft.close_confirmation);
        draft.discard();
        draft.close_confirmation = false;
        draft.maintenance_pending = Some(3);
        request_close(&mut draft);
        assert!(draft.open && draft.close_confirmation);
        draft.maintenance_pending = None;
        draft.avr_pending = Some(4);
        draft.close_confirmation = false;
        request_close(&mut draft);
        assert!(draft.open && draft.close_confirmation);
        draft.avr_pending = None;
        draft.close_confirmation = false;
        request_close(&mut draft);
        assert!(!draft.open && draft.wifi_password.is_empty());
    }
    #[test]
    fn page_deep_links_are_stable_and_invalid_names_are_not_guessed() {
        assert_eq!(page_index("WiFi"), Some(2));
        assert_eq!(page_index("firmware"), Some(9));
        assert!(page_index("unknown").is_none());
    }
    #[test]
    fn navigation_covers_real_parameter_domains_without_empty_search_results() {
        for query in [
            "SSID",
            "MQTT",
            "token",
            "TM1637",
            "EEPROM",
            "credentials",
            "physical",
            "UART",
            "welcome",
            "threshold",
        ] {
            assert!((0..PAGES.len()).any(|p| matches(p, query)), "{query}");
        }
        assert!(!(0..PAGES.len()).any(|p| matches(p, "not-a-setting")));
        assert_eq!(PAGES.len(), 10);
    }
}
