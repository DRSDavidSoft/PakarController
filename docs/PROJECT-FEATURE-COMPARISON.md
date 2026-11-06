# PakarController and Orbit: useful-feature inventory

Source baseline: Pakar `a0628bd`, Orbit canonical/mirror `51d659e` (6 November
2026). This is a feature-level map, not a claim that every source file has been
reviewed or that every feature has passed fresh hardware tests. The exhaustive
file decisions remain in the existing donor ledger. A feature listed below has
an implementation/consumer, not merely an issue requesting it.

## What each repository is

- **PakarController:** a multi-client LED/game controller. ESP32-S3 owns the
  scene, effects, networking and maintenance; the AVR owns deterministic inputs,
  shift outputs, sound and the four-digit display. Table and key-box roles share
  generic services without sharing their business UI.
- **Orbit:** an ESP/modem management and fleet workstation. Its ESP services,
  Windows application, Web console, CLI and backend provide local and Internet
  management, with GNSS, cellular and vehicle tooling on top.

## Detailed inventory and transfer decisions

Paths in the Pakar/Orbit columns are repository-relative implementation evidence.
“Keep specific” means preserve the feature in its project, not discard its generic
utilities. “Candidate” means not imported by this checkpoint.

| Feature family | PakarController contains | Orbit contains | Useful cross-project action |
|---|---|---|---|
| Desktop workspace | Rust/egui dockable panels, saved workspaces, context menus, shortcuts, zoom, table/box selection (`desktop/src/ui`, `application_shortcuts.rs`) | WPF device/fleet pages, connection profiles, structured journals, capability explorer (`tools/AtWorkbench/Wpf`) | Compare contextual navigation, discoverability and journals; retain framework-native controls |
| Android | Flutter controls, Android USB/BLE/document/installer integrations (`mobile/`) | Web/PWA; no equivalent tracked Flutter Android application | Do not advertise native Android parity; reusable mobile Web tools are candidates |
| Web application | One canonical source, shared components, responsive panel layout, theme/density/language, installable PWA (`web/index.html`, `web/modules`, `assets/web`) | Shared responsive bilingual console, network tabs, fleet/cloud transport and PWA (`firmware/web`, `WebUi.h`, `PackObdUi.cs`) | Compare components and interactions, not entire duplicate apps |
| LED scenes | Persistent per-cell semantic states, configurable state colors, lock, mapping, animations (`LedGrid`, `SceneStore`, `StateAppearanceSettings`) | Configurable status RGB LED and automatic device-state indication (`StatusLed.h`, `LedProfile.h`) | Keep matrix/game semantics specific; compare generic status/effect utilities |
| Pixel tools | Diagnostics, physical/logical addressing, enabled correction rules, live previews and per-group pixel counts (`PixelCorrectionStore.h`, `PixelGroupStore.h`, `assets/web`) | No matrix or pixel-correction subsystem | Keep specific; do not import unrelated LED business features |
| Games and presentation | Board-owned key minigames, round/time configuration, reset, per-group effects; host-owned overlays/streaming and external Snake (`KeyMiniGameService.h`, `HostPresentationStore.h`, `mini-apps/`) | No contestant-box/game subsystem | Keep specific; generic job/stream progress remains reusable |
| Assignment documents | JSON/CSV and Rust/Flutter XLSX import/export, selection by state (`docs/CELL-ASSIGNMENTS.md`, client implementations) | Structured telemetry and exchange export, network evidence summaries | Reuse export UX and metadata, not incompatible business documents |
| Microcontroller maintenance | ESP OTA/backup, AVR ISP/Urboot flash/EEPROM and bootloader recovery (`MaintenanceRoutes.h`, `UrbootRoutes.h`, `AvrIsp`) | ESP URL manifest/check/download/install/cancel with Wi-Fi/SIM routes and running-image verification (`FirmwarePull.h`, `firmware-pull.js`) | Orbit resumable download/asset-only publication and Pakar maintenance UX are candidates; AVR stays specific |
| Files | LittleFS and opt-in microSD, searchable file rows, chunk transfers, original dates, raw/TAR downloads (`UserFileStore.h`, `assets/web/user-files.js`) | User-file chunk operations, HTTP ranges/date validators and storage metrics; Web UI previously required typed filenames (`StorageFiles.h`, `remote-tools.js`) | **Pakar → Orbit: searchable rows, metadata, per-file download/delete and overwrite confirmation** |
| CPU/performance | Heap/PSRAM/FS, identity/uptime, link and renderer metrics; no measured per-core CPU load at this baseline (`MaintenanceRoutes.h`) | Measured per-core/task runtime, component elapsed/max timings and temperature (`EspLoad.h`, `RuntimePerf.h`, `EspThermal.h`) | **Orbit → Pakar: measured CPU load and a readable Web resource monitor**; component timing/thermal controls remain candidates |
| Diagnostics and support | API inspector, event/activity UI, shell diagnostics and exportable API state | Structured exchange history, UART capture, performance pages, network capture/evidence exports | Session recording/history and compact support export are candidates; never export secrets/locations silently |
| Terminal | Telnet, editable history/completion, Unicode TUI, framed USB shell; optional SSH exec/SCP/SFTP/FTP bindings (`ShellService.h`, `ShellTui.h`, `Ssh*Binding.h`, `FtpBinding.h`) | Structured CLI/raw AT console and remote tools; no equivalent general embedded shell implementation found in firmware inventory | Pakar shell/property catalogue is a substantial Orbit candidate, not completed by adding an API command box |
| Network setup | AP/STA/both, saved station priorities, DHCP/static, optional Ethernet and host Wi-Fi actions | AP/STA, persistent addressing, Wi-Fi recovery, saved local/cloud device routes | Compare station-profile management and host-assisted association |
| Discovery/peers | mDNS/DNS-SD, SSDP/icons, MNDP, NetBIOS, peer routes/relay; optional Ethernet services | mDNS/DNS-SD, SSDP, NetBIOS, peer discovery and fleet identity | Compare real peer selection/event routing; basic discovery is not full bridge parity |
| Network utility tools | DNS, ICMP, public IP, HTTP Date/NTP and scan/configure (`NetworkTools.h`, `NetworkToolsBinding.h`) | Wi-Fi/SIM HTTP GET/POST, exchange inspect/export, DNS public IP, peer discovery and network tools (`NetworkTools.h`, `remote-tools.js`) | Orbit request composer/history → Pakar is a useful candidate; no false equivalence between Wi-Fi and SIM |
| Remote transport | HTTP/WebSocket, MQTT, outbound WebSocket; USB API/maintenance; limited BLE and streaming carriers | USB/WebSocket/MQTT and independent WSS, backend route selection over Wi-Fi or cellular | Shared semantics and UI mechanisms, not incompatible packet/opcode transplantation |
| Telemetry | Live board/AVR resources, remote management, client telemetry, configurable cloud endpoints (`CloudServices`, `ClientTelemetry`) | Compact and full typed telemetry, periodic HTTP JSON, exchange history, configurable tracking templates (`CompactTelemetry.h`, `HttpTelemetry.h`, `TrackingTemplate*`) | Compact snapshots/templates/history are useful Pakar candidates; product location payloads stay specific |
| Fleet/identity | Board/boot identity, peer introductions, selected-owner bridge and discovered clients | Factory-MAC identity, per-device profiles and independent fleet route/session state | Compare multi-device UI and remembered routes; preserve actual identity throughout |
| Notifications | In-app activity/toasts, optional ntfy, app/board update progress | Bilingual device errors, notifications, operation receipts and durable event delivery/ACK spool | Compare notification controls, session history and event delivery; this is more than error hardening |
| App distribution | Client-management service, APK/package discovery, presentation assets, enrolled support operations (`services/client-management`) | Backend/Windows packages and device firmware-host service (`backend/firmware-host`, `tools/ci`) | App-update discovery and asset-only UI delivery are separate useful candidates |
| Location | No cellular/GNSS product implementation | GNSS controls, satellite signal/navigation cards, last-known freshness, Wi-Fi and cell geolocation, maps and survey export | Keep specific; timestamp/freshness and chart/export helpers remain reusable |
| Modem/communications | No AT/SIM/call management product | SMS/PDU, calls, USSD, modem sleep, APN/operator/LTE selection, signal/latency comparisons | Keep business/peripheral layer specific; reuse generic queues, UI and jobs selectively |
| Vehicle/peripherals | AVR keys/relays/display/buzzer, optional LCD/W5500 | CAN/K-Line OBD, SOS/carrier pins and GNSS antenna configuration | Keep electrical/protocol ownership specific; no blind pin/driver copy |
| Build/docs | Firmware HEX/BIN and Windows/Android artifacts, resource budgets, generated contracts, detailed donor ledger | Canonical Gitea plus GitHub mirror, backend/Windows packages, generated catalog, wiki and device evidence | Compare tooling separately from user-facing feature parity |

## This pass and what is not complete

The bold transfers above are the first useful-feature implementation checkpoint.
Neither the earlier storage hardening nor this inventory closes the whole audit.
The next functional candidates are **Orbit HTTP composer/exchange journal →
Pakar**, **Pakar terminal utilities → Orbit**, **Orbit component performance
timing → Pakar**, and **Pakar application-update utilities → Orbit**. These need
actual implementation/consumer comparisons, not a generic “feature parity” tick.

No factory settings, saved scenes, correction rules, credentials or production
boards are changed by this source pass. Build/fixture results and any later
installation evidence must be recorded separately.
