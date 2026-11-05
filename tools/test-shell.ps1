param([switch]$Plan,[switch]$WorkspaceOnly,[string]$CapturePrefix,[ValidatePattern('^[a-zA-Z0-9_-]+$')][string]$BuildName='native')
$ErrorActionPreference='Stop'
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern network-cloud | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern terminal-ui | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern device-shell -Gate | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern build-storage -Gate | Out-Null
$taskRoot=Split-Path -Parent $PSScriptRoot
foreach($taskName in @('portable_shell','telnet_shell','shell_tui','shell_workspace','shell_wire','shell_pixel','shell_remote_query','ssh_policy')){
  if($WorkspaceOnly -and $taskName -ne 'shell_workspace'){continue}
  $taskOutput=Join-Path $taskRoot "build/$BuildName/${taskName}_test.exe"
  $taskStandard=if($taskName -eq 'shell_tui'){'-std=c++23'}else{'-std=c++17'}
  $taskArgs=@($taskStandard,'-Wall','-Wextra','-Werror','-I',(Join-Path $taskRoot 'firmware/esp/src'),'-I',(Join-Path $taskRoot 'tests/access_stubs'),(Join-Path $taskRoot "tests/${taskName}_test.cpp"),'-o',$taskOutput)
  if($taskName -eq 'shell_workspace'){$taskArgs=@('-DARDUINO_ARCH_ESP32','-DPAKAR_FIRST_WINS_ENABLED=1')+$taskArgs}
  if($taskName -eq 'shell_workspace' -and $CapturePrefix){$taskArgs=@('-DPS_RENDER_FIXTURE=1')+$taskArgs}
  if($Plan){[pscustomobject]@{Executable='g++';Arguments=$taskArgs};continue}
  New-Item -ItemType Directory -Force -Path (Split-Path -Parent $taskOutput) | Out-Null
  & g++ @taskArgs
  if($LASTEXITCODE){throw "$taskName fixture compilation failed"}
  if($taskName -eq 'shell_workspace' -and $CapturePrefix){& $taskOutput $CapturePrefix}else{& $taskOutput}
  if($LASTEXITCODE){throw "$taskName fixture failed"}
}
if($WorkspaceOnly){return}
$taskAccess=Join-Path $taskRoot "build/$BuildName/access_config_test.exe"
$taskAccessArgs=@('-std=c++17','-Wall','-Wextra','-Werror','-I',(Join-Path $taskRoot 'tests/access_stubs'),'-I',(Join-Path $taskRoot 'libraries/BoardCore/src'),'-I',(Join-Path $taskRoot 'firmware/esp/src'),(Join-Path $taskRoot 'tests/access_config_test.cpp'),(Join-Path $taskRoot 'firmware/esp/src/AccessConfig.cpp'),'-o',$taskAccess)
$taskDiscovery=Join-Path $taskRoot "build/$BuildName/discovery_mdns_test.exe"
$taskDiscoveryArgs=@('-std=c++17','-Wall','-Wextra','-Werror','-I',(Join-Path $taskRoot 'libraries/BoardCore/src'),(Join-Path $taskRoot 'tests/discovery_mdns_test.cpp'),'-o',$taskDiscovery)
if($Plan){[pscustomobject]@{Executable='g++';Arguments=$taskAccessArgs};[pscustomobject]@{Executable='g++';Arguments=$taskDiscoveryArgs};return}
& g++ @taskAccessArgs
if($LASTEXITCODE){throw 'Shell authority persistence fixture compilation failed'}
& $taskAccess
if($LASTEXITCODE){throw 'Shell authority persistence fixture failed'}
& g++ @taskDiscoveryArgs
if($LASTEXITCODE){throw 'Shell mDNS fixture compilation failed'}
& $taskDiscovery
if($LASTEXITCODE){throw 'Shell mDNS fixture failed'}
& node (Join-Path $taskRoot 'tests/shell_service_test.mjs')
if($LASTEXITCODE){throw 'Actual shell service lease fixture failed'}
& node (Join-Path $taskRoot 'tests/shell_tone_dispatch_test.mjs')
if($LASTEXITCODE){throw 'Actual shell-to-AVR tone dispatch fixture failed'}
& node (Join-Path $taskRoot 'tests/portable_shell_source_test.mjs')
if($LASTEXITCODE){throw 'Portable-shell source inventory fixture failed'}
& node (Join-Path $taskRoot 'tests/telnet_client_test.mjs')
if($LASTEXITCODE){throw 'TCP terminal loopback fixture failed'}
& node (Join-Path $taskRoot 'tests/udp_device_query_test.mjs')
if($LASTEXITCODE){throw 'UDP diagnostic client loopback fixture failed'}
