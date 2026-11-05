# Workspace presentation

Preferences → Appearance → Workspace interface offers **egui Dock** (default)
and **CraftUI**. Both use the same panels, capability checks, selected board,
workspace profiles, hidden-panel intent and saved layout. Changing the mode is
a live preference: Save confirms it and Discard restores the saved choice.
It never reconnects a device or replays a hardware command.

CraftUI uses the application's existing colors, density, fonts and OS-theme
updates. Its tab menu opens the same Preferences/panel commands. Drag a tab to
reorder, split or float it; resize splitters and floating groups normally.
Closing a panel follows the existing workspace visibility owner.

Pakar's egui 0.36 dependency family remains coherent. CraftUI is pinned to its
upstream egui 0.36 revision; Pealayer uses its own reviewed egui 0.37 revision.
This is not a second stored layout or an embedded media-player implementation.

CraftUI's 0.36 floating surface presents a floating split window as a tab group.
The original nested split/collapse state remains stored and returns in egui
Dock; individual tab actions target their original identity. Whole-window
dragging repositions the group; redocking is by individual tab. These boundaries
must not be described as native cross-window or arbitrary floating-split parity.

Group Preferences navigation controls the uppercase section headings in both
application Preferences and Device Settings. Existing DEVICE, CONNECTIONS and
CONTROLLERS sections were present, not removed. Grouping never changes page IDs.

## Verification

The required native wrapper passed Check and all 173 focused UI tests. Tests cover projection
without mutation, preserved collapse state, shared moves/resize/close/reveal,
exact floating geometry before first paint and after save, invalid gesture
rejection, and grouped navigation/mode preview and persistence.
Execution results and native captures belong to the PR checkpoint; source
assertions alone are not visual or production acceptance.

The release build completed successfully. Native framebuffer captures at a
1280×800 logical viewport show egui Dock, CraftUI and grouped Appearance with
the actual mode selector. They use the same simulated, read-only loopback peer
and recorded binary identity, not a physical device. Files live under the
canonical PakarController verification directory. Both presentations also
expose existing fixed-content clipping in a very narrow Keys panel; this is a
separate responsive-content gap, not accepted as resolved by a docking switch.

Pealayer companion: ToghrolTP/pealayer#225. Pakar companion: #49, stacked on #48.
No physical board, firmware or output changes are part of this work.

For repeatable isolated screenshots, the existing native visual fixture accepts
`--workspace-ui=egui_dock` or `--workspace-ui=craft_ui`. Its Preferences surface
also accepts `--preferences-page=appearance`. Each run requires a new private
profile and output file, records the executable identity and uses only a
read-only loopback peer. These fixture values are not production defaults.
