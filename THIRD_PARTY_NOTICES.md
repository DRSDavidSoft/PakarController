# Third-party notices

PakarController is integration code built from earlier AtomicDeploy projects.

- `desktop/src/rgb_input` adapts the actual PCController
  `Tools/Controller/internal/rgbinput` decoders, receiver and serial framing,
  with Pakar connection ownership and 480-pixel/cell-map output. Its SignalRGB
  plugin is copied and rebranded in `integrations/SignalRGB/PakarController.js`.
  Original copyright (c) 2026 David Refoua and PCController contributors; used
  under the MIT option in `docs/licenses/PCController-MIT.txt`. Exact dirty-donor
  hashes and concrete outstanding parity are in the existing source ledger.

- `tools/check-doc-links.mjs` adapts `localLinkReference`, Markdown heading
  normalization/anchor enumeration and local-link checking from PCController's
  `.github/scripts/repository-check.mjs` and `repository-policy.mjs`, read at
  `b0111ae0c80dc34149ea1044de7dd5488920301b`. Used under its MIT option; see
  `docs/licenses/PCController-MIT.txt`. Pakar adds fenced-example handling for
  link scanning, HTML branding links, repository boundaries and offline fixtures;
  donor-specific privacy/license/required-path policies are not copied wholesale.

- `firmware/esp/src/PortableShellCore.h` adapts command descriptors/lookup and
  VT100 formatting from `atomicdeploy/portable-shell` revision
  `67c5dcb8a63f4055cbdaf02e9ff271c53b46410d` (requested branch
  `copilot/organize-readme-into-plan`). Copyright (c) 2024 Portable Shell
  Contributors, MIT. The complete license and 16 original regular files are
  retained in `third_party/portable-shell/upstream/`; its root CodeQL symlink is
  inventoried but not recreated. The original C runtime is reference-only;
  the bounded adaptation is used by Pakar's single authenticated device shell.
  Tests adapt the donor's command/context/VT100 scenarios and extend them with
  non-executing completion, secret isolation and fragmented-input assertions.
  `tools/telnet-shell.mjs` also adapts the donor's TCP read/write callback model
  into real cross-platform stdin/stdout streams, with incremental Telnet
  negotiation, deadlines and backpressure instead of blocking POSIX calls.

- `tools/prepare-avr-image.mjs` adapts Intel HEX validation, AVR jump encoding,
  unused-vector validation and boot-chain merging from PCController
  `Tools/Bootloader/Urboot-Custom/build.mjs` at
  `b0111ae0c80dc34149ea1044de7dd5488920301b`, under its MIT option;
  see `docs/licenses/PCController-MIT.txt`. The hardware-specific custom
  TM1637 backend and its different pins are not enabled.
- The stock MiniCore UART0 Urboot image is GPL-3.0-only, independent of this
  repository's MIT integration code. See `docs/licenses/Urboot-GPL-3.0.txt`,
  the installed MiniCore package's source/notices and
  https://github.com/stefanrueger/urboot/tree/b25d491a0b16eda79e8c5c10dd861d16396c12ae.
  Packaging pins the exact stock HEX hash rather than silently selecting a
  different bootloader. A build does not establish physical installation.

- `firmware/avr/BoardPins.h`, `ShiftRegisters.*`, `SevenSegments.*`,
  `TonePlayer.*`, `pitches.h`, and `BootMelody.*` are imported from
  AtomicDeploy PCController commit `2a0e971e2bdaf186b4e841c64a53b516509beb37`
  under its MIT option. `BootMelody.h` has one include-path adaptation for the
  flat Arduino sketch directory; PakarController integration remains separate.
- `docs/legacy/esp8266_programmer_reference.ino` is an adapted ArduinoISP
  reference sketch. Its in-file BSD notice and Randall Bohn copyright remain
  authoritative.
- The desktop app's native palette strategy and dependency choices are adapted
  from the MIT-licensed Pealayer project by its contributors.
- The AP+station lifecycle, independent flash-backed network configuration,
  build entrypoints, and CI layout are adapted from AtomicDeploy pu850-esp
  commit `431104072952f694a35b3a83c19f13fbb098ca5b`. PU product/business logic
  and its legacy UART payload implementation are not included. The source
  repository has no standalone license file; this reuse is recorded explicitly
  because both repositories are owner-directed AtomicDeploy work.
- The shared script console/logger/sanitizer/exit categories, build selection,
  scoped native runner, dependency-check/apply flow, image summaries and opt-in
  deployment workflow adapt actual PU850 `Tool/upload.js`, `Tool/build-all.ps1`,
  language build helpers, `build.cmd`, `build.sh`, `upgrade.sh`, `.vscode/prebuild.*`
  and `.github` workflows/configuration at that same revision. New owners are
  `tools/script-console.mjs`, `script-runtime.ps1`, `pakar.mjs`, `build-all.ps1`,
  `firmware-dependencies.mjs`, `firmware-report.mjs`, `deploy-artifact.mjs` and
  root/CI entrypoints. See `docs/SCRIPT-TOOLING.md` and the original-byte hashes,
  source identities and concrete omissions in the existing file-comparison ledger.
- `libraries/BoardCore/src/DiagnosticText.h` adapts ASA0002E.h's actual
  `convertToHexString`, `getFormattedUptime` and `humanReadableBytes` at the
  same PU850 revision for live session/random encoding and telemetry display.
  Fixed caller buffers and unsigned bytes replace growing Strings; original
  author/repository provenance is retained here. No separate donor license
  exists in its complete Git tree; the same owner-directed reuse applies.
- `tools/download-firmware.mjs` and its executable fixture adapt actual PU850
  `Tool/Go/download.go`, `download_test.go`, `watch.go` and `watch_test.go` at
  the same revision: exact Range/length and all advertised hash checks, resumable
  verified partials, commit only after verification and unchanged-file guards.
  Current Pakar peer/auth/release admission and bounded chunked reads remain
  authoritative. `.vscode/settings.json` adapts donor workspace/settings file
  associations and generated-output filters; tasks invoke existing guarded plans.
- `tools/firmware-manifest.mjs` and its fixture adapt PU850 C#
  `FirmwareManifest.cs` and `SelfTests.TestManifestAsync` at the same revision:
  portable filename selection, exact raw/compressed records, bounded gzip and
  verification of both hashes. Existing explicit ESP8266 artifact compression
  invokes it and emits a separate `.gz.md5`; S3 application bytes remain raw.
  C++ puff.h/puff.c were read with Mark Adler's original notice, retained in the
  unchanged donor. Their implementation is not redistributed here: the existing
  Node zlib runtime supplies bounded gzip decoding rather than another decoder.
- The same backup implementation also adapts PU850 `Tool/C++/upload.cpp`
  `single_header`, `parse_header_md5` and `collect_download_hashes`: canonical
  Digest-MD5 validation and rejection of duplicate critical response headers.
  Node headersDistinct preserves the original duplicate evidence; no Windows
  WinHTTP/BCrypt stack, unsafe replacement/deletion behavior or puff code is copied.
- Shell `firmware.hash` and `build.date` read-only descriptors/getters adapt
  `LocalLib/Commands.cpp` Firmware_Hash and Build_Date at the same PU850 revision.
  They report SDK sketch MD5 and compiler build date/time, not installation time
  or an independent running-image integrity/slot/release certificate.
- `tools/script-console.mjs` also adapts PU850 C# `ConsoleUi`'s independent
  stdout/stderr terminal-color policy at that same revision. Existing Node
  rendering/sanitation remains the owner; no .NET/Chalk runtime is added.
- Optional verified backup-before-update sequencing in `tools/update-firmware.mjs`
  adapts actual Node `performUpload` and C# `UploaderApp.RunAsync` /
  `BackupInstalledFirmwareAsync` at the same PU850 revision. Existing downloader,
  shared peer identity, strict release/auth admission and one-write updater stay
  authoritative; backup failures never authorize update or automatic rollback.
- FastLED, Arduino ESP8266 core, MiniCore, WebSockets 2.7.2 by Markus Sattler,
  PubSubClient 2.8.0 by Nick O'Leary, xcap 0.9.8, eframe/egui, reqwest, and serde are
  build/runtime dependencies under their
  respective upstream terms; their sources are not vendored here. WebSockets
  is used under LGPL-2.1-or-later for the authenticated same-origin ESP8266
  event endpoint.

Historical PuzzleBoard, 40LED, and Puzzles behavior was audited and selected
game behavior was reimplemented behind the bounded PakarController protocol.
PCController hardware services listed above are reused as source rather than
rewritten. Exact inspected files and revisions are recorded in
`docs/LEGACY-CODE-AUDIT.md`.

Android advanced network dependencies: mqtt_client 10.12.0 (MIT; included
Observable code also carries the Dart authors' BSD-style notice), event_bus
2.0.1 (MIT), and crypto 3.0.7 (Dart BSD-style license). Original package license
texts remain in the dependency distribution and Flutter's generated license
bundle. mqtt_client implements the MQTT wire protocol; Android DNS-SD uses the
system NsdManager rather than a copied Go runtime. Owner-directed PU850 MNDP
TLV/metadata adaptation and concrete omissions are hash-bound in the existing
file comparison ledger; there is no claim of complete donor parity.

Android peripheral mirroring adapts the actual MIT-option PCController
`Tools/Controller/internal/hostbridge/buzzer_mirror.go` and its tests at
`704f44042e6bf46c4459bd63f37230460f221389`: timestamp cadence, bounded jitter,
finite deadlines and source cancellation. Android AudioTrack replaces the
platform-incompatible PC speaker drivers; concrete omissions and original-byte
hashes are recorded in the same file-comparison ledger.
