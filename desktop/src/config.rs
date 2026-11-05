//! Pealayer shell preferences with PakarController-owned persistence and panels.
pub use crate::ui::palette::Palette as ColorPalette;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::{OnceLock, RwLock},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppTheme {
    #[default]
    System,
    Light,
    Dark,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppLanguage {
    #[default]
    #[serde(alias = "system")]
    System,
    #[serde(alias = "en")]
    English,
    #[serde(alias = "fa")]
    Persian,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppDirection {
    #[default]
    Auto,
    Ltr,
    Rtl,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceUi {
    #[default]
    EguiDock,
    CraftUi,
}
impl WorkspaceUi {
    pub const ALL: [Self; 2] = [Self::EguiDock, Self::CraftUi];
    pub const fn label(self) -> &'static str {
        match self { Self::EguiDock => "egui Dock", Self::CraftUi => "CraftUI" }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UiControlDensity {
    #[default]
    Compact,
    Regular,
    Comfortable,
    Spacious,
}
impl UiControlDensity {
    pub const ALL: [Self; 4] = [
        Self::Compact,
        Self::Regular,
        Self::Comfortable,
        Self::Spacious,
    ];

    pub const fn base_height(self) -> f32 {
        match self {
            Self::Compact => 24.0,
            Self::Regular => 28.0,
            Self::Comfortable => 32.0,
            Self::Spacious => 36.0,
        }
    }
}

// Adapted from actual Pealayer config::parse_language_tag/resolve_language/
// resolve_rtl. Keep old serialized English/Persian preferences compatible.
pub fn parse_language_tag(value: &str) -> Option<AppLanguage> {
    let normalized = value.trim().replace('_', "-").to_ascii_lowercase();
    match normalized.as_str() {
        "system" | "auto" => Some(AppLanguage::System),
        "en" | "english" => Some(AppLanguage::English),
        "fa" | "fa-ir" | "persian" | "farsi" => Some(AppLanguage::Persian),
        _ if normalized.starts_with("fa-") => Some(AppLanguage::Persian),
        _ if normalized.starts_with("en-") => Some(AppLanguage::English),
        _ => None,
    }
}
fn system_language() -> AppLanguage {
    for key in ["LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(value) = std::env::var(key) {
            if let Some(language) = parse_language_tag(&value) {
                if language != AppLanguage::System {
                    return language;
                }
            }
        }
    }
    #[cfg(windows)]
    {
        // Use the existing Windows binding instead of adding a registry crate.
        let mut name = [0u16; 85];
        let count = unsafe { windows::Win32::Globalization::GetUserDefaultLocaleName(&mut name) };
        if count > 1 {
            if let Some(language) =
                parse_language_tag(&String::from_utf16_lossy(&name[..count as usize - 1]))
            {
                if language != AppLanguage::System {
                    return language;
                }
            }
        }
    }
    AppLanguage::English
}
pub fn resolved_language(config: &AppConfig) -> AppLanguage {
    let preference = std::env::var("APP_LOCALE")
        .ok()
        .and_then(|value| parse_language_tag(&value))
        .unwrap_or(config.language);
    if preference == AppLanguage::System {
        system_language()
    } else {
        preference
    }
}
pub fn resolved_rtl(config: &AppConfig) -> bool {
    let direction = match std::env::var("APP_DIRECTION")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "ltr" => AppDirection::Ltr,
        "rtl" => AppDirection::Rtl,
        "auto" => AppDirection::Auto,
        _ => config.direction,
    };
    match direction {
        AppDirection::Auto => resolved_language(config).is_rtl(),
        AppDirection::Ltr => false,
        AppDirection::Rtl => true,
    }
}
impl AppLanguage {
    pub fn is_rtl(self) -> bool {
        self == Self::Persian
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct NumericInputSteps {
    pub normal: f64,
    pub fine: f64,
    pub coarse: f64,
}
impl NumericInputSteps {
    pub fn for_step(step: f64) -> Self {
        Self {
            normal: step,
            fine: step / 10.0,
            coarse: step * 10.0,
        }
    }
    pub fn is_valid(self) -> bool {
        [self.normal, self.fine, self.coarse]
            .iter()
            .all(|v| v.is_finite() && *v > 0.0)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProfile {
    pub name: String,
    pub icon: String,
    pub layout: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub client_management: crate::client_management::Settings,
    pub firmware_feed: crate::firmware_feed::Settings,
    pub bridge: crate::bridge::Settings,
    pub stream_fps: u16,
    pub display: crate::display::Settings,
    pub theme: AppTheme,
    pub color_palette: ColorPalette,
    pub accent: String,
    pub language: AppLanguage,
    pub direction: AppDirection,
    pub ui_scale: f32,
    pub ui_control_density: UiControlDensity,
    pub preferences_grouped_navigation: bool,
    pub workspace_ui: WorkspaceUi,
    pub single_instance: bool,
    pub close_to_tray: bool,
    pub always_on_top: bool,
    pub native_dialogs: bool,
    pub keyboard_shortcuts_enabled: bool,
    pub application_shortcuts: crate::application_shortcuts::ApplicationShortcuts,
    pub notifications: crate::platform::notifications::Preferences,
    pub endpoint: String,
    pub user: String,
    pub serial_port: String,
    pub connection_method: crate::connection::Method,
    pub connection_enabled: bool,
    pub auto_connect: bool,
    pub last_network_route: Option<crate::peer_http::SavedRoute>,
    pub dock_layout: String,
    pub events_auto_hide: bool,
    /// One-time recovery of layouts saved by the disconnected-workspace bug.
    pub dock_context_revision: u8,
    pub workspace_profiles: BTreeMap<String, WorkspaceProfile>,
    /// Per-board/application presentation, never a live device cache.
    pub native_workspaces: crate::native_workspace::Store,
    pub numeric_steps: BTreeMap<String, NumericInputSteps>,
    pub additive_cell_selection: bool,
    pub animate_cells: bool,
    pub show_cell_indicators: bool,
    /// Color the semantic lamp face; sampled per-pixel previews keep their RGB.
    pub color_cell_backgrounds: bool,
    pub cell_order: Vec<u8>,
    pub cell_shape: crate::ui::cell_pixels::Shape,
    pub pixel_diagnostics: bool,
    /// Preview preference only; leases and active preview state never persist.
    pub pixel_correction_live_preview: bool,
    pub status_items: crate::ui::status_bar::Items,
}
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            client_management: Default::default(),
            firmware_feed: Default::default(),
            bridge: Default::default(),
            stream_fps: crate::streaming::DEFAULT_FPS,
            display: crate::display::Settings::default(),
            theme: AppTheme::System,
            color_palette: ColorPalette::Studio,
            accent: "#38D27A".into(),
            language: AppLanguage::System,
            direction: AppDirection::Auto,
            ui_scale: 1.0,
            ui_control_density: UiControlDensity::Compact,
            preferences_grouped_navigation: true,
            workspace_ui: WorkspaceUi::default(),
            single_instance: true,
            close_to_tray: false,
            always_on_top: false,
            native_dialogs: true,
            keyboard_shortcuts_enabled: true,
            application_shortcuts: Default::default(),
            notifications: Default::default(),
            endpoint: "http://pakarcontroller".into(),
            user: "admin".into(),
            serial_port: String::new(),
            connection_method: Default::default(),
            connection_enabled: true,
            auto_connect: true,
            last_network_route: None,
            dock_layout: String::new(),
            events_auto_hide: true,
            dock_context_revision: 0,
            workspace_profiles: BTreeMap::new(),
            native_workspaces: Default::default(),
            numeric_steps: BTreeMap::new(),
            additive_cell_selection: false,
            animate_cells: true,
            show_cell_indicators: false,
            color_cell_backgrounds: false,
            cell_order: (1..=80).collect(),
            cell_shape: Default::default(),
            pixel_diagnostics: false,
            pixel_correction_live_preview: true,
            status_items: Default::default(),
        }
    }
}
impl AppConfig {
    pub fn startup_connection_enabled(&self, fixture: bool) -> bool {
        if fixture {
            return self.connection_enabled;
        }
        self.auto_connect
            && (self.connection_method == crate::connection::Method::WiFi
                || !self.serial_port.trim().is_empty())
    }
    pub fn path() -> PathBuf {
        if let Some(path) = std::env::var_os("PAKAR_CONFIG_FILE") {
            return path.into();
        }
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir)
            .join("Programs/PakarController/config/preferences.json")
    }
    pub fn load() -> Result<Self, String> {
        match std::fs::read(Self::path()) {
            Ok(bytes) => {
                let mut config: Self =
                    serde_json::from_slice(&bytes).map_err(|e| format!("Read preferences: {e}"))?;
                config.cell_order = crate::ui::cell_table::normalized_order(&config.cell_order);
                config.native_workspaces.normalize();
                Ok(config)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(format!("Read preferences: {e}")),
        }
    }
    pub fn save(&self) -> Result<(), String> {
        self.client_management.validate()?;
        if self.bridge.enabled {
            self.bridge.validate()?;
        }
        if !self.ui_scale.is_finite()
            || self.stream_fps > crate::streaming::MAX_FPS
            || !(0.75..=2.0).contains(&self.ui_scale)
            || parse_rgb_hex(&self.accent).is_none()
        {
            return Err("Choose a valid accent and UI scale".into());
        }
        self.application_shortcuts.validate()?;
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let bytes = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        let staged = path.with_extension("pending");
        std::fs::write(&staged, bytes).map_err(|e| e.to_string())?;
        std::fs::rename(&staged, &path).map_err(|e| e.to_string())
    }
}
pub fn parse_rgb_hex(value: &str) -> Option<[u8; 3]> {
    let hex = value.strip_prefix('#').unwrap_or(value);
    if hex.len() != 6 || !hex.is_ascii() {
        return None;
    }
    Some([
        u8::from_str_radix(&hex[0..2], 16).ok()?,
        u8::from_str_radix(&hex[2..4], 16).ok()?,
        u8::from_str_radix(&hex[4..6], 16).ok()?,
    ])
}
fn live_slot() -> &'static RwLock<AppConfig> {
    static VALUE: OnceLock<RwLock<AppConfig>> = OnceLock::new();
    VALUE.get_or_init(Default::default)
}
pub fn live() -> AppConfig {
    live_slot().read().expect("preferences lock").clone()
}
pub fn set_live(config: &AppConfig) {
    *live_slot().write().expect("preferences lock") = config.clone();
}

pub fn apply(ctx: &eframe::egui::Context, config: &AppConfig) {
    crate::ui::i18n::configure_ui_fonts(ctx, resolved_language(config).is_rtl());
    use eframe::egui::{Color32, Stroke, Theme, ThemePreference};
    ctx.set_theme(match config.theme {
        AppTheme::System => ThemePreference::System,
        AppTheme::Dark => ThemePreference::Dark,
        AppTheme::Light => ThemePreference::Light,
    });
    let rgb = parse_rgb_hex(&config.accent).unwrap_or([56, 210, 122]);
    let accent = Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
    for theme in [Theme::Dark, Theme::Light] {
        let mut visuals = if theme == Theme::Dark {
            eframe::egui::Visuals::dark()
        } else {
            eframe::egui::Visuals::light()
        };
        crate::ui::palette::apply(&mut visuals, config.color_palette);
        visuals.selection.bg_fill = accent;
        let bright =
            u32::from(rgb[0]) * 299 + u32::from(rgb[1]) * 587 + u32::from(rgb[2]) * 114 > 150_000;
        visuals.selection.stroke = Stroke::new(
            1.0,
            if bright {
                Color32::from_rgb(20, 20, 20)
            } else {
                Color32::WHITE
            },
        );
        visuals.hyperlink_color = accent;
        visuals.window_corner_radius = eframe::egui::CornerRadius::same(10);
        visuals.menu_corner_radius = eframe::egui::CornerRadius::same(8);
        ctx.set_visuals_of(theme, visuals);
    }
    ctx.all_styles_mut(|style| {
        style.interaction.selectable_labels = false;
        style.interaction.multi_widget_text_select = false;
        for font in style.text_styles.values_mut() {
            font.size = font.size.min(13.0);
        }
        style.spacing.item_spacing = eframe::egui::vec2(8.0, 6.0);
        style.spacing.button_padding.x = 8.0;
        crate::ui::control_density::configure(style, config.ui_control_density);
    });
    ctx.set_zoom_factor(config.ui_scale);
    crate::ui::elegance_theme::set_source_palette(ctx, config.color_palette);
    set_live(config);
    ctx.send_viewport_cmd(eframe::egui::ViewportCommand::WindowLevel(
        if config.always_on_top {
            eframe::egui::WindowLevel::AlwaysOnTop
        } else {
            eframe::egui::WindowLevel::Normal
        },
    ));
}

#[cfg(test)]
mod locale_tests {
    use super::*;
    #[test]
    fn cell_background_preference_is_opt_in_and_roundtrips_without_other_changes() {
        let old: AppConfig = serde_json::from_str("{}").unwrap();
        assert!(!old.color_cell_backgrounds);
        let mut enabled = old.clone();
        enabled.color_cell_backgrounds = true;
        let mut restored: AppConfig = serde_json::from_slice(&serde_json::to_vec(&enabled).unwrap()).unwrap();
        assert!(restored.color_cell_backgrounds);
        restored.color_cell_backgrounds = false;
        assert_eq!(restored, old);
    }
    #[test]
    fn locale_tags_and_saved_preferences_remain_compatible() {
        for tag in ["fa", "fa_IR", "fa-AF", "Persian", "Farsi"] {
            assert_eq!(parse_language_tag(tag), Some(AppLanguage::Persian));
        }
        assert_eq!(
            parse_language_tag("en_US.UTF-8"),
            Some(AppLanguage::English)
        );
        assert_eq!(parse_language_tag("auto"), Some(AppLanguage::System));
        assert_eq!(parse_language_tag("zz"), None);
        let old: AppConfig = serde_json::from_str(r#"{"language":"Persian"}"#).unwrap();
        assert_eq!(old.language, AppLanguage::Persian);
        assert_eq!(old.direction, AppDirection::Auto);
        let roundtrip: AppConfig =
            serde_json::from_slice(&serde_json::to_vec(&old).unwrap()).unwrap();
        assert_eq!(roundtrip.language, old.language);
        assert_eq!(roundtrip.direction, old.direction);
    }
}
