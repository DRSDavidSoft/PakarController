# One PakarController application icon

## Current high-DPI control-grid design

The prototype Circuitry raster is replaced by the editable
`assets/branding/app-icon.svg`: staggered illuminated white/emerald/mint cells
and a circular button on a deep teal tile. It is original vector artwork,
not an enlarged font glyph. Native resources and README use the rendered
1024px master; Android's adaptive launcher uses the same high-DPI PNG.
Windows ICO retains all seven donor-derived sizes (16–256), rendered directly
from that master. Existing role-specific board artwork and custom peer icons
retain their identity and priority.

`generate-application-artwork.mjs --render` deliberately invokes the external
Sharp rasterizer through PAKAR_SHARP_MODULE. Ordinary generation and `--check`
require no image package: source/output hashes in application-artwork.json
guard every committed derivative. The compact 64px base64 derivative remains
under the 4096-character identity budget for ESP/peer/Web headers. README,
native executable and Android launcher no longer upscale that transport icon.

The following deployment observations describe the historical prototype
checkpoint only; they are not evidence that these new assets are installed.

## Android launcher correction

The previous mobile import covered only the in-app header, leaving all five
Flutter template launcher PNGs in place. This was an omitted OS consumer.
`generate-branding.mjs` now preserves the exact shared master in
`drawable-nodpi/pakar_application.png`; legacy and API-26 adaptive launcher XML
use that drawable, a Studio background and mask-safe insets. Both manifest icon
and roundIcon resolve this same resource. The five obsolete template PNGs are
removed (recoverable from Git), not retained as density-dependent fallbacks.
No extra artwork source, dependency download, resize cache or donor build.
Compiled and streamed-installed the corrected preview without repeating the
Flutter test/analyzer pass. AAPT resolves the APK launcher to its adaptive XML,
whose foreground is drawable/pakar_application. The APK's optimized res/XZ.png
has SHA-256 05bc6ce48c57eb8bf6c8f1acd98f10389ae2e6fcc40b4bf41b09f9ce39c85783,
byte-identical to the canonical PNG, not a Flutter density fallback. Physical
ADB readback confirms MainActivity in foreground; the actual running UI was
captured and inspected. Launcher-grid pixels are not inferred from header pixels.
That historical 64x64 limitation is addressed by the current SVG/high-DPI
pipeline above. Runtime board artwork cannot rewrite the installed Android launcher.

The mismatch was an incomplete integration: the native PE/window/tray generated
Phosphor Circuitry, the native menu/About used a separate theme-tinted font
glyph, and the Web header hardcoded a diamond with no application favicon.
These were independent branding sources, not a deliberate design requirement.

Read actual Pealayer `src/branding.rs`, `build.rs`, `assets/icons/README.md`,
`web_ui/src/components/HeaderBar.tsx` and `web_ui/src/webPlatform.ts` completely.
Its resolved artwork feeds native windows/tray and the Web header/favicon;
runtime preferences do not silently rewrite executable or pinned icons.

## Canonical master and exact code imports

The former `assets/branding/app-icon.png.base64` was a 740-byte lossless PNG encoding of the
existing 64x64 native Phosphor Circuitry raster. Verified current warm-cache
native pixels SHA-256:
`d24d849f01bfeaa0cffef1cc9b861062b52e1a320f4c943608fb5fd952ded387`.
PNG SHA-256:
`05bc6ce48c57eb8bf6c8f1acd98f10389ae2e6fcc40b4bf41b09f9ce39c85783`.
No new drawing, font substitution, donor build, dependency download or duplicate
cache was needed. The neutral blue-gray color is the existing shell artwork;
branding is no longer independently tinted by each surface's accent palette.

| Actual donor code | Pakar consumer |
|---|---|
| `src/branding.rs::icon_data_from_bytes` | Imported unchanged in `desktop/src/branding.rs`; window icon and cached header/About texture decode the same master. |
| `src/branding.rs::native_icon_bytes` | Actual native-master preservation, Lanczos3 alpha-preserving resize, seven PNG-frame ICO directory/offset encoding in `desktop/src/brand_image.rs`. Only the AppConfig/playback resolver is replaced by supplied shared bytes; made host-independent for build use. |
| `shell_derivative_preserves_native_master_and_multiple_resolutions` test | Adapted to parse/decode every ICO PNG frame, as Pakar deliberately enables PNG-only image decoding. Native multi-frame preservation is tested too. |
| `builtin_icon_bytes`/`icon_bytes` embedded/no-cache behavior | Single Pakar master, decoded once in memory; no media states/preset copies and no disk cache. Full custom/preset/state resolver remains pending below. |
| `HeaderBar.tsx` resolved `img src={appIconPath...}` with application-logo alt text | Web header and login now render an image from the same master, not a diamond or font fallback. `tools/generate-branding.mjs` mechanically embeds it. |
| `webPlatform.ts` favicon/touch artwork binding | Static standalone ESP page binds favicon and touch image to the same resolved master. Inline data works before login and avoids a separate protected icon endpoint/external dependency. |
| `build.rs` application icon resource vs quick-action resource separation | Pakar PE/windows/tray use master derivatives; actual imported Phosphor quick-action rasterizer remains for semantic action icons, not application branding. |

Native consumers: executable resource, startup window/taskbar icon, tray,
toolbar application mark and About. Web consumers: authenticated header, login,
browser favicon and touch artwork. Semantic settings/hardware/action icons keep
their actual Phosphor vocabulary; they are not application-logo variants.

## Explicit unfinished useful code and reasons

- Pealayer `resolved_icon_path`, state environment overrides, Current/Classic
  presets, fallback/precedence tests and validated custom-icon preferences are
  still useful generic configuration work. Their filesystem paths and media
  playback state cannot be copied blindly to a standalone ESP. They remain
  pending with a shared board/client transport and bounded storage contract.
- `sync_native_window_icon` and source-signature caching are now adapted to
  firmware application metadata in `branding::sync_peer`. Connected name/title,
  icon, header/About texture and tray update in memory; stale/disconnected peers
  reset to host branding. Web header/favicon/touch use the same bounded PNG.
  Native execution is still blocked by the canonical cache guard; the icon
  editor, playback resolver/presets and installed/pinned resource rewrites are
  not claimed. See BOARD-APPLICATION-IDENTITY.md for exact contracts/proof.
- Header collapse/connection modal/capability-aware action cluster, localization,
  and WebPlatform install/update/share/notifications/haptics/audio/wake-lock/
  badge lifecycle remain pending full imports; reading those files or using
  their image-binding mechanism does not certify the whole files.
- Only media title, video state/chapters/seeking and media-session transport are
  excluded media business. Generic capability checks and preferences are not.
- Build metadata, external branding manifest/environment override validation,
  generic publisher/version/provenance resource fields remain pending. macOS
  mpv linker probing is excluded by the requested mpv-free controller.
- High-DPI artwork is now implemented above. Platform-specific Apple layered/
  tinted catalogs and installed/pinned-icon cache refresh still need their
  own packaging/runtime acceptance; no OS-wide cache deletion is performed.

`tests/branding_test.mjs` verifies high-DPI and bounded derivative identities,
four identical embedded Web images, native consumers and no-write generation.
Three Rust tests verify native pixels/window/tray derivatives, all ICO frames
and shared texture reuse in both themes. Source tests do not certify compiled
resource extraction or deployed runtime/board appearance. CI runs asynchronously;
no claim that the current running app or board has already received this patch.
