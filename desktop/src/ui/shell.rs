//! Pealayer application shell adapted for a controller rather than media playback.
//! Menu -> typed action -> app dispatcher is shared with native launch/IPC/tray.
use super::dropdown::DropdownUiExt;
use crate::{
    ui::{
        dialog, icons,
        layout::{self, Panel},
        menu,
    },
    *,
};
use eframe::egui;
use platform::instance::Command;

pub fn launch_command(args: &[String]) -> Command {
    let option = |flag: &str| {
        args.iter()
            .position(|v| v == flag)
            .and_then(|at| args.get(at + 1))
            .filter(|v| !v.starts_with("--"))
            .cloned()
    };
    if args.iter().any(|v| v == "--snake") {
        Command::Snake
    } else if args.iter().any(|v| v == "--wifi") {
        Command::UseWifi {
            endpoint: option("--endpoint"),
        }
    } else if args.iter().any(|v| v == "--usb") {
        Command::UseUsb {
            port: option("--port"),
        }
    } else if args.iter().any(|v| v == "--disconnect") {
        Command::Disconnect
    } else if args.iter().any(|v| v == "--update-esp") {
        Command::UpdateEsp {
            path: option("--update-esp").unwrap_or_default(),
            confirmed: args.iter().any(|v| v == "--confirm"),
        }
    } else if args.iter().any(|v| v == "--led-effect") {
        Command::LedEffect {
            value: option("--led-effect").unwrap_or_default(),
        }
    } else if args.iter().any(|v| v == "--led-output") {
        Command::LedOutput {
            value: option("--led-output").unwrap_or_default(),
        }
    } else if args.iter().any(|v| v == "--pixel-arm") {
        Command::PixelArm
    } else if args.iter().any(|v| v == "--pixel-show") {
        Command::PixelShow {
            index: option("--pixel-show")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
        }
    } else if args.iter().any(|v| v == "--led-mode") {
        Command::LedMode {
            value: option("--led-mode").unwrap_or_default(),
        }
    } else if let Some(at) = args.iter().position(|v| v == "--led-cells") {
        Command::LedCells {
            ids: args
                .get(at + 1)
                .filter(|v| !v.starts_with("--"))
                .cloned()
                .unwrap_or_default(),
            state: args
                .get(at + 2)
                .filter(|v| !v.starts_with("--"))
                .cloned()
                .unwrap_or_default(),
            effect: args
                .get(at + 3)
                .filter(|v| !v.starts_with("--"))
                .cloned()
                .unwrap_or_else(|| "immediate".into()),
        }
    } else if let Some(at) = args.iter().position(|v| v == "--led-cell") {
        Command::LedCell {
            id: args.get(at + 1).and_then(|v| v.parse().ok()).unwrap_or(0),
            state: args
                .get(at + 2)
                .filter(|v| !v.starts_with("--"))
                .cloned()
                .unwrap_or_default(),
            effect: args
                .get(at + 3)
                .filter(|v| !v.starts_with("--"))
                .cloned()
                .unwrap_or_else(|| "immediate".into()),
        }
    } else if let Some(at) = args.iter().position(|v| v == "--screenshot") {
        Command::Screenshot {
            path: args.get(at + 1).cloned().unwrap_or_default(),
        }
    } else if args.iter().any(|v| v == "--manage-cells") {
        Command::CellManager
    } else if args.iter().any(|v| v == "--notification-test") {
        Command::NotificationTest
    } else if args.iter().any(|v| v == "--cell-backgrounds") {
        Command::CellBackgrounds {
            value: option("--cell-backgrounds").unwrap_or_default(),
        }
    } else if let Some(at) = args.iter().position(|v| v == "--preferences-page") {
        Command::PreferencesPage {
            page: args.get(at + 1).cloned().unwrap_or_default(),
        }
    } else if args.iter().any(|v| v == "--refresh") {
        Command::Refresh
    } else if args.iter().any(|v| v == "--fullscreen") {
        Command::Fullscreen
    } else if args.iter().any(|v| v == "--minimize") {
        Command::Minimize
    } else if let Some(at) = args.iter().position(|v| v == "--settings-page") {
        Command::DeviceSettingsPage {
            page: args.get(at + 1).cloned().unwrap_or_default(),
        }
    } else if args.iter().any(|v| v == "--device-settings") {
        Command::DeviceSettings
    } else if args.iter().any(|v| v == "--connect-dialog") {
        Command::Connect
    } else if args.iter().any(|v| v == "--quit") {
        Command::Quit
    } else if args.iter().any(|v| v == "--app-status") {
        Command::Status { pixels: false }
    } else if args.iter().any(|v| v == "--preferences") {
        Command::Preferences
    } else if args.iter().any(|v| v == "--about") {
        Command::About
    } else if args.iter().any(|v| v == "--workspaces") {
        Command::Workspaces
    } else if let Some(at) = args.iter().position(|v| v == "--find-cell") {
        Command::FindCell {
            query: args.get(at + 1).cloned().unwrap_or_default(),
        }
    } else if let Some(at) = args.iter().position(|v| v == "--panel") {
        Command::Panel {
            name: args.get(at + 1).cloned().unwrap_or_else(|| "Table".into()),
        }
    } else {
        Command::Show
    }
}

pub(crate) fn led_effect_code(value: &str) -> Option<u8> {
    match value {
        "none" => Some(0),
        "rainbow" => Some(1),
        "fire" => Some(2),
        _ => None,
    }
}
pub(crate) fn led_output_request(value: &str) -> Option<(Vec<(String, String)>, Vec<u8>)> {
    match value {
        "on" | "off" => Some((
            vec![("enabled".into(), (value == "on").to_string())],
            vec![u8::from(value == "on")],
        )),
        "static" | "effects" => Some((
            vec![("mode".into(), value.into())],
            vec![2, u8::from(value == "effects")],
        )),
        "gpio-static" | "rmt-dma" => Some((
            vec![("backend".into(), value.into())],
            vec![3, crate::led_backend::code(value)?],
        )),
        _ => None,
    }
}
pub(crate) fn led_mode_code(value: &str) -> Option<u8> {
    match value {
        "cell" => Some(1),
        "snake" => Some(3),
        _ => None,
    }
}
pub(crate) fn led_cell_payload(id: u8, state: &str, effect: &str) -> Option<Vec<u8>> {
    if !(1..=80).contains(&id) {
        return None;
    }
    let state = match state {
        "white" => 0,
        "red" => 1,
        "off" => 2,
        _ => return None,
    };
    let effect = match effect {
        "immediate" => 0,
        "fade" => 1,
        "pulse" => 2,
        _ => return None,
    };
    Some(vec![id, state, effect])
}
fn connection_change_blocked(
    preferences_dirty: bool,
    device_dirty: bool,
    pending: bool,
    maintenance: bool,
    identity_pending: bool,
) -> bool {
    preferences_dirty || device_dirty || pending || maintenance || identity_pending
}

#[cfg(test)]
mod lifecycle_launch_tests {
    use super::*;
    fn parsed(args: &[&str]) -> serde_json::Value {
        serde_json::to_value(launch_command(
            &args
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>(),
        ))
        .unwrap()
    }
    #[test]
    fn explicit_lifecycle_launches_retain_optional_configured_endpoints() {
        assert_eq!(
            parsed(&["--cell-backgrounds", "on"]),
            serde_json::json!({"command":"cell_backgrounds","value":"on"})
        );
        assert_eq!(
            parsed(&["--cell-backgrounds", "off"]),
            serde_json::json!({"command":"cell_backgrounds","value":"off"})
        );
        assert_eq!(parsed(&["--cell-backgrounds"])["value"], "");
        assert_eq!(
            parsed(&["--wifi"]),
            serde_json::json!({"command":"use_wifi","endpoint":null})
        );
        assert_eq!(
            parsed(&["--wifi", "--endpoint", "http://ledboard.local"]),
            serde_json::json!({"command":"use_wifi","endpoint":"http://ledboard.local"})
        );
        assert_eq!(
            parsed(&["--usb"]),
            serde_json::json!({"command":"use_usb","port":null})
        );
        assert_eq!(
            parsed(&["--usb", "--port", "COM22"]),
            serde_json::json!({"command":"use_usb","port":"COM22"})
        );
        assert_eq!(
            parsed(&["--disconnect"]),
            serde_json::json!({"command":"disconnect"})
        );
        assert_eq!(
            parsed(&["--led-output", "on"]),
            serde_json::json!({"command":"led_output","value":"on"})
        );
        assert_eq!(
            parsed(&["--led-output", "off"]),
            serde_json::json!({"command":"led_output","value":"off"})
        );
        for mode in ["static", "effects"] {
            assert_eq!(
                parsed(&["--led-output", mode]),
                serde_json::json!({"command":"led_output","value":mode})
            );
            let (form, payload) = led_output_request(mode).unwrap();
            assert_eq!(form, vec![("mode".into(), mode.into())]);
            assert_eq!(payload, vec![2, u8::from(mode == "effects")]);
        }
        assert!(led_output_request("animated").is_none());
        for (backend, code) in [("gpio-static", 1), ("rmt-dma", 2)] {
            let (form, payload) = led_output_request(backend).unwrap();
            assert_eq!(form, vec![("backend".into(), backend.into())]);
            assert_eq!(payload, vec![3, code]);
        }
        assert_eq!(
            parsed(&["--pixel-arm"]),
            serde_json::json!({"command":"pixel_arm"})
        );
        assert_eq!(
            parsed(&["--pixel-show", "480"]),
            serde_json::json!({"command":"pixel_show","index":480})
        );
        assert_eq!(parsed(&["--pixel-show", "bad"])["index"], 0);
        assert_eq!(
            parsed(&["--wifi", "--endpoint", "--app-status"])["endpoint"],
            serde_json::Value::Null
        );
    }
    #[test]
    fn esp_update_launch_requires_a_separate_explicit_confirmation() {
        assert_eq!(
            parsed(&["--update-esp", "C:\\firmware\\pakar.bin"]),
            serde_json::json!({"command":"update_esp","path":"C:\\firmware\\pakar.bin","confirmed":false})
        );
        assert_eq!(
            parsed(&["--update-esp", "C:\\firmware\\pakar.bin", "--confirm"]),
            serde_json::json!({"command":"update_esp","path":"C:\\firmware\\pakar.bin","confirmed":true})
        );
        assert_eq!(parsed(&["--update-esp", "--confirm"])["path"], "");
    }
    #[test]
    fn led_launches_use_the_exact_existing_api_and_wire_values() {
        assert_eq!(
            parsed(&["--led-cells", "1,8-12,80", "red", "fade"]),
            serde_json::json!({"command":"led_cells","ids":"1,8-12,80","state":"red","effect":"fade"})
        );
        for (name, code) in [("none", 0), ("rainbow", 1), ("fire", 2)] {
            assert_eq!(
                parsed(&["--led-effect", name]),
                serde_json::json!({"command":"led_effect","value":name})
            );
            assert_eq!(led_effect_code(name), Some(code));
        }
        for (name, code) in [("cell", 1), ("snake", 3)] {
            assert_eq!(
                parsed(&["--led-mode", name]),
                serde_json::json!({"command":"led_mode","value":name})
            );
            assert_eq!(led_mode_code(name), Some(code));
        }
        assert_eq!(
            parsed(&["--led-cell", "80", "white"]),
            serde_json::json!({"command":"led_cell","id":80,"state":"white","effect":"immediate"})
        );
        assert_eq!(
            parsed(&["--led-cell", "2", "off", "pulse"]),
            serde_json::json!({"command":"led_cell","id":2,"state":"off","effect":"pulse"})
        );
        for (state, code) in [("white", 0), ("red", 1), ("off", 2)] {
            for (effect, transition) in [("immediate", 0), ("fade", 1), ("pulse", 2)] {
                assert_eq!(
                    led_cell_payload(1, state, effect),
                    Some(vec![1, code, transition])
                );
            }
        }
        for args in [
            &["--led-cell", "256", "white"][..],
            &["--led-cell", "garbage", "white"][..],
            &["--led-cell"][..],
        ] {
            assert_eq!(parsed(args)["id"], 0);
        }
        assert!(led_effect_code("").is_none());
        assert!(led_effect_code("confetti").is_none());
        assert!(led_mode_code("first-wins").is_none()); // No AVR/game shortcut in this bounded interface.
        assert!(led_cell_payload(0, "white", "immediate").is_none());
        assert!(led_cell_payload(81, "white", "immediate").is_none());
        assert!(led_cell_payload(1, "unknown", "immediate").is_none());
        assert!(led_cell_payload(1, "white", "unknown").is_none());
    }
    #[test]
    fn each_dirty_or_pending_owner_independently_blocks_connection_changes() {
        assert!(!connection_change_blocked(
            false, false, false, false, false
        ));
        for bit in 0..5 {
            let mut guards = [false; 5];
            guards[bit] = true;
            assert!(connection_change_blocked(
                guards[0], guards[1], guards[2], guards[3], guards[4]
            ));
        }
    }
}
fn badge(ui: &mut egui::Ui, label: &str, active: bool) {
    let color = ui::palette::color(
        config::live().color_palette,
        ui.visuals().dark_mode,
        if active { "green" } else { "subtle" },
    );
    ui.label(
        egui::RichText::new(format!("●  {label}"))
            .color(color)
            .size(11.0),
    );
}
fn field(ui: &mut egui::Ui, icon: &str, label: &str, value: &mut String, password: bool) {
    dialog::setting_row(ui, icon, label, None, |ui| {
        ui.add_sized(
            [ui.available_width().min(340.0), 28.0],
            dialog::singleline_text_edit(ui, value).password(password),
        );
    });
}

fn finish_table_area(ui: &mut egui::Ui, area: egui::Rect) {
    // Absolute ui.put calls move the cursor to the last cell.
    // Restore the complete staggered-grid extent before placing the footer.
    ui.advance_cursor_after_rect(area);
    ui.add_space(8.0);
}

#[cfg(test)]
mod table_geometry_tests {
    use super::*;
    #[test]
    fn table_scene_fits_small_dock_and_zooms_at_pointer() {
        for width in [280.0, 900.0] {
            let ctx = egui::Context::default();
            let (pitch, height) = super::super::cell_table::grid_metrics(1280.0, 400.0, true);
            let (canvas, origin) = super::super::cell_table::grid_canvas(pitch, height);
            let mut view = canvas;
            let mut visible = egui::Rect::NOTHING;
            let mut viewport = egui::Rect::NOTHING;
            for zoom in [1.0, 2.0] {
                let input = egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(width, 220.0))),
                    events: vec![egui::Event::PointerMoved(egui::pos2(width/2.0,110.0)), egui::Event::Zoom(zoom)], ..Default::default() };
                let _ = ctx.run_ui(input, |ui| {
                    let size = ui.available_size();
                    viewport = ui.available_rect_before_wrap();
                    let fit = (size.x/canvas.width()).min(size.y/canvas.height());
                    egui::Scene::new().zoom_range(fit*0.5..=fit*8.0).max_inner_size(canvas.size())
                        .show(ui, &mut view, |ui| {
                            let transform = ui.ctx().layer_transform_to_global(ui.layer_id()).unwrap();
                            visible = transform * canvas;
                            if zoom == 1.0 {
                                for id in 1..=80 {
                                    assert!(viewport.expand(0.1).contains_rect(transform *
                                        super::super::cell_table::cell_rect(origin, pitch, height, id).expand(2.0)),
                                        "Fit clips cell {id} at dock width {width}");
                                }
                            }
                            ui.set_min_size(canvas.size());
                        });
                });
                if zoom == 1.0 { assert!(visible.width() <= width + 1.0); }
            }
            assert!(view.width() < canvas.width());
            assert!((view.center().x-canvas.center().x).abs() < 2.0);
        }
    }
    #[test]
    fn footer_stays_below_every_half_staggered_row_after_absolute_widgets() {
        for pitch in [30.0, 60.0, 94.0] {
            let ctx = egui::Context::default();
            let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                let height = pitch * 0.72;
                let (area, _) = ui.allocate_exact_size(
                    egui::vec2(pitch * 20.0, height * 4.5),
                    egui::Sense::hover(),
                );
                let last_cell = crate::ui::cell_table::cell_rect(area.min, pitch, height, 80);
                ui.put(last_cell, egui::Label::new("80"));
                finish_table_area(ui, area);
                let footer = ui.label(crate::ui::i18n::text("Cell selected")).rect;
                assert!(
                    footer.top() >= area.bottom() + 8.0,
                    "Footer overlaps staggered grid at pitch {pitch}: {area:?} {footer:?}"
                );
            });
            output.textures_delta.clear();
        }
    }
}
impl PakarControllerApp {
    pub fn receive_configuration(&mut self, path: &str, value: serde_json::Value) {
        if path == pixel_groups::PATH {
            self.pixel_groups.pending = None;
            let result = self.live_status().and_then(peer_http::Pin::from_status)
                .filter(|peer| self.pixel_groups.owner.as_ref()==Some(peer))
                .ok_or_else(|| "Physical group editor's board session changed".to_owned())
                .and_then(|peer| pixel_groups::Layout::receipt(&value,&peer));
            match result {
                Ok(actual) => {
                    if self.pixel_groups.expected.take().is_some_and(|expected| !expected.verifies(&actual)) {
                        self.pixel_groups.message = "Group layout readback differs; reload before saving again".into(); return;
                    }
                    self.pixel_groups.value=Some(actual); self.pixel_groups.dirty=false;
                    self.pixel_groups.message="Physical group layout read back from board".into();
                }
                Err(error) => { self.pixel_groups.expected=None; self.pixel_groups.message=error; }
            }
            return;
        }
        if path == pixel_corrections::PATH {
            self.pixel_corrections.pending = None;
            match serde_json::from_value::<pixel_corrections::Configuration>(value) {
                Ok(value) if value.validate().is_ok() => {
                    if self.live_status().and_then(peer_http::Pin::from_status) != self.pixel_corrections.owner {
                        self.pixel_corrections.message = "Correction readback belongs to a retired board".into();
                        return;
                    }
                    if self
                        .pixel_corrections
                        .expected
                        .take()
                        .is_some_and(|expected| {
                            expected.enabled != value.enabled || expected.rules != value.rules
                        })
                    {
                        self.pixel_corrections.message =
                            "Correction readback differs; reload before editing again".into();
                        return;
                    }
                    self.pixel_corrections.value = Some(value);
                    self.pixel_corrections.dirty = false;
                    self.pixel_corrections.live.committed();
                    self.pixel_corrections.message = "Board configuration read back".into();
                }
                _ => self.pixel_corrections.message = "Invalid correction readback".into(),
            }
            return;
        }
        if let Err(error) = self.device_settings.observe(path, value.clone()) {
            self.device_settings.message = error;
            return;
        }
        if path == "/api/device/name" {
            let dirty = self.identity_saved.as_ref().is_some_and(|(host, label)| {
                self.device_name.as_ref() != Some(host)
                    || self.board_display_name.as_ref() != Some(label)
            });
            if dirty {
                return;
            } // Periodic readback must not erase a name draft.
            if let Some(name) = value["hostname"]
                .as_str()
                .filter(|name| crate::identity::normalize_name(name).is_ok())
            {
                self.device_name = Some(name.into());
            }
            self.board_display_name = value["display_name"]
                .as_str()
                .filter(|v| crate::identity::valid_display_name(v))
                .map(str::to_owned);
            self.identity_saved = self
                .device_name
                .clone()
                .zip(self.board_display_name.clone());
        }
    }
    pub fn open_preferences(&mut self) {
        if self.preferences.is_none() {
            self.preferences = Some(ui::preferences::PreferencesDraft::new(self.config.clone()));
        }
    }
    pub fn dispatch(&mut self, command: Command, ctx: &egui::Context) {
        match command {
            Command::UseWifi { endpoint } => {
                if self.connection_change_guard() {
                    return;
                }
                if let Some(endpoint) = endpoint {
                    let endpoint = endpoint.trim();
                    if reqwest::Url::parse(endpoint).ok().is_none_or(|url| {
                        endpoint.len() > 512
                            || !matches!(url.scheme(), "http" | "https")
                            || url.host_str().is_none()
                            || !url.username().is_empty()
                            || url.password().is_some()
                            || url.query().is_some()
                            || url.fragment().is_some()
                            || !matches!(url.path(), "" | "/")
                    }) {
                        self.message =
                            "Use http(s)://hostname[:port] without credentials or a path".into();
                        return;
                    }
                    self.base = endpoint.to_owned();
                }
                self.set_connection(Transport::WiFi, true);
                ctx.request_repaint();
                return; // Process-local lifecycle must never focus the desktop.
            }
            Command::UseUsb { port } => {
                if self.connection_change_guard() {
                    return;
                }
                if let Some(port) = port {
                    if port.trim().is_empty()
                        || port.len() > 512
                        || port.chars().any(char::is_control)
                    {
                        self.message = "Use a nonempty USB serial port or hardware path".into();
                        return;
                    }
                    self.serial_port = port.trim().to_owned();
                }
                self.set_connection(Transport::Serial, true);
                ctx.request_repaint();
                return;
            }
            Command::Disconnect => {
                self.set_connection(self.transport, false);
                ctx.request_repaint();
                return;
            }
            Command::UpdateEsp { path, confirmed } => {
                if !confirmed {
                    self.message = "ESP firmware update requires explicit --confirm".into();
                } else if let Err(error) = self.update_esp_from_file(&path) {
                    self.message = error;
                }
                ctx.request_repaint();
                return; // The guarded existing maintenance owner handles OTA.
            }
            Command::LedEffect { value } => {
                if let Some(code) = led_effect_code(&value) {
                    self.action(
                        "/api/effect",
                        vec![("value".into(), value)],
                        OP_PATTERN,
                        vec![code],
                    );
                } else {
                    self.message = "Unknown LED effect; use none, rainbow or fire".into();
                }
                ctx.request_repaint();
                return;
            }
            Command::LedOutput { value } => {
                if let Some((form, payload)) = led_output_request(&value) {
                    let known = self.live_status().is_some_and(|s| match value.as_str() {
                        "gpio-static" | "rmt-dma" => crate::led_backend::allowed(s, &value),
                        "static" | "effects" => led_output_mode(s).is_some(),
                        _ => s
                            .led_output
                            .get("enabled")
                            .and_then(|v| v.as_bool())
                            .is_some(),
                    });
                    if known {
                        self.action("/api/led/output", form, OP_LED_OUTPUT, payload);
                    } else {
                        self.message = "LED output state is unknown for the current board; refresh before changing it".into();
                    }
                } else {
                    self.message = "LED output requires on, off, static, effects, gpio-static or rmt-dma".into();
                }
                ctx.request_repaint();
                return;
            }
            Command::PixelArm => {
                self.arm_pixel_diagnostic();
                ctx.request_repaint();
                return;
            }
            Command::PixelShow { index } => {
                self.show_pixel_diagnostic(index);
                ctx.request_repaint();
                return;
            }
            Command::LedMode { value } => {
                if let Some(code) = led_mode_code(&value) {
                    self.action(
                        "/api/mode",
                        vec![("value".into(), value)],
                        OP_SET_MODE,
                        vec![code],
                    );
                } else {
                    self.message = "Unknown LED mode; use cell or snake".into();
                }
                ctx.request_repaint();
                return;
            }
            Command::LedCell { id, state, effect } => {
                if let Some(payload) = led_cell_payload(id, &state, &effect) {
                    self.action(
                        "/api/cell",
                        vec![
                            ("id".into(), id.to_string()),
                            ("state".into(), state),
                            ("effect".into(), effect),
                        ],
                        OP_CELL_ACTION,
                        payload,
                    );
                } else {
                    self.message = "Invalid LED cell, state or transition".into();
                }
                ctx.request_repaint();
                return;
            }
            Command::LedCells { ids, state, effect } => {
                match super::cell_search::parse(&ids)
                    .and_then(|ids| crate::cell_batch::Request::new(&ids, &state, &effect))
                {
                    Ok(batch) => {
                        if let Err(error) = self.dispatch_cell_batch(batch) {
                            self.message = error;
                        }
                    }
                    Err(error) => self.message = error,
                }
                ctx.request_repaint();
                return;
            }
            Command::Screenshot { path } => {
                if let Err(error) = self.capture.request(path) {
                    self.message = error;
                }
                ctx.request_repaint();
                return; // A capture request never changes focus, panels or visibility.
            }
            Command::DeviceSettings => {
                self.device_settings.open = true;
                self.configuration_loaded = false;
            }
            Command::DeviceSettingsPage { page } => {
                if let Some(index) = ui::device_settings::page_index(&page) {
                    self.device_settings.open = true;
                    self.device_settings.page = index;
                    self.device_settings.search.clear();
                    self.configuration_loaded = false;
                } else {
                    self.message = "Unknown device settings page".into();
                }
            }
            Command::Quit => {
                self.exiting = true;
                self.save_workspace();
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                return;
            }
            Command::Minimize => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                ctx.request_repaint();
                return; // Do not immediately restore through the shared tail.
            }
            Command::Preferences => self.open_preferences(),
            Command::CellBackgrounds { value } => {
                // App-only presentation, never a board command. Do not commit
                // a live Preferences preview or overwrite an unrelated draft.
                self.message = if self.preferences.is_some() {
                    "Close Preferences before changing lamp backgrounds remotely".into()
                } else if !matches!(value.as_str(), "on" | "off") {
                    "Cell backgrounds require on or off".into()
                } else {
                    let enabled = value == "on";
                    if self.config.color_cell_backgrounds == enabled {
                        "Lamp backgrounds unchanged".into()
                    } else {
                        let mut config = self.config.clone();
                        config.color_cell_backgrounds = enabled;
                        match config.save() {
                            Ok(()) => {
                                self.config = config;
                                crate::config::set_live(&self.config);
                                "Lamp backgrounds saved".into()
                            }
                            Err(error) => error,
                        }
                    }
                };
                ctx.request_repaint();
                return; // A preference update must not focus the desktop.
            }
            Command::PreferencesPage { page } => {
                if let Some(index) = ui::preferences::page_index(&page) {
                    self.open_preferences();
                    if let Some(draft) = &mut self.preferences {
                        draft.select_page(index);
                    }
                } else {
                    self.message = "Unknown preferences page".into();
                }
            }
            Command::Refresh => self.refresh(),
            Command::Fullscreen => {
                let fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
            }
            Command::NotificationTest => self.notification_test(),
            Command::Connect => self.show_connection = true,
            Command::About => self.show_about = true,
            Command::Snake => ctx.open_url(egui::OpenUrl::new_tab("http://127.0.0.1:18490/snake")),
            Command::Workspaces => self.show_workspaces = true,
            Command::CellManager => {
                if !self.has_table() {
                    return;
                }
                self.table.manager_open = true;
                self.reveal_panels.push(Panel::Table);
            }
            Command::FindCell { query } => {
                if !self.has_table() {
                    return;
                }
                if ui::cell_search::parse(&query).is_ok() {
                    self.cell_search = query;
                    self.apply_cell_search_selection();
                    self.reveal_panels.push(Panel::Table);
                    self.focus_cell_search = true;
                }
            }
            Command::Panel { name } => {
                if let Some(panel) = Panel::from_name(&name) {
                    if panel.available(self.peer_context()) {
                        self.reveal_panels.push(panel);
                    }
                }
            }
            Command::Status { .. } => return, // Read-only diagnostics must never raise/focus the window.
            Command::Show => {
                // eframe starts hidden until its first rendered frame. Its
                // hidden-window logic-only path otherwise prevents an isolated
                // non-activating fixture from ever reaching that first frame.
                if self.screenshot_harness {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                }
            }
        }
        if !self.screenshot_harness {
            self.usb_ui_visible = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }
        ctx.request_repaint();
    }
    pub fn notify(&self, title: &str, message: &str, severity: messaging::Severity) {
        self.notify_id("pakar.command", title, message, severity);
    }
    pub fn observe_winner(&mut self, status: &crate::DeviceStatus) {
        let state = status.capabilities.as_ref().map(|c| {
            (
                status.board_id.as_str(),
                status.boot_id.as_str(),
                c.role.as_str(),
                status.winner,
            )
        });
        if let Some((winner, round)) = self.winner_notice.observe(state) {
            let message = format!("Box {winner} wins");
            self.record_event("Winner", &message);
            self.notify_id(
                &format!("pakar.winner.{winner}.{round}"),
                "Round winner",
                &message,
                messaging::Severity::Success,
            );
        }
    }
    pub fn notify_id(&self, id: &str, title: &str, message: &str, severity: messaging::Severity) {
        // Toast expiry is not error recovery. Only a matching successful
        // operation clears its error; unrelated successes cannot hide it.
        if severity == messaging::Severity::Error
            || (id == "pakar.connection" && severity == messaging::Severity::Warning)
        {
            self.workspace.errors.borrow_mut().insert(id.into());
        } else if severity == messaging::Severity::Success {
            self.workspace.errors.borrow_mut().remove(id);
        }
        let request = messaging::ToastRequest {
            id: Some(id.into()),
            title: title.into(),
            message: message.chars().take(2048).collect(),
            severity,
            timeout_ms: 5000,
        };
        if messaging::publish(request.clone(), "desktop").is_ok() {
            self.notifications.borrow_mut().enqueue(request);
        }
    }
    pub fn notify_detailed_id(
        &self,
        id: &str,
        title: &str,
        message: &str,
        details: &str,
        severity: messaging::Severity,
    ) {
        if severity == messaging::Severity::Error
            || (id == "pakar.connection" && severity == messaging::Severity::Warning)
        {
            self.workspace.errors.borrow_mut().insert(id.into());
        } else if severity == messaging::Severity::Success {
            self.workspace.errors.borrow_mut().remove(id);
        }
        let request = messaging::ToastRequest {
            id: Some(id.into()),
            title: title.into(),
            message: message.chars().take(2048).collect(),
            severity,
            timeout_ms: 5000,
        };
        if messaging::publish_detailed(
            request.clone(),
            "desktop",
            details.chars().take(8192).collect(),
        )
        .is_ok()
        {
            self.notifications.borrow_mut().enqueue(request);
        }
    }
    pub fn notification_test(&self) {
        self.notify_id("pakar.notification-test", "Notification test", "This is an in-app toast and a native Windows notification request. OS visibility is reported separately in Preferences and --app-status.", messaging::Severity::Info);
    }
    pub fn record_event(&mut self, kind: &str, message: &str) {
        self.event_log.record(kind, message);
    }
    fn save_status_bar_preferences(&mut self) {
        // Adapt Pealayer's narrow status-bar save: one explicit user change,
        // outside the widget draw. Do not serialize dock placement or commit
        // unrelated Preferences drafts just because a footer item was hidden.
        let result = config::AppConfig::load().and_then(|mut saved| {
            if saved.status_items == self.config.status_items {
                return Ok(());
            }
            saved.status_items = self.config.status_items.clone();
            saved.save()
        });
        if let Err(error) = result {
            self.message = error.clone();
            self.event_log.failure("Status bar preferences", &error);
        } else {
            self.event_log.reset_sources("Status bar preferences");
        }
    }
    pub fn save_workspace(&mut self) {
        self.remember_ui_workspace();
        self.config.connection_method = self.transport;
        self.config.connection_enabled = self.connection_enabled;
        if self
            .config
            .last_network_route
            .as_ref()
            .is_some_and(|route| route.endpoint != self.base.trim_end_matches('/'))
        {
            if let Some(route) = &self.config.last_network_route {
                crate::peer_http::invalidate_address(&route.endpoint);
            }
            self.config.last_network_route = None;
        }
        if self.active_context.is_some() {
            self.config.dock_layout = self.workspace.layout.snapshot().0;
        }
        if config::AppConfig::load().ok().as_ref() != Some(&self.config) {
            if let Err(error) = self.config.save() {
                self.message = error;
            } else {
                self.workspace.saved();
            }
        }
    }
    fn reset_workspace_layout(&mut self) {
        self.workspace.layout = Default::default();
        let available: Vec<_> = Panel::ALL
            .into_iter()
            .filter(|p| p.available(self.presentation_context()))
            .collect();
        self.workspace
            .layout
            .synchronize(&mut self.dock, &available);
        self.save_workspace();
    }
    pub(crate) fn connection_change_guard(&mut self) -> bool {
        let mapping_dirty = self.show_mapping
            && !self
                .mappings
                .iter()
                .find(|m| m.logical_id == self.mapping_logical)
                .is_some_and(|m| {
                    m.physical_cell == self.mapping_physical && m.name == self.mapping_name
                });
        let blocked = connection_change_blocked(
            self.preferences
                .as_ref()
                .is_some_and(|draft| draft.is_dirty()),
            self.device_settings.dirty(),
            self.device_settings.pending.is_some()
                || self.table.has_pending_edits()
                || self.table.has_assignment_draft()
                || mapping_dirty,
            self.device_settings.maintenance_pending.is_some()||self.device_settings.maintenance_confirmation.is_some(),
            self.identity_pending,
        );
        if blocked {
            self.message = "Save or discard settings drafts and wait for pending operations before changing connection".into();
        }
        blocked
    }
    pub(crate) fn live_status(&self) -> Option<&DeviceStatus> {
        match self.command_route() {
            Some(Transport::WiFi) => self.status.as_ref(),
            Some(Transport::Serial) => self.serial_status.as_ref(),
            _ => None,
        }
    }
    pub(super) fn refresh(&mut self) {
        self.configuration_loaded = false;
        self.last_poll = Instant::now() - Duration::from_secs(2);
        self.last_mapping_poll = Instant::now() - Duration::from_secs(30);
    }
    pub(crate) fn peer_context(&self) -> Option<&crate::capabilities::Manifest> {
        self.live_status()?
            .capabilities
            .as_ref()
            .filter(|c| c.known())
    }
    /// Acknowledged layout slots only; never use this for a command or live value.
    pub(crate) fn presentation_context(&self) -> Option<&crate::capabilities::Manifest> {
        self.presentation.context()
    }
    fn presentation_has_feature(&self, feature: &str) -> bool {
        self.presentation_context().is_some_and(|c| c.has(feature))
    }
    fn presentation_has_table(&self) -> bool {
        self.presentation_context().is_some_and(|c| c.table())
    }
    fn panel_live(&self, panel: Panel) -> bool {
        if matches!(panel,Panel::Events|Panel::Peers) {
            return true;
        }
        if !panel.available(self.peer_context()) {
            return false;
        }
        let required: &[&str] = match panel {
            Panel::Table => &["table"],
            Panel::PixelDiagnostics => &["pixel-diagnostic"],
            Panel::PixelCorrections => &["pixel-corrections"],
            Panel::Keys => &["inputs", "outputs"],
            Panel::Game | Panel::GameSettings => &["inputs", "outputs"],
            Panel::Melodies => &["buzzer"],
            Panel::EffectControls => &["table", "leds", "inputs", "outputs"],
            Panel::FrontPanel => &["display", "text", "buzzer", "lcd"],
            Panel::Hardware => &["telemetry", "inputs", "outputs"],
            Panel::Network => &["network"],
            Panel::Streaming => &["table", "streaming"],
            Panel::Events | Panel::Peers => &[],
        };
        required
            .iter()
            .all(|feature| !self.presentation_has_feature(feature) || self.has_feature(feature))
    }
    pub(crate) fn has_feature(&self, feature: &str) -> bool {
        self.peer_context().is_some_and(|c| c.has(feature))
    }
    pub(crate) fn has_table(&self) -> bool {
        self.peer_context().is_some_and(|c| c.table())
    }
    pub(crate) fn mapping_status(&self) -> String {
        if !self.has_table() {
            return "This connected board has not acknowledged table mapping".into();
        }
        if self.command_route() == Some(Transport::Serial)
            && self.live_status().and_then(peer_http::Pin::from_status) == self.mapping_peer
            && !self.mappings.is_empty()
        {
            return "Confirmed mapping from this board’s immutable USB snapshot · Wi-Fi is not used".into();
        }
        if self.command_route() != Some(Transport::WiFi) {
            return "Mapping readback requires the selected Wi-Fi connection; USB never falls back to Wi-Fi".into();
        }
        let peer = self.status.as_ref().and_then(peer_http::Pin::from_status);
        let confirmed = peer.is_some() && peer == self.mapping_peer && !self.mappings.is_empty();
        match (
            confirmed,
            self.mapping_request_pending.is_some(),
            self.mapping_error.as_deref(),
        ) {
            (true, _, Some(error)) => format!(
                "Using confirmed mapping for this board; refresh failed and will retry: {error}"
            ),
            (false, _, Some(error)) => format!("Mapping not received yet; retrying: {error}"),
            (true, true, _) => "Confirmed mapping for this board · refreshing…".into(),
            (true, false, _) => "Confirmed mapping for this board".into(),
            (false, true, _) => "Reading this board’s mapping…".into(),
            _ => "Waiting to read this board’s mapping; not evidence of an unsupported capability"
                .into(),
        }
    }
    fn synchronize_context(&mut self, ctx: &egui::Context) {
        let route = self.connection_enabled.then(|| {
            format!(
                "{:?}:{}",
                self.transport,
                if self.transport == Transport::Serial {
                    &self.serial_port
                } else {
                    &self.base
                }
            )
        });
        let current = self.live_status().cloned();
        let restore = self.prepare_ui_workspace(ctx, current.as_ref());
        let change = self.presentation.update(route.as_deref(), current.as_ref());
        let available: Vec<_> = Panel::ALL
            .into_iter()
            .filter(|p| p.available(self.presentation_context()))
            .collect();
        let key = self.presentation.workspace_key();
        if key != self.active_context {
            self.active_context = key;
            self.device_name = self
                .peer_context()
                .and_then(|c| crate::identity::normalize_name(&c.hostname).ok());
            self.board_display_name = self
                .peer_context()
                .filter(|c| crate::identity::valid_display_name(&c.name))
                .map(|c| c.name.clone());
            self.identity_saved = self
                .device_name
                .clone()
                .zip(self.board_display_name.clone());
            self.configuration_loaded = false;
            self.table.stop_pending_edits();
            self.table.pixel_sample = None;
            // Only a genuinely different selected peer changes this key. A
            // friendly-name refresh or expired lease cannot discard user state.
            let peer = self.live_status().and_then(peer_http::Pin::from_status);
            if peer.is_none() || peer != self.mapping_peer {
                self.mappings.clear();
                self.mapping_peer = None;
                if !self
                    .mapping_request_pending
                    .as_ref()
                    .is_some_and(|pending| Some(&pending.peer) == peer.as_ref())
                {
                    self.mapping_request_pending = None;
                    self.mapping_error = None;
                }
                self.last_mapping_poll = Instant::now() - Duration::from_secs(16);
                let _ = self.ws_tx.send(WsCommand::Mapping(Vec::new()));
            }
            let _ = self.ws_tx.send(WsCommand::Stream(false));
            let _ = self.ws_tx.send(WsCommand::Sample(false));
            self.streaming = false;
            self.sampling = false;
            self.show_mapping = false;
            self.table.manager_open = false;
        }
        if change.boot_changed {
            // Same board keeps its dock/selection, but no old command, stream or
            // pixel sample survives a new boot's authority boundary.
            self.table.stop_pending_edits();
            self.table.pixel_sample = None;
            self.show_mapping = false;
            self.table.manager_open = false;
        }
        if let Some(snapshot) = restore {
            self.apply_ui_workspace(ctx, snapshot);
        }
        // Capability projection never mutates the durable layout or hide intent.
        self.workspace
            .layout
            .synchronize(&mut self.dock, &available);
        if !self.presentation_has_table() {
            self.table.stop_pending_edits();
            self.table.manager_open = false;
            self.table.editing_order = false;
            self.show_mapping = false;
            self.confirm_reset = false;
        }
        if !self.has_table()
            || !self.has_feature("streaming")
            || self.command_route() != Some(Transport::WiFi)
        {
            if self.streaming || self.sampling {
                let _ = self.ws_tx.send(WsCommand::Stream(false));
                let _ = self.ws_tx.send(WsCommand::Sample(false));
                self.streaming = false;
                self.sampling = false;
            }
        }
        let title = self
            .presentation_context()
            .map(|c| c.title())
            .unwrap_or_else(|| "PakarController · No board connected".into());
        let app_name = self
            .presentation_context()
            .and_then(|p| p.application())
            .map(|a| a.name.clone())
            .unwrap_or_else(|| "PakarController".into());
        if let Some(icon) = crate::branding::sync_peer(ctx, self.presentation_context()) {
            #[cfg(windows)]
            if let Some(desktop) = &self.windows_desktop {
                if let Err(error) = desktop.set_icon(&icon) {
                    self.record_event("Windows application artwork", &error);
                }
            }
            if let Some(tray) = &self.tray {
                if let Err(error) = tray.set_application(&app_name, &icon) {
                    self.record_event("Application artwork", &error);
                }
            }
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(title));
    }
    // Pealayer src/app.rs App::logic: command/event/snapshot work is not paint work.
    pub fn native_logic(&mut self, ctx: &egui::Context) {
        self.window_startup.show_initial(ctx);
        ui::window_geometry::reconcile(ctx);
        while let Ok(command) = self.control.rx.try_recv() {
            self.dispatch(command, ctx);
        }
        if let Some(action) = self.tray.as_mut().and_then(|tray| tray.poll()) {
            match action {
                platform::windows_tray::TrayAction::Show => self.dispatch(Command::Show, ctx),
                platform::windows_tray::TrayAction::Preferences => {
                    self.dispatch(Command::Preferences, ctx)
                }
                platform::windows_tray::TrayAction::DeviceSettings => {
                    self.dispatch(Command::DeviceSettings, ctx)
                }
                platform::windows_tray::TrayAction::Workspaces => {
                    self.dispatch(Command::Workspaces, ctx)
                }
                platform::windows_tray::TrayAction::Refresh => self.refresh(),
                platform::windows_tray::TrayAction::Exit => self.dispatch(Command::Quit, ctx),
            }
        }
        if !self.exiting
            && self.config.close_to_tray
            && self.tray.is_some()
            && ctx.input(|i| i.viewport().close_requested())
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            self.usb_ui_visible = false;
            self.save_workspace();
        }
        self.poll();
        self.bridge_tick();
        let (minimized, occluded) = ctx.input(|i| {
            (
                i.viewport().minimized.unwrap_or(false),
                i.viewport().occluded.unwrap_or(false),
            )
        });
        self.service_pixel_correction_preview(!minimized && !occluded && !self.exiting
            && self.usb_ui_visible && ctx.input(|i|i.focused));
        let ui_demand = crate::usb_demand::visible_demand(
            self.usb_ui_demand,
            self.usb_ui_visible,
            minimized,
            occluded,
            self.exiting,
        );
        let demand =
            if self.connection_enabled && self.transport == Transport::Serial && !self.exiting {
                ui_demand
                    .merge(self.control.demand.demand())
                    .merge(self.bridge.demand())
            } else {
                Default::default()
            };
        let pixels = self.connection_enabled
            && self.transport == Transport::WiFi
            && !self.exiting
            && ui_demand
                .merge(self.control.demand.demand())
                .merge(self.bridge.demand())
                .flags
                & crate::usb_demand::PIXELS
                != 0;
        let ws_demand = (self.connection_gate.epoch(), pixels);
        if self.ws_pixel_demand != Some(ws_demand) {
            let _ = self.ws_tx.send(crate::WsCommand::PixelSubscribe(pixels));
            self.ws_pixel_demand = Some(ws_demand);
        }
        let next = (
            self.serial_port.clone(),
            self.connection_gate.epoch(),
            demand,
        );
        if self.usb_sent_demand.as_ref() != Some(&next) {
            let _ =
                self.serial_tx
                    .send(crate::SerialCommand::Demand(next.0.clone(), next.1, next.2));
            self.usb_sent_demand = Some(next);
        }
        self.synchronize_context(ctx);
        self.checkpoint_ui_workspace(ctx);
        self.process_cell_queue(ctx);
        self.deliver_notifications();
        #[cfg(windows)]
        if let Some(desktop) = &mut self.windows_desktop {
            let errors = desktop.sync(
                ctx.theme() == egui::Theme::Dark,
                self.config.color_palette,
                self.device_settings.pending.is_some()
                    || self.device_settings.maintenance_pending.is_some(),
            );
            for error in errors {
                self.record_event("Windows desktop", &error);
            }
        }
        ctx.request_repaint_after(Duration::from_millis(250));
        let snapshot = self.live_status().cloned();
        let fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
        if let Ok(mut value) = self.control.snapshot.lock() {
            *value = serde_json::json!({"application":self.peer_context().and_then(|p|p.application()).map(|a|a.name.as_str()).unwrap_or("PakarController"),"client_application":"PakarController","app_title":self.peer_context().map(|p|p.title()),"version":env!("CARGO_PKG_VERSION"),"configured_endpoint":self.base,"process_id":std::process::id(),"ready":true,"single_instance":self.config.single_instance,"panels":self.dock.iter_all_tabs().map(|(_,p)|p.name()).collect::<Vec<_>>(),"preferences_open":self.preferences.is_some(),"mapping_open":self.show_mapping,"workspaces_open":self.show_workspaces,"about_open":self.show_about,"device_online":snapshot.is_some(),"websocket_live":self.websocket_live,"serial_live":self.serial_live,"cell_search":ui::cell_search::format_selection(&self.table.selection.ids),"cell_search_draft":self.cell_search,"selected_cells":self.table.selection.cells(),"highlighted_cells":self.table.selection.cells(),"search_matches":self.table.selection.cells(),"selected_cell":self.selected_cell});
            value["device_settings"] = serde_json::json!({"open": self.device_settings.open, "page": self.device_settings.page, "network_loaded": self.device_settings.network.is_some(), "cloud_loaded": self.device_settings.cloud.is_some(), "write_pending": self.device_settings.pending.is_some(), "maintenance_pending": self.device_settings.maintenance_pending.is_some()});
            value["workspace"] = self.workspace.layout.status();
            value["connection"] = serde_json::json!({
                "method": self.transport.label(), "enabled": self.connection_enabled,
                "releasing": self.connection_gate.releasing().map(|method| method.label()),
                "online": self.command_route() == Some(self.transport),
            });
            value["serial_port"] = serde_json::json!(self.serial_port);
            value["pixel_diagnostic"] = serde_json::json!({"wire_pixel":self.pixel_diagnostic.index.clamp(1,480),
                "raw_index":self.pixel_diagnostic.index.clamp(1,480)-1,"pending":self.pixel_diagnostic.pending.is_some(),
                "count":self.pixel_diagnostic.probe().count(),"rgb":self.pixel_diagnostic.probe().rgb,
                "observed":self.pixel_diagnostic.observed.as_ref().map(|o|serde_json::json!({
                    "revision":o.revision,"active":o.active,"index":o.probe.first(),"count":o.probe.count(),"rgb":o.probe.rgb,"off":o.off})),
                "armed":snapshot.as_ref().is_some_and(|s| pixel_diagnostic::ready(s)
                    && peer_http::Pin::from_status(s) == self.pixel_diagnostic.armed),"message":self.pixel_diagnostic.message});
            value["device_status"] = serde_json::json!(snapshot);
            value["firmware_progress"] = serde_json::json!(self.firmware_observation());
            value["client_build"] = serde_json::json!(crate::client_management::build());
            value["peer_identity"] = serde_json::json!({"session_id":crate::peer_identity::session_id(),"application":crate::peer_identity::PRODUCT,"version":crate::peer_identity::VERSION,"compatibility":crate::peer_identity::advertised_policy()});
            value["app_management"] = serde_json::json!(crate::client_management::snapshot());
            value["firmware_feed"] = serde_json::json!(self.firmware_feed.as_ref().map(|f|serde_json::json!({"state":f.state,"session":f.session,"board_id":f.board,"boot_id":f.boot,"delivery_armed":f.settings.deliver})));
            value["bridge"] = self.bridge.snapshot();
            value["message"] = serde_json::json!(self.message);
            value["device_settings"]["message"] = serde_json::json!(self.device_settings.message);
            value["peer_capabilities"] = serde_json::json!(self.peer_context());
            value["presentation"] = serde_json::json!(self.presentation);
            value["messages"] = serde_json::json!(messaging::snapshot());
            value["cell_management"] = serde_json::json!({"open":self.table.manager_open,"selection":self.table.selection.ids,"additive_selection":self.config.additive_cell_selection,"animations":self.config.animate_cells,"indicators":self.config.show_cell_indicators,"color_backgrounds":self.config.color_cell_backgrounds});
            value["screenshot"] = self.capture.status();
            value["minimized"] = serde_json::json!(ctx.input(|i| i.viewport().minimized));
            value["preferences_dirty"] =
                serde_json::json!(self.preferences.as_ref().is_some_and(|d| d.is_dirty()));
            value["diagnostics"] = serde_json::json!({"events":self.event_log.len(),"active":self.event_log.active_count(),"repeated_occurrences":self.event_log.repeated_count(),"status_request_pending":self.status_request_pending,"status_retry_ms":self.status_retry.delay().as_millis()});
            value["diagnostics"]["recent"] = serde_json::json!(self.event_log.iter().rev().take(12)
                .map(|row| serde_json::json!({"kind":row.kind,"message":row.message,"active":row.active,"occurrences":row.occurrences})).collect::<Vec<_>>());
            value["diagnostics"]["active_incidents"] = serde_json::json!(self.event_log.iter().rev().filter(|row| row.active).take(12)
                .map(|row| serde_json::json!({"kind":row.kind,"message":row.message,"occurrences":row.occurrences})).collect::<Vec<_>>());
            value["mapping"] = serde_json::json!({"count":self.mappings.len(),"pending":self.mapping_request_pending.is_some(),"status":self.mapping_status(),"error":self.mapping_error});
            value["usb_commands_pending"] = serde_json::json!(self.serial_operations_pending);
            value["usb_command"] = serde_json::json!(self.serial_timing);
            value["usb_snapshot"] = serde_json::json!(self.serial_capture);
            value["pixel_preview"] = serde_json::json!({
                "available":self.current_pixel_frame().is_some(),
                "frame_revision":self.table.pixel_sample.as_ref().map(|sample|sample.frame.revision),
                "age_ms":self.table.pixel_sample.as_ref().map(|sample|sample.at.elapsed().as_millis()),
                "recent":self.table.pixel_sample.as_ref().is_some_and(|sample|sample.fresh()),
            });
            value["notifications"] = self.notification_snapshot();
            value["tray_available"] = serde_json::json!(self.tray.is_some());
            value["tray_shell"] = self
                .tray
                .as_ref()
                .map(|tray| tray.shell_snapshot())
                .unwrap_or(serde_json::Value::Null);
            value["fullscreen"] = serde_json::json!(fullscreen);
        }
    }
    pub fn draw_shell(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(super::i18n::vertical_layout(super::i18n::rtl()), |ui| {
            self.draw_shell_contents(ui);
        });
    }
    fn draw_shell_contents(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        // App replacement uses the same supported graceful Quit lifecycle. An
        // open editor (even clean), transfer or pending device action postpones it.
        let can_update = self.preferences.is_none()
            && !self.device_settings.open
            && !self.device_settings.dirty()
            && self.device_settings.pending.is_none()
            && self.device_settings.maintenance_pending.is_none()
            && !self.identity_pending
            && self.serial_operations_pending == 0
            && self.board_silent.pending.is_none()
            && !self
                .firmware_observation()
                .is_some_and(|progress| progress.active)
            && !self.table.has_pending_edits()
            && self.pixel_diagnostic.pending.is_none()
            && !self.pixel_groups.dirty
            && self.pixel_groups.pending.is_none()
            && self.pixel_api_reply.is_none()
            && !self.table.manager_open
            && !self.show_mapping
            && !self.show_connection
            && !self.show_workspaces
            && !self.confirm_reset
            && !self.streaming
            && !self.sampling;
        if self.config.single_instance
            && std::env::var_os("PAKAR_SCREENSHOT_PATH").is_none()
            && crate::client_management::tick(&self.config.client_management, &ctx, can_update)
        {
            self.dispatch(Command::Quit, &ctx);
            return;
        }
        let snapshot = self.live_status().cloned();
        if let Some(action) = crate::application_shortcuts::take_action(
            &ctx,
            self.config.keyboard_shortcuts_enabled,
            &self.config.application_shortcuts,
        ) {
            use crate::application_shortcuts::ApplicationAction;
            match action {
                ApplicationAction::Fullscreen => self.dispatch(Command::Fullscreen, &ctx),
                ApplicationAction::Refresh => self.refresh(),
                ApplicationAction::Preferences => self.open_preferences(),
                ApplicationAction::DeviceSettings => {
                    self.device_settings.open = true;
                    self.configuration_loaded = false;
                }
                ApplicationAction::Connect => self.show_connection = true,
                ApplicationAction::CellMapping if self.has_table() => self.show_mapping = true,
                ApplicationAction::CellMapping => {}
            }
        }
        if self.preferences.is_none()
            && !self.device_settings.open
            && !self.show_connection
            && !self.confirm_reset
            && !self.table.manager_open
            && !self.show_mapping
            && !self.show_workspaces
            && !self.show_about
        {
            if self.has_table()
                && ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::F))
            {
                self.reveal_panels.push(Panel::Table);
                self.focus_cell_search = true;
            }
            let previous = ctx.input_mut(|i| i.consume_key(egui::Modifiers::SHIFT, egui::Key::F3));
            let next = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F3));
            if self.has_table() && (previous || next) {
                if let Some(id) = self.table.selection.navigate(self.selected_cell, previous) {
                    self.selected_cell = id;
                    self.reveal_panels.push(Panel::Table);
                }
            }
        }
        let shortcuts = self.config.application_shortcuts.clone();
        egui::Panel::top("pakar-menu").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                menu::top_menu_button(ui, crate::ui::i18n::text("File"), |ui| {
                    if ui
                        .add(
                            egui::Button::new(format!(
                                "{}  {}",
                                icons::CIRCUITRY,
                                crate::ui::i18n::text("Device settings…")
                            ))
                            .shortcut_text(&shortcuts.device_settings),
                        )
                        .clicked()
                    {
                        self.device_settings.open = true;
                        self.configuration_loaded = false;
                        ui.close();
                    }
                    if ui
                        .add(
                            egui::Button::new(format!(
                                "{}  {}",
                                icons::PLUG,
                                crate::ui::i18n::text("Connect…")
                            ))
                            .shortcut_text(&shortcuts.connect),
                        )
                        .clicked()
                    {
                        self.show_connection = true;
                        ui.close();
                    }
                    if self.has_table()
                        && ui
                            .add(
                                egui::Button::new(format!(
                                    "{}  {}",
                                    icons::GRID_FOUR,
                                    crate::ui::i18n::text("Cell mapping…")
                                ))
                                .shortcut_text(&shortcuts.cell_mapping),
                            )
                            .clicked()
                        && self.has_table()
                    {
                        self.show_mapping = true;
                        ui.close();
                    }
                    ui.separator();
                    if ui
                        .add(
                            egui::Button::new(format!(
                                "{}  {}",
                                icons::GEAR,
                                crate::ui::i18n::text("Preferences…")
                            ))
                            .shortcut_text(&shortcuts.preferences),
                        )
                        .clicked()
                    {
                        self.open_preferences();
                        ui.close();
                    }
                    ui.separator();
                    if ui
                        .button(format!(
                            "{}  {}",
                            icons::POWER,
                            crate::ui::i18n::text("Quit")
                        ))
                        .clicked()
                    {
                        self.dispatch(Command::Quit, &ctx);
                    }
                });
                menu::top_menu_button(ui, crate::ui::i18n::text("Workspace"), |ui| {
                    if ui
                        .add(
                            egui::Button::new(format!(
                                "{}  {}",
                                icons::APP_WINDOW,
                                crate::ui::i18n::text("Toggle fullscreen")
                            ))
                            .shortcut_text(&shortcuts.fullscreen),
                        )
                        .clicked()
                    {
                        self.dispatch(Command::Fullscreen, &ctx);
                        ui.close();
                    }
                    ui.separator();
                    icons::submenu(
                        ui,
                        format!("{}  {}", icons::TABS, crate::ui::i18n::text("Panels")),
                        |ui| layout::draw_workspace_tab_menu(self, ui),
                    );
                    ui.separator();
                    if ui
                        .button(format!(
                            "{}  {}",
                            icons::TABS,
                            crate::ui::i18n::text("Manage workspaces…")
                        ))
                        .clicked()
                    {
                        self.show_workspaces = true;
                        ui.close();
                    }
                    if ui
                        .button(format!(
                            "{}  Reset workspace layout",
                            icons::ARROW_COUNTER_CLOCKWISE
                        ))
                        .clicked()
                    {
                        self.reset_workspace_layout();
                        ui.close();
                    }
                });
                menu::top_menu_button(ui, crate::ui::i18n::text("Device"), |ui| {
                    if ui
                        .add(
                            egui::Button::new(format!(
                                "{}  {}",
                                icons::ARROW_CLOCKWISE,
                                crate::ui::i18n::text("Refresh")
                            ))
                            .shortcut_text(&shortcuts.refresh),
                        )
                        .clicked()
                    {
                        self.refresh();
                        ui.close();
                    }
                    if self.has_feature("network")
                        && ui
                            .button(format!(
                                "{}  {}",
                                icons::GLOBE,
                                crate::ui::i18n::text("Network & cloud")
                            ))
                            .clicked()
                    {
                        self.reveal_panels.push(Panel::Network);
                        ui.close();
                    }
                    if self.has_table()
                        && ui
                            .add_enabled(
                                snapshot.is_some(),
                                egui::Button::new(format!(
                                    "{}  Reset game…",
                                    icons::ARROW_COUNTER_CLOCKWISE
                                )),
                            )
                            .clicked()
                    {
                        self.confirm_reset = true;
                        ui.close();
                    }
                });
                menu::top_menu_button(ui, crate::ui::i18n::text("Help"), |ui| {
                    if ui
                        .button(format!(
                            "{}  {}",
                            icons::DOWNLOAD_SIMPLE,
                            crate::ui::i18n::text("Save workspace screenshot…")
                        ))
                        .clicked()
                    {
                        ui.close();
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("PNG image", &["png"])
                            .set_file_name("PakarController.png")
                            .save_file()
                        {
                            if let Err(error) = self.capture.request(path.to_string_lossy().into())
                            {
                                self.message = error;
                            }
                        }
                    }
                    if ui
                        .button(format!(
                            "{}  About {}…",
                            icons::INFO,
                            self.peer_context()
                                .and_then(|p| p.application())
                                .map(|a| a.name.as_str())
                                .unwrap_or("PakarController")
                        ))
                        .clicked()
                    {
                        self.show_about = true;
                        ui.close();
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(icons::GEAR)
                        .on_hover_text(crate::ui::i18n::text("Preferences (Ctrl+,)"))
                        .clicked()
                    {
                        self.open_preferences();
                    }
                    if ui
                        .button(icons::PLUG)
                        .on_hover_text(crate::ui::i18n::text("Connect (Ctrl+K)"))
                        .clicked()
                    {
                        self.show_connection = true;
                    }
                    ui.label(
                        egui::RichText::new(
                            self.peer_context()
                                .map(|c| c.title())
                                .unwrap_or_else(|| "PakarController".into()),
                        )
                        .strong(),
                    );
                    crate::branding::peer_logo(
                        ui,
                        18.0,
                        self.peer_context().and_then(|p| p.application()),
                    );
                });
            });
        });
        let s = snapshot.clone().unwrap_or_default();
        let firmware_progress = self.firmware_observation().filter(|p| p.active);
        if let Some(progress) = &firmware_progress {
            egui::Panel::top("pakar-update-progress").show(ui, |ui| {
                crate::firmware_progress::draw(ui, progress);
                if ui.small_button("Firmware details…").clicked() {
                    self.device_settings.open = true;
                    self.device_settings.page = 9;
                }
            });
        }
        let progress_label = firmware_progress.as_ref().map(|p| p.phase.replace('_', " "));
        let progress_detail = firmware_progress.as_ref().map(|p| {
            let bytes = p.bytes_total.map(|total| format!("{} / {total} bytes", p.bytes_done))
                .unwrap_or_else(|| format!("{} bytes · total unknown", p.bytes_done));
            format!("{}\n{bytes}", p.label())
        });
        let status_actions = ui::status_bar::draw(
            ui,
            ui::status_bar::StatusBar {
                status_rgb: s.status_rgb.as_ref(),
                connected: self.command_route().is_some(),
                connection_enabled: self.connection_enabled,
                serial_selected: self.transport == Transport::Serial,
                event_count: self.event_log.len(),
                error_count: self.event_log.active_count(),
                progress: firmware_progress.as_ref().map(|p| ui::status_bar::Progress {
                    label: progress_label.as_deref().unwrap_or_default(),
                    detail: progress_detail.as_deref().unwrap_or_default(),
                    percent: p.fraction.map(|fraction| fraction * 100.0),
                }),
                peer_known: snapshot.is_some(),
                avr_available: ["display", "text", "buzzer", "inputs", "outputs", "settings"]
                    .iter()
                    .any(|v| self.has_feature(v)),
                display_available: self.has_feature("display"),
                network_available: self.has_feature("network"),
                telemetry_available: self.has_feature("telemetry"),
                message: &self.message,
                hostname: &s.hostname,
                station: if self.transport != Transport::WiFi {
                    ""
                } else if s.uplink == "ethernet" && s.uplink_ipv4_ready == Some(true) {
                    "Ethernet"
                } else {
                    &s.station_ssid
                },
                avr_online: s.avr_online,
                tm1637_ready: s.tm1637_ack,
                websocket_live: self.transport == Transport::WiFi && self.websocket_live,
                serial_live: self.transport == Transport::Serial && self.serial_live,
            },
            &mut self.config.status_items,
        );
        for action in status_actions {
            match action {
                ui::status_bar::Action::Save => self.save_status_bar_preferences(),
                ui::status_bar::Action::Refresh => self.refresh(),
                ui::status_bar::Action::Connect => self.show_connection = true,
                ui::status_bar::Action::Hardware => self.reveal_panels.push(Panel::Hardware),
                ui::status_bar::Action::Network => self.reveal_panels.push(Panel::Network),
                ui::status_bar::Action::Preferences => self.open_preferences(),
                ui::status_bar::Action::Events => self.reveal_panels.push(Panel::Events),
                ui::status_bar::Action::Updates => {
                    self.device_settings.open = true;
                    self.device_settings.page = 9;
                }
            }
        }
        let mut connection_actions = Vec::new();
        egui::Panel::top("pakar-workspace-toolbar")
            .frame(egui::Frame::new().inner_margin(egui::Margin::symmetric(12, 8)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(icons::GRID_FOUR)
                            .size(19.0)
                            .color(ui.visuals().selection.bg_fill),
                    );
                    ui.strong(
                        self.presentation_context()
                            .map(|c| c.name.as_str())
                            .filter(|s| !s.is_empty())
                            .unwrap_or("Controller workspace"),
                    );
                    ui.separator();
                    badge(
                        ui,
                        if snapshot.is_some() {
                            "Device connected"
                        } else {
                            "Device offline"
                        },
                        snapshot.is_some(),
                    );
                    ui::connection_controls::draw(
                        ui,
                        ui::connection_controls::State {
                            method: self.transport,
                            enabled: self.connection_enabled,
                            live: self.command_route().is_some(),
                            peer_known: self.presentation_context().is_some(),
                            endpoint: &self.base,
                            port: &self.serial_port,
                            identity_pending: self.identity_pending,
                        },
                        || {
                            serialport::available_ports()
                                .map(|ports| ports.into_iter().map(|port| port.port_name).collect())
                                .map_err(|error| error.to_string())
                        },
                        &mut connection_actions,
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .button(format!(
                                "{}  {}",
                                icons::TABS,
                                crate::ui::i18n::text("Workspaces")
                            ))
                            .clicked()
                        {
                            self.show_workspaces = true;
                        }
                        ui.menu_button(
                            format!("{}  {}", icons::TABS, crate::ui::i18n::text("Panels")),
                            |ui| layout::draw_workspace_tab_menu(self, ui),
                        );
                        if self.has_table()
                            && ui
                                .button(format!(
                                    "{}  {}",
                                    icons::GRID_FOUR,
                                    crate::ui::i18n::text("Mapping")
                                ))
                                .clicked()
                            && self.has_table()
                        {
                            self.show_mapping = true;
                        }
                        if ui
                            .button(icons::ARROW_CLOCKWISE)
                            .on_hover_text(crate::ui::i18n::text("Refresh"))
                            .clicked()
                        {
                            self.refresh();
                        }
                    });
                });
            });
        for action in connection_actions {
            use ui::connection_controls::Action;
            match action {
                Action::Method(method) if !self.identity_pending => {
                    self.set_connection(method, false)
                }
                Action::Lifecycle(enabled) if !self.identity_pending => {
                    self.set_connection(self.transport, enabled)
                }
                Action::Port(port) if !self.identity_pending => {
                    if self.connection_change_guard() {
                        continue;
                    }
                    self.set_connection(self.transport, false);
                    if self.connection_enabled {
                        continue;
                    } // A refused release cannot change its endpoint.
                    self.serial_port = port;
                    self.config.serial_port = self.serial_port.clone();
                    self.save_workspace();
                }
                Action::Configure => self.show_connection = true,
                _ => {}
            }
        }
        self.open_panels.clear();
        let layout_changed = !self.close_panels.is_empty() || !self.reveal_panels.is_empty();
        for panel in std::mem::take(&mut self.close_panels) {
            if panel == Panel::Events {
                self.workspace
                    .layout
                    .events_visibility(&mut self.dock, None);
            }
            if let Some(path) = self.dock.find_tab(&panel) {
                self.dock.remove_tab(path);
            }
        }
        for panel in std::mem::take(&mut self.reveal_panels) {
            if panel.available(self.presentation_context()) {
                self.workspace.layout.show(&mut self.dock, panel);
            }
        }
        if layout_changed {
            self.save_workspace();
        }
        // Workspace restore/menu actions occur after discovery synchronization.
        // Apply the same gate before any tab header is painted in this frame.
        let available: Vec<_> = Panel::ALL
            .into_iter()
            .filter(|p| p.available(self.presentation_context()))
            .collect();
        self.workspace
            .layout
            .synchronize(&mut self.dock, &available);
        let events_policy = (
            self.config.events_auto_hide,
            self.workspace
                .errors
                .borrow()
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
        );
        if self.workspace.events_policy.as_ref() != Some(&events_policy) {
            let show = if !events_policy.1.is_empty() {
                Some(true)
            } else if events_policy.0 {
                Some(false)
            } else {
                None
            };
            self.workspace
                .layout
                .events_visibility(&mut self.dock, show);
            self.workspace.events_policy = Some(events_policy);
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(0))
            .show(ui, |ui| {
                if self.dock.iter_all_tabs().count() == 0 {
                    ui.centered_and_justified(|ui| {
                        if ui
                            .button(format!(
                                "{}  {}",
                                icons::TABS,
                                crate::ui::i18n::text("Restore workspace")
                            ))
                            .clicked()
                        {
                            self.reset_workspace_layout();
                        }
                    });
                } else if self.config.workspace_ui == crate::config::WorkspaceUi::CraftUi {
                    self.open_panels = self.dock.iter_all_tabs().map(|(_, p)| *p).collect();
                    super::craft_workspace::show(self, ui);
                    self.open_panels.clear();
                } else {
                    self.open_panels = self.dock.iter_all_tabs().map(|(_, p)| *p).collect();
                    // An offline error must not turn the event log into the
                    // entire main workspace while board capabilities are absent.
                    if self.open_panels == vec![Panel::Events] {
                        ui.add_space((ui.available_height() - 200.0).max(0.0));
                    }
                    let tab_rects_id = egui::Id::new("pakar-workspace-tab-button-rects");
                    ctx.data_mut(|data| data.insert_temp(tab_rects_id, Vec::<egui::Rect>::new()));
                    let mut dock =
                        std::mem::replace(&mut self.dock, egui_dock::DockState::new(vec![]));
                    let mut style = egui_dock::Style::from_egui(ui.style().as_ref());
                    let tab_bar_height = style.tab_bar.height;
                    style.buttons.collapse_tabs_color = egui::Color32::TRANSPARENT;
                    style.buttons.collapse_tabs_active_color = egui::Color32::TRANSPARENT;
                    egui_dock::DockArea::new(&mut dock)
                        .style(style)
                        .show_leaf_collapse_buttons(true)
                        .show_inside(ui, &mut layout::PakarTabViewer { app: self });
                    self.dock = dock;
                    layout::paint_dock_disclosure_icons(ui, &self.dock, tab_bar_height);
                    // Adapted from Pealayer app.rs: empty header popup excludes real tab hit rects.
                    let tab_rects = ctx.data_mut(|data| {
                        data.get_temp::<Vec<egui::Rect>>(tab_rects_id)
                            .unwrap_or_default()
                    });
                    let pointer = ctx.pointer_hover_pos();
                    let in_header = pointer.is_some_and(|pos| {
                        self.dock.iter_leaves().any(|(_, leaf)| {
                            let header = egui::Rect::from_min_size(
                                leaf.rect.min,
                                egui::vec2(leaf.rect.width(), tab_bar_height),
                            );
                            header.contains(pos) && !tab_rects.iter().any(|rect| rect.contains(pos))
                        })
                    });
                    let open = in_header
                        && ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Secondary));
                    let anchor = ui.interact(
                        ui.max_rect(),
                        egui::Id::new("pakar-empty-header-anchor"),
                        egui::Sense::hover(),
                    );
                    egui::Popup::menu(&anchor)
                        .id(egui::Id::new("pakar-empty-header-popup"))
                        .at_pointer_fixed()
                        .open_memory(open.then_some(egui::SetOpenCommand::Bool(true)))
                        .show(|ui| {
                            ui::cell_table::compact_menu(ui);
                            ui.strong(crate::ui::i18n::text("Panels"));
                            ui.separator();
                            layout::draw_workspace_tab_menu(self, ui);
                        });
                    self.open_panels.clear();
                }
            });
        ui::preferences::draw(self, &ctx);
        ui::device_settings::draw(self, &ctx);
        self.draw_dialogs(&ctx);
        self.draw_cell_manager(&ctx);
        if self.sync_cell_selection_search() {
            ctx.request_repaint();
        }
        let details_label = ui::i18n::text("Technical details");
        ui::toasts::draw(&ctx, &details_label);
    }
    pub fn draw_panel(&mut self, ui: &mut egui::Ui, panel: Panel) {
        if !panel.available(self.presentation_context()) {
            return;
        }
        let flags = match panel {
            Panel::Table if self.config.cell_shape != super::cell_pixels::Shape::Rounded => {
                crate::usb_demand::PIXELS
            }
            Panel::Keys | Panel::PixelDiagnostics => crate::usb_demand::PIXELS,
            Panel::FrontPanel | Panel::Melodies => crate::usb_demand::PANEL | crate::usb_demand::PERIPHERAL,
            Panel::Events => crate::usb_demand::PANEL,
            _ => 0,
        };
        self.usb_ui_demand.0 = self.usb_ui_demand.0.merge(crate::usb_demand::Demand {
            flags,
            table: !matches!(panel,Panel::Events|Panel::Peers),
        });
        let live = self.panel_live(panel);
        dialog::scroll_column(
            ui,
            ("panel-body", panel),
            Some(ui.available_height()),
            |ui| {
                let width = ui.available_width();
                let body = ui.add_enabled_ui(live, |ui| match panel {
                    Panel::Table => self.draw_table(ui),
                    Panel::PixelDiagnostics => {
                        if self.live_status().is_some_and(pixel_groups::advertised) {
                            ui.horizontal_wrapped(|ui| {
                                ui.selectable_value(&mut self.pixel_groups.open,false,format!("{}  Pixel diagnostics",icons::MAGNIFYING_GLASS));
                                ui.selectable_value(&mut self.pixel_groups.open,true,format!("{}  Physical groups",icons::SLIDERS_HORIZONTAL));
                            });
                            ui.add_space(8.0);
                        }
                        if self.pixel_groups.open && self.live_status().is_some_and(pixel_groups::advertised) {
                            dialog::section(ui,icons::SLIDERS_HORIZONTAL,"Pixels per physical group",|ui|self.draw_pixel_groups(ui));
                            return;
                        }
                        dialog::section(ui, icons::MAGNIFYING_GLASS, "Pixel diagnostics", |ui| {
                            self.draw_pixel_diagnostic(ui)
                        });
                    }
                    Panel::PixelCorrections => {
                        dialog::section(ui, icons::SLIDERS_HORIZONTAL, "Pixel corrections", |ui| {
                            self.draw_pixel_corrections(ui)
                        });
                    }
                    Panel::Keys => self.draw_hardware(ui, true),
                    Panel::Game => self.draw_game(ui),
                    Panel::GameSettings => self.draw_game_settings(ui),
                    Panel::Melodies => self.draw_melodies(ui),
                    Panel::EffectControls => self.draw_effects(ui),
                    Panel::FrontPanel => self.draw_front_panel(ui),
                    Panel::Hardware => self.draw_hardware(ui, false),
                    Panel::Network => self.draw_network(ui),
                    Panel::Peers => {
                        let destination=self.live_status().filter(|_|self.peer_maintenance_ready()).and_then(|s|Some((s.capabilities.as_ref()?.role.clone(),s.version.clone(),peer_http::Pin::from_status(s)?)));
                        if let Some(action)=super::peers::draw(ui,&mut self.peer_workspace,destination.as_ref().map(|(role,version,pin)|(role.as_str(),version.as_str(),pin))) {
                            match action {
                                super::peers::Action::OpenControls(origin) if !self.connection_change_guard()=>{
                                    self.set_connection(self.transport,false);
                                    self.base=origin;self.user="admin".into();self.password.clear();
                                    self.set_connection(Transport::WiFi,false);self.show_connection=true;
                                },
                                super::peers::Action::Receive(peer)=>crate::client_management::request_peer(peer,self.config.client_management.clone(),ui.ctx()),
                                super::peers::Action::Apply=>crate::client_management::request(crate::client_management::Action::Apply,self.config.client_management.clone(),ui.ctx()),
                                super::peers::Action::StageBoard{source,target} if !self.connection_change_guard()=>{
                                    self.device_settings.open=true;self.device_settings.page=super::device_settings::page_index("firmware").unwrap_or(9);
                                    self.device_settings.peer_source_user="admin".into();self.device_settings.peer_source_password.clear();
                                    self.device_settings.maintenance_confirmation=Some(crate::maintenance::Operation::PeerStage{source,authorization:String::new(),target});
                                },
                                _=>{},
                            }
                        }
                    },
                    Panel::Streaming => self.draw_streaming(ui),
                    Panel::Events => self.draw_events(ui),
                });
                let minimum = self.presentation.panel_min_height(
                    panel.name(),
                    width,
                    body.response.rect.height(),
                    live,
                );
                if minimum > 0.0 {
                    ui.set_min_height(minimum);
                }
            },
        );
    }
    fn draw_table(&mut self, ui: &mut egui::Ui) {
        if !self.presentation_has_table() {
            return;
        }
        self.sync_cell_selection_search();
        let snapshot = self.live_status().cloned();
        ui.horizontal(|ui| {
            ui.heading(crate::ui::i18n::text("Pakar table"));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(s) = &snapshot {
                    ui.label(
                        egui::RichText::new(format!("{} · {} pixels", s.mode, s.pixels)).weak(),
                    );
                }
            });
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let height = super::control_density::height(ui).max(30.0);
            ui.spacing_mut().interact_size.y = height;
            let search_width = super::cell_table::find_row_width(ui.available_width(), height, ui.spacing().item_spacing.x);
            let search = super::cell_table::find_field(ui, &mut self.cell_search, search_width)
                .on_hover_text(crate::ui::i18n::text("Exact logical cell IDs, lists and ranges. Search does not change LEDs. Ctrl+F: find; Enter/F3: next; Shift+F3: previous; Escape: clear."));
            if self.focus_cell_search {
                search.request_focus();
                self.focus_cell_search = false;
            }
            if search.has_focus() && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                self.cell_search.clear();
                self.apply_cell_search_selection();
            }
            if search.changed() { self.apply_cell_search_selection(); }
            if search.lost_focus() && ui::cell_search::parse(&self.cell_search).is_ok() {
                self.cell_search = ui::cell_search::format_selection(&self.table.selection.ids);
            }
            let previous = ui.add_enabled(!self.table.selection.ids.is_empty(), egui::Button::new(icons::CARET_LEFT).min_size(egui::vec2(height, height)))
                .on_hover_text(crate::ui::i18n::text("Previous selected cell (Shift+F3)")).clicked();
            let next = ui.add_enabled(!self.table.selection.ids.is_empty(), egui::Button::new(icons::CARET_RIGHT).min_size(egui::vec2(height, height)))
                .on_hover_text(crate::ui::i18n::text("Next selected cell (Enter / F3)")).clicked()
                || (search.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)));
            if (previous || next) && let Some(id) = self.table.selection.navigate(self.selected_cell, previous) {
                self.selected_cell = id;
                if search.lost_focus() { search.request_focus(); }
            }
            if ui.add_enabled(!self.cell_search.is_empty(), egui::Button::new(icons::X).min_size(egui::vec2(height, height)))
                .on_hover_text(crate::ui::i18n::text("Clear cell selection")).clicked() {
                self.cell_search.clear();
                self.apply_cell_search_selection();
            }
        });
        let search_result = ui::cell_search::parse(&self.cell_search);
        let (mut helper, tone) = if let Err(error) = &search_result {
            (error.clone(), super::cell_table::HelperTone::Error)
        } else if self.mapping_error.is_some() {
            (self.mapping_status(), super::cell_table::HelperTone::Caution)
        } else if !self.table.selection.ids.is_empty() {
            let cells = self.table.selection.cells();
            let current = cells.iter().position(|id| *id == self.selected_cell);
            (match current {
                    Some(index) => format!(
                        "Selection {} of {} · logical cell {}",
                        index + 1,
                        cells.len(),
                        self.selected_cell
                    ),
                    None => format!("{} selected cells", cells.len()),
                }, super::cell_table::HelperTone::Normal)
        } else {
            ("Select cells on the table, or enter IDs and ranges.".into(), super::cell_table::HelperTone::Normal)
        };
        if tone == super::cell_table::HelperTone::Normal && !self.table.feedback().is_empty() {
            helper.push_str(" · ");
            helper.push_str(self.table.feedback());
        }
        super::cell_table::table_helper(ui, &helper, tone);
        self.cell_selection_toolbar(ui);
        ui.add_space(8.0);
        let (pitch, height) = super::cell_table::grid_metrics(1280.0, 400.0,
            self.config.cell_shape == super::cell_pixels::Shape::Rounded);
        let (canvas, grid_origin) = super::cell_table::grid_canvas(pitch, height);
        ui.horizontal_wrapped(|ui| {
            if ui.selectable_label(self.table.viewport.is_none(), "Fit").on_hover_text("Fit table to this panel").clicked() {
                self.table.viewport = None;
            }
            for (label, factor) in [("−", 1.25), ("+", 0.8)] {
                if ui.button(label).clicked() {
                    let view = self.table.viewport.unwrap_or(canvas);
                    let scale = (view.width() * factor / canvas.width()).clamp(0.125, 2.0);
                    self.table.viewport = Some(egui::Rect::from_center_size(view.center(), canvas.size() * scale));
                }
            }
            ui.weak("Pinch / Ctrl+wheel · middle-drag to pan");
        });
        let sample_helper = if self
            .table
            .pixel_sample
            .as_ref()
            .is_none_or(|sample| !sample.fresh())
        { "Cell states · awaiting pixel RGB" }
        else { "Live pixel colors" };
        let size = egui::vec2(ui.available_width().max(1.0),
            (ui.clip_rect().bottom() - ui.cursor().min.y - 38.0).max(80.0));
        let fit = (size.x / canvas.width()).min(size.y / canvas.height());
        let mut view = self.table.viewport.unwrap_or(canvas);
        let viewport = ui.allocate_ui_with_layout(size, egui::Layout::top_down(egui::Align::Min), |ui| {
            // Scene transforms both painting and hit-testing. Its pointer-anchored
            // scale matches Pealayer timelineZoomAtPointer without media semantics.
            let result = egui::Scene::new()
                .max_inner_size(canvas.size())
                .zoom_range(fit * 0.5..=fit * 8.0)
                .drag_pan_buttons(egui::containers::DragPanButtons::MIDDLE)
                .show(ui, &mut view, |ui| {
                    self.draw_cell_grid(ui, canvas, grid_origin, pitch, height);
                    ui.set_min_size(canvas.size());
                });
            if result.response.changed() { self.table.viewport = Some(view); }
        });
        finish_table_area(ui, viewport.response.rect);
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!(
                    "Cursor: cell {} · {} selected · {}",
                    self.selected_cell,
                    self.table.selection.ids.len(),
                    sample_helper,
                ))
                .weak(),
            );
            if snapshot.is_none() {
                if ui
                    .button(format!(
                        "{}  {}",
                        icons::PLUG,
                        crate::ui::i18n::text("Connect…")
                    ))
                    .clicked()
                {
                    self.show_connection = true;
                }
            }
        });
    }
    pub(crate) fn peer_maintenance_ready(&self)->bool {
        self.has_feature("peer-exchange") && (self.http_writable() ||
            self.command_route()==Some(Transport::Serial)&&self.has_feature("usb-api")&&self.pixel_api_reply.is_none())
    }
    pub(crate) fn submit_peer_maintenance(&mut self,id:u64,operation:crate::maintenance::Operation)->Result<(),String>{
        if self.command_route()!=Some(Transport::Serial){
            return self.http_tx.send(HttpCommand::Maintenance(self.endpoint(),id,operation)).map_err(|_|"Maintenance queue unavailable".into());
        }
        if !self.peer_maintenance_ready(){return Err("USB peer maintenance is busy or unsupported; nothing sent".into());}
        let pin=self.live_status().and_then(peer_http::Pin::from_status).ok_or("USB destination unavailable")?;
        if let crate::maintenance::Operation::PeerStage{target,..}=&operation && *target!=pin{return Err("USB destination changed; nothing sent".into());}
        if let crate::maintenance::Operation::RemoteApply(job)|crate::maintenance::Operation::RemoteCancel(job)=&operation
            && !pin.matches_identity(pin.board_id(),&job.boot_id){return Err("USB job belongs to a different boot; nothing sent".into());}
        let (method,path,body)=crate::maintenance::usb_remote_request(&operation)?;
        self.begin_pixel_api(pixel_groups::Purpose::Maintenance{id,operation},method,path,body)
    }
    fn begin_pixel_api(&mut self, purpose: pixel_groups::Purpose, method: &str, path: &str, body: serde_json::Value) -> Result<(),String> {
        if self.pixel_api_reply.is_some() || self.command_route()!=Some(Transport::Serial) { return Err("USB API owner busy or unavailable; nothing sent".into()); }
        let status=self.live_status().ok_or("Fresh USB board identity unavailable")?;
        let peer=peer_http::Pin::from_status(status).ok_or("USB peer unavailable")?;
        let transaction=crate::usb_api::Transaction::new(status,method,path,body)?;
        let admission=self.serial_admission().ok_or("USB admission unavailable")?;
        let (reply,receiver)=std::sync::mpsc::channel();
        self.serial_tx.send(crate::SerialCommand::Api(self.serial_port.clone(),crate::usb_api::Work{admission,transaction,reply,wait:None,revoked:vec![]}))
            .map_err(|_|"USB worker unavailable")?;
        self.pixel_api_reply=Some((peer,purpose,receiver)); Ok(())
    }
    pub(crate) fn poll_pixel_api(&mut self) {
        let Some((_,_,receiver))=&self.pixel_api_reply else {return;};
        let result=match receiver.try_recv() { Ok(value)=>value, Err(TryRecvError::Empty)=>return,
            Err(TryRecvError::Disconnected)=>Err("USB API ended without a complete receipt; no replay".into()) };
        let (peer,purpose,_)=self.pixel_api_reply.take().unwrap();
        let result=if self.transport!=Transport::Serial || self.live_status().and_then(peer_http::Pin::from_status)!=Some(peer.clone()) {
            Err("USB API peer retired; result not applied".into())
        } else {result};
        match purpose {
            pixel_groups::Purpose::Maintenance{id,operation}=>{
                let result=result.and_then(|value|{
                    if !peer.matches_identity(peer.board_id(),value["boot_id"].as_str().unwrap_or("")){return Err("USB firmware receipt belongs to a different boot; no replay".into());}
                    crate::maintenance::verify_remote_receipt(&operation,value)
                });
                self.finish_maintenance(id,result);
            },
            pixel_groups::Purpose::Minigame(request) => {
                match result.and_then(|value|request.verify(value)) {
                    Ok(observed) => {
                        if let Some(status)=self.serial_status.as_mut() {
                            if observed.command_clock_ms>=status.command_clock_ms {
                                status.minigame=observed.minigame;
                                status.command_clock_ms=observed.command_clock_ms;
                                status.winner=observed.winner;
                            }
                        }
                        self.message="Game command ACK and same-board readback verified over USB".into();
                    },
                    Err(error)=>self.message=error,
                }
            },
            pixel_groups::Purpose::Frame{revision,layout_revision,at}=> {
                if let Ok(value)=result && let Some(status)=self.live_status() {
                    let peer=peer_http::Pin::from_status(status);
                    let current=pixel_groups::for_status(status);
                    if peer.is_some_and(|pin|pin.matches_identity(value["board_id"].as_str().unwrap_or(""),value["boot_id"].as_str().unwrap_or("")))
                        && current.is_some_and(|layout|layout.revision==layout_revision)
                        && let Ok(frame)=serde_json::from_value::<super::cell_pixels::Frame>(value["pixel_frame"].clone())
                        && frame.valid() && frame.group_layout_revision==Some(layout_revision) && frame.revision>=revision {
                        super::cell_pixels::Sample::observe_at(&mut self.table.pixel_sample,Some(frame),at);
                    }
                }
            }
            pixel_groups::Purpose::Groups => match result {
                Ok(value)=>self.receive_configuration(pixel_groups::PATH,value),
                Err(error)=>{self.pixel_groups.pending=None;self.pixel_groups.expected=None;self.pixel_groups.message=error;}
            },
            pixel_groups::Purpose::Diagnostic{id,probe,off,request_id,previous}=> {
                let checked=result.and_then(|value|pixel_diagnostic::verify_api_receipt(&previous,&value,&request_id,probe,off));
                match checked { Ok(observed)=> {
                    self.pixel_diagnostic.observed=Some(observed);
                    self.pixel_diagnostic.finish(id,"Board confirmed diagnostic publication over USB".into());
                }, Err(error)=>self.pixel_diagnostic.finish(id,error) }
            }
        }
    }
    pub(crate) fn request_missing_usb_pixels(&mut self) {
        if self.pixel_api_reply.is_some() || self.pixel_diagnostic.pending.is_some() || self.pixel_groups.pending.is_some()
            || self.command_route()!=Some(Transport::Serial) || self.usb_ui_demand.0.flags & crate::usb_demand::PIXELS == 0 {return;}
        let Some(status)=self.live_status() else{return;};
        let part=&status.pixel_frame_partial;
        if part["rgb_omitted"]!=true || part["pending"]!=false {return;}
        let Some(revision)=part["revision"].as_u64().and_then(|v|u32::try_from(v).ok()) else{return;};
        let Some(layout)=pixel_groups::for_status(status) else{return;};
        if part["group_layout_revision"].as_u64()!=Some(u64::from(layout.revision))
            || self.table.pixel_sample.as_ref().is_some_and(|sample|sample.frame.revision>=revision && sample.frame.group_layout_revision==Some(layout.revision)) {return;}
        let _=self.begin_pixel_api(pixel_groups::Purpose::Frame{revision,layout_revision:layout.revision,at:Instant::now()},
            "GET","/api/pixels/frame",serde_json::json!({"group_layout_revision":layout.revision}));
    }
    fn draw_pixel_groups(&mut self, ui: &mut egui::Ui) {
        let status=self.live_status().cloned();
        let pin=status.as_ref().and_then(peer_http::Pin::from_status);
        if self.pixel_groups.owner!=pin { self.pixel_groups=pixel_groups::Editor{owner:pin.clone(),open:true,..Default::default()}; }
        let Some(status)=status.filter(pixel_groups::advertised) else {ui.weak("Physical group configuration is not acknowledged by this board.");return;};
        let ready=pin.is_some() && self.command_route()==Some(self.transport) && !status.programming_latch
            && (self.transport==Transport::WiFi || status.capabilities.as_ref().is_some_and(|c|c.has("usb-api")));
        if self.pixel_groups.pending.is_some_and(|at|at.elapsed().as_secs()>=25) {
            self.pixel_groups.pending=None;self.pixel_groups.expected=None;
            self.pixel_groups.message="Group layout outcome unavailable. Read from board; no write replayed.".into();
        }
        let busy=self.pixel_groups.pending.is_some();
        let mut read=ready && !busy && self.pixel_groups.value.is_none() && self.pixel_groups.message.is_empty();
        let mut save=false;
        // A confirmed configuration receipt may precede the next telemetry push.
        // Do not reload it repeatedly from an older snapshot or starve USB capture.
        let changed=self.pixel_groups.value.as_ref().zip(pixel_groups::for_status(&status)).is_some_and(|(draft,actual)|actual.revision>draft.revision);
        if changed && !busy {
            if self.pixel_groups.dirty {ui.colored_label(ui.visuals().warn_fg_color,format!("{}  Layout changed on board. Reload before saving.",icons::WARNING));}
            else {read=true;}
        }
        ui.add_enabled_ui(ready&&!busy,|ui| {
            super::panel_details::action_row(ui,&[(icons::ARROW_CLOCKWISE,"Read from board"),(icons::FLOPPY_DISK,"Save to board")],|ui| {
                if dialog::action_button(ui,icons::ARROW_CLOCKWISE,if self.pixel_groups.dirty {"Discard edits / reload"} else {"Read from board"}).clicked(){read=true;}
                if ui.add_enabled_ui(self.pixel_groups.dirty&&!changed&&self.pixel_groups.value.as_ref().is_some_and(|v|v.request().is_ok()),|ui|
                    dialog::primary_action_button(ui,icons::FLOPPY_DISK,"Save to board")).inner.clicked(){save=true;}
            });
            ui.add_space(8.0);
            if let Some(value)=&mut self.pixel_groups.value { self.pixel_groups.dirty|=super::panel_details::physical_groups(ui,value,&self.mappings); }
        });
        if !ready {ui.weak("Connect using a supported USB API or Wi-Fi route to edit group sizes.");}
        if !self.pixel_groups.message.is_empty(){ui.label(&self.pixel_groups.message);}
        if read || save {
            let body=if save {self.pixel_groups.value.as_ref().and_then(|v|v.request().ok())}else{Some(serde_json::json!({}))};
            if let Some(body)=body {
                self.pixel_groups.expected=if save{self.pixel_groups.value.clone()}else{None};
                self.pixel_groups.pending=Some(Instant::now());self.pixel_groups.message=if save{"Saving physical layout; awaiting readback…"}else{"Reading physical group layout…"}.into();
                if self.transport==Transport::Serial {
                    if let Err(error)=self.begin_pixel_api(pixel_groups::Purpose::Groups,if save{"POST"}else{"GET"},pixel_groups::PATH,body) {
                        self.pixel_groups.pending=None;self.pixel_groups.expected=None;self.pixel_groups.message=error;
                    }
                } else if save {self.post(pixel_groups::PATH,vec![("configuration".into(),body.to_string())]);}
                else {let _=self.http_tx.send(HttpCommand::Configuration(self.endpoint(),pixel_groups::PATH));}
            }
        }
    }
    fn draw_pixel_corrections(&mut self, ui: &mut egui::Ui) {
        let pin = self.live_status().and_then(peer_http::Pin::from_status);
        if pin != self.pixel_corrections.owner {
            let preview_enabled = self.config.pixel_correction_live_preview;
            self.pixel_corrections = pixel_corrections::Editor {
                tab: true,
                owner: pin.clone(),
                ..Default::default()
            };
            self.pixel_corrections.live.enabled = preview_enabled;
        }
        if self.transport != Transport::WiFi || pin.is_none() {
            ui.weak("Correction configuration requires the board Wi-Fi connection.");
            return;
        }
        if self
            .pixel_corrections
            .pending
            .is_some_and(|at| at.elapsed().as_secs() >= 25)
        {
            self.pixel_corrections.pending = None;
            self.pixel_corrections.message="No correction receipt arrived. Read from board before retrying; no write replayed.".into();
        }
        self.pixel_corrections.live.painted_at = Some(Instant::now());
        let busy = self.pixel_corrections.pending.is_some() || self.pixel_corrections.live.pending.is_some();
        // Match Flutter: load once when this peer's editor opens. A failed or
        // uncertain read keeps its message and requires an explicit retry.
        let mut read = self.pixel_corrections.value.is_none()
            && !busy
            && self.pixel_corrections.message.is_empty();
        let mut save = false;
        let rule_toggle=self.live_status().and_then(|s|s.capabilities.as_ref())
            .is_some_and(|c|c.has("pixel-correction-rule-toggle"));
        let preview_supported=self.live_status().and_then(|s|s.capabilities.as_ref())
            .is_some_and(|c|c.has(pixel_corrections::PREVIEW_FEATURE));
        let cursor=self.live_status().and_then(|status| {
            let observed=pixel_diagnostic::Observation::read(status)?;
            (observed.active && !observed.off && !observed.probe.group && self.pixel_diagnostic.pending.is_none())
                .then_some(pixel_corrections::DiagnosticCursor{raw:observed.probe.index,revision:status.pixel_corrections_revision?})
        });
        if preview_supported {
            let mut enabled=self.pixel_corrections.live.enabled;
            if ui.checkbox(&mut enabled,ui::i18n::text("Live preview")).changed() {
                self.pixel_corrections.live.toggle(enabled);
                self.config.pixel_correction_live_preview=enabled;
                // Explicit preference only: do not commit open Preferences
                // drafts, a preview lease, or board configuration here.
                if let Err(error)=config::AppConfig::load().and_then(|mut saved|{
                    saved.pixel_correction_live_preview=enabled;saved.save()
                }) {self.pixel_corrections.message=format!("Preview preference not saved: {error}");}
            }
            if self.pixel_corrections.live.observed.is_some() {
                ui.weak("Temporary preview · Save makes it persistent");
            }
            if !self.pixel_corrections.live.error.is_empty() {
                ui.colored_label(ui.visuals().warn_fg_color,&self.pixel_corrections.live.error);
            }
        }
        let changed = self
            .live_status()
            .and_then(|s| s.pixel_corrections_revision)
            .zip(self.pixel_corrections.value.as_ref().map(|v| v.revision))
            .is_some_and(|(live, saved)| live > saved);
        if changed && !busy {
            if self.pixel_corrections.dirty {
                ui.colored_label(
                    ui.visuals().warn_fg_color,
                    "Corrections changed on board. Reload before saving.",
                );
            } else {
                read = true;
            }
        }
        let mut correction_actions = vec![
            (icons::ARROW_CLOCKWISE, "Read from board"),
            (icons::FLOPPY_DISK, "Save to board"),
        ];
        if self.pixel_corrections.dirty {
            correction_actions.push((icons::ARROW_COUNTER_CLOCKWISE, "Discard edits / reload"));
        }
        ui.add_enabled_ui(!busy, |ui| {
            super::panel_details::action_row(ui, &correction_actions, |ui| {
                if ui
                    .add_enabled_ui(!self.pixel_corrections.dirty, |ui| {
                        dialog::action_button(ui, icons::ARROW_CLOCKWISE, "Read from board")
                    })
                    .inner
                    .clicked()
                {
                    read = true;
                }
                if ui
                    .add_enabled_ui(
                        self.pixel_corrections.dirty
                            && self.pixel_corrections.value.is_some()
                            && !changed,
                        |ui| dialog::primary_action_button(ui, icons::FLOPPY_DISK, "Save to board"),
                    )
                    .inner
                    .clicked()
                {
                    save = true;
                }
                if self.pixel_corrections.dirty
                    && dialog::action_button(
                        ui,
                        icons::ARROW_COUNTER_CLOCKWISE,
                        "Discard edits / reload",
                    )
                    .clicked()
                {
                    read = true;
                }
            });
            ui.add_space(8.0);
            if let Some(value) = self.pixel_corrections.value.as_mut() {
                let edited=super::panel_details::correction_rules(ui, value, rule_toggle,self.pixel_corrections.live.observed.as_ref(),cursor);
                self.pixel_corrections.dirty |= edited;
                if edited && preview_supported {self.pixel_corrections.live.edited(Instant::now());}
            }
        });
        ui.label(&self.pixel_corrections.message);
        if read {
            self.pixel_corrections.live.stop();
            if self.pixel_corrections.live.save_lease().is_some() {
                self.pixel_corrections.reload_after_preview=true;
                self.pixel_corrections.message="Stopping temporary preview before reloading…".into();
                return;
            }
            self.pixel_corrections.pending = Some(std::time::Instant::now());
            self.pixel_corrections.expected = None;
            self.pixel_corrections.message = "Reading board configuration…".into();
            let _ = self.http_tx.send(HttpCommand::Configuration(
                self.endpoint(),
                pixel_corrections::PATH,
            ));
        }
        if save && let Some(value) = self.pixel_corrections.value.as_ref() {
            let mut saved=value.write_value();
            if let Some(lease)=self.pixel_corrections.live.save_lease() {saved["lease_id"]=lease.into();}
            match serde_json::to_string(&saved) {
                Ok(json) => {
                    self.pixel_corrections.expected = Some(value.clone());
                    self.pixel_corrections.pending = Some(std::time::Instant::now());
                    self.pixel_corrections.message =
                        "Saving corrections; awaiting board readback…".into();
                    self.post(
                        pixel_corrections::PATH,
                        vec![("configuration".into(), json)],
                    );
                }
                Err(error) => self.pixel_corrections.message = error.to_string(),
            }
        }
    }
    fn service_pixel_correction_preview(&mut self, foreground:bool) {
        let now=Instant::now();
        let status=self.live_status().cloned();
        let pin=status.as_ref().and_then(peer_http::Pin::from_status);
        if pin.is_some() && pin!=self.pixel_corrections.owner {
            let enabled=self.config.pixel_correction_live_preview;
            self.pixel_corrections.live=pixel_corrections::LivePreview::default();
            self.pixel_corrections.live.enabled=enabled;
            return; // Never send old-owner cleanup to a newly connected board.
        }
        let allowed=foreground && self.http_writable() && pin.is_some()
            && status.as_ref().and_then(|s|s.capabilities.as_ref()).is_some_and(|c|c.has(pixel_corrections::PREVIEW_FEATURE))
            && self.pixel_corrections.live.painted_at.is_some_and(|at|now.saturating_duration_since(at)<Duration::from_millis(750));
        let changed=status.as_ref().and_then(|s|s.pixel_corrections_revision)
            .zip(self.pixel_corrections.value.as_ref().map(|v|v.revision)).is_some_and(|(live,draft)|live!=draft);
        if !allowed || changed {self.pixel_corrections.live.stop();}
        // Unknown/disconnected owners expire in firmware; no discovery or
        // reconnection is allowed to redirect a preview mutation.
        if pin.is_none() || !self.http_writable() || self.pixel_corrections.pending.is_some() {return;}
        if self.pixel_corrections.reload_after_preview && self.pixel_corrections.live.pending.is_none()
            && self.pixel_corrections.live.save_lease().is_none() {
            self.pixel_corrections.reload_after_preview=false;
            self.pixel_corrections.pending=Some(now);
            self.pixel_corrections.expected=None;
            self.pixel_corrections.message="Reading board configuration…".into();
            if self.http_tx.send(HttpCommand::Configuration(self.endpoint(),pixel_corrections::PATH)).is_err() {
                self.pixel_corrections.pending=None;
                self.pixel_corrections.message="Configuration worker unavailable; nothing sent".into();
            }
            return;
        }
        let Some(value)=self.pixel_corrections.value.as_ref() else {return;};
        let Some(request)=self.pixel_corrections.live.next(value,now) else {return;};
        let status=status.unwrap();
        // Choose one path before dispatch. An unavailable socket may use one
        // HTTP request, but an uncertain WS send is never replayed over HTTP.
        let result=if self.websocket_live && realtime::supported(&status,pixel_corrections::PREVIEW_PATH) {
            realtime::Intent::new(self.endpoint(),&status,pixel_corrections::PREVIEW_PATH,
                vec![("configuration".into(),request.payload().to_string())],self.http_tx.capture(),
                self.connection_gate.stamp(Transport::WiFi,self.connection_gate.epoch()),None)
                .and_then(|mut intent| {
                    intent.correction_request=Some(request.clone());
                    self.ws_tx.send(WsCommand::Control(intent)).map_err(|_|"Preview worker unavailable; nothing sent".to_owned())
                })
        } else {
            self.http_tx.send(HttpCommand::CorrectionPreview(self.endpoint(),request.clone()))
                .map_err(|_|"Preview worker unavailable; nothing sent".to_owned())
        };
        if let Err(error)=result {self.pixel_corrections.live.finish(request.id,Err(error),now);}
    }
    fn draw_pixel_diagnostic(&mut self, ui: &mut egui::Ui) {
        let Some(status) = self
            .live_status()
            .cloned()
            .filter(pixel_diagnostic::supported)
        else {
            ui.weak("Connect to a board with acknowledged pixel diagnostics first.");
            return;
        };
        let Some(_peer) = peer_http::Pin::from_status(&status) else {
            return;
        };
        let busy = self.pixel_diagnostic.pending.is_some();
        let atomic = status
            .capabilities
            .as_ref()
            .is_some_and(|c| c.has("pixel-diagnostic"));
        let ready = !busy
            && atomic
            && !self.streaming
            && !self.sampling
            && !status.programming_latch
            && self.command_route() == Some(self.transport)
            && match self.transport {
                Transport::Serial => pixel_diagnostic::usb_supported(&status),
                Transport::WiFi => realtime::supported(&status, "/api/led/diagnostic"),
            };
        ui.weak("Physical wire order · all other pixels cleared");
        ui.horizontal_wrapped(|ui| {
            if self.streaming || self.sampling {
                if ui.button("Stop active streams").clicked() {
                    self.streaming = false;
                    self.sampling = false;
                    let _ = self.ws_tx.send(WsCommand::Stream(false));
                    let _ = self.ws_tx.send(WsCommand::Sample(false));
                }
            }
        });
        let mut probe = self.pixel_diagnostic.probe();
        let Some(layout) = pixel_groups::for_status(&status) else {
            return;
        };
        let pixels=layout.used_pixels;
        let Some(current)=pixel_diagnostic::Probe::for_status(&status,probe.index.min(pixels-1),probe.group,probe.rgb) else{return;};
        probe=current;
        let group_pixels=probe.group_pixels;
        self.pixel_diagnostic.layout=Some(layout.clone());
        self.pixel_diagnostic.custom_layout=pixel_groups::advertised(&status);
        let mut changed = false;
        let mut action = None;
        ui.add_enabled_ui(!busy, |ui| {
            ui.horizontal(|ui| {
                changed |= ui
                    .selectable_value(&mut probe.group, false, "1 pixel")
                    .changed();
                changed |= ui
                    .selectable_value(
                        &mut probe.group,
                        true,
                        format!("{group_pixels}-pixel group"),
                    )
                    .changed();
            });
        });
        self.pixel_diagnostic.set_probe(probe);
        ui.heading(probe.label());
        let group = layout.containing(probe.first()).unwrap();
        let (group_start,group_pixels)=layout.span(group).unwrap();
        if status
            .capabilities
            .as_ref()
            .is_some_and(|c| c.role == "first-wins")
        {
            let key = (group + 1) / 2;
            let kind = if group % 2 == 1 {
                "illumination"
            } else {
                "indicator"
            };
            ui.weak(format!("Box {key} · {kind} · physical group {group} of {}",layout.group_count));
        } else {
            let logical = self
                .mappings
                .iter()
                .find(|m| m.physical_cell == group)
                .map(|m| m.logical_id);
            ui.weak(format!(
                "Physical group {group} of {}{}",layout.group_count,
                logical
                    .map(|id| format!(" · logical cell {id}"))
                    .unwrap_or_default()
            ));
        }
        ui.horizontal_wrapped(|ui| {
            for sector in 0..u16::from(group_pixels) {
                let index = group_start + sector;
                if ui
                    .add_enabled(
                        ready,
                        egui::Button::new(format!("{}", index + 1)).selected(
                            index >= probe.first() && index < probe.first() + probe.count(),
                        ),
                    )
                    .on_hover_text(format!("Physical pixel {}", index + 1))
                    .clicked()
                {
                    action = Some((
                        pixel_diagnostic::Probe {
                            index,
                            group: false,
                            ..probe
                        },
                        false,
                    ));
                }
            }
        });
        ui.horizontal_wrapped(|ui| {
            if ui
                .add_enabled(
                    ready && probe.moved_on(-1,&layout).is_some(),
                    egui::Button::new(if probe.group {"← Previous group".into()} else {"← Previous pixel".to_string()}),
                )
                .clicked()
            {
                action = probe.moved_on(-1,&layout).map(|p| (p, false));
            }
            ui.label(format!("Physical pixel · 1–{pixels}"));
            ui.add_enabled(
                !busy,
                egui::DragValue::new(&mut self.pixel_diagnostic.index).range(1..=pixels),
            );
            if ui.add_enabled(ready, egui::Button::new("Show")).clicked() {
                action = Some((self.pixel_diagnostic.probe(), false));
            }
            if ui
                .add_enabled(
                    ready && probe.moved_on(1,&layout).is_some(),
                    egui::Button::new(if probe.group {"Next group →"} else {"Next pixel →"}),
                )
                .clicked()
            {
                action = probe.moved_on(1,&layout).map(|p| (p, false));
            }
        });
        ui.add_enabled_ui(!busy, |ui| {
            ui.horizontal_wrapped(|ui| {
                for (rgb, label) in [
                    (0xffffff, "White"),
                    (0xff0000, "Red"),
                    (0x00ff00, "Green"),
                    (0x0000ff, "Blue"),
                ] {
                    changed |= ui.selectable_value(&mut probe.rgb, rgb, label).changed();
                }
                let mut color = [
                    (probe.rgb >> 16) as u8,
                    (probe.rgb >> 8) as u8,
                    probe.rgb as u8,
                ];
                if ui.color_edit_button_srgb(&mut color).changed() {
                    probe.rgb =
                        u32::from(color[0]) << 16 | u32::from(color[1]) << 8 | u32::from(color[2]);
                    changed = true;
                }
                ui.monospace(format!("#{:06X}", probe.rgb));
            });
        });
        if changed {
            self.pixel_diagnostic.set_probe(probe);
            if ready
                && self
                    .pixel_diagnostic
                    .observed
                    .as_ref()
                    .is_some_and(|o| o.active && !o.off)
            {
                action = Some((probe, false));
            }
        }
        if ui
            .add_enabled(
                ready,
                egui::Button::new(format!("All off · clear {pixels} pixels")),
            )
            .clicked()
        {
            action = Some((self.pixel_diagnostic.probe(), true));
        }
        ui.add(
            egui::ProgressBar::new(if busy { 0.5 } else { 0.0 })
                .desired_height(4.0)
                .animate(busy),
        );
        if let Some(observed) = &self.pixel_diagnostic.observed {
            ui.label(observed.label());
        }
        if !atomic {
            ui.weak("This board has not acknowledged atomic pixel diagnostics. Update its firmware before using these controls.");
        }
        ui.weak("Show clears every other pixel. Colour and group changes apply after Show. All off keeps your cursor.");
        if let Some((probe, off)) = action {
            self.send_pixel_probe(probe, off);
        }
        if !self.pixel_diagnostic.message.is_empty() {
            ui.label(&self.pixel_diagnostic.message);
        }
    }
    fn send_pixel_probe(&mut self, probe: pixel_diagnostic::Probe, off: bool) {
        let result = (|| -> Result<(), String> {
            if !probe.valid()
                || self.command_route() != Some(self.transport)
                || self.streaming
                || self.sampling
                || self.pixel_diagnostic.pending.is_some()
            {
                return Err("Diagnostic owner unavailable; nothing queued".into());
            }
            let status = self.live_status().ok_or("No fresh board status")?;
            if pixel_diagnostic::Probe::from_form(status,&probe.form(off)).is_none() {
                return Err("Physical group layout changed; nothing sent. Refresh this board's diagnostics.".into());
            }
            if self.transport == Transport::Serial {
                if !pixel_diagnostic::usb_supported(status) {
                    return Err("This board has not acknowledged USB pixel diagnostics; no network fallback".into());
                }
                if probe.group_layout_revision.is_some() {
                    let previous=status.clone();
                    let (request_id,body)=pixel_diagnostic::api_command(&previous,probe,off)?;
                    let id=self.pixel_diagnostic.begin();
                    if let Err(error)=self.begin_pixel_api(pixel_groups::Purpose::Diagnostic{id,probe,off,request_id,previous},"POST","/api/command",body) {
                        self.pixel_diagnostic.finish(id,error);
                    } else {self.pixel_diagnostic.message="Sending one layout-fenced USB diagnostic…".into();}
                    return Ok(());
                }
                let admission = self.serial_admission().ok_or("USB owner unavailable")?;
                let payload = probe.usb(off).ok_or("Invalid pixel group")?;
                let (reply, receiver) = std::sync::mpsc::channel();
                let cancel = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
                let id = self.pixel_diagnostic.begin();
                if self
                    .serial_tx
                    .send(crate::SerialCommand::Bridge(
                        self.serial_port.clone(),
                        admission,
                        0x65,
                        payload,
                        crate::bridge::Reply::new(reply, cancel.clone()),
                    ))
                    .is_err()
                {
                    self.pixel_diagnostic
                        .finish(id, "USB worker unavailable; nothing sent".into());
                } else {
                    self.pixel_usb_reply = Some((id, receiver, cancel));
                    self.pixel_diagnostic.message =
                        "Sending one USB diagnostic and verifying published RGB…".into();
                }
                return Ok(());
            }
            let mut request = realtime::Intent::new(
                self.endpoint(),
                status,
                "/api/led/diagnostic",
                probe.form(off),
                self.http_tx.capture(),
                self.connection_gate
                    .stamp(Transport::WiFi, self.connection_gate.epoch()),
                None,
            )?;
            let id = self.pixel_diagnostic.begin();
            request.pixel_request = Some(id);
            if self.ws_tx.send(WsCommand::Control(request)).is_err() {
                self.pixel_diagnostic
                    .finish(id, "Diagnostic worker unavailable; nothing sent".into());
            } else {
                self.pixel_diagnostic.message =
                    "Sending one board-owned diagnostic command…".into();
            }
            Ok(())
        })();
        if let Err(error) = result {
            self.pixel_diagnostic.message = error;
        }
    }
    fn arm_pixel_diagnostic(&mut self) {
        let status = self.live_status().cloned();
        if self.command_route() != Some(Transport::WiFi)
            || !self.websocket_live
            || self.streaming
            || self.sampling
            || self.pixel_diagnostic.pending.is_some()
            || !status.as_ref().is_some_and(pixel_diagnostic::can_arm)
        {
            self.pixel_diagnostic.message = "Use the enabled Wi-Fi Cell board, stop global effects/streams, then confirm diagnostics.".into();
            return;
        }
        let peer = peer_http::Pin::from_status(status.as_ref().unwrap()).unwrap();
        self.pixel_diagnostic.armed = None;
        let id = self.pixel_diagnostic.begin();
        self.pixel_diagnostic.message =
            "Confirming effects mode via WebSocket ACK and same-board readback…".into();
        let request = pixel_diagnostic::Request {
            id,
            index: 0,
            peer,
            endpoint: self.endpoint(),
            at: Instant::now(),
        };
        if self.ws_tx.send(WsCommand::ArmPixel(request)).is_err() {
            self.pixel_diagnostic
                .finish(id, "WebSocket worker unavailable; no frame sent".into());
        }
    }
    fn show_pixel_diagnostic(&mut self, index: u16) {
        if self.live_status().is_some_and(|s| {
            s.capabilities
                .as_ref()
                .is_some_and(|c| c.has("pixel-diagnostic"))
        }) {
            let current = self.pixel_diagnostic.probe();
            if let Some(probe) = index.checked_sub(1).and_then(|raw_index|
                pixel_diagnostic::Probe::for_status(self.live_status().unwrap(), raw_index,
                    current.group, current.rgb)) {
                self.send_pixel_probe(probe, false);
            } else {
                self.pixel_diagnostic.message = "Pixel outside the current physical group layout; nothing sent".into();
            }
            return;
        }
        let status = self.live_status().cloned();
        let peer = status.as_ref().and_then(peer_http::Pin::from_status);
        if !(1..=480).contains(&index)
            || self.command_route() != Some(Transport::WiFi)
            || !self.websocket_live
            || self.streaming
            || self.sampling
            || self.pixel_diagnostic.pending.is_some()
            || peer.is_none()
            || self.pixel_diagnostic.armed != peer
            || !status.as_ref().is_some_and(pixel_diagnostic::ready)
        {
            self.pixel_diagnostic.message = "Confirm individual-pixel effects mode on the current Wi-Fi board before showing a pixel.".into();
            return;
        }
        self.pixel_diagnostic.index = index;
        let id = self.pixel_diagnostic.begin();
        self.pixel_diagnostic.message = format!("Sending physical wire pixel {index} once…");
        let request = pixel_diagnostic::Request {
            id,
            index: index - 1,
            peer: peer.unwrap(),
            endpoint: self.endpoint(),
            at: Instant::now(),
        };
        if self.ws_tx.send(WsCommand::Pixel(request)).is_err() {
            self.pixel_diagnostic
                .finish(id, "WebSocket worker unavailable; frame not sent".into());
        }
    }
    fn draw_effects(&mut self, ui: &mut egui::Ui) {
        let snapshot = self.live_status().cloned();
        let modes = super::effect_controls::visible_modes(
            self.presentation_context()
                .map(|c| c.modes().cloned().collect::<Vec<_>>())
                .unwrap_or_default(),
        );
        let live_modes = snapshot
            .as_ref()
            .and_then(|s| s.capabilities.as_ref())
            .map(|c| c.modes().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        if let Some(mode) = super::effect_controls::mode_picker(
            ui,
            &modes,
            snapshot.as_ref().map(|s| s.mode.as_str()),
            &live_modes,
        ) {
            self.action(
                "/api/mode",
                vec![("value".into(), mode.id)],
                OP_SET_MODE,
                vec![mode.code],
            );
        }
        if modes.iter().any(|mode| mode.id == "first-wins") {
          if !snapshot.as_ref().is_some_and(|s| crate::minigame::snapshot(s).is_some()) {
            ui.horizontal_wrapped(|ui| {
                ui.add_enabled_ui(
                    snapshot
                        .as_ref()
                        .is_some_and(super::effect_controls::reset_available),
                    |ui| {
                        if dialog::primary_action_button(
                            ui,
                            icons::ARROW_COUNTER_CLOCKWISE,
                            "Reset to white",
                        )
                        .clicked()
                        {
                            self.action("/api/game/reset", vec![], OP_RESET, vec![]);
                        }
                    },
                );
            });
            ui.add_space(10.0);
          }
        }
        // Table owns semantic selection/actions and cell identity. Do not keep
        // a second hardcoded White/Red card that contradicts its saved palette.
        self.draw_led_settings(ui);
    }
    pub(super) fn draw_led_settings(&mut self, ui: &mut egui::Ui) {
        let snapshot = self.live_status().cloned();
        if self.presentation_has_feature("leds") {
            ui.add_enabled_ui(self.has_feature("leds"), |ui| {
        dialog::section(ui, icons::LIGHTBULB, "LED output", |ui| {
            if let Some(status) = snapshot.as_ref() {
                if let Some(backend) = super::effect_controls::backend_picker(ui, status) {
                    if let Some((form, payload)) = led_output_request(backend) {
                        self.action("/api/led/output", form, OP_LED_OUTPUT, payload);
                    }
                }
            }
            if let Some(mut enabled) = snapshot.as_ref()
                .and_then(|s| s.led_output.get("enabled")).and_then(|v| v.as_bool()) {
                ui.horizontal_wrapped(|ui| {
                    if ui.checkbox(&mut enabled, "LED output service").changed() {
                        self.action("/api/led/output", vec![("enabled".into(), enabled.to_string())],
                            OP_LED_OUTPUT, vec![u8::from(enabled)]);
                    }
                    if super::effect_controls::choice(ui, icons::POWER, "Off", !enabled, true).on_hover_text("Send black again, even when output is already off.").clicked() {
                        self.action("/api/led/output", vec![("enabled".into(), "false".into())], OP_LED_OUTPUT, vec![0]);
                    }
                });
            } else {
                ui.label(egui::RichText::new("LED output service: Unknown").weak());
            }
            if let Some(observed) = snapshot.as_ref().and_then(led_output_mode) {
                ui.horizontal_wrapped(|ui| {
                    for (mode, icon, label) in [("static", icons::LAMP, "Static grouped"), ("effects", icons::SPARKLE, "Animated effects")] {
                        if super::effect_controls::choice(ui, icon, label, mode == observed, true).clicked() && mode != observed {
                            if let Some((form, payload)) = led_output_request(mode) {
                                self.action("/api/led/output", form, OP_LED_OUTPUT, payload);
                            }
                        }
                    }
                });
            } else { ui.weak("Output mode: Unknown"); }
            ui.add_space(6.0);
            ui.add_enabled_ui(snapshot.is_some(), |ui| {
                dialog::setting_row(ui, icons::SUN, "Brightness", None, |ui| {
                    let owner = self.command_route().zip(snapshot.as_ref().and_then(peer_http::Pin::from_status))
                        .map(|(route, peer)| super::effect_controls::BrightnessOwner {
                            route, peer, epoch: self.connection_gate.epoch(),
                        });
                    if let Some(value) = super::effect_controls::brightness(ui, owner, snapshot.as_ref().map(|s|s.brightness)).inner {
                        self.action("/api/brightness", vec![("value".into(), value.to_string())], OP_BRIGHTNESS, vec![value]);
                    }
                });
            });
        });
        ui.add_space(10.0);
        dialog::section(ui, icons::SPARKLE, "Strip effect", |ui| {
            let observed_effect=snapshot.as_ref().map(|s|s.effect.as_str()).filter(|effect|!effect.is_empty());
            ui.horizontal_wrapped(|ui| {
                ui.weak(crate::ui::i18n::text("Current effect"));
                ui.strong(observed_effect.map(|effect| match effect { "none" => "State colors", "rainbow" => "Rainbow", "fire" => "Fire", other => other }).unwrap_or("Unknown"));
            });
            let test_ready = snapshot.as_ref().is_some_and(|s|
                s.led_output.get("enabled").and_then(|v| v.as_bool()) == Some(true));
            if super::effect_controls::choice(ui, icons::SPARKLE, "Rainbow test", observed_effect == Some("rainbow"), test_ready)
                .on_hover_text("Rainbow animates in both modes, keeping each board-defined LED group together in static mode. Does not change output mode or enable the service.")
                .clicked() {
                self.action("/api/effect", vec![("value".into(), "rainbow".into())], OP_PATTERN, vec![1]);
            }
            ui.add_enabled_ui(snapshot.is_some(), |ui| {
                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    super::dropdown::ComboBox::from_id_salt("strip-effect")
                        .width(ui.available_width().clamp(80.0, 220.0))
                        .selected_text(match self.pattern.as_str() { "none" => "State colors", "rainbow" => "Rainbow", "fire" => "Fire", _ => "Unknown" })
                        .show_ui(ui, |ui| {
                            for (name, label) in [("none", "State colors"), ("rainbow", "Rainbow"), ("fire", "Fire")] {
                                ui.dropdown_value(&mut self.pattern, name.into(), label);
                            }
                        });
                if dialog::primary_action_button(ui, icons::PLAY, "Apply").clicked() {
                    let value = match self.pattern.as_str() {
                        "rainbow" => 1,
                        "fire" => 2,
                        _ => 0,
                    };
                    self.action(
                        "/api/effect",
                        vec![("value".into(), self.pattern.clone())],
                        OP_PATTERN,
                        vec![value],
                    );
                }
                });
            });
        });
        });
        }
        if let Some(s) = snapshot.as_ref().filter(|s| {
            s.capabilities
                .as_ref()
                .is_some_and(|c| c.has("state-presentation") && c.has("leds"))
        }) {
            // These two endpoints currently use the selected HTTP/WS owner;
            // USB game settings do not imply USB state-appearance support.
            ui.add_enabled_ui(self.http_writable(), |ui| {
            ui.add_space(10.0);
            dialog::section(ui, icons::PALETTE, "State colors and effects", |ui| {
                if let Some(patch) = crate::state_appearance::controls(
                    ui,
                    &s.state_presentation,
                    s.capabilities.as_ref().is_some_and(|c| c.table_selection()),
                ) {
                    self.post(
                        "/api/led/presentation",
                        vec![("settings".into(), patch.to_string())],
                    );
                }
            });
            ui.add_space(10.0);
            dialog::section(ui, icons::EYE, "Temporary color test", |ui| {
                let table = s.capabilities.as_ref().is_some_and(|c| c.table_selection());
                let ids = if table && !self.table.selection.ids.is_empty() {
                    self.table
                        .selection
                        .ids
                        .iter()
                        .map(u8::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                } else if table {
                    "1-80".into()
                } else {
                    "1-18".into()
                };
                if let Some(form) = crate::state_appearance::preview_controls(ui, &ids) {
                    self.post("/api/led/preview", form);
                }
            });
            }).response.on_disabled_hover_text("Unavailable on selected connection");
        }
    }
    pub(super) fn draw_front_panel(&mut self, ui: &mut egui::Ui) {
        let snapshot = self.live_status().cloned();
        let requested = snapshot
            .as_ref()
            .and_then(peer_http::Pin::from_status)
            .and_then(|peer| self.panel_events.current(&peer))
            .cloned();
        ui.add_enabled_ui(self.panel_live(Panel::FrontPanel), |ui| {
            if self.presentation_has_feature("lcd") {
                if let Some(lcd) = snapshot.as_ref().and_then(|s| s.lcd.as_ref()) {
                    if let Some(lines) = lcd.confirmed_lines() {
                        dialog::section(ui, icons::TEXT_ALIGN_LEFT, "LCD display", |ui| {
                            ui.label(
                                egui::RichText::new(format!(
                                    "I²C 0x{:02X} · 16×2",
                                    lcd.address.unwrap()
                                ))
                                .weak(),
                            );
                            egui::Frame::group(ui.style()).show(ui, |ui| {
                                for line in lines {
                                    ui.monospace(line);
                                }
                            });
                            if lcd.pending {
                                ui.spinner();
                            }
                        });
                    }
                }
            }
            if self.presentation_has_feature("display") {
                dialog::section(ui, icons::GAUGE, "Live display", |ui| {
                    if let Some(s) = &snapshot {
                        if s.tm1637_segments.len() == 4 {
                            ui::seven_segment::draw(
                                ui,
                                &s.tm1637_segments,
                                s.tm1637_brightness,
                                s.tm1637_ack,
                            );
                        } else {
                            ui.label(crate::ui::i18n::text("Segment readback unavailable"));
                        }
                        badge(
                            ui,
                            if s.tm1637_ack {
                                "TM1637 acknowledged"
                            } else {
                                "No display acknowledgment"
                            },
                            s.tm1637_ack,
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(crate::ui::i18n::text("Display state unavailable"))
                                .weak(),
                        );
                    }
                    // Keep the two compact rows stable when a short notice expires;
                    // requested text never replaces the actual acknowledged segments.
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), 44.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.weak("Requested");
                            let value = requested
                                .as_ref()
                                .map_or("—", |event| event.presentation.text.as_str());
                            let response = ui.add(
                                egui::Label::new(egui::RichText::new(value).monospace()).truncate(),
                            );
                            if let Some(event) = &requested {
                                response.on_hover_text(format!(
                                    "{}\n{} · {} · {} · priority {} · value {} · uptime {} ms",
                                    event.text,
                                    event.kind,
                                    event.presentation.blink,
                                    event.presentation.marquee,
                                    event.presentation.priority,
                                    event.value,
                                    event.uptime_ms
                                ));
                            }
                        },
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.label(crate::ui::i18n::text("Number"));
                        ui.add(egui::DragValue::new(&mut self.display_value).range(-999..=9999));
                        if dialog::primary_action_button(ui, icons::GAUGE, "Show number").clicked()
                        {
                            self.action(
                                "/api/display",
                                vec![("value".into(), self.display_value.to_string())],
                                OP_DISPLAY,
                                self.display_value.to_le_bytes().to_vec(),
                            );
                        }
                    });
                });
            }
            ui.add_space(10.0);
            if self.presentation_has_feature("text") {
                dialog::section(ui, icons::TEXT_ALIGN_LEFT, "Text", |ui| {
                    ui.add_sized(
                        [ui.available_width(), 28.0],
                        dialog::singleline_text_edit(ui, &mut self.display_text)
                            .char_limit(40)
                            .hint_text(crate::ui::i18n::text("Front-panel text")),
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.label(crate::ui::i18n::text("Step"));
                        ui.add(
                            egui::DragValue::new(&mut self.config.display.speed_ms)
                                .range(80..=5000)
                                .suffix(" ms"),
                        );
                        ui.checkbox(
                            &mut self.config.display.force_scroll,
                            crate::ui::i18n::text("Scroll short text"),
                        );
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.label(crate::ui::i18n::text("Repeat"));
                        super::dropdown::ComboBox::from_id_salt("display-repeat")
                            .selected_text(self.config.display.repeat.label())
                            .show_ui(ui, |ui| {
                                for value in [
                                    crate::display::Repeat::Once,
                                    crate::display::Repeat::Loop,
                                    crate::display::Repeat::Interval,
                                ] {
                                    ui.dropdown_value(
                                        &mut self.config.display.repeat,
                                        value,
                                        value.label(),
                                    );
                                }
                            });
                        if self.config.display.repeat == crate::display::Repeat::Interval {
                            ui.label(crate::ui::i18n::text("Pause"));
                            ui.add(
                                egui::DragValue::new(&mut self.config.display.interval_seconds)
                                    .range(1..=255)
                                    .suffix(" s"),
                            );
                        }
                        if self.display_text.len() <= 4
                            && !self.config.display.force_scroll
                            && self.config.display.repeat != crate::display::Repeat::Loop
                        {
                            ui.label(crate::ui::i18n::text("Hold"));
                            ui.add(
                                egui::DragValue::new(&mut self.config.display.hold_ms)
                                    .range(0..=60000)
                                    .suffix(" ms"),
                            );
                        }
                    });
                    let payload = self.config.display.payload(&self.display_text);
                    if let Err(error) = &payload {
                        ui.colored_label(ui.visuals().error_fg_color, error);
                    }
                    super::panel_details::action_row(
                        ui,
                        &[(icons::PLAY, "Present"), (icons::STOP_CIRCLE, "Stop text")],
                        |ui| {
                            if ui
                                .add_enabled_ui(
                                    snapshot.is_some()
                                        && payload.is_ok()
                                        && !self.display_text.is_empty(),
                                    |ui| dialog::primary_action_button(ui, icons::PLAY, "Present"),
                                )
                                .inner
                                .clicked()
                            {
                                self.action(
                                    "/api/display",
                                    self.config.display.form(&self.display_text),
                                    OP_DISPLAY,
                                    payload.clone().unwrap(),
                                );
                                self.save_workspace();
                            }
                            if ui
                                .add_enabled_ui(
                                    snapshot.as_ref().is_some_and(|s| s.tm1637_host_active),
                                    |ui| dialog::action_button(ui, icons::STOP_CIRCLE, "Stop text"),
                                )
                                .inner
                                .clicked()
                            {
                                self.action(
                                    "/api/display",
                                    vec![("release".into(), "true".into())],
                                    OP_DISPLAY,
                                    crate::display::RELEASE.to_vec(),
                                );
                            }
                        },
                    );
                });
            }
        });
    }
    pub(super) fn draw_melodies(&mut self, ui: &mut egui::Ui) {
        let snapshot = self.live_status().cloned();
        ui.add_enabled_ui(self.panel_live(Panel::Melodies), |ui| {
            if self.presentation_has_feature("buzzer") {
                dialog::section(ui, icons::MUSIC_NOTE, "Melodies", |ui| {
                    ui.add_enabled_ui(
                        snapshot
                            .as_ref()
                            .is_some_and(|s| s.avr_online && !s.audio_cues.is_empty()),
                        |ui| {
                            // Scoped choice buttons cannot wrap after they report their
                            // minimum width. A measured row keeps every cue reachable.
                            let cues: Vec<_> = audio::CUES.into_iter().filter(|(cue, _)| snapshot.as_ref().is_some_and(|s| s.audio_cues.iter().any(|v| v == cue))).collect();
                            let labels: Vec<_> = cues.iter().map(|(_, name)| (icons::PLAY, *name)).collect();
                            super::panel_details::action_row(ui, &labels, |ui| {
                                for (cue, name) in audio::CUES {
                                    if !snapshot
                                        .as_ref()
                                        .is_some_and(|s| s.audio_cues.iter().any(|v| v == cue))
                                    {
                                        continue;
                                    }
                                    if ui::effect_controls::choice(
                                        ui,
                                        icons::PLAY,
                                        name,
                                        false,
                                        true,
                                    )
                                    .clicked()
                                    {
                                        self.action(
                                            "/api/audio",
                                            vec![("cue".into(), cue.into())],
                                            OP_AUDIO,
                                            vec![audio::id(cue).expect("catalog ID")],
                                        );
                                    }
                                }
                            });
                        },
                    );
                    if icons::disclosure_header(ui, "custom-tone", "Custom tone", false) {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(crate::ui::i18n::text("Frequency"));
                            ui.add(
                                egui::DragValue::new(&mut self.buzzer_frequency)
                                    .range(20..=12000)
                                    .suffix(" Hz"),
                            );
                            ui.label(crate::ui::i18n::text("Duration"));
                            ui.add(
                                egui::DragValue::new(&mut self.buzzer_duration)
                                    .range(1..=5000)
                                    .suffix(" ms"),
                            );
                        });
                        if ui
                            .add_enabled_ui(snapshot.is_some(), |ui| {
                                dialog::primary_action_button(ui, icons::PLAY, "Play tone")
                            })
                            .inner
                            .clicked()
                        {
                            let mut p = self.buzzer_frequency.to_le_bytes().to_vec();
                            p.extend(self.buzzer_duration.to_le_bytes());
                            self.action(
                                "/api/buzzer",
                                vec![
                                    ("frequency".into(), self.buzzer_frequency.to_string()),
                                    ("duration_ms".into(), self.buzzer_duration.to_string()),
                                ],
                                OP_BUZZER,
                                p,
                            );
                        }
                    }
                });
            }
        });
    }
    pub(super) fn draw_game_settings(&mut self, ui: &mut egui::Ui) {
        let Some(s) = self.live_status().cloned() else { return; };
        let mask = s.key_game["enabled_mask"].as_u64().unwrap_or(511) as u16;
        let minigame = crate::minigame::snapshot(&s).is_some();
        let minigame_writable = self.minigame_writable(&s);
        for request in crate::minigame::settings_controls(ui, &s, minigame_writable) {
            if let Err(error) = self.send_minigame(request) { self.message = error; }
        }
                if self.has_feature("key-settings") {
                  ui.add_enabled_ui(!minigame || minigame_writable, |ui| {
                    if self.has_feature("game-presentation")
                        && s.key_game["presentation"].is_object()
                    {
                        if let Some(patch) = crate::game_presentation::controls(
                            ui,
                            &s.key_game["presentation"],
                            &format!("{}:{}", s.board_id, s.boot_id),
                        ) {
                            self.post(
                                "/api/game/settings",
                                vec![("presentation".into(), patch.to_string())],
                            );
                        }
                    }
                    dialog::section(ui, icons::GEAR, "Boxes & presentation", |ui| {
                        if self.has_feature("game-melodies") {
                            for (field, label) in [("winner_cue", "Winner melody"), ("reset_cue", "Reset melody")] {
                                if let Some(current) = s.key_game[field].as_str() {
                                    let mut selected = current.to_owned();
                                    ui.horizontal(|ui| {
                                        ui.label(label);
                                        super::dropdown::ComboBox::from_id_salt(field).selected_text(&selected).show_ui(ui, |ui| {
                                            ui.dropdown_value(&mut selected, "silent".into(), "Silent");
                                            for (cue, title) in audio::CUES {
                                                if s.audio_cues.iter().any(|available| available == cue) {
                                                    ui.dropdown_value(&mut selected, cue.into(), title);
                                                }
                                            }
                                        });
                                        if ui.add_enabled(selected != "silent", egui::Button::new(icons::PLAY))
                                            .on_hover_text("Preview on board").clicked() {
                                            if let Some(id) = audio::id(&selected) {
                                                self.action("/api/audio", vec![("cue".into(), selected.clone())], OP_AUDIO, vec![id]);
                                            }
                                        }
                                    });
                                    if selected != current {
                                        self.post("/api/game/settings", vec![(field.into(), selected)]);
                                    }
                                }
                            }
                        }
                        if let Some(milliseconds)=s.key_game["auto_reset_ms"].as_u64() {
                            let mut enabled=milliseconds>0;
                            if ui.checkbox(&mut enabled,"Automatic round reset").changed() {
                                self.post("/api/game/settings",vec![("auto_reset_ms".into(),if enabled{"10000"}else{"0"}.into())]);
                            }
                            if enabled {
                                let mut seconds=milliseconds as f64/1000.0;
                                let response=ui.add(egui::DragValue::new(&mut seconds).range(0.001..=86400.0).suffix(" s").speed(1.0));
                                if response.drag_stopped() || (response.lost_focus() && response.changed()) {
                                    self.post("/api/game/settings",vec![("auto_reset_ms".into(),((seconds*1000.0).round() as u64).to_string())]);
                                }
                            }
                        }
                        if let Some(mut enabled)=s.key_game["extended_effects"].as_bool() {
                            if ui.checkbox(&mut enabled,"Extended effects").changed() {
                                self.post("/api/game/settings",vec![("extended_effects".into(),enabled.to_string())]);
                            }
                        }
                        ui.add_enabled_ui(self.transport==crate::Transport::WiFi,|ui|{
                            if let Some(period)=s.key_game["chase_period_ms"].as_u64(){
                                let mut period=period as u32;
                                let response=ui.add(egui::Slider::new(&mut period,400..=3000)
                                    .text("Chase lap (ms)"));
                                if response.drag_stopped() || (response.lost_focus()&&response.changed()) {
                                    self.post("/api/game/settings",vec![("chase_period_ms".into(),period.to_string())]);
                                }
                            }
                            if let Some(mut neighbors)=s.key_game["chase_neighbors"].as_bool(){
                                if ui.checkbox(&mut neighbors,"Affect adjacent pixels").changed(){
                                    self.post("/api/game/settings",vec![("chase_neighbors".into(),neighbors.to_string())]);
                                }
                            }
                            if let Some(mut invert)=s.key_game["chase_invert"].as_bool(){
                                if ui.checkbox(&mut invert,"Bright dot (instead of dim)").changed(){
                                    self.post("/api/game/settings",vec![("chase_invert".into(),invert.to_string())]);
                                }
                            }
                        });
                        ui.separator();
                        ui.label(egui::RichText::new("Active boxes").strong());
                        ui.label(egui::RichText::new("Choose a layout, then adjust individual boxes. Disabled boxes stay dark on the strip.").weak());
                        let mut next=mask;
                        ui.horizontal_wrapped(|ui| {
                            for (name,value) in [("2 boxes",3),("4 boxes",15),("9 boxes",511)] {
                                if ui.add(egui::Button::new(name).selected(mask==value)
                                    .min_size(egui::vec2(94.0,34.0))).clicked(){next=value;}
                            }
                        });
                        ui.add_space(6.0);
                        ui.horizontal_wrapped(|ui|{
                            for key in 1..=9 {
                                egui::Frame::group(ui.style())
                                    .inner_margin(egui::Margin::symmetric(10,7))
                                    .corner_radius(8.0)
                                    .show(ui, |ui| {
                                        ui.set_min_width(98.0);
                                        let mut enabled=next&(1<<(key-1))!=0;
                                        if ui.checkbox(&mut enabled,format!("Box {key}")).changed(){
                                            if enabled{next|=1<<(key-1)}else{next&=!(1<<(key-1))}
                                        }
                                        ui.label(egui::RichText::new("2 × 12 pixels").small().weak());
                                    });
                            }
                        });
                        if next!=mask && next!=0 {
                            self.post("/api/game/settings",vec![("enabled_mask".into(),next.to_string())]);
                        }
                        ui.add_space(8.0);
                        ui.separator();
                        ui.label(egui::RichText::new("Round palette").strong());
                        for (name,label,fallback) in [("idle_rgb","Idle",0xffffffu32),("win_rgb","Winner",0x00ff00),("lose_rgb","Other boxes",0xff0000)] {
                            let rgb=s.key_game[name].as_u64().unwrap_or(fallback as u64) as u32;
                            let id=egui::Id::new(("key-game-color",&s.board_id,name,rgb));
                            let mut color=ui.data_mut(|data|data.get_temp::<[u8;3]>(id))
                                .unwrap_or([(rgb>>16) as u8,(rgb>>8) as u8,rgb as u8]);
                            ui.horizontal(|ui|{
                                super::panel_details::field_caption(ui,[108.0,24.0],label);
                                if ui.color_edit_button_srgb(&mut color).changed() {
                                    ui.data_mut(|data|data.insert_temp(id,color));
                                }
                                if ui.small_button("Save").clicked() {
                                    let value=(u32::from(color[0])<<16)|(u32::from(color[1])<<8)|u32::from(color[2]);
                                    self.post("/api/game/settings",vec![(name.into(),value.to_string())]);
                                }
                            });
                        }
                    });
                  });
                }

    }
    fn minigame_writable(&self, s: &DeviceStatus) -> bool {
        crate::realtime::supported(s, crate::minigame::CONTROL) && match self.command_route() {
            Some(Transport::WiFi) => self.http_writable(),
            Some(Transport::Serial) => self.pixel_api_reply.is_none() && self.has_feature("usb-api"),
            None => false,
        }
    }
    pub(super) fn draw_game(&mut self, ui: &mut egui::Ui) {
        let Some(s) = self.live_status().cloned() else { return; };
        for request in crate::minigame::controls(ui, &s, self.minigame_writable(&s)) {
            if let Err(error) = self.send_minigame(request) { self.message = error; }
        }
        ui.horizontal_wrapped(|ui| {
            if dialog::action_button(ui, icons::GEAR, "Game settings").clicked() { self.reveal_panels.push(Panel::GameSettings); }
            if dialog::action_button(ui, icons::HAND_TAP, "Keys").clicked() { self.reveal_panels.push(Panel::Keys); }
        });
    }
    fn send_minigame(&mut self, request: crate::minigame::Request) -> Result<(),String> {
        match self.command_route() {
            Some(Transport::WiFi)=>self.queue_realtime(request.path,request.form,None),
            Some(Transport::Serial)=> {
                let status=self.live_status().ok_or("Fresh USB game status unavailable")?;
                let stamp=self.serial_admission().ok_or("USB game owner unavailable")?.stamp;
                let (request,packet)=crate::minigame::UsbRequest::prepare(status,request,stamp)?;
                self.begin_pixel_api(pixel_groups::Purpose::Minigame(request),"POST","/api/command",packet)
            },
            None=>Err("No selected live game transport".into()),
        }
    }
    pub(super) fn draw_hardware(&mut self, ui: &mut egui::Ui, game_only: bool) {
        let Some(s) = self.live_status().cloned() else {
            ui.label(crate::ui::i18n::text("Hardware state unavailable"));
            return;
        };
        let avr_acknowledged = ["display", "text", "buzzer", "inputs", "outputs", "settings"]
            .iter()
            .any(|feature| self.has_feature(feature));
        if game_only
            && s.capabilities
                .as_ref()
                .is_some_and(|c| c.role == "first-wins")
        {
            let mask = s.key_game["enabled_mask"].as_u64().unwrap_or(511) as u16;
            let minigame = crate::minigame::snapshot(&s).is_some();
            let minigame_writable = crate::realtime::supported(&s,crate::minigame::CONTROL)
                && match self.command_route() {
                    Some(Transport::WiFi)=>self.http_writable(),
                    Some(Transport::Serial)=>self.pixel_api_reply.is_none() && self.has_feature("usb-api"),
                    None=>false,
                };
            dialog::section(ui, icons::HAND_TAP, "Keys", |ui| {
              if !minigame {
                ui.horizontal_wrapped(|ui| {
                    ui.strong(if s.winner == 0 {
                        "Ready for first press".to_owned()
                    } else {
                        format!("Winner · Box {}", s.winner)
                    });
                    ui.add_enabled_ui(s.avr_online && !s.programming_latch, |ui| {
                        if dialog::primary_action_button(
                            ui,
                            icons::ARROW_COUNTER_CLOCKWISE,
                            "Reset to white",
                        )
                        .clicked()
                        {
                            // A round control, not a destructive settings operation: send
                            // immediately through the selected ACK-tracked WS/USB owner.
                            self.action("/api/game/reset", vec![], OP_RESET, vec![]);
                        }
                    });
                });
              }
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.key_trigger_on_click, "Trigger on click")
                        .on_hover_text("Off: clicks only select. Ctrl-click toggles; Shift-click selects a range. Right-click for explicit actions.").changed() {
                        self.save_workspace();
                    }
                    ui.weak(format!("{} selected", self.key_selection.ids.len()));
                    if dialog::action_button(ui, icons::PLAY, "Game").clicked() { self.reveal_panels.push(Panel::Game); }
                    if dialog::action_button(ui, icons::GEAR, "Settings").clicked() { self.reveal_panels.push(Panel::GameSettings); }
                    if super::key_boxes::arrange_control(ui, self.key_layout_edit).clicked()
                    {
                        self.key_layout_edit = !self.key_layout_edit;
                    }
                    if self.key_layout_edit
                        && ui
                            .button(format!(
                                "{}  Reset positions",
                                icons::ARROW_COUNTER_CLOCKWISE
                            ))
                            .clicked()
                    {
                        self.key_positions = crate::native_workspace::default_key_positions();
                        self.save_workspace();
                    }
                });
                if minigame && self.key_trigger_on_click {
                    crate::minigame::input_controls(ui, &s);
                }
                let boxes = std::array::from_fn(|index| {
                    let key = (index + 1) as u8;
                    let winner = if minigame {s.minigame["winner"].as_u64().unwrap_or(0) as u8} else {s.winner};
                    let (color_name, fallback) = if winner == key {
                        ("win_rgb", 0x00ff00)
                    } else if winner != 0 {
                        ("lose_rgb", 0xff0000)
                    } else {
                        ("idle_rgb", 0xffffff)
                    };
                    let rgb = s.key_game[color_name].as_u64().unwrap_or(fallback) as u32;
                    super::key_boxes::BoxView {
                        enabled: mask & (1 << index) != 0,
                        pressed: if minigame {
                            s.minigame["held_mask"].as_u64().unwrap_or(0) & (1 << index) != 0
                        } else if key == 9 {
                            s.input_9
                        } else {
                            s.active_inputs & (1 << index) != 0
                        },
                        winner: winner == key,
                        round_active: winner != 0,
                        color: egui::Color32::from_rgb(
                            (rgb >> 16) as u8,
                            (rgb >> 8) as u8,
                            rgb as u8,
                        ),
                        groups: std::array::from_fn(|group| {
                            self.table
                                .pixel_sample
                                .as_ref()
                                .filter(|sample| sample.fresh())
                                .filter(|sample| !pixel_groups::advertised(&s) || pixel_groups::for_status(&s)
                                    .is_some_and(|layout|sample.frame.group_layout_revision==Some(layout.revision)))
                                .and_then(|sample| sample.frame.key_group(key, group as u8))
                        }),
                    }
                });
                let can_trigger = if minigame {minigame_writable && crate::minigame::ready(&s)}
                    else {super::effect_controls::game_press_available(&s)};
                let result = super::key_boxes::show(
                    ui,
                    &s.board_id,
                    &mut self.key_positions,
                    &boxes,
                    &mut self.key_selection,
                    self.key_layout_edit,
                    can_trigger,
                    self.key_trigger_on_click,
                    self.config.animate_cells,
                );
                if let Some(editing) = result.editing { self.key_layout_edit = editing; }
                if let Some(panel) = result.panel {
                    if panel.available(self.presentation_context()) { self.reveal_panels.push(panel); }
                }
                if result.layout_changed || result.selection_changed {
                    self.save_workspace();
                }
                if let Some(key) = result.trigger {
                    if minigame {
                        let request=crate::minigame::input(key,&crate::minigame::input_action(ui,&s));
                        if let Err(error)=self.send_minigame(request) {self.message=error;}
                    } else {
                        // Use the same immediate, ACK-tracked USB/WS owner as
                        // other hardware actions, including serial timing/readback.
                        self.action("/api/game/press", vec![("key".into(), key.to_string())], 0x68, vec![key]);
                    }
                }
                if let Some(key) = result.blocked_trigger {
                    let reason = if !s.avr_online { "AVR is offline" }
                        else if s.programming_latch { "AVR programming is active" }
                        else if s.key_mask_applied != Some(true) { "Enabled boxes have not been acknowledged by the AVR" }
                        else if minigame { "The game is not ready for input" }
                        else { "This board has not enabled remote button presses" };
                    self.notify("Box not triggered", &format!("Box {key}: {reason}."), messaging::Severity::Warning);
                }
            });
            ui.add_space(10.0);
        }
        if game_only {
            return;
        }
        dialog::section(ui, icons::CPU, "Controllers", |ui| {
            ui.horizontal_wrapped(|ui| {
                if avr_acknowledged {
                    badge(ui, "AVR link", s.avr_online);
                }
                if self.has_feature("display") {
                    badge(ui, "TM1637", s.tm1637_ack);
                }
            });
            super::panel_details::detail(ui, icons::CIRCUITRY, "Firmware", &s.version);
            super::panel_details::detail(ui, icons::GLOBE, "Host", &s.hostname);
            if avr_acknowledged {
                if let Some(version) = s.avr_version.as_deref() {
                    super::panel_details::detail(ui, icons::CPU, "AVR firmware", version);
                }
                super::panel_details::detail(ui, icons::PLUG, "UART", &s.link_mode);
                super::panel_details::detail(
                    ui,
                    icons::WARNING,
                    "UART errors",
                    &format!("ESP {} / AVR {}", s.esp_uart_errors, s.avr_uart_errors),
                );
            }
            if self.has_feature("isp") {
                super::panel_details::detail(
                    ui,
                    icons::LOCK,
                    "Program latch",
                    if s.programming_latch {
                        "Active"
                    } else {
                        "Released"
                    },
                );
            }
            if self.command_route() == Some(Transport::Serial) {
                super::panel_details::detail(
                    ui,
                    icons::PLUG,
                    "USB errors",
                    &s.usb_serial_errors.to_string(),
                );
            }
            ui.add_space(8.0);
            let firmware_available = self.has_feature("ota") || self.has_feature("isp");
            let mut actions = vec![(icons::INFO, "Board details…")];
            if firmware_available {
                actions.push((icons::FLOPPY_DISK, "Firmware & storage…"));
            }
            super::panel_details::action_row(ui, &actions, |ui| {
                if dialog::action_button(ui, icons::INFO, "Board details…").clicked() {
                    self.device_settings.page =
                        super::device_settings::page_index("overview").unwrap_or(0);
                    self.device_settings.search.clear();
                    self.device_settings.open = true;
                    self.configuration_loaded = false;
                }
                if firmware_available
                    && dialog::action_button(ui, icons::FLOPPY_DISK, "Firmware & storage…")
                        .clicked()
                {
                    self.device_settings.page =
                        super::device_settings::page_index("firmware").unwrap_or(0);
                    self.device_settings.search.clear();
                    self.device_settings.open = true;
                    self.configuration_loaded = false;
                }
            });
        });
        ui.add_space(10.0);
        if let Some(manifest) = s.capabilities.as_ref() {
            dialog::section(ui, icons::LIST_CHECKS, "Acknowledged hardware", |ui| {
                super::panel_details::detail(ui, icons::CIRCUITRY, "Role", &manifest.role);
                if let Some(output) = manifest.led_output.as_ref() {
                    super::panel_details::detail(
                        ui,
                        icons::LIGHTNING,
                        "LED driver",
                        output.label(),
                    );
                }
                if let Some(table) = manifest.table.as_ref() {
                    super::panel_details::detail(
                        ui,
                        icons::GRID_FOUR,
                        "Table",
                        &format!(
                            "{} × {} · {} pixels per cell",
                            table.columns, table.rows, table.pixels_per_cell
                        ),
                    );
                }
                let mut features = manifest.acknowledged_features.clone();
                features.sort();
                features.dedup();
                super::panel_details::feature_chips(ui, &features);
            });
            ui.add_space(10.0);
        }
        if self.has_feature("inputs") {
            dialog::section(ui, icons::HAND_TAP, "Inputs", |ui| {
                ui.horizontal_wrapped(|ui| {
                    for bit in 0..9 {
                        let active = if bit == 8 {
                            s.input_9
                        } else {
                            s.active_inputs & (1 << bit) != 0
                        };
                        badge(ui, &format!("{}", bit + 1), active);
                    }
                });
            });
        }
        ui.add_space(10.0);
        if self.has_feature("outputs") {
            dialog::section(ui, icons::LIGHTBULB, "Shift-register outputs", |ui| {
                ui.add_enabled_ui(s.avr_online && !s.programming_latch, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for bit in 0..8 {
                            let mut on = s.active_outputs & (1 << bit) != 0;
                            if ui.checkbox(&mut on, (bit + 1).to_string()).changed() {
                                let mask = if on {
                                    s.active_outputs | (1 << bit)
                                } else {
                                    s.active_outputs & !(1 << bit)
                                };
                                self.action(
                                    "/api/outputs",
                                    vec![("mask".into(), mask.to_string())],
                                    OP_OUTPUT_MASK,
                                    vec![mask],
                                );
                            }
                        }
                    });
                });
            });
        }
    }
    pub(super) fn draw_device_identity(&mut self, ui: &mut egui::Ui) {
        let writable = self.command_route() == Some(Transport::Serial) || self.http_writable();
        dialog::section(ui, icons::GLOBE, "Board identity", |ui| {
            if let Some(app) = self.peer_context().and_then(|p| p.application()) {
                ui.label(egui::RichText::new(&app.name).strong());
                ui.label(
                    egui::RichText::new(format!("{} · {}", app.title, app.id))
                        .small()
                        .weak(),
                );
                ui.label(egui::RichText::new(crate::ui::i18n::text("Application branding is reported by the running firmware; renaming this board does not change its software identity.")).small().weak());
            }
            if self.command_route() == Some(Transport::Serial)
                && self.peer_context().is_none_or(|c| c.hostname.is_empty())
            {
                ui.label(crate::ui::i18n::text("Reading this USB board’s identity…"));
                return;
            }
            if let Some(name) = &mut self.device_name {
                if let Some(label) = &mut self.board_display_name {
                    field(ui, icons::CIRCUITRY, "Board display name", label, false);
                }
                field(ui, icons::GLOBE, "Hostname", name, false);
                ui.label(egui::RichText::new(crate::ui::i18n::text("The display name is separate from the DNS hostname. Saved across OTA. Only a hostname change restarts Wi-Fi/AP; credentials stay unchanged.")).small().weak());
                let label = self.board_display_name.clone();
                if ui
                    .add_enabled_ui(writable && !self.identity_pending, |ui| {
                        dialog::primary_action_button(ui, icons::CHECK, "Save board identity")
                    })
                    .inner
                    .clicked()
                {
                    match crate::identity::normalize_name(name).and_then(|n| if label.as_ref().is_some_and(|s| !crate::identity::valid_display_name(s)) { Err("Board name must be 1..63 UTF-8 bytes without controls or edge spaces".into()) } else { Ok(n) }) {
                        Ok(name) => {
                            self.identity_pending = true;
                            if self.command_route() == Some(Transport::Serial) {
                                if let Some(admission)=self.serial_admission().filter(|a|a.peer.is_some()){
                                    let _=self.serial_tx.send(crate::SerialCommand::Identity(self.serial_port.clone(),admission,name,label.unwrap_or_default()));
                                }else{self.identity_pending=false;self.message="Read this USB board’s verified identity before saving; nothing queued".into();}
                            } else {
                                let _ = self.http_tx.send(crate::HttpCommand::NameChange(self.endpoint(), name, label));
                            }
                        }
                        Err(error) => self.notify(
                            "Invalid device name",
                            &error,
                            crate::messaging::Severity::Warning,
                        ),
                    }
                }
                if let Some((host, label)) = &self.identity_saved {
                    if self.device_name.as_ref() != Some(host)
                        || self.board_display_name.as_ref() != Some(label)
                    {
                        ui.label(
                            egui::RichText::new(crate::ui::i18n::text(
                                "Unsaved board identity changes",
                            ))
                            .small()
                            .weak(),
                        );
                        if dialog::action_button(ui, icons::X, "Discard name edits").clicked() {
                            self.device_name = Some(host.clone());
                            self.board_display_name = Some(label.clone());
                        }
                    }
                }
            } else {
                ui.label(if self.live_status().is_none() {
                    "Device offline. Connect to view its name."
                } else {
                    "Device name unavailable."
                });
            }
        });
    }
    pub(super) fn draw_device_access(&mut self, ui: &mut egui::Ui) {
        let writable = self.http_writable();
        dialog::section(ui, icons::LOCK, "Device access", |ui| {
            if let Some(info) = &self.access_info {
                if info.get("ready").and_then(|v| v.as_bool()) != Some(true) {
                    ui.colored_label(
                        ui.visuals().error_fg_color,
                        "Device access storage is unavailable. Physical USB recovery is required.",
                    );
                    return;
                }
                ui.add_enabled_ui(writable, |ui| {
                    ui.checkbox(&mut self.access_enabled, "Require authentication");
                });
                if self.access_enabled
                    && info.get("default_credentials").and_then(|v| v.as_bool()) == Some(true)
                {
                    ui.colored_label(
                        ui.visuals().warn_fg_color,
                        "Default login: admin / pakarcontroller. Change it here.",
                    );
                }
                if self.access_enabled {
                    field(
                        ui,
                        icons::LOCK,
                        "New username",
                        &mut self.access_user,
                        false,
                    );
                    field(
                        ui,
                        icons::LOCK,
                        "New password",
                        &mut self.access_password,
                        true,
                    );
                    field(
                        ui,
                        icons::LOCK,
                        "Confirm password",
                        &mut self.access_confirm,
                        true,
                    );
                }
                ui.weak(if self.access_enabled {
                    "Saved on the device across OTA. Existing clients reconnect after a change."
                } else {
                    "Authentication is disabled. No username or password is required."
                });
                if ui
                    .add_enabled_ui(writable, |ui| {
                        dialog::primary_action_button(ui, icons::CHECK, "Save device access")
                    })
                    .inner
                    .clicked()
                {
                    match if self.access_enabled {
                        crate::access::validate(
                            &self.access_user,
                            &self.access_password,
                            &self.access_confirm,
                        )
                    } else {
                        Ok(())
                    } {
                        Ok(()) => {
                            let next = crate::Endpoint {
                                base: self.base.clone(),
                                user: if self.access_enabled {
                                    self.access_user.clone()
                                } else {
                                    String::new()
                                },
                                password: if self.access_enabled {
                                    self.access_password.clone()
                                } else {
                                    String::new()
                                },
                            };
                            let _ = self.http_tx.send(crate::HttpCommand::AccessChange(
                                self.endpoint(),
                                next,
                                self.access_enabled,
                            ));
                        }
                        Err(error) => self.notify(
                            "Cannot save credentials",
                            &error,
                            crate::messaging::Severity::Warning,
                        ),
                    }
                }
            } else {
                ui.label(crate::ui::i18n::text("Credential settings have not been advertised by the device. Connect to the updated firmware to configure them."));
            }
        });
    }
    fn draw_network(&mut self, ui: &mut egui::Ui) {
        if let Some(status) = self.live_status().cloned() {
            if let Some(page) = super::panel_details::network(ui, &status) {
                self.device_settings.page = super::device_settings::page_index(page).unwrap_or(0);
                self.device_settings.search.clear();
                self.device_settings.open = true;
                self.configuration_loaded = false;
            }
        }
    }
    fn draw_streaming(&mut self, ui: &mut egui::Ui) {
        let rgb_peer=self.live_status().filter(|s|rgb_input::ready(s))
            .and_then(peer_http::Pin::from_status).filter(|_|self.websocket_live);
        let busy=self.streaming||self.sampling
            || (self.rgb_input.cells && (!streaming::valid_mapping(&self.mappings)
                || self.live_status().and_then(streaming::sampling_layout).is_none()));
        if let Some(command)=self.rgb_input.show(ui,rgb_peer,busy,&self.config.serial_port) {
            if let Err(error)=self.ws_tx.send(command) {
                if let WsCommand::Rgb(Some(request))=error.0 {rgb_input::reject(request,"Board worker is unavailable");}
            }
        }
        ui.add_space(10.0);
        dialog::section(ui, icons::MONITOR_PLAY, "Stream source", |ui| {
            ui.horizontal_wrapped(|ui| {
                badge(ui, "WebSocket", self.websocket_live);
                ui.weak(if self.streaming {
                    "Rainbow"
                } else if self.sampling {
                    "Primary screen"
                } else {
                    "Stopped"
                });
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if ui::effect_controls::choice(
                    ui,
                    if self.streaming {
                        icons::STOP_CIRCLE
                    } else {
                        icons::PLAY
                    },
                    if self.streaming {
                        "Stop rainbow"
                    } else {
                        "Stream rainbow"
                    },
                    self.streaming,
                    !self.rgb_input.active() && (self.streaming || self.websocket_live),
                )
                .clicked()
                {
                    self.streaming = !self.streaming;
                    if self.streaming {
                        self.sampling = false;
                    }
                    let _ = self.ws_tx.send(WsCommand::Stream(self.streaming));
                }
                if ui::effect_controls::choice(
                    ui,
                    if self.sampling {
                        icons::STOP_CIRCLE
                    } else {
                        icons::MONITOR_PLAY
                    },
                    if self.sampling {
                        "Stop sampler"
                    } else {
                        "Sample screen"
                    },
                    self.sampling,
                    !self.rgb_input.active() && (self.sampling
                        || (self.websocket_live && streaming::valid_mapping(&self.mappings)
                            && self.live_status().and_then(streaming::sampling_layout).is_some())),
                )
                .on_hover_text(crate::ui::i18n::text(
                    "Requires a complete validated cell map and physical group layout from this board",
                ))
                .clicked()
                {
                    self.sampling = !self.sampling;
                    if self.sampling {
                        self.streaming = false;
                    }
                    let _ = self.ws_tx.send(WsCommand::Sample(self.sampling));
                }
            });
        });
        ui.add_space(10.0);
        dialog::section(ui, icons::GAUGE, "Frame delivery", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(crate::ui::i18n::text("Target rate"));
                if ui
                    .add(
                        egui::DragValue::new(&mut self.config.stream_fps)
                            .range(0..=streaming::MAX_FPS)
                            .suffix(" FPS"),
                    )
                    .on_hover_text("0 = uncapped. Shared by RGB input, rainbow and screen sampling.")
                    .changed()
                {
                    let _ = self.ws_tx.send(WsCommand::Rate(self.config.stream_fps));
                }
            });
            let sent = self
                .stream_sent_fps
                .filter(|(_, at)| at.elapsed() < Duration::from_secs(3))
                .map(|(fps, _)| format!("{fps:.1} FPS"))
                .unwrap_or_else(|| "—".into());
            ui::panel_details::detail(ui, icons::BROADCAST, "Sent rate", &sent);
            ui.label(egui::RichText::new("Completed frame writes").small().weak())
                .on_hover_text("Transport throughput, not measured physical LED refresh.");
        });
    }
    fn draw_events(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("{} events", self.event_log.len())).weak());
            if ui
                .button(icons::TRASH)
                .on_hover_text(crate::ui::i18n::text("Clear events"))
                .clicked()
            {
                self.event_log.clear();
                self.workspace.errors.borrow_mut().clear();
            }
        });
        for event in self.event_log.iter().rev() {
            let kind = &event.kind;
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new(kind).color(if kind == "Error" {
                    ui.visuals().error_fg_color
                } else {
                    ui.visuals().weak_text_color()
                }));
                ui.label(&event.message);
                if event.occurrences > 1 {
                    ui.label(egui::RichText::new(format!("×{}", event.occurrences)).weak())
                        .on_hover_text(format!(
                            "{} occurrences; last seen {:.0}s ago; {}",
                            event.occurrences,
                            event.last_seen.elapsed().as_secs_f64(),
                            if event.active {
                                "unresolved diagnostic"
                            } else {
                                "closed diagnostic"
                            }
                        ));
                }
            });
        }
    }
    pub(super) fn select_mapping(&mut self, id: u8) {
        self.mapping_logical = id;
        if let Some(m) = self.mappings.iter().find(|m| m.logical_id == id) {
            self.mapping_physical = m.physical_cell;
            self.mapping_name = m.name.clone();
        }
    }
    fn draw_dialogs(&mut self, ctx: &egui::Context) {
        if self.confirm_reset {
            let response = egui::Modal::new(egui::Id::new("reset-game")).show(ctx, |ui| {
                ui.set_width(360.0);
                ui.heading(crate::ui::i18n::text("Reset the current game?"));
                ui.label(crate::ui::i18n::text(
                    "The winner and current game state will be cleared.",
                ));
                ui.add_space(12.0);
                dialog::action_row(ui, false, |ui| {
                    if dialog::primary_action_button(ui, icons::ARROW_COUNTER_CLOCKWISE, "Reset")
                        .clicked()
                    {
                        self.action("/api/game/reset", vec![], OP_RESET, vec![]);
                        self.confirm_reset = false;
                    }
                    if dialog::action_button(ui, icons::X, "Cancel").clicked() {
                        self.confirm_reset = false;
                    }
                });
            });
            if response.should_close() {
                self.confirm_reset = false;
            }
        }
        if self.show_connection {
            let mut method = self.transport;
            let mut draft = ui::connection_controls::Draft {
                endpoint: self.base.clone(),
                user: self.user.clone(),
                password: self.password.clone(),
                port: self.serial_port.clone(),
            };
            let mut lifecycle = None;
            let response = egui::Modal::new(egui::Id::new("connection-dialog")).show(ctx, |ui| {
                ui.set_width(480.0);
                ui.heading(format!(
                    "{}  {}",
                    icons::PLUG,
                    crate::ui::i18n::text("Connect to controller")
                ));
                ui.add_space(12.0);
                ui::connection_controls::method_picker(ui, &mut method, self.identity_pending);
                ui.add_space(10.0);
                ui::connection_controls::details(
                    ui,
                    method,
                    !self.identity_pending && !self.connection_enabled,
                    crate::access::requirement(&self.access_requirement, &draft.endpoint),
                    &mut draft,
                    || {
                        serialport::available_ports()
                            .map(|ports| ports.into_iter().map(|port| port.port_name).collect())
                            .map_err(|error| error.to_string())
                    },
                );
                if self.identity_pending {
                    ui.weak("Identity write in progress; connection changes are locked.");
                } else if self.connection_enabled {
                    ui.weak("Disconnect before editing the endpoint or credentials.");
                }
                if method == Transport::WiFi {
                    ui::host_wifi::draw(ui, !self.identity_pending);
                }
                ui.add_space(12.0);
                dialog::action_row(ui, false, |ui| {
                    let endpoint_present = match method {
                        Transport::WiFi => !draft.endpoint.trim().is_empty(),
                        Transport::Serial => !draft.port.trim().is_empty(),
                    };
                    if ui
                        .add_enabled_ui(
                            !self.identity_pending && (self.connection_enabled || endpoint_present),
                            |ui| {
                                dialog::primary_action_button(
                                    ui,
                                    icons::PLUG,
                                    if self.connection_enabled {
                                        "Disconnect"
                                    } else {
                                        "Connect"
                                    },
                                )
                            },
                        )
                        .inner
                        .clicked()
                    {
                        lifecycle = Some(!self.connection_enabled);
                        self.show_connection = false;
                    }
                    if dialog::action_button(ui, icons::X, "Close").clicked() {
                        self.show_connection = false;
                    }
                });
            });
            let connection_draft_changed = draft.endpoint != self.base
                || draft.user != self.user
                || draft.password != self.password
                || draft.port != self.serial_port;
            let connection_change_requested =
                method != self.transport || connection_draft_changed || lifecycle.is_some();
            if connection_change_requested && !self.connection_change_guard() {
                if method != self.transport {
                    self.set_connection(method, false);
                    // Switching the method is a deliberate disconnect, never
                    // a same-frame implicit connect through another worker.
                    lifecycle = None;
                }
                if method == self.transport && !self.connection_enabled && connection_draft_changed
                {
                    self.base = draft.endpoint;
                    self.user = draft.user;
                    self.password = draft.password;
                    self.serial_port = draft.port;
                    self.config.endpoint = self.base.clone();
                    self.config.user = self.user.clone();
                    self.config.serial_port = self.serial_port.clone();
                    self.save_workspace();
                }
                if method == self.transport {
                    if let Some(enabled) = lifecycle {
                        self.set_connection(method, enabled);
                    }
                }
            }
            if response.should_close() {
                self.show_connection = false;
            }
        }
        if self.show_mapping {
            let mut open = true;
            let g = dialog::bounded_geometry(
                ctx.content_rect(),
                20.0,
                egui::vec2(700.0, 560.0),
                egui::vec2(520.0, 380.0),
                egui::vec2(950.0, 750.0),
            );
            egui::Window::new(format!(
                "{}  {}",
                icons::GRID_FOUR,
                crate::ui::i18n::text("Cell mapping")
            ))
            .id(egui::Id::new("cell-mapping"))
            .open(&mut open)
            .default_rect(g.default_rect)
            .min_size(g.min_size)
            .max_size(g.max_size)
            .constrain_to(g.bounds)
            .collapsible(false)
            .frame(dialog::opaque_window_frame_from_context(ctx))
            .show(ctx, |ui| {
                ui.weak(self.mapping_status());
                ui.horizontal(|ui| {
                    ui.label(crate::ui::i18n::text("Logical"));
                    let changed = ui
                        .add(egui::DragValue::new(&mut self.mapping_logical).range(1..=80))
                        .changed();
                    if changed {
                        self.select_mapping(self.mapping_logical);
                    }
                    ui.label(crate::ui::i18n::text("Physical"));
                    ui.add(egui::DragValue::new(&mut self.mapping_physical).range(1..=80));
                    ui.label(crate::ui::i18n::text("Name"));
                    ui.add_sized(
                        [ui.available_width().min(180.0), 28.0],
                        dialog::singleline_text_edit(ui, &mut self.mapping_name).char_limit(15),
                    );
                });
                ui.add_space(8.0);
                dialog::scroll_column(
                    ui,
                    "mapping-list",
                    Some((ui.available_height() - 50.0).max(200.0)),
                    |ui| {
                        egui::Grid::new("mapping-grid")
                            .num_columns(3)
                            .striped(true)
                            .spacing([24.0, 6.0])
                            .show(ui, |ui| {
                                ui.strong(crate::ui::i18n::text("Logical cell"));
                                ui.strong(crate::ui::i18n::text("Physical cell"));
                                ui.strong(crate::ui::i18n::text("Name"));
                                ui.end_row();
                                for m in self.mappings.clone() {
                                    if ui
                                        .selectable_label(
                                            self.mapping_logical == m.logical_id,
                                            m.logical_id.to_string(),
                                        )
                                        .clicked()
                                    {
                                        self.select_mapping(m.logical_id);
                                    }
                                    ui.label(m.physical_cell.to_string());
                                    ui.label(&m.name);
                                    ui.end_row();
                                }
                            });
                    },
                );
                ui.separator();
                dialog::action_row(ui, false, |ui| {
                    if ui
                        .add_enabled(
                            !self.mappings.is_empty()
                                && self.command_route() == Some(Transport::WiFi),
                            egui::Button::new(format!(
                                "{}  {}",
                                icons::CHECK,
                                crate::ui::i18n::text("Save mapping")
                            ))
                            .min_size(egui::vec2(104.0, 28.0)),
                        )
                        .clicked()
                    {
                        self.post(
                            "/api/cells/mapping",
                            vec![
                                ("logical_id".into(), self.mapping_logical.to_string()),
                                ("physical_cell".into(), self.mapping_physical.to_string()),
                                ("name".into(), self.mapping_name.clone()),
                            ],
                        );
                        self.last_mapping_poll = Instant::now() - Duration::from_secs(30);
                    }
                    if dialog::action_button(ui, icons::X, "Close").clicked() {
                        self.show_mapping = false;
                    }
                });
            });
            if !open {
                self.show_mapping = false;
            }
        }
        if self.show_workspaces {
            self.draw_workspaces(ctx);
        }
        if self.show_about {
            let mut open = true;
            let app = self.peer_context().and_then(|p| p.application()).cloned();
            let name = app
                .as_ref()
                .map(|a| a.name.as_str())
                .unwrap_or("PakarController");
            egui::Window::new(format!("{}  About {name}",icons::INFO)).id(egui::Id::new("about-pakar")).open(&mut open).default_size([520.0,300.0]).collapsible(false).frame(dialog::opaque_window_frame_from_context(ctx)).show(ctx,|ui|{
                ui.horizontal(|ui|{crate::branding::peer_logo(ui,48.0,app.as_ref());ui.vertical(|ui|{ui.heading(name);if let Some(app)=&app {ui.label(&app.title);ui.label(egui::RichText::new(&app.id).weak());}ui.label(format!("PakarController client {}",env!("CARGO_PKG_VERSION")));});});ui.add_space(16.0);
                dialog::section(ui,icons::APP_WINDOW,"Application foundation",|ui|{ui.label(crate::ui::i18n::text("Built from Pealayer’s desktop workspace, dialog controls, icon and color pickers, themes, fonts, notifications, and native instance ownership."));ui.label(crate::ui::i18n::text("MIT License · Pealayer © 2026 ToghrolTP"));ui.hyperlink_to("Pealayer source","https://github.com/ToghrolTP/pealayer");});ui.add_space(12.0);
                dialog::action_row(ui,false,|ui|{if dialog::action_button(ui,icons::X,"Close").clicked(){self.show_about=false;}});
            });
            if !open {
                self.show_about = false;
            }
        }
    }
    fn draw_workspaces(&mut self, ctx: &egui::Context) {
        let mut open = true;
        let g = dialog::bounded_geometry(
            ctx.content_rect(),
            20.0,
            egui::vec2(760.0, 480.0),
            egui::vec2(520.0, 320.0),
            egui::vec2(1000.0, 700.0),
        );
        egui::Window::new(format!(
            "{}  {}",
            icons::TABS,
            crate::ui::i18n::text("Workspaces")
        ))
        .id(egui::Id::new("workspace-profiles"))
        .open(&mut open)
        .default_rect(g.default_rect)
        .min_size(g.min_size)
        .max_size(g.max_size)
        .constrain_to(g.bounds)
        .collapsible(false)
        .frame(dialog::opaque_window_frame_from_context(ctx))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [ui.available_width() - 240.0, 28.0],
                    dialog::singleline_text_edit(ui, &mut self.workspace_name)
                        .hint_text(crate::ui::i18n::text("Workspace name")),
                );
                icons::searchable_workspace_icon_picker(
                    ui,
                    "workspace-icon",
                    &mut self.workspace_icon,
                    90.0,
                    "Search icons",
                    "Workspace icons",
                    "No matching icons",
                    config::resolved_language(&self.config),
                );
                if dialog::primary_action_button(ui, icons::PLUS, "Save new").clicked()
                    && !self.workspace_name.trim().is_empty()
                {
                    if let Ok(layout) = layout::serialize(&self.dock) {
                        self.config.workspace_profiles.insert(
                            uuid::Uuid::new_v4().to_string(),
                            config::WorkspaceProfile {
                                name: self.workspace_name.trim().into(),
                                icon: self.workspace_icon.clone(),
                                layout,
                            },
                        );
                        self.workspace_name.clear();
                        self.save_workspace();
                    }
                }
            });
            ui.add_space(8.0);
            ui.separator();
            dialog::scroll_column(
                ui,
                "workspace-list",
                Some((ui.available_height() - 50.0).max(180.0)),
                |ui| {
                    for (id, profile) in self.config.workspace_profiles.clone() {
                        egui::Frame::group(ui.style())
                            .inner_margin(egui::Margin::symmetric(10, 7))
                            .corner_radius(8.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(icons::workspace_icon(&profile.icon))
                                            .color(ui.visuals().selection.bg_fill),
                                    );
                                    let mut name = profile.name.clone();
                                    if ui
                                        .add_sized(
                                            [ui.available_width() - 235.0, 28.0],
                                            dialog::singleline_text_edit(ui, &mut name),
                                        )
                                        .changed()
                                        && !name.trim().is_empty()
                                    {
                                        self.config.workspace_profiles.get_mut(&id).unwrap().name =
                                            name;
                                    }
                                    if ui
                                        .button(format!(
                                            "{}  Restore",
                                            icons::ARROW_COUNTER_CLOCKWISE
                                        ))
                                        .clicked()
                                    {
                                        let dock = layout::restore(&profile.layout);
                                        let hidden = Panel::ALL
                                            .into_iter()
                                            .filter(|p| dock.find_tab(p).is_none())
                                            .collect();
                                        self.workspace.layout =
                                            crate::workspace_layout::WorkspaceLayout::restore(
                                                &profile.layout,
                                                Some(hidden),
                                            );
                                        let available: Vec<_> = Panel::ALL
                                            .into_iter()
                                            .filter(|p| p.available(self.presentation_context()))
                                            .collect();
                                        self.workspace
                                            .layout
                                            .synchronize(&mut self.dock, &available);
                                        self.save_workspace();
                                    }
                                    if ui
                                        .button(icons::FLOPPY_DISK)
                                        .on_hover_text(crate::ui::i18n::text(
                                            "Replace with current layout",
                                        ))
                                        .clicked()
                                    {
                                        if let Ok(layout) = layout::serialize(&self.dock) {
                                            self.config
                                                .workspace_profiles
                                                .get_mut(&id)
                                                .unwrap()
                                                .layout = layout;
                                            self.save_workspace();
                                        }
                                    }
                                    if ui
                                        .button(icons::TRASH)
                                        .on_hover_text(crate::ui::i18n::text("Delete workspace"))
                                        .clicked()
                                    {
                                        self.config.workspace_profiles.remove(&id);
                                        self.save_workspace();
                                    }
                                });
                            });
                        ui.add_space(4.0);
                    }
                },
            );
            dialog::action_row(ui, false, |ui| {
                if dialog::action_button(ui, icons::X, "Close").clicked() {
                    self.show_workspaces = false;
                    self.save_workspace();
                }
            });
        });
        if !open {
            self.show_workspaces = false;
            self.save_workspace();
        }
    }
}
