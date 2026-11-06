# Standalone peer discovery and update exchange

One peer catalogue describes boards and controller applications. A board does
not need a connected controller, and an application does not need a connected
board. Existing board controls, APK installer and firmware maintenance owners
remain authoritative; finding a peer never starts a game, changes the selected
device, installs an app or reboots a board.

## Independent policies

- Public identity and descriptive releases remain readable through
  `GET /api/peer/info`, regardless of automatic checking or release enforcement.
- Automatic checking, artifact sharing and accepting an offer are separate
  controls. An enabled caller can query a passive peer whose automatic checks
  are disabled. A disabled check is not permission to bypass authentication,
  change the remote policy, accept an unsupported command or replay a write.
- A peer with a newer compatible artifact may offer it. An offer is a bounded,
  expiring suggestion, never an installation grant. A recipient requests the
  bytes and owns staging, cancellation, validation and explicit apply.
- Different artifact families remain visible but cannot update one another:
  table firmware is not key-game firmware; Android APKs are not desktop apps.
  Equal SemVer with different SHA/build metadata is not assumed newer.

## Additive metadata

Identity retains the existing product/application, board or instance identity,
session/boot, hostname, descriptive version, capabilities and compatibility
policy. `exchange` describes actual sharing/check/listener state. `artifacts`
describes only available packages with family, version, byte length, SHA-256,
relative download path and readiness/progress. Never publish credentials,
private settings, raw flash, an unavailable package or an invented checksum.

Board firmware exchange uses its running application partition and existing
inactive-slot staging/journal/apply pipeline. A LAN transfer pins source board,
boot, hostname, verified private IP, exact download path, image family, size
and whole-image SHA-256. It does not enable arbitrary HTTP firmware downloads.
The existing image/header/chip/partition checks and post-boot verification still
apply. Saved scenes, correction rules and settings are not part of the image.

Controller apps combine discovered boards and application peers in a dedicated
peer workspace. Read details without connecting a board; explicitly open the
selected board's existing controls or an advertised app-management capability.
Unreachable/auth-required/unsupported/stale peers retain distinct states.

## Acceptance (not claimed complete by this design)

- Firmware-only two-node discovery, newer/older offer and verified staging.
- App-only discovery and supported package exchange without a board.
- Mixed parties, separate application/artifact identities and management.
- Automatic checks disabled independently at either end, both release-policy
  build settings, authentication, malformed/equal/incompatible artifacts.
- No implicit flash, reboot, device selection or uncertain-write replay.
- Actual Flutter/Rust controls and deterministic transport/transfer tests;
  SDK builds and installed-device acceptance recorded separately.

## Implemented candidate endpoints and ownership

Tracked by [issue 58](https://github.com/atomicdeploy/PakarController/issues/58).
This is source-candidate documentation, not a claim of deployed acceptance.

| Operation | Endpoint | Effect |
| --- | --- | --- |
| Public identity and available artifacts | `GET /api/peer/info` | No selection or installation; readable with checks disabled |
| Board-owned catalogue | `GET /api/peers/discovered` | Bounded eight peers, age and stale state |
| Explicit discovery | `POST /api/peers/discover` | Runs the bounded SDK/network worker, never on the game loop |
| Independent board policies | `POST /api/peers/policy` | Persist `automatic_check`, `accept_offers`, `sharing` booleans with readback/rename |
| Advisory incoming offer | `POST /api/peer/offer` | One expiring suggestion, bound to actual private-LAN TCP source; never fetch/apply |
| Receiver-owned staging | `POST /api/peers/update` | Validate source and download to the inactive slot; never selects boot partition |
| Existing progress/apply/cancel | `/api/firmware/remote`, `/api/firmware/apply`, `/api/firmware/cancel` | Same journal, exact job/boot/hash ownership and explicit apply |

Discovery checks default on at a 60-second interval; turning them off does not
disable public identity, backup reads, explicit discovery or incoming offers.
Sharing off removes advertised artifacts and outgoing offers, not the existing
authenticated backup API. Source authentication is explicit, origin-pinned,
memory-only and cleared on completion/cancel/failure. No recipient credentials
are forwarded to a peer. General remote firmware HTTPS policy is unchanged.

For staging, send `source` with `hostname`, verified private `address`,
`board_id`, `boot_id` and one advertised `artifact`; optional `authorization`
is for that source alone. The running-image family is generated from MCU/FQBN,
role, pin map and compiled capabilities, not installation hostname, release,
initial box mask or an arbitrary Git hash. Validate the retained family marker
inside the complete SHA-verified image as well as the HTTP identity headers.
Different families are visible but cannot replace one another. A same-release
image is not automatically considered newer just because its digest differs.

The USB general API has the same catalogue, policy and staging operations.
WebSocket/MQTT use existing peer-fenced commands `peer-catalogue`,
`peer-discover` and `peer-update-stage`; no numeric wire opcode was added.

Flutter exposes Peers from Devices independently of a connection, with app APK
receive/offer actions using its existing signed-package transaction. Rust adds
a dockable Peers panel available disconnected and a separately configurable
passive identity listener; disabling API bridging still blocks WebSocket and
board/control/export calls. Opening a peer's controls uses the existing draft
guard, explicit connection and credential flow, not discovery authority.

Windows x64 apps also share only their running, hashed executable through
`GET/HEAD /api/peer/package`. Package preparation is asynchronous; unavailable
packages are never advertised. `peer_package_sharing` is separate from API
bridging and automatic update checks. Receivers re-probe the exact app/session,
pin its actual TCP address while retaining Host/TLS identity, validate family,
platform, release/build, byte length and complete SHA-256, then use the existing
managed candidate/rollback installer. Peer reception never inherits automatic
apply; Apply and restart binds the exact verified hash and still waits for all
drafts/operations to finish. Cached verified candidates are not re-streamed.
Other native platforms remain discovery/control peers but do not advertise a
Windows installer; their managed replacement adapters are explicitly absent.

Remaining: richer per-board transfer actions in both views, physical
board-only/app-only/mixed acceptance and measured RAM/latency/long-soak evidence.
Fixtures and SDK builds are not physical or installed-app acceptance.
