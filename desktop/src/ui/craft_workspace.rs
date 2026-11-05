//! CraftUI chrome over the existing, capability-filtered egui Dock workspace.
//! Adapted from Pealayer ui/egui_dock.rs. No second layout, persistence writer,
//! panel implementation, device session or command authority is introduced.
use super::layout::{PakarTabViewer, Panel};
use crate::PakarControllerApp;
use craft_ui::{
    docking::{
        Action, DockArea, DockContent, DockStyle, Floating, Layout, Node, PanelLimits, Permissions,
        Placement, Zone,
    },
    layout::{SplitAxis, SplitSize},
};
use eframe::egui;
use egui_dock::{DockState, NodeIndex, Surface, TabInsert, TabViewer};

fn node(tree: &egui_dock::Tree<Panel>, index: NodeIndex) -> Option<Node<Panel>> {
    match &tree[index] {
        egui_dock::Node::Empty => None,
        egui_dock::Node::Leaf(leaf) => (!leaf.tabs.is_empty()).then(|| Node::Tabs {
            panels: leaf.tabs.clone(),
            active: leaf.active.0,
        }),
        egui_dock::Node::Horizontal(split) | egui_dock::Node::Vertical(split) => {
            match (node(tree, index.left()), node(tree, index.right())) {
                (Some(first), Some(second)) => Some(Node::Split {
                    axis: if matches!(tree[index], egui_dock::Node::Horizontal(_)) {
                        SplitAxis::Horizontal
                    } else {
                        SplitAxis::Vertical
                    },
                    size: SplitSize::Ratio(split.fraction),
                    first: Box::new(first),
                    second: Box::new(second),
                }),
                (node, None) | (None, node) => node,
            }
        }
    }
}

// WindowState's public rect() excludes pending position/size until egui Dock
// paints it. Read those values from its existing serde representation so a
// CraftUI drag persists and is also respected on returning to egui Dock.
// This is a pinned dependency adapter, not a new application storage schema.
fn window_rect(state: &egui_dock::WindowState) -> [f32; 4] {
    let value = serde_json::to_value(state).unwrap_or_default();
    let mut rect = state.rect();
    if !rect.is_finite() || !rect.is_positive() {
        rect = egui::Rect::from_min_size(egui::pos2(40.0, 40.0), egui::vec2(640.0, 480.0));
    }
    if let Ok(pos) = serde_json::from_value::<egui::Pos2>(value["next_position"].clone()) {
        if pos.is_finite() {
            rect = egui::Rect::from_min_size(pos, rect.size());
        }
    }
    if let Ok(size) = serde_json::from_value::<egui::Vec2>(value["next_size"].clone()) {
        if size.is_finite() && size.x > 0.0 && size.y > 0.0 {
            rect = egui::Rect::from_min_size(rect.min, size);
        }
    }
    [rect.min.x, rect.min.y, rect.width(), rect.height()]
}

fn project(dock: &DockState<Panel>) -> Layout<Panel> {
    let mut layout = Layout::default();
    for surface in dock.iter_surfaces() {
        match surface {
            Surface::Main(tree) if !tree.is_empty() => layout.root = node(tree, NodeIndex::root()),
            Surface::Window(tree, state) if !tree.is_empty() => {
                // CraftUI's egui 0.36 surface presents floating groups as tabs.
                // Keep the original split/collapse state; explicit actions below
                // target the original Panel identity instead of replacing its tree.
                let panels: Vec<_> = tree
                    .iter()
                    .flat_map(|node| node.iter_tabs().copied())
                    .collect();
                let active_panel = tree
                    .focused_leaf()
                    .and_then(|i| tree[i].get_leaf())
                    .and_then(|leaf| leaf.tabs.get(leaf.active.0));
                let active = active_panel
                    .and_then(|p| panels.iter().position(|v| v == p))
                    .unwrap_or(0);
                if !panels.is_empty() {
                    layout.floating.push(Floating {
                        panels,
                        active,
                        rect: window_rect(state),
                    });
                }
            }
            _ => {}
        }
    }
    layout
}

fn apply(dock: &mut DockState<Panel>, action: Action<Panel>) -> Result<(), String> {
    // Validate the same rendered snapshot before changing any original state.
    // Unknown/missing identities and invalid geometry never mutate the workspace.
    project(dock)
        .apply(action.clone())
        .map_err(|e| e.to_string())?;
    match action {
        Action::Activate { panel } => {
            let path = dock
                .find_tab(&panel)
                .ok_or("Panel is no longer available")?;
            dock.set_active_tab(path).map_err(|e| e.to_string())?;
            dock.set_focused_node_and_surface(path.node_path());
        }
        Action::Close { panel } => {
            let path = dock
                .find_tab(&panel)
                .ok_or("Panel is no longer available")?;
            dock.remove_tab(path);
        }
        Action::Move {
            panel,
            anchor,
            placement,
        } => {
            let src = dock
                .find_tab(&panel)
                .ok_or("Panel is no longer available")?;
            let mut dst = dock
                .find_tab(&anchor)
                .ok_or("Destination is no longer available")?;
            let insertion = match placement {
                Placement::Tab {
                    before: Some(before),
                } => {
                    dst = dock
                        .find_tab(&before)
                        .ok_or("Destination is no longer available")?;
                    let index = dst.tab.0.saturating_sub(usize::from(
                        src.node_path() == dst.node_path() && src.tab.0 < dst.tab.0,
                    ));
                    TabInsert::Insert(egui_dock::TabIndex(index))
                }
                Placement::Tab { before: None } => TabInsert::Append,
                Placement::Split(Zone::Center) => {
                    let index = (dst.tab.0 + 1).saturating_sub(usize::from(
                        src.node_path() == dst.node_path() && src.tab.0 < dst.tab.0,
                    ));
                    TabInsert::Insert(egui_dock::TabIndex(index))
                }
                Placement::Split(zone) => TabInsert::Split(match zone {
                    Zone::Left => egui_dock::Split::Left,
                    Zone::Right => egui_dock::Split::Right,
                    Zone::Top => egui_dock::Split::Above,
                    Zone::Bottom => egui_dock::Split::Below,
                    Zone::Center => unreachable!(),
                }),
            };
            dock.move_tab(src, (dst.node_path(), insertion));
            if let Some(path) = dock.find_tab(&panel) {
                let _ = dock.set_active_tab(path);
                dock.set_focused_node_and_surface(path.node_path());
            }
        }
        Action::Float { panel, rect } => {
            let path = dock
                .find_tab(&panel)
                .ok_or("Panel is no longer available")?;
            let surface = dock.detach_tab(
                path,
                egui::Rect::from_min_size(
                    egui::pos2(rect[0], rect[1]),
                    egui::vec2(rect[2], rect[3]),
                ),
            );
            dock.get_window_state_mut(surface)
                .ok_or("Panel window unavailable")?
                .set_position(egui::pos2(rect[0], rect[1]))
                .set_size(egui::vec2(rect[2], rect[3]));
        }
        Action::MoveFloating { panel, rect } => {
            let surface = dock
                .find_tab(&panel)
                .ok_or("Panel is no longer available")?
                .surface;
            dock.get_window_state_mut(surface)
                .ok_or("Panel window unavailable")?
                .set_position(egui::pos2(rect[0], rect[1]))
                .set_size(egui::vec2(rect[2], rect[3]));
        }
        Action::ResizeSplit {
            path,
            size: SplitSize::Ratio(ratio),
        } => {
            let mut index = NodeIndex::root();
            for second in path {
                index = if second { index.right() } else { index.left() };
            }
            match &mut dock.main_surface_mut()[index] {
                egui_dock::Node::Horizontal(split) | egui_dock::Node::Vertical(split) => {
                    split.fraction = ratio
                }
                _ => return Err("Split is no longer available".into()),
            }
        }
        _ => return Err("This layout action is not available in this presentation".into()),
    }
    super::layout::sanitize_dock_rects(dock);
    Ok(())
}

impl DockContent<Panel> for PakarTabViewer<'_> {
    fn body(&mut self, ui: &mut egui::Ui, panel: &Panel) {
        let mut tab = *panel;
        TabViewer::ui(self, ui, &mut tab);
    }
    fn panel_menu(&mut self, ui: &mut egui::Ui, panel: &Panel) {
        if let Some(path) = self.app.dock.find_tab(panel) {
            let mut tab = *panel;
            TabViewer::context_menu(self, ui, &mut tab, path.node_path());
        }
    }
    fn group_header_width(&self, _: &[Panel], _: &Panel, _: bool) -> f32 {
        26.0
    }
    fn group_header(&mut self, ui: &mut egui::Ui, _: &[Panel], _: &Panel, _: bool) {
        ui.menu_button(super::icons::CARET_DOWN, |ui| {
            super::layout::draw_workspace_tab_menu(self.app, ui)
        });
    }
}

pub fn show(app: &mut PakarControllerApp, ui: &mut egui::Ui) {
    let layout = project(&app.dock);
    let layer = ui.layer_id();
    let blocked = !ui.ctx().memory(|m| m.is_above_modal_layer(layer));
    let mut style = DockStyle::from_ui(ui);
    style.tab_height = super::control_density::height(ui);
    style.font = egui::TextStyle::Button.resolve(ui.style());
    style.close_label = super::i18n::text("Close panel");
    style.float_label = super::i18n::text("Float panel");
    style.move_label = super::i18n::text("Move to group");
    style.panels_label = super::i18n::text("Panels");
    let output = ui
        .add_enabled_ui(!blocked, |ui| {
            DockArea::new(egui::Id::new("pakar-craft-workspace")).show_customized(
                ui,
                &layout,
                &style,
                |panel| format!("{}  {}", panel.icon(), super::i18n::text(panel.name())),
                |_| {
                    if blocked {
                        Permissions::PROTECTED
                    } else {
                        Permissions::default()
                    }
                },
                |_| PanelLimits::default(),
                &mut PakarTabViewer { app },
            )
        })
        .inner;
    if !blocked {
        for action in output.actions {
            if let Err(error) = apply(&mut app.dock, action) {
                ui.colored_label(ui.visuals().error_fg_color, error);
            }
        }
        if let Some(error) = output.error {
            ui.colored_label(ui.visuals().error_fg_color, error.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn craft_projection_is_read_only_and_keeps_original_collapsed_slots() {
        let mut dock = super::super::layout::create_initial_layout();
        let path = dock.find_tab(&Panel::Hardware).unwrap().node_path();
        dock.node_mut(path).unwrap().set_collapsed(true);
        let original = super::super::layout::serialize(&dock).unwrap();
        let layout = project(&dock);
        assert!(layout.validate().is_ok());
        assert_eq!(layout.panels().len(), dock.iter_all_tabs().count());
        assert_eq!(super::super::layout::serialize(&dock).unwrap(), original);
        apply(
            &mut dock,
            Action::Activate {
                panel: Panel::Network,
            },
        )
        .unwrap();
        assert!(dock.node(path).unwrap().is_collapsed());
    }
    #[test]
    fn craft_moves_resize_and_close_update_the_same_workspace() {
        let mut dock = super::super::layout::create_initial_layout();
        apply(
            &mut dock,
            Action::Move {
                panel: Panel::Hardware,
                anchor: Panel::Table,
                placement: Placement::Tab { before: None },
            },
        )
        .unwrap();
        assert_eq!(
            dock.find_tab(&Panel::Hardware).unwrap().node_path(),
            dock.find_tab(&Panel::Table).unwrap().node_path()
        );
        apply(
            &mut dock,
            Action::ResizeSplit {
                path: vec![],
                size: SplitSize::Ratio(0.6),
            },
        )
        .unwrap();
        assert!(
            matches!(project(&dock).root, Some(Node::Split { size: SplitSize::Ratio(r), .. }) if r == 0.6)
        );
        apply(
            &mut dock,
            Action::Close {
                panel: Panel::Hardware,
            },
        )
        .unwrap();
        assert!(dock.find_tab(&Panel::Hardware).is_none());
        super::super::layout::reveal(&mut dock, Panel::Hardware);
        assert!(dock.find_tab(&Panel::Hardware).is_some());
    }
    #[test]
    fn floating_position_is_exact_before_first_egui_dock_paint_and_survives_save() {
        let mut dock = super::super::layout::create_initial_layout();
        apply(
            &mut dock,
            Action::Float {
                panel: Panel::Hardware,
                rect: [80.0, 90.0, 400.0, 300.0],
            },
        )
        .unwrap();
        apply(
            &mut dock,
            Action::MoveFloating {
                panel: Panel::Hardware,
                rect: [120.0, 150.0, 420.0, 320.0],
            },
        )
        .unwrap();
        let restored =
            super::super::layout::restore(&super::super::layout::serialize(&dock).unwrap());
        assert_eq!(
            project(&restored).floating[0].rect,
            [120.0, 150.0, 420.0, 320.0]
        );
        assert!(project(&restored).validate().is_ok());
    }
    #[test]
    fn malformed_gesture_does_not_mutate_the_original_workspace() {
        let mut dock = super::super::layout::create_initial_layout();
        let original = super::super::layout::serialize(&dock).unwrap();
        assert!(
            apply(
                &mut dock,
                Action::Float {
                    panel: Panel::Hardware,
                    rect: [f32::NAN, 0.0, 400.0, 300.0]
                }
            )
            .is_err()
        );
        assert_eq!(super::super::layout::serialize(&dock).unwrap(), original);
    }
}
