//! Shared artwork resolver and the actual Pealayer native ICO conversion.
//! Build and runtime consume the same embedded PNG, never a disk icon cache.
use base64::{Engine, engine::general_purpose::STANDARD};

pub fn icon_bytes() -> &'static [u8] {
    include_bytes!("../../assets/branding/app-icon-1024.png")
}

/// Bounded identity-wire derivative of the same editable application master.
pub fn compact_icon_bytes() -> &'static [u8] {
    static BYTES: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    BYTES.get_or_init(|| {
        STANDARD
            .decode(include_str!("../../assets/branding/app-icon.png.base64").trim())
            .expect("validated bundled application icon")
    })
}

pub fn rgba_at_size(size: u32) -> Vec<u8> {
    image::load_from_memory(icon_bytes())
        .expect("bundled application artwork")
        .resize_exact(size, size, image::imageops::FilterType::Lanczos3)
        .into_rgba8()
        .into_raw()
}

/// Actual Pealayer src/branding.rs native_icon_bytes conversion, with resolution
/// supplied by our shared master instead of its AppConfig/playback resolver.
#[allow(dead_code)]
pub fn native_icon_bytes(bytes: &[u8]) -> Result<Vec<u8>, String> {
    if bytes.len() >= 6
        && bytes[..4] == [0, 0, 1, 0]
        && u16::from_le_bytes([bytes[4], bytes[5]]) > 1
    {
        return Ok(bytes.to_vec());
    }
    let image = image::load_from_memory(bytes).map_err(|error| error.to_string())?;
    let mut frames = Vec::new();
    for size in [16u32, 24, 32, 48, 64, 128, 256] {
        let resized = image.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let mut png = std::io::Cursor::new(Vec::new());
        resized
            .write_to(&mut png, image::ImageFormat::Png)
            .map_err(|error| error.to_string())?;
        frames.push((size, png.into_inner()));
    }
    let mut ico = vec![0, 0, 1, 0];
    ico.extend((frames.len() as u16).to_le_bytes());
    let mut offset = 6 + frames.len() as u32 * 16;
    for (size, png) in &frames {
        ico.extend([*size as u8, *size as u8, 0, 0]);
        ico.extend(1u16.to_le_bytes());
        ico.extend(32u16.to_le_bytes());
        ico.extend((png.len() as u32).to_le_bytes());
        ico.extend(offset.to_le_bytes());
        offset += png.len() as u32;
    }
    for (_, png) in frames {
        ico.extend(png);
    }
    Ok(ico)
}
