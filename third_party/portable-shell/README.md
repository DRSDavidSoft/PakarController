# Portable Shell source integration

## Canonical components (current integration)

PortableShell commit `6d07087f1ad5bffe76dd68903bbb7f6705a78448` now owns the
shared C Telnet/terminal primitives, C event editor, thin C++ callback facade
and optional snapshot/action device view. The exact ten component files are compiled
from `firmware/esp/src/portable-shell/`; `source.json` binds their upstream paths,
commit and SHA-256. CI rejects drift. Do not patch that slice independently:
change atomicdeploy/portable-shell first, then review and update this pin.

This pin includes the Ardush AVR/STM32 integration: configurable prompt and
suggestion storage, optional shift/subtract decimal formatting and a contextual
Unicode navigation footer. Pakar keeps its existing capacities and native
formatting default. The new in-place tokenizer and asynchronous cursor probe
remain available upstream; they are not copied into Pakar without a call site.
`tools/test-shell.ps1` and the 27-entry source/hash gate pass for this update.
Actual firmware CI and device acceptance remain separate gates.

`TelnetShell.h`, `ShellTui.h` and the Terminal binding in `PortableShellCore.h`
are thin adapters. Pakar retains command metadata, authentication, property
validation, hardware dispatch, SSH/FTP, USB ownership and session scheduling.
The generic library does not gain authority to operate a board. The shared view
retains secret drafts, capability checks and LED/pixel/game actions; diagnostics
can be compiled out with `PS_DEVICE_VIEW_DIAGNOSTICS=0`. Default rendering adds
red rounded framing and magenta selection, plus real Unicode direction symbols.

PortableShell also gained Pakar's secret-history policy, complete escape-payload
sanitization and explicit scrubbing. Its rich C provider panels, UTF-8 command
runtime, batch mode and host adapters remain available in that same repository.
The event interface preserves the existing authenticated dispatcher. CPU type does not select a reduced presentation.

The comparison and reusable integration guide is upstream `PAKAR-INTEGRATION.md`.
The 17 older entries below remain historical source evidence, not another live
implementation. Ten current component entries bring the authoritative ledger
to 27 files. Their original Git history is preserved; nothing is silently retired.

Exact donor: https://github.com/atomicdeploy/portable-shell/tree/67c5dcb8a63f4055cbdaf02e9ff271c53b46410d
Requested branch: `copilot/organize-readme-into-plan`. License: MIT, retained in
`upstream/LICENSE` and in every original source notice.

`upstream/` is an unmodified source/reference snapshot, **not a second firmware
shell or compiled dependency**. Original README/implementation claims describe
the donor and are not Pakar acceptance claims. The one-byte CodeQL environment
marker is a Git symlink to `.` (mode 120000), inventoried but not installed: it
contains no program or reusable tool and would introduce recursive traversal.
Every file's original SHA-256, exact mapping, omissions and remaining acceptance
are in the existing `docs/legacy-source-ledger.csv` and
`docs/file-comparison-evidence.json`, tracked by issue #29.

## Historical adaptation (before canonical component extraction)

`firmware/esp/src/PortableShellCore.h` adapts PSCommand's name/help descriptors,
strcmp command lookup and VT100 callback emitters. The catalogue now owns command
recognition, aliases, maintenance admission, help and completion. It uses immutable
descriptors rather than heap-linked registration; dynamic command removal is not
exposed on a production controller. `edit` opens the existing real Pakar TUI.

`TelnetShell.h` uses the adapted VT100 output helpers and adds bounded command and
property completion. Tab extends a unique/common prefix; repeated ambiguous Tab
lists matches. Completion never executes, remembers a draft, exposes a password,
or replaces text before/after a cursor in the middle of a command. `set` completion
only lists writable property names, never secret values.

`ShellService.h` retains the authenticated, loop-owned command handlers. Typed
properties keep validated durable settings writes rather than donor raw-pointer
mutation. There is still only one device shell, reached via existing Telnet or
owned binary USB shell envelopes (`tools/device-shell.ps1`); no raw serial escape,
new unauthenticated UDP listener, blocking network loop or second API is added.

## Donor stubs have real owners, not placeholder substitutes

| Donor gap | Actual implementation | Verification |
|---|---|---|
| Serial init always returns -1 | `ShellWire.h`, `handleShellFrame`, `tools/device-shell.ps1` | Boot/owner token, sequence deduplication, output ACK and lease fixtures |
| stdio initializer declared but absent | `tools/telnet-shell.mjs` and USB client's Console input/output | Actual TCP/stdin loopback tests, interactive raw-mode cleanup |
| Telnet sample never binds or accepts | `serviceDeviceShells` | Production initializer/service compiled with two-client socket doubles |
| edit only prints and spins | `ShellTui.h` and `ShellTuiBinding.h` | Edit, submit, cancel, read-only, hidden secret and stale-field tests |
| Tab only rings bell | `LineEditor::complete` and command/property catalogue | Unique/ambiguous/property completion, bounds and password isolation |

Network terminal (Node, Windows/Linux/macOS):
```sh
node tools/telnet-shell.mjs --host ledcontroller.local
node tools/telnet-shell.mjs --host ledcontroller.local --execute "status"
```
USB terminal (PowerShell, existing framed transport):
```powershell
./tools/device-shell.ps1 -Port COM24
./tools/device-shell.ps1 -Port COM24 -Execute 'status'
```
These are explicit example destinations, not automatic connection or verified
currently attached ports. Interactive network mode accepts the board's password
prompt without local echo. Batch mode refuses an authentication prompt without
sending the command. A returned prompt is terminal completion, not a hardware ACK.
Timeout/disconnection is reported as uncertain and never replays the command.
The TCP client handles fragmented Telnet negotiation, socket/stdout backpressure,
bounded connection/command deadlines and restoration of terminal state.

No UDP command shell is exposed: the donor's broken UDP reply implementation is
not a safe authentication/ownership protocol. Existing Pakar UDP discovery is
unchanged; a dedicated UDP command transport is not claimed by this integration.

## Source comparison findings

- Donor serial functions are stubs; the telnet example neither binds nor accepts.
  Its UDP socket never records/connects a destination for replies. Pakar's actual
  USB transport, Telnet negotiation, multi-session leases and output ACK windows
  remain authoritative instead of copying nonfunctional transports.
- Donor escape parsing synchronously reads subsequent bytes; Pakar handles
  fragmented CSI/SS3/NVT input without blocking LED/UART work.
- Donor edit is a print-and-wait loop, not an editable TUI. Pakar retains its
  staged property/table editor and explicit command admission.
- Donor argv parsing uses strtok and heap allocation; its setters use unchecked
  atoi/atof and raw pointers. Pakar retains bounded validators and protected owner
  APIs. Arbitrary runtime float-variable binding is not a board requirement.
- History remains eight fixed entries with password/credential filtering and
  disconnect scrubbing, not heap strdup of every command.
- Platform samples' direct GPIO writes and unbounded startup/stdio loops are not
  installed: they bypass board pin ownership and production scheduling. The
  examples are retained as original source, not described as working Pakar tools.
- Donor Makefile's clean/install/uninstall and debug profile do not override the
  existing low-write, cross-platform guarded build. Tests run in the established
  native cache via `tools/test-shell.ps1`, not a second donor build directory.

No claim of universal superiority or physical acceptance follows from this import.
Focused local fixtures cover the imported code; live USB/Telnet sessions and
production firmware deployment require their separate acceptance gate.

## Rich device workspace

The shared view uses semantic colors, two setting/value columns, border-integrated category titles, viewport scrolling, cell editing, observed/draft F2 dialogs, centered numbered choice menus, typed toggles and numeric controls, stale-edit rejection and changed-row repaint. The table supports 40..240 columns and a bounded complete-frame budget. Explicit PS_DEVICE_VIEW_* flags select exclusions; no MCU mode selects them. Terminal settings expose Default/Cyan/Violet/Red/One Dark, true color, horizontal/vertical tabs, tabs inside the table and border titles. Application validation and physical side effects stay with the same owner. See docs/TERMINAL-UI.md and its before/after gallery.

Native fixtures and emitted-byte pictures are not installed-board proof. Event text input is still ASCII; AP configuration/restore and complete application parity remain open in the existing donor ledger.
