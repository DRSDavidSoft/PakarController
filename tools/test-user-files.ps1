$ErrorActionPreference='Stop'
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern orbit-firmware | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern updates-telemetry -Summary | Out-Null
$taskRoot=Split-Path -Parent $PSScriptRoot
$taskOutput=Join-Path $taskRoot 'build/native/user_file_store_test.exe'
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $taskOutput) | Out-Null
$taskJson=Join-Path ([Environment]::GetFolderPath('MyDocuments')) 'Arduino/libraries/ArduinoJson/src'
& g++ -std=c++17 -O2 -g0 -Wall -Wextra -Werror -I "$taskRoot/tests/file_stubs" -I "$taskRoot/tests/access_stubs" -I $taskJson -I "$taskRoot/firmware/esp/src" "$taskRoot/tests/user_file_store_test.cpp" -o $taskOutput
if($LASTEXITCODE){throw 'Managed file store compilation failed'}
& $taskOutput
if($LASTEXITCODE){throw 'Managed file store assertions failed'}
& node "$taskRoot/tests/user_files_test.mjs"
if($LASTEXITCODE){throw 'Web file integrity/archive assertions failed'}
