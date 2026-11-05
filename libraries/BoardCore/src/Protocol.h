#pragma once

#include <stdint.h>
#include <string.h>

#include "WireCodec.h"
#include "ProtocolOpcodes.h"

namespace BoardCore {
namespace Protocol {

// Envelope, opcodes and errors come from the canonical generated registry.

enum DeviceRole : uint8_t { kRoleAvr = 1, kRoleEsp = 2, kRoleHost = 3 };

namespace UsbDemand {
// Explicit command/read replies never depend on these optional stream flags.
// Pixels only enriches a requested immutable table capture, never starts a push.
enum Flag : uint16_t { Status = 1, Panel = 2, Peripheral = 4, Pixels = 8 };
constexpr uint16_t kAllowedFlags = Status | Panel | Peripheral | Pixels;
constexpr uint16_t kMinimumLeaseMs = 250;
constexpr uint16_t kMaximumLeaseMs = 5000;
} // namespace UsbDemand

enum GameMode : uint8_t {
  kModeIdle = 0,
  kModeCellSelect = 1,
  kModeFirstWins = 2,
  kModeSnake = 3,
};
// Existing stored codes remain stable; appearance is independent of identity.
enum CellState : uint8_t { kCellWhite = 0, kCellRed = 1, kCellOff = 2,
  kCellWon = 3, kCellIdle = kCellWhite, kCellLost = kCellRed };
enum Effect : uint8_t { kEffectImmediate = 0, kEffectFade = 1, kEffectPulse = 2 };

enum AudioCue : uint8_t {
  kCueWelcome = 0,
  kCueUpdateStart = 1,
  kCueUpdateSuccess = 2,
  kCueError = 3,
  kCueWinner = 4,
  kCueFinish = 5,
  kCueLost = 6,
  kCueIncorrect = 7,
  kCueRapidError = 8,
  kCueFault = 9,
  kCueIncrease = 10,
  kCueDecrease = 11,
  kCueWelcomePuzzleBoard = 12,
  kCueWelcome40Led = 13,
  kCueAttention = 14,
  kCueCount = 15,
};

enum Capability : uint16_t {
  kCapShiftInputs = 1U << 0,
  kCapShiftOutputs = 1U << 1,
  kCapAdc9 = 1U << 2,
  kCapTm1637 = 1U << 3,
  kCapBuzzer = 1U << 4,
  kCapLedGrid = 1U << 5,
  kCapIsp = 1U << 6,
  kCapOta = 1U << 7,
  kCapLittleFs = 1U << 8,
  kCapApi = 1U << 9,
  kCapWebSocket = 1U << 10,
  kCapNtp = 1U << 11,
  kCapBusyWire = 1U << 12,
  kCapHostSerial = 1U << 13,
  kCapDisplayText = 1U << 14, // Individual HELLO acknowledgement of the text adapter.
  kCapEspPanel = 1U << 15, // Explicit paired ESP settings/front-panel adapter.
};

namespace EspPanel {
enum Action : uint8_t { Offer = 0, Query = 16, SetMode = 17,
  Reconnect = 18, Scan = 19, SetDhcp = 20 };
enum Notice : uint8_t { Current = 0, Draft = 1, Pending = 2,
  Saved = 3, Failed = 4, TimedOut = 5 };
struct Snapshot {
  uint8_t mode = 1; // 1 AP, 2 station, 3 both; zero is never a safe network mode.
  bool dhcp = true, stationConfigured = false, stationConnected = false;
  char hostname[33] = {};
};
inline uint8_t encodeSnapshot(const Snapshot &state, uint8_t *out) {
  uint8_t size = 0;
  while (size < 32 && state.hostname[size]) ++size;
  out[0] = state.mode; out[1] = state.dhcp;
  out[2] = state.stationConfigured; out[3] = state.stationConnected;
  out[4] = size;
  memcpy(out + 5, state.hostname, size);
  return 5 + size;
}
inline bool decodeSnapshot(const uint8_t *p, uint8_t size, Snapshot &state) {
  if (!p || size < 6 || p[0] < 1 || p[0] > 3 || p[1] > 1 ||
      p[2] > 1 || p[3] > 1 || !p[4] || p[4] > 32 || size != 5 + p[4])
    return false;
  for (uint8_t i = 0; i < p[4]; ++i) {
    const uint8_t ch = p[5 + i];
    if (!((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') ||
          (ch >= '0' && ch <= '9') || (ch == '-' && i && i + 1 < p[4])))
      return false;
  }
  state.mode = p[0]; state.dhcp = p[1];
  state.stationConfigured = p[2]; state.stationConnected = p[3];
  memcpy(state.hostname, p + 5, p[4]); state.hostname[p[4]] = 0;
  return true;
}
} // namespace EspPanel

struct Frame {
  uint8_t opcode;
  uint8_t sequence;
  uint8_t payloadLength;
  uint8_t payload[kMaximumPayload];
};

inline uint8_t encode(const Frame &frame, uint8_t *output, uint8_t capacity) {
  if (frame.payloadLength > kMaximumPayload || capacity < 2) return 0;
  uint8_t raw[kMaximumRaw];
  raw[0] = kMagic;
  raw[1] = kReserved;
  raw[2] = frame.opcode;
  raw[3] = frame.sequence;
  raw[4] = frame.payloadLength;
  if (frame.payloadLength != 0) {
    memcpy(raw + 5, frame.payload, frame.payloadLength);
  }
  const uint8_t rawLength = frame.payloadLength + kRawOverhead;
  raw[rawLength - 1] = WireCodec::crc8(raw, rawLength - 1);
  const uint8_t encoded = WireCodec::cobsEncode(
      raw, rawLength, output, static_cast<uint8_t>(capacity - 1));
  if (encoded == 0 || encoded >= capacity) return 0;
  output[encoded] = 0;
  return encoded + 1;
}

inline bool decode(const uint8_t *input, uint8_t length, Frame &frame) {
  uint8_t raw[kMaximumRaw];
  const uint8_t rawLength = WireCodec::cobsDecode(input, length, raw, sizeof(raw));
  if (rawLength < kRawOverhead || raw[0] != kMagic ||
      raw[1] != kReserved ||
      raw[4] > kMaximumPayload || rawLength != raw[4] + kRawOverhead ||
      raw[rawLength - 1] != WireCodec::crc8(raw, rawLength - 1)) {
    return false;
  }
  frame.opcode = raw[2];
  frame.sequence = raw[3];
  frame.payloadLength = raw[4];
  if (frame.payloadLength != 0) memcpy(frame.payload, raw + 5, frame.payloadLength);
  return true;
}

}  // namespace Protocol
}  // namespace BoardCore
