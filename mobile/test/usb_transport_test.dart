import 'dart:async';
import 'dart:convert';

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:http/testing.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:pakarcontroller/api.dart';
import 'package:pakarcontroller/usb_transport.dart';
import 'package:pakarcontroller/lan_resolver.dart';
import 'package:pakarcontroller/pixel_diagnostic.dart';
import 'package:pakarcontroller/models.dart' show audioCueNames;
import 'package:pakarcontroller/protocol_opcodes.dart';

class UsbFixture {
  UsbFixture() {
    transport.events.stream.listen((_) {}, onError: (Object _) {});
  }
  final channel = const MethodChannel('test/pakar-usb');
  final stream = StreamController<dynamic>.broadcast(sync: true);
  late final transport = PakarUsbTransport(
    channel: channel,
    nativeEvents: stream.stream,
  );
  final writes = <UsbFrame>[];
  var session = 7, demand = 0, lease = 0, token = 1;
  bool partial = false,
      wrongAck = false,
      denied = false,
      corruptChunk = false,
      expiredLease = false;
  Uint8List capture = Uint8List(0);
  Map<String, dynamic>? pixels;
  bool pixelFramePartial = false;
  int get groupCount => status['pixels'] == 216 ? 18 : 80;
  final status = <String, dynamic>{
    'version': '0.2.8',
    'compatibility': {
      'enforced': true,
      'minimum_app': '0.2.8',
      'minimum_board': '0.2.8',
    },
    'hostname': 'ledboard',
    'board_id': 'esp32s3-fixture',
    'boot_id': List.filled(32, 'a').join(),
    'pixels': 480,
    'mode': 'cell',
    'brightness': 255,
    'effect': 'none',
    'red_lock': false,
    'http_peer_fencing': true,
    'cells': List.filled(80, 'white'),
    'avr_online': false,
    'led_output': {'enabled': true, 'mode': 'effects'},
    'capabilities': {
      'board_kind': 'table',
      'role': 'table-selection',
      'name': 'Table',
      'hostname': 'ledboard',
      'table': {'columns': 10, 'rows': 8, 'pixels_per_cell': 6},
      'modes': [
        {'id': 'cell', 'code': 1, 'kind': 'presentation', 'label': 'Table'},
      ],
      'acknowledged_features': [
        'table',
        'leds',
        'cell-batch',
        'cell-assignments',
        'cell-lock',
        'command-ack',
        'pixel-diagnostic',
        'usb-demand',
        'usb-pixel-diagnostic',
      ],
    },
  };
  static List<int> le(int n, int count) =>
      List.generate(count, (i) => n >> (i * 8) & 255);
  static int read(List<int> p, int at, int count) => List.generate(
    count,
    (i) => p[at + i] << (i * 8),
  ).fold(0, (a, b) => a | b);
  void reply(UsbFrame request, int op, List<int> p) {
    scheduleMicrotask(
      () => stream.add({
        'session': session,
        'bytes': UsbFrame(op, request.sequence, p).encode(),
      }),
    );
  }

  Future<dynamic> invoke(MethodCall call) async {
    if (call.method == 'devices') {
      return [
        {
          'device_id': 12,
          'interface': 0,
          'name': 'PakarController',
          'permission': true,
        },
      ];
    }
    if (call.method == 'connect') {
      if (denied) {
        throw PlatformException(code: 'usb_permission', message: 'Denied');
      }
      return {'session': session};
    }
    if (call.method == 'disconnect') return null;
    if (call.method != 'write') throw PlatformException(code: 'unexpected');
    final bytes = (call.arguments['bytes'] as Uint8List);
    final f = UsbFrame.decode(bytes.sublist(0, bytes.length - 1))!;
    writes.add(f);
    if (partial) return bytes.length - 1;
    switch (f.opcode) {
      case 1:
        reply(f, 0x81, [0, 2, 0, 0, 0, 0, 0, 2, 8]);
      case 0x61:
        reply(f, 0x80, [0x61, 0]);
      case 0x64:
        demand = read(f.payload, 0, 2);
        lease = read(f.payload, 2, 2);
        reply(f, 0xba, [...le(demand, 2), ...le(expiredLease ? 0 : lease, 2)]);
      case 0x5d:
        final offset = read(f.payload, 1, 2);
        if (offset == 0) {
          token++;
          capture = Uint8List.fromList(
            utf8.encode(
              jsonEncode({
                'status': status,
                'physical': List.generate(groupCount, (i) => i + 1),
                'names': List.generate(groupCount, (i) => '${i + 1}'),
                'pixels': demand & 8 != 0 ? pixels : null,
                if (pixelFramePartial && demand & 8 != 0) 'pixel_frame_partial': true,
              }),
            ),
          );
        }
        reply(f, 0xb6, [
          f.payload[0],
          ...le(token, 4),
          ...le(capture.length, 2),
          ...le(corruptChunk ? offset + 1 : offset, 2),
          ...capture.sublist(offset, (offset + 39).clamp(0, capture.length)),
        ]);
      case 0x5e:
        final op = f.payload[16], p = f.payload.sublist(17);
        if (op == 0x5f) {
          for (var i = 0; i < 80; i++) {
            if (p[2 + i ~/ 8] & (1 << (i % 8)) != 0) {
              (status['cells'] as List)[i] = ['white', 'red', 'off'][p[0]];
            }
          }
        } else if (op == 0x5c && p.length == 2 && p[0] == 3) {
          final backend = p[1] == 1 ? 'gpio-static' : 'rmt-dma';
          final driver = status['led_output']['driver'] as Map;
          driver['requested_backend'] = backend;
          driver['switch_pending'] = true;
          Future<void>.delayed(const Duration(milliseconds: 80), () {
            driver['backend'] = backend;
            driver['switch_pending'] = false;
          });
        } else if(op==0x67) {
          const fields=['enabled_mask','idle_rgb','win_rgb','lose_rgb','extended_effects','auto_reset_ms'];
          const offsets=[1,3,6,9,12,13],widths=[2,3,3,3,1,4];
          for(var i=0;i<6;i++) {if(p[0]&(1<<i)!=0) {(status['key_game'] as Map)[fields[i]]=i==4?p[12]!=0:read(p,offsets[i],widths[i]);}}
          for(var i=0;i<2;i++) {if(p[0]&(64<<i)!=0) {(status['key_game'] as Map)[i==0?'winner_cue':'reset_cue']=p[17+i]==255?'silent':audioCueNames[p[17+i]];}}
          if(p[0]&192!=0) status['game_audio_applied']=true;
        } else if (op == 0x65) {
          final probe = PixelProbe(
            index: read(p, 0, 2),
            group: p[2] == (status['pixels']==216?12:6),
            rgb: p[3] << 16 | p[4] << 8 | p[5],
            pixels: status['pixels'] as int,groupSize:status['pixels']==216?12:6,
          );
          pixels = {
            'revision': 1,
            'pending': false,
            'rgb': probe
                .frame(off: p[6] == 1)
                .map((b) => b.toRadixString(16).padLeft(2, '0'))
                .join(),
            'physical': List.generate(groupCount, (i) => i + 1),
          };
        }
        reply(f, 0x80, [wrongAck ? 0x51 : op, 0]);
      default:
        throw StateError('Unexpected opcode ${f.opcode}');
    }
    return bytes.length;
  }

  Future<void> close() async {
    await transport.close();
    await stream.close();
  }
}

void main() {
  test('opcode namespaces never fall back across valid-CRC frames', () {
    List<int> encoded(List<int> raw) {
      raw.add(UsbFrame.crc(raw));
      final out = <int>[0];
      var at = 0, code = 1;
      for (final value in raw) {
        if (value == 0) { out[at] = code; at = out.length; out.add(0); code = 1; }
        else { out.add(value); code++; }
      }
      out[at] = code;
      return out;
    }
    for (var discriminator = 1; discriminator < 256; discriminator++) {
      expect(UsbFrame.decode(encoded([165, discriminator, 0x11, 7, 1, 4])), isNull);
    }
    final current = UsbFrame(PakarOpcode.audioCue, 7, [4]).encode();
    expect(UsbFrame.decode(current.sublist(0, current.length-1))!.opcode, PakarOpcode.audioCue);
    expect(() => UsbFrame(0x111, 7, []).encode(), throwsFormatException);
    final optional = UsbFrame(0xfc, 7, []).encode();
    expect(UsbFrame.decode(optional.sublist(0, optional.length-1))!.opcode, 0xfc);
    expect(ControllerOpcode.requests.contains(ControllerOpcode.pwmSet), isTrue);
    expect(ControllerOpcode.requests.contains(ControllerOpcode.ack), isFalse);
  });
  test('COBS rejects embedded delimiters even with a valid envelope CRC', () {
    final raw = [165, 0, 1, 1, 0];
    raw.add(UsbFrame.crc(raw));
    expect(UsbFrame.decode([raw.length + 1, ...raw]), isNull);
    final valid = const UsbFrame(1, 1, []).encode();
    expect(UsbFrame.decode(valid.sublist(0, valid.length - 1))?.opcode, 1);
  });
  TestWidgetsFlutterBinding.ensureInitialized();
  setUp(() => SharedPreferences.setMockInitialValues({}));
  test('USB backend switch waits for active driver, sends once, and never uses network', () async {
    final fixture = UsbFixture();
    fixture.status['capabilities']['led_output'] = {
      'backend': 'gpio-static', 'rmt': false, 'dma': false,
      'available_backends': ['gpio-static', 'rmt-dma'], 'runtime_switch': true,
    };
    fixture.status['led_output']['backend_saved'] = true;
    fixture.status['led_output']['driver'] = {
      'backend': 'gpio-static', 'requested_backend': 'gpio-static',
      'available_backends': ['gpio-static', 'rmt-dma'], 'switch_pending': false,
      'switch_error': 0, 'error_code': 0, 'ready': true,
    };
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(fixture.channel, fixture.invoke);
    var http = 0, dns = 0;
    final api = PakarApi(
      baseUrl: 'usb://12/0', usb: fixture.transport,
      client: MockClient((_) async { http++; throw StateError('No HTTP'); }),
      resolver: LanResolver(lookup: (_) async { dns++; throw StateError('No DNS'); }),
    );
    try {
      await api.status();
      final result = await api.fastCommand('/api/led/output', {'backend': 'rmt-dma'}, authorize: () => true);
      expect(result?.json['led_output']['driver']['backend'], 'rmt-dma');
      expect(result?.json['led_output']['driver']['switch_pending'], false);
      final writes = fixture.writes.where((frame) => frame.opcode == 0x5e).toList();
      expect(writes.length, 1);
      expect(writes.single.payload.sublist(16), [0x5c, 3, 2]);
      expect(http, 0);
      expect(dns, 0);
    } finally {
      await api.close();
      await fixture.close();
    }
  });
  test('first-wins USB connects, saves partial settings and publishes a twelve-pixel group',() async {
    final f=UsbFixture();
    f.status['pixels']=216;f.status['cells']=<String>[];f.status['cell_states']=List.filled(18,'idle');f.status['mode']='first-wins';
    final caps=f.status['capabilities'] as Map;caps['role']='first-wins';
    (caps['acknowledged_features'] as List).addAll(<String>['key-settings','usb-key-settings','game-melodies']);
    f.status['key_game']=<String,dynamic>{'enabled_mask':3,'win_rgb':65280,'idle_rgb':16777215,'auto_reset_ms':0};
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger.setMockMethodCallHandler(f.channel,f.invoke);
    try {
      await f.transport.connect('usb://12/0');
      expect(f.transport.latest!.json['cells'], isEmpty);
      expect(f.transport.latest!.json['cell_states'], List.filled(18, 'idle'));
      final saved=await f.transport.command('/api/game/settings',{'enabled_mask':'15','auto_reset_ms':'10000'},authorize:()=>true);
      expect(saved.json['key_game']['enabled_mask'],15);expect(saved.json['key_game']['win_rgb'],65280);
      final melodies=await f.transport.command('/api/game/settings',{'winner_cue':'attention','reset_cue':'silent'},authorize:()=>true);
      expect(melodies.json['key_game']['winner_cue'],'attention');
      expect(melodies.json['key_game']['reset_cue'],'silent');
      expect(melodies.json['game_audio_applied'],true);
      await expectLater(f.transport.command('/api/game/settings',{'chase_period_ms':'900'},authorize:()=>true),throwsFormatException);
      final actual=await f.transport.command('/api/led/diagnostic',{'index':'204','count':'12','rgb':'65280','off':'false'},authorize:()=>true);
      expect((actual.json['pixel_frame']['rgb'] as String).length,216*6);
      expect((actual.json['pixel_frame']['rgb'] as String).substring(204*6),'00ff00'*12);
      await expectLater(f.transport.command('/api/led/diagnostic',{'index':'210','count':'12','rgb':'65280','off':'false'},authorize:()=>true),throwsFormatException);
    } finally {await f.close();}
  });
  for (final semantic in [false, true]) {
    test('USB table snapshot preserves won state with semantic=$semantic', () async {
      final fixture = UsbFixture();
      (fixture.status['cells'] as List)[4] = 'won';
      if (semantic) {
        fixture.status['cell_states'] = List.filled(80, 'idle')..[4] = 'won';
      }
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(fixture.channel, fixture.invoke);
      try {
        await fixture.transport.connect('usb://12/0');
        expect(fixture.transport.latest!.json['cells'][4], 'won');
        expect(fixture.transport.latest!.json.containsKey('cell_states'), semantic);
        if (semantic) {
          expect(fixture.transport.latest!.json['cell_states'][4], 'won');
        }
        expect(fixture.writes.where((frame) => frame.opcode == 0x5e), isEmpty);
      } finally {
        await fixture.close();
      }
    });
  }
  for (final malformed in [List.filled(79, 'idle'), List.filled(80, 'unknown')]) {
    test('USB malformed semantic state does not fall back to plausible legacy cells: ${malformed.length}/${malformed.first}', () async {
      final fixture = UsbFixture();
      fixture.status['cell_states'] = malformed;
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(fixture.channel, fixture.invoke);
      try {
        await expectLater(fixture.transport.connect('usb://12/0'), throwsFormatException);
        expect(fixture.writes.where((frame) => frame.opcode == 0x5e), isEmpty);
      } finally {
        await fixture.close();
      }
    });
  }
  test('USB COBS CRC matches fixed protocol and rejects corrupt/reserved/oversized frames', () {
    for (final payload in [
      <int>[],
      [0, 1, 0, 255],
      List.generate(48, (i) => i),
    ]) {
      final packet = UsbFrame(0x5e, 42, payload).encode();
      final frame = UsbFrame.decode(packet.sublist(0, packet.length - 1));
      expect(frame!.payload, payload);
      expect(frame.sequence, 42);
      expect(frame.opcode, 0x5e);
      packet[packet.length - 2] ^= 1;
      expect(UsbFrame.decode(packet.sublist(0, packet.length - 1)), isNull);
    }
    expect(UsbFrame.crc(ascii.encode('123456789')), 0xf4);
    expect(
      () => UsbFrame(1, 1, List.filled(49, 0)).encode(),
      throwsFormatException,
    );
  });
  test('Real channel session performs HELLO introduction snapshot and atomic batch without HTTP or DNS', () async {
    final fixture = UsbFixture();
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(fixture.channel, fixture.invoke);
    var http = 0, dns = 0;
    final api = PakarApi(
      baseUrl: 'usb://12/0',
      usb: fixture.transport,
      client: MockClient((_) async {
        http++;
        throw StateError('Network must not be used');
      }),
      resolver: LanResolver(
        lookup: (_) async {
          dns++;
          throw StateError('DNS must not be used');
        },
      ),
    );
    try {
      final status = await api.status();
      expect(status.peer.tableSelection, isTrue);
      expect(status.json['compatibility'], isA<Map>());
      expect(api.connectionMethod, ConnectionMethod.usb);
      // A real board includes compatibility metadata; status publication must
      // not start the optional network introduction in an offline USB session.
      await Future<void>.delayed(Duration.zero);
      expect(http, 0);
      expect(dns, 0);
      final actual = await api.fastCommand('/api/cells', {
        'ids': '1,80',
        'state': 'red',
        'effect': 'immediate',
      }, authorize: () => true);
      expect(actual!.cells[0], 'red');
      expect(actual.cells[79], 'red');
      expect(actual.cells[1], 'white');
      expect(fixture.writes.where((f) => f.opcode == 0x5e), hasLength(1));
      expect((await api.mapping()).valid, isTrue);
      await expectLater(api.getJson('/api/wifi/scan'), throwsFormatException);
      await expectLater(
        api.maintenanceBytes('/api/isp/probe', authorize: () => true),
        throwsFormatException,
      );
      expect(http, 0);
      expect(dns, 0);
    } finally {
      await api.close();
      await fixture.stream.close();
    }
  });
  test('Pixel diagnostic requests a renewable RGB lease and verifies all480 published pixels', () async {
    final fixture = UsbFixture();
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(fixture.channel, fixture.invoke);
    try {
      await fixture.transport.connect('usb://12/0');
      final actual = await fixture.transport.command('/api/led/diagnostic', {
        'index': '474',
        'count': '6',
        'rgb': '65280',
        'off': 'false',
      }, authorize: () => true);
      expect(
        matchesDiagnosticFrame(
          actual,
          const PixelProbe(index: 474, group: true, rgb: 65280).frame(),
        ),
        isTrue,
      );
      expect(fixture.demand & 8, 8);
      fixture.transport.demand(1, foreground: false);
      await Future<void>.delayed(const Duration(milliseconds: 20));
      expect(fixture.demand, 0);
      expect(fixture.lease, 0);
      expect(fixture.writes.where((f) => f.opcode == 0x5e), hasLength(1));
    } finally {
      await fixture.close();
    }
  });
  test(
    'Partial write, wrong ACK and malformed capture never replay an intent',
    () async {
      final fixture = UsbFixture();
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(fixture.channel, fixture.invoke);
      try {
        await fixture.transport.connect('usb://12/0');
        fixture.wrongAck = true;
        await expectLater(
          fixture.transport.command('/api/cells', {
            'ids': '1',
            'state': 'red',
          }, authorize: () => true),
          throwsFormatException,
        );
        expect(fixture.writes.where((f) => f.opcode == 0x5e), hasLength(1));
        fixture.corruptChunk = true;
        await expectLater(fixture.transport.status(), throwsFormatException);
        fixture.corruptChunk = false;
        fixture.partial = true;
        await expectLater(fixture.transport.status(), throwsFormatException);
      } finally {
        await fixture.close();
      }
    },
  );
  test('Nonzero demand requires a live lease, permission denial never sends HELLO, detach clears authority', () async {
    final fixture = UsbFixture();
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(fixture.channel, fixture.invoke);
    final errors = fixture.transport.events.stream.listen(
      (_) {},
      onError: (Object _) {},
    );
    try {
      fixture.denied = true;
      await expectLater(
        fixture.transport.connect('usb://12/0'),
        throwsA(isA<PlatformException>()),
      );
      expect(fixture.writes, isEmpty);
      fixture.denied = false;
      await fixture.transport.connect('usb://12/0');
      fixture.expiredLease = true;
      fixture.transport.demand(1, foreground: true);
      await Future<void>.delayed(const Duration(milliseconds: 20));
      expect(fixture.transport.ready, isFalse);
      expect(fixture.transport.latest, isNull);
      fixture.expiredLease = false;
      await fixture.transport.connect('usb://12/0');
      fixture.stream.add({'session': fixture.session, 'type': 'disconnected'});
      expect(fixture.transport.ready, isFalse);
      expect(fixture.transport.table, isNull);
    } finally {
      await errors.cancel();
      await fixture.close();
    }
  });
  test('Semantic panel event uses its own sequence and preserves requested-only presentation', () async {
    final fixture = UsbFixture();
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(fixture.channel, fixture.invoke);
    try {
      await fixture.transport.connect('usb://12/0');
      final next = fixture.transport.events.stream.firstWhere(
        (e) => e['type'] == 'panel-event',
      );
      final text = ascii.encode('Cell 80 red');
      fixture.stream.add({
        'session': fixture.session,
        'bytes': UsbFrame(0xb8, 37, [
          ...UsbFixture.le(37, 4),
          ...UsbFixture.le(1000, 4),
          1,
          0,
          2,
          ...UsbFixture.le(2000, 2),
          80,
          0,
          text.length,
          ...text,
        ]).encode(),
      });
      final event = await next.timeout(const Duration(seconds: 1));
      expect(event['event_id'], 37);
      expect(event['board_id'], 'esp32s3-fixture');
      expect(event['text'], 'Cell 80 red');
      expect(event['hardware_confirmed'], isFalse);
      expect(event['presentation']['marquee'], 'once');
    } finally {
      await fixture.close();
    }
  });
}
