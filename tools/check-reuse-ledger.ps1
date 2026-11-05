[CmdletBinding()]
param(
    [string]$LedgerPath = (Join-Path $PSScriptRoot '..\docs\legacy-source-ledger.csv'),
    [string]$EvidencePath = (Join-Path $PSScriptRoot '..\docs\reuse-review-evidence.csv'),
    [switch]$RequireComplete
)
$ErrorActionPreference = 'Stop'
$rows = @(Import-Csv -LiteralPath $LedgerPath)
if ($rows.Count -eq 0) { throw 'Donor ledger is empty' }
$seen = @{}
$manual = @{}
foreach ($evidence in (Import-Csv -LiteralPath $EvidencePath)) {
    $key = "$($evidence.origin)|$($evidence.path)"
    if ($manual.ContainsKey($key)) { throw "Duplicate manual evidence: $key" }
    $manual[$key] = $evidence
}
$resolved = @('imported', 'adapted', 'verified-replacement', 'excluded-business', 'excluded-hardware', 'excluded-tooling')
# Complete reading and a comparison (which can retain gaps) are not completed
# delivery. These states require manual hash-bound evidence but remain open.
$reviewed = @('reviewed-open', 'complete-read-awaiting-comparison', 'compared', 'reviewed-excluded')
$known = $resolved + $reviewed + @('screened', 'missing', 'legacy-unverified', 'stale-review')
foreach ($row in $rows) {
    $key = "$($row.origin)|$($row.path)"
    if ($seen.ContainsKey($key)) { throw "Duplicate ledger row: $key" }
    $seen[$key] = $true
    if (-not $row.origin -or -not $row.path -or -not $row.revision) { throw "Incomplete identity: $key" }
    if ($row.review_status -notin $known) { throw "Invalid review status: $key" }
    if ($row.availability -notin @('present','missing','pinned-blob')) { throw "Invalid availability: $key" }
    if($row.availability -eq 'pinned-blob' -and ($row.revision -notmatch '^[a-f0-9]{40}$' -or -not $row.notes -or $row.review_status -eq 'missing')){throw "Invalid pinned blob identity: $key"}
    if ($row.availability -eq 'missing') {
        if ($row.review_status -ne 'missing' -or $row.sha256) { throw "Missing source falsely reviewed: $key" }
    } elseif ($row.sha256 -notmatch '^[0-9a-f]{64}$') { throw "Invalid source hash: $key" }
    if ($row.review_status -in $resolved -and -not $row.notes) { throw "Resolved row has no decision evidence: $key" }
    if ($row.review_status -in $resolved -or $row.review_status -in $reviewed) {
        $evidence = $manual[$key]
        if ($null -eq $evidence -or $evidence.sha256 -ne $row.sha256 -or
            $evidence.review_status -ne $row.review_status -or $evidence.reuse_target -ne $row.reuse_target) {
            throw "Manual status is not bound to matching evidence: $key"
        }
    }
    if ($row.review_status -in @('imported','adapted','verified-replacement') -and -not $row.reuse_target) {
        throw "Resolved reuse has no target: $key"
    }
}
$open = @($rows | Where-Object review_status -NotIn $resolved)
$rows | Group-Object origin,review_status | Sort-Object Name | Select-Object Name,Count | Format-Table -AutoSize
Write-Host "Ledger integrity passed: $($rows.Count) inventoried; $($open.Count) unresolved. Source screening is not manual acceptance."
if ($RequireComplete -and $open.Count) { throw "Production release blocked by $($open.Count) unresolved donor rows. Master issue #2 remains open." }
