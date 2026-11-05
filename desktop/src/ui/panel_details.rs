//! Contextual panel bodies using the imported Pealayer dialog/card/density
//! primitives. Rendering returns intent; the shell retains peer and ACK ownership.
use super::{
    dialog,
    dropdown::{ComboBox, DropdownUiExt},
    icons,
};
use crate::{DeviceStatus, pixel_corrections};
use eframe::egui;

/// Reserve the whole caption column while keeping its widget and text at the
/// left edge. add_sized centers the widget itself, even with Label::halign(Min).
pub fn field_caption(
    ui: &mut egui::Ui,
    size: impl Into<egui::Vec2>,
    caption: impl Into<egui::WidgetText>,
) -> egui::Response {
    let size = size.into();
    ui.allocate_ui_with_layout(size, egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.set_min_size(size);
        ui.add(egui::Label::new(caption).halign(egui::Align::Min).truncate())
    }).inner
}

pub fn detail(ui: &mut egui::Ui, icon: &str, label: &str, value: &str) {
    let value = if value.is_empty() { "—" } else { value };
    let caption = format!("{icon}  {}", super::i18n::text(label));
    if ui.available_width() < 330.0 {
        ui.add(egui::Label::new(egui::RichText::new(caption).small().weak()).wrap());
        ui.add(egui::Label::new(value).truncate())
            .on_hover_text(value);
    } else {
        ui.horizontal(|ui| {
            field_caption(
                ui,
                [132.0, 20.0],
                egui::RichText::new(caption).weak(),
            );
            ui.add(egui::Label::new(value).truncate())
                .on_hover_text(value);
        });
    }
}

pub fn square_action(ui: &mut egui::Ui, icon: &str, label: &str, enabled: bool) -> egui::Response {
    let side = super::control_density::height(ui);
    ui.add_enabled_ui(enabled, |ui| {
        ui.add_sized(egui::Vec2::splat(side), egui::Button::new(icon))
            .on_hover_text(super::i18n::text(label))
    })
    .inner
}

/// Reserve each chip's complete slot before wrapping. A Frame scope discovers
/// its width too late and can collapse the last label into a vertical column.
pub fn feature_chips(ui: &mut egui::Ui, features: &[String]) {
    let max_width = ui.available_width().max(1.0);
    let font = egui::TextStyle::Small.resolve(ui.style());
    let height = ui.text_style_height(&egui::TextStyle::Small) + 8.0;
    ui.horizontal_wrapped(|ui| {
        for feature in features {
            let text = ui.painter().layout_no_wrap(
                feature.clone(),
                font.clone(),
                ui.visuals().text_color(),
            );
            let size = egui::vec2((text.size().x + 16.0).min(max_width), height);
            let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), feature)
            });
            ui.painter()
                .rect_filled(rect, 4.0, ui.visuals().faint_bg_color);
            ui.painter()
                .with_clip_rect(rect.shrink2(egui::vec2(8.0, 0.0)))
                .galley(
                    egui::pos2(rect.left() + 8.0, rect.center().y - text.size().y * 0.5),
                    text,
                    ui.visuals().text_color(),
                );
            response.on_hover_text(feature);
        }
    });
}

/// Choose a complete row before laying out scoped dialog actions. Egui's
/// horizontal wrapping cannot wrap a scope after its button's minimum width is
/// known, so measuring translated captions prevents narrow dock overflow.
pub fn action_row<R>(
    ui: &mut egui::Ui,
    actions: &[(&str, &str)],
    contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let required_width = actions
        .iter()
        .map(|(icon, label)| {
            let text = ui.painter().layout_no_wrap(
                format!("{icon}  {}", super::i18n::text(label)),
                egui::TextStyle::Button.resolve(ui.style()),
                ui.visuals().text_color(),
            );
            (text.size().x + ui.spacing().button_padding.x * 2.0 + 2.0)
                .max(dialog::ACTION_BUTTON_WIDTH)
        })
        .sum::<f32>()
        + ui.spacing().item_spacing.x * actions.len().saturating_sub(1) as f32;
    ui.scope(|ui| {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
        if ui.available_width() < required_width {
            ui.vertical(contents).inner
        } else {
            ui.horizontal(contents).inner
        }
    })
    .inner
}

fn kind_label(kind: &str) -> &str {
    match kind {
        "black" => "Black out · keep slot",
        "remove" => "Remove slot",
        "shift" => "Shift suffix",
        _ => kind,
    }
}

/// PCController EffectEditor typed numeric draft/save separation, presented
/// with Pealayer's shared bounded cards/actions; no write occurs while editing.
pub fn physical_groups(ui: &mut egui::Ui, value: &mut crate::pixel_groups::Layout,
    mappings: &[crate::CellMapping]) -> bool {
    let mut dirty = false;
    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
    ui.horizontal_wrapped(|ui| {
        ui.weak(format!("{} physical groups · default {} pixels", value.group_count, value.default_count));
        ui.weak(format!("Revision {}", value.revision));
    });
    match value.draft_total() {
        Ok(total) => { ui.label(format!("{total} / {} pixels · {} unused",value.capacity,value.capacity-total)); }
        Err(error) => { ui.colored_label(ui.visuals().warn_fg_color,format!("{}  {error}",icons::WARNING)); }
    }
    let row_height = super::control_density::height(ui).max(28.0) + 8.0;
    egui::ScrollArea::vertical().id_salt("physical-groups-editor").max_height(360.0)
        .auto_shrink([false,true]).show_rows(ui,row_height,usize::from(value.group_count),|ui,rows| {
            for at in rows { let group = (at+1) as u8;
                ui.push_id(group,|ui| {
                    ui.horizontal(|ui| {
                        let mut count=value.overrides.iter().find(|item|item.group==group).map_or(0,|item|item.count);
                        let mut custom=count!=0;
                        let caption=mappings.iter().find(|map|map.physical_cell==group).map(|m|
                            format!("Group {group} · cell {}",m.logical_id)).unwrap_or_else(||format!("Group {group}"));
                        let label_width=(ui.available_width()-170.0).max(55.0);
                        field_caption(ui,[label_width,row_height-8.0],caption.as_str()).on_hover_text(&caption);
                        if ui.checkbox(&mut custom,super::i18n::text("Custom")).changed() {
                            count=if custom {value.default_count} else {0};
                            // Preserve an explicit draft toggle even before its value differs.
                            value.overrides.retain(|item|item.group!=group);
                            if custom {value.overrides.push(crate::pixel_groups::Override{group,count});}
                            dirty=true;
                        }
                        if custom { if ui.add(egui::DragValue::new(&mut count).range(1..=255)).changed() {
                            value.overrides.retain(|item|item.group!=group);
                            value.overrides.push(crate::pixel_groups::Override{group,count}); dirty=true;
                        }} else {ui.weak(format!("{} px",value.default_count));}
                    });
                });
            }
        });
    if dialog::action_button(ui,icons::ARROW_COUNTER_CLOCKWISE,"Use default for all").clicked() {
        dirty|=!value.overrides.is_empty(); value.overrides.clear();
    }
    if dirty { value.overrides.sort_by_key(|item|item.group); }
    dirty
}

/// Edits only a local revision-bound draft. No device write occurs on paint.
pub fn correction_rules(ui: &mut egui::Ui, value: &mut pixel_corrections::Configuration, rule_toggle: bool,
    preview:Option<&pixel_corrections::Configuration>,cursor:Option<pixel_corrections::DiagnosticCursor>) -> bool {
    let mut dirty = false;
    ui.scope(|ui| {
        // Narrow dock projections can inherit Extend from the surrounding tab.
        // Keep card headings, explanatory labels and action captions bounded.
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
        dialog::section(ui, icons::SLIDERS_HORIZONTAL, "Correction rules", |ui| {
            dirty |= ui
                .checkbox(&mut value.enabled, super::i18n::text("Apply corrections"))
                .changed();
            ui.label(
                egui::RichText::new("Ordered rules · 1-based pixels · raw scene preserved")
                    .small()
                    .weak(),
            );
            if value.rules.is_empty() {
                ui.add_space(6.0);
                ui.weak("No wire corrections. Pixel order is unchanged.");
            }
            let mut remove = None;
            let mut reorder = None;
            let count = value.rules.len();
            let cursor_slots:Vec<_>=(0..=count).map(|index|cursor.map(|cursor|value.wire_for_cursor(cursor,index))).collect();
            for (index, rule) in value.rules.iter_mut().enumerate() {
                ui.add_space(8.0);
                ui.separator();
                ui.push_id(("wire-rule", index), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        let toggle=ui.add_enabled(rule_toggle,egui::Checkbox::new(&mut rule.enabled,
                            egui::RichText::new(format!("Rule {}",index+1)).strong()));
                        dirty|=toggle.changed();
                        toggle.on_hover_text(super::i18n::text(if rule_toggle {"Enable rule"} else {"Per-rule enable is not supported by this board"}));
                        if square_action(ui, icons::ARROW_UP, "Move rule up", index > 0).clicked() {
                            reorder = Some((index, index - 1));
                        }
                        if square_action(ui, icons::ARROW_DOWN, "Move rule down", index + 1 < count)
                            .clicked()
                        {
                            reorder = Some((index, index + 1));
                        }
                        if square_action(ui, icons::TRASH, "Remove rule", true).clicked() {
                            remove = Some(index);
                        }
                    });
                    ComboBox::from_id_salt("kind")
                        .width(ui.available_width().min(250.0))
                        .selected_text(kind_label(&rule.kind))
                        .show_ui(ui, |ui| {
                            for kind in ["black", "remove", "shift"] {
                                if ui
                                    .dropdown_value(
                                        &mut rule.kind,
                                        kind.to_owned(),
                                        kind_label(kind),
                                    )
                                    .changed()
                                {
                                    rule.offset = 0;
                                    dirty = true;
                                }
                            }
                        });
                    ui.horizontal_wrapped(|ui| {
                        ui.label(super::i18n::text("Pixel"));
                        if square_action(ui,icons::MINUS,"Previous pixel",rule.pixel>0 && value.pixels>0).clicked() {
                            dirty|=rule.step_pixel(-1,value.pixels);
                        }
                        let mut pixel = rule.human_pixel();
                        if ui
                            .add(egui::DragValue::new(&mut pixel).range(1..=value.pixels.max(1)))
                            .changed()
                        {
                            dirty |= rule.set_human_pixel(pixel, value.pixels);
                        }
                        if square_action(ui,icons::PLUS,"Next pixel",value.pixels>0 && rule.pixel<value.pixels-1).clicked() {
                            dirty|=rule.step_pixel(1,value.pixels);
                        }
                    });
                    if let Some(slot)=&cursor_slots[index] {
                        let response=ui.add_enabled_ui(slot.is_ok(),|ui|dialog::action_button(ui,icons::TARGET,"Use diagnostic cursor")).inner;
                        if response.clicked() && let Ok(wire)=slot {
                            dirty|=rule.pixel!=*wire;rule.pixel=*wire;
                        }
                        response.on_hover_text(match slot {Ok(wire)=>format!("Output {} after preceding rules",wire+1),Err(error)=>error.clone()});
                    }
                    if rule.kind == "shift" {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(super::i18n::text("Offset"));
                            let bound = value.pixels.min(i16::MAX as u16) as i16;
                            if square_action(ui,icons::MINUS,"Decrease offset",rule.offset>-bound && bound>0).clicked() {
                                dirty|=rule.step_offset(-1,value.pixels);
                            }
                            dirty |= ui
                                .add(egui::DragValue::new(&mut rule.offset).range(-bound..=bound))
                                .changed();
                            if square_action(ui,icons::PLUS,"Increase offset",rule.offset<bound && bound>0).clicked() {
                                dirty|=rule.step_offset(1,value.pixels);
                            }
                        });
                    }
                });
            }
            if let Some(index) = remove {
                value.rules.remove(index);
                dirty = true;
            } else if let Some((from, to)) = reorder {
                value.rules.swap(from, to);
                dirty = true;
            }
            ui.add_space(8.0);
            if ui
                .add_enabled_ui(value.rules.len() < 32, |ui| {
                    dialog::action_button(ui, icons::PLUS, "Add rule")
                })
                .inner
                .clicked()
            {
                value.rules.push(pixel_corrections::Rule {
                    kind: "black".into(),
                    pixel: 0,
                    offset: 0,
                    enabled: true,
                });
                dirty = true;
            }
            if let Some(slot)=&cursor_slots[count] {
                let response=ui.add_enabled_ui(value.rules.len()<32 && slot.is_ok(),|ui|
                    dialog::action_button(ui,icons::TARGET,"Add at diagnostic cursor")).inner;
                if response.clicked() && let Ok(wire)=slot {
                    value.rules.push(pixel_corrections::Rule{kind:"black".into(),pixel:*wire,offset:0,enabled:true});dirty=true;
                }
                response.on_hover_text(match slot {Ok(wire)=>format!("Raw {} → output {}",cursor.unwrap().raw+1,wire+1),Err(error)=>error.clone()});
            }
        });
        ui.add_space(8.0);
        dialog::section(ui, icons::LIST_CHECKS, if preview.is_some() {"Live wire preview"} else {"Applied wire mapping"}, |ui| {
            detail(
                ui,
                icons::CHECK,
                "Saved revision",
                &value.revision.to_string(),
            );
            let sources=preview.map(|actual|actual.active_sources.as_slice()).unwrap_or(&value.sources);
            if sources.is_empty() {
                ui.weak("Mapping has not been returned by the board.");
            } else {
                ui.weak("Wire pixel ← raw source");
                let row_height = ui.text_style_height(&egui::TextStyle::Body).max(22.0);
                egui::ScrollArea::vertical()
                    .id_salt("wire-sources")
                    .max_height(220.0)
                    .auto_shrink([false, true])
                    .show_rows(ui, row_height, sources.len(), |ui, rows| {
                        for wire in rows {
                            let source = sources[wire];
                            ui.monospace(format!(
                                "{:>3}  ←  {}",
                                wire + 1,
                                if source < 0 {
                                    "Black".into()
                                } else {
                                    (i32::from(source) + 1).to_string()
                                }
                            ));
                        }
                    });
            }
        });
    });
    dirty
}

fn service(ui: &mut egui::Ui, name: &str, active: bool, inactive: &str) {
    let color = if active {
        ui.visuals().selection.bg_fill
    } else {
        ui.visuals().weak_text_color()
    };
    ui.horizontal_wrapped(|ui| {
        ui.colored_label(color, if active { icons::CHECK } else { icons::MINUS });
        ui.label(name);
        ui.weak(if active { "Active" } else { inactive });
    });
}

/// Settings navigation only. Service state remains the board's observation.
pub fn network(ui: &mut egui::Ui, status: &DeviceStatus) -> Option<&'static str> {
    let mut open = None;
    ui.scope(|ui| {
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
        dialog::section(ui, icons::PLUG, "Network interfaces", |ui| {
            detail(ui, icons::GLOBE, "Hostname", &status.hostname);
            if status.station_enabled || status.station_connected {
                ui.add_space(6.0);
                service(
                    ui,
                    "Wi-Fi station",
                    status.station_connected,
                    "Disconnected",
                );
                detail(ui, icons::RADIO, "Network", &status.station_ssid);
                if status.station_connected {
                    detail(ui, icons::LINK, "Address", &status.station_ip);
                    detail(
                        ui,
                        icons::GAUGE,
                        "Signal",
                        &format!("{} dBm", status.station_rssi),
                    );
                }
            }
            if status.ap_enabled || status.ap_started {
                ui.add_space(6.0);
                service(ui, "Access point", status.ap_started, "Stopped");
                detail(ui, icons::LINK, "AP address", &status.ap_ip);
            }
            if let Some(enabled) = status.ethernet["enabled"].as_bool() {
                ui.add_space(6.0);
                service(
                    ui,
                    "Ethernet",
                    status.ethernet["link_up"].as_bool() == Some(true),
                    if enabled { "No link" } else { "Disabled" },
                );
                if let Some(ip) = status.ethernet["ip"].as_str() {
                    detail(ui, icons::LINK, "Address", ip);
                }
            }
            ui.add_space(8.0);
            if dialog::action_button(ui, icons::GEAR, "Interface settings…").clicked() {
                open = Some("wifi");
            }
        });
        ui.add_space(8.0);
        dialog::section(ui, icons::BROADCAST, "Discovery & time", |ui| {
            service(ui, "mDNS / DNS-SD", status.mdns_started, "Stopped");
            service(ui, "SSDP", status.ssdp_started, "Stopped");
            service(ui, "NetBIOS", status.netbios_started, "Stopped");
            service(ui, "Network time", status.ntp_synced, "Not synchronized");
            if !status.canonical_url.is_empty() {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "Open board web app",
                    &status.canonical_url,
                ));
            }
            if dialog::action_button(ui, icons::PENCIL_SIMPLE, "Board identity…").clicked() {
                open = Some("identity");
            }
        });
        ui.add_space(8.0);
        dialog::section(ui, icons::GLOBE, "Remote connections", |ui| {
            service(
                ui,
                "MQTT",
                status.mqtt_connected,
                if status.mqtt_enabled {
                    "Disconnected"
                } else {
                    "Disabled"
                },
            );
            service(
                ui,
                "WebSocket client",
                status.cloud_websocket_connected,
                if status.cloud_websocket_enabled {
                    "Disconnected"
                } else {
                    "Disabled"
                },
            );
            detail(
                ui,
                icons::LINK,
                "Local WebSocket clients",
                &status.websocket_clients.to_string(),
            );
            if !status.cloud_error.is_empty() {
                ui.colored_label(ui.visuals().error_fg_color, &status.cloud_error);
            }
            action_row(
                ui,
                &[
                    (icons::BROADCAST, "MQTT settings…"),
                    (icons::LINK, "WebSocket settings…"),
                ],
                |ui| {
                    if dialog::action_button(ui, icons::BROADCAST, "MQTT settings…").clicked() {
                        open = Some("mqtt");
                    }
                    if dialog::action_button(ui, icons::LINK, "WebSocket settings…").clicked() {
                        open = Some("websocket");
                    }
                },
            );
        });
    });
    open
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detail_keys_share_left_edge_and_values_keep_their_column() {
        let labels = ["Firmware", "Host", "AVR firmware", "UART", "UART errors", "Role", "LED driver"];
        for dark in [false, true] {
            for pixels_per_point in [1.0, 1.25, 1.5, 2.0] {
                for width in [260.0, 600.0] {
                    let ctx = egui::Context::default();
                    ctx.set_visuals(if dark { egui::Visuals::dark() } else { egui::Visuals::light() });
                    ctx.set_pixels_per_point(pixels_per_point);
                    let mut left = 0.0;
                    let mut output = ctx.run_ui(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(
                                egui::Pos2::ZERO, egui::vec2(width, 800.0))),
                            ..Default::default()
                        },
                        |ui| {
                            // Constrain the logical dock width explicitly: a
                            // first-frame DPI change can resize the root viewport.
                            ui.set_width(width);
                            dialog::section(ui, icons::CPU, "Controllers", |ui| {
                                left = ui.cursor().left();
                                for (index, label) in labels.iter().enumerate() {
                                    detail(ui, icons::CPU, label, &format!("value-{index}"));
                                }
                            });
                        },
                    );
                    output.textures_delta.clear();
                    let keys: Vec<_> = output.shapes.iter().filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) if labels.iter().any(|label|
                            text.galley.text() == format!("{}  {}", icons::CPU, super::super::i18n::text(label))) => Some(text),
                        _ => None,
                    }).collect();
                    assert_eq!(keys.len(), labels.len());
                    for key in keys {
                        assert!((key.pos.x - left).abs() < 0.5,
                            "key {:?} starts at {}, not {left}; width={width}, scale={pixels_per_point}, dark={dark}",
                            key.galley.text(), key.pos.x);
                    }
                    let values: Vec<_> = output.shapes.iter().filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) if text.galley.text().starts_with("value-") => Some(text),
                        _ => None,
                    }).collect();
                    assert_eq!(values.len(), labels.len());
                    let value_left = values[0].pos.x;
                    assert!(values.iter().all(|value| (value.pos.x - value_left).abs() < 0.5));
                    if width > 330.0 {
                        assert!(value_left >= left + 132.0);
                    } else {
                        assert!((value_left - left).abs() < 0.5,
                            "stacked value starts at {value_left}, not {left}; width={width}, scale={pixels_per_point}, dark={dark}");
                    }
                }
            }
        }
    }

    #[test]
    fn feature_chips_wrap_as_whole_slots_without_vertical_labels_or_overflow() {
        let features: Vec<String> = [
            "buzzer",
            "display",
            "game-press",
            "inputs",
            "key-settings",
            "leds",
            "network",
            "outputs",
            "pixel-corrections",
            "pixel-events",
            "state-presentation",
            "text",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        for width in [220.0, 300.0, 600.0] {
            let ctx = egui::Context::default();
            let mut extent = 0.0;
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, 800.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    dialog::section(ui, icons::LIST_CHECKS, "Acknowledged hardware", |ui| {
                        feature_chips(ui, &features)
                    });
                    extent = ui.min_rect().width();
                },
            );
            output.textures_delta.clear();
            assert!(
                extent <= width + 1.0,
                "chip row overflow {extent} > {width}"
            );
            let chips: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text)
                        if features.iter().any(|feature| feature == text.galley.text()) =>
                    {
                        Some(text)
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(chips.len(), features.len());
            assert!(chips.iter().all(|text| text.galley.rows.len() == 1));
            assert!(
                chips
                    .iter()
                    .all(|text| text.pos.x >= 0.0 && text.pos.x + text.galley.size().x <= width)
            );
        }
    }
    #[test]
    fn scoped_action_rows_fit_narrow_cards_and_keep_wide_rows_compact() {
        for actions in [
            vec![(icons::PLAY, "Present"), (icons::STOP_CIRCLE, "Stop text")],
            vec![
                (icons::ARROW_CLOCKWISE, "Read from board"),
                (icons::FLOPPY_DISK, "Save to board"),
                (icons::ARROW_COUNTER_CLOCKWISE, "Discard edits / reload"),
            ],
        ] {
            for width in [220.0, 600.0] {
                let ctx = egui::Context::default();
                let mut rects = Vec::new();
                let mut extent = 0.0;
                let mut output = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 800.0),
                        )),
                        ..Default::default()
                    },
                    |ui| {
                        dialog::section(ui, icons::GEAR, "Actions", |ui| {
                            action_row(ui, &actions, |ui| {
                                for (index, (icon, label)) in actions.iter().enumerate() {
                                    let response = ui
                                        .add_enabled_ui(index == 0, |ui| {
                                            if index == 0 {
                                                dialog::primary_action_button(ui, icon, label)
                                            } else {
                                                dialog::action_button(ui, icon, label)
                                            }
                                        })
                                        .inner;
                                    assert!(!response.clicked());
                                    rects.push(response.rect);
                                }
                            });
                        });
                        extent = ui.min_rect().width();
                    },
                );
                output.textures_delta.clear();
                assert!(
                    extent <= width + 1.0,
                    "action row overflow {extent} > {width}"
                );
                if width == 220.0 {
                    assert!(
                        rects
                            .windows(2)
                            .all(|pair| pair[0].bottom() <= pair[1].top())
                    );
                } else {
                    assert!(rects.windows(2).all(|pair| pair[0].top() == pair[1].top()));
                }
            }
        }
    }
    #[test]
    fn ethernet_uses_the_board_link_up_observation() {
        for (ethernet, expected) in [
            (serde_json::json!({"enabled":true,"link_up":true}), true),
            (
                serde_json::json!({"enabled":true,"link_up":false,"link":true}),
                false,
            ),
        ] {
            let ctx = egui::Context::default();
            let status = DeviceStatus {
                ethernet,
                ..Default::default()
            };
            let mut output = ctx.run_ui(Default::default(), |ui| {
                assert!(network(ui, &status).is_none());
            });
            output.textures_delta.clear();
            assert_eq!(output.shapes.iter().any(|shape|matches!(&shape.shape,egui::Shape::Text(text) if text.galley.text()=="Active")),expected);
        }
    }
    #[test]
    fn network_and_corrections_stay_bounded_and_passive_in_narrow_panels() {
        for width in [220.0, 300.0, 600.0] {
            for dark in [false, true] {
                let ctx = egui::Context::default();
                ctx.set_visuals(if dark {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                });
                let status: DeviceStatus = serde_json::from_value(serde_json::json!({
                    "hostname":"controller-with-a-long-hostname.local", "station_enabled":true,
                    "station_connected":true,"station_ssid":"Network with long descriptive name",
                    "station_ip":"192.168.100.123","ap_enabled":true,"ap_ip":"192.168.4.1"
                }))
                .unwrap();
                let mut value = pixel_corrections::Configuration {
                    revision: 2,
                    enabled: true,
                    rules: vec![pixel_corrections::Rule {
                        kind: "shift".into(),
                        pixel: 479,
                        offset: -2,
                        enabled: false,
                    }],
                    pixels: 480,
                    sources: (0..480).collect(),
                    ..Default::default()
                };
                let before = serde_json::to_value(&value).unwrap();
                for corrections in [false, true] {
                    let mut extent = 0.0;
                    let mut intent = None;
                    let mut dirty = false;
                    let mut output = ctx.run_ui(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(
                                egui::Pos2::ZERO,
                                egui::vec2(width, 1600.0),
                            )),
                            ..Default::default()
                        },
                        |ui| {
                            if corrections {
                                dirty = correction_rules(ui, &mut value, true,None,None);
                            } else {
                                intent = network(ui, &status);
                            }
                            extent = ui.min_rect().width();
                        },
                    );
                    output.textures_delta.clear();
                    assert!(
                        extent <= width + 1.0,
                        "panel width {extent} exceeds {width}, corrections={corrections}; text={:?}",
                        output
                            .shapes
                            .iter()
                            .filter_map(|shape| match &shape.shape {
                                egui::Shape::Text(text) =>
                                    Some((text.galley.text(), text.pos.x, text.galley.size().x)),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                    );
                    assert!(!dirty);
                    assert!(intent.is_none());
                }
                assert_eq!(serde_json::to_value(&value).unwrap(), before);
            }
        }
    }
    #[test]
    fn pixel_corrections_toggle_is_capability_gated_and_changes_only_local_rule() {
        for supported in [false,true] {
            let ctx=egui::Context::default();
            let mut value=pixel_corrections::Configuration {revision:9,enabled:true,pixels:480,sources:vec![0,-1,2],
                rules:vec![pixel_corrections::Rule{kind:"shift".into(),pixel:7,offset:-2,enabled:false}],..Default::default()};
            let render=|value:&mut pixel_corrections::Configuration,events| {
                let mut changed=false;
                let mut out=ctx.run_ui(egui::RawInput{events,
                    screen_rect:Some(egui::Rect::from_min_size(egui::Pos2::ZERO,egui::vec2(300.0,900.0))),..Default::default()},|ui| {
                    changed=correction_rules(ui,value,supported,None,None);
                });
                let point=out.shapes.iter().find_map(|shape|match &shape.shape {
                    egui::Shape::Text(text) if text.galley.text()=="Rule 1"=>Some(text.pos+egui::vec2(4.0,4.0)),_=>None
                }).expect("Rule checkbox has an accessible visible label");
                out.textures_delta.clear();(point,changed)
            };
            let (point,changed)=render(&mut value,vec![]);assert!(!changed);
            render(&mut value,vec![egui::Event::PointerMoved(point),egui::Event::PointerButton{
                pos:point,button:egui::PointerButton::Primary,pressed:true,modifiers:egui::Modifiers::NONE}]);
            let (_,changed)=render(&mut value,vec![egui::Event::PointerButton{
                pos:point,button:egui::PointerButton::Primary,pressed:false,modifiers:egui::Modifiers::NONE}]);
            assert_eq!(changed,supported);assert_eq!(value.rules[0].enabled,supported);
            assert_eq!(value.rules[0].pixel,7);assert_eq!(value.rules[0].offset,-2);
            assert_eq!(value.revision,9);assert_eq!(value.sources,vec![0,-1,2]);
        }
    }
    #[test]
    fn pixel_groups_editor_remains_bounded_and_never_saves_on_paint() {
        for width in [240.0,330.0,720.0] {
            let ctx=egui::Context::default();
            let mut value=crate::pixel_groups::Layout::uniform(480,6).unwrap();
            let before=value.clone();let mut extent=0.0;
            let mut output=ctx.run_ui(egui::RawInput{screen_rect:Some(egui::Rect::from_min_size(egui::Pos2::ZERO,egui::vec2(width,700.0))),..Default::default()},|ui|{
                assert!(!physical_groups(ui,&mut value,&[]));extent=ui.min_rect().width();
            });output.textures_delta.clear();
            assert_eq!(value,before);assert!(extent<=width+1.0,"physical group editor overflow: {extent}/{width}");
        }
    }
}
