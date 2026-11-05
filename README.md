<div align="center">

[🧭 Clickable architecture and data-flow diagrams](docs/SYSTEM-MAPS.md)

<img src="docs/assets/app-icon.png" alt="PakarController" width="112" height="112" />

# PakarController

### Connected boards. One contextual control workspace.

[![Build · Test · Release](https://github.com/atomicdeploy/PakarController/actions/workflows/build.yml/badge.svg)](https://github.com/atomicdeploy/PakarController/actions/workflows/build.yml)
[![CodeQL](https://github.com/atomicdeploy/PakarController/actions/workflows/codeql.yml/badge.svg)](https://github.com/atomicdeploy/PakarController/actions/workflows/codeql.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-10b981.svg)](LICENSE)
[![Rust · egui](https://img.shields.io/badge/Desktop-Rust%20%7C%20egui-f59e0b?logo=rust)](desktop)
[![ESP + AVR](https://img.shields.io/badge/Firmware-ESP32--S3%20%2B%20ATmega328P-06b6d4)](firmware)
[![Android preview](https://img.shields.io/badge/Android-preview-8b5cf6?logo=android)](mobile/README.md)

[Overview](#overview) · [Get started](#get-started) · [Architecture](#architecture) · [Build](#build) · [Guides](docs/README.md) · [Roadmap](https://github.com/atomicdeploy/PakarController/issues/2)

</div>

> [!IMPORTANT]
> This is an actively developed controller, not a certified production release.
> Current production acceptance is tracked in [issue #2](https://github.com/atomicdeploy/PakarController/issues/2).
> Build artifacts, source tests and screenshots are not proof of physical-board acceptance.

## Overview

An ESP32-S3 + ATmega328P controller with an mpv-free Windows workspace, embedded
Web interface, CLI/TUI and an Android preview. Controls must follow the connected
board's acknowledged capabilities; offline or unsupported peripherals are not
invented by the client. Pealayer supplies real reusable desktop foundations,
PCController supplies protocol/contracts, and the earlier boards supply hardware
mapping and reusable drivers. Exact reuse gaps remain tracked rather than hidden.
ESP8266 source and explicit tooling remain preserved, but its full image currently
exceeds physical RAM/flash budgets and is not a release target. See
[the handoff scope and measured limits](docs/HANDOFF.md#release-targets).

Each firmware build selects one board role using `-GameMode table-selection`
or `-GameMode first-wins`. The table role provides a persistent red-to-white
lock and atomic cell-color imports. Rust and Flutter support genuine Excel,
CSV and JSON documents; the embedded Web UI supports CSV and JSON.
See [board roles, lock semantics and assignment APIs](docs/CELL-ASSIGNMENTS.md).

## Get started

| Your task | Start here |
|---|---|
| Control a provisioned board | Open its configured hostname in a browser, or select it in the Rust/Android Devices view. Check the returned board identity before issuing commands. |
| Connect by USB, without Wi-Fi | Select **USB** and its actual enumerated port in the desktop app. Only one upstream connection is active; sharing is a separate opt-in. |
| Join a board's access point | Use **Host Wi-Fi** in the client. OS approval and network credentials are separate from API authentication. [Platform behavior](docs/HOST-WIFI.md). |
| Build on a new workstation | Run `pakar.cmd doctor` (or `bash pakar.sh doctor`), then follow [Build](#build). Default plans do not flash or install anything. |
| Develop or hand off work | Use the [documentation hub](docs/README.md), [contribution guide](CONTRIBUTING.md), and [current GitHub PRs](https://github.com/atomicdeploy/PakarController/pulls). |

### Downloads and verification

Engineering packages are attached to successful [Build workflow runs](https://github.com/atomicdeploy/PakarController/actions/workflows/build.yml).
Select the intended commit, not just the newest timestamp. Firmware, Windows and
Android outputs are separate artifacts; release packaging adds SHA-256 manifests
and provenance. A successful build is not a deployment or a stable-release claim.
The firmware artifact includes AVR application-only `avr.ino.hex` and verified
full-flash `avr.ino.with_bootloader.hex` for polling, busy-wire and buffered
links. Draft release ZIPs include both as well. The latter contains Urboot and
is intended for explicit full-flash/ISP recovery; neither HEX writes EEPROM or
fuses. ESP32-S3 application/OTA images remain raw `.bin`, not Intel HEX.
See [release policy and verification](docs/REPOSITORY-GOVERNANCE.md).

<details>
<summary><b>Web device-settings preview</b> — actual embedded UI, simulated device state</summary>

![Device settings](docs/screenshots/web-device-settings.png)

This screenshot is from a read-only HTTP fixture. It is not a live board or a
claim that every requested native/mobile setting is complete.

</details>

## Architecture

```mermaid
flowchart LR
    Desktop[Windows · egui / CLI / TUI] <-->|JSON / WebSocket / USB| ESP[ESP32-S3 · retained ESP8266 target]
    Web[Embedded Web interface] <-->|Shared API state| ESP
    Mobile[Android preview] <-->|IP API / BLE / native USB CDC subset| ESP
    ESP --> LEDs[Addressable LED cells]
    ESP <-->|COBS + CRC · UART| AVR[ATmega328P]
    ESP -->|ISP recovery / updates| AVR
    AVR --> Panel[Keys · shift registers · TM1637 · buzzer]
    ESP <-->|Configurable TLS links| Cloud[MQTT / WebSocket]
```

Read [the current architecture and explicit gap register](docs/ARCHITECTURE.md)
for source ownership, the 480-pixel Table versus 216-pixel Keys profiles, selected
transports, rendering/queues, discovery, updates and remaining acceptance. The
shared application lives under `firmware/esp`; S3 compiles it with
its own adapters, not an ESP8266 binary. App-package distribution is a separate
service from the MQTT board-control relay.

## Access and recovery

### Peer identity and supported releases

The coordinated source release is **0.2.8**; the authoritative client build is
recorded in [the shared release identity](interfaces/client-release.json). Apps and
firmware enforce the shared minimum supported release by default, independently
disableable at build time. HTTP headers, discovery and `/api/peer/info` identify
each party; `/api/clients` exposes bounded, nonsecret peer introductions. API
paths remain unversioned, with additive capabilities rather than build-specific
adapters. Older installed products need a coordinated update; this source/build
checkpoint is not a deployment. See [policy, build flags and recovery](docs/PEER-IDENTITY.md).

### Share a selected connection

Native **Preferences → Connection → Share connection** can publish the selected
USB or network board through the existing HTTP/WebSocket table APIs, with opt-in
mDNS/DNS-SD, SSDP and MNDP discovery. USB remains offline unless sharing is
enabled. Other clients use `http://<configured-hostname>.local:8789`; downstream
board identity and sole serial ownership are preserved. See the
[bridge contract and remaining scope](docs/API.md#application-api-bridge--repeater).
The native network bridge forwards guarded upstream firmware transfers; a USB-only
route never silently starts network management. Binary pixel streaming and full
PCController peer-RPC parity remain pending; real LAN acceptance stays open in issue #2.

### Device access

Authentication defaults off in the current firmware; older installations can
retain persisted credentials. Inspect **Device access** or use the explicit USB
recovery utility rather than guessing a factory password.
See [Access and recovery](docs/ACCESS.md) and the explicit
[PU850 parity audit](docs/PU850-PARITY.md). ESPAsyncWebServer now hosts the
Web UI/API/WebSocket, ESP OTA and AVR ISP in the same firmware.

Version 0.2.7 adds [pinned remote two-board updates and full operational telemetry](docs/REMOTE-MAINTENANCE.md).
The [50-route PU850 mapping](docs/PU850-ROUTE-PARITY.md) records exact adaptations and exclusions.
Physical and publisher acceptance remain open in [issue #7](https://github.com/atomicdeploy/PakarController/issues/7); no production deployment is implied by a passing build.

PakarController is a two-controller game platform built from the reusable parts of
AtomicDeploy's earlier PuzzleBoard, 40LED, Puzzles, pu850-esp, PCController,
and Pealayer work.

- The ATmega328P owns deterministic keys, shift-register outputs, the optional
  ninth digital-GPIO button (not ADC6), TM1637 display, buzzer and melodies;
  feature-gated arbitration supports ESP-owned game orchestration.
- The ESP32-S3 owns the MT16703/WS2811-compatible LED field, Wi-Fi/API/UI,
  streaming, self-OTA, AVR ISP recovery, and LittleFS firmware backups.
- The retained ESP8266 target shares the integrated firmware; Bluetooth is S3-only.
  Its full-image memory/flash overflow is unfinished support, not accepted release coverage.
- Both use the PCController COBS + CRC-8/ATM UART envelope at 38400 baud. The
  AVR uses its hardware UART; S3 uses a separate hardware UART (ESP8266 uses
  SoftwareSerial), while the USB UART
  is a second framed 115200-baud PC transport.
- The Windows controller uses Pealayer's eframe/egui design approach without
  linking or loading mpv.
- Optional outbound cloud links provide TLS MQTT telemetry/commands and a
  configurable WebSocket client. MQTT and WebSocket secrets are write-only and
  stored in checksum-backed LittleFS configuration.
  See [bidirectional commands, events and integration contract](docs/DUPLEX-CONTROL.md)
  for LAN/Internet HTTP, WebSocket and MQTT paths and their verification limits.
- The PU850-derived network toolbox also includes mDNS discovery and a
  password-authenticated Telnet command shell without taking over either UART.

No I2C peripheral is required. Optional S3 character-LCD support is capability
gated; the AVR A4/A5 TM1637 connection is not I2C and must not be shared with it.

See `docs/HARDWARE.md`, `docs/HARDWARE-VERIFICATION.md`,
`docs/ARCHITECTURE.md`, `docs/API.md`, `docs/TOOLCHAIN.md`, and
`docs/LEGACY-CODE-AUDIT.md` before wiring or flashing.

The native Windows app is in `desktop/`. It deliberately has no mpv or
`libmpv2` dependency. Its eframe, reqwest, serde versions and system-aware
visual styling are derived from Pealayer's MIT-licensed desktop foundation.
It can stream a generated rainbow or sample the primary screen into the 10 by
8 staggered table through the authenticated WebSocket.
The Streaming panel exposes a shared target rate (60 FPS by default; 0 is
uncapped) and measured completed host frame writes. This is not measured LED
refresh. Monitor/region selection and live frame-rate acceptance remain open.

## Build

Use the current main checkout and existing SDK installations:

```powershell
git clone https://github.com/atomicdeploy/PakarController.git
cd PakarController
.\pakar.cmd doctor
.\pakar.cmd all                         # plan only, no SDK compilation
```

Windows and Bash entrypoints share one dispatcher, staged output, validated
arguments and guarded build wrappers. Local tooling requires Node 20.19+ and
PowerShell 7; the Node services require Node 24+ (management uses node:sqlite).
The central [firmware pins](tools/firmware-dependencies.json) are unchanged by
builds; installation and dependency updates are explicit:

```powershell
.\pakar.cmd dependencies --plan
.\pakar.cmd firmware --target esp32s3 --plan
.\pakar.cmd firmware --target esp32s3 --link-mode buffered --s3-flash-mb 16 --s3-psram opi --s3-led-gpio --s3-led-rmt-dma
.\pakar.cmd desktop --mode Check --plan
.\pakar.cmd mobile --mode ApkPreview --build-only --plan
```

Linux/macOS use `bash pakar.sh` with the same options. Legacy `build.cmd` and
`bash build.sh polling` / `bash build.sh busy-wire` remain aliases; tests are
explicit rather than hidden build side effects. CI builds polling/busy-wire/buffered
AVR profiles and buffered S3; deployed peers need compatible build
flags. Every merged AVR+Urboot image must be exactly 32 KiB.
Default release packages contain the matched buffered S3+AVR pair. The explicit
legacy tooling does not make an overflowing ESP8266 image releasable.
The S3 example explicitly includes both runtime LED backends. Omitting both
inclusion flags intentionally builds no matrix driver; saved selection still
determines which included backend runs. Check the board's actual flash/PSRAM profile.

Run `.\pakar.cmd test protocol` for COBS/CRC bounds/corruption fixtures.
The [script tooling reference](docs/SCRIPT-TOOLING.md) covers common help,
JSON/quiet output, proxy support, dependency maintenance, artifact checksums,
automatic update plans, platform limits and exact PU850 source imports.

Build the Windows application with:

```powershell
.\tools\build-desktop.ps1                 # Check, no executable link
.\tools\build-desktop.ps1 -Mode Test      # Changed native behavior
.\tools\build-desktop.ps1 -Mode Release   # Only when an app update is needed
```

Build storage policy: [docs/BUILD-STORAGE.md](docs/BUILD-STORAGE.md). Native
debug/test symbols are disabled by default, with incremental reuse retained.
Build scripts refuse low free space or an oversized canonical Cargo cache.
Firmware compilation is incremental; use `-Target avr` or `-Target esp8266` for
an isolated board change and `-Clean` only when diagnosing a stale cache.
Linux supports `bash build.sh busy-wire esp8266` (optional `--clean`).
Use one stable staging directory and `tools/stage-desktop.ps1`; identical builds
skip copying. Fixtures can run directly from the canonical built executable.

Before any task or tool failure, follow [donor-first preflight](docs/REUSE-PREFLIGHT.md):
`./tools/consult-reuse.ps1 -Concern build-storage` (or capture, ui-shell, preferences,
windows-desktop, network-cloud, firmware-hardware, updates-telemetry, games-streaming).
Read actual matched donor code before a replacement or unsupported claim. Build,
stage and capture wrappers enforce hash-bound reviewed-slice gates automatically;
pending mappings remain visible and are not accepted exclusions.

The same executable provides scriptable CLI and interactive TUI surfaces. Set
`PAKAR_ENDPOINT`, and optionally `PAKAR_USER` / `PAKAR_PASSWORD`, then use:

```powershell
PakarController.exe --status
PakarController.exe --command cell 17 red fade
PakarController.exe --command effect rainbow
PakarController.exe --tui
```

Logical IDs retain their 10×8 indexing, but the current visual table has 20
interleaved columns and four rows: `1,11,2,12,...,10,20`. The first column is
`1,21,41,61`; the next is `11,31,51,71`, half a cell lower. Cell1 is not offset
downward. Each table cell has six LEDs; physical strip order is a separate
editable permutation. The optional board-owned Snake engine is excluded by
default; the removable host mini-app is documented separately and remains hidden
from normal board menus. See [host mini-app installation](mini-apps/snake/README.md).
The interactive terminal now has contextual pages, shared Find/selection,
explicit bulk white/red/off actions and the shared CLI command palette.
See [terminal controls and remaining acceptance](docs/TERMINAL-UI.md).
Staggered graphics, richer editors and full TUI parity remain open in issue #2.

## Continuous integration & delivery

The [CI/CD pipeline](.github/workflows/build.yml) checks generated interface,
branding and donor-evidence contracts, tests all three AVR link profiles, builds
the Windows client and analyzes/tests/builds the Android preview. It stops at
the first failed command. Timeouts, superseded-run cancellation and short artifact
retention bound wasted work. The tooling matrix checks shared CLI fixtures and
guarded command plans on Windows/Linux/macOS; these are not native package or
hardware acceptance. [CodeQL](.github/workflows/codeql.yml) scans Actions
and JavaScript; it does not claim Rust, AVR C++ or physical hardware coverage.
Maintained entry guides also have a dependency-free local-link and heading check,
adapted from PCController. It runs offline and does not crawl external websites.

The expanded pipeline supports version tags and explicit owner dispatch for **draft prereleases**, with
Windows and two-board firmware packages, Android preview, SHA-256 checksums,
source/run manifest and provenance attestation. No automatic flash occurs and
no development artifact is silently promoted to a production release.
PU-derived dependency checking and exact-artifact device deployment have separate
guarded workflows. Deployment is disabled until the owner explicitly configures
its runner/environment/opt-in. See [shared tooling and activation](docs/SCRIPT-TOOLING.md)
and [repository governance](docs/REPOSITORY-GOVERNANCE.md). Source configuration
does not certify that a current GitHub run passed or a device was updated.

## Documentation & contribution

| Start here | Purpose |
|---|---|
| [Documentation hub](docs/README.md) | Task-oriented reading order, tooling and verification boundaries |
| [Hardware](docs/HARDWARE.md) | Pin mapping and wiring |
| [API](docs/API.md) | Device transport and commands |
| [Bluetooth LE](docs/BLUETOOTH.md) | S3 offline Android control, identity, settings and transport limits |
| [Client updates & monitoring](docs/CLIENT-MANAGEMENT.md) | Configurable release endpoint, enrollment, telemetry and safe cleanup |
| [Architecture](docs/ARCHITECTURE.md) | Board responsibilities |
| [Source comparison](https://github.com/atomicdeploy/PakarController/issues/3) | File-by-file reuse and justified omissions |
| [Production checklist](https://github.com/atomicdeploy/PakarController/issues/2) | Open acceptance and delivery work |
| [Repository governance](docs/REPOSITORY-GOVERNANCE.md) | Labels, settings, CI/CD and release policy |

Read [CONTRIBUTING.md](CONTRIBUTING.md) before changing code or wiring. Report
bugs with the device/application versions, acknowledged capabilities and
transport; redact credentials and personal telemetry. See [SECURITY.md](SECURITY.md)
for private vulnerability reporting. Licensed under [MIT](LICENSE); donor notices
and applicable third-party licenses must be retained.
