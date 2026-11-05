import 'dart:async';
import 'dart:convert';

import 'protocol_opcodes.dart';

import 'package:flutter/services.dart';

import 'models.dart';
import 'peer_identity.dart';
import 'peer_policy.dart';
import 'cell_assignments.dart';
import 'pixel_diagnostic.dart';
import 'board_sound.dart';
import 'panel_event.dart';
import 'command_readback.dart';
import 'game_settings_command.dart';
import 'led_backend.dart';
import 'command_wire.dart';

/// Same fixed COBS/CRC-8 envelope as BoardCore::Protocol and the Rust USB actor.
class UsbFrame {
  const UsbFrame(this.opcode, this.sequence, this.payload);
  final int opcode, sequence;
  final List<int> payload;
  static int crc(Iterable<int> bytes) {
    var value = 0;
    for (final byte in bytes) {
      value ^= byte;
      for (var bit = 0; bit < 8; bit++) {
        value = ((value << 1) ^ (value & 128 != 0 ? 7 : 0)) & 255;
      }
    }
    return value;
  }

  Uint8List encode() {
    if (payload.length > 48 || payload.any((b) => b < 0 || b > 255)) {
      throw const FormatException('USB payload exceeds 48 bytes');
    }
    if (opcode < 0 || opcode > 255 || sequence < 0 || sequence > 255) {
      throw const FormatException('USB opcode/sequence is not a byte');
    }
    final raw = [PakarOpcode.magic, PakarOpcode.reserved, opcode, sequence, payload.length, ...payload];
    raw.add(crc(raw));
    final out = [0];
    var at = 0, code = 1;
    for (final byte in raw) {
      if (byte == 0) {
        out[at] = code;
        at = out.length;
        out.add(0);
        code = 1;
      } else {
        out.add(byte);
        code++;
      }
    }
    out[at] = code;
    out.add(0);
    return Uint8List.fromList(out);
  }

  static UsbFrame? decode(List<int> encoded) {
    if (encoded.isEmpty ||
        encoded.length > 55 ||
        encoded.any((value) => value < 1 || value > 255)) {
      return null;
    }
    final raw = <int>[];
    var at = 0;
    while (at < encoded.length) {
      final code = encoded[at++];
      if (code < 1 || at + code - 1 > encoded.length) {
        return null;
      }
      raw.addAll(encoded.sublist(at, at + code - 1));
      at += code - 1;
      if (code != 255 && at < encoded.length) {
        raw.add(0);
      }
    }
    if (raw.length < 6 ||
        raw[0] != PakarOpcode.magic ||
        raw[1] != PakarOpcode.reserved ||
        raw[4] > 48 ||
        raw.length != raw[4] + 6 ||
        crc(raw.take(raw.length - 1)) != raw.last) {
      return null;
    }
    return UsbFrame(raw[2], raw[3], raw.sublist(5, raw.length - 1));
  }
}

class PakarUsbTransport {
  PakarUsbTransport({MethodChannel? channel, this.nativeEvents})
    : channel = channel ?? const MethodChannel('ir.digitalogic.pakar/usb');
  final MethodChannel channel;
  final Stream<dynamic>? nativeEvents;
  static Stream<dynamic>? _sharedNative;
  Stream<dynamic> get _native =>
      nativeEvents ??
      (_sharedNative ??= const EventChannel('ir.digitalogic.pakar/usb-events')
          .receiveBroadcastStream()
          .asBroadcastStream());
  final events = StreamController<Map<String, dynamic>>.broadcast(sync: true);
  final _pending = <int, Completer<UsbFrame>>{};
  final _expected = <int, Set<int>>{};
  final _encoded = <int>[];
  StreamSubscription<dynamic>? _subscription;
  Timer? _renew;
  int? _session;
  static int _nextNativeOwner = 0;
  int? _nativeOwner;
  int _epoch = 0, _sequence = 0, _nonce = 0, _demand = 0;
  bool _busy = false,
      _foreground = true,
      _overflow = false,
      _diagnosticPixels = false;
  Future<void>? _connecting;
  String _endpoint = '';
  DeviceStatus? latest;
  Map<String, dynamic>? table;
  Stopwatch? _captureAge;
  bool get ready => _session != null && latest != null;
  bool get busy => _busy;
  static String endpoint(int device, int interface) =>
      'usb://$device/$interface';
  static (int, int) address(String value) {
    final uri = Uri.tryParse(value);
    final device = int.tryParse(uri?.host ?? ''),
        port = int.tryParse(uri?.path.replaceFirst('/', '') ?? '');
    if (uri?.scheme != 'usb' ||
        device == null ||
        device < 0 ||
        port == null ||
        port < 0 ||
        uri!.hasQuery ||
        uri.hasFragment ||
        uri.userInfo.isNotEmpty ||
        uri.hasPort) {
      throw const FormatException('Select an attached USB CDC interface');
    }
    return (device, port);
  }

  Future<List<Map<String, dynamic>>> devices() async =>
      (await channel.invokeListMethod<dynamic>('devices') ?? [])
          .whereType<Map>()
          .map((v) => Map<String, dynamic>.from(v))
          .toList();

  Future<void> connect(
    String selected, {
    String expectedBoard = '',
    bool enforce = true,
  }) async {
    if (ready && _endpoint == selected) {
      return;
    }
    if (_connecting != null) {
      return _connecting;
    }
    final task = _connect(selected, expectedBoard, enforce);
    _connecting = task;
    try {
      await task;
    } finally {
      if (identical(_connecting, task)) {
        _connecting = null;
      }
    }
  }

  Future<void> _connect(
    String selected,
    String expectedBoard,
    bool enforce,
  ) async {
    final (device, interface) = address(selected);
    await disconnect();
    final owner = _epoch;
    final nativeOwner = ++_nextNativeOwner;
    _nativeOwner = nativeOwner;
    _subscription = _native.listen(_receive, onError: _fail);
    try {
      final opened = await channel
          .invokeMapMethod<String, dynamic>('connect', {
            'device_id': device,
            'interface': interface,
            'owner': nativeOwner,
          })
          .timeout(const Duration(seconds: 45));
      if (owner != _epoch || opened?['session'] is! int) {
        if (opened?['session'] is int) {
          await channel.invokeMethod<void>('disconnect', {
            'session': opened!['session'],
          });
        }
        throw const FormatException('USB owner retired');
      }
      _session = opened!['session'];
      _endpoint = selected;
      await PeerIdentity.current.initialize();
      final version = PeerVersion.parse(PeerIdentity.current.version);
      if (version == null || version.parts.any((p) => p > 255)) {
        throw const FormatException(
          'USB application release is not representable',
        );
      }
      final hello = await _exchange(PakarOpcode.hello, [3, ...version.parts], {PakarOpcode.helloResponse});
      if (hello.payload.length < 9 || hello.payload[1] != 2) {
        throw const FormatException('USB HELLO is not an ESP board');
      }
      final report = utf8.encode(
        PeerIdentity.current.reportJson(transport: 'usb', enforce: enforce),
      );
      for (var offset = 0; offset < report.length; offset += 44) {
        final end = (offset + 44).clamp(0, report.length);
        await _ack(PakarOpcode.peerReport, [
          ..._u16(report.length),
          ..._u16(offset),
          ...report.sublist(offset, end),
        ]);
      }
      final actual = await _snapshot();
      validatePeerBoard(
        actual.json,
        appVersion: PeerIdentity.current.version,
        enabled: enforce,
      );
      if (!actual.peer.known ||
          actual.json['board_id'] is! String ||
          !RegExp(r'^[0-9a-fA-F]{32}$').hasMatch('${actual.json['boot_id']}') ||
          expectedBoard.isNotEmpty &&
              actual.json['board_id'] != expectedBoard) {
        throw const FormatException(
          'USB board identity/capabilities do not match the selected board',
        );
      }
      if (!actual.peer.tableSelection &&
          !(actual.peer.role == 'first-wins' && actual.peer.has('leds'))) {
        throw const FormatException(
          'This Android USB build requires an acknowledged LED game board',
        );
      }
      latest = actual;
      _renew = Timer.periodic(const Duration(seconds: 1), (_) {
        if (!_busy && ready && latest!.peer.has('usb-demand')) {
          unawaited(
            _renewDemand().catchError((Object e) {
              _fail(e);
            }),
          );
        }
      });
      await _renewDemand();
    } on Object {
      await disconnect();
      rethrow;
    }
  }

  static List<int> _u16(int n) => [n & 255, n >> 8 & 255];
  static int _le(List<int> p, int at, int length) {
    var n = 0;
    for (var i = 0; i < length; i++) {
      n |= p[at + i] << (i * 8);
    }
    return n;
  }

  void _receive(dynamic raw) {
    if (raw is! Map || raw['session'] != _session) {
      return;
    }
    if (raw['type'] == 'disconnected') {
      _fail(
        const FormatException(
          'USB device detached; commands were not replayed',
        ),
      );
      return;
    }
    final bytes = raw['bytes'];
    if (bytes is! List<int>) {
      return;
    }
    for (final byte in bytes) {
      if (byte != 0) {
        if (_encoded.length < 55 && !_overflow) {
          _encoded.add(byte);
        } else {
          _encoded.clear();
          _overflow = true;
        }
        continue;
      }
      final frame = _overflow ? null : UsbFrame.decode(_encoded);
      _encoded.clear();
      _overflow = false;
      if (frame == null) {
        continue;
      }
      final pending = _pending[frame.sequence];
      if (pending != null &&
          (_expected[frame.sequence]!.contains(frame.opcode) ||
              frame.opcode == PakarOpcode.errorResponse)) {
        _pending.remove(frame.sequence);
        _expected.remove(frame.sequence);
        pending.complete(frame);
      } else if (ready &&
          frame.opcode == PakarOpcode.panelEvent &&
          (_captureAge?.elapsedMilliseconds ?? 8000) < 8000) {
        final value = _panelEvent(frame.payload);
        if (value != null) {
          events.add(value);
        }
      } else if (ready && (frame.sequence == 0 || frame.sequence == 254)) {
        if (frame.opcode == PakarOpcode.statusResponse ||
            frame.opcode == PakarOpcode.cellEvent ||
            frame.opcode == PakarOpcode.gameState) {
          events.add({
            'type': 'changed',
            'board_id': latest!.json['board_id'],
            'boot_id': latest!.json['boot_id'],
          });
        } else if (frame.opcode == PakarOpcode.panelEvent || frame.opcode == PakarOpcode.pcContractResponse) {
          events.add({
            'type': 'usb-peripheral',
            'opcode': frame.opcode,
            'payload': frame.payload,
            'board_id': latest!.json['board_id'],
            'boot_id': latest!.json['boot_id'],
          });
        }
      }
    }
  }

  Map<String, dynamic>? _panelEvent(List<int> payload) {
    const kinds = [
      'rest',
      'cell',
      'mode',
      'effect',
      'pixel',
      'network',
      'progress',
      'fault',
      'complete',
      'output',
      'menu',
      'scan',
      'state',
    ];
    if (payload.length < 16 ||
        payload[15] > 32 ||
        payload.length != 16 + payload[15] ||
        payload[8] >= kinds.length ||
        payload[9] > 2 ||
        payload.skip(16).any((b) => b < 32 || b > 126)) {
      return null;
    }
    final text = ascii.decode(payload.sublist(16));
    final event = <String, dynamic>{
      'type': 'panel-event',
      'board_id': latest!.json['board_id'],
      'boot_id': latest!.json['boot_id'],
      'event_id': _le(payload, 0, 4),
      'uptime_ms': _le(payload, 4, 4),
      'kind': kinds[payload[8]],
      'value': _le(payload, 13, 2),
      'text': text,
      'hardware_confirmed': false,
      'presentation': {
        'text': text.padRight(4).substring(0, 4),
        'lifetime_ms': _le(payload, 11, 2),
        'blink': ['steady', 'slow', 'double-fast'][payload[9]],
        'marquee': text.length > 4 ? 'once' : 'none',
        'priority': payload[10],
      },
    };
    return PanelEvent.decode(event) == null ? null : event;
  }

  Future<UsbFrame> _exchange(
    int op,
    List<int> payload,
    Set<int> replies,
  ) async {
    final session = _session, owner = _epoch;
    if (session == null || _pending.length >= 4) {
      throw const FormatException(
        'USB connection unavailable or busy; nothing sent',
      );
    }
    _sequence = _sequence % 253 + 1;
    final sequence = _sequence;
    final completer = Completer<UsbFrame>();
    _pending[sequence] = completer;
    _expected[sequence] = replies;
    // Observe a detach immediately, including while the native write is pending.
    final observed = completer.future;
    unawaited(observed.then<void>((_) {}, onError: (Object _) {}));
    try {
      final bytes = UsbFrame(op, sequence, payload).encode();
      final count = await channel
          .invokeMethod<int>('write', {'session': session, 'bytes': bytes})
          .timeout(const Duration(seconds: 2));
      if (count != bytes.length || owner != _epoch) {
        const error = FormatException(
          'USB write incomplete; outcome unknown, no replay',
        );
        _fail(error);
        throw error;
      }
      final frame = await observed.timeout(const Duration(seconds: 2));
      if (owner != _epoch) {
        throw const FormatException('USB owner retired');
      }
      if (frame.opcode == PakarOpcode.errorResponse) {
        throw FormatException(
          'Board rejected USB request 0x${op.toRadixString(16)} (code ${frame.payload.length > 1 ? frame.payload[1] : "unknown"}); not retried',
        );
      }
      return frame;
    } finally {
      _pending.remove(sequence);
      _expected.remove(sequence);
    }
  }

  Future<void> _ack(int op, List<int> p) async {
    final frame = await _exchange(op, p, {PakarOpcode.ack});
    if (frame.payload.length != 2 ||
        frame.payload[0] != op ||
        frame.payload[1] != 0) {
      throw const FormatException(
        'USB acknowledgement does not match; outcome unverified, no replay',
      );
    }
  }

  Future<DeviceStatus> _snapshot() async {
    _nonce = (_nonce + 1) & 255;
    final bytes = <int>[], started = Stopwatch()..start();
    var token = 0, total = 0;
    while (total == 0 || bytes.length < total) {
      if (started.elapsed > const Duration(seconds: 8)) {
        throw const FormatException('Immutable USB snapshot expired');
      }
      // Lease renewal has its own correlated sequence and runs BETWEEN chunks.
      if (ready &&
          latest!.peer.has('usb-demand') &&
          _demandAge.elapsedMilliseconds >= 900) {
        await _renewDemand();
      }
      final f = await _exchange(
        PakarOpcode.getTableSnapshot,
        [
          _nonce,
          ..._u16(bytes.length),
          ...List.generate(4, (i) => token >> (8 * i) & 255),
        ],
        {PakarOpcode.tableSnapshotResponse},
      );
      final p = f.payload;
      if (p.length < 10 || p[0] != _nonce || _le(p, 7, 2) != bytes.length) {
        throw const FormatException('USB snapshot chunk does not match');
      }
      final nextToken = _le(p, 1, 4), nextTotal = _le(p, 5, 2);
      if (nextToken == 0 ||
          nextTotal < 1 ||
          nextTotal > 16384 ||
          token != 0 && (token != nextToken || total != nextTotal)) {
        throw const FormatException('USB snapshot capture changed');
      }
      token = nextToken;
      total = nextTotal;
      bytes.addAll(p.skip(9));
      if (bytes.length > total) {
        throw const FormatException('USB snapshot overrun');
      }
    }
    final value = jsonDecode(utf8.decode(bytes));
    if (value is! Map<String, dynamic> ||
        value['status'] is! Map<String, dynamic>) {
      throw const FormatException('Malformed USB table snapshot');
    }
    final capturedStatus = Map<String, dynamic>.from(value['status']);
    if (value['status_partial'] == true &&
        value['omitted_fields'] is List &&
        (value['omitted_fields'] as List).length == 1 &&
        (value['omitted_fields'] as List).single ==
            'capabilities.application.icon_png_base64' &&
        latest?.json['board_id'] == capturedStatus['board_id'] &&
        latest?.json['boot_id'] == capturedStatus['boot_id']) {
      final previous = (latest?.json['capabilities'] as Map?)?['application'];
      final capabilities = capturedStatus['capabilities'];
      final application = capabilities is Map
          ? capabilities['application']
          : null;
      if (application is Map &&
          previous is Map &&
          application['id'] == previous['id'] &&
          !application.containsKey('icon_png_base64') &&
          previous['icon_png_base64'] is String) {
        capturedStatus['capabilities'] = {
          ...capabilities as Map,
          'application': {
            ...application,
            'icon_png_base64': previous['icon_png_base64'],
          },
        };
      }
    }
    final status = DeviceStatus(capturedStatus);
    if (status.peer.has('pixel-group-layout') && status.pixelGroups == null) {
      throw const FormatException('USB pixel group layout is malformed');
    }
    if (latest != null &&
        (latest!.json['board_id'] != status.json['board_id'] ||
            latest!.json['boot_id'] != status.json['boot_id'])) {
      throw const FormatException(
        'USB board rebooted or was replaced; reconnect explicitly',
      );
    }
    // Mapping/cell/RGB remain actual captured values, never guessed defaults.
    final keys =
        status.peer.known &&
        status.peer.role == 'first-wins' &&
        status.peer.has('leds');
    if (keys) {
      final physical = value['physical'];
      if (physical is! List ||
          physical.length != 18 ||
          physical.any((p) => p is! int || p < 1 || p > 18) ||
          physical.toSet().length != 18) {
        throw const FormatException('Invalid USB key group mapping');
      }
    } else {
      CellMap.fromJson(value);
    }
    // The first-wins image intentionally has no legacy table cells. Its
    // authoritative group states, and current table semantic states, live in
    // cell_states. Never fill missing rows or turn an unknown state into idle.
    final semantic = status.json.containsKey('cell_states');
    final cells = status.json[semantic ? 'cell_states' : 'cells'];
    final allowedStates = semantic
        ? const ['idle', 'lost', 'won', 'off']
        : const ['white', 'red', 'won', 'off'];
    if (cells is! List ||
        cells.length != (keys ? 18 : 80) ||
        cells.any((v) => !allowedStates.contains(v))) {
      throw const FormatException('USB table state incomplete');
    }
    if (value['pixels'] is Map<String, dynamic>) {
      status.json['pixel_frame'] = value['pixels'];
    }
    if (value['pixel_frame_partial'] == true &&
        (value['pixels'] as Map?)?['pending'] != true &&
        _foreground &&
        (_diagnosticPixels || _demand & 8 != 0) &&
        ready &&
        status.peer.has('usb-api')) {
      final owner = _epoch;
      final full = await _apiJson('GET', '/api/pixels/frame', {
        'group_layout_revision': status.pixelGroups!.revision,
      }, () => owner == _epoch && ready);
      final frame = full['pixel_frame'];
      if (full['board_id'] != status.json['board_id'] ||
          full['boot_id'] != status.json['boot_id'] ||
          frame is! Map<String, dynamic> ||
          status.frameGroups(frame)?.sameAs(status.pixelGroups!) != true ||
          frame['pending'] == true) {
        throw const FormatException(
          'USB RGB capture differs from current peer/layout',
        );
      }
      final combined = status.withPixelFrameEvent({
        'type': 'pixel-frame',
        'pixel_preview_requested': true,
        'board_id': full['board_id'],
        'boot_id': full['boot_id'],
        'pixel_frame': frame,
      });
      // The compact frame may carry the same revision. Validate independently
      // without preserving absent RGB from an earlier observation.
      final isolated = DeviceStatus({...status.json}..remove('pixel_frame'));
      final validated =
          combined ??
          isolated.withPixelFrameEvent({
            'type': 'pixel-frame',
            'pixel_preview_requested': true,
            'board_id': full['board_id'],
            'boot_id': full['boot_id'],
            'pixel_frame': frame,
          });
      if (validated == null) {
        throw const FormatException('USB RGB capture is malformed');
      }
      status.json['pixel_frame'] = validated.json['pixel_frame'];
    }
    table = value;
    latest = status;
    _captureAge = started;
    return status;
  }

  Future<T> _exclusive<T>(Future<T> Function() action) async {
    if (_busy) {
      throw const FormatException(
        'USB operation already active; nothing queued or replayed',
      );
    }
    _busy = true;
    try {
      return await action();
    } finally {
      _busy = false;
      _diagnosticPixels = false;
    }
  }

  Future<DeviceStatus> status() => _exclusive(_snapshot);
  final _demandAge = Stopwatch()..start();
  Future<void> _renewDemand() async {
    if (!ready || !latest!.peer.has('usb-demand')) {
      return;
    }
    final flags = _foreground ? _demand | (_diagnosticPixels ? 8 : 0) : 0;
    final response = await _exchange(
      PakarOpcode.hostDemand,
      [..._u16(flags), ..._u16(flags == 0 ? 0 : 3000)],
      {PakarOpcode.hostDemandResponse},
    );
    if (response.payload.length != 4 ||
        _le(response.payload, 0, 2) != flags ||
        (flags == 0
            ? _le(response.payload, 2, 2) != 0
            : _le(response.payload, 2, 2) == 0) ||
        _le(response.payload, 2, 2) > 3000) {
      throw const FormatException('USB observation lease readback mismatch');
    }
    _demandAge.reset();
  }

  void demand(int flags, {required bool foreground}) {
    if (flags & ~15 != 0) {
      throw ArgumentError.value(flags);
    }
    if (_demand == flags && _foreground == foreground) {
      return;
    }
    _demand = flags;
    _foreground = foreground;
    if (ready && !_busy) {
      unawaited(
        _renewDemand().catchError((Object e) {
          _fail(e);
        }),
      );
    }
  }

  // Reuse the common boot-fenced USB API transaction for additive diagnostic
  // options. Its mutation executes once; all subsequent packets are read-only.
  // The HTTP form wrapper is deliberately NOT used: USB body is the command.
  Future<Map<String, dynamic>> _apiJson(
    String method,
    String path,
    Map<String, dynamic> body,
    bool Function() authorize,
  ) async {
    final owner = _epoch;
    void admit() {
      if (owner != _epoch || !ready || !authorize()) {
        throw const FormatException('USB diagnostic owner retired; no replay');
      }
    }

    admit();
    final before = latest!;
    admit();
    if (!before.peer.has('usb-api')) {
      throw const FormatException(
        'Advanced diagnostics require USB API support; update the board firmware. Nothing was sent.',
      );
    }
    final boot = '${before.json['boot_id']}';
    final fence = [
      for (var i = 0; i < 32; i += 2)
        int.parse(boot.substring(i, i + 2), radix: 16),
    ];
    final bytes = utf8.encode(
      jsonEncode({
        'method': method,
        'path': path,
        'board_id': before.json['board_id'],
        'boot_id': boot,
        'body': body,
      }),
    );
    if (bytes.length > 4096) {
      throw const FormatException(
        'USB diagnostic request exceeds the bounded API size',
      );
    }
    final clock = Stopwatch()..start();
    var token = 0;
    List<int> tokenBytes() => List.generate(4, (i) => token >> (i * 8) & 255);
    Future<List<int>> exchange(
      int action,
      List<int> payload, {
      int? total,
      int offset = 0,
      bool opening = false,
    }) async {
      admit();
      if (clock.elapsed > const Duration(seconds: 15)) {
        throw const FormatException(
          'USB diagnostic response timed out; outcome unknown, no replay',
        );
      }
      if (_demandAge.elapsedMilliseconds >= 900) await _renewDemand();
      admit();
      final frame = await _exchange(PakarOpcode.usbApi, [action, ...payload], {PakarOpcode.usbApiResponse});
      admit();
      final p = frame.payload;
      if (p.length < 9 ||
          p[0] != action ||
          _le(p, 1, 4) == 0 ||
          (!opening && _le(p, 1, 4) != token) ||
          _le(p, 7, 2) != offset ||
          (total != null && _le(p, 5, 2) != total)) {
        throw const FormatException(
          'USB API chunk identity differs; no replay',
        );
      }
      return p;
    }

    try {
      final opened = await exchange(
        0,
        [...fence, ..._u16(bytes.length)],
        total: 0,
        opening: true,
      );
      token = _le(opened, 1, 4);
      for (var offset = 0; offset < bytes.length; offset += 41) {
        await exchange(1, [
          ...tokenBytes(),
          ..._u16(offset),
          ...bytes.skip(offset).take(41),
        ], total: 0);
      }
      // Command expiry is based on the board clock. Never execute a staged
      // request after its admission window, even if a slow USB link recovered.
      if (clock.elapsed >= const Duration(seconds: 7)) {
        throw const FormatException(
          'USB diagnostic admission expired before execution; nothing replayed',
        );
      }
      final executed = await exchange(2, tokenBytes());
      final total = _le(executed, 5, 2);
      if (total < 1 || total > 8192 || executed.length != 9) {
        throw const FormatException(
          'USB API response length invalid; outcome unknown, no replay',
        );
      }
      final response = <int>[];
      while (response.length < total) {
        final part = await exchange(
          3,
          [...tokenBytes(), ..._u16(response.length)],
          total: total,
          offset: response.length,
        );
        if (part.length == 9 || response.length + part.length - 9 > total) {
          throw const FormatException(
            'USB API response overrun or empty chunk; no replay',
          );
        }
        response.addAll(part.skip(9));
      }
      final decoded = jsonDecode(utf8.decode(response));
      if (decoded is! Map<String, dynamic>) {
        throw const FormatException('USB API command receipt invalid');
      }
      if (path != '/api/command' &&
          (decoded['accepted'] == false ||
              decoded['error'] is String &&
                  (decoded['error'] as String).isNotEmpty)) {
        throw FormatException(
          'USB API rejected: ${decoded['error'] ?? 'not accepted'}; no replay',
        );
      }
      return decoded;
    } finally {
      if (token != 0 && owner == _epoch && ready && authorize()) {
        try {
          await _ack(PakarOpcode.usbApi, [4, ...tokenBytes()]);
        } on Object {
          /* No replay. */
        }
      }
    }
  }

  Future<Map<String, dynamic>> pixelGroupsApi({
    Map<String, dynamic>? configuration,
    required bool Function() authorize,
  }) => _exclusive(() async {
    if (!ready ||
        !authorize() ||
        !latest!.peer.has('pixel-group-layout') ||
        !latest!.peer.has('usb-api')) {
      throw const FormatException(
        'USB pixel group configuration is unavailable',
      );
    }
    return _apiJson(
      configuration == null ? 'GET' : 'POST',
      '/api/pixels/groups',
      configuration ?? const {},
      authorize,
    );
  });

  Future<Map<String, dynamic>> networkApi(
    String method,
    String path,
    Map<String, dynamic> body, {
    required bool Function() authorize,
  }) => _exclusive(() async {
    if (!ready || !authorize() || !latest!.peer.has('network-tools')) {
      throw const FormatException(
        'Network tools are not advertised by this board',
      );
    }
    const reads = {
      '/api/network/tools',
      '/api/time',
      '/api/wifi/scan',
      '/api/network',
    };
    const writes = {'/api/network/tools', '/api/network/probe', '/api/network'};
    if (!(method == 'GET'
            ? reads
            : method == 'POST'
            ? writes
            : const <String>{})
        .contains(path)) {
      throw const FormatException('Unsupported USB network operation');
    }
    return _apiJson(method, path, body, authorize);
  });

  /// The common boot-fenced API transaction also carries correction settings.
  /// Preview stays RAM-only; only the explicit non-preview POST persists.
  Future<Map<String, dynamic>> pixelCorrectionsApi({
    Map<String, dynamic>? configuration,
    bool preview = false,
    required bool Function() authorize,
  }) => _exclusive(() async {
    if (!ready ||
        !authorize() ||
        !latest!.peer.has('usb-api') ||
        !latest!.peer.has('pixel-corrections') ||
        (preview && !latest!.peer.has('pixel-corrections-preview')) ||
        (preview && configuration == null)) {
      throw const FormatException(
        'USB pixel corrections are unavailable; nothing was sent',
      );
    }
    return _apiJson(
      configuration == null ? 'GET' : 'POST',
      preview ? '/api/pixels/corrections/preview' : '/api/pixels/corrections',
      configuration ?? const {},
      authorize,
    );
  });

  Future<DeviceStatus> _advancedDiagnostic(
    PixelProbe probe,
    bool off,
    bool Function() authorize,
  ) async {
    final owner = _epoch;
    void admit() {
      if (owner != _epoch || !ready || !authorize()) {
        throw const FormatException('USB diagnostic owner retired; no replay');
      }
    }

    admit();
    _diagnosticPixels = true;
    await _renewDemand();
    final before = await _snapshot();
    admit();
    final fields = probe.commandFields(before, off: off);
    final packet = commandPacket(before, '/api/led/diagnostic', fields);
    final decoded = await _apiJson(
      'POST',
      '/api/command',
      Map<String, dynamic>.from(jsonDecode(packet) as Map),
      authorize,
    );
    if (decoded['status_partial'] == true && decoded['accepted'] == true) {
      // Only the explicit compact USB receipt permits a merge. Identity is
      // checked BEFORE inheriting any old state. Fresh diagnostic/frame data
      // replaces its old value entirely; old RGB must not masquerade as live.
      final fragment = decoded['status'];
      if (fragment is! Map<String, dynamic> ||
          fragment['board_id'] != before.json['board_id'] ||
          fragment['boot_id'] != before.json['boot_id'] ||
          fragment['capabilities'] is! Map ||
          fragment['pixels'] is! int ||
          fragment['led_output'] is! Map ||
          !fragment.containsKey('pixel_frame') ||
          (fragment['pixel_frame'] != null &&
              fragment['pixel_frame'] is! Map)) {
        throw const FormatException(
          'USB partial diagnostic receipt lacks same-peer output state; no replay',
        );
      }
      final output = fragment['led_output'] as Map;
      if (output['diagnostic'] is! Map) {
        throw const FormatException(
          'USB partial diagnostic specification is missing; no replay',
        );
      }
      decoded['status'] = <String, dynamic>{
        ...before.json,
        ...fragment,
        'led_output': <String, dynamic>{
          if (before.json['led_output'] is Map)
            ...Map<String, dynamic>.from(before.json['led_output'] as Map),
          ...Map<String, dynamic>.from(output),
        },
      };
    }
    var actual = verifyCommandReply(
      decoded,
      packet,
      '/api/led/diagnostic',
      fields,
    );
    final prior = PixelDiagnosticObservation.read(before)?.revision;
    final settle = Stopwatch()..start();
    while (!matchesDiagnosticObservation(
          actual,
          probe,
          off: off,
          afterRevision: prior,
        ) &&
        settle.elapsed < const Duration(seconds: 3)) {
      admit();
      final observed = PixelDiagnosticObservation.read(actual);
      if (observed != null &&
          (prior == null || observed.revision > prior) &&
          (!observed.probe
                  .copyWith(index: probe.index, group: probe.group)
                  .sameAs(probe) ||
              observed.off != off)) {
        throw const FormatException(
          'USB diagnostic was superseded by another command; no replay',
        );
      }
      await Future<void>.delayed(const Duration(milliseconds: 30));
      admit();
      actual = await _snapshot();
    }
    admit();
    if (!matchesDiagnosticObservation(
      actual,
      probe,
      off: off,
      afterRevision: prior,
    )) {
      throw const FormatException(
        'USB diagnostic accepted but completed output is unconfirmed; no replay',
      );
    }
    latest = actual;
    return actual;
  }

  Future<DeviceStatus> command(
    String path,
    Map<String, String> values, {
    required bool Function() authorize,
  }) => _exclusive(() async {
    if (!ready || !authorize()) {
      throw const FormatException('USB board not ready; nothing sent');
    }
    final before = latest!, features = before.peer;
    if (path.startsWith('/api/minigame/')) {
      if (!features.allows(path, values) || !features.has('usb-api')) {
        throw const FormatException('USB minigame command unavailable');
      }
      final packet = commandPacket(before, path, values);
      final reply = await _apiJson(
        'POST',
        '/api/command',
        Map<String, dynamic>.from(jsonDecode(packet) as Map),
        authorize,
      );
      final actual = verifyCommandReply(reply, packet, path, values);
      if (!authorize()) {
        throw const FormatException('USB minigame owner retired');
      }
      latest = actual;
      return actual;
    }
    const gamePaths = {
      '/api/game/settings',
      '/api/game/reset',
      '/api/game/press',
      '/api/brightness',
      '/api/effect',
      '/api/mode',
      '/api/led/output',
      '/api/led/diagnostic',
    };
    final keyOperation =
        features.known &&
        features.role == 'first-wins' &&
        features.has('leds') &&
        gamePaths.contains(path);
    if ((!features.tableSelection && !keyOperation) ||
        before.json['programming_latch'] == true) {
      throw const FormatException(
        'USB table controls unavailable for this board role or during programming',
      );
    }
    if (path == '/api/led/diagnostic') {
      final probe = PixelProbe.fromCommandFields(before, values);
      if (probe.extended || features.has('pixel-group-layout')) {
        if (!features.has('usb-api') || !features.has('usb-demand')) {
          throw const FormatException(
            'Advanced diagnostics require USB API support; update the board firmware. Nothing was sent.',
          );
        }
        return _advancedDiagnostic(
          probe,
          _boolean(values['off']) == 1,
          authorize,
        );
      }
    }
    int op;
    List<int> payload;
    int integer(String key, int low, int high) {
      final n = int.tryParse(values[key] ?? '');
      if (n == null || n < low || n > high) {
        throw FormatException('Invalid $key');
      }
      return n;
    }

    int state(String? v) => switch (v) {
      'white' || 'idle' => 0,
      'red' || 'lost' => 1,
      'won' => 3,
      'off' => 2,
      _ => throw const FormatException('Invalid cell state'),
    };
    switch (path) {
      case '/api/game/press':
        if (!features.has('game-press') ||
            !before.avrOnline ||
            before.json['key_mask_applied'] != true) {
          throw const FormatException(
            'Acknowledged AVR game press unavailable',
          );
        }
        op = PakarOpcode.gamePress;
        payload = [integer('key', 1, 9)];
      case '/api/game/settings':
        if (values.containsKey('presentation')) {
          if (!features.has('game-presentation') || values.length != 1) {
            throw const FormatException(
              'Save presentation separately over USB',
            );
          }
          final patch = gameSettingsPatch(values)['presentation'] as Map;
          final current = (before.json['key_game'] as Map?)?['presentation'];
          if (current is! Map) {
            throw const FormatException('Presentation readback unavailable');
          }
          op = PakarOpcode.keyPresentation;
          if (patch.containsKey('groups')) {
            final groups = patch['groups'] as List;
            if (patch.length != 1 || groups.length != 1) {
              throw const FormatException('Save one group at a time over USB');
            }
            final selected = groups.single as Map;
            final existing = (current['groups'] as List).where(
              (g) => g['id'] == selected['id'],
            );
            if (existing.length != 1) {
              throw const FormatException('Group readback unavailable');
            }
            final group = <String, dynamic>{
              ...Map<String, dynamic>.from(existing.single as Map),
              ...Map<String, dynamic>.from(selected),
            };
            presentationPatch({
              'groups': [group],
            });
            payload = [
              1,
              group['id'] as int,
              group['override_colors'] == true ? 1 : 0,
              for (final key in ['win_rgb', 'lose_rgb'])
                for (var byte = 0; byte < 3; byte++)
                  ((group[key] as int) >> (byte * 8)) & 255,
              group['win_motion'] as int,
              group['lose_motion'] as int,
            ];
          } else {
            final timing = {
              for (final key in presentationLimits.keys)
                key: patch[key] ?? current[key],
            };
            presentationPatch(timing);
            payload = [
              0,
              timing['flashes'] as int,
              for (final key in [
                'flash_on_ms',
                'flash_off_ms',
                'fade_ms',
                'pulse_ms',
                'chase_ms',
              ])
                ..._u16(timing[key] as int),
            ];
          }
          break;
        }
        if (features.role != 'first-wins' ||
            !features.has('usb-key-settings')) {
          throw const FormatException('USB key settings not acknowledged');
        }
        final hasMelodies = gameMelodyPatch(values);
        if (hasMelodies && !features.has('game-melodies')) {
          throw const FormatException('USB game melodies not acknowledged');
        }
        op = PakarOpcode.keySettings;
        payload = List.filled(hasMelodies ? 19 : 17, 0);
        const fields = [
          'enabled_mask',
          'idle_rgb',
          'win_rgb',
          'lose_rgb',
          'extended_effects',
          'auto_reset_ms',
          'winner_cue',
          'reset_cue',
        ];
        const offsets = [1, 3, 6, 9, 12, 13, 17, 18],
            widths = [2, 3, 3, 3, 1, 4, 1, 1];
        final patch = gameSettingsPatch(values);
        if (patch.keys.any((field) => !fields.contains(field))) {
          throw const FormatException(
            'This game setting is not supported over USB',
          );
        }
        for (var i = 0; i < fields.length; i++) {
          final selected = patch[fields[i]];
          if (selected == null) continue;
          final value = selected is bool
              ? (selected ? 1 : 0)
              : selected is String
              ? (selected == 'silent' ? 255 : audioCueNames.indexOf(selected))
              : selected as int;
          payload[0] |= 1 << i;
          for (var byte = 0; byte < widths[i]; byte++) {
            payload[offsets[i] + byte] = (value >> (byte * 8)) & 255;
          }
        }
      case '/api/cell':
        if (!features.has('leds')) {
          throw const FormatException('USB LEDs not acknowledged');
        }
        op = PakarOpcode.cellAction;
        payload = [
          integer('id', 1, 80),
          state(values['state']),
          _effect(values['effect']),
        ];
      case '/api/cells':
        if (!features.has('cell-batch')) {
          throw const FormatException('USB cell-batch not acknowledged');
        }
        final ids = _ids(values['ids']);
        final bitmap = List.filled(10, 0);
        for (final id in ids) {
          bitmap[(id - 1) ~/ 8] |= 1 << ((id - 1) % 8);
        }
        op = PakarOpcode.cellsAction;
        payload = [
          state(values['state']),
          _effect(values['effect']),
          ...bitmap,
        ];
      case '/api/cells/assignments':
        if (!features.has('cell-assignments')) {
          throw const FormatException('USB assignments not acknowledged');
        }
        final assignments = CellAssignments.fromRows(
          jsonDecode(values['assignments'] ?? 'null'),
        );
        final bitmap = List.filled(10, 0), packed = List.filled(20, 0);
        for (final entry in assignments.values.entries) {
          final i = entry.key - 1;
          bitmap[i ~/ 8] |= 1 << (i % 8);
          packed[i ~/ 4] |= state(entry.value) << ((i % 4) * 2);
        }
        op = PakarOpcode.cellAssignments;
        payload = [...bitmap, ...packed];
      case '/api/cells/lock':
        if (!features.has('cell-lock')) {
          throw const FormatException('USB lock not acknowledged');
        }
        op = PakarOpcode.cellLock;
        payload = [_boolean(values['enabled'])];
      case '/api/brightness':
        op = PakarOpcode.setBrightness;
        payload = [integer('value', 1, 255)];
      case '/api/effect':
        op = PakarOpcode.setLedPattern;
        payload = [
          switch (values['value']) {
            'none' => 0,
            'rainbow' => 1,
            'fire' => 2,
            'cell-chase' => 3,
            _ => throw const FormatException('Unknown effect'),
          },
        ];
      case '/api/mode':
        if (values['value'] != before.mode) {
          throw const FormatException('This board has one baked-in game mode');
        }
        op = PakarOpcode.setMode;
        payload = [
          switch (values['value']) {
            'cell' => 1,
            'first-wins' => 2,
            'snake' => 3,
            _ => throw const FormatException('Unsupported mode'),
          },
        ];
      case '/api/game/reset':
        if (!features.has('reset-ack')) {
          throw const FormatException(
            'Acknowledged game reset is not supported by this firmware',
          );
        }
        op = PakarOpcode.gameReset;
        payload = [];
      case '/api/led/output':
        op = PakarOpcode.setLedOutput;
        if (values.containsKey('backend') && !features.allows(path, values)) {
          throw const FormatException(
            'This LED transmitter is not available on the connected board',
          );
        }
        payload = values.containsKey('backend')
            ? [
                3,
                switch (values['backend']) {
                  'gpio-static' => 1,
                  'rmt-dma' => 2,
                  _ => throw const FormatException('Unknown LED transmitter'),
                },
              ]
            : values.containsKey('mode')
            ? [
                2,
                switch (values['mode']) {
                  'static' => 0,
                  'effects' => 1,
                  _ => throw const FormatException('Unknown LED mode'),
                },
              ]
            : [_boolean(values['enabled'])];
      case '/api/led/diagnostic':
        if (!features.has('usb-pixel-diagnostic') ||
            !features.has('usb-demand')) {
          throw const FormatException('USB pixel diagnostics not acknowledged');
        }
        final pixels = diagnosticPixelCount(before),
            groupSize = diagnosticGroupSize(before);
        final index = integer('index', 0, pixels - 1),
            count = integer('count', 1, groupSize),
            rgb = integer('rgb', 0, 0xffffff);
        if (count != 1 && count != groupSize ||
            index + count > pixels ||
            count == groupSize && index % groupSize != 0) {
          throw const FormatException('Invalid pixel range');
        }
        _diagnosticPixels = true;
        await _renewDemand();
        op = PakarOpcode.pixelDiagnostic;
        payload = [
          ..._u16(index),
          count,
          rgb >> 16,
          (rgb >> 8) & 255,
          rgb & 255,
          _boolean(values['off']),
        ];
      default:
        throw const FormatException(
          'This operation is not available through USB; no network fallback attempted',
        );
    }
    final boot = '${before.json['boot_id']}';
    final fence = [
      for (var i = 0; i < 32; i += 2)
        int.parse(boot.substring(i, i + 2), radix: 16),
    ];
    if (!authorize()) {
      throw const FormatException(
        'USB command admission retired; nothing sent',
      );
    }
    // One physical dispatch only. ACK is not accepted as an applied state.
    final reply = await _exchange(PakarOpcode.tableCommand, [...fence, op, ...payload], {PakarOpcode.ack});
    if (reply.payload.length != 2 ||
        reply.payload[0] != op ||
        reply.payload[1] != 0) {
      throw const FormatException('USB command receipt mismatch; no replay');
    }
    var actual = await _snapshot();
    if (op == PakarOpcode.keySettings && gameMelodyPatch(values) ||
        op == PakarOpcode.setLedOutput && values.containsKey('backend')) {
      // The ESP persists first; AVR playback configuration settles on its next
      // service turn. Observe that receipt without replaying the mutation.
      final started = Stopwatch()..start();
      while (!commandReadbackMatches(actual, path, values) &&
          started.elapsed < const Duration(seconds: 3)) {
        if (!authorize() ||
            actual.json['board_id'] != before.json['board_id'] ||
            actual.json['boot_id'] != before.json['boot_id']) {
          throw const FormatException('Command owner retired; no replay');
        }
        if (op == PakarOpcode.setLedOutput) {
          final driver = LedBackendState.fromStatus(actual.json);
          if (driver?.failed == true) {
            throw FormatException(driver!.failureDescription);
          }
        }
        await Future<void>.delayed(const Duration(milliseconds: 30));
        actual = await _snapshot();
      }
    }
    if (op == PakarOpcode.pixelDiagnostic) {
      final start = Stopwatch()..start();
      while ((actual.json['pixel_frame'] as Map?)?['pending'] == true &&
          start.elapsed < const Duration(seconds: 5)) {
        await Future<void>.delayed(const Duration(milliseconds: 30));
        actual = await _snapshot();
      }
      final probe = PixelProbe(
        index: payload[0] | payload[1] << 8,
        group: payload[2] == diagnosticGroupSize(before),
        rgb: payload[3] << 16 | payload[4] << 8 | payload[5],
        pixels: diagnosticPixelCount(before),
        groupSize: diagnosticGroupSize(before),
      );
      if (!matchesDiagnosticFrame(actual, probe.frame(off: payload[6] == 1))) {
        throw const FormatException(
          'USB diagnostic published RGB differs or is pending; no replay',
        );
      }
    }
    if (!authorize()) {
      throw const FormatException(
        'USB command owner retired; outcome not claimed',
      );
    }
    if ((op == PakarOpcode.keySettings || op == PakarOpcode.keyPresentation) &&
            !commandReadbackMatches(actual, path, values) ||
        op == PakarOpcode.gameReset &&
            (actual.winner != 0 ||
                actual.effect != 'none' ||
                actual.ledOutputEnabled != true) ||
        op == PakarOpcode.cellAction && actual.json['cells'][payload[0] - 1] != values['state'] ||
        op == PakarOpcode.cellsAction &&
            _ids(values['ids'])
                .any((id) => actual.json['cells'][id - 1] != values['state']) ||
        op == PakarOpcode.cellLock && actual.json['red_lock'] != (payload[0] == 1) ||
        op == PakarOpcode.setBrightness && actual.brightness != payload[0] ||
        op == PakarOpcode.setLedPattern && actual.effect != values['value'] ||
        op == PakarOpcode.setMode && actual.mode != values['value'] ||
        op == PakarOpcode.setLedOutput && !commandReadbackMatches(actual, path, values) ||
        op == PakarOpcode.cellAssignments &&
            CellAssignments.fromRows(jsonDecode(values['assignments']!))
                .values
                .entries
                .any((e) => actual.json['cells'][e.key - 1] != e.value)) {
      throw const FormatException(
        'USB post-ACK readback differs; nothing replayed',
      );
    }
    return actual;
  });
  static int _boolean(String? value) => switch (value) {
    'true' || '1' => 1,
    'false' || '0' => 0,
    _ => throw const FormatException('Invalid boolean'),
  };
  static int _effect(String? value) => switch (value) {
    null || 'immediate' => 0,
    'fade' => 1,
    'pulse' => 2,
    _ => throw const FormatException('Unknown effect'),
  };
  static Set<int> _ids(String? value) {
    final result = <int>{};
    for (final token in (value ?? '').split(',')) {
      final range = token.split('-');
      final first = int.tryParse(range.first), last = int.tryParse(range.last);
      if (range.length > 2 ||
          first == null ||
          last == null ||
          first < 1 ||
          last > 80 ||
          last < first) {
        throw const FormatException('Invalid cell selection');
      }
      for (var id = first; id <= last; id++) {
        result.add(id);
      }
    }
    if (result.isEmpty) {
      throw const FormatException('Select cells');
    }
    return result;
  }

  Future<UsbFrame> contract(int opcode, List<int> payload) =>
      _exclusive(() async {
        if (!ready) {
          throw const FormatException('USB board not ready');
        }
        if (!ControllerOpcode.requests.contains(opcode)) {
          throw const FormatException('Unknown controller request; no dialect fallback');
        }
        return _exchange(PakarOpcode.pcContract, [opcode, ...payload], {PakarOpcode.pcContractResponse});
      });
  Future<void> cue(String cue) => _exclusive(() async {
    const names = [
      'welcome',
      'update-start',
      'update-success',
      'error',
      'winner',
      'finish',
      'lost',
      'incorrect',
      'rapid-error',
      'fault',
      'increase',
      'decrease',
      'welcome-puzzleboard',
      'welcome-40led',
      'attention',
    ];
    final index = names.indexOf(cue);
    if (index < 0 || latest?.audioCues.contains(cue) != true) {
      throw const FormatException('Melody not acknowledged by this USB board');
    }
    await _ack(PakarOpcode.audioCue, [index]);
  });
  Future<BoardSound> boardSound({bool? silent}) => _exclusive(() async {
    if (latest?.avrOnline != true ||
        latest?.peer.has('settings') != true ||
        latest?.peer.has('buzzer') != true) {
      throw const FormatException('USB AVR sound settings unavailable');
    }
    Future<BoardSound> read() async {
      final f = await _exchange(PakarOpcode.pcContract, [ControllerOpcode.getSettings], {PakarOpcode.pcContractResponse});
      if (f.payload.length != 23 || f.payload.first != ControllerOpcode.settingsResponse) {
        throw const FormatException('USB AVR settings response does not match');
      }
      return BoardSound.parse(BoardSound.encode(f.payload.skip(1)));
    }

    var result = await read();
    if (silent != null && (!result.persisted || result.silent != silent)) {
      if (latest?.json['programming_latch'] == true) {
        throw const FormatException('USB programming active');
      }
      final expected = result.withSilent(silent);
      final bytes = [
        for (var i = 0; i < expected.length; i += 2)
          int.parse(expected.substring(i, i + 2), radix: 16),
      ];
      final reply = await _exchange(PakarOpcode.pcContract, [ControllerOpcode.setSettings, ...bytes], {PakarOpcode.pcContractResponse});
      if (reply.payload.length != 3 ||
          reply.payload[0] != ControllerOpcode.ack ||
          reply.payload[1] != ControllerOpcode.setSettings ||
          reply.payload[2] != 0) {
        throw const FormatException('USB AVR settings ACK mismatch; no replay');
      }
      result = await read();
      if (!result.persisted || BoardSound.encode(result.bytes) != expected) {
        throw const FormatException('USB Silent readback differs; no replay');
      }
    }
    return result;
  });
  Future<void> display(Map<String, String> form) => _exclusive(() async {
    if (latest?.peer.has('text') != true || latest?.avrOnline != true) {
      throw const FormatException('USB text adapter not acknowledged');
    }
    final text = ascii.encode(form['text'] ?? '');
    if (text.length > 40 || text.any((b) => b < 32 || b > 126)) {
      throw const FormatException(
        'Display supports 40 printable ASCII characters',
      );
    }
    final speed = int.tryParse(form['speed_ms'] ?? '260'),
        hold = int.tryParse(form['hold_ms'] ?? '1500'),
        interval = int.tryParse(form['interval_seconds'] ?? '5');
    final repeat = switch (form['repeat'] ?? 'once') {
      'once' => 0,
      'loop' => 1,
      'interval' => 2,
      _ => -1,
    };
    if (speed == null ||
        speed < 80 ||
        speed > 5000 ||
        hold == null ||
        hold < 0 ||
        hold > 60000 ||
        interval == null ||
        interval < 1 ||
        interval > 255 ||
        repeat < 0) {
      throw const FormatException('Invalid display timing');
    }
    await _ack(PakarOpcode.displayText, [
      5,
      ..._u16(speed),
      text.length,
      repeat | (form['force_scroll'] == 'true' ? 128 : 0),
      ..._u16(hold),
      interval,
      ...text,
    ]);
  });
  void _fail(Object error) {
    final session = _session;
    ++_epoch;
    _session = null;
    latest = null;
    table = null;
    _captureAge = null;
    _renew?.cancel();
    _renew = null;
    for (final pending in _pending.values.toList()) {
      if (!pending.isCompleted) {
        pending.completeError(error);
      }
    }
    _pending.clear();
    _expected.clear();
    if (!events.isClosed) {
      events.addError(error);
    }
    if (session != null) {
      unawaited(
        channel
            .invokeMethod<void>('disconnect', {'session': session})
            .catchError((Object _) {}),
      );
    }
  }

  Future<void> disconnect() async {
    ++_epoch;
    final session = _session;
    final nativeOwner = _nativeOwner;
    _nativeOwner = null;
    _session = null;
    latest = null;
    table = null;
    _captureAge = null;
    _renew?.cancel();
    _renew = null;
    _encoded.clear();
    _overflow = false;
    for (final pending in _pending.values.toList()) {
      if (!pending.isCompleted) {
        pending.completeError(
          const FormatException('USB owner retired; no replay'),
        );
      }
    }
    _pending.clear();
    _expected.clear();
    await _subscription?.cancel();
    _subscription = null;
    if (session != null || nativeOwner != null) {
      try {
        await channel
            .invokeMethod<void>('disconnect', {
              'session': session,
              'owner': nativeOwner,
            })
            .timeout(const Duration(seconds: 1));
      } on Object {
        /* Physical owner is already retired. */
      }
    }
  }

  Future<void> close() async {
    await disconnect();
    await events.close();
  }
}
