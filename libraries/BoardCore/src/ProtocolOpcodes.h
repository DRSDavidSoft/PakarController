// Generated from PCController Project/ProtocolContract.json; DO NOT EDIT.
#pragma once
#include <stdint.h>

namespace BoardCore { namespace Protocol {
constexpr uint8_t kMagic = 0xA5;
constexpr uint8_t kReserved = 0;
constexpr uint8_t kMaximumPayload = 48;
constexpr uint8_t kRawOverhead = 6;
constexpr uint8_t kMaximumRaw = kMaximumPayload + kRawOverhead;
constexpr uint8_t kMaximumEncoded = kMaximumRaw + 2;

enum Opcode : uint8_t {
  kHello = 0x01,
  kGetStatus = 0x02,
  kSetStreamPeriod = 0x03,
  kBuzzer = 0x10,
  kAudioCue = 0x11,
  kAudioStream = 0x12,
  kDisplayText = 0x38,
  kSetMode = 0x4B,
  kCellAction = 0x4C,
  kGameReset = 0x4D,
  kOutputMask = 0x4E,
  kPing = 0x4F,
  kProgrammingState = 0x50,
  kSetBrightness = 0x51,
  kSetLedPattern = 0x52,
  kNetworkConfig = 0x53,
  kAccessConfig = 0x54,
  kDeviceName = 0x55,
  kPcContract = 0x56,
  kPcContractChunk = 0x57,
  kGetTelemetry = 0x58,
  kGetIdentity = 0x59,
  kEspPanel = 0x5A,
  kAvrRxDiagnostic = 0x5B,
  kSetLedOutput = 0x5C,
  kGetTableSnapshot = 0x5D,
  kTableCommand = 0x5E,
  kCellsAction = 0x5F,
  kShellRequest = 0x60,
  kPeerReport = 0x61,
  kCellAssignments = 0x62,
  kCellLock = 0x63,
  kHostDemand = 0x64,
  kPixelDiagnostic = 0x65,
  kKeyMask = 0x66,
  kKeySettings = 0x67,
  kGamePress = 0x68,
  kKeyPresentation = 0x69,
  kUsbApi = 0x6A,
  kAck = 0x80,
  kHelloResponse = 0x81,
  kErrorResponse = 0x82,
  kStatusResponse = 0x90,
  kGameState = 0xA3,
  kInputEvent = 0xA4,
  kCellEvent = 0xA5,
  kPong = 0xA6,
  kPcContractResponse = 0xB0,
  kTelemetryResponse = 0xB1,
  kIdentityResponse = 0xB2,
  kEspPanelResponse = 0xB3,
  kAvrRxDiagnosticResponse = 0xB4,
  kResetTelemetryResponse = 0xB5,
  kTableSnapshotResponse = 0xB6,
  kTableCommandState = 0xB7,
  kPanelEvent = 0xB8,
  kShellResponse = 0xB9,
  kHostDemandResponse = 0xBA,
  kAudioStreamResponse = 0xBC,
  kUsbApiResponse = 0xBD,
};

enum Error : uint8_t {
  kNoError = 0x00,
  kBadEnvelope = 0x01,
  kUnsupported = 0x02,
  kBadPayload = 0x03,
  kHardwareUnavailable = 0x04,
  kBusy = 0x05,
  kUnsafe = 0x06,
  kRedCellsLocked = 0x07,
  kInvalidCellAssignments = 0x08,
  kUnsupportedBoardRole = 0x09,
};

} } // namespace BoardCore::Protocol
