[CmdletBinding()]
param()
$ErrorActionPreference='Stop'
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern updates-telemetry | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern remote-ota -Gate | Out-Null
$root=Split-Path -Parent $PSScriptRoot
$output=Join-Path $root 'build/native/remote_s3_test.exe'
$json=Join-Path ([Environment]::GetFolderPath('MyDocuments')) 'Arduino/libraries/ArduinoJson/src'
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $output) | Out-Null
# Same canonical compile/crypto contract as test-protocol.ps1; no extra cache.
& g++ -std=c++17 -Wall -Wextra -Werror -DARDUINO_ARCH_ESP32 -DESP32 '-DPAKAR_PEER_IMAGE_FAMILY="fixture-family"' `
  -I (Join-Path $root 'tests/s3_ota_stubs') -I (Join-Path $root 'tests/remote_stubs') `
  -I $json -I (Join-Path $root 'libraries/BoardCore/src') `
  (Join-Path $root 'tests/remote_s3_test.cpp') (Join-Path $root 'firmware/esp/src/RemoteFirmware.cpp') -lbcrypt -o $output
if($LASTEXITCODE){throw 'S3 remote updater fixture compilation failed'}
& $output
if($LASTEXITCODE){throw 'S3 remote updater fixture assertions failed'}
$peerOutput=Join-Path $root 'build/native/peer_catalogue_test.exe'
& g++ -std=c++17 -Wall -Wextra -Werror -DARDUINO_ARCH_ESP32 -DESP32 '-DPAKAR_PEER_IMAGE_FAMILY="fixture-family"' `
  -I (Join-Path $root 'tests/s3_ota_stubs') -I (Join-Path $root 'tests/remote_stubs') `
  -I $json -I (Join-Path $root 'libraries/BoardCore/src') `
  (Join-Path $root 'tests/peer_catalogue_test.cpp') -lbcrypt -o $peerOutput
if($LASTEXITCODE){throw 'Actual peer catalogue fixture compilation failed'}
& $peerOutput
if($LASTEXITCODE){throw 'Actual peer catalogue fixture assertions failed'}
