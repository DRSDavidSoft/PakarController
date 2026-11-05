use super::*;
use crate::ui::{
    dialog,
    dropdown::{ComboBox, DropdownUiExt},
    icons,
};
use eframe::egui;

#[derive(Default)]
pub struct Controls {
    pub config: Config,
    pub view: View,
    serial_mode: bool,
    pub cells: bool,
}
impl Controls {
    pub fn active(&self) -> bool {
        let s = snapshot(&self.view);
        s.running || s.pending
    }
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        ready: Option<Pin>,
        busy: bool,
        board_port: &str,
    ) -> Option<crate::WsCommand> {
        let mut action = None;
        let state = snapshot(&self.view);
        let active = state.running || state.pending;
        dialog::section(ui, icons::BROADCAST, "External RGB", |ui| {
            if !active {
                let previous = self.config.protocol.clone();
                ComboBox::from_id_salt("rgb-protocol")
                    .selected_text(&self.config.protocol)
                    .show_ui(ui, |ui| {
                        for name in protocol::PROTOCOLS {
                            if ui
                                .dropdown_choice(self.config.protocol == *name, *name)
                                .clicked()
                            {
                                self.config.protocol = (*name).into();
                            }
                        }
                    });
                if self.config.protocol != previous
                    && self.config.listen == protocol::endpoint(&previous)
                {
                    self.config.listen = protocol::endpoint(&self.config.protocol);
                }
                if !protocol::serial_supported(&self.config.protocol) {
                    self.serial_mode = false;
                }
                ui.horizontal_wrapped(|ui|{
                    ui.checkbox(&mut self.cells,"80 logical cells").on_hover_text("Expand each logical cell through the board's current physical group map. Otherwise receive all 480 physical pixels.");
                    ui.add_enabled_ui(protocol::serial_supported(&self.config.protocol),|ui|{ui.checkbox(&mut self.serial_mode,"Serial bridge");});
                });
                self.config.pixels = if self.cells { 80 } else { 480 };
                if self.serial_mode {
                    ui.horizontal(|ui| {
                        ui.label("Input port");
                        ui.text_edit_singleline(&mut self.config.serial);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Baud");
                        ui.add(egui::DragValue::new(&mut self.config.baud).range(1200..=2_000_000));
                    });
                } else {
                    ui.horizontal(|ui| {
                        ui.label("Listen");
                        ui.text_edit_singleline(&mut self.config.listen);
                    });
                    ui.horizontal(|ui|{ui.label("Trusted sender");ui.text_edit_singleline(&mut self.config.source).on_hover_text("Optional on loopback; required for LAN. IP filtering is not authentication.");});
                }
                if matches!(self.config.protocol.as_str(), "artnet" | "sacn") {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Universe");
                        ui.add(egui::DragValue::new(&mut self.config.universe).range(
                            if self.config.protocol == "sacn" {
                                1..=63999
                            } else {
                                0..=32767
                            },
                        ));
                        ui.label("First channel");
                        ui.add(egui::DragValue::new(&mut self.config.channel).range(1..=512));
                    });
                }
                ui.horizontal(|ui| {
                    ui.label("Sender timeout");
                    ui.add(
                        egui::DragValue::new(&mut self.config.timeout_secs)
                            .range(1..=60)
                            .suffix(" s"),
                    );
                });
            } else {
                ui.label(format!(
                    "{} · {} colors",
                    self.config.protocol, self.config.pixels
                ));
                if !state.sender.is_empty() {
                    ui.monospace(&state.sender);
                }
                ui.weak(format!(
                    "{} received · {} sent · {} coalesced · {} rejected",
                    state.received, state.sent, state.coalesced, state.rejected
                ));
                ui.ctx().request_repaint_after(Duration::from_millis(200));
            }
            if !state.error.is_empty() {
                ui.colored_label(ui.visuals().error_fg_color, &state.error);
            }
            let mut config = self.config.clone();
            if self.serial_mode {
                config.listen.clear();
                config.source.clear();
            } else {
                config.serial.clear();
            }
            let error = config.validate().err().or_else(|| {
                (self.serial_mode
                    && (config.serial.trim().is_empty() || same_serial(&config.serial, board_port)))
                .then(|| {
                    "Choose a separate virtual serial input, not the controller board port".into()
                })
            });
            let enabled = active || (ready.is_some() && !busy && error.is_none());
            let response = crate::ui::effect_controls::choice(
                ui,
                if active {
                    icons::STOP_CIRCLE
                } else {
                    icons::PLAY
                },
                if active {
                    "Stop RGB input"
                } else {
                    "Start RGB input"
                },
                active,
                enabled,
            );
            if response.clicked() {
                if active {
                    action = Some(crate::WsCommand::Rgb(None));
                } else if let Some(peer) = ready {
                    self.view = View::default();
                    self.view.lock().unwrap().status.pending = true;
                    action = Some(crate::WsCommand::Rgb(Some(Request {
                        config,
                        peer,
                        view: self.view.clone(),
                        at: Instant::now(),
                    })));
                }
            } else if !enabled {
                response.on_hover_text(error.unwrap_or_else(||"Connect the enabled table board in Effects output mode, with no global effect or other stream; load its cell map for logical mode".into()));
            }
        });
        action
    }
}
