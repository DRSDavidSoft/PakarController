$ErrorActionPreference='Stop'
$taskRoot=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
function Assert-Reuse($value,$message){if(-not $value){throw $message}}
function Assert-Rejected([scriptblock]$action,[string]$pattern){
    $taskRejected=$false
    try{& $action | Out-Null}catch{Assert-Reuse ($_.Exception.Message -match $pattern) "Wrong rejection: $_"; $taskRejected=$true}
    Assert-Reuse $taskRejected "Expected rejection: $pattern"
}
$taskConsult=Join-Path $taskRoot 'tools/consult-reuse.ps1'
$taskIndex=Get-Content (Join-Path $taskRoot 'docs/reuse-capabilities.json') -Raw | ConvertFrom-Json
$taskInventory=@(Import-Csv (Join-Path $taskRoot 'docs/legacy-source-ledger.csv'))
Assert-Reuse (@($taskIndex.concerns.id | Select-Object -Unique).Count -eq $taskIndex.concerns.Count) 'Unique concern IDs'
foreach($taskConcern in $taskIndex.concerns){
    $taskLookup=& $taskConsult -Concern $taskConcern.id -DonorRoots @{}
    Assert-Reuse ($taskLookup.Matched -gt 0 -and $taskLookup.Inventoried -eq $taskInventory.Count) "Inventory coverage: $($taskConcern.id)"
    Assert-Reuse ($taskLookup.Meaning -match 'NOT accepted') 'No lookup-as-acceptance claim'
}
Assert-Rejected {& $taskConsult -Concern invented -DonorRoots @{}} 'Unknown'
$taskAll=& $taskConsult -Concern donor-audit -DonorRoots @{}
Assert-Reuse ($taskAll.Matched -eq $taskAll.Inventoried) 'Exhaustive route must include every source, asset, script, test and absent blob'
$taskOrbit=& $taskConsult -Concern orbit-firmware -DonorRoots @{}
$taskOrbitInventory=@($taskInventory | Where-Object origin -eq Orbit)
Assert-Reuse ($taskOrbitInventory.Count -ge 153 -and $taskOrbit.Matched -eq $taskOrbitInventory.Count) 'Every Orbit firmware path remains visible, not keyword-filtered'
foreach($taskOrbitPath in @('firmware/modem_bridge/ControlPlane.h','firmware/modem_bridge/StorageFiles.h','firmware/modem_bridge/orbit-console.gz','firmware/modem_console/modem_console.ino','firmware/web/sw.js')){
    Assert-Reuse (@($taskOrbit.Rows | Where-Object Key -eq "Orbit|$taskOrbitPath").Count -eq 1) "Mixed/peripheral/generated/Web Orbit path retained: $taskOrbitPath"
}
Assert-Reuse ($taskAll.AuditOnly -gt 0) 'Specialist routing gaps must remain visible'
Assert-Reuse (@($taskAll.Rows | Where-Object {@($_.Remaining).Count -eq 0 -and $_.Comparison -eq 'pending-comparison'}).Count -eq 0) 'Every unread file needs an action'
$taskPealayer=& $taskConsult -Concern donor-audit -Origin Pealayer -DonorRoots @{}
Assert-Reuse ($taskPealayer.Matched -eq @($taskInventory | Where-Object origin -eq Pealayer).Count -and $taskPealayer.Matched -gt 0) 'All Pealayer paths are reachable, not just keyword matches'
$taskExact=& $taskConsult -Concern donor-audit -Origin Pealayer -SourcePath scripts/package-probe-windows.ps1 -DonorRoots @{}
Assert-Reuse ($taskExact.Matched -eq 1 -and $taskExact.Rows[0].SourceRead -eq 'complete' -and $taskExact.Rows[0].Proof.Count -gt 0) 'Exact-file lookup exposes source read and proof'
$taskOmissions=& $taskConsult -Concern donor-audit -Origin Pealayer -SourcePath scripts/libmpv-windows.ps1 -DonorRoots @{}
Assert-Reuse ($taskOmissions.Rows[0].Omissions.Count -gt 0 -and $taskOmissions.Rows[0].Omissions[0].reason) 'Exact omission reasoning exposed, not just verdict'
Assert-Rejected {& $taskConsult -Concern donor-audit -SourcePath 'scripts/*.ps1' -DonorRoots @{}} 'No inventoried donors'
Assert-Rejected {& $taskConsult -Concern preferences -Gate -DonorRoots @{}} 'No reviewed gate'
Assert-Rejected {& $taskConsult -Concern build-storage -RequireLive -DonorRoots @{}} 'unavailable or stale'
# A CI runner has neither C: nor the canonical donor checkout. A missing PS
# drive must label live bytes unavailable, not throw from Join-Path or pretend
# the donor was live-verified. Existing snapshot/target gates still apply.
$taskAbsentDriveRoots=@{Pealayer='PakarAbsentDonorDrive:\source';RayanLamp='PakarAbsentDonorDrive:\lamp'}
Assert-Reuse (-not (Get-PSDrive -Name PakarAbsentDonorDrive -ErrorAction SilentlyContinue)) 'Negative fixture drive must not exist'
$taskAbsentGate=& $taskConsult -Concern build-storage -Gate -DonorRoots $taskAbsentDriveRoots
Assert-Reuse ($taskAbsentGate.GateChecked -and @($taskAbsentGate.Rows | Where-Object Live -ne unavailable).Count -eq 0) 'Unavailable drive retains snapshot gate without live claims'
Assert-Rejected {& $taskConsult -Concern build-storage -Gate -RequireLive -DonorRoots $taskAbsentDriveRoots} 'unavailable or stale'
$taskAbsentLamp=& $taskConsult -Concern web-presentation -Gate -DonorRoots $taskAbsentDriveRoots
Assert-Reuse ($taskAbsentLamp.GateChecked -and @($taskAbsentLamp.Rows | Where-Object Live -ne unavailable).Count -eq 0) 'Rayan inventory traversal also skips unavailable drives'
foreach($taskConcern in @('build-storage','capture')){
    $taskGate=& $taskConsult -Concern $taskConcern -Gate -DonorRoots @{}
    Assert-Reuse $taskGate.GateChecked "Cached reviewed-slice gate: $taskConcern"
}
# Tiny stable owned fixtures, not copies of full donor repositories or packages.
$taskFixture=Join-Path $taskRoot 'build/native/reuse-preflight-fixture'
New-Item -ItemType Directory -Force -Path $taskFixture | Out-Null
$taskReadLedger=Join-Path $taskFixture 'read-ledger.csv'
$taskReadEvidence=Join-Path $taskFixture 'read-evidence.csv'
$taskLedgerCheck=Join-Path $taskRoot 'tools/check-reuse-ledger.ps1'
foreach($taskReadStatus in @('reviewed-open','complete-read-awaiting-comparison','compared','reviewed-excluded')){
    $taskReadRow=[pscustomobject]@{origin='Fixture';path='policy.h';revision='fixture';sha256=('a'*64);review_status=$taskReadStatus;availability='present';notes='Exact manual read, not completed delivery';reuse_target='tests/fixture'}
    $taskReadRow | Export-Csv -LiteralPath $taskReadLedger -NoTypeInformation
    $taskReadRow | Export-Csv -LiteralPath $taskReadEvidence -NoTypeInformation
    & $taskLedgerCheck -LedgerPath $taskReadLedger -EvidencePath $taskReadEvidence | Out-Null
    Assert-Rejected {& $taskLedgerCheck -LedgerPath $taskReadLedger -EvidencePath $taskReadEvidence -RequireComplete} 'unresolved donor rows'
    $taskReadRow.sha256='b'*64
    $taskReadRow | Export-Csv -LiteralPath $taskReadEvidence -NoTypeInformation
    Assert-Rejected {& $taskLedgerCheck -LedgerPath $taskReadLedger -EvidencePath $taskReadEvidence} 'Manual status is not bound'
}
$taskLedgerPath=Join-Path $taskFixture 'ledger.csv'
$taskEvidencePath=Join-Path $taskFixture 'evidence.json'
$taskIndexPath=Join-Path $taskFixture 'index.json'
$taskSource=Import-Csv (Join-Path $taskRoot 'docs/legacy-source-ledger.csv') | Where-Object { $_.origin -eq 'Pealayer' -and $_.path -eq 'scripts/package-probe-windows.ps1'}
$taskEntry=(Get-Content (Join-Path $taskRoot 'docs/file-comparison-evidence.json') -Raw | ConvertFrom-Json).files | Where-Object { $_.origin -eq 'Pealayer' -and $_.path -eq $taskSource.path}
$taskSource | Export-Csv -LiteralPath $taskLedgerPath -NoTypeInformation
$taskFixtureIndex=@{schema=1; concerns=@(@{id='fixture';origins=@('Pealayer');path_pattern='.*';required_reviews=@("Pealayer|$($taskSource.path)");issue=3})}
[IO.File]::WriteAllText($taskIndexPath,($taskFixtureIndex | ConvertTo-Json -Depth 10))
function Write-FixtureEvidence($entry){[IO.File]::WriteAllText($taskEvidencePath,(@{schema=1;files=@($entry)} | ConvertTo-Json -Depth 15))}
function Invoke-Fixture{& $taskConsult -Concern fixture -Gate -DonorRoots @{} -LedgerPath $taskLedgerPath -EvidencePath $taskEvidencePath -IndexPath $taskIndexPath}
Write-FixtureEvidence $taskEntry
Assert-Reuse (Invoke-Fixture).GateChecked 'Valid exact source/target/test fixture'
$taskChanged=$taskEntry | ConvertTo-Json -Depth 15 | ConvertFrom-Json
$taskChanged.source_read='screened'; Write-FixtureEvidence $taskChanged
Assert-Rejected {Invoke-Fixture} 'not fully reviewed'
$taskChanged=$taskEntry | ConvertTo-Json -Depth 15 | ConvertFrom-Json
$taskChanged.sha256='0'*64; Write-FixtureEvidence $taskChanged
Assert-Rejected {Invoke-Fixture} 'not fully reviewed'
$taskStale=& $taskConsult -Concern fixture -DonorRoots @{} -LedgerPath $taskLedgerPath -EvidencePath $taskEvidencePath -IndexPath $taskIndexPath
Assert-Reuse ($taskStale.Stale -eq 1 -and $taskStale.Rows[0].SourceRead -eq 'stale') 'Ungated lookup cannot present changed snapshot as completed comparison'
$taskChanged=$taskEntry | ConvertTo-Json -Depth 15 | ConvertFrom-Json
$taskChanged.targets=@(); Write-FixtureEvidence $taskChanged
Assert-Rejected {Invoke-Fixture} 'not fully reviewed'
$taskChanged=$taskEntry | ConvertTo-Json -Depth 15 | ConvertFrom-Json
$taskChanged.targets[0].sha256='0'*64; Write-FixtureEvidence $taskChanged
Assert-Rejected {Invoke-Fixture} 'target/test changed'
$taskChanged=$taskEntry | ConvertTo-Json -Depth 15 | ConvertFrom-Json
$taskChanged.targets[0].path='../escape.ps1'; Write-FixtureEvidence $taskChanged
Assert-Rejected {Invoke-Fixture} 'Escaping reuse target'
Write-FixtureEvidence $taskEntry
. (Join-Path $taskRoot 'tools/windows-build-host.ps1')
$taskPriorTarget=$env:CARGO_TARGET_DIR
try{
    $env:CARGO_TARGET_DIR='relative-cache'
    Assert-Reuse ((Get-PakarCargoTargetDirectory $taskRoot) -eq (Join-Path $taskRoot 'relative-cache')) 'Actual donor relative cache resolution'
}finally{if($null -ne $taskPriorTarget){$env:CARGO_TARGET_DIR=$taskPriorTarget}else{Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue}}
. (Join-Path $taskRoot 'tools/package-probe-windows.ps1')
$taskProbe=Invoke-PackageProbe $env:COMSPEC '/d /c echo Pakar-probe' $taskRoot
Assert-Reuse ($taskProbe.Trim() -eq 'Pakar-probe') 'Actual imported short-lived process stdout'
Assert-Rejected {Invoke-PackageProbe $env:COMSPEC '/d /c exit 7' $taskRoot} 'failed \(7\)'
Assert-Rejected {Assert-PakarPackageSourceStable $taskRoot ('0'*40)} 'source commit changed'
# Imported storage summary sets StrictMode in callers: single remaining actions
# must be treated as arrays, not scalar strings with a nonexistent Count.
. (Join-Path $taskRoot 'tools/storage-hygiene.ps1') -FunctionsOnly
$taskStrictLookup=& $taskConsult -Concern device-context -DonorRoots @{}
Assert-Reuse ($taskStrictLookup.OpenActions -gt 0) 'StrictMode-safe donor lookup after imported storage summary'
$taskMobilePlan=& (Join-Path $taskRoot 'tools/build-mobile.ps1') -Mode Test -Plan
Assert-Reuse ($taskMobilePlan.WorkingDirectory -eq (Join-Path $taskRoot 'mobile')) 'Mobile tests run in mobile, not repository root'
Assert-Reuse ($taskMobilePlan.ContextGate -and $taskMobilePlan.StorageGate) 'Mobile plan retains reviewed gates'
Assert-Reuse $taskMobilePlan.SettingsGate 'Mobile settings requires the actual complete-read PreferencesTab mapping'
Assert-Reuse $taskMobilePlan.FirmwareGate 'Mobile firmware requires actual complete-read picker and extension helpers'
Assert-Reuse ($taskMobilePlan.CaptureView -eq 'Table') 'Mobile capture defaults to the reviewed table fixture'
$taskSettingsPlan=& (Join-Path $taskRoot 'tools/build-mobile.ps1') -Mode Test -Plan -CaptureView SettingsPhone
Assert-Reuse ($taskSettingsPlan.CaptureView -eq 'SettingsPhone') 'Mobile renderer scenario is explicit; no overwritten evidence or production UI'
Assert-Reuse (($taskMobilePlan.Commands | ForEach-Object {$_ -join ' '}) -contains 'test --no-pub') 'Warm resolved mobile tests; no dependency upgrade'
Write-Host 'Reuse routing, cached gates, stale/incomplete/escaping rejection, donor cache resolver and actual imported process probe tests passed; no compiler/build/package copy.'
