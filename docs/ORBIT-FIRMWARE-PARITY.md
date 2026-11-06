# Orbit firmware cross-review

Owning issues: [Pakar #3](https://github.com/atomicdeploy/PakarController/issues/3)
and [Orbit #24](https://github.com/DigitalogicIR/orbit/issues/24).

**Partial checkpoint.** Exhaustive inventory is not exhaustive reading,
comparison, integration or physical acceptance. No boards/settings were changed.

## Evidence and continuation

Use the existing authoritative `legacy-source-ledger.csv`,
`reuse-review-evidence.csv`, `file-comparison-evidence.json` and generated
`file-comparison.csv`/`FILE-COMPARISON.md`. There is no independent Orbit CSV that
can silently disagree with the donor ledger.

```powershell
./tools/generate-reuse-ledger.ps1 -OnlyOrigin Orbit
./tools/consult-reuse.ps1 -Concern orbit-firmware
./tools/consult-reuse.ps1 -Concern donor-audit -Origin Orbit -SourcePath firmware/modem_bridge/WebAssetPolicy.h
./tools/generate-file-comparison.ps1 -Check
```

Initial pinned inventory: all **153** firmware paths at Orbit
`6e92d963607152cf45d7a75d4d021317e4369ae6`. Selective refresh preserves every other
donor. Missing paths remain visible, and later file-set drift fails consultation.
Peripheral/business exclusion is a per-file/section decision after reading,
never a regex that hides a file from inventory.

## First bidirectional implementation

Orbit's `WebAssetPolicy.h` and `EventStoragePolicy.h` informed the production
`UploadSpacePolicy.h`: align allocation demand, retain 64 KiB plus a metadata
block, use 64-bit arithmetic, and check **fresh** free capacity before managed
file creation and each new chunk. Missing capacity fails closed. Never admit
writes using cached telemetry or delete unrelated files to make space. Both
LittleFS and optional SD supply their actual capacity probes.

Orbit now adapts Pakar's strict weekday/date validation, encoding negotiation,
ETag-list parsing and flushed-byte readback before advancing upload progress.
The paired review also fixes Pakar's early-return ETag bug: a matching first tag
must not conceal a malformed suffix. Existing compiled asset and API ownership
remain intact; no second Web app, protocol version or board behavior is added.

## Remaining work (not accepted exclusions)

1. Finish mixed runtime/control files section by section, OTA, networking,
   discovery, time, configuration/security, telemetry and Web/PWA utilities.
2. Relate generated headers/gzip to canonical sources and packagers. Validate
   decoded bytes; do not call binary hashing a completed source comparison.
3. Compare all read-only policies against actual Pakar counterparts. Complete
   reads awaiting comparison are explicitly separate from accepted mappings.
4. Add each justified modem/AT/GNSS/OBD/product-only exclusion, retaining generic
   sections where present. Pakar pixel/game/AVR code is outside Orbit adaptation.
5. Full paired ESP SDK builds, resource deltas, full CI and separately authorized
   live/hardware verification. No parity or production-complete claim yet.

## Explicit peripheral exclusions after complete reading

| Exact Orbit file | Code and reason not adapted |
| --- | --- |
| `firmware/common/board_pins.h` | Orbit carrier UART/DTR/NET/CAN/K-Line GPIO map; copying it would conflict with Pakar's reviewed board wiring |
| `firmware/i2c_scanner/i2c_scanner.ino` | Peripheral I2C scan sketch tied to that map; not an ESP service/OTA utility |
| `firmware/modem_console/modem_console.ino` | Raw AT UART passthrough for A7670E; no generic API/OTA/Web service to transplant |
| `firmware/modem_bridge/DiagnosticProfiles.h` | Generated modem AT query groups and modem response-field invalidation |
| `firmware/modem_bridge/HttpActionResult.h` | SIMCom `+HTTPACTION`/`+HTTPPOSTFILE` AT result grammar; Pakar has no SIMCom carrier |
| `firmware/modem_bridge/HttpBinaryFrames.h` | SIMCom counted `+HTTPREAD` framing, not an interchangeable Pakar USB/COBS carrier |

Only these exact files are excluded by this checkpoint. Mixed files remain
pending, even where they reference the same peripherals. Each exclusion also
has its own raw hash, complete read range and rationale in the main ledger.

## Current gates

Pakar's local managed-file test wrapper stopped **before compilation** on existing
Pealayer inventory drift. Do not bypass the guard, modify active donor drafts or
present tests as run. Orbit's four focused host suites passed (29 HTTP checks,
39 Web-space checks, 21 event-space checks and verified-write fault assertions),
plus source-wiring invariants. Native fixtures
are not physical flash/readback or network proof.

Canonical Orbit Gitea access initially failed SSH host-key verification. David
then confirmed a directly verified trust repair on this PC; a strict repository
read matched both remotes at `6e92d963`. No bypass was used. Mirror publication is
tracked separately from canonical merge/CI.
