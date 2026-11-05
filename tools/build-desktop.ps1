[CmdletBinding()]
param(
    [ValidateSet('Check', 'Test', 'Release')] [string] $Mode = 'Check',
    [ValidateRange(1, 32)] [int] $Jobs = 2,
    [ValidateRange(1, 1024)] [int] $MinimumFreeGiB = 8,
    [ValidateRange(1, 1024)] [int] $MaximumCacheGiB = 8,
    [string] $ExpectedCommit,
    [switch] $DebugArtifacts,
    [switch] $DisablePeerVersionChecks,
    [string] $TestFilter,
    [switch] $Plan
)
$ErrorActionPreference = 'Stop'
& node (Join-Path $PSScriptRoot '../interfaces/controller-wire/consumer.mjs') --pakar --check | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw 'Canonical opcode bindings are stale.' }
. (Join-Path $PSScriptRoot 'script-runtime.ps1')
if ($TestFilter -and $Mode -ne 'Test') { throw 'TestFilter requires Test mode.' }
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern api-contract -Gate | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern client-app-updates -Gate | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern pu-tooling -Gate | Out-Null
& node (Join-Path $PSScriptRoot 'check-living-contract.mjs') | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw 'Living API policy failed before native build.' }
$taskReuse = & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern build-storage -Gate
$taskLocaleReuse = & (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern localization -Gate
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern native-dropdowns -Gate | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern peer-binding -Gate | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern windows-startup -Gate | Out-Null
& node (Join-Path $PSScriptRoot 'generate-interface-contract.mjs') --check | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw 'Shared interface catalog is stale.' }
$taskRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
& node (Join-Path $PSScriptRoot 'check-crate-boundary.mjs') | ForEach-Object { Write-Host $_ }
if ($LASTEXITCODE -ne 0) { throw 'Donor crate/resource guard failed before native build.' }
. (Join-Path $PSScriptRoot 'windows-build-host.ps1')
$taskManifest = Join-Path $taskRoot 'desktop/Cargo.toml'
$taskTarget = Get-PakarCargoTargetDirectory -RepositoryRoot $taskRoot
$taskVerb = if ($Mode -eq 'Check') { 'check' } elseif ($Mode -eq 'Test') { 'test' } else { 'build' }
$taskArguments = @($taskVerb, '--locked', '--manifest-path', $taskManifest,
    '--target-dir', $taskTarget, '--jobs', "$Jobs", '--bin', 'pakarcontroller')
# Reuse the warm production dependency profile for routine checks/tests too.
# Debug output is an explicit, essential-diagnostics opt-in, not another cache.
if ($Mode -eq 'Release' -and $DebugArtifacts) { throw 'Release cannot request debug artifacts.' }
if (-not $DebugArtifacts) { $taskArguments += '--release' }
if ($Mode -eq 'Test') {
    if ($TestFilter) { $taskArguments += $TestFilter }
    # Actual Pealayer CI test contract: compile jobs != test concurrency.
    # Native tests share process-wide preferences; preserve serial execution.
    # Emit assertion output immediately so an eventual native abort cannot hide
    # the earlier failures until a test-harness summary that never arrives.
    $taskArguments += @('--', '--test-threads=1', '--nocapture')
}
$taskOriginalConfig = [Environment]::GetEnvironmentVariable('PAKAR_CONFIG_FILE', 'Process')
$taskOriginalPeerPolicy=[Environment]::GetEnvironmentVariable('PAKAR_ENFORCE_PEER_VERSIONS','Process')
$taskTestConfig = if ($Mode -eq 'Test') {
    if ($taskOriginalConfig) { $taskOriginalConfig } else { Join-Path $taskRoot 'build/native/native-tests.json' }
} else { $null }
if ($Plan) {
    [pscustomobject]@{Executable='cargo'; Arguments=$taskArguments; Cache=$taskTarget;
        MinimumFreeGiB=$MinimumFreeGiB; MaximumCacheGiB=$MaximumCacheGiB; ReuseGate=$taskReuse.GateChecked;
        TestConfigurationFile=$taskTestConfig; DebugArtifacts=[bool]$DebugArtifacts}
    return
}
$taskDrive = Get-PSDrive -PSProvider FileSystem | Where-Object {
    $taskTarget.StartsWith($_.Root, [StringComparison]::OrdinalIgnoreCase) -and $_.Name -ne 'Temp'
} | Sort-Object { $_.Root.Length } -Descending | Select-Object -First 1
if (-not $taskDrive -or $null -eq $taskDrive.Free) { throw 'Cannot verify build-volume free space.' }
if ($taskDrive.Free -lt $MinimumFreeGiB * 1GB) {
    throw "Build refused: less than $MinimumFreeGiB GiB free. Inspect storage before compiling."
}
if (Test-Path -LiteralPath $taskTarget) {
    $taskSize = (Get-ChildItem -LiteralPath $taskTarget -File -Recurse -Force | Measure-Object Length -Sum).Sum
    if ($taskSize -gt $MaximumCacheGiB * 1GB) {
        throw "Build refused: canonical cache exceeds $MaximumCacheGiB GiB. Review obsolete artifacts; do not repeatedly clean/rebuild."
    }
}
# Cargo freshness does the work; never cargo clean or create another target tree.
if ($ExpectedCommit) { Assert-PakarPackageSourceStable $taskRoot $ExpectedCommit }
try {
    if($DisablePeerVersionChecks){$env:PAKAR_ENFORCE_PEER_VERSIONS='0'}
    # Donor CI isolates its tests. Local Test must also avoid the user profile;
    # retain an explicitly supplied fixture path and restore process environment.
    if ($Mode -eq 'Test') { $env:PAKAR_CONFIG_FILE = $taskTestConfig }
    Write-PakarStep "Desktop $Mode · canonical cache · $Jobs compile jobs"
    Invoke-PakarNative -WorkingDirectory $taskRoot -FilePath 'cargo' -Arguments $taskArguments
    if ($ExpectedCommit) { Assert-PakarPackageSourceStable $taskRoot $ExpectedCommit }
} finally {
    if($DisablePeerVersionChecks){
        if($null -eq $taskOriginalPeerPolicy){Remove-Item Env:PAKAR_ENFORCE_PEER_VERSIONS -ErrorAction SilentlyContinue}
        else{[Environment]::SetEnvironmentVariable('PAKAR_ENFORCE_PEER_VERSIONS',$taskOriginalPeerPolicy,'Process')}
    }
    if ($Mode -eq 'Test') {
        [Environment]::SetEnvironmentVariable('PAKAR_CONFIG_FILE', $taskOriginalConfig, 'Process')
    }
}
