import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:pakarcontroller/discovery.dart';

class _Transport extends AutomatedTestWidgetsFlutterBinding {
  @override
  bool get overrideHttpClient => false;
}

void main() {
  _Transport();
  test('Application identity is inspectable without board status or supported control release', () async {
    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    final paths = <String>[];
    server.listen((request) async {
      paths.add(request.uri.path);
      request.response.headers.contentType = ContentType.json;
      request.response.write(
        jsonEncode({
          'product': 'PakarController',
          'kind': 'desktop-app',
          'instance_id': 'native-peer',
          'hostname': 'desktop',
          'version': '99.0.0',
          'exchange': {'automatic_check': false},
        }),
      );
      await request.response.close();
    });
    try {
      final peer = await verifyDiscoveredDevice(
        DiscoveredDevice('native', 'http://127.0.0.1:${server.port}', 'Manual'),
      );
      expect(peer, isNotNull);
      expect(peer!.kind, 'desktop-app');
      expect(peer.boardId, isEmpty);
      expect(peer.metadata['exchange']['automatic_check'], false);
      expect(paths, ['/api/peer/info']);
    } finally {
      await server.close(force: true);
    }
  });
  test('Conflicting installation identity never merges a reused address', () {
    const a = DiscoveredDevice(
      'a',
      'http://peer.local:8789',
      '',
      kind: 'desktop-app',
      instanceId: 'one',
      verified: true,
    );
    const b = DiscoveredDevice(
      'b',
      'http://peer.local:8789',
      '',
      kind: 'desktop-app',
      instanceId: 'two',
      verified: true,
    );
    expect(coalesceDevices([a, b]), hasLength(2));
  });
}
