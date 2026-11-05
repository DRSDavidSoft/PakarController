[CmdletBinding()]
param(
    [ValidateSet('Analyze','Test','ApkPreview','PixelProbe')][string]$Mode='Test',
    [string]$FlutterPath,
    [ValidateRange(1,1024)][int]$MinimumFreeGiB=8,
    [ValidateRange(1,1024)][int]$MaximumCacheGiB=4,
    [switch]$Restore,
    [switch]$BuildOnly,
    [switch]$BoardProductionHold,
    [switch]$DisablePeerVersionChecks,
    [ValidatePattern('^test/[A-Za-z0-9_/-]+_test\.dart$')][string]$TestPath,
    [string]$CapturePath,
    [ValidateSet('Table','TableTablet','TablePhone','SettingsTablet','SettingsPhone','FirmwareTablet','FirmwarePhone','BridgeTablet','BridgePhone','FrontPanelTablet','FrontPanelPhone','DevicesTablet','DevicesPhone','AppUpdatesTablet','AppUpdatesPhone','AutomationTablet','AutomationPhone','GameModePhone','GameModeTablet')][string]$CaptureView='Table',
    [switch]$Plan
)
$ErrorActionPreference='Stop'
& node (Join-Path $PSScriptRoot '../interfaces/controller-wire/consumer.mjs') --pakar --check | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw 'Canonical opcode bindings are stale.' }
. (Join-Path $PSScriptRoot 'script-runtime.ps1')
& (Join-Path $PSScriptRoot 'verify-versions.ps1')
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern api-contract -Gate | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern pu-tooling -Gate | Out-Null
& node (Join-Path $PSScriptRoot 'check-living-contract.mjs') | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw 'Living API policy failed before mobile build.' }
$taskRoot=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$taskMobile=Join-Path $taskRoot 'mobile'
$taskContext=& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern device-context -Gate
$taskSettings=& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern mobile-settings -Gate
$taskFirmware=& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern mobile-firmware -Gate
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern mobile-app-updates -Gate | Out-Null
$taskNetwork=& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern mobile-network -Gate
$taskMirror=& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern mobile-mirroring -Gate
$taskStorage=& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern build-storage -Gate
foreach($taskGenerator in @('generate-font-assets.mjs','generate-interface-contract.mjs','generate-branding.mjs','generate-web-appearance.mjs','generate-basic-ui.mjs','generate-web-compressed.mjs')){
    $taskGeneratorOutput=& node (Join-Path $PSScriptRoot $taskGenerator) --check 2>&1
    $taskGeneratorOutput | ForEach-Object {Write-Host $_}
    if($LASTEXITCODE -ne 0){throw "Shared mobile contract/assets stale: $taskGenerator"}
}
if(-not $FlutterPath){
    $taskConfiguredFlutter=[Environment]::GetEnvironmentVariable('PAKAR_FLUTTER')
    if($taskConfiguredFlutter){$FlutterPath=$taskConfiguredFlutter}
    elseif(Get-Command flutter -ErrorAction SilentlyContinue){$FlutterPath=(Get-Command flutter).Source}
    elseif([Environment]::OSVersion.Platform -eq [PlatformID]::Win32NT -and (Test-Path -LiteralPath 'C:\development\flutter\bin\flutter.bat')){$FlutterPath='C:\development\flutter\bin\flutter.bat'}
    else{$FlutterPath='flutter'}
}
if(-not(Test-Path -LiteralPath $FlutterPath)){
    $taskSdk=Get-Command flutter -ErrorAction SilentlyContinue
    if($taskSdk){$FlutterPath=$taskSdk.Source}
    elseif(-not $Plan){throw 'Flutter absent: inspect mobile/README.md and configured existing SDK before installing another copy.'}
}
$taskCommands=[Collections.Generic.List[object]]::new()
if($TestPath -and ($Mode -ne 'Test' -or -not(Test-Path -LiteralPath (Join-Path $taskMobile $TestPath) -PathType Leaf))){
    throw 'TestPath requires Test mode and an existing relative mobile/test file.'
}
if($BuildOnly -and $Mode -ne 'ApkPreview'){throw 'BuildOnly requires ApkPreview; tests remain explicit.'}
if($Mode -eq 'PixelProbe' -and ($Restore -or $CapturePath)){throw 'PixelProbe reuses resolved packages and does not capture a UI.'}
if($Restore){$taskCommands.Add(@('pub','get','--offline','--enforce-lockfile'))}
if($Mode -eq 'PixelProbe'){
    # Same SDK/packages/cache budget, one small standalone diagnostic consumer
    # of the actual mobile API; no Flutter SDK installation on the remote PC.
    $taskDartName=if([Environment]::OSVersion.Platform -eq [PlatformID]::Win32NT){'dart.bat'}else{'dart'}
    $FlutterPath=Join-Path (Split-Path $FlutterPath -Parent) $taskDartName
    $taskProbeName=if([Environment]::OSVersion.Platform -eq [PlatformID]::Win32NT){'pixel-probe.exe'}else{'pixel-probe'}
    $taskCommands.Add(@('compile','exe','--packages=.dart_tool/package_config.json','tool/pixel_probe.dart','--output',('build/'+$taskProbeName)))
}
elseif(-not $BuildOnly){
    $taskCommands.Add(@('analyze','--no-pub'))
    if($Mode -ne 'Analyze'){
        $taskTestArguments=@('test','--no-pub')
        if($TestPath){$taskTestArguments+=@($TestPath)}
        $taskCommands.Add($taskTestArguments)
    }
}
if($Mode -eq 'ApkPreview'){
    $taskApkArguments=@('build','apk','--release','--no-pub')
    if($BoardProductionHold){$taskApkArguments+='--dart-define=PAKAR_BOARD_PRODUCTION_HOLD=true'}
    if($DisablePeerVersionChecks){$taskApkArguments+='--dart-define=PAKAR_ENFORCE_PEER_VERSIONS=false'}
    $taskCommands.Add($taskApkArguments)
}
if($Plan){return [pscustomobject]@{
    Executable=$FlutterPath;WorkingDirectory=$taskMobile;Commands=$taskCommands;
    Cache=(Join-Path $taskMobile 'build');CacheBudgetGiB=$MaximumCacheGiB;
    MinimumFreeGiB=$MinimumFreeGiB;ContextGate=$taskContext.GateChecked;
    StorageGate=$taskStorage.GateChecked;SettingsGate=$taskSettings.GateChecked;FirmwareGate=$taskFirmware.GateChecked;NetworkGate=$taskNetwork.GateChecked;CaptureView=$CaptureView;Signing=if($Mode -eq 'PixelProbe'){'Standalone transport test tool, not an Android package'}else{'Existing debug identity: preview, not production'}
}}
. (Join-Path $PSScriptRoot 'storage-hygiene.ps1') -FunctionsOnly
$taskCache=Get-DirectorySummary (Join-Path $taskMobile 'build')
if($taskCache.GiB -gt $MaximumCacheGiB){throw 'Mobile canonical cache over budget; inspect ownership before any cleanup. Never clean/rebuild or relocate around this guard.'}
$taskDrive=Get-PSDrive -PSProvider FileSystem | Where-Object {
    $taskMobile.StartsWith($_.Root,[StringComparison]::OrdinalIgnoreCase) -and $_.Name -ne 'Temp'
} | Sort-Object {$_.Root.Length} -Descending | Select-Object -First 1
if(-not $taskDrive -or $null -eq $taskDrive.Free -or $taskDrive.Free -lt $MinimumFreeGiB*1GB){
    throw 'Insufficient verified mobile build-volume free space.'
}
if(-not $Restore -and -not(Test-Path -LiteralPath (Join-Path $taskMobile '.dart_tool/package_config.json'))){
    throw 'Dependencies not resolved. Use -Restore for checksum-locked offline restoration; no automatic upgrade or new SDK.'
}
$taskNoProxy=[Environment]::GetEnvironmentVariable('NO_PROXY','Process')
$taskCapture=[Environment]::GetEnvironmentVariable('PAKAR_MOBILE_CAPTURE_PATH','Process')
$taskCaptureView=[Environment]::GetEnvironmentVariable('PAKAR_MOBILE_CAPTURE_VIEW','Process')
if($CapturePath){
    if($Mode -eq 'Analyze' -or -not[IO.Path]::IsPathRooted($CapturePath)){throw 'Capture requires absolute output and Test/ApkPreview.'}
    $taskOutputRoot=[IO.Path]::GetFullPath((Join-Path $taskRoot '../../outputs/PakarController'))
    $taskCaptureFull=[IO.Path]::GetFullPath($CapturePath)
    if(-not $taskCaptureFull.StartsWith($taskOutputRoot+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){
        throw 'Mobile capture must stay in user deliverable outputs/PakarController.'
    }
    if(Test-Path -LiteralPath $taskCaptureFull){throw 'Capture exists; inspect it rather than overwriting evidence.'}
    New-Item -ItemType Directory -Force -Path ([IO.Path]::GetDirectoryName($taskCaptureFull)) | Out-Null
}
try{
    $env:NO_PROXY='localhost,127.0.0.1,::1'
    if($CapturePath){$env:PAKAR_MOBILE_CAPTURE_PATH=$taskCaptureFull}else{Remove-Item Env:PAKAR_MOBILE_CAPTURE_PATH -ErrorAction SilentlyContinue}
    $env:PAKAR_MOBILE_CAPTURE_VIEW=$CaptureView
    Push-Location $taskMobile
    try{
        foreach($taskArguments in $taskCommands){
            Write-PakarStep "Mobile $Mode · $($taskArguments[0]) · canonical cache"
            & $FlutterPath @taskArguments
            if($LASTEXITCODE -ne 0){throw "Mobile $Mode failed: $($taskArguments -join ' ')"}
        }
    }finally{Pop-Location}
}finally{
    if($null -eq $taskNoProxy){Remove-Item Env:NO_PROXY -ErrorAction SilentlyContinue}else{$env:NO_PROXY=$taskNoProxy}
    if($null -eq $taskCapture){Remove-Item Env:PAKAR_MOBILE_CAPTURE_PATH -ErrorAction SilentlyContinue}else{$env:PAKAR_MOBILE_CAPTURE_PATH=$taskCapture}
    if($null -eq $taskCaptureView){Remove-Item Env:PAKAR_MOBILE_CAPTURE_VIEW -ErrorAction SilentlyContinue}else{$env:PAKAR_MOBILE_CAPTURE_VIEW=$taskCaptureView}
}
if($Mode -eq 'ApkPreview'){
    $taskApk=Join-Path $taskMobile 'build/app/outputs/flutter-apk/app-release.apk'
    if(-not(Test-Path -LiteralPath $taskApk)){throw 'Successful command did not produce the expected APK.'}
    [pscustomobject]@{Path=$taskApk;SHA256=(Get-FileHash -LiteralPath $taskApk).Hash.ToLowerInvariant();
        Signing='Preview only; production signing and physical device acceptance remain required'}
}
if($Mode -eq 'PixelProbe'){
    $taskProbe=Join-Path $taskMobile ('build/'+$taskProbeName)
    if(-not(Test-Path -LiteralPath $taskProbe)){throw 'Expected compiled transport probe absent.'}
    [pscustomobject]@{Path=$taskProbe;SHA256=(Get-FileHash -LiteralPath $taskProbe).Hash.ToLowerInvariant();Scope='Actual mobile transport, not Android UI or optical acceptance'}
}
