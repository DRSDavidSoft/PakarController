import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:pakarcontroller/models.dart';
import 'package:pakarcontroller/peer_context.dart';

import 'peer_fixture.dart';

void main() {
  test('Peer exchange requires an explicit acknowledged capability', () {
    expect(
      peerFixture(features: ['peer-exchange']).peer.has('peer-exchange'),
      true,
    );
    expect(
      peerFixture(features: ['network', 'usb-api']).peer.has('peer-exchange'),
      false,
    );
    final mixed = peerFixture(features: ['peer-exchange']).json;
    mixed['capabilities']['hostname'] = 'other';
    expect(DeviceStatus(mixed).peer.has('peer-exchange'), false);
  });
  test('Advertised game press and presentation survive capability parsing', () {
    final raw = peerFixture(
      kind: 'keys',
      features: [
        'inputs',
        'outputs',
        'key-settings',
        'game-press',
        'game-presentation',
      ],
    ).json;
    (raw['capabilities'] as Map)
      ..['role'] = 'first-wins'
      ..['keys'] = {'count': 9, 'groups_per_key': 2, 'pixels_per_group': 12};
    final peer = DeviceStatus(raw).peer;
    expect(peer.has('game-press'), true);
    expect(peer.has('game-presentation'), true);
    expect(peer.allows('/api/game/press', {'key': '1'}), true);
    expect(peer.allows('/api/game/press', {'key': '9'}), true);
    expect(peer.allows('/api/game/reset', {}), true);
    for (final key in ['0', '10', '-1', '1,2']) {
      expect(peer.allows('/api/game/press', {'key': key}), false);
    }
    (raw['capabilities'] as Map)['acknowledged_features'] = [
      'inputs',
      'outputs',
    ];
    expect(
      DeviceStatus(raw).peer.allows('/api/game/press', {'key': '1'}),
      false,
    );
    expect(DeviceStatus(raw).peer.has('game-presentation'), false);
    (raw['capabilities'] as Map)
      ..['role'] = 'table-selection'
      ..['acknowledged_features'] = ['game-press'];
    expect(
      DeviceStatus(raw).peer.allows('/api/game/press', {'key': '1'}),
      false,
    );
  });

  test('LED backend capability is explicit; omitted or invalid is unknown', () {
    for (final item in [
      {'backend': 'gpio-static', 'rmt': false, 'dma': false},
      {'backend': 'rmt-dma', 'rmt': true, 'dma': true},
      {'backend': 'fastled', 'rmt': false, 'dma': false},
    ]) {
      final json = peerFixture().json;
      (json['capabilities'] as Map)['led_output'] = item;
      final driver = DeviceStatus(json).peer.ledOutput!;
      expect(driver.backend, item['backend']);
      expect(driver.rmt, item['rmt']);
      expect(driver.dma, item['dma']);
    }
    for (final item in [
      null,
      {},
      {'backend': 'rmt-dma', 'rmt': false, 'dma': true},
      {'backend': 'gpio-static', 'rmt': false, 'dma': 'false'},
    ]) {
      final json = peerFixture().json;
      (json['capabilities'] as Map)['led_output'] = item;
      final peer = DeviceStatus(json).peer;
      expect(peer.known, true);
      expect(peer.ledOutput, isNull);
    }
    expect(peerFixture().peer.ledOutput, isNull);
    final noLeds = peerFixture(features: ['buzzer']).json;
    noLeds['capabilities']['led_output'] = {
      'backend': 'rmt-dma',
      'rmt': true,
      'dma': true,
    };
    expect(DeviceStatus(noLeds).peer.ledOutput, isNull);
  });
  test(
    'No peer ACK cannot be inferred from live pixels, cells or AVR flags',
    () {
      final status = DeviceStatus({
        'hostname': 'ledboard',
        'pixels': 480,
        'avr_online': true,
        'tm1637_ack': true,
        'cells': List.filled(80, 'white'),
      });
      expect(status.peer.known, false);
      expect(status.peer.table, false);
      expect(status.peer.frontPanel, false);
      expect(status.peer.modes, isEmpty);
      expect(status.peer.allows('/api/cell', {'id': '1'}), false);
    },
  );
  test('Independent ACKs and measured display gate follow donor ready AND advertised', () {
    final buzzer = peerFixture(features: ['buzzer'], kind: 'controller');
    expect(buzzer.peer.has('buzzer'), true);
    expect(buzzer.peer.has('text'), false);
    expect(buzzer.peer.has('display'), false);
    expect(buzzer.peer.table, false);
    expect(buzzer.peer.modes, isEmpty);
    expect(peerFixture(displayAck: false).peer.has('display'), false);
    expect(
      peerFixture(features: ['text'], displayAck: false).peer.has('text'),
      true,
    );
  });
  test('Table geometry and advertised mode identity must agree; First wins needs IO', () {
    expect(peerFixture(kind: 'controller').peer.modes, isEmpty);
    expect(peerFixture(features: ['table']).peer.modes.map((m) => m.id), [
      'cell',
    ]);
    final status = peerFixture();
    expect(status.peer.modes.map((m) => m.id), ['cell', 'first-wins']);
    expect(status.peer.allows('/api/mode', {'value': 'snake'}), false);
    expect(status.peer.allows('/api/mode', {'value': 'not-advertised'}), false);
    final invalid = peerFixture().json;
    (invalid['capabilities']['table'] as Map)['rows'] = 9;
    expect(DeviceStatus(invalid).peer.table, false);
    (invalid['capabilities']['table'] as Map)['rows'] = 8.0;
    expect(DeviceStatus(invalid).peer.table, false);
  });
  test('Mixed host and missing feature ACKs fail closed', () {
    final value = peerFixture().json;
    value['capabilities']['hostname'] = 'other';
    expect(DeviceStatus(value).peer.known, false);
    value['capabilities']['hostname'] = 'ledboard';
    value['capabilities']['acknowledged_features'] = null;
    expect(DeviceStatus(value).peer.known, false);
  });
  test(
    'Inline identity text/icon are bounded; no URL fetch or name translation',
    () {
      expect(
        peerFixture(title: 'میز فارسی').peer.application!.title,
        'میز فارسی',
      );
      for (final title in ['', 'a' * 161, 'Bad\nTitle']) {
        expect(peerFixture(title: title).peer.application, isNull);
      }
      final bytes = Uint8List(33);
      bytes.setRange(0, 8, [137, 80, 78, 71, 13, 10, 26, 10]);
      bytes.setRange(12, 16, ascii.encode('IHDR'));
      ByteData.sublistView(bytes).setUint32(16, 64);
      ByteData.sublistView(bytes).setUint32(20, 64);
      final json = {
        'id': 'app',
        'name': 'App',
        'title': 'App title',
        'icon_png_base64': base64Encode(bytes),
      };
      final icon = PeerApplication.parse(json)!.icon;
      expect(icon, isNotNull);
      expect(
        identical(PeerApplication.parse(Map.of(json))!.icon, icon),
        isTrue,
        reason: 'Repeated telemetry must retain the MemoryImage cache key',
      );
      ByteData.sublistView(bytes).setUint32(16, 4096);
      json['icon_png_base64'] = base64Encode(bytes);
      expect(PeerApplication.parse(json)!.icon, isNull);
      json['icon_png_base64'] = 'https://other/logo.png';
      expect(PeerApplication.parse(json)!.icon, isNull);
    },
  );
}
