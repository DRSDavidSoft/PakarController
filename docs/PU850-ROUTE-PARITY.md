# PU850 / ASA0002E route adaptation

Donor revision: 431104072952f694a35b3a83c19f13fbb098ca5b. Route-registration slice only; not whole-project parity, translation parity, production deployment or physical certification.

Every route registration in the reviewed slice is classified below, including commented endpoints. Source hashes and current target hashes/anchors are in PU850-ROUTE-PARITY.json. No pending generic route is silently labelled equivalent; unavailable sensor/business and unsafe behaviors are explicitly excluded.

## ESP32-S3 migration review

Donor registration slice 560-1554 and complete listed LocalLib sources reread against shared route implementations and current S3 conditional changes; generated hashes are bindings, not execution proof.

Normal builds retain AVR hardware UART. Optional PAKAR_AVR_LINK_ENABLED=0 quiet commissioning omits normal AVR TX and permits an explicit USB-only passive UART2 RX18 capture at 38400/115200 baud, at most 8 seconds/128 retained bytes. The helper checks TX17 input ownership and never drives reset GPIO10. Capture is diagnostic evidence, not a compatible AVR protocol or repaired reset-loop claim.

- Real authenticated LAN GET/HEAD/Range/ETag/download-byte comparison and interrupted upload/power-loss/AVR recovery acceptance remain open for both platforms.
- S3 ISP erase/programming remains disabled until the actual AVR voltage, reset isolation and ISP wiring are electrically verified; UART communication alone does not certify ISP.
- S3 NBNS name/node-status queries are implemented with bounded per-netif UDP137 ownership; WINS registration/conflict arbitration, SMB sessions and LLMNR are not implemented. Host fixtures and SDK compilation are not live multicast or Windows name-resolution proof.
- Native/Web/mobile Wi-Fi and optional Ethernet DHCP/static editors use existing peer-bound ACK/readback; TUI addressing editing and full mobile RGB settings parity remain pending. Catalog fields alone do not certify all-interface parity.
- Legacy /lang continues to report translated=false; partial Web translations are not whole application/route translation parity.
- S3 hostname lookup currently uses the SDK default blocking deadline; explicit asynchronous/bounded request scheduling and compressed OTA acceptance need separate work.
- Quiet passive capture needs actual GPIO/received-byte/timing acceptance and reset-loop causal diagnosis; it does not replace normal UART/AVR protocol delivery. Atomic capture-state checks now cover WebSocket/cloud dispatch and new async ESP/AVR upload ownership alongside HTTP/USB guards and TLS service suspension. These checks do not certify a race-free cross-task maintenance lease; production multi-client/concurrency acceptance remains open.

| Donor source | Route | Disposition | Current implementation / reason |
| --- | --- | --- | --- |
| LocalLib/AsyncOTA.cpp:12 | /update GET | commented-replaced | firmware/esp/src/WebUi.h:422 — Commented donor HTML placeholder replaced by real firmware settings; no second OTA engine. |
| LocalLib/AsyncOTA.cpp:19 | /update POST | adapted | firmware/esp/application.ino:3896 — Integrated guarded authenticated upload, owner/disconnect fencing and platform OTA capacity/abort retained beside pinned remote download. Quiet S3 passive capture rejects new upload ownership. U_FS replacement is disabled to preserve credentials/recovery; S3 compressed-image support is not certified. |
| LocalLib/AsyncWebServer.h:22 | /ws WebSocket | adapted | firmware/esp/application.ino:2888 — Authenticated multiple-client WebSocket retains board state and adds read-only full telemetry and pinned remote-job commands. Incoming messages are explicitly rejected during quiet S3 passive capture, before binary streaming or command dispatch. |
| LocalLib/Authentication.cpp:22 | /auth GET | adapted | firmware/esp/src/MaintenanceRoutes.h:534 — Local owner auth status, no credential disclosure. |
| LocalLib/Authentication.cpp:57 | /pass POST | adapted | firmware/esp/src/MaintenanceRoutes.h:540 — Requires current password, persistent change and revocation of Web/Telnet sessions. |
| LocalLib/FirmwareTransfer.cpp:213 | /firmware/download GET|HEAD | imported | firmware/esp/src/FirmwareTransfer.cpp:173 — Actual donor range/HEAD/ETag/MD5/export code imported. S3 reads offsets relative to its running application partition with mbedTLS Base64; ESP8266 aligned flash reads remain. Full flash export is disabled to protect secrets. |
| LocalLib/SSDP.cpp:113 | /description.xml GET | adapted | firmware/esp/application.ino:4334 — Actual product/hostname root-device XML; S3 delegates escaped description generation to the adapted donor SSDP device, ESP8266 keeps its existing description. Neither advertises nonexistent SOAP/GENA services. S3 NBNS readiness is independently observed from exact interface-bound UDP137 sockets, not inferred from SSDP. |
| pu850-esp.ino:591 | /beep POST | adapted | firmware/esp/src/MaintenanceRoutes.h:567 — Actual AVR buzzer ACK, 1..8 bounded count. Mutating GET removed. |
| pu850-esp.ino:597 | /message/send POST | adapted | firmware/esp/src/MaintenanceRoutes.h:575 — Actual TM1637 single-pass ASCII text; existing display engine owns timing, no duplicate scrolling engine. |
| pu850-esp.ino:611 | /weight/display GET|POST | not-applicable | PU weighing UI/value format requires a load cell; none is fitted. Generic TM1637 text/number is implemented separately. |
| pu850-esp.ino:647 | /action/tare POST | not-applicable | PU load-cell tare/calibration command; this board has no weighing hardware. |
| pu850-esp.ino:652 | /power GET | not-applicable | PU battery/power measurements require sensors not present; telemetry does not invent voltage/current. |
| pu850-esp.ino:657 | /weight/get GET | not-applicable | PU weight measurement protocol and load cell are absent. |
| pu850-esp.ino:662 | /receipt GET|POST | not-applicable | PU weighing receipts/chart records and associated business protocol are deliberately removed. |
| pu850-esp.ino:703 | /report GET | commented-business | Commented donor business-report endpoint, not a working feature; no weighing records exist. |
| pu850-esp.ino:757 | /file/cancel POST | adapted | firmware/esp/src/MaintenanceRoutes.h:466 — Exact job/boot ownership; no blind cancellation of another client or running erase. |
| pu850-esp.ino:764 | /file/download GET|HEAD | adapted | firmware/esp/src/ArtifactDownload.h:16 — Allowlisted actual ESP/staged/AVR flash and EEPROM, imported Range parser, HEAD and storage read leases. Arbitrary PU filenames/full filesystem dumps removed. |
| pu850-esp.ino:1040 | /debug/info GET | adapted | firmware/esp/src/MaintenanceRoutes.h:585 — Actual remote/manual OTA phase, errors, UART counters and ACK observations; no fabricated PU enum. |
| pu850-esp.ino:1048 | /debug/data GET | adapted | firmware/esp/src/MaintenanceRoutes.h:585 — Actual bounded diagnostics, no raw UART dump containing secrets. |
| pu850-esp.ino:1057 | /debug/clear POST | adapted | firmware/esp/src/MaintenanceRoutes.h:595 — Clear observed reply only; persistent update journal and non-resettable counters explicitly retained. |
| pu850-esp.ino:1064 | /eProcessState GET | adapted | firmware/esp/src/MaintenanceRoutes.h:585 — Actual remote updater state replaces PU process enum. |
| pu850-esp.ino:1068 | /eResponseStatus GET | adapted | firmware/esp/src/MaintenanceRoutes.h:585 — Actual last ACK and reply observation replaces PU response enum. |
| pu850-esp.ino:1072 | /crash GET|POST | adapted | firmware/esp/src/MaintenanceRoutes.h:584 — 410: deliberate null write/division/watchdog corruption is unsafe and is not a production capability. |
| pu850-esp.ino:1082 | /datetime/get GET | adapted | firmware/esp/src/MaintenanceRoutes.h:490 — ESP software clock, UTC/source/validity; no AVR RTC is fitted. |
| pu850-esp.ino:1097 | /datetime/set POST | adapted | firmware/esp/src/MaintenanceRoutes.h:490 — Strict Gregorian UTC parse then software-clock update; no fake AVR time ACK. NTP may subsequently adjust. |
| pu850-esp.ino:1118 | /datetime/parse POST | adapted | firmware/esp/src/MaintenanceRoutes.h:490 — Strict read-only UTC validation, including leap-day checks; donor Debug gate no longer required. |
| pu850-esp.ino:1139 | /datetime/active GET | adapted | firmware/esp/src/MaintenanceRoutes.h:490 — Actual software clock snapshot; no stale PU adjusted timestamp. |
| pu850-esp.ino:1153 | /ntp/get GET | adapted | firmware/esp/src/MaintenanceRoutes.h:490 — Previously commented; reports real SNTP callback observation rather than claiming every valid epoch is NTP. |
| pu850-esp.ino:1179 | /wifi/signal GET | adapted | firmware/esp/src/MaintenanceRoutes.h:506 — Real RSSI when connected; null when offline, no fake percentage. |
| pu850-esp.ino:1198 | /wifi/country GET | adapted | firmware/esp/src/MaintenanceRoutes.h:511 — Actual SDK regulatory country/channel values. |
| pu850-esp.ino:1210 | /wifi/status GET | adapted | firmware/esp/src/MaintenanceRoutes.h:504 — Shared ESP8266/S3 STA/AP mode, connectivity, MAC/IP/netmask/gateway/DNS/channel/DHCP and AP client count. S3 DHCP is the persisted requested policy, not a lease-acquisition assertion. |
| pu850-esp.ino:1217 | /config/ap GET | adapted | firmware/esp/src/MaintenanceRoutes.h:504 — Actual AP configuration, passwords redacted. Writes remain /api/network. |
| pu850-esp.ino:1244 | /config/st GET | adapted | firmware/esp/src/MaintenanceRoutes.h:504 — Actual station configuration, passwords redacted. Writes remain /api/network. |
| pu850-esp.ino:1277 | /hostname/check GET|POST | adapted | firmware/esp/src/MaintenanceRoutes.h:521 — Validated DNS hostname and SDK lookup; refuses during remote firmware maintenance. ESP8266 has an explicit 3-second timeout; S3 uses the SDK default, not an asserted identical latency bound. |
| pu850-esp.ino:1307 | /login POST | commented-replaced | firmware/esp/src/WebAccess.h:217 — Commented donor stub replaced by existing authenticated expiring cookie/session login with same-origin validation. |
| pu850-esp.ino:1311 | /auth_dump GET | adapted | firmware/esp/src/MaintenanceRoutes.h:534 — Authenticated username/readiness only; raw password/token dumping deliberately removed. |
| pu850-esp.ino:1325 | /sessions_dump GET | adapted | firmware/esp/src/MaintenanceRoutes.h:534 — Authenticated bounded session count, never bearer/cookie values. |
| pu850-esp.ino:1343 | /random GET | adapted | firmware/esp/src/MaintenanceRoutes.h:550 — Fresh hardware-entropy random bytes, not a live authorization token. |
| pu850-esp.ino:1358 | /username GET | adapted | firmware/esp/src/MaintenanceRoutes.h:534 — Actual local device-owner identity instead of unsupported PU user-role RPC. |
| pu850-esp.ino:1380 | /lang GET | adapted | firmware/esp/src/MaintenanceRoutes.h:556 — Persisted locale preference; translated=false explicitly, no claim of translated application parity. |
| pu850-esp.ino:1384 | /lang POST | adapted | firmware/esp/src/MaintenanceRoutes.h:556 — Bounded validated locale persisted atomically; PU language RPC not sent. |
| pu850-esp.ino:1406 | /pu GET | not-applicable | PU chart/master-state protocol does not describe this AVR; actual AVR settings/status/telemetry replace applicable diagnostics. |
| pu850-esp.ino:1441 | /pu_reboot GET|POST | not-applicable | PU-specific reboot RPC replaced by the negotiated AVR RESET MCU contract; no unauthenticated GET reset. |
| pu850-esp.ino:1460 | /pu_reset GET|POST | not-applicable | Old PU reset pin/chart state are not copied: ESP8266 uses guarded D1/GPIO5 ISP reset, S3 uses reviewed build pin ownership and refuses ISP until its electrical-verification flag is enabled. |
| pu850-esp.ino:1470 | /is_exec GET | not-applicable | PU chart execution state has no equivalent on this table; actual game/automation state is exposed through negotiated contracts. |
| pu850-esp.ino:1475 | /page_id GET | not-applicable | PU chart page identifier is not this board's front-panel menu. PCController GET MENU remains the actual supported contract. |
| pu850-esp.ino:1480 | /page_goto POST | not-applicable | PU page/business navigation cannot be copied as a table action. PCController SET MENU is available through the guarded contract API. |
| pu850-esp.ino:1493 | /update/info GET | adapted | firmware/esp/application.ino:3884 — Version/profile and both-board remote maintenance URLs; no misleading build-host/time identity. |
| pu850-esp.ino:1513 | /info GET | adapted | firmware/esp/application.ino:3885 — Authenticated full structured ESP/AVR/client/network/clock/cloud telemetry; optional format=text. |
| pu850-esp.ino:1543 | / GET | adapted | firmware/esp/src/WebUi.h:422 — Real control workspace rather than donor static placeholder; access gate and actual peer state retained. Optional DHCP/static editor preserves stored station credentials and RGB preview requires fresh actual output, not a synthetic animation. |

Broader repository/file parity remains tracked separately in FILE-COMPARISON.md and GitHub issue #3. Live route/TLS/board acceptance remains open.
