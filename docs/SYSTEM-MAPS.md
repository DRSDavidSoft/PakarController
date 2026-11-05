# PakarController: clickable system maps

This is a navigable companion to [the detailed architecture and gaps](ARCHITECTURE.md). The ESP owns board configuration and command admission; AVR firmware owns its physical inputs and peripherals. Clients keep drafts and display observed state. PortableShell supplies reusable terminal mechanisms.

Click a component to inspect its implementation. Links are pinned to source commit `0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b`, so this map remains inspectable as the code evolves. Arrows describe source-level relationships, not proof of deployment or physical behavior.

## Whole-system ownership

Arrows show application and transport responsibilities. Optional capabilities require matching build flags, runtime configuration and observed device support.

```mermaid
flowchart TD
    desktop["Desktop / CLI / TUI client"]
    mobile["Mobile client"]
    esp["Integrated ESP application"]
    s3["ESP32-S3 entry point"]
    avr["AVR firmware and physical peripherals"]
    wire["Shared wire framing"]
    led["LED scene and rendering owner"]
    auth["Access configuration and persistence"]
    policy["Supported-peer policy"]
    web["Browser client"]
    web -->|HTTP and WebSocket| esp
    click web "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/web/index.html" "Inspect web/index.html" _blank
    class web component
    cloud["Outbound MQTT and WebSocket"]
    esp --> cloud
    click cloud "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/CloudServices.cpp" "Inspect firmware/esp/src/CloudServices.cpp" _blank
    class cloud component
    desktop -->|selected transport| esp
    mobile -->|selected transport| esp
    s3 --> esp
    esp --> auth
    esp --> wire
    wire --> avr
    esp --> led
    policy -. admission contract .-> esp
    click desktop "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/desktop/src/main.rs" "Inspect desktop/src/main.rs" _blank
    click mobile "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/mobile/lib/main.dart" "Inspect mobile/lib/main.dart" _blank
    click esp "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/application.ino" "Inspect firmware/esp/application.ino" _blank
    click s3 "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp32s3/esp32s3.ino" "Inspect firmware/esp32s3/esp32s3.ino" _blank
    click avr "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/avr/avr.ino" "Inspect firmware/avr/avr.ino" _blank
    click wire "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/libraries/BoardCore/src/WireCodec.h" "Inspect libraries/BoardCore/src/WireCodec.h" _blank
    click led "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/LedGrid.h" "Inspect firmware/esp/src/LedGrid.h" _blank
    click auth "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/AccessConfig.cpp" "Inspect firmware/esp/src/AccessConfig.cpp" _blank
    click policy "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/interfaces/peer-policy.json" "Inspect interfaces/peer-policy.json" _blank
    classDef component fill:#202936,stroke:#71d5e8,color:#f0f3f7,stroke-width:2px
    class desktop,mobile,esp,s3,avr,wire,led,auth,policy component
```

## Terminal command and table lifecycle

ShellService owns connection/authentication, bounded output and dispatch. The TUI renders snapshots and returns actions; the application validates and applies them. On exit, mouse reporting is disabled before the alternate screen is restored.

```mermaid
flowchart TD
    service["Serial / Telnet / optional SSH sessions"]
    editor["Shared editor facade"]
    core["Portable command support"]
    props["Typed property metadata"]
    binding["Snapshots and action routing"]
    view["Shared DeviceView"]
    table["Shared table renderer"]
    service --> editor
    service --> core
    service --> props
    service --> binding
    binding -->|observed snapshot| view
    view --> table
    view -->|requested action| binding
    binding -->|application validation and dispatch| service
    service -->|bounded output to peer| editor
    click service "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/ShellService.h" "Inspect firmware/esp/src/ShellService.h" _blank
    click editor "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/TelnetShell.h" "Inspect firmware/esp/src/TelnetShell.h" _blank
    click core "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/PortableShellCore.h" "Inspect firmware/esp/src/PortableShellCore.h" _blank
    click props "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/ShellProperties.h" "Inspect firmware/esp/src/ShellProperties.h" _blank
    click binding "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/ShellTuiBinding.h" "Inspect firmware/esp/src/ShellTuiBinding.h" _blank
    click view "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/portable-shell/portable_device_view.hpp" "Inspect firmware/esp/src/portable-shell/portable_device_view.hpp" _blank
    click table "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/portable-shell/portable_device_table.hpp" "Inspect firmware/esp/src/portable-shell/portable_device_table.hpp" _blank
    classDef component fill:#202936,stroke:#71d5e8,color:#f0f3f7,stroke-width:2px
    class service,editor,core,props,binding,view,table component
```

## Refresh, validation and release boundaries

The component pin records current shared headers separately from historical reference snapshots. Passing host fixtures does not prove SSH wire interoperability, installed firmware or physical light/sound.

```mermaid
flowchart TD
    pin["Canonical component revision and hashes"]
    evidence["File comparison evidence"]
    sourceTest["Verify pinned files and wiring"]
    suite["Gated shell / loopback test runner"]
    workspace["Actual nine-page binding fixture"]
    firmware["Guarded firmware builds"]
    receipt["Refresh and migration acceptance notes"]
    pin --> sourceTest
    evidence --> suite
    sourceTest --> suite
    workspace --> suite
    suite -. source evidence .-> receipt
    firmware -. build evidence .-> receipt
    click pin "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/third_party/portable-shell/source.json" "Inspect third_party/portable-shell/source.json" _blank
    click evidence "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/docs/file-comparison-evidence.json" "Inspect docs/file-comparison-evidence.json" _blank
    click sourceTest "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/tests/portable_shell_source_test.mjs" "Inspect tests/portable_shell_source_test.mjs" _blank
    click suite "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/tools/test-shell.ps1" "Inspect tools/test-shell.ps1" _blank
    click workspace "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/tests/shell_workspace_test.cpp" "Inspect tests/shell_workspace_test.cpp" _blank
    click firmware "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/tools/build-firmware.ps1" "Inspect tools/build-firmware.ps1" _blank
    click receipt "https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/docs/REUSE-PREFLIGHT-RECEIPT.md" "Inspect docs/REUSE-PREFLIGHT-RECEIPT.md" _blank
    classDef component fill:#202936,stroke:#71d5e8,color:#f0f3f7,stroke-width:2px
    class pin,evidence,sourceTest,suite,workspace,firmware,receipt component
```

## Source index

These links also work in readers that disable clickable diagram nodes.

- [desktop/src/main.rs](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/desktop/src/main.rs)
- [mobile/lib/main.dart](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/mobile/lib/main.dart)
- [firmware/esp/application.ino](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/application.ino)
- [firmware/esp32s3/esp32s3.ino](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp32s3/esp32s3.ino)
- [firmware/avr/avr.ino](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/avr/avr.ino)
- [libraries/BoardCore/src/WireCodec.h](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/libraries/BoardCore/src/WireCodec.h)
- [firmware/esp/src/LedGrid.h](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/LedGrid.h)
- [firmware/esp/src/AccessConfig.cpp](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/AccessConfig.cpp)
- [interfaces/peer-policy.json](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/interfaces/peer-policy.json)
- [firmware/esp/src/ShellService.h](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/ShellService.h)
- [firmware/esp/src/TelnetShell.h](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/TelnetShell.h)
- [firmware/esp/src/PortableShellCore.h](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/PortableShellCore.h)
- [firmware/esp/src/ShellProperties.h](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/ShellProperties.h)
- [firmware/esp/src/ShellTuiBinding.h](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/ShellTuiBinding.h)
- [firmware/esp/src/portable-shell/portable_device_view.hpp](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/portable-shell/portable_device_view.hpp)
- [firmware/esp/src/portable-shell/portable_device_table.hpp](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/portable-shell/portable_device_table.hpp)
- [third_party/portable-shell/source.json](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/third_party/portable-shell/source.json)
- [docs/file-comparison-evidence.json](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/docs/file-comparison-evidence.json)
- [tests/portable_shell_source_test.mjs](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/tests/portable_shell_source_test.mjs)
- [tools/test-shell.ps1](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/tools/test-shell.ps1)
- [tests/shell_workspace_test.cpp](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/tests/shell_workspace_test.cpp)
- [tools/build-firmware.ps1](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/tools/build-firmware.ps1)
- [docs/REUSE-PREFLIGHT-RECEIPT.md](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/docs/REUSE-PREFLIGHT-RECEIPT.md)
- [web/index.html](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/web/index.html)
- [firmware/esp/src/CloudServices.cpp](https://github.com/atomicdeploy/PakarController/blob/0a80e8cc3aad5c0f8adcc554863a2f2805b38b1b/firmware/esp/src/CloudServices.cpp)
