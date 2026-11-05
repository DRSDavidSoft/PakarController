import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
const read = path => readFileSync(new URL('../'+path, import.meta.url),'utf8');
const protocol = read('libraries/BoardCore/src/Protocol.h');
assert(protocol.includes('#include "ProtocolOpcodes.h"'));
assert(!/enum\s+(Opcode|Error)\s*:/.test(protocol), 'Do not recreate an independent firmware registry');
const native = ['main','cell_batch','panel_event','peer_identity','usb_api','usb_demand','usb_table'];
for (const module of native) {
  const source = read('desktop/src/'+module+'.rs');
  assert(!/(?:pub )?const\s+(?:OP_\w+|OPCODE|USB_OPCODE|USB_REPORT|READ|RESPONSE|COMMAND|COMMAND_STATE|REQUEST):\s*u8\s*=\s*(?:0x|\d)/.test(source),
    module+': numeric opcode definitions must be generated references');
}
const dart = read('mobile/lib/usb_transport.dart');
assert(dart.includes("import 'protocol_opcodes.dart'"));
assert(!/_(?:ack|exchange)\(\s*(?:0x|\d)/.test(dart), 'USB commands cannot use inline numbers');
assert(!/frame\.opcode\s*==\s*(?:0x|\d)/.test(dart), 'USB responses cannot use inline numbers');
assert(dart.includes('ControllerOpcode.requests.contains(opcode)'), 'Explicit tunnel must reject non-request values');
assert(dart.includes('raw[1] != PakarOpcode.reserved'));
assert(read('desktop/src/main.rs').includes('raw[1] != wire::RESERVED'));
console.log('Firmware, Rust and Flutter opcode bindings have one generated owner; explicit namespace admission retained.');
