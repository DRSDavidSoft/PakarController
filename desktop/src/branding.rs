//! Actual Pealayer image decoder adapted to Pakar's single cross-surface master.
pub use crate::brand_image::{icon_bytes, compact_icon_bytes};
use eframe::egui;
use std::hash::{Hash, Hasher};
use base64::Engine;

// Adapted from Pealayer's sync_native_window_icon and icon_source_signature:
// peer-owned application metadata replaces playback/config; no disk cache.
pub fn sync_peer(
    ctx: &egui::Context,
    peer: Option<&crate::capabilities::Manifest>,
) -> Option<egui::IconData> {
    let app = peer.and_then(|p| p.application());
    let mut hash = std::collections::hash_map::DefaultHasher::new();
    app.hash(&mut hash);
    peer.map(|p| &p.role).hash(&mut hash);
    let signature = hash.finish();
    let changed = ctx.data_mut(|data| {
        let id = egui::Id::new("pakar-peer-window-icon");
        if data.get_temp::<u64>(id) == Some(signature) {
            false
        } else {
            data.insert_temp(id, signature);
            true
        }
    });
    if !changed {
        return None;
    }
    let icon = app
        .and_then(|a| a.icon_bytes())
        .filter(|b| b.as_slice() != icon_bytes() && b.as_slice() != compact_icon_bytes())
        .and_then(|b| icon_data_from_bytes(&b))
        .or_else(|| {
            let encoded = match peer.filter(|p| p.known()).map(|p| p.role.as_str()) {
                Some("table-selection") => include_str!("../../assets/branding/table.png.base64"),
                Some("first-wins") => include_str!("../../assets/branding/keys.png.base64"),
                _ => return None,
            };
            base64::engine::general_purpose::STANDARD.decode(encoded.trim()).ok()
                .and_then(|bytes| icon_data_from_bytes(&bytes))
        })
        .unwrap_or_else(window_icon);
    let texture = ctx.load_texture(
        "Connected application artwork",
        egui::ColorImage::from_rgba_unmultiplied(
            [icon.width as usize, icon.height as usize],
            &icon.rgba,
        ),
        egui::TextureOptions::LINEAR,
    );
    ctx.data_mut(|data| data.insert_temp(egui::Id::new("pakar-peer-brand-texture"), texture));
    let shared = std::sync::Arc::new(icon.clone());
    for viewport in ctx.input(|input| input.raw.viewports.keys().copied().collect::<Vec<_>>()) {
        ctx.send_viewport_cmd_to(viewport, egui::ViewportCommand::Icon(Some(shared.clone())));
    }
    Some(icon)
}
pub fn peer_logo(ui: &mut egui::Ui, size: f32, app: Option<&crate::identity::Application>) {
    let texture = ui.ctx().data_mut(|data| {
        data.get_temp::<egui::TextureHandle>(egui::Id::new("pakar-peer-brand-texture"))
    });
    if let Some(texture) = texture {
        ui.add(egui::Image::new(&texture).fit_to_exact_size(egui::vec2(size, size)))
            .on_hover_text(format!(
                "{} application logo",
                app.map(|a| a.name.as_str()).unwrap_or("PakarController")
            ));
    } else {
        logo(ui, size);
    }
}

// Imported unchanged from Pealayer src/branding.rs.
pub fn icon_data_from_bytes(bytes: &[u8]) -> Option<egui::IconData> {
    let image = image::load_from_memory(bytes).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    Some(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}

pub fn window_icon() -> egui::IconData {
    icon_data_from_bytes(icon_bytes()).expect("bundled application artwork")
}

pub fn logo(ui: &mut egui::Ui, size: f32) {
    let id = egui::Id::new("pakar-shared-brand-texture");
    let texture = ui
        .ctx()
        .data_mut(|data| data.get_temp::<egui::TextureHandle>(id));
    let texture = texture.unwrap_or_else(|| {
        let icon = window_icon();
        let texture = ui.ctx().load_texture(
            "PakarController application icon",
            egui::ColorImage::from_rgba_unmultiplied(
                [icon.width as usize, icon.height as usize],
                &icon.rgba,
            ),
            egui::TextureOptions::LINEAR,
        );
        ui.ctx()
            .data_mut(|data| data.insert_temp(id, texture.clone()));
        texture
    });
    ui.add(egui::Image::new(&texture).fit_to_exact_size(egui::vec2(size, size)))
        .on_hover_text("PakarController application logo");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compact_default_does_not_mask_the_connected_board_role() {
        let ctx = egui::Context::default();
        let peer = crate::capabilities::Manifest {
            board_kind: "controller".into(),
            role: "first-wins".into(),
            application: Some(crate::identity::Application {
                id: "keys.app".into(), name: "Keys".into(), title: "Keys".into(),
                icon_png_base64: include_str!("../../assets/branding/app-icon.png.base64").trim().into(),
            }),
            ..Default::default()
        };
        let expected = base64::engine::general_purpose::STANDARD
            .decode(include_str!("../../assets/branding/keys.png.base64").trim()).unwrap();
        let expected = icon_data_from_bytes(&expected).unwrap();
        let mut output = ctx.run_ui(egui::RawInput::default(), |_| {
            assert_eq!(sync_peer(&ctx, Some(&peer)).unwrap().rgba, expected.rgba);
        });
        output.textures_delta.clear();
    }
    #[test]
    fn peer_icon_lifecycle_reuses_refresh_and_revokes_on_disconnect() {
        let ctx = egui::Context::default();
        let mut peer = crate::capabilities::Manifest {
            board_kind: "controller".into(),
            application: Some(crate::identity::Application {
                id: "table.app".into(),
                name: "Pakar Table".into(),
                title: "Table controller".into(),
                icon_png_base64: include_str!("../../assets/branding/app-icon.png.base64")
                    .trim()
                    .into(),
            }),
            ..Default::default()
        };
        let mut changes = 0;
        let mut output = ctx.run_ui(egui::RawInput::default(), |_| {
            changes += usize::from(sync_peer(&ctx, Some(&peer)).is_some());
            assert!(sync_peer(&ctx, Some(&peer)).is_none());
        });
        output.textures_delta.clear();
        assert_eq!(changes, 1);
        assert!(output.viewport_output.values().any(|v| {
            v.commands
                .iter()
                .any(|c| matches!(c, egui::ViewportCommand::Icon(Some(_))))
        }));
        peer.application.as_mut().unwrap().name = "Pakar Keys".into();
        let mut output = ctx.run_ui(egui::RawInput::default(), |_| {
            assert!(sync_peer(&ctx, Some(&peer)).is_some());
            assert!(sync_peer(&ctx, None).is_some());
            assert!(sync_peer(&ctx, None).is_none());
        });
        output.textures_delta.clear();
    }

    #[test]
    fn bundled_artwork_decodes_without_files_or_theme_specific_replacements() {
        let icon = window_icon();
        assert_eq!((icon.width, icon.height), (1024, 1024));
        assert_eq!(
            crate::brand_image::rgba_at_size(64).as_slice(),
            &include_bytes!(concat!(env!("OUT_DIR"), "/pakar-icon-64.rgba"))[..]
        );
        assert_eq!(
            crate::brand_image::rgba_at_size(32).as_slice(),
            &include_bytes!(concat!(env!("OUT_DIR"), "/pakar-icon-32.rgba"))[..]
        );
        assert!(icon.rgba.chunks_exact(4).any(|pixel| pixel[3] > 0));
        assert!(icon_data_from_bytes(b"broken artwork").is_none());
    }

    // Adapted donor shell_derivative_preserves_native_master_and_multiple_resolutions:
    // decode the ICO's PNG frames, since Pakar's image dependency is PNG-only.
    #[test]
    fn shell_derivative_preserves_native_master_and_multiple_resolutions() {
        let bytes = crate::brand_image::native_icon_bytes(icon_bytes()).unwrap();
        assert_eq!(&bytes[..4], &[0, 0, 1, 0]);
        let count = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        assert_eq!(count, 7);
        for index in 0..count {
            let at = 6 + index * 16;
            let size = if bytes[at] == 0 {
                256
            } else {
                bytes[at] as u32
            };
            let length = u32::from_le_bytes(bytes[at + 8..at + 12].try_into().unwrap()) as usize;
            let offset = u32::from_le_bytes(bytes[at + 12..at + 16].try_into().unwrap()) as usize;
            let frame = icon_data_from_bytes(&bytes[offset..offset + length]).unwrap();
            assert_eq!((frame.width, frame.height), (size, size));
        }
        assert_eq!(
            crate::brand_image::native_icon_bytes(&bytes).unwrap(),
            bytes
        );
    }

    #[test]
    fn header_and_about_share_one_cached_texture_in_both_themes() {
        for dark in [false, true] {
            let ctx = egui::Context::default();
            ctx.set_visuals(if dark {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            });
            let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                logo(ui, 18.0);
                logo(ui, 48.0);
            });
            let uploaded_master = output
                .textures_delta
                .set
                .values()
                .flatten()
                .any(|delta| delta.image.size() == [1024, 1024]);
            // Headless tests have no texture backend. Drain before assertions
            // so a failed assertion cannot cause a second destructor panic.
            output.textures_delta.clear();
            assert!(uploaded_master);
            let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                logo(ui, 18.0);
                logo(ui, 48.0);
            });
            let uploaded_again = output
                .textures_delta
                .set
                .values()
                .flatten()
                .any(|delta| delta.image.size() == [1024, 1024]);
            output.textures_delta.clear();
            assert!(!uploaded_again, "do not decode/upload on every frame");
        }
    }
}
