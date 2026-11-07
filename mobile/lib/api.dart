import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:http/http.dart' as http;
import 'package:http/io_client.dart';
import 'package:crypto/crypto.dart';
import 'package:web_socket_channel/io.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import 'models.dart';
import 'board_sound.dart';
import 'lan_resolver.dart';
import 'discovery_datagrams.dart';
import 'command_wire.dart';
import 'command_readback.dart';
import 'game_settings_command.dart';
import 'led_backend.dart';
import 'command_failure.dart';
import 'pixel_diagnostic.dart';
import 'ble_transport.dart';
import 'peer_identity.dart';
import 'peer_policy.dart';
import 'usb_transport.dart';

enum ConnectionMethod { network, usb, bluetooth }

class PakarApi {
  PakarApi({
    String baseUrl = 'http://ledboard.local',
    String user = 'admin',
    String password = '',
    http.Client? client,
    LanResolver? resolver,
    PakarBleTransport? bluetooth,
    PakarUsbTransport? usb,
    this.discovery,
    this.readTimeout = const Duration(seconds: 6),
    this.enforcePeerVersions = peerVersionChecksEnabled,
    this.commandHttpFallback = true,
  }) : resolver = resolver ?? LanResolver(),
       bluetooth = bluetooth ?? PakarBleTransport(),
       usb = usb ?? PakarUsbTransport(),
       _baseUrl = normalizeEndpoint(baseUrl),
       // Preserve public argument names; private setters own revisions.
       // ignore: prefer_initializing_formals
       _user = user,
       // ignore: prefer_initializing_formals
       _password = password {
    if (readTimeout <= Duration.zero ||
        readTimeout > const Duration(seconds: 30)) {
      throw ArgumentError.value(readTimeout, 'readTimeout');
    }
    _client = client ?? IOClient(_ioClient);
    if (_baseUrl.startsWith('ble://')) {
      _connectionMethod = ConnectionMethod.bluetooth;
    } else if (_baseUrl.startsWith('usb://')) {
      _connectionMethod = ConnectionMethod.usb;
    }
  }

  String _baseUrl, _user, _password;

  /// WebSocket remains preferred. Disable only command HTTP recovery, not
  /// independent read-only status/identity or the explicit Bluetooth transport.
  bool commandHttpFallback;

  /// Network discovery policy, independent of the explicit transport choice.
  bool automaticDiscovery = true;
  int _connectionRevision = 0;
  int get connectionRevision => _connectionRevision;
  bool _connectionWanted = true;
  // Optional UI reads must stop synchronously, even before Android renders the
  // frame that propagates inactive page properties to hidden widgets.
  bool _appForeground = true;
  bool get appForeground => _appForeground;
  set appForeground(bool value) {
    if (value == _appForeground) return;
    _appForeground = value;
    usb.demand(_effectiveUsbDemand, foreground: value);
    _syncPixelSubscription();
    if (!_connectionUpdates.isClosed) _connectionUpdates.add(null);
  }

  final _connectionUpdates = StreamController<void>.broadcast(sync: true);
  Stream<void> get connectionUpdates => _connectionUpdates.stream;
  bool get connectionWanted => _connectionWanted;
  ConnectionMethod _connectionMethod = ConnectionMethod.network;
  ConnectionMethod get connectionMethod => _connectionMethod;
  set connectionMethod(ConnectionMethod value) {
    if (_connectionMethod == value) return;
    _connectionMethod = value;
    resetTransport();
  }

  // All transports share admission; USB branches precede every network path.
  void _requireNetwork() {
    if (_unsupportedPeer != null) throw _unsupportedPeer!;
    if (!_connectionWanted) throw const ConnectionStopped();
  }

  void disconnect() {
    _connectionWanted = false;
    resetTransport();
  }

  void connect() {
    _connectionWanted = true;
    resetTransport();
  }

  void selectTransport(
    ConnectionMethod method,
    String endpoint, {
    String board = '',
  }) {
    final normalized = normalizeEndpoint(endpoint);
    if ((method == ConnectionMethod.usb) != normalized.startsWith('usb://') ||
        (method == ConnectionMethod.bluetooth) !=
            normalized.startsWith('ble://')) {
      throw const FormatException('Select an endpoint for this transport');
    }
    _connectionWanted = false;
    _connectionMethod = method;
    _baseUrl = normalized;
    _knownNetworkEndpoint = normalized;
    _knownNetworkBoard = method == ConnectionMethod.network && board.isNotEmpty
        ? board
        : null;
    _lastVerifiedNetworkOrigin = null;
    expectedBluetoothBoard = board;
    _identityRequired = false;
    resetTransport();
  }

  void Function()? onPeerChanged;
  void Function()? onConfigurationChanged;
  Timer? _configurationRefresh;
  int _configurationRevision = 0;
  final _configurationUpdates = StreamController<int>.broadcast(sync: true);
  Stream<int> get configurationUpdates => _configurationUpdates.stream;
  final _networkObservations = StreamController<Map<String, dynamic>>.broadcast(
    sync: true,
  );
  Stream<Map<String, dynamic>> get networkObservations =>
      _networkObservations.stream;
  void Function()? onTransportRetired;
  void Function(Map<String, dynamic>)? onPeripheralEvent;
  void Function()? onPeripheralStreamLost;
  bool _identityRequired = false;
  String get baseUrl => _baseUrl;
  set baseUrl(String value) {
    final normalized = normalizeEndpoint(value);
    if (_baseUrl != normalized) {
      _baseUrl = normalized;
      _knownNetworkEndpoint = _knownNetworkBoard = _lastVerifiedNetworkOrigin =
          null;
      _identityRequired = false;
      resetTransport();
    }
  }

  String get user => _user;
  set user(String value) {
    if (_user != value) {
      _user = value;
      resetTransport();
    }
  }

  String get password => _password;
  set password(String value) {
    if (_password != value) {
      _password = value;
      resetTransport();
    }
  }

  late final http.Client _client;
  final LanResolver resolver;
  final PakarBleTransport bluetooth;
  final PakarUsbTransport usb;
  bool get isUsb => connectionMethod == ConnectionMethod.usb;
  int _usbDemandFlags = 1;
  int _usbPixelConsumers = 0;
  int get _effectiveUsbDemand =>
      _usbDemandFlags | (_usbPixelConsumers > 0 ? 8 : 0);
  bool? _sentPixelDemand;
  bool get _wantsNetworkPixels =>
      connectionWanted &&
      appForeground &&
      !isUsb &&
      !isBluetooth &&
      (_effectiveUsbDemand & 8) != 0;
  void _syncPixelSubscription() {
    if (!_socketReady ||
        _channel == null ||
        _verifiedBoard == null ||
        _verifiedBoot == null) {
      return;
    }
    final enabled = _wantsNetworkPixels;
    if (_sentPixelDemand == enabled) return;
    if (_sentPixelDemand == null && !enabled) {
      _sentPixelDemand = false;
      return; // New sockets default to no optional pixel traffic.
    }
    // Optional preview demand never dispatches a hardware command or HTTP read.
    _channel!.sink.add(
      jsonEncode({'type': 'pixel-subscribe', 'enabled': enabled}),
    );
    _sentPixelDemand = enabled;
  }

  void Function() retainUsbPixels() {
    ++_usbPixelConsumers;
    usb.demand(_effectiveUsbDemand, foreground: appForeground);
    _syncPixelSubscription();
    var active = true;
    return () {
      if (!active) return;
      active = false;
      --_usbPixelConsumers;
      usb.demand(_effectiveUsbDemand, foreground: appForeground);
      _syncPixelSubscription();
    };
  }

  /// Visible UI consumers own their optional observations; no animation push
  /// starts merely because a USB handle is open. Board leases expire in 3s.
  void setUsbObservations({
    bool panel = false,
    bool peripheral = false,
    bool pixels = false,
  }) {
    _usbDemandFlags =
        1 | (panel ? 2 : 0) | (peripheral ? 4 : 0) | (pixels ? 8 : 0);
    usb.demand(_effectiveUsbDemand, foreground: appForeground);
    _syncPixelSubscription();
  }

  String expectedBluetoothBoard = '';
  bool get isBluetooth => connectionMethod == ConnectionMethod.bluetooth;
  bool get supportsBinaryFirmware =>
      connectionMethod == ConnectionMethod.network;
  bool supportsRoute(String method, String path) {
    if (isUsb) {
      if (_latestStatus?.peer.has('network-tools') == true &&
          (method == 'GET'
                  ? const {
                      '/api/network/tools',
                      '/api/time',
                      '/api/wifi/scan',
                      '/api/network',
                    }
                  : const {
                      '/api/network/tools',
                      '/api/network/probe',
                      '/api/network',
                    })
              .contains(path)) {
        return true;
      }
      return method == 'GET'
          ? {'/api/status', '/api/cells/mapping'}.contains(path)
          : {
                  '/api/cell',
                  '/api/cells',
                  '/api/cells/assignments',
                  '/api/cells/lock',
                  '/api/mode',
                  '/api/effect',
                  '/api/brightness',
                  '/api/led/output',
                  '/api/audio',
                  '/api/display',
                }.contains(path) ||
                path == '/api/led/diagnostic' &&
                    _latestStatus?.peer.has('usb-pixel-diagnostic') == true;
    }
    if (!isBluetooth) return connectionMethod == ConnectionMethod.network;
    final ble = _latestStatus?.json['ble'];
    return ble is Map &&
        ble['routes'] is List &&
        (ble['routes'] as List).contains('$method $path');
  }

  late final HttpClient _ioClient = resolver.createClient()
    ..userAgent = PeerIdentity.current.userAgent;
  final Future<Set<String>> Function()? discovery;
  final Duration readTimeout;
  final bool enforcePeerVersions;
  UnsupportedPeerVersion? _unsupportedPeer;
  UnsupportedPeerVersion? get unsupportedPeer => _unsupportedPeer;
  Map<String, String> receivedServerHeaders = const {};
  String? _introducedPeer;
  String? get selectedRouteLocalIp =>
      _verifiedEndpoint == baseUrl &&
          _verifiedBoard != null &&
          _verifiedBoot != null &&
          connectionMethod == ConnectionMethod.network
      ? resolver.localAddressFor(Uri.parse(transportUrl))
      : null;
  Stopwatch? _introductionAge;
  final _readAborts = <Completer<void>>{};
  WebSocketChannel? _channel;
  bool _socketReady = false;
  DeviceStatus? _latestStatus;
  final _statusUpdates = StreamController<DeviceStatus>.broadcast(sync: true);
  Stream<DeviceStatus> get statusUpdates => _statusUpdates.stream;
  // RGB-only observations never drive full-status consumers/firmware policy.
  final _rgbUpdates = StreamController<DeviceStatus>.broadcast(sync: true);
  Stream<DeviceStatus> get rgbUpdates => _rgbUpdates.stream;
  final _pixelUpdates = StreamController<DeviceStatus>.broadcast(sync: true);
  Stream<DeviceStatus> get pixelUpdates => _pixelUpdates.stream;
  DeviceStatus? get latestStatus => _latestStatus;
  Duration? get statusObservationAge => _statusAge?.elapsed;

  /// Current full status authority, independent of a route's last rebuild or
  /// the diagnostic value's revision. RGB/progress-only events do not renew it.
  DeviceStatus? get freshStatus {
    final value = _latestStatus, age = statusObservationAge;
    if (!_connectionWanted ||
        !_appForeground ||
        _unsupportedPeer != null ||
        value == null ||
        age == null ||
        age >= const Duration(seconds: 6) ||
        _verifiedEndpoint != baseUrl ||
        _verifiedBoard == null ||
        _verifiedBoot == null ||
        value.json['board_id'] != _verifiedBoard ||
        value.json['boot_id'] != _verifiedBoot) {
      return null;
    }
    return value;
  }

  Stopwatch? _statusAge;
  Stopwatch? _liveAge, _socketAge;
  Duration? get liveSilence => _liveAge?.elapsed ?? _socketAge?.elapsed;
  final _commandReplies = <String, Completer<Map<String, dynamic>?>>{};
  bool get liveFresh =>
      (isUsb
          ? usb.ready
          : isBluetooth
          ? bluetooth.ready
          : _socketReady) &&
      _liveAge != null &&
      _liveAge!.elapsedMilliseconds < 4000;

  Uri get webSocketUri {
    final source = Uri.parse(transportUrl);
    return source.replace(
      scheme: source.scheme == 'https' ? 'wss' : 'ws',
      // Dart does not infer a default port for ws/wss when replacing schemes.
      port: source.port,
      path: '/ws',
      query: null,
    );
  }

  /// A resume probe is a read on the existing socket, not a new connection.
  bool requestLiveStatus() {
    if (isUsb) {
      if (!connectionWanted || !appForeground || !usb.ready || usb.busy) {
        return false;
      }
      unawaited(status().then<void>((_) {}, onError: (Object _) {}));
      return true;
    }
    if (isBluetooth) {
      if (!_connectionWanted ||
          !bluetooth.ready ||
          bluetooth.busy ||
          !appForeground) {
        return false;
      }
      unawaited(status().then<void>((_) {}, onError: (Object _) {}));
      return true;
    }
    if (!_connectionWanted || !_socketReady || _channel == null) return false;
    _channel!.sink.add('{"type":"status"}');
    return true;
  }

  bool _diagnosticPending = false;

  /// One deliberate full-frame commit on the selected peer. Prefer its atomic
  /// acknowledged command; older firmware uses the existing raw socket path.
  /// No reconnect, UDP substitution, delayed queue or new-intent replay.
  Future<DeviceStatus> pixelDiagnostic(
    PixelProbe probe, {
    bool off = false,
    required bool Function() authorize,
  }) async {
    if (isUsb) {
      final current = freshStatus;
      if (current == null) throw const StaleConnection();
      final actual = await _usbCommand(
        '/api/led/diagnostic',
        probe.commandFields(current, off: off),
        authorize,
      );
      return actual;
    }
    _requireNetwork();
    if (_diagnosticPending) {
      throw const FormatException('A diagnostic frame is still pending');
    }
    final revision = connectionRevision;
    final board = _verifiedBoard, boot = _verifiedBoot;
    void admit() {
      _requireNetwork();
      if (board == null ||
          boot == null ||
          !appForeground ||
          revision != connectionRevision ||
          _verifiedBoard != board ||
          _verifiedBoot != boot ||
          !authorize()) {
        throw const StaleConnection();
      }
    }

    _diagnosticPending = true;
    try {
      final frame = probe.extended ? null : probe.frame(off: off);
      admit();
      var current = await _commandStatus(authorize);
      admit();
      if (!supportsPixelDiagnostic(current) ||
          current.json['programming_latch'] == true ||
          (current.json['remote_firmware'] as Map?)?['busy'] == true) {
        throw const FormatException(
          'Pixel diagnostics unavailable during programming or updates',
        );
      }
      if (current.peer.has('pixel-diagnostic')) {
        // Board owns clear/stage/show in one transaction. Existing request IDs
        // allow lost WS receipts to recover over HTTP without a second intent.
        final fields = probe.commandFields(current, off: off);
        final priorRevision = PixelDiagnosticObservation.read(current)
            ?.revision;
        final received = await fastCommand(
          '/api/led/diagnostic',
          fields,
          authorize: authorize,
        );
        admit();
        if (received == null) {
          throw const FormatException(
            'The board advertises pixel diagnostics but did not acknowledge the command contract',
          );
        }
        DeviceStatus actual = received;
        if (current.peer.has('pixel-group-layout') ||
            current.peer.has('pixel-diagnostic-effects') ||
            current.peer.has('pixel-diagnostic-overlay')) {
          // The board renders effects and preserves the underlying scene. A
          // static RGB comparison cannot verify a moving phase or overlay.
          final clock = Stopwatch()..start();
          while (!matchesDiagnosticObservation(
                actual,
                probe,
                off: off,
                afterRevision: priorRevision,
              ) &&
              clock.elapsed < const Duration(seconds: 3)) {
            admit();
            final observed = PixelDiagnosticObservation.read(actual);
            if (observed != null &&
                (priorRevision == null || observed.revision > priorRevision) &&
                (observed.probe.first != probe.first ||
                    observed.probe.count != probe.count ||
                    !observed.probe
                        .copyWith(index: probe.index, group: probe.group)
                        .sameAs(probe) ||
                    observed.off != off)) {
              throw const OperationUncertain(
                'Another diagnostic replaced this request before readback. No frame was replayed.',
              );
            }
            await Future<void>.delayed(const Duration(milliseconds: 80));
            admit();
            if (_socketReady && _channel != null) {
              requestLiveStatus();
              actual = _latestStatus ?? actual;
            } else if (commandHttpFallback || isBluetooth) {
              actual = DeviceStatus(await _get('/api/status?pixels=1'));
              admit();
              _acceptIdentity(actual.json);
            }
          }
          admit();
          if (!matchesDiagnosticObservation(
            actual,
            probe,
            off: off,
            afterRevision: priorRevision,
          )) {
            throw const OperationUncertain(
              'Diagnostic settings were accepted but completed board output was not confirmed. No frame was replayed.',
            );
          }
          _rememberStatus(actual);
          return actual;
        }
        if (!diagnosticReady(actual)) {
          throw const OperationUncertain(
            'The board left diagnostic output mode before readback.',
          );
        }
        final pixels = actual.json['pixel_frame'];
        if (pixels is! Map) {
          throw const OperationUncertain(
            'The board accepted the command but returned no pixel frame.',
          );
        }
        if (pixels['pending'] == true) {
          throw const OperationUncertain(
            'The LED frame is still pending; completed output has not been observed.',
          );
        }
        if (!matchesDiagnosticFrame(actual, frame!)) {
          throw const OperationUncertain(
            'The board pixel readback differs from the requested frame.',
          );
        }
        return actual;
      }
      if (probe.extended) {
        throw const FormatException(
          'Diagnostic effects and overlays require acknowledged board support; no frame was sent.',
        );
      }
      if (!_socketReady || _channel == null) {
        throw const FormatException(
          'This firmware needs WebSocket streaming for diagnostics. Update the board for acknowledged HTTP fallback.',
        );
      }
      Future<void> configure(String path, Map<String, String> values) async {
        admit();
        current =
            await fastCommand(path, values, authorize: authorize) ??
            (() => throw const FormatException(
              'Acknowledged commands required for diagnostics',
            ))();
        admit();
      }

      if (current.mode != 'cell') {
        await configure('/api/mode', {'value': 'cell'});
      }
      if (current.effect != 'none') {
        await configure('/api/effect', {'value': 'none'});
      }
      if (current.ledOutputEnabled != true ||
          current.ledOutputMode != 'effects') {
        await configure('/api/led/output', {
          'enabled': 'true',
          'mode': 'effects',
        });
      }
      // Correlated command replies contain actual same-boot output state.
      // Reuse that or the fresh WS snapshot instead of three HTTP round trips.
      var prior =
          (current.json['pixel_frame'] as Map?)?['revision'] ??
          ((current.json['led_output'] as Map?)?['driver']
              as Map?)?['revision'];
      if (prior is! int) {
        current = DeviceStatus(await _get('/api/status?pixels=1'));
        admit();
        _acceptIdentity(current.json);
        admit();
        prior = (current.json['pixel_frame'] as Map?)?['revision'];
      }
      if (!diagnosticReady(current)) {
        throw const FormatException('Diagnostic output mode not acknowledged');
      }
      final socket = _channel!;
      for (final packet in diagnosticPackets(frame!)) {
        admit();
        if (!identical(socket, _channel)) throw const StaleConnection();
        socket.sink.add(packet);
      }
      final clock = Stopwatch()..start();
      do {
        final actual = DeviceStatus(await _get('/api/status?pixels=1'));
        admit();
        _acceptIdentity(actual.json);
        admit();
        if (!diagnosticReady(actual)) {
          throw const OperationUncertain(
            'The board left diagnostic output mode before readback.',
          );
        }
        final next = (actual.json['pixel_frame'] as Map?)?['revision'];
        if (next != prior && matchesDiagnosticFrame(actual, frame)) {
          _rememberStatus(actual);
          return actual;
        }
        await Future<void>.delayed(const Duration(milliseconds: 50));
      } while (clock.elapsed < const Duration(seconds: 8));
      throw const OperationUncertain(
        'Timed out waiting for the requested pixel frame to be published.',
      );
    } finally {
      _diagnosticPending = false;
    }
  }

  bool _rememberStatus(DeviceStatus status) {
    _checkPeerVersion(status.json);
    if (status.olderThan(_latestStatus)) return false;
    status = status.preservingNewerUpgrade(_latestStatus);
    status = status.preservingNewerRgb(_latestStatus);
    status = status.preservingNewerPixels(_latestStatus);
    _latestStatus = status;
    _statusAge = Stopwatch()..start();
    if (!_statusUpdates.isClosed) _statusUpdates.add(status);
    _introducePeer();
    return true;
  }

  void _checkPeerVersion(Map<String, dynamic> value) {
    try {
      validatePeerBoard(
        value,
        appVersion: PeerIdentity.current.version,
        enabled: enforcePeerVersions && peerVersionChecksEnabled,
      );
    } on UnsupportedPeerVersion catch (error) {
      // Revoke every captured command/maintenance owner synchronously, not
      // just its status lease. Keep this one persistent rejection until an
      // explicit reconnect/endpoint edit, with no automatic retry chatter.
      if (_unsupportedPeer == null) {
        _connectionWanted =
            false; // Rejected, not an endlessly reconnecting link.
        resetTransport(unsupported: error);
      }
      rethrow;
    }
  }

  // One optional, nonsecret introduction per verified board boot, refreshed
  // at most once a minute on normal foreground observations for the board's
  // 120s registry lease. No timer, per-frame metadata or blocking readback.
  void _introducePeer() {
    // The USB actor has already completed its correlated HELLO and peer-report
    // chunks before publishing the first snapshot. Never use the HTTP report
    // fallback (or advertise its periodic refresh) for a physical USB owner.
    if (isUsb) return;
    if (_verifiedBoard == null ||
        _verifiedBoot == null ||
        _latestStatus == null ||
        !appForeground ||
        _latestStatus!.json['compatibility'] is! Map) {
      return;
    }
    final owner = '$_verifiedBoard/$_verifiedBoot/$connectionRevision';
    if (_introducedPeer == owner &&
        (_introductionAge?.elapsed ?? Duration.zero) <
            const Duration(seconds: 60)) {
      return;
    }
    _introducedPeer = owner;
    _introductionAge = Stopwatch()..start();
    final revision = connectionRevision;
    unawaited(() async {
      try {
        final report = PeerIdentity.current.reportJson(
          transport: isBluetooth ? 'bluetooth' : 'network',
          enforce: enforcePeerVersions && peerVersionChecksEnabled,
          reportedIp: selectedRouteLocalIp,
        );
        if (revision != connectionRevision) return;
        if (isBluetooth) {
          // Sequential/ACKed transport, no alternate network path.
          await bluetooth.request(
            'POST',
            '/api/clients/report',
            {'report': report},
            board: _verifiedBoard!,
            boot: _verifiedBoot!,
            expires: 0, // Metadata report, never an actuator command.
          );
        } else if (_socketReady &&
            _channel != null &&
            utf8.encode(report).length <= 1200) {
          _channel!.sink.add(
            jsonEncode({
              'type': 'peer-introduction',
              'report': jsonDecode(report),
            }),
          );
        } else {
          final request =
              http.Request('POST', _uri(transportUrl, '/api/clients/report'))
                ..headers.addAll(_writeHeaders)
                ..followRedirects = false
                ..bodyFields = {'report': report};
          final response = await _client.send(request).timeout(readTimeout);
          await response.stream.listen((_) {}).cancel();
        }
      } on Object {
        // Metadata failure does not become a control timeout or reconnect toast.
      }
    }());
  }

  /// A single intent, preferred WS receipt, then the SAME idempotent envelope
  /// over HTTP if its reply is lost. Never fall back to a second legacy write.
  Future<DeviceStatus?> fastCommand(
    String path,
    Map<String, String> values, {
    required bool Function() authorize,
  }) => _fastCommand(path, values, authorize: authorize);

  Future<DeviceStatus?> _fastCommand(
    String path,
    Map<String, String> values, {
    required bool Function() authorize,
    void Function(Map<String, dynamic>)? receipt,
  }) async {
    if (!commandNames.containsKey(path)) return null;
    // Preserve the existing dedicated USB cue path. These new WS commands
    // require the AVR-correlated envelope, not the table-only USB opcode switch.
    if (isUsb &&
        const ['/api/audio', '/api/buzzer', '/api/outputs'].contains(path)) {
      return null;
    }
    if (isUsb) return _usbCommand(path, values, authorize);
    var snapshot = _latestStatus;
    if (snapshot?.peer.has('command-ack') != true) {
      if (path.startsWith('/api/minigame/')) {
        throw const FormatException('Minigame command receipts unavailable');
      }
      return null;
    }
    if (const ['/api/audio', '/api/buzzer', '/api/outputs'].contains(path) &&
        snapshot?.peer.has('avr-command-ack') != true) {
      return null;
    }
    if (path == '/api/game/reset' && snapshot?.peer.has('reset-ack') != true) {
      return null;
    }
    final revision = connectionRevision;
    final allowHttp = commandHttpFallback;
    void requireSelectedCommandPath() {
      if (!isBluetooth && !allowHttp && (!_socketReady || _channel == null)) {
        throw const FormatException(
          'WebSocket command connection is unavailable. HTTP command fallback is disabled; no command was sent.',
        );
      }
    }

    _requireNetwork();
    requireSelectedCommandPath();
    snapshot = await _commandStatus(authorize);
    _requireNetwork();
    if (revision != connectionRevision || !authorize()) {
      throw const StaleConnection();
    }
    if (isBluetooth) {
      final clock = await _freshBluetoothClock();
      snapshot = DeviceStatus({...snapshot.json, 'command_clock_ms': clock});
      if (revision != connectionRevision || !authorize()) {
        throw const StaleConnection();
      }
    }
    final packet = commandPacket(snapshot, path, values);
    final id = (jsonDecode(packet) as Map)['request_id'] as String;
    Map<String, dynamic>? reply;
    if (isBluetooth) {
      _retireStatusRead();
      try {
        reply = await _bleRequest('POST', '/api/command', {
          'packet': packet,
        }, authorize: authorize);
      } finally {
        _retireStatusRead();
      }
      if (revision != connectionRevision || !authorize()) {
        throw const StaleConnection();
      }
      final actual = await _settleAppliedReply(
        reply,
        packet,
        path,
        values,
        revision,
        authorize,
      );
      _acceptIdentity(actual.json);
      _rememberStatus(actual);
      receipt?.call(reply);
      return actual;
    }
    Completer<Map<String, dynamic>?>? pending;
    try {
      requireSelectedCommandPath();
      if (_socketReady && _channel != null) {
        pending = Completer<Map<String, dynamic>?>();
        _commandReplies[id] = pending;
        try {
          _retireStatusRead();
          _channel!.sink.add(packet);
          reply = await pending.future.timeout(
            const Duration(milliseconds: 800),
            onTimeout: () => null,
          );
        } on Exception {
          /* Same-ID HTTP recovery below; no second intent. */
        } finally {
          _retireStatusRead();
        }
      }
      if (revision != connectionRevision || !authorize()) {
        throw const StaleConnection();
      }
      if (reply == null && !allowHttp) {
        if (pending == null) {
          throw const FormatException(
            'WebSocket command connection is unavailable. HTTP command fallback is disabled; no command was sent.',
          );
        }
        try {
          // Keep the original combined 800ms preferred + four-second recovery
          // budget, but wait only for this WS intent instead of sending HTTP.
          reply = await pending.future.timeout(const Duration(seconds: 4));
        } on TimeoutException {
          throw const OperationUncertain(
            'The board did not confirm this WebSocket command within 4.8 seconds. It may have been applied; refresh board state before trying again. HTTP command fallback is disabled.',
          );
        }
      } else if (reply == null) {
        final request = http.Request('POST', _uri(transportUrl, '/api/command'))
          ..headers.addAll(_writeHeaders)
          ..bodyFields = {'packet': packet}
          ..followRedirects = false;
        _retireStatusRead();
        final cancel = Completer<void>();
        Future<Map<String, dynamic>> receipt() async {
          final response = await _writeResponse(
            request,
            const Duration(seconds: 4),
            cancel: cancel.future,
          );
          _rejectWriteResponse(response);
          try {
            final decoded = jsonDecode(response.body);
            if (decoded is! Map<String, dynamic>) {
              throw const OperationUncertain(
                'The board returned a malformed command receipt.',
              );
            }
            return decoded;
          } on FormatException {
            throw const OperationUncertain(
              'The board returned a malformed command receipt.',
            );
          }
        }

        try {
          // The same request ID remains registered throughout HTTP recovery.
          // A delayed valid WS receipt can settle it and cancel only this HTTP
          // request; Future.any observes the losing transport's eventual error.
          reply = pending == null
              ? await receipt()
              : await Future.any([receipt(), pending.future]);
        } on TimeoutException {
          throw const OperationUncertain(
            'The board did not confirm this command within 4 seconds. It may have been applied; refresh board state before trying again.',
          );
        } on SocketException {
          throw const OperationUncertain(
            'The connection was lost while waiting for a command receipt.',
          );
        } on http.ClientException {
          throw const OperationUncertain(
            'The connection was lost while waiting for a command receipt.',
          );
        } finally {
          if (!cancel.isCompleted) cancel.complete();
          _retireStatusRead();
        }
      }
      if (revision != connectionRevision || !authorize()) {
        throw const StaleConnection();
      }
      if (reply == null) {
        throw const OperationUncertain(
          'The command owner retired before a receipt arrived.',
        );
      }
      final actual = await _settleAppliedReply(
        reply,
        packet,
        path,
        values,
        revision,
        authorize,
      );
      _acceptIdentity(actual.json);
      _rememberStatus(actual);
      receipt?.call(reply);
      return actual;
    } finally {
      if (pending != null) {
        _commandReplies.remove(id);
        if (!pending.isCompleted) pending.complete(null);
      }
    }
  }

  Future<DeviceStatus> _settleAppliedReply(
    Map<String, dynamic> reply,
    String packet,
    String path,
    Map<String, String> values,
    int revision,
    bool Function() authorize,
  ) async {
    final melodies = path == '/api/game/settings' && gameMelodyPatch(values);
    final backend = path == '/api/led/output' && values.containsKey('backend');
    final receipt = verifyCommandReply(
      reply,
      packet,
      path,
      values,
      requireGameReadback: !melodies,
    );
    if ((!melodies && !backend) ||
        commandReadbackMatches(receipt, path, values)) {
      return receipt;
    }
    void admit() {
      if (revision != connectionRevision ||
          !connectionWanted ||
          !authorize() ||
          _verifiedBoard != receipt.json['board_id'] ||
          _verifiedBoot != receipt.json['boot_id']) {
        throw const StaleConnection();
      }
    }

    final completed = Completer<DeviceStatus>();
    void observe(DeviceStatus value) {
      if (completed.isCompleted) return;
      try {
        admit();
        if (value.json['board_id'] != receipt.json['board_id'] ||
            value.json['boot_id'] != receipt.json['boot_id']) {
          throw const StaleConnection();
        }
        if (backend && value.olderThan(receipt)) return;
        if (backend) {
          final driver = LedBackendState.fromStatus(value.json);
          if (driver?.failed == true) {
            throw FormatException(driver!.failureDescription);
          }
        }
        if (commandReadbackMatches(value, path, values)) {
          completed.complete(value);
        }
      } on Object catch (error, stack) {
        completed.completeError(error, stack);
      }
    }

    final updates = statusUpdates.listen(observe);
    final connection = connectionUpdates.listen((_) {
      if (completed.isCompleted) return;
      try {
        admit();
      } on Object catch (error, stack) {
        completed.completeError(error, stack);
      }
    });
    final clock = Stopwatch()..start();
    try {
      admit();
      // Publish the accepted handoff, not an optimistic active driver. This
      // also retires any previous failed attempt's status before observing it.
      if (backend) _rememberStatus(receipt);
      if (_latestStatus case final current?) observe(current);
      while (!completed.isCompleted && clock.elapsedMilliseconds < 3000) {
        // Only read requests after the one correlated mutation. A late AVR ACK
        // or LED driver handoff is not a reason to replay the user's intent.
        if (_socketReady && _channel != null) {
          requestLiveStatus();
        } else if (isBluetooth || commandHttpFallback) {
          _retireStatusRead();
          try {
            observe(await status().timeout(const Duration(milliseconds: 500)));
          } on TimeoutException {
            /* Keep the original bounded receipt wait. */
          }
        }
        if (completed.isCompleted) break;
        await Future.any<void>([
          completed.future.then<void>((_) {}),
          Future<void>.delayed(const Duration(milliseconds: 100)),
        ]);
        admit();
      }
      if (!completed.isCompleted) {
        throw OperationUncertain(
          backend
              ? 'The LED transmitter change was accepted, but the active driver was not confirmed; no command was replayed.'
              : 'Game melodies were saved, but AVR application was not confirmed; no command was replayed.',
        );
      }
      final actual = await completed.future;
      admit();
      return verifyCommandReply(
        {...reply, 'status': actual.json},
        packet,
        path,
        values,
      );
    } finally {
      await updates.cancel();
      await connection.cancel();
    }
  }

  /// A current full WS observation can satisfy command admission while an
  /// older background GET is stalled. Preserve the same two-second clock and
  /// peer/owner checks; neither source may revive a retired command owner.
  Future<DeviceStatus> _commandStatus(bool Function() authorize) async {
    final owner = connectionRevision,
        board = _verifiedBoard,
        boot = _verifiedBoot;
    void admit() {
      _requireNetwork();
      if (owner != connectionRevision ||
          !appForeground ||
          !authorize() ||
          board != _verifiedBoard ||
          boot != _verifiedBoot) {
        throw const StaleConnection();
      }
    }

    bool fresh() =>
        _latestStatus != null &&
        _statusAge != null &&
        _statusAge!.elapsedMilliseconds < 2000;
    admit();
    if (fresh()) return _latestStatus!;
    final result = Completer<DeviceStatus>();
    final updates = statusUpdates.listen((value) {
      if (result.isCompleted || !_socketReady || !fresh()) return;
      try {
        admit();
        // Retire only the joined status read. Its ignored result remains
        // handled below, and its real HTTP socket is aborted before control.
        final abort = _statusReadAbort;
        _retireStatusRead();
        if (abort != null && !abort.isCompleted) abort.complete();
        result.complete(_latestStatus!);
      } on Object catch (error, stack) {
        result.completeError(error, stack);
      }
    });
    final connection = connectionUpdates.listen((_) {
      if (result.isCompleted) return;
      try {
        admit();
      } on Object catch (error, stack) {
        result.completeError(error, stack);
      }
    });
    unawaited(
      settledStatus(stillNeeded: () => !result.isCompleted).then<void>(
        (value) {
          if (!result.isCompleted) result.complete(value);
        },
        onError: (Object error, StackTrace stack) {
          if (!result.isCompleted) {
            result.completeError(
              error is TimeoutException
                  ? TimeoutException(
                      'Fresh board status unavailable; command was not sent.',
                      readTimeout,
                    )
                  : error,
              stack,
            );
          }
        },
      ),
    );
    requestLiveStatus();
    var admitted = false;
    try {
      final value = await result.future.timeout(
        readTimeout,
        onTimeout: () {
          throw TimeoutException(
            'Fresh board status unavailable; command was not sent.',
            readTimeout,
          );
        },
      );
      admit();
      admitted = true;
      return value;
    } finally {
      await updates.cancel();
      await connection.cancel();
      if (!admitted && owner == connectionRevision) {
        final abort = _statusReadAbort;
        _retireStatusRead();
        if (abort != null && !abort.isCompleted) abort.complete();
      }
    }
  }

  /// One deadline owns both connection/headers and body. Aborting the socket
  /// after that deadline never implies the board rolled a command back.
  Future<http.Response> _writeResponse(
    http.Request source,
    Duration limit, {
    Future<void>? cancel,
  }) async {
    if (isUsb) {
      throw const FormatException(
        'HTTP transport is disabled while USB is selected',
      );
    }
    final abort = Completer<void>();
    final request =
        http.AbortableRequest(
            source.method,
            source.url,
            abortTrigger: abort.future,
          )
          ..headers.addAll(source.headers)
          ..followRedirects = source.followRedirects
          ..bodyBytes = source.bodyBytes;
    final elapsed = Stopwatch()..start();
    var timedOut = false;
    void stop() {
      if (!abort.isCompleted) abort.complete();
    }

    if (cancel != null) {
      unawaited(cancel.then((_) => stop(), onError: (Object _) => stop()));
    }
    final timer = Timer(limit, () {
      timedOut = true;
      stop();
    });
    StreamIterator<List<int>>? reader;
    try {
      final response = await _client.send(request).timeout(limit);
      reader = StreamIterator(response.stream);
      final bytes = BytesBuilder(copy: false);
      while (true) {
        final remaining = limit - elapsed.elapsed;
        if (remaining <= Duration.zero) {
          throw TimeoutException('Command receipt deadline');
        }
        if (!await reader.moveNext().timeout(remaining)) break;
        if (bytes.length + reader.current.length > 64 * 1024) {
          throw const OperationUncertain(
            'The command receipt exceeds its size limit.',
          );
        }
        bytes.add(reader.current);
      }
      return http.Response.bytes(
        bytes.takeBytes(),
        response.statusCode,
        headers: response.headers,
        request: request,
        isRedirect: response.isRedirect,
        persistentConnection: response.persistentConnection,
        reasonPhrase: response.reasonPhrase,
      );
    } on http.RequestAbortedException {
      if (timedOut) throw TimeoutException('Command receipt deadline');
      rethrow;
    } finally {
      timer.cancel();
      stop();
      try {
        await reader?.cancel().timeout(const Duration(milliseconds: 500));
      } on Object {
        /* Cleanup cannot keep a timed-out command pending. */
      }
    }
  }

  String? _discoveredBaseUrl;
  String? _transportOwner;
  String? _verifiedEndpoint;
  String? _verifiedBoard, _verifiedBoot;
  String? _knownNetworkEndpoint, _knownNetworkBoard, _lastVerifiedNetworkOrigin;

  /// Saved route metadata is only a read-probe hint, never peer authority.
  void restoreNetworkRecovery(Object? saved) {
    if (saved is! Map ||
        saved['endpoint'] != baseUrl ||
        saved['board_id'] is! String ||
        !RegExp(r'^[!-~]{1,80}$').hasMatch(saved['board_id'])) {
      return;
    }
    try {
      final origin = normalizeEndpoint('${saved['origin']}');
      final uri = Uri.parse(origin), selected = Uri.parse(baseUrl);
      if (InternetAddress.tryParse(uri.host) == null ||
          uri.scheme != selected.scheme ||
          uri.port != selected.port) {
        return;
      }
      _knownNetworkEndpoint = baseUrl;
      _knownNetworkBoard = saved['board_id'];
      _lastVerifiedNetworkOrigin = origin;
    } on FormatException {
      /* Invalid metadata never changes selected intent. */
    }
  }

  Map<String, String>? get networkRecovery =>
      _knownNetworkEndpoint == baseUrl &&
          _knownNetworkBoard != null &&
          _lastVerifiedNetworkOrigin != null
      ? {
          'endpoint': baseUrl,
          'board_id': _knownNetworkBoard!,
          'origin': _lastVerifiedNetworkOrigin!,
        }
      : null;

  void _rememberNetworkRoute(Map<String, dynamic> value) {
    if (isBluetooth || _verifiedBoard == null || _verifiedBoot == null) return;
    final uri = Uri.parse(transportUrl);
    final ip = resolver.connectedAddressFor(uri);
    final hostname = value['hostname'];
    if (_knownNetworkEndpoint == baseUrl &&
        _knownNetworkBoard == _verifiedBoard &&
        _localLabel() != null &&
        hostname is String &&
        RegExp(r'^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$').hasMatch(hostname) &&
        hostname != _localLabel()) {
      // A same-board verified rename updates the canonical label, retaining
      // the current numeric route for this session instead of resolving per tick.
      _baseUrl = Uri.parse(baseUrl).replace(host: '$hostname.local').toString();
      _transportOwner = baseUrl;
      _discoveredBaseUrl = ip == null ? null : uri.replace(host: ip).toString();
    }
    _knownNetworkEndpoint = baseUrl;
    _knownNetworkBoard = _verifiedBoard;
    if (ip != null) {
      _lastVerifiedNetworkOrigin = uri.replace(host: ip).toString();
    }
  }

  Map<String, String> get _headers => {
    ...PeerIdentity.current.headers,
    if (password.isNotEmpty)
      'authorization': 'Basic ${base64Encode(utf8.encode('$user:$password'))}',
  };
  Map<String, String> get _writeHeaders => {
    ..._headers,
    if (_verifiedBoard != null && _verifiedBoot != null) ...{
      'X-Pakar-Board': _verifiedBoard!,
      'X-Pakar-Boot': _verifiedBoot!,
    },
  };

  String get transportUrl =>
      _transportOwner == baseUrl ? _discoveredBaseUrl ?? baseUrl : baseUrl;

  static String normalizeEndpoint(String value) {
    final text = value.trim();
    if (text.startsWith('usb://')) {
      final selected = PakarUsbTransport.address(text);
      return PakarUsbTransport.endpoint(selected.$1, selected.$2);
    }
    if (text.startsWith('ble://')) {
      return PakarBleTransport.endpoint(
        PakarBleTransport.addressFromEndpoint(text),
      );
    }
    final uri = Uri.tryParse(text.contains('://') ? text : 'http://$text');
    if (uri == null ||
        !['http', 'https'].contains(uri.scheme) ||
        uri.host.isEmpty ||
        uri.userInfo.isNotEmpty ||
        uri.hasQuery ||
        uri.hasFragment ||
        (uri.path.isNotEmpty && uri.path != '/')) {
      throw const FormatException(
        'Enter an HTTP(S) device hostname or origin without credentials or a path',
      );
    }
    return uri.replace(path: '').toString();
  }

  Uri _uri(String base, String path, [Map<String, String>? query]) =>
      Uri.parse('${base.replaceFirst(RegExp(r'/$'), '')}$path')
          .replace(queryParameters: query);

  Future<http.Response> _readResponse(
    String base,
    String path,
    Map<String, String> headers, {
    Duration? timeout,
    http.Client? readClient,
    bool recordHeaders = true,
  }) async {
    if (isUsb) {
      throw const FormatException(
        'HTTP transport is disabled while USB is selected',
      );
    }
    final owner = connectionRevision, limit = timeout ?? readTimeout;
    const maximum = 64 * 1024;
    final abort = Completer<void>();
    _readAborts.add(abort);
    if (Uri.parse(path).path == '/api/status') _statusReadAbort = abort;
    var timedOut = false;
    void stop() {
      if (!abort.isCompleted) abort.complete();
    }

    final elapsed = Stopwatch()..start();
    final timer = Timer(limit, () {
      timedOut = true;
      stop();
    });
    final request =
        http.AbortableRequest(
            'GET',
            _uri(base, path),
            abortTrigger: abort.future,
          )
          ..headers.addAll(headers)
          ..followRedirects = false;
    StreamIterator<List<int>>? reader;
    StreamSubscription<List<int>>? rejectedBody;
    try {
      final response = await (readClient ?? _client)
          .send(request)
          .timeout(limit);
      if (recordHeaders && owner == connectionRevision) {
        receivedServerHeaders = Map.unmodifiable({
          for (final key in [
            'server',
            'x-pakar-product',
            'x-pakar-version',
            'x-pakar-build',
            'x-pakar-board-version',
            'x-pakar-instance',
          ])
            if (response.headers[key] != null) key: response.headers[key]!,
        });
      }
      if (owner != connectionRevision ||
          (response.contentLength ?? 0) > maximum) {
        // StreamIterator subscribes lazily at moveNext. The installed IOClient
        // attaches its response-abort hook only when the body is listened to;
        // cancelling an untouched iterator leaves that HTTP socket open. Start
        // this discard listener synchronously before abort/cancel in finally.
        // Nothing is retained or decoded, and no unbounded drain is awaited.
        rejectedBody = response.stream.listen((_) {}, onError: (Object _) {});
      }
      if (owner != connectionRevision) throw const StaleConnection();
      if ((response.contentLength ?? 0) > maximum) {
        throw const FormatException(
          'Device JSON response exceeds its safety limit',
        );
      }
      reader = StreamIterator(response.stream);
      final bytes = BytesBuilder(copy: false);
      while (true) {
        final remaining = limit - elapsed.elapsed;
        if (remaining <= Duration.zero) {
          throw TimeoutException('Device read deadline');
        }
        if (!await reader.moveNext().timeout(remaining)) break;
        if (owner != connectionRevision) throw const StaleConnection();
        if (bytes.length + reader.current.length > maximum) {
          throw const FormatException(
            'Device JSON response exceeds its safety limit',
          );
        }
        bytes.add(reader.current);
      }
      return http.Response.bytes(
        bytes.takeBytes(),
        response.statusCode,
        headers: response.headers,
        request: request,
        isRedirect: response.isRedirect,
        persistentConnection: response.persistentConnection,
        reasonPhrase: response.reasonPhrase,
      );
    } on http.RequestAbortedException {
      if (owner != connectionRevision) throw const StaleConnection();
      if (timedOut) throw TimeoutException('Device read deadline');
      rethrow;
    } finally {
      timer.cancel();
      // Future.timeout alone leaves IO and body subscriptions alive. The
      // installed http IOClient supports AbortableRequest before/after headers.
      stop();
      try {
        await reader?.cancel().timeout(const Duration(milliseconds: 500));
        await rejectedBody?.cancel().timeout(const Duration(milliseconds: 500));
      } on Object {
        // An unresponsive old read cannot hold ownership retirement forever.
      }
      _readAborts.remove(abort);
      if (identical(_statusReadAbort, abort)) _statusReadAbort = null;
    }
  }

  void _abortReads() {
    for (final abort in _readAborts.toList(growable: false)) {
      if (!abort.isCompleted) abort.complete();
    }
  }

  Future<Map<String, dynamic>> _get(String path) async {
    _requireNetwork();
    final statusRead = Uri.parse(path).path == '/api/status';
    if (isUsb) {
      await _ensureUsb();
      if (statusRead) return (await usb.status()).json;
      if (path == '/api/cells/mapping') {
        await usb.status();
        return usb.table!;
      }
      if (path == '/api/pixels/groups') {
        final owner = connectionRevision;
        return usb.pixelGroupsApi(
          authorize: () => owner == connectionRevision && connectionWanted,
        );
      }
      if (path == '/api/pixels/corrections') {
        final owner = connectionRevision;
        return usb.pixelCorrectionsApi(
          authorize: () => owner == connectionRevision && connectionWanted,
        );
      }
      final uri = Uri.parse(path);
      if (const {
        '/api/network/tools',
        '/api/time',
        '/api/wifi/scan',
        '/api/network',
      }.contains(uri.path)) {
        final owner = connectionRevision;
        return usb.networkApi('GET', uri.path, {
          if (uri.queryParameters['results'] == 'true') 'results': true,
        }, authorize: () => owner == connectionRevision && connectionWanted);
      }
      throw const FormatException(
        'This read is unavailable through USB; no network fallback attempted',
      );
    }
    if (isBluetooth) return _bleRequest('GET', path, const {});
    if (!statusRead && _verifiedEndpoint != baseUrl && enforcePeerVersions) {
      await status();
      _requireNetwork();
    }
    final revision = connectionRevision, headers = _headers;
    final response = await _withDiscovery(
      (base) => _readResponse(base, path, headers),
    );
    if (revision != connectionRevision) throw const StaleConnection();
    if (response.statusCode == 401) throw const AuthenticationRequired();
    if (response.statusCode ~/ 100 != 2) {
      throw FormatException(
        'Device read rejected (HTTP ${response.statusCode})',
      );
    }
    return jsonDecode(response.body) as Map<String, dynamic>;
  }

  Future<Map<String, dynamic>> getJson(String path) => _get(path);
  Future<Map<String, dynamic>> peerExchange(
    String method,
    String path,
    Map<String, dynamic> body, {
    bool Function()? authorize,
  }) async {
    final revision = connectionRevision;
    bool admitted() =>
        revision == connectionRevision &&
        connectionWanted &&
        (authorize?.call() ?? true);
    if (isUsb) {
      await _ensureUsb();
      return usb.peerExchange(method, path, body, authorize: admitted);
    }
    if (method == 'GET') return getJson(path);
    const writes = {
      '/api/peers/discover',
      '/api/peers/policy',
      '/api/peers/update',
      '/api/firmware/apply',
      '/api/firmware/cancel',
    };
    if (method != 'POST' || !writes.contains(path)) {
      throw const FormatException('Unsupported peer operation');
    }
    final result = await postJson(
      path,
      path == '/api/peers/policy'
          ? {'settings': jsonEncode(body)}
          : {
              for (final entry in body.entries)
                entry.key: entry.value is Map
                    ? jsonEncode(entry.value)
                    : '${entry.value}',
            },
      authorize: admitted,
    );
    if (!admitted()) throw const StaleConnection();
    return result;
  }

  /// Same boot-fenced USB API or correlated WS-first command, with the existing
  /// user-configurable HTTP fallback. No independent network/command owner.
  Future<Map<String, dynamic>> networkToolsWrite(
    String path,
    Map<String, dynamic> body,
  ) async {
    final owner = connectionRevision;
    bool admitted() =>
        owner == connectionRevision &&
        connectionWanted &&
        _latestStatus?.peer.has('network-tools') == true;
    if (isUsb) {
      await _ensureUsb();
      final result = await usb.networkApi(
        'POST',
        path,
        body,
        authorize: admitted,
      );
      if (result['settings'] is Map && result['clock'] is Map) return result;
      return usb.networkApi(
        'GET',
        '/api/network/tools',
        const {},
        authorize: admitted,
      );
    }
    Map<String, dynamic>? receipt;
    final actual = await _fastCommand(
      path,
      {
        if (path == '/api/network/tools') 'settings': jsonEncode(body),
        if (path == '/api/network/probe') ...{
          'kind': '${body['kind']}',
          if (body.containsKey('target')) 'target': '${body['target']}',
        },
      },
      authorize: admitted,
      receipt: (value) => receipt = value,
    );
    if (actual == null || receipt?['network_tools'] is! Map<String, dynamic>) {
      throw const FormatException(
        'Network operation receipt unavailable; inspect before retrying',
      );
    }
    final result = receipt!['network_tools'] as Map<String, dynamic>;
    if (path == '/api/network/probe') {
      final job = result['job'];
      if (job is! Map ||
          job['kind'] != body['kind'] ||
          const {'dns', 'ping'}.contains(body['kind']) &&
              job['target'] != body['target']) {
        throw const OperationUncertain(
          'The current diagnostic belongs to another request; no probe was replayed',
        );
      }
    }
    if (path == '/api/network/tools') {
      final saved = result['settings'];
      if (saved is! Map ||
          !body.entries.every((entry) => saved[entry.key] == entry.value)) {
        throw const OperationUncertain(
          'Saved network tools differ from the requested settings',
        );
      }
    }
    return result;
  }

  /// Bounded single-dispatch maintenance transport. Even ISP probe uses this
  /// path: its GET resets AVR, so it must not be retried through discovery.
  Future<Uint8List> maintenanceBytes(
    String path, {
    bool post = false,
    bool mayMutate = false,
    Map<String, String> form = const {},
    Uint8List? upload,
    String? uploadField,
    int maximum = 8192,
    bool verifySketch = false,
    void Function(int done, int total)? uploadProgress,
    Duration timeout = const Duration(seconds: 180),
    required bool Function() authorize,
  }) async {
    _requireNetwork();
    if (isUsb) {
      throw const FormatException(
        'USB firmware/maintenance is not implemented; no HTTP, Wi-Fi or DNS fallback attempted',
      );
    }
    if (isBluetooth) {
      if (upload != null ||
          verifySketch ||
          !supportsRoute(post ? 'POST' : 'GET', Uri.parse(path).path)) {
        throw const FormatException(
          'This Bluetooth peer does not advertise binary firmware transfer or this maintenance route',
        );
      }
      if (!authorize()) throw const StaleConnection();
      final value = await _bleRequest(
        post ? 'POST' : 'GET',
        path,
        form,
        authorize: authorize,
      );
      return Uint8List.fromList(utf8.encode(jsonEncode(value)));
    }
    final revision = connectionRevision;
    if (_verifiedEndpoint != baseUrl) await status();
    if (revision != connectionRevision || !authorize()) {
      throw const StaleConnection();
    }
    final uri = _uri(transportUrl, path);
    final http.BaseRequest request;
    var uploaded = 0;
    if (upload != null) {
      if (!post || !{'image', 'firmware'}.contains(uploadField)) {
        throw const FormatException('Invalid firmware upload contract');
      }
      request = http.MultipartRequest('POST', uri)
        ..files.add(
          http.MultipartFile(
            uploadField!,
            Stream<List<int>>.fromIterable([
              for (var at = 0; at < upload.length; at += 16384)
                Uint8List.sublistView(
                  upload,
                  at,
                  (at + 16384).clamp(0, upload.length),
                ),
            ]).map((chunk) {
              if (revision != connectionRevision || !authorize()) {
                throw const StaleConnection();
              }
              uploaded += chunk.length;
              uploadProgress?.call(uploaded, upload.length);
              return chunk;
            }),
            upload.length,
            filename: uploadField == 'image' ? 'avr.bin' : 'esp.bin',
          ),
        );
    } else {
      request = http.Request(post ? 'POST' : 'GET', uri)..bodyFields = form;
    }
    request.headers.addAll(_writeHeaders);
    if (upload != null && uploadField == 'firmware') {
      request.headers['X-Pakar-Firmware-Size'] = '${upload.length}';
      request.headers['X-Pakar-Firmware-MD5'] = md5.convert(upload).toString();
    }
    request.followRedirects = false; // No credential forwarding or POST replay.
    final mutation = post || mayMutate;
    final elapsed = Stopwatch()..start();
    try {
      if (mutation) _retireStatusRead();
      final response = await _client.send(request).timeout(timeout);
      if (revision != connectionRevision) {
        await response.stream.listen((_) {}).cancel();
        throw const StaleConnection();
      }
      if (response.statusCode ~/ 100 != 2 ||
          (!post && response.statusCode != 200) ||
          (response.contentLength ?? 0) > maximum) {
        await response.stream.listen((_) {}).cancel();
        throw const FormatException(
          'Maintenance rejected or response exceeds its limit; nothing replayed',
        );
      }
      final bytes = BytesBuilder(copy: false);
      final reader = StreamIterator(response.stream);
      try {
        while (true) {
          final remaining = timeout - elapsed.elapsed;
          if (remaining <= Duration.zero) {
            throw TimeoutException('Maintenance deadline');
          }
          if (!await reader.moveNext().timeout(remaining)) break;
          if (revision != connectionRevision) throw const StaleConnection();
          final chunk = reader.current;
          if (bytes.length + chunk.length > maximum) {
            throw const FormatException(
              'Maintenance response exceeds its limit; nothing replayed',
            );
          }
          bytes.add(chunk);
        }
      } finally {
        await reader.cancel();
      }
      if (revision != connectionRevision) throw const StaleConnection();
      final result = bytes.takeBytes();
      if (response.contentLength != null &&
          response.contentLength != result.length) {
        throw const FormatException(
          'Truncated firmware transfer; no export created',
        );
      }
      if (verifySketch) {
        final digest = response.headers['x-firmware-md5'];
        if (digest == null ||
            !RegExp(r'^[a-fA-F0-9]{32}$').hasMatch(digest) ||
            md5.convert(result).toString() != digest.toLowerCase()) {
          throw const FormatException(
            'Running firmware transfer digest unavailable or mismatched; no export created',
          );
        }
      }
      return result;
    } on StaleConnection {
      rethrow;
    } on FormatException {
      rethrow;
    } on Exception {
      if (mutation) throw const OperationUncertain();
      throw const FormatException(
        'Maintenance read failed; no operation was replayed',
      );
    } finally {
      if (mutation) _retireStatusRead();
    }
  }

  Future<Map<String, dynamic>> maintenanceJson(
    String path, {
    bool post = false,
    bool mayMutate = false,
    Map<String, String> form = const {},
    Uint8List? upload,
    String? uploadField,
    void Function(int done, int total)? uploadProgress,
    required bool Function() authorize,
  }) async {
    if (isUsb &&
        {
          '/api/firmware/remote',
          '/api/firmware/apply',
          '/api/firmware/cancel',
        }.contains(path)) {
      if (!authorize()) throw const StaleConnection();
      final body = <String, dynamic>{
        for (final entry in form.entries)
          entry.key: entry.key == 'job_id'
              ? int.parse(entry.value)
              : entry.value,
      };
      return peerExchange(
        post ? 'POST' : 'GET',
        path,
        body,
        authorize: authorize,
      );
    }
    final bytes = await maintenanceBytes(
      path,
      post: post,
      mayMutate: mayMutate,
      form: form,
      upload: upload,
      uploadField: uploadField,
      uploadProgress: uploadProgress,
      authorize: authorize,
    );
    final value = jsonDecode(utf8.decode(bytes));
    if (value is! Map<String, dynamic>) {
      throw const FormatException(
        'Malformed maintenance acknowledgment; inspect before any new request',
      );
    }
    return value;
  }

  /// Same correlated WS/HTTP command owner as game actions. Older peers retain
  /// their explicit single-dispatch HTTP lease endpoint; never retry a rejected
  /// or uncertain preview through a different endpoint.
  Future<Map<String, dynamic>> previewCorrections(
    Map<String, dynamic> configuration, {
    required bool Function() authorize,
  }) async {
    const path = '/api/pixels/corrections/preview';
    final values = {'configuration': jsonEncode(configuration)};
    if (!isUsb &&
        freshStatus?.peer.has('pixel-corrections-preview-command') == true) {
      Map<String, dynamic>? envelope;
      final actual = await _fastCommand(
        path,
        values,
        authorize: authorize,
        receipt: (value) => envelope = value,
      );
      if (actual == null ||
          envelope == null ||
          envelope!['pixel_corrections_preview'] is! Map) {
        throw const OperationUncertain(
          'Correction preview receipt unavailable; no preview was replayed.',
        );
      }
      return {'preview': envelope!['pixel_corrections_preview']};
    }
    return postJson(path, values, authorize: authorize);
  }

  /// Single dispatch, form BODY (never secret-bearing URL parameters), and
  /// caller-owned capability check after any awaited identity verification.
  Future<Map<String, dynamic>> postJson(
    String path,
    Map<String, String> values, {
    bool Function()? authorize,
  }) async {
    _requireNetwork();
    if (isUsb && path == '/api/network') {
      await _ensureUsb();
      final owner = connectionRevision;
      const booleans = {
        'station_enabled',
        'ap_enabled',
        'dhcp',
        'station_open',
        'auto_switch',
      };
      return usb.networkApi(
        'POST',
        path,
        {
          for (final entry in values.entries)
            entry.key:
                booleans.contains(entry.key) &&
                    const {'true', 'false'}.contains(entry.value)
                ? entry.value == 'true'
                : entry.value,
        },
        authorize: () =>
            owner == connectionRevision &&
            connectionWanted &&
            authorize?.call() != false,
      );
    }
    if (path.startsWith('/api/minigame/')) {
      final actual = await fastCommand(
        path,
        values,
        authorize: authorize ?? () => true,
      );
      if (actual == null) {
        throw const FormatException('Minigame command receipt unavailable');
      }
      return {
        'accepted': true,
        'board_id': actual.json['board_id'],
        'boot_id': actual.json['boot_id'],
        'status': actual.json,
      };
    }
    if (isUsb) {
      if (path == '/api/pixels/corrections' ||
          path == '/api/pixels/corrections/preview') {
        await _ensureUsb();
        final owner = connectionRevision;
        final configuration = jsonDecode(values['configuration'] ?? 'null');
        if (configuration is! Map<String, dynamic>) {
          throw const FormatException('Invalid pixel correction configuration');
        }
        return usb.pixelCorrectionsApi(
          configuration: configuration,
          preview: path.endsWith('/preview'),
          authorize: () =>
              owner == connectionRevision &&
              connectionWanted &&
              authorize?.call() != false,
        );
      }
      if (path == '/api/pixels/groups') {
        await _ensureUsb();
        final owner = connectionRevision;
        final configuration = jsonDecode(values['configuration'] ?? 'null');
        if (configuration is! Map<String, dynamic>) {
          throw const FormatException('Invalid pixel group configuration');
        }
        return usb.pixelGroupsApi(
          configuration: configuration,
          authorize: () =>
              owner == connectionRevision &&
              connectionWanted &&
              authorize?.call() != false,
        );
      }
      final actual = await _usbCommand(path, values, authorize ?? () => true);
      return {
        'accepted': true,
        'board_id': actual.json['board_id'],
        'boot_id': actual.json['boot_id'],
        'status': actual.json,
      };
    }
    if (isBluetooth) {
      return _bleRequest('POST', path, values, authorize: authorize);
    }
    final revision = connectionRevision;
    if (_verifiedEndpoint != baseUrl) await status();
    if (revision != connectionRevision || authorize?.call() == false) {
      throw const StaleConnection();
    }
    late final http.Response response;
    try {
      final request = http.Request('POST', _uri(transportUrl, path))
        ..headers.addAll(_writeHeaders)
        ..bodyFields = values
        ..followRedirects = false;
      _retireStatusRead();
      response = await _writeResponse(request, const Duration(seconds: 12));
    } on TimeoutException {
      throw const OperationUncertain(
        'No device write receipt arrived within 12 seconds.',
      );
    } on Exception {
      throw const OperationUncertain(
        'The connection failed while waiting for a device write receipt.',
      );
    } finally {
      _retireStatusRead();
    }
    if (revision != connectionRevision) throw const StaleConnection();
    _rejectWriteResponse(response);
    try {
      final result = jsonDecode(response.body);
      if (result is! Map<String, dynamic>) {
        throw const OperationUncertain(
          'The board returned a malformed write receipt.',
        );
      }
      return result;
    } on FormatException {
      throw const OperationUncertain(
        'The board returned a malformed write receipt.',
      );
    }
  }

  Future<void> post(
    String path, [
    Map<String, String> values = const {},
  ]) async {
    _requireNetwork();
    if (path.startsWith('/api/minigame/')) {
      await postJson(path, values);
      return;
    }
    if (isUsb) {
      await _ensureUsb();
      if (freshStatus == null && !usb.busy) await status();
      if (path == '/api/audio') {
        await usb.cue(values['cue'] ?? '');
        return;
      }
      if (path == '/api/display') {
        await usb.display(values);
        return;
      }
      await _usbCommand(path, values, () => true);
      return;
    }
    if (isBluetooth) {
      await _bleRequest('POST', path, values);
      return;
    }
    final revision = connectionRevision;
    // Resolve/verify through a read first; a POST must never be automatically
    // replayed after a timeout or disconnect (the board may have applied it).
    if (_verifiedEndpoint != baseUrl) await status();
    if (revision != connectionRevision) throw const StaleConnection();
    late final http.Response response;
    try {
      final request = http.Request('POST', _uri(transportUrl, path))
        ..headers.addAll(_writeHeaders)
        ..bodyFields = values
        ..followRedirects = false;
      _retireStatusRead();
      response = await _writeResponse(request, const Duration(seconds: 12));
    } on TimeoutException {
      throw const OperationUncertain(
        'No device write receipt arrived within 12 seconds.',
      );
    } on Exception {
      throw const OperationUncertain(
        'The connection failed while waiting for a device write receipt.',
      );
    } finally {
      _retireStatusRead();
    }
    if (revision != connectionRevision) throw const StaleConnection();
    _rejectWriteResponse(response);
  }

  void _rejectWriteResponse(http.Response response) {
    if (response.statusCode == 401) throw const AuthenticationRequired();
    Map<String, dynamic>? receipt;
    if (response.body.isNotEmpty) {
      try {
        final decoded = jsonDecode(response.body);
        if (decoded is Map<String, dynamic>) receipt = decoded;
      } on FormatException {
        // Older successful REST routes may return plain text; never echo it.
      }
    }
    if (response.statusCode ~/ 100 != 2 ||
        (receipt?.containsKey('accepted') == true &&
            receipt?['accepted'] != true)) {
      throw DeviceCommandRejected(
        error: receipt?['error'],
        httpStatus: response.statusCode,
      );
    }
  }

  Future<DeviceStatus>? _statusTask;
  Completer<void>? _statusReadAbort;
  int? _statusTaskRevision;
  int _statusReadSequence = 0;

  void _retireStatusRead() {
    ++_statusReadSequence;
    _statusTask = null;
    _statusTaskRevision = null;
  }

  /// Adapt PeerSession.refresh's single-flight ownership. Concurrent consumers
  /// share only this read; commands and post-command readback are never queued
  /// or replayed. A replacement connection starts a separate owned read.
  Future<DeviceStatus> status() {
    final owner = connectionRevision;
    final active = _statusTask;
    if (active != null && _statusTaskRevision == owner) return active;
    late final Future<DeviceStatus> task;
    task = _readStatus(_statusReadSequence).whenComplete(() {
      if (identical(_statusTask, task)) {
        _statusTask = null;
        _statusTaskRevision = null;
      }
    });
    _statusTaskRevision = owner;
    return _statusTask = task;
  }

  /// Internal read retirement is not a failed user command. Only retry reads;
  /// never replay a mutation or let a different peer complete this intent.
  Future<DeviceStatus> settledStatus({bool Function()? stillNeeded}) async {
    final owner = connectionRevision;
    final deadline = Stopwatch()..start();
    for (var attempt = 0; attempt < 3; attempt++) {
      if (stillNeeded != null && !stillNeeded()) throw const SupersededRead();
      if (owner != connectionRevision) throw const StaleConnection();
      final remaining = readTimeout - deadline.elapsed;
      if (remaining <= Duration.zero) break;
      try {
        final result = await status().timeout(remaining);
        if (owner != connectionRevision) throw const StaleConnection();
        return result;
      } on SupersededRead {
        if (owner != connectionRevision) throw const StaleConnection();
        // Ask for the current snapshot, not the retired request's cached value.
      }
    }
    throw TimeoutException(
      'Device state could not be confirmed. No command was replayed.',
      readTimeout,
    );
  }

  Future<DeviceStatus> _readStatus(int sequence) async {
    late final DeviceStatus result;
    try {
      result = DeviceStatus(
        await _get(
          connectionMethod == ConnectionMethod.network &&
                  (_usbDemandFlags & 8) != 0
              ? '/api/status?pixels=1'
              : '/api/status',
        ),
      );
    } on Object {
      if (sequence != _statusReadSequence) throw const SupersededRead();
      rethrow;
    }
    if (sequence != _statusReadSequence) throw const SupersededRead();
    // Invalid critical identity must retire captured owners even when hostname
    // validation would reject the same response first. Never accept the peer
    // before the requested-host check below.
    _validateIdentity(result.json);
    if (!_matchesLocalIdentity(result.json)) {
      throw const FormatException(
        'Device identity does not match the requested hostname',
      );
    }
    _acceptIdentity(result.json);
    _verifiedEndpoint = baseUrl;
    _rememberNetworkRoute(result.json);
    _verifiedEndpoint = baseUrl;
    if (!_rememberStatus(result)) throw const SupersededRead();
    return _latestStatus!;
  }

  // Adapt Session's synchronous retirement boundary: retire captured owners
  // before a new board/boot may authorize any old dialog or in-flight request.
  void _validateIdentity(Map<String, dynamic> value) {
    final board = value['board_id'], boot = value['boot_id'];
    bool identity(Object? v) =>
        v is String && RegExp(r'^[!-~]{1,80}$').hasMatch(v);
    if ((_identityRequired ||
            value.containsKey('board_id') ||
            value.containsKey('boot_id')) &&
        (!identity(board) || !identity(boot))) {
      if (_verifiedBoard != null) {
        resetTransport();
        onPeerChanged?.call();
      }
      throw const StaleConnection();
    }
  }

  bool _acceptIdentity(Map<String, dynamic> value) {
    _checkPeerVersion(value);
    _validateIdentity(value);
    final board = value['board_id'], boot = value['boot_id'];
    bool identity(Object? v) =>
        v is String && RegExp(r'^[!-~]{1,80}$').hasMatch(v);
    final changed =
        _verifiedBoard != null &&
        (board != _verifiedBoard || boot != _verifiedBoot);
    if (changed) {
      resetTransport();
      onPeerChanged?.call();
    }
    _verifiedBoard = identity(board) && identity(boot) ? board as String : null;
    _verifiedBoot = identity(board) && identity(boot) ? boot as String : null;
    if (_verifiedBoard != null) _identityRequired = true;
    return changed;
  }

  Future<BoardSound> boardSound({bool? silent}) async {
    if (isUsb) {
      await _ensureUsb();
      return usb.boardSound(silent: silent);
    }
    final revision = connectionRevision;
    final before = freshStatus ?? await status();
    final board = before.json['board_id'], boot = before.json['boot_id'];
    void admit() {
      _requireNetwork();
      if (!appForeground ||
          revision != connectionRevision ||
          board != _verifiedBoard ||
          boot != _verifiedBoot) {
        throw const StaleConnection();
      }
    }

    admit();
    if (before.json['avr_online'] != true ||
        !before.peer.has('settings') ||
        !before.peer.has('buzzer') ||
        (silent != null && before.json['programming_latch'] == true)) {
      throw const FormatException('Board sound settings unavailable');
    }
    if (_verifiedBoard == null || _verifiedBoot == null) {
      throw const StaleConnection();
    }
    Future<Map<String, dynamic>> contract(int opcode, String payload) async {
      admit();
      final queued = await postJson('/api/avr/contracts', {
        'opcode': '$opcode',
        'payload_hex': payload,
      }, authorize: () => revision == connectionRevision);
      final id = queued['request_id'];
      if (id is! int || id <= 0 || queued['boot_id'] != boot) {
        throw const OperationUncertain();
      }
      final deadline = DateTime.now().add(const Duration(seconds: 3));
      while (true) {
        admit();
        final receipt = await _get('/api/avr/contracts/result?id=$id');
        if (receipt['request_id'] != id ||
            receipt['boot_id'] != boot ||
            receipt['request_opcode'] != opcode) {
          throw const OperationUncertain();
        }
        if (receipt['state'] == 'responded') {
          final response = receipt['response_opcode'],
              hex = receipt['payload_hex'];
          if (hex is! String ||
              (response != 0x82 &&
                  (opcode == 4
                      ? response != 0x91
                      : response != 0x80 || hex != '0500'))) {
            throw const OperationUncertain();
          }
          // Consume only the matching terminal receipt, never a queued write.
          admit();
          final request =
              http.Request(
                  'DELETE',
                  _uri(transportUrl, '/api/avr/contracts/result', {
                    'id': '$id',
                    'boot_id': '$boot',
                    'opcode': '$opcode',
                  }),
                )
                ..headers.addAll(_writeHeaders)
                ..followRedirects = false;
          if (!isBluetooth) {
            await http.Response.fromStream(await _client.send(request))
                .timeout(const Duration(seconds: 4));
          }
          admit();
          if (response == 0x82) {
            throw const FormatException(
              'AVR rejected sound settings; not retried',
            );
          }
          return receipt;
        }
        if (!['queued', 'awaiting'].contains(receipt['state']) ||
            DateTime.now().isAfter(deadline)) {
          throw const OperationUncertain();
        }
        await Future<void>.delayed(const Duration(milliseconds: 40));
      }
    }

    final read = await contract(4, '');
    var value = BoardSound.parse(read['payload_hex'] as String);
    if (silent != null && (!value.persisted || value.silent != silent)) {
      final expected = value.withSilent(silent);
      await contract(5, expected);
      final readback = await contract(4, '');
      value = BoardSound.parse(readback['payload_hex'] as String);
      if (!value.persisted || BoardSound.encode(value.bytes) != expected) {
        throw const FormatException(
          'Persistent Silent mode readback did not match',
        );
      }
    }
    // An existing validated full WS/status observation keeps its original
    // lease. Do not add a redundant HTTP snapshot after actual AVR readback.
    final after = freshStatus ?? await status();
    admit();
    if (after.json['board_id'] != board || after.json['boot_id'] != boot) {
      throw const StaleConnection();
    }
    return value;
  }

  Future<CellMap>? _mappingRead;
  int? _mappingOwner;
  int? _mappingConfiguration;

  Future<CellMap> mapping() {
    final pending = _mappingRead;
    if (pending != null &&
        _mappingOwner == connectionRevision &&
        _mappingConfiguration == _configurationRevision) {
      return pending;
    }
    _mappingOwner = connectionRevision;
    _mappingConfiguration = _configurationRevision;
    late final Future<CellMap> task;
    task = _readMapping().whenComplete(() {
      if (identical(_mappingRead, task)) {
        _mappingRead = null;
        _mappingOwner = null;
        _mappingConfiguration = null;
      }
    });
    _mappingRead = task;
    return task;
  }

  Future<CellMap> _readMapping() async {
    final revision = connectionRevision;
    final configuration = _configurationRevision;
    final before = freshStatus ?? await settledStatus();
    if (revision != connectionRevision) throw const StaleConnection();
    final board = before.json['board_id'], boot = before.json['boot_id'];
    if (board == null || boot == null) {
      throw const FormatException('A current board identity is required');
    }
    final value = await getJson('/api/cells/mapping');
    if (revision != connectionRevision) throw const StaleConnection();
    final result = CellMap.fromJson(value);
    final identified =
        value.containsKey('board_id') || value.containsKey('boot_id');
    if (identified &&
        (value['board_id'] != board || value['boot_id'] != boot)) {
      throw const StaleConnection();
    }
    // The response already identifies its producer. A healthy full WS status
    // fences it without a second HTTP read that can be superseded by a command.
    // A response omitting additive identity still needs explicit status fencing.
    final after = identified
        ? freshStatus ?? await settledStatus()
        : await settledStatus();
    if (revision != connectionRevision ||
        after.json['board_id'] != board ||
        after.json['boot_id'] != boot) {
      throw const StaleConnection();
    }
    if (configuration != _configurationRevision) throw const SupersededRead();
    return result;
  }

  Stream<DeviceStatus> liveStatus() async* {
    _requireNetwork();
    if (isUsb) {
      yield* _liveUsb();
      return;
    }
    if (isBluetooth) {
      yield* _liveBluetooth();
      return;
    }
    final revision = connectionRevision;
    // A same-origin socket may establish identity from its first full status.
    // Do not make working real-time control depend on a slow HTTP preflight.
    if (revision != connectionRevision) throw const StaleConnection();
    final previous = _channel;
    _sentPixelDemand = null;
    _channel = null;
    _socketReady = false;
    _liveAge = _socketAge = null;
    // A vanished peer must not hold replacement behind an unbounded close
    // handshake. Retire ownership before awaiting it, like the donor session.
    await _retireSocket(previous);
    if (revision != connectionRevision) throw const StaleConnection();
    final uri = webSocketUri;
    final active = IOWebSocketChannel.connect(
      uri,
      headers: {..._headers}..remove('User-Agent'),
      // Android may pause Dart timers/network while backgrounded. A built-in
      // ping timeout would turn that suspension into a needless disconnect.
      // Foreground supervision owns read probes and bounded silent-socket reset.
      pingInterval: null,
      connectTimeout: const Duration(seconds: 6),
      customClient: _ioClient,
    );
    _channel = active;
    try {
      // Failed HTTP upgrades must pass through the same cleanup as a socket
      // that was already live; otherwise each reconnect retains a dead owner.
      await active.ready;
      if (revision != connectionRevision || !identical(_channel, active)) {
        return;
      }
      _socketReady = true;
      _socketAge = Stopwatch()..start();
      active.sink.add('{"type":"status"}');
      await for (final message in active.stream) {
        if (revision != connectionRevision || !identical(_channel, active)) {
          return;
        }
        if (message is! String) continue;
        final value = jsonDecode(message);
        if (value is Map<String, dynamic> &&
            value['type'] == 'network-observation') {
          if (value['board_id'] == _verifiedBoard &&
              value['boot_id'] == _verifiedBoot &&
              _verifiedBoard != null &&
              !_networkObservations.isClosed) {
            _networkObservations.add(value);
          }
          continue;
        }
        if (value is Map<String, dynamic> &&
            value['type'] == 'configuration-changed') {
          final changed = value['revision'];
          if (value['board_id'] == _verifiedBoard &&
              value['boot_id'] == _verifiedBoot &&
              _verifiedBoard != null &&
              changed is int &&
              changed > _configurationRevision) {
            _configurationRevision = changed;
            _configurationRefresh ??= Timer(
              const Duration(milliseconds: 250),
              () {
                _configurationRefresh = null;
                if (revision == connectionRevision &&
                    connectionWanted &&
                    appForeground) {
                  if (!_configurationUpdates.isClosed) {
                    _configurationUpdates.add(_configurationRevision);
                  }
                  onConfigurationChanged?.call();
                }
              },
            );
          }
          continue; // A settings edge never renews the full-status lease.
        }
        if (value is Map<String, dynamic> &&
            value['type'] == 'command-result') {
          if (value['board_id'] == _verifiedBoard &&
              value['boot_id'] == _verifiedBoot) {
            final pending = _commandReplies[value['request_id']];
            if (pending != null && !pending.isCompleted) {
              pending.complete(value);
            }
          }
          continue;
        }
        if (value is Map<String, dynamic> && value['type'] == 'pixel-frame') {
          if (_wantsNetworkPixels &&
              liveFresh &&
              value['board_id'] == _verifiedBoard &&
              value['boot_id'] == _verifiedBoot) {
            final next = _latestStatus?.withPixelFrameEvent(value);
            if (next != null) {
              _latestStatus = next;
              if (!_pixelUpdates.isClosed) _pixelUpdates.add(next);
            }
          }
          continue; // No lease renewal, full-status emission or firmware work.
        }
        if (value is Map<String, dynamic> && value['type'] == 'status-rgb') {
          if (liveFresh &&
              value['board_id'] == _verifiedBoard &&
              value['boot_id'] == _verifiedBoot) {
            final next = _latestStatus?.withStatusRgbEvent(value);
            if (next != null) {
              _latestStatus = next;
              if (!_rgbUpdates.isClosed) _rgbUpdates.add(next);
            }
          }
          continue; // No _statusAge/_liveAge update, full-status yield or replay.
        }
        if (value is Map<String, dynamic> &&
            value['type'] == 'update-progress') {
          // An incremental lifecycle packet cannot establish peer identity or
          // renew full-status/hardware freshness. Existing progress consumers
          // share the statusUpdates stream; the main full-status stream stays
          // authoritative for cell state, capability and selection handling.
          if (_verifiedBoard != null &&
              _verifiedBoot != null &&
              value['board_id'] == _verifiedBoard &&
              value['boot_id'] == _verifiedBoot) {
            final next = _latestStatus?.withUpgradeEvent(value);
            if (next != null) {
              _latestStatus = next;
              if (!_statusUpdates.isClosed) _statusUpdates.add(next);
            }
          }
          continue;
        }
        if (value is Map<String, dynamic> && value['type'] == 'panel-event') {
          if (liveFresh &&
              value['board_id'] == _verifiedBoard &&
              value['boot_id'] == _verifiedBoot) {
            onPeripheralEvent?.call(value);
          }
          continue; // Does not refresh any full-status or hardware lease.
        }
        if (value is Map<String, dynamic> &&
            value['type'] == 'pc-contract-event') {
          // Events cannot establish identity or refresh an expired status lease.
          if (_verifiedBoard != null &&
              value['board_id'] == _verifiedBoard &&
              value['boot_id'] == _verifiedBoot) {
            onPeripheralEvent?.call(value);
          }
          continue;
        }
        if (value is Map<String, dynamic> && value.containsKey('hostname')) {
          if (!_matchesLocalIdentity(value)) {
            throw const StaleConnection();
          }
          if (_acceptIdentity(value)) {
            return; // Reconnect only reads; never replay.
          }
          if (_verifiedBoard != null && _verifiedBoot != null) {
            _verifiedEndpoint = baseUrl;
            _rememberNetworkRoute(value);
            _verifiedEndpoint = baseUrl;
          }
          final status = DeviceStatus(value);
          if (!_rememberStatus(status)) continue;
          _liveAge = Stopwatch()..start();
          _syncPixelSubscription();
          yield _latestStatus!;
        }
      }
    } finally {
      if (identical(_channel, active)) {
        _socketReady = false;
        _liveAge = _socketAge = null;
        _channel = null;
      }
      unawaited(_retireSocket(active));
    }
  }

  Future<void> _ensureBluetooth() async {
    _requireNetwork();
    final owner = connectionRevision, endpoint = baseUrl;
    if (!isBluetooth) throw const StaleConnection();
    await bluetooth.connect(
      PakarBleTransport.addressFromEndpoint(endpoint),
      expectedBoard: expectedBluetoothBoard,
    );
    if (owner != connectionRevision ||
        !isBluetooth ||
        endpoint != baseUrl ||
        !connectionWanted) {
      throw const StaleConnection();
    }
  }

  Future<void> _ensureUsb() async {
    _requireNetwork();
    if (!isUsb) throw const StaleConnection();
    final owner = connectionRevision;
    await usb.connect(
      baseUrl,
      expectedBoard: expectedBluetoothBoard,
      enforce: enforcePeerVersions && peerVersionChecksEnabled,
    );
    if (owner != connectionRevision || !connectionWanted || !isUsb) {
      throw const StaleConnection();
    }
    usb.demand(_effectiveUsbDemand, foreground: appForeground);
  }

  Future<DeviceStatus> _usbCommand(
    String path,
    Map<String, String> values,
    bool Function() authorize,
  ) async {
    _requireNetwork();
    final owner = connectionRevision;
    await _ensureUsb();
    if (freshStatus == null && !usb.busy) await status();
    bool admitted() =>
        owner == connectionRevision &&
        isUsb &&
        connectionWanted &&
        appForeground &&
        authorize();
    if (!admitted()) throw const StaleConnection();
    final actual = await usb.command(path, values, authorize: admitted);
    if (!admitted()) throw const StaleConnection();
    _acceptIdentity(actual.json);
    _verifiedEndpoint = baseUrl;
    _rememberStatus(actual);
    _liveAge = Stopwatch()..start();
    return actual;
  }

  Stream<DeviceStatus> _liveUsb() async* {
    final owner = connectionRevision;
    await _ensureUsb();
    final updates = StreamController<DeviceStatus>();
    bool retired = false, reading = false;
    Future<void> refresh() async {
      if (retired ||
          reading ||
          usb.busy ||
          !appForeground ||
          owner != connectionRevision) {
        return;
      }
      reading = true;
      try {
        final actual = await status();
        if (!retired && owner == connectionRevision) {
          _liveAge = Stopwatch()..start();
          updates.add(actual);
        }
      } on Object catch (error) {
        if (!retired) updates.addError(error);
      } finally {
        reading = false;
      }
    }

    final subscription = usb.events.stream.listen(
      (event) {
        if (event['type'] == 'changed') {
          unawaited(refresh());
        } else if (!retired && owner == connectionRevision) {
          onPeripheralEvent?.call(event);
        }
      },
      onError: (Object e) {
        if (!retired) updates.addError(e);
      },
    );
    final timer = Timer.periodic(
      const Duration(seconds: 3),
      (_) => unawaited(refresh()),
    );
    try {
      await refresh();
      yield* updates.stream;
    } finally {
      retired = true;
      timer.cancel();
      await subscription.cancel();
      unawaited(updates.close());
    }
  }

  Future<Map<String, dynamic>> _bleRequest(
    String method,
    String path,
    Map<String, String> values, {
    bool Function()? authorize,
  }) async {
    final owner = connectionRevision;
    if (method == 'POST' && !appForeground) throw const ConnectionStopped();
    await _ensureBluetooth();
    final uri = Uri.parse(path);
    final bootstrap = method == 'GET' && uri.path == '/api/status';
    if (!bootstrap &&
        (_latestStatus == null ||
            method == 'POST' &&
                (_statusAge?.elapsedMilliseconds ?? 9999) >= 2000)) {
      await status();
    }
    if (owner != connectionRevision ||
        !isBluetooth ||
        !connectionWanted ||
        authorize?.call() == false) {
      throw const StaleConnection();
    }
    if (!bootstrap && !supportsRoute(method, uri.path)) {
      throw const FormatException(
        'This Bluetooth board does not advertise that operation',
      );
    }
    var clock = _latestStatus?.json['command_clock_ms'];
    if (method == 'POST' &&
        (_verifiedBoard == null ||
            _verifiedBoot == null ||
            clock is! int ||
            clock < 0)) {
      throw const StaleConnection();
    }
    if (method == 'POST') clock = await _freshBluetoothClock();
    if (owner != connectionRevision ||
        authorize?.call() == false ||
        method == 'POST' && !appForeground) {
      throw const StaleConnection();
    }
    try {
      final response = await bluetooth.request(
        method,
        uri.path,
        {...uri.queryParameters, ...values},
        board: _verifiedBoard ?? '',
        boot: _verifiedBoot ?? '',
        expires: method == 'POST' ? (clock as int) + 8000 : 0,
      );
      if (owner != connectionRevision ||
          !isBluetooth ||
          authorize?.call() == false) {
        throw const StaleConnection();
      }
      if (response.status == 401) throw const AuthenticationRequired();
      if (response.status ~/ 100 != 2) {
        throw FormatException(
          'Bluetooth device $method rejected (${response.status}); not retried',
        );
      }
      if (bootstrap) {
        final status = DeviceStatus(response.body);
        if (!status.peer.known ||
            response.body['board_id'] != bluetooth.boardId ||
            response.body['boot_id'] != bluetooth.bootId) {
          throw const FormatException(
            'Bluetooth full board/application identity is not verified',
          );
        }
      }
      return response.body;
    } on StaleConnection {
      rethrow;
    } on AuthenticationRequired {
      rethrow;
    } on FormatException {
      rethrow;
    } on BleBusy {
      rethrow;
    } on Exception {
      if (method == 'POST') throw const OperationUncertain();
      rethrow;
    } finally {
      if (owner == connectionRevision && isBluetooth && !bluetooth.ready) {
        onPeripheralStreamLost?.call();
      }
    }
  }

  Future<int> _freshBluetoothClock() async {
    final value = await _bleRequest('GET', '/api/ble/clock', const {});
    final clock = value['command_clock_ms'];
    if (value['board_id'] != _verifiedBoard ||
        value['boot_id'] != _verifiedBoot ||
        clock is! int ||
        clock < 0 ||
        clock > 0x1fffffffffffff - 8000) {
      throw const StaleConnection();
    }
    return clock;
  }

  Stream<DeviceStatus> _liveBluetooth() async* {
    final owner = connectionRevision;
    await _ensureBluetooth();
    final updates = StreamController<DeviceStatus>();
    Timer? poll;
    bool retired = false;
    void accept(Map<String, dynamic> value) {
      if (retired ||
          owner != connectionRevision ||
          !isBluetooth ||
          !connectionWanted) {
        return;
      }
      if (value['board_id'] != bluetooth.boardId ||
          value['boot_id'] != bluetooth.bootId) {
        return;
      }
      final event = value['type'] == 'ble-event'
          ? value['event']
          : value['type'];
      final body = value['type'] == 'ble-event' ? value['body'] : value;
      if (body is! Map<String, dynamic>) return;
      if (event == 'update-progress') {
        final next = _latestStatus?.withUpgradeEvent(body);
        if (next != null) {
          _latestStatus = next;
          if (!_statusUpdates.isClosed) _statusUpdates.add(next);
        }
        return;
      }
      if (event == 'pc-contract-event' || event == 'pc-contract-result') {
        onPeripheralEvent?.call(body);
        return;
      }
      if (event == 'panel-event') {
        if (liveFresh &&
            body['board_id'] == _verifiedBoard &&
            body['boot_id'] == _verifiedBoot) {
          onPeripheralEvent?.call(body);
        }
        return;
      }
      final raw = event == 'status' || value['type'] != 'ble-event'
          ? body
          : null;
      if (raw is! Map<String, dynamic> ||
          raw['hostname'] is! String ||
          raw['board_id'] != bluetooth.boardId ||
          raw['boot_id'] != bluetooth.bootId) {
        return;
      }
      try {
        if (_acceptIdentity(raw)) return;
        final actual = DeviceStatus(raw);
        if (!actual.peer.known || !_rememberStatus(actual)) return;
        _verifiedEndpoint = baseUrl;
        _liveAge = Stopwatch()..start();
        updates.add(_latestStatus!);
      } on Object catch (error) {
        updates.addError(error);
      }
    }

    final subscription = bluetooth.events.stream.listen(
      accept,
      onError: (Object error) {
        if (!retired && owner == connectionRevision && isBluetooth) {
          onPeripheralStreamLost?.call();
          updates.addError(error);
        }
      },
    );
    try {
      final initial = await status();
      if (owner != connectionRevision) return;
      _socketAge = Stopwatch()..start();
      _liveAge = Stopwatch()..start();
      updates.add(initial);
      final ble = initial.json['ble'];
      final push =
          ble is Map &&
          ble['push'] is List &&
          (ble['push'] as List).contains('status');
      if (!push) {
        poll = Timer.periodic(const Duration(seconds: 1), (_) {
          if (retired ||
              owner != connectionRevision ||
              !appForeground ||
              bluetooth.busy) {
            return;
          }
          unawaited(
            status().then<void>(
              (value) {
                if (!retired) {
                  _liveAge = Stopwatch()..start();
                  updates.add(value);
                }
              },
              onError: (Object error) {
                if (!retired) updates.addError(error);
              },
            ),
          );
        });
      }
      yield* updates.stream;
    } finally {
      retired = true;
      poll?.cancel();
      await subscription.cancel();
      await updates.close();
    }
  }

  Future<void> _retireSocket(WebSocketChannel? socket) async {
    if (socket == null) return;
    try {
      await socket.sink.close().timeout(const Duration(milliseconds: 500));
    } on Object {
      // Retirement is cleanup, never a reason to replay a request or clear UI.
    }
  }

  /// Retire only the live stream owner before awaiting its subscription's
  /// cancellation. Keep HTTP, address cache, peer revision and fresh UI intact.
  Future<void> retireLiveSocket() async {
    final active = _channel;
    _channel = null;
    _socketReady = false;
    _liveAge = _socketAge = null;
    for (final pending in _commandReplies.values) {
      if (!pending.isCompleted) pending.complete(null);
    }
    _commandReplies.clear();
    if (active != null) onPeripheralStreamLost?.call();
    await _retireSocket(active);
  }

  Future<void> close() async {
    _configurationRefresh?.cancel();
    _configurationRefresh = null;
    unawaited(_configurationUpdates.close());
    unawaited(_networkObservations.close());
    await bluetooth.close();
    await usb.close();
    _abortReads();
    _socketReady = false;
    _latestStatus = null;
    _statusAge = null;
    _liveAge = _socketAge = null;
    for (final pending in _commandReplies.values) {
      if (!pending.isCompleted) pending.complete(null);
    }
    _commandReplies.clear();
    ++_connectionRevision;
    _statusTask = null;
    _statusTaskRevision = null;
    final active = _channel;
    _channel = null;
    _verifiedEndpoint = null;
    _verifiedBoard = _verifiedBoot = null;
    try {
      await active?.sink.close().timeout(const Duration(milliseconds: 500));
    } on Exception {
      // A vanished board cannot hold client disposal/connection retirement.
    } finally {
      _client.close();
      unawaited(_statusUpdates.close());
      unawaited(_connectionUpdates.close());
      unawaited(_rgbUpdates.close());
      unawaited(_pixelUpdates.close());
      _ioClient.close(force: true);
    }
  }

  void resetTransport({UnsupportedPeerVersion? unsupported}) {
    _configurationRefresh?.cancel();
    _configurationRefresh = null;
    _configurationRevision = 0;
    _unsupportedPeer = unsupported;
    _introducedPeer = null;
    _introductionAge = null;
    receivedServerHeaders = const {};
    unawaited(bluetooth.disconnect());
    unawaited(usb.disconnect());
    _abortReads();
    _socketReady = false;
    _latestStatus = null;
    _statusAge = null;
    _liveAge = _socketAge = null;
    for (final pending in _commandReplies.values) {
      if (!pending.isCompleted) pending.complete(null);
    }
    _commandReplies.clear();
    ++_connectionRevision;
    _statusTask = null;
    _statusTaskRevision = null;
    _verifiedEndpoint = null;
    _verifiedBoard = _verifiedBoot = null;
    onTransportRetired?.call();
    if (!_connectionUpdates.isClosed) _connectionUpdates.add(null);
    final active = _channel;
    _channel = null;
    if (active != null) {
      unawaited(_retireSocket(active));
    }
  }

  String? _localLabel() {
    final host = Uri.tryParse(baseUrl)?.host.toLowerCase();
    if (host == null || !host.endsWith('.local')) return null;
    final label = host.substring(0, host.length - 6);
    return RegExp(r'^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$').hasMatch(label)
        ? label
        : null;
  }

  bool _matchesLocalIdentity(Map<String, dynamic> value) {
    if (_knownNetworkEndpoint == baseUrl &&
        _knownNetworkBoard != null &&
        value['board_id'] != _knownNetworkBoard) {
      return false;
    }
    final expected = _localLabel();
    if (expected == null || value['hostname'] == expected) return true;
    // A confirmed board may rename itself without becoming another peer.
    if (_knownNetworkEndpoint == baseUrl &&
        _knownNetworkBoard != null &&
        value['board_id'] == _knownNetworkBoard) {
      return true;
    }
    final bridge = value['bridge'];
    if (bridge is! Map ||
        bridge['product'] != 'PakarController' ||
        bridge['hostname'] != expected ||
        bridge['instance_id'] is! String) {
      return false;
    }
    final path = bridge['path'];
    return path is List &&
        path.isNotEmpty &&
        path.length <= 4 &&
        path.toSet().length == path.length &&
        path.last == bridge['instance_id'] &&
        path.every(
          (id) =>
              id is String &&
              RegExp(r'^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$')
                  .hasMatch(id),
        );
  }

  Future<http.Response> _withDiscovery(
    Future<http.Response> Function(String base) request,
  ) async {
    final revision = connectionRevision, headers = _headers;
    try {
      return await request(transportUrl);
    } on Exception catch (error) {
      if (revision != connectionRevision) throw const StaleConnection();
      final knownHostname =
          _knownNetworkEndpoint == baseUrl &&
          _knownNetworkBoard != null &&
          InternetAddress.tryParse(Uri.parse(baseUrl).host) == null;
      if ((_localLabel() == null && !knownHostname) ||
          (error is! SocketException &&
              !(error is TimeoutException && transportUrl == baseUrl))) {
        rethrow;
      }
    }

    final verifiedTlsAddresses = <String, InternetAddress>{};
    final verifiedBoardIds = <String, String>{};
    Future<bool> verified(String candidate) async {
      if (revision != connectionRevision) throw const StaleConnection();
      final origin = Uri.tryParse(candidate);
      if (origin == null ||
          !['http', 'https'].contains(origin.scheme) ||
          origin.scheme != Uri.parse(baseUrl).scheme ||
          origin.userInfo.isNotEmpty) {
        return false;
      }
      http.Client? isolated;
      try {
        final probeHeaders = {...headers}..remove('authorization');
        final numeric = InternetAddress.tryParse(origin.host);
        var probeOrigin = candidate;
        if (origin.scheme == 'https' &&
            numeric != null &&
            InternetAddress.tryParse(Uri.parse(baseUrl).host) == null) {
          if (origin.port != Uri.parse(baseUrl).port) return false;
          // A saved IP is NOT certificate identity. Probe that TCP address in
          // isolation while retaining the selected URL/Host/SNI. No credential
          // or shared resolver cache is touched before board+boot validation.
          final probeResolver = LanResolver(
            nativeFirst: false,
            securityContext: resolver.securityContext,
            lookup: (_) async => [numeric],
          );
          final io = probeResolver.createClient()
            ..findProxy = ((_) => 'DIRECT');
          isolated = IOClient(io);
          probeOrigin = baseUrl;
        }
        final probe = await _readResponse(
          probeOrigin,
          '/api/status',
          probeHeaders,
          timeout: const Duration(seconds: 2),
          readClient: isolated,
          recordHeaders: false,
        );
        if (probe.statusCode ~/ 100 != 2) return false;
        if (revision != connectionRevision) throw const StaleConnection();
        final json = jsonDecode(probe.body);
        if (json is! Map<String, dynamic> || !_matchesLocalIdentity(json)) {
          return false;
        }
        validatePeerBoard(
          json,
          appVersion: PeerIdentity.current.version,
          enabled: enforcePeerVersions,
        );
        final board = json['board_id'], boot = json['boot_id'];
        if (board is! String ||
            boot is! String ||
            !RegExp(r'^[!-~]{1,80}$').hasMatch(board) ||
            !RegExp(r'^[!-~]{1,80}$').hasMatch(boot)) {
          return false;
        }
        verifiedBoardIds[candidate] = board;
        if (isolated != null && numeric != null) {
          verifiedTlsAddresses[candidate] = numeric;
        }
      } on Object {
        if (revision != connectionRevision) throw const StaleConnection();
        return false;
      } finally {
        isolated?.close();
      }
      return true;
    }

    // Saved hostname remains first. Only its failed read permits the last
    // same-board numeric route; this is never used to retry a mutation.
    final remembered = _knownNetworkEndpoint == baseUrl
        ? _lastVerifiedNetworkOrigin
        : null;
    String? selected;
    if (transportUrl != baseUrl && await verified(baseUrl)) {
      selected = baseUrl;
    } else if (remembered != null &&
        remembered != transportUrl &&
        await verified(remembered)) {
      selected = remembered;
    } else {
      final candidates = (await (discovery?.call() ?? _discoverSsdp()))
          .take(32)
          .toList();
      final good = <String>{};
      var next = 0;
      final deadline = DateTime.now().add(const Duration(seconds: 12));
      await Future.wait(
        List.generate(4, (_) async {
          while (next < candidates.length &&
              DateTime.now().isBefore(deadline)) {
            final candidate = candidates[next++];
            if (await verified(candidate)) good.add(candidate);
          }
        }),
      );
      // A first-open hostname can collide. Count physical identities after
      // the whole bounded sweep, not whichever response happened to win.
      final ids = good.map((origin) => verifiedBoardIds[origin]).toSet();
      if (ids.length == 1 && next >= candidates.length) {
        selected = candidates.where(good.contains).firstOrNull;
      }
    }
    if (selected != null) {
      final candidate = selected;
      if (revision != connectionRevision) throw const StaleConnection();
      _transportOwner = baseUrl;
      final tlsAddress = verifiedTlsAddresses[candidate];
      if (tlsAddress != null) {
        resolver.rememberVerifiedAddress(Uri.parse(baseUrl), tlsAddress);
        _discoveredBaseUrl = null;
      } else {
        _discoveredBaseUrl = candidate;
      }
      // Only the read probe can advance to another candidate. Do not swallow
      // an error from the selected request and silently choose another board.
      return await request(tlsAddress == null ? candidate : baseUrl);
    }
    throw SocketException(
      '${Uri.parse(baseUrl).host} is unreachable and no matching verified route replied',
    );
  }

  Future<Set<String>> _discoverSsdp() async {
    final socket = await RawDatagramSocket.bind(InternetAddress.anyIPv4, 0);
    final results = <String>{};
    void receive(Datagram packet) {
      final text = utf8.decode(packet.data, allowMalformed: true);
      final lines = const LineSplitter().convert(text);
      if (lines.isEmpty ||
          !lines.first.toUpperCase().startsWith('HTTP/1.1 200')) {
        return;
      }
      for (final line in lines.skip(1)) {
        final separator = line.indexOf(':');
        if (separator < 0 ||
            line.substring(0, separator).trim().toLowerCase() != 'location') {
          continue;
        }
        final location = Uri.tryParse(line.substring(separator + 1).trim());
        if (location?.host.isEmpty ?? true) continue;
        results.add(
          location!
              .replace(path: '', query: null, fragment: null)
              .toString()
              .replaceFirst(RegExp(r'/$'), ''),
        );
      }
    }

    const request =
        'M-SEARCH * HTTP/1.1\r\n'
        'HOST: 239.255.255.250:1900\r\n'
        'MAN: "ssdp:discover"\r\n'
        'MX: 1\r\n'
        'ST: ssdp:all\r\n\r\n';
    await collectDiscoveryDatagrams(
      events: socket,
      receive: socket.receive,
      close: socket.close,
      onPacket: receive,
      window: const Duration(milliseconds: 1400),
      send: () {
        socket.multicastHops = 2;
        socket.send(
          utf8.encode(request),
          InternetAddress('239.255.255.250'),
          1900,
        );
      },
    );
    return results;
  }
}

class AuthenticationRequired extends FormatException {
  const AuthenticationRequired() : super('Sign in required');
}

class StaleConnection implements Exception {
  const StaleConnection();
  @override
  String toString() =>
      'Connection changed; obsolete board replies and unsent commands were discarded.';
}

class ConnectionStopped implements Exception {
  const ConnectionStopped();
  @override
  String toString() => 'Disconnected; press Connect to resume device access.';
}

class OperationUncertain implements Exception {
  const OperationUncertain([this.detail]);
  final String? detail;
  @override
  String toString() =>
      '${detail == null ? '' : '$detail '}Command outcome unknown; it was not retried. Refresh device state before deciding whether to send again.';
}

class SupersededRead implements Exception {
  const SupersededRead();
  @override
  String toString() =>
      'Read superseded by a command; waiting for fresh device state.';
}
