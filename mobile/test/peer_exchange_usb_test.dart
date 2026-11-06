import 'package:flutter_test/flutter_test.dart';

import 'pixel_diagnostic_usb_api_test.dart' show DiagnosticUsbApiFixture;

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  test('Peer staging/progress/cancel use the existing boot-fenced USB API exactly once', () async {
    final fixture = DiagnosticUsbApiFixture();
    (fixture.status['capabilities']['acknowledged_features'] as List).add(
      'peer-exchange',
    );
    final paths = <String>[];
    fixture.apiResponse = (request) {
      paths.add('${request['method']} ${request['path']}');
      if (request['method'] == 'POST' &&
          request['path'] == '/api/peers/update') {
        expect(request['body']['source']['board_id'], 'source-board');
        expect(request['body']['authorization'], 'Bearer source-only');
      }
      return {
        'job_id': 1,
        'boot_id': fixture.status['boot_id'],
        'phase': 'download_esp',
      };
    };
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(fixture.channel, fixture.invoke);
    try {
      await fixture.transport.connect('usb://12/0');
      final result = await fixture.transport.peerExchange(
        'POST',
        '/api/peers/update',
        {
          'source': {'board_id': 'source-board'},
          'authorization': 'Bearer source-only',
        },
        authorize: () => true,
      );
      expect(result['job_id'], 1);
      await fixture.transport.peerExchange(
        'GET',
        '/api/firmware/remote',
        {},
        authorize: () => true,
      );
      await fixture.transport.peerExchange('POST', '/api/firmware/cancel', {
        'job_id': 1,
        'boot_id': fixture.status['boot_id'],
      }, authorize: () => true);
      expect(paths, [
        'POST /api/peers/update',
        'GET /api/firmware/remote',
        'POST /api/firmware/cancel',
      ]);
      expect(fixture.executes, 3);
    } finally {
      await fixture.close();
    }
  });
  for (final failure in [
    'unsupported',
    'owner-retired',
    'foreign-chunk',
    'rejected',
  ]) {
    test('Peer USB $failure does not replay or fall back', () async {
      final fixture = DiagnosticUsbApiFixture();
      final features =
          fixture.status['capabilities']['acknowledged_features'] as List;
      if (failure != 'unsupported') features.add('peer-exchange');
      var current = true;
      if (failure == 'owner-retired') fixture.onAppend = () => current = false;
      fixture.wrongToken = failure == 'foreign-chunk';
      fixture.apiResponse = (_) => {
        'accepted': false,
        'error': 'maintenance_busy',
      };
      TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
          .setMockMethodCallHandler(fixture.channel, fixture.invoke);
      try {
        await fixture.transport.connect('usb://12/0');
        await expectLater(
          fixture.transport.peerExchange(
            'POST',
            '/api/peers/update',
            {},
            authorize: () => current,
          ),
          throwsFormatException,
        );
        expect(
          fixture.executes,
          failure == 'foreign-chunk' || failure == 'rejected' ? 1 : 0,
        );
      } finally {
        await fixture.close();
      }
    });
  }
}
