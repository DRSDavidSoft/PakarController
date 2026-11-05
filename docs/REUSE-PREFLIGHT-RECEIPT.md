# Donor-first preflight receipt

## Firmware HEX delivery and high-DPI artwork, 2026-10-30 (#11 / #5)

The existing CI already emitted application HEX; the actual AVR packaging owner
also generated bootloader-inclusive HEX but artifact upload/report/release
omitted it. Reuse prepare-avr-image's actual Intel HEX parser/encoder and
full-flash image rather than a second objcopy or fake extension. Validate HEX
checksums/reset presence and compare every byte of merged HEX against its BIN.
Both HEX forms now participate in CI integrity reports and release archives;
ESP application binaries remain unchanged/raw and no board is programmed.

Read the actual donor branding/native ICO conversion, header artwork binding
and Web favicon owner. Retain multi-resolution ICO/Lanczos and cached textures,
but replace the prototype Circuitry art with an editable SVG control-grid mark.
The external existing Sharp rasterizer renders 24–1024px assets once; committed
source/output hashes make ordinary checks dependency-free and low-write.
Native/README/Android launcher use the high-resolution master while firmware
identity and inline Web art retain the strict 4096-character compact derivative.
Generic compact peer artwork still resolves by role rather than masking it.

An initial lookup used a nonexistent branding concern; consult the existing
application-identity route instead. The concurrent Pealayer owner's new tracked
paths required a selective inventory refresh before consultation; new rows
remain unreviewed, not accepted omissions. The existing Pakar PR43 Windows
WebSocket-bootstrap test failure is separate from this icon/HEX work; its
507-passing/one-failing run is not called all-green. No production runtime,
pin/cache configuration or hardware state is changed in this pass.

Branding/no-write generation, role/UPnP image metadata, Web identity, script
tooling, manifest/governance and actual release-ZIP fixtures pass. The retained
local outputs validate all six application/full-flash HEX artifacts; they are
not attributed to a newly rebuilt firmware image. Ordinary/check artwork paths
need no Sharp installation; 1024px native master is 94,542 bytes and the 64px
identity PNG is 2,570 bytes (within the existing base64 identity limit).

Selective donor refresh detected 13 unrelated previously reviewed files changed
by the active Pealayer owner. Keep those actual historical per-file pins rather
than falsely rebind their completed comparisons to unread new code; new tracked
paths are inventoried. Live-source checks still expose drift. The guarded native
branding test attempt stopped at the changed i18n donor before Cargo compiled;
it is not a passed Rust/runtime test. A scratch inventory helper initially hit
Node's 1 MiB subprocess buffer and was corrected to a bounded 4 MiB read; no
repository files were modified by that failed read. Generated comparison checks
pass after the explicit reviewed-pin preservation and changed-target rebinding.

## Canonical opcode ownership and namespace admission, 2026-10-30 (#41 / PC#702)

Read the actual complete PCController contract generator and canonical registry,
Pakar Protocol.h and active client routing slices, PC UART/Go dispatch boundaries,
and Pealayer's complete native/compact decoder source. Reuse the donor generator
as the shared portable source rather than create another hand-maintained table.
Native opcodes remain byte-stable; Pakar's existing published assignments become
an explicit second namespace in that same canonical source. Consumer source pins,
generated C++/Rust/Dart and generated Go names prevent independent numeric drift.

PC UART's advisory marker acceptance was an actual cross-namespace hazard:
Pakar0x11 audio could reach PWM. PC firmware and Go now reject every foreign
fixed marker; the actual C++ test exercises all255 and recovery to a valid frame.
Pealayer's native decoder receives the same gate and generated command constants.
Unknown operations inside the admitted namespace retain Unsupported semantics.
The explicit tunneled adapter is preserved; no payload semantics are conflated.

Five generator tests, stable-path native/UDP tests and actual UART fixtures pass.
The full96-test build-tool suite initially caught the new sync entrypoint missing
loadProjectEnv; reread its exact env loader/test and reuse the mandatory import,
without weakening the test. All96 then pass. Consumer/API/build and physical
acceptance remain separate. Local raw-GitHub upstream byte verification timed out;
offline pinned-byte/generation checks pass and mandatory CI upstream verification
is still independent evidence. No production board, settings or runtime changed.

The immutable four-file source pin was subsequently verified through authenticated
GitHub API byte reads, with no downloaded code executed. Flutter analysis and14 USB
tests pass; Pealayer's actual protocol suite passes11. Pakar's two compiled native
namespace/response-coverage tests pass. Name/access provisioners now reuse the
shared bounded serial owner rather than their duplicate decoders/loose ACK checks.
Actual mock provisioning/RX tests pass without opening a serial device.

Full protocol testing exposed an unchanged baseline fixture bug: its snapshot
slice ended at an earlier configureNtp forward declaration. Verified both source/
test files unchanged against HEAD, reread the actual definition and demand owner,
then require an end boundary after the asserted definition. The original demand/
privacy assertions remain and the scoped demand tests pass; the earlier aborted
large wrapper is not retroactively certified complete. Generated-symbol source
guards were updated without removing the actual wire/rejection/no-replay tests.

The broader compiled Pakar USB filter also passes all 49 tests, covering actual
serial actor, correlated responses, settings, snapshots and subscriptions. The
transport-convergence donor consultation passes with zero stale mapped sources.
The Pealayer inventory was refreshed after publishing its canonical imports;
the complete changed workflow was reread and its source hash rebound. Newly
inventoried files remain pending, not silently accepted as completed comparisons.
Local quality passed its prefix up to the stale generated comparison report;
after explicit metadata refresh, the failed report check and remaining suffix
were rerun separately. This is segmented evidence, not an invented single green
full-suite run. Paired source reviews are PCController PR704 and Pealayer PR223.
The existing structured mobile-plan test caught success text from the new opcode
preflight leaking onto PowerShell's object pipeline. Route that informational
output through Write-Host in all three build wrappers; keep the mandatory check,
nonzero-exit rejection and the existing plan assertions unchanged.

## Controller transport convergence first slice, 2026-10-30 (#41 / PC#702)

Read current repository instructions, transport and merge-readiness documents;
consult avr-convergence and exact donor-audit Project/ProtocolCodec.h mappings.
Read complete PC/Pakar C++ CRC/COBS primitives and current Go COBS, plus concrete
UART/header, UDP packet, Rust/Dart USB and Pealayer decoder slices. The latter
are slice review, not whole-file donor certification. Added a transport-convergence
index route so previously unindexed host UDP/framing sources remain visible.
Both root epics track actual unrelated-history/file-by-file merger requirements
and reference the existing MCU/firmware owners, not competing architecture epics.

The pre-fix Go conformance test accepts0200 as valid COBS. C++/Go/Rust/Dart
decoders now reject embedded zeroes without changing valid encodings. PC UDP
already reuses the Go primitive and now rejects short nonfinal fragments in its
encoder, matching the existing decoder. Pealayer already rejects embedded zeroes;
its runtime is left unchanged. Shared JSON fixtures are byte-identical in both
repos; no third protocol, socket owner, allocator or dispatcher is introduced.
PC's read-only MCU list exposes existing Mega/native lanes and explicit adapter
metadata; it does not claim an STM32/ESP hardware port or change328P programming.

Initial verification hit the existing hash gate after protocol_test.cpp changed;
no bypass was used. Existing proofs are rebound only after reviewing the bounded
diff and retaining anchors. A Go command initially requested a UDP package with
no tests; real UDP codec tests were added, not a test-selector exception. The
first UDP fixture compared nil and empty payload slices as different; byte-wise
payload comparison now retains every actual wire-field assertion. Source/CI,
installed apps, LAN loss/timing and hardware acceptance remain separate.

PC checkpoint f2b1b3ff is pushed in PR703: stable-path Go native/UDP tests pass,
canonical CMake firmware_protocol_codec_unit passes, and both MCU inventory
tests pass without a toolchain invocation. Pakar's canonical Rust test passes
shared_cobs_vectors; canonical mobile analysis reports no issues and all13 USB
transport tests pass. The shared fixture SHA256 is81683deafa749b231e483d2b75e4c45cae2a3262cab014ad0a20c74a82b31694.
The larger protocol wrapper passed its framing/renderer/storage/programming
fixtures but stopped at a later stale reference after a formatting-only source
edit; it was not labeled a complete passing suite. Final comparison checks and
CI remain exact-head gates. No source/profile tests are physical timing proof.

Repository reconciliation is not complete: the initial refresh found ten PC
PRs (before this new PR), including failing host/source checks on699, an unstable
692, blocked688 and several active drafts; Pealayer has seven, including unstable
drafts216/212. Owner worktrees are clean and preserved, along with22 PC and three
Pealayer stashes. They are not discarded or blindly merged. Pakar40's stale route
gate is repaired; its final full CI is still independent from this transport pass.

The full protocol wrapper's later isolated LLDP build then failed to find
GameRole.h via S3 BoardPins.h. Read the actual test-lldp/test-ethernet wrappers
and headers, consult ethernet/build-storage, and reuse the sibling wrapper's
BoardCore include directory. The unchanged LLDP codec/ownership/TCPIP fixture now
compiles and passes with SDK doubles. No production header, assertion or gate
was weakened; the larger wrapper is not retroactively labeled fully passed.
PC's new guide links initially failed its privacy policy; a document-and-repo
specific exception plus negative tests preserves unrelated/private-path rejection.
The13 policy tests and repository checker pass, and the corrected PC source
checkpoint is f122ccdf; its exact-head AVR/source CI passes while other jobs
remain separate gates. Existing unrelated PR failures/drafts are still open.

PR42's full CI then detected the independent original contract-manifest binding
to WireCodec.h still held the old target hash. The exhaustive comparison was
already updated; reread the23-entry source-mapping guard and focused record, then
refresh only that reviewed target binding and retained-behavior note. The original
donor revision/hash, every other mapping and the strict guard stay unchanged.
The mapping test now agrees with the authoritative comparison. Under the user's
renewed clean-worktree request, the one known generated pyc was removed normally
with native Remove-Item; this supersedes its old cleanup blocker. No alternative
shell, ignore entry, stash deletion or source/history loss was used.

After that binding repair the complete canonical `node tools/ci-checks.mjs quality`
runner exited0. The still-running runner was observed by its exact PID/command
and its retained process handle reported the exit code; it was not rerun merely
because earlier console output had scrolled away. Full paginated Pakar records
were refreshed with the existing auditor, and PR40/42 bodies and empty review/
thread sets were read. PR40 merged normally as d5008bd after all exact-head checks
passed; PR42 was retargeted to main. PC703's complete conversation is also empty
and all checks now pass. Separate final metadata/head CI remains required.

## Web correction preview connection/foreground ownership, 2026-10-30 (#3)

Reread the complete actual PCController mobile/lib/unified/pwm_control.dart
at its unchanged eda65965 snapshot hash and the complete existing Flutter
PixelCorrectionPreview. Read PixelCorrectionsPage's allowed, connection,
foreground/active and departure lifecycle slices, the current shared Web HTTP
owner, Web editor and firmware's real corrections/preview handlers. No donor
source changed. Two mistaken relative donor paths were absent; the actual
installed donor was read and hash-verified rather than substituting new code.

The current canonical page already embeds the preview controller and wires the
default-on, capability-gated preview-on-edit checkbox; staged-only ledger text
was stale. Its adapter nevertheless reacquired generic command admission and
identified ownership by board/boot only. Preview acquire/renew/read/release now
retain the original shared HTTP pin including connection generation. There is
no named WS preview command, so none was invented. The existing HTTP owner
still provides asynchronous preflight fencing, auth, deadlines and no replay.

Hidden-document and pagehide retire the temporary lease; foreground and BFCache
return remain passive. Disposal removes these listeners. Save remains the only
persistent correction writer; saved rules and diagnostic numbering are unchanged.
The DOM fixture now supplies the browser URLSearchParams API and unique lease
IDs per acquisition (the production lease contract), rather than reusing7 after
release. The pre-fix regression failed; focused editor/controller/HTTP tests
pass after the fix. No test assertion or ownership guard was weakened.

Only canonical full/onboard Web assets are rebuilt. Explicit workspace-panel
hiding, actual rendered lifecycle/RTL/touch and device/output acceptance remain
open. Browser automation remains blocked by unsupported Windows browser URL
policy enforcement; no alternate automation route is used. No production board,
settings, running client, firmware upload or actuator is contacted or changed.

Verification: editor, lease-controller, shared HTTP, single-source, app-contract,
dialog and control-presentation tests pass, as do living-contract and comparison
Check. Rebound51 changed-file references only after checking their prior hashes
and retained anchors; unchanged donor counts remain436 comparisons/4302 pending.
The first comparison generation correctly rejected an omitted canonical-page
reference; those four generated-page references were then explicitly verified.
Full/onboard builds and no-write checks match revision aa518a96eea2dd7d,
546206 HTML bytes /154031 gzip bytes. Existing read-only simulated preview at
localhost:64453 serves the updated source (HTTP200, simulated header); this is
not a new screenshot or rendered/hardware acceptance. CI remains its own gate.

PR40 CI41029329937 subsequently failed the independent PU route gate on all
three script hosts: the file-comparison bindings were current but two root/UI
route bindings still hashed the prior generated WebUi.h. Read the actual donor
root-registration slice and current target, then ran the canonical auditor on
the existing donor checkout. Exactly two SHA bindings changed; registrations,
anchors, line numbers and dispositions are unchanged. pu_routes_test now passes;
no route assertion or build gate was bypassed.

## Game picker acceptance gaps, side conversation, 2026-10-29 (#32)

Scope is the Flutter game-mode selector, not board deployment or the main
thread's other work. Read the current selector/tests and actual history: the
first rich row retained a fixed 64 px height and no number, while 7e013fab added
numbering and one two-entry test. Read the complete 233-line PCController
ActivityStrip source at its unchanged recorded hash; retain its themed icon
surface, bounded text and stable geometry, not a second command owner.

The fixed 26 px number column was not accessibility-safe. The new shared picker
measures number width, keeps digits LTR in RTL layouts, reserves both title and
muted description lines, and exposes full labels/descriptions to accessibility
and tooltips. Only advertised modes appear in board order. Parent ACK/peer
admission is unchanged; focus, opening and selecting the current mode do not
start or reconfigure a game. The canonical mobile capture selector additionally
accepts GameModePhone/GameModeTablet for actual-widget fixture inspection.

Canonical mobile wrappers report no analyzer issues and 33 passing tests:
22 minigame, four box-designer and seven picker tests. Picker coverage includes
all 11 advertised entries, 18 phone/tablet/RTL/text-scale combinations, bold
text, keyboard selection, disabled/unchanged choices, custom modes, description
fallbacks and closing the old popup on a board change without issuing a command.
The first fixture run exposed a lazy-list Finder error and a missed-tap warning;
the fixture now scopes to the actual popup and treats missed taps as failures.
Expanded phone and tablet actual-widget captures were visually inspected. These
are local source/fixture results, not installed-device or physical acceptance.
No APK, running app, board, saved game setting or Git branch is replaced here.

## SSH single-command and direct TUI entry, 2026-10-29 (#33)

Consulted ssh-transport plus exact PU850 donor-audit LocalLib/Shell.cpp/.h rows
at 4311040; read their complete session/dispatch code. Preserved the adapted
session-owned output and shared dispatcher instead of adding another command
catalogue or an OS shell. Inspected the pinned LibSSH-ESP32 5.10.0 command-request
parser, channel exit-status and close/EOF implementations. No PortableShell
vendored editor or concurrent shell integration checkout was overwritten.

Ordinary SSH exec was rejected while SCP exec was accepted. Added a bounded
128-byte command event after SSH authentication, preserving token/security
revision and maintenance guards. Non-PTY commands omit prompt/banner/echo,
report nonzero command failures, drain results before exit status, and survive
host stdin EOF. PTY tui/edit uses the same TUI, resize and input owner; exit
closes the exec channel rather than opening a second interactive shell.

New fixtures exercise actual production functions with host transport/queue
and peripheral doubles in both roles. Initial fixture compilation exposed
Windows libssh dllimport and the host `eof` symbol; the test now selects the
library's documented static API declarations and uses inputEof. The actual
DeviceView exit API and ArduinoJson String writer were used rather than
inventing an end method. No production validation was relaxed. Test results
and final SDK link are recorded with the checkpoint; encrypted board login,
live memory/LED coexistence and deployment remain issue #33 acceptance gates.

Final verification: tools/test-ssh.ps1 passed command/policy/NVS fixtures,
production transport API compilation with transfers included/excluded, both
role-specific actual shell/SSH/TUI bindings, public-file adapters and real
OpenSSH transfer clients over stdio doubles. tools/test-shell.ps1 passed the
existing editor/navigation, framed USB, authority, production session and
TCP/UDP loopbacks. Comparison generation/check and the living-contract guard
passed. A standalone optional guard was first called by the wrong filename;
the canonical tools/check-living-contract.mjs was then run successfully.

Canonical SDK build passed: ESP32-S3 keycontroller, buffered UART, 16 MiB/OPI,
both LED backends, BLE initialization excluded, W5500/acknowledged ISP and all
four optional SSH/SCP/SFTP/FTP services included. It reused the existing cache
with 12 workers; changing the resolved dependency list rebuilt vendor objects.
The actual application object was compiled after the final source changes.
Raw image is 2,734,096 bytes / 3,342,336-byte OTA slot (81.80%); static RAM
136,436 bytes is not measured free heap. SHA-256:
6715191bb505fcc587673e0c37c102619ff35d719d79f1e96507882dc0b02ed1.
No board, Android or desktop deployment, listener enable, reboot or actuator
test occurred. Existing runtime opt-ins and all default-off build flags remain
unchanged. The image is a build candidate, not encrypted/live SSH acceptance.

## PortableShell component consolidation, 2026-10-28 (issue #29)

Read portable-shell, device-shell, terminal-ui and donor-audit mappings; read the
complete live Pakar TelnetShell/PortableShellCore/ShellTui and bindings/tests at
b09650e3, and the canonical PortableShell C core. Upstreamed the reusable event
editor, Telnet and snapshot/action view as bounded components, retaining all
product dispatch/authentication and USB/SSH/transfer owners here. Source pin and
four complete-read mappings join the same existing ledger; old evidence stays.

A plain type alias failed the existing explicit `~ShellTui()` destruction fixture.
After consulting terminal-ui again and checking the real destructor regression,
a zero-state derived facade preserves that source interface and inherited scrub.
No destructor test was removed or weakened. Standard ANSI color replay support
was corrected upstream; inspected Agave images use the actual component bytes.

`tools/test-shell.ps1` passes catalogue/completion, editor, TUI, USB ownership,
pixel topology, diagnostic/SSH policy, auth persistence, two-platform production
session doubles, exact beep dispatch and TCP/UDP loopbacks. Full firmware build,
CI and physical USB/Telnet/board acceptance remain distinct; nothing is flashed.
Work is isolated from the active task's checkout and production processes.

GitHub's first CLI lookup failed TLS negotiation. The standard OpenSSL Git TLS
backend successfully verified the same remote; no certificate checks were
removed. Subsequent GitHub CLI requests worked normally. Source was preserved
throughout the connection failure; a failed lookup was never reported as a push.

## Handoff review: independent presentation storage, 2026-10-28 (#26)

Full source review found HostPresentationStore and StateAppearanceSettings both
owned `/presentation.json` and `/presentation.pending`. The actual combined-store
fixture reproduced the collision: a valid palette was treated as damaged host
presentation storage, and a persistent host save could overwrite its bytes.
Reconsulted games-streaming and pixel-group-layout; reread the actual PCController
Project/SettingsStore.h/.cpp, including explicit EEPROM-region overlap assertions,
validated load and unchanged-write ownership, plus both Pakar stores in full.

Host layers now use distinct `/host-presentation.json` and staging paths, keeping
the existing validated close/readback/rename implementation. There is no guessed
migration or automatic write on load. The regression uses both actual production
store types and verifies bidirectional independence through save, failed rename
and reboot, including untouched palette staging, correction and scene bytes.
This is source/fixture evidence only; no production filesystem was contacted.

CI 40745265472 next exposed the passive-RX fixture's 10ms preflight race on Linux:
the test could time out before START, then misleadingly report "Uncertain start
retried" because its count was zero. Read the complete actual capture helper,
shared serial-wire owner, RX/network fixtures and PU850's C++ integration test.
Only the RX fixture changes: deterministic EOF after queued replies, a 1000ms
preflight allowance, and exact failure-phase checks. One START/cleanup STOP and
no repeated STOP assertions remain. The shared network fixture still exercises
actual deadline expiry. Production serial timeouts and retry policy are unchanged.

## Cold-start matrix Fire and runtime direction, 2026-10-28 (issue #26)

Reconsulted led-benchmark, games-streaming and the PCController AddressableLeds
donor-audit rows. Reread both complete AddressableLeds sources at b0111ae0;
stage/compose/show ownership stays in the existing renderer. No AVR transport,
pin timing, alternate framebuffer sender or blocking delay is copied.
Read the installed FastLED3.10.6 Fire2012.ino and FireMatrix.h completely:
original-byte SHA256 respectively
14eb8ece1707c8bd472d64df6b49d0b24e52cc42a20229f9172000535f858cec and
994200db3111be15424bb04a57f3e9125402fb5b86f624f63f3f98dddf12b622.
Retained Fire2012's cool/diffuse/ignite/HeatColor structure; adapted lateral
diffusion and full-base ignition to the real staggered20x8 half-cell geometry.
FireMatrix's matrix-coordinate separation informs orientation; its100x100
buffers, compiler UI, palettes and Perlin dependencies are not imported.
The donor MIT notice is retained in docs/licenses/FastLED-MIT.txt.

Found two regressions: retained heat on explicit re-selection, and the grouped
static service guard freezing Fire after its first frame. Fire now publishes
a cold frame on every explicit selection, then advances on a rollover-safe
16ms simulation clock with bounded catch-up, independently of output cadence.
Both existing GPIO/RMT paths receive the same full mapped frame. Semantic cell
states, custom group lengths and correction rules are not rewritten. The table
heat field uses160 bytes rather than the old480-byte cable heat buffer.

The existing validated presentation patch/store owns fire_rotation in degrees.
It persists independently of the unchanged binary scene record; omitted fields
keep their values, old palettes default to0, and failed saves do not apply.
Web, Flutter and Rust expose only observed table support, send partial patches
through their existing command owners, and adopt confirmed live readback.
0/180 cover opposite sides;90/270 provide the other matrix orientations.
Reading status or opening a panel does not select or restart Fire.

Actual renderer/scene/queue/stress fixtures pass. New simulation/settings tests
cover cold replay, all rotations, full-width ignition, cadence/rollover,
transactional failure, no-op saves and unrelated scene/correction preservation.
Table/key-role and variable-group renderer fixtures pass. Corrected a pre-existing
route assertion: managed public-file writes already exist; private/raw settings
remain gated. Web command/readback/mount/backend and single-source asset checks
pass. Flutter analysis and3 focused widget tests pass;3 Rust appearance tests
pass. The isolated Web preview was visually inspected and accepted180-degree
memory-only readback. It is not a real board or optical acceptance.

Final incremental ESP32-S3 table-selection SDK build passes in the canonical
buffered cache: program2,325,955/3,342,336 bytes and static RAM138,436/327,680.
The16MB/OPI candidate includes GPIO and RMT-DMA, RMT fresh-board default,
GPIO48 status RGB, acknowledged ISP and optional Ethernet; BLE and optional
SSH/SCP are excluded. Existing saved preferences still win over build defaults.
build/esp32s3-buffered/esp32s3.ino.bin SHA256:
4a1b345d687b805c3b2c515a56176eb96513233baae6c91eaaf7d12bc8e3906c.
Full/onboard Web assets match revision889acc3167ad48184249846b114f6011d6ac35d8ad347fda83aa19762aed4a42
(152,981 gzip bytes). Screenshot: build/ui-evidence/fire-rotation-20261028/web-direction.png.
No board was flashed, reset, reconfigured or actuated for this task. Real cold
ignition, rotation and smoothness on the480-pixel table remain an approved
hardware test. The pre-existing PcControllerRuntime.h SOURCE-MAPPING.json
certificate failure was reproduced and remains a separate CI acceptance gate.

## Device terminal polish and Simon default, 2026-10-28 (issue #29 / #32)

Reconsulted device-shell, terminal-ui, PU850 Shell.cpp donor-audit and
games-streaming. Reread the actual PU850 Shell/Telnet/Commands sources at
4311040, portable-shell command/VT100 sources at 67c5dcb8, and RayanLamp's
contextual key/form ownership and terminal key-byte routing. Retained incremental
input, scrubbing, bounded output, role gates and shared command owners. No Go
runtime or blocking donor editor loop is imported onto the ESP.

The concrete missing command was `beep`: it was absent from the catalogue and
dispatch. The new finite Hz/ms parser feeds the existing buzzer/AVR owner. An
actual-source fixture compiles both shell and AVR-route branches together and
checks bytes, bounds, rejection and no replay. Muted descriptor-only suggestions
use the existing editor; accepting them never executes. Help groups and wraps
the existing catalogue. TUI uses readable labels and accepts additional CSI/SS3
navigation, smaller viewports, in-place redraw and a 350 ms standalone-Escape
delimiter on Telnet/framed USB. Full Unicode editing/RTL remains open.

Canonical shell/property tests, production metadata/service fixtures, TCP/UDP
loopback, actual tone dispatch and all/excluded-mode game fixtures pass. A changed
message assertion initially failed; its abandoned Windows fixture process then
locked the cache executable. After the automatic Stop-Process was denied, no
alternate filename, kill utility or cleanup workaround was used. The process
subsequently exited; the same canonical executable rebuilt and passed. The
first tone fixture also exposed a missing direct ShellWire dependency, fixed
with its existing bounded unsigned-number parser. No duplicated numeric parser.

Simon defaults are one round with eight steps on new/default mode selection;
saved records are not migrated or overwritten. ASUS SSH still refuses the
current localhost:8022 connection. This pass has not flashed, rebooted, changed
a saved scene, or sounded a production board. Installed terminal navigation,
audio and human acceptance remain pending; source/loopback/build results do
not replace them. The firmware cache is rebuilt with the unchanged keycontroller
hardware profile, GPIO fallback and included runtime RMT, BLE/SSH disabled.


## Host Wi-Fi join and management (issue #31)

Read the complete actual PCController Android DiscoveryChannel.kt, verified
original-byte SHA256 b4b829b34dfd99d1cadcd05b4d15b23e1277d6d8ffa82090f2abfd4843d48c5c.
Its main-loop callback ownership and close-time cleanup inform HostWifiChannel;
the donor contains no Wi-Fi join implementation. Android consent/network binding
and Windows Native Wi-Fi profile calls are new OS adapters. Existing Pakar shared
dialog/dropdown controls retain their fixed geometry. Re-read the touched device
picker and native connection-modal sections; unrelated transport/authentication,
workspace, firmware and document-picker behavior stays unchanged.

The current Android documentation and Windows WlanConnect contract informed the
implementation: OS save/request acceptance is not a verified peer connection.
Source checks must not join networks or interrupt production routes. The first
mobile analysis found five local style lints; those were fixed, not suppressed.
No donor availability or capability failure was worked around. Hashes below are
rebound only for explicitly reviewed changed consumers; hardware/installed OS
consent/routing acceptance remains tracked separately in #31.

## PU850 shell consistency follow-up (issue #29)

Re-read actual PU850 4311040 LocalLib/Shell.cpp/.h, Telnet.cpp/.h,
Commands.cpp/.h and RemoteCommand.cpp. The pasted 2a17b8c audit predates
6a8d911's group revision, terminal negotiation and Game page changes. This
follow-up uses current PixelGroupLayout::diagnosticRange rather than trusting
the earlier capacity-only text parser: single pixels or exact current physical
groups, explicit zero-based index/one-based group syntax, custom lengths and
revision forwarding. Shared role gates cover help/completion/dispatch.

CompStr identifier folding and upper/noupper input-display behavior are adapted
without changing values, secret handling, history bytes or command arguments.
The existing property owners now also expose station/AP flags and auto-switch;
separate Cloud/Notifications TUI pages cover the current descriptor catalogue.
Independent AP credential ownership, fallback-list forms, richer game override
forms and Unicode input remain pending; no fake property adapters were added.

RemoteCommand.cpp's listener/rebind/datagram/reply structure is adapted as an
explicitly built public read-only diagnostic listener. Its function-pointer
startup bug, truncated dispatch, broadcast hello, unauthenticated mutations and
unconditional success are not copied. Requests are bounded, nonce-correlated,
rate-limited and unicast; enabling authentication disables this public endpoint.
This is not full UDP control parity. No production device has been contacted.

The first test attempt correctly stopped at the donor hash gate after reviewed
consumer changes. Only the changed, rereviewed consumer/proof hashes are refreshed;
the gate is retained. Focused verification results are recorded on issue #29.

Final source fixtures passed: canonical shell/Telnet/TUI/serial/access/service,
live-topology pixel parser, UDP lifecycle and real loopback client, four property
profiles and actual getter/TUI-catalogue coverage for both S3 roles. Comparison
generation/check and whitespace checks passed. Final S3 first-wins SDK compile
with UDP included uses 2,311,887 bytes program/125,732 bytes static RAM; image
SHA256 `736d4c9f462730cd5cc7f186dd15ad59aac6ba2291ea43330049651b685d56e7`.
Final table SDK compile with UDP excluded uses 2,309,735 bytes program/138,724
bytes static RAM; image SHA256
`4dba1664ea561719b25ba381071991b157d233ac34570a2f26a9ff6c3468670b`.
Both profiles use buffered UART, 16MB/OPI, Ethernet, both LED backends and BLE
disabled. Only the table carries its acknowledged ISP electrical build flag.
The canonical cache holds the last table candidate; no duplicate release-cache
copy, flash, reboot, network query to a board or settings change was performed.

Previous-head GitHub run 40553395778 failed on a stale basic_cells_test assertion
requiring direct request->send after the route moved to StaticAssetService.
Reviewed the actual bounded SnapshotResponse, HTTP fixture and donor
AsyncWebServer.cpp/.h; updated the assertion to require the current immutable
GET/HEAD route, explicit asset length and validator. This keeps the guard rather
than removing it. The basic-cell behavioral fixture passes. This is not a claim
that the next complete GitHub run has passed. A route-hash preflight also rejected
unnecessary entrypoint includes; dependencies now reside in the shell header,
leaving application.ino and its unrelated route evidence unchanged. One local
PowerShell report pipeline failed on formatting objects; generation was rerun
through Out-Null without filtering the generator's internal formatting records.

## Portable Shell integration (issue #29)

Read all 17 tracked entries from the exact requested branch revision
`67c5dcb8a63f4055cbdaf02e9ff271c53b46410d`, including all code, tests, examples,
build rules, license and documentation. The 16 regular source files are preserved
verbatim under `third_party/portable-shell/upstream`; the CodeQL root symlink
targets `.` and is recorded rather than recreated. Donor checkout was not changed.

The compiled adaptation is `PortableShellCore.h`: PSCommand name/help catalogue,
lookup and VT100 callback output used by the existing shell editor/service. Help,
command aliases, completion and maintenance admission share that catalogue.
Added real command/property Tab completion; `edit` opens the actual staged TUI.
No second shell, raw serial interpreter, UDP execution listener or unchecked
parameter pointer setter is installed. Preserved existing password/history
scrubbing, Telnet negotiation, USB replay fencing/output ACKs, leases and partial
write backpressure. Exact per-file omissions are in the existing comparison ledger.

The first new fixture caught a missing help descriptor for `noupper`; fixed and
rerun successfully. `tools/test-shell.ps1`, `tests/shell_metadata_test.mjs`,
`tests/reuse_preflight_test.ps1`, the 17-file snapshot check, donor gate and
comparison `-Check` passed. Final ESP32-S3 buffered build passed: 2,302,399 bytes
of 3,342,336-byte application budget; static globals 138,724 bytes. These are
compiler figures, not runtime heap or timing measurements. The profile retains
16 MiB flash/OPI PSRAM, both LED backends, RMT default, BLE disabled and optional
Ethernet. No AVR/client rebuild was needed for this ESP-shell-only change.
Production firmware and live USB/Telnet sessions remain separate gates; no board
was contacted, flashed, reset or actuated by this integration.

Follow-up stub audit: existing production `handleShellFrame`/SerialChannel replaces
the serial stub; `serviceDeviceShells` actually accepts bounded Telnet sessions;
ShellTui/ShellTuiBinding implement editable, validated settings rather than the
donor's print-and-spin editor. Completed the missing bundled TCP/stdin client in
`tools/telnet-shell.mjs`. Local TCP fixtures verify fragmented IAC negotiation,
success/rejection, refusal to send a batch command at a password prompt, timeout
without replay, hidden interactive password input and terminal/raw-mode cleanup.
The augmented shell suite and three-profile property tests pass. No client-to-real-
board acceptance is inferred from the loopback server. Raw UDP command-shell
exposure is deliberately not introduced; it needs a separately designed owner/
authentication contract, not the donor's unconnected UDP send implementation.

## Static file serving comparison (issue #28)

Reread PU850-ESP `pu850-esp.ino` routes `/file/download`, `/`, and default
headers (bounded route review, not a new full-file completion claim).
PU's root sends text/plain; its catch-all LittleFS serving is commented out.
Its device-file download supports HEAD/ranges/date metadata but uses shared
PU transfer state and serial-pumping callback loops. Those loops are not copied
into public flash-asset serving or the isolated managed-file namespace.

Pakar now shares one static service across seven exact public asset routes:
bounded ACK-safe flash snapshots, GET/HEAD, byte-derived strong ETags with weak
If-None-Match matching, bodyless 304, revalidation, MIME/nosniff and explicit
Accept-Encoding handling/Vary. Compressed-only page/font return 406 when gzip
is explicitly unacceptable; the uncompressed /basic recovery page remains.
Range downloads remain in ArtifactDownload/FirmwareTransfer, not misadvertised
for the static shell. Generic private filesystem access stays prohibited.
No claim of universally superior throughput, hardware acceptance, WebDAV or
full PU parity follows from this source comparison. See focused test results
and GitHub #28; HTTP semantics checked against RFC 9110 sections 8.8, 12.5.3,
13.1.2 and 15.4.5.

## Managed file workspace (issue #28)

Read the actual complete Pealayer `ServerFilePicker.tsx` and `preferenceFiles.ts`;
consulted the existing inventory, artifact read leases, shared USB maintenance
owner, PixelGroupStore staging/readback, and PU850 `/file/download` metadata/range
slice. Reused bounded listing/filter/explicit acceptance and stale-peer rejection
in `assets/web/user-files.js`, rather than introducing React or arbitrary host
filesystem paths. Existing firmware backup routes and private settings exports
are unchanged. The new managed namespace uses immutable data and two checksummed
catalogue generations, since FAT cannot assume LittleFS rename replacement.

The first focused compile rejected misleading single-line condition indentation;
split those statements and retained `-Werror`. No gate was disabled. Focused
store/UI tests now cover bytes/dates, explicit overwrite, integrity, interrupted
publication, private paths, late peers and generation fallback. New SDK compilation
and physical SD/power-cut acceptance are separate. No board or installed app is
contacted or changed. `docs/USER-FILES.md` states limits and no WebDAV claim.

## GitHub handoff consolidation and CI repair

Completed code-bearing verification at c1d2dd17: Build 40508742014 passed every
workflow/tooling/quality/firmware/Windows/Android job; CodeQL 40508742015 and its
standalone finding check passed. No acceptance check was disabled. PR #24 merged
normally as f4259db with an identical code tree; GitHub confirms #21–#23 merged
through their retained integration commits. Every original source/dependency and
temporary repair head is a verified main ancestor. Merged refs were removed only
after verification, and the local main checkout was fast-forwarded without reset.
No source stash/other worktree or dirty application work remains. The following
receipt commit is documentation/audit only, with separate post-merge CI.

Fresh paginated conversation review includes the final merged states and receipt;
only those four reviewed PR records plus the fully read new legacy issue are
rebound. Historical no-worker/current-branch wording is corrected without
certifying a full PWA import. An initial out-of-order metadata patch was rejected
without edits; placing its insertion before later hunks applied it correctly.
The README now shows the same explicit dual-backend flags as canonical CI.
Private settings/backups remain local; no board, installed app or Android device
was modified. Issues #2/#3/#25 retain production/donor/legacy unfinished scope.

The9bd8074 Linux protocol suite now passed through LEDs, queue/runtime backends,
cloud/network/OTA models, TM1637, audio, UART and USB. Its final old PC flag loop
still expected RF to be unimplemented, contrary to actual BoardFeatures.h and
RadioAdapter. Consulted api-contract and read those complete current sources,
PcController RF dispatch/receive, test-radio.ps1 and its adapter fixture. Retained
all six genuinely unsupported hardware failures, replaced the stale RF rejection
with the SAME real adapter fixture used by Windows, and added non-boolean RF
rejections. Raw forwarding, decoder validation, active-round TX rejection and RX
restoration are exercised; learning is still unsupported and no RF hardware is
claimed. Governance guards tie the script to those actual feature semantics.
Bare bash selected WSL with a Windows c++ and could not resolve /mnt/c paths;
the explicitly installed Git Bash supplied correct host path translation and
the complete flag script passed. Its linker emitted existing incompatible-search
warnings before finding usable libraries; no host PATH or compiler was changed.
Linux CI remains the Linux acceptance gate, not that Windows result.

The early e33f54c quality gate caught intermittent download_firmware_test
rejection failure before SDK work. Read the actual downloader/fixture and the
mapped pinned PU Go watch digestUnchangedFile/validateWatchCandidate functions
(blob61bdc4e0554059a142d83daec5f33b4993ebafa5), not an invented replacement.
The existing late same-size partial write can escape Windows stat timestamps
while a handle remains open. Reuse the existing bounded descriptor digest after
the awaited final peer-status read, compare both hashes, then retain all original
descriptor/path/size/time and atomic no-overwrite checks. The fixture now names
the failing scenario and verifies the late mutation actually ran and remains an
unpublished failed partial. Ten consecutive complete download fixtures pass;
backup-before-update integration is rechecked after rebinding only the two
reviewed target/test hashes. Its first guarded invocation correctly refused the
changed stale hash, rather than bypassing the gate. No device requests or writes
were performed; this is backup-integrity/source evidence, not physical OTA proof.

Actual GitHub54c0dbe passed native tests/package, Android analysis/tests/APK,
all quality/tooling jobs and CodeQL. All three AVR profiles and all three S3
variants also compiled, but the Linux host suite then failed because its
LedWire-dependent scheduler command omitted BoardCore's GameRole include path.
Consulted build-storage and the actual Bash/PowerShell protocol, pipeline and
worker runners plus LedWire/GameRole. Added the same BoardCore include used by
the working PowerShell runner to both affected Bash commands. Early tests now
create their build directory and precede SDK compilation; no scenario is removed.
Source governance checks the actual command paths/order. Also corrected the
final canonical S3 command to explicitly include GPIO+RMT: absent inclusion flags
intentionally produce no matrix driver and must not define the release image.
The no-driver source option remains intact; optional quiet/Ethernet outputs are
still replaced before upload. Actual Windows pipeline/worker fixtures and new
governance guards verify the narrow test/profile change; fresh Linux execution
and final-head CI remain required, not inferred from successful SDK compilation.

Fresh49b1a9b CodeQL re-exposed two real report-file races (alerts9/10): derived
unchangedWrite used exists-then-read, while artifact collection validated a path
then reopened it. Consulted pu-tooling and reread the existing descriptor-based
running-image verifier, report and manifest fixtures. Adapted its same-open-file
ownership into readArtifact: O_NOFOLLOW where supported, descriptor/path identity,
regular-file/root/4MiB bounds, fixed-size descriptor reads and final size/mtime/EOF
checks with guaranteed close. Missing optional files are handled from open errors,
not preflight pathname checks. Derived report comparison directly reads/catches
ENOENT; no original image/settings is rewritten. Fixtures cover valid raw bytes,
empty/oversized/nonregular/out-of-root files and symlink rejection where allowed;
legacy compression/manifest tests pass. One negative fixture initially chose a
containing root, so it correctly did not throw; corrected it to an unrelated
root. No security query is disabled or finding dismissed to obtain a merge.
The actual S3 build passes2318667/3342336B, globals133008/327680; full local quality
passes. New exact-head scan/build gates remain separate from these source results.

The owner explicitly selected ESP32-S3 + AVR as release targets, preserving
unfinished ESP8266 support. After moving the shared DiagnosticReceipt helper
include outside the optional correction-store guard, an actual ESP8266 polling
build passed compilation but failed real linking: BSS ends0x3fffdd20 against
0x3fffc000 (7456B over), and irom0.text is0x1054c4. No physical memory/OTA limits
were raised. The helper's cross-feature regression and USB host tests pass.
The primary CI now builds all three AVR UART profiles and buffered S3, while
default reporting/packaging requires the actual S3 application/AVR pair rather
than stale ESP8266 paths. Explicit legacy build/report/gzip/package functionality
and source remain; no file from the original preservation checkpoint was deleted.
The existing manifest's S3 chip-id9/hashed-application guard is reused, not a
new permissive parser. Real packaging fixtures pass nine primary assets, checksums,
overwrite rejection and separately opted-in legacy pairs; source governance and
repeat tooling fixtures pass. Revisited actual Pealayer CI release/provenance
and PU build size/hash/gzip mechanisms; retained immutable drafts and raw images,
not donor timestamp rewrites or gzip renamed to .bin. Only the changed reviewed
workflow/report/package/tests/docs hash bindings are refreshed. No unread donor
rows are promoted. The receipt is docs/REUSE-PREFLIGHT-RECEIPT.md (an initial
root-path lookup missed it); the sparse PU workflow is absent and was not
invented. GitHub native/Android checks atceb4be5 both passed; final-head firmware
checks remain required. No board, installed client or production scene was touched.

Windows CI at8f676bf passed compilation,461 tests, real PE/resource startup and
all built CLI fixtures, then failed the focused23-file PC contract certificate.
The authoritative exhaustive ledger already had current hashes; the smaller
SOURCE-MAPPING.json retained five obsolete target hash/line bindings. Consulted
the actual indexed api-contract and exact UartProtocol.h donor route (an initial
guessed pccontroller-contract concern was rejected, not bypassed). Read the pinned
20c6115 UartProtocol.h from GitHub, current complete Protocol.h and runtime h/cpp,
and their RF changes: optional raw reception/validated transmit do not replace
the MacroRing owner or envelope; new audio-stream/USB API opcodes retain48-byte
COBS/CRC framing and isolated tunneled donor namespace. Rebound only those five
target entries, retained original donor hashes/revision and clarified stale
ADC6/ESP8266-D2 narrative. No source feature or validation assertion was removed.
Moved this pure static certificate check into the early quality phase so stale
projections fail BEFORE expensive firmware/native builds. Its23 mappings pass.
The failed native job is not mislabeled green; fresh final-head CI is required.

Firmware CI40494129965 exposed a real AVR overflow:32608B against the32384B
Urboot-safe boundary. Consulted build-storage/firmware-hardware, the actual
MiniCore3.1.3 LTO menu/recipes and the existing canonical wrapper/donor runner.
The wrapper's compiler.cpp.extra_flags override erased the selected menu's
-flto while C/link still retained it. Restoring that selected C++ option (AVR
only) produced28318B/32384B,1486B static SRAM; busy-wire28564B and buffered28498B,
each with vector/384B Urboot-chain validation passing. The actual build/storage
Plan regressions pass. No features, bootloader bytes, upload limits or fuses were
removed/changed; no device was contacted. Actual Plan regression checks now
require both the selected LTO profile and retained C++ option. Android CI at
8f676bf completed its analysis/tests/APK successfully. Fresh GitHub firmware
and final-head package verification remain separate gates.

A read-only review initially used two nonexistent guessed maintenance filenames;
the real inventory led to BoardCore/FilesystemInitialization.h and UsbApiService.h
plus the guarded initialization tool, which were read without replacement code.
A mechanical hash patch initially used the task-root-relative path and made no
edit; the absolute repo path was then used. Only reviewed wrapper/test hashes
were rebound; no unread source rows were promoted.

The complete local quality phase at80cd685 passed through repository governance,
including ledger/route hashes and all registered Web/host contracts. The fresh
GitHub CodeQL run40493579318 completed successfully, including the standalone
CodeQL finding check; all seven new review threads are resolved. Complete PR21-24
conversation decisions are hash-bound in the existing audit. The pending build
workflow is not claimed successful, and no Copilot or device acceptance is
inferred. Only superseded owned workflow40492884038 was canceled to release
the latest exact-head checks; no acceptance check or query was disabled.

Post-dependency verification at57244ea passed Rust461/461 and Flutter722 with
3 environment/live skips; analysis is clean. The fresh GitHub security scan
confirmed the descriptor/configuration fixes, but found an additional forgiving
HTML-end-tag case in the test-only script extractor. It now includes whitespace
and ignored end-tag attributes, with negative syntax tests for all variants.
No security check was disabled. Exact final-head CI remains the merge gate.

Dependency integration preserves the original PR21/22/23 commits through normal
merges, not squash/cherry-pick deletion of their history. PR23 archive4 requires
its real rawContent.getStream(decompress:false) and CompressionType.deflate APIs;
bounded manual inflation/CRC checks remain, and XML7 namespaceUri replaces the
deprecated argument. Locked pub restoration succeeded through authorized10808.
PR22's mdns-sd0.21.6 fix is retained. Its eframe0.37 bump is deliberately NOT
retained: dock/Phosphor/elegance still require egui0.36, so the new graph contains
incompatible Context/Ui types. Restored the proven coherent graph while keeping
the exact PR mdns version/checksum, and added a negative-tested lockfile type-family
guard to CI. The coordinated egui0.37 migration is deferred, not hidden as done.
Offline Cargo resolution lacked a compatible cached transitive graph; an online
metadata attempt stalled and only the two exact owned cargo-update processes
were stopped. The existing proven lock graph was reconciled explicitly rather
than cleaning/rebuilding SDKs or waiting on unbounded network retries. No cache,
runtime or user settings were removed. Post-dependency native/mobile tests and
fresh GitHub CI remain required before main merge.

Final pre-dependency mobile pass:722 tests passed,3 environment/authorized-live
skips, analysis clean. Fixed a real Hardware ExpansionTile storage collision:
its boolean expansion state now has a distinct peer-scoped PageStorageKey and
cannot read the parent scroll position as a bool. Unknown output retains disabled
geometry rather than disappearing, and Rainbow test remains disabled until the
output is observed enabled. Canonical Rust461 tests and Web/contracts passed
locally; this is not exact future dependency-head CI or device acceptance.

Reviewed all seven PR24 CodeQL annotations. Candidate file reading now validates
and reads the SAME open descriptor, checking path inode/device, regular-file
status, size and post-read metadata;25 verifier scenarios include a real input
file and directory rejection. Script extraction is now case insensitive. Removed
redundant native-fixture exists probes: exclusive wx creation remains the atomic
configuration protection; spawn handles missing executable paths. New scans must
verify these fixes rather than declaring their alerts cleared from source alone.
Alert28 was reviewed/dismissed as a false positive: receipt publication is already
atomic wx and the competing-owner tests pass. Alert29 is an intentional bounded
200-entry JSON request log in an isolated loopback test with a fixed CLI path.
Alert18 is intentional explicit firmware upload after image/board/boot/address/
capacity admission, not arbitrary file exfiltration. Exact scoped dismissal
comments are on GitHub. Historical unrelated security findings are not declared
resolved, checks are not disabled and no CodeQL-success claim precedes a new scan.

Follow-through found two genuine mobile lifecycle gaps among the stale fixtures:
missing critical board identity could be rejected by hostname validation before
retiring captured firmware owners, and the new zoom/pan viewport was not part of
the existing durable workspace snapshot. Identity validation now retires malformed
peers without accepting an unverified hostname; viewport drafts restore before
their listener joins the existing coalesced writer. No device command is replayed.
Existing lifecycle tests now inspect the actual viewport instead of removed
horizontal scroll widgets. Remaining fixture changes target Output controls in
their own panel, explicitly expand Hardware capabilities, use current build
identity, semantic Lost state and Cancel connection during an unfinished scan.
Workflow PR21 was locally merged into the integration branch after complete diff
and conversation review; pinned action patch/minor updates retain permissions and
trust boundaries. Its changed workflow hashes were reviewed and rebound. Tests
for this follow-through are in progress, not yet a green exact-head claim.

User explicitly requested preserving and publishing all mixed source and merging
ready PRs for pull-based handoff. Inventoried index/worktree/branches/stashes
before mutation: one branch, no stashes/worktrees beyond this checkout. Preserved
all source in a623944 on handoff-consolidation / PR24; private outputs, firmware
backups and Wi-Fi profiles remain untouched locally and are now ignored.
The localhost8118 route failed; authorized SOCKS10808 restored actual GitHub
read/fetch/push. Full paginated Pakar conversations were refreshed; dependency
PR21/22/23 had no comments, reviews or threads. Their diffs and release notes
are reviewed separately; failed checks are not bypassed. Copilot reviewer
requests were rejected (not a repository collaborator), not completed reviews.

Canonical Rust Test passed461 tests and Flutter Analyze found no issues. The
quality pass exposed stale fixtures: formatted Rust no longer matched literal
whitespace assertions, actual command-transport dependencies were missing from
Web VM slices, and the full auth fixture lacked current layout/parent/ID and
numeric-validity DOM behavior. Those fixtures now execute the current shared
code; negative peer-fence injections still fail. Field-local notices are not
global toasts; success haptics now follow readback rather than precede it. BLE
and peer-relay loop services remain checked inside the explicit network guard,
allowing unrelated maintenance/expiry before it, with missing/escaped negative
fixtures. Governance expectations match both explicitly included LED drivers
and the canonical normal/quiet build commands. No production behavior changed.

The old legacy ledger had complete hash-bound comparison entries missing from
its CSV projection and invalid reviewed-equivalent/reviewed-improved labels.
Reconciled only exact complete source-hash matches, normalized labels to the
existing vocabulary and retained reviewed-open where earlier acceptance was
open. No unread source became reviewed. Rebound only changed fixture/.gitignore
hashes; route bindings for the already-reviewed canonical Web header refreshed
without changing routes. Whole feature/physical acceptance remains open.

## Preview contract checked before Web integration

Consulted pixel-workbench and the exact donor-audit pwm_control mapping; read
the full matching canonical installed donor, full existing Flutter preview
controller and actual firmware store plus scoped HTTP/USB/command contracts.
The Web controller is staged behind injected transport functions, not a new
socket or invented WebSocket command. Original saved revision, peer/boot/route
and lease determine all readback/cleanup. Clock/transport race tests and syntax
checks pass; CI registration is not full-CI execution. UI integration remains
open and no production device was contacted or modified.

A receipt-refresh command mistakenly tried root REUSE-PREFLIGHT.md. The actual
AGENTS path docs/REUSE-PREFLIGHT.md was then read and pixel-workbench/exact-file
donor-audit rerun; no fallback source or exclusion was guessed from that error.

Publication is pending: the scoped issue2 comment failed during CLI GraphQL
through the authorized localhost8118 proxy. Consulted github-coordination and
read docs/GITHUB-COORDINATION.md; verified the proxy listener, then a read-only
REST comments check failed with the same forcibly closed connection. No blind
comment replay or security bypass. Recheck comments before publishing after
recovery. This does not undo the earlier successfully published checkpoint.

## Consolidated Web checkpoint published

Using the previously authorized local HTTP proxy, verified prior comments then
successfully published the circle, rule-window and pixel-tool persistence
checkpoint: https://github.com/atomicdeploy/PakarController/issues/2#issuecomment-6405340227.
This resolves only the historical GitHub-comment publication blocker below.
It does not publish the mixed source as a commit or deploy it to any board/app.
Local ledger generation and focused tests/builds passed; expanded browser input,
RTL/touch and actual-browser restore acceptance remain open.

## Shared presentation persistence and GitHub route recovery

Reverified unchanged stable-ID workspace donor, consulted preferences and its
exact donor-audit record, then extended only the existing writer. Matching fresh
rule fingerprints retain presentation without reconstructing configuration.
Passive restoration/equal-event and peer-isolation regressions pass.

Read-only inventory confirmed the user's previously authorized HTTP proxy on
localhost8118. Using it in the child CLI process restored issue-comment reads;
no global proxy setting, credential change or TLS-verification bypass. Fresh
comments establish the circle and rule-window checkpoints remain unpublished.
Those historical pending-publication notes below are superseded by this route
recovery; final publication receipt will be recorded after ledger validation.
Expanded browser input acceptance remains independently blocked.

## Rule-window continuation remains local

Consulted complete pixel-workbench mappings/pending actions and exact Pealayer
NumericValueControl entry. Actual current Flutter operationOption/diagram slices
were reread at unchanged hashes before adapting the six-slot prefix window.
The full map's existing renderer and current published-RGB validator remain the
owners. Explicit local disclosure requires no device request or command, and
presentation state is excluded from configuration payloads. Tests/builds pass.
Read-only GitHub REST issue refresh still failed TLS handshake; there was no new
comment submission. The pending-publication and expanded-browser blockers below
remain, not claimed resolved by DOM tests.

## Circle-diagram GitHub publication pending network recovery

The scoped issue#2 checkpoint could not be published: GitHub CLI failed its
GraphQL request with TLS handshake timeout. A read-only CLI check failed the
same way. Consulted github-coordination; the independent GitHub connector's
read-only issue-comments fetch also failed its transport to the service.
No second comment submission was attempted; no GitHub synchronization is claimed.
Publish the circle-diagram checkpoint from FILE-COMPARISON-RECEIPTS.md after
first checking whether an earlier attempt exists. Local ledger generation and
focused tests/builds succeeded; source remains an uncommitted mixed candidate.
This network blocker is separate from the existing browser-input/expanded-map
visual acceptance blocker. No production board contact or policy workaround.

## Circular diagram source and observation ownership

Consulted ui-shell and exact NumericValueControl/PCController pwm donor entries.
Read actual complete donors (canonical installed PC source matches inventory),
complete Flutter correction editor, relevant page methods and firmware raw-scene
publication slices. The existing source projection and peer lifecycle are reused;
unknown/pending colors are neutral, never optimistic previews. Frame updates
reuse DOM circles and do not create another transport/renderer/loop.

One patch was rejected because a CSS selector was embedded in a longer minified
line; inspected the actual source and appended a scoped override instead. A test
initially expected absent style after clearing a color; the actual in-place CSS
reset correctly uses the empty string, and the assertion now verifies that
explicit clearing. Source lookup glob/escaping errors were resolved by rg file
inventory and literal-pattern lookup, not guessed replacements. No guard disabled.

## Diagnostic persistence and packaging-order recovery

Initial exact donor lookup mistakenly used src/workspace_profiles.rs. The
inventory resolved the correct src/ui/workspace_profiles.rs; its complete source
was reread and original-byte hash verified before adapting stable workspace
ownership. Actual Flutter constructor/snapshot slices were read too. The existing
Web storage writer remains the only owner; sanitized user drafts are separate
from fresh board observations and pending command authority.

Single-source check first rejected the old onboard package after only the full
profile had been rebuilt. Consulted build-storage, read the actual equality test
and canonical build pipeline; built the missing onboard profile from the same
source, then reran equality successfully. No guard relaxation, clean, extra cache,
native/APK/firmware build or production deployment. Preview refresh and screenshot
work; existing input blocker remains pending rather than silently waived.

## Advanced diagnostic dictionary binding

Comparison generation correctly rejected stale localization target bindings
after the final frame-publication translation was added. Consulted localization
and inspected generate-font-assets.mjs controller-copy/dictionary validation;
the final generated dictionary includes that exact newly added copy. Rebound
only the five previous-current localization hashes, preserving other sources
and all guards. No blind regeneration waiver or broad stale-record refresh.

## Advertised disclosure action remains nonresponsive

Canonical preview refresh works and shows current Wire map copy. The observed
Expand secondary action was invoked once through the supported browser API;
fresh AX state showed no expansion. No repeated locator/coordinate/viewport
action or alternative driver used. Actual Flutter local projection/preview code
was read and reused for the source feature; expanded visual acceptance now has
one outstanding manual-opening request, distinct from passing DOM tests.

## Retired row ownership and read-only browser recovery

Consulted ui-shell and exact PC pwm donor; original indexed path unavailable,
so read the actual canonical installed checkout and verified its full SHA256
matches the inventory. Complete Pealayer NumericValueControl reread too. The
new regression failed before implementation by removing a replacement-peer row;
captured model/peer/render/row fencing fixes that failure without weakening it.
Read-only browser inventory, AX and screenshot now respond, but no stalled
input/viewport action was retried and no alternate driver/native input used.
Inspected the Windows automation skill only; native automation was not needed.

## Pixel-group confirmation continuation

Consulted ui-shell/exact donor-audit and actual EffectGroupDialog plus Flutter
group editor/correction reload before adapting the existing shared modal.
Located test ownership through rg --files after guessed Web test names did not
exist; actual group coverage is in web_feature_workspaces_test.mjs. Corrected
fixture-only dirty accessor/duplicate-group assumptions, retained the actual
ownership assertions and passed focused checks. No browser-control retries or
alternate browser driver used to hide the still-open rendered acceptance gate.

## Shared dialog bounded verification

Consulted ui-shell and exact donor-audit EffectGroupDialog.tsx, reread its actual
complete source and adapted only localized actions, deliberate focus and modal
lifecycle to the existing shared HTML dialog. New actual-component test first
exposed missing classList in its tiny SVG fixture, not an application failure;
added that DOM method and reran focused tests successfully. Existing browser
input/viewport failure was not blindly retried or bypassed. Rendered keyboard,
touch and RTL acceptance remains open in the same ledger and production issue.

## Canonical route rendering and browser viewport timeout

Pealayer App.tsx was fully reread before adapting its actual allowlisted fragment
and history lifecycle into Studio. Fresh canonical #/diagnostics rendered the
expected current workspace at1280px and a screenshot was inspected/saved. This
does not certify click input: Add rule still left the empty rule list unchanged.
The documented browser viewport capability timed out during a442x800 responsive
check, resetting the tool session; the documented reset attempt also timed out.
Stop retries; viewport restoration remains unverified. No alternate browser
driver, hidden-state mutation or board command was used. Existing screenshot
and browser input mechanisms were consulted; future responsive/click acceptance
remains open in the same ledger/receipt and production issue.

## Web correction stepper localization build guard

The canonical font/localization generator rejected dynamically constructed
Decrease/Increase offset labels as unused dictionary entries. Consulted the
localization route and existing DesignerFieldLabel mapping; inspected the actual
generator source before changing code. Kept its first-pass unused-copy guard and
used explicit translatable labels instead of weakening validation or adding
dummy copy. Full/onboard Web builds then passed. Actual complete
NumericValueControl and Flutter editor/address projection reads and useful
remaining omissions are hash-bound in the existing comparison ledger/receipt.
Known browser-input stall still prevents rendered control acceptance; no blind
browser retry, board contact or unrelated build was used to hide that limitation.

## Web correction confirmation and browser-input failure

The prior correction Reload browser confirmation stalled the preview tab's
focus/input commands. Before replacement, consulted ui-shell and the exact donor
inventory. There is no inventoried web_ui/src/components/ConfirmDialog.tsx;
selected and completely read the actual EffectGroupDialog.tsx instead (live hash
matches snapshot), plus Flutter PixelCorrectionsPage.reload and the existing
WebUI.dialog implementation. Adapted their local draft / explicit cancel/commit
lifecycle using the existing HTML dialog, not a fabricated donor or second
framework. Unknown/missing dialog support preserves edits and fails closed.

The stale tab could not be recovered with supported dialog observation; a fresh
isolated preview rendered, but its workspace clicks left state unchanged. Stop
blind browser retries. New guard's DOM contract tests and shared full/onboard
packaging passed, but actual dialog keyboard/focus/responsive acceptance remains
pending. No board commands, save, deployment or hardware acceptance was inferred.
Exact source/proof hashes and useful omitted icon-picker behavior are recorded in
file-comparison-evidence.json and the comparison receipt, not silently excluded.

## Configurable physical-group pixel counts — board, tablet and native checkpoints delivered

This checkpoint adds optional per-group counts; unconfigured groups
retain the board's six-pixel table or twelve-pixel key-group default. Physical
cable groups own their cumulative start/count, independently of logical cell
names/order. Positive overrides are bounded to 1–255; omitted/zero uses the
default, and the complete layout must fit the compiled 480/216-pixel capacity.
Unused tail pixels are explicitly black. Pixel corrections remain a separate
post-render subsystem, not a second copy of the scene/effects engine.

`/api/pixels/groups` reads/replaces the revisioned layout through the existing
HTTP/USB API owner. Settings persist in a separate checksum-validated board file;
unchanged saves avoid flash writes, invalid requests do not partially apply, and
failed mount/read never triggers formatting. Configuration import/export includes
the layout. Diagnostics and published RGB carry layout revision/counts/starts so
clients cannot reinterpret old pixels using new addresses. Explicit USB partial
snapshots may omit optional icon bytes/RGB, never silently truncate the frame.

The existing Flutter, Rust and Web diagnostic workspaces gain physical-group
editors with bounded numeric drafts, address previews and explicit save/readback.
Custom groups affect sector previews, group stepping and rendering; advertised
but missing/invalid layout metadata does not silently fall back to six/twelve.
The donor-first ledger binds the actual PCController SettingsStore/AddressableLeds,
ControllerDialog/PWM/effect-editor and Pealayer NumericValueControl reuse to exact
target/test hashes. Custom fine/coarse steps and clipboard menus remain an
explicit pending donor omission; this is not a claim of full donor parity.

Firmware model/store/renderer tests pass for table and first-wins builds, including
persistence faults, captured layout consistency and full tail clearing. The
canonical LED pipeline and USB envelope budget tests pass. The canonical S3
candidate is 2,279,280 bytes, SHA256
`e4bbdf31ab92855581de23b3424b70d473ea56c821faefe5ab718f70e1ae11b5`.
Final Web gzip `cdd30c73c926dcee8aefdd94cad062279377e54299862712ba9606bafb777bd9`
is embedded byte-for-byte. The existing 16 MB/OPI/buffered/verified-ISP profile,
disabled BLE, dual output drivers and preserved AVR recovery image are retained.
Flutter analysis is clean and 85 focused tests pass: group models/editor (8),
group USB integration (4), existing workbench (53), generic diagnostic USB (8)
and existing USB transport (12). The tests caught and fixed normalized no-op
saves, delayed older telemetry falsely invalidating a saved layout, unsupported
effect drafts being replaced with solid, and malformed compact metadata.
Eight targeted Web suites pass, including real module/DOM-double editing,
variable-sector rendering, command transport, settings and gzip/localization.
These are source/host tests, not an installed-tablet or browser visual sign-off.

End-to-end review additionally found and corrected fixed six-pixel addressing in
the existing device TUI and native screen sampler. The TUI now emits exact spans
and layout revisions; its rendering never issues a command. The sampler fills
validated physical spans and leaves unused output black. The native bridge's
matching route/diagnostic acceptance passes 28 focused pixel tests and 25 bridge
tests. Two older fixtures were corrected to require diagnostic-state evidence
alongside matching RGB and to include additive backend capability defaults;
production validation was not weakened. Native correction rules now retain their
enabled flags through edits/reordering/save and expose bounded pixel/offset steps.

Direct authorization for delivery: "Finish up the pixel correction stuff and
deploy first then work on the rest." ASUS verified the exact board identity
esp32s3-907069eaaf34, saved the previous running image (SHA f1d71fb9...2be19)
and correction/scene/mapping exports, then performed one local HTTP OTA upload
in 25.307 seconds. No AVR programming or filesystem formatting occurred.
ASUS required reassociation to its existing ledcontroller AP profile after
reboot. The original updater's 64 KiB readback timed out; no firmware upload
was repeated. The new read-only verifier checked all 2,279,280 running bytes
using 140 bounded 16 KiB ranges with zero transport retries in 73.213 seconds.
SHA256/MD5 match, selected/running slot 3407872 is VALID, and the same-board
WebSocket snapshot completed in 98 ms. Receipt: outputs/PakarController/
pixel-groups-running-image.json. Boot: 0a23bebe91f1b4c033bb637e21e9f94e.
The verifier reuses existing peer, download-integrity and updater owners;
23 mocked cases pass. It never writes firmware or an additional binary copy.

Saved correction revision31 and rules survived; RMT remains selected, ready and
reports zero failed transmissions. Group configuration reports default6, groups80,
used480, revision0. The change from one red cell to all-off was confirmed by David
as an intentional concurrent action, not restored over his choice. No optical
claim follows from these digital observations, and no new group override was
applied to production merely for testing.

Tablet SM-X906B/R52T60C6V9B was discovered on the AP and independently identified
before installation. Its previous installed APK was pulled and checksum-verified;
settings/data were retained. Installed 0.2.8/build23 SHA256
2368945317057ceb950e34ebcd1527785f62dccf3b485f4c3a0de4daabe53d17
was verified from the actual installed package. This includes group counts and
smooth tablet pinch/pan (6 focused tests), hiding only tablet zoom buttons.
The attached phone was untouched. The subsequent final tablet package is
62,890,597 bytes, SHA256
`0cfec25c48bc4a47a621bc8b1dffbc795990457b1e353c519a18dcc2bc10cc10`,
also 0.2.8/build23 with BoardProductionHold. It was installed without clearing
data, and its actual installed base.apk has the identical hash. Its correction
editor keeps Save/Reload/Apply/Preview and Add rule outside the rules scroller,
uses compact horizontal tablet rows and phone cards, one selected-cursor action,
and fixed-diameter pixel diagrams with current wire/source numbers and observed
raw RGB. The large diagnostics card and user-facing countdown were removed.
Explicit live preview renews its same-peer lease while foreground-owned;
disconnect, stale ownership, uncertain receipts or leaving the page stop renewal.
The board safety TTL remains, and only explicit Save persists correction rules.

Final analyzer and 15 editor + 2 existing persistence tests pass; 7 preview
lifecycle tests pass. Phone/tablet fixtures were inspected. Wireless ADB dropped
during the first actual screenshot (a 62-byte partial PNG is preserved as failure
evidence); reconnecting only the tablet endpoint restored it. Actual installed
tablet screenshots `tablet-corrections-final.png` and
`tablet-corrections-operation.png` were captured and inspected. They show the
compact live editor, retained rule pixel336 and cursor343, true circular diagram
pixels and correct observed white/black values. No rule Save or output-changing
preview was sent for the visual inspection. The page is ready for David's test;
this does not claim human acceptance or a physical preview-duration soak.

David explicitly authorized discarding desktop drafts and replacing the exact
ASUS PakarController process. Native release SHA256
`96844a47b27a568259bcba810caeab74f55d92174a1e1950611ee14134fb2048`
passed the canonical release/stage/probe path and replaced only PID5592 after
preserving its executable and saved profile. New PID23444/session2 reports ready
and the exact ledcontroller peer over Wi-Fi/WebSocket; USB is not selected.
Saved profile hashes match before/after. Actual internal-framebuffer screenshot
`asus-pixel-workbench-corrections.png` was inspected with Corrections open.
Two startup HTTP timeout incidents remained at one occurrence each; the later
bounded observation found no new incidents. Full pixel RGB was unavailable.
Source/runtime correlation found the precise capability gap: `pixel-events` was
advertised only inside the first-wins role branch although subscription/publishing
was shared. Native correctly refused to subscribe without the capability; its
healthy general WebSocket also suppressed ordinary HTTP polling. David approved
the narrow fix with one brief reboot. Shared Network+Leds availability now governs
advertisement, admission and publication; driverless/network-disabled builds do
not advertise it. Both-role tests and the focused integration guard pass.

The final running firmware is 2,279,360 bytes, SHA256
`0908c56d45bf7103b44e74e0debf9d580a170f9eb855e0406dc47d2529a6874f`.
One upload took21.651 seconds after the prior e4bbdf31 image was backed up and
verified. ASUS required reassociation to the same saved AP profile. A 64 KiB
readback timed out; no write was replayed. Independent 140-range16 KiB readback
verified the full image in78.187 seconds with zero retries and59 ms WebSocket
status. Running/selected slot65536 is VALID; boot
`ccfc1af16026c1e43132da93bcf53041`. Receipt:
`outputs/PakarController/pixel-events-running-image.json`. Correction revision31,
rules, all group counts/starts, saved assignments and RMT backend survived.
Whole group JSON differs only in its expected boot identity after reboot.

The unchanged installed Rust process automatically reconnected, observed the
capability and received fresh RGB frame606 (312 ms old); pixel preview is
available/recent and all active incidents recovered through real observations.
The actual framebuffer `asus-pixel-workbench-rgb.png` shows six RGB sectors and
Live pixel colors; Corrections remains open. This verifies native consumption,
not physical LEDs. To prevent repeated verification failures, both updater and
standalone verifier now share16 KiB postboot ranges; backup stays64 KiB. Focused
fixtures cover >1 MiB, final short range, one pinned read retry and rejection
without upload replay. No second board update was performed for that tool fix.
Native live correction preview and USB correction editor parity remain open.

## Pixel correction workbench and board-rendered diagnostics

Direct user authorization: "complete approval" covers this ledcontroller firmware
update, tablet replacement, brief correction/diagnostic verification and restoration
of the previous scene. It is not a PBX approval receipt, AVR programming authority,
or an update to keycontroller. Tablet builds retain BoardProductionHold so saved
automatic firmware jobs cannot silently start during app replacement.

The new pixel-workbench concern maps complete donor reads of PCController
controller_dialog.dart, pwm_control.dart and effect_editor.dart, plus Pealayer
NumericValueControl.tsx. Actual shared dialog code is imported; bounded numeric
steps, local drafts/explicit commits and responsive timing fields are adapted.
The existing AddressableLeds and SettingsStore comparisons additionally map
board-owned diagnostics, correction leases and durable rule validation. Concrete
omissions and cross-client gaps remain in the same hash-bound ledger.

Flutter now provides per-rule enable checkboxes, both reorder arrows, selected/lit
diagnostic shortcuts, address/offset step controls, operation diagrams, local map
preview and opt-in expiring board preview. Holding a table cell opens physical
pixel inspection and explicit single/group actions. Advanced diagnostics offer
solid/flash/breathe/rainbow and overlay without replacing saved cell states.
Only explicit correction Save writes the board's configuration. Temporary
preview ownership expires and is not automatically replayed across reconnects.

Verification: canonical mobile analysis is clean; 53 focused model/widget cases
and 34 separate real-loopback/USB transport cases pass. Testing found and fixed
a small phone bottom-sheet overflow and a real tablet AlertDialog intrinsic-layout
exception; both now use constrained/shared scrolling. The actual screenshot
fixtures were inspected at phone and tablet sizes and remain labelled simulated,
not installed-device or optical evidence. USB snapshots now accept semantic Won
states and valid first-wins snapshots without weakening malformed-state checks.

Firmware correction, role, LED pipeline, route and USB tests passed. Captured
production status verifies exact complete USB envelopes under the unchanged
16384-byte cap (worst escaped-name case 16371 bytes); advanced USB diagnostics use
an explicit same-peer partial receipt, never truncated JSON. The final canonical
S3 build includes both output backends, 16 MB/OPI, buffered AVR link, acknowledged
ISP wiring and preserved AVR recovery image; BLE remains disabled. No AVR image
was programmed by this checkpoint.

Candidate identities from the preserved shared working tree on base 8ce8f29:

- ESP image: 2265456 bytes, SHA256
  f1d71fb9461c58b0e09bd612f58be7a21ac3135ddff2c66d729fffcfbdb2be19.
- Android APK: 62546133 bytes, SHA256
  236751a5ac8118d71ee31e2652291aa271727411411879c4473860d2b1f038ac.
- Tablet rollback APK was read from the actual installed package and verified:
  368830460dcb587a4c535a62415787fa8f9b53b03df7df9f0ce158129fd451c6.
- Before OTA the current ESP image was downloaded and verified:
  e70b3984d0fc812a373da1f00b8b87190724c62ac708c39c27a20a58797a769c.

Delivery is tracked separately below; build/upload acceptance alone does not
establish the running image. Rust parity for the new workbench controls, Flutter
correction-editor USB/BLE configuration and localization of new strings remain
open. Physical LED correctness and human usability acceptance are not inferred
from host tests, counters or screenshots.

Installed ledcontroller evidence: board esp32s3-907069eaaf34, new boot
57ca379848c71e33ff5b582305ce866d. Local HTTP OTA accepted the image in 20.537 s
after the verified backup. ASUS retained a stale AP association after reboot;
reconnecting the same saved ledcontroller profile restored DHCP/ARP/DNS/HTTP,
without changing its tethered Internet route. The first updater verification
then hit a firmware-range read deadline. No upload was repeated. A separate
read-only verification hashed all 2265456 running bytes in RAM in 19.747 s,
without retry or an extra binary copy: SHA256 and MD5 match the candidate,
running/selected offset 65536 is VALID, and the same-peer WebSocket status took
170 ms. ASUS retains the small running-image-check.json receipt.

The prior image had compile-time RMT but no runtime backend preference. The
new dual-driver image started with its GPIO default; one explicit runtime API
selection restored and persisted rmt-dma. Readback confirms requested/active RMT,
ready=true, no pending switch/error and no driver failure. This is digital
runtime evidence, not an optical acceptance claim or proof of GPIO/Wi-Fi causality.

The commissioning probe initially used a hyphenated UUID that the actual
32-hex command-receipt contract correctly rejected; diagnostics remained at
revision 0/inactive. The probe now uses the real ID format, retains the original
failure and does not send cleanup after a correlated rejection. Its corrected
live run verified disabled-row bypass, enabled-row blackout, lease expiry,
board-rendered breathing overlay, explicit stop and restoration, without flash
writes. Correction revision31 and its existing black-pixel335 rule were preserved.
Post-update comparison also confirms identical mode, effect, winner, brightness,
cells, cell_states, state_presentation and red_lock. Both preview and diagnostic
are inactive after the check; RMT reports zero failed frames.

Tablet delivery is not complete: the exact final APK was transferred to ASUS
and SHA256-verified, but install never started because SM-X906B left the AP.
After reassociation ASUS owns 192.168.4.2 (the tablet's old address), so that
cached ADB target was not reused. AP reports one client, ASUS; ADB mDNS and a
bounded known-AP discovery found no tablet. The user was asked to reconnect
tablet Wi-Fi or USB. Prior package, app settings and rollback remain intact;
no phone package was changed. This blocker does not undo the verified ESP update.

The commissioning tool's regression test executes its actual source in an
isolated VM with only in-memory adapters. All 14 bounded cases pass, including
32-hex correlation IDs, rejection/unknown outcomes, no blind cleanup, competing
preview ownership and semantic scene changes. It is registered in the existing
CI quality phase. This test does not contact a device or replace the live receipt.

The scoped checklist is recorded at
https://github.com/atomicdeploy/PakarController/issues/2#issuecomment-6402359405.
Use that exact comment ID for follow-up; issue #2 remains open.

## Preserve the compact Flutter footer on phone and tablet

Compared the actual previous Pakar NavigationBar before a7d69ce with the complete
canonical PCController mobile/lib/unified/workspace_chrome.dart, unchanged SHA256
f00ed672c1f72f1a66fb65cff1cfe0a89dea5f7b6222f154db00a7c52c2e1a82.
The reduced footer keeps that original Material design when labels and touch
targets fit; overflow retains donor stable IDs, scrolling and selected reveal.
Width, RTL, text scaling and inset changes cannot justify hiding destinations.
Portrait tablets share the adaptive footer; wide tablets keep a scrollable rail.
This does not decide which pages the user wants removed, replace status controls,
change selection/device state or certify installed phone/tablet visuals. Focused
widget verification is separate from physical gesture and human acceptance.
The narrow RTL/large-text fixture exposed an over-wide donor-style horizontal
label; labels now wrap within one viewport-sized target instead of clipping.
The broader shell run also exposed an old selection fixture assuming the former
multiselect-on/indicators-off defaults and en-dash range output. That fixture now
explicitly requests its old test settings and asserts the existing canonical
ASCII range; production selection defaults and formatting were not changed.
An unknown shared-selection lookup was corrected to the existing ui-shell/exact
workspace_chrome donor route; no unknown lookup became an accepted exclusion.
Final canonical verification: four footer, five output/navigation and eleven
shell widget tests pass (20 distinct cases), with clean analysis. Phone,
portrait-tablet and short landscape-tablet layouts are widget evidence only;
no APK rebuild/install, board access or physical UI acceptance is claimed.

## USB demand and Android native OTG

Consulted peer-binding, exact donor-audit and the new usb-otg route. Root reread
the complete actual PCController Project/UartProtocol.h/.cpp; mobile reread the
canonical MainActivity.kt at SHA256 5bf90a090b5975f5dc3b7e09ed47ba09e9dad8bf812515902b24cacc35e9a618.
Adapt synchronous bounded framing, split RX/TX, session retirement and owned
platform-channel cleanup. There is no Android USB driver in the donor inventory;
the Android UsbManager CDC adapter is a new platform binding to the existing
living protocol, not a claimed missing import. The unrelated automation,
preferences and firmware/ISP contracts remain separately pending.

Reviewed the pinned Espressif 3.3.12 USB.h/USBCDC.h completely and main.cpp startup,
USBCDC.cpp line-state/line-coding/write sections plus boards.txt USB mode menus.
CDC-on-boot would expose reboot hooks before setup; the OTG build therefore owns
initialization, disables reboot first and preserves UART0 as the default profile.
No production board update/contact or physical acceptance is authorized by these
source changes. Default quiet finite observation leases, actual client demand,
exact post-ACK RGB verification and Android attach/permission/retirement need
focused fixture and SDK verification followed by approved physical testing.

Central verification passed 50 distinct focused native tests and 22 Flutter
transport/API/preferences tests with clean analysis. The final production-hold
Android APK compiled at 60541629 bytes, SHA256
4b8ebb1de1cf4fc07294a7759d744ad36185eb6124201ffb8d94c125250ab10d.
Host lease/three CDC initialization profiles, snapshot budgets and UART ingress
fixtures are exercised through their canonical wrappers. An initial fixture
relative-include error and a Windows pooled closed-socket test race were fixed,
not bypassed. The aggregate protocol runner stopped at an obsolete exact source
guard; that guard now requires admission, lease and capture retirement, with
negative fixtures for each. Its canonical UART-flow rerun and the aggregate's
remaining Web-access/task runners pass separately; this is not a claim that the
original aggregate invocation exited successfully.

Review also fixed USB publication accidentally entering HTTP peer reporting when
real compatibility metadata exists, compiled version-disable propagation,
semantic panel event sequence handling, Android worker exception settlement and
observed native disconnect RX/parser retirement. Actual peer/control authority
still requires fresh HELLO, identity and post-ACK readback. USB firmware/ISP and
multi-role Android coverage, preempting an active Android snapshot, full USB
peripheral mirror adaptation, and rapid unobserved detach/reconnect detection
remain useful unfinished work. Source/fixture success is not physical OTG,
power-isolation, permission, phone lifecycle or optical acceptance.

The nondefault OTG/hardware-CDC image outputs are separate from UART0's existing
default updater path, while retaining one shared warm compiler cache. An OTG
candidate must not masquerade as a UART0 recovery image. Changing the actual USB
core/FQBN invalidates SDK objects once; ordinary source passes keep that profile.
The final real ESP32-S3 OTG SDK rebuild passed: application2352944 bytes, SHA256
4ff8c2ff90c60e04d1f422b28cd3bca95fe4b96e67619bd0e25d70946ee1e943,
static RAM156236/327680 bytes. It is the OTG output, not a default UART0 image.
The first OTG build's generated outputs were moved into that explicit profile
directory before the warm final rebuild; previous/rollback backups were untouched.
No current UART0 SDK rebuild or physical USB test is claimed from these results.

## Automatic selected-board discovery and route continuity

Consulted mobile-network, peer-binding and exhaustive exact donor-audit routes;
added the missing automatic-connection index route instead of treating a keyword
miss as an accepted omission. Root and native specialist read actual PCController
Tools/Controller/internal/link/discovery.go and lanresolver/resolver.go plus
resolver_test.go through EOF at their ledger hashes. Mobile specialist also read
the actual donor mobile discovery.dart and unified/transport.dart completely.
Firmware specialist reread PU850 LocalLib/MNDP.cpp and MNDP.h through EOF and
ConfirmSettings/loopMNDP sketch sections. This section review is not a new claim
that root completely read the whole sketch or the sparse-missing mndp.go file.

Adapted actual preferred-target ownership, successful TCP cache and ambiguity
rules into the existing Rust/Flutter selected connection owners. One completed,
bounded read-only scan may choose exactly one compatible physical board after
the saved target fails; multiple interface routes count once. No discovered
credentials, serial auto-open, automatic auth dialog, mutation replay or second
connection owner is introduced. Runtime Disconnect, USB/BLE selection, disabled
automatic preference and dirty editor/maintenance ownership remain authoritative.
Persisted last-IP data is only a same-board endpoint/scheme/port-bound recovery
hint. Both clients retain original Host/TLS SNI and certificate verification;
loopback ASUS forwarded ports and remote DNS aliases are not rewritten into a
board.local endpoint. Verified active renames retain the board workspace/drafts.

Firmware imports ConfirmSettings' durable-save versus subsequent application
distinction without its blocking delay or private credential broadcasts. A tiny
phase journal coalesces typed nonsecret configuration notifications; status and
USB's existing identity/status hints refresh current state through existing
owners. MNDP relevant-change announcements coalesce at250ms instead of waiting
for30 seconds. A local applied event is not physical association/LAN acceptance.
The mutable hostname MQTT topic/ACL limitation remains explicit in
CONNECTION-LIFECYCLE.md, rather than widening broker authority silently.

Focused verification exposed a native saved-route fixture rejecting localhost
aliases by configured-label equality despite a correct persisted board binding,
plus Flutter analyzer style issues and immediate asynchronous socket failure
ownership in the new trusted-CA recovery fixture. Corrections retain identity,
release, explicit connection generation and normal TLS admission. Startup UI
checks also exposed a real handoff stall: the new endpoint and
connection intent existed, but no selected-board status read occurred while
old subscription cancellation waited. Closing the old WebSocket owner before
bounded cancellation, clearing its shell pointer first and fencing generations
restores the actual HTTP observation and acknowledged-board UI without a fabricated
WS status. A separate real silent-WebSocket fixture checks retirement/replacement.
Native rename review caught another delay: a retired HTTP scope left an optional
configuration-read slot occupied until24 seconds. Rename now retires that slot
with its request generation; ordinary external events still coalesce behind a
valid pending read. These fixes do not establish Android radio/soak acceptance.
Canonical wrappers and existing source-hash gates remain mandatory; stale hashes observed
while a specialist completed its source freeze were rebound centrally, not
bypassed. Final source/fixture and SDK results are recorded in issue#2; no current
production/client deployment, router/DHCP rename or physical acceptance follows.

## Duplex selected-owner commands and events

Consulted network-cloud, peer-identity, bridge-repeater and exact donor-audit
PCController Tools/Controller/internal/localdevice/events.go before implementation.
Root and native specialist read events.go completely (09edc7514d6a499445e0cb76554daf02698366071b50f7c5f47dafd019991c4d),
plus actual hostbridge/peer_response.go and peer_response_test.go through EOF.
Mobile specialist reread mobile/lib/bridge.dart and hostbridge/lifecycle.go;
firmware specialist reread PU850 LocalLib/AsyncWebServer.cpp/.h completely.
Their actual code supplies bounded typed events, exact receipt correlation,
generation retirement and no replay. The donor fixed ContractID/product payload
is replaced by Pakar's existing living packet, not copied as another API selector.
The authoritative ledger now routes and binds the new event source and affected
native/mobile/Internet relay proof files; other unread donors remain pending.

Concrete defects corrected: Internet WS previously ignored commands and dropped
non-status MQTT events; native network repetition discarded the original request
ID and synchronously blocked its event loop; Android advanced bridge only emitted
heartbeat snapshots. The shared selected-owner boundaries now fan out live events
and correlate outcomes without opening another serial/board owner. HTTP consumers
get bounded cursor history; cloud ACKs never masquerade as status-bearing WS success.

Focused verification also exposed and fixed three implementation/test defects:
an unrelated WS reconnect cleared MQTT subscription failure; a byte-budget fixture
counted45 instead of46 bytes; and mobile analyzer rejected two style/type warnings.
The first Node run caught a duplicate local variable before tests; after a fresh
source read it was renamed, with no relaxation of parsing or receipt admission.
A ledger anchor incorrectly said pub struct instead of actual pub(super) struct;
the exact anchor was corrected without weakening the hash gate.

Fresh prior-head CI failed because tests/access_stubs/ESPAsyncWebServer.h lacked
new JSON body callback/contentType/_tempObject behavior used by AsyncHttpService.
Reread PU donor and installed SDK body/destructor/content-type sections, then
adapted semantic fragmented-body callbacks and malloc ownership, including moves.
Actual split/overlap/gap/NUL/size/disconnect/move fixtures pass; no empty methods
were added to make a compile artificially green. SDK section reads are not claimed
as whole-SDK reads, nor are transport doubles hardware or TLS implementation proof.

See DUPLEX-CONTROL.md for exact supported paths, protocol boundaries, real isolated
broker proof and remaining release gates. No production firmware/relay deployment
or LED changes occurred. The live board's older cloud ACK lacks the new identity
fields, so its strict relay and firmware must be deployed together only with the
required production approval; no exact-version adapter or silent downgrade added.

## Single-role builds and semantic cell documents

Consulted games-streaming, ui-shell and the exhaustive Pealayer donor inventory.
Reread actual Pealayer web_ui/src/preferenceFiles.ts and
web_ui/src/components/ServerFilePicker.tsx completely, with unchanged source
hashes. Web imports reuse the exact case-insensitive suffix rule and separate
selection/loading/cancellation/explicit acceptance ownership. A file read never
changes LEDs. The preview retains the original board/boot owner and every row
validates before one atomic heterogeneous assignment command. Browser-native
File/Blob selection replaces the donor server filesystem browser because the
embedded ESP has no host-directory browse authority or React/Ant runtime.
That replacement is not an exclusion of the remaining generic donor browser.
Native and Flutter retain their existing themed dialogs and native document
providers; runtime XLSX codecs are application features, not a second build or
spreadsheet-authoring service. Source/fixture results do not establish current
production firmware, installed-client or physical LED acceptance. Production
hold remains in effect for these unrelated features.

## Flutter batch error disclosure

The table's verified-command bool wrapper swallowed the command exception and
replaced it with "Batch outcome unverified". Consulted error-management and
reread Pealayer src/messaging.rs through EOF, including publish_with_details
and its retained-original-diagnostic test. The table now requests propagation
of its own command exception and displays that reason; other command callers
retain their existing lifecycle behavior. An already-pending command reports
that this action was not sent. No mutation replay or per-cell fallback was added.
The actual table widget fixture checks the reason and exactly one dispatch;
mobile analyzer and all11 widget tests pass. This source correction is not an
installed-phone fix or proof of the reported command's transport failure.

## WebSocket upgrade panic recovery

The physical S3 crash partition was read through ASUS COM22 and its ELF/CRC
validated. Its embedded ELF hash prefix `4632c31c7` matches the preserved
`a593f6d3...` application ELF. The fault stack is AsyncClient::remoteIP ->
WebAccess::bindSocket -> onWebSocketEvent -> AsyncWebSocket::_newClient.
The installed SDK transfers request->clientRelease() before WS_EVT_CONNECT;
dereferencing request->client() there is invalid. Consulted network-cloud and
the exact PU850 LocalLib/AsyncWebServer.cpp donor, read through EOF; its connect
handler correctly uses client->remoteIP(). Pakar now passes that live socket
address to admission while keeping request metadata/auth checks. The actual
WebAccess fixture models the released request client and retains address/version.
The first flash read at460800 was corrupt; a bounded115200 read completed with
valid core CRC. No inference of optical LED, phone or long-soak acceptance follows
from the crash decode or host regression test. Deployment and live tests require
their own receipts before this incident is considered recovered.

## Release reconciliation: browser fixture and live inventory

Fresh GitHub enumeration found zero open Pakar PRs and only main locally/remotely;
no unique local commits, stashes or extra worktrees were discarded. Complete
conversation enumeration was refreshed, with new bodies still pending review.
Exact-head CI39216691834 failed in browser_console_test: the actual WebSocket
handler now calls setUiText, but its isolated VM omitted the localization owner.
Consulted error-management/web-presentation and the exact RayanLamp test path,
reread browser-console.ts and browser-console.test.ts through EOF, and loaded
the real Pakar rayanlamp/localization modules instead of adding a no-op stub.
English/Persian/language-switch and reconnect-label assertions pass alongside
all existing incident, redaction, owner and no-replay assertions. Rebound only
this reviewed test's four existing hashes; no production behavior was changed
to satisfy the fixture. A mistaken browser-console concern lookup was rejected;
the existing error-management index supplies the actual matching route.
Builds and deployments have separate receipts; this correction is not hardware
or full donor-parity acceptance.

The subsequent quality run reached the equivalent isolated reconnect fixture,
which omitted the same localization owner. It now loads the shipped modules and
asserts the actual connection label. The remaining locale fixture omitted
document.body after production template ownership became explicitly body-scoped;
its DOM double now supplies that root. No production null guard or skipped
assertion masks these test-environment defects. The three focused fixtures pass;
all matching existing hashes were rereviewed/rebound, without promoting unrelated
donor gaps. The full quality run remains a separate recorded result.

## ESP0.2.8 Internet migration and temporary panel access

User authorized ESP update and panel password replacement. Changed only the
private panel access/config hash and restarted its dedicated service; device
authentication and broker credentials remain unchanged. Private local access
file updated. No password was added to source or GitHub.

Reverified prepared S3 image2239504B/SHA23e660ec.../MD53ac8bff1...,16MB/OPI,
buffered/RMT-DMA/verified-ISP/BLE-disabled/optional-Ethernet/status-GPIO48 profile.
Published immutable HTTPS manifest SHA ff49acbf... and verified419B manifest and
full image externally. MQTT fetch21 failed before receiving image bytes with
TLS-32512; not replayed. ASUS8022 key conflicted with its port record, but exactly
matched already-trusted david-asus/david-asus.local/1022 keys. HostKeyAlias used
that existing trust; no key checking disabled or known-host records overwritten.

The old producer lacked newer response identity headers. Reused actual reviewed
boardPeer/updater/download owners, not an alternate wire API: explicit exact-board
maintenance admission preserves normal default release checks and requires the
new boot to pass them. Backup may lack BOTH response identity headers only during
that explicit recovery; same address, whole MD5 and before/after board/boot remain.
Partial/conflicting headers still reject. A manifest-TLS failure with no parsed
image requires exact job/boot/target/manifest and zero bytes, never a fabricated
staged image. Added real multipart chunk/backpressure and optional pacing.

Actual old image backup2193760B/SHA cb1f18f16c62e369e52120ec0986bd30d1b5d38e71204216c47aded721a6de0f
was verified on ASUS before upload. One fast and one explicitly paced local
upload aborted737292B/1188264B respectively; each unchanged boot and terminal
abort was independently inspected. No uncertain write was replayed automatically.
Initial backup stopped read-only on missing headers; first recovery fixture
mistakenly removed the required MD5 too and was corrected to the observed wire.
Focused real HTTP/upload/download/recovery fixtures then passed. Source gates
caught changed target hashes and were rebound only after reviewing the diffs.

Temporarily disabling only MQTT freed about43KB; HTTPS stage22 then downloaded
and checksum-verified the full image. Restored MQTT Enabled/Connected BEFORE the
separate exact-job Apply. New actual boot a193c8d230f0bd24efb97f4c57cb35fb runs0.2.8.
Independent35-range full running-image SHA/MD5, integrity and selected/running
VALID app1/0x340000 matched exactly. Canonical updater's final LAN WebSocket read
was interrupted after its hash stage, so its exit1 is retained, not relabeled a
successful tool receipt. Separate Internet WSS received two same-boot live states;
remote HTTPS cell1=its existing white received matching ACK/readback in294ms.
Saved80-cell scene/mode/effect/brightness/output were compared unchanged. No
physical LED success or long soak follows. Remote phase is complete but its
completion_verified field remains false; keep that reporting defect open.

AVR flash/fuses/EEPROM untouched. Actual AVR0.2.7 is now intentionally unadmitted
by the ESP0.2.8 floor, so AVR-backed capabilities remain unavailable pending a
separately coordinated companion update. This deployment uses the already-built
candidate, not the subsequent PU source imports. Production/physical acceptance,
automatic low-memory TLS coordination and reliable LAN upload/WS remain open.

## PU850 actual verified backup before update

2026-10-19 afterf6e0bb3: consulted pu-client-download/remote-ota and actual Node
performUpload, C# UploaderApp.RunAsync/BackupInstalledFirmwareAsync, Go
uploadFirmware and C++ upload_current_build at unchanged431104072952f694a35b3a83c19f13fbb098ca5b.
Imported conditional verified-backup-before-write sequencing through existing
updater/downloader owners, not another transfer stack. Added --backup NEW_PATH,
explicit --resume-backup, no-contact plan and exact receipts/timings. Enclosing
peer pins FIRST backup read; output MD5/size/board/boot matches admitted metadata.
Fresh auth/job/boot/latch admission precedes local upload or remote fetch/apply.
No optional environment credentials enter this still-auth-off updater. Default
supported-release rules unchanged; explicit old-release fixture permits0writes.

Focused19-scenario actual-source/filesystem suite passes: full/multi-range,
partial/complete explicit resume, local update, remote stage/apply and existing
ready-job resume; bad section/peer/final boot/metadata/auth/job/post-backup peer,
deadline, existing destination/partial and destination race. Failed backups
permit no fetch/apply/upload and preserve available data; later failures retain
verified backup. Actual Node CLI AND guarded PowerShell plan subprocesses pass
option propagation and0device contact, without creating a backup file.
Existing updater/download/peer-policy and PowerShell syntax/runner fixtures pass.

The actual CLI fixture initially failed with unsettled top-level-await exit13:
dynamic backup import reached updater's peer helper while updater entrypoint
was awaiting that same import. Reconsulted peer-identity/actual donor sequencing;
moved the EXACT unchanged boardPeer body into existing peer-identity and
re-exported it from updater. Direct API and CLI regressions now pass; no release
expression, identity check or API generation adapter was added/weakened.
Comparison guard caught one compact historical JSON reference whose old hash
was skipped by whitespace-sensitive rebinding; inspected exact checkWebSocket
anchor and rebound its current hash rather than bypassing the guard.

Existing evidence/ledger/receipts/index updated with pu-backup-update required
Node/C# reviews and fixture/target hashes. Fresh109/109 PU remains complete,
96gap files/0unread; all314/4697comparisons,4383other-donor pending,253gaps.
Stable watch/coalescing/graceful stop/skip policy/progress and remaining generic
runtime/platform/storage/gzip/negative-case gaps remain open. No live backup,
firmware/server deployment, SDK/donor build, reboot/network/settings/LED/audio/
PBX/relay/rainbow operation, extra runtime/cache/worktree or policy bypass.

## PU850 fresh complete-tree coverage and final tooling reread

2026-10-19 after5eb45d1: final28 earlier tooling/CI/dependency/docs certificates
freshly read EOF, every helper/function/test/job/manifest section mapped in the
existing PU850-PARITY/evidence. All109 PU files now have fresh section records
and current revision, not only historical certificates. Fresh GitHub main/tree
still431104072952f694a35b3a83c19f13fbb098ca5b:121 entries/109 blobs, untruncated,
0submodules/0symlinks. All46 present sources match original working hashes and
normalized Git bytes; all63 sparse-absent sources read from exact immutable
blobs. All67 explicit blob records verified against actual SHA1/object bytes,
SHA256 and length. No standalone license or binary asset hidden outside the tree;
embedded assets/configs/test/dependency/tooling files are included.

That byte audit found one historical metadata defect: build.cmd source_blob
incorrectly recorded its13809-byte CRLF checkout identity as the13410-byte LF
Git object. Actual Git blob SHA25687cca41c65336039a92ed8dc0de40b898f7a445a6472fcbd92347ed1b84ea696
and SHA1/length now recorded; original source certificate/working SHA256
a2361ded706871a3f67f25ef572cb73cdf025e98c838185fb1346f589f4b4f72 retained separately
with exact line-ending-only equality. No donor file was rewritten. Initial
source-prefix lookup collided with another target path and safely refused;
anchored it to the actual source SHA before adding review fields. No blind hash
acceptance or read exclusion was used to pass this check.

Actual C# ConsoleUi independent stdout/stderr terminal policy imported into
existing script-console, with independent redirect direction, NO_COLOR, dumb
terminal and existing JSON/quiet/sanitization/exit fixtures. Node and PowerShell
script-tooling source fixtures pass; pu-tooling/pu-client-manifest gates pass.
No extra Chalk/Chokidar/.NET/Go/WinHTTP runtime, donor build, SDK rebuild, CI wait,
cache/worktree, dependency install, workflow dispatch or real device/service
contact. No deployment/network/settings/reboot/LED/audio/PBX, policy bypass,
relay or rainbow change.

Read coverage109/109 is complete; implementation/parity is NOT. Existing PU96
gap files remain; all314/4697 comparisons,4383 pending other-donor files,253 gaps.
Generic watcher/coalescing/graceful stop, verified backup-before-upload, stream
progress, AP/restore/panel/clock/file/tracing, Unicode/platform/distribution,
gzip corpus and exact ledger omissions remain open. Source/CI/package/live/
physical/human/soak gates are separate; #2/#3 stay open and root owns live work.

## PU850 fresh runtime reread and actual shell metadata import

2026-10-19 afterb3647b5: freshly read all23 earlier runtime/auth/shell/discovery/
sketch certificate files completely through EOF, including complete2394-line
sketch and850-line SSDPDevice. Recovered clipped SSDP middle with explicit
overlapping chunk. Same pinned/current431104072952f694a35b3a83c19f13fbb098ca5b;
original-byte hashes and normalized working/blob equality verified, exact
section/function decisions now in existing evidence/PU850-PARITY. Fresh pass
is81/109; remaining28 earlier tooling/CI/dependency/docs certificates still need
fresh read. Historical comparison totals remain109PU/96gaps and314all/4383pending/
253gaps; unchanged certificates are not new proof.

Actual Commands.cpp Firmware_Hash/Build_Date descriptor/getter code now serves
read-only firmware.hash/build.date through current ShellProperties/shellGet/TUI.
SDK sketch MD5 and compiler time are observations, not full-image integrity,
slot, release, installation or deployment proof. Secret-first guard stays ahead
of getters; auth/current release policy unchanged. Classified generic busy/ready/
audio callbacks as replaced by current owner, not excluded as business.

Passed focused source tests: property descriptors in ordinary/ESP8266 PROGMEM/
S3 profiles; actual getter prefix in both platform profiles; actual production
session/initializer/service loop fixtures in both profiles. Includes read-only,
secret-first, metadata source, two clients, late clocks/rollover, deadlines,
backpressure and auth retirement. No SDK build, donor build, CI wait or live
terminal/device operation. Continued using existing stable build/native cache.

Evidence formatting was heterogeneous: whole-block patch safely rejected;
inspected exact layouts and used narrow descriptor/proof contexts. Diff audit
caught an over-broad disposition hunk touching one unrelated AVR-display
exclusion; immediately restored that exact original field, retaining only the
two intended generic shell callback corrections. Source identities untouched;
no generic pending work or physical acceptance was waived.
No network/settings/deployment/reboot/LED/audio/PBX/relay/rainbow operation or
supported-release bypass; all such acceptance remains open with root.

## PU850 C++ integrity and complete UART source checkpoint

2026-10-19 continuation after25eb318: complete C++ upload.cpp and UART.h/.cpp
read through EOF in bounded chunks at unchanged pinned/current431104072952f694a35b3a83c19f13fbb098ca5b.
Exact function groups, dependencies, concrete memory/timeout/baud/credential
defects, actual import and generic pending vs exact business-only decisions
extend PU850-PARITY and existing evidence, not a second checklist. Current
SerialLink/UartFlow/WireCodec/Protocol read completely for replacement decisions.
Consulted uart-flow-control and literal donor-audit entries; added pu-uart and
pu-cpp-transfer specialist routes. An initially guessed donor-audit script name
did not exist; reread the documented consult-reuse entrypoint and used its
donor-audit concern. No missing script or sparse path was treated as exclusion.

Actual C++ single_header/parse_header_md5/collect_download_hashes now serve the
existing backup owner: canonical full Digest-MD5 and duplicate-critical-header
refusal. Shared request response adds in-memory headersDistinct evidence only.
Focused backup, actual request and existing updater fixtures pass; a real local
HTTP fixture proves duplicate ETag preservation, with wrong Digest/duplicate
Digest and boot rejection. No board/WAN contact, write replay or admission change.

Comparison generation and read-only Check bind all109 PU files (96 gap files,
0 pending,0 specialist-unrouted); all donors314/4697,4383 pending,253 gaps.
THIS IS NOT whole-project fresh-review completion:58 previously uncertified
files plus Authentication.cpp/.h are freshly read in this run, while49 earlier
certificate files still need fresh per-section review. Their certificates are
historical coverage, not fresh execution evidence or accepted missing behavior.

The first evidence patch assumed all historical reference properties had one
ordering and safely failed before edits. Inspected the real differing proof
layout and rebound exact existing hash fields; no source hashes or anchors were
invented. A PowerShell brace-expansion typo similarly failed before execution;
explicit paths replaced it. Donor and unrelated files remain untouched.
No SDK/donor rebuild, CI wait, cache/worktree, deployment/reboot/network/settings,
LED/audio/PBX operation, live relay change or version-policy downgrade. Generic
UART file/panel/language/clock/tracing, watcher/platform and all live/physical/
human acceptance remain open.

## PU850 C# manifest and deflate source checkpoint

2026-10-19 continuation after ac60a1a: all fourteen C# uploader files, all three
CMake files and complete puff.h/puff.c read through EOF at donor
431104072952f694a35b3a83c19f13fbb098ca5b. A truncated UploadClient result was
completed with overlapping bounded chunks before certifying it. All nineteen
new comparisons include original bytes, immutable Git objects and exact method/
section decisions in the SAME evidence/ledger/report. PU is 106/109 complete,
3 pending, 93 gap files, 3 specialist-unrouted. All donors: 311/4697 complete,
4386 pending, 250 gap files. A complete comparison still records useful gaps;
it is not production parity.

Actual C# FirmwareManifest parsing/portable-name/paired-hash/bounded-gzip code
and TestManifestAsync cases now serve tools/firmware-manifest.mjs and the real
explicit ESP8266 compression branch in firmware-report. Separate .gz/.gz.md5
outputs preserve raw image names and bytes; S3 stays raw. Existing Node zlib
owns bounded decoding instead of another puff implementation. Mark Adler's
license was read and retained in the unchanged donor; no puff code redistributed.

Executed focused manifest tests: exact two records, portable names, duplicate/
extra/mismatched records, invalid/empty/corrupt/oversized gzip, both hash failures,
generated sidecar, actual artifact-report consumer, unchanged ESP8266/S3 raw bytes
and no S3 compressed output. Existing script-tooling fixtures also pass. No
donor/.NET/C++/SDK build, installation, extra cache, device/service contact or
release/deployment/relay/policy/rainbow change. Existing raw-S3 updater and its
identity/auth/ownership/integrity/release-admission rules remain unchanged.

Entirely unread sources remain Tool/C++/upload.cpp and LocalLib/UART.cpp/.h;
previously certified async/OTA/auth/shell/discovery/main/tooling still needs the
requested fresh function reread. Generic watcher, platform/runtime distribution,
IPv6, editor database, backup-before-upload/progress, stream transport and full
gzip/deflate corpus plus live/physical/human acceptance remain explicitly open.

One read-only verification invocation piped the ledger's Format-Table records
through Select-Object -Last, producing an invalid FormatEndData output sequence
after the ledger itself passed. Reconsulted pu-tooling/Tool/build-all.ps1 and its
actual formatted-host output ownership, then consumed the complete formatter
stream with Out-Null. Ledger and whitespace checks passed; no source guard,
comparison data, compiler or device operation was bypassed to fix presentation.

## PU850 editor and actual Go download source checkpoint

2026-10-19 continuation after e159b5d: all eleven Go source/test/module blobs,
three editor JSON blobs, secret example and WebSocket sample fully read through
EOF at unchanged donor431104072952f694a35b3a83c19f13fbb098ca5b. Exact sections/
functions, sparse Git objects, original bytes and adapted consumers extend the
same ledger. PU coverage is 87/109 complete, 22 pending, 76 gap files and 18 paths
without a specialist route; overall 292/4697 complete, 4405 pending, 233 gaps.

Actual Go download/resume/range/all-hash/part/commit source and unchanged-file
guard are ported into tools/download-firmware.mjs, its guarded PowerShell wrapper
and existing pakar dispatcher. Default is a no-contact plan. Explicit operation
reads same-peer running firmware into a new backup; existing backup is never
replaced. Partial/full length, all advertised hash metadata, full MD5/SHA256,
same board/boot/TCP address, final status, release admission, changed partial,
deadline/cancel and atomic no-overwrite commit have executable local fixtures.
Optional Basic credentials stay environment-only; no bearer bypass, policy
downgrade, write retry, flash or remote updater is added/invoked.

First fixture exposed EBADF from new exclusive write-only creation followed by
same-descriptor hash verification. Consulted pu-client-download/exact download.go
and reread actual donor close/reopen/hash flow; corrected exclusive creation to
wx+ and retained descriptor identity through verification. Added size/time/mode
checks from actual watch.go so a same-length partial edit during final metadata
read is rejected too. Focused fixtures passed after those material corrections.
Donor tests were read and selectively ported, not built/executed as target proof.

Actual editor associations/vocabulary/generated-output filters now exist in
.vscode/settings.json. Three process tasks call existing firmware/desktop/mobile
guarded Plan entrypoints, with no autonomous build or deployment. JSON/plan
ownership validation passes. Full compile-database/S3 IntelliSense, generic
watcher, gzip/paired manifest/backup-before-upload, remaining client/UART and
fresh previous-certificate rereads remain open. No donor/SDK build, new cache,
live board/app/server/relay/network mutation, reboot, LED/audio/PBX action or
physical/human acceptance occurred. Current policy/rainbow/relay are untouched.

## ControllerBoardMini/Puzzles editor-profile closure for four-box provisioning

Consulted `donor-audit -Origin ControllerBoardMini/Puzzles` and read the final
three pending donor files completely through EOF: `.vscode/arduino.json` (8
lines), `.vscode/c_cpp_properties.json` (539 lines) and `.vscode/settings.json`
(15 lines), with their original-byte ledger hashes unchanged. The useful
MiniCore ATmega328P/Urboot/EEPROM/BOD/LTO/16 MHz profile is retained in
`tools/build-firmware.ps1`; portable Arduino/C/C++ associations and generated
tree exclusions are retained and extended in `.vscode/settings.json`; guarded
plan tasks remain in `.vscode/tasks.json`.

Fixed COM12/USBasp selection was replaced with device inventory plus explicit
deployment, editor-owned `build` output with the canonical low-write build
owner, and the 539-line absolute Arduino15/compiler-macro snapshot with the
actual pinned compiler invocation. The donor's `gcc-x64` IntelliSense choice
and copied `USBCON` macro were not imported because they misdescribe an
ATmega328P and become stale with toolchain/package changes. A future generated
compile database remains useful only if it comes from the canonical build
without machine paths or another cache. These are reviewed replacements, not
silent exclusions. No serial port was present on David-ASUS during this review,
so it establishes no backup, flash, UART, optical or physical acceptance.

The sibling `ControllerBoardMini/Timer` editor profile was then read with the
same full-file discipline. Its 539-line `c_cpp_properties.json` and 15-line
`settings.json` are byte-identical to the Puzzles files; `arduino.json` differs
only by its retired `Puzzle_WithTimer.ino` sketch name. The same portable build,
editor and deployment decisions therefore apply, with independent source hashes
and ledger rows. This closes three Timer pending comparisons without claiming
the remaining Timer firmware files or physical board behavior.

For the active first-wins hardware path, Timer `Keys.h/.cpp` and
`ShiftRegisters.h/.cpp` were also read completely and compared byte-for-byte to
the reviewed Puzzles copies. Their independent source hashes now bind the same
mode-aware classifier, typed active-low shift owner, direct-port wire sequence
and role-specific 1 ms service proofs. This avoids duplicate runtime code while
leaving physical key order, bounce, relay bits, signal integrity and end-to-end
UART latency explicitly open for the connected four-box board.

Timer `SevenSegments.h`, `TonePlayer.h/.cpp` and `WS2811.h/.cpp` were then read
completely and confirmed byte-identical to the reviewed Puzzles files. Their
independent rows now point to the existing A4/A5 front-panel driver,
nonblocking PB1 hardware-timer queue and S3-owned grouped LED renderer. The
blocking buzzer wait, global timer state and fixed eleven-pixel AVR BRG scene
remain deliberately replaced. Physical TM1637 ACK, audible timing/flash sync,
channel order and all96-pixel four-box output remain explicit commissioning
gates rather than inferred from source fixtures.

The final Timer files were also read through EOF. `RotaryEncoder.h/.cpp` remains
an explicit optional gap because its D2/D3 interrupt ownership conflicts with
feature-gated RC hardware and no encoder is confirmed. `LooseTestCodes.txt` is
replaced by the bounded, editable, secret-redacting shell TUI rather than copied
as non-compilable private scheduler dumps. `program.cmd` is replaced by the
identity-fenced, backup-first, hash/readback-verified update owner instead of
unauthenticated curl, hard-coded COM5/host paths and avrdude `-V`.
`Puzzle_WithTimer.ino` was read all 483 lines: generic display animation,
melodies, cooperative scheduler, input/relay and serial intent are mapped to
current owners; its 15-minute countdown, rotary adjustment and six-key timer
puzzle are explicitly excluded business logic. This closes the Timer inventory,
not its remaining physical front-panel/audio/relay/UART acceptance.

## PuzzleBoard front-panel and controller integration review

Consulted the PuzzleBoard donor audit and read all 1223 lines of
`PuzzleBoard.ino` through EOF at its recorded original-byte hash. The reusable
12-step four-digit perimeter animation, active-low input/relay service, status
color/audio distinction, serial diagnostics and cooperative-service intent map
to the existing FrontPanel, TM1637, ShiftRegisters and TonePlayer owners. The
target animation is bounded and semantic rather than permanently repeating.

The donor's ATmega32-style PC6/PC7/PC3-PC5, PD6/PB4 and LCD A1-A7 map is not the
confirmed ATmega328P ControllerBoardMini hardware and was not copied. Its 2231
puzzle, countdown modes and relay sequence are excluded business logic.
Blocking beeps/five-second delay and single-byte 9600 commands are replaced by
nonblocking queued audio and framed 38400 UART. Its multiplex loop's
`digitIndex++ == 4` reads `displayData[4]` before resetting; the cached TM1637
driver removes that overrun. LCD/expanded16-input hardware and all physical
display/key/relay/audio/UART acceptance remain explicit open gates.

`XS/7Seg_Remote.ino` was also read all 192 lines through EOF. Its useful
RCSwitch receive/reset lifecycle maps to the current cooperative RadioAdapter,
which forwards the complete raw code, bit count, protocol and pulse length to
ESP for learning and configurable action policy. The donor's fixed low-byte
action table, blocking 200 ms delay and dedicated 74HC595 display pins are
replaced by typed raw forwarding, nonblocking service and the confirmed TM1637
A4/A5 owner. Physical RF receive/transmit and receive-restoration acceptance
remain open until the fitted radio is exercised.

The 1,523-line `XS/40LED.ino` was then read through EOF at its recorded hash.
Its exact 590 ms welcome variation, fractional progress interpolation,
monotonic animation intent, changed-state reporting and automatic/manual output
ownership are retained through the nonblocking Timer1 cue, grouped wall-clock
LED transitions, indicator pulse/configurable neighbour chase, cooperative
register owner and framed hardware UART. Fixed FINAL_TABLE/RADIOGRAM score,
bonus, fan/countdown and analog-meter rules are excluded business/hardware.
Incompatible A0/D8/A1-A5 pins, unchecked ASCII/`atoi` parsing, unconditional
9600-baud forwarding, blocking delays/watchdog reset and the explicitly rejected
confetti/sinelon/BPM/juggle examples were not copied. Optical channel/order,
effect cadence and audible 590 ms verification remain live hardware gates.

`XS/40LED.ino.new` received complete semantic coverage using that fully read
base plus its complete two-hunk no-index diff; no unchanged region or changed
line was skipped. Its disabled 74HC165 scan/direct-button remap is an explicit
hardware-profile choice in Pakar rather than a shared-loop comment. Its
suppressed zero-value notices are replaced by authoritative winner-zero and
reset-revision events, and reset audio is semantic/idempotent: ready-to-ready
retries repair state without replaying a cue. The newly connected four-box
pair still needs physical input, UART, LED and reset verification.

The root PuzzleBoard `.vscode/arduino.json`, `c_cpp_properties.json` and
`settings.json` were read completely (8, 536 and 19 lines). The portable build
retains external 16 MHz, UART bootloader, EEPROM retention, 2.7 V BOD, Arduino
language ownership and generated-tree hygiene. The donor's wrong-MCU MightyCore
ATmega32 profile, fixed COM1/USBasp, absolute Arduino15 paths, stale ATmega32 and
USBCON macro dump, false gcc-x64 mode and duplicate editor Build directory are
replaced by the actual MiniCore ATmega328P arduino-cli owner, inventoried
backup-first deployment and reusable low-write caches. These editor mappings do
not establish a live build or flash.

`Loose Code.txt` was read completely. Its raw-edge timestamp and stable-state
press action map to the shared key classifier, which uses a configurable 20 ms
rollover-safe debounce and emits the primary Down action immediately after that
interval while classifying Up/click/double-click/hold separately. This replaces
the fragment's unspecified initialization and fixed 50 ms press-only behavior;
actual switch bounce and first-wins latency remain physical measurements.

The PuzzleBoard `Tools` update slice was read through EOF: `Client.js` (193
lines), `Server.js` (96), `run.cmd` (18), `package.json` (10), and the complete
836-line npm v2 lockfile. Remote notification/update intent is retained through
immutable HTTPS manifests, the durable ESP update state machine and native
client updater. Fixed COM6/AVR_HOME, shell-composed avrdude `-V`, anonymous
`fileModified` flash triggers, raw image broadcast, mtime identity and the
infinite curl/overwrite/execute loop are replaced by explicit plan/run/apply,
backup-first board/boot/address fencing, exact size/hash/profile/version checks,
post-boot running-image verification and no mutation replay. The unused 2023
Node watcher dependency graph is replaced by the actually linked native and
embedded transport stacks. Live USB/OTA/ISP and power-loss recovery remain
hardware acceptance gates.

## PU850 exhaustive continuation: foundation source checkpoint

2026-10-19: AGENTS/preflight and owning issues #2/#3 read. Target started clean
at feefd9d299a710f0be4945b90d4e23695d8668ac. Fresh GitHub main, ls-remote and
local sparse HEAD all match donor431104072952f694a35b3a83c19f13fbb098ca5b.
Untruncated recursive GitHub tree121entries/109files/no submodules agrees with
the existing inventory. Sparse files were read by immutable git show, never by
expanding/advancing/resetting the shared donor checkout.

The initial preferences lookup with PU origin correctly refused because no PU
route existed. Read actual foundation sources/dependencies and added the missing
pu-foundation route; donor-audit remains exhaustive. Complete EOF reads of20
previously pending files (including all four Temp dependencies), with exact
section/function decisions, are recorded in the SAME comparison evidence and
PU850-PARITY.md. No screened/unread file became an accepted exclusion.

Actual ASA0002E.h convertToHexString/getFormattedUptime/humanReadableBytes source
is adapted into BoardCore/DiagnosticText.h and invoked by live WebAccess session
creation, /random and telemetry uptime/heap text. Fixed buffers, unsigned bytes,
capacity/overflow/truncation checks replace unchecked dynamic/signed formatting.
Real16-byte system entropy, numeric telemetry, auth/identity/ownership and default
supported-release admission remain the current owners. Imported donor ignore
patterns protect ~local.h/~secrets.h; no donor secrets or credential hints copied.

Focused execution passed: all256byte encodings; capacity/SIZE_MAX/null cases;
seconds/day/UINT32_MAX uptime and B/KB/MB/GB boundaries; actual WebAccess
all-method epoch/auth/origin/async dispatch double; access persistence/wrong
password/failed commit/restart/corruption/physical-USB-only recovery double;
all50donor route source/target hash+line anchors; cached-filesystem telemetry
source guard; git check-ignore for both private-header names. Runner is wired
into existing CI quality checks and shares build/native output.

The comparison generator correctly refused the changed CI runner binding before
report generation. Inspected the one-command quality-list extension and unchanged
runChecks owner, then selectively rebound that single Pealayer CI reference;
all existing affected WebAccess/MaintenanceRoutes and50route mappings were
rechecked for surviving anchors and rebound to the inspected source. Rejected
patch-generation/context attempts made no source changes; no gate was bypassed.

PU coverage is71/109complete,38pending,64gap files,31specialist-unrouted. Overall
276/4697comparisons,4421pending,221gap files. Generic calendar/hex decode/path/
URL/checksum/crash/UDP/editor/UART/client behavior remains explicitly pending.
No donor build, firmware SDK rebuild, CI wait, network/app/server mutation,
deployment, reboot, LED/audio/PBX action or physical/human acceptance occurred.
Current rainbow/production settings and live relay remain untouched. Continue
the remaining files and fresh function rereads in bounded source checkpoints.

## Temporary private Internet panel

The scoped office service now serves the actual project Web UI and basic panel
at https://ledboard.digitalogic.ir over valid wildcard HTTPS. Created only the
missing nonproxied ledboard CNAME to office.digitalogic.ir (TTL120), then verified
authoritative/public DNS and external HTTPS401 without credentials. Authenticated
HTTPS200 and WebSocket received fresh status from esp32s3-907069eac800, unchanged
boot ff6da73b47d6fd790fc1eb36c491d6ef/version0.2.7 and80 logical cells. The Internet
path uses the existing board-initiated TLS MQTT session, not ASUS, UPnP or an
exposed device HTTP port. No firmware, AVR, LED scene or network setting changed.

Read actual existing deploy-client-service.mjs/install-office.sh,
mqtt-live-probe.py/provision-mqtt-device.py, CloudCommandGuard, CloudServices
handlers, TelemetryReceiver, peer identity and actual Web peer/basic code. Reuse
their explicit installation ownership, pinned dependencies, bounded SSH children,
request ID/deadline, board/boot fencing and checked telemetry reassembly. Reread
the entire canonical PCController b0111ae0 session.go blob and verified its
1311fdcde SHA; the older repos checkout e171bf35 is not substituted as that
source. Existing ledger entry gains exact relay targets and HTTP/WS proof hashes;
serial priority/open/reset/AVR and broad donor parity gaps remain open.

Mutations are bounded, one-shot QoS0/non-retained publications, require current
supported release/identity, correlated board ACK and new same-boot readback.
Offline/retained status grants no lease; connection loss reports uncertainty,
never replay. Project UI release identity is preserved, not copied from the MCU.
Private HTTPS login is distinct from unchanged device authentication. Broker and
DNS credentials stay server-side; generated panel login is root-only and copied
only to a private local outputs file, never GitHub or browser assets. Unsupported
relay routes/features fail closed. Physical mappings/pixel frames are not in the
current MQTT status, so they are not invented. Full telemetry probe returned
invalid/foreign snapshots; complete /api/telemetry remains unavailable unless a
fresh checked same-board snapshot actually arrives. Status is independently live.

Eight focused real HTTP/WebSocket and protocol tests passed: auth/Origin, retained
and wrong-peer data, release rejection, correlated ACK/readback, timeout/no replay,
broker loss and bounded forms. Added the locked test job on all tooling runners
and expanded the living-contract guard to all services. Initial new-service npm
ci correctly failed without a lock; initialized once with npm install, then used
the lock for actual server ci. Deployment preflight also caught the intentionally
changed workflow hash: reread the complete pinned PU850 workflow blob and actual
target diff, then rebound existing evidence before the guarded redeploy. No gate
bypass, board reboot, optical success or full production acceptance is claimed.

Final external-host probes (not office loopback) received HTTPS200 and two
successive WSS snapshots with the same actual MCU/boot,80 cells and rainbow.
Locked relay dependencies reported zero npm audit vulnerabilities at this check;
this is not a security audit of the entire project. Living-contract negative
fixtures, release-package fixtures, script tooling, comparison generation/check
and peer-binding gate passed. The release packager's tiny dummy-image fixture is
not a firmware candidate. Existing application/SDK caches were not rebuilt.

Control is deliberately still blocked: current Web minimum is0.2.8 while the
live board is0.2.7. Await the user's compatible-panel/update choice; do not silently
downgrade policy. Once admitted, verify a real user command/visible result and
remaining contextual settings/mapping/full telemetry over this temporary relay.

## Flutter capabilities belong to Hardware

Consulted preferences, device-context and exact donor-audit entries. Read the
actual Pealayer hardwareAvailability.ts and installed PCController mobile
peer_context.dart; the latter is not the ledger's unavailable legacy checkout,
so its pending comparison is not promoted. Reuse the existing imported
PeerContext and board-scoped PanelInventory rather than creating another
capability source or inferring hardware from measurements.

Move the remaining Overview badges into Hardware alongside its measurements.
All acknowledged profiles can reach Hardware, including boards without network
or telemetry. Retained badges dim on lost ACK/disconnect and clear for another
board; they remain presentation only, not mutation authority. Overview retains
identity/connection information. Focused hardware-page tests cover navigation,
unknown peers, disconnect/reconnect, replacement ownership and phone/tablet
layout. Installed Android and human acceptance remain separate.

The focused phone test caught overflow in the inherited fixed-aspect measurement
cards. Consulted the actual installed Flutter GridView/delegate implementation;
use a text-scale-aware card extent instead of shrinking or clipping the labels.
The regression covers normal and double-size text on phone/tablet widths.

## Persian Web UI regression

Consulted localization and donor-audit for Pealayer web_ui/src/i18n.ts and
tests/i18n_surface_audit_test.rs; reread both actual current sources. Retain
dictionary fallback and application-copy ownership, not a DOM mutation observer
that guesses which live peer names to translate. The missing controller captions
now cover preferences, network/cloud/access/firmware settings, cell management,
effect labels and runtime status. Mode captions generated by DeviceContext.h
are included in dictionary discovery instead of being incorrectly pruned because
they are not literal HTML. Explicit option values remain unlocalized commands.

Localized text/attribute owners refresh after locale changes; real addresses
release earlier status-copy ownership. Live status repaint uses the same saved
snapshot, without refetching, changing selection, replacing settings drafts or
sending commands. Persian headings no longer inherit Latin letter spacing.
The exact bundled Vazirmatn font and its route are unchanged.

Focused Node regression tests exercise repeated telemetry, locale round trips,
stable option values, peer-name isolation and settings/selection behavior. The
actual embedded HTML was inspected in a read-only loopback browser fixture at
mobile width: Persian preferences and Wi-Fi settings, English/Persian switching,
reload persistence and localized live status were observed. Screenshot:
build/evidence/web-persian-settings.jpg (local, ignored evidence artifact).
This is not a flash/deployment or live board, Android or native app acceptance.
Other dynamic errors, confirmations and advanced diagnostics still need a
complete copy audit; donor parity and issue #2 remain open.

## Opt-in board firmware notifications

Consulted remote-ota and read the complete actual PCController
docs/UPDATE-PROGRESS.md and PU850 LocalLib/AsyncOTA.{cpp,h} and
FirmwareTransfer.{cpp,h}. Reuse the measured-stage, finite notification and
verified-new-boot lifecycle already adapted here. No donor ntfy publisher was
found in the searched PCController mobile, Pealayer src or RayanLamp internal/Web
sources; this is not an exhaustive exclusion of their remaining files.

Board cloud configuration adds optional ntfy_enabled/host/topic/token_set and
write-only ntfy_token under the same unversioned /api/cloud and typed cloud
command settings. Default off, default host ntfy.sh, no default topic. Web,
native and mobile firmware settings use existing pinned ACK/readback/draft
contracts, not independent client notification preferences. Old cloud.bin
credentials migrate without writing on boot; explicit saves retain the pending,
readback and rename transaction and identical saves cause no flash writes.

NtfyNotice consumes loop-owned lifecycle events, coalesces measured progress
and never labels receiving100% or reboot-pending as success. One volatile latest
notice expires after five minutes, not a durable replay spool. HTTPS runs on an
opt-in S3 worker outside the LED loop/OTA callbacks; dispatch defers during
maintenance. Progress during flash may therefore collapse into the verified
post-boot result. Certificate/hostname validation, valid clock, bounded HTTP,
no redirects and numeric-only status adapt RemoteFirmware. No POST retry after
ambiguous failure and no incoming ntfy command/auto-flash authority. Token is
never returned, topic/token are absent from telemetry and public draft export.
An already dispatched POST cannot be retracted when opt-out is saved.

ESP8266 has no nonblocking HTTPS worker in this slice: optional settings are not
advertised and enable is rejected, rather than adding another blocking call to
its LED/UART loop. Public ntfy delivery, real update/reboot notification, phone
receipt and TLS/OTA/LED coexistence remain pending opt-in live acceptance. No
topic is invented, no firmware deployment or external notification is implied.

The same PU Commands.h typed property/get/set owner exposes ntfy.enabled,
ntfy.host, ntfy.topic and ntfy.token on S3 Telnet/framed serial; topic/token are
write-only and excluded from shell history. Read Commands.{h,cpp} completely;
do not import its readable Wi-Fi passwords. Existing TUI network view has a
bounded sixteen-row layout; ntfy remains available through its get/set shell,
not a fictitious added TUI page. A dedicated TUI cloud page remains follow-up.

Build preflight caught newly stale generated translations/compressed Web assets:
consulted localization and its Pealayer Web i18n mapping plus the actual
generate-font-assets/portable-gzip pipeline; regenerate deterministic outputs,
then rebind the reviewed Web/route hashes, without replacing the shared font or
clearing caches. An invalid Select-Object pipeline over the comparison generator's
formatted output was removed, not worked around by skipping its gate. The S3
linker then caught a missing translation-unit wrapper: reuse the actual
SharedCloudServices.cpp include pattern as SharedNtfyService.cpp. Native tests
passed; the first new mobile fixture incorrectly supplied cloud data for its
required status/version read. Fix the fixture with peerFixture(), not a disabled
release check. These failures are not device/network/notification evidence.

Focused verification: nineteen native device-settings/UI tests, clean mobile
analyzer and two new ntfy settings/session tests; actual cloud file migration,
transactional fault/no-op tests on both host profiles and handler tests on three
profiles; lifecycle/receipt/tool/panel fixtures; Web settings/workspace/gzip and
S3 wiring guards; all three ordinary/ESP8266/S3 shell descriptor profiles passed.
No installed app, board, ntfy server or phone was contacted for these tests.

## Flutter board identity versus interface-route normalization

Read the complete actual PCController mobile/lib/discovery.dart and
Tools/Controller/internal/lanresolver/resolver.go at b0111ae0; original hashes
fc37b663... and8dec2b65... match the existing source ledger. Consulted the
mobile-network slice and exhaustive exact discovery-file mapping. The donor
retains exact probe endpoints; Pakar's added identity coalescing incorrectly let
Connect choose the first merged alias, ignoring a saved AP/STA/Ethernet origin.

Keep one board identity, but retain exact IP/scheme/port routes and current-sweep
observations separately from historical aliases. Probe saved/advertised aliases
with the same bounded four-worker public pipeline. A route selector preserves
the explicit saved/selected origin across rescans and Connect uses only that
origin; unprobed/offline aliases do not inherit another interface's Available
badge. Unselected defaults prefer a freshly observed hostname, not a stale one.
No firmware, native, resolver eviction policy, API generation, authentication,
board/boot checks, credentials or no-mutation-replay rules were changed.

Canonical Flutter analyzer and the16 focused device/discovery tests pass:
AP/STA/Ethernet origin preservation, per-route freshness, separate alias probes,
explicit route retention through reversed discovery order, exact status-request
destination and saved selection; existing390/1200 layouts, password challenge,
real loopback XML/status, redirect and source-pinning guards still pass. The
analyzer initially required braces in the new dropdown handler; corrected that
handler and its exact hash binding, without disabling the lint or any test.
Actual phone installation, AP/STA/Ethernet LAN and full-release acceptance remain
open. No board or phone was contacted by these source/loopback/widget checks.
Canonical APK preview build-only also passed:0.2.8+15,58132833B, SHA256
`3c0da08ba57e656bbd12585b6e486b06abb0c591309b2a3256359e0b92422dbb`.
Existing release cache/signing were reused; APK was not installed or activated.

## Partial rainbow remains a physical failure

Fresh read-only trusted ASUS/ledboard.local still reports ESP/AVR0.2.7, same
ff6da73... boot, rainbow/RMT-DMA enabled and advancing counters. The user's
first23-cell failure remains authoritative; no RGB/OTA checkpoint closed it.
Reread actual esp32WS2811 driver/header/README, exact-IDF ring/worker tests and
reviewed RMT TX/RX allocation/receive/disable/DMA/EOF implementation. Read all
eight WS2811V1.4 PDF pages as text plus rendered visuals: timings, cascade/bit
order, power/level thresholds, application circuits and packages, not keywords.
The same ledger now binds the PDF timing and still-pending electrical acceptance.

Source centers the RMT pulse widths within that datasheet's ranges and adds an
explicit whole-frame digital-pad capture, not another renderer/driver fallback.
The ring remains bounded2046 symbols and still refills; neither SDK accounting
nor pad readback proves strip input voltage or physical light. Existing static
cycle timing, matrix scene/mapping/BGR, cloud features and AVR memory are intact.
No new RMT SDK-wide flags, dependencies, alternate cache or clean build.

Fresh480-pixel RAM snapshots include changing139/480 and no black tail, not
physical success. Actual internal largest block31732B rejects the initial
46112B contiguous capture design. Read the pinned SDK partial-RX descriptor
mount/callback behavior: use an8128B ring (two4064B halves), stateful incremental
comparison and bounded callback-span checks. TX retains witnessed priority3;
RX uses the ordinary SDK GDMA allocator, not the TX-only override. Verified
`intr_priority` does not control SDK DMA RX allocation; no invented RX receipt.
No ISR heap/copy or permanent ring reservation; RX explicitly does not use
PSRAM. Observation adds IRQ load and may itself overrun; mismatch alone cannot
blame the transmitter, and digital matching cannot prove strip voltage/light.

Final SDK link caught Xtensa `dangerous relocation: l32r` in the header-defined
IRAM RX callback. Reread the pinned SDK RX callback/cache contract and existing
donor/output ISR placement. Stock RX cache-safe support is off, so retain its
ordinary callback contract rather than forcing a weak header function into
IRAM or changing global compiler/SDK flags. TX's cache-safe refill remains
unchanged; a custom cache-safe RX SDK is not certified by this checkpoint.

First fixture failure was its naive first-match error counter parser reading
the NEW nested capture error instead of the existing final driver error. Checked
the actual worker/capture JSON and corrected the fixture's counter lookup; no
production error was hidden. Both static/DMA RGB/BGR profiles, injected capture
corruption/malformed-chunk/timeout/no-memory/failed quiesce, exact-IDF ring/decoder and existing
renderer/persistence checks pass. Full physical failure and coordinated-release
rollout gate remain open; do not label a timing candidate as the confirmed fix.

Prior main run39194759112 is red independently of this candidate: native's WS
fixture expected a control frame but received the new peer introduction; mobile
has six failing network/settings/bridge/update fixtures (including missing peer
release metadata and null/widget lookups); ESP8266 exceeds its DRAM `.bss` region.
These were read from the actual failed job logs, not inferred from a red badge.
No tests/checks were disabled or broad unrelated client/ESP8266 changes attempted
while prioritizing the physically failing S3 matrix. CI and coordinated release
remain open gates; focused LED passes are not a full production-green claim.

## Semantic onboard RGB integration

Reread the actual PCController Project/StatusLedMath.h and
StatusLedController.h/.cpp, plus the complete Pealayer src/ui/status_bar.rs.
All four live source hashes match the existing comparison ledger. Adapted the
donor's absolute-time Flash/Cycle/Transition mechanisms into fixed S3 profiles,
adding pulse/double-blink without a second matrix or client animation engine.
Generic editable profiles/manual cue ownership remain explicitly pending.

Focused production-engine/configuration tests pass for static/breathing/pulse,
real-black blinking, color fades, held-black shutdown, exact repeated states,
priority, progress, timer wrap, pin safety, persisted settings and driver failure
retry throttling. The existing Web emitted-RGB/fresh-peer/settings tests pass;
native Check passes. These are not optical or deployment acceptance.
The earlier CI peer-fence guard was stale after the reviewed configured-hostname
argument was added; it now checks all three actual identity arguments and rejects
hostname replacing board identity. No fence was weakened. A report-check output
formatting error was caused by piping its Format-Table objects to Select-Object;
the unpiped canonical -Check succeeds with no script change or bypass.

Fresh trusted ASUS access (HostKeyAlias david-asus, existing matching key) and
hostname-first /api/status show ESP/AVR0.2.7, onboard GPIO48 confirmed/enabled,
brightness64 persisted, accepted Ready RGB[0,64,10], and the matrix still rainbow.
No board mutation or optical confirmation is implied. The candidate0.2.8 has
default-on release admission requiring a coordinated AVR/client update; asked
the user before broadening this RGB task into that rollout. Do not silently
disable admission or replace the saved matrix scene.

## Peer identity and supported-release admission

Source checkpoint moves the coordinated package identity to0.2.8/build15 and
support floor0.2.8, so the previously shipped0.2.7 builds are actually distinguishable.
No exact-build adapters or numbered routes were added. PCController's actual
complete discovery_metadata.go supplies the existing bounded nonsecret,
push-fed metadata discipline; RayanLamp's complete api.ts supplies the existing
HTTP identity/fence/error wrapper. Release-range admission itself is a new Pakar
requirement, not a claim that either donor already enforced it. The missing
generic `http` lookup was routed as `peer-identity` in the SAME capability index.

Focused core tests caught a mutable-vs-const ArduinoJson object test which
rejected rich reports; inspected the actual JsonVariantConst conversion and
corrected it to JsonObjectConst. The firmware reports retain original secrets
allowlisting and observed-vs-self-reported addresses, now with strict bounded
UTF-8 model names and same-session USB/Wi-Fi association. These IDs are not auth.
Hash bindings are refreshed only for reviewed changed targets/proofs; existing
donor-wide gaps/verdicts remain open. Direct peers and each bridge's downstream
registry are tracked; forwarding an indirect downstream client's entire report
through a USB bridge into the MCU registry remains a separate bounded-queue task.
No device contact, firmware flash, app replacement, multicast or optical proof
is implied by these source changes. See PEER-IDENTITY.md for policy/build controls.

Verification found two additional concrete regressions and retained the original
acceptance bounds: AVR's full string parser exceeded flash, so numeric UART
bounds are constant-folded without importing the ESP parser; final AVR32284B,
globals1478B,100B flash headroom. The native Windows USB capture fixture measured
age from port-open instead of actual capture start after introduction; its same
eight-second snapshot age remains, plus bounded introduction time. No command
deadline or retry policy was weakened. A missing-version positive firmware-source
fixture was corrected to the actual shared release, not a permissive fallback.

The native build wrapper also stopped converting an absent peer-policy environment
variable into an empty value when no override was requested: Cargo had correctly
treated null versus empty as different compile identities and relinked needlessly.
Explicit override restoration removes a formerly absent variable. Canonical warm
caches and all existing storage guards stay intact. Built-CLI mocks now introduce
the current supported release before operations; auth-info remains read-only
bootstrap, and their actual new-executable execution remains a separate CI gate.

Final override verification found and fixed one real capability mismatch: older
firmware can explicitly reject the additive USB-report opcode even when the
client's release check is disabled. Only an exact opcode/sequence-correlated
Unsupported response can disable that report for the physical session in an
explicit disabled-policy build. Default admission still rejects it; timeouts and
other errors do not grant admission. Non-table USB now shares the existing exact
board/boot Pin syntax before online status, without inventing HTTP capabilities.
Final default44 and separately compiled disabled4 focused checks pass; core
HTTP/USB ownership and no-replay behavior stay intact. No physical connection.

Remote client-management changes follow complete donor store.go/download.go/
artifacts_http.go rereads. Nine actual loopback HTTP/WebSocket/SQLite tests pass
for consistent headers, admission, auth, bounded reports and read-only recovery.
The installed layout includes the canonical helper and release/policy files with
their original relative paths; operator/systemd paths match it. No remote install
or reverse-proxy header verification occurred. Full donor parity stays open.

## Source-only esp32WS2811 review and startup hardening

User-requested bertmelis/esp32WS2811 pinned0222ef025f2b8e03f8c8aef35d63c545a98ad579:
all14 C++ files read through EOF;24 tracked paths inventoried in the existing
ledger. Actual upstream issue2/5 and open PR6 patch/conversations were read;
PR6 has no comments/reviews/threads and fixes GRB->RGB. Keep our logical RGB and
explicit board R/B swap, not an unreviewed channel-order change.

Existing worker/stage/show/immutable frame/coalescing/clear/readback/colour/latch
ownership is mapped in ESP32-WS2811-REUSE.md. The actual missing idle-LOW setup
is now adapted before either backend allocates/initializes: reset/preload LOW/
enable output; failure at each step stops before heap/RMT/TX. Canonical worker
fixture passes22 cases across GPIO/DMA and RGB/BGR, including asymmetric channels
through pixel480, GPIO failures, full black/immutable TX and existing faults.
Source guard, live donor gate, inventory fixture and S3 SDK build pass. Candidate
2193824B/SHA2a4e0d3e66c0523e5b930746125a4966092143c762f021feb8fcaffe97f14705
is BUILT ONLY. No device/app deployment, live scene change or board contact.

All four extra effect algorithms remain explicit useful parity gaps, not falsely
imported: bounded nonblocking Aurora/Autumn/Circus/SnowSparkle plus shared effect
capability/API/USB/persistence/all-client exposure. Concrete unsafe/obsolete
bodies and hardware/package omissions are recorded by exact donor file. Fifteen
new hash-bound comparisons (including retained MIT license) give255/4697 total,
4442 still pending,205 known parity gaps. Required source gates run before S3
builds and actual-worker tests, and the normal LED lookup now routes this donor.
Previously deployed20a242b/ff6da73... remains the last verified running firmware;
these source changes do not establish optical acceptance of the user's23-cell fault.

## Current actually recovered firmware

Corrected20a242b is installed by verified application-only USB recovery:
2193760B/SHA cb1f18f16c62e369e52120ec0986bd30d1b5d38e71204216c47aded721a6de0f,
boot ff6da73b47d6fd790fc1eb36c491d6ef. Full34-range running SHA/VALID app0 and
WebSocket49ms pass. Loop/submit/publishTelemetry compiled stack frames fell
2976->592/1488->48/1408->80B. Actual fixed IRQ level3/core1 is observed. Following
explicit host UTC synchronization (autonomous NTP remains open), all26 samples
across16.418s retain this boot and connected MQTT,3 publish cycles,775 transmitted
increments/failed0/no zero-frame intervals, max sampled gap516ms. This is bounded
runtime evidence, NOT proof all80 physical cells/waveform/long soak are fixed.
Actual LAN mDNS/DNS-SD/SSDP/MNDP/NBNS/NBSTAT and Windows nbtstat passed; router IGD
and actual W5500/LLDP hardware remain open. Current complete evidence/limitations
are in ESP32-S3-COMMISSIONING.md. First failed candidate and panic receipts stay
preserved rather than overwritten; older output-disabled checkpoints are historical.

## Rainbow pause / DMA refill and optional LAN checkpoint

The first candidate2194352B/SHA ec2d49733d5f8160127fdc4a971c96992f98fb327031bbde80451944befb8a70
was actually installed by LAN OTA and full running-image readback, but is NOT
accepted: restoring MQTT exposed a repeatable loopTask stack-canary panic.
Actual exclusive read-only COM22 capture and exact-ELF address decoding locate
buildStatusJson under buildTelemetryJson/publishTelemetry/loop. This is an actual
new failure, not a brownout or a successful delivery disguised by counters.
Cloud reentry was inspected; the measured caller frames instead exceeded safe
headroom. The corrected source separates noinline message/stream/fragment scratch
lifetimes and uses a serialized producer-owned LED mailbox, retaining queue1 and
immutable worker/DMA storage. It does not increase the SDK loop stack or change
SDK flags. Actual-source worker concurrency and both-platform cloud wire-format
fixtures pass. The corrected warm SDK build passes; live recovery remains open.
ASUS ROM inventory confirms the same MAC, CRC-bound selected VALID seq23/app0;
the application is held in download mode pending the corrected application-only
USB recovery. Bootloader, partitions, saved settings and rollback app1 stay intact.

The user now reports active rainbow with only about23 cells reliable and a
two-second pause every five seconds. Fresh ASUS reads confirm the same running
647bbf boot, active rainbow and cloud maximum service2552ms. Submission/completion
counters cannot prove the waveform or the remaining cells are correct. Preserve
the current scene; the older output-disabled instructions below are historical.

Actual pinned IDF5.5.5 RMT/GDMA allocation, bytes/copy encoders and EOF handling
were read, not inferred from the completion counter. The scoped GDMA adapter now
requests/measures level3 on core1 for this channel only; changed prerequisites,
source/flags or measured ownership fail closed. The native two-half DMA ring
still requires timely refills. The actual-source RGB/BGR ring regression model
passes but is explicitly not physical timing proof. Donor PCController
AddressableLeds files were reread and the concrete replacement/omissions are in
the existing hash-bound ledger.

Actual LittleFS info calls were walking storage during full telemetry. That
diagnostic is now a boot-cached, age/known/null observation; storage admission
retains its separate fresh check. The same renderer also advances between cloud
writes, without a second effects engine or unsafe reentry. Focused filesystem,
LED pipeline/worker and IRQ guard fixtures pass. Synchronous individual cloud
calls remain a measured limitation, not a completed nonblocking-I/O claim.

Optional native ETH/lwIP W5500, bounded per-interface NBNS/NBSTAT and gated LLDP
have source/host fixtures. Fresh Ethernet is disabled by default. Native/Web/
mobile addressing editors have ACK/readback fixtures and built packages, not
installed-client or W5500 acceptance. Actual PU NetDiscovery and SSDPDevice
sources were read; the ledger remains240 comparisons/4433 pending/197 gaps.

Read-only ASUS LAN/IGD inventory found ledboard SSDP but no router IGD descriptor.
No router login, port mapping, firewall change or Ethernet enable was made.
Router credentials and physical W5500 confirmation remain external gates. The
integrated S3 SDK build/approved LAN OTA are not yet deployment evidence at this
source checkpoint; record the eventual image/boot/readback separately.

## Latest deployment and bounded CI recovery

Final follow-up cc8e711 is also actually deployed:2074112B/SHA
2909fdb06411d3ea935a29d16889a13cd1b1fa6065061da66741cf965cd49e36;
boot647bbf1e7608295ec5ce6de2e97ba20e. Local transfer19515ms/full verification60022ms,
VALID slot/full running SHA/auth-off/WS checked. Both real Telnet sessions now
answer independently. Seven HTTP reads387..732ms/WebSocket53ms/NTP/MQTT/AVR/TM
passed. ASUS owner20976 recovered with80 mappings/zero active incidents. Actual
egui1920x991 image inspected; capture runner adapted the actual Pealayer Process
handle/wait/output mechanism rather than mislabel its shell-launch discrepancy
as a production IPC outage. Private settings/drafts/layout/rollbacks preserved.
No optical LED or unavailable-phone acceptance. Full current identities and
receipts are in ESP32-S3-COMMISSIONING.md; the earlier checkpoint below remains
historical evidence, not the final running boot. Source/test gates passed without
a broad local suite or waiting for remote CI.

The corrected RMT image from f6d3fce was actually deployed over the guarded
Async LAN path:2073872bytes, SHA256
0532332175ea496216ab8a125e21a6ea442f98acc2e316cbc1379ddc9d190fc8.
New boot37992b7973b2501b3bccde7292b7c96d, full ranged running-image SHA,
selected/running VALID slot, auth-off and same-peer WebSocket passed. Transfer
23158ms/total48719ms. LED output remained disabled and80 saved cell colors were
preserved; AVR flash/EEPROM/fuses were not touched. The private immutable receipt
hashes to fe51bc5887bde31c5bdaba100ebb95f747b4fa2f5323d00081ca2d25fc43dff4.
This is not physical all-dark or RF/timing acceptance.

The preceding Internet job20 failed before image bytes with TLS-32512, the
installed SDK's SSL allocation failure. Its diagnostic remains intact; old Ready
job19 was explicitly cancelled rather than applied. Read complete actual donor
PU850 LocalLib/AsyncOTA.cpp and the canonical updater. Added explicit exact-job
failure acknowledgement, preserving the receipt and re-observing identity/error
before ONE write; distinct later candidates may pin the original failed image.
Loopback/preflight negative fixtures pass. No uncertain upload was replayed.
Temporarily pausing ONLY MQTT freed internal heap from roughly70KB to114236B;
the original setting was restored and TLS MQTT reconnected with credentials,
network and other cloud fields unchanged. Independent server telemetry15607B
and correlated accepted reply passed; checksum588bb3c9f7f3bec5e0e67f384ad8b841d5ab37d48e0e33f4304a218505907ea3.

Actual ASUS LAN packets establish mDNS/DNS-SD PTR/A/SRV/TXT, SSDP multicast and
unicast, and MNDP90:70:69:ea:c8:00/ledboard. Private wire receipt SHA
f933f74a5423fb3330ba5cdec1d0fc974084e290b7152b8b421e4c6fd34d6776.
ASUS native owner20976/hash23ac25... recovered itself: exact new boot, Wi-Fi/WS,
confirmed80 mappings, no active incidents/toasts, closed drafts/no pending writes.
Status receipt5db51aac97fdf144ab3cbb378771001149ba24501d51f47d2a981e19eab7f5ec.
The shell listener advertised correctly but new real sessions closed before
answer. Actual source captures its lease clock BEFORE accept/greeting, then
subtracts later activity timestamps unsigned. Fresh per-session/serial clocks
preserve genuine idle/login/backpressure/fault/authentication retirement.
Actual-function clock/socket fixtures cover this edge separately from real
Telnet acceptance; no terminal success is inferred from configuration alone.

New Android preview0.2.7/build14 was built once in the warm canonical cache,
SHAf1013c0e08716393fbd5974046c63245f8b76d2b3b98729137944f3153013257,
57919841B, and staged/hash-verified on both PCs. Both ADB inventories were empty;
no phone installation or production-signing acceptance is claimed.

Windows CI's WSA10035 came from accepted fixture sockets inheriting nonblocking
mode. Explicit blocking+bounded read/write timeouts and a delayed-input regression
fix only tests; canonical terminal_ui::tests:: passed14/14. No production native
rebuild was required for that fixture change. Legacy8266's complete223213B WebUI
is compressed to66056 portable-gzip bytes, served through the existing ACK-owned
snapshot actor; NUL/short-add/failed-send fixtures pass. The44 immutable shell
descriptors use8266 PROGMEM with by-value reads; both metadata profiles pass.
One canonical8266 polling link now fits:907104B flash, binary SHA
240d044641ccfa6aa93e69365a98d8d07090ac68b206e797470ebcfe1650f727.
RAM81376/80192B remains101% of the core's recommended static budget: this is NOT
a healthy legacy runtime. Larger remaining owners are live queues/state, not
accepted feature omissions; no buffers/features/SDK/cache were stripped.

Short checks found NTP not yet observed on this new boot and a historical
HTTP timeout that later recovered. Memory/cloud coexistence, physical LEDs,
actual phone, W5500/TFT/BLE, external-power restart and full donor parity remain
open. No broad suite/CI wait or long soak substitutes for those gates.

## Native Windows identity and console correction

Confirmed the missed import from actual Pealayer5a6961ce: its complete six-line
src/main.rs declares the Windows GUI subsystem, whereas Pakar's entrypoint did
not. The previous installed local executable really has PE subsystem3 and
CompanyName=Digitalogic; this was not merely a screenshot or naming assumption.
Read the complete152-line6485-byte src/diagnostics.rs and actual process-control
child-spawn policy. Import GUI subsystem and fallible stdout/stderr writers;
keep updater CREATE_NO_WINDOW, explicit existing-terminal-only CLI/TUI attachment,
inherited pipe/file handles and buffered TUI output. No console is allocated.

One PakarController constant now supplies native resources, initial GUI name,
AppUserModelID and Jump List; install the shell identity before the first HWND.
The new windows-startup route reaches previously unrouted main.rs, diagnostics,
startup and process_control. Required reviewed entrypoint/output slices gate
native build/stage, and staging validates the actual PE/resources. Bounded panic
and shell journals remain explicitly pending in the SAME file ledger.

Guarded final release and five focused native tests pass. Real PE/resources,
redirected help/version/helper stderr, invalid-PE/console/wrong-brand rejection,
loopback TUI rejection and remote CLI/no-replay fixtures pass. No broad local
suite, firmware/APK build, SDK clean or board command was run for this fix.
Actual isolated GUI PID205048/session1 had title PakarController · No board
connected; an independent Windows AttachConsole probe returned INVALID_HANDLE6,
confirming no attached console. Protected settings and fixture profile remained
byte-identical. Internal capture did not complete; actual imported Win32 fallback
found no visible non-minimized window. NO screenshot/visible-window acceptance
is claimed. The owned fixture was stopped; no unrelated console was killed.

Delivered SHA256:23ac25fcf7f36fa4258e4e1ebbdd253f9c4721f9a145df27b16269ccc80f1297.
The inactive local installed executable was atomically replaced with verified
settings backup and old-executable rollback; no local production app was killed
or unexpectedly launched. Its settings stayed byte-identical.

ASUS localhost:8022 initially reset SSH, then returned with the trusted identity.
Actual same-session draft/pending-operation checks passed, old PID15928 exited
through IPC, and the verified existing production task launched PID20976/session1.
All four native resource fields equal PakarController and the independent
same-session console probe returned detached:6. Settings stayed byte-identical;
old executable and settings rollback remain. A real egui1920x991 framebuffer
was retrieved and visually inspected; SHA256
7c5bfdf9ad8e405c5060fbb6222191e56055e0398622f0a4e2d96983d2efb10d.
The private replacement/runtime receipts hash to d5b8f5027aa4de998188aa0f18f58e4a76522675e26d83d639575107de2bddb5
and25bb0cb7a931acfc0dcd1d801e3e2fe467639ce23eda88eecae652e902ecdc11.
Wi-Fi was offline before and immediately after replacement; this is native
delivery/capture proof, not device or network acceptance. The earlier RMT
candidate remains unapplied; no recovery apply or color-test command was sent.

## Current RMT correction, shell and upstream publication checkpoint

User-reported RMT/DMA optical instability supersedes the old job19 recovery
reboot. No apply/cancel or new visual pattern was sent. The current same-boot
LAN/Internet observations showed80 white cells and480ffffff published pixels,
RMT ready/active/failed0/error0; these cannot certify the waveform. The user's
instruction is to fix the glitches first. Historical minimum internal heap84B
is an additional unresolved production risk, not a healthy memory-soak result.

Reconsulted led-cell-test/led-benchmark and read actual canonical PCController
Project/AddressableLeds.cpp/.h through EOF at43417181.../a28009e6...; an older
local donor checkout differed and was not rebound as the authoritative source.
Read actual IDF5.5.5 DMA/channel/encoder source, installed qio_opi configuration
and linked IRAM graph. Worldsemi classic WS2811 high-speed250/600ns, not the
previous ad-hoc350/700ns, now defines the requested wire profile. The ring's
two4092-byte descriptors are already maximum-sized; full11520-bit output still
requires refill. The task-scoped GDMA allocator wrapper requests cache-safe
refill only for this LED channel, with SDK validation and fail-closed admission.
No SDK source, global DMA policy or shared RMT completion IRQ was changed.
Existing complete-frame flash fences remain; short SDK symbol accounting stops
instead of publishing success. No scope/optical guarantee is claimed.

Focused proof:10 actual-worker host cases pass across RGB/BGR and static/DMA,
including immutable frames, startup/Off black, coalescing, flash holds, timeout,
failed stop and short completion. Actual GNU-linked DMA guard tests pass supported
and missing-prerequisite profiles. The canonical LED pipeline passes elapsed-time
rainbow, same-clock rendering without FPS ceiling, rollover, unanimated static
selection, scene persistence, message queues and atomic80-cell stress. Requested
timing/status accounting is not physical GPIO/LED readback. The earlier shell-only
SDK compile was deliberately stopped when LED correction became the priority;
its cancellation is not a compile result. A corrected integrated build is pending.

That corrected canonical real S3 build subsequently passed at sourcef6d3fce:
2073872bytes/SHA2560532332175ea496216ab8a125e21a6ea442f98acc2e316cbc1379ddc9d190fc8,
MD5070edbac497b46c65265be7d5cccbfb3, sketch2073727/3342336,
static globals104016/327680. Actual new object disassembly proves the cache-safe
allocation path is present and the18-byte completion callback has no flash call;
the final map places it in IRAM at0x4037a174. Static memory totals do not dismiss
the observed live minimum84B. Board deployment/physical checks remain pending
the corrected-image reboot question. The old Ready job was not applied/cancelled.

Read PU Shell/Telnet/Commands and Rayan form/terminal actual sources completely;
eight exact rows now bind the bounded multi-session shell, history8, VT100 TUI,
typed settings, secret-safe output/history, loop-owned teardown, actual bound
DNS-SD port and boot/token/sequence-fenced COBS USB shell. Five focused fixtures
and PowerShell parsing/Plan pass; no raw USB escape or automatic command replay.
ASCII, broader donor aliases/AP editing, full client settings UI parity and real
multi-client/USB/RF coexistence remain explicit gaps. The authoritative ledger
contains236 comparisons/4437 pending/194 gaps, not donor-wide completion.

Current CI failure was read: Android Requested-row geometry changed40px after
ACK loss, and6 focused contract mappings were stale. The retained row now uses
seenSegmentsMirror while live content/mutations stay ACK-gated. Canonical mobile
analysis and4 panel tests pass;23 exact contract mappings pass after rereviewing
actual target/donor changes. No APK/native replacement or phone acceptance is
claimed by those source checks.

Scoped upstream submissions are published at ESP32Async/ESPAsyncWebServer#497
(4e0f94ae35e7f8541367534800cce89798173aca) and espressif/arduino-esp32#12999
(2f2359a4cd1cc7a212650b3eec6d31da2aafcf83). Actual-method host fixtures pass
baseline-fail/patched-pass response credit/ACK and TCP/IP-context DNS cache/retry
cases. Installed dependency files/pins remain unchanged. The Espressif CLA must
be signed personally by the user; no agent signature or maintainer acceptance.

Internet telemetry, correlated status command and verified ESP-only download
are independently proved from office; job19 remains Ready, unapplied. External
power/unplug survival and post-update independent reachability are not proved.
Do not yet call ASUS/USB disconnection ready or expose auth-disabled HTTP/Telnet
publicly. Immutable recovery artifacts and exact receipts are in commissioning.

## LED all-off physical regression and scoped recovery

The user rejected physical all-off after an explicit80-cell Off batch. The
published480-pixel RGB buffer was entirely black and gpio-static counters
advanced without reported error; those observations did not prove a valid
GPIO4 waveform or latch. Stop color tests, preserve80Off and disable output.

Reconsulted firmware-hardware/led-cell-test and actual canonical PCController
Project/AddressableLeds.cpp/.h through EOF at43417181.../a28009e6...; the existing
owning ledger rows now bind the recovery, not a second disconnected inventory.
Adapt donor stage/show/startup-clear ownership to a complete immutable worker
mailbox, replacing AVR D6/PORTD16MHz ASM/MacroQueue/100-pixel addressing because
the confirmed S3 owns GPIO4 and480 pixels. Read actual installed IDF5.5.5 RMT
bytes-encoder/channel implementation and IRAM configuration, not only examples.
Use1440 internal wire bytes and bounded SDK DMA ring; retain full-frame ownership,
flash fence,300us low latch, startup black and fail-closed stop behavior. The
new350/700ns recovery timing is requested, not measured, and static remains a
separate selectable backend. Eight worker doubles (RGB/BGR/static/DMA/failure),
panel and LED pipeline fixtures, and canonical real S3 compile passed.

Fresh post-drain render time and hardware-UART admission fix a genuine renewed
SoftwareSerial-lease starvation path. Segment cloud snapshots now coalesce
outside AVR receive; other synchronous cloud I/O remains, including the drain's
pre-publish timestamp limitation. Do not claim HTTP/OTA stalls fully solved.

One LAN OTA aborted42460/2046320bytes; independent reads established the unchanged
old boot/image before explicitly selected USB recovery. Fresh CRC-valid OTA19/20
and same bridge/MAC selected app1; only that app range was written and verified.
Full live image SHA d46b74b74d3471215fff3ac85cc4777287a7f9718f42cfd504804e195628f88f,
VALID selected/running slot, auth-disabled WS/HTTP and same-owner native reconnection
were verified. Exact image/boot/receipts and response timings are in commissioning.
Three startup/clear transactions, error0 and80Off still require the user's
physical all-black feedback; do not check optical production acceptance yet.

## Semantic TM1637 and live client events

Read actual PCController Project/Runtime/FrontPanelRuntime.inc.h through all1379
lines at b0111ae0 / SHA4b613488..., plus the existing SevenSegments adapter and
control/display.go finite-message contract. Adapt its650ms label dwell,
label/value alternation, menu/programming priority, finite notices and one-pass
terminal-blank marquee; retain A4/A5, explicit captures and deliberate repeats.
The donor row is now hash-bound and routed by tm1637-display rather than left
as an inventory-only promise. Hardware-specific actuator/sensor pages and full
generic transaction/menu-hierarchy parity remain explicit omissions, issue6.

The ESP owns allocation-free event admission, priority, resting context,
two-second notices, once-only long messages, slow/double-fast blink and finite
AVR leases. It sends separate requested events through WS and USB0xB8. Actual
AVR segment/brightness/ACK events remain the observed mirror; native now consumes
those immediate events rather than waiting only for a full status snapshot.
Clients retain independent, bounded semantic history and reject foreign peers,
duplicates and stale owners without manufacturing capabilities or ACKs. Their
compact requested row reserves its height, avoiding expiry-induced layout jumps.

Focused semantic/AVR presentation/inactivity and both TM bus variants passed.
Native panel3/raw-segment2 and Flutter panel3/real WS2 fixtures passed; Flutter
analyzer clean. The first real S3 compile exposed Effect/Network name collisions
with existing protocol/SDK owners; explicit PakarPanel qualification fixes them,
and the same cached profile now builds. No dependency changes or clean rebuild.
S3 image2045120bytes SHA f513b4fe4adc18874b9fce6735a179120adfac68e49dbe3bb766013f0d061202;
AVR application32168/32384bytes, SRAM1477/2048 (571bytes stack/local budget),
full-image SHA f5bfbe7c9019229dcf8c3b71304bda085869e729b443cf5884ce24a47b60601b.
The small AVR headroom is not a soak/stack certification; keep new policy on ESP.

Actual Urboot backup job1 completed without writes, then program job2 completed
full flash/preserved EEPROM verification and application recovery. Bootloader,
EEPROM and fuse writes were false; no ISP fallback. Final actual raw masks
[57,63,63,6] show C001, TM ACKtrue, AVRonline0.2.7/Progclear. Internal backup hashes
are verified by the current programmer but not exported by its API. ESP UART
hardware error57 was unchanged across final checks; AVR protocol errors0.
Physical readability, blink appearance and all production acceptance remain
separate from source, API/readback, raw masks and compile proof.

## ESP32-S3 Bluetooth LE

Consulted network-cloud, mobile-network, peer-binding, donor-audit and the new
bluetooth route. Fully read canonical PCController Session and Flutter
PeerSession through EOF at the existing ledger hashes1311fdcde.../5263044d....
The older ../repos Session copy has different bytes and is not substituted for
the canonical review. Adapt actual single-inflight admission, final ownership,
correlated receipts and synchronous retirement into main-loop GATT RPC and the
existing Flutter API/UI. Reuse the shared command/configuration/updater and
mirroring handlers; no duplicate LED engine or board HTTP loopback is introduced.
Bundled Arduino-ESP323.3.12 NimBLE headers and scoped implementation blocks were
read against primary Espressif/Android APIs. Bluetooth Classic/SPP/audio is not
S3 hardware. Direct BLE binary firmware/raw streaming, protected/bonded BLE and
automatic board-feed BLE policy remain explicit issue #20 gaps, not parity claims.

Review fixed extra-owner notification leakage, stale queued authority, INFO
host/loop String races, conflicting final-fragment reuse and lost tone-stop
events. Ordered event loss revokes/resynchronizes; status alone is coalesced.
Android retains healthy GATT on pause, blocks new background mutations, drains
admitted protocol ACKs and preserves per-board workspace/selection. App updater
and opt-in remote management remain independent of the selected board transport.

The first focused mobile gate caught two redundant imports; analyzer and all9
BLE cases now pass. The first wider run passed275/skipped2 but found a390px
dropdown overflow and an old mirroring fixture skipping its own sequence after
a foreign event. Compact expanded dropdown and correct own sequence fix those
failures without weakening overflow or gap assertions; focused device-directory
and mirroring files each pass7 cases. This is source/loopback proof, not phone RF.

Current main CI39160022601 exposed actual ESP8266 DRAM overflow and unsupported
byte atomic exchange in the prior completion mailbox. Reread actual PU850
AsyncOTA.cpp, UpdateProgress, LoopMessages and fixtures. Adapt the existing
ESP8266 InterruptLock completion publication and bound its cooperative ingress
to4 complete messages (S3 remains8); preserve explicit overflow rejection/FIFO.
Both host publication branches and queue fixtures pass; real ESP8266 busy-wire
link now passes with77552/80192 DRAM and1024792/1048576 flash bytes. The margins
are small; this is not a deployment or memory-soak claim for ESP8266.

Gates correctly refused moving API/firmware hashes during concurrent source
work; builds waited for frozen source and rereviewed bindings, not bypasses.
The new Windows read-only BLE probe hit two real PowerShell WinRT projection
limits. Primary Microsoft watcher APIs and installed WinMetadata/.NET event
subscription replaced unsupported RCW Add/Register-ObjectEvent calls. Actual
ASUS scan starts and returns zero candidates before the new firmware. It does
not pair, connect, change the adapter or establish board/phone BLE acceptance.

BLE delivery then exposed two separate host failures. An interrupted status GET
left fully verified remote job18 ready; reread the actual updater and canonical
PCController UPDATE-PROGRESS/artifact downloader contracts. Add an explicit
remote-only resume: pin existing job/boot/manifest/target/image, recheck immediately
before one apply, never fetch again. Only transport-failed GET/HEAD with the same
pinned address/headers may retry once; HTTP/auth/identity failures and every write
remain non-replaying. The canonical update fixture wrapper passes both target
publication branches, lifecycle/receipt/panel fixtures and resume/no-write cases.

The exact staged image was applied once. USB confirms a new boot, BLE started,
Wi-Fi uplink ready and AVR online; ASUS receives the BLE service at -42/-44dBm.
LAN verification timed out while ASUS Wi-Fi was software-off, with no blind
reflash. Windows GATT service discovery also timed out. The first INFO diagnostic
incorrectly closed a still-running WinRT operation and crashed on exit; primary
Microsoft IAsyncInfo.Close/owner lifetime rules required terminal-only close and
a retained operation/resource lease. The corrected bounded INFO probe exits
cleanly but still has no successful INFO read. These are actual partial board/
radio observations, not phone RPC or full installed-image/optical acceptance.
See BLUETOOTH.md and issues #20/#7 for exact hashes/boot IDs and open gates.

## Client updates, enrollment and temporary office service

Reviewed the actual existing Android package/coordinator/native channel and
completely reread PCController app_upgrade.dart, UpgradeChannel.kt, artifacts
download/store, ipcjson artifacts HTTP, Windows self-update/process ownership,
desktop ownership and Credential Manager backend. Pealayer update.rs was fully
read by the native implementation subagent; its named downloader, PE validator,
create-new journal/helper and health mechanisms are adapted into actual native
code, not only visual inspiration. Corrected its alternate checkout's changed
selfupdate_windows.go hash by a complete canonical installed-donor reread: the
retained wait function is identical, presentation changes are not source proof.
Six new manual comparisons use the same ledger with exact source/target/test
hashes;229/4673 compared,4444 pending,187 known gaps, not donor-wide completion.

Retained HTTPS origin/redirect/length/digest bounds, exact self package/signers,
durable write-ahead single claims, current-user secure credentials, managed
EXE/profile/PID/helper paths, graceful Quit and startup health. Force-kill and
automatic rollback deletion are intentionally rejected because unknown drafts
and unhealthy new processes must remain recoverable. Generic archive/media/Go
artifact-kind parity, approved uninstall inventory, background Android uptime,
two-installed-new-version tests and physical acceptance remain pending issue7.

Actual service is deployed on office's Node24 loopback18743 under a dedicated
unprivileged systemd user, scoped existing Apache HTTPS include and private
SQLite/content store. PBX/MQTT services remain active without configuration
changes. Operator/client tokens are random, separate and hashed at rest server-
side; only root has the operator token file, Windows client uses Credential
Manager, Android secure storage. API is unversioned, no arbitrary execution.
Independent full actual public EXE/APK downloads hash-match, live external WSS
offers are session-bound, private inventory without auth returns401.

Actual ASUS guarded interactive replacement succeeded, PID22880→20244/build9,
settings and old executable preserved. Initialized exact managed installation,
enrolled using one-use private token, received periodic real telemetry identifying
DAVID-ASUS and completed remotely claimed check-update/cleanup. The latter found
no obsolete pending package: no material deletion. Native automatic helper
two-new-version exchange is implemented but is not established by this initial
deployment wrapper. Native local launch/config was denied by host policy before
execution; no workaround. Exact EXE staged locally and APK staged on ASUS, no ADB
device; no Android installation or local GUI runtime acceptance claimed.

Focused native7 / service9 / Flutter13 cases and analyzer pass, canonical warm
native release and actual Kotlin/APK build0.2.7+9 succeed. No SDK upgrade, clean,
duplicate cache, firmware build/flash, board programming/audio or PBX call.
Existing main CI failed only stale WebUi.h PU route target hashes after additive
upgrade progress code; reread the actual audit/test and change, refresh hashes
and line anchors through the existing generator,50 route fixture now passes.

Failures were not silently retried: fixed Rust digest formatting using the
existing firmware_source::digest byte-to-hex mechanism without dependency change;
fixed six fixture brace lints. Office health probe inherited a local proxy and
received rejection; service logs + explicit no-proxy loopback showed healthy200,
then fixed the scoped probe. Added an actual restart fixture after finding
mkdir needed existing-directory handling. Read-only audit caught malformed
upgrade-URL crash exposure and telemetry-off polling freshness; both fixed.
Validated exact /usr/bin/node matching systemd. Public HTTPS/WS verification
passed after these changes. A mistakenly assumed SDK path failed lookup; actual
installed SDK inventory located apksigner/aapt and verified signer/package/build.
No fallback SDK installed. Parallel comparison generation collided with my
quality reader; serialized the owning generator/check after reader exit instead
of disabling stale-report protection or rebuilding clients.

See CLIENT-MANAGEMENT.md for exact artifacts, capabilities, limitations, service
paths and actual receipts. Current-app remote uninstall remains disabled pending
the user's removal-scope choice; Android local uninstall still requires OS consent.

## Graceful upgrade lifecycle and sound restoration

Completely reread PCController TonePlayer, BootMelody, AddressableLeds source and
headers, UPDATE-PROGRESS and hostui/update_progress.go/tests; PU850 AsyncOTA source
and header at the inventoried hashes. Reuse the actual finite Timer1 cues, ESP
stage/show ownership and measured/unknown progress rules. No duplicated AVR
sender, donor watchdog disabling, PU business UART or permissive updater copied.
Existing comparison rows bind new lifecycle/receipt helpers and client regressions;
all4450 unread/pending files and181 known parity gaps stay pending.

Read-only review caught and fixed LED/cloud activity under raw Urboot, a misplaced
cosmetic delay after bootloader entry, inherited100% verification counters,
per-cell flattening during fade and premature audio notice consumption. Raw UART
now quiesces the existing worker before reset/acquisition; framed parsing and TLS
publication cannot steal the session. New upload IDs start above the durable
receipt ID, preventing postboot terminal/new-upload identity collisions.

Canonical update, renderer/scene/worker, audio, Urboot/ISP, S3 updater and complete
protocol checks pass. Native8 focused progress tests and Flutter analyzer plus
5 focused model/actual-loopback transport tests pass. SDK production build passes
without clean/profile/SDK churn. AVR firmware is unchanged; its existing450ms
descending update cue is reused. Native/mobile source is tested, not claimed as
newly installed host/phone binaries for this checkpoint.

Failures retained: PowerShell5 stripped quoting in the first node-e settings
attempt before any mutation; base64 evaluation of shipping Web settings code
recovered it. GET/SET/GET verified unmute and EEPROM persistence without test sound.
Old source guards assumed the previous callback layout; updated them to assert
fresh capability admission, verified-only notices and raw-ownership exclusion.
A Flutter null-aware lint in a new fixture was corrected, not disabled. Local SSH
HTTP forwards returned empty replies; ASUS direct hostname access worked. Its
old Node18 dynamic-import/eval path crashed before upload admission; the actual
unchanged CLI module worked after local canonical preflight and staged hash checks.
No runtime/SDK reinstall, write fallback or uncertain mutation replay followed.

Actual S3 install then controlled same-image lifecycle OTA used the reviewed
automatic local Async HTTP path on ASUS. First pass verified full running SHA and
VALID slot; second produced59 ordered WS events and exact postboot size/MD5/
integrity receipt, with completion notice consumed once and sound enqueued.
The second readback deadline is a read failure, not permission to flash again.
Read-only recovery subsequently exported30 exact boot-fenced ranges/all1958288
bytes with zero retries and matching SHA256; no additional mutation was issued.
AVR remains online, silent=false persisted, Prog clear, LED failure count0.
Optical/audio sign-off, actual AVR programming and interrupted-power proof remain
open; an API receipt is not physical acceptance.

## Android self-distribution and pinned board automation

Read actual PCController app_upgrade.dart, peer_upgrades.dart, UpgradeChannel.kt,
AndroidManifest.xml, DiscoveryChannel.kt, firmware_page.dart and downloader test
at the recorded b0111ae0 inventory identity; the existing complete discovery,
firmware/downloader/hostbridge/progress rows were consulted too. Seven new
complete-read comparisons bind actual Dart/Kotlin implementations and fixtures;
donors remain read-only. App/package transport is separate from board discovery
and never acquires board command authority. Android's installer confirmation,
not an RPC or advertised package, owns installation. The new mobile-app-updates
index route is a mandatory build gate, not a whole-project parity certificate.

AppUpdates/Peers adapt bounded ordered package transfer and expiring receiver
approval; native staging rechecks exact hash, installed package/signer/newer
version/minSDK/CPU, then uses PackageInstaller with required user action. SAF
import/export, digest-pinned HTTP, verified discovery (DNS-SD/mDNS, SSDP, MNDP),
read-only offer-feed reconnect and durable consumed receipts compose under one
app-lifetime coordinator. HTTP catalogs and WSS/real Socket.IO accept only the
active session, exact source origin and finite offers. A timed-out dial now
closes its eventual socket, using the existing actual FirmwareFeed mechanism.
Android permission-required retries retain the same verified package identity.
No incoming remote installer method or generic command relay was exposed.

FirmwareAutomation persists exact board/application assignment and independent
opt-in stage/apply policy. Explicit automatic Apply uses the actual full pinned
manifest transaction for ESP/AVR/both; raw AVR remains stage-only, raw ESP is a
confirmed immediate upload. The latter initially had a model-level explicit
Apply gate but no usable page action; review added its correctly labelled,
confirmed Upload ESP image action and a cancel/accept widget regression. Manual
and automatic maintenance exclude each other. Hash-bound ISP/Urboot maintenance
replaces the unpinned shared-image Program UI; queued is not verified complete.
The page does not own the feed, so navigation no longer destroys configured
automation. Foreground retirement still pauses delivery and never replays an
uncertain write. No Android background-service promise or direct USB-host
programmer was added; these, split APKs, key rotation and full donor parity remain
concrete pending omissions in the authoritative ledger and issue #7/#2/#3.

Final canonical analyzer is clean;257 Flutter tests pass,2 opt-in captures skip.
Focused actual HTTP/WS tests exercise bounded chunks, receiver capability,
manifest/origin/identity/deadline checks, write-ahead storage failure, no replay,
owner/background retirement and late-socket cleanup. Phone/tablet real widgets
assert no writes, offline availability, opt-in defaults and installer-permission
actions. The first real HTTP fixtures encountered Flutter's default HTTP400
stub; actual existing socket-fixture binding patterns were inspected and the
fixture binding now retains the messenger without intercepting loopback HTTP.
New widget scrolling initially matched inner TextField scrollables, then the
raw-upload button was partially outside the600px viewport: actual existing
firmware_page_test ensureVisible and the outer ListView now drive the fixture.
Assertions and production guards were not weakened. A JSON evidence patch
initially omitted the enclosing array comma; no ledger mutation occurred and
the exact row patch was corrected before regeneration. Initial lint/syntax
errors were repaired through the pinned formatter/fix tooling, not ignored.

Actual renderer PNGs (simulated, no device writes) were inspected:
mobile-app-updates-phone.png SHA256
80e2301433a0922e381650c03ee47b9dfa01d6570d4a4af82bf6916f8294e192;
mobile-firmware-automation-tablet.png SHA256
02f2750cc6a52910c7806a0b1d326ca10b8c3a59aeaa55e65d20ab8a3af306b2.
They use actual Workspace widgets, theme, Vazirmatn and Phosphor/MaterialIcons.

Canonical preview assembleRelease compiled both actual new Kotlin channels and
installer receiver. APK0.2.7+8 is57,346,153 bytes, SHA256
a60d7f968276d81ca3bd792ba9ad8911d6463ab635491f27d41846e7ff06b6fe.
apksigner verification passes; the existing preview's debug signer is retained:
cb2620ad0c885fffd5e1ee7df0bbef8ff3262729e4e880888e9001fbbfb83e26.
This is not production signing. All four software source versions stay0.2.7;
Android's monotonic build code increments7→8 so a newer package can be accepted.
One warm mobile cache1.116GiB/4392 files; no new SDK/cache/clean or unrelated
native/firmware build. Final small UI changes required one incremental APK
rebuild, not a dependency/profile change. Local staging hash matches and the
existing rollback is preserved. Fresh trusted ASUS hostname is David-ASUS;
ADB lists no attached devices. Phone installation, two-device multicast/replica/
OS consent, real configured remote WSS/Socket.IO/TLS, both-board flash/EEPROM/
Urboot/power-loss and human acceptance remain open. No board reset, flash,
EEPROM/fuse write, sound or PBX call occurred in this mobile slice.

ASUS transfer completed through the trusted8022 tunnel; checksum and length were
verified before promoting the pending APK to
C:\Users\David\Documents\PakarController\staging\PakarController.apk.
Actual host DAVID-ASUS readback matches the same57,346,153 bytes and
a60d7f968276d81ca3bd792ba9ad8911d6463ab635491f27d41846e7ff06b6fe SHA256.
Its existing Android rollback is preserved. This is staged, installed=false;
ADB was empty. Existing desktop owner/settings and both boards were untouched.

PowerShell tooling/preflight, versions and generated comparison checks pass.
223 hash-bound comparisons/4673 inventory,4450 pending and181 known parity-gap
files is not donor-wide completion. Fresh GitHub/installed staging receipts
follow the checkpoint; no all-green CI or installed Android claim is inferred.

## Reconnect-safe native dock visibility

Reread complete live Pealayer workspace_profiles.rs at the recorded matching
hash. Read actual restore_tab_to_canonical_slot/reveal_and_focus_tab,
create_initial_layout/sanitize_dock_rects and app capture/apply/toggle/save
function slices too; layout.rs/app.rs remain pending complete-file comparisons.
Adapted canonical sibling/anchor restoration and ancestor unfolding, not
focused-leaf appending. No donor source or unrelated firmware/mobile changed.

The native full dock now remains separate from its capability-filtered visible
projection. Explicit hidden panels are durable board/application intent;
offline/partial-ACK removal never becomes that intent. Old layouts without
explicit visibility metadata recover missing default slots once. Explicitly
closing every panel persists; Restore workspace deliberately resets to organized
defaults. Unknown hardware still cannot paint a panel or authorize a command.
The redundant per-board runtime layout cache was replaced by the existing
durable owner store plus this single layout model.

Canonical native verification:277 tests pass, including painted partial-ACK
transitions, exact split/active-tab retention over disconnect/reboot, hidden
streaming/close-all restart and canonical reveal/unfold. The added egui fixture
initially used an obsolete run API and unsupported SHA formatter; actual current
connection_controls.rs tests and firmware_source::digest supplied the correct
existing APIs. No dependency/cache/profile changes were required. Initial
--verbose parameter syntax failed before building; the documented wrapper was
then invoked normally. Build and source fixtures are not installed acceptance.
A guessed tests/preferences_test.mjs command found no file; rg confirmed there
is no such separate suite. Actual native Preferences tests passed in the277-test
run; native-render-lifecycle and connection-controls source checks also pass.

tools/verify-desktop-layout.ps1 adapts existing same-session owner/hash/draft
guards and checks the retained layout hash, hidden intent, panels and selection
through an explicit application disconnect/reconnect. It never commands/reset
boards or switches implicitly to USB. Runtime/deployment evidence follows only
after verified replacement; pending donor-wide parity gates remain open.

The first installed-layout harness stopped after disconnect because its general
pending-operation guard also rejected the expected asynchronous transport release.
The actual owner remained healthy with the same retained-layout SHA and no drafts.
Reread existing verify-desktop-workspace/observe-desktop release guards; the new
helper now waits boundedly for release, and explicit failed-receipt resume checks
the same PID/hash/layout/selection and disconnected intent before reconnecting
once. The failed receipt is retained; no board mutation or forced close followed.

Installed ASUS evidence: executable28,605,952 bytes, SHA256
72270d79867da8fd50baecac8b8c3b659f2dc6c75a63dba5464d0d96edddb6fc,
version0.2.7. Graceful replacement receipt layout-replacement.json preserves the
prior executable/settings; new PID22880/session1. Actual owner reconnect receipt
layout-reconnected.json passes: identical retained layout SHA256
2256740a29af0b6585967f0fa1f50fc705814f3ab671a35ff0515a50eaf4039c
before, disconnected (Events-only projection), and reconnected. Same board
esp32s3-907069eac800, Wi-Fi online, seven original panels restored, empty hidden
intent and selection unchanged. This is real app/board transport evidence, not
an AVR/ESP flash, optical LED test or all-production acceptance claim.
Actual expected-PID egui framebuffer capture layout-after.png was copied and
visually inspected (1920x991), SHA256
12a12f4a38d45ad8a2176e25fa206dba6cd7050cf1b635cd7a0b503a140508a9.
Table stays left, Effect controls/Hardware monitor right, Front panel and
Events/Streaming/Network tabs below; live board/AVR/WebSocket remain reported.
Image/UI evidence is not physical LED/audio or David's design acceptance.

## Durable scenes and passive workspaces

Completely reread matching PCController SettingsStore.cpp/header and adapted
markDirty/service/changed-byte avoidance into SceneStore, not another AVR EEPROM
layout. Pealayer workspace_profiles.rs/config-persistence test and complete Web
App.tsx plus PCController mobile preferences/settings and Web quick-header
preferences underpin the actual native/mobile/Web workspaces. The authoritative
ledger binds exact source/target/test hashes; app.rs/config.rs function slices
are not certified whole-file reads.216 comparisons/4673 inventory,4457 pending,
174 known gap files is not donor-wide parity.

The96-byte semantic scene stores80 logical white/red/off cells, mode/effect,
brightness, output/static mode and cursor/winner. Stage/close/exact readback/
atomic LittleFS rename retains the previous committed scene on failure.
1500ms debounce,10s maximum coalescing and failed-write retry backoff avoid
frame/unchanged write churn. Mapping and scene restore precede first LED output.
AVR HELLO and the SetMode-induced reset revision are baselines, not new Reset/key
actions. Snake phase, streamed/diagnostic pixel bytes and pending commands are
not restored. Status reports pending/saved/error; explicit scene/save can flush.

Native persists bounded board/application dock/active tabs, actual egui scroll/
collapse memory, unified selection/find/cursor and safe front-panel drafts.
Flutter extends its existing ordered writer with diagnostic/front-panel/public
settings drafts, navigation, connection intent and phone presentation choices.
Web retains passive board/application workspace/public drafts and conflict
baselines. No cache grants fresh capabilities or replays a command. Native
privileged settings drafts remain pending safe public-baseline persistence;
secret edits and live audio sessions deliberately remain unpersisted.

Verification:274 native tests;216 Flutter tests plus2 opt-in captures skipped;
analyzer clean; actual renderer/SceneStore filesystem-fault/corruption/no-op/
frame-write/rollover tests; Web storage/conflict/no-write tests; local quality
runner; guarded S3 SDK build all pass. New mobile lint issues and a real late
rename-ACK/disposed-cache regression were repaired without weakening owner
checks or the unchanged failing widget assertion. Final preview APK SHA256
6b47c68f95e140e4e28d2db2a6f7b3dc148425694d28637027abe068a2887175.
No ADB device is attached on ASUS, so this is built/staged, not phone acceptance.

Actual ASUS graceful replacement preserved settings/prior executable:
native SHA2564050dfd22061fb086131d4b2b13085554c8b7cfe674b8be8f42bbd749ca32cf9.
Process11476 later returned live selection1,21,80 after disconnect/reconnect
and process restart. The lifecycle harness itself retained its failed startup/
25s hostname-reconnect receipts; the later independent owner readback is the
restart proof, not a falsely successful harness. Find-cell has plain-text IPC
admission, unlike lifecycle commands; verify its actual readback. UTF-8 console
handling and read-only startup retries are retained in the reusable verifier.

Actual local Async HTTP OTA uploaded1939600bytes in17.716s; full running-image
SHA2568c3f22cd4f21c0e377f1715ef1d6ba56a8c5b0ba8f0ec5a0bd867dc45b4346f5,
VALID selected/running slot, auth Off and WebSocket before/after were verified.
The first reset boot was5e29a2e44168c1f6f3494cfbac09f64e. The original scene
was explicitly reapplied once through the actual ASUS app after this first
migration, only while the new scene was still untouched white. Normal reconnect
never runs this migration helper. Intermittent mDNS/HTTP read failures used a
previously verified TCP hint retaining hostname/Host and fresh board/boot checks;
only read-only attempts were bounded/retried, never an upload or cell mutation.
The updater's injected-request seam labels simulated:true; the outer receipt
explicitly records actual TCP and instrumentation, and separate physical image/
new-boot readback establishes the live result. Do not relabel that seam as stock.

An actual esptool5.3.1 read-mac/RTS reset on the known CH343 COM22 bridge
identified ESP32-S3 rev0.2, MAC90:70:69:ea:c8:00. It wrote no flash/EEPROM/fuses
and did not program AVR. New boot9a09fc22574f30deb81a2a46b9bd73b8 read back
all80 original cells plus mode/effect/brightness/output exactly:
restored=true,saved=true,pending=false,error=null,writes=0; AVR online, LED driver
failed=0. Receipts on ASUS: deploy/persistence-ota.json,
persistence-scene-restored.json,persistence-board-reboot.json and
persistence-workspace-current.json. This is controlled reset/readback proof,
not abrupt-power-loss, emitted-colour, Wi-Fi soak or human acceptance.

Existing main CI39148355283 failed because Ubuntu's fortified glibc strlcpy
conflicted with the Arduino host fallback. Both cloud doubles now share a
native-libc-aware compatibility header; Windows cloud-service/management
fixtures pass. Remote Linux confirmation remains the next CI result, not an
assumed green build. Warm production caches/profile/versions0.2.7 were preserved;
no clean, debug SDK rebuild, dependency upgrade, PBX or audible test was requested.

## Connection deadlines and retained contextual panels

Read the connection-resilience, error-management, network-cloud, native-lifecycle
and capture routes plus the actual donor Session, unified mobile transport,
workspace profiles, peripheral availability and LAN resolver/tests. Existing
hash-bound mappings were extended, not replaced with a bulk completion claim.

The actual ASUS baseline ran605.38s:112 samples,27 Windows notifications,
27 USB and5 Wi-Fi panel-list changes. It exposed immutable USB snapshot
timeouts, status/mapping HTTP timeouts and transient AVR-online notices. The
first updated600.60s window retained the panels (one initial transition per
method) but still had read failures; it was not accepted as a timeout fix.
Read-only ASUS HTTP timings then measured2.04..2.19s per hostname lookup.

Adapted read single-flight and pre/post-write retirement in Flutter; actual IO
requests now abort on deadline/owner change. A failing real-socket oversized
response test exposed IOClient's lazy onListen cancellation hook; subscribing
before abort fixed it without weakening the test. Prior ACKs retain panel slots,
not write authority. Native full USB reads use four token-bound chunks and one
unchanged absolute8s capture/freshness deadline; command ACK deadlines/replay
rules remain unchanged. Progress is typed IPC, not new toast identities.
Verified TCP routing retains hostname/Host/SNI and the old hint during bounded
relookup; HTTP/auth/body timeouts do not invalidate DNS. Same-board recovery
retains selection/drafts, while current identity and capability ACKs gate writes.

Actual pinned ESP32 connection APIs were inspected independently from TLS:
MQTT TCP connect is bounded, retry is scheduled from completion, and measured
attempt durations are additive telemetry. DNS remains synchronous. Existing
cloud tests model slow20s attempts, rollover and completion-relative delay on
both platforms; this is not proof of a fully nonblocking cloud implementation.

ASUS owner replacement was graceful, hash/PID/session/draft checked, settings
backed up and the previous executable retained. A PS5 -File Boolean-switch
binding failed before shutdown; a read-only reviewed invocation and corrected
binding preceded replacement. No forced kill, settings rewrite or board command
replay was used. S3 OTA accepted the new image but its final range readback was
interrupted. No second upload followed: read-only full export independently
verified1,924,224bytes SHA256
71b66e9bd43fb3fa4ff92933696428f6169562a4b508bd10c99b33e6e15a41ff
on the same board's new boot fd454041772f20e7aeb44e4201816186.
The AVR image/fuses/EEPROM were not changed in this pass.

Focused native USB/presentation/notification/peer-HTTP tests,32 Flutter tests
across API/panels/deadlines/realtime/workspace lifecycle, analyzer, APK preview,
cloud fixtures and all six linked flash wrappers passed. The Linux CI failure
was a missing three --wrap flags in its host test, not missing production guards;
its command now matches the existing PowerShell runner. No protection or test
was disabled. Native/firmware0.2.7 and mobile0.2.7+7 remain consistent.

Final native candidate b8109cf50c209ac95c890a7c7c59ce737a6c80ff1b9cd4127445226d80978ca1
is separately delivered for another600s live observation. Final window results
must be reported separately; no phone was authorized on ADB, so APK build and
widget/loopback evidence are not Android installation or human acceptance.

That window still exposed USB capture expiry and HTTP/WS stalls; retaining
panels alone is not timeout acceptance. Actual broker logs showed repeated clean
DISCONNECTs, not duplicate-client takeover evidence. NetworkPath treated DNS-only
refreshes as route changes and closed established sessions; it now updates DNS
without tearing down TCP, while real interface/address/gateway changes still
retire sessions. Actual Wi-Fi and Ethernet policy fixtures passed. Additive
route/resolver and cloud close-cause counters distinguish the runtime causes.

Large cloud telemetry wrote all packets in one loop pass; ordinary yield() did
not drain USB. Cooperative read-only host progress now runs between payload
writes/fragments and around socket calls, with explicit no-recursion/partial
packet guards. USB mutations receive Busy while cloud I/O owns the loop; nothing
is replayed. Buffered AVR replies are drained after cloud work before presence
expiry, avoiding an unprocessed reply being reported as peer loss. Individual
DNS/TLS/socket calls remain synchronous. Both-platform cloud fixtures exercise
progress between real writes, timing, recursive-call refusal and partial-write
TCP close without replay. A new actual S3 build passed; deployment and another
live window remain separate evidence.

All264 native tests passed through the canonical warm release/test cache. The
WebSocket worker regression initially failed because its server fixture
mistook the new read-only liveness probe for a mutation, then mistook the probe's
initial status for a command receipt. The fixture now answers liveness separately
and waits for verified command completion plus its readback, preserving exact
wire-envelope and under500ms assertions. All5 realtime tests passed. No command
verification or production deadline was relaxed to make the test pass.

The third live window completed602.72s/251samples/15 notifications: USB96/125
online and Wi-Fi123/126 online. Its persistent failures were not accepted.
After the additional cooperative/DNS fix, one LAN upload transferred22.00s.
The updater's final range-readback connection reset again; no upload replay.
Independent full export verified1,925,984bytes SHA256
b3e6ed4225111c5671e06177b8b196759a976bc4bcc99e80af5651ebb3e2017a
on new boot03fdc1ae101783ede943ebb1e95682b2. Actual first counters showed one
MQTT attempt/zero closes despite five resolver changes and one route generation;
maximum measured service2163ms, publish263ms. A new600s owner observation was
started only after that readback, with no other serial reader or device mutation.

That fourth window completed601.42s/249samples: USB123/124 online with zero
active-error samples; Wi-Fi124/125 online, but39 samples had HTTP status/mapping
read failures or abrupt WS resets. Each method's single offline sample was its
initial handoff. Both retained exactly Events|Front panel after acquisition,
one unchanged boot, and USB completed captures stayed at or below2145ms.
Windows notifications fell to2, not zero. Raw ASUS receipt SHA256
6ca583851b649e1eb0eebdee78557e4fa2b539d7689bad85a073240713ece9cd.
Actual stable-frame capture SHA256
784ac899ef535b4bd96ec6b8d6edd1ed2d34e014f889c7a851859f97fa048f1f
has clear status message and zero active diagnostics. This is panel/USB progress,
not a clean Wi-Fi soak or proof all timeouts are fixed.

Final native WS setup removes its independent DNS/HTTP bootstrap entirely:
numeric or actually verified shared routes only, quiet interruptible startup
waiting, selected-route checks and absolute bounded handshake, preserving
Host/SNI/TLS validation. Final native suite268/268 and mobile analyzer plus
209 tests passed (two opt-in capture tests skipped). Mobile producer tests also
reject older HTTP/WS observations before renewing freshness; application changes
retire old mode shells without clearing per-board user selection/scroll. The
old melody test incorrectly required disappearance on AVR loss; its replacement
checks identical bounds, disabled controls, no offline dispatch and fresh-cue
recovery. No freshness/deadline test was weakened. Preview package SHA256
a621a579ff3436bc3784bcf45934cbda1df7fc532fcdca350855d8f3d4933934.

Remaining Wi-Fi attribution is open: pinned AsyncTCP5s ACK timeout directly
closes WS TCP; whole status broadcasts ignore backpressure/send outcomes; HTTP
dispatch waits for loop ownership and response attachment for AsyncTCP polls.
These are reachable source paths, not proven causes of each observed reset.
Queue-full does NOT force-close by default in the pinned WebSocket library.
Per-stage HTTP and WS queue/ACK-timeout counters are the next evidence step;
do not claim a hardware fault or hide incidents by raising read deadlines.

Final native candidate SHA256
9b0d8c01a72de24d3b994a6d8c4c8771888c04f665630bdd7ceda5f09ef27e8e
was gracefully delivered as ASUS PID2068 with settings/rollback preserved.
Its62.31s USB/Wi-Fi smoke had25samples, zero active-error samples and stable
connected panels; it is not another ten-minute soak. Raw receipt SHA256
5fdb830a25a9934099f8fdf6d156a4603e9939699de1cfd09795da57045a7a92.
The preceding full window's two Windows notices occurred at7.48s/305.84s,
both successful connection confirmations during deliberate transport switches.
No disconnect notification was inferred from those counts. Android remains
build/test-only until an authorized phone is available.

The source checkpoint's GitHub quality job exposed an obsolete literal native
guard in web_context_test.mjs: it demanded live-gated disappearance. The guard
now asserts retained presentation slots AND the enclosing panel_live/fresh-peer
gate for every retained peripheral; existing dynamic Web authority tests remain.
The complete canonical local `ci-checks quality` phase then passed. This is a
test/ledger-only correction, with no further app/firmware rebuild or deployment.
Remote exact-head CI continues asynchronously; no green result is invented.

## AVR ISP and Urboot programming paths

Consulted firmware-hardware, avr-boot-chain, remote-ota and exhaustive literal
PCController programmer inventory. Actual preflash.go and urclock_retry.go were
read completely and match the inventoried b0111ae0 original-byte hashes. The
existing stock Urboot builder/README were rechecked against their exact live
hashes; its custom512-byte/wrong-pin backend remains excluded from deployment
pending explicit profile adaptation, not silently substituted.

Adapt the actual preflash source reinspection/durable-backup/no-write-replay
boundaries into a single S3 maintenance job. The published Urboot wire
specification, actual pinned stock source MCU IDs/feature encoding and installed
MiniCore image were inspected: modern Urboot is NOT ordinary STK500v1. The
small independent protocol implementation uses byte addresses,128-byte pages,
consistent device-specific sync bytes and actual boot metadata. No copied GPL
bootloader source or new compiler/toolchain installation. No chip erase/EEPROM
write/boot-region write through UART; ISP remains the explicit recovery method.

Actual initial signature-only ISP read returned1E950F. Existing flash, EEPROM
and fuse metadata were exported to ASUS before reset. The legacy fresh-backup
request lost its HTTP response; matching retained files and later UART recovery
are NOT, alone, a positive fresh-capture completion receipt. That failure drove
the shared queued ISP/Urboot job and cooperative existing ISP callback instead
of a blind retry or a longer opaque script. New runtime evidence is recorded
separately after actual deployment. No fuse change is introduced.

The first Web progress test caught equal missing boot IDs causing a missing
optional-object dereference; require the object and a real boot identity.
One Dart braces lint was fixed without disabling analysis. Gates caught changed
proof hashes after reviewed formatting; update only their actual target/proof
bindings in the same ledger. The UART emulator and actual ISP callback/
preservation tests are not electrical, flash-hardware or physical acceptance.

Independent read-only review found maintenance-boundary HTTP/fragmented-WS
ingress, expiry-before-reset and stale ISP UART recovery gaps. Corrections bind
queued HTTP and complete/partial WS messages to an ingress epoch, close Telnet
and its partial editor, stop/rebind UDP, and discard USB ring/partial COBS input
at release. The same actual SerialLink fixture now exercises the discard
boundary; async HTTP fixtures reject both pre-maintenance and maintenance-era
queued mutations. ISP restores a clean framed parser and accepts only its newly
requested Hello sequence. Expiry is checked before target effects, Prog arming,
reset and first programming; no cancellation/retry is injected mid-flash.
Urboot accepts only the reviewed e7/features20 stock profile, rejecting custom
self-patching/dual-boot profiles. These tests are not concurrent-client soak.

The canonical updater Plan exposed two PATH Node executables becoming one
invalid command string. Read actual pinned PU850 Tool/build-all.ps1 Git blob
through EOF; retain the imported runtime's single PATH-preferred executable
resolution instead of installing another SDK. The root Plan now works with both
installed Nodes and the updater fixture passes. Local OTA responses/readbacks
can still be interrupted: inspect boot, phase and running image before another
explicit update; an aborted5124-byte upload is not successful installation.

Actual final evidence is now in REMOTE-MAINTENANCE.md: pinned Internet ESP job17
and separate real same-image AVR Urboot job1/ISP job2 all completed, with flash/
EEPROM verification, fresh UART/application recovery and Prog clear. Preserved
ASUS receipts distinguish these from earlier response-loss-only attempts.
No fuse writes, audio/PBX test or physical LED/display acceptance was performed.

The existing full CI was failing a source regex requiring a removed native
Firmware-page polling block. Reread complete actual PCController link/session.go;
retain its owner admission/identity checks in the new assertion for selected
network/USB observations, with negative mutations. Do not restore unnecessary
polling. The S3 source guard likewise now requires both disabled-link and raw-
UART exclusion instead of the obsolete single-condition spelling. No production
guard was weakened. Rebind only reviewed test references in the existing ledger.

## Firmware subscription, file delivery and external update progress

Consulted updates-telemetry, remote-ota, network-cloud and exhaustive PC donor
inventory. Fully read actual canonical hostbridge/manager.go (1851lines),
hostui/update_progress.go and its tests, artifacts/download.go and mobile
firmware.dart; exact live bytes match the existing inventory snapshot. Add five
bounded complete-file comparisons and firmware-delivery index route in the same
ledger; unrelated manager/OS notification/artifact-store/USB/background-service
parity remains explicitly pending. Reread actual PU AsyncOTA.cpp and the current
integrated OTA callbacks; retain raw-size/stage/ACK versus completion boundaries.

Native and Android adapt the real outbound WS/Socket.IO open, namespace,
heartbeat and event tuple to a finite, explicit peer-bound firmware inbox. URL
offers require a trusted origin, unique ID, current session/board/boot, deadline
and SHA-256. Manual local/HTTP(S) images share guarded existing board upload
routes. Source credentials stay separate from board auth; redirects and bad
digest/size/format cannot write the board. No new disk artifact cache or second
serial owner. Native feed configuration is a preserved dirty draft until valid
Start; secrets remain memory-only. AVR/paired programming stays explicit.

Fix the actual missing DTO path: native previously dropped update/remote job
fields. Rust/USB/bridge now retain them, Android consumes existing status events,
and Web uses its normal status stream. No redundant fresh-WS job polling or
fake flash percentage. Transfer acceptance is informational, not completion.

Focused native real WS/Socket.IO and HTTP download/upload fixtures pass; initial
SHA2-0.11 output lacked LowerHex, corrected using explicit byte encoding rather
than downgrading or disabling digest checks. Android analyzer and six focused
delivery/progress/socket cases pass. Its initial24 brace lints were repaired by
the analyzer's scoped mechanical fix; no lint was disabled. Source-hash gate
caught formatter drift and was rebound only after reviewing the affected edits.
Final package/ASUS/board observations and remaining actual phone/WSS endpoint
acceptance are separate from these fixtures; do not claim those from builds.

Final analyzer and six firmware-page cases pass after adding the missing
dart:async import to the new same-board reboot fixture. That fixture proves
fresh read-only controller rebinding and retained controls without replay.
Canonical release/preview/production-profile SDK builds pass. Actual ASUS USB
owner observed external Internet job16 download at38% and81%, and verified
completion on the new boot after an explicit native CLI Apply. The earlier
MQTT Apply had no receipt; its unchanged ready state was checked before that
new explicit action, not blindly replayed. Native owner/settings/rollback,
image/boot/digests/slot, final APK and remaining physical/phone/service gates
are recorded in REMOTE-MAINTENANCE.md. No actual AVR erase/reset/audio test.

## Board-advertised LED wire implementation

Consulted device-context/led-cell-test and literal donor-audit. Fully reread
PCController peripheral-availability.ts/tests, AddressableLeds.cpp/header and
Pealayer board_info.rs. Preserve explicit peer-owned capability/measurement
separation; AVR register timing and unrelated settings are not copied onto S3.
Add backend/rmt/dma metadata from the actual selected output worker through the
existing common status builder. Live driver JSON shares that definition, while
readiness/active/error remain separate observations. HTTP/WS/MQTT/full USB
snapshots therefore share one statement. Native typed serialization and USB
compact readback preserve it; optional invalid/missing metadata is unknown.
Rust/Flutter/Web use compact contextual read-only labels, not an editable box
for a build flag. Existing source/proof hash references are rebound only after
reviewing these additive changes; no broad donor-parity claim is made.

Actual GPIO/RMT-DMA worker fixtures (both channel orders), renderer fixtures,
three focused native capability/USB/bridge tests, Web context/label assertions,
Flutter analyzer and six peer-context tests pass. A Dart brace lint was fixed
without weakening the analyzer. Previous main CI then exposed a Linux runner
compiling the default static worker but invoking DMA-only scenario names. The
Linux runner now uses the same four explicit profiles/scenarios as PowerShell;
its exact Bash worker loop passes locally without weakening assertions. This
is not a complete Linux CI run. Same-file hash-bound proof is refreshed.

Actual ESP-only MQTT/HTTPS job15 completed with a new boot and expected running
size/MD5/integrity and postboot image SHA verification. Its capability reports
gpio-static/rmt=false/dma=false over actual MQTT, HTTP, WebSocket and the ASUS
app's USB COM22 path. ASUS native replacement was hash-checked and atomic after
confirming no running owner; settings and prior executable are preserved.
Actual owner IPC reports USB selected/online with the same backend metadata.
Android preview builds, but its transfer was interrupted and the authorized
phone is now offline; no installation or mobile runtime acceptance is claimed.
See REMOTE-MAINTENANCE.md for image/boot/package evidence. No backend switch,
AVR programming or optical success is inferred.

## MQTT, Internet delivery and opaque maintenance forwarding

Consulted network-cloud, updates-telemetry, remote-ota, bridge-repeater and
donor-audit; fully read PU FirmwareTransfer.cpp/header and canonical PCController
preflash.go/restore_preflash_test.go. New hash-bound comparisons retain complete
backup/cancellation/candidate-reinspection boundaries, with concrete pending
host archive/executor omissions. Transparent forwarding uses the existing ESP
updater/ISP coordinator, not a second serial owner or programmer.

Fixed the native bridge's JSON-only assumption: binary firmware/AVR exports,
real multipart bodies, HEAD/Range/ETag and actual201/202/error replies survive
without reinterpretation. Fixed allowlist, peer/boot/route lease, loop trace,
bounded body/export sizes and no-replay semantics remain.16 focused bridge/socket
tests pass, including valid multipart, stale boot before dispatch, HEAD and a
truncated stream that cannot yield a successful completion receipt. Network
capabilities are ACK-gated; unsupported USB maintenance/streaming stay hidden.

Latest main CI failures were inspected, not bypassed. Fully reread Protocol.h
and actual donor UartProtocol.h: additive USB snapshots/bitmap commands did not
change framing or tunneled dialect. Corrected stale SOURCE-MAPPING target hash;
23 exact mappings and the50 PU route check pass. ESP8266 SDK linking exposed
missing atomic fetch-and, add and compare/exchange routines. The first CAS-only
attempt was insufficient. Consulted actual SDK interrupts.h/core feature code;
use its bounded previous-level-restoring InterruptLock only for ESP8266 state
copies, retaining native/S3 atomics. Both production branches have host fixtures;
real ESP8266 busy-wire SDK compile/link now succeeds. No SDK mutation, libatomic
stub, weakened assertion, fresh cache or cold clean was used.96% RAM/IRAM on
ESP8266 is a remaining headroom risk, not S3 production readiness evidence.

Actual scoped broker provisioning, full checksummed Internet telemetry, separate
MQTT HTTPS stage/apply/new boot, ASUS native replacement and LAN discovery are
recorded in REMOTE-MAINTENANCE.md. Probe correlation initially accepted an
unrelated null-ID event; corrected to unique IDs plus typed command replies.
Paho callback API2 uses explicit reason-code equality, and socket-written versus
accepted outcomes are distinct. Failed probes remain preserved, not reported as
successful command round trips. Windows shared5353 contention was diagnosed
using the actual LAN; default ephemeral-port probing receives real service
records. ASUS helper IPC must run in its interactive session, not SSH session0.

No current AVR flash/reset/fuse operation, physical display/audio/LED acceptance,
Android installation, broad donor parity or all-green exact-head CI is claimed.
Independent HTTP export still timed out; keep that operational defect open.
Its explicit read-only Range resume completed with the exact full running-image
SHA256; no firmware mutation was replayed. Single-stream reliability is still
unaccepted. Postdeployment LAN probes again returned all four requested discovery
protocols, and actual ASUS interactive owner IPC confirms USB selected/online.

## Operational Flutter pixel diagnostics

Reread canonical PCController Project/AddressableLeds.cpp and .h at recorded
43417181/a28009e6 hashes, peripheral-availability.ts, and Pakar's actual native
diagnostic/frame path. Retain the donor clear/stage/show transaction on the ESP;
do not copy AVR timing/registers or its100-pixel limit. The prior Flutter path
required a live socket, separately switched modes, sent two unacknowledged raw
chunks and polled large HTTP snapshots. Its UI advanced before success and hid
All off below multiple sections. The new optional acknowledged diagnostic command
performs the complete operation on the loop owner, returns full RGB readback and
uses existing same-ID HTTP recovery, never a new mutation after lost WS receipt.
Compact controls, six tappable wire indices, verified logical mapping labels,
board-keyed cursor/colour and failure-retained cursor use the existing workspace.

The first canonical mobile test stopped at the expected changed peer_context.dart
hash gate; reread the donor availability source and rebind only reviewed affected
targets in the same ledger. Seven real loopback command cases, seven page cases
and six older-stream transport cases pass; the opt-in physical case remains
skipped. Actual renderer tests include tail/group/all-off/conflicting stream and
invalid-input no-partial-mutation. No screenshot or buffer match closes optical
acceptance. Initial Resolve-DnsName/Win32_SerialPort inventory missed the ESP;
direct hostname HTTP and present PnP identity subsequently confirmed ledboard
and CH343 COM22. No authorized phone is connected, and its advertised ADB endpoint
times out. The first ESP compile caught an incorrect local enum name kModeCell;
use the existing shared Protocol.h kModeCellSelect, not a new alias. No cache or
toolchain change is needed. Build/stage and device/phone installation are separate
gates. Existing unrelated ESP8266 CI atomic-link errors remain outside this slice.

ESP32-S3 16MB/OPI/static/buffered compilation passes. Initial default unverified
ISP profile75dd9f7a was compile-only, not a production replacement; preserve the
live board's existing ISP capability with -S3IspElectricalVerified. That exact
profile is now recorded in AGENTS.md so future passes keep the same flags/cache.
Correct1904976-byte candidate SHA256
cdf162ae73fd3069498264238016588ef23ff737c424e0e79fc333287d44b183
is staged on ASUS, NOT flashed. Final APK
f0e9ad15effc236247d7b6513e6b33234253d4b26251246998489eccecf3b941
is built; Android installation remains unverified. An opted-in live transport run
through the SSH forward failed with connection closed before complete HTTP
headers; this is NOT a passing physical check, and no mutation is replayed.
Direct ASUS hostname HTTP works. A small compiled runner imports the real mobile
API, requires explicit endpoint/board/boot/new receipt and a full black raw frame,
and exercises one/six/tail/off without a remote SDK install. The canonical mobile
wrapper retains the same cache/storage/locked-package guards in PixelProbe mode;
its compile-only plan fixture passes. Reread actual pinned PU Tool/build-all.ps1
and Pealayer storage-hygiene.ps1; no clean, SDK copy or extra cache was introduced.
Standalone Dart receipt creation uses actual SDK exclusive create, not an assumed
FileMode constant; package imports fix the analyzer's relative-lib warnings.

Live scene observations changed between all-off, white and red/off while work
continued. The one-shot OTA preflight refused before dispatch (saved receipt has
dispatched=false, unchanged old firmware MD5, idle/zero upload). Do not retry or
overwrite that receipt. Pause competing LED controls and obtain the user's test
window before update/temporary clears. The phone remains unavailable; source,
loopback, framebuffer, built packages, board update and optical acceptance remain
separate. No AVR operation or desktop replacement was performed in this slice.

Further actual Dart diagnostics found the concrete live WebSocket failure:
custom connectionFactory used Uri.port0 for ws://ledboard.local with no explicit
port (Windows SocketException10049). Preserve the HTTP source port on ws/wss
conversion and use80/443 defaults in both native and cached socket paths.
Initial native hostname connection avoids a separately timed-out mDNS preflight,
caches only a successful TCP address, retains TLS/SNI and original Host, and
classifies genuine lookup failure for read-only discovery. Start socket and HTTP
reads independently; first same-origin full socket status may establish its
board/boot fence without HTTP preflight. Socket disposal has a bounded500ms grace.

Six focused real-socket/DNS/proxy/TLS cases and four workspace lifecycle cases
pass; seven atomic diagnostic command cases still pass. Production mobile API
compiled into the adjacent-PC runner subsequently connected by default-port
WebSocket on ASUS, read board esp32s3-907069eac800 / boot
360d2ef9857b3220b0c037c3ea6671e0, and rejected a deliberate foreign identity with
zero LED commands (pixel-probe-default-port.json). Published RGB/optical tests
remain blocked on the user's quiet test window; the running ESP remains old
image8bc0cd92. Pure lookup still times out on ASUS while native HTTP/socket paths
can work; general LAN/HTTP truncation reliability is not declared fixed.

## Flutter lifecycle and stable board workspace

Consulted peer-binding, mobile-settings and literal Pealayer donor-audit.
Reread canonical Session retirement/admission and complete live
src/ui/workspace_profiles.rs at matching410d274f source hash. Adapt its stable
user-owned IDs into board-keyed Flutter UI state; retain the existing command
owner fences separately. The same comparison ledger now binds workspace source,
consumer and lifecycle/transport proof. Generic workspace-manager CRUD/icons
remain an explicit pending donor import, not an accepted exclusion.

Telemetry expiry previously removed Table from the IndexedStack and changed
connection-revision keys. Background timers and hidden settings also issued
reads, and HTTP refreshed the socket-health clock. Retain the last acknowledged
layout while disabling expired writes, preserve local selection/query/effect/
scroll by board ID, and save bounded UI-only cache once on changed background
state. No device snapshot, command or authorization is restored from that cache.
Pause foreground supervision without closing a healthy socket; resume with one
read probe, independent WS freshness, quiet bounded reconnect and HTTP fallback.
Hidden/inactive optional readers stop before another rendered frame is needed.

Focused widget fixtures use the actual Flutter inactive/hidden/paused sequence;
their initial missing lazy canvas and invalid lifecycle transitions were harness
issues, corrected without relaxing production checks. Real socket tests remain
isolated from TestWidgetsFlutterBinding. Source/fixture acceptance does not
establish Android/Doze/router-loss or physical LED acceptance.

## Mobile deliberate pixel diagnostics

Consulted games-streaming, led-cell-test, mobile-settings and literal donor-audit
for PCController Project/AddressableLeds.cpp. Read its complete current cpp/header
through EOF; source hashes match the existing ledger. Reuse the existing native
pixel_diagnostic.rs, PKAR chunk/commit format and shared Studio Flutter surfaces.
The AVR register sender and100-pixel limit are intentionally not imported into
this480-pixel ESP client; the existing concrete omission remains authoritative.
The same AddressableLeds ledger rows bind mobile source and focused tests.

Analyzer and new unit/widget/real-loopback tests pass. The first aggregate live
sequence hit the runner's default30s limit after exact RGB readback for wire
pixels1,6,7. No failed mutation was replayed. A separate explicit all80-off batch
and static output selection cleared the published480-pixel buffer on the same
board/boot. The live harness now has120s aggregate budget; per-command deadlines
are unchanged. A later delayed RGB observation exposed the too-short2s
confirmation budget: reuse fresh same-boot WS/command snapshots before sending,
and bound read-only confirmation to8s without adding any mutation replay. A real
loopback delayed-snapshot regression passes. The next live preflight connection
closed before its HTTP header; this did not authorize another diagnostic write.
Full tail/group live sequence remains unverified. Phone installation/physical
optical acceptance remain separate; no board reset, flash or AVR cue was run.

## Native board settings and application preferences

Consulted preferences and donor-audit, owning issue4 and the existing settings
catalog/editor before implementation. Reread the complete canonical PCController
Project/SettingsStore.h/.cpp (raw SHA256 ccdcfb6e1ab186abd6a4f0b5a302bb14bab295c9a0bd9b4a306036fa32f1eaf3
and 7309fd36f0680722a4260e8f1251ae5a0fc67e99fc60f2426bb1294f789e87d4).
Reviewed Pealayer's existing PreferencesDraft/clean navigation/section controls
and organization guide; its mapped source hash remains unchanged. Same ledger
rows now bind the native full-record editor, transaction and tests. No new
whole-donor completion certification or incompatible donor EEPROM layout copy.

Device Settings / AVR & inputs now offers all supported saved Pakar fields with
explicit draft/save, validation, conflict/readback and pending-close ownership.
Both selected USB and fenced HTTP use the same GET/SET/GET transaction. Fresh
baseline disagreement refuses SET; exact ACK and persistent full readback are
required. This is optimistic conflict detection, not atomic multi-client CAS.
LED settings uses the same actual controls as the live panel. Preferences Board
now links to device settings for any acknowledged board, preserving dirty local
preferences. USB header shows the selected port, not an unrelated HTTP endpoint.

Initial Check exposed one local borrow lifetime in the expanded Preferences
view; calculated availability before mutation. Reused the canonical build gate
and owned-file hash rebinding, not direct cargo/cache bypass. A ledger patch
initially assumed uniform JSON formatting, then omitted the closing row comma;
both failed without changing source decisions. Retained actual row text and
delimiter, updating only the same three reviewed rows. No parallel ledger.
Native Check, 11 initial focused tests and final 24 settings-focused tests pass.
No board contact, sound, flashing, native replacement or APK build. Mobile/Web
full AVR editor and physical/installed visual acceptance remain open. Large
pre-existing uncommitted changes were preserved, not bulk committed.

## Cell-selection flicker, cable mapping and two-driver chase

Read AGENTS/preflight and owning issue2; consulted games-streaming and exact
donor-audit PCController inventory. The guessed LocalLib/LedStrip.h source does
not exist; the actual dependency is Project/AddressableLeds.h/.cpp. Read both
canonical files through EOF and verified unchanged raw hashes
`a28009e6211056f0cdb1071e7232d6d277a27cf9dc8e3e9951be9830745f18d4`
and`43417181b23cab40b8a58ec26159ca025fb41d5bdbaf52d20a46bfc926fee1b0`.
Retain donor buffer/show separation, not AVR D6 timing or a duplicate engine.
The new led-cell-test route binds both complete source reviews to exact mapping,
renderer/worker/boot/build and actual-test hashes in the same canonical ledger.
Gate passes2 matched/0 stale; full donor parity remains incomplete.

User cable sequence is80,60,40,20,10,30,50,70 then subtract1 per eight-group lane.
Mapping tests exposed the important distinction: that pattern ends with physical
group80=logical61; logical1 is group77/pixels457–462. Asked for clarification,
and use a logical80-down-to1 chase so all80 cells are visited and it stops at1.
Each step composes six identical red pixels and474 white pixels before one
publication.750ms dwell is board-owned, no per-cell HTTP/flash/AVR work. Test
boot override is RAM-only and opt-in; ordinary saved mappings/names are retained.
The named mapping preset preserves names and saves the full permutation once.

Source inspection found RMT-DMA continually replayed unchanged enabled frames.
Removed unsubmitted replay: both workers now consume immutable complete frames,
clear startup black, and sleep until a submission. Renderer repair refresh and
uncapped rainbow remain. This is a candidate firmware improvement, not measured
proof of DMA refill/power/wire correctness. Tests also caught/fixed the delayed
first-render skip and Off/On immediate-advance edge cases; no skipped catch-up,
loop or fade. Source states/frame snapshots are not optical sensors.

Actual renderer, FIFO,10000 full80-cell batches per render mode, static/DMA
RGB/BGR worker scenarios (coalescing, unchanged sleep, startup/Off black,
flash holds and failure), new boot/preset wiring and S3 source guards pass.
First mapping assertion correctly failed because it conflated cable termination
with logical chase termination; corrected the interpretation rather than
changing the permutation to fit an unsupported last-pixel assumption.
Ledger patch initially failed on a row delimiter; reread exact original lines
and retained each row's actual trailing comma without wholesale JSON rewriting.

Final SDK candidates, including delayed boot/Off dwell and completed-active fixes:

- RMT-DMA:1881839/3342336 application bytes,99084 static RAM; raw1881984-byte
  `build/led-comparison/rmt-dma.bin`, SHA256
  `0a4dcf47b2856a600adbe27c4852c2e0987943ffee980508f1c3c147d04e3800`.
- Non-RMT GPIO:1881747/3342336 application bytes,99084 static RAM;
  raw1881888-byte`build/led-comparison/gpio-static.bin`, SHA256
  `98ddf93d90e9ff87c267b6f04a2cf10a662057dd720db41c0e7df79ee6c80c33`.

Build uses16MB/OPI, buffered, PauseAvrLink, CellChaseOnBoot. A preliminary DMA
compile before the dwell fix was superseded, not copied or deployed. The earlier
23b4e75d candidate was replaced only after verifying that exact owned hash.
No clean, debug cache, SDK upgrade or donor mutation.
Only the small comparison images need retaining; no duplicated package tree.
Host has about5GB available RAM/64GB and31GB pagefile already in use; next full
profile build uses2 jobs instead of worsening memory pressure with all16 CPUs.
Other running applications/processes remain untouched.

Reconsulted build-storage and reread actual Pealayer LOCAL-WORKSPACE-HYGIENE.md,
plus pinned PU Tool/build-all.ps1 through the GitHub content API after the guessed
local sparse-donor path proved absent. Do not create another checkout/cache.
Removed backend selection from global compiler flags: canonical wrapper now
writes a changed-only application header at build/config/esp32s3/LedBackend.h.
Plan is write-free and has identical SDK/compiler arguments for both backends.
Exclusive FileShare.None lease covers profile write and the full compile; tested
that another handle is refused while building and that the lease releases.
Full build-storage Plan suite and actual worker profiles pass. Actual compiled
worker literals distinguish GPIO from DMA correctly, not merely a plan string.
All150 cached SDK/library object/archive sizes and modification times remained
unchanged across final warm static and backend-switch DMA builds; metadata
fingerprint7b61bad0ea149181e8f782545bc3e38417779d26b6e6d714d2c56f3d63f71821.
This is measured cache reuse, not a claim of zero writes or measured SSD wear.

ASUS8022 refuses, hostname paths do not resolve, and former58080 forward is
absent. Bounded read-only last-address and authorized server hostname attempts
also failed. No board/UART/AVR/ISP/OTA/phone/native deployment occurred. Existing
PBX service query does not normalize the human phone like its call command:
the first raw-number query expired. After verified expiry/no active channel,
used the canonical trunk target; that separate attempt also expired without
DTMF. Neither is delivered-call or optical evidence. No automatic redial or
service/dialplan change. Prepared separate RMT/GPIO visual prompts with
1=stable/correct,2=flicker,3=wrong mapping,4=no output, for use after deployment.
Private task-local PBX query adapter/prompts are outside shipping source.

Tracking: https://github.com/atomicdeploy/PakarController/issues/2#issuecomment-6246618912.
Source remains the preserved uncommitted worktree based on2dc319fb; no release,
complete CI or physical flicker-fix claim. Both exact-image deployments, pixel
readbacks and driver-specific PBX/DTMF visual acceptance remain open.

## Finite attention/update melodies and companion-driven phone mirroring

Reconsulted board-silent, mobile-mirroring, remote-ota and donor-audit. Fully
reread actual PCController TonePlayer.h/.cpp, Project/BootMelody.h/.cpp,
hostbridge/buzzer_mirror.go and its entire514-line test source at current local
b0111ae0c80dc34149ea1044de7dd5488920301b. Four existing tone/mirror raw hashes
still match the inventory. Actual PU850 AsyncOTA.cpp reread through EOF; retain
explicit start/end/error intent, not PU UART/watchdog callbacks. Donors unchanged.

Implemented ACK-advertised additive attention cue ID14,240ms (C6/90ms,pause60ms,
E6/90ms), using the existing single Timer1 player. All prior14 IDs and original
welcome schedules remain. Desktop/Web/mobile catalogs agree; old peer counts do
not acquire the new cue. Existing fenced API/audio command provides operator
attention; physical Silent and Prog still suppress sound.

Both S3 and8266 local upload callbacks now publish atomic/coalesced notice intent,
including error/abort and explicit finalization. Only the loop emits one UART
cue; no750ms ACK wait, guessed fallback tone or replay after companion recovery.
Remote phase/job observation sends start only on apply, verified completion only
after the existing terminal checks, and uses a cooperative500ms pre-Prog lead.
Missing audio is not a firmware/erase safety acknowledgement. Local commit cue
means finalized upload before reboot, not proof of the subsequently running image.

AVR note publication previously required an active desktop contract session.
Added a separate three-second companion mirror lease on real HELLO/PING/status
traffic, without host/menu/macro authority. Buffered/busy-wire sends one changed
frame per loop; polling receives one solicited changed mirror frame. Audio data
uses one shared timestamped encoder and the original13-byte payload. Polling
can undersample short notes; no missed-note invention. S3 buffered is preferred.

Preserved donor-independent, explicitly enabled phone audio even while physical
AVR Silent is on; that was intentional, not a missing-mute bug. Added explicit
Follow board Silent mode and flush on policy change. Mute/unmute metadata cannot
restart one musical step or resume an interrupted note. Fresh short notes now
rebuffer after missed transport cushion; device-expired notes remain expired.
Stop, duplicate/owner/lease/lifecycle cancellation and finite native PCM remain.

Focused audio/AVR production-code doubles pass all15 cue durations/mute/rollover,
upload coalescing/terminal/no-old-completion, companion-no-desktop/polling/expiry
and no-host-ownership cases. S3 menu-audio, Web picker ACK filtering, updater
loopback and script/actionlint checks pass. First Flutter analysis correctly
rejected two unbraced new ifs; corrected the actual statements without lint
suppression. Analysis then passed, all7 mirroring tests and separate real loopback
WS test passed. Native locked release-profile Check passed with existing warnings.

Actual warm-cache SDK builds pass, source versions0.2.7:

- AVR buffered:31538/32384 application bytes,1465/2048 static SRAM. Full32768-byte
  stock Urboot/vector-chain image verified, SHA256
  4f3f00454ec913551c88da4746ea11955087835c9619e6331406c67f15034fb1.
- S3 buffered16MB/OPI/RMT-DMA/W5500/ISP build acknowledgement:1995639/3342336
  application bytes,99424 static RAM. Final raw bin1995792 bytes, SHA256
  6006ecf027f16cdc821c93a81f52c70af7b0d4f4b522f457efbfde7079c8a05d.
- Android preview APK55740293 bytes, SHA256
  c98e83381b64a1c25817d75b22ff17cb6dacca4fa1c353bc653b3955036f5fba.
  Existing preview signing/cache; no new SDK, clean, debug or package copy.
- Native Release build passed (3m13s, existing seven warnings), executable
  26200064 bytes, SHA256
  8561faf3a8970aa83c72452bf7a75389122cb18d8f5e668cbead95a52b896037.
  CLI-only --version probe returned PakarController 0.2.7 with exit code 0.
  It was not launched/replaced as a GUI or accepted visually.

ASUS SSHlocalhost:8022 refuses. No board write, phone install, audible cue,
speaker/server/PBX action, fuse/EEPROM change or physical acceptance this turn.
Local ADB inventory is empty as well. The final S3 recompile includes the
fresh-HELLO/version-known gate on every automatic cue, not stale capability
cache alone; it supersedes the earlier1995760-byte candidate in the issue note.
Native/Web host-speaker consumers remain unimplemented; this slice fixes actual
AVR publication and existing Android consumer, not those remaining consumers.
No current GitHub CI or production-release claim; preserved large worktree is
uncommitted on main2dc319fb. New canonical comparison total189/4673,4484 pending,
147 known gap files. Seven-source audio-attention gate and report checks pass.

Four changed runtime/header contract-manifest entries were rebound only after
matching their old hashes to untouched HEAD and reviewing this exact delta.
The separate23-file mapping test still rejects pre-existing Protocol/EepromLayout/
PcLedContract target drift; did not blindly refresh it. Existing PU route-audit
drift also remains. Full-project quality is not certified by focused audio passes.

GitHub tracking: [implementation and focused proof](https://github.com/atomicdeploy/PakarController/issues/6#issuecomment-6246432940)
and [final candidate identities and deployment hold](https://github.com/atomicdeploy/PakarController/issues/6#issuecomment-6246468378).
Issue #6 remains open for actual paired-board/phone acoustic acceptance and the
native/Web host-speaker consumers; no production checklist was closed.

## Actual PU850 script/CI import and shared cross-platform owner

Read this repository's AGENTS/preflight and existing build-storage, repository,
update and donor-audit routes before implementation. Verified PU850 local and
GitHub main revision431104072952f694a35b3a83c19f13fbb098ca5b, clean read-only
sparse checkout. Fully read28 actual script/workflow/config/test/documentation
files through EOF, including1114-line upload.js,568-line upload.test.js and
424-line C++ loopback suite. Exact original-byte SHA256, Git blob, bytes,
individual target/proof anchors, omissions and remaining actions are in the
existing canonical comparison ledger; no second source inventory was created.

Imported/adapted actual PU logger/sanitizer/exit categories, HashSet component
selection, Write-Step/Assert-Executable/Invoke-Native/scoped directory/elapsed
helpers, registry-index check/apply flow and staged artifact/deployment concepts.
Root Windows/Bash build/upgrade aliases and pakar.cmd/pakar.sh now share one
Node owner and existing reviewed PowerShell build/update policies. No default
clean, SDK/npm installation, beep, debug artifacts, host-config reset or flash.
See docs/SCRIPT-TOOLING.md for commands, activation/platform limits and concrete
reasons not to import incompatible ASA wire formats or duplicate language stacks.

Failures found and corrected, without bypassing reviewed guards:

- Current main CI39074428460 failed on a boot-audio source assertion that
  predated silent/reset-loop guards. Reread actual AVR boot/reset path; corrected
  the test and added negative removal checks for every required guard. No
  welcome/silent firmware policy was weakened. Latest GitHub run remains failed;
  this uncommitted repair is not a remote green build.
- Donor terminal sanitizer alternation left OSC title payload after broad ESC
  matching. Actual hostile-title fixture now verifies complete removal; color
  is also conditioned on the corresponding output stream, not stdout alone.
- Plan execution exposed string-array option binding in build-all; use typed
  hashtable splatting so Plan/Mode/Target reach the actual wrappers.
- Sparse donor files read from exact Git blobs were wrongly rejected by the
  filesystem-only gate. Added a distinct pinned-blob state, exact repository/
  revision/tree-object/raw-hash binding and regeneration tests. RequireLive still
  rejects absent files; unavailable snapshots are not current filesystem proof.
- Dependency comparison now handles numeric/two-component stable versions,
  prevents downgrade/concurrent overwrite, caches each index once in memory and
  honors configured HTTP/SOCKS proxies via bounded curl without logged credentials.
- Actual WSL Bash execution reached Windows node.exe with an incompatible
  /mnt/c absolute module path. Reused PU build.sh's scoped repository cwd and
  relative entrypoint; real Bash/Windows launchers and outside-cwd help fixtures
  now pass. Bash syntax alone was insufficient evidence.
- AVR boot packaging source test still assumed the old Bash compiler body.
  Verify the complete launcher/dispatcher/canonical packager chain instead, with
  negative cases breaking each hop; full32768-byte/vector/Urboot checks remain.

Focused Node/PowerShell tooling, child exit/directory handling, real subprocess
offline/invalid update plans, unchanged boot guard, storage dispatch, repository
governance, updater loopback multipart/fragmented WS and build-script security
checks pass. Actual actionlint passes; all three component plans and build-all
plans run without SDK compilers. Ledger fixture verifies wrong Git object/repo
reverts to missing. Selective PU ledger regeneration preserves the reviewed blob
rows without expanding donor checkout or losing other origins.

Broader source-quality reached the pre-existing PU route audit then failed:
WebUi.h actual4ca3a928335f176211f3756823372a93eb54080bec0711e2681f4d0753f2ee71
versus its route evidence45e734f5fa28c77b9844cbbc3aa50ee81f2fc65222cc5d418a2d8bac66f1ab16.
That file/evidence already had unrelated firmware/UI edits before this task.
Did not rehash it blindly or weaken the audit. Full local quality is NOT green;
the earlier targeted tooling/boot/storage/governance/loopback results stand.
Remaining checks are diagnostic continuation only, not a skipped-gate pass.

Live official registry check succeeded: current pins remain unchanged; FastLED
3.10.8 is available versus pinned3.10.6. It is reported, not installed/applied.
No dependency version or firmware/native/mobile version was changed by this slice.
Current canonical inventory4673, complete comparisons187, pending4486, known
parity-gap files147; PU46/109. All28 tooling reads remain honest scoped gap
records, not full uploader/distribution parity or whole-repository superiority.

New CI source adds OS-matrix fixtures/plans, centralized locked SDK install,
integrity/size summaries, guarded draft dependency PRs and disabled-by-default
exact-artifact deployment. Actions are pinned; no GitHub settings/runner/bot
activation, new workflow execution, SDK/app build, board contact or deployment
is claimed. Changes remain in the preserved dirty checkout based on2dc319fb;
do not sweep unrelated app/firmware work into a tooling commit. Watch/resume/
verified CLI backup/export, full donor case/native-package parity, actual OS CI
execution and physical acceptance remain open in #11/#2.
Source/fixture checkpoint recorded on GitHub:
https://github.com/atomicdeploy/PakarController/issues/11#issuecomment-6246254783
and master pointer:
https://github.com/atomicdeploy/PakarController/issues/2#issuecomment-6246257339 .
Remaining diagnostic checks (comparison negatives, versions0.2.7, donor routing,
storage dispatch/hygiene, crate boundary, README artwork and governance) passed.
This does not change the full-quality PU route audit failure above.

## Integrated Async/WS/OTA progress and automatic network update path

Reconsulted remote-ota, updates-telemetry and tm1637-display, then exact donor
audit rows. Read actual PU850 AsyncOTA.h/.cpp and FirmwareTransfer.h/.cpp through
EOF, canonical PCController docs/UPDATE-PROGRESS.md through EOF and the existing
Pakar AVR FrontPanel.h/.cpp. Original-byte SHA matches the recorded PU updater
and PC progress document. Reused measured stage counters/unknown progress and
explicit completion, not PU-specific UART callbacks or watchdog disabling.

Implemented exact optional upload-size/MD5 headers, short/overrun rejection,
coherent async-to-loop progress, read-only HTTP/WS status during OTA, boot-cached
info, finite/coalesced existing AVR scheduled-text frames and matching optional
LCD progress. Native Async HTTP and persisted auth-off behavior remain integrated;
the new network updater requires auth-off readback, defaults auto/local-first,
supports pinned Internet stage/separate apply and verifies new-boot full image
SHA/MD5/size/VALID slot and same-peer WS. Unknown writes are never replayed.
Replaced task-level work/remote-ota.ps1's obsolete fixed IP/factory-password call
with the guarded checkout wrapper; no SDK/dependency duplication or clean.

First compiler/actual RemoteFirmware fixture run caught the newly inserted
progress accessor outside its class. Inspected the actual class/ownership and
consulted donor measured-stage contract; moved it into the existing public
owner, not a global or a new updater. Corrected actual S3 updater fixture passes.
The first build also correctly rejected a stale literal ledger anchor after
otaRebootAt became atomic; reread/rebound its exact source anchor, not a gate
bypass. Full comparison generation/check remains authoritative.

Focused progress/unchanged AVR consumer, real loopback multipart/fragmented WS,
no-replay/auth/new-boot/SHA fixtures pass. Actual async HTTP publication/lifetime,
WebAccess auth/origin/epoch/persistence and S3 remote staging/tamper/abort fixtures
pass. These are host doubles/source evidence, not physical flash/display proof.
Current ASUS localhost:8022 refuses connections, ledboard.local is unresolved,
and local ports show only COM1. No board contact, auth change, AVR reset, ISP,
flash, audible/PBX call, deployment, speed benchmark or USB-removal acceptance
is claimed. Restore actual access before installing/verifying this candidate.
Office SSH and its MQTT service are reachable. A bounded read-only subscription
without stored operator credentials is correctly rejected; no usable current
board/boot observation was obtained. Kept the multi-tenant broker authentication
and ACL unchanged; broker reachability is not an ESP/cloud/OTA success. No remote
firmware or control command was published as a substitute for board evidence.

Candidate SDK build passed through tools/build-firmware.ps1, S3 buffered,
16MB/OPI PSRAM/RMT-DMA, ISP electrical build acknowledgement and W5500 enabled.
Version0.2.7; application1994971/3342336, globals99408/327680. Raw image1995120
bytes, SHA256c5c4b39067e33d37bd65f6726d5f22310414fa5bf04b9c701b00bf49dafaf047,
MD5 4b565333ed0f6538d44cc47591af2005. Source is the preserved dirty worktree
based on main2dc319fb8add1516a0f2caa4306bee1bcb1284ab, not a new pushed commit
or installed release. Native/mobile sources remain0.2.7; not rebuilt by this
firmware/tooling slice. Automatic updater plan reads this exact raw image and
performs no device contact. Comparison generation and -Check pass:4673files,
159hash-bound comparisons,4514pending,119known parity gaps. No all-donor claim.

Remaining: live authenticated-state disable/readback if needed, LAN/Internet
measured comparison, aborted/concurrent
OTA, external-power/AP recovery, physical displays and all-client durable update
outcome/UI parity. No all-donor completeness is claimed by this reviewed slice.

## TM1637 freshness and separate S3 LCD initialization/presentation

Read current canonical PCController SevenSegments.h/.cpp, I2cLcd.h/.cpp,
CompactI2c.h/.cpp and BoardPins.h. Kept PuzzleBoard's actual AVR A4/DIO,A5/CLK
mapping, not the donor's ISP-conflicting PB3/PB5 mapping. Existing physical-bus
host fixtures pass both generic and actual AVR register branches: pulled-up
ACK, every byte's NACK, replay after observed loss, off-state restoration,
brightness fade and rollover. No AVR firmware change/flash was needed here.
S3 JSON now exposes fresh tm1637_detection_known/nullable tm1637_detected;
legacy JSON/USB ACK also expires when AVR evidence is absent/stale. Actual
production freshness helper is exercised for HELLO/capability/status loss,
expiry and rollover. A cached ACK is no longer a current observation.

Imported/adapted real PCController common PCF8574/HD44780 transport and16x2
normalization/cache into BoardCore LcdDisplay. Dedicated S3 I2C1 GPIO15/SDA and
GPIO16/SCL avoid the incompatible AVR TM1637 connector and the other13 assigned
signals; UART/ISP/W5500/LED pins stay unchanged. Wire3.3.12 owns native
I2C at100kHz with5ms transaction timeout. Fixed donor unconditional initialized
success after ignored write errors: paced wake/control, checked writes and both
completed rows are required before available/lcd capability. Discovery is
bounded to0x27/0x3F; failed/observed-disconnected modules lose ready state and
reinitialize/replay without an MCU reset. Unobserved LCD-only power reset needs
explicit rescan; a backpack ACK cannot certify glass, contrast or geometry.

Standalone LCD shows hostname/cell/state and update phase. Protected/fenced
GET/POST /api/display/lcd exposes actual progress, finite ASCII presentation,
release/rescan and requested/presented revisions. A202 is admission only. Native,
Web and Flutter expose the last complete driver frame only after this display's
own ACK/initialization/geometry/row validation; absent displays stay hidden.
Full native/mobile/USB LCD editors, arbitrary I2C, alternate backpack/geometry
and the donor host-owned prompt/priority policy remain pending, not invented.

New bus fixtures cover every one of246 wake/init/frame write failures, bounded
scan, cache/padding, observed disconnect/recovery and rollover. One native
validation test, focused Flutter model/ACK test, Flutter analysis and actual
Web renderer/context tests pass. Full existing protocol/AVR/front-panel host
suite passes once at this completion boundary; none used a board or played
audio. LCD/CompactI2c four-source live-hash gate passes. Ledger now158 compared,
4515 pending,118 known parity gaps out of4673; no donor-wide parity claim.

Native release/staging SHA256
4510f06c0e15a8c133de956e89fbee6e0a1b579dedc71a79569fd0dd847da5bf;
Android preview APK SHA256
a7db816cf1e3d796a4c7c19a30c07594250b27929be470f182dd6b2205d70d5a.
Versions remain0.2.7. The new Web label correctly triggered stale generated
localization; the existing shared Vazirmatn/translation generator was rerun
and its no-write check passes. Flutter's new unbraced-if lint was corrected
and the changed Dart files formatted; no guard was bypassed.

Native optional LCD deserialization now discards only invalid display metadata,
not the valid parent board status; the focused native test covers this and the
final release was rebuilt/staged. Actual production LCD HTTP handlers also pass
bounds/auth/maintenance, admission-vs-frame, release/rescan and absent-module
fixtures, rather than just a separate validator. S3 SDK3.3.12 candidate build
passes with W5500,buffered UART,16MB/OPI and RMT-DMA:1991423 sketch bytes and
99392 globals. Application1991568 bytes, SHA256
5421f6d15b10574dc90ff22503b936719bc1c2e8d40e1eb53fd1f7d8fd84d055;
image contains the actual LCD endpoint and new TM1637 freshness fields.
This is compilation, not actual I2C/Ethernet/DMA acceptance.
Fresh ASUS SSH8022 still refuses. No board deployment, reset/write, actual I2C/
TM1637 transaction, optical test, audible cue or PBX occurred. Confirm the LCD
backpack/voltage/pull-ups (5V bus requires a bidirectional level shifter), deploy
the candidate, then test both displays with actual AVR UART and concurrent
LED/Ethernet traffic. No live insertion or production acceptance is checked.

## Optional W5500: shared native network candidate, physical tests pending

Reread complete canonical PCController netpolicy/http_url.go and PU850
AsyncWebServer.h/.cpp, AsyncOTA.h/.cpp and FirmwareTransfer.h/.cpp. Added a live
hash-checked seven-source Ethernet gate to the feature-enabled build and CI.
Three newly reviewed headers/implementation rows and four expanded existing
mappings preserve the shared HTTP/WebSocket/OTA contract, explain unsafe donor
buffer mutation/direct callback and PU globals exclusions, and distinguish native
SDK Ethernet additions from donor functionality. The durable inventory now has
154 hash-bound comparisons,4519 pending and114 known parity gaps out of4673.
Seven current Ethernet donors match live bytes; this does not close those gaps.

The explicit native ETH/lwIP option uses W5500 private HSPI/SPI3 pins
SCK5/MISO6/MOSI7/CS8/RST9/IRQ14. AVR FSPI/SPI2, UART TX17/RX18 and LED4 are
unchanged. No WIZnet socket library, parallel HTTP server or separate updater is
introduced. Installed AsyncTCP3.5.0 binds its existing server to INADDR_ANY.
The same routes/auth, NTP, cloud and firmware fetch use the common native stack;
Ethernet takes preference only while IPv4-ready. Independent persisted
DHCP/static settings and route-change cloud invalidation remain in place.

Fixed the absent-module recovery gap: explicit Ethernet settings apply retries
initialization, including safe partial-driver cleanup; cable loss never resets
a healthy driver and service never retries automatically. Cheap status now
publishes initialized/started/carrier/IPv4/DNS/MAC/error, with null disconnected
addresses rather than stale IPs or synthetic connectivity. Native connection
method now says Network for Wi-Fi/Ethernet, preserving exclusive USB ownership;
status can show the acknowledged Ethernet uplink instead of an old station SSID.
Dedicated graphical DHCP/static Ethernet editors remain open.

Focused port lifecycle (both flags), NetworkPath (both flags), config persistence,
typed management/status (both flags), discovery, pin-map and live-probe fixture
tests pass. Probe tests include actual loopback TCP/redirect rejection, but no
ESP or WAN. One focused native Ethernet-label test and connection wiring guards
pass. The probe checks the actual wired TCP address, Web/JSON/XML and firmware
HEAD/Range; optional one-shot pinned HTTPS ESP staging requires Wi-Fi station
disconnected and never applies/cancels firmware or touches AVR. Lost results,
changed boot/peer/job/hash or existing maintenance stop without mutation replay.

Actual Arduino-ESP32 3.3.12 buffered16MB/OPI/RMT-DMA/W5500 SDK build passes:
1961319 sketch bytes,97720 global bytes. Application binary1961472 bytes,
SHA256 7e197801d4b05bf690312f272ea073e9e2da176c08cab4be636a875a0b095225.
The first attempt linked a stale copied EthernetPort.cpp after its source was
changed during Arduino's initial library build; the undefined symbols were
diagnosed in the cached object. Final incremental recopy/build resolved both
methods without a clean, SDK patch or duplicate cache. No library rebuild was
forced for the final source-only pass.

Native release passes and its verified staging candidate is26164224 bytes,
SHA256 36cbe1e44e3bc7ea90452fdba084f73055764659b616228c78baf7e23fbc0430.
All app/firmware versions remain0.2.7. Existing AVR and Android candidates are
unchanged. Build/staging identities do not establish an installed runtime.

Fresh ASUS SSH8022 refuses connections; this PC's ledboard.local lookup did not
resolve within3s. Module supply labels still need confirmation (3V3 vs carrier
VCC/VIN requirement), not a board photo. No firmware deployment, live Ethernet,
OTA/WAN fetch, AVR reset/write, tone or PBX action occurred. Physical DHCP/static,
HTTP/WebSocket, wired multicast discovery, Ethernet-only fetch/apply/reboot,
cable/fallback/AP recovery and LED traffic soak remain unchecked in issue2.
See ESP32-S3-COMMISSIONING.md for the exact pinout and safe acceptance probe.

## Board Silent preference: persistent owner and all three Preferences surfaces

Read the complete current PCController LocalLib/TonePlayer.h/.cpp,
Project/SettingsStore.h/.cpp and Tools/Controller/internal/appconfig/buzzer_runtime.go,
plus Pealayer src/ui/board_info.rs. Their original hashes are recorded in the same
comparison ledger. The new board-silent gate verifies all six against live donor
bytes; generic remaining donor controls/backend/routing work stays pending.

Adapted the existing MCU-owned flags bit0, independent of host notification sound.
Native Preferences > Board and Flutter/Web Preferences > Board sound are contextual
on AVR settings+buzzer ACKs. Native uses the selected fenced HTTP or revocable USB
session only (USB never falls back to Wi-Fi); all surfaces use the existing inner
GET_SETTINGS/SET_SETTINGS/GET_SETTINGS contract. Readback must match the complete
preserved record and report persisted=true. Unknown is not Off; pending/expired,
rejected and stale-peer results cannot optimistically update the toggle. No
uncertain mutation is automatically replayed or saved as an app preference.

AVR mute now publishes its revision immediately, including between notes, and
unmute follows donor behavior without restarting an interrupted note. Prog safety
stays independent. Actual production tone/register fixtures cover all fourteen
muted cues; actual settings/runtime EEPROM fixtures cover reboot persistence,
untouched flags and rejection while programming. Four focused native tests and
four Flutter model/API/widget tests pass, as do Flutter analysis and the focused
Web transaction, auth/Preferences, appearance, peer HTTP and localization tests.
Corrected stale test doubles: Web listeners must compose instead of overwriting;
appearance owns its real locale/haptics dependencies; old per-cell batch source
assertions now check the implemented atomic request/readback instead.

Native release candidate staged (not production replacement):
SHA256 df110c19c3eafdb9d594f5f0f1709080bf4c5f7e87e8f72d872f586d61748d10.
S3 buffered16MB/OPI/RMT-DMA build passes, application binary1863072 bytes,
SHA256 1254f86ef970c04dd6fdfa74fed8a777404b9bcded4b722788d5be1473eba8a3.
AVR buffered build passes at31602 application bytes and1460 static SRAM bytes;
verified Urboot merged32768-byte image SHA256
3fa829d2104295d92a24194f5ce85b5f38647240b5bc7d6a60de8dfade9d53af.
Native/firmware/mobile versions remain0.2.7.
Android preview APK build passes; SHA256
34a231b50c64bc013b04ad9e66c09d0b99779ab82c7fc4006bf46d069ee12672.
These are build identities, not installed firmware/runtime receipts.
Fresh ASUS SSH8022 refuses connections;
no board contact, flash, reset, audible test or PBX call was made. Physical
mute/unmute, welcome/menu/cue silence after power cycle and actual USB/Wi-Fi
deployment remain open. No hardware acceptance checkbox was checked.

## AVR ISP/UART restoration: boot-chain defect corrected, hardware unavailable

User explicitly requested live ISP/UART recovery, brownout and bootloader repair.
Fresh SSH8022 refuses; bounded TCP checks on the current8022 and historical
7022/1022 tunnels are unavailable. This PC enumerates only ACPI COM1, not an
ESP/AVR USB endpoint. Requested restored ASUS access; no different Cafe/PCController
board was substituted. No actual AVR reset, signature/fuse read, backup, chip
erase, programming, UART exchange or audible operation occurred.

Ran canonical AVR reset tests and the full test-protocol.ps1 suite successfully,
including actual production ISP/EEPROM-preservation code against its SPI emulator,
all three UART flow profiles, Prog/settings contracts, TM1637 and melody timing
fixtures. Buffered AVR compile passes at31610 application bytes and1460 static
SRAM bytes, leaving588 static headroom (not a runtime stack/soak measurement).
Inspected current .init3 output: capture MCUSR/r2, clear MCUSR, WDT disable and
fall-through at0x17E to data initialization. Host tests do not prove live health.

Found a real packaging defect: the old 32768-byte merged ISP image contained
stock vector Urboot but reset still jumped directly to the application at0x14E.
Before correction SHA25648ec5cb57429f18641d698a2d8e6d7029b0285e304234ee3b302de3374a3b3ac.
Read PCController Tools/Bootloader/Urboot-Custom/build.mjs and README.md completely,
including reset/trampoline, metadata, unused-vector ELF proof and round-trip code.
Imported/adapted those actual mechanisms into tools/prepare-avr-image.mjs and
both canonical PowerShell/shell builds. The packager pins the installed stock
UART0/384-byte HEX hash, verifies __bad_interrupt ownership before reserving
vector25, normalizes erased gaps and checks every decoded output byte.

One packaging check correctly rejected the stock Intel HEX type03 start-segment
record. The donor parser ignores non-data metadata; the adaptation now explicitly
validates its shape, bounded/aligned entry and agreement with Urboot's metadata,
with a regression, rather than silently ignoring arbitrary records. No flash or
fuse write was involved in that failure or repair. The first live-donor gate
rejected work/repos/PCController inventory drift (old2a0e971e/1207 tracked paths).
Diagnosed, then used the actual canonical installed PCController source at
b0111ae0/1517 paths, matching the inventory; avr-boot-chain -Gate -RequireLive
passes there. Both donor file hashes match the recorded source bytes.

The final candidate's reset is wrapped RJMP0xCF3F to0x7E80; metadata-selected
vector25 at0x64 is an absolute jump to the original application at0x14E. Urboot
bytes are unchanged, full flash is32768 bytes, and HEX round-trip/negative-vector,
overlap, checksum, EOF and start-record tests pass. The canonical build passes;
build storage/gate checks pass. Corrected binary SHA256:
5e836361ff97119a96e54f719fcb4443f64840abb85274e04cf905ec1a050140.
HEX SHA2564e2889f4dba489b0be2ab4a718a01a1e6a81480203e313834b124a012ee2dfa5.
build/avr-buffered/avr-image.json carries exact source/profile/hash proof and
physical_validation=false. Firmware/application versions are unchanged.

No fuses were written or guessed. The build still describes external16MHz/BOD2.7V;
the reported3.3V rail and actual speed grade/clock/fuses must be reconciled before
deployment. BOD disabling is not a repair for inadequate voltage/clock margin.
The existing ISP API deliberately does not expose arbitrary fuse writes. Any
required constrained fuse recovery needs the actual readback and verified backup
first; current hardware BOD/bootloader/UART are NOT accepted. Relevant primary
references: installed MiniCore3.1.3 boards.txt and stock Urboot README, and
Microchip ATmega328/P datasheet DS40002061.

Two complete donor reviews added with explicit remaining custom-progress work:
the donor512-byte TM1637 D13/D11 backend must not be deployed on A5/A4 hardware.
Stock Urboot remains selected. Original MIT and upstream GPL notices retained.
Ledger149 compared,4524 pending,109 parity gaps; not full donor parity. Production
ESP/app images, rollback, settings, unknown edits and donor trees remain untouched.

## Configured hostname and contextual authentication repair

Read the installed Arduino-ESP32 3.3.12 NetworkManager/WiFi startup implementation:
the global default is generated as chip target plus MAC suffix, and mode() copies
it into the new station netif. Pakar applied setHostname after mode(), leaving
DHCP with the SDK name. Reviewed ASA0002E WiFi_Setup and onValueUpdate HostName
slices: saved identity is applied before enabling interfaces and reused by
discovery. Shared NetworkConfig now restores that order, applies/checks the
full saved label on station and AP netifs, and stops interfaces on a hostname
application failure. It retains DeviceIdentity as authority, without suffixes.
This is a slice review, not certification of the entire PU850 sketch.

Read LocalLib/Authentication.cpp and .h completely; live hashes match the
inventoried PU850 snapshot. Both now have exact file-comparison decisions and
omission reasons. Replaced hardcoded protection/password-token dumps and the
incomplete role skeleton with the actual middleware, bounded sessions and
atomic access persistence. Ledger: 147 compared, 4526 pending, 107 parity gaps;
full donor/production acceptance is not claimed.

Authentication has a real persisted enabled flag, off on fresh installations
and migrated old factory-credential records. Legacy custom credentials retain
protection; users can explicitly disable it. New enable/change/disable commits
verify staged readback before rename. An enabled configuration requires its
current password for changes. Corrupt or unmounted access storage remains
unavailable, not an anonymous fallback. HTTP/WS/Telnet use the same flag; origin,
peer/boot fencing, WS ownership and firmware-busy guards remain in place.

Web login and Sign out are hidden when disabled, including /basic. A network
failure offers retry instead of inventing a login requirement. Direction,
language and supported-only haptics are in browser Preferences with Apply/Cancel.
Native credential fields require explicit enabled metadata and are absent for
USB. Exact-endpoint public requirement metadata survives Disconnect only for
offline credential editing; it never grants device control. CLI/TUI can use
auth disable. The native implicit factory-password fallback was removed.

Executed focused checks: shared network model for ESP8266 and ESP32; actual
WebAccess plus access-record persistence/legacy migration and unavailable-storage
doubles; embedded Web auth-on/off, lost-connection and Preferences open/cancel;
basic controller; localization and nine donor locale cases; S3 platform guards;
three native connection-control tests and two access tests; built CLI access and
hostname mutation/acknowledgement tests. No full CI/suite wait. The locale fixture
was updated from obsolete per-cell progress wording to the actual current atomic
batch readback message, retaining the exact Persian assertion.

Native Check and Release passed using the canonical warm release cache. Staging
PakarController.exe is checksum-verified and CLI-probed, SHA256:
bac4c656aa1633c3464c93656388dbe8620d2b691a1c203c482d2e945a632361.
Production executable, rollback and settings were not replaced.
S3 firmware compilation passed with the existing buffered UART, 16 MB flash,
OPI PSRAM, RMT-DMA and ISP-electrical-verified build profile, SDK3.3.12; no
W5500 flag. Image1,856,096 bytes, SHA256:
d3d144ce7998c655d632d12fd84c143ef2d873914e8f08d731d774207675d047.
Compiler reports1,855,947 bytes program and97,340 bytes static RAM. A binary
content probe confirms the final Preferences handler and checked-hostname
failure path are actually embedded; it is not an older cached image. This
candidate remains in build/esp32s3-buffered, not a flashed/accepted image.

Source is base 2dc319fb8add1516a0f2caa4306bee1bcb1284ab plus preserved working-tree
edits, not a clean-head release. Fresh localhost:8022 SSH and direct TCP checks
refuse; ledboard.local does not resolve within the bounded lookup. No ASUS
install, firmware deployment, live DHCP/mDNS/AP/login test, AVR, audio or PBX
action is claimed. Issue2 hardware and human acceptance remain open.

## W5500 separate-bus wiring preparation

Consulted network-cloud and literal PU850 donor audit before selecting pins.
Fully reread actual PU850 NetDiscovery.h/cpp: unused configuration and blocking
Wi-Fi-only prototype loops are not an Ethernet driver and were not copied.
Existing broader discovery/import gaps remain pending, not silently excluded.
Inspected the current AvrIsp enter/leave SPI lifetime: its global SPI.end()
cannot own Ethernet's SPI object. Reserved W5500 SCK5/MISO6/MOSI7/CS8/RSTn9/INTn14
on independent SPI3/HSPI, retaining AVR SPI2/FSPI and TX17/RX18 and LED4.
All thirteen signals are unique; source guards cover documentation and defaults.

Fully read installed Arduino-ESP32 3.3.12 libraries/SPI/src/SPI.h,
libraries/Ethernet/src/ETH.h and the ETH_W5500_Arduino_SPI example. SHA256:
SPI.h 4fdfa675a719087981d67cfa8d6c1719e17386121dd859b28114cca459f892df;
ETH.h 5c8fe676e61ffe33be8363d20d23c5bf242c11f1b85e2a2a91d100177dd79904;
example ac085528784f6187507d1740809254f62adf55a56b7e27f303744841d81d3628.
The selected future integration is core ETH/lwIP, not a parallel Arduino
Ethernet socket stack. No driver/example code was imported or enabled here.
Inspected only the bus-definition excerpt of esp32-hal-spi.h, not its full file.

Sent the explicit pin plan to Merge PCController PRs for Pealayer, then read its
completed review: it agrees, conditional on exact DevKit GPIO exposure and
module supply confirmation. Official WIZnet electrical/reset specifications
were checked. Carrier power input, integrated S3 Ethernet/ISP, discovery,
OTA/network continuity, and physical tests remain open. Pin source guards and
diff checks pass; no SDK build, dependency installation or powered test occurred.

## S3 UART wiring agreement

Read the referenced Merge PCController PRs for Pealayer chat, including its
actual ISP/UART reply, rather than inferring from its title. ISP pins agree:
MOSI11, MISO13, SCK12, RESET-control10 through a voltage-safe external driver.
Its conventional TX17/RX18 differed from Pakar's provisional RX17/TX18 defaults.
Both are routable, but mixed code/wiring would fail communication. Standardized
Pakar's defaults and migration table to TX17 -> AVR D0/RXD and AVR D1/TXD -> RX18.
The old chat answer/table is superseded; no AVR peripheral assignments changed.

Consulted uart-flow-control and literal donor-audit; fully reread actual current
PCController Project/UartProtocol.h, SHA287a95a65a9b9655e64a15ece051bc792a06564b648a1b53eed4f373c564e458,
and actual Pakar BoardPins/AvrUart. Existing frame/hardware UART behavior is
unchanged. Official Arduino-ESP32 begin() documentation confirms RX-before-TX
argument order. Added a lightweight S3 source-wiring guard to existing CI; changed
CI target proof hash after rereviewing the added command and unchanged fail-closed
runner. Existing donor omissions and integrated S3/hardware gates remain open.

Source guards and diff checks pass. No dependency installation, unrelated app/
firmware build, flash, fuse/EEPROM write, reset or electrical test. This is pin
configuration/document agreement, not a working S3 programmer/deployment claim.

## Android peripheral mirroring

Started from clean a6dec77; refreshed owning issue #2 and open PR state (none).
Prior Build38275253969 completed with Android/Windows/quality success but the
firmware setup failed three times on the ESP8266 python3 tool HTTP500; CodeQL
passed. That is not all-green CI or evidence of a firmware source failure.

Consulted firmware-hardware and literal donor-audit for PCController
buzzer_mirror.go, buzzer_mirror_test.go and FrontPanelModel.h. Fully read current
Go scheduler/tests through EOF at704f44042e6bf46c4459bd63f37230460f221389, exact
original SHA2a6d0fd4.../83b8726d...; unrelated dirty donor files preserved.
Adapted actual cadence, wrap, jitter cushion, absolute expiry, Stop/reconnect
flush, opt-in and independent failure transitions into Android PCM mirroring.
FrontPanelModel was read, but motion/RF/sensor menus are not fabricated into this
hardware profile; broader menu/action imports remain explicitly pending.

Added AVR musical-start/observed clocks, explicit HELLO timestamp acknowledgment
and actual front-panel state. ESP's existing raw event projection now binds
board/boot/AVR-session/event ID; Android events cannot authorize identity or
renew stale status. Hardware UART, A4/A5 and busy/poll/buffered profiles remain.
Added a mobile-mirroring donor gate and exact source/target/test hashes to the
existing ledger, plus refreshed original PC contract target hashes/line anchors
and PU route target hashes. No disconnected replacement ledger was created.

Recovered expected proof-hash failures by rereviewing only changed shipping
targets, not bumping unread donor hashes. Pealayer inventory moved564->567 during
this slice; ten changed previously reviewed files retain their immutable prior
source snapshots with current live hashes and explicit reread actions. Later
repository-wide preflight encountered further concurrent tracked-path drift;
local whole quality is not declared passed or bypassed. A patch-generation
helper hit the default1MiB child-output buffer and unordered hunks; selected
read-only rows were bounded and sorted before apply_patch, without shell writes
or donor mutations. The donor-first tooling remains mandatory.

The first UI test's short viewport lazily omitted the display; an explicit tall
fixture verifies independent mirrors. A real socket test was separated from
widget-binding's intentional HTTP400 override, not patched into production.
The first tablet capture rendered correctly but left a test lease active at
invariant checking; explicit idempotent disposal fixes the harness. Existing
capture evidence was inspected and preserved, never overwritten.

Final guarded analyze and86/86 mobile tests pass, including real authenticated
loopback WebSocket routing and inspected phone capture. C++ production protocol
suite passes; AVR busy-wire/polling/buffered and integrated ESP8266 busy-wire
builds pass. AVR RAM1327/2048 (721stack bytes available), busy-wire flash28062/
32384; ESP RAM56700/80192, IRAM63055/65536, flash897192/1048576. Clock/register/
transport doubles are not hardware waveform, memory-margin or audible proof.

Final preview APK55396153bytes, SHA256
74c10aabf1e6c4735c403969a1cf730346871090f6bd138bea1ab242dbcca011,
copied to the stable delivered APK and hash-verified. Debug signing remains a
preview limitation. Tablet screenshot86063bytes/SHA97d37cedd9c6b7d2f9843b04660d5c41b3e831383351bd9a93d2c823f735e4f6;
phone86459bytes/SHAfb57825d3b2df5ea7ef46d898d62709285368811417212a627c375ca9b15c279.
Both are explicit simulated peers with silent output, not live device evidence.
ADB freshly lists no device. No firmware, fuses, EEPROM or remote buzzer writes.
S3 integration, phone installation/audible timing/focus tests and native/Web
audio consumers/optional bridge note forwarding remain open in issue #2.

## Web streaming pacing

Started from clean 5b54e4d; Build38154906733 and CodeQL38154906685 passed.
No open Pakar PR or new completed Copilot review. ASUS1022 refused and
ledboard.local did not resolve. Preserved live native process/settings/rollback.
Consulted all 77 games-streaming rows and exact donor-audit rows; the two
PCController status-frame-batcher files were unrouted and not yet compared.
Read both actual sources completely; original byte hashes match the inventory.
Adapted actual clock/generation/cancellation/disposal code and fake-clock test
scenarios, not an assertion of whole status/metrics parity. Added specialist
web-streaming routing/gate to quality and ESP builds. Useful generic status
history and error-reducer omissions remain pending in the authoritative ledger.

Replaced fixed 40ms Web timer with 60/default, 0/uncapped pacing and bounded
latest-frame socket buffering. Focused tests run only after this substantial
source slice, including real embedded Web start/reboot revocation. These are
fixtures, not live board throughput. No native/mobile/AVR rebuild or deployment.

Quality passed. Final ESP8266 busy-wire compile passed in the same warm cache,
and exact module bytes were found in its945360-byte image. No hardware flash.
Live PCController inventory gate caught25 added/14 removed tracked paths:
selectively refreshed only PCController, then strict comparison and RequireLive
scheduler gate passed. New board-mirroring paths remain unreviewed; current owner
dirty work was hashed, never edited or called committed source. Totals4282/106
manual/4176 pending/73 gaps. English/Persian live stream receipts are fixture-tested.
Production executable and preferences hashes are unchanged. Initial GUI PID58400
was absent at final check, with no Pakar process found; this turn did not terminate,
replace or launch it. Do not present preserved executable bytes as a running GUI.

## Web board/boot adoption

Started from clean b3e9d19 with Build38150481947 and CodeQL38150481964 green;
no open Pakar PR or new completed Copilot review. ASUS1022 remains refused and
ledboard.local unresolved. Current GUI/settings/rollback are preserved.
Consulted web-presentation, exact RayanLamp api.ts and peer-binding ownership.
api.ts and api.test.ts were reread through EOF; live source hashes match inventory.
Adapted actual APIError, Headers, same-origin/no-store fetch, AbortController
deadline and finally cleanup into assets/web/peer-http.js. No donor 401 mutation
replay or nonexistent Go session endpoint. Existing PCController session.go's
before/after admission contract now also applies to Web; no new serial parity
claim. Mechanical embedding uses the existing generator and byte-equality test.

Focused verification caught empty typed codes masking distinct HTTP failures,
a bootstrap/sign-in ownership race, and external abort retaining its timer.
Reconsulted the already fully read api.ts request/finally and sync.ts stop paths;
fixed the actual adapter and explicit auth generation, not test suppression.
DOM fixtures were upgraded with Headers/AbortController and explicit simulated
board/boot identities; old fixtures were not physical/capability evidence.
False batch ACK, shared transaction pins, old-peer draft clearing and no replay
are covered. Final full quality and ESP8266 busy-wire build passed. A final
redirect-error refinement was compiled again in the same cache; final image
and exact embedded module proof are in FILE-COMPARISON-RECEIPTS.md. No flag
churn or debug cache. The nonunique PU route anchor rebind was rejected;
reread both actual route bindings and retained their verified line244 rather
than choosing an arbitrary occurrence. Only target/proof leaves were rebound.

Known gap newly exposed: Web streamTick still uses a 40 ms interval (25 FPS).
Configurable/backpressured pacing remains open; native60/0 policy is not Web
parity. GUI/Flutter/other transports, AVR-only reboot, ESP32-S3 integration/DMA,
readback durability and physical multi-client tests remain open. No debug/native
or Android build, cleanup, GUI replacement or hardware write is authorized by
these fixture results.

## HTTP board/boot fence continuation

Started from clean pushed d450866; Build38147533498 and CodeQL38147533441
both succeeded, no open Pakar PR or new Copilot review was observed. ASUS
localhost1022 refused and ledboard.local did not resolve; no physical write.

Consulted donor-audit/device-context; session.go lookup caught PC inventory
drift. Selectively refreshed PCController (1234 paths); did not promote new
files into reviewed decisions. Completely read actual internal/link/session.go
and session_test.go, with original hashes in the comparison ledger. Consulted
firmware_identity.go/test too, but their immutable build stamp is not a boot
epoch: no import/read certificate is invented for that unused investigation.

Adapted before/after-admission cancellation and matching-identity tests. Added
real optional HTTP epoch headers and firmware rejection before dispatch and
both pre-middleware upload writes. Missing/invalid identity keeps TUI read-only.
Generic useful serial acquisition/quarantine/ACK/reset/event tests remain open,
with exact code and reasons; Go HELLO schema is not copied as API negotiation.

A broad cargo fmt invocation accidentally formatted unrelated native files.
Restored ONLY this turn's formatting via reverse diffs/apply_patch (initial
native checkout was clean), retained intended changes, then formatted the
terminal module alone with skip_children. No checkout/reset/user edit removal.
First evidence patch failed atomically on final-property comma formatting;
used exact existing lines, without changing original donor hashes. Quality
gates then caught the separate PU850 route target hashes/line anchors. Read
affected upload/WS/SSDP/info/login consumers, checked every mapped target,
rebound exactly six route references. No assertions deleted/bypassed.

151 native tests pass, including same-host reboot/replacement, draft revocation,
invalid/missing fence read-only, wire headers in cell/shared CLI paths and HTTP409
stop/no replay. Production C++ validation tests and full quality phase pass.
First ESP8266 busy-wire compile passed; SDK macAddress ignores its SDK return,
so the MAC buffer was explicitly zero-initialized to fail closed on no output.
Final compile/package results are recorded in FILE-COMPARISON-RECEIPTS.md.

Current inventory4271, complete103, pending4168, parity-gap70. Two new fully read
PC files are scoped parity gaps, not full transport parity. Existing GUI owner,
settings and rollback remain untouched. No AVR/APK/donor rebuild or new cache.

## Minimized native command processing

Prior turn was progress: clean0c3eca9 source was pushed, release/stage/probe
passed and deployed process54616 matched candidate SHA B3FAAC2569B18B94EB772791ADE5083F1A4583F8FA0517DE821229ED098901C9.
Saved preferences SHA remained 5C6133A73641C8087A5D00B0CC978FA03CD2829C6D81F3959DA1F63F50F974DD;
existing rollback files were preserved. Both current-source CI workflows passed.
No physical board acceptance: ASUS SSH1022 refused and ledboard.local DNS failed.

Live capture failed on a minimized owner, exposing accepted-but-undrained IPC.
Consulted capture and actual donor read-only scripts; Win32 correctly refused
a non-visible/non-minimized target, not a blanket screenshot capability loss.
Consulted native-lifecycle/donor-audit and actual Pealayer App::logic and shell
command slices. Full app.rs is not certified. Adapted non-render command,
device-result, notification and snapshot placement; source guard checks no
duplicate drain in UI. SourceStatus still needs a read-only rendering snapshot.
Initial compile failed on that missing local and ctx.style (removed in locked
egui); corrected with the rendering snapshot and donor's ctx.theme comparison.
Locked egui run_logic read and used for the minimized capture-deadline test.
No removed assertion, alternate cache, dependency upgrade or unrelated build.

Capture wrapper also had an idle-snapshot race. Added request_id/path correlation
and bounded older-owner pending/result handling. Its gate refused a changed
capture target before use; rereviewed the wrapper and rebound only exact target
and source-guard hashes, preserving all original donor hashes.
An attempted unexpanded registry glob and a guessed test/tool filename failed;
used rg --files and exact existing paths, not a blind replacement or unavailable
claim. Second production Preferences capture timed out; no output file exists.
Actual earlier framebuffer was inspected after supported window restoration.
Physical Escape stopped Windows Computer Use; desktop input was not resumed.
Open Preferences/draft preservation blocks replacement only, not source work.

Four additional full Pealayer conversations70/78/87/102 read and hash-bound.
They constrain private branding/resource ownership, transition/config revisions,
hidden-window lifecycle/draft safety and shared portaled button geometry.
Fresh source corpus counts remain separate from full semantic review.
The guarded native Test completed:129 passed/0failed/0ignored, including
the actual locked-egui run_logic minimized deadline fixture. Later user reply
confirmed Preferences closed; IPC unavailable plus zero enumerated owners
confirmed terminal state, not an observation timeout. Product --minimize and
dirty/minimized status add a repeatable no-desktop-input live acceptance path.
An overlapping adjacent-line evidence patch failed before mutation; grouped
only consecutive changed references and preserved donor hashes. No test/gate
bypass. A final native compile/test of the added control is still separate.
That final guarded Test now passed129/0/0 (2m40 compile,0.17s execution),
including strict typed minimize parsing. Source guards and exact comparison
generation checks pass. Full production live lifecycle smoke is not inferred
from these tests. Current0c source CI was confirmed success without waiting.

48e0165 release completed4m37 and stage/help/version probes passed. Installed
process72012 matched SHA597999E33688A5B0ACCECE9CDA5B8ECFBC2EDC5DCCEC87E822A4A6F4C19216BB;
settings and five existing rollback executables were unchanged. Live CLI smoke
then failed: screenshot request while observed minimized saved96x96 cropped UI,
not a usable workspace image. Inspected and renamed only that owned5474-byte
failure PNG to desktop-minimized-invalid.png; not accepted screenshot evidence.
Show subsequently applied and observed minimized=false; no desktop input.
Read locked egui ViewportInfo::visible, egui-winit viewport sampling and eframe
Glow/epi logic/paint paths. Explicitly suppress screenshot requests/receives
while invisible and reject below320x240 root results, retaining low-level2x2 PNG
encoder tests. Added actual egui minimized-paint/stale-event regression instead
of weakening the live timeout/no-file assertion. Close-to-tray cancellation also
moved into non-rendering logic so hidden close handling does not require UI.

48 CI CodeQL succeeded; build failed its native wiring assertion in the Web
context test: it required the obsolete &ctx spelling/location after moving to
native_logic(ctx). Full Web no-ACK/offline/mode/stale tests are retained; replace
only that assertion with stricter non-render placement and no duplicate UI call.
Comparison generation independently rejected the earlier post-check GitHub-doc
append's stale target hash. Consulted actual REVIEW-COMMENT-AUDIT plus current
GitHub doc and rebound only that target, keeping donor original bytes unchanged.
Do not claim the48 build all-green. Source/fixture/native/live gates remain
distinct and the updated native/live tests must execute before replacement.
Final guarded Test of the visibility/stale-frame fix:130 passed/0failed/0ignored
(2m51 compile,0.16s execution). Quality first stopped on a new unsupported
partial-read label in the old screening ledger. Read its actual validator;
used existing screened status with explicit partial scope rather than widening
resolved statuses or certifying unread source. Regenerated only Pealayer and
resumed the quality phase from its exact failed child; every preceding child
had passed and every remaining child now passed. Full CI will rerun the phase.
The strict production completeness gate still rejects unresolved donor files.

## Actual TM1637 bus recovery acceptance

Reread complete live PCController LocalLib/SevenSegments.h and .cpp; both match
their original snapshot hashes. Corrected the failed guessed Firmware/LocalLib
lookup using the actual ledger path LocalLib/SevenSegments.cpp and explicit
canonical PCController root. The default lookup only configures Pealayer/Rayan
roots; unavailable in that lookup did not mean the real PCController was absent.
Read Tools/VirtualBoard/tests/firmware_controls_test.cpp through EOF and its
Arduino mock dependency. Actual donor require/fade/ACK tests are adapted, not
replaced by an unrelated presence probe. Original donor source stays unchanged.

Found DIO released with pull-up disabled, permitting an unplugged floating line
to resemble LOW ACK. Release DDRC before enabling weak PORTC pull-up; fallback
uses INPUT_PULLUP. Correct PC4/PC5 pins, ISP isolation, frame/brightness cache,
zero/off and NACK aggregation remain. Titan/Microchip timing and DDR/PORT docs
are linked in TM1637-RECOVERY.md. No global pull-up, EEPROM, pin or clock edits.

New byte-decoding GPIO and instrumented-register fixtures execute both actual
branches: input/pull-up on every ninth slot, no ACK contention, other PORTC bits
unchanged, exact mode/address/segments/control replay, every NACK slot, true-off
recovery, fade and 32-bit cadence rollover pass. First AVR fixture compile caught
a singleton string splat; consulted actual Pealayer run-windows.ps1 typed-array
runner and changed only the new test flag vector. First comparison rejected an
invalid omission enum; used the existing not-applicable disposition with exact
unfitted-hardware reasons. No guard weakened or failure hidden.

New reviewed-slice gate is required by firmware build/test wrappers. Source
comparison generation, reuse preflight regressions and source-ledger integrity
pass. Warm AVR builds pass for busy-wire (26898/32384 flash) and polling
(26614/32384 flash); both use1225/2048 RAM. Busy-wire merged Urboot image SHA256
8d20b942f0c7157170c5428e6f4ac19cbeb395128d330d10bca3030a9f1d5c08.
Both source layouts remain0.2.7, not a new public API generation. Existing native
cache measured4013127900 bytes, release preserved; no debug cache or unrelated
ESP/Android rebuild. No display flash/unplug/replug: fresh localhost1022 SSH
refuses and ledboard.local DNS fails. ACK still is not actual emitted-pixel proof,
and a shorted-low wire is not diagnosed by these doubles.

Same ledger now88/4242 compared,4154 pending,55 known gap files; generic donor
key/transition/menu/tone test import remains pending. No production-complete claim.

## PR completion and exhaustive conversation collection

PR14 and PR15 merged normally; only main remains on GitHub and locally. Read all
their current diff/conversation/review-thread pages. Copilot requests are posted;
no completed review is claimed. PR15's stale workflow target hashes were rebound
only after rereading complete actual Pealayer CI/CD and CodeQL and Pakar workflows.
Repository governance and comparison Check passed, without a protection bypass.

Read Pealayer REVIEW-COMMENT-AUDIT.md completely and adapted its actual procedure
into tools/audit-github-conversations.mjs: every issue/PR/review/thread/comment page,
incomplete-response rejection, one stable ignored snapshot, unchanged-write skip,
separate hash-bound manual scope, and no closed-to-resolved promotion. First complete
792-record collection contains 142 Pealayer and 634 PCController records plus 16
Pakar records. All four unresolved donor thread bodies read; only five complete
donor issue records and those two comment scopes are marked reviewed. The rest
are pending. Rayan has no verified GitHub repo; asked for URL, read its live chat
and retained filesystem donor. Did not mutate other projects or their comments.
Final Pakar-only refresh includes security issue17: 17 Pakar records, 793 total.
Donor snapshots were retained rather than downloaded again; manual review scope
remains five complete donor records and two review-comment scopes.

Reread actual Rayan formatDateTime/BidiLtr and i18n test contracts. Imported real
BDI LTR/isolate textContent-only rendering for hostname/URL summary and locale-
specific Gregorian/Persian NTP dates. Existing protocol/secret values are not
translated or rewritten. Nine shipped localization tests pass. Full Web/quality
batch and affected failed-tail rechecks pass after correcting a missing style
property in the DOM double and regenerating the exact route-proof snapshot.
New GitHub pagination/manual-decision tests pass; this does not prove all records
were reviewed. ESP-only warm busy-wire build passes: flash code 876696 bytes,
RAM 56476 bytes, IRAM 63055 bytes. Image SHA-256:
883cd159cd4d4b073fed29ef66e2abe45214d7e019b45f3e29629d6b8d68af30.

87 completed source comparisons, 4155 pending, 54 known gaps out of 4242 files.
Pealayer live revision was refreshed without promoting other read states or
changing donor sources. Existing anchors selectively revalidated; no native,
AVR or APK rebuild, package replacement, hardware writes, new debug cache or
rendered-pixel acceptance. Native/mobile/browser cross-surface BDI/calendar,
general event/gap and output confirmation and retention imports remain pending.
GitHub glib alert1 remains open; Dependabot cannot resolve beyond 0.18.5 despite
fixed baseline 0.20.0. Track and coordinate it rather than disabling tray features
or pretending zero open PRs means security-clear.

## Living unversioned API correction

Tracking issue #16. Read the PCController network runtime, client UX and
firmware chats plus consolidation/Pealayer context. Read actual Go-Embedding-API
policy and rest_routes_test.go completely; inspect ipc.go's top-level rejection
function without certifying the unread remainder. Original bytes of the two
complete-read files match the pinned donor snapshot. Added api-contract lookup
and required gates; preserved every other pending donor row.

Applied versionless REST across firmware, discovery, Web, native/CLI and Android;
removed JSON generation selectors and numbered product subprotocol/profile/stream
identities. Reserved-zero framing and explicit capability/audio acknowledgement
replace generation/build-version inference. Authentication, operation ownership,
hash/signature/model checks and private saved settings are retained. No aliases.

Validation found an enum/non-enum warning, a nullable malformed-ACK test fixture
type error, and guard stdout contaminating typed build Plan objects. Fixed the
actual cast, fixture type and Write-Host routing. Imported storage/capture/build
gates were used, not bypassed. A normalization mistake in evidence preparation
was corrected using generate-file-comparison.ps1's exact LF hash rule; compact
and multiline references were rechecked by their unchanged anchors.

Native: 116 tests passed. Android: analysis clean, 72 cases passed in the full
batch; the failing peer-context fixture was corrected and its five-case file
passed through the gated wrapper. C++ protocol/ISP/remote update/persistence/
display/audio/PC contracts passed. Web/quality checks passed in the completed
batch plus the failed-tail recheck, without repeating unrelated suites.
Final firmware build identities and CI/physical boundaries are recorded in the
issue and FILE-COMPARISON-RECEIPTS.md, not inferred from these source checks.

Live migration remains unverified: ledboard.local lookup fails and SSH
localhost:1022 refuses connection. No physical flash, retained-settings readback,
or current production-app replacement is claimed. Never perform a client-only
rollout against the old numbered firmware API.

## Live desktop replacement, Android launcher and low-write recovery

Consult build-storage, localization, mobile-settings, capture and exact donor-audit.
Fully reread actual Pealayer storage-hygiene, workspace-hygiene, changed CI helpers,
runner, artwork consumers and appearance test; preserve concurrent donor changes.
Refresh its inventory to 514 files and selectively rebind reviewed original-byte
source hashes only after actual rereading. Totals are 84/4242 complete, 4158 pending,
51 compared gaps; Pealayer 39/514, Rayan 20/2349. Screening is not completion.

Explicit new cleanup authorization allows pruning only ignored, inactive,
non-reparse desktop/target/debug: 11852 files / 10.408 logical GiB. Protected warm
release/production/rollback executable hashes are unchanged. Routine native
Check/Test share --release; -DebugArtifacts is an essential-diagnostics opt-in.
Default storage inventory remains read-only. Newer donor cache-tag/effective
Cargo cache validation, full prune fixtures and owned RAM relocation remain pending.
Eight GiB is a guard, not a minimum RAM disk; 1 GiB scratch is optional/unallocated.

Before the defer-tests instruction: native 115/115 and Flutter 73/73 pass; affected
storage/source/security guards pass. Current native Release compiles and stages
once; graceful quit after checking all drafts, checksum-verified pending rename,
immutable deployed-inode rollback hardlink and byte-identical preferences verified.
Current production PID 70980 runs e2464fa194e666dc8f85f34017d8b20ce37152dde3ba9958e2f0333baa935966.
Actual current framebuffer PNG was inspected: offline context suppresses Table/
Games/peripheral panels; WebSocket and USB controls are interactive shell elements.
Hidden/minimized window initially queues IPC captures without updating snapshots;
blank/old status is not completion. Supported computer-use activates the uniquely
returned current window; queued capture then completes. Wrapper acknowledgement
race remains useful recovery work, not a claim of unavailable Win32 capability.

Android template launcher PNGs were an omitted OS consumer; generated canonical
PNG plus legacy/adaptive launcher XML and roundIcon correct it. Compile-only APK
is 53970105 bytes, SHA256 1dde7a0025552b5d8e0f74d845b00e314b3c074c9d46298fc7d16e57926ec941.
Streamed install succeeds; AAPT confirms foreground resource and ZIP PNG hash is
byte-identical to shared master. Running physical MainActivity screenshot inspected.
Human authorizes wake/swipe for unsecured locks; android-device.ps1 directly uses
the actual imported Pealayer package-probe helper for text probes. One bounded,
dimension-derived swipe succeeds on secure=false; foreground readback/capture
confirms launch. No PIN/password bypass or launcher-grid pixel claim. No repeated
Rust/Flutter suites after the defer-tests instruction; no SDK/cache duplication.

Fix four GitHub CodeQL findings: case-insensitive actual script extraction,
direct read/ENOENT instead of exists/read race, and single-pass literal entity
decode. Prior focused actual-script/helper regression execution passes; fresh
security-policy scan awaits publishing. Do not dismiss alerts or merge failed scans.
Copilot reviewer API rejects noncollaborator request; explicit @copilot PR comment
requests review instead, with remaining acceptance preserved on issues 2/3/9/10.

Published source checkpoint 0054114c780700d397963d5eff8335f6b790a426 has a successful
fresh CodeQL policy check; all six review threads are now resolved with concrete
fix replies, not alert dismissal. PR13 is actually merged as
5f1c3e9ae83ca7a974df1cac094295bd65bc9672. Merge main normally into the feature branch;
keep its GTK/tray dependencies, ab_glyph and added native consumers rather than
taking the older lockfile wholesale. Scoped offline Cargo reconciliation removes
one unused generic-array entry; it does not update every dependency or download
an SDK. Unify runtime/build base64 at 0.23.1, import tungstenite 0.30.0 from PR13.
Guarded release-profile Check passes on the combined source in the same canonical
cache. This is compile-only, not repeated tests or a newly deployed native binary.
No current-source feature code/settings/artwork is replaced by a rollback.

This is current package/runtime evidence, not full donor parity, board acceptance,
an all-green new CI checkpoint or production-signed Android release.

## Rayan realtime reconnect and bounded preparation

Consult exact donor-audit for sync.ts/sync.test.ts/api.ts/api.test.ts; read all
four actual files through EOF and verify source bytes. api.test.ts is a full read,
not a completed import: authenticated host/config/hotkey/audio/registration tests
remain pending their adapted implementations. App.tsx lifecycle slice was read
as context only, not certified. Three exact new comparisons bind imported
subcontracts, not whole-file feature parity. Readback-before-open uses Pakar's
cookie-authorized status GET, not the donor Go /app/session CSRF mutation.

Import actual exponential base 500 ms, cap 10 s, 0.8..1.2 jitter, failed preparation
reschedule, reset-on-open and stop timer cleanup. Import api.ts AbortController,
12 s bound and finally cleanup for this GET only. Add capped exponent, one queued
retry owner, stale timer/promise generation checks, cancellation on login loss,
pause/manual reconnect/pagehide and constructor failure recovery. Signal flows
through actual refreshAuthenticated/api; no command replay, server write,
endpoint guess or fabricated live capability. Generic lease/lock/channel election,
revision/sequence projection, audio ownership and typed host API remain pending.

Port actual prepare-before-reconnect and stop test concepts; execute actual shipped
module plus Web connectWs/refreshAuthenticated/api with fake HTTP/socket/timers.
Cover jitter/cap, failed reads, 401 login loss, late callbacks, deadline/cancel,
constructor failure and HTTPS hostname URL. First jitter-edge assertion exposes
normal floating-point 600.0000000000001; use tight 1e-9 numeric tolerance without
rounding production delay. Integration test pins its injected Math.random only,
not the global host RNG. Source/VM proof is not LAN/physical acceptance.

Coverage becomes 84/4237 complete, 4153 pending, 51 gaps; Rayan 20/2349, 2329 pending.
Full CSS, API test suite adaptation, native/mobile synchronization and hardware
acceptance remain open. Canonical generator is checked in one pass; affected route
and comparison references require exact anchor/hash revalidation before builds.
The complete quality gate catches the old pause lifecycle in canonical
assets/web/device-settings.js before compilation. Inspect its generator/consumer,
apply cancellation there too and regenerate; do not accept only a hand-edited
embedded copy. The PU route guard similarly rejects the changed WebUi.h hash:
inspect unchanged remote workspace/job/readback/apply consumers, then rebind
only its two exact target hashes (Verified remote update remains line 244).
Server registrations/source identities and all 50 route classifications remain
unchanged. Preparation cancellation also suppresses stale rendering even if an
injected fetch ignores abort; timer/deadline cleanup is immediate on cancellation.
Final review cancels preparation when applying a hostname change and relinquishes
old socket identity before replacement construction. Constructor failure visibly
reports retrying; captured old callbacks cannot regain ownership. Final shipped
code/HTTP/socket tests and full local source quality pass. Sixteen-reviewed gate
retains full CSS/API-test comparisons pending. Exact donor source hashes remain
unchanged; no unread source is accepted as an exclusion. Final warm ESP-only
busy-wire build passes; RAM 70%, IRAM 96%, flash code 83%. Exact image identity
is recorded below. No AVR/native/APK rebuild, extra cache, flash or app replacement.
Final image: 926960 bytes, SHA256
cf52f0404185f0b5bcccba918e6911eec726b7f15391c43bb7aae2ae2a4cc16b.
RAM 56620/80192; IRAM 63055/65536; flash code 876848/1048576. IRAM headroom remains
tight. Final edge-case corrections reused the same warm build path, not a clean
rebuild or downloaded SDK. Freshly reread Build RayanLamp Go app without resuming
or messaging it; its interrupted Windows regression remains unverified.

## Native menu foreground policy traced, not recolored

Consult ui-shell and exact donor-audit mappings; fully reread actual Pealayer
menu.rs and palette.rs, original bytes still match their reviewed hashes. Inspect
locked egui 0.36.2 Button::atom_ui/shortcut_text, widget_style/button_style,
Visuals::text_color/weak_text_color and menu_style. The first pointer-free Button
uses Inactive foreground as its AtomLayout fallback, not noninteractive label
ink. Stock dark 180/140 and light 60/80 differ; imported palettes map both to text.
Correct only the native paint-test oracle, preserving exact per-vertex ink checks,
all six atoms/right edges/separation and geometry. Expand to stock plus actual
Studio/Native palettes in both themes; assert named primary/muted roles and
explicitly distinguish stock label/button ink. Add diagnostic assertion context
and mandatory source regression. No production style/font/menu changes.

Full local source quality, native menu/lifecycle guards, no-write Rust parser
(edition 2024), comparison Check and Test Plan pass. Rustfmt initially exposes
a brittle literal-whitespace source guard: keep the same exact expressions but
normalize whitespace before matching; preserve every semantic assertion. Four
existing target/proof hashes selectively rebound after literal-anchor inspection,
plus the exact mandatory lifecycle proof. No source-count inflation.
Actual Test again refuses the existing >8 GiB canonical cache before Cargo;
no bypass, duplicate cache, cleanup reroute or unrelated firmware/APK rebuild.
Fresh SSH localhost:1022 refuses; ledboard.local resolution times out. No native
execution, running-app replacement or physical flash is claimed. Latest 8530a33
Windows job 114337095213 is terminal failure; new native execution/package and
deployed visual acceptance remain separate and asynchronous, not waited on.
Its actual completed job log confirms the same label-ink assertion at menu.rs:125:
114 passed, 1 failed, 0 ignored/filtered, 1.41 s; no texture/double-panic abort.

## Actual Rayan toast/store/Icon import

Freshly reread Build RayanLamp Go app, task 019f8cfc-271a-72e1-8747-e3eb883de5ec;
not resumed or messaged. Consult exact donor-audit mappings. Fully read actual
components/toasts.tsx, toast-layout.test.ts, store.ts and components/icon.tsx and
verify original-byte hashes. Actual toast styles/viewport/motion/forced-colors
slices were read from app.css; that entire file is NOT certified. Existing shared
Pealayer palette/locale and MIT notice remain authoritative; no donor build/edit.

Adapt actual serial ID, last-three-plus-new queue, 4500 ms per-ID expiry/filter,
polite/nonatomic live region, status rows/title/message, four exact SVG consumers
and translated dismissal. All current Web reports and identity success paths use
one queue outside authenticated workspace; the old shared five-second timer no
longer erases newer messages. Add four-timer eviction/teardown, bounded text and
duration, safe text nodes, keyboard Escape, focus-preserving dismissal, independent
focus/hover expiry holds, locale-dismissal repaint and pagehide cleanup. No storage
or network writes, new framework or application-brand substitution by this queue.

Port actual declarationsFor and both center-axis layout checks, plus executable
queue/DOM tests for old-timer ownership, cap/eviction, focus/hover overlap, keyboard,
literal injected markup, locale refresh, motion/forced-colors/RTL/viewport rules and
canonical embedding. Actual Web auth/identity test still checks accepted/readback
semantics against the new queue. Deterministic generator is stable after one pass.
Generic store bootstrap/revision/persistence/transport, other icon API/consumers,
full CSS, real responsive/modal screenshots, Windows OS notifications and shared
native/mobile delivery remain open. Do not port optimistic uncertain-write replay.

Coverage becomes 81/4237 complete, 4156 pending, 48 gap files; Rayan 17/2349,
2332 pending. Four complete reads are comparisons, not all-feature acceptance.

Full local source-quality suite passes. Its route guard initially rejects the
changed WebUi.h identity: inspect unchanged update workspace and route consumers,
then rebind the two exact PU route target hashes/line anchors without changing
server registrations or source hashes. All 50 route classifications pass. Nineteen
changed comparison references are selectively rebound; thirteen reviewed files
are required by the expanded gate, with the routed full app.css still pending.
The generator also rejects a scoped-improvement row without a named advantage;
add the exact already-tested queue/timer advantage, not a bypass or verdict change.

Warm ESP-only busy-wire build passes: bin 924368 bytes, SHA256
c2e97d576cf2d00ce8c6e65c880c7b0a66b5176fe2a5eed2cd3a85a5c09ceed8.
RAM 56620/80192 (70%), IRAM 63055/65536 (96%), flash code 874256/1048576 (83%).
No AVR/native/APK rebuild, new cache or physical flash; IRAM headroom stays tight.
Actual labeled read-only Web fixture: logical viewport 1280x720 and 390x844,
no horizontal overflow, both toast/copy center alignments, all rows inside viewport.
Keyboard Escape removes only the focused error row and restores next dismiss focus.
Wide/narrow JPEGs were inspected, exclusive-created and copied byte-identically
into docs/screenshots and user outputs: 89058/46865 bytes; SHA256
408dc6867b44ef2fdd14d4ac110fffbbe794379627b6f435202744e415820e08 and
ffbb898c242b05a7560bec25be43fcdcd3c8943b8e7422f38ad60c30a7717b41.
Viewport reset, owned tab closed and PID/command-line-verified fixture stopped.
This proves browser rendering with simulated notifications, not hardware or native
delivery, open-modal behavior, mobile runtime or all CSS parity.

During independent work, prior source 8849ab8 CI became terminal: workflow/source,
both firmware variants, Android preview and CodeQL pass. Windows job 114334311601
now completes 114 tests successfully, one fails (menu label_colors primary ink,
src/ui/menu.rs:125). The old texture/double-panic abort is gone; no all-green or
native package/runtime claim. Inspect real widget text policy next, not change
production colors or weaken assertions to force a pass. No blocking CI wait.

## Native render ownership and painted-color test repair

Terminal 0c723f1 Build run 38092457174 passes workflow/source quality, both
firmware UART variants and Android preview. Windows job 114331734735 fails.
The restored --nocapture log establishes four unhandled TexturesDelta destructor
panics (branding, neutral-cell, connection, font-retention tests), not their
previously inferred feature assertion failures. The shortcut test separately
compares cached PLACEHOLDER with muted ink; its assertion plus destructor panic
causes the observed 0xc0000409 abort. Earlier diagnosis is superseded by this log.

Consulted localization, ui-shell and exact donor-audit mappings; fully reread
Pealayer src/ui/i18n.rs, src/ui/menu.rs, src/ui/palette.rs and src/branding.rs.
Actual headless donor tests drain texture deltas before assertions. Read locked
epaint 0.36.2 TexturesDelta Drop, TextShape color fields and tessellator resolution,
plus egui FullOutput::drop_without_applying_deltas. No new dependency or donor edit.
Restore that ownership contract in five existing Rust test modules; check shortcut
ink through real tessellated glyph vertices rather than the galley placeholder.
All production styling/font registration and original behavioral assertions stay
intact. New native_render_lifecycle_test.mjs prevents this source-level regression
and is mandatory in source quality; it does not execute Rust or certify pixels.

No additional donor file is certified by these test repairs. Coverage remains
77/4237 complete, 4160 pending, 46 gaps; RayanLamp 13/2349 complete, 2336 pending.
Current hostname probe cannot resolve ledboard.local; ASUS SSH localhost:1022
refuses at banner exchange. No board write, app replacement or unrelated build.
Actual native re-execution, package and deployed physical/UI acceptance remain
open. The existing cache budget must remain enforced and CI is not waited on.

Complete local source-quality suite passes, including the new lifecycle guard
and exact comparison regeneration/check. All five Rust modules parse with the
actual Cargo edition 2024 using rustfmt stdout-only; this is not a typecheck.
Test Plan passes and actual Test passes donor/crate gates, then refuses the
canonical cache above 8 GiB before Cargo. Sixteen target/proof references across
eight reviewed files/documents are selectively rebound with every literal anchor
verified; no source hash refresh, donor-count inflation or new cache. The prior
source's separate CodeQL run 38092457168 is now verified terminal success.

## Restored donor native test execution contract

Previous goal turn made progress: source c814379 imported actual browser trace,
passed source quality and built ESP locally. Current authoritative CI run
38091589105 is terminal: workflow/source, both ESP/AVR UART variants and Android
preview pass; Windows fails. Separate CodeQL 38091589143 succeeds. Linux strict
font equality now passes on the real runner, not only injected OS metadata.
Firmware job 114329208305 also passes protocol/profile fixtures and uploads eight
files in artifact 11684557100 (1331965 bytes; artifact archive digest
e06cfdb894bfe958711b29c4c7287017e2eeca4b3466e79e8e2c636005107e74).
This is build/fixture/artifact proof, not download integrity or physical flashing.

Windows job 114329208291 repeats neutral-cell-fill, offline dropdown and
language-change font-retention failures, followed by 0xc0000409. Its aborted
capturing test harness hides assertion details. A logs request while that job was
live returned BlobNotFound; only the later terminal job/status/logs established
failure. No timeout/transient error was treated as terminal or restarted.

Consulted localization, ui-shell, build-storage and exact donor audit mappings.
Fully reread Pealayer src/ui/i18n.rs (1236 lines; original-byte SHA256
811f1e08e7126932772c49da8aa9669ce8f47d4e2208c1ef6cde435dde20bdf4),
scripts/run-windows.ps1 and complete .github/workflows/ci-cd.yml; live hashes match.
Locked egui/elegance registry code was inspected for active theme, menu IDs and
next-pass font installation. No donor/dependency change or new whole-file review.
Pealayer actually runs all native tests with --test-threads=1 because process-wide
application state is shared. Pakar omitted this; --jobs only limits compilation.
config::live is process-wide and preference tests mutate it, so concurrent tests
can race. This does not yet prove the cause of all three failures or the abort.

Restore serial runtime tests in the mandatory wrapper and --nocapture so failure
details survive a later crash. Keep compile jobs, assertions, test selection,
dependency versions, symbols, same target/cache and storage guards unchanged.
Restore donor CI isolated configuration and RUST_BACKTRACE=1. Local Test defaults
to a stable build/native/native-tests.json instead of the user profile, preserves
an explicit fixture override, and restores its process environment in finally.
Check/Release never receive test runtime flags or default test configuration.
Plan/regression tests cover exact dispatch and isolation; they are not compilation.

Complete local source-quality suite passes after these changes. Actual native
Test wrapper still refuses the >8 GiB canonical cache before cargo starts; no
bypass, cleanup, extra cache, app replacement, firmware/APK rebuild or physical
write. ASUS SSH localhost:1022 still refuses. Three native assertions/crash and
new execution under restored contracts remain open. Nine existing references
across five affected files are selectively rebound, with two additional exact
CI-to-wrapper/test mappings. Coverage unchanged: 77/4237, 4160 pending, 46 gaps;
Rayan 13/2349. CI is not waited on.

PERSIAN-PARITY and the existing font/CI ledger actions now explicitly distinguish
individually passing native contextual glyph tests from failing font retention.
The previous unexecuted-test wording is superseded by observed CI results;
remaining runtime/translation/grouping gaps are preserved, not silently certified.

## Rayan browser trace and portable font recovery

Reread the actual task Build RayanLamp Go app without resuming or messaging it.
Read browser-console.ts and browser-console.test.ts completely and verify live
original-byte hashes. The actual sanitizer, error projection, styled sink and
quiet-window mechanism are imported; lamp names and fixed colors are replaced
explicitly by device vocabulary and the existing shared Pealayer palette.
Shipped HTTP and WebSocket consumers are tested, not merely a disconnected helper.
Added safeguards include bounded lifetime, camelCase secret keys, prototype/format
protection, invalid host containment and stale-socket ownership. No storage writes
or automatic uncertain-command replay. Successful telemetry/polls/frames are quiet.

Terminal source 2a5574c Build run 38090552957 passed lint, source quality and Android
preview. CodeQL 38090552932 passed. Firmware passed the repaired missing-drive
gate and AVR polling compile, then failed strict FontAssets.h equality under
Node 22 on Linux. Windows compiled past the previous 11 API errors, then failed
cell neutral-fill, offline connection dropdown and font-family tests; the process
also ended with 0xc0000409. Those native assertions/crash need separate donor-first
recovery and remain open. No all-green, native package or production claim.

Fully reread Pealayer build.rs and ci-cd.yml before the font recovery. The latter's
Node 24 setup is now also used by firmware verification. Windows gzip OS metadata
is 10; normalize only the non-checksummed OS byte to RFC 1952 unknown=255. Deflate
body, trailer/CRC and decompressed licensed font bytes remain unchanged. Header
flags/MTIME that would make this unsafe are rejected. Strict asset equality stays
enabled. Tests cover Unix/Windows OS fixtures; an actual Linux rerun is still
required, not inferred from injected header bytes. No font/CDN/dependency change.

The new module initially made localization settle only on a second generator
pass. Recovery analyzes the canonical module before extracting translations,
independent of its previous embed. One generation followed immediately by --check
and shipped browser trace tests passes. Complete local source-quality suite passes.
Selective 22 existing target/proof references and two PU route references retain
their exact anchors/decisions; two new Rayan certificates bring coverage to 77/4237,
4160 pending, 46 gaps; Rayan 13/2349, 2336 pending. Toast/tokens files remain pending.

ASUS localhost:1022 still refuses SSH. No native cache bypass, cleanup, donor build,
additional cache, app replacement or board flash. Only affected ESP busy-wire
firmware is built through the mandatory warm wrapper; record its result separately.

That ESP build passed: 915632-byte bin SHA256
0544853c13cf64dbf78f488b765379d726e6c2da244b467c5a1156cc541cc5e3;
RAM 70%, IRAM 96%, flash code 82%. Existing toolchain/link flags and cache retained.

Optional local actionlint invocation failed at process execution: existing Go
temporary root PCController/go-noexec-temp denies execution of actionlint.exe.
Consulted repository-governance and the fully read Pealayer CI workflow; its
Ubuntu lint job is the accepted execution surface. No ACL/temp-root bypass,
second build cache or dependency churn. Current Node governance tests pass;
new workflow actionlint execution remains pending CI. Comparison --check itself
passes; piping its Format-Table stream into Select-Object -Last caused a local
formatting error, so the check was rerun without that output projection.

## Native API and cross-platform gate recovery

Terminal Build run 38090009184 at source 527d8e9 repeats the exact 11 native
compile errors and Linux Join-Path error from the prior run. Lint, complete
source quality and Android succeed; separate CodeQL 38090009284 succeeds.
Read the failing jobs rather than waiting or interpreting a timeout as failure.
Consulted build-storage and exact Pealayer menu/branding/preferences audit rows.
Fully reread libmpv-windows.ps1, menu.rs and branding.rs; their live original-byte
hashes match the recorded comparisons. Preferences audit remains pending: its
partially inspected donor is NOT newly certified. The actual fully read donor
menu's scoped with_layout idiom and locked egui 0.36.2 Ui/Context/TexturesDelta
implementation informed the API adaptation; no dependency upgrade or metric
change was introduced.

consult-reuse now probes literal donor roots before Join-Path or traversal.
Absent Windows PS drives on Linux become explicitly live-unavailable, while
snapshot/target/hash/anchor gates and RequireLive rejection remain intact.
No catch-all suppression or fake live source claim. Added missing-drive tests
for both reviewed build and Rayan Web slices; the actual preflight test passes.
Desktop texture tests flatten the real multi-delta map; menu tests select the
active theme's style; shell and Preferences use scoped child layouts. Table
queue ownership stays private behind stop_pending_edits, which revokes old
unsent/readback work without retrying or claiming sent writes were undone.
Added a native revocation regression; it has not executed locally.

Complete local no-compile quality suite and Test build Plan pass. Actual Test
wrapper refuses the existing >8 GiB canonical cache before cargo starts. No
budget bypass, second cache, donor build, deletion, dependency churn, firmware
or APK rebuild. ASUS SSH localhost:1022 freshly refuses connections. Native
compilation/paint tests, Ubuntu builds, runtime screenshots and physical
deployment remain pending new CI/host acceptance. Only 15 references to the six
actually changed target/test files were rebound; no new donor certificate,
blanket source hash refresh or inventory-count change.

## RayanLamp locale continuation

Reverified Build RayanLamp Go app (019f8cfc-271a-72e1-8747-e3eb883de5ec);
not loaded, not resumed or messaged. Fully read i18n.tsx (656 lines) and its
71-line test; original-byte hashes match inventory. Reread actual Pealayer Web
i18n and current locale/generator/test consumers. Imported real normalization,
translation placeholders, number/digit presentation and automatic locale
lifecycle, preserving independent explicit direction and peer identity. No
OS-event writes, request/replay or draft replacement. Seven tests and complete
source-quality run pass; actual browser mixed-script selection and reversible
network labels inspected. Dynamic copy still visibly incomplete; native/mobile
and Persian-calendar/BidiLtr consumers remain explicit gaps.

Interface --check caught edits in generated device-settings embedding. Fully
read the actual generator/canonical module, moved edits to assets/web/device-settings.js
and regenerated deterministically. No --check bypass. Reread exact PCController
card-layout-controls and Pealayer HeaderBar; only affected target/test references
were rebound. Actual canonical PCController bytes match its inventory even
though the old lookup root is unavailable. PU route guard caught Web document
hash drift; only the two unchanged Verified remote update rows at line 220 were
reviewed/rebound. No blanket source certificate or route behavior change.

Terminal CI run 38089030955 confirms the prior CodeQL lint fix works on Ubuntu:
lint, quality and Android success, separate CodeQL run 38089030973 success.
Overall Build fails: Linux firmware donor lookup trips on a missing C: drive;
Windows tests have 11 egui/private-table API compile errors. These are real
remaining release defects, not a job being awaited or a reason to suppress gates.
ASUS SSH localhost:1022 refuses; no physical firmware/app replacement occurred.
Warm mandatory ESP-only busy-wire wrapper then passed: 906928-byte image SHA256
02464a4211b97e98ff1e22b2b88feeef32608a6465a480f0c96a055aa43e7012;
IRAM remains 63055/65536, 96%. No AVR/native/Android rebuild, copied firmware
package, new dependency/cache, clean or budget bypass.

## CI security-summary recovery

Fresh run 38088825179 is terminal failure, not a job still awaited. Ubuntu's
actionlint v1.7.12 invokes ShellCheck and reports SC2129 on repeated security
summary redirects; all product build jobs were skipped, so no green native/APK
claim is possible. Consulted repository-governance and the exact CodeQL row;
reread the full actual Pealayer codeql.yml. Imported its grouped redirection,
preserving the success gate and honest analyzer coverage, with no suppression.
Added a local grouping/non-suppression regression; existing actionlint v1.7.12
and governance tests pass. Local Windows lacks ShellCheck, so that is not an
Ubuntu ShellCheck execution claim. No tool download, clean, rebuild or CI wait.
Only the two affected target/proof hashes were rebound after explicit rereview.

## RayanLamp Web theme continuation

Reconsulted full Rayan inventory and exact theme rows plus existing appearance
mapping. Imported actual pre-paint/controller/metadata code and donor tests into
one shared palette engine. Added web-presentation five-source gate, before ESP
builds and quality; Bash uses the same PowerShell gate and checks all generated
font/brand assets. Partial i18n/settings reads remain pending, not exclusions.

Guard caught ci-checks.mjs hash drift when registering the new tests. Consulted
the exact Pealayer workflow row and reread its full source; runner still stops on
the first failed child and retains governance contracts. Rebound only that
reviewed hash. Next guard caught old WebUi hash/line in existing PU route evidence:
reread the actual route checker, inspected unchanged target forms and updated
only two Web targets to anchor line 220. No altered donor source or blanket
certification. Documentation patch with an incorrect heading failed atomically;
read exact headings and retried the targeted patch, without shell rewrite.

Full source quality, six new theme tests, ten appearance tests, actual browser
Persian light/dark reloads and ESP warm compilation pass. Browser evidence saves
with exclusive-create to avoid overwriting; native capture/build tools not used
because native code is unchanged. Existing licensed font and peer identity are
preserved. ASUS SSH still refuses; no device mutation, native replacement,
Android rebuild, CI wait, duplicate SDK/cache or blocked cleanup reroute.
Exact image/build identities and acceptance limits are in the existing receipts.

## RayanLamp import recovery

Located Build RayanLamp Go app through the task index and read its exact thread.
Cross-task listing did not contain the older task; bounded read-only lookup
located its original source, which has no Git metadata. No donor task was
resumed/messaged and no donor files were edited.

Consulted the existing games-streaming route, then extended the SAME ledger and
donor-audit index with RayanLamp. Fully read haptics.ts/haptics.test.ts,
connection-preferences.ts/its tests, digits.go and LICENSE before importing.
Guessed absent paths were corrected by enumerating actual filenames; no missing
filename became an excuse to discard a feature. Oversized thread/evidence output
was filtered into bounded metadata and source-ordered diff hunks. A grouped-hash
patch and an invalid receipt anchor were rejected; corrected exact hunks were
applied, without wholesale file rewriting or donor mutation.

The existing Web table/settings harnesses extract isolated script slices. They
initially lacked the newly canonical dependency; updated those harnesses to load
the actual rayanlamp.js first rather than adding fake production fallbacks.
Unchanged donor certificates were retained; affected target/test/generator hashes
were rebound only after reviewing their actual diffs and preserving old contract
semantics. The font generator remains deterministic and skips identical writes.

First-party/vendor source is inventoried and original-byte hashed; generated
directories are pruned BEFORE traversal. Live filesystem inventory drift is
checked, not just selected file hashes. Useful unreviewed files remain pending.
See RAYANLAMP-REUSE.md and issue #3; test-code anchors are not runtime acceptance.

## Persian regression recovery

Consulted localization, preferences, build-storage and exact donor-audit entries.
Read the complete actual Pealayer src/ui/i18n.rs, web_ui/src/i18n.ts,
tests/i18n_surface_audit_test.rs and src/ui/mod.rs; configuration and Preferences
reads were selected functions/excerpts, NOT whole-file certificates. Refreshed
only the changed Pealayer inventory, leaving every donor read-only. Three new
complete-read certificates bind real targets/tests and concrete pending omissions.

Actual donor font/dictionary/shaping/alignment/layout and audit patterns were
adapted to native, Web and Android shared bytes. Added a localization reviewed-
slice build gate, without pretending it certifies unfinished UI/mobile surfaces.
Read actual egui/Elegance APIs: Elegance adds fonts, but host set_fonts resets
the registry. Language refresh now preserves registered widget fonts; initial
install occurs once through resolved config. Tests cover both native families
and font retention in code, but native execution remains blocked by storage.

Native wrapper refused the existing 13.49 GiB cache against its unchanged 8 GiB
budget. No clean, alternate cache, dependency churn or guard bypass followed.
Read donor storage/low-write mappings; requested obsolete-debug-only cleanup
while preserving release/running app/settings/rollback. No approval or cleanup
is claimed. Existing production app remains unchanged.

Read-only actual Web fixture rendered Persian Vazirmatn and RTL. First screenshot
exposed still-English hints/placeholders; added canonical dictionary entries,
owned attributes and dynamic settings copy, then inspected the refined capture.
Peer names/hostnames remained untouched. Browser font-loaded check passed and
no horizontal overflow was observed. This is simulated Web, not physical proof.

The new test DOM mock initially had a missing closing brace; fixed the test
syntax without changing production behavior. A comparison patch accidentally
appended literal plus characters to three JSON separators; generation failed,
separators corrected and all comparison checks passed. Existing route hashes
and menu anchors were explicitly re-reviewed/rebound after localization insertions;
source certificates and omission semantics were not replaced. Separate reordered
PU target/hash fields required exact-field patching, not broad source acceptance.

Local quality suite and affected ESP busy-wire build passed. No CI wait, native
replacement, AVR build, Android package or hardware flash occurred. Full mapping,
candidate hash, source/runtime boundaries and remaining actions: PERSIAN-PARITY.md.

## Web device-settings and interface-contract continuation

Issue #4 owns the requested Web/native parity. Consulted ui-shell/preferences,
exact donor-audit HeaderBar/card-layout-controls/pointer-reorder and build-storage
run-windows rows, and read those actual sources completely. Native Network/Cloud
Draft/SecretEdit/Write semantics are adapted, not a replacement defaults form.
Shared generated Rust/Web/Dart page/field catalogs and build checks now reject
catalog drift. INTERFACE-CONTRACT.md names still-unported editors/platform work;
page catalog synchronization is not full mobile or whole-product parity.

Computer-use skill selected the actual embedded Web, read-only simulated fixture.
Verified capability-gated pages, MQTT/WebSocket split, unsaved draft navigation
and responsive geometry. The first narrow screenshot exposed intrinsic nav/grid
overflow; inspected the actual shrinkable controller CSS and corrected min-width
and mobile minmax grid. Retest reports equal dialog client/scroll width. Secret
rows follow observed presence and hide outside Replace. Screenshots are small
outputs, not copies of build packages. No physical writes or secrets were sent.

Unexpected diagnostics: guessed PU route-evidence and mobile settings-file paths
did not exist; inspected actual file inventories and used PU850-ROUTE-PARITY.json
and mobile main.dart instead. A CSS anchor patch failed atomically; reread the
exact line before applying. Local browser binding name was absent after context
refresh; reused selected browser ID 2 and the owned tab, not another browser.
The initial required-field/access DOM double lacked getElementById; the helper
uses documented querySelector fallback. Exact target hashes changed as expected;
rebound only after reviewing preserved handlers and new module code. No donor
checkout, global credentials, private preferences or build cache was overwritten.

Acceptance: actual settings module/HTTP doubles, Web access/context/table/cells,
appearance (10 tests), identity, remote, display, remote tools, all 50 PU route
registrations, branding, native-menu and transport source guards passed. Shared
generator --check, ledger integrity, file-comparison, reuse-preflight/storage
tests and Rust edition-2024 source parsing passed. ESP busy-wire warm build
passed: RAM 56460/80192, IRAM 63055/65536, flash 756732/1048576. AVR unchanged;
no AVR/APK or donor build. Native Test refused at the existing 8 GiB cache guard
before Cargo; native execution and replacement are not claimed. Flutter/Dart
were not on PATH; added mobile catalog tests remain unexecuted locally.
Fresh ASUS SSH localhost:1022 is refused; ledboard.local:80 cannot resolve.
No fallback to production Cafe, firmware flash or physical acceptance claimed.

Pealayer inventory refresh detected changed board_info/status_bar/menu/icons.
Reread all four fully. Another task has dirty tracked consumers and an untracked
src/ui/dropdown.rs dependency: do not overwrite/import its moving work as a
finished donor release. Named pending dependency/consumer migration is recorded
in the same comparisons; icons verdict downgraded to parity-gap. Icons changed
again during review, so captured its original bytes/hash once, read the entire
captured text, and bound that exact snapshot in inventory/comparison. No later
live revision is implied. Exact dropdown lookup cannot reach the untracked file;
that is pending provenance/review, not an exclusion. Actual card-layout-controls
source comparison added; pointer-reorder was read fully but its unported panel
ordering remains pending. Current comparison totals: 1885 / 53 reviewed / 1832
pending / 35 gaps, not complete superiority.

Comparison patch diagnostics: fixed indentation/comma preservation and absolute
target paths after atomic failed hunks. Hash-only patch initially matched no rows
due indentation; inspected actual JSON before rebinding eight changed targets.
The source-comparison gate correctly rejected moving donor hashes; complete
rereads and named omissions replaced stale equivalence, not blind hash updates.
Browser viewport override was reset and owned simulated tab/fixture closed.
Final inspected Wi-Fi image is outputs/PakarController/web-parity/settings-wifi.png.

## Front-panel WebSocket/USB dropdown continuation

Issue #10 owns this transport-context change. Read AGENTS/preflight and issue
#10; ui-shell consultation first rejected concurrent Pealayer tracked inventory
drift. Refreshed only its inventory, then consulted exact donor-audit status_bar
and menu mappings. Both donor files were reread fully, including the real
connection selector/custom OS-path fallback; donors remain read-only. The
front-panel labels now call connection_controls popups with typed routing,
port/disconnect, reconnect and configuration actions. USB scan happens only
inside its open popup, reports failure and preserves unavailable configured
ports. Same-endpoint WebSocket reconnect no longer silently drops the request.

No native build guard bypass, cache duplication, automatic prune, firmware build
or production replacement. Rust popup/pointer tests are added; Node wiring and
menu source guards plus Rust parsing passed, not native runtime acceptance.
Unexpected diagnostics: a guessed esp/src/web* path failed (no files changed);
subsequent inventory uses exact known paths. Combined lookup output was too large,
so reread both full donor files in bounded chunks. A receipt patch with a guessed
heading failed atomically; reapplied only after reading its actual header.
Current native build/deployment acceptance is recorded below and in issue #10.

Result: guarded native Test refused before Cargo because desktop/target remains
13.49 logical GiB / 16068 files against its 8 GiB policy. Consulted build-storage
and completely reread actual Pealayer scripts/storage-hygiene.ps1; selected the
already imported read-only summary, not donor recursive prune. No new native
binary/screenshot or running-app replacement is claimed. Existing build gate,
reuse preflight, ledger/file-comparison tests, connection/menu source guards and
diff whitespace validation passed. Comparison patch attempts first exposed JSON
anchor escaping and then out-of-order hunks; both failed atomically. Reapplied
with JSON-escaped anchors and ordered, separate insertion after exact inspection.
Inventory is 1885 files / 52 complete-read comparisons / 1833 pending / 33 gaps.

## Board/application identity continuation

Issue #10 owns this identity request. Read AGENTS/preflight and issues #4/#10;
consulted ui-shell, preferences, application-identity and exact donor-audit
branding/config/Commands rows. Pealayer src/branding.rs and HeaderBar.tsx were
completely read; actual runtime cached icon/multi-viewport/tray and name/artwork
binding are adapted. config.rs resolver and PU Commands.cpp are screened
slices, still pending entire-file review; no whole-repository parity claim.
Pealayer was refreshed to the current tracked 505 paths (all origins 1884).

Current acceptance: actual embedded Web identity/context/access/branding,
display/cells/remote/menu tests and production DeviceIdentity host tests pass.
New C++ tests compile actual APP macros for Table, non-Table and custom app.
Rust source parses as edition 2024; unit tests/USB write ACK/readback/CLI/native
tray remain unexecuted because donor-gated build Test refuses 13.49 GiB canonical
cache against 8 GiB policy. Read actual donor scripts/storage-hygiene.ps1 and
Pakar storage report after this block; no prune, policy bypass, duplicate Cargo
cache, SDK removal, native build/stage or running-app replacement occurred.
Existing production PID 62448 was freshly observed, not attributed this source.

Unexpected operation recovery: first ESP compile required an explicit declaration
of existing appendJsonEscaped for the new USB handler; inspected actual helper
and added its matching prototype, no alternate encoder. Web fixtures initially
lacked the new identity renderer/metadata; extended actual application tests,
not product fallback. Ledger-bound route test correctly rejected changed Web
bytes; explicitly rereviewed the affected name/application/discovery/info slice
and rebound exact anchors/hashes. Rejected patch hunks had no partial writes;
corrected from exact source. Invalid rg glob/guessed cli/stub/workflow/muda paths
were resolved from inventories; no skipped functionality was justified by them.

Computer-use skill selected read-only browser verification of the actual embedded
Web served by a visibly SIMULATED controller fixture. Verified Pakar Keys header,
Keypad controller title, application descriptor, separately populated full-width
board-name/hostname fields and real artwork. Saved/inspected no-overwrite images;
identity-settings.png is final alignment, identity-controller.png preserves the
prior narrower fields. The owned tab was closed; fixture PID identity was verified
before cleanup. No physical/API mutations or login secrets were sent.
ASUS SSH localhost:1022 still refuses connection; ledboard.local port 80 is
unreachable. A read-only check of the remembered alternate SSH port returned
cafe-pc, not the requested ASUS board host, so it was not used to build/deploy.
Native/physical acceptance
checkboxes remain open; CI is asynchronous and is not waited on.

Publication recovery: first push and proxy/helper retry returned a real GitHub
403: the active DRSDavidSoft account had pull=true/push=false. The already
configured atomicdeploy account had push permission for this repository.
Used its existing credential only in the publishing subprocess, restored the
previous GH_TOKEN and did not switch global login or print/store the token.
Source checkpoint bded015be023e773b3c7dce276803df666a3d543 was verified on
the remote branch. Issue #10 comment 6100668023 holds the checked/unchecked
identity acceptance; #3 comment 6100668240 records exact ledger continuation.
The PowerShell formatted-report/Select-Object diagnostic failed only in output
formatting; reran the actual remaining tests directly and they pass.

Tracking: issues #3 (all donor comparisons), #9 (storage tooling), #8 (capture).

Pealayer inventory refreshed file by file from clean canonical source
a54933aa7d0c05a058045dc9cd21b1b7fbc7378e: 493 tracked paths, none missing,
original-byte hashes/sizes and screening state retained. Other five inventories
were preserved. Automated screening is not a complete manual read. Six actual
tooling/hygiene files were read fully and added to the existing authoritative
comparison ledger: package probe, libmpv/shared resolver, runner, packager,
crate-boundary guard and workspace hygiene. Each has exact target/test anchors,
hashes and individual omitted-code reasons.

Pealayer comparisons increased from 11 to 17; 476 remain pending and nine reviewed
files still have gaps. Across all donors: 1,868 inventoried, 39 complete comparisons,
1,829 pending and 22 known gap files. Five newly reviewed tooling files correctly
remain parity-gap, not accepted exclusions. Only the complete package-probe helper
is byte-identical imported code (SHA-256
f4516884990ad76c020fbc6534d902c4949abae0b48da8ea40c008afa44dfd4f).

Implemented prevention: nine capability routes into the same file ledger;
read-only lookup with unknown-concern and unreviewed-gate rejection; exact required
source/target/test hash checks; live donor byte and tracked-path verification;
mandatory gates before native build/Plan, staging and screenshot capture; first-step
instructions in AGENTS.md for every task, failure and missing capability.

Executed locally:

- tests/reuse_preflight_test.ps1: all nine routes nonempty; cached build/capture
  gates; rejects unavailable live sources, unconfigured gate, unknown concern,
  screened rather than complete read, empty target mapping, stale source/test hash
  and escaping target. Actual imported Process helper returns short-lived stdout
  and rejects exit 7. Actual adapted resolver expands relative cache override.
  Source-stability helper rejects wrong commit.
- Live build-storage lookup with Gate and RequireLive: ten matching files,
  all live hashes match refreshed inventory, four pending comparisons and five
  reviewed gaps remain visible. Consultation does not certify them complete.
- Imported Process helper runs existing Pakar --help successfully. Optional stage
  -Probe on identical current artifact returns Changed=false/Written=false.
- Build storage plans/parser tests, reuse/ledger comparison adversarial tests,
  screenshot source guards and version checks pass. No compiler or firmware
  flasher was run, no app replacement and no private settings write occurred.
- Repeated unchanged inventory/comparison generation leaves write timestamps
  unchanged for legacy-source-ledger.csv, file-comparison.csv and FILE-COMPARISON.md.

The capture wrapper's only functional addition is the preflight gate; its previous
PID/backend/no-overwrite behavior is unchanged and both donor target hashes were
rereviewed. This pass does not claim a new runtime capture of that wrapper.

Limitations remain explicit: full package manifest/embedded build identity,
saved host/toolchain provenance, generic runner fallback, crate-boundary adaptation,
font/Web/branding/resource packaging and full screenshot/interaction matrix.
The original Process probe does not terminate its child on timeout. Native CLI
help is not full renderer/DLL smoke. Arbitrary direct shell commands are not
sandbox-enforced; continuation instructions prohibit bypassing these gates.

Earlier cleanup is still blocked; no old cache/SDK/source/app backup was removed.
The running native app remains the prior cell-surface executable. Source/CI
checks are not board, LAN, Android or whole-project production acceptance.

## Exhaustive consultation and actual guard follow-up

The earlier counts above describe the previous checkpoint, not current closure.
This follow-up retains the SAME inventory and comparison authorities:
1,868 files, 39 complete comparisons, 1,829 pending, now 21 reviewed gap files.
Pealayer remains 493 files / 17 compared / 476 pending; eight reviewed gaps.
Live canonical Pealayer is still clean at a54933aa7d0c05a058045dc9cd21b1b7fbc7378e;
all 493 inventoried bytes match. No new complete-file read is claimed from lookup.

Specific failure: specialist keyword routes cover only 530 distinct paths.
The other 1,338 (including 309 Pealayer paths) were inventoried, not approved
exclusions, but weren't returned by any specialist lookup. Added donor-audit
to reach every path, plus exact Origin/SourcePath lookup, visible AuditOnly,
read/availability, code/reason/disposition omissions, proof meaning, remaining
actions and owning issues. Existing CSV now includes routes and issue links.
Snapshot-stale comparisons are visibly stale even in ungated lookup.

Actual recovery source consulted fully in this follow-up:

- Pealayer docs/verification/LOCAL-WORKSPACE-HYGIENE.md: preserve the canonical
  cache, stable candidate and owners; do not route around blocked cleanup.
- Pealayer scripts/check-crate-boundary.mjs: adapted into
  tools/check-crate-boundary.mjs, not merely copied into a plan. Its actual
  module ownership/thin executable/resource guards now execute before native
  builds and in CI. The original assumes a library; Pakar detects optional
  lib.rs, applies shared ownership guards when present, and checks duplicate
  declarations/resources today. Exact source/target/test hashes and reasons
  replace this file's previous pending comparison in the same manifest.

Locally executed: exhaustive/filtered lookup, stale-source comparison checks,
all existing routing/gate/adversarial tests; comparison integrity and stale
report rejection; storage plan/parser tests; actual single-binary guard and six
negative crate/resource regression cases. Comparison -Check makes no writes and
refuses stale committed reports; CI uses it. Repeated unchanged generation leaves
CSV/report timestamps unchanged. Comparison fixtures use one owned stable
build/native directory instead of creating a fresh GUID directory each run.

Next-file worklist (all remain open in authoritative rows / issue #3):

- scripts/build-metadata.rs and scripts/package-windows.ps1: embedded source
  identity, package manifest and actual smoke tests, not just CLI --help.
- scripts/libmpv-windows.ps1 and scripts/run-windows.ps1: generic host provenance,
  saved configuration ownership, toolchain fallback and mpv-free runner.
- scripts/new-icon-pack.ps1: inspect actual icon authoring pipeline before
  replacing or claiming tool unavailability; currently only audit-routed.
- scripts/verify-messaging.mjs: inspect actual event/action verification harness,
  then map generic contracts; currently only audit-routed.
- Pending UI/preferences/Web/mobile/PCController/PuzzleBoard/PU sources stay
  open. This is not an import-completion or production claim.

No compiler, donor build, SDK download, firmware flash, app replacement,
cache removal or package copy was performed for this follow-up.

## Live donor drift caught and actual imports continued

The historical counts above remain historical. Canonical Pealayer advanced from
a54933a to clean d3f11b18d3b969398df18c26bc5871b426d4c332 during this pass. The build
preflight rejected the changed tracked inventory **before** the ESP compiler ran.
No failed-build retry bypassed that guard. Read the actual new storage-hygiene
script and all changed libmpv-windows, package-windows and hygiene sources through
EOF, then explicitly refreshed their individual source/mapping/test evidence.

New actual imports: Get-DirectorySummary and Test-PealayerFileContentEqual
(Pakar-named), called by read-only storage inventory and candidate staging.
Delete-destination-first copying is replaced by verified pending-copy/rename;
mutable compiler output is not hardlinked. Cache-inside-checkout rejection needs
an owned nonduplicating migration of the existing sole cache. Prune remains
pending: importing a newly available deletion script is not permission to reroute
blocked cleanup. Read-only scan still measures Cargo 13.49 logical GiB, mobile
build 0.971 GiB; these are not SSD writes or physical recovered-space claims.

Also read actual appearance.ts, appearance.test.mjs, App.tsx and styles.css.
All five appearance functions and four actual donor tests are now adapted into
the shipped Web UI, with the actual palette catalog rather than another theme
engine. Complete generic App/styles mappings and native/Web/mobile preference
transport remain pending. See WEB-APPEARANCE.md for exact uses and browser proof.

Current authoritative comparison: 1,869 files, 42 complete source comparisons,
1,827 pending, 23 reviewed files with gaps. Pealayer: 494 / 20 compared / 474
pending / 10 gap files. Increased gap count exposes additional unfinished useful
behavior; it is not accepted as an exclusion or a whole-repository parity claim.
New storage/appearance/App/styles routes prevent these files remaining invisible
to specialist lookup; exhaustive donor-audit still reaches every inventory row.

Executed: live build-storage Gate/RequireLive (11 paths, zero stale, four pending,
five mapped gap files visible); donor routing/hash/adversarial tests; actual
read-only summary and content-equality tests; comparison integrity and stale-report
rejection; storage Plan/parser tests; ten appearance and existing access/display/
cells/remote/PU-route tests; screenshot source guards and source version 0.2.7.
Comparison Check also correctly rejected the report after its target hash was
changed to reflect readable inventory formatting; regeneration follows explicit
rereview, not a blind build retry. Firmware compile/deployment is separate proof.

Both previous ecf882d CI runs (38059703363 and 38059700392) passed Windows,
both firmware variants/protocol tests and Android preview. They do not certify
the new appearance/storage checkpoint. Current CI is tracked on the existing PR.
No native/Android/donor build, firmware flash, app replacement or cleanup was
performed for these imports; only the affected ESP compile was requested.

## Native menu regression: actual source comparison and guarded recovery

Read full Pealayer src/ui/menu.rs and src/ui/mod.rs at 3d5f7ae. Read egui
0.36.2 shortcut atoms/menu_style/Spacing dependency code; both apps share that
version. Actual top_menu_button helper imported with only state ID/visibility
changes. All six current Pakar accelerators now use the donor shortcut_text
idiom, not spaces in primary labels. Four root headings use active-menu hover
switching. Three native regression tests cover actual painted right edges/weak
colors, popup switching/closure/context isolation and production call sites.

Height diagnosis is documented separately: Pakar minimum 28 / vertical spacing
6 / font cap 13 versus Pealayer default minimum 18 / spacing 3 / font cap 12.
Popup padding is overridden by egui, not the explanation for that difference.
No documented requirement justified the divergence. Height/font/spacing/zoom
remain unchanged as the user requested; no claim of corrected menu heights.

Donor advanced during this pass to clean 8d9bc1d86988987354e373a049ef953f9cf3034d.
Menu/mod source hashes remain unchanged. Selectively refreshed the same
inventory to 496 Pealayer paths; its two new verification/doc paths are pending.
App.tsx changed since its earlier read and is correctly stale-review, not carried
forward as current certification. Current complete comparison count is 43 of
1871; 1828 pending and 24 reviewed parity gaps remain visible. Complete file
reading is not complete import acceptance: the new menu row is parity-gap,
with concrete generic omissions/next tests in NATIVE-MENU-COMPARISON.md.

Executed native_menu_test.mjs source guard: normalized imported helper body
matches donor exactly; four production root headings and six shortcut atoms;
unchanged height policy. Rustfmt parse/check and comparison Check passed.
Attempted the mandatory native Test wrapper: donor/build gate and crate guard
passed, then the canonical-cache budget guard refused compilation (>8 GiB).
No bypass, new cache, deletion reroute, donor build, or package copy attempted.
Native paint/pointer tests must execute on CI; source guards are not native
visual/runtime screenshots. The running app was not replaced in this pass.

## Cross-surface application branding regression

Cause: native build used Phosphor Circuitry; native toolbar/About separately
tinted a font glyph; Web hardcoded a diamond and omitted application favicon.
Read all of actual Pealayer src/branding.rs, build.rs, assets/icons/README.md,
HeaderBar.tsx and webPlatform.ts before adapting the resolver/image binding.
Imported icon_data_from_bytes unchanged and actual seven-frame ICO conversion
with its shell-derivative test adapted to Pakar's PNG-only decoder. One 740-byte
lossless PNG master preserves the verified existing native glyph pixels; native
PE/window/tray/header/About and Web header/login/favicon/touch consume it.
No per-surface recoloring, new drawing, donor build, compiler or icon disk cache.
Existing semantic Phosphor quick-action generator remains unchanged.

New generator first rejected adjacent empty bootstrap markers; fixed the order
guard to explicitly accept an empty block. Appearance stale check caught new
branding CSS placed inside its generated block; moved it outside that owned
block before regeneration. Existing generate-web-appearance.mjs was reread and
used as the no-write/marker-ownership reference. Both current-generation checks
and unchanged-timestamp branding tests now pass; no blind repeat/build bypass.

Live donor inventory then advanced to 498 tracked paths; the exhaustive guard
rejected drift. Refreshed all Pealayer paths without touching donor files.
Reread current src/ui/icons.rs through EOF: its only new change was the STOP
re-export, now imported; old picker functions/eleven tests are retained. New
unreviewed paths stay pending and unrelated changed files stay unreviewed/stale.
Current report: 1873 paths, 47 complete-read comparisons, 1826 pending, 28 gap
files. Branding/HeaderBar/WebPlatform/build rows are explicitly parity-gap,
not claims that custom runtime presets or all browser capabilities are imported.

Executed: actual decoder-body identity guard, exact PNG/native pixel hashes,
four identical embedded Web assets, native production consumer/source checks,
no-write generation, native menu source guard, ten appearance tests, Web access,
Rustfmt parse/check, offline locked dependency metadata, comparison Check and
reuse gate/adversarial tests. Reused already locked base64/image versions as
build dependencies; no new dependency set/download/cache was created.
New native pixel/ICO/texture tests run asynchronously on CI per user instruction;
no blocking wait, running-app replacement or board flash claimed for this patch.

## Tint-free unified cell selection and two-way Find cell

Consulted ui-shell mappings and exhaustive exact donor lookup. An assumed
src/ui/selection.rs path had no inventory row; stopped that lookup, searched
actual files and consulted src/ui/controls.rs instead. Its neutral/fixed-geometry
interaction and selection-before-paint slices were read, not falsely promoted
to a whole-file review. Added controls.rs to ui-shell routing; its generic
remaining capabilities remain pending in issue #3. Existing actual imported
dialog/palette/icons are retained. Tinting transport intent is deliberately not
copied onto cells because the user explicitly requested no tint.

Native/Web selected and matched cells now share an amber outline and neutral
alternating cell surface; red LED state no longer colors the background. Two-way
query/selection binding covers clicks, toggles, ranges, row/all/clear, manager,
keyboard and marquee; invalid drafts remain editable, but repeated selection
gestures replace them. Native revision-based sync requests repaint; F3 navigation
does not collapse multiple selected matches. No selection writes LEDs.

Extended actual shipped Web DOM/HTTP-doubles tests pass, including same-cell
reselection after invalid input and exact range display. The old appearance
test correctly caught its obsolete matched-only selector; updated its expected
contract to unified tint-free emphasis and retained all four donor tests.
Reviewed focused changes at all existing shell/appearance/icon/branding evidence
anchors before rebinding hashes. Native tests added/extended for query
roundtrip, revision/draft handling, neutral LED state and shared painter output.

Native Test wrapper passed donor/crate gates then refused the canonical cache
above the existing 8 GiB budget. Consulted actual recorded build/storage recovery;
no guard bypass, cleanup reroute, new cache or direct compiler retry. Rust tests
and deployed screenshot acceptance remain pending; CI is not waited on.

## One authoritative selection domain, with normalization retained

Reconsulted ui-shell and exact controls.rs donor row and actual neutral/editor
slices; no full-file import or new donor parity claim. The preceding fix still
passed a separately reparsed matches list into native painting/navigation/status.
Removed that path: Selection owns validated IDs, Find is its input adapter and
highlighting is its membership projection. Web likewise removes the duplicate
matched class and redundant Select matches command. Selection IDs now drive all
paint, navigation, counts, manager/menu and bulk targets. Status canonical query
and editor draft are explicitly distinct; membership aliases read the same set.

User clarified that reparsing for normalization is useful: parsing is retained
at validation/commit and valid blur/Enter; normalization derives sorted,
deduplicated ranges from the model. Invalid drafts preserve canonical membership
and navigation. Added one native/Web shared transition fixture and shipped-Web
membership/ARIA/count/normalization/Enter/F3/Escape checks. Tightened native
numeric input to match Web ASCII-digit and single-# rules; +3/##3 cannot diverge.

Patch validation rejected a repeated file target and then out-of-order hunks;
no partial edits were assumed. Split into verified ordered patches. No blind
compiler retry: native Test passed donor/crate gates and again refused the
existing canonical >8 GiB cache. Actual source/Node checks are distinct from
pending Rust execution, native captures, app replacement and board deployment.

## Manage cells hover geometry

Consulted ui-shell and exhaustive exact dialog.rs donor mapping, then read all
952 live donor lines through EOF (snapshot hash matches). Adapted actual
navigation_control fixed-slot/clipped-text/inside-stroke mechanics to the cell,
row and state selectors. Palette/selection/animation still use the table's one
cell_surface; there is no separate tinted selection engine. Other generic dialog
helpers and actual donor numeric/host/geometry tests remain named pending items
in the same comparison manifest, not exclusions or a whole-file parity claim.

Traced the regression through locked egui 0.36.2 Button::selectable / atom_ui:
the inactive unselected Frame omits its stroke, while hover supplies a styled
Frame; AtomLayout sizes and positions contents using Frame::total_margin, which
includes stroke width. That is layout-changing chrome, not just an optical
outline. The manager now allocates constant 32/96/104 x 28 rectangles and paints
strokes inside them, with centered text independent of the stroke. Row headers
remain top-aligned when cell slots wrap. Web fixes the border-box slot and puts
focus chrome inside; selected focus cannot add a competing outline.

Added an actual native painter/input regression across narrow/wide widths,
animation on/off and idle/hover/press/focus/selection. Native Test passed donor
and crate gates but refused the existing canonical >8 GiB cache before compiling.
No cleanup reroute, direct compiler, new cache, app replacement or board flash.
Rustfmt parses/formats the patch; native execution and captures are pending.
Shipped Web selection/manager DOM-double tests and geometry source guards,
all ten appearance tests, access/display/branding/menu/generation checks and
reuse/comparison adversarial checks pass. These are not browser pixel evidence.

A first Web patch assumed a standalone closing style tag and was rejected
without edits; inspected the actual embedded boundary before applying. A display
test filename was wrong; rg inventory identified web_display_test.mjs, which
passed. Donor read/comparison is 48 complete files of 1874; 1826 are pending.
Useful generic functionality remains open and no project-wide superiority is
claimed. The existing issue #8 owns runtime acceptance; CI is not waited on.

## Explicit mode-gated cell reorder

Reconsulted ui-shell and exact donor-audit layout.rs/HardwareTab.tsx rows. Actual
bounded drag source/target/rank and Web drop/drag-image lifecycle slices were
read and adapted; neither large file is certified fully read/compared. Native
layout source has changed from snapshot. Recorded current hash and screened
status rather than rebinding a stale donor comparison. Added HardwareTab to the
ui-shell index so this generic capability is not missed by future lookups.

Implemented one validated client order used by both table and manager. Reorder
and reset require explicit Edit order mode; mode defaults off and cancels active
drags/marquee on transition. Before/after insertion preserves IDs, mapping, LED
state and canonical selection. Native drag ownership is surface-specific and
visible-target clipped; Web drops require an active own source, not external
transfer text. Valid saved orders repair duplicate/missing/out-of-range IDs.
Actual donor geometry/release/insertion and Web lifecycle code are adapted, not
replaced by a style-only approximation. Generic host presentation RPC, full
manager/lock/hide features and remaining donor tests stay pending.

Same nine-step fixture drives native/Web insertion semantics. Actual shipped
Web DOM/drop tests additionally cover both surfaces, cancellation, mode gates,
normal-mode marquee, navigation and exact persistence-write counts; no reorder
calls any device API. Existing appearance/access/display/menu/branding checks,
generation consistency, Rustfmt parse and hash-bound comparison/preflight tests
pass. Native Test wrapper again stops before compiling at the canonical >8 GiB
guard; no bypass, alternate cache, cleanup or running deployment was attempted.
CI is not waited on. Browser pixels/native mouse gestures/mobile touch behavior
and cross-client shared presentation transport remain separate acceptance.

Initial combined Web patch had an earlier CSS hunk after later script hunks and
was rejected without partial edits. Sorted verified hunks before applying. The
new fixture caught a stale query caused by an older test's direct Set assignment;
normalized that fixture through the actual paint adapter before asserting order
preserves canonical selection. Existing donor anchors were rereviewed in the
focused diff; only affected target/proof hashes were updated. No extra completed
donor-file reviews or whole-project parity claim.

Publication follow-up: the first push failed its TLS handshake through the
configured localhost:10808 proxy. Verified read-only HTTPS through the user's
localhost:8118 endpoint, then succeeded with a per-command Git proxy override;
no persistent Git setting changed. A subsequent network-cloud lookup detected
concurrent Pealayer tracked-inventory drift after the earlier passing checks.
No donor content/source hashes were overwritten or rebound and no further build
was attempted. The next donor consultation must refresh tracked inventory first;
earlier test/comparison counts are checkpoint evidence, not a fresh claim about
the concurrently changed donor. Recovery and source publication are recorded
in issues #8/#3 and PR #1. Feature source is f5940fd; runtime acceptance stays open.

## Six-pixel view and deployment gate

Refreshed all tracked Pealayer paths before the new lookup, including concurrent
source drift. Consulted ui-shell, games-streaming, firmware-hardware and exact
donor-audit sources. Hardware-control/controls indicator and neutral fixed-slot
slices remain screened, NOT complete comparisons. WS2811.h/cpp were reread fully;
actual full-brightness WS2811/TypicalLEDStrip setup stays, BRG acceptance remains
pending. Added WS2811 and hardware_control to specialist routing. Rereviewed
unchanged Pealayer menu/icon/shortcut/appearance/branding/dialog and PU Shell/
Telnet anchors before binding modified target hashes. No new full-file parity.

Implemented configurable Circle/Hexagon/Rounded surfaces, six top-first CCW RGB
sectors and optional zero-based physical indices. New opt-in authenticated status
snapshot reads the committed ESP buffer and physical mapping. No cell-state or
sampler-intent RGB invention, duplicate buffer, continuous UART broadcast, LED
rate cap or per-frame file persistence. Missing/invalid/uncommitted/stale samples
remain unavailable; native USB-only does not borrow another Wi-Fi peer's values.
Immediate cell rendering and partial streamed-frame isolation have actual
LedGrid host tests with Arduino/FastLED doubles.

Web DOM/HTTP tests, shared geometry fixture, appearance/access/display/remote,
branding/menu/capture guards and comparison/preflight checks pass. Both existing
ESP profile caches compile final source; no unrelated AVR/APK/donor build.
Native Test passes donor/crate gates then stops before compilation at the >8 GiB
canonical-cache guard. No bypass, cache relocation, cleanup retry, package copy
or app replacement. ASUS SSH localhost:1022 is refused; no physical deployment
or EEPROM write. CI is not waited on. See CELL-PIXEL-VIEW.md for exact images,
hashes, API semantics, hardware boundaries and remaining acceptance in #8/#3.

The computer-use skill directed the supported browser workflow. Extended the
existing read-only Web fixture with --pixels; no second capture/package harness.
Actually observed Circle and Hexagon, side indices 474–479 after remapping,
amber selection, and Find cell changing to 1. Saved no-overwrite JPEGs through
the browser screenshot API and inspected them; closed the owned tab and fixture.
These prove current embedded Web rendering, not the older running native binary.

Recovery details: incorrect CSS/doc anchors and out-of-order hash hunks were
rejected with no partial edits, then corrected from exact live context/file
order. Comparison guard exposed additional LedGrid and ESP/Telnet target bindings;
actual relevant donors were consulted before updating them. A raw PowerShell
foreach pipeline was a parse error, repaired by collecting results first. The
FastLED test double initially lacked std::max; inspected actual approach() and
added the missing stub import, not a product workaround. The access fixture now
matches the status pathname including its opt-in query, and long-name tests
assert shortened labels plus full tooltips. A getByLabel locator mismatch was
resolved from a fresh snapshot using the observed combobox role. rustfmt on the
root module touched unrelated children; reversed only this turn's mechanical
formatting and used skip_children for affected modules thereafter. No unknown
user/donor edits were discarded or source hashes silently certified.

Final inventory: 1875 paths, 48 completed comparisons, 1827 pending and 29 known
gap files. Refreshing screened evidence does not close their remaining imports.

## Acknowledged peer context checkpoint

Consulted ui-shell, firmware-hardware and exact donor-audit mappings before
implementation. Read PCController peripheral-availability.ts and its test through
EOF, plus all 1279 lines of Pealayer board_info.rs. Inspected status_bar.rs
hardware readiness/optional telemetry slices only; its full-file comparison is
still pending. Added device-context routing and three complete-read comparisons,
with exact omissions rather than a claim of complete application parity.

Fresh manifest ownership now gates native panels, menus, shortcuts, settings,
title, status bar and independently acknowledged Display/Text/Buzzer cards.
Table-only modes are advertised, Snake is a presentation, First wins additionally
requires inputs/outputs. Workspace restoration cannot paint unavailable tabs.
Domain commands, unsent batches, mapping/pixel selection and streaming are guarded
on context loss/switch. Per-peer dock state is bounded memory, not discovery-driven
SSD persistence. No runtime build/profile/cache policy changed.

Passed production protocol/LED/remote/ISP/client/Telnet/busy/audio/identity/access/
async/range/front-panel/PC-profile host tests, the context test including an extra
Table-disabled C++ build, Web context/access/cells/display/remote/appearance,
branding/menu/screenshot source guards, crate boundary, reuse ledger/preflight and
comparison checks. Native source parses with the actual Cargo edition 2024.
Native Test was attempted through the donor-gated wrapper and stops at the same
>8 GiB canonical cache guard BEFORE Rust compilation. No bypass, duplicate cache,
release copy or running app replacement. Runtime acceptance remains open in #10.

Both AVR and ESP busy-wire/polling profiles compile in existing caches. No SDK
installation, clean rebuild, unrelated APK/donor build, physical flash or EEPROM
write. ledboard.local port 80 is unreachable and SSH localhost:1022 refused.
CI is not waited on. Build output and checksum receipt below are static evidence.

The computer-use skill directed actual read-only browser verification of explicit
simulated controller, unacknowledged and Table peers. No board mutation was sent.
Saved three no-overwrite JPEGs, inspected them, and retained small copies under
docs/screenshots. Closed all three owned tabs and stopped only fixture PIDs whose
node command lines were freshly verified. Source remains deployable work, not a
claim that the older native executable or hardware already displays this UI.

Recovery: the first ESP compile exposed String/const-char* JSON-escape mismatch;
consulted firmware-hardware and inspected the actual helper before using c_str.
A status-bar lookup caught concurrent tracked donor inventory drift; refreshed all
tracked Pealayer paths before consultation, leaving donors read-only. PCController
also gained four concurrent tracked paths; refreshed its inventory, not its code.
Hash rebinding preserved existing donor source certificates and unchanged anchor
semantics. Initial evidence hunks assumed field order and were rejected without
partial writes; corrected from exact lines. Oversized combined JSON diagnostic
output was truncated; switched to bounded references/hashes, not larger copies.
PowerShell foreach-pipeline diagnostics were corrected into collected output.
The guessed screenshot test filename was missing; inventoried tests and executed
the actual screenshot_harness_test. Rust parsing initially used edition 2021;
read Cargo.toml and re-ran with its actual edition 2024. No product workaround or
guard bypass resulted from these diagnostic errors.

Current inventory: 1884 paths; 51 complete-read comparisons, 1833 pending and
32 known gap files. See DEVICE-CONTEXT.md and issue #10 for unchecked native,
generic-adapter and physical acceptance. Screenshots are simulated Web proof only.

## Mobile context and RayanLamp continuation

Rechecked the actual Build RayanLamp Go app task read-only; it was not resumed
or messaged. Read current mobile sources, PCController peripheral availability
implementation/tests and Pealayer branding.rs through EOF; consulted device-context,
ui-shell and build-storage. Re-read actual RayanLamp digits.go before Dart reuse.
Extended only those reviewed comparison entries and affected exact proof hashes.
No new blanket certificate or unread-source exclusion.

The mobile README identified the existing C:\development\flutter SDK. PATH
absence did not justify installing/extracting another copy. Added the mandatory
mobile wrapper with existing source/storage gates, resolved packages, canonical
cache budgets and no clean. Analyzer passes and 29 tests execute, including
individual ACK loss, cross-profile identity, queued stale writes, digits/ranges,
shared selection, partial bulk failure and phone/tablet layout. Actual loaded-font
Flutter capture was inspected; complete capture preserves all 80 cells.

Recovery evidence: an initial root-scoped Flutter test found no test directory;
the wrapper now owns mobile cwd and has a regression assertion. New hostile-value
fixture initially inferred Map<String,int> and rejected the test's double before
the parser; changed the fixture to dynamic without weakening parser validation.
Analyzer lint failures stopped before packaging and were corrected; one patch
placed a typedef in the wrong context, caught by parsing and fixed before tests.
Two interaction assertions lacked the actual viewport/stream-settle state; fixed
test setup, not application ACK policy. First capture clipped its last row; kept
it intact and captured a separate full-height image, then inspected both.

Imported storage summary enables StrictMode in callers. Both the lookup and its
test needed explicit @(...Remaining).Count for scalar pipeline results. A Plan
assertion also exposed generator stdout leaking into the result pipeline: wrapper
now emits diagnostics to the host and preserves the one structured Plan result.
Regressions cover these cases; no guard was disabled or bypassed. Evidence patch
generation initially lacked per-line addition markers and was rejected atomically;
corrected from exact anchors, preserving all existing omission decisions.

Warm Android preview build passed; cache grew from 0.971 to 1.021 GiB. Existing
Flutter 3.47.3/Dart 3.13.3 and debug signing template, no dependency upgrade,
SDK download/extraction, extra cache, unrelated firmware/native build or physical
flash. Source master version remains 0.2.7. SSH localhost:1022 refused and
ledboard.local resolution timed out; old native app is preserved. CI not waited on.
MOBILE-CONTEXT.md and issues #2/#3/#10 retain physical/signing/localization/settings
and remaining donor-import gaps. Inventory is still 4237/67/4170/42, not full parity.

## Actual mobile device settings continuation

Consulted preferences, exact Pealayer PreferencesTab donor-audit, RayanLamp
settings/digits audit, device-context, build-storage and capture. Read all 593
PreferencesTab lines through EOF plus canonical native/Web settings, shared
catalog, firmware identity/auth routes and AccessPolicy. RayanLamp's 1827-line
settings.tsx remains pending; it was not certified by lookup or historical claims.
Re-read actual digits.go and invoked its imported map only for numeric fields.

Implemented mobile Wi-Fi, MQTT, outbound WebSocket, full board identity and
access editors, explicit positive ACK/public readback, secret modes, request
epochs, context revocation and acknowledged auth/rename handoff. Added one
complete-read parity-gap comparison, concrete useful omissions and a mandatory
mobile-settings build gate. No generic Pealayer preference mechanism is excluded
as media business. Updated the same ledger/report and issues #3/#4; counts now
4237 paths, 68 complete, 4169 pending, 43 known gaps. RayanLamp remains six reads.

Analyzer clean; 46 Flutter tests pass. Actual loaded-font simulated phone/tablet
captures inspected, zero device writes. First captures exposed a missing standard
MaterialIcons font in the harness; loaded the already bundled font and supplied
actual verified Phosphor caret glyphs. Preserved initial files, captured separate
corrected evidence and verified small repository copies by exact byte hashes.

Recoveries: an omitted test brace failed lint before tests; an import patch without
an anchor landed after declarations and was fixed before testing. Phone dropdown
overflow was a real defect: made it expanded with ellipsis and verified both
widths, without weakening overflow assertions. Delayed-response test callback
setup required an event-loop flush before indexing its pending replies. Guessed
numeric-helper paths failed; inventoried mobile sources and exact donor rows to
find cell_search.dart/internal/i18n/digits.go, not another replacement helper.

Staging succeeded with existing content-equality/file-identity helpers, required
gates, one verified pending copy and recoverable rename of the old preview.
The following comparison generation under imported helper StrictMode exposed
empty specialist route `.issue` access and optional missing `.improvement`.
Consulted build-storage, read the actual generator/test in full, adapted pipeline
enumeration/optional-property handling, added strict-mode fixtures and verified
both full generation and read-only check after report regeneration. No source
guard was disabled, no unread comparison promoted, no blind retry/cleanup bypass.

Warm APK SHA256 8b01492724b48daf0d42ed3666f82144c6e7d6aa55d26614eabd25502f00f596;
version 0.2.7, existing debug signing template, preview only. Prior preview hash
4aabc188bdab5f984b064e17ec417f85e467436d8e08d0eb94eaf984fbbeb1b4 is retained at
outputs/PakarController/mobile/PakarController-preview-previous.apk, not deleted.
Mobile cache 1.021 to 1.027 GiB. No SDK/download/dependency upgrade, duplicate
cache, unrelated native/firmware build, physical flash, running-app replacement
or CI wait. Full omissions/acceptance and screenshots: MOBILE-SETTINGS.md.

## Mobile firmware / picker continuation

Reconsulted updates-telemetry and exact exhaustive picker/helper rows. Fully read
Pealayer ServerFilePicker.tsx and preferenceFiles.ts, native Pakar maintenance,
actual ESP routes/upload/RemoteFirmware JSON and Android MainActivity/manifest.
Added specialist mobile-firmware routing and gate after the audit-only rows were
identified. Actual .bin suffix logic is imported; SAF user-granted navigation
replaces an unavailable ESP server-filesystem contract, not arbitrary path entry.
Concrete remaining provider/localization/generic-consumer behavior stays pending.

Recovery: first analyzer rejected redundant imports/non-null assertion, then an
added callback brace; fixed code without disabling lint. Runtime widget test
found a genuine stale authorization closure: FirmwareController now reads the
current widget callback after capability updates. A perpetual confirmation-time
indicator also prevented settle: separate confirmation lock and actual operation
state instead of weakening the assertion. All 72 tests then passed cleanly.
Hash rebinding patch initially emitted out-of-order hunks and failed atomically;
inspected exact references and applied source-ordered reviewed-line updates, not
a blind all-target recertification. Incorrect relative lookup paths were resolved
with the actual root/inventory; no missing tool/platform conclusion was drawn.

Tablet/phone renderer images inspected; actual Kotlin bridge compiled in the warm
APK build. All gates and source-quality children pass. New documents are selected
by SAF with no broad storage permission; exports validate full backup size and
consume a one-use created-URI token. These are mock/code/package proofs, not
physical provider/board/TLS/EEPROM/Urboot/power-loss acceptance. Exact hashes and
recoverable package rotation are in FILE-COMPARISON-RECEIPTS.md. No CI wait or
hardware write, donor mutation, production app replacement or cleanup bypass.

Final integrated parent regression required a real lock-release fix after ACK
revocation; 73 tests now pass. The fixture's guessed Firmware label was corrected
from the shared generated catalog. A final warm APK build replaces only this
turn's unpublished intermediate after exact owner/pending/rollback verification.
File.Replace null-backup binding failed before mutation; inspected all hashes,
reconsulted build-storage, reread actual stage-desktop.ps1 and used its established
verified pending-copy/Move-Item pattern. Old published settings and earlier
rollback APKs are retained, no redundant intermediate archive or new copy made.

## TM1637 / local CLI / verified desktop replacement

Source checkpoints be41d1af and a6f13a0a fully reread actual PCController
LocalLib/SevenSegments.h/.cpp, firmware_controls_test.cpp and its Arduino
dependency, and actual Pealayer src/cli.rs through EOF. The authoritative
comparison ledger records exact source/target/proof hashes and concrete omissions.
Actual display ACK/recovery tests now execute both fallback GPIO and AVR PC4/PC5
register branches. The CLI imports donor help/version formatting and early-exit
contracts; saved hostname/user defaults are shared with GUI configuration.

Unexpected staging failure: the original package probe invoked --help, which
Pakar mistakenly treated as GUI startup. It timed out, and the surrounding shell
sequence failed to inspect the child staging exit code before using old staging
bytes. Consequently the initial production replacement was NOT a latest-build
deployment. No settings or older rollback files were removed. The mistakenly
launched source executable was inspected for open drafts and gracefully closed
through --quit before rebuilding. The correction adds actual donor CLI output
before config/IPC/GUI/network creation; outer staging now checks its exit code
before any production mutation. Probe timeout child disposal alone is still not
child shutdown; that original donor limitation remains pending.

Guarded release of a6f13a0a3de64fca9420df6938e01e455f08ee8c passed using the
existing warm desktop/target release cache. The built and staged executable
SHA256 is 828d499745c93b580c0cc2b1096bef012017f9b2b280ddb4a04a5d348de5edeb.
Actual bounded --help and --version probes both exited successfully without
creating an app. Then verified current production and the existing rollback
matched one another, verified no owner was running, copied a checksum-verified
pending candidate and renamed only production/PakarController.exe. No extra
rollback rotation or package directory copy. Preferences were byte-identical
before and after launch.

Live production PID 81396 reports version 0.2.7, single_instance=true,
configured_endpoint=http://ledboard.local, no Preferences/settings write draft,
and no acknowledged connected board. --version and --app-status secondary
processes retained that one owner. Internal framebuffer capture completed for
that exact production PID; outputs/PakarController/desktop-current-cli.png was
actually inspected. It truthfully hides table/peripheral/mode cards offline.
It also exposes uncorrected caret glyph boxes, repeated disconnected transport
errors and a Windows-shell initialization error. Those are useful native parity
gaps, not acceptable exclusions or proof of an elegant completed UI.

External Windows WGC capture failed with 0x8007041D, including one fresh-window
recovery; no further retries or stale-coordinate input. The existing imported
internal/Win32 capture wrapper succeeded via framebuffer instead. No blanket
Windows screenshot-unavailable claim and no blank-image acceptance.

CI be41d1af run38100858767 passed workflow/source, both firmware link modes and
Android preview, but the Windows phase rejected a stale SevenSegments.cpp hash
in the older PC contract manifest. Reconsulted tm1637-display and reread actual
donor/target source; selectively rebound that one mapping, documented weak-bias
adaptation and added contract-vs-authoritative-ledger hash agreement. The focused
23-map assertion now passes. No assertion removed or unrelated hash rebound.

Source-stability checks establish the clean compiled checkout, not embedded Git
identity in the executable. Full app metadata/manifest/updater and native UI
parity remain open. Local release execution is not all-green CI or physical
hardware proof. ASUS SSH localhost:1022 refuses and ledboard.local does not
resolve; no AVR/ESP flash or live bus/LED/audio acceptance this checkpoint.

## Actual dropdown/color continuation

Fully read Pealayer dropdown.rs (631 lines), color_picker.rs (446 lines) and
icons.rs through EOF at clean donor HEAD 3265bf80. Import the actual modules,
all three dropdown/four color tests and icon search/list/grid registration hooks.
Read the locked egui ComboBox factory and the actual Preferences Accent branch;
do not certify the entire Preferences dependency from that slice.
Preferences/device-settings/shell selectors and both connection popups now use
the shared focus/search/choice lifecycle. Session recents are temporary state,
not per-click persistence. Busy USB choices and Custom palette have new native
behavior fixtures. No top-menu height change or unrelated board build.

Initial generated patch used a relative path and created one owned dropdown
file outside the canonical checkout; verified its exact transformed donor bytes,
then used apply_patch to move only that content into the absolute canonical
target and remove only the mistakenly created file. No user files removed.
An icon-context mismatch and a truncated shell patch were rejected atomically;
actual bounded source rereads preceded precise replacement patches.
PowerShell R/H aliases invalidated a generated evidence patch; fail-closed
prefix validation rejected it before mutation, and explicit helper names plus
terminating errors corrected the generator. A whitespace-dependent icon
omission match and an incorrect proof anchor also failed before certification;
inspected exact source/anchor and corrected them rather than suppressing gates.

Selectively refreshed only Pealayer inventory and rebound reviewed native/
wrapper target references. Original donor hashes were not blindly updated.
Ledger now accounts for 4242 paths: 91 complete-read hash-bound comparisons,
4151 pending comparisons and 57 known gap files. The older inventory's screened,
stale or legacy hints are not extra accepted comparisons.
Actual mapping and omissions: NATIVE-DROPDOWN-REUSE.md and the authoritative
file-comparison-evidence.json. Dynamic System accent and complete shared
Preferences/menu/board adapters remain pending, not excluded business.

Fresh GitHub enumeration fetched every body/comment/review/thread page for
Pakar17, Pealayer143 and PCController634 records. Read eight additional full
UI/state conversations and rechecked all four unresolved donor threads;
seventeen complete-record and two partial-comment decisions are hash-bound.
Collection of the other records is not semantic review or PR-diff acceptance.
Pakar/PCController have zero open PRs; Pealayer #142 remains under its owner
with conversation read but full diff/readiness not certified here.
Current committed b6bd876 Windows/firmware/Android workflow and CodeQL both
completed successfully. New uncommitted native imports are separate.

Read the exact Build RayanLamp Go app task again without resuming it or
messaging it. Its source has no Git metadata; no verified GitHub repository
can be inferred. Existing local imports remain usable, but a Rayan issue/PR
audit cannot be claimed without its actual repository URL.

Guarded native Test compiled in the existing release cache: 125 passed, three
failed. All three donor dropdown tests, four color helpers and busy-port
keyboard fixture passed. The new Accent fixture chose the closed Custom label
instead of the popup row; its 11-row palette also needed an explicit bounded
height. Scope the target to the measured popup, retain the no-mutation assertion
and verify the actual palette content. The icon test failures were outdated
Seat-in-viewport assertions: reread actual current donor tests, which explicitly
use the visible Folder row after fixed-height scrolling. Import those exact
reopen/filter assertions without deleting their behavior checks.
Reconsulted native-dropdowns and read actual donor Accent interaction tests and
icon test dependencies before correction. A path-before-hash-only mechanical
rebinding missed new JSON references with reversed key order; the mandatory
generator rejected it. Correct only the two reviewed target paths independent
of key order, preserving source hashes and all assertions.

Second Test run: 127 passed; icon cases fixed, Accent fixture still failed.
Read actual locked egui Popup::show and Memory::end_pass: a probe-only frame
abandons the popup. Move root-ID measurement before opening any popup; retain
open-state, value-preservation and actual Shared-colors paint assertions.
Final guarded Test: all 128 pass, zero ignored/failures. The source-quality
phase passed; final focused wiring/menu/connection/lifecycle/API/PC mapping
and generated-comparison checks pass. No test removed, gate bypassed or second
cache/dependency tree created. No additional firmware/APK build for this change.
Five existing/unused-helper warnings are not claimed fixed.

Reread the drifted Pealayer #142 body and all six comments; refreshed only its
conversation decision hash. All seventeen reviewed-record and two comment-scope
decisions now match the refreshed corpus; 775 records remain semantically pending.
Pakar #3 current counts updated without marking runtime acceptance prematurely.
Fresh ASUS SSH1022 still refuses and ledboard.local still fails DNS.

## S3 flow-control lookup and provisional DevKit clarification

The requested uart-flow-control concern was missing and its lookup failed closed.
Consulted firmware-hardware instead and inspected all matched protocol/UART/event
pending actions. Added an exact uart-flow-control route to the same index, including
PCController UartProtocol, codec/runtime and VirtualBoard UART/codec tests. These
files remain pending comparisons, not newly certified donor reads. No replacement
protocol or untested AVR event-flow mutation was introduced after that failure.
The current AVR no-busy build is still polling-only; paired buffered mode remains
an explicit migration gate. S3 pinout now records the user's provisional common
DevKit assumption without inventing flash/PSRAM or partition details, and calls
out the AVR RESET pull-up voltage hazard in addition to TXD/MISO.

## Connection incidents, distinct failures and quiet recovery

Consulted network-cloud, donor-audit Pealayer messaging.rs, web-presentation and
the new error-management route. Read actual Pealayer messaging.rs, ui/error.rs
and network.rs, plus RayanLamp browser-console.ts and full tests. Missing
Pealayer src/error.rs was resolved with rg to src/ui/error.rs, not declared absent.
Pealayer four_d/engine.rs transport-rejection/mark-disconnected slice informs
separate lifecycle versus explicit-operation outcomes; no full-file certificate.

Adapted the existing Rayan logger's keyed mechanism to bounded source/cause
incidents rather than periodic category suppression. Native diagnostics now
coalesce alternating failures, preserve differing causes/routes/ports and every
explicit operation result, count repeats and report source-specific recovery.
Added read-only retry backoff, one in-flight status poll and stream cancellation
on WebSocket loss. No donor mutation, new dependencies or persistent event files.

First browser recovery count assertion saw21, correctly including the prior401;
retained it with the exact explanation. Browser tests then passed. The old
hash-only rebinding regex missed reference objects whose quoted anchors contain
braces, so the comparison gate rejected api.ts's Web target. Reconsulted the
actual API source and guard; used quoted-string-aware reference-object matching
for only the reviewed target/proof paths, leaving all donor hashes untouched.
Regenerated the same ledger and added exact error-management routing; no pending
donor becomes an exclusion or complete read merely through this change. Native
build/live proof is recorded separately in CONNECTION-DIAGNOSTICS.md.

Completed CI run38107886274 exposed a stale realtime-retry VM fixture: its
isolated globals omitted the actual logger now consumed by connectWs.
Consulted web-presentation/error-management, reread the shipped retry/logger/test
and the actual Rayan sync.ts scheduleReconnect/prepareReconnect slice. Load the
real production logger instead of stubbing recover, preserving every existing
session/cancellation/no-replay assertion. Added repeated503 preparation, distinct
500 preparation and matching-handshake recovery assertions. Preparation errors
now use the same WebSocket incident owner, not periodic three-second suppression.
No new full donor-file read certificate from that slice. Rebind only changed
target/proof bytes in the existing ledger; do not alter original donor hashes.

The local quality run then found two older PU route-parity Web target hashes.
Consulted updates-telemetry and reread both mapped target rows and actual line244:
the remote-update section, authenticated root route, capability gate and no-replay
warning are unchanged. Only diagnostic JS changed elsewhere. Rebound those two
target hashes, retained their exact line244 anchor and all50 route assertions;
no donor exclusion or firmware-route behavior was changed.

## Buffered AVR/S3 preparation and actual UART donor review

Fresh source/main clean; prior60d8668 complete CI38140468478 succeeded (Windows,
both firmware variants, Android); f0adf3a still running at snapshot, not waited.
ASUS SSH refused, ledboard.local DNS absent; continue offline as authorized.
Consulted uart-flow-control, exact donor-audit and build-storage. Read complete
actual PCController UartProtocol.h/cpp, uart_protocol_test.cpp and codec/contract
dependencies. Canonical c26cc561 clean; refresh only its1211-file inventory,
preserving twelve earlier reviewed hashes and every other donor row.

Adapted actual bounded RX snapshot and nested/split/noisy test behaviors into
shared SerialLink; cap64 bytes for large S3 rings, reject negative reads/occupancy,
expose existing ESP counters for ESP32, and repair a failed TX boundary before a
later independent frame without payload replay. Partial outcomes stay unknown;
a delimiter may complete a CRC-valid prefix, never infer rejection from failure.
Added mutually exclusive buffered AVR macro/profile, full-frame TX readiness and
failed game-state dirty retention. Reject buffered ESP8266 and preserve busy-wire
default/INPUT_PULLUP, polling and physical pinout. No extra protocol/effects engine.

Comparison gate correctly exposed changed old test/proof hashes. Reread changed
target behavior and existing assertions; only script test invocation/build mode,
AVR dirty handling/flow branch and small firmware CI additions changed. The first
rebinding attempt assumed multi-line refs but one was compact JSON; it failed
atomically. Match parsed selected refs and original whole lines independent of
formatting; rebind only those reviewed target/proof hashes, no donor certificates.
Same ledger now94/4242 complete,4148 pending,60 gap files. Useful zero-copy,
counter taxonomy, timestamp/runtime and physical omissions remain explicit.

Actual focused UART tests pass in three profiles; full source-quality phase passes.
All three final AVR compiles pass:26840/26958/26670 flash bytes (buffered/busy/poll),
1226 static SRAM bytes,32768 merged Urboot images. See UART-FLOW-MIGRATION.md for
exact meaning. Existing desktop/settings hashes preserved; no physical write or
native/Android rebuild. Final partial-write refinement used only focused tests,
same warm AVR caches and deterministic comparison generation, not another broad
GUI/Android suite. Latest source CI proceeds asynchronously.

## Interactive TUI: donor-backed bounded checkpoint

Baseline 5d02b07 Build38141677102 and CodeQL38141677110 both passed;
no open PRs. SSH1022 timed out and ledboard.local remained unresolved. User is
migrating hardware, so no physical upload, discovery or LED/display success.

Consulted real Rayan terminal/run/text/keys/runtime sources and their platform
dependencies through EOF, plus actual PCController table.go viewport formula.
Seven new hash-bound comparisons retain exact consumers and tests, with useful
omissions pending rather than claiming full TUI parity. Crossterm replaces Go
toolkit ownership; selection, Find, incidents and CLI parsing remain shared.
New terminal-ui gate is part of source-quality CI. No GUI owner or settings edits.

Donor inventory drift was detected rather than ignored: reread full changed
Pealayer i18n.rs, web i18n.ts, Windows shell test and CI workflow (including new
macOS diagnostic sampler), plus PCController TonePlayer/ShiftRegisters/
SevenSegments.cpp. Rebound only their original source hashes after actual reads.
ShiftRegisters comparison is now parity-gap: donor's new 328P-only guard is
not silently called byte-identical. Correct PuzzleBoard A4/A5 pins and input
validation remain intentionally stronger; no copied PB3/PB5 regression.

Inventory now4255 files,101 manual comparisons,4154 pending,68 known gaps;
not complete imports. Guessed legacy-review filename failed; rg --files resolved
the actual reuse-review-evidence.csv. A full evidence JSON diagnostic exceeded
output bounds before mutation; limited changes to parsed reference leaves.

Final native Test:147 passed/0failed, including HTTP batch peer-change/false
receipt stop, captured shared CLI output and post-preflight cancellation. Four
bounded native passes reused the same release cache; no Android/firmware rebuild
or debug artifacts. Release, built redirected-terminal guard and current source
quality execution are separate receipts, not inferred from these unit tests.

Full source-quality phase passed. First Release (4m56) and built CLI suites
passed, then a real Windows ConPTY session (unreachable private loopback fixture)
opened Events/Help and q restored raw mode/cursor/alternate screen with exit0.
Repeated connection failures stayed one incident with increasing occurrences.
This session predates the small resize clipping refinement, whose own native
run verifies width0..33/height0..8. It is not board or graphical UI acceptance.
Also wired the existing bounded describe_error cause-chain helper into shared
blocking_status: otherwise the TUI would merge distinct DNS/socket/TLS failures
behind reqwest's identical outer Display. Final Release/built status test verifies
that integration; existing native diagnostic tests cover cause distinctions.

## Native HTTP queue ownership continuation

Exact HEAD 5212914 Build/CodeQL passed before this slice. ASUS SSH1022 refused
and ledboard.local did not resolve; no hardware action or deployment occurred.
PCController session.go/session_test.go were reread fully, matching the existing
1bf4d52/ff6c7b2 source hashes. Admission checks are adapted into a revocable GUI
HTTP generation with shared strict TUI pin validation; serial lifecycle/ACK/
priority/reset and non-HTTP/mobile ownership remain explicitly pending.

First guarded native Test stopped at compilation: main's common peer reset could
not access TableState.stop_pending_edits, which was pub(super) inside ui.
Consulted peer-binding again and inspected the actual existing table reset;
changed only that method's visibility to pub(crate), not its behavior or cache.
No package was produced/replaced, no direct Cargo bypass, cleanup or debug output.
Rereview/rebind affected target hashes before retry; test execution is separate
from source/proof anchors and cannot establish physical acceptance.

Second guarded build compiled;160/162 tests passed. Two new loopback fixtures
failed before sending because their test Wi-Fi password had only seven bytes.
Inspected the actual device_settings::Write::network eight-byte lower bound and
existing fixture tests, corrected the fixtures without weakening validation.
Also retain generic retired receipts before the endpoint envelope filter, so
an endpoint change cannot silently erase the old outcome. Payloads and credentials
are absent from those receipts. Source-quality passed before this final correction;
final native and hash/source guards must be rechecked, not inferred.

Final guarded native Test passed163/163. Focused final living-contract, render
lifecycle, settings, peer wiring, versions and generated comparison checks pass.
No release/deployment or physical success follows automatically from that.

Final RequireLive caught two newly tracked PCController control/mirror_segments
files at clean HEAD38f48d0. All20 compared donor hashes remain unchanged. Refreshed
only PC inventory, left both new files pending, regenerated comparisons and passed
the strict live peer-binding gate. Counts4284/106/4178/73. No bypass or donor edits.

73769c6 guarded Release/stage/probe passed, then atomic File.Replace moved the
staged payload into production with prior B5133DC9 executable retained, no running
owner and settings hash22F8AEED unchanged. Actual PID74392 framebuffer captured
and inspected. It revealed bootstrap configuration reads while offline, producing
redundant preflight/expired events. Reconsulted peer-binding and actual native
poll/admission path; gate settings and remote-job reads on fresh selected Wi-Fi
status, retaining legacy reads. Added source guard. Windows Shell E_FAIL startup
event remains separate production work; tray_available alone is not Shell success.

2bdbf76 adds only fresh-status admission for configuration/remote-job reads.
Its source guard and final comparison/version gates pass; guarded Release passed
in3m15 using the same cache. Final payload25394176 bytes, SHA256
4440B2B24C622A5BF58B5C466369FB621F453870A2E5BCA1D567B0C4DF81E435.
Per user policy no repeated full incremental native suite:163 tests passed on
73769c6; final small admission correction has source guard/build/live follow-up.

Stage stopped on Pealayer tracked drift, not a compiler failure. Reconsulted
build-storage; all43 compared Pealayer source hashes still match at clean
fbf2d162073fe6171cb17ba94ec68c3b202911db. Two added paths and one removed
generated bundle; refreshed only that inventory and left additions pending.
Current4285/106/4179/73. The subsequent ledger-only commit changes no compiled
desktop inputs; binary build identity remains2bdbf76, not the documentation HEAD.
Final output Native-HTTP-Ownership.json records actual replacement/capture status.

## ISP GET ownership boundary

Fresh c1517b2 Build and CodeQL both succeeded; Pakar open PR listing was empty.
ASUS localhost:1022 still refused SSH and ledboard.local did not resolve. Actual
desktop PID79784 was left running unchanged; no draft, settings, rollback or
physical firmware replacement occurred.

Consulted all firmware-hardware rows (88 matched/70 pending/11 gaps) and the
peer-binding specialist plus exact PCController donor-audit row. Reread complete
session.go and session_test.go through EOF; original-byte hashes remain
1bf4d52d6810630d1fd4d5f0a0c18254ccdd73c0942dbe94f4c97a06360680c5 and
ff6c7b21f72bfb944d232c87210f8ffa139dfe016e8b5756f2582604489fffc4.
Adapted admission/dispatch checks into all-method supplied HTTP epoch validation
and the real loop authenticate boundary. Hardware-affecting GET probe is no
longer exempt. Header-free compatibility remains explicitly unfenced; this does
not establish AVR-only identity, remembered pairing or non-HTTP ownership.

Comparison generation caught stale tools/test-protocol.sh after adding the new
test. Reviewed its exact additive diff and existing display runner anchor, then
rebound that proof reference without changing donor hashes or closing omissions.
Quality subsequently stopped at pu_routes_test.mjs on the changed firmware hash.
Reviewed both shipping diffs and affected route anchors; updated six existing
PU850 target hashes/line numbers, including previously stale Markdown anchors.
No generic route classifications, source hashes or check strength were changed.

Actual WebAccess middleware and AsyncHttpService queue execute in C++ using
transport/auth/SDK doubles. Stale/partial/malformed pairs fail, matching pairs
dispatch, legacy is explicit, auth/origin rejection precedes identity checks,
and retirement between enqueue/dispatch never reaches the hardware-call double.
These are not Basic-auth parser, SDK entropy, LAN or physical ISP proof.
Guarded peer-binding RequireLive and shared C++ protocol checks passed. Only
integrated ESP8266 busy-wire firmware was rebuilt in the existing warm cache:
945392 bytes, SHA256 CBF2F9A2108CE47C4CAA9EC337296F8F591ED0E2407E2177F431449E865C3EE4.
RAM56620, IRAM63055 including32KiB instruction cache, flash code895328. No native,
AVR, Android or donor rebuild; no clean/debug/incremental cache was introduced.
All source versions remain0.2.7, not proof of deployed version consistency.
Quality checks before the stale route failure passed; remaining checks resumed
from that exact failed step and all succeeded, avoiding repeat suites/compiles.
Ledger remains4285 inventoried/106 comparisons/4179 pending/73 parity gaps. This
bounded safety adaptation is not complete donor parity or production acceptance.

## Cooperative Tasks and Prog reset ownership

Fresh exact79c28571 Build38166374087 and CodeQL38166373952 both completed
successfully. Pakar open PR listing was empty. SSHlocalhost:1022 refused and
ledboard.local did not resolve; no board action or app replacement occurred.

Consulted firmware-hardware and exact donor-audit Tasks rows, then fully read
PCController LocalLib/Tasks.h/cpp and ControllerBoardMini Puzzles/LocalLib and
Timer/LocalLib Tasks.h/cpp. All six original-byte hashes match live donors; the
PCController checkout is clean38f48d06f6922225e9a100d0afc2d787b5a000bd.
The exact hashes, targets, omissions and next physical actions are in the SAME
file-comparison evidence, not a second inventory or a claim to have read others.

Found actual eight-slot PCController-derived code already present in BoardCore
but unconsumed/untested; earlier Puzzles ledger text incorrectly called it absent.
Hardened it in place instead of duplicating it: ten slots from Puzzles/Timer,
explicit occupancy/context/retirement from PCController, bounded eligibility
mask, cancellation/clear/recursive-update protection and half-range deadlines.
Actual AVR loop consumes it for one100ms Reset callback. Entering Prog cancels
that pending callback; dispatch checks the latch again before watchdog reset.
ACK remains scheduling acceptance, not reboot or completed programming proof.

Mechanical evidence patches initially failed context matching without altering
files. Reviewed exact ledger contexts, applied ordered hunks, and rechecked
affected AVR/CI/protocol-runner anchors. No stale donor source hash or failed
gate was bypassed. Added cooperative-tasks routing and mandatory test/build gates.

Guarded production C++ Tasks tests, actual sketch source wiring guard, strict
RequireLive six-file donor gate and complete source-quality phase all passed.
All affected AVR profiles compiled using existing warm caches (no Clean/debug):
busy-wire program27656/RAM1317, polling27356/RAM1315, buffered27538/RAM1317.
This leaves731-733bytes for stack/locals, not a measured runtime high-water mark.
Each with_bootloader image is32768bytes; SHA256 by mode:

- busy-wire:4577A98CE1FFD124FF5A41C8858637C412BF765A2527EFA2D4683605E9B7F1C6
- polling:78E50DE1F66F10BEFE4F3546139A4712D97101E3EF4C32D23684F4B5932A1201
- buffered:307365D1CBEFE8BDA3B163569B39BD266CED9DDD6D6B51AA5890AD95727B7261

No ESP/native/Android/donor rebuild or deployment; source versions remain0.2.7.
Actual clock-double scheduler tests do not execute AVR watchdog/ISP or establish
physical service latency, welcome timing or wiring. Reusable slot handles require
owner discipline; callbacks must return promptly and API is loop-only, not ISR.
Ledger now4285 inventoried/110 comparisons/4175 pending/71 known parity gaps.
Physical stress/reset/Prog acceptance and remaining production issue2 stay open.

## GUID-bound tray tooltip recovery

Exact9ac741ab Build38170618852 and CodeQL38170618787 completed successfully;
fresh Pakar PR listing is empty. ASUS SSH1022 refuses and ledboard.local is
unresolved. Actual production PID79784 remains; current IPC reports Preferences
closed/not dirty, mapping/settings closed and no pending settings/maintenance.

Consulted windows-desktop plus donor-audit. Initial exact src/tray.rs lookup
failed closed because that donor path does not exist. Located inventoried
src/platform/windows.rs instead; did not manufacture a mapping or skip import.
Fully read donor taskbar/tray plan and WINDOWS-SHELL-RECOVERY.md through EOF.
Read actual windows.rs2364..2421 registration and2483..2513 tooltip sections;
complete large-file review remains pending, NOT promoted by this focused read.
New tray-tooltip route exposes that exact pending source and both full-read docs.

Actual pinned tray-icon0.26.1 Windows implementation registers/apply_guid on
creation/icon/disposal but omits it in set_tooltip237..269. Microsoft confirms
subsequent Shell calls require the original GUID. Adapt Pealayer direct
NIM_MODIFY/TIP to existing HWND/shared GUID and UTF16 helper, preserving crate
menu/RAII/restart ownership. No dependency-source patch, upgrade or second icon.
Failed presentation is not cached; bounded latest-value retries feed existing
incident dedup/recovery. Observed TaskbarCreated invalidates cache after the
crate's own recreation, not an inferred successful visible restart.

PowerShell rg wildcard path was not expanded; resolved actual registry directory
read-only before inspection. One generated evidence patch used relative paths
at tool cwd and failed without modifying files; changed to absolute repository
target. A caller tested stale LASTEXITCODE after a successful PowerShell generator
and falsely reported failure; no generator/gate change or blind compiler retry.
A live-check call used nonexistent SourceRoots; read parameters and used default
DonorRoots. Correct strict live tray gate subsequently passed with zero stale.

Reviewed actual shipping diffs and rebound all eight affected main/shell/hook/CI
target references, preserving donor hashes/decisions and anchors. All45 compared
Pealayer raw hashes still match clean8337a722b0c93bd393b49d4c9ca3b5ea64ae9c32;
selectively refreshed only its521-file inventory, with no additional source read
or false pending promotion. Same ledger4285/112 comparisons/4173 pending/73 gaps.

Guarded native Test passed166/166 in the existing release cache (1m21compile,
0.26sec execution); production retry/NOTIFYICONDATA/UTF16 fixtures are included.
Full source-quality phase passed. No firmware/APK/donor rebuild or clean/debug
cache. Versions0.2.7 remain source identity, not physical deployed consistency.
Real package replacement/Shell acceptance, Explorer restart/visible action
acceptance and all board/full donor production gates remain separate evidence.

3fa9bc676f813c1b0050db24a2bd95ebf5e7069a guarded Release passed in1m58, then
stage/source-stability/CLI probe passed. Replacement guard initially compared
forward-slash input with backslash process path and stopped BEFORE quit. Applied
IO.GetFullPath/OrdinalIgnoreCase to both verified identities, rechecked current
draft/pending state and gracefully quit PID79784; verified exit, no forced kill.
Atomic File.Replace moved the staged payload to production and preserved old
4440B2B24C622A5BF58B5C466369FB621F453870A2E5BCA1D567B0C4DF81E435 executable
as PakarController.before-tray-recovery.exe. Saved preferences remain unchanged
22F8AEED4BF0CEBC3116C670F681D90A3D1B1B833ECE3DE23A491D0D896A8711.
Older rollback packages remain untouched; staging executable no longer exists.

Actual new PID79484/version0.2.7/25414656bytes, production SHA256
8F433FFAE3A812703CCD6BFBD6CB86C9ECCF8FD4ACBFCD104BE187879CD53EC4.
Fresh IPC reports tooltip_applied:true, attempts:1, explorer_epoch:0,
last_error/retry_ms:null. Later snapshot retains one attempt, while network
errors coalesce independently. This is actual Shell NIM_MODIFY acceptance, not
visible icon or Explorer restart proof; no restart was induced.

Captured and visually inspected actual PID framebuffer1999x1453. Screenshot
SHA256 B292582543B369A3E17972CA382D0AC9E610C705BC8C98C722A2E9C5DAAA9910:
offline Events workspace shows separate HTTP DNS/timeout and repeated WebSocket
DNS, no tray startup error or invented board cards. Small user-facing
outputs/PakarController/Tray-Recovery.json records source/package/capture proof.
No simulated or screenshot observation is physical board acceptance. New CI
remains asynchronous; production issue2 and visible shell gates stay open.

## Android device/network workspace checkpoint

Actual missing feature found: previous mobile firmware actions existed, but
Android only had one configured endpoint with SSDP fallback, not a saved device
catalog, DNS-SD/MNDP scanner or Android-owned MQTT/bridge service. Fully read
PCController Tools/Controller/internal/discovery/discovery.go and PU850
LocalLib/MNDP.h/cpp, NetDiscovery.h/cpp and SSDP.h/cpp; the three selected actual
implementation mappings were added to the SAME file-comparison-evidence.json.
NetDiscovery's old endless example loops and wider SSDP dependencies are not
certified whole-file imports. The new mobile-network gate pins the selected
source/target/test hashes. Imported Pealayer file picker/SAF semantics, Phosphor,
Vazirmatn and peer-context gates remain; no invented file or firmware APIs.

Preflight first found Pealayer inventory drift to 555 tracked paths at
48fd59ce65022b039d28570288ff967bf52bca8e, then refreshed to 564 paths at
290f85f352668e556bb535ae6eb38ba5e1a7c13b during concurrent donor work. Fully reread current PreferencesTab.tsx,
board_info.rs, capture-visible-window-windows.ps1 and appearance.ts. New
appearance animation/density/font fields/helpers are explicitly pending, not
claimed imported. Ten other changed already-compared files retain their prior
immutable source snapshots, with the current live hash and reread action in
each ledger note. Their current live-source gates intentionally still reject;
old snapshot comparisons are not fresh donor parity. All source/target anchors
are regenerated, including six early sketch-line shifts in PU850 route mapping.
4328 inventoried /115 snapshot-bound comparisons /4213 pending /76 known gaps
is not complete imports or superiority. Existing screenshots/older proof survive.

Verified: mobile analyze no issues, complete 79-test suite, then all six focused
bridge/discovery/export tests after changing generic POST arguments to form
bodies (never credential-bearing URL queries). Tests execute real loopback
HTTP with bearer refusal, same-ID no replay and board/boot headers; they are
not a real ESP, MQTT broker, TLS handshake or Android background acceptance.
Full production C++ protocol suite including bounded MNDP packet passed.
ESP8266 busy-wire integrated build passed: RAM56684/80192, IRAM63055/65536,
flash896388/1048576. Final image SHA256
3debe61b5d19e94716776c172318a09a228b2b23fc3477009dfbf075b8ea8e70.
Discovery retries after failed mDNS are spaced by ten seconds so loop retries
cannot repeatedly tear down/respond/announce the new MNDP service every frame.
AVR hardware UART, confirmed TM1637 A4/A5 and busy/polling choices were unchanged;
AVR/native/S3/donor sources were not rebuilt or deployed.

Rendered and inspected explicit simulated1200x1100 BridgeTablet via the existing
mobile capture wrapper; no fixture mutations. Canonical release APK build passed,
including native Android NSD. Source versions remain0.2.7. Only the existing warm
mobile cache was used; no clean/new SDK/debug build or production app replacement.
No authorized ADB device is connected. Master issue2 physical/production gates
stay open, including integrated S3, LAN phone advertisement and foreground-service
ownership; current bridge stops when page/app backgrounds or peer expires.

Source-quality phase passed through PU route parity after re-review/rebinding,
then reuse/ledger/comparison/version/preflight checks passed. The broader
build-storage test is blocked by the actual changed live Pealayer src/ui/i18n.rs
review gate; it was NOT bypassed or reported green. Remaining source-quality
commands and CI are separate from already-passing mobile/C++/ESP checks.
The remaining quality commands after that blocked storage check passed separately;
the entire phase is still not reported green.

Final safety review reread actual PCController session.go (879 lines) and
session_test.go (788 lines), original SHA2561bf4d52d.../ff6c7b21... unchanged.
Adapted synchronous owner retirement into mobile HTTP/WebSocket board/boot
acceptance and root state revocation; invalid/partial or vanished learned identity
cannot silently regain legacy readiness. Retained firmware dialogs now lose
authority on a same-host replacement/reboot. One-second bridge freshness watchdog
is independent from a user-selected long telemetry interval, and late MQTT
callbacks cannot publish through a replacement client. Actual mapping/proof was
added to the existing PCController session.go ledger entry, not a second ledger.

After the same-host safety addition, the final guarded analyze and complete
80-test mobile suite passed. Final guarded release APK passed (55215657 bytes),
SHA256 e11c9b392a9856f5fbd2d7d20c4cb97c5809bfdd1b4848a0fd1514ab0b6bb850;
identical delivered outputs/PakarController/mobile/PakarController.apk verified.
Package contains assets/flutter_assets/NOTICES.Z. This is debug-key-signed preview,
not production signing or an installed phone test. Simulated screenshot86160bytes,
SHA2566ad4ade13a464c4a29a92fc0818a04dc36604aaba373eee07f2feec50fd140b2,
also retained in docs/screenshots/mobile-bridge-tablet.png. No source screenshot
claims Android NSD/MCU/peripheral acceptance or a completed production release.

## Integrated ESP32-S3 source and current-donor reconciliation

The S3 migration keeps the canonical integrated sketch, update/ISP state machines
and 480-pixel LedGrid/FastLED engine. Private FSPI owns AVR ISP GPIO11/13/12 with
reset-driver GPIO10; the unverified electrical/polarity gate defaults closed.
Hardware UART1 owns AVR TX17/RX18 at 38400 with bounded buffers. AVR hardware
UART and TM1637 A4/A5 remain unchanged. Shared GPIO4 matrix output has no hidden
frame-interval cap; a separate semantic RGB writer does not duplicate effects.
Actual FastLED IDF5 RMT implementation was read, but neither DMA operation nor
frame rate, UART losslessness, electrical ISP safety or visible output is proved
by those API/source observations.

Fully read actual PU850 FirmwareTransfer.cpp/h were added to the SAME comparison
ledger with exact source hashes, retained Range/HEAD/ETag/MD5/auth mappings and
bounded production-parser/source-guard anchors. S3 downloads are relative to the
running application partition, never raw flash offset zero. OTA capacity uses a
distinct next application partition and explicit Update.abort on cancellation;
the fail-closed remote staging/journal/backup workflow stays shared. Full-flash
network downloads remain disabled to protect credentials. ESP32 image validation,
mbedTLS hashing/CA trust and Update APIs replace ESP8266-specific APIs; this is
not a claim of power-loss-safe rollback or real OTA/AVR flash acceptance.

Current installed Pealayer inventory was selectively refreshed by the root task.
Actual changed compared sources were fully reread rather than bulk-stamped:
native dropdown/color-picker/icons, dialog/menu, i18n, branding, status-bar,
Web appearance/i18n/Preferences/file-picker/header/browser-platform, Windows
package script and workspace hygiene guidance. The status/front-panel agent
completely reread dialog/menu and their actual Pakar targets; network and status
agents completely reread the physical RGB status-bar donor. Existing source
comparisons and target/test meanings survive. New generic behavior is explicitly
pending, including density-aware controls, bounded diagnostic details, native
font-family controls, fixed close/toggle/confirmation focus helpers, endpoint
trigger geometry, HWND/preset icon lifecycle and browser/PWA capabilities.
The earlier MessageHub byte identity remains recorded as historical, not current
equivalence. Source re-review is not implementation or whole-donor parity.

Three actual PCController StatusLedMath/StatusLedController mappings were added
with exact hashes and bounded S3 semantic RGB/test targets. The native/Web mirror
uses actual already-scaled output bytes only with fresh peer/have_output guards;
it does not multiply brightness twice. AVR PWM/EEPROM profile storage is replaced
by the S3 addressable output and existing persistence ownership, while arbitrary
profile authoring/manual overrides/cue priorities remain pending. The changed
PCController status-led-event.ts complete ordering adapter is NOT certified by
these three source imports. Native/Web rendering and optical/audio acceptance
are separate; the mobile generated settings catalog alone is not a mobile
addressing editor or physical RGB mirror. TUI DHCP/static editor parity remains
pending too.

Read actual canonical-main/platform diffs before rebinding retained auth/shell
anchors. Live api-contract, build-storage, native-dropdowns, peer-binding,
web-presentation and web-streaming gates passed at their reviewed frozen hashes;
peer-binding main hash f08f05f2633775d99d3d47d7d9415d77e541f225cb9834f2beb33b9ec50561c4.
The root task solved Arduino include preprocessing by removing literal quotes
from the expanded -I root property, retaining both C/C++ recipe flags; S3 build
uses -Os -g0 to avoid unnecessary debug artifacts. Subsequent root-owned SDK
compile diagnostics are not hidden or replaced by source checks. No hardware
flash/read/write or native build was performed by this evidence task.

Verified here: focused s3_platform_wiring_test.mjs source guards, strict
generate-file-comparison.ps1 -Check and git diff --check. The generated reports
account for 4403 inventoried files, 126 hash-bound manual comparisons, 4277 still
pending comparisons and 90 known parity gaps. No compared current donor source
or target/proof hash was bypassed. Test-code anchors do not mean tests ran;
current SDK/native build, actual hardware deployment, SSID association/discovery,
matrix/peripheral behavior and production acceptance are reported separately by
their owners. Keep issue #2 and all applicable physical acceptance gates open.

The actual PU850 SSDP.h/cpp and SSDPDevice.h/cpp are now mapped in the same
ledger too. The network agent fully read all four original files and adapted
the complete device engine; the evidence owner reread the small wrappers and
the complete source/header adaptation diffs. Delayed bounded/de-duplicated
M-SEARCH, alive/update/byebye, topology/identity and escaped descriptions are
retained, with instance-owned S3 sockets and AP/STA interface membership replacing
the ESP8266 singleton/manual IGMP policy. This is actual donor code, not only a
Wi-Fi SDK call. Real LAN/router/AP/phone discovery remains pending; optional
unimplemented SOAP/GENA is not advertised. NetBIOS remains truthfully false on
S3 rather than reported as available. CI target references were rereviewed for
the added pinned S3 core/AsyncTCP, buffered build artifacts and focused source/
network checks; that source update is not a claim that CI has executed.

Subsequent legitimate SDK diagnostic found RemoteFirmware's WiFiClient alias
was not transitively declared by ESP32 HTTPClient/secure-client headers. Read
the actual installed 3.3.12 WiFiClient.h (NetworkClient alias), added its explicit
include only to the ESP32 branch, and added/passed a focused source regression
assertion. This fixes the observed declaration error without replacing the
shared updater or declaring the next SDK build successful. Refreshed main target
bindings after reviewing the added status payload bytes38/39/40 (configured
station/AP/DHCP distinct from observed link state) and atomic USB provisioning
candidate/save/commit with staged secret retirement. Current reviewed main is
4b60de4011dac897ede9983b042e2902246ad819f16c95c3cea697ce7ce3964a.

Network agent's separate guarded native receipt: Check passed, 169 Rust tests
passed with zero failures, Release passed from the single warm release cache,
stage-desktop.ps1 -Probe passed and candidate --version reports0.2.7. Staged
candidate outputs/PakarController/staging/PakarController.exe is25549312bytes,
SHA2567666716054c1d3e5fabe38a8ca468e356e95a72af17d5f6eb13ff09e6fef8dc9.
This is actual native build/test/stage evidence, not a production GUI replacement,
live peer test, screenshot acceptance or physical board proof. The SDK build and
device deployment remain separately owned and must be reported with their own
final result, source identity and live observations.

The next real S3 SDK diagnostic exposed the AVR-only AutomationStore EEPROM
implementation being compiled on ESP32 because its previous guard excluded only
ESP8266. Fully read actual donor Project/AutomationStore.cpp/h and EEPROM layout,
full current targets and adaptation diffs before extending the guard to both
ESP families and their ARDUINO_ARCH macros. This retains the AVR/native fixture
owner, shared record contract, dual-bank/CRC/readback validation and nine-key
adaptation; no ESP EEPROM shim, local rule migration or hardware write was added.
Actual g++ syntax-only checks pass for four excluded ESP macro variants with no
EEPROM/Arduino include paths, and for included HOST/__AVR__ fixtures. Focused
S3 source regression passes too. Common-library scan found no other EEPROM or
AVR-only includes in the remaining C++ translation units; this does not predict
all future SDK errors. Root owns the resumed full SDK build result.

Added the fully reviewed AutomationStore cpp/h to the same evidence ledger;
strict generation/check and diff checks pass with4403 inventoried/128 hash-bound
comparisons/4275 pending/92 known parity gaps. Reviewed test-protocol wrappers
retain all prior cases and add panel/status/config/provisioning fixtures;
refreshed only their changed target proof and passed the live tm1637-display
reuse gate before the owner resumed remaining AVR checks. Existing S3 migration
documentation changes introduced no stale comparison references.

Optional TFT/W5500 backlog is durable in GitHub issue19 under issue2. Checked
existing issue titles/bodies first; applied existing labels using only a scoped
owner credential when the active account lacked label permission, without
changing global authentication. The S3 status/front-panel document links it and
separates completed source callback/configuration wiring from still-unchecked
SDK/deployment/key/display/audio/optical acceptance. No TFT or Ethernet driver,
link, pin-voltage approval or hardware success is implied by reservations.

## Final S3 identity and quiet-link review

Root reported the first actual integrated S3 SDK build passed: app1841104bytes,
SHA256834fe8da7e35967c4c68c53b7887549b8e11679b4656faab2d68d001e15a0f56.
Its authorized COM22 deployment verified written hashes and USB HELLO0.2.7 plus
41-byte status/CRC; AP started, station association and NTP were subsequently
observed, and ledboard.local resolved from ASUS. These are root-owned observations
of that earlier image, not acceptance of the final source or the AVR. AVR RX
bytes remained zero and the paired buffered AVR image has not been flashed.
User reported an AVR welcome/reset loop and later that it stopped, without
changing wiring or grounding AVR RESET. The cause remains unknown; the S3-only
bootloader pause is a temporal correlation, not a wiring fix. VCC is user-confirmed3.3V.
No further photos or PBX calls are requested; all announcements remain stopped.

Actual AP-only HTTP commissioning exposed an empty board identity from the
inactive station interface. Reviewed the final source fix and fully read actual
installed S3 SDK esp_mac.h: esp_efuse_mac_get_default reads the factory-programmed
MAC; it does not write eFuses or mutate the base/interface MAC. Existing zero/
broadcast/multicast checks still fail closed. Stable hardware identity therefore
does not depend on enabling STA, while boot identity and peer-bound request
checks remain separate. Fully reread the unchanged actual PCController session
test file (788lines, SHA256ff6c7b21f72bfb944d232c87210f8ffa139dfe016e8b5756f2582604489fffc4)
including close-before-notify, pending acquisition, inactive control lines,
one-shot reset hooks and correlated/error-response behavior. This is source
review, not execution of donor Go tests or permission to pulse hardware.

Other reviewed final changes: truthful buffered link-mode JSON; station-selected
IP with AP fallback; clear ISP capabilities when the electrical gate is closed;
no ESP32-specific18ms matrix/normal-cloud delay inherited from ESP8266; optional
S3-only PauseAvrLink build flag. Quiet-link source does not begin AVR UART, leaves
TX/RX as inputs and rejects normal sendToAvr calls. Production defaults still
enable the link. Final normal builds omit the already-used one-shot filesystem
initialization flag; onboard RGB GPIO48 is an explicit common-DevKit assumption
to be configured through its existing API, not claimed optical identification.

Read the complete build-script diff and updated S3 source guards, verified every
surviving target anchor, then rebound only actual changed comparison references.
Final reviewed main SHA256eb0de02bb00252adfc89fc4431acfb8d9f88195238c237a81113888776fc6972;
source-guard test SHA256f3c253d28d73a419928f4f454a6f0cc3363177574fae47b7267666c24aafaaff.
Source guards, strict comparison generation/check, diff check and live
api-contract/peer-binding/web-presentation/web-streaming/uart-flow-control gates
all pass. Final exact-source SDK build/deployment, live fenced mutations,
WebSocket/client rendering, AVR link/peripherals, RGB and matrix/OTA/ISP remain
separate acceptance steps; prior image observations do not close them.

## First physical S3 commissioning: bounded evidence and limitations

Root's actual first integrated SDK report: program1840959/3342336bytes,
globals74892/327680bytes; emitted application1841104bytes,
SHA256834fe8da7e35967c4c68c53b7887549b8e11679b4656faab2d68d001e15a0f56.
On COM22, esptool wrote and hash-verified the four S3 ranges for bootloader,
partition table, boot_app0 and application, then reset the S3. Hardware MAC read
90:70:69:ea:c8:00. Original complete16MB flash backup was independently retained
and verified on both PCs outside Git, SHA256
cd0f30ac9491da499eaf2167f07b5accf23cf6779c506110431aa766f346273e.
These are S3 writes only, not AVR deployment or successful AVR communication.

USB readback had valid COBS/CRC, ESP role/version0.2.7 and41-byte status.
Private station provisioning used one explicit commit with AP+station flags;
persist ACK and configured-state readback succeeded. Subsequent ASUS observation:
ledboard.local resolved10.252.139.27, ping and authenticated HTTP Basic worked,
DRS Tether was connected at reported RSSI-48, NTP synchronized and AP started.
mDNS/SSDP/MNDP only reported started at this checkpoint: no discovery query,
multicast interoperability or phone/AP acceptance was proved. No private Wi-Fi
secret is recorded here. AVR received0bytes/0frames, version was null and
TM1637 ACK was false; no UART, display, keypad or buzzer acceptance follows.

Initial AP-only commissioning also exposed zero station-interface MAC/empty
board identity, false HTTP peer-fencing readiness, polling text for the buffered
profile and untruthful ISP advertisement. Source fixes described above are bound
to main eb0de02bb00252adfc89fc4431acfb8d9f88195238c237a81113888776fc6972;
their subsequent exact-source SDK/deployment result must be reported separately.
That normal UART-enabled build was in progress, not a finished firmware claim.

Clarification: the user did not change wiring and NEVER held AVR RESET to GND.
The later report that the problem stopped followed root placing only the S3 in
its ROM bootloader; this correlation is not an established cause or a physical
wiring fix. Root used no erase for that pause. No AVR ISP/fuse/flash/EEPROM or
reset-control write occurred. Keep all PBX calls and test melodies stopped.
The next intended integrated image pauses AVR UART entirely for quiet diagnosis;
normal UART-enabled firmware is not to be deployed as an assumed remedy.

Both PCs run the native candidate hash
7666716054c1d3e5fabe38a8ca468e356e95a72af17d5f6eb13ff09e6fef8dc9,
and captured screenshots are genuine offline application views, not a live AVR
or peripheral test. The reuse source gate remains128 bound comparisons with
4275 pending comparisons; this commissioning evidence does not close donor
parity, physical AVR, final firmware or production acceptance.

## Passive UART SDK design review only

Read actual installed Arduino-ESP323.3.12 HardwareSerial begin/end/setPins,
HAL pin validation/attach/detach/driver initialization/end, inversion and RX-pull
function bodies, plus applicable IDF UART/GPIO declarations. This was a targeted
API review, not a claim to have imported or fully read every unrelated line in
the large SDK files. Original-byte SDK hashes:

- HardwareSerial.h:f0a25409647e9c5daf3cb9b59f5d7020a422962aab7afe7cb0e637e510af92ac
- HardwareSerial.cpp:df718558a059669e35b1c034d6e458b00d13317ec8403e5a74dd9a84898544bf
- esp32-hal-uart.h:7de851dbb9d263cf80d5c7fe547a6e3055b863250900f5f2d1a522968231fca1
- esp32-hal-uart.c:e6ec2520ffa76edba0170694bc984cdad74d47d64b23a60cec0c1396d8363994

Negative TX means retain an existing TX assignment, not detach. On a fully
detached UART, explicit RX18/TX-1 avoids default pin assignment because defaults
are chosen only when BOTH effective pins are negative. HardwareSerial::end and
HAL uartEnd detach registered UART pin matrix/ownership and clear stored pins.
There is no ordinary UART_RX_ONLY mode in these actual APIs; IRDA_RX changes the
wire protocol and TX-buffer-size0 only selects blocking transmission, not a TX
disable. Neither is a substitute for keeping the physical TX pad as input.

Recommended, not implemented: a dedicated spare UART2 capture owner only in
quiet-link builds, no write/HELLO/actuator API, no normal SerialLink callback or
automatic resume, bounded explicit38400/115200 capture and optional RX inversion.
Using UART2 avoids the normal UART1 avrLink.service draining diagnostic bytes.
Require UART2 TX pin unassigned before/after begin, GPIO17 input/no pulls with
public gpio_get_io_config output-enable readback, and no GPIO10 operation.
Disable optional internal RX pull before begin for passive observation; fail
closed and end capture on any ownership/driver/pin-readback disagreement.
No diagnostic production code, COM access, waveform, reset or AVR command was
performed by this review. Software pin-state checks still are not electrical
measurement or proof that the unknown AVR firmware will transmit spontaneously.

Separate status-agent compatibility receipt: guarded ESP8266 busy-wire SDK build
at reviewed main eb0de02 passed, binary958848bytes,
SHA256165537f5c8552c9e34ee1411793f671b2903146c3b4e8667a56511d3b98fa236;
RAM56936/80192, IRAM63055/65536 and IROM908640/1048576. This preserves the old
target as a built option; tight8266 headroom and all physical acceptance remain
visible. It was not deployed and does not replace final quiet-S3 build evidence.

## Approved bounded passive capture source

After the preceding design review, root explicitly approved the new narrow
firmware/esp32s3/src/PassiveAvrRx.h owner and pure production-helper test, not
main/Protocol changes or hardware access. The adapter exists only for S3 quiet
builds with PAKAR_AVR_LINK_ENABLED=0 and references spare Serial2/UART2. Its
constructor starts no driver/GPIO. Explicit begin accepts38400/115200 only;
requires unowned RX/TX/flow-control assignments and GPIO17 input/output-disabled
readback, disables optional RX pull before attachment, uses RX18/TX-1 only after
that detached check, then verifies driver/RX18/input/no-TX readback again.
Failure ends only its owned driver. There is no write, reset-pin operation,
normal frame decoder/callback, auto-sweep or normal-UART resume path.

The actual shared helper stores at most128bytes, counts received and overflow
drops independently, bounds chunks to32bytes/offset128, and preserves captured
bytes after explicit stop, unsafe-state stop and wrap-safe8000ms expiry.
Error callbacks update a separate atomic count; cooperative reads have a64-byte
per-loop budget. The root owner must continue servicing during capture: expiry
is enforced in service(), not claimed to be a hard real-time interrupt deadline
if unrelated code blocks the loop. The adapter has no remote HTTP API; explicit
USB start/status/chunk integration remains root-owned and was not added here.

Actual g++17 -O2 -g0 execution of tests/s3_passive_avr_rx_test.cpp passed exact
capacity, independent drops, invalid chunk/null limits, duplicate/invalid begin
preservation, stopped-buffer retention and deadline rollover. That test compiles
the SAME pure helper, not fake SDK GPIO/UART implementations; actual adapter
SDK compilation, GPIO readback, bytes and safety under hardware remain pending.
Header SHA256037a61bd063248358093549efef01479670da34695a3e1b26088c0b29d4c4662;
test SHA25641072f11f04a65099ee2a2d75ebf66e4d9370e055ec9e4a4c1c93cc8f3f6d065.
The existing uart-flow-control reuse gate was consulted before this work; no
SDK API excerpts are falsely classified as a new complete donor-file import.
No donor, main, protocol, serial port, audio or PBX mutation occurred in this slice.

## Quiet USB integration source gate

Reviewed root's frozen canonical main
e19a7c10fe7e0bcd2ed523a9a011123748ba1640637827bfe5fa72411930b5f2:
the capture header follows the quiet build flag; only that profile constructs
the spare UART2 adapter. Physical USB opcode0x5B returns status/start/stop or
bounded chunks through0xB4, with strict request lengths, a27-byte snapshot and
at most32 raw bytes per chunk. The normal status appends the explicit link-enabled
byte without changing its existing prefix. Protocol reserved-envelope byte0 and
COBS/CRC validation remain unchanged; no stock-PCController framing adapter is
implied. RESET GPIO10 is only observed with digitalRead/gpio_get_io_config here.

Reviewed the actual loop service, cloud-TLS pause and USB/HTTP/WS/cloud/upload
capture guards. They reject concurrent mutations during the bounded diagnostic;
they are not claimed to constitute a proven race-free global maintenance lease.
Commissioning still requires exclusive USB ownership and no concurrent client
mutations. Ordinary UART1 requests return false in the quiet build, and GPIO17
stays input; the new adapter has no AVR write or automatic resume path.

The source guard7e011e779e0aec34d73931ccde54566fb87bec811291bbae52fe5cffce57dd17
passed. All five existing main references and both platform-test proof references
were rebound only after reviewing the changed integration and surviving anchors.
Also reviewed the additive script-test registrations in protocol wrappers and
quality checks: no existing check was removed. Protocol.sh hash
00a526480bb65c5245ebe1c945af554663bdfe137b00b50badf7c85397014f31 and ci-checks.mjs
449c07e0b26f93a640444a7ab1729ea1a0e2d6c2ae5c0beae49b1a2961cb1c60 are now bound.
Generated comparison and strict Check passed:4403 inventoried,128 bound,
4275 pending and92 known parity gaps. api-contract, peer-binding,
web-presentation, web-streaming and uart-flow-control gates all passed; diff
whitespace validation passed. These are source-evidence checks, not an SDK build.

The preceding normal UART-enabled S3 image at eb0de02 completed SDK compilation:
program1840543bytes, globals74876bytes, app SHA256
ef153bd25f9c8080530702e038090176829152c17b1aa21d5cd0a8590829a3c2.
It was NEVER flashed. The exact quiet adapter/integration SDK build and hardware
pad/capture observations are still pending at this receipt. No physical success,
reset-loop cause, pin electrical safety, AVR communication or production closure
is inferred. User confirmed3.3V and the GPIO10 RESET connection, but never changed
wiring or grounded AVR RESET; correlation with pausing S3 remains only correlation.
All PBX calls and unsolicited test audio remain stopped.

## Explicit S3 ISP commissioning build option

Reviewed tools/build-firmware.ps1
ee572248013c49c6cc2690c1256ac337ff581809ed2b46255007c52895400700:
the new S3IspElectricalVerified switch is false by default, rejects every non-S3
target before build setup, and adds PAKAR_S3_ISP_ELECTRICAL_VERIFIED=1 only when
explicitly requested in the S3 branch. The source macro still defaults to0 and
the quiet passive adapter has not gained any reset or transmit operation.
Actual Plan execution confirmed non-S3 rejection and both C/C++ recipe flags
for the explicit16MB/OPI quiet profile; Plan starts no compiler or serial port.

Root intentionally selected this acknowledgment for the user's reported current
3.3V AVR and direct AVR RESET-to-GPIO10 wiring, retaining the planned active-low
reset polarity. This is an operator-selected build gate, not an electrical
measurement, verified AVR signature or proof of every ISP connection. The quiet
image can therefore advertise the enabled ISP feature, but no probe/reset/erase,
flash/fuse/EEPROM operation is automatically authorized or executed by enabling
the flag. Root's exact quiet SDK build was started, not yet reported successful
here. Any later deliberate signature probe must be a separately controlled
operation: releasing AVR RESET could cause its existing boot cue once. No such
operation, hardware access, audio or PBX call was performed by this review.

## Quiet adapter CI source coverage

Reviewed the complete current build workflow and governance test against the
existing fully reviewed Pealayer ci-cd.yml mapping; the donor hash remains
bba9a295ee7b59e318f76e0b6f6a80f0aec631e249f7d8c798f6c92c0bb7fd8c.
The guarded quiet S3 SDK command now precedes the normal S3 command in the same
firmware job. Both reuse the existing S3 cache/output; no clean, second diagnostic
cache or separate quiet package is added. The normal build runs last before
upload and replaces the diagnostic artifact. CI's quiet profile does not enable
the electrical-acknowledgment ISP switch. Governance asserts exact single
commands, ordering before upload and no cold diagnostic caches. All existing
firmware, protocol, desktop, mobile and release checks remain.

Rebound only workflow target4e284a5e8b4f01ae8bfd62731b533cc02c3e24bf10c108b49f9120ce07b92bf6
and governance proof5bc629302abaeb749345d1eebc7b2ed1144251e65c7a87b7bbbd1d6bbe05125b;
ci-checks.mjs remains449c07e0 and its actual fail-fast child runner is unchanged.
Actual repository_governance_test and build_script_security_test execution,
comparison generation/strict Check and whitespace validation passed. The network
owner additionally reports actual actionlint passing for this workflow. This is
local source/lint evidence, not a GitHub Actions SDK run, artifact publication,
quiet-image deployment or AVR hardware acceptance. Existing pending reuse and
physical requirements remain open without changing their dispositions.

## Actual quiet S3 SDK, deployment and passive observations

Root reports the real quiet SDK build at frozen shared main
e19a7c10fe7e0bcd2ed523a9a011123748ba1640637827bfe5fa72411930b5f2 passed:
sketch1843223/3342336, globals74916. Actual application1843376bytes SHA256
6c65e309173c91284605476662c2aad1f26349e5c6fa68bc2431c4482c3789f5 was then
flashed only at0x10000 on COM22/same MAC and esptool hash-verified. Full original
backup remains unchanged; this was not another bootloader/partition rewrite.
Build flags included PauseAvrLink and explicit S3IspElectricalVerified,16MB/OPI.
Thus the actual SDK compiled the adapter, not only its pure helper fixture.

USB ESP0.2.7 returned valid42-byte GetStatus with normal AVR link0,
station/AP/NTP true and reported errors0. Root performed two explicitly selected
RX-only captures, with pin-ownership checks and stop confirmed:

| Baud | Elapsed ms | Received | Stored | Dropped | Hardware RX errors |
|---|---:|---:|---:|---:|---:|
|38400|3059|140|128|12|96|
|115200|3030|172|128|44|92|

RESET GPIO10 was observed high/output-disabled in both. RX18 stopped high;
115200 also observed low during capture. Bytes were mostly00/FF/FE and no
plausible framing was established. Counts are observations, not valid AVR
frames, known baud, electrical measurement or causal diagnosis. No AVR TX or
RESET drive occurred during capture; no ISP probe, AVR flash/fuse/EEPROM action
or AVR reset-control write was performed.

The user confirms welcome audio STILL repeats with quiet UART firmware.
Cause remains UNKNOWN: this does not establish MCU resets or a software/electrical
root cause. Root stopped all board tests and requested AVR power disconnected,
or the whole assembly if power is shared. No PBX or test melody was issued.
The user never changed wiring or grounded RESET; do not describe that as a fix.

Read-only source audit found NO existing persistent reset-hold capability.
AvrIsp::enter/leave are private; public probe enters, reads signature and always
leaves, deasserting RESET then selecting INPUT. Backup/program use the same
scoped lifecycle. GET /api/avr/probe is not electrically non-mutating and cannot
be used as a safe hold. Legacy direct console W toggle is disabled on S3 and
serviceConsole is not loop-dispatched. Game reset only resets game state.
No new hold API or workaround was implemented or invoked. Valid AVR ACK,
display/keypad/audio/matrix, discovery interoperability and final acceptance
remain open; this evidence does not close issue2 or the pending donor gaps.

Final wrapper review: root added the actual pure s3_passive_avr_rx_test to both
existing host-helper compile/run loops, preserving all prior cases. It is no
longer only an ad-hoc invocation. Reviewed PS1 hash
61844cd674bbc5d163b1800f3382fc45bd0a8c466dfd00804f97533c6930bc5c and SH hash
1563e4959ca64611c6519cb300cae044a57faa0c53eb5d91269f56f33281296e;
rebound the existing firmware_controls_test mapping's SH target only, retaining
its actual surviving checks. Root executed that exact g++17 pure-helper case
successfully. No full suite or SDK rerun was performed for this documentation
finalization; strict comparison and whitespace checks remain the final guards.

## Exclusive native transport donor preflight

For the user's new single Wi-Fi-or-USB lifecycle requirement, read AGENTS and
REUSE-PREFLIGHT, issue2, peer-binding and exact donor-audit entries first.
Default consultation reports PCController live-unavailable; explicitly selecting
the actual canonical installed root correctly detected tracked inventory drift.
Root authorized a PCController-only generator refresh: canonical b0111ae0c80dc34149ea1044de7dd5488920301b,
1517 tracked paths,4673 total inventory. All other donor-origin rows were checked
and preserved exactly. The270 additional paths are inventoried, NOT newly read.

Fully reread current files through EOF, preserving prior mapping history:

| Canonical donor | Lines | Original-byte SHA256 |
|---|---:|---|
|Tools/Controller/internal/link/session.go|916|1311fdcde94b2ef8ed06db63ff874a255c135dc1969709e6f43ff4b5c5c8235b|
|Tools/Controller/internal/link/session_test.go|788|ff6c7b21f72bfb944d232c87210f8ffa139dfe016e8b5756f2582604489fffc4|
|Tools/Controller/internal/link/request_admission_test.go|489|0148ea4aac3cf230050045756a4308157adf786a2b6ca2d4fd99bcf665a622c1|
|Tools/Controller/internal/hostbridge/buzzer_mirror.go|435|dcb9bf3c1a31cb6cc06f4f5ed52def32b7388fd4d33587e233476a3a76e1f032|
|Tools/Controller/internal/hostbridge/buzzer_mirror_test.go|539|d92d0826cd9665520087a3b5704ed874ee5c605695d571975b3c15b7a54aa84a|
|Tools/Controller/internal/discovery/discovery.go|783|4ca50ce622f998f1922a828350ad71aea5fa5dee7506069ca3df2e857536198e|
|Tools/VirtualBoard/tests/firmware_controls_test.cpp|639|4facf265f8593b83f7610ae99ce58050013226cf1aa5a4d6e60bdbd6e5cc399e|

Five previously compared sources changed; only those five source hashes were
refreshed after complete reads. Unchanged session_test was reread too; the extra
request_admission test remains a reviewed dependency awaiting actual target
mapping, not an invented whole-file import. Existing target/proof hashes are not
stamped while root and UI owner still change their implementations. All original
omissions remain; newly useful generic gaps were appended to the same entries.

Concrete session contracts: closing/context checks before AND after admission;
final WithRequestWriteGuard immediately before port.Write after both queues;
physical-handle ownership through late open/configure cancellation and retryable
close; synchronous pre-close observer; persistent terminal error; ACK/Error
opcode+sequence matching and unsolicited routing; reserved macro sequence;
priority and bounded strip ACK slots; inactive initial DTR/RTS and deliberate
one-shot reset. A Rust route/epoch lease must be reviewed at its real OS-write
boundary. It cannot retract bytes already handed to the OS or substitute an
HTTP board/boot fence for complete downstream serial ACK ownership.

The extra drift review found useful pending short-note/timer rebuffering and
source-wide scheduler rebasing, original receipt-time preservation and avoiding
ready speaker reprobes; discovery started-vs-configured failure reporting,
secret-filtered bounded TXT/instance UUID and eight-slot public enrichment;
numeric overflow display and exhaustive byte-transition tests. Android's current
ToneTimeline lacks donor rebaseForStart, and display-bus tests cover only their
stated fade/ACK subset. No donor suite, production code, SDK build, device,
speaker/PBX or audio action was run in this source review. Final frozen native
target/proof binding, gates and runtime exclusivity evidence are still pending.

## Exclusive native transport frozen source binding

Reviewed the root's complete new connection.rs and changed main/config/HTTP/IPC
sections, plus the UI owner's complete unified control, shell dispatch/dialog
changes and actual replacement source guard. Fully reread Pealayer dropdown.rs
646lines (134f1613df05c9db44f8ccd178981a8eca4fb6c254ced6466229afca66b40ee9)
and menu.rs883lines (a0196878c7b76b898d0a5791b122181d9045d3fbbe9f56de52bc43242edf2d9d).
Current meaningful source identities:

- main95016f60e1032efe11d0d5e2e1f30b8f271bb4ec05d1a242d290f930a4c43c63
- connection38778812b8bf03975198808c672421f808a1486c402452473af3e63a83b0eda6
- peer_httpab52253344193953b470f963358afca041f8e0871a86d74778c91044bd5bc055
- shell8e250a5e1dfb4d7902913b98c97e57fdff3288a743b429195a8b7a9b323aec2c
- connection_controls573fe0f11df41826f7ff797cec716abf76a2482339dac7f5e9ff592fffbad6d4
- replacement source guard574c605348414b7f65ce48f3e92821ab1abdf41f05ccb4be1d6f7b5526d6a38b

Before admission, both method and epoch must match; selected USB rejects Wi-Fi
leases and a second USB handle. Revoked HTTP/WS resources retain leases until
drop; normal serial checks its lease before writes and drops the actual port
before handoff. Old events are discarded while generic uncertain operation
receipts can stay in Events. Existing board/boot HTTP scopes remain independent.
One persisted method/enabled intent replaces Both; USB status does not require
a usable Wi-Fi URL and no HTTP worker/discovery/telemetry is admitted on USB.
These are reviewed source mechanisms, not measured resource-release timing.

Review identified two concrete gaps and owners fixed both before binding:
root disabled idle HTTP pooling so leases do not leave pooled TCP owners;
UI now applies the common dirty/pending guard before Port mutation and verifies
a refused release cannot change its port. Dialog commits share that guard and
check the selected method after handoff. The live indicator now requires fresh
selected-board status, not only a WebSocket socket-open event. Existing manual
paths, focus/keyboard/dropdown semantics, settings drafts and delayed outcome
receipts are retained. Dynamic density/chevron/endpoint geometry remains pending.

Rebound only reviewed surviving targets/proofs; removed dual-popup function/test
anchors now map real unified draw/lifecycle/lazy-scan tests. Added ONE narrow
request_admission_test comparison with actual production Gate and zero-network
HTTP-worker fixture references. Its priority, exact ACK/sequence, reset, late
open/close and physical timing omissions remain explicit; no whole-file parity.
The UI owner reports source wiring/six negative guards passed. Rust fixtures,
native Check/Test/Release, runtime screenshots and true USB/Wi-Fi isolation were
NOT executed by this evidence slice and must be recorded by their owning runner.

Comparison generation and strict Check passed:4673 inventoried,129 comparisons,
4544 pending,93 known gaps. Mandatory build-storage, localization,
native-dropdowns, peer-binding and api-contract gates all passed with exact
current source/target/proof hashes; whitespace validation passed. This unblocks
guarded native compilation without weakening a gate, closing the donor ledger
or claiming physical acceptance. No firmware/AVR/ISP/audio/PBX action occurred.

Actual first guarded Check then reached Rust compilation and reported E0624:
main attempted PreferencesDraft::is_dirty, whose existing visibility is
pub(super). Root consulted preferences reuse and the actual existing draft
helper, then reused the SAME shell connection_change_guard from set_connection
and OTA instead of widening the Preferences API or duplicating its checks.
Reviewed final main65e92fb29dd0b55895c725be9f78fa8a868366d30fd7aa0b0cf9284fe8bac0a0
and shell46697fee16f151e0f1e6b836a9318dc9ca061247cf1c9511adffffa79ab9a4b3:
the centralized guard is pub(crate), and Preferences/device dirty drafts,
pending settings/maintenance and identity writes all still block. Preferences
source is unchanged. Rebound only these two reviewed target/proof hashes;
generation/strict comparison, the same five mandatory gates and whitespace
checks passed again. The first compile failure is not hidden as success;
subsequent actual Check/Test/Release results remain the build owner's evidence.

## Actual bounded ASUS LAN discovery and read-only API observations

After root explicitly confirmed the quiet image alive and passive captures
stopped, executed one bounded read-only helper on ASUS Wi-Fi10.252.139.56,
interface9, targeting hostname ledboard.local. No scan of unrelated addresses,
firewall/service change, board mutation/reset/audio, tunnel or app endpoint change
was performed. The normal AVR link remained disabled and AVR remained offline;
the unresolved repeating welcome report still requires silence and causal
diagnosis, not more actuation. Credentials were process-scoped and cleared.

The original private evidence remains byte-preserved on ASUS staging and local
outputs/PakarController/verification/S3-network-discovery-proof.json, SHA256
e56d9ddb4ec253de0291e5271594cefb526226a8a7e815a30596e0a7c3725f3c.
Its actual helper source was cf19bfd4568048f52a44ba5b547362aed14158e7e9046cc08a0cd851c6f5c465.
Raw query/response datagrams are retained as Base64, filtered to the one resolved
board address10.252.139.27; this is actual LAN evidence, not a loopback fixture.

- Six mDNS packets contained only A records for ledboard.local10.252.139.27.
  No PTR/SRV/TXT response was observed; the DNS-SD follow-up is null, not one
  successful packet. The first summary's PowerShell `@($null).Count` was misleading
  and was corrected after reading the original wire proof. DNS-SD remains open.
- Direct unicast SSDP M-SEARCH returned three actual root/basic/UUID responses,
  with interface-correct LOCATION http://10.252.139.27:80/description.xml and
  PakarController0.2.7 identity. Multicast M-SEARCH yielded zero responses within
  the bounded window; this is not full multicast-discovery acceptance.
- A four-zero-byte MNDP query received one valid TLV reply: station MAC
  90:70:69:ea:c8:00, identity ledboard, ESP32-S3, version0.2.7, station address
  10.252.139.27 and board ID esp32s3-907069eac800.
- Authenticated GET /api/status, /api/network, /api/time and /description.xml
  each returned200. Status matched board ID esp32s3-907069eac800 and fresh boot
  c7f43b2d700581a7bc10c14dbaf9bd7c, with HTTP peer fencing true, buffered mode,
  AVR link false/AVR offline, configured STA connected and AP started.
- WebSocket opened, delivered initial status and answered the read-only
  `{"type":"telemetry"}` request with actual type telemetry and the same boot.
  Its clock reported source ntp, observed true and valid true. This proves the
  measured JSON exchange, not remote cloud MQTT/WebSocket or AVR communication.
- RGB reported available/enabled/have_output false, pin-1; no optical success
  was inferred. ISP available true reflects root's explicit operator-selected
  electrical-acknowledgment build flag, not a signature probe or physical proof.

The helper exposed a PowerShell Task<void> result-shape bug: ConnectAsync and
SendAsync emitted two empty objects ahead of the actual HTTP result map. The
original proof is not rewritten; its final map contains the verified responses.
Only those task results are now explicitly discarded, with a narrow static
regression added to the existing offline parser fixture. No live rerun or firmware
change accompanies this correction. The helper/test are not yet target/proof
bindings in file-comparison-evidence.json, so no unrelated hashes were refreshed.

Actual PTR/SRV/TXT DNS-SD, multicast SSDP/router/phone interoperability, SSID scan,
mobile AP Web UI, semantic RGB/matrix optical output, AVR UART/peripherals and
production multi-client/physical acceptance remain open. mDNS on the ASUS LAN
does not imply multicast crosses the routed VPN to the local PC. App forwarding
and endpoint changes remain held until silence is confirmed.

## Exclusive connection Preferences and exact contract-proof recovery

Fully reread current Pealayer src/ui/preferences.rs through all2308 lines,
SHA1fa2482a389fee84c79eef3d69ad8cc0ecb922c319fd222219af3e90d7e6f965,
after consulting Preferences and exact donor-audit entries. The actual
sync_external_appearance three-baseline merge is now adapted to the live
connection owner, not merely copied as styling. Reviewed frozen Pakar Preferences
c5356e250994432a76e9bc3533e482d3c9903dd5db73864c1479f5c7d1b81739:
selected-method summary delegates to the canonical dialog only from a clean
draft; incoming owner fields refresh config/saved/previewed together; unrelated
Save cannot restore stale Wi-Fi/USB fields. New source-bound donor row preserves
concrete pending helper HWND/process, watcher/config-authority, typed-controls,
density, picker and full lifecycle-test imports. Media-specific options alone
are not applicable; generic mechanisms remain pending. No whole-file parity.

Parent reports actual guarded Check and181 Rust tests passed including the two
new Preferences cases; the earlier checkpoint had179. Release ExpectedCommit
abbreviation39c878d was rejected by its full40-hex identity guard before compilation;
the full39c878da3e9abca43db7bfdb12445ed8d11014f1 was accepted. This is not a
compiler failure, and source tests are not installed-app/human acceptance.

CI correctly exposed two obsolete dual-control fixture assumptions after the
unified connection UI. Reviewed native_dropdown_test.mjs and the full updated
native_render_lifecycle_test.mjs, including new negative mutants: the single
shared consumer remains mandatory, lazy enumeration belongs only to selected USB,
keyboard Configure stays tested and clear-before-return cannot be removed.
Only their reviewed reference hashes were rebound; no acceptance test was
disabled. Parent reports the focused fixtures passed; this reviewer ran no
native/SDK build or device action.

The separate23-file historical docs/pccontroller/SOURCE-MAPPING.json had stale
target hashes despite the exhaustive ledger already identifying the newer S3
sources. Fully reread pinned20c6115 donor UartProtocol.h, MacroQueue.h/.cpp,
FrontPanelModel.h, AutomationStore.cpp and all1007 ProtocolRuntime.inc.h lines,
together with complete current Protocol.h, SerialLink.h, PcControllerRuntime.h/
.cpp and AutomationStore.cpp. Additive ESP front-panel/passive-diagnostic opcodes
do not alter fixed-zero envelope integrity, bounded COBS/CRC or tunneled0x56/57
semantics. ESP menu callbacks do not replace the shared MacroRing owner; EEPROM
guards exclude both ESP proxies while retaining AVR store ownership. Updated
only four target identities and two shifted anchors; preserved the original
source revision/hashes and all hardware/pending omissions. The focused
pc_contract_mapping test passed all23 exact mappings without weakening its
checks. AVR/ISP/audio and physical EEPROM remain deferred, not reaccepted.

Root also changed three architecture-neutral maintenance labels from ESP8266
to ESP/ESP board-module wording and esp.bin. Reviewed the text-only diff:
request paths, file validation and the existing2MiB client cap are unchanged.
This label fix does not establish partition-aware larger-image desktop upload
or module/size verification parity. Final Release/Test after the checkpoint,
runtime replacement, actual USB/no-network observation and human UI acceptance
remain distinct evidence owned by the deployment task.

## Android optional settings catalog failure: bounded parser repair

The sole reported Android CI39045757076 failure exposed real parser/catalog
drift, not a clock/lease workaround: generated optional:boolean/optional:string
DHCP/static fields were treated as unknown required types, so an otherwise valid
older peer response could leave the network draft absent. Fully reread actual
Pealayer PreferencesTab.tsx563lines (30a63cc1db61a46346c2d4c30ce011dc74081a8a6da2e29853e56d39811a0de1),
the frozen mobile DeviceSettingsSession/parser and entire focused test, with
the actual Web settingsType/settingsSnapshot/settingsValidateAddressing and
native Addressing::validate functions. No new source-complete claim is made
for the rest of the native settings module or donor dependencies.

Reviewed mobile35309b6a6d18d9fb8a5ef687509edee9c9f3aa25d63ad3026081bc96b17d5f1b:
only absent optional keys are skipped; present values stay strictly typed.
The six addressing fields must be absent together or complete, with valid IPv4,
static mask/host/gateway/DNS bounds and no fabricated default observations.
Peer ownership, late-read/write epochs, secret handling, positive ACK and exact
post-write readback remain unchanged. Focused test365cb9c9e9986b9f5ad42bf99fdd910be723b690113ebb126253c5c337b79c63
adds absent/complete/partial/null/type/address-boundary checks and explicit
successful network/cloud load assertions without removing the existing no-replay
or ownership cases. This does not complete mobile static-address editor/UI parity.

Rebound only the existing PreferencesTab mobile target/test references; preserved
all generic control/localization/picker omissions. Strict comparison generation,
all seven mobile wrapper gates and whitespace checks passed, with130 comparisons
out of4673 inventoried files unchanged. The UI owner was released to run only
the guarded focused test file; no APK, full suite, device or firmware operations
were performed by the evidence reviewer. Its actual test result remains separate
from this source review and from Android phone/physical board acceptance.

## Deliberate white boot and unchanged LED-record write avoidance

Fully read current actual PCController Project/SettingsStore.cpp/header and
the EEPROM ownership dependencies; source cpp is7309fd36f0680722a4260e8f1251ae5a0fc67e99fc60f2426bb1294f789e87d4.
The applicable donor saveNow mechanism is EEPROM.update changed-byte avoidance,
not an ESP filesystem driver. Reviewed the entire white-boot diff, new
LedPreferences.h and both changed test files: the existing six-byte private
record/checksum is retained, old valid animation records restore brightness but
reset the table white, and saves store pattern-none. Existing exact record bytes
are compared before opening LittleFS with truncation; transient effect changes
therefore do not trigger identical writes. Runtime rainbow/fire remain explicit
commands, not automatic boot animations. No new API generation/layout selector.

Main e25f493d620f64c1b82e863d84003060a4914e50d24d19c490259d18d94df61c
changes only these record/save/load callsites; peer binding, UART, ISP and
maintenance paths were not relaxed. Rebound its surviving historical anchors
and the extended s3_platform_wiring proof007155a9be17532b0912e97aea8a3c65d98f7ab5a08b6fe9b65b1ec96c5a4fb1.
New SettingsStore narrow comparison binds headerbb0a86a9b0506953270c5bf58f0fb9a49d5e6990b67f3adbd1b22d7e3a71d6cc
and LED test136526419cbf0b2985119d24117082970c0cececc9d57c4b232319434a34da07.
Changed-record coalescing, transactional rename/verified save error propagation
and exact filesystem write-count tests remain explicit pending gaps. This is
not crash-safe persistence or donor-wide settings parity.

Root reports the actual protocol wrapper passed the LED case and preceding
protocol/ISP/tone/front-panel fixtures, then correctly stopped at the stale
peer-binding main reference before the WebAccess tail. No full-wrapper success
was inferred; root owns the remaining gated tail and final SDK results. This
reviewer ran no test suite, build, flash, reset, audible test or device request.
Comparison generation/strict Check and API/peer-binding gates passed with131
comparisons,4542 pending and95 known gaps. Generic firmware-hardware is routing,
not a configured gate; no nonexistent S3-specific gate acceptance is claimed.

The preceding read-only reset audit stays separate: saved S3 telemetry reason9
matches the installed SDK brownout enum, not AVR reset cause. Current AVR
telemetry has no MCUSR field; welcome can also replay on game/key/mode/Prog
actions without a CPU reboot. Donor ResetTelemetry's early capture/watchdog
disable is useful pending work, but its EEPROM journal336..719 would overlap
Pakar automation storage270..411 if transplanted unchanged. Actual AVR image,
bootloader/fuses, ISP signature and verified recovery backup remain unproven.
No firmware-cause diagnosis or working paired-programmer claim is made.

Final root verification: the protocol wrapper's substantive C++ cases passed;
its expected stale-evidence stop was followed by successful gated WebAccess and
Tasks tail runs. Quality stopped at the independently hash-bound PU route
manifest; the actual route bodies were rereviewed and its five shared-sketch
hashes/shifted anchors repaired, not the guard bypassed. The same quality runner
resumed at that failed checkpoint and all remaining checks passed. Guarded
16MB/OPI quiet S3 compile passed, using the existing warm cache without clean.
Candidate SHA599509ab551e4b372ad1f15b8ac98d3f22dfdea093d0d107e0ddda4e7ba9fac9
matches ASUS staged readback. No board deployment or physical white-boot test
was possible. Android preview SHA3ce5304a88caad521238216107da51f789a982a69882c44ce08a3b90e516e1b9
was installed preserving data and verified from the actual installed APK; phone
lock blocked launch/capture after one permitted swipe. No second swipe, server
restart, audio, ISP, erase or fuse operation was performed.

The subsequent quality run exposed the separate PU route manifest's stale
main-file binding. Read its complete test/audit definitions, actual PU AsyncOTA,
AsyncWebServer and SSDP sources plus the bounded info-registration slice;
rereviewed the five current route bodies against the LED-only diff. Their route,
authentication, upload/capture ownership and response contracts did not change.
Updated only those five hashes and shifted target lines in the existing
PU850-ROUTE-PARITY.json/.md; source revision/hashes,50 dispositions and every
pending omission remain unchanged. Focused pu_routes_test passed all50 mappings.
Search across docs/tests/tools/.github machine-readable manifests found no other
old main or S3 wiring-test hash. No route source/test guard was weakened.

## S3 inactive-slot remote staging and live OTA admission failure

Read the actual complete PU850 LocalLib/AsyncOTA.cpp/header and
FirmwareTransfer.cpp/header, with the current RemoteFirmware, RemotePolicy,
FirmwarePlatform, MaintenanceRoutes and ArtifactDownload implementations.
AsyncOTA.cpp donor SHA3d681d678b58e5e615448acddc52fd623914c8ec4a76f80f5bbb75cd34c3aaec
is now a narrow hash-bound comparison. The new remote-ota index reaches the
four OTA/transfer donor files; its gate validates two required reviewed cpp
sources, not every donor. AsyncOTA.h still has no independent comparison row.

The identified S3 application exceeds LittleFS capacity. Adapted the actual
ESP-IDF OTA handle lifecycle into S3OtaStage: sequential inactive-partition
staging, bounded writes, exact length/SHA/SDK image validation, no boot selection
while downloading, explicit journaled apply, persisted-stage revalidation and
actual postboot running-image checks. Quiet ESP-only maintenance bypasses only
absent-AVR preparation, never paired Prog/backup/version gates. Ready stages block
competing local OTA and retain bounded artifact export. ESP8266 staging remains.
The publisher now accepts the actual S3 hardware profile/raw image and an omitted
AVR component; no versioned API or compatibility alias was introduced.

The old SDK's getSketchSize verifies the complete image. Actual installed-core
Esp.cpp confirmed the firmware-download chunk callback repeated it for every
chunk. Captured verified response bounds once instead; no repeated image scan
inside the reader. A live old-image download timed out, with a new panic-reset
boot observed during the interval. Without a backtrace this is not proven reset
causality. The user separately reports correcting the SS36 supply drop; that is
not our measured external-power acceptance or an AVR reset diagnosis.

Full protocol wrapper, the actual S3 RemoteFirmware host/SDK-double harness,
publisher cases, S3 wiring guards and complete quality runner passed. Fixtures
exercise a1843552-byte image beyond FS capacity, real BCrypt SHA, stage/resume,
explicit apply, journal/HTTP/length/encoding/write/end/hash/tamper/cancel failures.
Final guarded16MB/OPI quiet S3 build passed without clean/new cache. Raw candidate
1850096bytes SHA5b5d775a77e66d853566e56b4b1af8ccadaabfd30272e52845f0b4d5c4739f12
was staged and hash-checked on ASUS; no new native/APK/AVR builds were needed.

Published only raw application and pinned manifest into a new immutable scoped
office HTTPS directory. Server disk image hash and HTTPS manifest hash match;
manifest SHA isb828e7dfd3833461aa7e83bc1281761569f973828d69d7fb4f4aa08005e231c9.
No server configuration, unrelated tenant, broker credential or full flash backup
was published/changed. This is server artifact proof, not board Internet delivery.

Native IPC enqueue did not expose a durable update result. Fresh same-board old
image/boot/idle/zero-upload resolved no applied image. Later direct one-shot OTA
returned HTTP400: Another operation is active or multiple files were supplied.
Retrieved that exact saved response after SSH returned; no blind replay. A
subsequent attempt stopped at preflight without POST. Disconnected ASUS's native
app through guarded typed IPC to quiesce polling; observed enabled=false and no
retiring resource. It still requires a guarded reconnect before handoff.

ASUS retained Ethernet and joined ledboard AP after replacing only the newly
owned current-user profile with the same all-user profile; original XML was
preserved, existing profiles untouched. Hostname resolution failed on this AP.
Use explicitly selected current192.168.4.1 only with actual board/boot guards;
do not silently change routes. Tunnel access then became intermittent before
the latest private station credentials were applied. No password is in this
receipt or source. Preserve AP recovery and verify real network persistence.

Rebound only reviewed target/test hashes and affected route anchors; all50 PU
route dispositions remain. Comparison count132/4673 leaves4541 pending and96
known gaps, not completed donor parity. Native outcome persistence, bounded
DNS/TLS opening, real OTA admission/HTTP export, slow-client ownership, board
TLS fetch/apply, rejection/cancel/reconnect, rollback/power-loss/soak and optical
white boot remain open. No AVR command/reset/ISP/fuse/audio or PBX was performed.

Live continuation recovered ASUS, read the same isolated board/boot through the
AP, and saved the new private station settings once (202 accepted). Actual
association and AP-enabled readback passed; hostname resolution through ASUS
worked after the OTA reboot. No station secret was written to source or GitHub.
The prior upload's failed phase/32bytes plus saved negative ACK were resolved:
the installed ESPAsync parser treats a Content-Type-bearing MD5 field as a file.
Removed that header and quoted multipart dispositions in the private uploader;
did not weaken the device's duplicate-file/ownership guard. Corrected one-shot
upload returned200 updated/rebooting=true. Boot62122282967feaf4192b9e2f9fa7646e
reported1850096bytes and matching MD56b2b76343d415a3a1a3584f4bb181137,
all80 cells white and480 committed RGB pixels white at255; network settings
persisted. Initial AP readback failed, so recovery used the now-resolving hostname
without repeating any upload. Full-image SHA/optical acceptance remain open.

Measured a65536-byte HTTP range:206, exact Content-Range0-65535/1850096,
17.3seconds. Full stream received at least265120bytes before closing; a later
boot change was observed. Bounded passive USB capture used the existing guarded
SerialPort setup with DTR/RTS false and transmitted zero bytes. It returned
POWERON boot text and an esp_cache_msync invalid-address warning, but cannot
establish when that boot happened or attribute it to download, power or port
opening. Do not claim that an explicit no-reset command guarantees no electrical
reset, or diagnose panic from this log. Subsequent read-only status succeeded.
S3 host fixtures are also wired into the Linux protocol CI runner using its
existing OpenSSL SHA adapter; Windows uses the already-executed BCrypt adapter.

The added Linux S3 invocation correctly tripped the existing display donor's
whole-runner hash binding. Reread the actual current639-line PCController
Tools/VirtualBoard/tests/firmware_controls_test.cpp through EOF and the complete
runner/diff; donor hash4facf265f8593b83f7610ae99ce58050013226cf1aa5a4d6e60bdbd6e5cc399e
still matches. Only the new S3 test/gate was added; the display loop and all
existing invocations are unchanged. Rebound that single runner proof hash,
retaining every display/key/transition/tone pending omission and hardware scope.

The remote-ota gate now runs before every ESP build/Plan and the focused S3 test;
the guarded quiet S3 Plan passed all gates without compiling or writing assets.
The full Linux invocation remains CI execution, not locally asserted Linux proof.
An instrumented full HTTP200 running-image download then completed1850096bytes
in19.24seconds with SHA5b5d775a77e66d853566e56b4b1af8ccadaabfd30272e52845f0b4d5c4739f12.
Request headers pinned bootdc2b802cd3318072494ae7282e8c9e46. Independent telemetry
reported matching MD5/integrity and panic reset4 on that boot; the180second passive
console window contained zero new bytes. Happy-path OTA/image proof is now real,
but intermittent failures, panic cause, AP recovery, optical and soak remain open.

The first Internet stage guard refused an unset clock before dispatch. A pinned
owner set UTC from ASUS and read it back valid; this is not SNTP proof, and the
existing manual-time route still reports source unset (a remaining telemetry gap).
A later stage POST lost its response; actual new boot/idle/job0 was inspected,
never interpreted as completion and never followed by Apply. No two-board action
or remote application/rollback success is claimed. Keep USB available.

Resolved the later pooled-client failure separately: same bootdc2b..., idle/job0,
no error/ACK and no console output. A fresh non-reused mutation transport obtained
the real202 state ACK for job1, manifest phase/ESP-only/apply=false/received0.
The next bootc900bd0d775661e6ab978a527590fba1 recovered that journal as failed:
Interrupted update; inspect Prog and backups. No operation was replayed.
No Apply was sent. Preserve the acknowledged-job distinction from transport-only
failures and diagnose the actual ESP crash before another Internet download.

## Actual S3 DNS/SNTP panic and project-only core-lock adapter

Read the current physical partition table instead of inferring it from the
original factory backup: app0 at0x10000/app1 at0x340000, each0x330000,
LittleFS0x670000/0x180000, crash0x7f0000/0x10000. Read only4KiB table,
64KiB crash and8KiB OTA selection through the identified CH343/COM22 and
MAC90:70:69:ea:c8:00. Those read-only esptool operations reset ESP; they are not
claims of no electrical reset. No AVR UART/ISP/reset command or fuse operation.

Crash partition SHA17e1459ecb1e13022795dc361c7a691d296c0d00cbefd7c0cdbf0762f68c4e8c
has a valid24580-byte ESP ELF/CRC record. Its note contains the9e9299abf ELF
hash prefix matching the preserved fault ELF's complete SHA
9e9299abf8424e06c52d22721976eb9d4e508f3310251b98bce4afe84820130e.
GDB with auto-loading disabled and arguments suppressed shows:
RemoteFirmware::openDownload -> HTTPClient::GET -> NetworkClientSecure::connect
-> NetworkManager::hostByName -> dns_clear_cache -> sntp_dns_found
-> sntp_request -> dns_gethostbyname -> udp_new_ip_type assertion:
Required to lock TCPIP core functionality! This establishes this crash's
software cause; do not attribute it to the earlier corrected supply drop.
Raw RAM/core dumps may contain secrets and remain private, never web exports.
The one preserved13.8MB fault ELF is essential diagnosis/verification evidence,
not another debug build cache or a copied desktop package.

Fully read actual Arduino3.3.12 NetworkManager.cpp (SHA
a4c0c9c35fa185884d5acbc3c9a3e029d0fb9e4bf824d313e81b5dd4c6d5328a),
the applicable secure-client connect overloads, and the actual lwIP core-lock
query/mark macros and nonrecursive mutex contract. Reviewed Espressif's core
dump loader/header/CRC and bootloader OTA selection implementation. The latest
stable core remains3.3.12; did not downgrade or install a prerelease SDK.
The project linker adapter wraps ONLY dns_clear_cache, acquiring the core lock
only when this caller does not already own it. It holds no lock around blocking
DNS/TLS/socket operations. Original DNS, SNTP, SNI and CA validation remain.
Shared SDK bytes are unchanged; the exact S3 build wrapper carries the linker
flag and a source guard prevents forgetting it. Actual linked disassembly
confirms NetworkManager::hostByName calls __wrap_dns_clear_cache.

The actual production adapter fixture passed caller-owned, already-locked,
TCPIP-task and nested-callback cases through the real linker wrap. Added it
to both protocol runners, preserving every prior test/display assertion; only
their hash bindings and the four extended source-guard references were rebound
after reviewing those diffs. Guarded16MB/OPI quiet SDK build passed in the same
cache with no clean. Candidate1850176bytes SHA
87a62afd005c104ab713a013ff51069325fd5dd97d7c3ed1587cd1ed02700d58,
MD57a6cbc3d511cc13c912bb1fcd4340a55, ELF SHA
16b53202d7c4b71cb08de0700ed3ae91ded3dc7a610c0abd48c7325d919ac7ce.

Consulted the network-cloud PU mappings and read the entire actual PU
WiFi_Setup function plus CrashHandler.cpp/header through EOF. The ESP8266
crash UART-suppression hook/cont_t callback do not implement an S3 crash
collector; most persistence is commented TODO code. Do not transplant that
suppression or invent log-rotation parity. Generic safe diagnostics/export/UI
remain pending, as do the donor-wide comparisons; this focused SDK adapter is
not PU shell/discovery/crash-feature completeness.

LAN upload of the fixed candidate ended after520627bytes; actual same boot,
aborted phase and unchanged old image resolved no commit. Did not replay it.
The required one-time USB bootstrap wrote ONLY the selected app1 application
at0x340000..0x503fff, with esptool hash verification. Actual OTA selection
records were CRC-verified(seq1 and seq2/valid); table, OTA metadata, NVS,
LittleFS, inactive slot and crash partition were not written. Known-good5b
application and original16MB backup remain preserved. Boot
fa9d4563d68224b0f87919a17c0f618c reports matching new size/MD5 and persisted
station/AP configuration. ASUS disconnected only its ledboard Wi-Fi client,
retaining Ethernet; ESP AP remains enabled. This USB bootstrap is not passed
off as successful LAN OTA of this fixed image. Internet stage/apply and full
postboot hash/soak still require their own actual receipts.

The post-bootstrap Internet job2 failed cleanly in the same fa9 boot, without
the prior panic. Actual telemetry showed IPv6-only primary DNS and no publisher
resolution; the network gateway answered the identical lookup from ASUS. Read
the SDK's actual STA dnsIP setter: esp_netif_set_dns_info changes the resolver
without stopping DHCP. Added a rate-bounded S3-only DHCP recovery policy using
only the usable backup or this network's IPv4 gateway; static profiles, working
IPv4/IPv6 primary or routed IPv6 backup, offline state and AP settings remain
untouched. No public DNS, insecure TLS, hardcoded publisher IP or settings write.
The build macro can disable fallback. Actual production-header fixtures cover
missing/global IPv6, valid primary/backup, static/offline, unavailable gateway,
setter failure/retry, throttling and millis rollover. Kept the existing PU
AsyncOTA comparison, exact target/proof hashes and remaining live gates rather
than creating a second checklist or claiming donor-wide network parity.

DNS recovery candidate1850752bytes/SHA82ba2e463536b6a86935f629300ddbb78a80f88d7ef89efba1c10a2cfc311554
was USB-bootstrapped after another LAN abort (25202bytes, no commit). Current
OTA metadata was reread before writing only app1. Bootb8a67e1bf0d56ee8f5e5b788008dda4e
reports matching size/MD5; actual DNS became192.168.1.1, publisher resolution
succeeded and SNTP synchronized automatically at41154ms (source ntp,
ntp_observed=true). No manual clock set in this boot. Station/AP persisted.

The .NET fetch helper failed in read-only preflight with dispatched=false; no
mutation was replayed. A new curl-based request returned an actual job3 manifest
ACK. The private helper incorrectly expected an accepted boolean rather than
the documented job-state response; this harness false negative was corrected,
not counted as device rejection. Readback, not a second POST, established that
the board had parsed the manifest but rebooted; interrupted journal became
failed and no Apply occurred. Current crash record SHA
5c6082a3b5f594a108f0386266c5dffa67a4cbeb211bb543d2e35885c5489d35
has a valid23332byte ELF/CRC record. The preserved matching eeffe49a fault ELF
resolves the call chain through HTTPClient::setTimeout → openDownload → service.
PC0 and the installed SDK source establish a dangling client on the transition
from manifest to image: end() leaves _client set if already disconnected, but
Pakar destroyed the socket before begin/setTimeout reused that pointer.

Read actual ESP32 HTTPClient begin/end/disconnect/connected/setTimeout/destructor
and the corresponding ESP8266 ownership implementation. Keep the stopped client
object alive through rebinding, bind the new client before replacing the old
owner or inspecting it, and declare HTTPClient after its socket so destruction
is correctly ordered. stop releases TLS resources; no idle open socket is kept.
The host double now models end retaining a disconnected client and asserts
client liveness during setters, rebinding, end and destruction. Added explicit
manifest→image, cancel→fresh job and destruction coverage. Both actual S3 and
shared updater fixtures pass with this stricter model; old unconditional-null
test behavior was insufficient. No TLS/auth/identity/apply gate was weakened.
Keep private raw cores and the essential matching fault ELF out of GitHub.

The lifetime-fixed94d227ed candidate was app1-only USB-verified, then Internet
job4 failed cleanly before receiving its manifest, in the unchanged0715a909
boot. Automatic SNTP was observed again at96510ms. The generic combined error
was not sufficient diagnosis, so openDownload now preserves distinct URL,
clock/heap, binding, numeric HTTP/TLS, length/encoding and FS/inactive-slot
failure reasons through cleanup/journaling. It never exports response bodies,
signed URLs, auth headers or token text. Added exact rejection assertions and
negative-HTTP numeric TLS coverage; focused production-module fixtures pass.
The fixture initially exposed its own missing String+integer overload; changed
production formatting to bounded snprintf, rather than weakening the tests.

Diagnostics candidate1852176bytes SHA
40a4024e620877295a63385812f43f83344cf9919a4232e337f21c89489294e1,
MD5c7a394b090864fe59b81e8ea709794bd, ELF SHA
e8cc6afc4ea86df5c92c3fd9b37f37bba9cd4ab8aae28cbe9c2534e01d782776
passes the guarded16MB/OPI buffered quiet build and selected-app1 USB hash
verification. Current metadata was reread before each bootstrap; no OTA select,
NVS/FS/table/bootloader/crash or AVR write. Immutable publisher file hashes
match; actual ASUS TLS1.2 HTTPS returns200 and the server verifies200 using
RootX1 with no default CA directory. None of these bootstrap/server/PC facts
substitute for successful board Internet Apply. Keep the LAN aborts, intermittent
empty HTTP replies and full physical/discovery/soak/client gates open.

Live boot76e35a7c showed DNS being overwritten again: IPv6 primary at26577
and57296ms, then gateway restored at34053/71019ms; SNTP was still unset at147s.
This exposes a periodic-repair race, not evidence that one successful NTP boot
means startup is robust. Added immediate, still validated DNS preparation before
the initial configTime call and only when RemoteFirmware needs a new download
connection. Normal background checks remain five-second bounded. No new public
resolver or arbitrary probe: the exact same policy preserves working/static/
offline configurations. Production-header tests inject RDNSS replacement inside
the throttle interval and cover first-boot preparation, immediate repair and no
rewrite of already-valid DNS. Source guards require both actual call sites.

Job5 subsequently reached Ready with all1852128 Internet bytes and the complete
staged raw SHA8acf9939... in72.708seconds. Lost HTTP Apply responses were not
treated as commits; actual journal/boot readback stayed Ready. A later genuine
panic(reset4) occurred while Ready. The CRC-valid23524byte crash record SHA
15604c6d170f81da048a7292b980d790328c631a20724bc6aaee9499f4394aaa
decoded against the current c2cad8f6 ELF shows __throw_bad_function_call →
std::function<void()>::operator() → AsyncHttpService::service → loop. Read the
actual request continuation implementation and adapter: AsyncTCP and loop
concurrently touched the same weak-request/std::function slot; service could
clear the callback while the producer was between assigning it and publishing
the weak request. Weak/shared pointer counters do not synchronize that pair.

Queue publication/take is now one mutex-protected unit on S3/host, with an atomic
processing flag for native upload admission. ESP8266 keeps its cooperative
single-core path. No mutex is held across pause, handler execution, response I/O
or reentrant service. An impossible empty callback returns503 rather than
actuating/panicking. The actual adapter fixture injects the old publication
interleaving through pause, then accounts for every one of20000 concurrent
producer/consumer requests as exactly one200 dispatch or bounded503 rejection.
Retains all old weak-lifetime/disconnect/reentrancy assertions; the actual
WebAccess/epoch fixture also passes. These are doubles/stress, not hardware soak.
Ready's cancellable report now agrees with its actual API; source fixture checks
it. No auth/fence, image validation or explicit Apply requirement was weakened.

The first USB bootstrap guard correctly refused changed metadata before writing:
sector0 was fully erased; sector1 remained seq2/stateVALID/CRC55f63774. Reviewed
Espressif v5.5.5 esp_ota_begin/invalidate-inactive implementation: rollback-aware
staging erases the inactive selection record without selecting it for boot.
Only that exact erased-inactive + unchanged valid app1 case was added to the
guard; current metadata was reread before app1-only write. No metadata was
rewritten to force a slot. Primary implementation:
https://raw.githubusercontent.com/espressif/esp-idf/v5.5.5/components/app_update/esp_ota_ops.c

Queue-fixed candidate1852656bytes SHA
a0dfcd6388e5590acdd184c051e379a411891c9180a9748f10bbb2d50e2b6e74,
MD579c037feb006c16ac3d066aa8b37c12e, ELF SHA
6e0ceea211031e8ed93f1b847811db36570fefe38a7a42f77b1b906c715db9da
passes the guarded warm16MB/OPI build and USB flash hash verification. The old
stage was explicitly cancelled in boota7372ca8 with positive ACK/readback and
no read errors, so it cannot later regress the queue fix. Job6 must stage this
new image before any explicit Apply; no final delivery is inferred from builds.

## Changed-image Internet OTA through the ASUS application

The queue-fixed bootstrap remained app1. A subsequent guarded warm S3 build
added actual esp.ota selected/running offsets and nullable SDK state telemetry;
this additive living contract does not introduce an API version or infer slots
from a backup. Final binary1853328bytes SHA256
29fca3373f32e875ae7181e775ef9b4373b85c898f3ae5375a1d0af6d0cea15c,
MD557dbbb3f6cfbe03a50289adb50539567, ELF SHA
ccce33b9afce12c575649d567b6f2925d93fea25dbdb9aec22ce451d19f0ac8b.
The raw application was published under its immutable digest directory; server
files match and the separately trusted manifest SHA is
71cf1a19e786c27eaa0cc84f899cf40d0be070d5b20744ddbec0a551547e0035.
No server configuration/service/account change. No Wi-Fi secrets in this ledger.

After explicit cancellation of obsolete stages5/6, Internet job7 fetched this
different image. Its observer lost a read late in transfer; no Fetch was replayed.
The installed ASUS PakarController e63c1bb0... CLI independently observed exact
Ready job/boot/manifest/image/length, then sent one explicit Apply and returned
exit0 with prepare-phase acknowledgment. New boot13acc69e049a90569d631f1bc7950281
reports Complete job7/expectedSHA, actual running_offset=boot_offset=65536 and
running_state_code=2 VALID. This final image was not USB-flashed. The old boot
was a7372ca8158212349c4e01295a351446. Independent image identity, changed
firmware-only fields, completed postboot journal and actual slot state are
stronger evidence than merely seeing the same descriptive version0.2.7.

Short same-boot readback through uptime982638ms reports reset3, persisted
station/AP, automatic NTP observed at131463ms, all80 cells white and all480 RGB
tripletsffffff atbrightness255, modecell/effectnone/AVRfalse. Not optical,
external-power cycling, arbitrary concurrent clients or long soak. One small
read needed a retry; original failure is recorded. A complete single running
image stream truncated after269428bytes and the first range observer failed
after262144bytes; neither was counted as successful full raw export. The ESP
remained on the same boot afterward. Private named receipts are documented in
REMOTE-MAINTENANCE; retain failed evidence and recovery availability.

Latest focused verification: guarded S3 build passes; real-linked DNS wrapper
and production DNS policy tests, actual S3/shared RemoteFirmware with stricter
HTTP client liveness doubles, actual async adapter deterministic interleaving
and20000-request producer/consumer fixture, WebAccess/epoch fixture and S3
wiring/PU routes/living-contract guards pass. The10 new telemetry lines required
explicit PU route anchor rebinding; the initial stale-anchor failure was fixed,
not skipped. No unrelated desktop/mobile rebuild or repeated full suite.
Existing comparison count remains132/4673,4541 pending,96 explicit gaps.
No claim of all-donor, AVR, discovery, UI, phone, rollback or production acceptance.

Final independent running-image readback subsequently passed:29 boot-fenced
HTTP206 ranges, exactly1853328bytes, SHA29fca337... matching the changed OTA
image, same hardware/boot before and after. Every range checked Content-Range,
Content-Length and expected MD5 ETag; complete chunks only entered the aggregate
SHA in RAM. Three empty replies and one partial45second read timeout were
retained in the receipt and retried only as bounded reads; no mutation replay
or duplicate firmware SSD copy. The failed initial streams remain evidence.
This establishes complete bytes, not reliable streaming/resume or network soak.

Restored the existing ASUS interactive owner9796 via guarded no-draft typed
Wi-Fi Connect. Positive app readback reports online=true, the exact board/final
boot and all-white cells with no pending maintenance, not just enabled=true.
Kept the same e63c1bb0... native executable, original protected preferences,
separate candidate profile and rollback; no force-stop, USB fallback or native
rebuild. Remote OTA and live app connectivity are separate from visual/human
acceptance and from direct Internet commands while the ASUS relay is absent.

## Continuous LED output and Wi-Fi scheduling regression

User reports visible stale/garbled output during Wi-Fi activity. Current board
was reread over hostname on ASUS: same13acc69e... quiet ESP-only boot, cell/none.
Read games-streaming lookup and literal Puzzles WS2811.cpp inventory; fully read
actual Puzzles LocalLib/WS2811.h/.cpp and verified original hashes39509602... /
0f7fec78... against the ledger. Retain its255 brightness and TypicalLEDStrip
correction; not its AVR D6/eleven-pixel buffer or BRG order. Independent channel/
480-address/optical testing remains required, not a declared donor exclusion.
Also fully read current PCController Tasks.h/.cpp and Pakar's existing adapted
Tasks: those loop-only cooperative callbacks cannot isolate seconds-long DNS/
TLS work or synchronize an AsyncTCP producer; no duplicate scheduler was copied.

Actual code inspection found WebSocket callbacks mutating LedGrid and producing
status from the network task while loop renders/transmits. Complete incoming
messages now pass a bounded eight-slot synchronized FIFO into the main loop;
closed/unauthorized peers are rechecked, overflow explicitly closes1013, and no
network payload pointer survives the callback. The FIFO publication discipline
adapts the existing reviewed AsyncHttpService fix, with no I/O under its lock.

Separate staged RGB storage prevents incomplete chunks editing committed pixels;
an explicit frame commit snapshots before later chunks, and abandoned partial
stages expire after500ms. The S3 output worker owns immutable current RGB and
preencoded internal-DRAM symbols, a capacity-one latest-complete-frame mailbox,
mandatory RMT DMA and a bounded100ms completion wait before buffer reuse. RMT is
initialized on core1 separately from Wi-Fi core0. Maximum legal2046 DMA symbols
respects the reviewed SDK descriptor4092-byte-per-half bound; an entire480-pixel
symbol frame would exceed it and was not blindly configured. Existing FastLED
3.10.6 WS2811 legacy250/350/650ns timings were read and retained exactly at40MHz,
with RGB order, TypicalLEDStrip correction,300us low latch and no temporal dither.
No SPI2/3 bus is borrowed from AVR ISP or future W5500. Initialization/transmit
failures remain visible, with no silent non-DMA fallback or in-flight overwrite.

Idle is removed from Table advertisements/client selection without renumbering
the remaining mode codes or changing the deferred AVR firmware. A runtime LED
output service defaults On each boot; explicit Off queues a full black frame,
then sleeps while settings/mode remain accessible. REST /api/led/output, MQTT/
WebSocket command led-output with boolean enabled, USB opcode0x5C, native CLI/
TUI led-output on|off and Telnet expose the same setting. Native/Web/mobile show
the switch only when the current peer actually reports enabled; missing is not
treated as enabled. Driver ready/active/transmitted/failed/revision/error_code
are separate from the requested switch and the renderer command-buffer revision.

Focused actual renderer and20000-message concurrent FIFO fixtures pass with
Arduino/FastLED doubles, including bounded overflow, copying, all480 pixel
encoding, complete black/latch, static refresh, disable/re-enable and abandoned
partial isolation. Web mode/context and S3 source guards pass. These do not prove
SDK compilation, actual DMA, optical waveform or Wi-Fi contention. SDK build,
app checks/deployment and live wire-worker observations are pending at this point.
Physical user confirmation remains required before marking the glitch fixed.

Independent safety review then found and fixed two additional ownership paths:
legacy PC0xFC commits now validate staging owner and return Busy on refusal,
and successful raw stream commits retire pending cell fades without repainting
their targets over the raw frame. Wrong-owner/expired-commit and old-fade
regressions pass in actual LedGrid/PcLedContract fixtures. Continuing raw commits
while output is disabled update logical RGB only; they do not repeatedly submit
black. PKAR/WS/UDP owner tags prevent cross-source staging, but legacy PC-contract
transports still share owner0, and the existing PKAR header has no frame sequence
to reject late/reordered chunks from the same source. These remain explicit gaps.

Actual SDK disassembly showed RMT DMA ISR cache safety is not enabled in this
bundled configuration. Flash partition write/write_raw/erase wrappers therefore
acquire a bounded150ms full-frame quiescence hold, nest safely across tasks, and
refuse unsafe ISR/cache-off/suspended-scheduler contexts before blocking. They
cover current LittleFS/NVS/OTA application paths, not lower-level panic dump
writers. No shipping direct esp_flash or ESP.flashWrite bypass was found. Actual
worker host fixtures pass functional/timeout/stop-failure cases, including2000
concurrent submissions, immutable in-flight encoding, stable refresh, black once,
resume, nested holds and failed DMA stop refusing flash. Actual linked wrapper
fixtures pass failure/error propagation, unsafe contexts and2000 concurrent calls.
These are source/double proofs; final ELF binding and physical coexistence remain
independent gates. The initial SDK compile caught FastLED CRGB memcpy ADL
ambiguity; qualifying ::memcpy resolved it rather than altering SDK sources.

Async WebSocket dispatch retains client IDs, never a raw client pointer across
tasks. Admission/session maps have short locks with SDK send/close outside them;
the actual middleware fixture passes20000 concurrent lifecycle operations and a
reentrant-close lock test. Existing credential-record access paths outside this
slice still need a complete auth synchronization review before broad concurrency
acceptance. The Linux fixture now links pthread explicitly.

Native guarded Check and Release pass: version0.2.7,25529344-byte executable
SHA91a3ea57c7ae6cf16bc1577f5bf3d54bdfe187267141216766354a86ce29c6a8.
Guarded stable staging plus CLI identity probe pass. Full native suite deferred
per incremental user policy. Mobile analyze and focused LED4/peer5 tests pass;
initial test-only Map identity matching was corrected to deep-map assertions.
Preview APK0.2.7 is55396153bytes,
SHA38bd2a0edad8b3fb3fb4e995d5e6f35876a84023d29de074ad352f5b740af423.
ASUS ADB currently has no devices, so no phone installation/launch is claimed.
Web generic-LED placement, live Persian Output localization, pending interlock,
unknown-state hiding and post-readback behavior now execute in the focused DOM
fixture; not browser/human acceptance. Generic non-table Flutter placement and
programming-latch admission parity remain pending before resumed AVR integration.
The private rebinding helper initially used an incorrect manifest property;
the comparison gate caught the stale test hash. Corrected to actual files array,
rebound reviewed changed targets only and retained every original donor hash.

### Follow-up: reconnect, unavailable preview/mapping and requested R/B swap

The user then explicitly requested reconnect and pixel/mapping fixes plus a
red/blue swap. S3 wire encoding now defaults to BGR, configurable at build time
with PAKAR_S3_LED_SWAP_RED_BLUE=0 for RGB. Logical API/effect/preview RGB values
and per-logical-channel correction remain unchanged. Actual worker fixtures
pass functional/timeout/stop-failure in both profiles. A rerun exposed a fixture
race: its live flag could identify an already-completing frame rather than a
driver wait stopped at the test barrier. Generation-specific barrier ACKs now
make the forced-quiesce-timeout test deterministic; no production timeout was
increased. Ten focused runs of the corrected scenario passed.

LedGrid now keeps a distinct immutable last-published RGB/map/brightness snapshot
through render-dirty, partial streaming and disabled-service states. Its optional
pending flag distinguishes a newer uncommitted draft. This removes the actual
null-preview starvation path without exposing uncommitted pixels or implying
physical transmission. Actual renderer fixtures verify snapshot/revision
retention and subsequent publication. The API mapping response now checks a
complete worst-case escaped-name reserve and returns503 rather than attempting
truncated200 JSON after allocation failure. Mapping POST atomic persistence and
stricter name validation remain separate pending hardening.

AsyncHttpService now dispatches occupied slots round-robin: first-free producer
reuse can no longer indefinitely starve older mapping/status requests behind a
continually refilled low slot. Deterministic seven-older-request fairness and
existing20000-operation lifetime/publication fixtures pass. This is one concrete
timeout path, not a claim that all intermittent HTTP failures are fixed; SDK
DNS/TLS/MQTT calls can still block the loop longer than a client's read deadline.
Hostname-first ASUS reads of the old13acc69e... boot returned actual valid80-cell
mapping and complete2880-character RGB in281–344ms; these did not prove the new
firmware was installed or that a physical reset caused the client reconnects.

The guarded final S3 build passes for16MB/OPI PSRAM, buffered UART profile with
normal AVR link paused:1773600bytes,
SHAc48201f02e2e52046d3dc3e0f7c5a5935acb2263291b3ffb24fb067aec917c43,
ELF494d5a33504f010f26440f3d16e74952024f157b29ff1b1fdde6718e11155a08.
Independent final map/disassembly verifies all three partition wrappers in IRAM,
actual LittleFS/NVS/encrypted/OTA caller rewrites, context/acquire/real/release
flow and unchanged lower-level panic flash APIs. Not hardware coexistence proof.

The Rust reconnect fix retains a fresh same-peer view after transient HTTP loss
rather than clearing it while WebSocket is healthy; eight-second peer freshness,
preflight/epoch/write fencing and explicit Disconnect remain. Null pixel samples
retain only their original five-second age, never renew it; invalid data, changed
mapping and revoked peers clear it. Mapping reads validate80 unique physical IDs,
coalesce pending requests and retry initial failures with bounded backoff.
WebSocket recovery/backoff resets only on an acknowledged actual status, not TCP
upgrade, and write waits are bounded. Guarded Check/Release and, at this
significant milestone, one guarded native Test pass186/186. New executable is
25562112bytes, SHA557cc3d2cb695f539bcc182d104df9b942c3e019efcb69fb35f8c1370330015e.
Source stability and stable stage/probe pass; actual replacement/readback remain
separate. Six preexisting compiler warnings remain, not new clean-build claims.

The first Internet Fetch lost its direct curl52 response. Independent actual SDK
inspection showed closed TCP makes a queued weak request ineligible to execute;
the loop-served job remained old7 (queue increments ID before any yield). After
that explicit outcome reconciliation, one native CLI Fetch returned new job8,
then exact Ready identity/length/hash was observed. The first Apply helper stopped
before dispatch because native info returned telemetry-error, not a board object;
its failed receipt is retained. Correcting read-only preflight to actual status
kept the only Apply in the verified native CLI. That Apply acknowledged prepare;
same-board new boot85c496400eebcf04b5b57540724914b8 reports Complete/c482...,
DMA ready/active, BGR, failed0/error0 and all480 preview pixels. No USB/AVR writes.

This first deployed scheduler did NOT pass the bounded HTTP-load test:12 valid
status/mapping pairs,9 empty replies and no LED mutations before two consecutive
read failures stopped it. Actual wire counter advanced1341 to3539; this separates
working driver service from unreliable API delivery. Do not check reliable
reconnection/Off/On/optical acceptance from this observation.

Further actual ESPAsyncWebServer3.12.1 inspection found a precise response race:
its AsyncTCP _onData calls _send after middleware returns; our loop could already
call request.send, while _sent is set only after response._respond. ACK/poll and
response fields were likewise not protected by the application's queue mutex.
AsyncHttpService now constructs independent owned responses in loop and publishes
them into a bounded per-request mailbox. A public AsyncClient.onPoll event sends
only on the SDK's network actor; the installed SDK's exact _onPoll ACK progression
is retained. Weak request ownership, disconnect cancellation and destructor
cleanup avoid dead connections or leaked unsent responses. No SDK/global-library
patch, forced delay or arbitrary sleep establishes safety. Peer IP is captured
at admission instead of dereferencing the mutable PCB in loop. The two deferred
raw send sites now publish through this adapter; native SDK-context OTA/auth/file
routes retain their direct sends. Two deferred AVR backup downloads no longer
explicitly close the File handle shared with an asynchronous response; actual
ESP32 FS shared_ptr destruction and AsyncFileResponse ownership were inspected.

The focused actor fixture models the real outer SDK send tail, late sent flag,
network-actor-only response/ACK operations, disconnect/duplicate cleanup, fair
dispatch and20000 concurrent publications. It passes; WebAccess tests now use
fresh requests for independent authorization probes and explicit network poll
events for queued responses. The initial reused already-sent fixture failed,
was corrected without relaxing production authentication, and passes. Real SDK
compile/deployment/load confirmation for this actor is still pending here.

The telemetry failure has a separately reproduced cause: ArduinoJson's actual
ArduinoStringWriter clears its destination, unlike the old generic host write
stub. ClientTelemetry.json serialized directly into an existing array string,
losing its prefix and previous clients. It now serializes a bounded entry, then
explicitly appends it with length checks. The fixture enables the real Arduino
String writer: old code fails, new empty/one/eight-client arrays, failed writes,
TTL, observed-IP distinction and secret allowlist pass. No blind heap/snapshot
limit increase was used. Production telemetry and actor coexistence still need
their own live proof; maintenance-admission counters, credentials/Telnet task
ownership and blocking outbound DNS/TLS remain independent concurrency gaps.

### Critical optical regression and emergency stop

The user then reported rapid visible flashing on current c482.../boot85c49640...
while the replacement actor image was being staged. One authenticated pinned
Off request was acknowledged and read back enabled=false/driver.active=false.
The user confirmed flashing stopped but some pixels remained lit: no all-black
or fixed-waveform claim is made. Job9 Ready/bc624443... was explicitly cancelled
through native557c/PID19088 and read back cancelled; it was never applied.

Actual SDK review found the missing coverage: esp_partition_read/read_raw and
esp_image_verify's bootloader_flash_read path can disable cache too. The installed
GDMA refill interrupt lacks the cache-safe registration; a2046-symbol two-half
DMA buffer still needs refills about every1.28ms during a14.7ms480-pixel frame.
The higher RMT interrupt priority does not configure the independently allocated
GDMA refill interrupt. This is a concrete scheduling defect and strong causal
candidate, not a measured waveform diagnosis. Dynamic PM clock changes were
excluded by the actual SDK config; timings match the installed FastLED legacy
WS2811 setup. Increasing RMT memory beyond its two4092-byte descriptors is not a
valid full-frame DMA fix.

Added task-context read/read_raw/full image-verifier wrappers with the same
frame-boundary pause. Guarded FirmwarePlatform metadata helpers cover Arduino's
direct getSketchMD5/flashRead and flash-mode reads; sketchSize now initializes
metadata and checks verification failure instead of reading an uninitialized
result. Lower-level panic esp_flash APIs remain untouched. Actual GNU-linked
fixtures pass all six guarded boundaries, denied acquisition, unsafe contexts,
original errors, nested verifier/read/metadata holds and8000 concurrent accesses.
Focused actual RemoteFirmware fixtures pass on both ESP8266 and S3 using narrowly
updated SDK doubles, without weakening existing assertions.

Next quiet SDK build uses PauseLedOutput; its standard source default remains On.
Quiet begin queues one black frame while keeping scene and service Off, so the
next image must not automatically resume a potentially flashing effect. SDK/
linked-artifact and physical black-clear acceptance remain pending at this point.

### Static six-pixel fallback requested after emergency stop

The user explicitly requested a non-DMA default, immediate identical values for
each six-pixel cell, no animation or transition, deployment and an explicit
rainbow test. This supersedes the prior quiet-only commissioning plan. Consulted
games-streaming and reread actual Puzzles LocalLib/WS2811.h and WS2811.cpp: retain
its FastLED WS2811 timing/correction and full brightness intent, but its AVR
11-pixel driver is not a suitable 480-pixel S3 transport implementation.

LedGrid now defaults to static mode, applies cell changes immediately, normalizes
each physical six-pixel group to its first pixel (including streamed commits),
and does not refresh unchanged scenes. Explicit rainbow in static mode is a
stationary grouped color test; effects is an explicit runtime opt-in. The same
led_output.mode contract is being wired into API and clients. The focused actual
LedGrid fixture verifies immediate pulse requests, identical grouped pixels,
stationary rainbow, grouped raw input and no unchanged retransmission. Existing
effects-path tests remain explicitly selected and pass. These are source/fixture
results; new backend SDK, deployed image and optical evidence remain pending.

User correction before deployment: rainbow test must animate, including with
static grouped cell control selected. The renderer now explicitly animates that
test while retaining equal six-pixel groups; ordinary cell changes remain static.
Repeated Off requests now enqueue black even when requested state is already Off,
instead of returning early and leaving WS2811's latched garbage untouched. Native
UI has an explicit Off action to resend it. Focused renderer checks cover repeated
black and animated grouped rainbow. Compiled-object inspection also caught GCC
inlining the IRAM pulse routine into flash-resident work(); noinline is required
and the superseded image must not deploy. These corrections require one warm
firmware rebuild; no CI wait or full test rerun is a prerequisite for user viewing.

## AVR recovery resumed

User explicitly resumed hardware UART and ISP restoration. Actual PCController
`Project/ResetTelemetry.cpp` and `.h` were completely read, with EEPROM layout,
codec and reset-fixture dependencies. Root verified the donor checkout at
`b0111ae0c80dc34149ea1044de7dd5488920301b`, with raw source SHA256
`8699f6cb75170fb62ee94e24ce0faa74bdfb51021b9eba6fb95c61013619199c`
and `7ad43276b93bc31248848ad80cb4c2f5f40500dd33bf736ddeb6c2d5114256a9`.
The existing comparison ledger records both actual imports; its new `avr-reset`
route prevents this startup capability being missed by broad hardware keywords.

Adapted early MCUSR/r2 capture and watchdog disable, journal416..799 with
commit-last CRC/readback and quiet watchdog/brownout/unknown welcome policy.
Optional B5 adds reset flags/count/persistence without changing46-byte B1.
The buffered AVR build passed at31628flash/1406static-SRAM. Final ELF
`c6d916c3c04c85af668c74845f354a16c61660d76ba40592afbe35befa9157e7`
contains the intended early instructions and fall-through. Raw32KiB image SHA256
is `8e06ffe3b286012ed1fd743c1d93166349ac01bc83d71c8ff5e45a8bae16db9a`.
Focused actual reset-journal, packet/identity and ISP EEPROM-preservation fixtures
passed; no broad suite or CI wait was used. These are source/host/compile results.

S3 source now uses its existing dedicated UART1 adapter instead of duplicating
unchecked initialization. It reports driver-ready/hardware errors, requires
valid AVR identity before online state and avoids the legacy18ms drain delay
for buffered hardware UART. GPIO assignments, AVR hardware serial, original ISP
backup/byte verification and fuse policy are unchanged.

ASUS localhost:8022 timed out before SSH login; local ledboard.local resolution
also failed. No physical AVR probe/backup/program or audible action was possible.
Recovery remains open until access, verified backups and both live paths work.
The user subsequently stopped the private evidence PowerShell helper: its last
run had finished; it must not be invoked again or replaced with another wrapper.

## Basic White/Red delivery

Delivered `/basic` on the integrated S3 firmware, with80 logical buttons and
explicit White/Red controls using the existing shared peer HTTP implementation,
application artwork and palette. Actual RayanLamp `webui/src/api.ts`, Pealayer
`web_ui/src/appearance.ts` and PU850 `LocalLib/AsyncWebServer.h/.cpp` were consulted;
this bounded delivery is not donor-wide UI parity.

The first page request failed with an empty reply. Reviewed installed SDK3.12.1
`ESPAsyncWebServer.h`, `WebRequest.cpp` and `WebResponses.cpp`: ESP32's deprecated
char-pointer `send_P` selects a heap-backed BasicResponse. Serve this immutable
page directly on the native SDK actor with explicit bytes/length instead. Hardware
mutations remain queued; do not restore loop-side network sends. Direct ASUS GET
returned200/18672bytes. Local forwarded-browser failures remain a separate open
transport/browser acceptance item; neither their cause nor a universal fix is claimed.

USB application-slot flash verified image SHA256
`f5572b0b5cf8d750f00390ad25c1c53e8699410d0086487bc66e9a174cdc82da`.
Live boot `d62f69605c125734372ebd96d8cb3c7e` acknowledged Cell mode;
static `gpio-static`, DMAfalse and all80 white were read back. Shipping controller
code then performed cell1 Red and White against the actual board from ASUS:
both POST202 and subsequent state readback succeeded. This was a Node transport
integration using the actual app code, not a browser-click or optical/mapping test.
The focused controller fixture and S3 build passed; no broad suite/CI wait ran.
ASUS has the hostname-based desktop shortcut. The one requested PBX call completed
with archived Completed receipt and matching ANSWERED CDR,30 connected seconds.
Physical cell placement/colors and browser interaction remain for live acceptance.
Final direct readback matched running sketch MD5
`4297dbf58cf1aa6a36c3b79186db241e`, same boot, Cell/none,80 white/0 red,
11 transmitted frames and0 reported driver failures. The isolated source
checkpoint still needs reconciliation with preserved concurrent working-tree edits.

## ASUS native USB-only delivery

The prior compact USB packet did not contain all80 cell states or mapping. Added
a frozen, bounded nonce/token/offset snapshot and exact-boot-fenced table commands.
The reviewed PCController Session admission/matched-ACK/no-replay mechanisms are
adapted in `desktop/src/usb_table.rs`; UI connection epoch, physical-open session,
peer pin and enqueue time are captured before queueing, never rebound by the worker.
Capture-start time remains the freshness authority. Windows serialport4.10.1 uses
first-byte COMMTIMEOUTS; its60ms timeout is not a per-chunk fixed sleep.

ESP application image SHA256
`34e685d5191c62fbb4a69739a08d9053015a260731babc6c25d27a867c15089b`
was written through ASUS COM22 to the previously verified application slot0x10000,
with esptool byte/hash verification. AVR firmware/EEPROM/fuses were not changed.
The boot stayed `2143183069bb9f814ab1cd714a2da268` across app USB reconnect.

Native candidate SHA256
`4c49835d24c96b3663ecfc7276095557436ef6ffa23212a05d3e31f859b3ced8`
is running interactively on ASUS as PID18680, version0.2.7, one process, COM22 USB,
WebSocket disconnected and no established TCP connections. It reads80 real cells
and80 mappings. The preceding USB candidate performed cell1 Red then White with
matched ACK and new same-board snapshot readback. Disconnect cleared live state,
an offline command was rejected, and reconnect returned80 White with no replay.
This is live protocol evidence, not confirmation of physical placement/color.
The final UI adds readable semantic state circles when individual RGB is unknown;
the user clarified logical11 is right/half below1. Native and mobile now interleave
paired logical rows for presentation only; physical mapping and API IDs stay intact.
Actual final framebuffer capture: outputs/PakarController/native-usb-ready.png.
The old browser shortcut was preserved in staging and replaced by a native shortcut.
Settings, original executable and the working USB rollback remain preserved.

All196 guarded native tests passed after correcting the TUI's stale Down-from1
assertion to21; that final correction is test/comment-only after the runtime build.
Flutter analyze and5 lifecycle/readback tests plus2 geometry/phone-panning tests
passed. Preview APK SHA256
`2fa5d2e296bd6bdf948cc0adc2442ab2906cf4d3326429049b4dba1cf3557364`
was built and checksum-verified on ASUS. Neither host lists an ADB device; phone
installation/touch/physical mobile acceptance remain open. Android direct USB is
not implemented or presented as operational, and no network fallback is claimed.

Rainbow was restored by the actual ASUS native USB path at the user's request.
Same boot, gpio-static/non-DMA, output enabled, increasing transmitted counters,
no reported driver error. The user reports glitches/rough movement: acceptance is
OPEN. Source shows frame-count-based hue increments and six-pixel grouping with a
large inter-cell hue jump. This establishes animation roughness in firmware, not
that all corruption is software; GPIO counters cannot validate the wire waveform.
No further driver/physical-mapping changes were made from that observation alone.

These are preserved-working-tree candidates, not a clean reproducible release or
complete donor/production acceptance. Master issue2 remains open.

### USB RGB/effect follow-up — deployment pending

The later user request to remove the Wi-Fi telemetry limitation is implemented
as actual480-pixel published RGB in the same frozen USB capture, not a text-only
change. Firmware/client bounds are16KiB. Existing Frame validation, capture-start
age, current peer and confirmed mapping gate preview/tooltip use. USB also updates
the observed effect; the panel has an explicit selected Rainbow test control.
New native candidate SHA256
`f800bd10b0c4982e31ed4c945bbc83a12327d4cb2cdb33ed61375c071a5693ef`;
new firmware SHA256
`0528a38523f3fa76fa363dee09b056bb6fa8bc943d8ff707de6fd939d80f2067`.
The new source passes197 native tests and the S3 build. A test-only serde Value
construction correction followed the release build; shipping behavior is unchanged.

LedGrid adds RAM-only actual-pattern-transition attribution with constant reasons,
previous/new pattern, uptime and then-published frame revision. It changes neither
wire timing/defaults nor EEPROM/flash. It cannot identify individual API clients
or distinguish an AVR caller from another caller of the same grid method.

Neither new candidate is deployed: ASUS tunnel8022 alternated banner timeout/reset,
then lost its listener/refused connections. The existing USB app remains the
previous4c49835d candidate; the final rainbow retry was only IPC-accepted before
confirmation became unavailable. Do not claim current optical effect success.
The owned untriggered PakarControllerUsbControl task still holds that last explicit
rainbow-and-status action; inspect and reset to status-only before any blind start
after reconnect. No automatic replay or new PBX call is authorized by this note.

### GPIO4 stop investigation and static recovery

ASUS8022 returned. The actual selected-USB app first observed boot
`0c4babc5dee21e8d343747a12a54abe3`, effect none, enabled gpio-static, no flash
pause/driver errors. One explicit native rainbow command was followed by about
1490 completed frames and then effect none on the same ESP boot. This proves a
logical cancellation occurred, not its caller or an electrically correct frame.
AVR input9 was true; a held input alone does not prove repeated press edges.

The already-built0528a385 diagnostic firmware was application-slot flashed and
byte-verified on ESP90:70:69:ea:c8:00, and matching nativef800bd10 was installed
gracefully with current settings and previous binaries preserved. The older
4c49835d app rejected the larger USB RGB snapshot; pairing the already-built
client restored complete table/mapping state. Fresh live snapshots on the new
boot413ed378 show rainbow selected and243,1947,4925,7477,10880 submitted frame
revisions, completed counts one higher, no driver failure/pause, and actual2880
RGB hex characters over USB. No Wi-Fi transport fallback or optical acceptance
is inferred from these reports. No AVR programming or audible test was run.

Read the actual Puzzles LocalLib/WS2811.cpp and header completely again; their
original hashes remain0f7fec78 and39509602. They initialize FastLED/brightness/
black, but do not solve delayed external strip power. The shared LedGrid's
static early returns plus the fallback worker's queue wait explained why an
unchanged static scene never sent again. Added an enabled-only1000ms full-buffer
maintenance resend to needsRender/service, without stepping fire/fades, changing
cell values, adding persistence, or limiting rainbow throughput. Exposed its
interval in led_output. Focused actual-renderer/FIFO fixtures pass, including
999/1000ms boundary, identical RGB, static-fire nonadvancement and disabled-output
behavior; the guarded S3 build passes (1826727 sketch bytes,90052 global bytes).
Candidate image62d26c0f96c67d3f461e3f96df03046acca5cb98fd43d549557e0a29ce490f76.

Read-only review also found old AVR action/reset revisions survive a HELLO/new
session and can replay phantom deltas after a reconnect. This remains a distinct
source-confirmed defect, not today's proven cancellation cause; no speculative
AVR or session behavior change is hidden in the static-refresh fix. Live waveform,
visible colors, all480 pixels and the user's original logical mapping remain open.

### Explicit RMT-DMA run

The user then requested rainbow using RMT and DMA. Added canonical wrapper option
`-S3LedBackend rmt-dma`, passing PAKAR_S3_LED_USE_DMA=1 to the paired S3 build.
Static remains the default; other targets reject this S3-specific selection.
The same cache/output path is reused without clean. Added Plan assertions for
the explicit flag/default/target restriction; that broader script suite is
deferred, not reported as executed before deployment. Changing the global
compiler flag required a full dependency rebuild; no duplicate cache was made.

Guarded build passed (1826843 sketch bytes,90036 global bytes). Image SHA256
94da0453188c3b07a1ab1ebc6a0e651887a18dcfeb820b3b3996d980d4fadaa4 was copied,
hash checked, application-slot flashed and byte-verified on the same COM22 S3.
Non-DMA62d26 remains available in ASUS staging/usb-table-refresh.bin. Current
nativef800bd10 was gracefully restarted without changing user preferences or AVR.
Fresh native USB readback on boot6b13a6ab5f9b383db8ca671e76b949ec reports actual
backend rmt-dma,dma true,ready/active true,failed/error0 and transmitted1569 then
3451. Full effects mode was explicitly commanded through the app and ACK plus
same-board readback verified before the separate rainbow request. Station link
remained connected; this is not a high-load Wi-Fi or optical acceptance test.
Subsequent fresh same-boot native snapshots verify rainbow/effects and RMT-DMA
with submitted1055 then2632, completed5695 then7273, failed/error0, no flash
pause, actual480-pixel RGB previews changing from06fd0101ef10... to9a0166ac0153....
Both commands have ACK plus same-board readback. Current task is status-only,
not a retained mutation for replay. The app remains USB-only; AVR remains
unprogrammed and no audible/PBX tests were performed. Visible smoothness and
electrical correctness await the user's observation; master issue2 stays open.

### Atomic cell updates and Flutter selection/network controls

Implemented one board-owned selection mutation across ESP HTTP `/api/cells`,
cloud JSON `cells`, authenticated Telnet `cells`, native GUI/TUI/CLI, USB opcode
0x5F, Flutter and both Web controllers. The full80-cell selection is validated
before mutation; the12-byte USB bitmap fits the existing boot fence. One common
start timestamp drives300ms wall-clock fade/pulse on the ESP; delayed rendering
catches up rather than requiring client frame uploads. Static grouped fallback
deliberately remains immediate. No EEPROM/LittleFS scene write or80-command AVR
fan-out occurs. Explicit `cell-batch` acknowledgment is required; no per-cell
fallback, optimistic success or automatic uncertain-command replay. Every
selected target state requires matching-board post-ACK readback. This does not
prove completion of a fade or physical light output.

Flutter adds Select all, Invert and Clear using the same normalized Find
selection, transition choice and live AVR-advertised melody chooser. Snake is
hidden across clients and excluded from the default firmware mode manifest;
implementation remains behind PAKAR_SNAKE_ENABLED. Successful hostname addresses
survive reconnect/transport retirement; real lookup failure, not HTTP/auth errors,
evicts a route. Missed discovery scans retain explicitly unverified records.
Actual PCController lanresolver and netpolicy/proxy source/tests were read fully,
adapted and bound in the existing ledger. Actual canonical AddressableLeds.cpp/.h
stage/show ownership is adapted, not its AVR timing sender or old100-pixel limit.
PU850 Shell.cpp/.h were reread fully at their original hashes; Telnet now forwards
the correct audio `cue` field and uses escaped JSON for the new bulk command.
Generic shell registry/broadcast/serial-shell parity remains open.

Failures were investigated rather than bypassed: obsolete queue translations
were replaced and canonical assets regenerated. The widget binding intentionally
blocked real HTTP with400; transport fixtures were isolated using the existing
mirroring transport pattern. Their next timeout exposed an actual proxy defect:
ledboard.local went through the dependency-download proxy (zero local lookups).
Imported donor LAN/private/link-local proxy bypass while retaining remote proxy
policy. Actual Dart SDK custom-factory code also required explicit direct TLS;
SecureSocket.startConnect now preserves hostname validation and cancellable SDK
ownership. Public localhost test-only certificate/key are not APK assets. Real
loopback tests verify original HTTP Host/cache reuse, untrusted/wrong-host TLS
rejection and explicitly trusted fixture success; no bad-certificate bypass.

Verification: guarded S3 RMT-DMA build passes (1833047 sketch bytes,90676 global
bytes);202 native tests and native Release/stage probe pass; Flutter analyze and
seven selection/cache/melody plus three actual transport/TLS tests pass; Android
preview release APK builds. Focused Web table/basic/context tests and complete
protocol wrapper pass with actual renderer/FIFO/worker/ownership host doubles.
The native read-only fixture's first transient failure did not recur unchanged;
the separate batch fixture was corrected to require the real accepted count.
Comparison generation/Check and configured mobile-network gate pass. The broad
firmware-hardware lookup has no acceptance gate configured and correctly refuses
`-Gate`; it is routing, not a claim that all hardware donor parity is certified.

Checksum-verified candidates in outputs/PakarController/staging:

- PakarController.exe:8f176ca8ad5d0c71ee2b4094125adf5681b7c630cdb5072101a70192cf138084.
- PakarController.apk:1070e6c34ae6216ed010ef5da7bbab949c63a6cf5a4be85b248ba3b13860be19.
- PakarController-esp32s3.bin:75f2b18280fc494d8a251d33bc691244f1669c555f68cde97950095f7eca4aef.

These remain preserved-working-tree candidates, not clean-main/release promotion.
ASUS SSH8022 refuses connections and ledboard.local is unavailable here; no
current candidate was flashed/installed on ASUS/phone. Local ADB lists no device.
The previously deployed94da045 RMT image is not replaced or silently switched to
another backend. AVR programming and all audible/PBX tests remain held.

Critical remaining gate: SDK5.5.5 RMT DMA is interrupt-refilled ping-pong, not an
autonomous full-frame transfer. Existing immutable buffer/loop-owner tests do not
establish refill-deadline safety or optical stability under Wi-Fi. Bulk atomicity
fixes staggered client requests but is NOT claimed to fix all green flashes.
Deploy the matched candidates after access returns, then verify USB and direct
Wi-Fi80-cell batches, clean-white-to-red fades, all480 pixels, mapping and actual
visible stability. No production/hardware checkbox is closed; issue2 stays open.

### Quick stress/benchmark continuation

Host fixtures pass static/DMA x BGR/RGB (2000 mailbox submissions, timeout/flash
holds;8.45s wrapper including compilation), plus async HTTP20000 concurrent
publications accepted200 or explicitly rejected503 (2.955s wrapper). Actual
renderer10000 full80-cell batches per mode: static10.759ms total/p95 0.900us,
effects21.076ms/p95 2.100us. Desktop CPU/FastLED doubles, NOT ESP/API/FPS evidence.

Fixed two reproduced issues: an abandoned partial stream repainted unrelated
committed raw pixels from stale logical targets; Rust manager could confirm
another same-sized batch. Regression first fails then passes in both renderer
modes; immutable operation ID+exact selection/state/effect+peer+route now crosses
HTTP/USB admission/result, rejecting foreign/identical-but-different-ID/retired
receipts and altered payloads.202 native tests, Release/stage and S3 RMT-DMA SDK
build pass. Cheap optional free_heap_bytes status avoids sketch hashes/flash holds.

Added bounded hostname-only tools/benchmark-led-api.mjs:8seconds each static/
effects, two5Hz readers and one5Hz writer on cells1/80, strict boot fencing and
ACK/count/all80 readback, cancellation/no replay, response bounds/85s deadline.
Reports HTTP latency/throughput, minimum observed heap, submitted/transmitted/
revision/error deltas; frozen repeated frames are failure, not FPS. Success
restores logical targets/pattern/mode, NOT raw-stream pixels or animation phase.
Real local HTTP tests cover both backend profiles, bad count, changed boot,
read failure, driver mismatch, stalled transmission and frozen revision. Fixed
a deadline-edge pacing burst caught by that fixture. Added it to existing CI.
Read actual PCController http_url.go and dependency network.mjs completely;
adapted URL bounds/secret-safe diagnostics, rejected dependency retry semantics
for hardware mutations. Existing ledger/led-benchmark gate reflects exact hashes.

Live ledboard.local run failed preflight after3005ms, zero completed writes and
zero mutations. SSH8022 refuses; no local8022 listener or S3 USB device exists.
No physical/API throughput, optical success or deployment claimed. New staged
firmware15b00358308ca3acd8eea1ebd05db6215c22020b2f7af31460264e3386880fbc;
native5cb4ad2e3cd1c4433a150c8cba11d655ef7f51dfb5497abdf9c50cc66a3704f9.
APK/caches/settings/rollbacks preserved; no AVR/audio/PBX contact. RMT refill and
green-glitch physical acceptance remain OPEN on issue2.

### Standalone Internet and Ethernet continuation

Actual ESP boot already loaded network/cloud settings without PC/USB ownership.
Fixed STA-only readiness in cloud, NTP, DNS and discovery using NetworkPath;
ready Ethernet is preferred, station is fallback, AP stays local, and route/IP/
gateway/DNS changes revoke old cloud sessions. Native W5500 uses isolated HSPI/
SPI3, not AVR FSPI/SPI2. Build feature defaults off. Separate verified/pending/
rename Ethernet DHCP/static persistence never overwrites the Wi-Fi profile;
HTTP /api/network/ethernet and cloud ethernet-config/configure-ethernet expose it.
Dedicated graphical Ethernet editors and physical discovery remain pending.

CloudConfig imports the actual NetworkConfig no-op/pending/readback/rename
transaction. Shared typed cloud/network/Ethernet handlers save copied candidates
before active RAM publication and defer transport changes after receipts. Remote
commands use bounded board/boot/deadline/replay admission before dispatch, with
typed top-level normalized JSON closing duplicate/nested-key parser disagreement.
Status streams in256-byte MQTT writes without enlarging the2048-byte RX buffer.
Existing cloud sessions carry update progress/cancel without new TLS handshakes
or full captures during maintenance; boot-image telemetry identity caches once,
while update integrity checks remain fresh. ESP-only OTA is AVR-independent when
unlatched; paired backups/Prog/version gates remain. Cached original availability
topics now get offline on graceful changes; failed writes preserve broker LWT.
Stable hardware MQTT client IDs and high-uptime/long-offline retry tests added.

Focused actual-source host fixtures pass for ESP8266/ESP32 cloud config, cloud
services, command admission/typed getters and settings; Ethernet off/on adapter,
route/discovery and management tests pass, plus independent Ethernet storage
fault tests. Protocol runners include these checks on Windows/Linux. Two stale
DNS/TUI source guards were re-reviewed against actual shared route/batch code,
with negative mutants rather than reduced coverage. Ledger -Check and living
API guard pass;142 complete hash-bound comparisons,4531 pending are unchanged.

Frozen-source ESP32-S3 SDK3.3.12 buffered/RMT-DMA/16MB/OPI/W5500-enabled build
passes:1948979 bytes flash,97720 bytes static RAM. Image SHA256:
9deb77baa18b188e6a10e7514215fe33659b864fe14564d0f66d20bf8343ccee.
This is a working-tree optional-Ethernet build, not a clean-head production
release, staged Wi-Fi-only replacement or a deployed board image. Native/mobile
packages, settings, rollbacks and warm caches were preserved; no unrelated build.

Office broker TLS certificate verification, operator MQTT authentication and
ledboard status subscription pass. Only three Pakar telemetry/part ACL grants
were added after exact baseline hash comparison, isolated config parse validation
and rollback preservation. Broker remains active after HUP. Active ACL SHA256:
65efddef57a02bb6d5dfb584fcf34a8ea83fb4d948d86de2845098e250bd962f;
rollback /etc/mosquitto/acl.pakar-standalone-rollback retains2991371e... .
Temporary validation config was hash-verified then removed; passwords unchanged.
Mosquitto2.0.22 warns shared passwd/ACL ownership will need future migration;
no unrelated ownership/service upgrade was performed.

ASUS SSH8022 still refuses. A bounded authenticated Internet read-only status
request got no fresh or retained board record within12seconds. Broker proof is
NOT ESP cloud/physical proof. Actual ESP deployment, PC-unplugged commands,
telemetry, remote settings, Internet stage/apply/reboot/recovery and W5500
DHCP/static/cable-loss/multicast acceptance remain OPEN on issue2. No AVR/audio/
PBX action and no hardware success inferred. Pre-existing working-tree changes
were preserved rather than swept into a blanket commit or release.

### Flutter physical table stagger correction

Kept the display origin at cell1, with11 to the right and half a cell below,
and added alternating half-cell horizontal row offsets. Display coordinates are
separate from API IDs and the board's physical LED-chain mapping. The preview
uses1.5-cell vertical row pitch to prevent diagonally overlapping square touch
targets. Alternating-versus-cumulative row offset and exact physical row pitch
remain user-confirmation items; no physical geometry acceptance is claimed.
Canvas width includes the last staggered column; phone targets stay44px minimum
and pan rather than being squeezed. RTL does not mirror the physical table.

Consulted ui-shell and exact donor-audit entries, then read actual PCController
mobile/test/appearance_layout_test.dart, relay_layout_test.dart and
front_panel_scroll_test.dart completely. Reused the viewport matrix, measured
widget bounds, DPR/size teardown and isolated fixtures, not the donor's unrelated
relay/telemetry business. Added a narrow hash-bound appearance-layout comparison
with concrete remaining appearance/lifecycle cases rather than certifying the
entire donor. Existing main.dart source references were explicitly rereviewed and
rebound;143 comparisons are complete,4530 pending and107 have known parity gaps.

Canonical build-mobile wrapper analyze and six focused table layout tests pass.
All80 bounds, cell1/11 geometry, row offsets, no overlapping hit targets, phone
scroll/tap selection, independent reverse physical mapping and LTR/RTL phone/
tablet layouts are checked. The separate real-widget renderer capture passes
with zero mutations and was visually inspected. No repeated full suite, SDK
upgrade, clean, duplicate build cache, firmware/native build or board contact.

Reviewed source SHA256 (UTF8-LF):
- mobile/lib/table_layout.dart:3d4e3a476034e29fe197fe0d8eb704e64d57e3ecc164d0293c6fd7c8eb31cd69.
- mobile/test/table_layout_test.dart:00e5e159745a24dd53771a7a65a004a25cb0d9185b9ecc42534f3521f40a6647.
- mobile/lib/main.dart:2b4b39772724d58c38d51bc0e3ebf36e2711ea91ea06d33b1fc966bb128945f3.

Simulated outputs/PakarController/mobile/table-stagger.png SHA256:
075205e9f984d2cb894a5e75cdbb07069f6da91973b0ed82ef28819c9cc4e408.
Warm release-mode preview APK build passed in50seconds;55,494,457 bytes, SHA256:
ba89236aa89dfe1409a19b5e9fc80daddee6d6748a591a43e9db6a946605c749.
Updated only the owned stable staging/PakarController.apk after donor storage
gate, exact previous/candidate hashes and verified pending-copy/rename. Existing
production/rollback APKs, settings and warm caches remain untouched. This is a
debug-signed working-tree preview, not a clean-head production release.

Fresh ASUS SSH localhost:8022 refuses and local ADB has no device. No APK install
or phone screenshot was possible; phone/touch/physical-table acceptance and
issue2 remain open. No AVR, audio, PBX, broker or firmware mutation this pass.

### Unified Flutter directory and shared workspace redesign

Read the actual complete PCController mobile/lib/discovery.dart and
mobile/lib/devices_page.dart, its complete discovery.go, the complete Pealayer
PreferencesTab.tsx, current Pakar client/profile/NSD/XML/MNDP implementations and
affected fixtures. The existing ledger now maps both additional donor files,
their concrete replaced contracts, exact target/test hashes and remaining live
acceptance:145 comparisons,4528 pending and107 known parity gaps. This is not
whole-repository parity. Existing references were rereviewed and rebound; stale
Android discovery/build-wrapper hashes stopped the gate before compilation and
were corrected after reviewing the actual affected source.

Discovery no longer keys rows by endpoint+protocol. One directory merges saved
profiles and compatible observations using board ID, UDN, normalized MAC and
canonical IP/origin aliases; explicit conflicting identities remain separate.
Actual ESP DNS-SD instance/hostname correlation preserves a legacy saved .local
profile's relationship to the resolved IP even if ordinary Android DNS fails.
Different origins are publicly probed before coalescing so a failed hostname
does not suppress a working IP. Failed scans retire availability, not remembered
intent. Saved names survive and saved profiles not reached in the latest scan
dim while remaining manually selectable. Search, All/Available/Saved filtering,
compact source/status badges and icon-labelled profile actions share one list.

Adapted the donor's four-worker/deadline/source-pinned public verification to
Pakar's actual description.xml and capability-bearing status, not host healthz.
Generic HTTP/router MNDP records are filtered; MNDP must carry the actual Pakar
model/MAC fields. DNS-SD restricts generic HTTP to the advertised Pakar app ID.
XML/status probes send no credentials, follow no redirects, cap response size
and time, and never mutate. XML metadata verifies compatibility, NOT crypto
identity or hardware presence; fresh hostname/capability/board/boot checks still
own selected-peer control. Existing authenticated GETs now also refuse redirects.

New profiles save without credentials. Existing profile edits keep, replace or
clear an optional stored password; no plaintext/default-password fallback.
Connection first performs a read-only probe, prompts only after401 and verifies
the submitted login before save/selection. Discovery does not read the vault.
The selected-profile path no longer incorrectly invokes credential-rotation
readback on a newly selected open board. Rotation's authenticated readback stays.

Shared app_theme.dart/workspace_widgets.dart now own Studio colors, typography,
stable input borders, header/actions, grouped sections, menus/dialogs and badges
across Devices, connection preferences, device settings, firmware and bridge.
Bridge Session/MQTT/WebSocket/Routing navigation retains controller drafts.
The screenshot harness imports the actual application theme. The requested
permanent plain-HTTP warning was removed from embedded WebUi.h; server access
protections are unchanged. The embedded Web change is SOURCE ONLY, not deployed.

Focused canonical-wrapper verification passes:7 device/registry/widget cases,
5 real HTTP/XML/DNS-SD cases,3 credential-storage cases,7 existing bridge/discovery
cases and7 existing network/batch cases. Two renderer captures pass with zero
device mutations and were visually inspected. Initial narrow badge overflow,
optional-profile alias normalization and progress animation behind sign-in were
fixed rather than hiding failures. The existing batch fixture now settles the
focused field's bring-into-view work before measuring/tapping; no missed-tap
warning or assertion was suppressed. Analyzer and ledger -Check pass. Unrelated
formatter-only changes to generated assets/settings were exactly reversed;
canonical generator checks pass, no donor source was edited and no full suite
or CI wait was introduced.

Actual simulated captures under outputs/PakarController/mobile:
- devices-phone.png SHA256:37d3de6e23f1fc5bd6eaea30a0b1f1c5d4f9748fcab60243aa0922110507a863.
- bridge-workspace.png SHA256:5e7e2c060aea581116af4a75bc3f471955875bbc4081fda803d233c34da34bcd.

Final warm release-mode APK build passed in40seconds after the additional
DNS-SD alias regression. Preview55,723,909 bytes, SHA256:
d624597060b2fb5094c454eea702da84b38dadfb68188c577de72ccf2815833d.
Checksum-promoted once to the owned stable staging/PakarController.apk using
the donor storage gate and verified pending-copy/rename, replacing only the old
staging candidate. No extra archived APK, cache clean, SDK install or unrelated
firmware/native build. Existing production/rollback/settings remain untouched.
Base2dc319fb8add1516a0f2caa4306bee1bcb1284ab plus preserved working-tree edits;
debug-signed preview, not a clean-head commit or production-release claim.

Fresh ASUS SSH8022 refuses and local ADB lists no phone. No APK install, actual
Android/LAN multicast/credential test or Web firmware deployment is claimed.
Phone/human, live board discovery and complete application/donor acceptance stay
open in issue2. No AVR, board, broker, audio or PBX action this pass.

## Integrated S3/AVR and application handoff checkpoint

ASUS identity was freshly established through its already-trusted hostname key;
the stale forwarded-port key was not accepted or deleted. Both Rust candidates
are running from verified staging/deploy paths, not overwritten primary binaries.
Release SHA256 is14110a245082ba48bdb66477dd69dfc3b700967d22109d1d314c748892cd879b.
The local candidate acknowledges the actual board over its ASUS SSH HTTP forward,
with live WebSocket, contextual title, all80 mappings and actual RGB snapshots.
Settings, previous executables, recovery images and warm release cache survive.

Flutter's columns are now exactly1/21/41/61,11/31/51/71,2/22/42/62,...,
with no horizontal row offset. Neighboring columns keep their half-cell vertical
stagger. All134 Flutter tests/analyzer pass, including all80 positions and actual
phone/tablet LTR/RTL bounds. APK SHA256
fee48458e08ce49bd786fe0651266b8f08636eca318dbefb3748990d9f20264b
was installed and launched on the authorized ASUS-connected phone. Its captured
runtime was disconnected, not visual table acceptance. The separately inspected
aligned-table renderer capture is explicitly simulated. Disconnect also stops
hidden board-preferences polling. All214 Rust tests and source quality pass.

Integrated RMT S3 imagebc73583039b79113d32500dd46bc5577c6f5ec54041ccefdbba1d491c84bec36
was uploaded over LAN in20.95seconds, rebooted, and matched a complete downloaded
SHA256; selected/running OTA slot is VALID, auth is disabled and same-peer HTTP/WS
pass. No USB, filesystem format or fuse write was used. The initial private .NET
helper accidentally typed the MD5 form field as a file; installed SDK3.12.1
WebRequest.cpp::_parseMultipartPostByte confirmed that Content-Type marks a file.
After read-only rejection/unchanged-boot inspection, the corrected plain-field
wire format from tools/update-firmware.mjs/PU850 AsyncOTA.cpp succeeded. Existing
firmware backup was verified and preserved, not overwritten or retried blindly.

AVR full image4f3f00454ec913551c88da4746ea11955087835c9619e6331406c67f15034fb1
was programmed through the integrated ISP path in43.98seconds with byte-verify,
durable Prog/UART admission/release and EEPROM preservation. Original recovery
and fresh pre-erase flash/EEPROM/metadata were exported privately. Hardware UART
returned online, settings read back unchanged and Silent remains enabled.
Post-update watchdog reset reason is recorded, not evidence of a continuing loop.

User optical feedback still reports cross-cell spill and incomplete cell80.
The steady cell80 test snapshot contains exactly physical pixels1..6 white;
English PBX DTMF3 reported missing pixels without spill. Manual individual pixel1
and pixel2 frames sent through the actual Rust app matched exact RGB readback,
but operator DTMF3 reported each dark. This does not establish a wiring or driver
root cause. Non-DMA comparison and physical acceptance remain open, as do the
display, power/clock, network recovery and all outstanding donor/production gates.
PBX coordination is now keep-open, with a separate scoped Jitsi coordinator;
queued audio, bot presence and screenshots are not bidirectional audio proof.

PR18's complete dependency diff/conversations were reviewed and normally merged
as71d8f0f5153718fa4f6593ff95ca47b3a9574073; no open PR remained at recheck.
Its exact-head CI does not certify this integrated tree. Current native CLI
fixtures exposed stale contract mapping hashes/one line anchor: reread actual
UartProtocol.h, EepromLayout.h and AddressableLeds.h/.cpp and current adapters;
rebound only the four changed consumer references. Donor revisions/source hashes
remain unchanged; no unread donor row or hardware checkbox was promoted.

The follow-up GPIO-static OTA is now independently verified as
a92300d546b44e81ab5ddd3f6cf80fd1988bf44324385b0b694d1ea97b4750ef,
1899200bytes, bootc42e6ee31bcabb93af86df14c3ad01ac. The canonical updater's
post-upload verification lost a forwarded HTTP connection; it was not rerun.
Direct ASUS full-image readback established the result. A fresh32KiB AVR ISP
capture also matches4f3f00454ec913551c88da4746ea11955087835c9619e6331406c67f15034fb1;
UART returned and Prog is clear. User reports three steady white pixels and one
cycling color in cell80 under non-DMA too; hardware acceptance remains failed.

Checkpointdbb2f0631a7052d0b1701815431feaffa8434a4f exposed a cross-platform
CI fixture exit-status bug: script_tooling_test.ps1 intentionally rejects a
child exit7, but left that as the last native status in GitHub's pwsh-Command
wrapper. Reread the exact PU850 Tool/build-all.ps1 pinned Git blob and imported
runner; production failure propagation is retained. Add an actual successful
child case and assert caller-directory/status restoration instead of clearing
errors or weakening CI. The exact GitHub wrapper now exits0 locally, and the
failure case still asserts exit7. Only this fixture's28 proof references rebound.

The next CI run passed all three tooling OS jobs, then exposed missing
ArduinoJson.h in the clean Windows quality runner. Native LCD/cloud/management
fixtures use real ArduinoJson; local installed headers had masked provisioning.
Reread PU850's pinned .github/workflows/build.yml: retain explicit dependency
installation from reviewed pins, but install only ArduinoJson before host tests,
not any board SDK or the donor's unpinned fallback. The existing canonical JSON
pin remains authoritative. Workflow-order/no-SDK regression and local LCD fixtures
pass; CI remains asynchronous. No firmware/app rebuild or deployment for this fix.

ASUS native recovery: the actual session1 owner was connected with80 confirmed
mappings but only Events in its dock. Disconnected save/startup pruning could
persist that temporary view as the connected workspace. Preserve the last
connected layout, cache it before retirement, and migrate the legacy Events-only
layout once without overriding subsequent explicit panel choices. Continue to
gate every visible panel on the current board's acknowledged capabilities.

The first live recovery checks exposed additional real failures, not success:
optional settings transactions flooded the HTTP queue, and cell writes expired.
Read complete current PCController session.go and Pealayer workspace_profiles.rs;
the latter's stable user-owned profiles are retained, not replaced by a reset of
the entire user profile. Adapt actual Session grantNextLocked priority/FIFO for
unsent native HTTP work, serialize optional settings reads with bounded retry,
and reuse the fresh worker preflight for atomic cell batches. Keep mutation8s
expiry, generation revocation, server board/boot fencing and post-ACK readback.
Separate read24s admission never extends mutation permission or replays writes.
Only existing consumer/proof hashes were rebound; no unread donor is certified.

Native release a853fbfd291dd36117d94613650400aa9645ba281aabeab1c05118262a9e4d01
is now the ASUS deploy owner5196 in session1. Settings and the prior executable
were preserved before graceful IPC replacement, not primary-install overwrite.
Actual app IPC reconnect restored Table/effect/hardware/network panels and80
mappings. Under background network-settings loading, cell80 red then white took
1048ms and4974ms, with two Atomic cell batch ACK/same-board readback events,
zero active incidents and zero repeated occurrences. Network and cloud settings
both loaded. Internal framebuffer capture completed and was visually inspected:
efae0a85f705281fd7f9799b56e540773d51d54e0279060b3a49f01015f77220.
Private receipts remain on ASUS deploy/asus-priority-* and local outputs.
20 focused native tests passed across layout, optional reads, HTTP fencing and
atomic batches; the warm release build passed without rebuilding SDK libraries,
firmware or APK. This verifies the native/LAN command path, NOT physical LED
placement/color or production completion. Issue2 stays open. Existing main CI
39111550290 passes quality/native/Android but fails the separate ESP8266 link
on missing __atomic_*_4 symbols; that firmware issue is not fixed by native work.

USB control latency: reread the complete current canonical PCController
Session, session_test.go and request_admission_test.go (live hashes match the
existing b0111ae0 ledger). Adapt actual control-over-background admission and
fragmented fakePort/opcode-sequence assertions into the production Rust worker.
The new optional same-sequence0xB7 packet contains observed post-dispatch board
state, not request-derived success. Boot/session/intent/value checks remain;
missing companions use read-only verification without replay, including when
another command is queued. Background captures pause only between chunks and
resume, retain capture-start freshness, overlay newer actual command state and
discard pre-command RGB. Partial receipts do not renew peripheral observations.

Measured through the actual ASUS session1 app IPC, COM22, Wi-Fi/WebSocket
inactive: old630459cb had2440-4031ms confirmed completion (four commands), while
e897e313 plus the new ESP had29-63ms (six immediate and twelve commands spaced
1200ms apart); wire ACK+board readback10-12ms, zero active incidents, original
cell/search restored. Original full scene also matched after the ESP update;
the restoration helper refused to overwrite its non-white values, then direct
comparison established that no restoration write was needed. Internal app
framebuffer captured/inspected with exact PID13368. No optical LED proof inferred.

Native eight focused tests pass, including the actual worker under a slow,
fragmented preview and prevention of older-capture state rollback. Additional
admission assertions cover lost-companion/legacy readback with queued writes.
The first test compilation used PartialEq on a donor manifest lacking that
trait; compare serialized observations in the fixture, not change the public
model just for a test. ESP32-S3 16MB/OPI/static/buffered warm build passes.

OTA preflight failed before dispatch; later hostname resolution and curl reads
worked. Internet job12 failed TLS before staging image bytes; no mutation was
replayed. The first USB pass read chip MAC/partition table but failed its next
bootloader handoff before any flash write. Reconciled that no-write receipt,
then reused the fully read Install-S3SlotRecovery per-operation default-reset/
hard-reset recipe rather than retrying a held-stub sequence blindly. Actual
partition table plus CRC-valid VALID OTA records selected0x340000; full prior
1903312-byte running backup matches481ef136bb6cecb50a85a610abd31fc7ec8db06231fc75a0dadb4e35cb8a59f2.
New1903488-byte ESP image8bc0cd92dd00e8b78f5ce40187d549095859470f32d95ea372521b45a9f75558
was written/verified; the same MAC returned with a new boot and80 USB mappings.
AVR was not programmed. Private stage/flash/IPC receipts remain under the
existing outputs/PakarController/verification and ASUS deploy directories.
LAN/TLS/update reliability and the unrelated existing ESP8266 CI failure are
not fixed or accepted by this USB checkpoint. Production issue2 remains open.

Final nativef74e21addd521852ef9ec37cae40b2475d8f042264e0a11be98648f4dae58d3b
adds the queued-write/missing-companion fallback safeguard, passes the same eight
focused tests and is the ASUS session1 owner10908. Six additional actual app-IPC
USB commands spaced1200ms apart confirm in32-64ms, wire readback10-12ms, zero
active incidents, original cell/search restored, no HTTP/WebSocket fallback.
The full source/receipt-hash and aggregate timing record is USB-LATENCY.json.

Flutter screen-off retention: reread the complete live Pealayer
src/ui/workspace_profiles.rs (410d274f51e5e7db7ce81582d36166a97240b097e993cec5fee8707a95183e2f)
and PreferencesTab.tsx. Extend the existing stable-board workspace adaptation,
not a new parallel preference framework. Saving only when entering background
left a process-suspension/death window; scroll-end PageStorage could also restore
an older offset. User edits now notify a 350 ms quiet-period/2 s maximum-wait writer,
with immediate inactive flush, ordered/coalesced writes and identical-snapshot
skips. No telemetry-driven or per-scroll-pixel storage writes. Selection/Find,
indicator visibility, multi-select, transition, activity and both scroll axes
restore from bounded UI-only snapshots, never cached device authority or commands.
Reattached scroll controllers start at the latest actual offset. Colour actions
now precede the separate selection-management row, with stable explicit keys.

Resume retains healthy sockets; a 750 ms read probe replaces only a silent
pre-suspend socket, with read-only HTTP fallback. HTTP success does not cancel
that WS recovery; generation/foreground/explicit-Disconnect fencing remains.
Seventeen focused production-widget/ownership/writer cases and analyzer pass
through the canonical wrapper. A fixture initially tapped the navigation bar
before the scroll layout settled; target the visible actual control instead.
The socket fixture also needed one real event-loop turn for broadcast-cancel
completion, not an increased production reconnect delay or disabled assertion.
Temporary tracing was removed. One analyzer brace warning was corrected without
weakening the gate. Existing donor omissions stay pending in the same ledger.

ASUS is reachable but its authorized Samsung wireless-ADB advertisement times
out; devices -l is empty. No install, app-data clear, shared ADB restart, firmware,
board command or physical acceptance is claimed. Prior APK staging is not phone
delivery. The final candidate is built/staged separately after source settles;
actual Android screen-off/process-pressure and human acceptance remain open.

The one warm release-mode preview build completed in 44.4 s; no dependency
upgrade, new cache or clean. APK 56,215,521 bytes, SHA256
b5df3f10904af6ce2a23e91923afb6e7b7f6032350ea586ad50f8633e019a449;
local staging independently matches. The existing Flutter-rendered simulated
capture also passes (18 total focused cases), visually inspected with colour
actions above selection controls and no overflow. PNG SHA256
da22502fa23accdafd6809f74607f75dc87da1e7111579200fdd72ac1c8cac1f.
Preview signing, fixture rendering and APK staging are not Android installation
or physical/human acceptance. No production issue checklist is closed by them.

Native selected-owner bridge checkpoint: fully reread PCController discovery.go,
discovery_test.go, mndp.go, protocols.go, hostbridge peer_response and tests,
lifecycle.go, discovery_metadata and tests; Pealayer server/discovery.rs and
network.rs. Exact live bytes match the inventoried b0111ae0 / 5a6961ce snapshots.
The existing ledger binds these adaptations and concrete omissions;
bridge-repeater routes the remaining manager/public/enrichment/adapter work.
Generic PCController RPC, Android LAN server, opaque firmware bytes and binary
pixel streaming are not implemented or certified by JSON table-command sharing.

The native opt-in listener uses the existing selected USB worker and its matched
ACK + observed command-state/full snapshot. No second COM handle, implicit
network fallback, discovery actuation, auth prompt or owner acquisition is added.
Distinct bridge instance/hostname/path metadata preserves board_id/boot_id and
board hostname. Same-ID command receipts are consumed before dispatch; HTTP
recovery accepts the existing packet form. Native mDNS/DNS-SD, SSDP and MNDP
advertise/browse only when sharing is enabled. Loopback fixtures open no LAN
discovery sockets. Active protocols and faults reflect actual startup; conflicting
mDNS automatic suffixes are rejected instead of misreporting configured identity.
Flutter consumes bridge endpoints and explicit MNDP API ports without
pretending application instance IDs or virtual MACs are physical board IDs.

Initial offline Cargo metadata resolution conflicted with the already-locked
zbus/xcap dependency. Consulted build-storage and actual donor dependency slices;
online resolution added six packages without changing any existing locked version.
mdns-sd is pinned to the actual donor 0.21.5; socket2, httparse and network-interface
use cached/donor APIs. Warm release cache retained; no clean, debug profile,
donor build, SDK upgrade or alternate target directory. First compile caught
helper visibility/parser ownership errors: reuse existing shell LED encoders,
not duplicates. Two Dart analyzer brace findings were fixed without weakening
the gate. A chained fixture used the nonexistent effect name instant; inspecting
the actual encoder showed the contract is immediate, so only the fixture changed.
Failed assertions did not trigger any production command or board contact.

Thirteen focused native bridge/socket/parser tests and the actual fragmented USB
worker test pass, as do six Flutter bridge identity/discovery tests and analyzer.
These use explicit loopback/fake serial peers, not physical boards. LAN/router/
firewall, installed Android, optical LED and generic peer-RPC acceptance remain
open in issue #2. No firmware or board was contacted or changed; no PBX/audio
test was requested or performed for this checkpoint.

Sharing-off has its own revocation token on queued USB requests; it does not
release the primary USB owner. Both pre-dispatch admission and result delivery
observe that token. Explicit board Disconnect remains separately gate-fenced.

Existing main CI failed tests/pu_routes_test.mjs before this bridge checkpoint:
PU850 route bindings still named the ESP source hash at451dff7. Read the entire
451dff7..f162a8c ESP delta, actual commandReply/WebSocket/OTA handlers, complete
donor AsyncOTA.cpp and AsyncWebServer.h, and the actual audit-pu-routes.mjs.
Donor source bytes remain unchanged. Only ESP main target bindings were stale:
the additive USB command-state and atomic pixel diagnostic do not remove or
weaken the50 classified route handlers. Use the existing generator to refresh
current target hashes/line anchors, not disable its assertion. network-cloud
did not route pu850-esp.ino; donor-audit found it, and pu-routes now keeps that
slice and its still-pending complete-file comparison discoverable. This review
does not certify an entire read/import of the PU main sketch or live OTA success.

The specific failed route check now passes with the actual donor path configured,
including registration enumeration. Reports were regenerated after adding the
new reuse route and their stale-report check passes. This is not a claim that
all CI jobs have passed; new CI is allowed to run asynchronously.

Canonical release/preview builds succeed with version0.2.7 (Android+7). Native
27,829,760 bytes, SHA2563107992850817b2829ce9787f73a804de2903792a259510b465f926d5c39f819;
APK56,215,521 bytes, SHA256d45ae5612e484bc3ed430c42a03eea6b9445b21952a69a8368b9f9015ff6edf5.
Stable local staging matches; native CLI/package probe passes. Existing installed
apps, settings and rollback are untouched. Android app is built/staged, not
installed; native is a staged candidate, not an ASUS replacement or LAN test.

An isolated disconnected Preferences fixture was launched with its own config,
no production IPC/tray hooks and sharing disabled. Internal framebuffer produced
no image; explicit expected-PID Win32 fallback found no visible non-minimized
window (including a second normal-launch fixture). Consulted capture and reread
the actual Pealayer read-only capture source; no focus/restore/input workaround,
blank-image acceptance or claim of visual QA followed. Only identity-verified
fixtures created in this pass were stopped. UI screenshot acceptance is pending;
this does not mean the available Win32 capture capability was removed/unavailable.

## S3 integrated RGB, network and client delivery checkpoint

The complete PU main sketch and actual AsyncWebServer/FirmwareTransfer/AsyncOTA
sources were consulted, together with the indexed PCController session,
status-LED math/controller and Pealayer status-bar sources. The authoritative
comparison ledger contains exact source/target/test hashes and concrete remaining
omissions; 231 comparisons do not represent completion of the 4,673-file inventory.

Semantic RGB now has persisted, explicitly confirmed GPIO38/48 configuration,
finite driver submission, peer-bound emitted-byte mirrors and native/Web/Flutter
settings. Network fixes retain AP recovery after station configuration failure,
strict saved credentials, hostname/AP-name limits, route fallback and checked
mDNS service registration. Front-panel query replies are fresh and do not evict
mutation receipts. No test result establishes visible RGB or display output.

Actual firmware delivery exposed the old clients' 2MiB image limit. Both clients
now share bounded selected-board image admission, retaining the S3 chip/hash-header
and application-slot limits, legacy ESP8266 bounds and AVR image requirements.
Native manual/HTTP/inbox and Android SAF/HTTP/inbox paths use that policy; a document
selection is never an automatic flash. Native build13 is running on ASUS, hash
5b254a909c558006cc5898edb12845f268d76da2e15f1e1711d656bdc4cd6229.
APK13 is staged there, hash
b89bbf60fc69bcb12e47902aefcd28bf833fa5452f7b9f614f7d583fe8b28131;
no authorized ADB device was attached, so it is not an installed-phone result.
Final native suite passed299 tests; mobile analyzer and287 tests passed, with2
opt-in captures skipped. A stale invalid ESP header in a mobile fixture was
corrected without weakening its ownership assertions or changing the APK.

An actual mapping failure was an unnecessary HTTP status dependency. The native
pinned mapping response now validates its own producer identity and live owner;
only an absent envelope takes the original legacy status/mapping/status fences.
Six loopback tests and an ASUS runtime with80 acknowledged mappings passed.
Later board unavailability is recorded separately, not relabeled mapping success.

Two Wi-Fi updater attempts stopped before upload at the bounded WebSocket
preflight. The tool now requests exactly one read-only status after a validated
upgrade; only correctly pinned compact events can precede the complete status.
Actual loopback fragmentation/partial/deadline fixtures passed; no uncertain
firmware write is retried. A reviewed USB bootstrap followed: read metadata,
preserve the running image, verify its hash, then write only the selected app slot.
The460800baud read failed short-packet;115200baud read matched the known-good image.
PowerShell5 hashtable sorting chose an unexpected OTA record; the offset guard
stopped before any write, and explicit highest-sequence selection replaced it.
No partition-table, bootloader, filesystem, AVR image, EEPROM or fuse write followed.

The new2220352-byte image80dcfb2155b54a2ed6fe70244d2c2779948fde0cc89196d730657b4d0849d86d
booted with verified integrity and80 white cells, AVR0.2.7 online and individual
display/text acknowledgements. It then exposed unsafe internal heap pressure:
8272 free bytes /5108 largest block despite8MiB PSRAM. Consulted the actual installed
esp32-hal-psram.c, SHA2f47f8b635de8ffb64d160576a2705b063235fb9f54e01b98e8d2bf3f48352bd,
and reviewed precompiled NimBLE SDK allocation configuration. Do not pretend a
compiler define can change that precompiled allocator or move cache-sensitive LED
buffers. BLE application ingress/framing and S3 WebSocket queue now use explicit
one-lifetime PSRAM allocations with no internal fallback; SDK handles/locks/stacks
and LED wire/DMA buffers stay internal. Allocation failure closes admission,
and telemetry exposes queue readiness plus free/largest/minimum INTERNAL8BIT heap.
Actual whole8192-byte minimum-MTU framing, identical fragment retry, allocation
failure/lifetime and cross-thread queue fixtures passed. The ESP8266 inline queue
is unchanged. SDK build, measured live headroom and OTA acceptance of this memory
fix must be recorded separately before claiming a Wi-Fi-only handoff.

## Web-service incident and recovery policy

Fresh ASUS observation found the same79db5e boot and PSRAM image still running,
BLE enabled:false/started:false, about62KB internal free/largest31KB, but one
HTTP connect timeout and variable receive latency. Later root Web/auth/status
responses completed; do not label this incident Bluetooth-caused or a reboot fix.
The existing guarded interactive native observer verified actual COM22 USB
recovery and return to Wi-Fi/WebSocket with80 mappings/white cells and zero active
incidents. Its inherited PT1M task deadline terminated the60-second observer;
preserve the partial progress and independent final guarded receipt, not a
fictional completed soak. No scene, AVR, buzzer or settings rewrite was performed.

Consulted the actual PU850-ESP WiFi_Setup at unchanged75d5492a hash, the installed
Arduino-ESP32 WiFiGeneric.cpp setSleep/getSleep and STA.cpp startup hook, plus
Espressif's power-save latency documentation. S3 now explicitly requests awake
Wi-Fi before/after startup, with opt-in PAKAR_S3_WIFI_MODEM_SLEEP and actual
esp_wifi_get_ps status readback. This is an SDK adaptation of donor wake ownership,
not a claim that power saving alone caused every observed network failure.
ESP8266 behavior and persisted network layout are unchanged. Default/opt-in and
failed-application/AP-preservation fixtures passed.

BLE's real lifecycle owner now defaults missing/corrupt records Off, distinguishes
saved request from running radio, never enters SDK initialization from a runtime
setting, and reports reboot/build requirements and retained SDK resources.
The original reply deadline still retires owners, including a late GAP connection.
The build wrapper exposes -S3Bluetooth runtime|disabled; disabled preserves USB
flags and rejects new enable requests. Existing pure lifecycle/storage fixtures,
source guards and canonical build-plan tests passed; SDK deployment/HTTP and
post-update native acceptance are separate receipts, not implied by those tests.

Source audit also found synchronous PubSubClient TCP/CONNACK and outbound
WebSocketsClient connection paths still run on the main loop. An explicit
hostLink progress callback around writes does not run inside these SDK waits.
The correct single-owner cloud-worker/immutable-queue redesign remains open;
do not claim continuously serviced application USB or BLE/TLS/OTA coexistence
from this bounded recovery slice. The hardware USB bootloader recovery path is
independent, but no USB flash was required or performed for this incident yet.

The initial c75edea runtime-capable build2222176bytes passed the real S3 SDK,
network/default/opt-in/failure cases, BLE policy/storage and build-plan fixtures.
One canonical Wi-Fi upload then failed at5898ms. Both independent HTTP and
actual native USB readback found operation3 aborted at206164bytes, unchanged
79db5ecd43872d3e64e5dbd5dea65997 boot, internal minimum31132bytes and BLE not
started: not the prior BLE-on exhaustion and not a committed update. The exact
old2221280-byte SHAfa1a7a9aca7427517485d1729393666c3599b2ad88078e0b19f8fc89bdc8831a
remains preserved. No upload retry or automatic USB fallback occurred.

A twelve-request uncached-hostname probe spent roughly2seconds per DNS lookup;
its finite overall deadline included that delay, so client cancellation must not
be mislabeled a board-generated reset. A separate already-verified cached-address
probe preserved Host/peer identity and returned12/12 complete responses, median
307.6ms. Subsequently Ethernet-bound HTTP200 stalled at4194/10160bytes before a
six-second read deadline. The proposed adapter-specific route change stopped
at its failed preflight: no route was changed. These are measured separate DNS,
TCP and partial-body symptoms, not a successful full service soak.

Reread actual PU LocalLib/AsyncWebServer.cpp and.h through EOF at unchanged
inventory hashes and installed ESPAsyncWebServer3.12.1 WebResponses.cpp and
WebRequest.cpp. AsyncAbstractResponse can advance to END while a final buffer is
not fully admitted, and short-add accounting omits accepted bytes/credits.
The existing double did not exercise actual add/send/ACK-based close behavior.
A project-owned known-length snapshot adapter and real short-write fixtures are
being implemented; the SDK/donor checkouts remain unchanged. Upload ingress and
WebSocket upgrade are distinct and are not claimed fixed by that response work.

The final 802512c SDK recovery build with application-only BLE exclusion passed
the complete protocol suite, focused network/ble/platform cases and canonical
build-plan/storage guards. Direct guarded USB app-slot recovery then verified
the write; complete live running-image SHA, VALID slot, auth-disabled HTTP and
same-peer WebSocket passed. Native actual USB then Wi-Fi passed with80 mappings,
white state, unchanged preferences and zero active incidents. See commissioning
for exact image/boot/receipts and remaining cloud/OTA/UI/physical boundaries.

CI subsequently caught a separate stale PU850 route-target binding after the
six-line status extension. Reviewed the affected sketch diff (no route change)
and regenerated all50 existing route hashes/anchors from the actual unchanged
donor, then ran pu_routes_test. The firmware build wrapper now runs that same
guard before compilation; it does not auto-certify or regenerate evidence.
This tooling/evidence correction requires no firmware rebuild or device write.

## Shared pixel diagnostics continuation

Reread the complete PCController Project/AddressableLeds.h/.cpp and Web
status-frame-batcher.ts sources. The existing clear/stage/show mapping remains:
one deliberate diagnostic command owns a complete480-pixel frame. AVR-specific
assembly,100-pixel buffers and register assignments still do not apply to the
S3 renderer. The status reducer's identity/order discipline is adapted to a
board/boot-scoped diagnostic revision, not a second animation engine or a public
API generation. Generic metric/history imports remain pending.

LedGrid now publishes index/count/RGB/off/active/revision with normal status.
Rust and Flutter consume that observation without command replay. Duplicate and
older diagnostic revisions cannot overwrite a newer observation or user draft;
normal scene, mapping, stream and maintenance takeovers invalidate the diagnostic.
Rust's previously white-only controls now include colours and aligned six-pixel
groups. Only draft cursor/group/colour persist; pending writes and observed boot
authority do not. Flutter's page subscribes to existing status and connection
streams instead of a periodic repaint timer. Focused renderer, transport and UI
tests are the intended source evidence; fixture screenshots, packages and actual
deployment remain separate evidence and cannot certify physical pixels.

The current CI Windows WS fixture expected a mutation immediately after status,
but the shipping worker also introduces its peer identity. The fixture now
accepts that non-actuating introduction before asserting the exact command;
release admission and production packet handling are unchanged.

The bounded delivery subsequently completed canonical native/APK builds and
focused tests, guarded USB S3 app-slot recovery, then native replacements on
both PCs. Exact hashes/live owner observations and remaining Wi-Fi/AVR/phone
gates are in ESP32-S3-COMMISSIONING.md. No SDK/donor checkout was patched.

Actually inspected both rendered diagnostic captures: native framebuffer at
1920x991 and Flutter widget phone geometry390x1180. Both expose single/six-pixel
steps,475–480 tail bounds, arbitrary RGB, full480 clear and a board-reported
observation. Native additionally labels logical cell1 from the fixture's remap;
the mobile fixture has no supplied map. Platform geometry/sector styling is not
pixel-identical or full-app visual parity. Comparison caught an oversized native
progress bar and observation loss on a WS drop while HTTP remained fresh; both
were corrected and the native capture repeated. Real WS pushes in the read-only
fixture produce zero incident events. No screenshot executes board commands.

Native capture initially encountered an unavailable WGC helper and an honestly
refused hidden-window Win32 capture. The approved read-only fallback was
consulted, not declared absent. Installed eframe0.36.2 glow/window-runner sources
show the hidden-until-first-paint path; a process-owned one-time logic reveal
and isolated fixture persistence now allow real framebuffer capture without
production IPC/tray, focus or overwrite. A speculative repaint-waker change was
removed after rereading callback order; it is not presented as a proven cause.
Original failed captures remain outside the repo; only the inspected successful
SIMULATED native/mobile captures are committed. Donor inventory totals remain
314/4697 compared,4383 pending and253 known gap files, not complete parity.

The next real S3 recovery and detailed PBX white acceptance are recorded in
ESP32-S3-COMMISSIONING.md#neutral-white-acceptance-and-480-pixel-timing-checkpoint.
Actual pinned IDF5.5.5 RMT sources and all eight WS2811 V1.4 datasheet pages
were consulted; no SDK/donor patch or speculative overclock was applied.
The native pending-frame CI failure exposed a production bug, not merely a
fixture typo: pending RGB must not invalidate independent identity/mappings.
Corrected structural/presented validation passes USB13/diagnostic6/pixel3 tests.
The first native release attempt with ExpectedCommit stopped before compilation
because reviewed work was still dirty. Preserve that source-stability guard:
checkpoint the reviewed sources before packaging, never bypass the gate.
The attempted live animation benchmark received Busy/error5 during USB command
dispatch. No mutation replay or animation-FPS claim followed; actual cloud I/O
and PCController bounded-parser sources identify nested progress dispatch as
the next ingress-scheduling correction. Physical steady-white acceptance does
not close that USB, Wi-Fi service, AVR, animation, phone or full donor gate.

## Flutter Output panel extraction

Read actual Pealayer `src/ui/workspace_profiles.rs` completely at source
`5a6961ce6a477a4726b7baf8a84d12fde1cdf6ad`, SHA256
`410d274f51e5e7db7ce81582d36166a97240b097e993cec5fee8707a95183e2f`.
Also read complete generic function sections in `src/ui/layout.rs`:
`hardware_channel_drag_handle`, `finish_hardware_channel_card`,
`hardware_channel_drop_target`, `effect_library_card_header`, `effect_card`,
`hardware_card_header`, stable tab/menu registry and canonical tab restoration.
That large file remains whole-file pending, SHA256
`d60b1913e9f5beb1417ef0951ccaadcdf89cd555a32279c3e6c70f14a94a3e17`.

Adapted stable IDs, reserved header action space and handle-owned ordering into
Flutter OutputPanel: Scene, LED output and Pixel diagnostics cards. Arrangement
is explicit and persisted in the existing board-owned UI snapshot; it sends no
device commands. The Table now owns selection, transition, mapping and canvas.
Output service, mode, rainbow and game reset retain their actual existing APIs;
diagnostics opens the one existing guarded page from either Output or its menu.
Retained panels are not control authorization. Production panel keys and an
additional identity check prevent old board capabilities leaking into a new peer.
Generic profile CRUD, arbitrary docking and full donor card/action parity remain
pending. Firmware, settings and advanced bridge retain their existing dedicated
pages. This is executable source work, not an installed-phone or physical claim.

## Current application health checkpoint

Canonical PCController `Tools/Controller/internal/link/session.go` was fully
reread at SHA256 `1311fdcde94b2ef8ed06db63ff874a255c135dc1969709e6f43ff4b5c5c8235b`.
Its actual RemoteError/decodeRemoteError behavior is adapted in the Rust USB
introduction path: preserve correlated opcode/error code, distinguish Busy,
invalid report/capacity and safety rejection, and reserve release mismatch for
Unsupported. No policy override or retry is introduced. The focused canonical
native test passed. The earlier admission rejection was not reproduced: actual
ASUS app PID6244 reconnected over COM22 and stayed online through twelve bounded
samples with no active incidents. Internal framebuffer capture completed and
showed the real current board, full mapping and green logical cell15.

The ESP recovery image SHA256
`abde3c024d6ad5956689aa61f8d1dad06685d936d20b54616d37433d5683334a`
was actually written/verified to app1 only. USB ACK/readback and ten fresh
WebSocket connections passed on boot `0c5cbe8456b1082d8d29f9abeab2561c`.
The user's newer request supersedes rainbow: the current published frame has
only physical pixels258-263 green (logical15 mapped to physical cell44), with
all other pixels black. This is readback/capture evidence, not optical proof.

Flutter5497e19 fixes retained all-off and cross-boot diagnostic observations;
analysis and eleven focused tests passed. APK SHA256
`3ed702e5008332b08b4b3807fa44d4dade66d530112204e1170894a060e08329`
was built and transferred to ASUS. Actual Android installation was rejected
for insufficient storage (290MiB free). The authorized USB phone still reports
0.2.7, below the board's0.2.8 admission floor. Existing app data was preserved;
the user is freeing space before installation. Package/source success is not
installed-phone acceptance, and no downgrade or blind install retry was used.

The Rust release from ff64236 was built, package-probed and staged through the
canonical cache. An initial source guard rejected the literal ExpectedCommit
value HEAD; resolving it to the actual commit passed without changing the guard.
Release SHA256 `f1ec8f7bfe832a6e0d918bdbe96a26876cb2ebf932a7a64a56327ecd6c0797ff`
was then gracefully installed on ASUS. New PID11208 is ready, USB-only online,
mapping80 confirmed and active diagnostics empty; ESP boot stayed unchanged.
Settings and old executable were preserved by the guarded replacement tool.
That tool now uses the actual Pealayer retained Process probe for status/quit,
because the older direct GUI executable invocation produced false IPC failures.
PowerShell parsing, package probing and the actual graceful replacement passed;
no forced close was needed. The new tool is not evidence of unrelated full
Pealayer scripting or UI parity.

## Flutter recovery and six-pixel indicators

The actual PCController `Tools/Controller/web/src/peripheral-availability.ts`
was reread completely (SHA256 `fcf2e397564c843711fd64db2bd2d820552180ac4f5abdf0d44ccbbb86be7a84`).
Retain its ready/advertised/valid-value distinction: a backgrounded socket alone
does not revoke an HTTP observation whose original six-second lease remains
valid. When that lease expired during suspension, resume starts its HTTP read
immediately while the independent socket recovery proceeds. No command replay,
renewed cached lease or synthetic acknowledgment is introduced.

Flutter's indicator previously painted semantic white/red regardless of active
diagnostics, demonstrably wrong for the live board's green six-pixel probe. The
new `cell_pixel_preview.dart` adapts the existing Rust `Frame::cell` physical-map
and six-sector semantics into six fixed indicator slots. Published RGB uses the
sample's own mapping; diagnostic state uses the acknowledged map and is labelled
as board diagnostic state. An animated mode without RGB stays unknown instead
of manufacturing white or an app-side rainbow. These are presentation values,
not optical success. Canonical analysis, four focused preview/widget tests and
all fourteen existing workspace lifecycle tests passed. Installed phone
acceptance remains separate. The earlier six-second handshake hypothesis was not reproduced:
a real stalled WebSocket upgrade reports one handled timeout under the app's
cancel-on-error lifecycle; no speculative API timeout change was made.
Follow-up presentation guards reject unconfirmed diagnostic mapping and disabled
output as sources for semantic-white RGB. Remaining generic state-unavailable
copy now names the missing board update, display frame or firmware job progress.

## Flutter command admission and HTTP receipt deadlines

Reread canonical PCController `mobile/lib/unified/transport.dart` through EOF,
SHA256 `5263044d7abac3865b6dcc8887fa6c11ad9a1c89b8c4e2374ffcd3bcc72014c5`.
Its generation-bound full-state acceptance and single-flight refresh are retained.
Pakar previously joined a stalled background HTTP status read even after fresh
same-boot WebSocket status arrived; the command could wait for a six-second read
failure without dispatch. Command admission now accepts that fresh live status,
retains the original two-second clock/identity/foreground/authorization gates,
and aborts only the superseded status read. A preflight timeout explicitly says
the command was not sent. No cached lease is renewed and no mutation is replayed.

The HTTP receipt timeout previously started after awaiting response headers.
The existing four-second command and twelve-second ordinary-write limits now
cover connection, headers and bounded body together, using the installed http
AbortableRequest transport to close stalled sockets. A dispatched write with no
receipt remains uncertain, including IOClient connection errors. The donor's
Future.timeout-only transport is not treated as a complete cancellation solution.

Canonical analysis and both new real TCP/HTTP/WebSocket tests passed: live status
overtakes a deliberately stalled GET, and stalled POST headers end within the
four-second receipt budget with one command and observed socket closure. These
local regressions identify actual code defects; they do not establish which path
caused the earlier phone screenshot or prove installed Android/physical output.

## Android wake coordinates and PBX routing observation

The tablet reports natural `wm size` 1848x2960 but its active rotated input
viewport is 2960x1848. The Android helper now reads the active logical viewport
after waking, immediately before its one authorized unsecured-lock swipe. It
deduplicates identical records and refuses ambiguous/missing geometry or another
display instead of guessing. Secure locks still require manual unlock. Pure
parse fixtures are source evidence, not proof that a live lock was dismissed.
The retained process probe was reread from Pealayer's actual
`scripts/package-probe-windows.ps1`; its bounded retained-process behavior stays.

PBX query `17924153254544402` expired without reaching David's mobile: the
query CLI does not insert the external trunk selector for a national `09...`
target, unlike its plain announcement path. The archived call has `Expired`,
not an answered/DTMF result. Future separately authorized query calls must use
the configured external-route target (`9` plus the national mobile number).
No answer, board approval or user refusal is inferred, and no repeat call was
made after the user requested automatic wake/swipe instead.

## Tablet wireless handoff routing recovery

Actual SM_X906B R52T60C6V9B/code19: classic TCP5555 and modern TLS both stalled
over ASUS Ethernet, while the same TLS endpoint/key immediately worked through
a temporary USB TCP forward. ICMP including1472-byte no-fragment probes passed,
so successful ping was not accepted as ADB proof. ASUS Ethernet43/.123 and
Wi-Fi9/.240 share192.168.1.0/24. An explicit ActiveStore-only host route for
192.168.1.115/32 through Wi-Fi9, on-link/metric1, immediately made actual wireless
TLS ADB succeed at192.168.1.115:37939 with matching hardware serial. This proves
the path-specific recovery, not the precise router/driver filtering cause.
Default Internet routes, shared ADB server and production boards were unchanged.

Modern wireless debugging was enabled through Android's own trusted-network
dialog; existing host keys were retained. The failed classic listener was
disabled, owned test connections/SSH forwards were stopped, the temporary USB
forward15557 removed, and the owned temporary pairing XML deleted. All following
shell/capture/screen-off/resume checks explicitly used the wireless target, not
the attached USB serial. Process5431 survived screen-off/resume and the captured
app recovered fresh board state with cell22, indicators on and multiselect off.
Android graphics-buffer format probe errors were observed in logcat; no claim
of zero errors/all bugs solved, physical command acceptance or long soak is made.
The scoped route is temporary until ASUS reboot; Wi-Fi/IP changes require fresh
discovery and route verification. Actual donor process helper and existing
android-device.ps1/viewport safety were consulted; no new toolchain/cache/build.
PBX handoff query17924181358883596 requires separate completed spool/CDR and DTMF
verification; a queued call is not user acceptance or permission for any board
firmware/output change.
The call subsequently completed: matching outgoing_done Status:Completed,
CDR ANSWERED/43 connected seconds and zero remaining active channels. DTMF3 was
received as not_tested, not physical/client acceptance. No redial or reschedule
was inferred. Both USB and verified TLS wireless ADB remained listed after the
call; actual USB removal is not yet observed. No production board mutation.

## Tablet diagnostics and startup intent recovery

Actual PCController `mobile/lib/unified/transport.dart` was reread through EOF
at revision b0111ae0 and SHA5263044d7abac3865b6dcc8887fa6c11ad9a1c89b8c4e2374ffcd3bcc72014c5.
Reuse its single-flight refresh, generation-bound result installation and explicit
enabled/disconnect discipline. Pixel diagnostics now has a read-only Refresh state
action, reports failed reads, accepts its first delayed verified owner and never
adopts an existing owner's replacement or replays a pixel command automatically.

Focused workspace tests exposed a real startup regression: automatic discovery
overrode saved explicit Disconnect for both network and USB. Restore that intent
unconditionally, retaining default-on discovery for absent/true intent. Three
transport-specific startup regressions accompany the existing persistence tests;
first-use auto-discovery remains covered instead of labelling saved Disconnect
as incidental. The initial diagnostic analyzer run caught two missing braces;
they were fixed before the passing analyzer/15 diagnostic cases. Verification
results and package identity are reported separately in issue2/10.

No authorized ADB device was visible on this PC, and ASUS localhost8022 refused
three bounded SSH attempts. Office management is reachable but has no enrolled
Android client; its deployed flat operator path was verified from systemd before
the read-only inventory. No service, router, shared ADB-server or board settings
were changed. Tablet installation/runtime, radio/long-soak and physical commands
remain pending access; do not replace those gates with source/widget/APK proof.

Final bounded verification: analyzer clean; workspace14, automatic-startup7,
command-admission real-loopback3, diagnostic transport6 (live-board probe skipped)
and diagnostic UI15 passed. Canonical maintenance APK0.2.8+20 built in52.8s with
BoardProductionHold. apksigner verified debug-preview signer
cb2620ad0c885fffd5e1ee7df0bbef8ff3262729e4e880888e9001fbbfb83e26;
aapt verified package ir.digitalogic.pakar.pakarcontroller/code20/minSDK24.
APK60689161bytes/SHA2565148fcc293e0e30baf63c24083aefe917ae20c6c9385f72fd71aa4b3d0d20f3e
is published in the existing office static Android candidate store under that
content hash. Server-side full hash/size and HTTPS200/Content-Length/range support
verified. No automatic-update feed changed and no installation requested through
the management service. This is a downloadable candidate, NOT installed-tablet,
production-signing, touch/radio, physical LED or zero-bug acceptance.

## App update and support delivery without primary ADB dependence

Reread actual PCController unified/app_upgrade.dart and peer_upgrades.dart through
EOF at their unchanged SHAa13fdcf3.../SHA97ca9e2c.... Reuse manifest-pinned bounded
transfer, foreground retirement, separate installer consent and capability-owned
support. New palette-data notices share the configured WS/Socket.IO package feed
but only trigger the configured trusted manifest read, never a payload-selected
URL or executable code. Coalesced checks now join their actual read rather than
returning while an automatic check is pending. Existing explicit sources remain
preserved; new installs default to the working office data source and WSS feed.

Server publication now supplies the missing app-update-offer type expected by
FirmwareFeedWire. Opted-in presentation notices are additive; APK-only consumers
receive no new data offers. Root-authenticated SQLite publication enforces digest,
size, increasing revisions and immutable palette bytes; no arbitrary file paths
or execution. Support adds token/session/deadline-fenced read-only board status
and mapping, redacts unrelated fields and separately permits assets.check. One
UpdateCenter contains data/support/manual/automatic settings. Same/older package
notices no longer report a spurious failure; explicit wrong-session input still
fails closed. Compiled Flutter/native code changes still need an APK/OS consent.

Analyzer and focused checks: presentation transport1, coordinator15, support6,
presentation activation/rollback3 and update-center1 passed; service10 passed.
Initial analyzer errors (helper declared below its first reference and missing
braces) were fixed before these passing runs. No broad test/CI wait substituted
for device acceptance. The unused first21 candidate was rebuilt after the older
notice fix, reusing the canonical release cache, not another debug cache.

The actual office management service was upgraded to build21 through the guarded
deployer, preserving its existing data and root-only administrator credential.
Old flat application source remains; the previous systemd unit and environment
were copied to systemd.before-tablet-updates/service.env.before-tablet-updates.
Apache syntax and service health passed; PBX/MQTT were not restarted. Published
108-byte Studio palette revision1/SHAaf51f425...; actual HTTPS full bytes/hash and
WSS session/expiry/source notice verified by verify-presentation-service.mjs.

Final maintenance APK0.2.8+21:60754697bytes/SHA256
4aff174e7f5ebfa5d59366b81fb4357ca20787628c9c66c9027f87e8822f22f3;
apksigner verifies the existing cb2620ad... Android-debug preview signer and aapt
verifies code21/minSDK24/three ABIs. BoardProductionHold remains compiled in.
ASUS8022 still refuses SSH, this PC has no authorized ADB target and the service
has no enrolled Android client. Installed tablet, touch/radio, OS consent and
human/physical command acceptance remain pending, not certified by the endpoint
or package checks. No production board was contacted or changed.

Publication completed from source ca50045f0d618dca146b127deb148b6d33719fbd:
the canonical publisher registered Android build21 at the immutable /pakar/packages/
4aff174e7f5ebfa5d59366b81fb4357ca20787628c9c66c9027f87e8822f22f3/PakarController.apk.
verify-client-service.mjs downloaded and hashed the full60754697bytes through
actual public HTTPS and received the exact package/session-bound WSS offer.
Presentation HTTPS/WSS verification passed independently. Public Windows artifact
build13 was merely read/verified, not updated or claimed current. The temporary
root upload copy was byte-compared to the immutable registered package and removed;
the registered package and local canonical build remain recoverable. Final ASUS
and local ADB probes are unchanged/unavailable; no actual tablet install claimed.

## Current architecture documentation reconciliation

Documentation-only inspection used source commit
8ce8f29eee01a979bbd42e7d3573a6e3ace9eba2 plus preserved concurrent local edits.
Read the reuse procedure, master acceptance issue, current role/pin/renderer/
transport/network/client/update source and the complete PCController donor
Tools/Controller/docs/Go-Embedding-API.md. The donor's single-owner and lifecycle
contracts informed the architecture explanation; this is not a new assertion of
complete donor parity.

Updated ARCHITECTURE.md and the root, S3, mobile, toolchain, identity, context,
client-management and acceptance introductions. Corrected ESP8266-first wording,
runtime role switching, table geometry, native-USB scope and current source
release/build information. Historical deployment receipts remain historical.
Recorded the failed exact-head CI include path, unresolved LED/AVR/client gates
and partial donor coverage rather than treating source support as deployment.

Only the changed README and DEVICE-CONTEXT document target hashes were rebound
in the existing comparison evidence; unrelated evidence edits were preserved.
Comparison regeneration stopped at a concurrent change to tools/test-game-role.ps1
for ControllerBoardMini/Puzzles|Puzzles.ino. That source/proof pair was not
reviewed or rebound by this documentation pass. Generated-ledger validation is
therefore blocked, not passed. No compilation, deployment, board connection,
firmware update, client replacement or GitHub mutation was performed by this
documentation reconciliation.

## PCController gap comparison and future AVR convergence

Read owning issue #6, firmware-hardware and donor-audit routes. The reference
is the clean installed donor b0111ae0c80dc34149ea1044de7dd5488920301b,
not the older work/repos/PCController checkout. Fully read entrypoint/config,
feature bundles, pins, EEPROM layout, settings header, codec/generated contract,
front-panel model, UART, event producers and shared-production/host-lifecycle
documentation. ProtocolRuntime was read through EOF; other named implementation
differences were inspected without certifying their entire dependency trees.

Added nine complete-file, hash-bound parity-gap records to the existing ledger
and avr-convergence routing to issue #6. Their target is the explicit design/
comparison report, not a claimed new code import. Existing macro-ring/header
text equality was checked read-only with LF normalization. Recorded exact baud,
reserved-byte/opcode, ACK/tone timing, pins and EEPROM incompatibilities; corrected
stale five-byte-only mirroring and old-profile wording in contract docs.

Focused consultation with the real donor root and -Gate passed: 12 matched
records, zero pending/stale comparisons, all 12 retaining known parity gaps.
The main work concurrently relocated shared ESP sources to firmware/esp and
updated its source/evidence references. Those edits were preserved. After that
reconciliation, generate-file-comparison.ps1 -Check passed with 328 total records,
including 105 PCController comparisons; this supersedes the earlier blocked
documentation-check observation, not any unperformed code/hardware test.
No new build, app/board contact, firmware change, donor edit, Git commit or GitHub
mutation was performed by this comparison. Physical migration remains pending.

## First-wins raw RF continuation

Read PCController `Project/Runtime/RadioRuntime.inc.h` fully at b0111ae0,
including actual receive, learning, mapped actions and transmit flow. Adapted
raw receive/transmit only in `firmware/avr/RadioAdapter.h`; inspected rc-switch
2.6.4 ISR and synchronous transmission implementation. This dependency's TX
blocks input scanning, so the opt-in adapter rejects TX during an armed game.
Persistent learning/mapping and nonblocking TX remain explicit gaps in the
existing file-comparison ledger, not accepted exclusions. The enabled AVR
candidate compiled with only 411 bytes left for stack/local state; this is a
warning and a physical stack/latency acceptance gate, not production approval.

The build-profile regression fixture initially failed its obsolete direct USB
compiler-define assertion. Consulted `usb-otg` and the current canonical build
owner: USB CDC/recovery is selected by FQBN, while the command path defaults to
UART0. Updated the assertion to inspect both actual selections; no USB firmware
policy was changed to satisfy a test. Focused RF mocks pass; they do not prove
RF reception, transmission, wiring or end-to-end client delivery.
# GPIO47 keycontroller isolation

User requested another LED pin after GPIO4 remained white with both RMT/DMA and
the static transmitter. The common DevKitC-1 GPIO47 (J3-17) mapping was reviewed
against Espressif's pin table and Pakar's UART/ISP/W5500/LCD/reset/RGB ownership.
The existing transmitter is retained; `-S3LedDataPin 47` selects only its output,
with GPIO4 still the default. The application-only header preserves SDK cache
flags. Driver telemetry now reports `data_gpio`.

Reread PU850 `sketch.yaml` and the canonical build wrapper. Exact dependency pins,
FQBNs, cache ownership and flash gates remain unchanged. The reviewed target
hashes were refreshed without marking additional donor features complete.
The existing actual-worker fixture now covers GPIO47's upper GPIO register bank;
its static/color/off/initialization/queue/flash-hold cases passed. An omitted
BoardCore include path in the fixture runner was corrected. This is host evidence,
not electrical or optical proof; deployment and user visual feedback remain required.

## Fresh first-wins profile default, 2026-10-21

- Failed lookup: `game-role` and `device-settings` are not indexed concerns. Stopped the guessed route instead of bypassing donor review; selected the existing `games-streaming` and `firmware-hardware` routes for the first-wins/key-mask profile.
- Scope: make a newly provisioned `keycontroller4` image start with boxes 1-4 enabled before a runtime settings file exists. Runtime `enabled_mask` remains authoritative and persisted; this does not create another game mode or protocol generation.
- Physical boundary: David-ASUS currently reports no present COM/USB-serial/JTAG device after a PnP rescan, so no flash or UART acceptance is claimed.
- CI follow-up: the latest release workflow failed in `tools/test-led-pipeline.ps1` because the committed scheduler compile lacked the shared `libraries/BoardCore/src` include path required by `LedWire.h`/`GameRole.h`. The current source migration already adds that include and points the runner at `firmware/esp/src`; the exact focused wrapper now passes all three queue profiles plus renderer, persistence, stress and chase wiring checks. This is host/CI evidence, not device timing or optical proof.
- Preflight lookup: guessed concern `led-pipeline` was rejected as unknown. Registered `firmware-hardware` and `games-streaming` routes were consulted without `-Gate` after their routing-only status correctly rejected acceptance gating. Their pending rows and parity gaps remain open; this repair does not claim donor parity.
- Full repository-health follow-up reached the peer-fence guard and exposed a stale formatting-sensitive assertion: production code still unwraps only a current route/scope and updates remote progress from both peer-bound network status and the selected USB snapshot, but rustfmt moved the assignment across lines. After consulting the registered `updates-telemetry` route, the guard now compares a whitespace-normalized source view while retaining its negative ownership and unrelated-USB mutations. This changes no transport behavior and makes the safety assertion follow semantics rather than one formatting layout.
- The next full-run failure was another formatting-sensitive source guard in `native_dropdown_test.mjs`. The Pealayer-derived Preferences draft still synchronizes method, enabled state, endpoint, user and serial port into all owner baselines before drawing and saving; its call is now multiline. After consulting the registered `preferences` route, the assertion now normalizes whitespace while continuing to require that exact single-owner call. No Preferences or connection behavior was weakened.
- The subsequent `web_cell_table_test.mjs` failure was the same class: the native shell still synchronizes the one shared selection/search concept after dialogs and requests repaint only when it changes, but rustfmt expanded the block. The existing `web-presentation` consultation had already passed immediately before this test. The assertion now normalizes whitespace and still requires the exact synchronization/repaint branch; it does not accept a missing or unconditional replacement.
- The remaining ordered CI slice then reached `connection_controls_test.mjs`. Production still validates the connection-change guard before releasing the selected USB owner, refuses to mutate the port if release fails, and persists the port only afterward; rustfmt made both guards multiline. Reused the already-passed `native-dropdowns`/Preferences review and changed the source guard plus its mutation-negative cases to whitespace-tolerant regular expressions. The ordering assertion remains exact and no dual/fallback transport behavior was introduced.
## Four-box first-wins profile build guard

The new ESP32-S3 first-wins build accepts a bounded initial key mask so a fresh
four-box controller can boot with boxes one through four enabled instead of the
nine-box default. The first focused storage test correctly stopped at its
hash-bound Pealayer `scripts/run-windows.ps1` proof after the test file changed.
Re-read `docs/REUSE-PREFLIGHT.md`, the complete donor runner and Pakar's complete
`tools/build-desktop.ps1`; retained the donor's canonical locked-cache/explicit
build-mode contract and added only plan-time assertions for game role, mask 15
and the unique `keycontroller4` hostname. Updated the four exact proof hashes;
no build cache, compiler output, board firmware or deployed application changed.

## Puzzles shift-register review and first-wins latency

Read the complete ControllerBoardMini/Puzzles LocalLib/ShiftRegisters.h/.cpp,
Puzzles call sites, Pakar BoardPins.h, and both complete Pakar shift-register
sources. The first attempted test patch failed because the expected tail had
already gained virtual-input coverage; it made no change. Re-read the actual
file and extended that existing fixture instead of overwriting it. The executable
fixture now covers both game-role input orders, virtual inputs, invalid indices,
active-low single/batch outputs and the exact shifted byte.

The donor serviced the shared register every loop, while Pakar imposed a 5 ms
interval. First-wins now uses a 1 ms interval before its existing debounce/UART
path; the table role retains 5 ms. Source guards and both role fixtures pass.
Added exact hash-bound comparisons for the two donor files: Puzzles pending
comparisons fell from five to three and scoped improvements rose from four to
six. Physical input/relay identity, bounce, clock integrity and measured
switch-to-UART latency remain live-board acceptance gates.

## PuzzleBoard residual XS closure and four-box hardware boundary

Read the remaining nine PuzzleBoard source files through EOF, including the
byte-identical XS/XSS UART pairs, finite Timer2 blink sketch, direct segment
demo, 623-line Bluetooth/power controller, 468-line Android/RTC controller and
447-line relay/input controller. Every PuzzleBoard inventory row now has an
individual hash-bound decision: 7 equivalent-behavior, 14 scoped-improvement
and 1 parity-gap, with zero generic pending comparisons. Applicable silent
settings, input lifecycle, relay ownership, finite indication, color damping,
audio and serial intent map to the shared role-aware owners. Unsafe/unframed
ASCII parsing, SoftwareSerial, RTC/I2C, invalid ADC6/ADC7 pull-up assumptions,
incompatible pins, blocking delays and donor product business remain explicitly
replaced or not applicable with reasons.

The generated comparison now accounts for all 4,697 donor files with 375
hash-bound comparisons, 4,322 pending in other donors and 275 known parity
gaps. Generator and JSON/diff checks pass. The legacy evidence-binding checker
still stops first at the pre-existing ControllerBoardMini
`Timer/.vscode/arduino.json` row because its manual CSV evidence is absent; this
was not relabelled as a success.

David-ASUS was queried through both Win32 serial and present Plug-and-Play
inventory. It currently exposes only built-in COM1 and no ESP32-S3 USB/JTAG,
CP210x, CH34, FTDI or other new serial device; neither `keycontroller.local`
nor `ledboard.local` resolves there. Prepared four-box ESP/AVR images therefore
remain staged and untouched. Physical backup, flash/readback, 38,400 UART,
buttons 1-4, relay/audio/LED behavior and human acceptance remain open.

## Keycontroller rename and command-admission recovery

Live ASUS COM27 identified ESP32-S3 revision 0.2, MAC 7c:e8:b1:b0:61:10,
16 MB flash and 8 MB PSRAM. The instance was renamed through its fenced API
to keycontroller; ASUS resolved keycontroller.local to the same peer. Enabled
mask 15, extended effects, 700 ms chase and adjacent-pixel fade were saved.
An unconfirmed key mask and timed-out AVR settings read recovered after one
ESP restart. Subsequent mask/audio ACKs and settings GET/SET/GET succeeded:
Silent=false, persisted=true, unchanged other settings; two API resets both
received AVR ACK and retained winner=0. This proves two-way UART, not physical
button/optical/GPIO acceptance. The ASUS tunnel subsequently became unavailable.

Reread complete PCController UartProtocol.cpp and link/discovery.go. Adapted
bounded HELLO readmission after three unconfirmed idempotent mask exchanges;
no reset/key/melody replay. First-wins boot now clears its old winner and
diagnostic while retaining palette, enabled boxes and effects. Protocol and
first-wins renderer fixtures pass; candidate compilation/deployment and human
feedback are separate pending gates. S3 identity now includes its generated
application profile, so hostname defaults no longer depend on SDK-wide flags.
Build cache selection retains an existing exact FQBN/compiler-flags match
without copying it and isolates other hardware profiles.

Physical follow-up: David confirmed a deliberate power cycle and then confirmed
rainbow working. The next boot retained UART telemetry but not mask admission;
auto_reset_ms remained 0. Added bounded HELLO recovery, current AVR reset revision
and GPIO21 assertion telemetry. Reset-counter synchronization now preserves a
winner contained in the same observation. Restating the active baked role via
USB, REST, cloud/WebSocket or AVR SetMode ACKs without clearing the round.
Explicit reset remains separate. No new delayed network-boot reset was retained.
Scene fixtures, boot/reset source guards and radio fixtures passed; both actual
firmware builds passed. These do not prove the physical hold/reset sequence.
The full comparison-report regeneration hit an unrelated stale bridge_events.rs
mapping; it was not rehashed or relabeled as reviewed to bypass that boundary.

ESP local Async OTA completed with full running-image SHA256
868831269524dd13a88d718df10cdae71f062f3fcd67074afdff751c8c27dabb,
2312704 bytes, VALID selected/running slot and same-peer WebSocket. Boot
fcee210a6c6a7dd16b49bfa001d67557 automatically confirmed mask/audio; GPIO21
observed released, auto-reset 0, silent Off. AVR candidate
95ff7cb02d308a03dc3556432ba0ef34228118b8f1e36a89066caff02449db6c
was verified on ASUS. Urboot job1 stopped before any flash write at Prog ACK
admission; AVR boot count advanced and BORF was reported. No blind write retry
or protection/fuse downgrade. AVR returned online, Prog clear and mask confirmed.
Rainbow reselected; new-image optical/winner-hold acceptance remains pending.

Four-box audio isolation: user confirmed Silent winner remains latched. A single
1032 Hz / 70 ms API tone then reproduced AVR restart (27803 -> 27804, BORF=4),
with ESP boot unchanged. GPIO21 release has no audio-stop handler; only Down
requests reset. Keep winner Silent pending unloaded-buzzer/power diagnosis; do
not disable BOD or claim production audio fixed. Reread PCController LocalLib/
TonePlayer.cpp and Project/BootMelody.cpp completely; the Timer1 hardware driver
is retained. Prepared a shared WinnerPresentation schedule: 240/120/240/120/360
ms, three pure-color pulses and corresponding ascending notes, then existing
ring/pulse effects. Renderer fixtures pass; these source changes are not deployed.

Subsequent user-requested unmute was applied and ACKed over ASUS Wi-Fi. A
70 ms tone still caused boot 27805 -> 27806/BORF; do not describe the running
board as silent or sound-stable. No further audible test/deployment in this pass.

Requested configurable presentation supersedes the fixed three-pulse draft:
ESP default is two 500 ms winner-button flashes with 250 ms gaps, 750 ms loser
fade, per-group colors/motion, pulse and a shared chase over enabled losing
indicator groups. KeyPresentation is used by the renderer; KeyGameSettings
stores validated settings with the existing staged/readback/rename persistence.
ESP schedules finite AVR notes from the flash clock and sends Silent as the
AVR's cached autonomous winner cue. Other named cues remain selectable but
their internal notes do not follow arbitrary custom flash timings. Settings
changes preserve the active presentation epoch. GPIO21 remains Down-only.
HTTP/WS game-settings accept presentation patches; fenced USB opcode 0x69
accepts one global or group patch. Remote press opcode 0x68 requires new AVR
HELLO support, uses chooseWinner, rejects disabled/conflicting winners and has
HTTP /api/game/press and WS game-press dispatch. Web includes parameter/group
editors and a test-press selector. Rust/Flutter now implement timing/group editors,
capability-gated remote test presses and selected-field post-ACK readback. Existing
native/mobile live pixel previews are retained; Web live per-pixel group preview
is still pending. None of these new client controls has on-device acceptance.
Renderer fixtures for both roles and embedded Web JS syntax pass. AVR builds
at 31502 flash / 1642 static SRAM; only 406 bytes remain for stack/locals, so
physical stack and audio acceptance remain open. No AVR flash was attempted.

Flutter analysis reports no issues and both focused presentation validation/readback
tests pass. Native compile found a missing eframe::egui import in the new editor;
fixed without adding a dependency; focused native build and both tests pass. Native USB
encoding now validates inherited settings before narrowing integer casts, with
regressions for malformed timing/motion/RGB readback. Formatting-only evidence
refreshes were checked against recorded HEAD hashes and independently identical
rustfmt output. The separate table_workspace change adds bounded saved key
positions, not command authority; reviewed before refreshing its evidence.

Final ESP candidate build passes at 2323131 bytes flash and 116788 static RAM.
Both role renderer fixtures pass again, including requested flash boundaries,
loser colors, disabled groups and non-restarting presentation on settings refresh.
Full comparison report generation remains blocked by a pre-existing changed
desktop/src/bridge_events.rs mapping; no blind whole-ledger refresh performed.
Issue #2 checkpoint: https://github.com/atomicdeploy/PakarController/issues/2#issuecomment-6336793222

Client/network deployment attempt: actual ASUS keycontroller was reachable by
hostname, but old build23 Keys pixel preview was stale. Added explicit on-demand
pixel-subscribe / pixel-frame WS observations to ESP, Rust and Flutter, with
same-board/boot validation, latest-frame backpressure and no command-clock renewal.
Native/UI/bridge demand and Flutter foreground/view demand govern subscriptions.
Static subscribers receive a repeated frame confirmation every two seconds;
animation observation is bounded independently of LED rendering. Flutter analysis
and focused foreground/reconnect test pass; native release and focused frame
identity test pass. ESP builds at 2324195 flash / 116916 static RAM.

Actual AVR silent setting was read false, set On, then read true through ASUS
Rust network CLI. No audible tests followed. Current image backup verified at
868831269524dd13a88d718df10cdae71f062f3fcd67074afdff751c8c27dabb.
First Wi-Fi upload lost the SSH tunnel: subsequent /update/info proved aborted
at 2274624/2324320 bytes, unchanged boot and running-image MD5. No blind replay.
Background updater/downloader launch was explicitly blocked by execution policy;
no alternate launch was attempted. Current ESP remains old, without pixel-events
or game-presentation. Current ASUS PID5416 remains old; final observation had
network/WS offline despite board HTTP working, and stale pixel preview. No claim
that the requested runtime repair is deployed or production accepted.

Verified ASUS staged APK: staging/key-presentation/PakarController.apk,
61246653 bytes, SHA256 a932b71ad114b08b275a187e5fc97f87ed8dc4fe46fe809248e689a01abbc4e1.
Verified staged ESP: esp32s3.ino.bin,2324336 bytes,
SHA256 be34c0fec48b88f73ae7c25c0fdadc8b5f91087e9d9d276515b6e42062215a4d.
Native staged release SHA256 3f507719c4e84bf72b275f656994d34e676703f3f0d1846f442db1178894ec56,
31897088 bytes, also verified at office HTTPS /pakarcontroller/pakarcontroller-presentation.exe.
ASUS and local ADB listed no authorized device. Android install/launch and actual
client network command/live-pixel acceptance remain open; user action requested.

Follow-up: ASUS and authorized Samsung R5CT44LZ42K are now reachable. Existing
Rust build23 connected with live WebSocket; existing Android received board
state. Actual screenshots exposed a SupersededRead message and disappearing
contextual controls. The refresh handler now ignores only SupersededRead, leaving
the existing expiry/authentication/connection fencing intact. Re-read the actual
PCController peripheral-availability.ts ready-and-advertised rule; no capability
is fabricated to keep controls enabled. Inline artwork now reuses the last
validated PNG buffer (one bounded entry), avoiding MemoryImage cache churn on
identical telemetry. The icon identity regression and all six peer-context tests
pass; all six connection lifecycle tests pass; Flutter analysis is clean.
Selectively refreshed only main.dart, peer_context.dart and peer_context_test.dart
hash references after reviewing their changes. No whole-ledger refresh.
Android still has only 560 MB free; prior install failed for insufficient storage.
The board still reports the old image and aborted OTA. The prior policy-denied
updater launch has not been rerouted. No firmware, effect configuration or sound
changes were deployed during this follow-up. Direct HTTP/WS buzzer handlers call
sendToAvr without a deliberate wait; actual audible latency remains unmeasured.

Keycontroller AP handoff: ASUS Wi-Fi joined the existing keycontroller profile;
USB tether Ethernet 12 metric is 10, Wi-Fi metric 500 (previous automatic metrics
25 and 55). SSH returned after the interface transition. Both hostname HTTP and
direct AP 192.168.4.1 identify esp32s3-7ce8b1b06110, AVR online. Root UI returned
200; /update/info exposes AsyncWebServer /update and the user-deployed candidate
MD5 5888f6c9e4a99cb838c11dba3a8f7579. This is API/path proof, not a second OTA.
Ten finite tones from a short opening phrase of Robson Couto's 2019
arduino-songs/pinkpanther/pinkpanther.ino were accepted via /api/buzzer, with
unchanged ESP boot and AVR online afterward. Attribution source:
https://github.com/robsoncouto/arduino-songs/blob/master/pinkpanther/pinkpanther.ino
Sequential HTTP requests took 312-500 ms and stretched the phrase; no claim of
accurate melody timing or physical audibility. Human feedback requested.
Added optional last_flash_on_ms (zero inherits the regular flash width), shared
renderer/audio phase calculations, and boundary tests for three 120 ms flashes
plus a fourth 400 ms flash separated by 100 ms. Both role fixture suites pass.
Only affected application/test hash bindings were refreshed; route registrations
are unchanged. Candidate build pending; running settings remain two 500 ms flashes.

## USB provisioning and maintenance draft

Read PU850 LocalLib/AsyncOTA.cpp and .h, the existing FirmwareTransfer platform
boundary, ShellWire exact-duplicate admission, serial-wire.ps1, and
SystemPlatform mount policy. Reused shared command/configuration dispatch and
firmware-read helpers instead of copying the donor's automatic final boot
selection or filesystem-upload path. Normal ESP32 startup now never formats,
including when an obsolete commissioning flag is supplied; the build wrapper
rejects that obsolete switch with the replacement tool path.

Added UsbApiRequest and FilesystemInitialization bounded guards, UsbApiService,
invoke-usb-api.ps1 and initialize-device-filesystem.ps1. Only completely erased,
unmounted storage can be initialized, with physical USB, explicit output disable,
an expiring consumed confirmation and separate reboot request. This is not a
corrupt-filesystem erase/repair command. Shared API envelope provides existing
commands, redacted network/cloud/Ethernet export, single-section typed import,
bounded firmware reads and restricted user-file reads with chunk SHA-256.

Native guard tests and PowerShell syntax checks pass. Full firmware compilation
stops at the pre-existing review gate: PCController LocalLib/I2cLcd.h target
firmware/esp/src/WebUi.h changed. No gate bypass, board action, candidate binary
or physical acceptance. The USB adapter itself is NOT compile/device verified.
Remaining: installer integration/postboot mount verification, USB staged firmware
upload/apply with shared ownership, file writes/listing and immutable snapshots,
atomic complete settings restore including explicit secret policy, opt-in cloud
sync, clients/progress, interruption/reconnect/security tests. No production claim.

### Flutter cell actions and stable header artwork

Read the complete PCController mobile/lib/unified/workspace_chrome.dart donor:
adapted its 48px targets, reserved one-pixel outlines, muted accent surfaces and
explicit selected state into mobile/lib/cell_controls.dart. Main table commands
retain their existing connection/capability/pending guards. Options keep their
existing persisted workspace values. Header artwork now compares bytes before
changing MemoryImage identity, uses gapless playback, and is keyed per board so
another board cannot inherit the previous icon. No command or board mutation.

Canonical build-mobile Test with test/cell_controls_test.dart: analyzer clean,
two widget tests passed (accessible icon actions/disabled guard; unchanged image
bytes retained across telemetry rebuild). This is source/widget evidence, not
an installed APK or observed on-device flicker acceptance. Reviewed main.dart
references refreshed without changing their retained capability/ownership anchors.
Full comparison generation remains blocked by a separate changed target:
PCController Tools/Controller/internal/localdevice/events.go ->
desktop/src/bridge_events.rs. Its hash was not blindly refreshed; full ledger
acceptance remains open pending that independent source review.

## ESP LAN peer relay checkpoint

Consulted bridge-repeater and network-cloud; reread PCController
Tools/Controller/internal/hostbridge/peer_response.go and its tests plus
internal/netpolicy/http_url.go. Adapted request/receipt identity, uncertain
outcome handling and restricted destination policy into PeerRelay.h and
PeerRoutePolicy.h; existing target/test hashes remain in the same file ledger.
This does not close the mesh, streaming, discovery or peer firmware/file gaps.

Real probe caught missing JSON body ownership for the new peer endpoint.
Extended the existing AsyncHttpService allowlist (also fixing its maintenance
JSON endpoint), not a second HTTP server. Fragmented-body regression in
web_access_fence_test.cpp and async queue tests passed. Corrected S3 build and
OTA followed; full image readback SHA256 independently matched after the inline
updater readback timed out. ASUS AP reassociation recovered post-reboot access.
Evidence: outputs/PakarController/recovery/keycontroller/peer-deployment.json.
Keycontroller relayed identity/status GETs to the actual ledcontroller, whose
AVR was online. No cross-board actuation, reverse relay, BLE, ESP-NOW, USB-host,
file forwarding or physical LED acceptance is claimed.

## Native button geometry and key-box workspace checkpoint

The regression review traced actual donor history, not an inferred visual style:
PCController `f2260e30525de82a95b00adddd2ebb3eda8d8443` keeps the CSS button
border/padding fixed and transitions only color; `a304c3ec366b0e9c11634526587ee9e5575acdff`
removes the active `clip-path: inset(1px)` geometry change. Pealayer
`5b3d5c150d714a3f951378867791b06855155094`, `hardware_action_button` in
`src/ui/layout.rs`, pins colored RichText to the Button font and recolors the
inherited stroke width instead of inserting a new one-pixel layout margin.
The accompanying hardware-status-interaction verification explains the observed
two-point caption-height shift. These are scoped donor function/history reviews;
they do not certify the complete Designer or every native control consumer.

Pakar adapts those invariants in `desktop/src/ui/dialog.rs`, `palette.rs` and
`control_density.rs`: hover/press/focus change color, not allocated geometry, and
semantic actions respect a zero-stroke menu context. The new production-helper
regressions live in `button_geometry_tests.rs`; its checkpoint SHA256 is
`2930a5e706fa835f704f097c44e664903aeed7f11b63557408ae2a8411d66219`.

The contextual key workspace now delegates to `desktop/src/ui/key_boxes.rs`.
Its drag grab-point/source-rectangle mechanics adapt Pealayer `layout.rs`
`remember_drag_offset_on_press` and `drag_translation`, traced through history
`e47f77392c35cb8f5d8177445c688c345ca5fa63` and
`d71c88d6dfa670339dbfc808c9724c52ac12c821`. Pakar keeps its own per-board
normalized layout owner and game authorization: fixed inside strokes, snapped
nonoverlapping cards, pointer-distance grid illumination, valid/invalid drop
previews, release-only persistence, keyboard nudging, Escape cancellation and
disabled-box dimming do not dispatch game commands while editing. The two group
previews use received RGB samples, not inferred output success.

The mandatory native gates initially rejected stale mapped targets. Scoped
rereads covered PCController session admission/write/close ownership and the
current mobile lifecycle/readback paths; Pealayer dropdown production wrappers
and their consumers; translation input alignment; color-picker Recent colors;
workspace icon picker; and shared Web localization. Only those affected records
were rebound to reviewed current target hashes. Historical complete-file claims
remain historical; pending parity items and omissions remain open. Localization,
native-dropdowns and peer-binding gates now pass. The shared localization test
also passes; this is not native runtime, board or deployed-client acceptance.

`tools/native-key-workspace-fixture.mjs` reuses the existing peer fixture identity
and request contracts to expose a loopback-only SIMULATED first-wins peer. It
uses a new isolated config, disabled production IPC/tray and four explicit
ready/winner/pressed/all-box scenarios. Hardware POSTs are rejected, WebSocket
actuation is never acknowledged or forwarded, and Ctrl+C retires only its own
spawned fixture child. The receipt records executable hash, PID, endpoint and
profile; native capture must target that exact PID and a new output file. Node
syntax checking passed. Production ASUS SSH still refused the banner exchange
on localhost:8022 during this checkpoint. Native build/test, inspected captures,
ASUS replacement and human visual acceptance are pending the root checkpoint.

Root's subsequent canonical native Test checkpoint passed 121 tests selected by
the `ui::` filter (including its `terminal_ui::` matches), with zero failures.
This includes the new button/menu geometry regressions and the key-box drag,
collision, cancellation and command-isolation tests. Both complete small donor
files `control_density.rs` and `palette.rs` were reread, their reviewed current
target hashes refreshed, and the actual new button-test hashes/anchors bound to
their existing records. Active/open stock chrome is now explicit semantic color
with constant width rather than egui's default white two-pixel pressed border.
Release compilation, inspected native captures and actual ASUS replacement are
still pending; test success does not close those gates.

Subsequent release/visual checkpoint: canonical `build-desktop.ps1 -Mode Release
-Jobs 8` passed, and `stage-desktop.ps1 -Probe` verified the stable candidate.
Executable SHA256 is
`4702ec91a55084f07451b011012af4242f0b1d29914d94f1ddc921974a2e0a54`.
This package includes the preserved current working tree, not a newly claimed
clean source commit. No firmware or Android build was needed for this native fix.

The isolated native fixture's initial WebSocket handshake omitted the requested
`arduino` subprotocol. Only the fixture was corrected; supported negotiation and
all 18 twelve-pixel group slices were checked. The inspected actual framebuffer
at `build/native/key-workspace-reviewed/startup.png` shows WS live, both winner
group previews green, enabled losers red, disabled boxes dimmed, and aligned
fixed-size cards. `arrange.png` in the same folder is a read-only Win32 capture of
that exact fixture PID; its name does not imply arrangement mode was activated.
The external Windows input helper could not supply trustworthy coordinate
geometry, so no manual pointer-drag acceptance is claimed. Drag/drop, overlap,
cancel, keyboard and release-only persistence remain covered by the passing
native tests. Captures are simulated peer evidence, not physical-board evidence.

ASUS subsequently became reachable again. Production replacement remains a
separate same-session lifecycle/draft-checked operation; the successful local
build, stage and fixture are not an installed-ASUS claim. No board command,
firmware update, reset or LED change was used for this UI review.

ASUS replacement then completed through the existing reviewed
`replace-desktop.ps1` in David's interactive session. Its fresh guard confirmed
all drafts closed before gracefully retiring PID 19500 (old executable SHA256
`489404dc08fd3d04e527d86252bc05dc6504ca0d9d36231272c93c3b9334ca4e`).
New PID 23104 is ready with the candidate hash above. Settings and the previous
executable were preserved. Evidence is in the task workspace's
`outputs/PakarController/verification/key-grid-replacement.json`.

The first observation helper incorrectly treated the UI-launch CLI acceptance
line as JSON. The failed observation was preserved; the helper was corrected
against `main.rs`'s actual control-only versus forwarded-launch contract, without
replacing or rebuilding the app again. The subsequent same-session observation
completed: Wi-Fi online, WebSocket live, zero active diagnostic incidents,
`ledcontroller.local` selected and its acknowledged AVR online. The correlated
1125x750 framebuffer capture `key-grid-ASUS.png` was retrieved and inspected.
This shows the actual deployed table workspace; the new key-box design remains
the separately labelled simulated native capture, not an assertion that ASUS
was switched to or physically tested with keycontroller. No board actuation.

## Tablet footer density and already-applied cell actions

The requested Flutter correction reread the complete canonical PCController
`mobile/lib/unified/relay_style.dart` and `workspace_chrome.dart`, not just their
mapping descriptions. Their live original-byte hashes still match the recorded
`a473cc74124b998689c69e1dc92422ce603d5ac488a706b86f0a8c906731a9d7`
and `f00ed672c1f72f1a66fb65cff1cfe0a89dea5f7b6222f154db00a7c52c2e1a82`.
The former's actual tonal variant is now adapted for a satisfied cell action:
transparent semantic fill, contrasting semantic ink, unchanged hit/paint
geometry and a check icon. The donor relay palette and unrelated presentation
preference are not copied. Complete selected semantic state plus a live
connection determines satisfaction; rendered RGB, a successful send, partial
observations and empty selections cannot mark it applied. Explicit taps remain
available to replay a requested effect. Pending outlined/relay-wide parity is
not silently closed by the newly reused tonal variant.

The footer retains the donor's minimum 48px touch-control rule but removes the
additional outside vertical padding which previously produced a 64px row.
One-line message/peer details keep ordinary telemetry from changing its height;
tooltips, screen-reader text and the existing error dialog retain full details.
Large accessibility text can still grow when needed. Safe insets are owned by
the parent workspace and not duplicated by the status row. Existing activity
strip/rail navigation and selection ownership were reviewed, not replaced.

Scoped source proof is bound to `cell_action_application_test.dart` and
`status_bar_layout_test.dart`: semantic aliases/mixed/missing states, readback
rather than optimistic send success, contrast and invariant geometry, 48px
tablet row, long live details, error-detail action and phone/tablet large-text
LTR/RTL coverage. Their addition is not a test-execution or installation claim;
the root checkpoint will record those outcomes separately.

The mandatory mobile gate also exposed stale whole-file hashes for two existing
consumer slices. PCController's complete `peripheral-availability.ts` was reread
at unchanged source hash `fcf2e397564c843711fd64db2bd2d820552180ac4f5abdf0d44ccbbb86be7a84`;
Flutter `_command` still rechecks selected owner, fresh advertised capability
and correlated post-ACK readback. Pealayer's complete 1280-line `board_info.rs`
was reread at unchanged source hash
`ab1c99e674e902e37626963e2dd5cf7d993965334411d5e33dafd1ea2bf79e51`;
the actual native `synchronize_context` slice still separates retained workspace
presentation from live command authority, retires intents on peer/boot changes
and keeps capability projection out of durable hide intent. The prior native
box redesign did not change that mapped contract. Only these reviewed consumer
hashes and the two Flutter UI mapping records were rebound. No unrelated stale
source, firmware, comparison report or pending omission was mass-refreshed.
All nine mandatory mobile build concerns now pass their current reviewed gates.
Production board firmware, LED output and commands were not touched for this
source-review checkpoint.

The next canonical mobile test hit the shared-font generation guard because the
preceding native key-grid work added seven translations which were not yet
regenerated into the shared Web dictionary. The complete canonical Pealayer Web
`i18n.ts`, current `localization.js` runtime, font generator, HTML-source decoder,
portable-gzip helper and canonical Web packaging functions were read before
repair. `build-web.mjs` regenerated only owned static packaging/embedded regions;
branding, interface and appearance generators reported no changes. Comparing the
localization module in memory before/after proved its handwritten runtime was
identical; the dictionary added only Box, Column, Row, Button, Indicator, Winner
and Lost. Canonical HTML, its C++ header, portable gzip and worker content
revision were refreshed together. This is generated static asset work, not a
firmware build or board deployment.

`generate-font-assets.mjs --check`, `generate-web-compressed.mjs --check` and
`tests/localization_test.mjs` passed. The existing localization mapping and the
two already-reviewed device-context Web consumer hashes were rebound after
checking their current ready/capability and board-sound ACK/readback guards.
Device-context and localization gates pass. Other stale historical mappings
were not silently rewritten. The generated full Web application is 410127 raw
bytes, 117246 gzip bytes, revision
`184ee67a97e49eb0d36034dfe3cec207c93b00a4329993e694f8d5eb121136bc`;
this does not certify unrelated Web feature parity or installed firmware.

Root's subsequent canonical focused Flutter test checkpoint passed all three
`cell_action_application_test.dart` cases and all four
`status_bar_layout_test.dart` cases; analysis was clean. The applied-state test's
final Dart-format change was checked as indentation-only and its proof hashes
rebound. Existing button-style regression execution, APK packaging, actual
tablet installation and visual/touch acceptance remain pending at this point.

Final local tablet-UI checkpoint: all five existing cell-control style regressions
also passed, for 12 focused Flutter tests total with clean analysis. Canonical
APK generation with `-BuildOnly -BoardProductionHold` succeeded; package SHA256
is `6aaa098338d44d47a4a10b1af08d6e4f31ac2b8a385028947633cf0dfcda370e`.
The preserved dirty working tree is based on
`8ce8f29eee01a979bbd42e7d3573a6e3ace9eba2`; this is not a new clean-commit claim.
ASUS staging completed at
`C:\Users\David\Documents\PakarController\staging\PakarController.apk`;
the remote 61723797-byte candidate matched the complete local SHA256 before
replacing the previous staging candidate. The final ADB inventory still listed
only phone `R5CT44LZ42K`, not tablet `R52T60C6V9B`. Nothing was installed on the
phone. Tablet installation and actual visual/touch acceptance remain open.
Production hold remains active; no board command,
firmware update or output change was performed for this client-only correction.

## Native semantic lamp backgrounds and stable hover contrast

This correction uses the existing Pealayer-derived Preferences draft,
Preview/Save/Discard lifecycle and fixed-geometry painting contracts. The
canonical donor Preferences draft helpers, dialog controls, dropdown wrapper,
control density, localization, icon wrappers, CLI output and native startup
were reread against the actual affected consumers. Pealayer's update download,
integrity and staged apply slices were also reread for the existing
`app_management` Preferences mapping; the new workspace preference does not
change that updater or introduce a replacement path. PCController's full
`session.go` and request-admission regression source were reviewed for the
current peer-owner, correlated-operation and non-replay boundaries. This is a
scoped extension of existing records, not a claim that unrelated pending donor
features were completed.

`color_cell_backgrounds` remains opt-in for existing configurations. The new
Workspace preference paints the acknowledged semantic state color on the lamp
surface; sampled individual-pixel RGB remains its separate live preview. The
palette accepts known idle/white, lost/red, won and off states, rejects malformed
or unknown state values, and avoids falsely naming a custom lost-blue palette
as RED. Hover strengthens the resting edge toward a contrasting ink while
preserving selected/focused identity, fixed hit rectangles and inside-painted
stroke geometry. The final reviewed render consumers are
`cell_table.rs` at
`cde79ae213c75757a8e248672feeddaa8a7b36ea33c70e09dca8810e0922a8e6`
and `cell_pixels.rs` at
`489c81021c648bdf0d21558ab0c27077345bbc9e6047fc3812f750d1b983df66`.

The app-only typed `CellBackgrounds on/off` IPC/CLI operation refuses to modify
an open Preferences draft, saves a cloned configuration before changing live
state, and exposes readback in app status. It neither focuses the window nor
issues a board command. Configuration round-trip, Preferences preview/discard,
typed IPC/CLI validation, semantic palette/caption and stable hover/preview
geometry regression anchors are recorded as source proof; their execution and
the requested live-user preference application belong to the root checkpoint.

Only reviewed consumer/proof hashes were rebound. Mandatory native concerns
`api-contract`, `client-app-updates`, `pu-tooling`, `build-storage`,
`localization`, `native-dropdowns`, `peer-binding` and `windows-startup` all
pass their current gates. The earlier shared Flutter peer-lifecycle mapping was
checked against its selected-owner retirement and correlated readback code;
later Flutter work must bind its own final source hash. No broad stale-record
refresh, firmware update, board actuation, app replacement or test/build
execution was performed by this source-review checkpoint.

The missing Flutter Pixel corrections entry was then traced to the acknowledged
feature allowlist, not missing page implementation. The complete canonical
PCController `peripheral-availability.ts`, actual Flutter parser, existing menu
and linked-tab navigation, and new navigation regression were read. Adding the
explicit `pixel-corrections` feature preserves ready/advertised and selected-peer
validation; it does not infer capability or expose it on unsupported boards.
The actual menu regression covers supported and unsupported manifests, mixed
hostname rejection, tab switching and zero writes while opening the tools.
The final parser hash is
`561c3b3e40d668ec302bca8fab546fa52ce47d21be6beee7dbd88cf6ed291043`;
the new regression hash is
`dd2af453296e262dbc08627b3b5b2a79652ed31dac16a7d627e792a6ab41a77f`.

The mobile gates also covered native files changed by the lamp preference.
Pealayer's current board-info advertised-capability, draft and board-settings
slices were compared with `synchronize_context` and `board_controls`; preserved
workspace state still cannot grant live authority or replay commands. The full
PCController LAN resolver was reread against native verified-response routing:
actual successful TCP endpoints are remembered only after peer validation,
while only connect failures request relookup. These existing mapped consumers
were rebound after review; no networking behavior was changed. All nine
canonical mobile reuse concerns pass. Test execution, builds and installation
remain separate root-owned evidence.

Root's focused source checkpoint completed successfully: the canonical native
`ui::` filter passed 127 tests (including `terminal_ui`), instance validation
passed 3, config locale tests passed 2, and CLI console tests passed 4, for 136
native tests total. Flutter analysis was clean and all 3
`pixel_tools_navigation_test.dart` cases passed. Canonical native Release and
Android APK packaging were started next; package hashes, installation, runtime
readback and visual acceptance were not yet available at this checkpoint.
These focused results do not close broader production or physical acceptance.

Both canonical packages subsequently completed. The native candidate SHA256 is
`876ad59544c6e3910c80da53407abb3dcc224f855482c3787578cf3f2d7f1370`.
Root's authorized, exact-process graceful ASUS replacement changed PID 23104
to PID 2452, preserving the prior executable and preferences. The saved
preference and actual app-status readback both report lamp backgrounds enabled;
Wi-Fi and WebSocket are online with zero active diagnostic incidents. The
workspace-root receipts
`outputs/PakarController/verification/lamp-background-replacement.json` and
`lamp-background-applied.json` were reread for these values. The actual
1920x991 framebuffer `lamp-background-after.png`, SHA256
`d9667ae334c43d0ca18c2252b7b44e7539fbb562392c572e4db76dd196a78b29`,
was viewed: white semantic lamp faces have readable black labels. That capture
also contains the startup mapping-read message; it is not proof of completed
mapping refresh or new physical LED behavior. No board actuation was required.

Root reported the production-hold Android candidate as 61723797 bytes with
SHA256 `a05cbfcd14f44081470c9561d31a86b50e564ede9108f71e8100651c5e92e4c2`.
It was replace-installed only on tablet `R52T60C6V9B`; installed base-APK hash
matches, and package identity is 0.2.8/build 23. Menu/tab screenshots and actual
tablet visual verification were still pending at this checkpoint. This package
installation alone does not establish complete mobile or production acceptance.

The later settled ASUS receipt `lamp-background-settled.json` resolves that
startup mapping caveat: all 80 mappings are confirmed, pending is false and
mapping error is null. The same PID 2452 still reports backgrounds enabled,
WebSocket live and zero active incidents. Its framebuffer hash is
`86438547d9e54e3d0b03a2a9679b252f1e6782e2bab514b756c6d9738097f256`.
The retained layout hash changed from `48c365...` to `b0a67b...` while visible
slots and split fractions remained unchanged; hidden Events/Streaming subtree
normalization is under investigation. No byte-exact layout preservation claim
is made.

The actual tablet captures at workspace-root
`outputs/PakarController/flutter-tablet-corrections-updated.png` and
`flutter-tablet-pixel-menu.png` were viewed. They show the retained 80-cell
selection and canonical `1-80` search, checkmarked tonal White action versus
solid Red/Green actions, the compact status footer, and both Pixel diagnostics
and Pixel corrections in the header overflow menu. Loaded corrections-page/tab
verification is still pending; screenshots do not substitute for exercising a
board mutation or physical pixel acceptance.

Tablet navigation verification subsequently completed on the same installed
build. Actual `flutter-tablet-corrections-tab.png` and
`flutter-tablet-diagnostics-tab.png` were viewed: Corrections loads the board's
applied mapping at revision 0, and the linked Diagnostics tab presents the
480-pixel range with board diagnostics explicitly inactive. Returning to Table
retained 80 selected cells, `1-80` search and live status. The delivery agent
reports no Save, Show, color or other board-write action during this check;
application data was preserved and phone `R5CT44LZ42K` was untouched. This closes
the scoped menu/loading/navigation inspection, not physical pixel actuation,
human acceptance or the full production checklist.

## Cross-client panel refinement and contextual control parity

The actual ten native dock-panel bodies and corresponding Flutter activity,
diagnostic and settings implementations were compared by function. The existing
`INTERFACE-CONTRACT.md` now records that inventory and corrects stale claims that
the mobile network/cloud and identity editors were absent. This is not a new
donor exclusion ledger or a claim of full feature parity.

Native panel rendering reuses the actual Pealayer-derived `dialog::section`,
action/setting rows, bounded scrolling and shared density/dropdown helpers. The
current Pealayer `src/ui/dialog.rs` was fully reread by the root/native agent;
new `effect_controls.rs`, `panel_details.rs` and the reorganized
`state_appearance.rs` are scoped consumers of those helpers. Fixed-role boards
allocate no mode-picker card or spacer, while first-wins reset remains available
independently. Correction rules use styled bounded controls and virtualized
mapping rows; network/hardware details use compact actionable groups without
turning display state into command authority. Responsive geometry regression
tests must pass before the candidate is called source-verified.

Flutter refinement reuses its existing workspace components and the actual
PCController `workspace_chrome.dart` Material/InkWell, semantic surface and
directional-affordance pattern, fully read by the Flutter agent. New card/grid
consumers are source mappings, not automatic certification of every donor
activity or preference. Numeric display, custom tones, configurable marquee
hold/interval and LED brightness/effect actions are aligned with existing native
API commands; command completion cannot fabricate observed board state.

For authentication parity, the complete actual firmware `AccessConfig.h/.cpp`
and `WebAccess.h`, native `access.rs`/`AccessChange` behavior, and the full donor
PCController `mobile/lib/device_settings.dart` were read. The latter's SHA256 is
`16c5f8ec88c242baa279bc42bdf4c81024f4473eccc62ec8e317267937a9ab2c`.
Mobile now has an explicit auth-enabled toggle and contextual credentials:
enabling from disabled does not require an old password; protected changes use
an entered current password or the existing connection secret; disabling does
not require new credentials. An exact ACK plus public readback precedes the
credential handoff. Conflicting public metadata preserves drafts and blocks a
write; a retired connection cannot receive a late handoff. First-wins LED
settings route to Output rather than the absent table-selection workspace.

The first canonical focused mobile test attempt stopped before Flutter at the
device-context gate because concurrent `peer_context.dart` changes made its
existing mapped hash stale. No direct test/build bypass was used. Root's initial
native UI test run passed 131 tests but found one new narrow-panel overflow; its
helper was corrected and the canonical rerun is pending in this checkpoint.
Final reviewed hashes and test results will be appended after source freeze.
One current ASUS SSH8022 attempt refused the connection; no app replacement or
board action is claimed by this source checkpoint.

Explicit remaining parity includes mobile streaming/screen-capture and event
history, native Urboot maintenance-method selection, Flutter snapped
collision-safe box placement/grid feedback, and platform-specific mirroring and
workspace preference differences. Installed visual/touch acceptance and physical
effects remain separate. Production board updates or output-changing tests were
not authorized by this panel-only request and were not performed.

The canonical Flutter follow-up passed a clean analyzer and the actual
`panel_refinement_test.dart` 10/10, `device_settings_page_test.dart` 13/13 and
`overview_hardware_test.dart` 3/3. These tests caught and drove the real custom-tone
PageStorage-key collision fix: nested TextField scroll storage must not inherit
the expansion's boolean key. The output-panel suite separately exposed a stale
test expectation (`1,21` instead of the established canonical `1, 21`); only that
expectation was corrected. No shipping selection normalization was changed to
make a test pass. Output-suite rerun and installed screenshots are separate.

Root's canonical native `workspace_layout` run passed 10/10, including hidden
Events/Streaming split preservation across focus/restart, view-only collapse/tab
scroll, and real split/tab edits. No native UI deployment is implied. Existing
reviewed shell references were rebound to
`c44f721fc91170d7c89cd1c06d2e80e6625ce8cb6972ca87669f6b835fd776d9` only after
the actual Streaming/read-on-open/shared-cell-action changes and anchors were
reviewed. The detailed panel/helper follow-up retains its independent gate.

The maintenance comparison uncovered more than a cosmetic selector gap:
native observes ISP/Urboot progress but still initiates legacy ISP operations.
It needs typed `/api/avr/maintenance` admission and exact boot/request/hash/expiry
receipt validation before exposing a truthful method control. Flutter already
implements ordinary ISP/Urboot probe/backup/program admission, but its receipt
parser and both client editors do not yet expose the newer explicit EEPROM
write/initialize operations. These gaps remain open in the existing ledger.
Android screen capture separately requires MediaProjection consent and runtime
lifecycle support; ordinary mobile event-history, host-stream controls and
collision-safe box-grid parity must not be excluded as platform limitations.

Full comparison generation was attempted through
`tools/generate-file-comparison.ps1` and stopped at the pre-existing/concurrent
`ControllerBoardMini|Timer/.vscode/arduino.json -> tools/build-firmware.ps1`
target-hash mismatch. A read-only audit found eighteen distinct stale target
paths, including this pass's owned interface document and unrelated
firmware/Web/build/theme/settings changes. This pass updates only its reviewed
UI source/proof references and final interface-document references. It does not
mass-refresh untouched records or certify unknown donor work. The global
comparison reports remain stale until their owning changes are actually reviewed;
no successful full-ledger regeneration is claimed.

After screenshot-driven hierarchy refinement, the final canonical Flutter
panel suite passed 11/11 with a clean analyzer. Actual board controls now precede
mirror preferences, use a responsive tablet grid and retain one instance of each
mirror toggle. The new test uses the real FrontPanelPage and an inert tone
fixture, not a source-text assertion. Fixture-only follow-ups retired the mirror
lease before invariant checks and settled scrolling before hit-tested taps;
invalid-tone/no-command assertions were retained. Other focused suites passed
settings 13/13, overview/hardware 3/3 and Output 6/6. Frozen mobile main SHA256 is
`44ff20603518e54a9fa117ea3087bff85166539e10828820fa64fc517ca83b94`; panel test
SHA256 is `ece795def39cd4e56bb00e3a7a5fc19aba8f9f68cc03c3f51b3ae320d15d8606`.
These are source/widget results; installed tablet and physical board acceptance
remain distinct.

The new `outputs/PakarController/mobile-panel-parity-front-tablet-controls-first.png`
(1200x1100) and `mobile-panel-parity-front-phone-controls-first.png` (390x1500)
were opened and visually inspected. Their visible SIMULATED fixture banner is
retained. They show number/text/melody controls before mirror preferences,
two aligned tablet columns versus one phone column, readable field labels and
bounded cards without visible text overflow. They do not establish installed
tablet state, an active board connection, tone playback or physical LEDs.

The final visual refinement moved acknowledged Live front panel into the fourth
responsive grid slot beside Board melody and reused the same WorkspaceSection
heading/icon/divider. Frozen main SHA256 is now
`0d65c17c84f76ae7880cec90802e2a5fba281052d4a30502c71640cc905488a9`, and final
panel-test SHA256 is
`1724b0b5a1229f68e251a6129401fe45060c1c23a17ff063af853510e87a4a7e`.
The canonical final rerun again passed 11/11 with a clean analyzer, including
both tablet row-alignment assertions. Final `mobile-panel-parity-front-tablet-final.png`
and `mobile-panel-parity-front-phone-final.png` were opened and inspected: the
tablet pairs display/text and melody/live status, while the phone stacks these
cards and keeps mirror preferences later. These final captures supersede the
intermediate layout illustration, not its source-test history.

Root reports four current ASUS SSH8022 refusals. No production board command,
firmware update or installed-device acceptance is claimed. The native isolated
capture uses a normal per-simulated-peer saved dock because startup panel IPC can
precede capability ACK; client auto-update is disabled only in that fixture.
This is not a production configuration change or an IPC authorization bypass.

Native focused checks passed the two Network/Corrections tests after fixing
inherited text wrapping and the actual Ethernet `link_up` field. Subsequent
inspection found that scoped action buttons cannot be reliably wrapped after
egui discovers their minimum width; a shared measured action row now selects
horizontal versus stacked layout before rendering. Present/Stop text,
correction read/save/discard and remote-settings actions use that helper.
The independent StateAppearance test then found a real 346px content extent in
a 220px panel. Its correction keeps the motion selector on its own row, uses
explicit local wrapping and the same responsive preview-action row. The strict
220px regression bound was not relaxed; final reruns remain recorded separately.

Flutter's final canonical `-Mode ApkPreview -BuildOnly -BoardProductionHold`
completed successfully. The final candidate is
`mobile/build/app/outputs/flutter-apk/app-release.apk` (reported 59.0 MB), SHA256
`af974a2cc77bfec2440ee1d5d274c2f7933cf07c352ad09e8025927c950de7ae`.
It uses preview signing and has not been claimed installed on ASUS's tablet;
package creation and screenshot fixtures do not close Android or production
acceptance. No board operation was part of this build.

Final native canonical focused reruns passed `panel_details` 3/3 and
`state_appearance` 2/2. The unchanged strict width bounds include 220px, both
themes, measured action-row layout, real Ethernet `link_up` observations and
render-without-command behavior. The previously reported `workspace_layout`
10/10 remains separate; no full native UI-suite rerun is inferred. Final helper
SHA256 is `0203bce2e25a7256187717be43fcf1fc11cd186aebcf7fdd6c389179fa55b8f6`,
appearance SHA256 is
`7543073f2dcf59cdb6beb31235c77bed0b575011857e8a5e155f353f7ba4cf02`, and reviewed
shell SHA256 is
`d70156f38160e1c81e44aa5a8444ed37cf3eb65ffb917372a12cdfee20f20301`.
The final panel audit retains ordinary parity gaps and platform-specific work;
the scoped reviewed evidence does not certify unrelated concurrent sources.

Root's subsequent canonical native `ui::` suite passed 134/134. Native Release
initially stopped at the expected stale Flutter-main evidence gate after the
last shared-source edit; no direct build bypass was used. The actual final
Flutter edit keeps Stop text allocated while inactive, enabling it only for a
fresh admitted text capability, no pending display command and explicit active
host-text state. The added geometry test covers unknown/off/on/pending/off at
390px and 1100px and checks a single release command. Reviewed main SHA256 is
`19afa076114d5e5eef8f2823de0d96af7a89d9010b0cc8a26f7b11ad5c886f81`; panel-test
SHA256 is `3bec5e0a800d4732cb99ad1dba61f7c9f8e2150f6ab8429448868e988aff26df`.
The prior APK is superseded pending the canonical test/package results, not
silently relabeled as current or installed.

The final source-admission pass found stale `desktop/src/main.rs` references at
the canonical wrapper gate. Rather than treating a new DTO field as grounds to
certify the entire file, all seven owning donor sources were fully reread:
Pealayer `src/main.rs`, `src/diagnostics.rs` and
`docs/superpowers/plans/2026-09-28-windows-taskbar-and-tray-suite.md`; PCController
`Tools/Controller/internal/hostbridge/lifecycle.go`, `link/session.go`,
`link/request_admission_test.go` and `lanresolver/resolver.go`. Their original-byte
SHA256 values still match the existing ledger. The default PCController lookup
reported live-unavailable, so the canonical installed source checkout was read
directly and its bytes checked; the lookup was not misreported as a live pass.

The corresponding nine target/proof anchors were examined in the current main,
with the actual `cli_console.rs` and `peer_http.rs` dependencies: GUI startup
does not allocate a console; explicit CLI attaches only an existing parent while
preserving redirection. HTTP scopes bind owner/boot/capability and never retry
uncertain mutations. Successful validated responses cache the real TCP address,
not an advertised foreign address; original hostname/SNI and TLS checks remain.
Connection failures request a fresh resolution without treating HTTP errors as
DNS changes. Read-only saved-route recovery revalidates the exact board, and
disconnect/method changes retire the route. USB ownership is checked at admission
and immediately before transmission, drops the old port before changing paths,
and preserves uncertain ACK/readback outcomes without replay. The existing
USB-selected/no-network-listener test and observed tray error reporting remain.
Only these nine references were rebound to main SHA256
`394784f6bc82cd1bd1fe8e29cf42468f7ab5c26459197dcc82fc4fb06748411a`.
Existing lifecycle, reset/electrical, panic-journal, full tray and physical
DNS/DHCP/TLS acceptance gaps were retained, not converted into complete parity.

Framebuffer inspection then exposed a real Hardware capability-chip overflow:
the final label could become a vertical column inside a late-sized Frame.
`panel_details::feature_chips` now reserves each measured single-line slot before
wrapping, clips exceptionally long labels, provides the complete tooltip and
accessible Label metadata. Its strict 220/300/600px test checks every feature,
one-line text and bounds. Controller actions reuse the same measured action-row
helper. Reviewed helper SHA256 is
`7aa67a431e46732ae5961ffce75621ee5bd4d4877e4ef8026c4311e0ee5acd1f`; shell SHA256 is
`7300ff3a1241edec21770d7d715c6bf0d5e84ec2a7f47919cfdb1af15bb6ed52`.
Native appearance/preview observations stay visible but commands are disabled
when the current transport cannot execute them; this is not USB feature parity.
Remote key admission now requires an explicitly applied mask, not an absent
Boolean default. Current USB table snapshots include that same mask through
`buildStatusJson(true)` and full `DeviceStatus` deserialization; legacy binary
status cannot manufacture an acknowledgement.

Flutter Stop-text stability passed the canonical 12-test panel suite with a
clean analyzer. The later output correction adds an explicit Off/reassert-black
action and one shared fresh LED-capability guard for output, mode, patterns and
brightness; Rainbow requires observed enabled output rather than silently
enabling it. The new eight-test Output suite is pending after source freeze at
this receipt. Source SHA256 is
`5cfaf88d1d511146d1fc265fd5bccd3f615868edd65a4e6c60b214158b535b92`; test SHA256 is
`8e5c47473a4d3fa6c48da9e6eebc1d77a2b860c2a2ffce228accb7a292acf607`.
The final native and APK rebuilds must follow these fixes; older passing tests,
packages and captures are not relabeled as acceptance of the new source.

After all reviewed hashes were current, the canonical native combined run passed
`ui::` 136/136 and `state_appearance::` 3/3. This includes unknown/false/true
applied-mask admission, strict whole-slot capability-chip layout and actual
disabled/enabled pointer interaction for state appearance and preview. The full
test binary compiled the current DeviceStatus constructors. This clears those
source-test gates, not installed runtime, USB actuation or physical outputs.
A fresh full target-hash scan leaves sixteen unrelated stale paths unchanged;
the owned interface document and actually rereviewed main references account
for the reduction from eighteen. Full comparison generation remains blocked
at the pre-existing `tools/build-firmware.ps1` reference, not silently bypassed.

The final canonical mobile Output suite passed 8/8 with a clean analyzer at the
recorded frozen hash. Together with panel 12/12, settings 13/13 and
overview/hardware 3/3, this is 36 focused source/widget tests. New
`mobile-panel-parity-front-tablet-stable-controls.png` and
`mobile-panel-parity-front-phone-stable-controls.png` in the existing outputs
directory were generated and opened: persistent disabled Stop text occupies
the same action row, actual board controls lead, tablet columns remain aligned
and phone cards remain bounded. These explicitly simulated fixtures supersede
the previous captures only for current layout proof. They do not establish
Android installation or any board command. The ASUS connection subsequently
recovered and the tablet was found; deployment remains a separate authorized
step with its own package/hash/runtime receipt.

The final canonical Flutter `ApkPreview -BuildOnly -BoardProductionHold` build
completed with exit 0 in 53.1 seconds. The 59.0 MB preview-signed package at
`mobile/build/app/outputs/flutter-apk/app-release.apk` has SHA256
`6149e8a52595e29bde315c9be201b398d96a56e397fe032defd658479a3aec8d` and includes
the stable Stop-text, explicit Off/readback and current LED admission changes.
It supersedes the earlier package hashes; transfer to the authorized tablet is
in progress and is not yet recorded as successful installation or acceptance.

A final read-only parity check found the native brightness slider used a field
updated by HTTP status but not USB snapshots, and a status refresh could replace
an in-progress drag. The bounded correction changes only the shared helper and
its existing LED-output setting row: live snapshots own the value outside a
gesture; a peer/boot/route/epoch-bound draft owns it during dragging. Release or
keyboard emits one existing command, while disconnect, owner change, Escape,
focus loss or a missed panel frame cancels. Main transport workers remain
unchanged. Reviewed helper SHA256 is
`bab8101d067b05e642be29cd28002aa9c102cc7c9c77c7a8a0242f8924c6a060`; shell SHA256 is
`f51b94dd6b3ab712d70b77c26c1dcdb4d852be26bfdabf92acf508faf98f72f0`.
Three actual pointer/keyboard/ownership tests were added; canonical results
remain pending here rather than being inferred from the earlier 136-test run.

The first brightness test compile found an FnMut ownership error in its new
offscreen harness: egui may rerun the frame closure, so it must clone the optional
owner when calling the helper. Only that test call was corrected; production
logic and shell stayed unchanged. Helper source/proof SHA256 is now
`71964b7c013434d3e7941081971061e6bda0d5d3a2c6a628176fd51c48330ef6`.

Tablet-only delivery then completed on authorized device `R52T60C6V9B`:
`adb install -r` returned Success, package version is 0.2.8/build23, and the
installed `base.apk` SHA256 exactly matched final candidate
`6149e8a52595e29bde315c9be201b398d96a56e397fe032defd658479a3aec8d`.
The canonical Android helper launched it after verifying an unsecured swipe
lock. Actual `outputs/PakarController/tablet-panels-live-launch.png` was opened:
the landscape shell is visible, but it reports no acknowledged board and waits
for fresh state. This is installed-package/foreground evidence only, not proof
that connected panels, commands or physical LEDs work. No board mutation,
firmware job or other Android device was part of this delivery.

The final canonical native `ui::` suite passed 139/139 with zero failures at
helper `71964b7c...` and shell `f51b94dd...`, including all three brightness
pointer/keyboard/ownership regressions. The separate `state_appearance::` 3/3
pass remains at unchanged source. Brightness's stale USB/refresh-during-drag gap
is therefore source-fixed and regression-tested; installed runtime and physical
brightness actuation are not inferred. Final Release/capture/deployment evidence
must name this later candidate, not the earlier pre-brightness executable.

Actual tablet navigation exposed an empty Flutter Scene wrapper for a fixed-mode
table, despite the individual mode choices being hidden. The bounded correction
projects that card out when no contextual content exists, retains its saved
position, and maps visible reorder indices back to stored card identities. A
first-wins board still retains its actionable game card. Transient missing state
preserves the known presentation without granting command authority. New source
SHA256 is `4af5131433c400eb8233ad4765fe7c36685d1cae2a0f85604427302ba2e9005e`;
test SHA256 is `48a2e554d9458892b6f6a82df46ac1a271b404ddda4b562662c980f62e63bffc`.
The added real widget regression covers one/multiple/null mode observations and
hidden-card reordering without commands. Its canonical test/package/deployment
results remain pending; installed package `6149e8a5...` is not relabeled as
containing this later correction.

The initial nine-test Output rerun correctly rejected an invented unsupported
fixture mode and a test's stale pre-reorder callback. The test now uses the
actual supported `first-wins` mode and obtains the current rendered reorder
callback before the second drag; shipping source is unchanged. Reviewed test
SHA256 is `d40cd2b5189af50d1e862629cae0facd6169d7151fbccb0325564a368312b4b6`.

Native final Release then completed and canonical staging with `-Probe` passed.
The candidate executable SHA256 is
`69f8c41c8f450d99c66af1e833b60a367d1bdcb32cf523f622d74536a11245d2`.
This is the post-brightness candidate; owner-safe ASUS replacement and current
runtime confirmation remain separate steps. The root also viewed later actual
tablet Hardware/Front/Output screens with ledcontroller.local connected,
22 individually acknowledged capabilities, AVR online and NTP synchronized.
Those later observations supersede the initial launch/no-ACK state, not the
need to install and verify the later empty-Scene correction.

The corrected canonical Output suite passed 9/9 with a clean analyzer, bringing
the scoped Flutter suite total to 37. Actual installed captures
`tablet-panels-live-hardware.png`, `tablet-panels-live-front.png` and
`tablet-panels-live-output.png` were also opened for the comparison record.
Hardware shows the observed 22 ACKs, AVR online and NTP synchronized; Front panel
correctly omits unacknowledged display/text controls while retaining acknowledged
melody and live inputs. The old Output capture is the evidence of the empty
Scene wrapper, not proof of its fix. A distinct final screenshot after the
replacement package is still required to close that visual defect.

Canonical ASUS replacement succeeded after owner/draft checks: old PID2452 was
replaced by PID5592 in session2, executable SHA256
`69f8c41c8f450d99c66af1e833b60a367d1bdcb32cf523f622d74536a11245d2`.
The receipt `outputs/PakarController/panel-review-replacement.json` reports
`replaced=true`, `error=null` and `ready=true`; old executable `876ad595...` and
settings backup were preserved. This establishes safe runtime replacement and
startup, not connected-state or physical acceptance. Final native framebuffer
and connected status were still pending at this checkpoint.

The replaced native process subsequently reported Wi-Fi online and the same
retained layout SHA256 before/after replacement:
`48c3650257a585e835b2122fd70b8ad512530390a41f319565ee926533b94091`.
An output-folder inspection wrapper initially expected JSON from `--panel`, but
the actual app branch returns `Forwarded to active PakarController`. The root
reconsulted the capture gate and donor package probe, then corrected only that
wrapper's exact text acknowledgement handling and rechecks the owner. A panel
acknowledgement is not screenshot completion: correlated request/path/result
and image inspection remain required. No application rebuild or claim that
Windows capture is unavailable was used to work around this tooling mismatch.

Actual final native `outputs/PakarController/panel-review-hardware.png` and
`panel-review-native-hardware.json` were verified and the PNG opened. Capture
SHA256 is `f1ae80946994c34cab69ca7825c57b110eb13d337044071807945c47405628a3`;
owner PID5592 runs executable `69f8c41c...`, Wi-Fi is online to
ledcontroller.local, WebSocket and AVR are live. The user's actual Table,
Network, Hardware and Effect controls split layout remains, not a fixture
projection. Capability chips are below the visible Hardware viewport, so this
capture alone does not prove their final visual layout. Panel navigation changes
view state, not the separately verified replacement-preserved layout topology.
The package was built from the shared dirty workspace based on
`8ce8f29eee01a979bbd42e7d3573a6e3ace9eba2`; no new commit or clean-tree claim is
made. No board firmware or physical output was changed by this UI delivery.

The final contextual Flutter package completed the canonical production-hold
build in 42.7 seconds: 61,839,005 bytes, SHA256
`5db85ea054539076e6daa08d80d56001eef5e1adb0330665b94f932116760431`.
ASUS's staged `panel-review/panel-review-contextual.apk` and authorized tablet
`R52T60C6V9B` installed `base.apk` matched exactly; `install -r` returned Success,
version0.2.8/build23, foreground PID7497. A bounded log query found no
Flutter/AndroidRuntime errors. Actual `tablet-panels-final-connected.png` was
opened: ledcontroller.local automatically reconnected, Output was restored,
the empty Scene wrapper is absent, LED/diagnostic controls are aligned and
live status is received. The earlier Hardware/Front captures remain evidence
for unchanged pages, not mislabeled as newly captured. This completes the
scoped client package/runtime/visual delivery, not physical actuation, a soak,
complete donor parity or a guarantee against all future bugs.

Publishing the scoped checkpoint to GitHub issue #3 was attempted through the
GitHub connector and rejected with HTTP403, `Resource not accessible by
integration`. No comment was published and no alternate-auth bypass was used.
The workspace remains dirty/uncommitted; the client package/runtime evidence
above must not be described as a pushed or published source checkpoint.

Final native Hardware and EffectControls fixtures used exact executable
`69f8c41c...` through the repository harness; both exited 0 and their owned
PIDs142672/246692 were absent afterwards. Both PNGs were opened:
`outputs/PakarController/panel-parity-review/hardware-final/startup.png`, SHA256
`33f35674793e7ba3629749d1425614d090998663b51955d44fcc00db2f907da2`, shows every
acknowledged capability chip fitting as a complete single-line slot with aligned
Controller details; `effects-final/startup.png`, SHA256
`18de066f7162a9dc1ae38b737090dc60651734bc2260021eec94bcc83f087525`, shows no
single-mode Board modes card, independent Reset and bounded output/brightness/
state controls in the visible viewport. Each folder retains fixture/request
receipts. Traffic was only GET status/auth and local client-report POST.
These are actual native framebuffers over clearly SIMULATED loopback data,
not device or physical-output evidence. No source changes followed these views.

## Flutter contextual header and connection modal

Scope: client-only header/status work. No board write, firmware deployment,
reset, audio or LED command. The retained production-hold APK flag prevents
saved automatic board-update jobs from starting after client replacement.

Reread complete donor sources: Pealayer `web_ui/src/components/HeaderBar.tsx`,
`src/ui/workspace_profiles.rs`, `src/branding.rs`; PCController
`mobile/lib/unified/workspace_chrome.dart`, `mobile/lib/unified/preferences.dart`,
`mobile/lib/connection_preferences.dart`, `mobile/lib/unified/relay_style.dart`,
`Tools/Controller/internal/link/session.go`, and
`Tools/Controller/web/src/peripheral-availability.ts`. Current donor hashes
matched the ledger. Existing unrelated omissions/stale references remain open.

Adapted the retained connection-modal/overflow lifecycle using the existing
SettingsPage and ConnectionDraft, not another transport or speculative replay.
The entire footer status icon/text opens Connection. Fresh ordinary telemetry
reads Connected; command failures remain inspectable through their separate
details action. Connect/unlink/cancel icons describe the action, not peer status.
Refresh is visible in the toolbar only while stale and connection is desired;
manual refresh remains in overflow. Reset remains a separate board action.
Header visibility/order is sanitized and stored by the existing coalesced local
workspace writer. Overflow cannot be hidden; defaults show connection plus
stale-only refresh. Editor supports drag handles and accessible move commands;
wide rows are compact, narrow/large-text rows wrap without shrinking targets.

Canonical build-mobile analysis passed. Six header tests, five status/layout
tests and six connection lifecycle tests passed, including zero mock board
writes on dialog/customization, reordered persistence after restart, all-hidden
recovery, stale/new-event transitions, USB isolation and failed-command retention.
Two stale global-text test matchers were scoped to their actual footer/dialog,
and the old OutlinedButton test now checks the real CellColorAction owner.

ASUS SSH and tablet R52T60C6V9B (SM-X906B) were rechecked. Final install -r and
launch succeeded for 0.2.8/build23; installed base.apk and local release SHA256
both equal `503d443f9e3f57f761be090b8eff3a40b4be3b213948be79ff3393827c73a638`.
User settings/data retained, phone untouched. Actual tablet screenshots under
outputs/PakarController show the live Connected footer, hidden fresh refresh,
and connection modal without disconnecting ledcontroller.local. Initial editor
capture revealed excessive tablet row height; this was corrected and rebuilt.
This is client/runtime evidence, not physical output or full donor parity.

Issue #3 progress-comment attempt was rejected HTTP403 Resource not accessible
by integration. No alternate-auth bypass, published comment or source commit
is claimed. Human touch/visual acceptance and phone deployment remain open.

## Table viewport: fit, anchored zoom and pan

Read Pealayer web_ui/src/timelineWheel.ts completely, source hash
1910311a3acba2f1d22d9d4fb57d43e08bcf6ee91f3ea567e381ae90e8a487cd.
Adapted its exponential pointer-anchor calculation to a two-dimensional table
in Flutter and the canonical Web UI. Native uses locked egui 0.36.2 Scene,
with global-to-local pointer conversion for marquee and order dragging.
Pealayer layout.rs gesture sections were inspected, not the entire file; its
pending full-file comparison remains pending. Media timing/gutter behavior is
not applicable; configurable modifier policies remain a documented omission.

Fit is the default and tracks available panel size. Manual view survives live
updates. Fit and bounded +/- controls complement pinch and Ctrl-wheel. Mobile
ordinary wheel input remains page scrolling; pinch must not activate cells.
Web wrapper consumers and marquee geometry now account for transformed bounds.
Final browser inspection confirmed compact square zoom buttons and all80 cells
visible after Fit; outputs/PakarController/web-table-fit.png is explicitly a
read-only simulated UI screenshot, not device evidence.

Canonical verification: Rust two scene/footer tests plus23 cell-table tests;
Flutter three viewport tests plus eight phone/tablet LTR/RTL table tests and
analysis; Web zoom math, cell-selection and canonical application contracts
passed. The Flutter pan test initially started on a clipped cell after zoom;
it now exercises a visible cell and confirms actual translation. An earlier
anchor recentering defect was fixed in both Flutter and Web. Generated Web
assets were refreshed through build-web after source changes; no gate bypass.

Only affected target hashes were rebound after checking retained anchors and
the scoped viewport changes. Full comparison-report regeneration remains
blocked by unrelated stale Timer/.vscode/arduino.json -> tools/build-firmware.ps1
evidence, which was not overwritten or certified. The table-viewport donor
gate passes with no stale references in this slice. No production firmware,
installed client, device scene, settings or actuator was changed in this pass.
Real installed touch/trackpad acceptance and client deployment remain open.

Final actual-device captures `tablet-header-editor-final.png` and
`tablet-header-final.png` were opened and inspected. All four editor rows fit
with aligned 48px move/drag controls; cancel restored the live Output workspace
without changing preferences or board state. PID11417 remained foreground,
Connected was visible with live RSSI, and bounded Flutter/AndroidRuntime error
log inspection returned no entries. All 15 affected hash/anchor references
validated; scoped diff whitespace check passed. These observations are bounded,
not a claim of long-term/network/physical fault freedom.

## Tablet chrome alignment and pixel-tool menu

Reread complete Pealayer HeaderBar.tsx and PCController relay_style.dart.
Retained the existing capability-gated grouped overflow and connection-modal
ownership rather than another navigation system. Flutter now uses one 48px
menu row with a consistent 20px icon gutter, a themed outline, and explicit
connection/device, diagnostic/advanced, and application/support groups.
Devices is no longer duplicated through header overflow. Header customization
and user settings remain intact; the corrected menu is the default on existing
instances as well. Corrections uses a tuning/sliders icon, distinct from the
diagnostic focus-frame icon; both are shown alongside their tab labels.

WorkspaceHeader trailing groups now fill their allocated alignment area and
align to the directional end. Badge, action and title rows are vertically
centered; narrow/large-text layouts wrap deliberately. The status detail area
fills its share and end-aligns text, keeping its trailing control at the edge.
Analysis and22 focused tests passed: eight table toolbar, five status bar,
three pixel navigation and six header customization tests. Added exact edge,
center, icon and ordering assertions with no board writes. Only affected
hash/anchor references were rebound; unrelated incomplete donor evidence was
preserved. Tablet deployment/runtime evidence is recorded separately below.

Tablet installation completed through ASUS ADB on SM-X906B serial
R52T60C6V9B, explicitly targeting wireless endpoint192.168.4.2:5555; the
connected phone was untouched. Canonical ApkPreview/BuildOnly build used
BoardProductionHold. Local APK, ASUS staging APK and installed base.apk all
matched SHA256
`368830460dcb587a4c535a62415787fa8f9b53b03df7df9f0ce158129fd451c6`.
Android reported install Success, version0.2.8/build23. Existing preferences
and80 selected cells survived. Actual screenshots inspected:
`outputs/PakarController/tablet-chrome-before.png`,
`tablet-chrome-settled.png`, `tablet-chrome-menu.png` and
`tablet-chrome-tabs.png` in the parent task's outputs directory. They show
trailing header/footer alignment, grouped overflow and both outlined tab icons.
After returning from Corrections without modifying any rules, live accessibility
state confirmed Table,80 selected and Connected to ledcontroller.local.
Bounded Flutter/AndroidRuntime logs showed no error entry; this is not a soak or
blanket fault-free claim. No board command, firmware update or LED mutation was
performed for this UI verification.

## LED fade and production RMT handoff

User explicitly requested smoother GPIO fallback and switching the production
ledcontroller to RMT/DMA. Live ASUS hostname access identified only
esp32s3-907069eaaf34, 16MB flash/8MB PSRAM,480 pixels/GPIO4/BGR, buffered AVR,
BLE disabled and optional Ethernet disabled at runtime. Keycontroller is outside
this action. The application-only rollback was downloaded and fully verified:
2194112 bytes, SHA256
`8bc2a22ceb40c437afb742abe23cb9c2c326f0fbc135c5b5d7199629ec1dc23f`.
Bundled AVR recovery remains32768 bytes, SHA256
`785ff080a43341632143d11348ad1d48e88e6102d6305c86c7112a2f5b1b9b60`;
its availability is preserved, not an authorization to program AVR.

Reread complete esp32WS2811 driver/header, Effect and Colour implementation and
headers. Keep the existing immutable mailbox and native IDF5.5.5 driver rather
than donor per-effect tasks/legacy driver. GPIO now shares latest-complete-frame
adoption after flash holds and measured completion/start-interval telemetry;
the necessary scheduling tick remains because bit-banging masks core1 IRQs.
Repeated identical Fade commands retain their origin/deadline; new targets,
Pulse and Immediate retain their semantics. Interpolation rounds once from
full wall-clock precision in either color direction. Neither change overclocks
WS2811 or claims to remove its8-bit brightness quantization.

Actual worker doubles pass all seven wire/backend profiles, including new
shared cadence/latest-after-hold checks. Renderer, persistence, queue,20000
cross-thread publications, two10000-batch stress runs, pinned IDF full480 ring
and supported/unsupported DMA ownership guards pass. These are host proofs,
not physical FPS. Fixed the test wrapper's case-insensitive Profile/loop-variable
collision. Build gates exposed existing stale Web route/appearance bindings;
only actual rereviewed target bindings are updated, never blanket certification.
Runtime deployment and optical feedback remain separate evidence below.

The final renderer review added an explicit previous-effect byte per cell so
Pulse-to-Fade on the same target starts a new transition; only Fade-to-Fade
retains its deadline. The renderer suite passes this case and clock rollover.
Consolidated buffered/table-selection S3 build passed:2243427 program bytes,
133684 global RAM bytes. Application image2243568 bytes, SHA256
`e70b3984d0fc812a373da1f00b8b87190724c62ac708c39c27a20a58797a769c`.
Only this application image was transferred; AVR was not flashed.

Canonical hostname-first local OTA uploaded once in20937ms. Automatic post-reboot
verification expired while ASUS retained a nonresponsive AP association; the
board's prior station address was also unreachable from that route. Reconnecting
ASUS's existing ledcontroller Wi-Fi profile restored HTTP without another board
upload/reboot. Separate read-only verification reused canonical peer/request and
WebSocket helpers: same board ID, new boot
`a4df4075cb3c9b0c2b246abd80d70c69`, full running-image SHA256 matched, selected and
running slot3407872 both VALID(state2), same-peer WebSocket status passed.
Do not relabel the original updater invocation as successful; recovery/readback
completed independently after its deadline. Station and AP both report connected.

Live backend is rmt-dma,480 pixels/GPIO4/BGR, neutral gains, brightness255,
cache-safe refill IRQ witnessed on core1/level3,11521 of11521 symbols completed,
zero driver failures and zero AVR/ESP UART errors. Last full-frame completion
14972us;250ms start intervals reflect unchanged-white repair frames, not an
animation FPS cap. One explicitly requested GPIO loopback capture compared all
11520 data bits with match=true, timing_errors=0 and capture error0. This is a
digital all-white signal check, not an optical or mixed-color fade acceptance.
All80 idle/white states, physical mapping and empty enabled corrections revision0
survived. User could not visually inspect now; no color-changing test was run.

Read-only follow-up review found no hidden RMT FPS cap. Optional synchronous
cloud connection/read calls can still stall the renderer loop; MQTT/cloud WS
are disabled on this board. That separate scheduling limitation remains open,
not evidence of a regression or reason to claim arbitrary cloud-load smoothness.

Issue #2 checkpoint publication was attempted through the GitHub connector and
rejected403 Resource not accessible by integration. The evidence above remains
local; no published comment, source commit or acceptance closure is claimed.

## Runtime GPIO and RMT/DMA backend checkpoint

User-requested independent inclusion flags replace the old DMA build selector.
Canonical dual-driver builds pass `-S3LedGpio -S3LedRmtDma`; GPIO is the generic
fresh-board default. No flags deliberately creates no matrix pin/task/heap owner.
One shared renderer, correction/mapping path and immutable-frame worker remain.
The worker finishes/latches, releases the previous resources, initializes the
chosen driver, and completes a full resend before declaring the switch applied.
Failed DMA release retains potentially owned resources and fails closed. Safe
RMT initialization failure can fall back to GPIO while reporting the failure.

Board-saved choice is independent of actual driver readiness. Fresh valid
defaults are saved once; corrupt/unmounted storage is never auto-repaired or
formatted. Atomic stage/readback/rename and unchanged-write avoidance adapt the
actual PCController SettingsStore and existing LittleFS ownership. Four existing
donor rows were extended (109 additive lines), including esp32WS2811, Pealayer
PreferencesTab and actual native dropdown controls. No whole-file donor-read
claim or unrelated stale-hash acceptance was made.

Web, Rust and Flutter Output controls report compiled choices, actual/requested
driver, pending handover, error and persistence separately from Static/Effects.
HTTP/WebSocket/MQTT share loop-owned selection; USB adds `0x5c [3,1|2]` and the
general API route. One mutation is followed by bounded read-only observation;
USB never requires Wi-Fi. An accepted receipt is not physical output proof.

Verification: ten actual-worker host variants passed, including dual-driver,
PM-lock, absent-driver, allocation/quiescence faults and pending-until-resend.
Scene/backend persistence faults, corrupt bytes, one-time default initialization,
the renderer pipeline, five native tests, twenty Flutter tests and Web backend/
context/transport tests passed. Build-plan storage tests passed. The broader
protocol run passed changed protocol/LED cases but stopped on an Ethernet
fixture's missing BoardCore include; affected Ethernet/NetworkPath wrappers were
repaired and passed separately. This is not an all-suite or exact-head CI claim.

Built candidates (not deployed):

- ESP32-S3 dual-driver GPIO-default application:2252080 bytes,
  SHA256 `56abd631f2b227c70e10417f35d38044a18f3dcb25dba0bafb7cf96d072e3305`;
  linked code2251927 bytes/67%, static RAM133740 bytes/40%. Profile retains16MB,
  OPI, GPIO4, buffered UART, UART0 host, BLE excluded, verified ISP and optional
  Ethernet; bundled AVR recovery retained. Current production RMT preference
  must be preserved explicitly during first migration before live switching.
- Windows Release:32431616 bytes,
  SHA256 `92a65c7453f58ad8e7d662d6e9ee84e2b86e62c7ec591447ccc87670103b62b1`;
  canonical stage/probe passed, running app unchanged.
- Android APK:62167209 bytes,
  SHA256 `b4e0f9a4df468106c6c9e4b443a2ee8328fab588f21a310c32bc4a6af6773559`;
  build-only production-hold profile, no installation or automatic board job.

All three are0.2.8 local dirty-tree candidates based on main8ce8f29, not a new
published release. Physical both-direction switching, reboot preference and
all480 optical acceptance remain open. Production board/settings were untouched.
Full report generation remains blocked by ten unrelated/concurrent stale refs:
mobile/test/capture_test.dart, desktop/src/ui/cell_pixels.rs,
desktop/src/ui/preferences.rs, tools/ci-checks.mjs, assets/web/device-settings.js,
tools/generate-branding.mjs, desktop/src/config.rs, desktop/src/ui/palette.rs,
tests/web_cell_table_test.mjs and mobile/test/workspace_lifecycle_test.dart.
Their existing anchors remain; owners must rereview their changed bytes.
Issue #2 can be read, but this checkpoint's comment attempt also returned403
Resource not accessible by integration. No GitHub publication or closure claimed.

## Web and native control refinement acceptance

This checklist extends the existing reuse ledger; donor/source/target hashes
remain in `file-comparison-evidence.json`. It is not a second provenance ledger.
The scope is presentation and interaction, not production-board changes.

- [x] Web controls contain their labels at narrow and wide widths, including
  translated/long labels; icon-only controls stay square.
- [x] Static and dynamically updated actions retain meaningful icons and names.
- [x] Web adopts the apps' green accent and explicit semantic success, danger,
  warning and muted roles, with light/dark contrast and stable focus geometry.
- [x] Rust Find icon is centered; controls use available width, helper text is
  muted, real cautions have an icon, and helper space does not jump.
- [x] Rust default fit includes the entire staggered cell table and margins.
- [x] Rust table context menus expose relevant selection, state, management and
  view actions through the shared donor-derived menu implementation.
- [x] Preferences footer stays adjacent to the bounded content area without a
  second unused expansion region below it.
- [x] Exact Pealayer and PCController donor files/adaptations/omissions are
  recorded with source and current target/test hashes.
- [x] Focused regression tests, canonical Web package and native Release build.
- [x] Actual local browser and native framebuffer renders inspected; findings
  corrected. Fixture renders are explicitly not live-device acceptance.
- [ ] Installed-user-app and physical-display acceptance (separate from source
  and local preview; production settings and boards remain untouched here).

Pre-change observation: the existing loopback Web preview at a narrow width
renders Device settings, Preferences, Select all and Manage cells without action
icons, while its selection badge/navigation use blue rather than the app accent.
No production commands were sent while inspecting this visibly simulated peer.

### Exact adaptations and verification boundary

The hash-bound ledger adds seven complete-read donor rows for this pass and
extends the previously reviewed rows. It records 35 new target/proof references;
all current anchors and normalized target hashes were checked. This is scoped
reuse, **not** certification that every feature of either donor is imported.

| Donor source | Pakar adaptation |
| --- | --- |
| Pealayer `src/ui/controls.rs` | Fixed editor/icon slots, contextual availability and dispatch in `desktop/src/ui/cell_table.rs`; full-width Find row and fit geometry in `shell.rs`. |
| Pealayer `src/ui/preferences.rs`, `dialog.rs`, `dropdown.rs` | Bottom-first footer allocation, shared actions and category controls; one bounded body for embedded/native Preferences. |
| Pealayer `web_ui/src/components/PlaybackButton.tsx`, `ControlBar.tsx`, `DesignerFieldLabel.tsx` | Stable icon/label slots, contextual control hierarchy and optical alignment in canonical Web components and styles. Media-specific playback behavior is not imported into board controls. |
| PCController `Tools/Controller/web/src/components.tsx`, `card-layout-controls.tsx` | Icon-bearing actions, square card controls, contextual panel menu and accessible collapsed navigation. |
| PCController `Tools/Controller/web/src/app-preferences-dialog.tsx` and test | Dialog content/action ownership and host-vs-board settings separation; React DOM focus mechanics are not copied into egui. |
| PCController `mobile/lib/unified/relay_style.dart` | Semantic enabled/disabled emphasis, adapted to Web state actions without inferring states from RGB. |

CSS inspection was sectional, not whole-file certification: PCController
`styles.css` lines379–415/625–632 and Pealayer `styles.css` lines77–102/521–552.
Exact source revisions, hashes, exclusions and remaining work are in the existing
JSON ledger, rather than another unbound inventory.

The Web fixes address two actual causes: localization used to replace a complete
button's `textContent` (destroying icons), and a legacy column header combined
with wrapping/100% flex-basis pushed actions into an offscreen column. Stable
SVG/label slots and explicit responsive flex ownership now replace those paths.
Semantic actions use explicit state attributes; arbitrary text matching does not
assign a device state or a red/green intent.

Executed verification:

- 22 focused Web test cases passed across controls, appearance, semantic
  presentation, utilities, zoom, shared app contracts and LED backend controls.
- Native `table_` filter:17 passed; `find_`:3 passed (one overlap);
  `preferences`:15 passed. Includes actual egui geometry at narrow/wide sizes,
  density variations, RTL/LTR and1.5 display scaling, exact helper alignment,
  stable warning slots and all80 transformed staggered cell bounds.
- Browser DOM geometry: no visible-button content overflow at320,442,768 and1280
  CSS-pixel widths. English/Persian and light/dark inspected; SVG icons survive
  localization, collapsed navigation retains accessible labels, and state
  captions honor Colors/States naming while selection remains state-based.
- Full and onboard Web packages contain the same UI, revision
  `6018e436ebd006ae21936484852021bd6569c132209c93ac1a275f614c0f44d9`;
  HTML437822 bytes, gzip124274. No second UI or disabled-feature substitute.
- Native framebuffer evidence captured through the repository harness, with an
  isolated configuration, synthetic loopback peer, no production IPC and no
  board forwarding. The first rendered pass exposed centered helper text and a
  width cap; both were corrected instead of accepting the first screenshot.
- Windows interactive menu verification is still open: the supported desktop
  input tool returned `GetCursorPos failed: Access is denied (0x80070005)`.
  No alternate input-injection mechanism was used. Internal screenshots work;
  context-menu source/availability tests are not a claim of manual click proof.

The whole-repository comparison report remains blocked by unrelated stale
records; `generate-file-comparison.ps1 -Check` first reports
`PCController|mobile/lib/firmware_page.dart -> mobile/test/capture_test.dart`.
Unrelated HeaderBar/App mappings affected by shared-file edits were not silently
re-certified. The seven new comparisons and this pass's references are current;
the global CSV/Markdown reports are not claimed fully regenerated or complete.

Final native Release compiled and canonical stage/probe passed:32680960 bytes,
SHA256 `ff8119940f15f0309ff4f432720d32600ac63516f7bfa34bc0de6233ba17de0b`.
The stable candidate is `../../outputs/PakarController/staging/PakarController.exe`.
No installed production executable was replaced. Current local screenshot proof:
`../../outputs/PakarController/ui-table-final.png`,
`ui-preferences-final.png`, `ui-web-controls-final.png`,
`ui-web-mobile-final.png`, and `ui-web-persian-phone-final.png` in that same output
directory. Final native captures bind the exact candidate hash in each adjacent
fixture receipt. All owned fixture processes exited after capture.

Owning production issue remains open; scoped progress and explicit remaining
gates posted at https://github.com/atomicdeploy/PakarController/issues/2#issuecomment-6401766350.
Use that explicit comment ID for follow-up edits. The CLI's `--edit-last` selected
older comment6259946867; its exact prior body was recovered from GitHub edit
record `UCE_lALOVDBl_c8AAAABdR81c88AAAABNVql9A`, restored and equality-verified.
The current checkpoint was then updated through its exact REST comment ID.

## Native status-bar donor integration

Preflight: read `AGENTS.md` and `docs/REUSE-PREFLIGHT.md`, consult `ui-shell`
and the exact `Pealayer|src/ui/status_bar.rs` donor. Independently read the
entire actual 1737-line source through EOF at
`C:/Users/David/AppData/Local/Programs/Pealayer/source/Pealayer`, revision
`5a6961ce6a477a4726b7baf8a84d12fde1cdf6ad`, SHA256
`e407025a1ee959230634f8350de20d58432313d6dd0f734ada735b8fc9fec4db`.
The complete-read ledger row remains `parity-gap`, not full status-bar parity.

Supporting source reads were deliberately scoped, not promoted to full-file
coverage: `src/config.rs` status widget/configuration/normalization sections
(SHA256 `a318f05844f617ba908aa08c79c35ff91b5d73fdd833c6f5d06c47ea30b17f8e`);
`src/app.rs` `PendingBarSave`, `poll_bar_save`, and status-save request/flush
(SHA256 `41e50c2188ef281b5afcce8a6c5ff46b5bb478ab4ecb65d0d62a4aee0bcadbfe`);
`src/downloads.rs::progress_if_initialized`
(`7db32d2df4f9e5a15a56d523ca131272768c8ce3874a37313f65b1807f0a3fb7`);
and `crates/download-core/src/lib.rs::JobProgress/active_progress`
(`d06c69f6b38ac2f764e7f2489c92fd6c865a232c8f0cbf3d3225a4f9efb41303`).
Those helpers established the no-service-initialization/no-per-frame-probe
boundary; their full download manager or remote configuration runtime was not
copied into the footer.

Actual adaptations:

- Eight board-contextual widgets have fixed-height geometry, independent
  visibility, normalized order and start/end placement. Compact icon controls
  and a keyboard-owned overflow preserve actions at narrow widths; captions,
  counts and progress do not set widths. Borders are constant and inside-only.
- All sixteen old four-boolean preference combinations are explicitly migrated.
  A formerly hidden telemetry group keeps its new Network child hidden; later
  independent child toggles do not re-enable siblings. Unknown/duplicate order
  IDs normalize without dropping valid items.
- Current-route authority gates connection and measured RGB. Cached peer data
  cannot turn a disconnected RGB mirror into live output. Actual emitted RGB
  is not brightness-scaled again; unsupported peripheral values are not invented.
- Firmware progress reuses the existing peer/boot-scoped observation and its
  real counters/optional fraction. Unknown is distinct from zero, and transfer
  percentage does not verify reboot or update completion. Render code has no
  filesystem/network/device/probe calls and passive telemetry returns no Save.
- Context menus support persistent move earlier/later, start/end, hide/restore
  and relevant panel navigation. Persistence occurs after draw, modifies only
  saved `status_items`, and does not serialize the dock or commit another
  Preferences draft. Save errors are reported; the requested in-memory layout
  is retained for retry, without a false Saved claim.
- `PreferencesDraft::sync_status_owner` applies the actual donor's
  `sync_external_appearance` triple-baseline rule. Footer customization survives
  later Preferences Save/Discard while unrelated dirty fields remain dirty.

The first compile exposed an ambiguous float accumulator; source review exposed
a possible old-group index panic after start/end placement. Both were fixed
before candidate acceptance, including an actual egui menu-click regression for
moving in both directions and issuing one Save. The overflow popup also uses an
explicit ID shared by open/close/focus ownership, rather than clearing keyboard
state under an unrelated popup ID.

Scoped proof references bind the nine new status tests plus existing RGB and
serde tests, the Preferences-owner regression, and the shell projection/save
anchors. The initial frozen status target hash is
`05bc4ac2c26be679e931afb179d62044d357cb910ff6a220e5c9cdb4b54b9d02`.
Canonical native tests, build and framebuffer review are owned by the parent
task and are reported separately when they finish. This source receipt does
not claim deployed Windows acceptance, live-board output or human testing.

Preservation hash refresh was limited to reviewed anchors: fifteen existing
shell references, nine main references, nineteen already-reviewed Preferences
references, four localization references and two screenshot-source-guard
references. The source guard's change only tolerates rustfmt whitespace; the
actual screenshot threshold is unchanged. Older unrelated Preferences and Web
review records remain stale, not silently re-certified. No global comparison
report regeneration or unrelated donor-read claim was made.

Still pending from this donor: direct pointer drag-and-drop/drag-out hide,
OSD-message lifetime, fullscreen policy, footer workspace selector, configurable
font/mono/divider/metric-range controls, bounded sensor charts and measured
UI/LED FPS presentation, and complete host/download operation inventory.
Media/timeline/cue-specific behavior remains excluded from the mpv-free
controller, not used as a reason to omit generic monitoring capabilities.

### Native status-bar final build and visual evidence

The source phase above is now followed by actual native verification. All 15
focused `status_bar` tests passed; the isolated screenshot-viewport guard passed
separately. Coverage includes migration, keyboard and pointer actions, overflow,
RTL/scaled/narrow geometry, Preferences draft preservation, no passive saves,
authoritative connection/RGB and measured progress. The screenshot source guard
and scoped `git diff --check` also passed. Existing compiler warnings remain.

Actual intermediate framebuffer review exposed three additional presentation
issues, now fixed and captured again: progress/focus used the theme's dark
selection text instead of its green background accent; the hardware warning
caption was truncated; and a disconnected startup could say Connecting. The
actual dark-palette and disconnected-startup cases have regression tests.

Final status source SHA256 is
`4fa040b2b78c8699a6b2e7eb1ab603804e7441ca9ab2c2aa98321a698e5e1465`;
main source SHA256 is
`e830e4b2dc62e34b638dcda401a5c9692ae65f55e94f3bcb2f1b2b8343b6575b`.
Canonical Release and `stage-desktop.ps1 -Probe` passed. Both built and staged
executables are 32647680 bytes, SHA256
`02505274f29498991b2f9e693274f73e4b4cbe7b44738d1d03e2a547e3e00aab`.
Candidate: `../../outputs/PakarController/staging/PakarController.exe`.
The warm release cache was reused; no obsolete artifact cleanup or dependency
reinstallation was needed. The build contains the preserved concurrent working
tree on base `8ce8f29`, not a claimed new published commit.

Final actual native-renderer captures, all inspected, are under
`../../outputs/PakarController/`:

| Capture | Logical viewport | PNG SHA256 |
| --- | --- | --- |
| statusbar-connected-reviewed.png | 1280 x 800 | 0dd4aba08e677801fab71f295f948e709a305542bdd3d42caee1440244ecd67c |
| statusbar-progress-reviewed.png | 900 x 600 | b41f52419d41b224d66f2732b5c41c2f7ae113957f720ee6f480957f4134bbc6 |
| statusbar-offline-reviewed.png | 900 x 600 | 80cf35dfe70032a8017924c67a5144a2e353eb3bdcc323bc6b85e249e67a9f5c |
| statusbar-warning-reviewed.png | 1280 x 800 | f52ed151b7820aad8c652851b936b4cec5ea80e2e558b5790e97e844bc96bea8 |

Each adjacent fixture directory records its exact executable hash, owned PID,
loopback endpoint and isolated Preferences path. Captures use native rendering
at 125 percent Windows scale, but their peer/progress data is explicitly
simulated. All four owned fixture processes exited after capture. No production
device was contacted, no installed Windows app was replaced, and no Android,
firmware, physical LED or human acceptance is claimed by this UI checkpoint.

Scoped checklist published at
https://github.com/atomicdeploy/PakarController/issues/2#issuecomment-6401982849;
the production issue remains open. Use this exact comment ID for any follow-up
edit, not the CLI's ambiguous last-comment selection.

## 2026-10-27 keycontroller live shell/client recovery

Owning issue: #29; checkpoint branch/PR: `fix/client-controls-presentations` / #27.
Reviewed existing `PanelInventory` (adapted PCController retained peer catalog),
Pealayer `src/platform/interop.rs` named-pipe implementation, the imported
`package-probe-windows.ps1` and guarded desktop replacement, and the PU telemetry
route binding. Pealayer's client also uses a 64 KiB reply buffer; copying that
unchanged cannot accommodate Pakar's expanded device snapshots. The native reply
budget is now bounded at 1 MiB, with a complete error response on overflow.
Replacement uses the hash-verified candidate's reader while retaining the exact
running PID/path/session, old/new hashes, draft and pending-operation checks.

Flutter Output navigation now follows acknowledged LED capability rather than
table geometry. The first-wins Overview shortcut opens the existing game page;
no Table page or command authority is fabricated for a keys board. ESP telemetry
uses its existing compact board serializer inside the aggregate; the full detail
endpoint is unchanged. This avoids increasing its 24 KiB contiguous allocation
on a live board whose largest free internal block was approximately 31 KiB.

Focused verification: Flutter analyze and five panel inventory/widget tests;
four native instance tests including a 180 KiB reply and oversized rejection;
filesystem telemetry and desktop replacement source guards; PU route/hash gates;
canonical S3, APK and native release builds. These are not physical acceptance.

Actual delivery on David-ASUS:

- Keycontroller identity `esp32s3-b81f3fbb0684`: original firmware downloaded and
  SHA256 verified before OTA. Saved six-box game settings matched after the first
  update. Telnet passed two simultaneous sessions, command/property completion,
  history recall, TUI enter/exit and heap reads without output/settings writes.
- Follow-up telemetry fix boot `cf6232cf6458bee3ca872fa8c38c72d3` reported AVR online
  and valid 20468-byte telemetry. Running MD5 `3e914ba8628330c9a0619f168646fcbe`;
  candidate SHA256 `36c7b8808caa49cbadbab6801dbcfe00b26d15b76c60b50d06f8ccc55226f1cd`.
  OTA transfer completed; automatic ASUS roaming to the other AP interrupted the
  updater's verification. Its identity fence correctly stopped without replay.
  Correct-board status/telemetry were then inspected after reassociation.
- Tablet SM-X906B / `R52T60C6V9B`: replacement APK installation succeeded, SHA256
  `094384268de7515c536f18dbaf058a69fc2e8729e8a37e5b16d4bda8b2ce442d`.
- Rust replacement receipt confirmed old PID 27356 -> new PID 22304, preserved
  settings/rollback, no error. New executable SHA256
  `035fda0342f40663686dc2614ce45ce71c4e1a871eba6a7a0d7e45ae3f64c2e0`.

Remaining at this checkpoint: tablet disconnected from ADB after install, then
ASUS's SSH tunnel stopped completing its banner handshake. Final updated-screen
inspection, both-board switching/control acceptance and final full-image readback
are therefore pending. ASUS's saved ledcontroller Wi-Fi profile was temporarily
set to manual to prevent roaming during keycontroller OTA; restore its automatic
mode when ASUS is reachable. No ledcontroller update, reset or actuation occurred.
Do not mark the entire portable-shell/production acceptance checklist complete.

## 2026-10-27 native box selection and reconnect controls

Owning issue #3 / PR #27. Searched Pealayer's custom context-menu call sites;
scoped reread of layout.rs draw_board_card_context_menu, draw_control_context_menu
and control_context_popup, effects_library.rs saved/empty menus, control_bar.rs
shared overflow menus, icons.rs submenu, peer_browser.rs copy menu,
media_inspector.rs copy menu, open_url.rs text-edit menu and dialog.rs numeric
menu. Actual icons::submenu implementation is imported into ui/menu.rs. Box
menus adapt card-owned popup, icon/label groups, muted subtitle, explicit actions
and selected-target preservation using Pakar's already imported dropdown engine.
Selection uses the existing cell_table::Selection rather than a second model.
Per-board workspace snapshots retain selected IDs; click-to-trigger is an explicit
session opt-in, never restored automatically. Existing individual-card grid drag
retains its source/grab/drop behavior; multi-selection is not group movement.

Not copied into box menus: media path/reveal, playback, cue/timeline/keyframe,
numeric step-size and text edit actions have no corresponding box-card model.
Generic toolbar/status/menu and rename/group customization remain useful broader
parity work; this scoped review does not certify every donor menu or unread file.
The inventory search is navigation evidence, not full source review.

Connection lifecycle and status bar share Connected/Connecting/Reconnecting/
Disconnected labels. Enabled but stale known peers display warning-orange
Reconnecting with a cancel action; no deadline inflation, command replay or fake
live status is introduced. Native UI suite: 164 passed, including interaction,
right-click non-actuation, shared range/toggle selection, warning color and cancel.
The initial compile caught egui 0.36 API changes; checked actual event/input and
style APIs, corrected the new tests to ModifiersChanged/style_of and the keyboard
guard to egui_wants_keyboard_input. Canonical cached build then passed.

ASUS localhost:8022 repeatedly timed out during SSH banner exchange. No runtime
logs could be inspected and no remote replacement occurred at this checkpoint;
the reported ASUS application timeout is not yet diagnosed. No board, firmware,
saved scene or production output was changed by this UI work.

Five workspace persistence tests also passed. Release build and isolated 1200x800
framebuffer inspection passed. Windows Computer Use capture failed with
IGraphicsCaptureItemInterop.CreateForMonitor (0x80070057); its accessibility tree
was available, but the one attempted right-click failed GetCursorPos with Access
is denied (0x80070005). No alternate input injection was attempted. The existing
verified-PID capture helper successfully used PrintWindow for fixture PID 15752;
the image was inspected, but it is not an open-popup screenshot. Native egui
event tests remain the context-menu interaction evidence. The accessibility tree
also exposed obsolete game-authority gating on selectable boxes; this was changed
to advertise their selected state independently from capability-gated triggering.
The fixture was stopped, not a production process. Its synthetic device-name 404
is a missing fixture response, not evidence of the ASUS timeout's root cause.

Final box regression repeat: 9 passed after the accessibility correction. Source
checkpoint 33b04470014c5665915e1295c2aa53db6a4f4335 built and staged through the
canonical source-stability and package-probe guards. Candidate SHA256:
1bec4c7c2aa6fe1ab28c7275a4371f443f5589906a5ef7158b385881fbbb289d.
An initial ExpectedCommit invocation used the short hash and was correctly
rejected by the exact-match guard; rerun used the full hash, without changing
the guard. No remote installation is claimed. The direct ASUS saved hostname
also failed DNS resolution; tunnel retry refused before authentication.

PR CI exposed original trailing whitespace in the immutable portable-shell
snapshot from the earlier integration. A path-specific whitespace attribute
preserves donor trailing spaces/EOF lines; all Pakar adaptations retain normal
checks. The 17-file original byte/hash test still passes. This is not an exception
to code checks for the production shell or to the snapshot integrity guard.

## Flutter keycontroller Game workspace, 2026-10-27

Reconsulted device-context and the PCController peripheral-availability donor
mapping, reading the actual TypeScript ready-and-advertised helper and Pakar's
PeerContext, PanelInventory, command/observation pipeline and OutputPanel. Tests
exposed the functional cause: knownFeatures omitted game-press and
game-presentation, although their consumers required those capabilities. Both
advertised capabilities now survive parsing; absent capabilities still grant no
authority. Existing firmware HTTP/WS game-press handlers were inspected without
contacting or changing a board.

The existing boxes, reset, winner and configuration implementation is reused in
a dedicated Game activity rather than duplicated. Output retains LED service and
diagnostics. Overview routing, persisted activity and on-demand pixel subscription
now follow Game. Tap sends the verified press; hold opens explicit actions without
actuation. Pending, disconnected, programming, disabled-box and unavailable AVR
conditions explain why no command was sent. A sheet cannot send after its board
or boot changes. Workspace arrangement remains local and does not trigger keys.

Regression coverage includes real Flutter tap/hold gestures, reset, denied
conditions, duplicate-pending suppression, unconfirmed command reporting,
peer-change fencing, navigation and retained output/arrangement behavior. These
fixtures are not installed-tablet or physical game acceptance. The existing
PU850 .gitattributes target was also reread after the prior immutable-donor
whitespace commit; its stale target hash was refreshed without changing donor
hashes, excluding unread code or bypassing the comparison gate.

Canonical mobile analysis passed. Focused tests passed: output_panel 13,
panel_inventory 5, peer_context 7. The first interaction run caught the missing
capability admission; a later assertion was corrected to compare map values
rather than Dart record/map identity. Final tests exercise the real gesture
callbacks, not direct invocation alone. ASUS localhost:8022 refused its SSH
handshake and local ADB listed no authorized device, so no tablet replacement
or physical remote-button result is claimed.

## Device terminal and contextual Game editor, 2026-10-27

Reconsulted device-shell and actual PU850 LocalLib/Telnet.cpp, portable-shell
src/portable_shell.c, include/portable_shell.h and src/vt100.c. The existing
bounded parser/editor and callback terminal ownership are retained. NAWS and
TTYPE now parse fragmented, escaped, bounded subnegotiation instead of discarding
it; the Node client reports terminal type and resizes. BINARY is negotiated;
unsupported options are refused. No full RFC1143 or CHARSET claim is made.

The existing device-owned TUI gains UTF-8 frame styling, Terminal and role-gated
Game pages, safe viewport edits, preserved resize drafts and explicit enabled-box
press/reset commands. Game settings use the current shared router and flash
persistence, not a second game engine. Terminal preferences are session-only.
Capability/admission loss and undersized windows cannot dispatch actions. Help
and completion hide unrelated role commands. Unicode text entry/RTL, arbitrary
presentation-override editing and complete graphical-client parity remain open;
UTF-8 framing is not evidence of these features.

The first SDK compile exposed C++23 char8_t incompatibility in u8 literals while
C++17 host tests passed. Actual donor VT100 uses char byte callbacks; the repair
keeps that transport and emits explicit UTF-8 bytes. The canonical TUI fixture
now compiles as C++23 to cover this boundary. No compiler flag workaround, cache
clean or donor modification was used. The focused shell tests pass, including
malformed/fragmented negotiation, client resize, authority/lease retirement,
two-session service, game gates and resize drafts; metadata tests cover ordinary,
ESP8266, S3 table and S3 first-wins profiles. Live terminal and physical game
acceptance remain separate. No production board was contacted or changed.

Final canonical S3 builds passed for both roles: table-selection uses 2,308,307
bytes of program storage and 138,724 bytes static RAM; first-wins uses 2,309,583
bytes and 125,604 bytes static RAM. Both use the 3,342,336-byte OTA slot, buffered
UART, 16MB flash/OPI profile, both LED drivers, Ethernet and BLE-disabled build.
Only the table profile carries its previously acknowledged ISP electrical flag;
the generic key candidate does not grant ISP authority. The final key application
file is 2,309,728 bytes, SHA256
`8eeb6551973ec3a13cbc3423f456e372c66e74ed3beec3402a75486200d06370`.
These are compile/package figures, not runtime free heap or target provisioning
proof. Source and comparison checks passed; no CI completion/main merge or live
deployment is claimed by this checkpoint.

## GitHub reconciliation and script regression, 2026-10-27

Fresh inventory found a clean feature branch, no stash, one worktree and one
open PR (#27), not the historical 508-entry backlog. Build 40562666802 failed
because portable-shell used the undefined status `reviewed`. Its thirteen such
manual records now use `reviewed-open`; the four existing adapted records stay
adapted. The generator never invents a review for the omitted CodeQL symlink;
hash-bound evidence is applied to verified pinned blobs as well as local files.
The ledger fixture now supplies its own DRS-jSnake root, avoiding an unavailable
developer drive on CI. No unresolved import is promoted to completion.

Read actual Pealayer `scripts/storage-hygiene.ps1` and its entire safety test.
Pakar's imported `Get-DirectorySummary` still failed for an empty directory under
Windows PowerShell 5.1: Measure-Object can emit no object, not merely a null Sum.
Added a null-measurement guard and an empty-directory regression, verified in
both Windows PowerShell 5.1 and PowerShell 7. Read errors still fail; no real
cache, running app, release artifact or rollback was removed. The existing
source/target/proof ledger was selectively rebound for this reviewed change.

### Repository presentation and CI follow-through

Compared the actual PCController README, repository-health workflow and complete
repository-check/repository-policy scripts; Pealayer README, complete CI/CD and
storage scripts; and PU850's Tool/README.md and complete tooling workflow at
the pinned revisions recorded in the canonical ledger. No donor was modified.

- Imported PCController's actual local-link parsing and Markdown heading/anchor
  code into `tools/check-doc-links.mjs`. Seven maintained entry guides are
  checked offline, with regression fixtures and an explicit coverage list.
  HTML branding links, malformed encoding, repository escape and longer fenced
  examples are handled. Full historical-document/privacy coverage remains open.
- Added a task-oriented documentation hub and README getting-started/download
  guidance. Corrected the old Android PR link, Snake description, two-versus-three
  AVR profile wording and obsolete fixed firmware-artifact count.
- Adapted Pealayer's actual run-artifact inventory into the final CI summary,
  with pagination, bytes/expiry, empty/unavailable distinctions and read-only
  Actions permission. Original failure gates remain mandatory.
- Retained the existing PU-derived dispatcher, safe plan default, locked installs,
  exact hash/backup/no-replay transfer rules and three-OS script matrix. Did not
  copy the donor's forced clean or create redundant .NET/Go/C++ product clients.

The complete local quality suite passed after the CI repair; focused documentation,
governance and both PowerShell storage fixtures passed. Source/fixture evidence
is not a green remote run, complete donor parity, package deployment or board proof.

## Resource reporting checkpoint (2026-10-27)

Consulted updates-telemetry and the exact PU850 build.sh donor mapping. Reviewed
its actual binary-size/flash-percentage/build-info export and the existing adapted
firmware-report.mjs. Retained that report's artifact hashing/unchanged-write
implementation; the new resource-report.mjs uses actual partition slots rather
than the donor's total-flash denominator, and AVR packaging/ELF rather than padded
recovery-image length. Forced cleans, gzip renamed to .bin and timestamp rewrites
remain excluded for the previously documented integrity/storage reasons.

CI now retains JSON resource reports and compares the previous successful main
report where its hardware profile matches. Numeric feature flags/version/FQBN
are captured only by actual successful builds; arbitrary private defines are
never exported. Live collection reuses deviceOrigin/deviceRequest and shared
client headers from check-ethernet-live.mjs without its mutating test flow.
MaintenanceRoutes.h and FilesystemTelemetry.h were inspected for exact fields,
AVR freshness and cached filesystem age. CPU usage and wear remain unmeasured.
Fixtures and existing-artifact parsing passed; live ASUS DNS lookup failed for
ledcontroller.local. No current board usage or production change is claimed.

## 2026-10-27 Web correction Reload focus recovery

Consulted ui-shell and the exact donor-audit EffectGroupDialog.tsx route; reread
the complete unchanged Pealayer source (8c90f7a8...) and existing shared dialog,
correction/group reload owners and their regression tests. The native HTML
dialog adaptation remains the sole dialog owner; no additional framework.

Actual 1280x720 loopback preview exposed approval returning keyboard focus to
the page background: Reload disabled its origin before dialog close could
restore focus. Both editors now await their original peer-pinned read inside
the existing asynchronous dialog action, enabling the trigger before close.
Late dismissed/changed-peer approvals stay inert. Delayed-read regressions for
both editors and existing shared-dialog tests pass. The first group test draft
selected an already overridden group and made no edit; corrected the test to
explicit group3 before testing the pending-read boundary.

Actual correction browser check after rebuild: Reload restores focus; Escape
preserves edits and restores Reload; Cancel is initial focus. Tab visits Reload,
browser chrome, then Close without entering background page controls. Expanded
Wire map, exact480 filter and saved filter across reload work; diagnostic cursor2
produces rule Wire2/Raw2 and a black second slot in its local six-slot diagram.
Test draft was discarded only in the isolated fixture. No board requests,
firmware/app deployment, GPIO/color command or cloud change occurred.

Full/onboard Web assets share revision
fe7608229fd89223b505c64720d187ff1b68081b26228b5bd559c736c75f3b34,
147889 gzip HTML bytes. Only Web assets rebuilt, using unchanged-write skipping.
Screenshot: outputs/PakarController/staging/web-correction-discard-20261027.png
under the task parent; current local preview remains localhost:63200.
The nested map/rules scroll regions remain cramped when both are expanded at
720px height; layout refinement and phone/tablet/RTL acceptance remain open.
This is rendered fixture evidence, not production hardware acceptance.

## Native SSH side-task, 2026-10-27 — issue #33

Consulted device-shell and exact donor-audit PU Shell.cpp routing. Reread the
pinned PU Shell.cpp/Shell.h/Telnet.cpp and the portable-shell command/VT100
sources. SSH reuses DeviceShellSession, inputDeviceShell, the shared TUI,
descriptor setters and owner commands; cryptography is not reimplemented.
LibSSH-ESP32 5.10.0 uses libssh 0.11.5. Its public-domain samplesshd bind/auth/
channel flow and key API were read; demo credentials, password logging, network
setup, SPIFFS formatting and echo loop are deliberately not imported. See SSH.md.

Arduino CLI's proxy HEAD request failed. Its existing exact-download cache was
populated with curl's successful HTTPS GET only after the official registry
SHA-256 matched; the normal pinned CLI install then completed. No proxy/security
configuration was changed. The optional transport uses a worker, bounded queues,
boot-local tokens/security revisions, persistent unique host key and mandatory
SSH login, even when HTTP auth is disabled. USB/Telnet remain separate owners.

Focused policy, actual settings-store-with-NVS-double and real-libssh-public-API
declaration compilation checks pass. Existing actual shell service tests pass
for S3/ESP8266, including the new preauthenticated transport initializer path.
The first SSH-enabled descriptor fixture exposed its old fixed property count;
the fixture now accounts explicitly for the two gated properties and checks
their types/private flags. This is not an encrypted SSH session or target link.

The canonical S3 build Plan first rejected the changed dependency-pin evidence;
only reviewed SSH-affected target identities were rebound. It then stopped at
the concurrently edited LedGrid.cpp donor binding (esp32WS2811 source mapping).
That unrelated work and the shared application/build-profile additions are
preserved, not overwritten or certified by this task. Full enabled/disabled
target builds, OpenSSH login, host-key stability, memory/TLS/LED coexistence and
device deployment remain pending. No production board was flashed or actuated.

## Board-owned key minigames, 2026-10-28 — issue #32

Read the complete ControllerBoardMini/Puzzles/Puzzles.ino donor and adapted its
game phase, countdown and relay predicates into one allocation-free board engine.
Blocking tones, fixed puzzle codes and nested timers were replaced by bounded
ticks, typed configuration and the existing shared HostPresentation renderer.
The reviewed source and exact target/proof hashes are in the Puzzles ledger row.
Flutter reused PCController's actual effect-editor draft/section conventions;
Rust reused Pealayer's dialog/setting/action controls, including narrow-row wrap;
Web reused its EffectGroupDialog behavior within the single canonical entrypoint.
PU tooling/workflow and Pealayer CI sources were read before adding focused game
tests and paired AVR/S3 compilation to existing CI, not another build pipeline.

The first S3 compile found an Arduino bit macro collision and a header included
inside the sketch namespace. Renaming the helper and moving that include restored
target compilation. Final review additionally fixed missing completing notes,
remote-menu events masquerading as physical input, macro/menu output-ownership
bypasses and stale queued presses after reset. AVR now carries an input epoch,
clears prior outputs on a new epoch and preserves idempotent retries. The existing
MacroRing cancellation/stop-request mechanism is reused without deleting saved
recordings. A fixture caught the raw-event versus outer-opcode offset before
deployment; the corrected encoder and nine-key burst/epoch tests pass.

Focused evidence: both full and press-only native core tests, four integration
guards, approved audio/PC-contract suite, Flutter analyzer and 22 tests, six Rust
tests plus Check/Release, and canonical Web command/DOM tests pass. Flutter APK,
Rust EXE and final paired ESP/AVR firmware candidates are built, not installed.
Per-mode physical feedback remains tracked on issue #32. Concurrent SSH work is
preserved independently. No production board or running app was modified.

### Windows selected-device taskbar artwork — 2026-10-28

The current winit Windows set_window_icon path updates ICON_SMALL only; its
separate set_taskbar_icon owns ICON_BIG. Pakar's egui viewport update therefore
changed the caption but left the taskbar on the executable icon. Read the entire
current Pealayer src/platform/window_icons.rs (SHA256
63bbaa244728a2fb4bd53ad5c35f7ff4b95790e70358d47c34ae704ea79d4a4e), its
branding apply path, and the complete taskbar_preview::bitmap dependency function.
Adapted the actual owned alpha HICON pair, window subclass, DPI refresh, disposal
and both hidden-window tests into desktop/src/platform/windows_window_icons.rs.
The existing synchronize_context peer-branding change now calls this root HWND
path as well as retaining the egui/tray consumers. Stable AppUserModelID and
pinned launcher artwork are not rewritten. No board changes are involved.

tools/build-desktop.ps1 -Mode Test -TestFilter windows_window_icons -Jobs 8
passed both real Win32 tests (both slots, DPI sizes, 32-bit bitmaps, replacement,
destruction and borrowed-handle survival). node tests/branding_test.mjs passed.
The initial build gate required rereview of shell.rs: its only change is the new
native icon call; dropdown, table, menu and status functions/anchors are intact.
Their existing exact target hashes and the GDI feature-only Cargo change were
rebound without promoting unrelated donor behavior. The guarded Release build
passed and tools/stage-desktop.ps1 -Probe verified/staged the candidate at the
existing outputs/PakarController/staging/PakarController.exe location, SHA256
1efe869b32ab3f6523f2ca3839723c0187e38448edd63bce6bdc6a7ae3291ff2.
No installed/running app was replaced. Actual Explorer taskbar appearance remains
acceptance after installation; the successful tests inspect the real Win32 slots.

Global comparison-report regeneration currently stops on the concurrently stale
ControllerBoardMini/Puzzles|Puzzles.ino proof tests/key_minigame_integration_test.mjs.
This side fix does not certify or overwrite that unrelated review. Preserve the
ongoing SSH/minigame work, reconcile its proof, then regenerate the shared reports
before publishing. The new native-icon mapping lives in the same evidence ledger.

## 2026-10-28 - Box interaction, menu geometry and connection toasts

Scope is the native box menu/trigger, Arrange boxes control, per-board workspace
preference and connection notifications. Read actual Pealayer 5b3d5c1 control
geometry fix, layout.rs card-owned context-popup/action sections and existing
shared dropdown/dialog helpers; layout.rs remains only a section review, not a
new whole-file parity claim. Retain the inherited stroke width and explicit
Button font; the Arrange toggle now paints a fixed inside border and fixed text
origin. AGENTS.md now requires response/text-origin regression checks across
states, palettes, density and fractional DPI, not static screenshots alone.

Bound the box popup to 220 content pixels instead of inheriting the designer
canvas. Persist Trigger on click per board/application workspace (old snapshots
still default to selection-only). Both painted groups share the card's input
target; blocked explicit clicks now report why nothing was sent. Legacy button
presses use the common acknowledged action owner for USB/WS instead of a separate
post call site. Connection OS toasts now carry the known ESP/AVR status without
copying arbitrary peer responses, URLs or credentials into notifications.

tools/build-desktop.ps1 -Mode Test -TestFilter key_boxes -Jobs 8 passed all 11
tests, including both illumination/indicator click targets, exactly one command,
blocked clicks, bounded popup width at 448/1200/2400-wide viewports, selection and
drag non-actuation. node tests/key_game_web_test.mjs passed the existing Web
main-box/read-only/pending/peer guards. Flutter's shared gameCommand route already
handles tap plus a concrete blocking reason; no mobile source edits were made
by this side fix.

Further native geometry/notification/workspace tests and the final Release
package are not yet verified: a subsequent guarded invocation stopped on the
concurrently edited mobile/lib/table_workspace.dart target hash in the existing
PCController session.go review. The mobile focused test wrapper reached analysis
but stopped on five findings in concurrently edited key_box_designer.dart,
minigame_panel.dart and table_workspace.dart, before running output_panel tests.
Preserve those edits; their owner must finish/review them before rerunning the
guarded checks. These results do not assert installed ASUS/Android or physical
button/LED acceptance. No running app, board, scene or production setting was
changed. The earlier staged executable does not include this later source fix.

## Flutter keycontroller designer, 2026-10-28

Read the complete unchanged PCController ActivityStrip at
`f00ed672c1f72f1a66fb65cff1cfe0a89dea5f7b6222f154db00a7c52c2e1a82`.
Reread Pealayer layout.rs hardware_channel_drag_handle, finish_hardware_channel_card,
hardware_channel_drop_target and effect_library_card_header for fixed action
gutters, grab-offset preservation and color-only hover. This is a scoped read,
not a completed full-file comparison of that large donor. The shared
KeyBoxDesigner uses the existing imported WorkspaceHeader/Badge/menu helpers,
dual observed-RGB circular groups, local multi-selection and bounded group drag.
It is shared by minigames and first-wins rather than replacing either command
owner. Existing positions are retained; key selection joins the same per-board
workspace snapshot. No new board setting or output command is sent on opening,
selecting or arranging.

Phone large-text layout and bottom-sheet checks initially exposed clipping;
bounded Flexible text and a scrollable action sheet resolve those concrete
failures. Analyzer now passes. Four designer tests, eight minigame tests and
thirteen OutputPanel tests pass, including input ACK/release serialization,
duplicate prevention, disabled reasons and stale-peer sheet rejection. The
production-hold APK build passes; SHA256
`e347a2c9f3d2e733d118c349ff48301185f1b363477ae4d0c817928b0b9841cd`.
The existing donor ledger's affected mobile target/proof hashes are refreshed.
Report generation encounters a separate stale Puzzles.ino reference to
tests/key_minigame_integration_test.mjs; its owning review is not certified by
this UI change. The SM-X906B installation is complete and its base.apk hash
matches the candidate. Connected live and Arrange screens were captured and
visually inspected; Arrange opened/closed without changing box positions or
sending game commands. Wireless ADB responded. David's visual acceptance remains
open; synthetic captures are renderer evidence only.

## SSDP board icons and UPnP description, 2026-10-28

Issues #19/#10, parent #2; existing PR #27. Consulted lan-link-discovery,
network-cloud, donor-audit and build-storage. Reread complete PU850 SSDP.cpp/.h,
SSDPDevice.cpp/.h and NetDiscovery.cpp/.h at 431104072952f694a35b3a83c19f13fbb098ca5b.
Retained the bounded delayed/de-duplicated SSDP queue and interface owner.
Adapted its actual streaming XML escaper into one shared description renderer;
did not substitute the donor's experimental NetDiscovery loop. Compared the
description against the UPnP 1.1 architecture and Basic:1 device template.

The existing table/key/generic PNG masters now produce flash-resident binary
HTTP assets, selected by board role with actual dimensions/depth/strong ETag.
Public GET/HEAD /board-icon.png uses the existing ACK-safe static asset service.
Description metadata is escaped and ordered; configId binds identity/artwork;
relative icon/presentation paths replace URLBase so clients retain the LOCATION
interface/port. No phantom SOAP/GENA service or router port mapping is advertised.
The retained ESP8266 description shares rendering; its unfinished release build
is not certified by this pass.

tools/test-upnp.ps1 passes compiled actual-schema fixtures for all three roles,
byte-for-byte PNG checks, XML parsing/order/config identity and relative URLs.
WebAccess public GET/HEAD/denied-write fixtures and existing async HTTP short-write,
HEAD/304/ETag/backpressure tests pass. Ethernet discovery integration and script
tooling tests pass. CI quality now invokes the combined UPnP regression wrapper.
SDK build initially caught Arduino's relocated-source include lookup; project-root
includes correct that integration defect. Final SDK result is recorded on GitHub.

A read-only gateway SSDP/IGD probe found no gateway IGD response; unrelated media
renderer announcements were not treated as router acceptance. No router mutation,
port mapping, board setting, firmware deployment or LED/audio command occurred.
ASUS SSH refused access; actual board icon fetch/rendering, wire discovery and
lifecycle/router interoperability remain open. This is not UPnP certification.
The pre-existing CI failure at pc_contract_mapping_test (AVR target hash) is
separate from this source checkpoint. Concurrent SSH/native/mobile work is left
unstaged rather than folded into this change.

## Native SSH and bounded SCP continuation, 2026-10-28

Issue #33 and existing PR #27. Consulted device-shell, terminal-ui, network-cloud,
donor-audit and build-storage. The initially attempted ssh-transport lookup had
no route; added an explicit issue #33 route to the existing complete-read PU850
Shell.cpp/.h records rather than bypassing consultation. Reread both complete
actual files at 431104072952f694a35b3a83c19f13fbb098ca5b. Existing callback/session
separation is reused by DeviceShellSession; crypto never executes game commands.
Read the installed pinned LibSSH-ESP32 5.10.0 README and complete public-domain
samplesshd-kbdint example; inspected actual bind/options/channel close behavior.
External dependency/source/archive identity and license are recorded in SSH.md.
No demonstration passwords, credential logging, network setup or formatting are
imported. The broader library is LGPL, not relicensed by the sample's exception.

Finished the inherited opt-in transport, persistent enable/port settings,
per-board ECDSA host key/fingerprint, mandatory SSH password authentication,
security-revision/token-fenced worker queues, shared loop-owned shell/TUI,
bounded PTY resize and actual-only DNS-SD advertisement. Added separately gated
SCP download for exact /user/ names through UserFileStore metadata/hash/chunk
reads; no raw filesystem access or new plaintext FTP listener. Private settings,
firmware partitions, uploads, SFTP, recursive copies and arbitrary exec remain
unavailable/explicitly reported. Upload integrity/replacement and bidirectional
SFTP are useful pending work, not fulfilled by the library's SCP client example.

Found that pinned ssh_bind_set_blocking changes a struct flag, not its listen
descriptor: explicit F_GETFL/F_SETFL/O_NONBLOCK prevents a select/accept race
from stranding revocation. Fatal select errors retire the worker. Final exit
status/close are drained with bounded socket progress before freeing the channel.

Focused SSH policy/NVS tests, real-library API compilation with SCP included and
excluded, SCP byte/backpressure/parser tests and actual SCP/public-store binding
tests pass. The actual OpenSSH scp -O client accepted binary and zero-byte files
over a stdio protocol fixture; Windows text-mode CRLF initially broke filename
validation, fixed in the fixture by binary stdin/stdout (network source already
emits correct LF/raw bytes). This is client compatibility, not an encrypted ESP
session. Shell/TUI/property/discovery/lease/audio-dispatch regressions and script
syntax/tooling tests pass. CI installs exact pinned headers, runs SSH tests and
builds opt-in SSH/SCP before restoring its ordinary firmware artifact. An old
ArduinoJson-specific workflow assertion was updated to validate the generalized
exact-name/version install loop; no guard is disabled.

Final ESP32 3.3.12 SDK build with SSH+SCP passes: program 2,657,491/3,342,336
bytes; static RAM 129,204/327,680. Candidate SHA256
ea17dba7e1c44afb157d48ee763df8fb76f8804dd0b03d0628fff679960126b7.
The explicit enabled profile was rebuilt after the final socket-close fix.
These are compile/static figures, not live crypto/heap or a clean signed release.
Feature-excluded policy/shell fixtures pass; a final excluded SDK rebuild is not
claimed by this continuation. Runtime listener remains disabled until configured.
ASUS localhost:8022
refused repeated read-only SSH probes. No production installation, reboot,
configuration or actuation occurred. Real OpenSSH login, wrong-password handling,
host-key persistence/reconnect, terminal resize and simultaneous API/UART/LED/TLS
load remain live acceptance gates. The upstream hardware-SHA concurrency warning
is retained; the SDK was not secretly reconfigured. The existing unrelated AVR
contract-mapping CI failure remains outside this checkpoint.

## 2026-10-28 ESP peer-relay resolution and bounded-memory checkpoint

User incident: ledcontroller could not resolve keycontroller.local (7.32 s),
opposite-direction status reads alternated between ~1.3 s success and failures.
The historical internal minimum of 216 bytes is a warning, not a diagnosed cause.
Owning acceptance: issues #7 and #2. Game-mode polish remains a separate queued
request; no completion of that request is claimed here.

Read AGENTS/preflight, bridge-repeater and network-cloud rows plus exact donor
audit for PCController Tools/Controller/internal/lanresolver/resolver.go. Reread
that complete actual resolver, resolver_test.go, hostbridge/peer_response.go and
peer_response_test.go through EOF. Adapt reconnect-sticky successful addresses,
original Host identity and strict malformed/uncertain command receipts. Retain
board/boot/compatibility checks, one hop, restricted reads, command expiry and
no mutation replay. No borrowed discovery record grants ownership.

Source changes: bounded 900ms mDNS for .local (not router DNS), asynchronous
TCPIP IPv4 DNS with safe object-owned late callbacks, four verified cached
addresses and four explicitly saved routes. Saved hints are reverified after
boot, not accepted as authority. Loop-owned staged/readback/rename saves are
byte-idempotent; observations do not write flash. HTTP and fenced USB share
the directory/route operations. Same-boot cached GET uses one HTTP request;
commands still preflight and never retry after dispatch. One lazily allocated
worker retains the existing 10240-byte stack instead of per-request allocation;
eight SHA256 fingerprints replace retained raw request bodies. Filtered JSON,
moved reply storage and mutex-scoped linked JsonString receipt serialization
avoid unnecessary full-status copies. Admission checks current internal free
memory and largest block, not PSRAM totals or the historical minimum.

SDK source review found HTTPClient3.3.12 ignores addHeader(Host/User-Agent).
PinnedClient now overrides only numeric TCP dialing while HTTPClient retains
the logical hostname; dedicated setUserAgent sets product identity. The fixture
implements the actual ignored-header behavior and asserts both dial and Host/UA.
SDK NetworkManager.cpp, NetworkClient.h, HTTPClient.cpp and ESPmDNS.cpp were read
at the affected functions; no SDK-wide patch or build-flag churn was introduced.

Actual production PeerRelay/PeerRouteStore/PeerLanResolver host fixtures pass:
failed-mDNS cached reads, cold saved-route identity verification, changed DHCP
address, untrusted hints/private ranges/self rejection, stale boot/identity,
auth and HTTP timeout without retargeting, lost/malformed command receipts and
no replay, receipt/heap limits, 18KB escaped response preservation, late DNS
callbacks and interrupted route publication. Read-only probe input/timing
fixtures, host-presentation scheduling, BLE/offline guards, peer fencing and
script/living-contract tests pass. The initial -Werror misleading-indentation
failure was corrected; the first SDK attempt correctly stopped on stale PU
route target hashes. Exact reviewed source/line bindings were refreshed, not
the guard bypassed. The focused runner is registered in CI quality.

ASUS localhost:8022 refused two read-only SSH probes during this turn. No board
settings, scene, corrections, cloud state, LEDs, audio or firmware changed.
Deployment requires the applicable explicit PBX DTMF authorization. Automatic
peer discovery, continuous peer event subscriptions and fully selectable relayed
Flutter/Rust/Web devices remain OPEN and continue to advertise false. Real
bidirectional board latency, current-memory/stack measurements and long soak
remain unverified; fixtures are not hardware acceptance.

Final ordinary (SSH/SCP excluded) ESP32-S3 SDK build passed twice, including a
last-source rebuild after the Host/UA correction: program 2,339,691/3,342,336
bytes, static RAM 139,380/327,680. Raw candidate 2,339,840 bytes, SHA256
ed86a0c1c01ff2a16f39a0afcd4b414a30db0a63dab050d3d8a1ce8ad0f3f703,
at build/esp32s3-buffered/esp32s3.ino.bin. Table role, buffered UART, 16MB/OPI,
both LED drivers with RMT default, acknowledged ISP, status RGB48, optional
Ethernet and BLE excluded retain this existing candidate profile. No claim of
matching the unreachable keycontroller's current hardware profile is made.
Actual async HTTP actor/short-write/final-ACK and USB request/diagnostic receipt
fixtures also passed. Existing AVR source-mapping CI mismatch remains unrelated
and unresolved; no all-green CI or installed runtime claim.

### Relay edge-case follow-up

Source follow-up adds a different caller-observed IP as an identity-probe
candidate after cached connect failure plus failed hostname lookup, never as
POST retry authority. Preallocates form bytes once. The injected stack-allocation
fixture proves a 31732-byte largest block becoming 21492 does not permanently
lock out subsequent status reads; actual response allocation remains checked.
SDK WString.cpp reread found empty assignment retains capacity, including empty
move assignment. Explicit null assignment invokes its invalidate/free path for
retired responses and completed command packets; queued response stays a JSON
string, not null. Fixtures exercise that actual release branch.

Final source SDK build passed: program 2,339,923/3,342,336 bytes, static RAM
139,380/327,680. Raw candidate 2,340,064 bytes; SHA256
f18721c3387951fdce2738e1fa166031c3cead60c868ecc5b79298c4b924107c.
This supersedes the pre-edge-case ed86a0c1 candidate above, in the same canonical
build path/profile. Actual relay fixtures and source/script/fence guards pass.
Run 40714319535 confirms the existing AVR mapping mismatch is still the CI
failure; no check bypass or broad unrelated code review is claimed. David said
he will restore ASUS access after additional requests: no repeated access prompt,
deployment, board actuation or new network configuration was attempted.

## 2026-10-28 — configurable clock and advanced network observations (#7, #27)

Scoped donor reread: PU850 sketch dormant NTPClient `/ntp/get` implementation
1148–1180, plus complete Public.cpp, RemoteCommand.cpp and Shell.h. Retain the
configuration/scan/command ownership intent; do not copy commented blocking
`forceUpdate`, mutable calendar globals, UDP placeholders or blanket success.
Actual pinned Arduino-ESP32 3.3.12 esp32-hal-time.c, ESP SNTP API, TLS client and
ping_sock.h were consulted. New network I/O uses those real SDK implementations,
the existing bounded resolver and one lazy worker, not a donor stub.

NTP settings/DHCP fallback and HTTPS Date/public-IP lookup configuration share
staged typed persistence and the existing local/USB/cloud command owner. UTC,
effective NTP servers, successful observations, age and last diagnostic errors
remain separate from saved settings. NTP is explicitly unauthenticated; HTTPS
never uses an insecure clock-bootstrap workaround. DNS and finite ICMP are real
SDK-backed jobs. Traceroute and IPv6 public-IP are explicitly unsupported.
Network scans no longer block the HTTP/game owner; Web/Flutter poll results
without resubmitting scans. No arbitrary acquired values can be imported as
measured truth. Configuration export/import includes network_tools.

Flutter now has an actual Advanced destination with separate board clock/tools,
Wi-Fi/Ethernet, board remote connections and app bridge pages. It reuses the
existing Workspace widgets and boot-fenced WS-first/USB API owners. Save sends
only changed fields, verifies readback and preserves drafts during event refresh.
Human-readable clock properties are included in the existing terminal/TUI.

Verification: typed date/weekday/public-IPv4/hostname policy; actual settings
store readback/unchanged suppression and write/rename failure preservation;
cloud/network maintenance exclusion; USB API bounds/replay; terminal metadata,
TUI/navigation, session lifecycle, tone routing and source peer/stack guards;
canonical Web script/contracts and focused Flutter tests/analyzer passed.
ESP32-S3 SDK build passed: 2,387,023/3,342,336 program bytes; static RAM
140,156/327,680. Candidate is 2,387,168 bytes, SHA256
ed8da2de86931261bfe9de4e922001513bf3155cf4b4bad4c23f05cd9e4a6673.
Android production-hold candidate built, 64,385,613 bytes, SHA256
f0a8bc68737a3705e37933bef13e003c68b3f26b636e57157322244d6cb511a3.
APK came from the preserved mixed workspace, including pre-existing signal
indicator edits; this scoped checkpoint does not claim ownership of those edits.

Rust compatibility follow-up consumes asynchronous Wi-Fi scan receipts without
clearing the previous SSID list or blocking the command worker. The existing
peer-stamped Configuration event path schedules one result-only read at a time,
500 ms apart, with a 15-second UI deadline. The focused native fixture passed for
pending, completed/deduplicated results, single-read scheduling and timeout/no
replay. This compiles the native test target, not a deployed Windows app.

No production board, Wi-Fi configuration or cloud-enabled policy was changed.
No live NTP/DHCP/Internet, SSID/RF, MQTT/remote WebSocket, USB device or tablet
acceptance is claimed. ASUS access remains deferred by David; do not repeat
access prompts or route around that boundary. CI's pre-existing AVR source
mapping certificate failure is not resolved by these targeted checks.

## 2026-10-28 — gated SCP/SFTP/FTP uploads (#33, #28, #27)

Reconsulted ssh-transport, donor-audit (PU Shell.cpp) and user-files; read actual
PU850 Shell.cpp/.h and Pealayer ServerFilePicker.tsx. Preserve the PU session /
loop-owner callback boundary and the managed store's immutable-blob/index commit,
not direct LittleFS access or a second settings/OTA writer. The pinned LibSSH
port has no SFTP server implementation; the bounded subsystem implements the
IETF SFTP-v3 wire subset itself on that existing authenticated SSH channel.
FTP is passive-only and requires credentials; it is not TLS/FTPS. Both services
are excluded from ordinary builds and require a persistent runtime opt-in when
included. Existing HTTP/USB declared-hash uploads retain their stricter contract.

Source/fixture evidence: tools/test-ssh.ps1 and tools/test-user-files.ps1 pass;
real OpenSSH SCP clients upload binary/empty files and SFTP's default 32 KiB
buffers complete a 65 KiB put/list/get byte roundtrip over stdio. Actual FTP
binding doubles cover partial socket writes, same-IP data admission, binary
STOR/RETR, explicit abort, connection reset, revocation, idle/passive timeouts,
durable opt-in and deferred flash-fence cleanup. The existing actual shell
service fixture passes for ESP32 and ESP8266. These are not ESP network tests.

Two implementation failures were addressed, not bypassed: OpenSSH creation
permissions differ from the fixed stored permissions (apply a safe creation
policy, reject unsupported later chmod); including protocol headers inside the
sketch owner created a shadow PakarS3 namespace (move them to ShellBuild's actual
global include layer). All three canonical SDK builds then passed: ordinary
2,387,744-byte image / 140,156-byte static RAM; FTP-only 2,401,440 / 144,748;
SSH/SCP/SFTP/FTP 2,694,000 / 147,556. Full inclusion fits 80.60% of the OTA slot.
SSH.md binds the exact profiles and three raw-image hashes. The ordinary,
services-excluded artifact is restored in the same canonical cache; these are
build observations, not runtime heap or live transfer acceptance. The failed
namespace build is not accepted as a candidate. Resolving different dependency /
include lists invalidated vendor objects; no clean or extra SDK cache was used.

No board flash/reboot, output, Wi-Fi/cloud policy, tablet or Windows deployment
is authorized or performed by this upload task. Encrypted ESP login/transfers,
worst-case live heap, persistent identity and TLS/LED coexistence remain open.
ASUS access is still deferred; no repeated access prompts or policy workarounds.

## 2026-10-28 - GitHub handoff reconciliation (#27, #26, #11)

User explicitly requested all remaining local work reviewed, published and merged.
Inventory found one feature branch, no stash/second worktree, and pending native
taskbar/notification, mobile signal-widget and related provenance changes. These
were preserved and reviewed, including the complete Pealayer window_icons.rs,
taskbar_preview bitmap dependency, and PCController workspace_chrome.dart.
Both real Win32 icon fixtures and the rest of the 495 native tests pass. Flutter
analysis passes; its four signal-widget tests initially exposed a 13-pixel menu
overflow. Flexible labels fix that overflow, and the rerun passes all four cases.
No installed app or production board was changed by this reconciliation.

Fresh complete Pakar conversation enumeration found PR27 alone open, with three
unresolved CodeQL threads. Read its body, every comment, review and thread,
including outdated state. Two guide-checker path check/reopen findings are fixed
using the actual descriptor ownership pattern from the fully reread PCController
repository-policy.mjs; guide, linked-file and anchor bytes share their opened
file, with finally-close and per-invocation snapshot reuse. Missing, empty,
directory and malformed references still fail. The script-body syntax fixture
now accepts upper-case SCRIPT tags, explicitly not an HTML sanitization API.

Current-head CI40740222299 also failed the old AVR source certificate. Reread the
complete current runtime header/implementation, donor MacroQueue.h/.cpp and
FrontPanelModel.h, plus the epoch/macro/input/companion-loss tests. Rebind the four
affected source-mapping records and actual anchors; do not modify firmware to
satisfy stale certificates. All23 mappings and the full audio/PC contract suite
pass. The quality run next exposed an obsolete update-presenter assertion; it
now asserts both game-countdown coexistence AND update priority. The actual
update lifecycle/OTA/AVR presenter fixtures pass. Only these reviewed test hashes
are rebound in the existing donor ledger. No blanket parity completion claimed.

Global reports are regenerated from the combined reviewed workspace, retaining
the previously unpublished owner records: 430 comparisons,4302 pending,310 gaps.
Complete remaining quality/CI and final PR diff review are pending at this
checkpoint, not waived by the focused successes. Physical/device acceptance,
unfinished ESP8266 support and the glib dependency issue remain separate.

The next complete-quality pass exposed three older fixture assumptions, not
firmware regressions: DNS recovery is now preceded by safe first-boot TCP/IP
initialization; table extraction must find its end marker after its own start,
not the earlier minigame output editor; static-group wording no longer assumes
six LEDs when custom group sizes exist. Assertions now inspect those exact
owners and retain their negative/readback checks. All three focused suites
pass. CodeQL then requested forgiving closing-tag extraction; the minigame
syntax fixture now uses the already-reviewed web_app_contract_test extraction,
including upper-case and ignored closing attributes. No sanitizer is exposed.
The complete final quality rerun remains separate, not implied by these tests.

That full quality rerun subsequently passed, as did all495 native tests and the
complete869-test Flutter suite (three explicit skips). Both analyzer and the
corrected signal-popup tests pass. The updated CodeQL workflow/standalone check
passes; its fourth forgiving-end-tag finding also auto-resolved after89b4a6f.
All thread pages, including the new/outdated finding and replies, were read.

Final source review found a real minigame configuration gap: next.save entered
LittleFS without the common ScopedLedFlashPause. It now acquires that same LED
flash owner before writing and returns led_flash_guard_busy without replacing
live settings/game state on refusal. Existing Puzzles.ino configuration mapping
is retained, not an added writer. The all-mode/press-only core and four actual
integration fixtures pass; no physical LED improvement is inferred.

CI40743454384 then failed before SDK compilation: ethernet_management_test.cpp
included the shared CloudManagement::busy owner without its newly added
artifactReaders observation. Read the actual shared header, the complete
Ethernet fixture and existing cloud-management fixture. Add the same typed
counter and a refusal/preserved-bytes test while a reader lease is held. Both
Ethernet feature0/feature1 fixtures pass. Add the existing canonical wrapper to
early quality so this mismatch fails before SDK/native/APK jobs. An initial new
test compared against an older DHCP snapshot; it was corrected to capture the
immediate pre-lease bytes, not weaken the refusal assertion. Guarded S3 build
Plan,23 source mappings,50 route bindings and provenance generation pass.

No user scene, correction values, board firmware, USB/UART, network, installed
apps or credentials were changed. Source handoff is separate from deployment.

The next exact-head CI40750230610 passed Windows (495 tests/package), Android,
both S3 keycontroller configurations and the RF AVR build, then found a real
table-role AVR compile error: boardKeyGesture referenced first-wins-only
espGameOwned outside its feature guard. Read the complete current AVR sketch,
the Puzzles/Timer Keys.h/.cpp and all three donors' Tasks.h/.cpp; retain their
shared callback/scheduler contracts. Guard only the ESP-owned key-game branch.
A fixture compiles the actual extracted callback with both role flags using
the real KeyEvent header, and checks Down/Up, ninth input, menu/capture gating
and exclusive game ownership. It first reproduced the missing symbol, then
passed after the guard. No timeout, debounce, input order or task behavior changes.
The first guarded SDK attempt correctly refused the stale AVR target hash;
the same reviewed file bindings are refreshed before rerunning, not bypassed.

## Concurrent shell pin and handoff (2026-10-29)

PR36 changed the canonical component pin and integration README, but comparison
generation refused its old README target bindings. Consulted portable-shell
again, read the complete current integration guide and changed canonical code,
and reviewed all nine affected source dispositions (including the actual donor
ignore policy). The21-file source gate verifies the retained historical snapshot
and four current components; no historical source/omission was reclassified.
The guide still distinguishes inert reference code, actual application owners,
source fixtures and pending hardware acceptance. Refresh only these README
target bindings, retain their existing anchors/dispositions, regenerate/check the
same reports and run the actual shell suite on the TLS-consolidated candidate.
No separate implementation, device mutation or guard bypass is introduced.

Final Git HTTPS refresh timed out before fetching the last merge. Inspected the
coordination procedure and verified GitHub's current main SHA; a bounded narrow
HTTP/1.1 fetch then succeeded without disabling TLS checks. Fast-forwarded local
main and verified its entire tree equals the accepted PR head before cleanup.
Never treated the failed fetch's stale origin/main ref as the final handoff.

## Keycontroller boot and PuzzleBoard timer presentation (2026-10-29)

Consulted firmware-hardware, games-streaming, donor-audit/PuzzleBoard.ino and
audio-attention. Read PuzzleBoard's MODE_RESET waitCounter, countdown/colon,
beepOff and multiplex helpers, and the actual PCController LocalLib/TonePlayer
header/implementation. The donor's exact bottom-first 12-step/100 ms animation
now has a TM1637 active-high adapter in BoardCore/GamePanelFeedback.h. Retain
A4/DIO and A5/CLK, bounded display admission and notice/menu/update ownership;
do not copy the incompatible active-low shift-register pin map or blocking beeps.

The same existing audio ring gains idle-only atomic admission (action8, explicit
HELLO audio bit2); it rejects timer ticks during notes, flash cues or prefill.
No extra AVR queue/RAM allocation, audio-rate interrupt, legacy stop-and-beep
fallback or missed-tick replay. Silent mode remains authoritative. Persistent
boot policy defaults to first available/Press to win, retains saved wiring and
uses one fresh-round admission only after AVR ownership is confirmed. An
interrupted link does not trigger another boot round.

Scope-reviewed diffs in the existing shared files leave their unrelated donor
routes/anchors unchanged; rebind only these changed target/proof bytes in the
existing ledger. Add exact helper/proof references to the PuzzleBoard mapping,
without promoting physical or remaining donor acceptance. The first Git refresh
timed out; candidate work starts from the verified prior clean a66d3a8 checkpoint,
not an assumption that a failed fetch refreshed origin/main. No production
board, GPIO, ASUS, tablet or PBX action is part of this source task.

Candidate checks caught the new native control in the read-only status scope;
moved it into the existing per-peer settings Draft and retained the imported
Pealayer ComboBox/DropdownUiExt geometry. Removed only this pass's unrelated
whole-crate rustfmt changes and restored their original evidence bindings.
The Web generation gate correctly rejected an edited packaging header; moved
the diff to canonical web/index.html and regenerated all assets through
build-web.mjs. Browser dependency lookup reused the bundled Playwright via
load_workspace_dependencies/NODE_PATH, without installing another package or
contacting a board. The first DOM run lacked NODE_PATH; the corrected isolated
browser fixture passes. A later narrow HTTPS fetch succeeded and verified
origin/main remains a66d3a8. No failed check was disabled or bypassed.

## Tablet game picker and paired-delivery continuation (2026-10-29)

Reread PCController's complete ActivityStrip in workspace_chrome.dart and the
existing mapped game-card implementation. Retain its semantic icon surface,
bounded text and stable target geometry; add catalog-relative numbering,
wrapping muted descriptions and one contextual Start/Pause/Resume action.
Rebind only the changed reviewed target/proof files and retain the donor's
unrelated open omissions. Analyzer and 14 focused widget/model tests pass.

Forwarded ADB intermittently lost the tablet transport; ASUS-local ADB completed
successive commands. Reuse the actual android-device.ps1, viewport and process
probe helpers on ASUS, retaining explicit serial, lock guards and fresh UI
observations. Do not restart the shared ADB server, touch the attached phone,
or call this an established electrical/cable diagnosis. The full installed APK
hash and actual tablet menu were verified. Windows process replacement found
no running owner; preserve old executable/preferences before the guarded
existing interactive production task launch. Session-zero IPC failure remains
separate from process/hash evidence, not routed around as a forced replacement.

The standard Range backup reached an owned partial then timed out; reuse the
already implemented Flutter espBytes/AndroidDocuments path rather than bypassing
identity/integrity rules or claiming the partial succeeded. Actual tablet export
matched the running image's complete size/MD5 and was copied off-device.
The tablet selected the candidate binary through SAF, confirmed it and performed
the ESP upload; captured byte progress and observed board OTA job completion
are not independent post-boot hash or physical acceptance. See KEY-MINIGAMES.md
for precise hashes and the still-open paired-deployment gates.

### Maintenance progress and exact backup ownership (2026-10-29)

Reused the reviewed PCController hostui/update_progress.go stage-local measured
versus unknown progress rule, retaining the imported themed Flutter controls.
The first accessibility run correctly rejected a prose-valued progress semantic;
the control now uses Flutter's numeric percentage semantics. Five responsive,
RTL, theme and large-text design tests, eight delivery tests and four upgrade
lifecycle tests pass. The new Urboot export fixtures initially omitted mandatory
board identity; corrected fixtures rather than weakening the production identity
guard. All 30 firmware-controller tests and analyzer pass. Exact backup downloads
are boot/job/request fenced before and after transfer, including exact byte
length, and never fall back from Urboot to unrelated ISP backups.

The S3 build's PU route guard detected stale application target bindings after
the narrow audio-owner edit. Reread the actual donor registrations and current
five mapped application regions; repair only target hashes and line references,
not donor identities or route dispositions. The 50-route guard and canonical
S3 build pass. Evidence hashes retain the separation between origin source rows
and target/proof references; comparison generation and Check pass. No failed
gate was disabled, and no failed fixture justified a production API downgrade.

### Game-feedback requirement audit (2026-10-29)

Audited the user's entire game feedback list against actual engine, service,
AVR player, Flutter controls and command transport, plus Rust and canonical Web
controls. Retain the existing semantic icon/description menus and correlated
WS-first transport; do not replace connection checks with optimistic success.
New targeted Flutter tests verify board-provided timing/reveal settings, narrow
patches and failed-save draft retention. FormatException presentation now strips
only the type prefix, keeping its actionable reason. The 18 game and six actual
transport tests pass with clean analyzer results; no tablet install is part of
this audit because David is using it and requested a later deployment retry.

The actual production audio blocks exposed another phase-only assumption:
repeated wrong-pair Feedback entries can be separated by input processing without
an intervening audio service pass. Bind feedback to presentation timestamp as
well as phase, retaining the mode/input-epoch fence. Compile the real dispatch
block and assert both repeated incorrect cues, cancellation by a new correct
pair, the exact atomic arpeggio and delayed configured winner cue. Core/settings,
all-mode/excluded-mode, seven integration and boot/reset tests pass.

Latest explicit policy overrides the older Color Pattern request: its fresh
introduction default is Off; optional reveal remains capped at 500 ms, and
Matching Pairs keeps its default-on reveal. Existing saved choices are not
migrated. This instance's requested Off write remains pending ASUS access.

Reread the unchanged complete Pealayer NumericValueControl and EffectGroupDialog
at their recorded hashes, preserving bounded typed drafts and explicit commit.
The existing Web form exposed boot/display/timer fields that its command
validator rejected. Align its typed field catalogue with those actual firmware
settings; invalid booleans/policies and unknown fields still reject before any
send. Transport and isolated browser interaction tests pass. Canonical Web
regeneration changed the embedded route target hash; the mandatory PU route
gate stopped the build. Re-review the affected donor/target registrations and
update only two target bindings, then pass the 50-route guard without bypass.

### Shorter Simon progression (2026-10-29)

The latest user request supersedes the prior one-round/eight-step default. Use
three rounds with the same prefix extended from three to five to seven steps;
300 ms lit plus 120 ms gap accommodates the existing 210 ms AVR-local arpeggio.
Keep correct-entry feedback at 240 ms. An additive bounded `sequence_growth`
setting (0–31, zero for fixed length) is advertised/persisted by the board and
editable in Flutter, Rust and the canonical Web controls; maximum sequence
length remains 32. The old-file missing-field default retains +1 growth in RAM
without automatically rewriting explicit saved timing or configuration.

Core tests play each complete prefix and response with exact feedback timing,
then fixed-length/capped variants. Persistence tests cover explicit growth and
unchanged older saved policies. Flutter's 19 game tests and six previously
verified unchanged transport tests, plus ten native minigame tests, pass. Web
typed-command and isolated DOM tests exercise growth bounds, contextual control
visibility and actual Save. Full/onboard assets share the same revision.

The native specialist reread all 1236 actual Pealayer dialog lines and all 646
dropdown lines after inspecting the used helper slices before edits; retain
their imported setting-row/navigation controls without a new widget or geometry.
Only explicit picker interaction applies new mode defaults. Painting/reopening
preserves edits, wiring, startup and timer policies; an unadvertised optional
field is not inserted for another board. Reuse/source proofs remain separate
from installed UI, audible timing and actual three-round hardware acceptance.
No device operation is authorized by these fixture results: deployment and the
current-instance Color Pattern/Simon settings are intentionally deferred while
David uses the tablet.

## Color Pattern pacing and visible matching (2026-10-29)

Consulted games-streaming and the exact Puzzles.ino donor-audit route. The
complete unchanged Puzzles sketch and its WS2811 header/implementation were
reread, alongside the actual PCController TonePlayer deadline owner and
Pealayer SoundEffectFields. Reuse bounded phase-local progression, finite
color-note ownership and the tone player's previous-deadline anchoring rather
than donor blocking welcome waits or an additional presentation engine.

The current Color Pattern intro used one saved 1000 ms interval per box and
reanchored each reveal to the current service tick, accumulating stalls. Cap
its effective interval/final pause at 500 ms, default new color-puzzle settings
to 500 ms and catch up over at most nine expired reveals without stale-note
bursts. Matching Pairs keeps its own reveal/preview policy. No board settings
or saved instance files are migrated by this source change.

The old default palette used three blue-family hues. Replace the default seven
with saturated Red/Green/Blue/Yellow/Magenta/Orange/White; optional extra slots
are Cyan/Pink. Preserve palette-index note associations. Exact engine state
matching does not use brightness or wire correction output. The observed
mask62/progress5 with six enabled boxes identifies box1 as unmatched, not a
disabled-box or off-by-one success condition. No independent matching algorithm
defect has been reproduced; neither the old snapshot nor new source tests can
prove what all physical groups displayed.

Add nonmutating per-participating-box intended palette positions, RGB, hex,
semantic color names and matched status to the existing Color Pattern snapshot,
plus its effective intro interval. Core regressions cover all511 nonempty masks
and all palette sizes2–9, repeated Down/release, sparse layouts, 5/6-to-6/6
completion, long-hold reversal, normal-brightness palette distinction and
reveal cap/stall/rollover behavior. Bind these reviewed bytes and proofs in the
same Puzzles ledger row before canonical verification. No board command,
firmware build, new round or optical acceptance was performed by this subtask;
fresh six-box human feedback remains required after approved deployment.

## Washed-color regression audit (2026-10-29)

Consulted the games-streaming and exact Puzzles/PCController AddressableLeds
ledger routes; reread the complete unchanged Puzzles sketch and AddressableLeds
implementation/header, then actual HostPresentation, KeyPresentation, StateMotion
and S3 wire conversion. Retain the shared RGB renderer and multiplicative
brightness/channel permutation, not a second color correction owner. The old
running-image palette had softened red/blue; the vivid palette is still pending
deployment. Remove the gray floor from optional Pink and explicitly request
HSV saturation/value 255/255 for normal and diagnostic rainbow, instead of the
installed FastLED convenience implementation's saturation 240. This source
finding alone does not diagnose severe physical washout or electrical health.

Add published-frame-to-wire regressions for all nine game colors and brightness
levels, plus zero-channel preservation under scalar effects. The first fixture
incorrectly inspected FastLED's restored background working buffer; correct it
to inspect the actual published composite. Its failed Windows process initially
locked the canonical executable; exact owned-process cleanup was denied, so
request manual cleanup rather than SSH/output-path/cache bypass. After that
process cleared, rerun the same canonical test wrapper successfully. Minigame
core/settings, both baked-role profiles and the full LED pipeline pass. The HSV
stub deliberately preserves old hue-clock encoding and observes producer
arguments only; it is not a claim of real HSV conversion or optical quality.
Saved settings, brightness, correction rules and channel calibration remain
untouched. Physical comparison and deployment remain separate pending gates.

## Mine Field shared minigame (2026-10-29)

Consult games-streaming and the exact Puzzles.ino donor-audit row; reread all
886 unchanged sketch lines and the current KeyMiniGame/HostPresentation/
StateMotion sources. Adapt the existing phase/countdown/output owner, not the
donor's blocking welcome waits, fixed puzzle code or a separate game engine.
Add Mine Field with seeded cyan/yellow initial states and at least one of each,
edge-deduplicated per-box toggles, either-color uniform success, frozen elapsed
result, shared high/low pulse and red timeout rendering. Shared countdown,
TM1637, finite color notes, configured result cue and output predicates remain
the only hardware owners. Preserve old enum IDs and keep Toggle last in the
advertised catalog. The new inclusion flag is On by default for key builds and
Off with the other key modes in table builds; no current settings are changed.

Reuse existing board-catalog menus, details/icons, typed drafts, explicit Save
and WS-first peer-bound controls in Flutter/Rust/Web. The existing show_ms field
is labeled Pulse period contextually; no API generation, synthetic per-frame
client animation or parallel Web app is introduced. Native's imported selector
keeps the same hit geometry; only explicit mode selection applies new defaults.

Canonical core/excluded-mode/settings tests pass, including all 502 enabled-box
masks with two or more boxes, both solutions, sparse layouts, duplicate edges,
countdown rollover/pause/timeout, output rules and persisted custom timings.
Flutter analyzer/22 focused game tests, native 13 focused tests, actual extracted
production dispatch integration, Web typed WS admission and isolated mobile
browser interaction pass. The first browser invocation lacked NODE_PATH;
reuse the documented bundled Playwright runtime via load_workspace_dependencies
and NODE_PATH, with no package install, before rerunning successfully. Canonical
Web full/onboard builds share the same source revision. Re-review unchanged PU
AsyncOTA.cpp and root route registration plus actual generated target anchors,
then rebind only the two target hashes, preserving the 50-route guard.
Device installation and optical/acoustic gameplay acceptance remain pending;
no board, active tablet or app process was changed in this implementation pass.
The final result controls show Winning time with millisecond precision for
Mine Field success instead of its leftover countdown; other modes are unchanged.
The S3 firmware and production-hold APK build pass; native evidence is the
focused compiled test binary, not a newly packaged production executable.

## Rust contextual game configuration (2026-10-29)

Consult games-streaming, native-dropdowns and the exact Pealayer dialog donor
row. Reread all1236 dialog and646 dropdown lines at their unchanged recorded
hashes, the current KeyMiniGame::Config defaults and native Game settings
consumer. Inspect the actual Pealayer5b3d5c1 hardware_action_button and PCController
f2260e3 CSS blame: recolor fixed geometry, never add a state-dependent stroke
width. Retain existing setting_row/detail-navigation/ComboBox helpers; no new
control implementation, modal runtime or board-side settings store is added.

Fix the consumer's incomplete mode-change defaults and ignored option exposure.
Known explicit mode changes refresh only advertised rule fields to the actual
engine defaults; wiring, device policies, unknown fields and same-mode edits
stay intact. Unknown future modes retain board-owned settings. Countdown,
challenge and presentation controls are contextual, reveal fields honor intro
Off, and stable numeric IDs survive hide/show. The output mapper changes only
after a real interaction, preserving optional metadata, and uses readable
predicate labels. Toggle behavior now uses the imported keyboard-aware selector
instead of stock selectable-value buttons. Apply remains a narrow peer-bound
patch with exact readback and stale-draft rejection. All 17 focused native tests
pass, including 110 different-mode transitions, 11 rule-visibility/layout cases,
new-mode warning validation and unchanged drafts/zero sends at 280/740 px.
The five shared-dropdown tests also pass, including keyboard focus/navigation,
search, fixed hover geometry and unrelated-editor arrow ownership.
Native-only changes require no firmware/APK rebuild or board/client replacement;
runtime acceptance stays separate from these focused source checks.

## Termux upgrade preflight and interruption (2026-10-29)

David authorized updating both ESP instances through Termux first, followed by
completion/review/merge of the mixed workspace. Preserve that order; no merge or
clean-working-tree claim is made while device deployment is blocked. The current
8022 key was matched against the previously trusted Termux key at5022 without
replacing ASUS's saved8022 key. Actual Termux/rish inventory found ledcontroller
on its saved station network, AVR online, no programming latch,480 pixels and
RMT/DMA active. Keycontroller was absent from hostname resolution and a fresh
phone Wi-Fi scan; never substitute a guessed device.

The canonical downloader obtained and verified the complete2314752-byte running
ESP application, but Android denied its final hardlink publication. Retain the
part file; independent transfer to this PC verified its running-image MD5 and
SHA256. This is a preserved firmware backup, not successful completion of the
downloader's final publication or a restore test. Android no-hardlink publication
still needs a safe exclusive-create/verified-copy fallback and platform tests.
Correction/group/configuration snapshots remain on the phone; retrieve and
independently verify them before it leaves. Do not restore historical scene values
over a user's newer intentional changes.

Actual preflight also rejected the growing compressed root Web UI because its
decoded body exceeded512KiB. Consult updates-telemetry/network-cloud and the
exact PCController netpolicy/http_url.go donor row, reading its authority checks,
PU850 FirmwareTransfer.h/.cpp and PCController UPDATE-PROGRESS.md in full. Keep
hostname-first same-board/boot/socket binding, auth, ownership, no-replay and
firmware digest checks. Increase only root HTML's compressed/decoded allowance
to a bounded1MiB in check-ethernet-live.mjs; all other response limits stay512KiB.
Loopback tests accept600KB raw/gzip HTML, reject over1MiB/decompression bombs and
retain the non-root JSON bound. Ethernet-live and updater fixture suites pass;
these are not device OTA or physical acceptance.

Both current ESP application candidates and pinned manifests were published in
new immutable HTTPS directories under office.digitalogic.ir/pakarcontroller/
firmware/esp32s3/, with server-side image/manifest SHA checks and direct200,
exact Content-Length for the ledcontroller image. This prepares an Internet
download source, not an installed image or autonomous remote command channel.
Before the upload started, Termux's SSH connection stopped completing handshakes
and then refused connections. Known alternate forwards did not answer. No board
firmware POST, reboot, scene/configuration write or AVR program operation occurred.
Shared chisel was left intact; only identity-verified owned stalled SCP clients
were stopped. Restore the phone tunnel and identify keycontroller before either
upgrade. The existing remote panel still pins an older board identity/topic;
do not run its historical installer or claim independent remote readiness.

### Same-session deployment continuation

Termux subsequently returned. Retrieve all ledcontroller snapshots onto this PC,
freshly match its old firmware metadata to the verified backup, and download the
candidate from the pinned HTTPS artifact. Launch exactly one canonical updater
under an exclusive marker with a detached private log; no mutation replay on
SSH loss. Actual local Async HTTP upload took42429ms, total188531ms including
full2695408-byte SHA256 readback. Same board `esp32s3-907069eaaf34` changed boot
from `62a8d50390ea3a9237c40df88afdbc87` to
`664d3dfebef9065a2ae21090a3583fc8`; running hash matches
`34ed92d994618ed5b7c23ae45ddc470929e6f20d34f25920ad37ed2df465430f`,
integrity is verified and selected/running OTA slot is VALID. Actual WS proofs
passed before/after. Independently compare saved correction rules/enabled/revision,
group counts/starts/overrides and all cell assignments/locks: unchanged. No AVR
programming or intentional LED-state command was performed. This is firmware and
state-preservation evidence, not optical/audio or power-loss acceptance.

David confirmed the Dlink router remains online and powered keycontroller back
on. Its AP then appeared, but not its LAN hostname. Termux joining this AP drops
the SSH route. A one-shot detached commissioner preserves the identified key
snapshots before any station patch and requests return to Dlink in its finalizer.
Its result must be retrieved before a retry; no key firmware upload was launched.
Do not confuse the absent tunnel with an absent powered board or assert that
its network mutation succeeded without the saved receipt.

Adapt the existing Mosquitto provisioner with explicit validated device scopes:
keep the old ledboard default, but create separate private credential files,
principals, ACL topics and rollback names for ledcontroller/keycontroller. Three
scope tests pass; register them and the actual cloud-service fixtures in CI.
Only ledcontroller's broker scope was provisioned so far. Its peer-fenced local
cloud settings commit received202 and readback confirms saved TLS MQTT policy.
Actual office probe authenticated/published to the broker, but the board never
replied and reports MQTT-2: this is NOT successful Internet board management.
The broker sees TLS EOF before MQTT authentication. Clock is synced. Strict
server TLS1.2/certificate checks pass; no certificate bypass is justified.

Read the actual installed ESP32 NetworkClientSecure/ssl_client and S3 qio_opi
sdkconfig. mbedTLS is prebuilt with internal-only allocations and16KiB record
buffers, so free PSRAM alone is not handshake headroom. Resource pressure is
plausible with measured57760 internal free/31732 largest, not yet proven. Add
bounded numeric/text SDK TLS failure, elapsed and pre/post internal heap detail
to the existing MQTT error only; preserve trust, timeout, ownership and retry
policy. Both ESP32/ESP8266 cloud fixtures pass, including control-byte sanitation,
long/unknown SDK detail, plain TCP and MQTT-auth failure distinction. Rebuild
the standard production profile without opt-in SSH/file-service inclusion flags;
those sources and the separately verified all-service image remain available.
Measure its actual live result before declaring the remote path fixed.

Review preparation found three open PRs, not a synchronized final main. Full
38/39 diffs and paginated conversations were reviewed read-only. Their combined
terminal source must preserve SSH command-only behavior and theme initialization,
and the new workspace fixture needs matching session fields. Semantic ledger
union and combined shell/SSH checks are required before normal merging. No
merge was performed ahead of the requested board-upgrade ordering.

The host denied cleanup of the test-created
`tools/__pycache__/provision-mqtt-device.cpython-312.pyc`. Do not reroute the denied
delete or hide it with ignore rules; user-assisted cleanup remains required.
The scope test now suppresses bytecode creation, including its CI invocation.

The standard16MiB/OPI/buffered/table/W5500/both-backend recovery build passed:
2388880 raw application bytes, static RAM140156 (7400 bytes less than the
all-service table image). SHA256
`24dfe517a2c8e0fb646bda5a1078b1ca1bff356ef4ca1a401bba1ed7da66f470`.
Its immutable HTTPS artifact/manifest were independently checked on office;
this is still a candidate, not another installed image or a live TLS fix.
Fresh Flutter picker analysis and all seven focused tests also pass. Key
commissioning confirmed no network API write: phone failed to stay associated
with the identified AP and requested restoration to Dlink. The later explicit
join lost SSH again; inspect current phone network before any board retry.

### MQTT TLS and detached commissioning continuation

Deploy the standard candidate24dfe517 through the canonical Termux updater once:
actual2388880-byte full SHA256 readback, VALID selected/running slot, same-peer
WebSocket and new bootc6cd787171b0adef7bffd78e03e6bc40 passed. Correction rules,
group layout and cell assignments/locks compare unchanged. MQTT TLS now connects
with the original root/hostname/time checks; office received correlated accepted
commands. This does not establish whole-system health: full status had grown
past16KiB and prevented all aggregate telemetry publication.

Reuse the existing compact projection for routine S3 cloud status, free its
owned temporary before building aggregate telemetry, and align fragmented S3
telemetry/receivers with the aggregate API's existing24KiB maximum. Keep768-byte
fragments and the existing4KiB command buffer, finite reassembly jobs/digest,
strict peer identity, deadlines and non-retained/no-replay command discipline.
Fixtures cover the boundary and oversized-status isolation. Fix a preexisting
source-guard slice that ended at configureNtp's earlier declaration, rather than
its definition. Rebind only reviewed changed target/proof hashes.

An actual Internet MQTT firmware-fetch job1 then failed BEFORE image bytes with
TLS-32512 (allocation failure), while retaining its existing MQTT TLS connection.
Temporarily switching RMT to GPIO did not repair this; job2 also failed before
bytes. The explicit saved RMT preference was restored via a correlated ACK.
The later temporary station-only recovery experiment must restore the prior AP
policy after inspection; acceptance does not prove an upgrade or AP restoration.

Read the actual installed mbedTLS esp_config/esp_mem headers and archive symbol
references, plus Espressif's esp_mem.c/Kconfig allocation strategy. Adapt its
SPIRAM calloc strategy only for record-sized allocations (>=16KiB), using the
project's existing GNU-wrap pattern rather than modifying the SDK or pretending
header macros reconfigure prebuilt libraries. No PSRAM or allocation failure
delegates to the original allocator; size multiplication overflow fails closed.
Small allocations retain the SDK policy, and its heap_caps_free handles both
regions. Certificate/SNI/time verification and record lengths are unchanged.
Linked host fixtures pass; actual two-connection/OTA hardware proof is separate.

For the newly requested detached board-to-board provisioning, reread actual
PCController netpolicy/http_url.go and hostbridge/peer_response.go. Reuse the
existing PeerRelay pinned dial/Host, bounded responses, explicit target identity
and no uncertain-write replay policy. Add a queued/persisted job with explicit
loop-only start after cloud callback return; transient NetworkConfig ownership
never edits saved source networking. It verifies target identity/role, snapshots
redacted settings, POSTs once, always requests restoration, and retains bounded
receipts across client loss/reboot. A pending rebooted job is not resumed.
Commission/network/relay fixtures pass; root integration/SDK and actual AP/LAN
execution are separate gates. See PEER-NETWORK-COMMISSIONING.md.

## Rich PortableShell device workspace (2026-10-29)

Consulted device-shell, terminal-ui, portable-shell inventory and the exact
PU850-ESP Shell/Telnet/Commands sources before replacing the limited renderer.
The canonical component pin is1235d4965ab9944be276807e31d6ef0298b9f280.
Six current files plus17 preserved historical records pass the23-entry source
lock. The original renderer/footer editor remain upstream behind explicit flags;
CPU/MCU identity never disables the rich table, themes, widgets or dialogs.

The actual ShellTuiBinding now supplies typed/category metadata and routes edits
through the same validators and hardware owners. Nine pages have responsive
columns, red rounded borders, inline editing, optional F2 dialogs, typed controls,
secret masking, stale-edit rejection and changed-row repaint. Session terminal
settings expose violet/red/lumen/onedark and true color. A complete-frame budget
and output-queue deferral prevent a larger table from faulting the old8KB queue.
History reporting uses its configured capacity instead of an unrelated literal.

The native shell wrapper, actual binding fixture, descriptor/PROGMEM profiles,
metadata gate,23-source lock and TCP/UDP loopbacks pass. The binding fixture
injects keys through actual application code for all9 pages, inline brightness,
toggle/theme dispatch, dialog cancel, masking and backpressure; its observations
and hardware owners are test doubles. Actual emitted ANSI bytes were replayed by
a C terminal utility and rendered with Agave. Inspection found and fixed Unicode
hint sanitization, palette-reset background patches, dialog line erasure and
unnecessary meters on generic numeric fields in the canonical upstream code.

Recovery evidence: an initial donor lookup used PU850 instead of the indexed
PU850-ESP name; inspected the inventory and consulted the correct records.
Comparison generation found Timer/LooseTestCodes.txt still bound to the former
4096-byte frame assertion. Read that entire donor and current test, then changed
only its proof anchor to the explicit complete-frame budget. Arduino sketch
staging omitted the new .inc member file; inspected the failing compiler path,
renamed the canonical member include to .hpp and repinned both consumers.
Fresh upstream CI passes native Windows/macOS/Linux with libc on/off, ARM32,
AVR components and explicit small/ASCII/legacy profiles. Consumer CI passes
Uno (view explicitly off), Mega (rich view on), STM32 and AVR simulation.

The first local guarded S3 SDK build passed (2396075 program bytes,140156 static
RAM bytes); source was still being refined during that iteration, so it is not
the final candidate attestation. Final frozen-source build and GitHub integration
are recorded separately. No board flash/reset, output diagnostic, production
session or saved setting was changed. The production hold remains active.
Event-editor Unicode/grapheme input, donor AP configuration/restore semantics and
full application parity remain open in the existing ledger/issue29. A richer
source renderer and fixture pictures do not establish universal superiority or
physical acceptance.

## Contextual matrix correction after user visual review (2026-10-29)

The first rich-table captures still repeated categories and overlapping footer
hints. User review also correctly questioned cell colors and the rectangular
layout. Reread PCController Tools/Controller/internal/tui/table.go completely at
b0111ae0 (SHA256 ecd399e2bf7718f3eb7b62e49f9dd884c0fb179e3d907bd15b238c01d31501e3),
plus actual LedGrid state/color accessors, StateAppearance, Protocol CellState,
TableCellMapping and web/modules/table-layout.js. The existing cell renderer
incorrectly collapsed state3 (won) into gray and ignored configured appearance.
The canonical view now takes provider RGB/state labels and the paired staggered
geometry; the application supplies its real StateAppearance colors. Selection
is a separate marker, not a replacement state color. Configured colors are not
physical light/driver confirmation. Geometry and arrow neighbors use the same
20-column paired offset mapping as the Web table, not a guessed wiring order.

Adapt PCController's logical-row/section-line separation into the shared bounded
view, with grouped headers and optional explicit category columns. Dedicated
matrix mode has its own viewport and contextual actions. Focus keeps cell/game
shortcuts out of settings; inline editing hides navigation hints; modal editing
owns its only Apply/Cancel hint. On/Off glyphs, friendly choices, brightness
percentages/ranges and units improve values without replacing raw owner keys.
Unicode dimensions use U+00D7; ASCII remains an explicit alternative.

Native tests cover configured lost/won colors, all80 binding RGB entries,
staggered neighbor IDs, narrow/ASCII rendering, focus refusal and displayed
owner rejection. Captures come from the real binding with deterministic doubles;
all9 pages and inline/dialog/Lumen/One Dark frames are re-rendered with Agave.
The same production hold applies. PR38 was returned to draft during correction;
its earlier green jobs do not certify this changed source. Complete final checks
and separately authorized hardware acceptance remain required.

The GitHub integration returned403 for comment/PR creation. Its read operations
remain usable; authenticated repository CLI writes were used, with no policy or
production approval bypass. Compiler errors from a local duplicate layout-counter
name and misleading indentation were fixed in the canonical renderer before
repinning; changed native assertions now check grouped/contextual behavior rather
than preserve the rejected repeated labels.

Final review removed the narrow-tab header's second navigation hint and changed
the shell's terminal-size response to Unicode multiplication with an ASCII
fallback. Canonical component pin f1804eb3261405d702e2ad276f462ba9fd1c13bf.
The full native shell wrapper passes after that pin. The preceding frozen4f6d1bc
SDK build passed at2400275 linked program bytes and140156 static RAM bytes;
its application BIN was2400416 bytes, SHA256
d34dafd380922c7e7f29a45c3d88761ac4feac0a10e521112a85739e52241d5a.
The final typography-only follow-up receives a separate build attestation on
PR38/issue29. Static RAM is not a runtime peak measurement. All historical donor
records and omissions remain; no production action was taken.

## Consolidated PR37/38/39 integration (2026-10-29)

Reconsulted github-coordination and the exact Pealayer review-audit donor, read
all refreshed PR conversations/reviews/threads and retained the complete reviewed
source diffs. PR39 and PR38 merged normally after their exact-head Build and
CodeQL success. No protection bypass or completed Copilot review is claimed.

The combined shell retains both command-only SSH prompt suppression and the
PortableShell theme, and the actual workspace fixture now models command-only
and closing fields used by the retained Exit branch. Both receipt appendices,
all source revisions/omissions and added mappings are retained. Resolve only
reference-hash conflicts for the reviewed combined shell/binding; regenerate the
existing Markdown/CSV projections, not a replacement ledger.

PR37 Build40827209405 failed because the BLE negative fixture's exact replacement
stopped matching when commissioning service was inserted before the relay call.
The production network guard was still present. The negative fixture now matches
that optional service, asserts that mutation actually occurs, and verifies LF
and CRLF cases. No guard is removed and no check is suppressed. The existing
bluetooth/source wiring was inspected; this is fixture repair, not BLE radio
acceptance. Combined shell and audit fixtures pass; final SSH/quality/SDK/client
CI remains a separate exact-head gate. No boards or production apps changed.

## Windows WebSocket-bootstrap fixture repair (2026-10-30, PR43)

Actions run41080991373 failed in the loopback server at peer_http.rs:746:
Winsock10035 (WouldBlock) on the first HTTP request read, followed by the
client's connection-reset error. The listener deliberately polls accept, but
Windows can inherit its nonblocking mode into accepted sockets. Explicitly
restore blocking mode before the existing bounded HTTP/WebSocket reads. Apply
the same correction to the adjacent cached-HTTP-route fixture. Preserve all
hostname, Host-header, verified-TCP-address, identity, revocation and deadline
assertions; no production transport code or command admission changes.

Consulted peer-binding's three mapped PCController session/admission sources
and outstanding lifecycle gaps, plus the actual netpolicy HTTP URL and public
transport tests. Those Go tests retain explicit route assertions, but their
net.Pipe/Go runtime does not supply a Rust/Winsock socket-mode fix. Keep Pakar's
existing bounded loopback fixture rather than replace production resolution.

The canonical focused native-test wrapper was attempted and refused before
Cargo: live Pealayer tracked-file inventory drift. Do not bypass that gate,
change its checkout, relabel unread donor changes as reviewed or claim a local
native pass. Exact-head Windows CI must execute the retained regression and
the broader native suite. No boards, installed apps or device settings changed.

The stacked artwork PR44 also exposed two stale PU route target bindings in
run41091875727. Reviewed the three generated WebUi.h changes: shared header
artwork, application icon bytes and favicon bytes only. Both existing remote
update anchors and route implementations remain unchanged at line417. Refresh
only those two target hashes in PU850-ROUTE-PARITY.json, keeping donor hashes,
route classifications and remaining acceptance unchanged. The actual50-route
checker, Web identity test and no-write application-artwork check pass. This is
not execution of native tests, firmware compilation or OTA/physical acceptance.
