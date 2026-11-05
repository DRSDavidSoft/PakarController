$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$scratch = Join-Path ([IO.Path]::GetTempPath()) ('PakarController-ledger-test-' + [guid]::NewGuid())
$donor = Join-Path $scratch 'donor'
New-Item -ItemType Directory -Path $donor -Force | Out-Null
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'reuse_fixture/source.h') -Destination $donor
& git -C $donor init --quiet
& git -C $donor add source.h
& git -C $donor -c user.name=LedgerFixture -c user.email=fixture@example.invalid commit --quiet -m fixture
if ($LASTEXITCODE -ne 0) { throw 'Fixture Git creation failed' }
$hash = (Get-FileHash -LiteralPath (Join-Path $donor 'source.h')).Hash.ToLowerInvariant()
$decisions = Join-Path $scratch 'decisions.csv'
$evidence = Join-Path $scratch 'evidence.csv'
$ledger = Join-Path $scratch 'ledger.csv'
@([pscustomobject]@{origin='Pealayer';path='source.h';review_status='adapted';reuse_target='test-target';notes='Fixture legacy decision'}) |
    Export-Csv -NoTypeInformation -LiteralPath $decisions
@([pscustomobject]@{origin='PCController';path='source.h';sha256=$hash;review_status='adapted';reuse_target='test-target';notes='Fixture manual evidence'}) |
    Export-Csv -NoTypeInformation -LiteralPath $evidence
$options = @{
    PealayerRoot=$donor; PCControllerRoot=$donor; PuzzlesRoot=$donor;
    PuzzleBoardRoot=$donor; ControllerBoardMiniRoot=$donor; Pu850Root=$donor; RayanLampRoot=$donor; Esp32Ws2811Root=$donor; DrsSnakeRoot=$donor;
    DecisionsPath=$decisions; EvidencePath=$evidence; OutputPath=$ledger
}
function Assert-State([string]$origin, [string]$status) {
    $row = Import-Csv -LiteralPath $ledger | Where-Object { $_.origin -eq $origin -and $_.path -eq 'source.h' }
    if ($row.review_status -ne $status) { throw "Expected $origin $status; got $($row.review_status)" }
}
try {
    & (Join-Path $root 'tools/generate-reuse-ledger.ps1') @options | Out-Null
    Assert-State Pealayer legacy-unverified
    Assert-State PCController adapted
    Assert-State RayanLamp screened
    Assert-State esp32WS2811 screened
    $marker = Import-Csv -LiteralPath $ledger | Where-Object { $_.origin -eq 'portable-shell' -and $_.path -eq '_codeql_detected_source_root' }
    if ($marker.review_status -ne 'screened') { throw 'Generated marker must not invent a manual review without evidence' }
    . (Join-Path $root 'tools/donor-files.ps1')
    foreach ($relative in @('node_modules','dist','vendor')) {
        New-Item -ItemType Directory -Path (Join-Path $donor $relative) -Force | Out-Null
        Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'reuse_fixture/source.h') -Destination (Join-Path $donor "$relative/source.h")
    }
    [IO.File]::WriteAllText((Join-Path $donor '.gitignore'), "*.test.exe`n")
    [IO.File]::WriteAllText((Join-Path $donor 'generated.test.exe'), 'generated fixture')
    $sourcePaths = @(Get-RayanLampFiles ($donor + [IO.Path]::DirectorySeparatorChar))
    if ('vendor/source.h' -notin $sourcePaths -or 'dist/source.h' -in $sourcePaths -or 'node_modules/source.h' -in $sourcePaths -or 'generated.test.exe' -in $sourcePaths) {
        throw 'Source snapshot must retain vendor and prune generated output before recursion'
    }
    & (Join-Path $root 'tools/generate-reuse-ledger.ps1') @options | Out-Null
    $consultIndex=Join-Path $scratch 'consult-index.json'
    $consultEvidence=Join-Path $scratch 'consult-evidence.json'
    [IO.File]::WriteAllText($consultIndex,'{"schema":1,"concerns":[{"id":"fixture","origins":["RayanLamp"],"path_pattern":".*","required_reviews":[],"issue":2}]}')
    [IO.File]::WriteAllText($consultEvidence,'{"schema":1,"files":[]}')
    $lookup=& (Join-Path $root 'tools/consult-reuse.ps1') -Concern fixture -LedgerPath $ledger -EvidencePath $consultEvidence -IndexPath $consultIndex -DonorRoots @{RayanLamp=$donor}
    if($lookup.Matched -ne $sourcePaths.Count){throw 'Consultation must use the same tracked/untracked source inventory as generation'}
    $initialRows = @(Import-Csv -LiteralPath $ledger)
    if (@($initialRows | Where-Object {$_.origin -eq 'RayanLamp' -and $_.path -match '^\.git/'}).Count) { throw 'Generated Git internals were inventoried as donor source' }
    & (Join-Path $root 'tools/check-reuse-ledger.ps1') -LedgerPath $ledger -EvidencePath $evidence | Out-Null
    $rejected = $false
    try { & (Join-Path $root 'tools/check-reuse-ledger.ps1') -LedgerPath $ledger -EvidencePath $evidence -RequireComplete | Out-Null }
    catch { $rejected = $_.Exception.Message -match 'Production release blocked' }
    if (-not $rejected) { throw 'Unresolved ledger wrongly passed release gate' }
    Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'reuse_fixture/changed.h') -Destination (Join-Path $donor 'source.h') -Force
    & (Join-Path $root 'tools/generate-reuse-ledger.ps1') @options | Out-Null
    Assert-State PCController stale-review
    # Simulate a sparse/missing tracked file without deleting its contents.
    Move-Item -LiteralPath (Join-Path $donor 'source.h') -Destination (Join-Path $scratch 'saved-source.h')
    & (Join-Path $root 'tools/generate-reuse-ledger.ps1') @options | Out-Null
    Assert-State PCController missing
    Assert-State RayanLamp missing
    # Exact reviewed sparse blobs survive regeneration only while their current
    # tree object/revision/raw identity agree. No checkout expansion or network.
    $taskRevision=(& git -C $donor rev-parse HEAD).Trim()
    $taskObject=(& git -C $donor rev-parse HEAD:source.h).Trim()
    $taskComparison=Join-Path $scratch 'comparison.json'
    $taskBlob=[ordered]@{repository='atomicdeploy/pu850-esp';revision=$taskRevision;git_sha=$taskObject;sha256=$hash;bytes=(Get-Item -LiteralPath (Join-Path $PSScriptRoot 'reuse_fixture/source.h')).Length}
    $taskCompared=@{files=@(@{origin='PU850-ESP';path='source.h';sha256=$hash;source_blob=$taskBlob})}
    function Write-BlobFixture { [IO.File]::WriteAllText($taskComparison,($taskCompared | ConvertTo-Json -Depth 10)) }
    Write-BlobFixture;$options.ComparisonPath=$taskComparison
    & (Join-Path $root 'tools/generate-reuse-ledger.ps1') @options | Out-Null
    $taskPinned=Import-Csv -LiteralPath $ledger | Where-Object { $_.origin -eq 'PU850-ESP' -and $_.path -eq 'source.h' }
    if($taskPinned.availability -ne 'pinned-blob' -or $taskPinned.sha256 -ne $hash){throw 'Reviewed immutable sparse blob lost on regeneration'}
    $taskBlob.git_sha='0'*40;Write-BlobFixture
    & (Join-Path $root 'tools/generate-reuse-ledger.ps1') @options | Out-Null
    Assert-State PU850-ESP missing
    $taskBlob.git_sha=$taskObject;$taskBlob.repository='unrelated/repository';Write-BlobFixture
    & (Join-Path $root 'tools/generate-reuse-ledger.ps1') @options | Out-Null
    Assert-State PU850-ESP missing
    Write-Host 'Ledger tests passed: legacy hints are not reviews; hashes invalidate stale evidence; missing tracked files remain inventoried; exact reviewed sparse blobs require current tree/repository identity; strict release gate rejects unresolved scope.'
} finally {
    # This tiny evidence fixture is intentionally retained for debugging.
    Write-Host "Fixture evidence: $scratch"
}
