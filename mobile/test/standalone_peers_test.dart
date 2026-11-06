import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:pakarcontroller/discovery.dart';
import 'package:pakarcontroller/peers_page.dart';

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
  for (final mismatch in [false, true]) {
    test(
      'Public board identity remains visible without command/status admission; mismatch=$mismatch',
      () async {
        const boot = '0123456789abcdef0123456789abcdef';
        final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
        server.listen((request) async {
          if (request.uri.path == '/api/peer/info') {
            request.response.headers.contentType = ContentType.json;
            request.response.headers.set(
              'x-pakar-board',
              mismatch ? 'foreign-board' : 'private-board',
            );
            request.response.headers.set('x-pakar-boot', boot);
            request.response.write(
              jsonEncode({
                'product': 'PakarController',
                'kind': 'board',
                'board_id': 'private-board',
                'boot_id': boot,
                'hostname': 'private-board',
                'version': '0.1.0',
                'exchange': {'automatic_check': false},
              }),
            );
          } else {
            request.response.statusCode = request.uri.path == '/api/status'
                ? 401
                : 404;
          }
          await request.response.close();
        });
        try {
          final peer = await verifyDiscoveredDevice(
            DiscoveredDevice(
              'hint',
              'http://127.0.0.1:${server.port}',
              'Manual',
            ),
          );
          if (mismatch) {
            expect(peer, isNull);
          } else {
            expect(peer?.boardId, 'private-board');
            expect(peer?.kind, 'board');
            expect(peer?.verified, true);
            expect(peer?.authRequired, true);
            expect(peer?.compatible, false);
            expect(peer?.observedAddress, '127.0.0.1');
          }
        } finally {
          await server.close(force: true);
        }
      },
    );
  }
  test(
    'Board transfer requires observed private IP, same role and newer artifact',
    () {
      DiscoveredDevice source(
        String ip, {
        String version = '0.2.9',
        String path = '/firmware/download',
      }) => DiscoveredDevice(
        'source',
        'http://source-board.local',
        'Manual',
        kind: 'board',
        verified: true,
        observedAddress: ip,
        metadata: {
          'hostname': 'source-board',
          'role': 'table',
          'board_id': 'source-board',
          'boot_id': '0123456789abcdef0123456789abcdef',
          'artifacts': [
            {
              'family': 'same-pin-build',
              'version': version,
              'bytes': 4096,
              'sha256': 'a' * 64,
              'path': path,
            },
          ],
        },
      );
      final result = boardPeerFirmwareSource(
        source('192.168.2.3'),
        'table',
        '0.2.8',
      );
      expect(result?['address'], '192.168.2.3');
      expect(result?['hostname'], 'source-board.local');
      for (final ip in ['', '8.8.8.8', '127.0.0.1', '::1']) {
        expect(boardPeerFirmwareSource(source(ip), 'table', '0.2.8'), isNull);
      }
      expect(
        boardPeerFirmwareSource(source('192.168.2.3'), 'first-wins', '0.2.8'),
        isNull,
      );
      expect(
        boardPeerFirmwareSource(
          source('192.168.2.3', version: '0.2.8+other'),
          'table',
          '0.2.8',
        ),
        isNull,
      );
      expect(
        boardPeerFirmwareSource(
          source('192.168.2.3', path: '/arbitrary'),
          'table',
          '0.2.8',
        ),
        isNull,
      );
    },
  );
}
