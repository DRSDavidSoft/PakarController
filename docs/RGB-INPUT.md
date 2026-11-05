# External RGB input

PakarController's native **Streaming → External RGB** panel receives complete
lighting frames and forwards them through its existing authenticated, selected
board WebSocket. It does not install a replacement firmware or open the AVR port.
The receiver and SignalRGB plugin are adapted from actual PCController sources;
the Rust implementation uses Pakar's own frame encoder and connection lease.

## Use

1. Connect the native app to the table board over Wi-Fi. The current board must
   acknowledge LEDs, streaming, identity and physical group layout. Select Cell
   mode, enable LED output in Effects mode and stop any global effect. These are
   explicit user actions; incoming network data never changes modes or enables
   output. Observe any production hold before doing this.
2. Stop rainbow, screen sampling and diagnostics. Choose an input protocol and
   either **480 physical pixels** (default) or **80 logical cells**. Logical mode
   requires the complete current cell map; every RGB triple expands through its
   physical group's reported span. Unused capacity is black. There is no assumed
   six-pixel stride, automatic resizing or silent partial-frame fill.
3. Choose the local endpoint and press **Start RGB input**. Configure the sender
   to emit exactly the selected number of RGB triples. Target rate is shared
   with the other streaming sources: 60 FPS default, configurable to 240, or 0
   for uncapped. Received frames use a one-frame latest-value mailbox.
4. **Stop RGB input** releases only this receiver. It does not issue a blackout,
   save a scene, restore a previous scene or change output mode. Board-owned
   behavior after the last frame follows the existing board renderer.

| Input | Default local endpoint | Supported subset |
|---|---|---|
| OpenRGB / Keyboard Visualizer | `127.0.0.1:65506` | AA + complete RGB + wrapping 16-bit checksum |
| DDP / SignalRGB plugin | `127.0.0.1:4048` | RGB24 display, offset zero, PUSH, one full frame |
| TPM2.NET | `127.0.0.1:65506` | One complete data packet |
| Adalight | `127.0.0.1:65506` or serial bridge | Standard N−1 count |
| Adalight OpenRGB | `127.0.0.1:65506` or serial bridge | OpenRGB sender's N count |
| TPM2 | `127.0.0.1:65506` or serial bridge | C9/DA data framing |
| Art-Net ArtDmx | `127.0.0.1:6454` | Unicast; one selected universe and channel window |
| sACN / E1.31 | `127.0.0.1:5568` | Unicast; one universe; CID/sequence pinning and termination |
| WLED realtime | `127.0.0.1:21324` | Complete DRGB/DNRGB; complete indexed WARLS for 80-cell mode |
| Hyperion raw RGB | `127.0.0.1:19446` | Exact complete RGB payload |

Art-Net and sACN use **80 logical cells** (240 DMX slots), with a configurable
first channel, so a single universe controls the whole table. A 480-pixel DMX
frame cannot fit in one universe and is rejected rather than partially applied.
Multi-universe assembly, DDP/TPM fragmentation, multicast, ArtPoll/ArtSync,
sACN sync/priority merging, RGBW and WLED JSON/device emulation are not implemented.
Full 480-pixel frames work through OpenRGB, DDP, TPM, Adalight, DRGB/DNRGB and raw RGB.
This host adapter currently targets the 480-pixel table, not the 216-pixel first-wins board.

## OpenRGB

Add an LED Strip device using the Keyboard Visualizer protocol and UDP destination
`127.0.0.1:65506`, with 480 LEDs (or 80 in logical mode). Select `openrgb` in
PakarController. If both PCController and PakarController run on the same machine,
give their listeners different ports; a bind conflict is an error, not takeover.

For serial senders, enable **Serial bridge** and use the receiving end of an
existing virtual COM/PTY pair; the sender opens the other end. A separate driver
or pair is not installed automatically. Match baud and protocol. Input is read
only, 8N1 with DTR deasserted on open; no probe or board command is transmitted.
The configured controller port (including Windows aliases) is rejected. Never
point a virtual-input setting at an unrelated physical device.

## SignalRGB

Copy `integrations/SignalRGB/PakarController.js` into the SignalRGB user plugin
directory (`Documents/WhirlwindFX/Plugins`). Edit its `target` endpoint and pixel
count to match this listener, restart SignalRGB, then explicitly start the DDP
input in PakarController and enable the plugin's **Stream RGB** control.
Default is 480 physical pixels at `127.0.0.1:4048`; set `pixels: 80` for logical
cells. Registration is an explicitly configured endpoint, not discovery or a
claim that the board is online. The plugin is an integration candidate awaiting
actual SignalRGB application verification. No commercial USB identity is cloned.

## Ownership and safety

Listeners are off at launch and never start on discovery or reconnect. LAN
binding requires a specific local unicast IP plus an explicit trusted sender IP;
IP filtering is not cryptographic authentication. The first valid source endpoint
is pinned, including Art-Net Physical or sACN CID. Foreign, malformed, duplicate
and stale sequenced frames do not refresh the lease. Sender silence stops input
after 5s by default (1–60 configurable); no initial valid frame stops after 60s.

The receiver is local to one authenticated WebSocket and board/boot identity.
Disconnect, identity/capability/mode/programming changes, stale board observation,
configuration change or cell-map changes stop it. An explicit realtime board
command releases it. Local generated streams and diagnostics cannot share its
lane. This does not introduce a new board-wide multi-client arbitration protocol;
other independent clients remain subject to existing firmware behavior.

Each output uses the existing two PKAR chunks, committing only the second, with
the current connection lease checked before each. Failed/uncertain writes stop
without replay or reconnect takeover. **Sent** means completed WebSocket writes,
not board ACKs or measured LED refresh; that distinction from PCController's
ACK-paced serial strip adapter is intentional.

## Verification boundary

Regression fixtures cover complete/truncated/oversized frames, checksums,
DMX limits, source ordering, termination, serial resynchronization and logical
mapping. Tests/CI are deferred at the user's request. No firmware, installed app
or production output is changed by this source port. Real OpenRGB/SignalRGB,
serial-pair, network, screenshots and physical acceptance remain separate gates.
Native UI is the start/stop surface for this host receiver; embedded Web, mobile
and TUI controls for it are not yet ported.

Wire references: [OpenRGB LED Strip source](https://gitlab.com/CalcProgrammer1/OpenRGB/-/blob/master/Controllers/LEDStripController/LEDStripController.cpp),
[SignalRGB UDP API](https://docs.signalrgb.com/developer/plugins/advanced-communication/),
[WLED realtime](https://kno.wled.ge/interfaces/udp-realtime/),
[Art-Net specification](https://art-net.org.uk/downloads/art-net.pdf).
