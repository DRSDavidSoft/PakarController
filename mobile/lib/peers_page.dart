import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'app_update_coordinator.dart';
import 'discovery.dart';
import 'icons.dart';
import 'workspace_widgets.dart';

/// A standalone catalogue. Opening details never selects a board or starts an
/// update; package actions reuse the existing verified APK transaction owner.
class PeersPage extends StatefulWidget {
  const PeersPage({
    super.key,
    this.updates,
    this.seeds = const [],
    this.openBoard,
  });
  final AppUpdateCoordinator? updates;
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
        found = [if (verified != null) verified];
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
      if (mounted)
        setState(() {
          peers = coalesceDevices([...peers, ...found]);
          if (found.isEmpty) message = 'No peers reached in this sweep';
        });
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
      if (mounted)
        setState(
          () => message =
              '${updates.updates.state['state'] ?? 'Transfer complete'}',
        );
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
        if (message.isNotEmpty)
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 12),
            child: Text(message),
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
                ],
              ),
            ),
          ),
      ],
    ),
  );
}
