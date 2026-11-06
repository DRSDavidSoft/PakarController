import 'dart:convert';

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:pakarcontroller/models.dart';
import 'package:pakarcontroller/pixel_diagnostic.dart';
import 'package:pakarcontroller/usb_transport.dart';

import 'usb_transport_test.dart' show UsbFixture;

class DiagnosticUsbApiFixture extends UsbFixture {
  DiagnosticUsbApiFixture() {
    status['command_clock_ms'] = 1000;
    status['effect'] = 'rainbow';
    (status['capabilities']['acknowledged_features'] as List).addAll(<String>[
      'usb-api',
      'pixel-diagnostic-effects',
      'pixel-diagnostic-overlay',
    ]);
  }
  final requestBytes = <int>[];
  List<int> responseBytes = [];
  Map? received;
  int executes = 0, apiToken = 0x12345678;
  bool wrongToken = false,
      accepted = true,
      partialStatus = false,
      pendingReceipt = false,
      foreignPartial = false;
  void Function()? onAppend;
  Map<String, dynamic> Function(Map)? apiResponse;
  @override
  Future<dynamic> invoke(MethodCall call) async {
    if (call.method != 'write') return super.invoke(call);
    final bytes = call.arguments['bytes'] as Uint8List;
    final frame = UsbFrame.decode(bytes.sublist(0, bytes.length - 1))!;
    if (frame.opcode != 0x6a) return super.invoke(call);
    writes.add(frame);
    final p = frame.payload, action = p.first;
    var offset = 0;
    if (action == 0) {
      expect(p.length, 19);
      expect(p.sublist(1, 17), List.filled(16, 0xaa));
      requestBytes.clear();
      responseBytes = [];
    } else {
      expect(UsbFixture.read(p, 1, 4), apiToken);
      if (action == 1) {
        expect(UsbFixture.read(p, 5, 2), requestBytes.length);
        requestBytes.addAll(p.skip(7));
        onAppend?.call();
      } else if (action == 2) {
        executes++;
        final envelope = jsonDecode(utf8.decode(requestBytes)) as Map;
        expect(envelope['board_id'], status['board_id']);
        expect(envelope['boot_id'], status['boot_id']);
        if (apiResponse != null) {
          responseBytes = utf8.encode(jsonEncode(apiResponse!(envelope)));
        } else {
          expect(envelope['path'], '/api/command');
          expect(envelope['method'], 'POST');
          expect(envelope['board_id'], status['board_id']);
          received = envelope['body'] as Map;
          final q = received!;
          expect(q['command'], 'pixel-diagnostic');
          expect(q.containsKey('packet'), false);
          status['led_output']['diagnostic'] = {
            'revision': executes,
            'active': q['enabled'],
            'enabled': q['enabled'],
            'index': q['index'],
            'count': q['count'],
            'rgb': q['rgb'],
            'off': q['off'],
            'effect': q['effect'],
            'composite': q['composite'],
            'period_ms': q['period_ms'],
            'duty_percent': q['duty_percent'],
            'min_brightness': q['min_brightness'],
            'max_brightness': q['max_brightness'],
            if (q.containsKey('group_layout_revision'))
              'group_layout_revision': q['group_layout_revision'],
          };
          pixels = {'revision': executes, 'pending': false};
          responseBytes = utf8.encode(
            jsonEncode({
              'type': 'command-result',
              'request_id': q['request_id'],
              'board_id': status['board_id'],
              'boot_id': status['boot_id'],
              'accepted': accepted,
              'error': accepted ? null : 'diagnostic_busy',
              if (partialStatus) 'status_partial': true,
              if (accepted)
                'status': partialStatus
                    ? {
                        for (final key in [
                          'board_id',
                          'boot_id',
                          'version',
                          'command_clock_ms',
                          'capabilities',
                          'pixels',
                          'led_output',
                        ])
                          key: status[key],
                        'pixel_frame': pendingReceipt ? null : pixels,
                        if (foreignPartial) 'boot_id': 'b' * 32,
                      }
                    : {...status, 'pixel_frame': pixels},
            }),
          );
        }
      } else if (action == 3) {
        offset = UsbFixture.read(p, 5, 2);
      } else if (action == 4) {
        reply(frame, 0x80, [0x6a, 0]);
        return bytes.length;
      }
    }
    reply(frame, 0xbd, [
      action,
      ...UsbFixture.le(wrongToken && action == 3 ? 9 : apiToken, 4),
      ...UsbFixture.le(responseBytes.length, 2),
      ...UsbFixture.le(offset, 2),
      if (action == 3) ...responseBytes.skip(offset).take(39),
    ]);
    return bytes.length;
  }
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  for (final flags in [(false, false), (true, false), (true, true)]) {
    final (partialStatus, pendingReceipt) = flags;
    test(
      'USB advanced diagnostic partial=$partialStatus pending=$pendingReceipt uses common API once and preserves scene without static frame fiction',
      () async {
        final fixture = DiagnosticUsbApiFixture()
          ..partialStatus = partialStatus
          ..pendingReceipt = pendingReceipt;
        TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
            .setMockMethodCallHandler(fixture.channel, fixture.invoke);
        try {
          await fixture.transport.connect('usb://12/0');
          const probe = PixelProbe(
            index: 479,
            group: true,
            effect: 'breathe',
            overlay: true,
            periodMs: 700,
            minBrightness: 15,
          );
          final actual = await fixture.transport.command(
            '/api/led/diagnostic',
            probe.commandFields(fixture.transport.latest!),
            authorize: () => true,
          );
          expect(matchesDiagnosticObservation(actual, probe), true);
          expect(actual.effect, 'rainbow');
          expect(actual.json['cells'], List.filled(80, 'white'));
          expect(actual.json['pixel_frame']['rgb'], isNull);
          expect(fixture.executes, 1);
          expect(fixture.received!['period_ms'], 700);
          expect(fixture.writes.where((f) => f.opcode == 0x5e), isEmpty);
          final stopped = probe.copyWith(enabled: false);
          final restored = await fixture.transport.command(
            '/api/led/diagnostic',
            stopped.commandFields(fixture.transport.latest!),
            authorize: () => true,
          );
          expect(
            matchesDiagnosticObservation(restored, stopped, afterRevision: 1),
            true,
          );
          expect(restored.effect, 'rainbow');
          expect(fixture.executes, 2);
        } finally {
          await fixture.close();
        }
      },
    );
  }
  for (final scenario in [
    'foreign-chunk',
    'rejected',
    'owner-retired',
    'unsupported',
    'foreign-partial',
  ]) {
    test(
      'USB diagnostic $scenario fails explicitly without replay or transport fallback',
      () async {
        final fixture = DiagnosticUsbApiFixture();
        var authorized = true;
        if (scenario == 'foreign-chunk') fixture.wrongToken = true;
        if (scenario == 'rejected') {
          fixture.accepted = false;
          fixture.partialStatus = true;
        }
        if (scenario == 'foreign-partial') {
          fixture.partialStatus = true;
          fixture.foreignPartial = true;
        }
        if (scenario == 'owner-retired') {
          fixture.onAppend = () => authorized = false;
        }
        if (scenario == 'unsupported') {
          (fixture.status['capabilities']['acknowledged_features'] as List)
              .remove('usb-api');
        }
        TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
            .setMockMethodCallHandler(fixture.channel, fixture.invoke);
        try {
          await fixture.transport.connect('usb://12/0');
          const probe = PixelProbe(effect: 'rainbow', overlay: true);
          await expectLater(
            fixture.transport.command(
              '/api/led/diagnostic',
              probe.commandFields(DeviceStatus(fixture.status)),
              authorize: () => authorized,
            ),
            throwsA(isA<Exception>()),
          );
          expect(
            fixture.executes,
            ['owner-retired', 'unsupported'].contains(scenario) ? 0 : 1,
          );
          expect(fixture.writes.where((f) => f.opcode == 0x5e), isEmpty);
        } finally {
          await fixture.close();
        }
      },
    );
  }
}
