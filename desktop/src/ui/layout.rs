//! Pealayer DockArea / TabViewer shell, adapted to PakarController domains.
use crate::PakarControllerApp;
use eframe::egui;
use egui_dock::{DockState, NodeIndex, TabViewer};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Panel {
    Table,
    PixelDiagnostics,
    PixelCorrections,
    Keys,
    Game,
    GameSettings,
    Melodies,
    EffectControls,
    FrontPanel,
    Hardware,
    Network,
    Peers,
    Streaming,
    Events,
}
impl Panel {
    pub const ALL: [Self; 14] = [
        Self::Table,
        Self::PixelDiagnostics,
        Self::PixelCorrections,
        Self::Keys,
        Self::Game,
        Self::GameSettings,
        Self::Melodies,
        Self::EffectControls,
        Self::FrontPanel,
        Self::Hardware,
        Self::Network,
        Self::Peers,
        Self::Streaming,
        Self::Events,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::Table => "Table",
            Self::PixelDiagnostics => "Pixel diagnostics",
            Self::PixelCorrections => "Pixel corrections",
            Self::Keys => "Keys",
            Self::Game => "Game",
            Self::GameSettings => "Game settings",
            Self::Melodies => "Melodies",
            Self::EffectControls => "Effect controls",
            Self::FrontPanel => "Front panel",
            Self::Hardware => "Hardware monitor",
            Self::Network => "Network & cloud",
            Self::Peers => "Peers",
            Self::Streaming => "Streaming",
            Self::Events => "Events",
        }
    }
    pub fn icon(self) -> &'static str {
        use super::icons::*;
        match self {
            Self::Table => GRID_FOUR,
            Self::PixelDiagnostics => MAGNIFYING_GLASS,
            Self::PixelCorrections => SLIDERS_HORIZONTAL,
            Self::Keys => HAND_TAP,
            Self::Game => PLAY,
            Self::GameSettings => GEAR,
            Self::Melodies => MUSIC_NOTE,
            Self::EffectControls => SLIDERS_HORIZONTAL,
            Self::FrontPanel => GAUGE,
            Self::Hardware => CIRCUITRY,
            Self::Network => GLOBE,
            Self::Peers => RADIO,
            Self::Streaming => BROADCAST,
            Self::Events => LIST,
        }
    }
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|panel| {
            panel.name().eq_ignore_ascii_case(name)
                || format!("{panel:?}").eq_ignore_ascii_case(name)
        })
    }
    pub fn available(self, context: Option<&crate::capabilities::Manifest>) -> bool {
        if matches!(self,Self::Events|Self::Peers) {
            return true;
        }
        let Some(c) = context else {
            return false;
        };
        match self {
            Self::Table => c.table(),
            Self::PixelDiagnostics => c.has("pixel-diagnostic"),
            Self::PixelCorrections => c.has("pixel-corrections"),
            Self::Keys => c.role == "first-wins" && c.has("inputs") && c.has("outputs"),
            Self::Game | Self::GameSettings => c.role == "first-wins" && c.has("inputs") && c.has("outputs"),
            Self::Melodies => c.has("buzzer"),
            Self::EffectControls => c.has("leds") || c.modes().next().is_some(),
            Self::FrontPanel => {
                c.has("display") || c.has("text") || c.has("buzzer") || c.has("lcd")
            }
            Self::Hardware => c.has("telemetry"),
            Self::Network => c.has("network"),
            Self::Streaming => c.table() && c.has("streaming"),
            Self::Events | Self::Peers => true,
        }
    }
}

pub fn create_initial_layout() -> DockState<Panel> {
    let mut dock = DockState::new(vec![Panel::Table, Panel::Keys]);
    let [center, right] = dock.main_surface_mut().split_right(
        NodeIndex::root(),
        0.72,
        vec![Panel::Game, Panel::GameSettings, Panel::EffectControls, Panel::PixelDiagnostics, Panel::PixelCorrections, Panel::Network, Panel::Peers],
    );
    dock.main_surface_mut()
        .split_below(right, 0.48, vec![Panel::FrontPanel, Panel::Melodies, Panel::Hardware]);
    dock.main_surface_mut()
        .split_below(center, 0.80, vec![Panel::Events, Panel::Streaming]);
    sanitize_dock_rects(&mut dock);
    dock
}

// Imported from Pealayer ui/layout.rs::sanitize_dock_rects.
pub fn sanitize_dock_rects<Tab>(dock_state: &mut DockState<Tab>) {
    for (_path, node) in dock_state.iter_all_nodes_mut() {
        if let Some(rect) = node.rect()
            && !rect.is_finite()
        {
            node.set_rect(egui::Rect::ZERO);
        }
        if let Some(leaf) = node.get_leaf_mut()
            && !leaf.viewport.is_finite()
        {
            leaf.viewport = egui::Rect::ZERO;
        }
    }
}
pub fn restore(json: &str) -> DockState<Panel> {
    serde_json::from_str::<DockState<Panel>>(json)
        .ok()
        .filter(|dock| dock.iter_all_tabs().count() > 0)
        .unwrap_or_else(create_initial_layout)
}
pub fn serialize(dock: &DockState<Panel>) -> Result<String, String> {
    let mut copy = dock.clone();
    sanitize_dock_rects(&mut copy);
    serde_json::to_string(&copy).map_err(|e| e.to_string())
}
pub fn reveal(dock: &mut DockState<Panel>, panel: Panel) {
    restore_canonical(dock, panel);
    if let Some(path) = dock.find_tab(&panel) {
        let mut node = Some(path.node);
        while let Some(index) = node {
            if let Ok(node) = dock.node_mut(egui_dock::NodePath::new(path.surface, index)) {
                node.set_collapsed(false);
            }
            node = index.parent();
        }
        let _ = dock.set_active_tab(path);
        dock.set_focused_node_and_surface(path.node_path());
    }
}
/// A first-wins board uses Keys as its primary workspace, in the same slot
/// where a table board uses Table. Older layouts accidentally placed Keys
/// beside Front panel in the narrow right dock; migrate only that placement.
pub fn promote_keys(dock: &mut DockState<Panel>) {
    if let Some(keys) = dock.find_tab(&Panel::Keys) {
        let primary = dock
            .find_tab(&Panel::Table)
            .or_else(|| dock.find_tab(&Panel::Events));
        if primary.is_some_and(|primary| primary.node_path() != keys.node_path()) {
            dock.remove_tab(keys);
        }
    }
    reveal(dock, Panel::Keys);
}
/// Adapt Pealayer restore_tab_to_canonical_slot: siblings first, then semantic
/// anchor splits. Never append a returning panel to whichever leaf has focus.
pub fn restore_canonical(dock: &mut DockState<Panel>, panel: Panel) {
    if dock.find_tab(&panel).is_some() {
        return;
    }
    if dock.iter_all_tabs().count() == 0 {
        *dock = DockState::new(vec![panel]);
        return;
    }
    if panel == Panel::Keys {
        if let Some(path) = dock
            .find_tab(&Panel::Table)
            .or_else(|| dock.find_tab(&Panel::Events))
        {
            if let Ok(leaf) = dock.leaf_mut(path.node_path()) {
                leaf.tabs.push(panel);
                sanitize_dock_rects(dock);
                return;
            }
        }
    }
    let siblings: &[Panel] = match panel {
        Panel::FrontPanel | Panel::Melodies | Panel::Hardware => &[Panel::FrontPanel, Panel::Melodies, Panel::Hardware],
        Panel::Events | Panel::Streaming => &[Panel::Events, Panel::Streaming],
        Panel::Game | Panel::GameSettings | Panel::Network | Panel::Peers | Panel::EffectControls | Panel::PixelDiagnostics | Panel::PixelCorrections => &[Panel::Game, Panel::GameSettings, Panel::EffectControls, Panel::PixelDiagnostics, Panel::PixelCorrections, Panel::Network, Panel::Peers],
        _ => &[],
    };
    if let Some(path) = siblings.iter().find_map(|p| dock.find_tab(p)) {
        if let Ok(leaf) = dock.leaf_mut(path.node_path()) {
            leaf.tabs.push(panel);
        }
    } else if let Some(path) = dock
        .find_tab(&Panel::Table)
        .or_else(|| dock.iter_all_tabs().map(|(path, _)| path).next())
    {
        if path.surface != egui_dock::SurfaceIndex::main() {
            // Floating-only workspaces have no main-surface anchor to split.
            dock.push_to_first_leaf(panel);
        } else {
            match panel {
                Panel::Table | Panel::Keys => {
                    dock.main_surface_mut()
                        .split_left(path.node, 0.72, vec![panel]);
                }
                Panel::Game | Panel::GameSettings | Panel::EffectControls | Panel::PixelDiagnostics | Panel::PixelCorrections => {
                    dock.main_surface_mut()
                        .split_right(path.node, 0.72, vec![panel]);
                }
                Panel::FrontPanel | Panel::Melodies | Panel::Hardware => {
                    let anchor = dock.find_tab(&Panel::EffectControls).unwrap_or(path);
                    if anchor.surface == egui_dock::SurfaceIndex::main() {
                        dock.main_surface_mut()
                            .split_below(anchor.node, 0.48, vec![panel]);
                    } else {
                        dock.push_to_first_leaf(panel);
                    }
                }
                _ => {
                    dock.main_surface_mut()
                        .split_below(path.node, 0.80, vec![panel]);
                }
            }
        }
    }
    sanitize_dock_rects(dock);
}
pub fn draw_workspace_tab_menu(app: &mut PakarControllerApp, ui: &mut egui::Ui) {
    super::cell_table::compact_menu(ui);
    for panel in Panel::ALL {
        if !panel.available(app.presentation_context()) {
            continue;
        }
        let is_open = app.dock.find_tab(&panel).is_some() || app.open_panels.contains(&panel);
        let mut visible = is_open;
        if ui
            .checkbox(
                &mut visible,
                format!(
                    "{}  {}  {}",
                    if is_open {
                        super::icons::EYE
                    } else {
                        super::icons::EYE_SLASH
                    },
                    panel.icon(),
                    panel.name()
                ),
            )
            .changed()
        {
            if visible {
                app.reveal_panels.push(panel);
            } else {
                app.close_panels.push(panel);
            }
            ui.close();
        }
    }
}
// Imported from Pealayer ui/layout.rs::paint_dock_disclosure_icons.
pub fn paint_dock_disclosure_icons(ui: &mut egui::Ui, dock: &DockState<Panel>, height: f32) {
    let pointer = ui.ctx().pointer_hover_pos();
    for (_, leaf) in dock.iter_leaves() {
        if !leaf.rect.is_positive() || leaf.tabs.is_empty() {
            continue;
        }
        let rect = egui::Rect::from_min_size(
            leaf.rect.min,
            egui::vec2(24.0_f32.min(leaf.rect.width()), height),
        );
        let hovered = pointer.is_some_and(|position| rect.contains(position));
        let color = if hovered {
            ui.visuals().strong_text_color()
        } else {
            ui.visuals().text_color()
        };
        let icon = if leaf.collapsed {
            super::icons::CARET_RIGHT
        } else {
            super::icons::CARET_DOWN
        };
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::proportional(13.0),
            color,
        );
    }
}
pub struct PakarTabViewer<'a> {
    pub app: &'a mut PakarControllerApp,
}
impl TabViewer for PakarTabViewer<'_> {
    type Tab = Panel;
    fn id(&mut self, tab: &mut Panel) -> egui::Id {
        egui::Id::new(*tab)
    }
    fn title(&mut self, tab: &mut Panel) -> egui::WidgetText {
        format!("{}  {}", tab.icon(), super::i18n::text(tab.name())).into()
    }
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Panel) {
        egui::Frame::new().inner_margin(12).show(ui, |ui| {
            self.app.draw_panel(ui, *tab);
        });
    }
    fn scroll_bars(&self, _: &Panel) -> [bool; 2] {
        [false, false]
    }
    // Actual Pealayer on_tab_button implementation: register rendered tab hit rects.
    fn on_tab_button(&mut self, _: &mut Panel, response: &egui::Response) {
        response.ctx.data_mut(|data| {
            let id = egui::Id::new("pakar-workspace-tab-button-rects");
            let mut rects = data.get_temp::<Vec<egui::Rect>>(id).unwrap_or_default();
            rects.push(response.rect);
            data.insert_temp(id, rects);
        });
    }
    fn context_menu(&mut self, ui: &mut egui::Ui, tab: &mut Panel, _: egui_dock::NodePath) {
        super::cell_table::compact_menu(ui);
        ui.strong(super::i18n::text(tab.name()));
        ui.separator();
        if ui
            .button(format!("{}  Preferences…", super::icons::GEAR))
            .clicked()
        {
            self.app.open_preferences();
            ui.close();
        }
        ui.separator();
        super::icons::submenu(ui, format!("{}  Panels", super::icons::TABS), |ui| {
            draw_workspace_tab_menu(self.app, ui)
        });
        if ui
            .button(format!("{}  Close panel", super::icons::X))
            .clicked()
        {
            self.app.close_panels.push(*tab);
            ui.close();
        }
        if ui
            .button(format!("{}  Close other panels", super::icons::EYE_SLASH))
            .clicked()
        {
            self.app
                .close_panels
                .extend(self.app.open_panels.iter().copied().filter(|p| p != tab));
            ui.close();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pixel_tools_are_independent_persistent_dock_panels() {
        let mut dock = create_initial_layout();
        let table = dock.find_tab(&Panel::Table).unwrap().node_path();
        for panel in [Panel::PixelDiagnostics, Panel::PixelCorrections] {
            assert_ne!(dock.find_tab(&panel).unwrap().node_path(), table);
            let old = dock.find_tab(&panel).unwrap();
            dock.remove_tab(old);
            restore_canonical(&mut dock, panel);
            assert_eq!(dock.find_tab(&panel).unwrap().node_path(), dock.find_tab(&Panel::EffectControls).unwrap().node_path());
        }
        let restored = restore(&serialize(&dock).unwrap());
        for panel in Panel::ALL {
            assert_eq!(restored.iter_all_tabs().filter(|(_, p)| **p == panel).count(), 1);
        }
        assert!(!Panel::PixelDiagnostics.available(None));
        assert!(!Panel::PixelCorrections.available(None));
    }
    #[test]
    fn first_wins_keys_migrate_from_narrow_front_panel_to_primary_slot() {
        let mut dock = create_initial_layout();
        dock.remove_tab(dock.find_tab(&Panel::Keys).unwrap());
        let front = dock.find_tab(&Panel::FrontPanel).unwrap().node_path();
        dock.leaf_mut(front).unwrap().tabs.push(Panel::Keys);
        promote_keys(&mut dock);
        let keys = dock.find_tab(&Panel::Keys).unwrap();
        let table = dock.find_tab(&Panel::Table).unwrap();
        assert_eq!(keys.node_path(), table.node_path());
        assert_eq!(dock.leaf(keys.node_path()).unwrap().active, keys.tab);
        assert_eq!(
            dock.iter_all_tabs()
                .filter(|(_, p)| **p == Panel::Keys)
                .count(),
            1
        );
    }
    #[test]
    fn restored_panel_returns_to_its_semantic_group_and_expands_collapsed_ancestors() {
        let mut dock = create_initial_layout();
        dock.remove_tab(dock.find_tab(&Panel::Hardware).unwrap());
        let front = dock.find_tab(&Panel::FrontPanel).unwrap().node_path();
        dock.node_mut(front).unwrap().set_collapsed(true);
        reveal(&mut dock, Panel::Hardware);
        let hardware = dock.find_tab(&Panel::Hardware).unwrap();
        assert_eq!(hardware.node_path(), front);
        assert!(!dock.node(front).unwrap().is_collapsed());
        assert_eq!(dock.leaf(front).unwrap().active, hardware.tab);
    }
    #[test]
    fn layouts_survive_serialization_and_closed_panel_reveal() {
        let initial = create_initial_layout();
        let json = serialize(&initial).unwrap();
        let mut restored = restore(&json);
        assert_eq!(
            initial.iter_all_tabs().count(),
            restored.iter_all_tabs().count()
        );
        reveal(&mut restored, Panel::Network);
        reveal(&mut restored, Panel::Network);
        assert_eq!(
            restored
                .iter_all_tabs()
                .filter(|(_, p)| **p == Panel::Network)
                .count(),
            1
        );
        for (_, node) in restored.iter_all_nodes() {
            if let Some(rect) = node.rect() {
                assert!(rect.is_finite());
            }
        }
    }
}
