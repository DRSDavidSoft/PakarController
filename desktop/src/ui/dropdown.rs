//! Imported from actual Pealayer src/ui/dropdown.rs; all generic behavior and tests retained.
//! Shared native dropdown focus, full-width choices and stable color lanes.
use eframe::egui;

#[derive(Clone)]
struct Entry {
    id: egui::Id,
    selected: bool,
    enabled: bool,
}
#[derive(Clone)]
struct MenuFrame {
    layer: egui::LayerId,
    entries: Vec<Entry>,
    target: Option<egui::Id>,
    activate: Option<egui::Id>,
    search: Option<egui::Id>,
    ready: bool,
}
fn active_id() -> egui::Id {
    egui::Id::new("pakar-active-dropdown")
}

/// Keep egui's IDs, popup behavior and existing API; add an explicit focus owner.
pub struct ComboBox {
    inner: egui::ComboBox,
    salt: egui::IdSalt,
}
impl ComboBox {
    /// Preserve the locked egui from_label identity for existing labeled controls.
    pub fn from_label(label: impl Into<egui::WidgetText>) -> Self {
        let label = label.into();
        let salt = egui::IdSalt::new(label.text());
        Self {
            inner: egui::ComboBox::from_label(label).icon(|ui, rect, visuals, _| {
                crate::ui::control_density::paint_chevron(ui, rect, visuals.text_color());
            }),
            salt,
        }
    }
    pub fn from_id_salt(salt: impl egui::AsIdSalt) -> Self {
        let hashed_salt = egui::IdSalt::new(&salt);
        Self {
            inner: egui::ComboBox::from_id_salt(salt).icon(|ui, rect, visuals, _| {
                crate::ui::control_density::paint_chevron(ui, rect, visuals.text_color());
            }),
            salt: hashed_salt,
        }
    }
    pub fn width(mut self, width: f32) -> Self {
        self.inner = self.inner.width(width);
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.inner = self.inner.height(height);
        self
    }
    pub fn selected_text(mut self, text: impl Into<egui::WidgetText>) -> Self {
        self.inner = self.inner.selected_text(text);
        self
    }
    pub fn truncate(mut self) -> Self {
        self.inner = self.inner.truncate();
        self
    }
    pub fn wrap_mode(mut self, mode: egui::TextWrapMode) -> Self {
        self.inner = self.inner.wrap_mode(mode);
        self
    }
    pub fn close_behavior(mut self, behavior: egui::PopupCloseBehavior) -> Self {
        self.inner = self.inner.close_behavior(behavior);
        self
    }
    pub fn icon(
        mut self,
        icon: impl FnOnce(&egui::Ui, egui::Rect, &egui::style::WidgetVisuals, bool) + 'static,
    ) -> Self {
        self.inner = self.inner.icon(icon);
        self
    }
    pub fn is_open(ctx: &egui::Context, id: egui::Id) -> bool {
        egui::ComboBox::is_open(ctx, id)
    }

    pub fn show_ui<R>(
        self,
        ui: &mut egui::Ui,
        body: impl FnOnce(&mut egui::Ui) -> R,
    ) -> egui::InnerResponse<Option<R>> {
        let button = ui.make_persistent_id(self.salt);
        let previous_height = ui.spacing().interact_size.y;
        let height = crate::ui::control_density::height(ui);
        if height != previous_height {
            ui.spacing_mut().interact_size.y = height;
        }
        let previous_body = ui.style().text_styles.get(&egui::TextStyle::Body).cloned();
        let button_font = egui::TextStyle::Button.resolve(ui.style());
        ui.style_mut().text_styles.insert(egui::TextStyle::Body, button_font);
        let result = self
            .inner
            .popup_style(egui::style::StyleModifier::new(dropdown_style))
            .show_ui(ui, |ui| menu_ui(ui, button, body));
        if let Some(font) = previous_body {
            ui.style_mut().text_styles.insert(egui::TextStyle::Body, font);
        }
        if height != previous_height {
            ui.spacing_mut().interact_size.y = previous_height;
        }
        if result.inner.is_none() {
            clear_menu(ui.ctx(), button);
        }
        result
    }
}

fn dropdown_style(style: &mut egui::Style) {
    style.interaction.selectable_labels = false;
    for widget in [
        &mut style.visuals.widgets.inactive,
        &mut style.visuals.widgets.hovered,
        &mut style.visuals.widgets.active,
        &mut style.visuals.widgets.open,
    ] {
        widget.expansion = 0.0;
    }
}

/// Also shared by compact selectors whose trigger is an icon rather than a ComboBox.
pub fn menu_ui<R>(ui: &mut egui::Ui, button: egui::Id, body: impl FnOnce(&mut egui::Ui) -> R) -> R {
    dropdown_style(ui.style_mut());
    let ctx = ui.ctx().clone();
    let state_id = button.with("dropdown-entries");
    let previous = ctx.data(|data| data.get_temp::<MenuFrame>(state_id));
    let focused = ctx.memory(|memory| memory.focused());
    let owns_focus = focused.is_none()
        || focused == Some(button)
        || focused
            .and_then(|id| ctx.read_response(id))
            .is_some_and(|response| response.layer_id == ui.layer_id());
    let entries: Vec<_> = previous
        .as_ref()
        .map(|frame| {
            frame
                .entries
                .iter()
                .filter(|entry| entry.enabled)
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    let mut target = None;
    let mut activate = None;
    let navigating = focused.is_none()
        || focused == Some(button)
        || entries.iter().any(|entry| Some(entry.id) == focused)
        || previous
            .as_ref()
            .and_then(|frame| frame.search)
            .is_some_and(|search| Some(search) == focused);
    if owns_focus && navigating && !entries.is_empty() {
        let focused_choice = entries.iter().position(|entry| Some(entry.id) == focused);
        let current = focused_choice
            .or_else(|| entries.iter().position(|entry| entry.selected))
            .unwrap_or(0);
        let (down, up, home, end, enter) = ctx.input_mut(|input| {
            (
                input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown),
                input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp),
                entries.iter().any(|entry| Some(entry.id) == focused)
                    && input.consume_key(egui::Modifiers::NONE, egui::Key::Home),
                entries.iter().any(|entry| Some(entry.id) == focused)
                    && input.consume_key(egui::Modifiers::NONE, egui::Key::End),
                entries.iter().any(|entry| Some(entry.id) == focused)
                    && input.consume_key(egui::Modifiers::NONE, egui::Key::Enter),
            )
        });
        let next = if home {
            0
        } else if end {
            entries.len() - 1
        } else if down && focused_choice.is_none() {
            current
        } else if down {
            (current + 1) % entries.len()
        } else if up && focused_choice.is_none() {
            entries.len() - 1
        } else if up {
            (current + entries.len() - 1) % entries.len()
        } else {
            current
        };
        if down || up || home || end {
            target = Some(entries[next].id);
            ctx.memory_mut(|memory| {
                memory.move_focus(egui::FocusDirection::None);
                memory.request_focus(entries[next].id);
            });
        }
        if enter {
            activate = Some(entries[next].id);
        }
    }
    let parent = ctx.data(|data| data.get_temp::<MenuFrame>(active_id()));
    let layer = ui.layer_id();
    let ready = ui.is_enabled();
    ctx.data_mut(|data| {
        data.insert_temp(active_id(), layer);
        data.insert_temp(
            active_id(),
            MenuFrame {
                layer: ui.layer_id(),
                entries: Vec::new(),
                target,
                activate,
                search: None,
                ready,
            },
        )
    });
    ui.spacing_mut().item_spacing.y = 2.0;
    let value = body(ui);
    let frame = ctx.data_mut(|data| {
        let frame = data
            .get_temp::<MenuFrame>(active_id())
            .expect("dropdown frame");
        data.remove::<MenuFrame>(active_id());
        data.remove::<egui::LayerId>(active_id());
        if let Some(parent) = parent {
            data.insert_temp(active_id(), parent.layer);
            data.insert_temp(active_id(), parent);
        }
        data.insert_temp(state_id, frame.clone());
        frame
    });
    if previous.as_ref().is_none_or(|frame| !frame.ready) {
        // Respect a search field's deliberate initial focus, otherwise
        // seed the current choice instead of leaving focus on the trigger.
        let focused = ctx.memory(|memory| memory.focused());
        let popup_focus = focused
            .and_then(|id| ctx.read_response(id))
            .is_some_and(|response| response.layer_id == ui.layer_id());
        if ui.is_enabled()
            && let Some(search) = frame.search
        {
            ctx.memory_mut(|memory| memory.request_focus(search));
            ctx.request_repaint();
        } else if !popup_focus
            && let Some(entry) = frame
                .entries
                .iter()
                .find(|entry| entry.enabled && entry.selected)
                .or_else(|| frame.entries.iter().find(|entry| entry.enabled))
        {
            ctx.memory_mut(|memory| memory.request_focus(entry.id));
            ctx.request_repaint();
        }
    }
    value
}

pub fn clear_menu(ctx: &egui::Context, owner: egui::Id) {
    ctx.data_mut(|data| data.remove::<MenuFrame>(owner.with("dropdown-entries")));
}

/// Register custom choice surfaces too (e.g. recording colors and icon tiles).
pub fn register_choice(
    ui: &mut egui::Ui,
    mut response: egui::Response,
    selected: bool,
) -> egui::Response {
    let registered = with_active_frame(ui, |frame| {
        frame.entries.push(Entry {
            id: response.id,
            selected,
            enabled: response.enabled(),
        });
        (
            frame.activate == Some(response.id),
            frame.target == Some(response.id),
        )
    });
    if let Some((activate, reveal)) = registered {
        if response.enabled() && activate {
            response
                .flags
                .insert(egui::response::Flags::FAKE_PRIMARY_CLICKED);
        }
        if reveal {
            response.scroll_to_me(Some(egui::Align::Center));
        }
    }
    response
}

fn with_active_frame<R>(ui: &mut egui::Ui, action: impl FnOnce(&mut MenuFrame) -> R) -> Option<R> {
    let layer = ui.layer_id();
    ui.data_mut(|data| {
        if data.get_temp::<egui::LayerId>(active_id()) != Some(layer) {
            return None;
        }
        let frame = data.get_temp_mut_or_insert_with::<MenuFrame>(active_id(), || {
            unreachable!("active menu frame")
        });
        Some(action(frame))
    })
}

/// Search is explicitly retained across egui's disabled popup sizing pass.
pub fn register_search(ui: &mut egui::Ui, response: &egui::Response) {
    if response.enabled() {
        with_active_frame(ui, |frame| frame.search = Some(response.id));
    }
}

pub fn paint_swatch(ui: &egui::Ui, rect: egui::Rect, color: egui::Color32) {
    let center = egui::pos2(rect.left() + 13.0, rect.center().y);
    ui.painter().circle_filled(center, 5.0, color);
    ui.painter()
        .circle_stroke(center, 5.0, ui.visuals().widgets.noninteractive.bg_stroke);
}

pub fn color_label(ui: &egui::Ui, text: &str) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    job.append(
        text,
        24.0,
        egui::TextFormat::simple(
            egui::TextStyle::Button.resolve(ui.style()),
            ui.visuals().text_color(),
        ),
    );
    job
}

pub fn choice(
    ui: &mut egui::Ui,
    selected: bool,
    text: impl Into<egui::WidgetText>,
    color: Option<egui::Color32>,
) -> egui::Response {
    choice_with_close(ui, selected, text, color, true)
}

/// Expansion rows own content inside the current popup and therefore must be
/// able to keep that popup open while the inline editor is active.
pub fn choice_with_close(
    ui: &mut egui::Ui,
    selected: bool,
    text: impl Into<egui::WidgetText>,
    color: Option<egui::Color32>,
    close_on_click: bool,
) -> egui::Response {
    let text = text.into();
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(width, crate::ui::control_density::height(ui)),
        egui::Sense::click(),
    );
    let response = register_choice(ui, response, selected);
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            text.text(),
        )
    });
    if response.clicked() && close_on_click {
        ui.close();
    }
    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, selected);
        ui.painter().rect(
            rect,
            4.0,
            visuals.weak_bg_fill,
            if response.has_focus() {
                ui.visuals().selection.stroke
            } else {
                egui::Stroke::NONE
            },
            egui::StrokeKind::Inside,
        );
        let left = rect.left() + if color.is_some() { 27.0 } else { 8.0 };
        let galley = text.into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            (rect.right() - left - 8.0).max(1.0),
            egui::TextStyle::Button,
        );
        ui.painter().with_clip_rect(rect.shrink(1.0)).galley(
            egui::pos2(left, rect.center().y - galley.size().y / 2.0),
            galley,
            visuals.text_color(),
        );
        if let Some(color) = color {
            paint_swatch(ui, rect, color);
        }
    }
    response
}

/// Outside dropdowns retain normal selectable widget behavior.
pub trait DropdownUiExt {
    fn dropdown_choice(
        &mut self,
        selected: bool,
        text: impl Into<egui::WidgetText>,
    ) -> egui::Response;
    fn dropdown_value<T: PartialEq>(
        &mut self,
        current: &mut T,
        selected: T,
        text: impl Into<egui::WidgetText>,
    ) -> egui::Response;
}
impl DropdownUiExt for egui::Ui {
    fn dropdown_choice(
        &mut self,
        selected: bool,
        text: impl Into<egui::WidgetText>,
    ) -> egui::Response {
        if self
            .data(|data| data.get_temp::<egui::LayerId>(active_id()))
            .is_some_and(|layer| layer == self.layer_id())
        {
            choice(self, selected, text, None)
        } else {
            self.selectable_label(selected, text)
        }
    }
    fn dropdown_value<T: PartialEq>(
        &mut self,
        current: &mut T,
        selected: T,
        text: impl Into<egui::WidgetText>,
    ) -> egui::Response {
        let mut response = self.dropdown_choice(*current == selected, text);
        if response.clicked() && *current != selected {
            *current = selected;
            response.mark_changed();
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_label_keeps_locked_egui_trigger_identity() {
        let ctx = egui::Context::default();
        let mut expected = egui::Id::NULL;
        let mut actual = egui::Id::NULL;
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            expected = ui.make_persistent_id(egui::IdSalt::new("Port"));
            actual = ComboBox::from_label("Port")
                .selected_text("Disconnected")
                .show_ui(ui, |_| {})
                .response
                .id;
        });
        output.textures_delta.clear();
        assert_eq!(actual, expected);
    }
    fn key(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }
    }
    #[test]
    fn dropdown_arrows_seed_focus_skip_disabled_commit_enter_and_cancel_escape() {
        for dark in [false, true] {
            let ctx = egui::Context::default();
            ctx.set_visuals(if dark {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            });
            let mut selected = 0;
            let mut frame = |events| {
                let mut rows = Vec::new();
                let mut trigger = None;
                let mut output = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(500.0, 500.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |ui| {
                        let expected_id =
                            ui.make_persistent_id(egui::IdSalt::new("keyboard-dropdown"));
                        trigger = Some(
                            ComboBox::from_id_salt("keyboard-dropdown")
                                .width(240.0)
                                .selected_text("Choice")
                                .show_ui(ui, |ui| {
                                    for index in 0..4 {
                                        let response = ui
                                            .add_enabled_ui(index != 1, |ui| {
                                                ui.dropdown_value(
                                                    &mut selected,
                                                    index,
                                                    format!("Choice {index}"),
                                                )
                                            })
                                            .inner;
                                        rows.push(response);
                                    }
                                })
                                .response,
                        );
                        assert_eq!(
                            trigger.as_ref().unwrap().id,
                            expected_id,
                            "preserve egui's exact trigger identity"
                        );
                    },
                );
                output.textures_delta.clear();
                (trigger.unwrap(), rows, selected)
            };
            let (trigger, _, _) = frame(vec![]);
            egui::Popup::open_id(&ctx, trigger.id.with("popup"));
            frame(vec![]); // egui measures a new popup in a disabled sizing pass.
            let (_, rows, value) = frame(vec![]);
            assert_eq!(value, 0);
            assert_eq!(ctx.memory(|memory| memory.focused()), Some(rows[0].id));
            let (_, moved, _) = frame(vec![key(egui::Key::ArrowDown)]);
            assert_eq!(ctx.memory(|memory| memory.focused()), Some(moved[2].id));
            let (_, moved, _) = frame(vec![key(egui::Key::ArrowUp)]);
            assert_eq!(ctx.memory(|memory| memory.focused()), Some(moved[0].id));
            frame(vec![key(egui::Key::End)]);
            let (_, _, committed) = frame(vec![key(egui::Key::Enter)]);
            assert_eq!(committed, 3);
            assert!(!ComboBox::is_open(&ctx, trigger.id));
            frame(vec![]);
            egui::Popup::open_id(&ctx, trigger.id.with("popup"));
            frame(vec![]);
            frame(vec![key(egui::Key::ArrowUp)]);
            let (_, _, cancelled) = frame(vec![key(egui::Key::Escape)]);
            assert_eq!(cancelled, 3);
            assert!(!ComboBox::is_open(&ctx, trigger.id));
        }
    }

    #[test]
    fn dropdown_search_keeps_text_focus_until_arrow_enters_choices() {
        let ctx = egui::Context::default();
        let mut search = String::new();
        let mut value = 0;
        let mut frame = |events| {
            let mut trigger = None;
            let mut choices = Vec::new();
            let mut search_response = None;
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(500.0, 500.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| {
                    trigger = Some(
                        ComboBox::from_id_salt("search-dropdown")
                            .width(240.0)
                            .selected_text("Search")
                            .show_ui(ui, |ui| {
                                let edit = ui.text_edit_singleline(&mut search);
                                register_search(ui, &edit);
                                if search.is_empty() {
                                    edit.request_focus();
                                }
                                search_response = Some(edit);
                                for index in 0..3 {
                                    choices.push(ui.dropdown_value(
                                        &mut value,
                                        index,
                                        format!("Choice {index}"),
                                    ));
                                }
                            })
                            .response,
                    );
                },
            );
            output.textures_delta.clear();
            (
                trigger.unwrap(),
                search_response,
                choices,
                search.clone(),
                value,
            )
        };
        let (trigger, _, _, _, _) = frame(vec![]);
        egui::Popup::open_id(&ctx, trigger.id.with("popup"));
        frame(vec![]);
        let (_, edit, _, _, _) = frame(vec![egui::Event::Text("Choice".into())]);
        assert_eq!(
            ctx.memory(|memory| memory.focused()),
            Some(edit.unwrap().id)
        );
        let (_, _, choices, text, _) = frame(vec![key(egui::Key::ArrowDown)]);
        assert_eq!(text, "Choice");
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(choices[0].id));
        frame(vec![key(egui::Key::ArrowDown)]);
        let (_, _, _, _, value) = frame(vec![key(egui::Key::Enter)]);
        assert_eq!(value, 1);
    }

    #[test]
    fn dropdown_unregistered_editor_retains_arrow_keys() {
        let ctx = egui::Context::default();
        let mut draft = String::new();
        let mut value = 0;
        let mut frame = |events| {
            let mut trigger = None;
            let mut editor = None;
            let mut editor_saw_arrow = false;
            let mut output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(500.0, 500.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| {
                    trigger = Some(
                        ComboBox::from_id_salt("editor-dropdown")
                            .width(240.0)
                            .selected_text("Editor")
                            .show_ui(ui, |ui| {
                                editor_saw_arrow = ui.input_mut(|input| {
                                    input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
                                });
                                let edit = ui.text_edit_singleline(&mut draft);
                                if draft.is_empty() {
                                    edit.request_focus();
                                }
                                editor = Some(edit);
                                for index in 0..3 {
                                    ui.dropdown_value(&mut value, index, format!("Choice {index}"));
                                }
                            })
                            .response,
                    );
                },
            );
            output.textures_delta.clear();
            (trigger.unwrap(), editor, value, editor_saw_arrow)
        };
        let (trigger, _, _, _) = frame(vec![]);
        egui::Popup::open_id(&ctx, trigger.id.with("popup"));
        frame(vec![]);
        let (_, editor, before, _) = frame(vec![]);
        let editor = editor.expect("editor rendered in open popup");
        assert_eq!(ctx.memory(|memory| memory.focused()), Some(editor.id));
        let (_, _, after, editor_saw_arrow) = frame(vec![key(egui::Key::ArrowDown)]);
        assert_eq!(before, after);
        assert!(
            editor_saw_arrow,
            "dropdown must not steal the editor's arrow"
        );
    }

    #[test]
    fn dropdown_color_rows_are_full_width_equal_height_and_hover_stable() {
        for dark in [false, true] {
            for scale in [1.0, 1.25, 1.5, 2.0] {
                let ctx = egui::Context::default();
                ctx.set_pixels_per_point(scale);
                ctx.set_visuals(if dark {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                });
                let frame = |pointer| {
                    let mut rows = Vec::new();
                    let mut row_height = 0.0;
                    let mut output = ctx.run_ui(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(
                                egui::Pos2::ZERO,
                                egui::vec2(400.0, 300.0),
                            )),
                            events: vec![egui::Event::PointerMoved(pointer)],
                            ..Default::default()
                        },
                        |ui| {
                            ui.set_width(310.0);
                            row_height = crate::ui::control_density::height(ui);
                            rows.push(
                                choice(ui, true, "System accent", Some(egui::Color32::BLUE)).rect,
                            );
                            rows.push(
                                choice(ui, false, "Pealayer green", Some(egui::Color32::GREEN))
                                    .rect,
                            );
                            rows.push(
                                crate::ui::color_picker::custom_menu_row(
                                    ui,
                                    false,
                                    "Custom",
                                    "#0078D4",
                                    [0, 120, 212],
                                )
                                .rect,
                            );
                        },
                    );
                    // Headless tests own no renderer; release texture deltas
                    // before an assertion can panic (actual Pealayer lifecycle).
                    output.textures_delta.clear();
                    for row in &rows {
                        assert_eq!(row.width(), 310.0);
                        assert_eq!(row.height(), row_height);
                    }
                    let inset = output
                        .shapes
                        .iter()
                        .find_map(|shape| match &shape.shape {
                            egui::epaint::Shape::Rect(rect)
                                if (rect.rect.width() - 124.0).abs() < 0.1 =>
                            {
                                Some((shape.clip_rect, rect.rect))
                            }
                            _ => None,
                        })
                        .expect("custom HEX preview");
                    assert!(
                        inset.0.contains_rect(inset.1),
                        "inset stroke clips at scale {scale}"
                    );
                    assert!(rows[2].contains_rect(inset.1));
                    rows
                };
                let rest = frame(egui::pos2(390.0, 290.0));
                assert_eq!(rest, frame(rest[0].center()));
                assert_eq!(rest, frame(rest[2].center()));
            }
        }
    }
}
