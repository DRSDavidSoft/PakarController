import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/services.dart';

import 'api.dart';
import 'discovery_datagrams.dart';
import 'peer_context.dart';
import 'peer_identity.dart';
import 'peer_policy.dart';

class DiscoveredDevice {
  const DiscoveredDevice(
    this.name,
    this.endpoint,
    this.protocol, {
    this.boardId = '',
    this.mac = '',
    this.udn = '',
    this.addresses = const {},
    this.aliases = const {},
    this.verified = false,
    this.authRequired = false,
    this.instanceId = '',
    this.kind = 'board',
    this.compatible = false,
    this.hostname = '',
    this.application,
    this.metadata = const {},
  });
  final String name, endpoint, protocol;
  final String boardId, mac, udn;
  final Set<String> addresses, aliases;
  final bool verified, authRequired, compatible;
  final String hostname;
  final PeerApplication? application;
  final String instanceId, kind;
  final Map<String, dynamic> metadata;

  Set<String> get origins =>
      {endpoint, ...aliases}.map(PakarApi.normalizeEndpoint).toSet();
  Set<String> get ips => {
    ...addresses.map(normalizeAddress).whereType<String>(),
    for (final origin in origins) ?normalizeAddress(Uri.parse(origin).host),
  };

  bool sameBoard(DiscoveredDevice other) {
    // A reused DHCP address must not merge two explicitly different boards.
    if (boardId.isNotEmpty && other.boardId.isNotEmpty) {
      return boardId == other.boardId;
    }
    if (instanceId.isNotEmpty && other.instanceId.isNotEmpty) {
      return instanceId == other.instanceId;
    }
    if (udn.isNotEmpty && other.udn.isNotEmpty && udn != other.udn) {
      return false;
    }
    if (udn.isNotEmpty && udn == other.udn) return true;
    final a = normalizeMac(mac), b = normalizeMac(other.mac);
    if (a != null && b != null) return a == b;
    return origins.intersection(other.origins).isNotEmpty ||
        Uri.parse(endpoint).port == Uri.parse(other.endpoint).port &&
            ips.intersection(other.ips).isNotEmpty;
  }

  DiscoveredDevice merged(DiscoveredDevice other) => DiscoveredDevice(
    other.verified ? other.name : name,
    // Prefer a hostname over an address; retain all matching transport aliases.
    InternetAddress.tryParse(Uri.parse(endpoint).host) == null
        ? endpoint
        : other.endpoint,
    {
      ...protocol.split(', '),
      ...other.protocol.split(', '),
    }.where((v) => v.isNotEmpty).join(', '),
    boardId: other.boardId.isNotEmpty ? other.boardId : boardId,
    mac: other.mac.isNotEmpty ? other.mac : mac,
    udn: other.udn.isNotEmpty ? other.udn : udn,
    addresses: {...ips, ...other.ips},
    aliases: {...origins, ...other.origins},
    verified: verified || other.verified,
    authRequired: other.authRequired,
    instanceId: other.instanceId.isNotEmpty ? other.instanceId : instanceId,
    kind: other.verified ? other.kind : kind,
    metadata: other.metadata.isNotEmpty ? other.metadata : metadata,
    compatible: other.verified ? other.compatible : compatible,
    hostname: other.hostname.isNotEmpty ? other.hostname : hostname,
    application: other.application ?? application,
  );
}

String? normalizeAddress(String value) =>
    InternetAddress.tryParse(value)?.address;
String? normalizeMac(String value) {
  final bytes = value.toLowerCase().replaceAll(RegExp('[:-]'), '');
  if (!RegExp(r'^[0-9a-f]{12}$').hasMatch(bytes) ||
      bytes == '000000000000' ||
      bytes == 'ffffffffffff') {
    return null;
  }
  return bytes;
}

List<DiscoveredDevice> coalesceDevices(Iterable<DiscoveredDevice> values) {
  final result = <DiscoveredDevice>[];
  for (var value in values.take(256)) {
    // Recheck all groups: a new MAC+IP observation can bridge hostname/IP rows.
    for (var i = result.length - 1; i >= 0; --i) {
      if (value.sameBoard(result[i])) value = result.removeAt(i).merged(value);
    }
    result.add(value);
  }
  return result;
}

/// Session-only advertisement history, never an authenticated peer registry.
/// A missed/failed scan is not evidence that a known hostname ceased to exist.
class DiscoveryCatalog {
  List<DiscoveredDevice> _known = [], _recent = [];
  bool seenThisScan(DiscoveredDevice value) => _recent.any(value.sameBoard);
  List<DiscoveredDevice> get currentVerified =>
      _recent.where((p) => p.verified).toList(growable: false);

  /// Only the exact origins observed/probed this sweep, NOT every alias of a
  /// merged board. One reachable interface does not verify its other routes.
  Set<String> observedOrigins(DiscoveredDevice value) => {
    for (final route in _recent.where(value.sameBoard))
      PakarApi.normalizeEndpoint(route.endpoint),
  };
  List<DiscoveredDevice> merge(Iterable<DiscoveredDevice> observations) {
    _recent = observations.take(128).toList();
    _known = coalesceDevices([..._known, ..._recent]);
    if (_known.length > 128) _known = _known.sublist(_known.length - 128);
    return _known.toList()..sort((a, b) => a.name.compareTo(b.name));
  }
}

DiscoveredDevice? parseMdns(Map<dynamic, dynamic> value) {
  if (value['endpoint'] is! String || !validText(value['name'], 96)) {
    return null;
  }
  try {
    final endpoint = PakarApi.normalizeEndpoint(value['endpoint']);
    final kind = value['kind'] == 'bridge' ? 'bridge' : 'board';
    final host =
        (value['hostname'] is String && (value['hostname'] as String).isNotEmpty
                ? value['hostname'] as String
                : kind == 'board'
                ? value['name'] as String
                : '')
            .toLowerCase();
    // Both actual ESP firmwares register DNS-SD with MDNS.begin(hostname) and
    // no instance-name override. Preserve that hostname/IP correlation even
    // when Android's ordinary .local resolver cannot resolve a saved profile.
    // It is a discovery alias only: the HTTP capability/identity probe still runs.
    return DiscoveredDevice(
      value['name'],
      endpoint,
      'mDNS / DNS-SD',
      instanceId: value['instance_id'] is String
          ? value['instance_id'] as String
          : '',
      kind: kind,
      addresses: {if (value['address'] is String) value['address'] as String},
      aliases: {
        if (validHostname(host))
          PakarApi.normalizeEndpoint(
            Uri.parse(endpoint).replace(host: '$host.local').toString(),
          ),
      },
    );
  } on FormatException {
    return null;
  }
}

/// Metadata only, adapted from PCController discovery.go and PU MNDP TLVs.
/// No credentials are sent by scanning, and advertisements never authorize.
class DeviceDiscovery {
  DeviceDiscovery({DiscoveryCatalog? catalog, this.advertisements, this.verify})
    : catalog = catalog ?? _sessionCatalog;
  static final _sessionCatalog = DiscoveryCatalog();
  final DiscoveryCatalog catalog;
  final Future<List<DiscoveredDevice>> Function()? advertisements;
  final Future<DiscoveredDevice?> Function(DiscoveredDevice)? verify;
  static const channel = MethodChannel('ir.digitalogic.pakar/discovery');
  final errors = <String, String>{};
  bool verificationComplete = false;
  bool _scanning = false;
  bool _overflow = false;

  void _addFound(List<DiscoveredDevice> found, DiscoveredDevice peer) {
    final index = found.indexWhere((p) => p.endpoint == peer.endpoint);
    if (index >= 0) {
      found[index] = found[index].merged(peer);
    } else if (found.length < 128) {
      found.add(peer);
    } else {
      _overflow = true;
    }
  }

  Future<List<DiscoveredDevice>> scan({
    Iterable<DiscoveredDevice> seeds = const [],
  }) async {
    if (_scanning) throw StateError('Discovery already running');
    _scanning = true;
    verificationComplete = false;
    _overflow = false;
    errors.clear();
    final found = <DiscoveredDevice>[];
    try {
      if (advertisements != null) {
        for (final peer in await advertisements!()) {
          _addFound(found, peer);
        }
      } else {
        await Future.wait([
          _guard('mDNS / DNS-SD', () async {
            final raw = await channel.invokeListMethod<dynamic>('scan');
            for (final v in raw ?? []) {
              if (v is Map) {
                final peer = parseMdns(v);
                if (peer != null) _addFound(found, peer);
              }
            }
          }),
          _guard('SSDP', () => _udp(found, false)),
          _guard('MNDP', () => _udp(found, true)),
        ]);
      }
      // Adapt donor discovery.dart's bounded four-worker compatibility probes.
      // Public XML/status only: no credentials, redirects or board mutations.
      // Probe each distinct origin before cross-channel merging: a saved
      // hostname failing Android DNS must not hide a working SSDP IP alias.
      final candidates = <String, DiscoveredDevice>{};
      for (final peer in found.followedBy(seeds)) {
        // A saved/merged board may have AP, station, Ethernet and hostname
        // routes. Verify each exact origin; never manufacture an interface IP
        // or inherit another alias's successful probe.
        for (final origin in peer.origins) {
          if (!{'http', 'https'}.contains(Uri.parse(origin).scheme)) {
            continue;
          }
          if (candidates.length >= 128 && !candidates.containsKey(origin)) {
            _overflow = true;
            continue;
          }
          final candidate = origin == peer.endpoint
              ? peer
              : DiscoveredDevice(
                  peer.name,
                  origin,
                  peer.protocol,
                  boardId: peer.boardId,
                  mac: peer.mac,
                  udn: peer.udn,
                  addresses: peer.addresses,
                  aliases: peer.origins,
                  instanceId: peer.instanceId,
                  kind: peer.kind,
                );
          candidates[origin] =
              candidates[origin]?.merged(candidate) ?? candidate;
        }
      }
      final queue = candidates.values.toList();
      final verified = <DiscoveredDevice>[];
      var next = 0;
      final deadline = DateTime.now().add(const Duration(seconds: 12));
      await Future.wait(
        List.generate(4, (_) async {
          while (next < queue.length && DateTime.now().isBefore(deadline)) {
            final peer = await (verify ?? verifyDiscoveredDevice)(
              queue[next++],
            );
            if (peer != null) verified.add(peer);
          }
        }),
      );
      verificationComplete =
          !_overflow && errors.isEmpty && next >= queue.length;
      return catalog.merge(verified);
    } on Object {
      catalog.merge(
        [],
      ); // A failed sweep cannot retain a previous online badge.
      rethrow;
    } finally {
      _scanning = false;
    }
  }

  Future<void> _guard(String protocol, Future<void> Function() work) async {
    try {
      await work();
    } on Object {
      errors[protocol] = 'Unavailable on this network/platform';
    }
  }

  Future<void> _udp(List<DiscoveredDevice> found, bool mndp) async {
    final socket = await RawDatagramSocket.bind(
      InternetAddress.anyIPv4,
      mndp ? 5678 : 0,
      reuseAddress: true,
    );
    await collectDiscoveryDatagrams(
      events: socket,
      receive: socket.receive,
      close: socket.close,
      window: const Duration(seconds: 4),
      onPacket: (packet) {
        final device = mndp
            ? parseMndp(packet.data, packet.address.address)
            : parseSsdp(packet.data, source: packet.address.address);
        if (device != null) _addFound(found, device);
      },
      send: () {
        socket.broadcastEnabled = true;
        if (mndp) {
          socket.joinMulticast(InternetAddress('224.0.0.88'));
          socket.send([0, 0, 0, 0], InternetAddress('255.255.255.255'), 5678);
        } else {
          socket.send(
            utf8.encode(
              'M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\n'
              'MAN: "ssdp:discover"\r\nMX: 1\r\nST: ssdp:all\r\n'
              'USER-AGENT: ${PeerIdentity.current.userAgent}\r\n'
              'X-Pakar-Version: ${PeerIdentity.current.version}\r\n\r\n',
            ),
            InternetAddress('239.255.255.250'),
            1900,
          );
        }
      },
    );
  }
}

DiscoveredDevice? parseSsdp(Uint8List packet, {String? source}) {
  if (packet.length > 4096) return null;
  try {
    final lines = const LineSplitter().convert(utf8.decode(packet));
    if (lines.isEmpty || !lines.first.startsWith('HTTP/1.1 200 ')) return null;
    final headers = <String, String>{};
    for (final line in lines.skip(1)) {
      final i = line.indexOf(':');
      if (i > 0) {
        final key = line.substring(0, i).trim().toLowerCase();
        if (headers.containsKey(key)) return null;
        headers[key] = line.substring(i + 1).trim();
      }
    }
    final u = Uri.parse(headers['location'] ?? '');
    if (u.userInfo.isNotEmpty ||
        u.host.isEmpty ||
        !['http', 'https'].contains(u.scheme)) {
      return null;
    }
    if (source != null &&
        normalizeAddress(u.host) != normalizeAddress(source)) {
      return null;
    }
    final endpoint = PakarApi.normalizeEndpoint(
      u.replace(path: '', query: null, fragment: null).toString(),
    );
    return DiscoveredDevice(
      headers['x-pccontroller-name'] ?? headers['usn'] ?? u.host,
      endpoint,
      'SSDP',
      addresses: {?source},
      udn: (headers['usn'] ?? '').split('::').first,
    );
  } on FormatException {
    return null;
  }
}

DiscoveredDevice? parseMndp(Uint8List packet, String source) {
  if (packet.length < 8 || packet.length > 2048) return null;
  final data = ByteData.sublistView(packet);
  final fields = <int, Uint8List>{};
  for (var i = 4; i < packet.length;) {
    if (i + 4 > packet.length) return null;
    final type = data.getUint16(i), length = data.getUint16(i + 2);
    i += 4;
    if (i + length > packet.length || fields.containsKey(type)) return null;
    fields[type] = packet.sublist(i, i + length);
    i += length;
  }
  try {
    final name = utf8.decode(fields[5] ?? []);
    if (name.isEmpty ||
        name.length > 96 ||
        RegExp(r'[\x00-\x1f\x7f]').hasMatch(name) ||
        !(utf8.decode(fields[12] ?? []) == 'PakarController' ||
            utf8.decode(fields[0xff01] ?? []) == 'pakarcontroller') ||
        fields[1]?.length != 6 ||
        packet[0] != 0 ||
        packet[1] != 0 ||
        normalizeAddress(source) == null) {
      return null;
    }
    final application = utf8.decode(fields[0xff01] ?? []) == 'pakarcontroller';
    final portBytes = fields[0xff02];
    if (application && (portBytes == null || portBytes.length != 2)) {
      return null;
    }
    final port = application
        ? ByteData.sublistView(portBytes!).getUint16(0)
        : 80;
    if (port == 0) return null;
    return DiscoveredDevice(
      name,
      PakarApi.normalizeEndpoint('http://$source:$port'),
      'MNDP',
      boardId: application ? '' : utf8.decode(fields[11] ?? []),
      instanceId: application ? utf8.decode(fields[11] ?? []) : '',
      kind: application ? 'bridge' : 'board',
      mac: fields[1]!.map((v) => v.toRadixString(16).padLeft(2, '0')).join(),
      addresses: {source},
    );
  } on FormatException {
    return null;
  }
}

/// Adapt actual PCController verifyDiscoveredDevice: bounded read-only HTTP,
/// DIRECT, no redirects/secrets. Pakar exposes public UPnP XML instead of healthz.
/// Compatibility is not authentication; command capability/boot fences remain.
Future<DiscoveredDevice?> verifyDiscoveredDevice(
  DiscoveredDevice candidate,
) async {
  final origin = Uri.parse(PakarApi.normalizeEndpoint(candidate.endpoint));
  final client = HttpClient()
    ..findProxy = ((_) => 'DIRECT')
    ..connectionTimeout = const Duration(seconds: 1);
  final observedIps = <String>{...candidate.ips};
  Future<(int, String)> read(String path) async {
    final request = await client.getUrl(origin.resolve(path));
    identifyRequest(request);
    request.followRedirects = false;
    final response = await request.close();
    final ip = response.connectionInfo?.remoteAddress.address;
    if (ip != null) observedIps.add(ip);
    final bytes = <int>[];
    if ((response.contentLength) > 16384) {
      throw const FormatException('Discovery size');
    }
    await for (final chunk in response) {
      if (bytes.length + chunk.length > 16384) {
        throw const FormatException('Discovery size');
      }
      bytes.addAll(chunk);
    }
    return (response.statusCode, utf8.decode(bytes));
  }

  Future<DiscoveredDevice?> probe() async {
    Map<String, dynamic> info = const {};
    try {
      final response = await read('/api/peer/info');
      if (response.$1 == 200) {
        final value = jsonDecode(response.$2);
        if (value is Map<String, dynamic> &&
            (value['product'] == 'PakarController' ||
                value['application'] == 'PakarController')) {
          info = value;
        }
      }
    } on Object {
      /* Older public discovery endpoints remain usable. */
    }
    final kind = info['kind'];
    if ({'android-app', 'desktop-app', 'bridge'}.contains(kind)) {
      final id = info['instance_id'] ?? info['id'];
      if (!validText(id, 80) ||
          candidate.instanceId.isNotEmpty && id != candidate.instanceId) {
        return null;
      }
      return DiscoveredDevice(
        info['name'] is String
            ? info['name'] as String
            : info['hostname'] is String
            ? info['hostname'] as String
            : candidate.name,
        candidate.endpoint,
        candidate.protocol,
        instanceId: id as String,
        kind: kind as String,
        verified: true,
        metadata: info,
        addresses: observedIps,
        aliases: candidate.origins,
      );
    }
    final xml = await read('/description.xml');
    final metadata = xml.$1 == 200 ? pakarXmlIdentity(xml.$2) : null;
    if (metadata != null &&
        candidate.udn.isNotEmpty &&
        candidate.udn != metadata.$2) {
      return null;
    }
    final status = await read('/api/status');
    Map<String, dynamic>? value;
    if (status.$1 == 200) {
      final raw = jsonDecode(status.$2);
      if (raw is Map<String, dynamic> &&
          PeerContext.fromStatus(raw).known &&
          validText(raw['board_id'], 80) &&
          validText(raw['boot_id'], 80)) {
        value = raw;
      }
    }
    if (metadata == null && value == null) return null;
    final instance = value?['bridge'] is Map
        ? value!['bridge']['instance_id'] as String? ?? ''
        : metadata?.$2.startsWith('uuid:') == true && candidate.kind == 'bridge'
        ? metadata!.$2.substring(5)
        : '';
    if (candidate.instanceId.isNotEmpty && instance != candidate.instanceId) {
      return null;
    }
    final board = value?['board_id'] as String? ?? candidate.boardId;
    if (candidate.boardId.isNotEmpty &&
        value != null &&
        candidate.boardId != board) {
      return null;
    }
    var compatible = false;
    if (value != null) {
      try {
        validatePeerBoard(value, appVersion: PeerIdentity.current.version);
        compatible = true;
      } on UnsupportedPeerVersion {
        /* Visible, but never auto-selected. */
      }
    }
    return DiscoveredDevice(
      value == null ? metadata!.$1 : PeerContext.fromStatus(value).name,
      candidate.endpoint,
      candidate.protocol,
      boardId: board,
      mac: candidate.mac,
      udn: metadata?.$2 ?? candidate.udn,
      addresses: observedIps,
      aliases: candidate.origins,
      verified: true,
      authRequired: status.$1 == 401,
      application: value == null
          ? null
          : PeerContext.fromStatus(value).application,
      instanceId: instance,
      kind: value?['bridge'] is Map ? 'bridge' : candidate.kind,
      compatible: compatible,
      hostname: value?['hostname'] is String
          ? value!['hostname'] as String
          : '',
      metadata: info,
    );
  }

  try {
    return await probe().timeout(const Duration(seconds: 3));
  } on Object {
    return null;
  } finally {
    client.close(force: true);
  }
}

/// Deliberately accepts only the bounded flat identity fields of our public
/// description, not arbitrary XML/DTDs, external entities or executable markup.
(String, String)? pakarXmlIdentity(String xml) {
  if (xml.length > 16384 ||
      RegExp(r'<!DOCTYPE|<!ENTITY', caseSensitive: false).hasMatch(xml) ||
      !RegExp(r'<root(?:\s|>)').hasMatch(xml) ||
      !xml.contains('<device>')) {
    return null;
  }
  String? tag(String name) {
    final matches = RegExp('<$name>([^<]*)</$name>').allMatches(xml).toList();
    if (matches.length != 1) return null;
    return matches.single
        .group(1)!
        .replaceAll('&amp;', '&')
        .replaceAll('&lt;', '<')
        .replaceAll('&gt;', '>')
        .replaceAll('&quot;', '"')
        .replaceAll('&apos;', "'");
  }

  final model = tag('modelName'), name = tag('friendlyName'), udn = tag('UDN');
  if (!{'PakarController', 'Pakar Table', 'Pakar Controller'}.contains(model) ||
      tag('manufacturer') != 'AtomicDeploy' ||
      (tag('modelURL') != null &&
          tag('modelURL') !=
              'https://github.com/atomicdeploy/PakarController') ||
      !validText(name, 96) ||
      udn == null ||
      !(tag('modelNumber') == 'bridge'
          ? RegExp(r'^uuid:[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$')
                .hasMatch(udn)
          : RegExp(r'^uuid:38323636-4558-4dda-9188-cda0e6[0-9a-f]{6}$')
                .hasMatch(udn))) {
    return null;
  }
  return (name!, udn);
}
