[CmdletBinding()]
param(
    [string]$LedgerPath = (Join-Path $PSScriptRoot '../docs/legacy-source-ledger.csv'),
    [string]$EvidencePath = (Join-Path $PSScriptRoot '../docs/file-comparison-evidence.json'),
    [string]$IndexPath = (Join-Path $PSScriptRoot '../docs/reuse-capabilities.json'),
    [string]$OutputPath = (Join-Path $PSScriptRoot '../docs/file-comparison.csv'),
    [string]$ReportPath = (Join-Path $PSScriptRoot '../docs/FILE-COMPARISON.md'),
    [string]$TargetRoot = (Join-Path $PSScriptRoot '..'),
    [switch]$Check,
    [switch]$RequireComplete
)
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'donor-files.ps1')
$targetDirectory = [IO.Path]::GetFullPath($TargetRoot).TrimEnd('\','/')
function Write-ChangedReport([string]$Path, [string]$Text) {
    $full = [IO.Path]::GetFullPath($Path)
    if (-not (Test-Path -LiteralPath $full) -or [IO.File]::ReadAllText($full).Replace("`r`n","`n") -ne $Text) {
        if ($Check) { throw "Committed comparison report is stale: $Path; regenerate after explicit rereview." }
        [IO.File]::WriteAllText($full, $Text, [Text.UTF8Encoding]::new($false))
    }
}
function Get-TextHash([string]$full) {
    # Evidence hashes retain every character except platform newline encoding.
    $bytes = [Text.Encoding]::UTF8.GetBytes([IO.File]::ReadAllText($full).Replace("`r`n", "`n"))
    $digest = [Security.Cryptography.SHA256]::Create()
    try { return ([BitConverter]::ToString($digest.ComputeHash($bytes))).Replace('-', '').ToLowerInvariant() }
    finally { $digest.Dispose() }
}
function Resolve-Target([string]$relative) {
    if ([IO.Path]::IsPathRooted($relative)) { throw "Absolute evidence target rejected: $relative" }
    $full = [IO.Path]::GetFullPath((Join-Path $targetDirectory $relative))
    if (-not $full.StartsWith($targetDirectory + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Evidence target escapes repository: $relative"
    }
    if (-not (Test-Path -LiteralPath $full -PathType Leaf)) { throw "Missing exact evidence target: $relative" }
    return $full
}
$manifest = Get-Content -LiteralPath $EvidencePath -Raw | ConvertFrom-Json
$capabilities = Get-Content -LiteralPath $IndexPath -Raw | ConvertFrom-Json
if ($capabilities.schema -ne 1) { throw 'Unsupported reuse capability index' }
if ($manifest.schema -ne 1 -or $manifest.target_hash_encoding -ne 'UTF8-without-BOM-LF') { throw 'Unsupported comparison schema or target hash encoding' }
$evidence = @{}
$knownVerdicts = @('equivalent-file', 'equivalent-behavior', 'scoped-improvement', 'parity-gap', 'not-applicable')
foreach ($entry in $manifest.files) {
    $key = "$($entry.origin)|$($entry.path)"
    if ($evidence.ContainsKey($key)) { throw "Duplicate comparison: $key" }
    if ($entry.sha256 -notmatch '^[0-9a-f]{64}$' -or $entry.source_read -ne 'complete') { throw "Comparison requires full manual read and source hash: $key" }
    if ($entry.verdict -notin $knownVerdicts -or -not $entry.preserved -or -not $entry.targets.Count) { throw "Incomplete comparison: $key" }
    if ($entry.origin -eq 'Orbit' -and
        (-not $entry.PSObject.Properties['source_lines'] -or $entry.source_lines -lt 1 -or
         -not $entry.PSObject.Properties['read_ranges'] -or @($entry.read_ranges).Count -ne 1 -or
         $entry.read_ranges[0].first -ne 1 -or $entry.read_ranges[0].last -ne $entry.source_lines)) {
        throw "Orbit comparison needs exact complete-text read coverage: $key"
    }
    if ($entry.verdict -eq 'not-applicable' -and
        (-not @($entry.omissions).Count -or @($entry.omissions | Where-Object disposition -ne 'not-applicable').Count -or @($entry.remaining).Count)) {
        throw "Excluded file needs specific non-applicable code/reason and no hidden pending functionality: $key"
    }
    foreach ($target in @($entry.targets) + @($entry.proof)) {
        $full = Resolve-Target $target.path
        if ($target.sha256 -notmatch '^[0-9a-f]{64}$' -or (Get-TextHash $full) -ne $target.sha256) {
            throw "Target or proof changed since comparison; rereview required: $key -> $($target.path)"
        }
        if (-not $target.anchor -or -not ([IO.File]::ReadAllText($full).Contains($target.anchor))) { throw "Missing proof anchor: $key -> $($target.path)" }
    }
    foreach ($omission in $entry.omissions) {
        if (-not $omission.code -or -not $omission.reason -or $omission.disposition -notin @('replaced','pending','not-applicable')) {
            throw "Every omitted behavior requires exact code, disposition and concrete reason: $key"
        }
        if ($omission.disposition -eq 'pending' -and $entry.verdict -ne 'parity-gap') {
            throw "Pending useful code cannot be certified equivalent or superior: $key"
        }
    }
    if ($entry.verdict -eq 'scoped-improvement' -and (-not $entry.improvement -or -not $entry.proof.Count)) { throw "Improvement has no named advantage or test evidence: $key" }
    if ($entry.verdict -eq 'parity-gap' -and -not $entry.remaining.Count) { throw "Parity gap needs a concrete next action: $key" }
    $evidence[$key] = $entry
}
function Format-Reference($reference) {
    $full = Resolve-Target $reference.path
    $lines = [IO.File]::ReadAllLines($full)
    $line = 0
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i].Contains($reference.anchor)) { $line = $i + 1; break }
    }
    if ($line -eq 0) { throw "Evidence anchor must fit one line: $($reference.path)" }
    return "$($reference.path):$line [UTF8-LF sha256=$($reference.sha256); anchor=$($reference.anchor)]"
}
$seen = @{}
$rows = foreach ($source in (Import-Csv -LiteralPath $LedgerPath)) {
    $key = "$($source.origin)|$($source.path)"
    if ($seen.ContainsKey($key)) { throw "Duplicate inventoried source: $key" }
    $seen[$key] = $true
    $entry = $evidence[$key]
    $routes = @($capabilities.concerns | Where-Object {
        $_.id -ne 'donor-audit' -and $source.origin -in $_.origins -and $source.path -match $_.path_pattern
    })
    if ($entry -and (-not (Test-PakarDonorSnapshot $source $entry) -or $entry.sha256 -ne $source.sha256)) { throw "Donor hash changed or source absent: $key" }
    [pscustomobject][ordered]@{
        origin=$source.origin; path=$source.path; revision=$source.revision; source_sha256=$source.sha256
        concerns=$(if ($routes.Count) { $routes.id -join ' | ' } else { 'donor-audit: specialist routing pending' })
        tracking_issues=(@(3) + @($routes | ForEach-Object {$_.issue}) | Sort-Object -Unique | ForEach-Object { "https://github.com/atomicdeploy/PakarController/issues/$_" }) -join ' | '
        comparison= $(if ($entry) { $entry.verdict } else { 'pending-comparison' })
        source_read=$(if ($entry) { 'complete' } elseif ($source.review_status -eq 'complete-read-awaiting-comparison') { 'complete-awaiting-comparison' } else { 'not-certified-by-this-comparison' })
        target_mapping=$(if ($entry) { ($entry.targets | ForEach-Object { Format-Reference $_ }) -join ' | ' } else { '' })
        preserved=$(if ($entry) { $entry.preserved -join ' | ' } else { '' })
        omitted_code=$(if ($entry) { ($entry.omissions | ForEach-Object { "$($_.code) [$($_.disposition)]" }) -join ' | ' } else { 'Not yet determined; not an accepted exclusion' })
        omission_reason=$(if ($entry) { ($entry.omissions | ForEach-Object { "$($_.code): $($_.reason)" }) -join ' | ' } else { "No approved reason to leave this file out. $($source.review_status) is not behavior-comparison proof." })
        proof_mapping=$(if ($entry) { ($entry.proof | ForEach-Object { Format-Reference $_ }) -join ' | ' } else { '' })
        improvement=$(if ($entry -and $entry.PSObject.Properties['improvement']) { $entry.improvement } else { '' })
        remaining=$(if ($entry) { $entry.remaining -join ' | ' } elseif ($source.review_status -eq 'complete-read-awaiting-comparison') { 'Compare the already-read exact bytes against target code; reread on hash drift. Reading alone is not an approved omission.' } else { 'Read and compare this exact donor; map every applicable behavior and justify each omission.' })
        runtime_acceptance='Not established by source/hash/test-code comparison; see issue #2'
    }
}
foreach ($key in $evidence.Keys) { if (-not $seen.ContainsKey($key)) { throw "Comparison source omitted from inventory: $key" } }
if (-not $Check) { New-Item -ItemType Directory -Force -Path (Split-Path -Parent $OutputPath) | Out-Null }
Write-ChangedReport $OutputPath ((($rows | ConvertTo-Csv -NoTypeInformation) -join "`n") + "`n")
$rows | Group-Object origin,comparison | Sort-Object Name | Select-Object Name,Count | Format-Table -AutoSize
$pending = @($rows | Where-Object comparison -eq 'pending-comparison').Count
$gaps = @($rows | Where-Object comparison -eq 'parity-gap').Count
$auditOnly = @($rows | Where-Object concerns -eq 'donor-audit: specialist routing pending').Count
Write-Host "$($rows.Count) files accounted for; $($evidence.Count) hash-bound comparisons; $pending pending; $gaps known parity gaps. Test-code anchors are not test execution or physical proof."
if ($ReportPath) {
    $report = [Collections.Generic.List[string]]::new()
    $report.Add('# PakarController donor comparison')
    $report.Add('')
    $report.Add('Whole-repository superiority is **not established**. This report exposes retained code, scoped improvements, omissions and gaps; it does not turn missing reviews into exclusions.')
    $report.Add('')
    $report.Add("Routing: $auditOnly paths have no specialist capability match. They remain visible through ``consult-reuse.ps1 -Concern donor-audit`` and issue #3; lack of a keyword match is NOT an exclusion. Every CSV row includes capability routing and owning issue links.")
    $report.Add('')
    $report.Add("Inventory: $($rows.Count) files; $($evidence.Count) complete source-file comparisons; $pending pending comparisons; $gaps compared files with known parity gaps. Header/implementation pairs can describe the same gap. Missing sparse files remain pending.")
    $report.Add('')
    $report.Add('Every inventoried path, source revision/hash, target/proof file and line, omitted code, disposition and concrete reason is in `file-comparison.csv`. Hash-bound manual decisions are in `file-comparison-evidence.json`. Donor SHA-256 values bind original bytes; target/proof SHA-256 values bind UTF-8 without BOM with LF newlines. Anchors are recomputed into exact line numbers. Test-code presence is not test execution; execution receipts are in `FILE-COMPARISON-RECEIPTS.md`.')
    $report.Add('')
    $report.Add('## Coverage')
    $report.Add('')
    $report.Add('| Donor | Inventoried | Compared | Explicit exclusions | Pending | Files with gaps |')
    $report.Add('|---|---:|---:|---:|---:|---:|')
    foreach ($group in ($rows | Group-Object origin | Sort-Object Name)) {
        $unread = @($group.Group | Where-Object comparison -eq 'pending-comparison').Count
        $open = @($group.Group | Where-Object comparison -eq 'parity-gap').Count
        $excluded = @($group.Group | Where-Object comparison -eq 'not-applicable').Count
        $report.Add("| $($group.Name) | $($group.Count) | $($group.Count - $unread - $excluded) | $excluded | $unread | $open |")
    }
    $report.Add('')
    $report.Add('## Exact reviewed-file mappings and omission decisions')
    foreach ($row in ($rows | Where-Object comparison -ne 'pending-comparison')) {
        $report.Add('')
        $report.Add("### $($row.origin): ``$($row.path)``")
        $report.Add('')
        $report.Add("Verdict: **$($row.comparison)**. Revision: ``$($row.revision)``. Original-byte SHA-256: ``$($row.source_sha256)``.")
        $report.Add('')
        $entry = $evidence["$($row.origin)|$($row.path)"]
        if($entry.PSObject.Properties['read_ranges']){$report.Add("- Complete text read: lines 1-$($entry.source_lines), exact source hash above.")}
        foreach ($target in $entry.targets) { $report.Add('- Target: `' + (Format-Reference $target) + '`.') }
        foreach ($retained in $entry.preserved) { $report.Add('- Retained: ' + $retained) }
        foreach ($omission in $entry.omissions) { $report.Add("- Omitted/replaced code: ``$($omission.code)``. Disposition: **$($omission.disposition)**. Reason: $($omission.reason)") }
        if (-not $entry.omissions.Count) { $report.Add('- No production-code omission claimed for this reviewed file.') }
        foreach ($proof in $entry.proof) { $report.Add('- Test-code evidence: `' + (Format-Reference $proof) + '`.') }
        if ($row.improvement) { $report.Add('- Scoped advantage: ' + $row.improvement) }
        foreach ($remaining in $entry.remaining) { $report.Add('- Remaining: ' + $remaining) }
    }
    $report.Add('')
    $report.Add('## Unreviewed files are not approved omissions')
    $report.Add('')
    $report.Add('Every pending path has its own CSV row. Its omission reason explicitly says there is no approved reason to leave it out. Prior keyword screening or an unbound historical adaptation note is not accepted as behavior proof. This includes all still-pending PuzzleBoard/40LED variants and ControllerBoardMini legacy files, plus unreviewed Pealayer/PCController/PU850 files, assets, dependencies, tests and scripts. Earlier focused variant notes remain useful routing evidence, not complete comparative certification.')
    $report.Add('')
    $report.Add('File-by-file checklist: https://github.com/atomicdeploy/PakarController/issues/3. Master production acceptance: https://github.com/atomicdeploy/PakarController/issues/2. Run `pwsh -File tools/generate-file-comparison.ps1 -RequireComplete` to enforce the comparison gate. It must fail while any pending comparison or known source parity gap remains. Physical, deployment, visual and all-surface acceptance are separate gates even after this one passes.')
    if (-not $Check) { New-Item -ItemType Directory -Force -Path (Split-Path -Parent $ReportPath) | Out-Null }
    Write-ChangedReport $ReportPath (($report -join "`n") + "`n")
}
if ($RequireComplete -and ($pending -or $gaps)) { throw "Superiority certification blocked: $pending un-compared files and $gaps known parity gaps." }
