# Controller transport convergence

Tracking: [PakarController#41](https://github.com/atomicdeploy/PakarController/issues/41)
and [PCController#702](https://github.com/atomicdeploy/PCController/issues/702).
This is an implementation inventory and staged decision, not a claim of complete
transport parity or wire interoperability. Existing donor decisions remain in
file-comparison-evidence.json; this document does not replace that ledger.

## Decision: shared semantics, appropriate carriers

Keep one command/event/service contract, portable framing primitives and tested
carrier adapters. Do not call COBS a USB transport or put a desktop-sized envelope
in every AVR message. USB-UART, hardware CDC and TinyUSB CDC are physical choices
for the same application protocol. UDP datagrams and serial streams have different
delivery properties; bootloader protocols are external recovery contracts.

Do not remove the current native-service boundary: outer opcode0x11 is an audio
cue in Pakar but PWM in PCController. Both use A5/COBS/CRC-8/ATM and48-byte payloads,
yet HELLO/status/settings, timestamped ACK/events and
EEPROM layouts differ. Sharing codec mathematics does not make these semantics
interchangeable. See PC-CONTROLLER-MERGE-READINESS.md for the preserved comparison.

Opcode hardening now gives both namespaces a single allocation owner in
PCController Project/ProtocolContract.json. The actual original generator was
extracted into a portable core, pinned and reused for firmware/Rust/Flutter views
and Pealayer's native sender. Published allocations and CI base comparisons forbid
renumbering, removal or reserved-slot reuse. Native A5/1 and Pakar A5/0 are fixed
namespace admission, not generation selectors. Both sides reject foreign markers
before dispatch; the existing explicit contract tunnel remains. Do not restore
advisory marker acceptance or cross-dialect fallback. Payload/service parity and
deployment still have their separate acceptance gates.

## Actual implementation comparison

| Path / current owner | Useful attributes | Gaps / decision |
| --- | --- | --- |
| Pakar BoardCore Protocol/WireCodec, ESP UsbApiService, Rust usb_api and Flutter usb_transport | Fixed buffers and48-byte frames; explicit identity/boot fencing, demand leases; USB API chunks reuse selected services without Wi-Fi | Not a universal file/API/maintenance parity claim. Audit every advertised operation and client surface. Keep offline recovery and on-demand streams. |
| PCController Project/ProtocolCodec + UartProtocol, Go internal/native | Allocation-free C++ primitives; separate RX/TX and synchronous borrowed frame lifetime; timestamped ACK/events; Go shares framing with host UDP | Preserve nested replies and AVR stack budget. Wire IDs collide with Pakar; do not reinterpret old commands or replace zero-copy views blindly. |
| PCController transport/udp and Pealayer server/{udp_codec,binary_udp,udp_client} | Already share DGLU typed-value RPC, UUID target/session,64-bit correlation, bounded fragmentation/reassembly, leases, replay ledger and event sequencing | Not encrypted/authenticated by CRC. Local access/authority policy remains required. Flutter/MCU parity and real LAN/loss/soak acceptance are not established by codec tests. |
| Pealayer four_d/protocol command framing | Existing compact cinema commands use CRC-8/Maxim; its native controller envelope and host UDP use CRC-8/ATM | These are distinct device contracts, not interchangeable because they both use COBS. Keep established wire bytes and verify the selected peer/service before dispatch; do not globally replace one CRC polynomial. |
| Pakar PKAR UDP pixel frames | Small bounded RGB chunks and staged frame commit; useful for low-latency presentation | Not a general command protocol: no authentication or peer/boot targeting in this packet. Keep trusted-LAN scope; never promote to Internet maintenance/control. Retain throughput while designing an owned streaming service. |
| Pakar HTTP/WebSocket/MQTT, BLE and USB API | Existing typed commands, shared capability/peer policy, correlated receipts, maintenance and presentation services | Enabled adapters must advertise actual commands/limits; a selected-route bridge is not complete API parity. WS-first applies where a named command exists, not an invented opcode. |
| PCController IPC/HTTP/WS, native stdio/TCP/C ABI | Shared production dispatch and protected host ownership rather than another firmware implementation | Reconcile its typed operation registry and push domains with Pakar; do not start a second serial owner or turn discovery into authority. |
| Urboot/urclock/ISP and ESP ROM/OTA | Standard programming/recovery, backup/integrity and target-specific electrical policies | Preserve these adapters. They are not competing business protocols and cannot be replaced by a generic packet decoder. |
| Telnet/SSH shell, SCP/SFTP/FTP | Human/standard-client access and bounded file services | Keep shell negotiation and access policy distinct; terminal text must not interleave framed USB. Not an excuse to duplicate command dispatch. |

Sources inspected include the complete current C++ framing primitives, Go COBS,
native and UDP packet codec slices, Rust/Dart USB codec slices, USB request/route
and demand contracts, Pealayer's actual COBS decoder, and the existing PC/Pakar
merge-readiness and host-UDP specifications. This is not a complete read of every
service/dispatcher/client. Per-file pending work stays pending.

## First implementation checkpoint

- Same malformed COBS input was accepted by PCController C++/Go and Pakar C++/
  Rust/Dart decoders: zero inside a data block. Reject it before dispatch, even
  if a surrounding envelope has a matching CRC. Valid wire bytes do not change.
- PCController UDP encoding now applies the same full-nonfinal-fragment rule as
  its decoder. Previously it could emit a packet its own receiver rejects.
- tests/fixtures/cobs.json is a byte-identical cross-repository fixture, not a
  new transport. Go and Rust consume it; C++ and Dart have targeted malformed
  envelope regressions. Keep CRC-8/ATM's123456789→F4 check and encoder bytes.
- Pealayer already rejects embedded zeroes; do not change its working runtime
  merely to claim an import. Its empty-decoded-buffer convention is not evidence
  that an empty application frame is accepted.
- PCController exposes its actual extra-MCU build lanes through the read-only
  `node Tools/Firmware/targets.mjs list`. Explicit adapter metadata prevents a
  future STM32 registration accidentally executing the AVR-specific build path.

No board update, network scan/actuation or production process replacement is
part of this checkpoint. Source/fixture checks, compiled targets, installed apps,
wire timing and physical output have separate acceptance.

## Next pass: living shared contract and adapters

1. Select a canonical portable core owner and generated operation registry. Keep
   hardware profile, business role and feature budget independent. Until the
   repository merger, pin shared sources/fixtures with reviewed hashes; do not
   invent a third dispatcher or silently fork the source owner.
2. Define shared identities, boot/session admission, request IDs, deadline/cancel,
   typed errors, receipts, event revision/time, leases and resynchronization.
   Preserve stable operation IDs and richer timing fields. Choose explicit
   service namespaces rather than treating the reserved byte as a version.
3. Accept unknown optional fields; reject unknown required semantics safely.
   Advertise enabled services and actual size/queue/fragment limits. Slim builds
   omit implementations with flags and return Unsupported, not success/no-op.
   Back/forward compatibility means a negotiated common capability subset; it
   cannot guarantee arbitrary future semantics or ambiguous existing opcode use.
4. Use reliable control/maintenance and owned lossy presentation separately.
   Bound queues, fragment lifetimes and memory; CRC is not authentication.
   Outcome-unknown mutations never automatically replay after reconnect/retry.
5. Exercise the same operations/events, files/firmware, subscriptions and errors
   through every enabled adapter and client. Each missing cell in that matrix is
   an issue, not an advertised completed transport. Do not remove useful recovery
   or high-rate paths merely because another carrier lacks their capability.

## Following pass: MCU HAL and history-preserving merger

PCController already builds the production sketch and source aggregators for
ATmega328P, explicit Mega2560 profiles and native conformance targets. Pakar's
separate AVR lifecycle/dispatch is not yet the same source tree. Reconcile it
under existing PC issues103/227/18/125/375 and Pakar issues3/6/16.

Extract GPIO, serial/USB, monotonic clocks/timers, storage, interrupts, buses,
watchdog/reset and LED-driver seams. Add actual STM32/ESP adapters only after chip,
pins, toolchain, peripherals and memory budgets are selected. A name or build flag
alone is not MCU support; no fabricated working target is added here. Protect
current ISR timing, RF input, tone scheduling, relay safety and storage commits.

The future repository destination remains undecided. Before merging, preserve
immutable refs and manifests, worktree/stash/private-state inventories and both
Git graphs. Stage a reviewed `git merge --allow-unrelated-histories`, with path
moves retaining provenance. Compare every unique/overlapping file: source hash,
owner, licenses, useful behavior, tests, conflicts, semantic result and omissions.
No copy-flattening, blanket ours/theirs choice or deletion of uncertain history.
Code must have a named canonical owner before duplicate implementations retire.

Raw EEPROM/settings images remain rollback artifacts, not cross-product imports.
Use guarded semantic import/readback. Require slim/full profile flash/RAM/stack,
interoperability, loss/reorder/reconnect/backpressure, real-device/soak and recovery
evidence before calling the consolidated system finished. Resolve all applicable
PR reviews and checks without merging unrelated active owner drafts blindly.
