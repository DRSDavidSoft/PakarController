#include <assert.h>
#include <stdint.h>
#include <string.h>
#include <initializer_list>

#include "Protocol.h"
#include "ApiPolicy.h"
#include "PeerFence.h"
#include "LedOutputControl.h"
#include "AvrPeerHealth.h"

using namespace BoardCore::Protocol;

int main() {
  // Every other namespace is rejected even with a valid CRC and colliding0x11.
  for (unsigned discriminator = 1; discriminator < 256; ++discriminator) {
    uint8_t raw[] = {kMagic, static_cast<uint8_t>(discriminator), 0x11, 7, 1, 4, 0};
    raw[6] = BoardCore::WireCodec::crc8(raw, 6);
    uint8_t encoded[kMaximumEncoded]; Frame foreign{};
    const auto size = BoardCore::WireCodec::cobsEncode(raw, sizeof(raw), encoded, sizeof(encoded));
    assert(size && !decode(encoded, size, foreign));
  }
  // Same malformed COBS vectors as PCController. Rejected before frame/CRC
  // dispatch; in-place decoding must retain the existing bounded ownership.
  uint8_t decodedScratch[3] = {};
  const uint8_t embeddedZero[] = {2, 0}, laterZero[] = {3, 0x11, 0};
  assert(BoardCore::WireCodec::cobsDecode(embeddedZero,2,decodedScratch,3)==0);
  assert(BoardCore::WireCodec::cobsDecode(laterZero,3,decodedScratch,3)==0);
  BoardCore::AvrAdmissionRecovery admission;
  assert(!admission.needsHello(3000,0));
  admission.sent();admission.sent();
  assert(!admission.needsHello(3000,0));
  admission.sent();
  assert(!admission.needsHello(1999,0));
  assert(admission.needsHello(2000,0));
  admission.confirmed();assert(!admission.needsHello(3000,0));
  admission.sent();admission.sent();admission.sent();
  assert(admission.needsHello(1000,UINT32_MAX-1000));
  Frame peer{};
  peer.opcode = kPong;
  assert(!BoardCore::validAvrObservation(peer, false));
  assert(BoardCore::validAvrObservation(peer, true));
  peer.opcode = kHelloResponse; peer.payloadLength = 9;
  peer.payload[1] = kRoleHost;
  assert(!BoardCore::validAvrObservation(peer, false));
  peer.payload[1] = kRoleAvr;
  assert(BoardCore::validAvrObservation(peer, false));
  peer.payloadLength = 8;
  assert(!BoardCore::validAvrObservation(peer, true));
  peer.opcode = kGameState; peer.payloadLength = 4;
  peer.payload[0] = kModeCellSelect; peer.payload[1] = 0;
  peer.payload[2] = 80; peer.payload[3] = kCellWhite;
  assert(BoardCore::validAvrObservation(peer, true));
  peer.payload[2] = 0;
  assert(!BoardCore::validAvrObservation(peer, true));
  peer.opcode = kResetTelemetryResponse; peer.payloadLength = 6; peer.payload[5] = 1;
  assert(BoardCore::validAvrObservation(peer, true));
  peer.payload[5] = 2;
  assert(!BoardCore::validAvrObservation(peer, true));
  peer.opcode = 0xFF;
  assert(!BoardCore::validAvrObservation(peer, true));
  BoardCore::LedOutputControl output;
  const uint8_t off[]={0},on[]={1},statik[]={2,0},effects[]={2,1},bad[]={2,2};
  assert(BoardCore::parseLedOutputControl(off,1,output)&&!output.modeOnly&&!output.value);
  assert(BoardCore::parseLedOutputControl(on,1,output)&&!output.modeOnly&&output.value);
  assert(BoardCore::parseLedOutputControl(statik,2,output)&&output.modeOnly&&!output.value);
  assert(BoardCore::parseLedOutputControl(effects,2,output)&&output.modeOnly&&output.value);
    const uint8_t gpioBackend[]={3,1},rmtBackend[]={3,2},badBackend[]={3,0};
    assert(BoardCore::parseLedOutputControl(gpioBackend,2,output)&&output.backend==1&&!output.modeOnly);
    assert(BoardCore::parseLedOutputControl(rmtBackend,2,output)&&output.backend==2&&!output.modeOnly);
    assert(!BoardCore::parseLedOutputControl(badBackend,2,output));
    assert(BoardCore::parseLedOutputControl(off,1,output)&&output.backend==0);
  for(const auto length:{size_t(0),size_t(1),size_t(2),size_t(3)})assert(!BoardCore::parseLedOutputControl(bad,length,output));
  assert(!BoardCore::parseLedOutputControl(nullptr,1,output));
  assert(!BoardCore::parseLedOutputControl(on,2,output));
  // Adapt donor matching-identity/request assertions to an actual boot fence.
  // Donor session.go HELLO build metadata itself is NOT a boot epoch.
  using BoardCore::PeerFence::accepts;
  const char *board = "esp8266-aabbccddeeff";
  const char *boot = "123456789abcdef0123456789abcdef0";
  assert(accepts(board, boot, nullptr, nullptr)); // Explicit legacy/unfenced.
  assert(accepts(board, boot, board, boot));
  assert(!accepts(board, boot, board, nullptr));
  assert(!accepts(board, boot, nullptr, boot));
  assert(!accepts(board, boot, "", boot));
  assert(!accepts(board, boot, board, "223456789abcdef0123456789abcdef0"));
  assert(!accepts(board, boot, "esp8266-aabbccddee00", boot));
  assert(!accepts(board, "", board, boot));
  assert(!accepts(board, boot, board, "00000000000000000000000000000000"));
  assert(!accepts(board, boot, board, "123456789ABCDEF0123456789abcdef0"));
  assert(!BoardCore::PeerFence::boardValid("bad\r\nheader"));
  assert(!BoardCore::PeerFence::boardValid(""));
  // Actual donor TestVersionedCoreAPIRoutesAreRejected cases, plus vN bounds.
  for (const char *path : {"/api/v1", "/api/v1/rpc", "/api/v1/snapshot",
       "/api/v1/peripherals", "/api/v2/status", "/api/v123/", "/api/v1?x=1"}) {
    assert(BoardCore::ApiPolicy::numberedRoute(path));
  }
  for (const char *path : {"/api/status", "/api/device/name", "/api/firmware/fetch",
       "/api/version", "/api/v1other", "/ws", "/"}) {
    assert(!BoardCore::ApiPolicy::numberedRoute(path));
  }
  static_assert(kAudioCue == 0x11, "audio cue opcode changed");
  static_assert(kProgrammingState == 0x50,
                "programming latch opcode changed");
  static_assert(kSetBrightness == 0x51 && kSetLedPattern == 0x52,
                "host LED control opcodes changed");
  static_assert(kNetworkConfig == 0x53,
                "USB network recovery opcode changed");
  static_assert(kHostDemand == 0x64 && kHostDemandResponse == 0xBA && kPixelDiagnostic == 0x65,
                "USB observation demand opcodes changed");
  static_assert(UsbDemand::kAllowedFlags == 15 && UsbDemand::Pixels == 8 &&
                UsbDemand::kMinimumLeaseMs == 250 && UsbDemand::kMaximumLeaseMs == 5000,
                "USB expiring optional observation contract changed");
  static_assert((kCapWebSocket | kCapNtp | kCapBusyWire | kCapHostSerial) ==
                    0x3C00,
                "ESP transport capability layout changed");
  static_assert(kCueUpdateStart != kCueUpdateSuccess,
                "update cues must be distinguishable");

  Frame original = {};
  original.opcode = kCellAction;
  original.sequence = 0;
  original.payloadLength = kMaximumPayload;
  for (uint8_t index = 0; index < original.payloadLength; ++index) {
    original.payload[index] = index % 7 == 0 ? 0 : index;
  }

  uint8_t encoded[kMaximumEncoded] = {};
  const uint8_t length = encode(original, encoded, sizeof(encoded));
  assert(length > 1);
  assert(encoded[length - 1] == 0);

  Frame decoded = {};
  assert(decode(encoded, length - 1, decoded));
  assert(decoded.opcode == original.opcode);
  assert(decoded.sequence == original.sequence);
  assert(decoded.payloadLength == original.payloadLength);
  assert(memcmp(decoded.payload, original.payload, original.payloadLength) == 0);
  uint8_t raw[kMaximumRaw] = {};
  const uint8_t rawLength = BoardCore::WireCodec::cobsDecode(encoded, length - 1, raw, sizeof(raw));
  assert(raw[1] == 0); // Fixed reserved byte, not a contract generation.
  raw[1] = 1;
  raw[rawLength - 1] = BoardCore::WireCodec::crc8(raw, rawLength - 1);
  const uint8_t numbered = BoardCore::WireCodec::cobsEncode(raw, rawLength, encoded, sizeof(encoded));
  assert(!decode(encoded, numbered, decoded)); // Valid CRC cannot enable another generation.

  uint8_t damaged[kMaximumEncoded] = {};
  memcpy(damaged, encoded, length);
  damaged[length - 2] ^= 0x40;
  assert(!decode(damaged, length - 1, decoded));

  Frame oversize = {};
  oversize.payloadLength = kMaximumPayload + 1;
  assert(encode(oversize, encoded, sizeof(encoded)) == 0);

  // A lease is an ordinary bounded correlated COBS/CRC request, not a stream
  // of raw terminal text or an API generation selector. Include embedded zeros.
  Frame demand{}; demand.opcode=kHostDemand; demand.sequence=217;
  demand.payloadLength=4; demand.payload[0]=UsbDemand::Status|UsbDemand::Pixels;
  demand.payload[2]=0xB8; demand.payload[3]=0x0B; // 3000ms LE.
  const uint8_t demandLength=encode(demand,encoded,sizeof(encoded));
  assert(demandLength>1 && decode(encoded,demandLength-1,decoded));
  assert(decoded.opcode==kHostDemand && decoded.sequence==217 && decoded.payloadLength==4);
  assert(memcmp(demand.payload,decoded.payload,4)==0);

  return 0;
}
