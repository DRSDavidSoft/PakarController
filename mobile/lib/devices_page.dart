import 'package:flutter/material.dart';

import 'device_registry.dart';
import 'host_wifi.dart';
import 'discovery.dart';
import 'icons.dart';
import 'cell_controls.dart';
import 'application_artwork.dart';

import 'dart:convert';

import 'api.dart';
import 'workspace_widgets.dart';
import 'ble_transport.dart';
import 'usb_transport.dart';
import 'app_update_coordinator.dart';
import 'peers_page.dart';

final _discoveryFallbackArtwork = base64Decode(applicationArtworkBase64);

class DeviceListEntry {
  const DeviceListEntry(
    this.peer,
    this.saved,
    this.available, {
    this.observedOrigins = const {},
  });
  final DiscoveredDevice peer;
  final SavedDevice? saved;
  final bool available;
  final Set<String> observedOrigins;
  String get name => saved?.name ?? peer.name;
  String get identityKey => peer.boardId.isNotEmpty
      ? 'board:${peer.boardId}'
      : peer.instanceId.isNotEmpty
      ? 'instance:${peer.instanceId}'
      : peer.endpoint;

  List<String> routes(ConnectionMethod method) {
    final values = peer.origins
        .where(
          (origin) => method == ConnectionMethod.usb
              ? origin.startsWith('usb://')
              : !origin.startsWith('usb://') &&
                    origin.startsWith('ble://') ==
                        (method == ConnectionMethod.bluetooth),
        )
        .toList();
    values.sort((a, b) {
      final fresh = (observedOrigins.contains(b) ? 1 : 0).compareTo(
        observedOrigins.contains(a) ? 1 : 0,
      );
      if (fresh != 0) return fresh;
      final address = (normalizeAddress(Uri.parse(a).host) != null ? 1 : 0)
          .compareTo(normalizeAddress(Uri.parse(b).host) != null ? 1 : 0);
      return address != 0 ? address : a.compareTo(b);
    });
    return values;
  }

  String? preferredOrigin(ConnectionMethod method) {
    final values = routes(method);
    // A saved literal IP/port is explicit route intent, not a suggestion to
    // normalize to another interface or to the merged board's hostname.
    if (saved != null && values.contains(saved!.endpoint)) {
      return saved!.endpoint;
    }
    return values.firstOrNull;
  }
}

List<DeviceListEntry> unifiedDevices(
  List<SavedDevice> saved,
  List<DiscoveredDevice> found,
  DiscoveryCatalog catalog,
) =>
    coalesceDevices([...saved.map((d) => d.identity), ...found])
        .map(
          (peer) => DeviceListEntry(
            peer,
            saved.where((d) => d.identity.sameBoard(peer)).firstOrNull,
            catalog.seenThisScan(peer),
            observedOrigins: catalog.observedOrigins(peer),
          ),
        )
        .toList()
      ..sort((a, b) {
        final order = (b.available ? 1 : 0).compareTo(a.available ? 1 : 0);
        return order != 0
            ? order
            : a.name.toLowerCase().compareTo(b.name.toLowerCase());
      });

class DevicesPage extends StatefulWidget {
  const DevicesPage({
    super.key,
    required this.select,
    this.registry,
    this.discovery,
    this.apiFactory,
    this.api,
    this.onDiscovered,
    this.updates,
  });
  final Future<void> Function(SavedDevice, String) select;
  final DeviceRegistry? registry;
  final DeviceDiscovery? discovery;
  final PakarApi Function(SavedDevice, String)? apiFactory;
  final PakarApi? api;
  final VoidCallback? onDiscovered;
  final AppUpdateCoordinator? updates;
  @override
  State<DevicesPage> createState() => _DevicesPageState();
}

class _DevicesPageState extends State<DevicesPage> {
  late final registry = widget.registry ?? DeviceRegistry();
  late final discovery = widget.discovery ?? DeviceDiscovery();
  List<SavedDevice> saved = [];
  List<DiscoveredDevice> found = [];
  bool busy = false, connecting = false, authPromptActive = false;
  String search = '', filter = 'All';
  String message = '';
  final selectedOrigins = <String, String>{};
  String? originFor(DeviceListEntry entry) {
    final selected = selectedOrigins[entry.identityKey];
    return entry.routes(method).contains(selected)
        ? selected
        : entry.preferredOrigin(method);
  }

  late ConnectionMethod method =
      widget.api?.connectionMethod ?? ConnectionMethod.network;
  final bluetooth = PakarBleTransport();
  final usb = PakarUsbTransport();
  final bleCatalog = DiscoveryCatalog();
  Map<String, dynamic> adapter = {};
  @override
  void dispose() {
    bluetooth.close();
    usb.close();
    super.dispose();
  }

  @override
  void initState() {
    super.initState();
    scan();
  }

  Future<void> reload() async {
    try {
      final v = await registry.load();
      if (mounted) setState(() => saved = v);
    } on Object {
      if (mounted) {
        setState(
          () => message =
              'Saved devices unavailable; existing data was not replaced.',
        );
      }
    }
  }

  Future<void> scan() async {
    if (busy) return;
    setState(() => busy = true);
    try {
      await reload();
      final List<DiscoveredDevice> v;
      if (method == ConnectionMethod.usb) {
        final attached = await usb.devices();
        v = bleCatalog.merge(
          attached.map(
            (d) => DiscoveredDevice(
              '${d['name']} · USB ${d['device_id']}',
              PakarUsbTransport.endpoint(
                d['device_id'] as int,
                d['interface'] as int,
              ),
              'USB OTG',
            ),
          ),
        );
      } else if (method == ConnectionMethod.bluetooth) {
        adapter = await bluetooth.adapter();
        final devices = await bluetooth.scan();
        v = bleCatalog.merge(
          devices.map(
            (device) => DiscoveredDevice(
              '${device['name'] ?? 'PakarController'}',
              PakarBleTransport.endpoint('${device['address']}'),
              'Bluetooth LE',
            ),
          ),
        );
      } else {
        v = await discovery.scan(
          seeds: saved
              .where(
                (d) =>
                    !d.endpoint.startsWith('ble://') &&
                    !d.endpoint.startsWith('usb://'),
              )
              .map((d) => d.identity),
        );
      }
      if (mounted) {
        setState(() {
          found = v;
          message =
              method != ConnectionMethod.network || discovery.errors.isEmpty
              ? ''
              : 'Scan incomplete: ${discovery.errors.keys.join(', ')}';
        });
        if (method == ConnectionMethod.network) widget.onDiscovered?.call();
      }
    } on Object catch (error) {
      if (mounted) {
        setState(
          () => message = method == ConnectionMethod.bluetooth
              ? 'Bluetooth scan: $error'
              : 'Unable to refresh devices. Saved profiles are unchanged.',
        );
      }
    } finally {
      if (mounted) setState(() => busy = false);
    }
  }

  Future<void> edit({SavedDevice? device, DiscoveredDevice? discovered}) async {
    final name = TextEditingController(
      text: device?.name ?? discovered?.name ?? 'New device',
    );
    final endpoint = TextEditingController(
      text: device?.endpoint ?? discovered?.endpoint ?? 'http://ledboard.local',
    );
    final user = TextEditingController(text: device?.user ?? 'admin');
    final password = TextEditingController();
    var location = device?.location ?? DeviceLocation.local;
    var error = '', writing = false, clearPassword = false;
    await showDialog<void>(
      context: context,
      builder: (context) => StatefulBuilder(
        builder: (context, update) => PopScope(
          canPop: !writing,
          child: AlertDialog(
            title: Text(device == null ? 'Add device' : 'Device profile'),
            content: SizedBox(
              width: 480,
              child: SingleChildScrollView(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    TextField(
                      controller: name,
                      decoration: const InputDecoration(
                        labelText: 'Display name',
                      ),
                    ),
                    const SizedBox(height: 12),
                    TextField(
                      controller: endpoint,
                      decoration: const InputDecoration(
                        labelText: 'Network origin or saved Bluetooth endpoint',
                      ),
                    ),
                    const SizedBox(height: 12),
                    DropdownButtonFormField<DeviceLocation>(
                      initialValue: location,
                      items: [
                        for (final l in DeviceLocation.values)
                          DropdownMenuItem(value: l, child: Text(l.name)),
                      ],
                      onChanged: writing
                          ? null
                          : (v) => update(() => location = v!),
                      decoration: const InputDecoration(
                        labelText: 'Local or remote transport',
                      ),
                    ),
                    if (device != null) ...[
                      const SizedBox(height: 12),
                      TextField(
                        controller: user,
                        decoration: const InputDecoration(
                          labelText: 'Username',
                        ),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: password,
                        obscureText: true,
                        decoration: InputDecoration(
                          labelText: 'New password (optional)',
                          helperText: 'Leave blank to keep the saved password',
                        ),
                      ),
                      SwitchListTile.adaptive(
                        contentPadding: EdgeInsets.zero,
                        title: const Text('Remove saved password'),
                        value: clearPassword,
                        onChanged: writing
                            ? null
                            : (v) => update(() => clearPassword = v),
                      ),
                    ],
                    if (error.isNotEmpty) Text(error),
                  ],
                ),
              ),
            ),
            actions: [
              TextButton(
                onPressed: writing ? null : () => Navigator.pop(context),
                child: const Text('Cancel'),
              ),
              FilledButton(
                onPressed: writing
                    ? null
                    : () async {
                        update(() => writing = true);
                        try {
                          var next = SavedDevice(
                            id: device?.id ?? randomIdentifier(),
                            name: name.text.trim(),
                            endpoint: endpoint.text,
                            location: location,
                            user: user.text.trim(),
                          );
                          final identity = discovered ?? device?.identity;
                          if (identity != null &&
                              identity.endpoint == next.endpoint) {
                            next = next.observed(identity);
                          }
                          final secret =
                              clearPassword || device?.endpoint != next.endpoint
                              ? ''
                              : password.text.isEmpty
                              ? null
                              : password.text;
                          await registry.save(next, secret);
                          if (context.mounted) Navigator.pop(context);
                        } on Object {
                          update(() {
                            error = 'Device settings could not be saved; check origin and credentials.';
                            writing = false;
                          });
                        }
                      },
                child: const Text('Save'),
              ),
            ],
          ),
        ),
      ),
    );
    await Future<void>.delayed(const Duration(milliseconds: 250));
    name.dispose();
    endpoint.dispose();
    user.dispose();
    password.dispose();
    await reload();
  }

  Future<(String, String)?> credentials(SavedDevice device) async {
    setState(() => authPromptActive = true);
    final user = TextEditingController(text: device.user),
        password = TextEditingController();
    final result = await showDialog<(String, String)>(
      context: context,
      builder: (dialog) => AlertDialog(
        title: const Text('Sign in'),
        content: SizedBox(
          width: 400,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(device.name, style: Theme.of(context).textTheme.titleMedium),
              const SizedBox(height: 16),
              TextField(
                controller: user,
                autofillHints: const [AutofillHints.username],
                decoration: const InputDecoration(labelText: 'Username'),
              ),
              const SizedBox(height: 16),
              TextField(
                controller: password,
                obscureText: true,
                autofocus: true,
                enableSuggestions: false,
                autocorrect: false,
                autofillHints: const [AutofillHints.password],
                decoration: const InputDecoration(labelText: 'Password'),
              ),
            ],
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialog),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () =>
                Navigator.pop(dialog, (user.text.trim(), password.text)),
            child: const Text('Sign in'),
          ),
        ],
      ),
    );
    await Future<void>.delayed(const Duration(milliseconds: 250));
    user.dispose();
    password.dispose();
    if (mounted) setState(() => authPromptActive = false);
    return result;
  }

  Future<void> connect(DeviceListEntry entry) async {
    if (connecting) return;
    setState(() {
      connecting = true;
      message = '';
    });
    final selected = originFor(entry);
    if (selected == null) {
      setState(() => connecting = false);
      return;
    }
    var device = SavedDevice(
      id: entry.saved?.id ?? randomIdentifier(),
      name: entry.name,
      endpoint: selected,
      user: entry.saved?.user ?? 'admin',
      location: entry.saved?.location ?? DeviceLocation.local,
      boardId: entry.saved?.boardId ?? entry.peer.boardId,
      aliases: entry.peer.origins.take(16).toSet(),
    ).observed(entry.peer);
    // Aliases share identity only after the live full-status verification.
    try {
      var secret = method != ConnectionMethod.network || entry.saved == null
          ? ''
          : await registry.password(device) ?? '';
      for (var attempt = 0; attempt < 2; ++attempt) {
        final api =
            widget.apiFactory?.call(device, secret) ??
            PakarApi(
              baseUrl: device.endpoint,
              user: device.user,
              password: secret,
            );
        api.connectionMethod = method;
        api.expectedBluetoothBoard = device.boardId;
        try {
          final status = await api.status();
          if (!status.peer.known ||
              !RegExp(r'^[!-~]{1,80}$')
                  .hasMatch('${status.json['board_id'] ?? ''}') ||
              !RegExp(r'^[!-~]{1,80}$')
                  .hasMatch('${status.json['boot_id'] ?? ''}')) {
            throw const FormatException(
              'Not a compatible PakarController board',
            );
          }
          final boardId = status.json['board_id'] as String? ?? '';
          if (device.boardId.isNotEmpty && device.boardId != boardId) {
            throw const FormatException(
              'The endpoint belongs to another board',
            );
          }
          if (!mounted) return;
          device = device.observed(
            DiscoveredDevice(
              status.peer.name,
              device.endpoint,
              entry.peer.protocol,
              boardId: boardId,
              mac: entry.peer.mac,
              udn: entry.peer.udn,
              addresses: entry.peer.ips,
              aliases: entry.peer.origins,
              verified: true,
            ),
          );
          device = await registry.save(
            device,
            method != ConnectionMethod.network ? null : secret,
          );
          // Retire the probe before the selected API establishes its session.
          // Closing a probe must never tear down a newly selected connection.
          await api.close();
          if (mounted) await widget.select(device, secret);
          return;
        } on AuthenticationRequired {
          if (!mounted) return;
          if (attempt > 0) {
            setState(
              () => message =
                  'Sign-in rejected. Check the username and password.',
            );
            return;
          }
        } finally {
          await api.close();
        }
        if (!mounted) return;
        final login = await credentials(device);
        if (login == null || !mounted) return;
        secret = login.$2;
        device = SavedDevice(
          id: device.id,
          name: device.name,
          endpoint: device.endpoint,
          location: device.location,
          user: login.$1,
        ).observed(device.identity);
      }
    } on Object catch (error) {
      if (mounted) {
        setState(
          () => message = error is FormatException
              ? error.message
              : 'Could not connect. Check the device address and network.',
        );
      }
    } finally {
      if (mounted) setState(() => connecting = false);
    }
  }

  Future<void> forget(SavedDevice device) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (dialog) => AlertDialog(
        title: const Text('Forget device?'),
        content: Text(device.name),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialog, false),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () => Navigator.pop(dialog, true),
            child: const Text('Forget'),
          ),
        ],
      ),
    );
    if (confirmed == true) {
      try {
        await registry.remove(device);
        await reload();
      } on Object {
        if (mounted) {
          setState(() => message = 'Could not remove the saved profile.');
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final entries =
        unifiedDevices(
              saved,
              found,
              method != ConnectionMethod.network
                  ? bleCatalog
                  : discovery.catalog,
            )
            .where(
              (entry) => entry.peer.origins.any(
                (value) => method == ConnectionMethod.usb
                    ? value.startsWith('usb://')
                    : !value.startsWith('usb://') &&
                          value.startsWith('ble://') ==
                              (method == ConnectionMethod.bluetooth),
              ),
            )
            .where((entry) => entry.peer.kind == 'board' || entry.saved != null)
            .toList();
    final visible = entries.where(
      (entry) =>
          (filter != 'Saved' || entry.saved != null) &&
          (filter != 'Available' || entry.available) &&
          ('${entry.name} ${entry.peer.origins.join(' ')} ${entry.peer.mac}')
              .toLowerCase()
              .contains(search.toLowerCase()),
    );
    final count = entries.where((d) => d.available).length;
    final savedCount = entries.where((d) => d.saved != null).length;
    return WorkspaceBody(
      children: [
        WorkspaceHeader(
          title: 'Devices',
          icon: PakarIcons.circuitry,
          subtitle: '$count available · $savedCount saved',
          actions: [
            if (method == ConnectionMethod.network)
              OutlinedButton.icon(
                icon: const Icon(PakarIcons.broadcast),
                label: const Text('Peers'),
                onPressed: () => Navigator.push<void>(
                  context,
                  MaterialPageRoute(
                    builder: (_) => PeersPage(
                      updates: widget.updates,
                      seeds: found,
                      openBoard: (peer) async {
                        Navigator.pop(context);
                        await connect(
                          DeviceListEntry(
                            peer,
                            null,
                            true,
                            observedOrigins: peer.origins,
                          ),
                        );
                      },
                    ),
                  ),
                ),
              ),
            if (method == ConnectionMethod.network && HostWifi.supported)
              IconButton.outlined(
                tooltip: 'Join or manage this device’s Wi-Fi',
                onPressed: connecting
                    ? null
                    : () => showHostWifi(
                        context,
                        knownNetworks: HostWifi.knownNetworks(
                          widget.api?.latestStatus?.json,
                        ),
                      ),
                icon: const Icon(PakarIcons.wifiHigh),
              ),
            OutlinedButton.icon(
              onPressed: busy || connecting ? null : scan,
              icon: const Icon(PakarIcons.arrowClockwise),
              label: const Text('Discover'),
            ),
            const SizedBox(width: 8),
            FilledButton.icon(
              onPressed: connecting ? null : () => edit(),
              icon: const Icon(PakarIcons.link),
              label: const Text('Add device'),
            ),
          ],
        ),
        DropdownButtonFormField<ConnectionMethod>(
          isExpanded: true,
          initialValue: method,
          decoration: const InputDecoration(labelText: 'Find devices using'),
          items: const [
            DropdownMenuItem(
              value: ConnectionMethod.usb,
              child: Text('USB OTG · direct offline control'),
            ),
            DropdownMenuItem(
              value: ConnectionMethod.network,
              child: Text('Network'),
            ),
            DropdownMenuItem(
              value: ConnectionMethod.bluetooth,
              child: Text('Bluetooth LE'),
            ),
          ],
          onChanged: busy || connecting
              ? null
              : (value) async {
                  if (value == null || value == method) return;
                  setState(() {
                    method = value;
                    found = [];
                    message = '';
                  });
                  await scan();
                },
        ),
        if (method == ConnectionMethod.usb)
          const Padding(
            padding: EdgeInsets.symmetric(vertical: 12),
            child: Text(
              'Connect the ESP32-S3 native USB port using an OTG cable. Requires the PakarController TinyUSB CDC build. UART bridges and USB-JTAG recovery ports are not controlled or reset.',
            ),
          ),
        if (method == ConnectionMethod.bluetooth) ...[
          const SizedBox(height: 12),
          Text(
            adapter['enabled'] == true
                ? 'Bluetooth on'
                : adapter['enabled'] == false
                ? 'Bluetooth off'
                : 'Bluetooth permission required',
          ),
          Wrap(
            spacing: 8,
            children: [
              OutlinedButton.icon(
                icon: const Icon(PakarIcons.lock),
                label: const Text('Nearby Devices permission'),
                onPressed: busy || connecting
                    ? null
                    : () async {
                        try {
                          await bluetooth.permission();
                          if (mounted) await scan();
                        } on Object catch (error) {
                          if (mounted) setState(() => message = '$error');
                        }
                      },
              ),
              OutlinedButton.icon(
                icon: const Icon(PakarIcons.radio),
                label: const Text('Enable Bluetooth'),
                onPressed: busy || connecting
                    ? null
                    : () async {
                        try {
                          await bluetooth.enable();
                        } on Object catch (error) {
                          if (mounted) setState(() => message = '$error');
                        }
                      },
              ),
            ],
          ),
        ],
        const SizedBox(height: 12),
        TextField(
          onChanged: (v) => setState(() => search = v),
          decoration: const InputDecoration(
            hintText: 'Find a device',
            prefixIcon: Icon(Icons.search),
          ),
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 8,
          children: [
            for (final item in ['All', 'Available', 'Saved'])
              ChoiceChip(
                label: Text(item),
                selected: item == filter,
                showCheckmark: false,
                onSelected: (_) => setState(() => filter = item),
              ),
          ],
        ),
        const SizedBox(height: 12),
        SizedBox(
          height: 3,
          child: busy || connecting && !authPromptActive
              ? const LinearProgressIndicator()
              : null,
        ),
        const SizedBox(height: 12),
        if (message.isNotEmpty)
          Padding(
            padding: const EdgeInsets.only(bottom: 12),
            child: Semantics(liveRegion: true, child: Text(message)),
          ),
        if (visible.isEmpty)
          WorkspaceSection(
            title: busy ? 'Looking for boards…' : 'No matching devices',
            icon: PakarIcons.radio,
            children: [
              Text(
                search.isNotEmpty
                    ? 'Try another name or address.'
                    : 'Discover nearby boards or add a hostname.',
              ),
            ],
          ),
        for (final entry in visible)
          Padding(
            padding: const EdgeInsets.only(bottom: 8),
            child: AnimatedOpacity(
              duration: const Duration(milliseconds: 180),
              opacity: entry.saved != null && !entry.available && !busy
                  ? .48
                  : 1,
              child: Card(
                margin: EdgeInsets.zero,
                clipBehavior: Clip.antiAlias,
                child: ListTile(
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 16,
                    vertical: 8,
                  ),
                  leading: entry.peer.application?.icon != null
                      ? SizedBox(
                          width: 40,
                          height: 40,
                          child: StableApplicationIcon(
                            key: ValueKey(entry.identityKey),
                            bytes: entry.peer.application!.icon!,
                            fallback: _discoveryFallbackArtwork,
                          ),
                        )
                      : Icon(
                          entry.saved?.location == DeviceLocation.remote
                              ? PakarIcons.cloud
                              : PakarIcons.circuitry,
                        ),
                  title: Text(
                    entry.name,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                  subtitle: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      if (entry.routes(method).length > 1)
                        DropdownButton<String>(
                          key: ValueKey('device-route:${entry.identityKey}'),
                          value: originFor(entry),
                          isExpanded: true,
                          underline: const SizedBox.shrink(),
                          style: Theme.of(context).textTheme.bodySmall
                              ?.copyWith(
                                color: Theme.of(context)
                                    .colorScheme
                                    .onSurfaceVariant,
                              ),
                          items: [
                            for (final origin in entry.routes(method))
                              DropdownMenuItem(
                                value: origin,
                                child: Row(
                                  children: [
                                    Expanded(
                                      child: Text(
                                        origin,
                                        maxLines: 1,
                                        overflow: TextOverflow.ellipsis,
                                      ),
                                    ),
                                    if (entry.observedOrigins.contains(
                                      origin,
                                    )) ...[
                                      const SizedBox(width: 8),
                                      const Icon(PakarIcons.check, size: 16),
                                    ],
                                  ],
                                ),
                              ),
                          ],
                          onChanged: connecting
                              ? null
                              : (value) {
                                  if (value != null) {
                                    setState(
                                      () => selectedOrigins[entry.identityKey] =
                                          value,
                                    );
                                  }
                                },
                        )
                      else
                        Text(
                          originFor(entry) ?? '',
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: TextStyle(
                            color: Theme.of(context)
                                .colorScheme
                                .onSurfaceVariant,
                          ),
                        ),
                      const SizedBox(height: 8),
                      Wrap(
                        spacing: 6,
                        runSpacing: 6,
                        children: [
                          WorkspaceBadge(
                            entry.observedOrigins.contains(originFor(entry))
                                ? 'Available'
                                : busy
                                ? 'Checking'
                                : 'Not reached',
                            active: entry.observedOrigins.contains(
                              originFor(entry),
                            ),
                          ),
                          if (entry.saved != null)
                            const WorkspaceBadge(
                              'Saved',
                              icon: PakarIcons.floppyDisk,
                            ),
                          if (entry.available && entry.peer.authRequired)
                            const WorkspaceBadge(
                              'Sign-in required',
                              icon: PakarIcons.lock,
                            ),
                          if (entry.available && entry.peer.protocol.isNotEmpty)
                            WorkspaceBadge(entry.peer.protocol),
                        ],
                      ),
                    ],
                  ),
                  onTap: connecting ? null : () => connect(entry),
                  trailing: PopupMenuButton<String>(
                    enabled: !connecting,
                    tooltip: 'Device actions',
                    onSelected: (value) => value == 'wifi'
                        ? showHostWifi(
                            context,
                            scope: entry.identityKey,
                            knownNetworks:
                                entry.peer.boardId.isNotEmpty &&
                                    entry.peer.boardId ==
                                        widget
                                            .api
                                            ?.latestStatus
                                            ?.json['board_id']
                                ? HostWifi.knownNetworks(
                                    widget.api?.latestStatus?.json,
                                  )
                                : const [],
                          )
                        : value == 'forget'
                        ? forget(entry.saved!)
                        : edit(device: entry.saved, discovered: entry.peer),
                    itemBuilder: (_) => [
                      if (method == ConnectionMethod.network &&
                          HostWifi.supported)
                        const PopupMenuItem(
                          value: 'wifi',
                          child: Row(
                            children: [
                              Icon(PakarIcons.wifiHigh, size: 18),
                              SizedBox(width: 10),
                              Text('Join Wi-Fi…'),
                            ],
                          ),
                        ),
                      PopupMenuItem(
                        value: 'edit',
                        child: Row(
                          children: [
                            Icon(
                              entry.saved == null
                                  ? PakarIcons.floppyDisk
                                  : PakarIcons.gear,
                              size: 18,
                            ),
                            const SizedBox(width: 10),
                            Text(
                              entry.saved == null
                                  ? 'Save device'
                                  : 'Edit profile',
                            ),
                          ],
                        ),
                      ),
                      if (entry.saved != null)
                        const PopupMenuItem(
                          value: 'forget',
                          child: Text('Forget device…'),
                        ),
                    ],
                  ),
                ),
              ),
            ),
          ),
      ],
    );
  }
}
