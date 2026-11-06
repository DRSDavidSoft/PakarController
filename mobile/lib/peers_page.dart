import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'app_update_coordinator.dart';
import 'discovery.dart';
import 'icons.dart';
import 'workspace_widgets.dart';
import 'api.dart';
import 'peer_policy.dart';

/// A verified source hint, not installation authority. The receiving firmware
/// still verifies its exact image family, source boot, hash and slot capacity.
Map<String, dynamic>? boardPeerFirmwareSource(
  DiscoveredDevice peer,
  String role,
  String version,
) {
  final address = InternetAddress.tryParse(peer.observedAddress);
  final octets = address?.rawAddress;
  final private =
      address?.type == InternetAddressType.IPv4 &&
      octets != null &&
      (octets[0] == 10 ||
          octets[0] == 172 && octets[1] >= 16 && octets[1] <= 31 ||
          octets[0] == 192 && octets[1] == 168);
  if (!peer.verified ||
      peer.kind != 'board' ||
      !private ||
      Uri.tryParse(peer.endpoint)?.port != 80 ||
      peer.metadata['role'] != role) {
    return null;
  }
  var hostname = '${peer.metadata['hostname'] ?? ''}';
  if (!hostname.endsWith('.local')) hostname = '$hostname.local';
  if (!RegExp(r'^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.local$')
      .hasMatch(hostname)) {
    return null;
  }
  final artifacts = peer.metadata['artifacts'];
  if (artifacts is! List) return null;
  for (final raw in artifacts) {
    if (raw is! Map || raw['path'] != '/firmware/download') continue;
    final artifact = Map<String, dynamic>.from(raw);
    final remote = PeerVersion.parse(artifact['version']),
        local = PeerVersion.parse(version);
    if (remote == null ||
        local == null ||
        remote.compareTo(local) <= 0 ||
        artifact['sha256'] is! String ||
        !RegExp(r'^[0-9a-f]{64}$').hasMatch(artifact['sha256'] as String) ||
        artifact['family'] is! String ||
        (artifact['family'] as String).isEmpty ||
        artifact['bytes'] is! int ||
        (artifact['bytes'] as int) < 24) {
      continue;
    }
    return {
      'hostname': hostname,
      'address': peer.observedAddress,
      'board_id': peer.metadata['board_id'],
      'boot_id': peer.metadata['boot_id'],
      'artifact': artifact,
    };
  }
  return null;
}

/// A standalone catalogue. Opening details never selects a board or starts an
/// update; package actions reuse the existing verified APK transaction owner.
class PeersPage extends StatefulWidget {
  const PeersPage({
    super.key,
    this.updates,
    this.seeds = const [],
    this.openBoard,
    this.api,
    this.openUpdates,
  });
  final AppUpdateCoordinator? updates;
  final PakarApi? api;
  final Future<void> Function()? openUpdates;
  final List<DiscoveredDevice> seeds;
  final Future<void> Function(DiscoveredDevice)? openBoard;
  @override
  State<PeersPage> createState() => _PeersPageState();
}

class _PeersPageState extends State<PeersPage> {
  final discovery = DeviceDiscovery(catalog: DiscoveryCatalog());
  final endpoint = TextEditingController();
  List<DiscoveredDevice> peers = [];
  bool busy = false;
  String message = '';
  bool staged = false;
  bool canStage(DiscoveredDevice peer) {
    final status = widget.api?.latestStatus;
    return status != null &&
        status.peer.has('peer-exchange') &&
        boardPeerFirmwareSource(
              peer,
              status.peer.role,
              '${status.json['version']}',
            ) !=
            null;
  }

  Future<void> stage(DiscoveredDevice peer) async {
    final api = widget.api;
    if (api == null || !canStage(peer) || busy) return;
    final owner = api.connectionRevision;
    final destination = api.latestStatus!;
    final source = boardPeerFirmwareSource(
      peer,
      destination.peer.role,
      '${destination.json['version']}',
    )!;
    final user = TextEditingController(text: 'admin'),
        password = TextEditingController();
    final approved = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Receive firmware from peer'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              '${peer.name} → ${api.latestStatus?.json['hostname'] ?? 'connected board'}',
            ),
            const SizedBox(height: 12),
            const Text(
              'Downloads and verifies an inactive image. Apply and reboot remain separate.',
            ),
            const SizedBox(height: 12),
            TextField(
              controller: user,
              decoration: const InputDecoration(labelText: 'Source username'),
            ),
            const SizedBox(height: 8),
            TextField(
              controller: password,
              obscureText: true,
              decoration: const InputDecoration(
                labelText: 'Source password (if required)',
              ),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('Cancel'),
          ),
          FilledButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text('Download & verify'),
          ),
        ],
      ),
    );
    final authorization = password.text.isEmpty
        ? ''
        : 'Basic ${base64Encode(utf8.encode('${user.text}:${password.text}'))}';
    user.dispose();
    password.dispose();
    if (approved != true || !mounted) return;
    if (api.connectionRevision != owner || !canStage(peer)) {
      setState(() => message = 'Board session changed. Inspect it again.');
      return;
    }
    setState(() {
      busy = true;
      message = '';
    });
    try {
      final receipt = await api.peerExchange('POST', '/api/peers/update', {
        'source': source,
        'authorization': authorization,
      });
      if (receipt['job_id'] is! int ||
          (receipt['job_id'] as int) <= 0 ||
          receipt['targets'] != 1 ||
          receipt['phase'] != 'download_esp' ||
          receipt['boot_id'] != destination.json['boot_id'] ||
          (receipt['esp'] as Map?)?['sha256'] !=
              (source['artifact'] as Map)['sha256']) {
        throw const FormatException(
          'Peer staging receipt differs from this source/destination; inspect progress before retrying.',
        );
      }
      if (mounted) {
        setState(() {
          staged = true;
          message = 'Firmware job ${receipt['job_id']}: ${receipt['phase']}';
        });
      }
    } on Object catch (error) {
      if (mounted) setState(() => message = '$error');
    } finally {
      if (mounted) setState(() => busy = false);
    }
  }

  @override
  void initState() {
    super.initState();
    scan();
  }

  @override
  void dispose() {
    endpoint.dispose();
    super.dispose();
  }

  Future<void> scan({String? origin}) async {
    if (busy) return;
    setState(() {
      busy = true;
      message = '';
    });
    try {
      List<DiscoveredDevice> found;
      if (origin != null) {
        final candidate = DiscoveredDevice(
          Uri.parse(origin).host,
          origin,
          'Manual',
        );
        final verified = await verifyDiscoveredDevice(candidate);
        found = [?verified];
      } else {
        found = await discovery.scan(seeds: widget.seeds);
        final updates = widget.updates;
        if (updates != null) {
          await updates.discovery.scan();
          found.addAll(
            updates.discovery.peers
                .where((p) => p.verified)
                .map(
                  (p) => DiscoveredDevice(
                    p.name,
                    p.origin,
                    '',
                    instanceId: p.instanceId,
                    kind: 'android-app',
                    verified: true,
                    metadata: p.manifest,
                  ),
                ),
          );
        }
      }
      if (mounted) {
        setState(() {
          peers = coalesceDevices([...peers, ...found]);
          if (found.isEmpty) message = 'No peers reached in this sweep';
        });
      }
    } on Object catch (error) {
      if (mounted) setState(() => message = 'Discovery failed: $error');
    } finally {
      if (mounted) setState(() => busy = false);
    }
  }

  Future<void> package(DiscoveredDevice peer, bool receive) async {
    final updates = widget.updates;
    if (updates == null || busy) return;
    setState(() {
      busy = true;
      message = '';
    });
    try {
      if (receive) {
        await updates.updates.pullPeer(peer.endpoint);
      } else {
        await updates.updates.pushPeer(peer.endpoint);
      }
      if (mounted) {
        setState(
          () => message =
              '${updates.updates.state['state'] ?? 'Transfer complete'}',
        );
      }
    } on Object catch (error) {
      if (mounted) setState(() => message = '$error');
    } finally {
      if (mounted) setState(() => busy = false);
    }
  }

  void details(DiscoveredDevice peer) {
    showModalBottomSheet<void>(
      context: context,
      isScrollControlled: true,
      showDragHandle: true,
      builder: (context) => SafeArea(
        child: Padding(
          padding: const EdgeInsets.fromLTRB(20, 4, 20, 20),
          child: ConstrainedBox(
            constraints: BoxConstraints(
              maxHeight: MediaQuery.sizeOf(context).height * .75,
            ),
            child: ListView(
              shrinkWrap: true,
              children: [
                Text(peer.name, style: Theme.of(context).textTheme.titleLarge),
                const SizedBox(height: 12),
                SelectableText(peer.endpoint),
                for (final entry in peer.metadata.entries.where(
                  (e) => {
                    'kind',
                    'version',
                    'role',
                    'instance_id',
                    'board_id',
                    'exchange',
                  }.contains(e.key),
                ))
                  ListTile(
                    contentPadding: EdgeInsets.zero,
                    title: Text(entry.key.replaceAll('_', ' ')),
                    subtitle: SelectableText(
                      entry.value is Map
                          ? const JsonEncoder.withIndent('  ')
                                .convert(entry.value)
                          : '${entry.value}',
                    ),
                  ),
                OutlinedButton.icon(
                  onPressed: () => Clipboard.setData(
                    ClipboardData(
                      text: const JsonEncoder.withIndent('  ')
                          .convert(peer.metadata),
                    ),
                  ),
                  icon: const Icon(Icons.copy_outlined),
                  label: const Text('Copy peer information'),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: const Text('Peers')),
    body: WorkspaceBody(
      children: [
        WorkspaceHeader(
          title: 'Boards & applications',
          icon: PakarIcons.broadcast,
          subtitle: '${peers.length} discovered',
          actions: [
            IconButton.outlined(
              tooltip: 'Discover peers',
              onPressed: busy ? null : () => scan(),
              icon: const Icon(PakarIcons.arrowClockwise),
            ),
          ],
        ),
        Row(
          children: [
            Expanded(
              child: TextField(
                controller: endpoint,
                keyboardType: TextInputType.url,
                decoration: const InputDecoration(
                  labelText: 'Peer address',
                  hintText: 'http://peer.local:8789',
                ),
                onSubmitted: busy ? null : (value) => scan(origin: value),
              ),
            ),
            const SizedBox(width: 8),
            IconButton.outlined(
              tooltip: 'Inspect address',
              onPressed: busy ? null : () => scan(origin: endpoint.text.trim()),
              icon: const Icon(Icons.search),
            ),
          ],
        ),
        const SizedBox(height: 12),
        if (busy) const LinearProgressIndicator(),
        if (widget.updates != null)
          ListenableBuilder(
            listenable: widget.updates!.updates,
            builder: (context, child) {
              final state = widget.updates!.updates.state;
              final total = state['bytes'],
                  done = state['received'] ?? state['sent'];
              if (total is! num || total <= 0 || done is! num) {
                return const SizedBox.shrink();
              }
              return Padding(
                padding: const EdgeInsets.symmetric(vertical: 8),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      '${state['state'] ?? 'App transfer'}',
                      style: Theme.of(context).textTheme.labelLarge,
                    ),
                    const SizedBox(height: 6),
                    LinearProgressIndicator(
                      value: (done / total).clamp(0, 1).toDouble(),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      '$done / $total bytes',
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                  ],
                ),
              );
            },
          ),
        if (message.isNotEmpty)
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 12),
            child: Text(message),
          ),
        if (staged && widget.openUpdates != null)
          Align(
            alignment: Alignment.centerRight,
            child: FilledButton.icon(
              onPressed: busy ? null : widget.openUpdates,
              icon: const Icon(Icons.system_update_alt),
              label: const Text('Firmware progress & actions'),
            ),
          ),
        for (final peer in peers)
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Icon(
                        peer.kind == 'board'
                            ? PakarIcons.circuitry
                            : Icons.devices,
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Text(
                          peer.name,
                          style: Theme.of(context).textTheme.titleMedium,
                        ),
                      ),
                      IconButton(
                        tooltip: 'Peer details',
                        onPressed: () => details(peer),
                        icon: const Icon(Icons.info_outline),
                      ),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Text(
                    peer.endpoint,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: Theme.of(context).textTheme.bodySmall,
                  ),
                  const SizedBox(height: 8),
                  Wrap(
                    spacing: 8,
                    runSpacing: 8,
                    children: [
                      WorkspaceBadge(
                        peer.kind == 'board' ? 'Board' : 'Application',
                      ),
                      if (peer.metadata['version'] is String)
                        WorkspaceBadge(peer.metadata['version'] as String),
                      if (peer.metadata['role'] is String)
                        WorkspaceBadge(peer.metadata['role'] as String),
                    ],
                  ),
                  if ((peer.kind == 'board' ||
                          peer.metadata['exchange'] is Map &&
                              peer.metadata['exchange']['bridge_enabled'] ==
                                  true) &&
                      widget.openBoard != null)
                    Align(
                      alignment: Alignment.centerRight,
                      child: TextButton.icon(
                        onPressed: busy ? null : () => widget.openBoard!(peer),
                        icon: const Icon(PakarIcons.link),
                        label: const Text('Open controls'),
                      ),
                    ),
                  if (peer.kind == 'android-app' && widget.updates != null)
                    Wrap(
                      spacing: 8,
                      children: [
                        OutlinedButton.icon(
                          onPressed: busy ? null : () => package(peer, true),
                          icon: const Icon(Icons.download_outlined),
                          label: const Text('Receive newer app'),
                        ),
                        OutlinedButton.icon(
                          onPressed: busy ? null : () => package(peer, false),
                          icon: const Icon(Icons.upload_outlined),
                          label: const Text('Offer this app'),
                        ),
                      ],
                    ),
                  if (canStage(peer))
                    Align(
                      alignment: Alignment.centerRight,
                      child: OutlinedButton.icon(
                        onPressed: busy ? null : () => stage(peer),
                        icon: const Icon(Icons.system_update_alt),
                        label: const Text(
                          'Receive firmware to connected board',
                        ),
                      ),
                    ),
                ],
              ),
            ),
          ),
      ],
    ),
  );
}
