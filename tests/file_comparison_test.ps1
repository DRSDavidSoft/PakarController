$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$scratch = Join-Path $root 'build/native/file-comparison-fixture'
New-Item -ItemType Directory -Force -Path $scratch | Out-Null
$fixtureSource = Join-Path $PSScriptRoot 'reuse_fixture/source.h'
if (-not (Test-Path -LiteralPath (Join-Path $scratch 'source.h')) -or
    (Get-FileHash -LiteralPath $fixtureSource).Hash -ne (Get-FileHash -LiteralPath (Join-Path $scratch 'source.h')).Hash) {
    Copy-Item -LiteralPath $fixtureSource -Destination $scratch
}
$source = Join-Path $scratch 'source.h'
$rawHash = (Get-FileHash -LiteralPath $source).Hash.ToLowerInvariant()
$text = [IO.File]::ReadAllText($source).Replace("`r`n", "`n")
$digest = [Security.Cryptography.SHA256]::Create()
try { $textHash = ([BitConverter]::ToString($digest.ComputeHash([Text.Encoding]::UTF8.GetBytes($text)))).Replace('-', '').ToLowerInvariant() }
finally { $digest.Dispose() }
$anchor = ([IO.File]::ReadAllLines($source) | Where-Object { $_.Trim() } | Select-Object -First 1)
$ledger = Join-Path $scratch 'ledger.csv'
$manifest = Join-Path $scratch 'evidence.json'
$output = Join-Path $scratch 'comparison.csv'
$rows = @(
    [pscustomobject]@{origin='fixture';path='source.h';revision='fixture';sha256=$rawHash;availability='present';review_status='screened'},
    [pscustomobject]@{origin='fixture';path='unreviewed.h';revision='fixture';sha256=$rawHash;availability='present';review_status='adapted'}
)
$rows | Export-Csv -NoTypeInformation -LiteralPath $ledger
$entry = @{
    origin='fixture';path='source.h';sha256=$rawHash;source_read='complete';verdict='equivalent-file'
    preserved=@('Fixture bytes retained');targets=@(@{path='source.h';sha256=$textHash;anchor=$anchor})
    proof=@();omissions=@();remaining=@();improvement=''
}
$baseline = @{schema=1;target_hash_encoding='UTF8-without-BOM-LF';files=@($entry)} | ConvertTo-Json -Depth 8
$options = @{LedgerPath=$ledger;EvidencePath=$manifest;OutputPath=$output;TargetRoot=$scratch;ReportPath=(Join-Path $scratch 'report.md')}
function Save-Manifest($value) { $value | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $manifest -Encoding utf8 }
function Fresh-Manifest { return $baseline | ConvertFrom-Json }
function Reject([string]$name, [scriptblock]$mutate, [string]$pattern) {
    $value = Fresh-Manifest
    & $mutate $value
    Save-Manifest $value
    $rejected = $false
    try { & (Join-Path $root 'tools/generate-file-comparison.ps1') @options | Out-Null }
    catch { $rejected = $_.Exception.Message -match $pattern }
    if (-not $rejected) { throw "Comparison incorrectly accepted $name" }
}
Save-Manifest (Fresh-Manifest)
& (Join-Path $root 'tools/generate-file-comparison.ps1') @options | Out-Null
$result = @(Import-Csv -LiteralPath $output)
$beforeCheck = (Get-Item -LiteralPath $output).LastWriteTimeUtc
& (Join-Path $root 'tools/generate-file-comparison.ps1') @options -Check | Out-Null
# Imported donor host/storage helpers enable StrictMode. An audit-only source
# has zero specialist routes, not an object with a missing issue property.
& {
    Set-StrictMode -Version Latest
    & (Join-Path $root 'tools/generate-file-comparison.ps1') @options -Check | Out-Null
}
$withoutOptionalAdvantage = Fresh-Manifest
$withoutOptionalAdvantage.files[0].PSObject.Properties.Remove('improvement')
Save-Manifest $withoutOptionalAdvantage
& {
    Set-StrictMode -Version Latest
    & (Join-Path $root 'tools/generate-file-comparison.ps1') @options -Check | Out-Null
}
if ((Get-Item -LiteralPath $output).LastWriteTimeUtc -ne $beforeCheck) { throw 'Read-only check wrote the ledger' }
$checkBlocked = $false
[IO.File]::WriteAllText($output, 'stale-report-fixture')
try { & (Join-Path $root 'tools/generate-file-comparison.ps1') @options -Check | Out-Null }
catch { $checkBlocked = $_.Exception.Message -match 'Committed comparison report is stale' }
if (-not $checkBlocked -or [IO.File]::ReadAllText($output) -ne 'stale-report-fixture') { throw 'Check must reject stale report without silently rewriting it' }
& (Join-Path $root 'tools/generate-file-comparison.ps1') @options | Out-Null
if ($result.Count -ne 2 -or $result[0].comparison -ne 'equivalent-file' -or
    $result[1].comparison -ne 'pending-comparison' -or
    $result[1].omission_reason -notmatch 'No approved reason' -or
    $result[0].target_mapping -notmatch ':\d+ \[UTF8-LF sha256=' -or
    $result[1].concerns -ne 'donor-audit: specialist routing pending' -or
    $result[1].tracking_issues -notmatch '/issues/3') {
    throw 'Coverage, explicit pending status or exact line reference lost'
}
$blocked = $false
try { & (Join-Path $root 'tools/generate-file-comparison.ps1') @options -RequireComplete | Out-Null }
catch { $blocked = $_.Exception.Message -match 'Superiority certification blocked' }
if (-not $blocked) { throw 'Incomplete scope incorrectly certified as superior' }
Reject 'stale target' { param($v) $v.files[0].targets[0].sha256 = 'a' * 64 } 'Target or proof changed'
Reject 'stale source' { param($v) $v.files[0].sha256 = 'b' * 64 } 'Donor hash changed'
Reject 'missing anchor' { param($v) $v.files[0].targets[0].anchor = 'not in file' } 'Missing proof anchor'
Reject 'escaping path' { param($v) $v.files[0].targets[0].path = '../source.h' } 'escapes repository'
Reject 'absolute path' { param($v) $v.files[0].targets[0].path = $source } 'Absolute evidence target'
Reject 'duplicate source' { param($v) $v.files += $v.files[0] } 'Duplicate comparison'
Reject 'vague omission' { param($v) $v.files[0].omissions = @(@{code='utility';disposition='pending'}) } 'Every omitted behavior'
Reject 'hidden pending gap' { param($v) $v.files[0].omissions = @(@{code='callback scheduler';disposition='pending';reason='Useful callback cancellation has no target implementation'}) } 'Pending useful code cannot be certified'
Reject 'unreviewed source' { param($v) $v.files[0].source_read = 'screened' } 'full manual read'
Reject 'unproven improvement' { param($v) $v.files[0].verdict = 'scoped-improvement' } 'no named advantage or test evidence'
Reject 'unactionable gap' { param($v) $v.files[0].verdict = 'parity-gap' } 'concrete next action'
Reject 'uninventoried donor' { param($v) $v.files[0].path = 'orphan.h' } 'omitted from inventory'
Reject 'blanket exclusion' { param($v) $v.files[0].verdict='not-applicable' } 'Excluded file needs'
$excluded=Fresh-Manifest
$excluded.files[0].verdict='not-applicable'
$excluded.files[0].omissions=@([pscustomobject]@{code='Fixture-only physical peripheral';disposition='not-applicable';reason='Explicit peripheral exclusion, no generic code present in fixture'})
Save-Manifest $excluded
& (Join-Path $root 'tools/generate-file-comparison.ps1') @options | Out-Null
if((Import-Csv $output)[0].comparison -ne 'not-applicable'){throw 'Specific exclusion lost'}
Write-Host 'Comparison tests passed: all rows accounted for; source/target hashes and line anchors bound; stale/escaping/duplicate/vague evidence rejected; incomplete scope cannot certify superiority.'
Write-Host "Tiny fixture retained for debugging: $scratch"
