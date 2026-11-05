# Board artwork

Editable masters: table.svg (staggered illuminated cells), keys.svg (arcade
pushbutton). Both use a transparent 64-unit canvas, restrained two-stop hue
ramps and substantial outlines. PNG samples at 24 and 256 pixels are generated
alongside bounded 64px transport artwork. No text, board hostname, or pixel
count is used to infer a role.

`node tools/generate-board-artwork.mjs` uses the external Sharp SVG rasterizer;
set PAKAR_SHARP_MODULE to an existing Sharp installation or install Sharp in the
tool environment. `--check` verifies embedded outputs without that dependency.
Flutter and ESP outputs are generated from the same PNG bytes. Rust embeds the
same base64 sources, without network fetching or writing an icon cache.

Firmware selects artwork by the existing role build flags. Flutter uses the
acknowledged role when older firmware supplies the generic Pakar icon; valid
custom icons retain priority. Verified discovery metadata carries the resolved
application artwork into the device list. Rust includes role in its change
signature and uses the same generic-artwork fallback. Renaming a hostname does
not switch identity; fresh capability metadata does. No new protocol selector.

OS launcher packaging is separate: these are board identities, not two new
applications. Pakar's application mark is now app-icon.svg with a 1024px native/
README/Android master and a bounded 64px peer derivative; see BRANDING-PARITY.md.
Dedicated Apple
layered/tinted icon catalogs, Android role shortcuts, and per-role Windows
launcher resources are not implemented or tested by this change. Platform
references: Apple HIG App icons; Android adaptive icons; Microsoft Windows app
icon design. Their simplicity, masking and restrained-gradient guidance informs
the masters; this is not a claim of native packaging certification.

Validation: mobile peer-context and board-artwork tests pass; analyzer clean.
Native build remains gated on unrelated cell_table.rs review evidence. Firmware
advertisement is source-integrated, not flashed or physically accepted.
