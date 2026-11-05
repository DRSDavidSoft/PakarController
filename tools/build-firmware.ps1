param(
    [ValidateSet('polling', 'busy-wire', 'buffered')]
    [string] $LinkMode = $(if ($env:PAKAR_LINK_MODE) { $env:PAKAR_LINK_MODE } else { 'busy-wire' }),
    [ValidatePattern('^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$')]
    [string] $DeviceName,
    [ValidateRange(1, 32)] [int] $Jobs = [Math]::Min(32, [Environment]::ProcessorCount),
    [ValidateSet('both', 'avr', 'esp8266', 'esp32s3')] [string] $Target = 'both',
    [ValidateSet('table-selection','first-wins')] [string] $GameMode = 'table-selection',
    [ValidateRange(1, 511)] [int] $InitialKeyMask = 511,
    [ValidateSet('press-to-win','illuminated','color-match','simon','memory-pairs','clear-all','color-order','switch-pattern','reaction-chase','mine-field','toggle')]
    [string[]] $KeyMiniGames = @('press-to-win','illuminated','color-match','simon','memory-pairs','clear-all','color-order','switch-pattern','reaction-chase','mine-field','toggle'),
    [ValidateSet('auto','enabled','disabled')] [string] $AvrAutomations = 'auto',
    [switch] $AvrRadio,
    [switch] $AvrLedDiagnostic,
    [ValidateSet(4, 8, 16, 32)] [int] $S3FlashMB = 4,
    [ValidateSet('disabled', 'enabled', 'opi')] [string] $S3Psram = 'disabled',
    [switch] $S3LedGpio,
    [switch] $S3LedRmtDma,
    [ValidateSet('auto','gpio-static','rmt-dma')] [string] $S3LedDefault = 'auto',
    [ValidateSet(4, 47, 48)] [int] $S3LedDataPin = 4,
    [ValidateSet('runtime', 'disabled')] [string] $S3Bluetooth = 'runtime',
    [ValidateSet('uart0','hwcdc','otg')] [string] $S3HostTransport = 'uart0',
    [ValidateSet(-1, 38, 48)] [int] $S3StatusRgbPin = -1,
    [ValidateRange(1,4)] [int] $ShellSessions = 2,
    [switch] $DisableSerialShell,
    [switch] $DisableTelnet,
    [switch] $S3Ssh,
    [switch] $S3Scp,
    [switch] $S3Sftp,
    [switch] $S3Ftp,
    [switch] $EnableUdpDiagnostics,
    [switch] $BoardSnake,
    [switch] $DisableNetworking,
    [ValidateSet('enabled','disabled')] [string] $PixelCorrections = 'enabled',
    [switch] $DisablePeerVersionChecks,
    [switch] $InitializeFilesystem,
    [string] $AvrRecoveryImage,
    [switch] $PauseAvrLink,
    [switch] $PauseLedOutput,
    [switch] $CellChaseOnBoot,
    [switch] $S3IspElectricalVerified,
    [switch] $S3Ethernet,
    [switch] $S3MicroSd,
    [ValidateSet(-1,1,2,39,40,41,42)] [int] $S3SdClk = -1,
    [ValidateSet(-1,1,2,39,40,41,42)] [int] $S3SdCmd = -1,
    [ValidateSet(-1,1,2,39,40,41,42)] [int] $S3SdD0 = -1,
    [switch] $VerboseBuild,
    [switch] $Clean,
    [switch] $Plan
)

$ErrorActionPreference = 'Stop'
& node (Join-Path $PSScriptRoot '../interfaces/controller-wire/consumer.mjs') --pakar --check | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw 'Canonical opcode bindings are stale.' }
if($PSBoundParameters.ContainsKey('KeyMiniGames') -and ($Target -ne 'esp32s3' -or $GameMode -ne 'first-wins')){throw 'Key mini-game inclusion flags require the ESP32-S3 keycontroller role.'}
if($GameMode -eq 'first-wins' -and -not $KeyMiniGames.Count){throw 'Include at least one key minigame.'}
if($BoardSnake -and ($Target -ne 'esp32s3' -or $GameMode -ne 'table-selection')) { throw 'Board Snake is opt-in for ESP32-S3 table builds only.' }
if ($S3MicroSd -and ($Target -ne 'esp32s3' -or $S3SdClk -eq -1 -or $S3SdCmd -eq -1 -or $S3SdD0 -eq -1 -or @(@($S3SdClk,$S3SdCmd,$S3SdD0) | Select-Object -Unique).Count -ne 3)) { throw 'microSD requires ESP32-S3 and three explicit, distinct reviewed SD_MMC pins.' }
if (-not $S3MicroSd -and ($S3SdClk -ne -1 -or $S3SdCmd -ne -1 -or $S3SdD0 -ne -1)) { throw 'SD pins require the explicit -S3MicroSd opt-in.' }
if ($InitializeFilesystem) { throw 'Commissioning auto-format builds are retired. Build normal firmware and use tools/initialize-device-filesystem.ps1 for explicit, erased-partition-only USB initialization.' }
if($S3Ssh -and ($Target -ne 'esp32s3' -or $DisableNetworking)){throw 'Native SSH requires the networked ESP32-S3 target.'}
if($S3Scp -and -not $S3Ssh){throw 'Native SCP requires the explicit -S3Ssh flag.'}
if($S3Sftp -and -not $S3Ssh){throw 'Native SFTP requires the explicit -S3Ssh flag.'}
if($S3Ftp -and ($Target -ne 'esp32s3' -or $DisableNetworking)){throw 'Native FTP requires the networked ESP32-S3 target.'}
$recoveryBytes=$null
if($AvrRecoveryImage){
    if($Target -ne 'esp32s3'){throw 'Bundled AVR recovery requires the ESP32-S3 target'}
    $recoveryBytes=[IO.File]::ReadAllBytes((Resolve-Path -LiteralPath $AvrRecoveryImage).Path)
    if($recoveryBytes.Length -ne 32768 -or $recoveryBytes[0] -ne 0x3f -or $recoveryBytes[1] -ne 0xcf -or
       $recoveryBytes[100] -ne 0x0c -or $recoveryBytes[101] -ne 0x94 -or
       $recoveryBytes[32762] -ne 3 -or $recoveryBytes[32763] -ne 25 -or
       $recoveryBytes[32766] -ne 0xe7 -or $recoveryBytes[32767] -ne 0x40){
        throw 'Recovery requires a complete reviewed 328P vector-patched application plus Urboot image'
    }
}
if ($AvrRadio -and $Target -notin @('avr','both')) { throw 'AVR radio requires an AVR build target.' }
if ($AvrLedDiagnostic -and ($Target -ne 'avr' -or $AvrRadio)) { throw 'D6 diagnostic requires an AVR-only build with RF disabled.' }
if(($PSBoundParameters.ContainsKey('ShellSessions') -or $DisableSerialShell -or $DisableTelnet -or $EnableUdpDiagnostics) -and $Target -ne 'esp32s3'){throw 'Shell build switches currently use the S3 application-only profile; other targets retain macro defaults.'}
if($EnableUdpDiagnostics -and $DisableNetworking){throw 'UDP diagnostics require networking.'}
. (Join-Path $PSScriptRoot 'script-runtime.ps1')
if ($Target -eq 'esp32s3' -and -not $PSBoundParameters.ContainsKey('LinkMode') -and -not $env:PAKAR_LINK_MODE) { $LinkMode = 'buffered' }
if ($Target -eq 'esp32s3' -and $LinkMode -ne 'buffered') { throw 'ESP32-S3 UART integration requires the explicit paired buffered profile.' }
if ($PauseAvrLink -and $Target -ne 'esp32s3') { throw 'Quiet AVR-link commissioning currently targets ESP32-S3 only.' }
if ($PauseLedOutput -and $Target -ne 'esp32s3') { throw 'Quiet LED commissioning currently targets ESP32-S3 only.' }
if ($CellChaseOnBoot -and ($Target -ne 'esp32s3' -or $PauseLedOutput)) { throw 'Cell-chase boot testing requires ESP32-S3 with LED output enabled.' }
if ($S3IspElectricalVerified -and $Target -ne 'esp32s3') { throw 'The S3 ISP electrical acknowledgment is target-specific.' }
if ($S3Ethernet -and $Target -ne 'esp32s3') { throw 'Native W5500 Ethernet requires ESP32-S3.' }
if ($DisableNetworking -and ($Target -ne 'esp32s3' -or $GameMode -ne 'first-wins')) { throw 'Offline fallback currently requires ESP32-S3 first-wins.' }
if ($InitialKeyMask -ne 511 -and ($Target -ne 'esp32s3' -or $GameMode -ne 'first-wins')) { throw 'A custom initial key mask currently requires ESP32-S3 first-wins.' }
if ($S3Bluetooth -ne 'runtime' -and $Target -ne 'esp32s3') { throw 'Bluetooth build selection requires ESP32-S3.' }
if ($PSBoundParameters.ContainsKey('S3HostTransport') -and $Target -ne 'esp32s3') { throw 'Host USB transport selection requires ESP32-S3.' }
if (($S3LedGpio -or $S3LedRmtDma -or $S3LedDefault -ne 'auto') -and $Target -ne 'esp32s3') { throw 'LED driver build flags require ESP32-S3.' }
if (($S3LedDefault -eq 'gpio-static' -and -not $S3LedGpio) -or ($S3LedDefault -eq 'rmt-dma' -and -not $S3LedRmtDma)) { throw 'Default LED driver must be explicitly included in this image.' }
$s3LedDefaultCode=if($S3LedDefault -eq 'rmt-dma'){2}elseif($S3LedDefault -eq 'gpio-static' -or $S3LedGpio){1}elseif($S3LedRmtDma){2}else{0}
if ($PSBoundParameters.ContainsKey('S3LedDataPin') -and $Target -ne 'esp32s3') { throw 'LED pin selection requires ESP32-S3.' }
if ($LinkMode -eq 'buffered' -and $Target -notin @('avr', 'esp32s3')) {
    throw 'Buffered mode currently targets the paired AVR only; ESP8266 requires busy-wire or polling.'
}
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern api-contract -Gate | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern pu-tooling -Gate | Out-Null
& node (Join-Path $PSScriptRoot 'check-living-contract.mjs') | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw 'Living API policy failed before firmware build.' }
$projectRoot = Split-Path -Parent $PSScriptRoot
if ($Target -in @('both', 'avr')) {
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern avr-boot-chain -Gate | Out-Null
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern cooperative-tasks -Gate | Out-Null
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern uart-flow-control -Gate | Out-Null
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern tm1637-display -Gate | Out-Null
}
if ($Target -eq 'esp32s3') {
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern esp32-ws2811 -Gate | Out-Null
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern uart-flow-control -Gate | Out-Null
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern lcd-display -Gate | Out-Null
    if ($S3Ethernet) {
        & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern ethernet -Gate | Out-Null
    }
}
if ($Target -ne 'avr') {
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern device-shell -Gate | Out-Null
    if($EnableUdpDiagnostics){& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern udp-diagnostic-query -Gate | Out-Null}
    # Route evidence is a separate reviewed artifact from the file ledger.
    # Fail before compiling/flashing if a changed sketch has stale route hashes.
    & node (Join-Path $PSScriptRoot '../tests/pu_routes_test.mjs') | ForEach-Object { Write-Host $_ }
    if ($LASTEXITCODE -ne 0) { throw 'PU route evidence is stale; re-review affected routes and regenerate their exact target bindings.' }
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern remote-ota -Gate | Out-Null
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern web-presentation -Gate | Out-Null
    & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern web-streaming -Gate | Out-Null
    & node (Join-Path $PSScriptRoot 'generate-font-assets.mjs') --check | ForEach-Object { Write-Host $_ }
    if ($LASTEXITCODE -ne 0) { throw 'Shared language/font assets are stale.' }
    & node (Join-Path $PSScriptRoot 'generate-interface-contract.mjs') --check | ForEach-Object { Write-Host $_ }
    if ($LASTEXITCODE -ne 0) { throw 'Shared interface catalog is stale.' }
    & node (Join-Path $PSScriptRoot 'generate-branding.mjs') --check | ForEach-Object { Write-Host $_ }
    if ($LASTEXITCODE -ne 0) { throw 'Application firmware/Web artwork is stale.' }
    & node (Join-Path $PSScriptRoot 'generate-web-appearance.mjs') --check | ForEach-Object { Write-Host $_ }
    if ($LASTEXITCODE -ne 0) { throw 'Shared Web appearance is stale; regenerate before ESP build.' }
    & node (Join-Path $PSScriptRoot 'generate-basic-ui.mjs') --check | ForEach-Object { Write-Host $_ }
    if ($LASTEXITCODE -ne 0) { throw 'Basic cell app is stale; regenerate before ESP build.' }
    & node (Join-Path $PSScriptRoot 'build-web.mjs') --profile=onboard --check | ForEach-Object { Write-Host $_ }
    if ($LASTEXITCODE -ne 0) { throw 'Compressed ESP8266 Web UI is stale; regenerate before ESP build.' }
}
$libraries = Join-Path $projectRoot 'libraries'
$build = Join-Path $projectRoot 'build'
$projectCli = Join-Path $PSScriptRoot 'arduino-cli.exe'
$cli = if (Test-Path -LiteralPath $projectCli) {
    $projectCli
} elseif ($Plan) {
    # Planning and storage tests do not require installing an unrelated SDK.
    'arduino-cli'
} else {
    (Get-Command arduino-cli -ErrorAction Stop).Source
}
$busyFlag = if ($LinkMode -eq 'busy-wire') { 1 } else { 0 }
$bufferedFlag = if ($LinkMode -eq 'buffered') { 1 } else { 0 }
$suffix = if ($busyFlag) { '-busy-wire' } elseif ($bufferedFlag) { '-buffered' } else { '' }
$avrOutput = Join-Path $build ("avr$suffix")
$espOutput = Join-Path $build ("esp8266$suffix")
$extraFlags = "-DPAKAR_USE_BUSY_WIRE=$busyFlag -DPAKAR_UART_BUFFERED=$bufferedFlag"
if($DisablePeerVersionChecks){$extraFlags += ' -DPAKAR_ENFORCE_PEER_VERSIONS=0'}
$espFlags = $extraFlags
$gameModeCode = if($GameMode -eq 'first-wins'){2}else{1}
if($Target -ne 'esp32s3'){$extraFlags += " -DPAKAR_GAME_MODE=$gameModeCode";$espFlags += " -DPAKAR_GAME_MODE=$gameModeCode"}
if ($DeviceName) {
    if ($DeviceName -in @('local', 'localhost')) { throw 'Reserved device hostname' }
    # S3 keeps board/profile-only identity in its generated application header
    # so changing a hostname does not invalidate every SDK/library object.
    if ($Target -ne 'esp32s3') { $espFlags += " -DPAKAR_INITIAL_NAME=$DeviceName" }
}

foreach ($board in @('avr', 'esp8266', 'esp32s3')) {
    if ($board -eq 'esp32s3' -and $Target -ne 'esp32s3') { continue }
    if ($Target -ne 'both' -and $Target -ne $board) { continue }
    $output = if ($board -eq 'avr') { $avrOutput } elseif ($board -eq 'esp32s3') { Join-Path $build 'esp32s3-buffered' } else { $espOutput }
    if($board -eq 'esp32s3' -and $S3HostTransport -ne 'uart0') {
        # Keep explicit connector variants out of the updater's default UART0
        # candidate path. Outputs differ; the existing warm compiler cache does not.
        $output=Join-Path $build ("esp32s3-buffered-$S3HostTransport")
    }
    $flags = if ($board -eq 'avr') { $extraFlags } else { $espFlags }
    if($board -eq 'avr') {
        # compiler.cpp.extra_flags replaces MiniCore's selected LTO menu value.
        # Preserve -flto explicitly, not just on C/link recipes, or the sketch
        # silently loses LTO and can exceed Urboot's application boundary.
        $flags += ' -flto'
        $avrAutomationFlag=if($AvrAutomations -eq 'enabled' -or ($AvrAutomations -eq 'auto' -and $GameMode -ne 'first-wins')){1}else{0}
        $flags += " -DPAKAR_PC_AUTOMATIONS=$avrAutomationFlag"
        if ($AvrRadio) { $flags += ' -DPAKAR_PC_RF=1' }
        if ($AvrLedDiagnostic) { $flags += ' -DPAKAR_AVR_LED_DIAGNOSTIC=1' }
    }
    $fqbn = if ($board -eq 'avr') {
        'MiniCore:avr:328:bootloader=uart0,eeprom=keep,baudrate=115200,variant=modelP,BOD=2v7,LTO=Os_flto,clock=16MHz_external'
    } elseif ($board -eq 'esp32s3') {
        $partitions = if ($S3FlashMB -eq 4) { 'min_spiffs' } else { 'default_8MB' }
        # Explicit hardware assumptions; deploy only after reading actual flash.
        # 16/32 MB hardware deliberately uses the proven first 8 MB layout.
        # The CLI expands this property to argv: embedded literal quotes become
        # part of GCC's search path. The canonical build roots contain no spaces.
        if ($projectRoot -match '\s') { throw 'S3 shared-source include root requires a whitespace-free checkout path.' }
        $hostCode=if($S3HostTransport -eq 'otg'){2}elseif($S3HostTransport -eq 'hwcdc'){1}else{0}
        $usbMode=if($S3HostTransport -eq 'otg'){'default'}else{'hwcdc'}
        $cdcBoot=if($S3HostTransport -eq 'otg'){'default'}else{'cdc'}
        # Manual OTG startup disables DTR/1200-baud reboot before USB.begin.
        $flags += " -I$($projectRoot.Replace('\','/')) -DPAKAR_S3_HOST_TRANSPORT=$hostCode"
        if ($S3StatusRgbPin -ne -1) { $flags += " -DPAKAR_S3_STATUS_RGB_PIN=$S3StatusRgbPin" }
        # S3 commissioning switches live in the application profile below;
        # changing them must not invalidate every SDK/library object.
        if ($PauseAvrLink) { $flags += ' -DPAKAR_AVR_LINK_ENABLED=0' }
        if ($PauseLedOutput) { $flags += ' -DPAKAR_LED_START_ENABLED=0' }
        if ($CellChaseOnBoot) { $flags += ' -DPAKAR_LED_START_CELL_CHASE=1' }
        if ($S3Ethernet) { $flags += ' -DPAKAR_S3_ETHERNET_ENABLED=1' }
        "esp32:esp32:esp32s3:CDCOnBoot=$cdcBoot,USBMode=$usbMode,FlashSize=${S3FlashMB}M,PartitionScheme=$partitions,PSRAM=$S3Psram"
    } else {
        if ($projectRoot -match '\s') { throw 'Shared-source include root requires a whitespace-free checkout path.' }
        $flags += " -I$($projectRoot.Replace('\','/'))"
        'esp8266:esp8266:nodemcuv2:eesz=4M2M'
    }
    # FQBN properties participate in every SDK/library compile. Keep incompatible
    # S3 hardware/USB profiles in separate warm caches so a 4 MB recovery build
    # cannot evict the 16 MB production cache (or vice versa).
    $cacheKey = if ($board -eq 'esp32s3') {
        "esp32s3$suffix-f${S3FlashMB}m-psram-$S3Psram-host-$S3HostTransport$(if($S3Ethernet){'-eth'}else{'-wifi'})"
    } else { "$board$suffix" }
    $cache = Join-Path $build "cache/$cacheKey"
    # Retain a pre-existing canonical warm cache when its hardware identity
    # matches exactly. Introducing profile keys must not copy or discard it.
    if ($board -eq 'esp32s3') {
        $legacyCache = Join-Path $build "cache/$board$suffix"
        $legacyOptions = Join-Path $legacyCache 'build.options.json'
        if (Test-Path -LiteralPath $legacyOptions) {
            $legacyProfile = Get-Content -LiteralPath $legacyOptions -Raw | ConvertFrom-Json
            if ($legacyProfile.fqbn -ceq $fqbn -and
                $legacyProfile.customBuildProperties -is [string] -and
                $legacyProfile.customBuildProperties.Contains("compiler.cpp.extra_flags=$flags,")) {
                $cache = $legacyCache
            }
        }
    }
    $arguments = @('compile', '--jobs', "$Jobs", '--fqbn', $fqbn,
        '--libraries', $libraries, '--build-property', "compiler.cpp.extra_flags=$flags",
        '--build-path', $cache, '--output-dir', $output,
        (Join-Path $projectRoot "firmware/$board"))
    if ($board -eq 'esp32s3') {
        # Arduino's initial sketch dependency scan also uses the C recipe.
        $arguments += @('--build-property', "compiler.c.extra_flags=$flags")
        $arguments += @('--build-property', 'compiler.optimization_flags=-Os -g0')
        $taskLinkFlags='compiler.c.elf.extra_flags=-Wl,--wrap=esp_mbedtls_mem_calloc,--wrap=dns_clear_cache,--wrap=gdma_new_ahb_channel,--wrap=esp_intr_alloc_intrstatus,--wrap=esp_partition_write,--wrap=esp_partition_write_raw,--wrap=esp_partition_erase_range,--wrap=esp_partition_read,--wrap=esp_partition_read_raw,--wrap=esp_image_verify'
        if($S3Ethernet){$taskLinkFlags+=',--wrap=esp_eth_update_input_path_info'}
        $arguments += @('--build-property',$taskLinkFlags)
    }
    if ($Clean) { $arguments += '--clean' }
    $ledProfile = $null
    if ($board -eq 'esp32s3') {
        $ledProfile = [pscustomobject]@{
            Path = Join-Path $build 'config/esp32s3/LedBackend.h'
            Content = "#pragma once`n#ifndef PAKAR_GAME_MODE`n#define PAKAR_GAME_MODE $gameModeCode`n#endif`n#ifndef PAKAR_INITIAL_KEY_MASK`n#define PAKAR_INITIAL_KEY_MASK $InitialKeyMask`n#endif`n#define PAKAR_S3_LED_GPIO_ENABLED $(if ($S3LedGpio) { 1 } else { 0 })`n#define PAKAR_S3_LED_RMT_ENABLED $(if ($S3LedRmtDma) { 1 } else { 0 })`n#define PAKAR_S3_LED_DEFAULT_BACKEND $s3LedDefaultCode`n#ifndef PAKAR_S3_BLE_ENABLED`n#define PAKAR_S3_BLE_ENABLED $(if ($S3Bluetooth -eq 'disabled') { 0 } else { 1 })`n#endif`n#define PAKAR_TELNET_SESSIONS $ShellSessions`n#define PAKAR_SERIAL_SHELL_ENABLED $(if ($DisableSerialShell) { 0 } else { 1 })`n#define PAKAR_TELNET_ENABLED $(if ($DisableTelnet) { 0 } else { 1 })`n"
        }
        $ledProfile.Content += "#define PAKAR_NETWORK_ENABLED $(if($DisableNetworking){0}else{1})`n"
        $ledProfile.Content += "#define PAKAR_REMOTE_QUERY_UDP_ENABLED $(if($EnableUdpDiagnostics){1}else{0})`n"
        $ledProfile.Content += "#define PAKAR_BOARD_SNAKE $(if($BoardSnake){1}else{0})`n"
        foreach($miniMode in @('press-to-win','illuminated','color-match','simon','memory-pairs','clear-all','color-order','switch-pattern','reaction-chase','mine-field','toggle')){
            $miniFlag=$miniMode.Replace('-','_').ToUpperInvariant()
            $ledProfile.Content += "#define PAKAR_GAME_$miniFlag $(if($GameMode -eq 'first-wins' -and $miniMode -in $KeyMiniGames){1}else{0})`n"
        }
        $ledProfile.Content += "#define PAKAR_MICROSD_ENABLED $(if($S3MicroSd){1}else{0})`n"
        if($S3MicroSd){$ledProfile.Content += "#define PAKAR_SD_CLK $S3SdClk`n#define PAKAR_SD_CMD $S3SdCmd`n#define PAKAR_SD_D0 $S3SdD0`n"}
        $ledProfile.Content += "#define PAKAR_AVR_RECOVERY_IMAGE_AVAILABLE $(if($recoveryBytes){1}else{0})`n"
        if($recoveryBytes){
            $recoveryHash=[Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($recoveryBytes)).ToLowerInvariant()
            $ledProfile.Content += "namespace PakarAvrRecovery { static const unsigned char image[32768]={"+($recoveryBytes -join ',')+"}; static const char sha256[]=`"$recoveryHash`"; }`n"
        }
        $ledProfile.Content += "#define PAKAR_PIXEL_CORRECTIONS $(if($PixelCorrections -eq 'enabled'){1}else{0})`n"
        $ledProfile.Content += "#define PAKAR_SSH_ENABLED $(if($S3Ssh){1}else{0})`n"
        $ledProfile.Content += "#define PAKAR_SCP_ENABLED $(if($S3Scp){1}else{0})`n"
        $ledProfile.Content += "#define PAKAR_SFTP_ENABLED $(if($S3Sftp){1}else{0})`n"
        $ledProfile.Content += "#define PAKAR_FTP_ENABLED $(if($S3Ftp){1}else{0})`n"
        if ($DeviceName) { $ledProfile.Content += "#ifndef PAKAR_INITIAL_NAME`n#define PAKAR_INITIAL_NAME $DeviceName`n#endif`n" }
        $ledProfile.Content += "#ifndef PAKAR_S3_LED_DATA`n#define PAKAR_S3_LED_DATA $S3LedDataPin`n#endif`n"
        $ledProfile.Content += "#ifndef PAKAR_S3_ISP_ELECTRICAL_VERIFIED`n#define PAKAR_S3_ISP_ELECTRICAL_VERIFIED $(if($S3IspElectricalVerified){1}else{0})`n#endif`n#ifndef PAKAR_INITIALIZE_FILESYSTEM`n#define PAKAR_INITIALIZE_FILESYSTEM $(if($InitializeFilesystem){1}else{0})`n#endif`n"
    }
    if ($VerboseBuild) { $arguments += '--verbose' }
    if ($Plan) {
        [pscustomobject]@{ Board=$board; Executable=$cli; Arguments=$arguments; LedBackendProfile=$ledProfile }
        continue
    }
    $ledProfileLease = $null
    try {
        if ($ledProfile) {
            New-Item -ItemType Directory -Force (Split-Path -Parent $ledProfile.Path) | Out-Null
            # Hold one canonical profile/cache owner throughout compilation.
            # Another backend build must fail, not rewrite this header mid-build.
            $ledProfileLease = [IO.File]::Open($ledProfile.Path + '.lock',
                [IO.FileMode]::OpenOrCreate, [IO.FileAccess]::ReadWrite, [IO.FileShare]::None)
            if (-not (Test-Path -LiteralPath $ledProfile.Path) -or
                [IO.File]::ReadAllText($ledProfile.Path) -cne $ledProfile.Content) {
                [IO.File]::WriteAllText($ledProfile.Path, $ledProfile.Content, [Text.UTF8Encoding]::new($false))
            }
        }
        Write-PakarStep "Firmware $board · $LinkMode · reusable cache (CLI may invalidate source/options); no automatic flash"
        Invoke-PakarNative -WorkingDirectory $projectRoot -FilePath $cli -Arguments $arguments
        # Preserve only public numeric feature flags, never arbitrary compiler
        # defines (which may contain names, paths or credentials).
        $resourceFlags = [ordered]@{}
        foreach ($match in [regex]::Matches($flags, '-D(PAKAR_[A-Z0-9_]+)=(-?\d+)(?=\s|$)')) {
            $resourceFlags[$match.Groups[1].Value] = [int]$match.Groups[2].Value
        }
        if ($ledProfile) {
            foreach ($match in [regex]::Matches($ledProfile.Content, '(?m)^#define (PAKAR_[A-Z0-9_]+) (-?\d+)\s*$')) {
                $resourceFlags[$match.Groups[1].Value] = [int]$match.Groups[2].Value
            }
        }
    } finally {
        if ($ledProfileLease) { $ledProfileLease.Dispose() }
    }
    if ($board -eq 'avr') {
        & node (Join-Path $PSScriptRoot 'prepare-avr-image.mjs') $avrOutput $cache
        if ($LASTEXITCODE -ne 0) { throw 'AVR Urboot vector-chain packaging failed; do not deploy raw merged output.' }
        $mergedImage = Join-Path $avrOutput 'avr.ino.with_bootloader.bin'
        if ((Get-Item -LiteralPath $mergedImage).Length -ne 32768) {
            throw 'Merged AVR application plus Urboot image is not exactly 32768 bytes'
        }
    }
    $resourceArtifact = if ($board -eq 'avr') { Join-Path $output 'avr.ino.with_bootloader.bin' } else { Join-Path $output "$board.ino.bin" }
    $resourceVersion = [regex]::Match([IO.File]::ReadAllText((Join-Path $projectRoot 'libraries/BoardCore/src/BuildInfo.h')), 'kVersion\[\] = "([^"]+)"').Groups[1].Value
    $resourceMetadata = [ordered]@{ fqbn=$fqbn; version=$resourceVersion; numeric_feature_flags=$resourceFlags;
        artifact_sha256=(Get-FileHash -LiteralPath $resourceArtifact -Algorithm SHA256).Hash.ToLowerInvariant() }
    $resourceMetadataPath = Join-Path $output 'resource-build.json'
    $resourceText = ($resourceMetadata | ConvertTo-Json -Depth 4) + "`n"
    if (-not (Test-Path -LiteralPath $resourceMetadataPath) -or [IO.File]::ReadAllText($resourceMetadataPath) -cne $resourceText) {
        [IO.File]::WriteAllText($resourceMetadataPath,$resourceText,[Text.UTF8Encoding]::new($false))
    }
}
if (-not $Plan) { Write-Host "Requested firmware builds passed in $LinkMode mode." }
