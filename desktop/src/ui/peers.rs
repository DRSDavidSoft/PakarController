use super::{dialog, icons};
use crate::peer_catalogue::Workspace;
use eframe::egui;

pub fn draw(ui: &mut egui::Ui, state: &mut Workspace) -> Option<String> {
    state.service();
    let mut open = None;
    ui.horizontal_wrapped(|ui| {
        ui.heading(format!("{}  Peers", icons::GLOBE));
        if ui
            .add_enabled_ui(!state.busy(), |ui| {
                dialog::action_button(ui, icons::ARROW_CLOCKWISE, "Discover")
            })
            .inner
            .clicked()
        {
            state.discover(None);
        }
    });
    ui.horizontal(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut state.endpoint)
                .hint_text("http://peer.local:8789")
                .desired_width((ui.available_width() - 90.0).max(100.0)),
        );
        if ui
            .add_enabled_ui(!state.busy() && !state.endpoint.trim().is_empty(), |ui| {
                dialog::action_button(ui, icons::MAGNIFYING_GLASS, "Inspect")
            })
            .inner
            .clicked()
        {
            state.discover(Some(state.endpoint.trim().to_owned()));
        }
    });
    if state.busy() {
        ui.spinner();
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(100));
    }
    if !state.error.is_empty() {
        ui.weak(&state.error);
    }
    ui.add_space(10.0);
    for peer in state.peers.values() {
        let stale = peer.observed.elapsed() > std::time::Duration::from_secs(120);
        dialog::section(
            ui,
            if peer.board() {
                icons::CIRCUITRY
            } else {
                icons::APP_WINDOW
            },
            peer.name(),
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        egui::RichText::new(
                            peer.identity["version"]
                                .as_str()
                                .unwrap_or("Unknown release"),
                        )
                        .strong(),
                    );
                    ui.weak(peer.identity["role"].as_str().unwrap_or(if peer.board() {
                        "Board"
                    } else {
                        "Application"
                    }));
                    ui.weak(if stale {
                        "Stale · inspect again"
                    } else {
                        "Available"
                    });
                });
                ui.weak(&peer.endpoint);
                if let Some(artifacts) = peer.identity["artifacts"].as_array() {
                    for item in artifacts.iter().take(8) {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(format!(
                                "{}  {}",
                                icons::ARROW_DOWN,
                                item["version"].as_str().unwrap_or("")
                            ));
                            ui.weak(format!("{} bytes", item["bytes"].as_u64().unwrap_or(0)));
                        });
                    }
                }
                ui.horizontal_wrapped(|ui| {
                    if (peer.board() || peer.identity["exchange"]["bridge_enabled"] == true)
                        && ui
                            .add_enabled_ui(!stale, |ui| {
                                dialog::action_button(ui, icons::PLUG, "Open controls…")
                            })
                            .inner
                            .clicked()
                    {
                        open = Some(peer.endpoint.clone());
                    }
                    ui.menu_button("Details", |ui| {
                        if dialog::action_button(ui, icons::COPY, "Copy address").clicked() {
                            ui.ctx().copy_text(peer.endpoint.clone());
                            ui.close();
                        }
                        if dialog::action_button(ui, icons::COPY, "Copy peer information").clicked()
                        {
                            ui.ctx().copy_text(
                                serde_json::to_string_pretty(&peer.identity).unwrap_or_default(),
                            );
                            ui.close();
                        }
                        ui.separator();
                        ui.monospace(peer.key());
                        if let Some(exchange) = peer.identity["exchange"].as_object() {
                            for (key, value) in exchange {
                                ui.horizontal_wrapped(|ui| {
                                    ui.weak(key.replace('_', " "));
                                    ui.label(value.to_string());
                                });
                            }
                        }
                    });
                });
            },
        );
        ui.add_space(10.0);
    }
    open
}
