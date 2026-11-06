[CmdletBinding()]
param(
    [string]$PealayerRoot = 'C:\Users\David\AppData\Local\Programs\Pealayer\source\Pealayer',
    [string]$PCControllerRoot = 'C:\Users\David\AppData\Local\Programs\PCController\source\PCController',
    [string]$PuzzlesRoot = 'C:\Users\David\Desktop\Projects\ControllerBoardMini\Puzzles',
    [string]$PuzzleBoardRoot = 'C:\Users\David\Desktop\Projects\PuzzleBoard',
    [string]$ControllerBoardMiniRoot = 'C:\Users\David\Desktop\Projects\ControllerBoardMini',
    [string]$Pu850Root = (Join-Path $PSScriptRoot '..\..\repos\pu850-esp'),
    [string]$RayanLampRoot = 'C:\Users\David\Documents\Codex\2026-07-23\ba\outputs\RayanLamp',
    [string]$Esp32Ws2811Root = (Join-Path $PSScriptRoot '../../repos/esp32WS2811'),
    [string]$DrsSnakeRoot = 'I:\HTML Projects\My Scripts\Test-Cat\JSNake',
    [string]$OrbitRoot = (Join-Path $PSScriptRoot '../../repos/orbit'),
    [string]$DecisionsPath,
    [string]$EvidencePath,
    [string]$ComparisonPath=(Join-Path $PSScriptRoot '../docs/file-comparison-evidence.json'),
    [string]$OutputPath,
    [ValidateSet('Pealayer','PCController','ControllerBoardMini/Puzzles','PuzzleBoard','ControllerBoardMini','PU850-ESP','RayanLamp','esp32WS2811','portable-shell','DRS-jSnake','Orbit')]
    [string]$OnlyOrigin
)

$ErrorActionPreference = 'Stop'
if ([string]::IsNullOrWhiteSpace($DecisionsPath)) { $DecisionsPath = Join-Path $PSScriptRoot '..\docs\reuse-decisions.csv' }
if ([string]::IsNullOrWhiteSpace($OutputPath)) { $OutputPath = Join-Path $PSScriptRoot '..\docs\legacy-source-ledger.csv' }
if ([string]::IsNullOrWhiteSpace($EvidencePath)) { $EvidencePath = Join-Path $PSScriptRoot '..\docs\reuse-review-evidence.csv' }

function Get-GitFiles {
    param([string]$Root)

    $commit = (& git -C $Root rev-parse HEAD).Trim()
    if ($LASTEXITCODE -ne 0) { throw "Unable to read Git revision for $Root" }
    $files = & git -C $Root ls-files
    if ($LASTEXITCODE -ne 0) { throw "Unable to enumerate Git files for $Root" }
    return @{ Commit = $commit; Files = $files }
}

. (Join-Path $PSScriptRoot 'donor-files.ps1')
$taskPinnedSources=@{}
if(Test-Path -LiteralPath $ComparisonPath){
    foreach($taskEntry in (Get-Content -LiteralPath $ComparisonPath -Raw | ConvertFrom-Json).files){
        if($taskEntry.PSObject.Properties['source_blob']){$taskPinnedSources["$($taskEntry.origin)|$($taskEntry.path)"]=$taskEntry}
    }
}

function Get-Category {
    param([string]$Origin, [string]$RelativePath)

    $path = $RelativePath.Replace('\', '/').ToLowerInvariant()
    if ($path -match '(^|/)(test|tests|fixtures?)(/|$)' -or $path -match '(test|spec)\.[^.]+$') { return 'tests' }
    if ($path -match '(^|/)(docs?|notes?)(/|$)' -or $path -match '\.(md|txt|pdf)$') { return 'documentation' }
    if ($path -match '(^|/)(assets?|icons?|images?|themes?)(/|$)' -or $path -match '\.(png|jpe?g|svg|ico|webp|json)$') { return 'design-assets' }
    if ($path -match '(^|/)(scripts?|tools?|ci|\.github)(/|$)' -or $path -match '\.(ps1|cmd|bat|sh|ya?ml)$') { return 'tooling-ci' }
    if ($path -match '(firmware|arduino|local-lib|locallib|\.ino$|\.cpp$|\.h$)') { return 'firmware-hardware' }
    if ($path -match '(web_ui|web/src|components?|views?|styles?|palette|theme|icons?|preferences?|tray|status_bar|dialog)') { return 'ui-design' }
    if ($path -match '(api|protocol|transport|websocket|mqtt|mdns|ssdp|discovery|serial|bridge)') { return 'communications' }
    if ($path -match '(cargo|go\.mod|package(-lock)?\.json|requirements|dependencies)') { return 'dependencies' }
    return 'application-core'
}

function Add-LedgerRows {
    param(
        [System.Collections.Generic.List[object]]$Rows,
        [string]$Origin,
        [string]$Root,
        [string]$Revision,
        [string[]]$RelativePaths
    )

    foreach ($relative in $RelativePaths) {
        if ([string]::IsNullOrWhiteSpace($relative)) { continue }
        $normalized = $relative.Replace('\', '/')
        $fullPath = Join-Path $Root $relative
        if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
            $taskPinned=$taskPinnedSources["$Origin|$normalized"]
            if($taskPinned -and $Origin -eq 'PU850-ESP' -and $taskPinned.source_blob.revision -eq $Revision){
                # Compare the cached read's Git object to the CURRENT tree without
                # fetching blobs, filling the sparse checkout or accepting path-only hints.
                $taskTree=& git -C $Root ls-tree $Revision -- $normalized
                if($LASTEXITCODE -ne 0){throw 'Cannot verify pinned source tree'}
                $taskOid=[regex]::Match(($taskTree -join "`n"),'^[0-9]+ blob ([a-f0-9]{40})\t').Groups[1].Value
                $taskCandidate=[pscustomobject]@{origin=$Origin;availability='pinned-blob';revision=$Revision;sha256=$taskPinned.sha256}
                if($taskOid -eq $taskPinned.source_blob.git_sha -and (Test-PakarDonorSnapshot $taskCandidate $taskPinned)){
                    $Rows.Add([pscustomobject][ordered]@{
                        origin=$Origin;revision=$Revision;path=$normalized;sha256=$taskPinned.sha256;bytes=$taskPinned.source_blob.bytes;
                        category=(Get-Category -Origin $Origin -RelativePath $normalized);availability='pinned-blob';screening_signals='';review_status='screened';reuse_target='';
                        notes='Exact immutable GitHub blob read, raw hash and current Git tree object bound in file-comparison-evidence.json. Sparse working-tree file remains absent; not live filesystem proof.'
                    });continue
                }
            }
            $Rows.Add([pscustomobject][ordered]@{
                origin=$Origin; revision=$Revision; path=$normalized; sha256=''; bytes='';
                category=(Get-Category -Origin $Origin -RelativePath $normalized);
                availability='missing'; screening_signals=''; review_status='missing';
                reuse_target=''; notes='Tracked file is absent from this checkout (including sparse paths). It was not read or reviewed. Inspect its exact Git blob before resolving.'
            })
            continue
        }
        $item = Get-Item -LiteralPath $fullPath
        $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $fullPath).Hash.ToLowerInvariant()
        $category = Get-Category -Origin $Origin -RelativePath $normalized
        $signals = [System.Collections.Generic.List[string]]::new()
        if ($item.Length -le 2MB -and $item.Extension -notmatch '^\.(png|jpe?g|gif|webp|ico|pdf|zip|gz|woff2?|ttf|otf|exe|dll|bin|hex)$') {
            try {
                $content = [IO.File]::ReadAllText($fullPath)
                foreach ($candidate in @(
                    'phosphor', 'palette', 'theme', 'panel', 'preference', 'action', 'event',
                    'tray', 'status', 'discovery', 'mdns', 'ssdp', 'telnet', 'shell',
                    'websocket', 'mqtt', 'serial', 'protocol', 'firmware', 'updater', 'ota',
                    'tm1637', 'seven.segment', 'shift.register', 'buzzer', 'melod', 'eeprom', 'isp'
                )) {
                    if ($content -match $candidate) { $signals.Add($candidate.Replace('.', '-')) }
                }
            } catch {
                $signals.Add('unreadable-text')
            }
        }
        $Rows.Add([pscustomobject][ordered]@{
            origin = $Origin
            revision = $Revision
            path = $normalized
            sha256 = $hash
            bytes = $item.Length
            category = $category
            availability = 'present'
            screening_signals = ($signals -join ';')
            review_status = 'screened'
            reuse_target = ''
            notes = "Content-hashed file-by-file screening complete; category=$category. Manual adaptation is recorded separately when applicable."
        })
    }
}

$rows = [System.Collections.Generic.List[object]]::new()
if ($OnlyOrigin) {
    if (-not (Test-Path -LiteralPath $OutputPath)) { throw 'Selective refresh requires the existing complete ledger.' }
    foreach ($row in (Import-Csv -LiteralPath $OutputPath | Where-Object origin -ne $OnlyOrigin)) { $rows.Add($row) }
}

if (-not $OnlyOrigin -or $OnlyOrigin -eq 'DRS-jSnake') {
    $snakeFiles=Get-ChildItem -LiteralPath $DrsSnakeRoot -Recurse -File | ForEach-Object { $_.FullName.Substring($DrsSnakeRoot.Length).TrimStart('\','/').Replace('\','/') }
    Add-LedgerRows -Rows $rows -Origin 'DRS-jSnake' -Root $DrsSnakeRoot -Revision 'filesystem-snapshot' -RelativePaths $snakeFiles
}

if (-not $OnlyOrigin -or $OnlyOrigin -eq 'Orbit') {
    $orbit = Get-GitFiles -Root $OrbitRoot
    # Every firmware path stays visible, including peripheral/business files,
    # generated assets, licenses and diagnostic sketches. Classification is NOT
    # a completed comparison or an accepted exclusion.
    Add-LedgerRows -Rows $rows -Origin 'Orbit' -Root $OrbitRoot -Revision $orbit.Commit -RelativePaths @($orbit.Files | Where-Object { $_.StartsWith('firmware/') })
}

if (-not $OnlyOrigin -or $OnlyOrigin -eq 'RayanLamp') {
    Add-LedgerRows -Rows $rows -Origin 'RayanLamp' -Root $RayanLampRoot -Revision 'filesystem-snapshot-no-git' -RelativePaths (Get-RayanLampFiles $RayanLampRoot)
}

if (-not $OnlyOrigin -or $OnlyOrigin -eq 'portable-shell') {
    $portableRoot=Join-Path $PSScriptRoot '../third_party/portable-shell'
    $portableSource=Get-Content -LiteralPath (Join-Path $portableRoot 'source.json') -Raw | ConvertFrom-Json
    foreach($file in $portableSource.files){
        if($file.snapshot){
            $text=[IO.File]::ReadAllText((Join-Path $portableRoot $file.snapshot)).Replace("`r`n","`n")
            $bytes=[Text.Encoding]::UTF8.GetBytes($text)
            $digest=[Security.Cryptography.SHA256]::Create()
            try{$hash=([BitConverter]::ToString($digest.ComputeHash($bytes))).Replace('-','').ToLowerInvariant()}finally{$digest.Dispose()}
            if($hash -ne $file.sha256 -or $bytes.Length -ne $file.bytes){throw "Pinned portable-shell snapshot changed: $($file.path)"}
        } elseif($file.path -ne '_codeql_detected_source_root'){throw 'Unexpected omitted portable-shell snapshot'}
        $rows.Add([pscustomobject][ordered]@{
            origin='portable-shell';revision=$(if($file.revision){$file.revision}else{$portableSource.revision});path=$file.path;sha256=$file.sha256;bytes=$file.bytes;
            category=(Get-Category -Origin 'portable-shell' -RelativePath $file.path);availability=$(if($file.snapshot){'present'}else{'pinned-blob'});
            screening_signals='shell';review_status='screened';reuse_target='';
            notes='Pinned branch read through EOF; immutable source snapshot (CodeQL marker recorded only). Manual dispositions in file-comparison-evidence.json.'
        })
    }
}

if (-not $OnlyOrigin -or $OnlyOrigin -eq 'Pealayer') {
$pealayer = Get-GitFiles -Root $PealayerRoot
Add-LedgerRows -Rows $rows -Origin 'Pealayer' -Root $PealayerRoot -Revision $pealayer.Commit -RelativePaths $pealayer.Files
}

if (-not $OnlyOrigin -or $OnlyOrigin -eq 'PCController') {
$pccontroller = Get-GitFiles -Root $PCControllerRoot
Add-LedgerRows -Rows $rows -Origin 'PCController' -Root $PCControllerRoot -Revision $pccontroller.Commit -RelativePaths $pccontroller.Files
}

if (-not $OnlyOrigin -or $OnlyOrigin -eq 'ControllerBoardMini/Puzzles') {
$puzzleFiles = Get-ChildItem -LiteralPath $PuzzlesRoot -Recurse -File |
    Where-Object { $_.FullName -notmatch '[\\/](build|\.git|\.pio)[\\/]' } |
    ForEach-Object { $_.FullName.Substring($PuzzlesRoot.Length).TrimStart('\', '/') }
$puzzleRevision = if (Test-Path -LiteralPath (Join-Path $PuzzlesRoot '.git')) {
    (& git -C $PuzzlesRoot rev-parse HEAD).Trim()
} else {
    'filesystem-snapshot'
}
Add-LedgerRows -Rows $rows -Origin 'ControllerBoardMini/Puzzles' -Root $PuzzlesRoot -Revision $puzzleRevision -RelativePaths $puzzleFiles
}

if (-not $OnlyOrigin -or $OnlyOrigin -eq 'PU850-ESP') {
$pu850 = Get-GitFiles -Root $Pu850Root
Add-LedgerRows -Rows $rows -Origin 'PU850-ESP' -Root $Pu850Root -Revision $pu850.Commit -RelativePaths $pu850.Files
}
if (-not $OnlyOrigin -or $OnlyOrigin -eq 'esp32WS2811') {
    $ws2811 = Get-GitFiles -Root $Esp32Ws2811Root
    Add-LedgerRows -Rows $rows -Origin 'esp32WS2811' -Root $Esp32Ws2811Root -Revision $ws2811.Commit -RelativePaths $ws2811.Files
}
foreach ($source in @(
    @{Origin='PuzzleBoard'; Root=$PuzzleBoardRoot},
    @{Origin='ControllerBoardMini'; Root=$ControllerBoardMiniRoot}
)) {
    if ($OnlyOrigin -and $OnlyOrigin -ne $source.Origin) { continue }
    $sourceFiles = Get-ChildItem -LiteralPath $source.Root -Recurse -File |
        Where-Object { $_.FullName -notmatch '[\\/](build|\.build|\.cache|\.git|\.pio|node_modules|target|\.vs|bin|obj)[\\/]' } |
        ForEach-Object { $_.FullName.Substring($source.Root.Length).TrimStart('\', '/') } |
        Where-Object { $source.Origin -ne 'ControllerBoardMini' -or $_ -notmatch '^Puzzles[\\/]' }
    Add-LedgerRows -Rows $rows -Origin $source.Origin -Root $source.Root -Revision 'filesystem-snapshot' -RelativePaths $sourceFiles
}

if (Test-Path -LiteralPath $DecisionsPath) {
    $decisions = Import-Csv -LiteralPath $DecisionsPath
    $decisionIndex = @{}
    foreach ($decision in $decisions) {
        $decisionIndex["$($decision.origin)|$($decision.path)"] = $decision
    }
    foreach ($row in $rows) {
        if ($OnlyOrigin -and $row.origin -ne $OnlyOrigin) { continue }
        $decision = $decisionIndex["$($row.origin)|$($row.path)"]
        if ($null -eq $decision) { continue }
        if ($row.availability -ne 'present') { continue }
        # Historical path-only decisions remain useful routing notes, not proof
        # that this exact current source was manually read or integrated.
        $row.review_status = 'legacy-unverified'
        $row.reuse_target = $decision.reuse_target
        $row.notes = "Unbound historical decision ($($decision.review_status)); requires source-hash evidence. $($decision.notes)"
    }
}

if (Test-Path -LiteralPath $EvidencePath) {
    $evidenceIndex = @{}
    foreach ($evidence in (Import-Csv -LiteralPath $EvidencePath)) {
        $key = "$($evidence.origin)|$($evidence.path)"
        if ($evidenceIndex.ContainsKey($key)) { throw "Duplicate manual evidence: $key" }
        if ($evidence.sha256 -notmatch '^[0-9a-f]{64}$') { throw "Manual evidence needs an exact SHA256: $key" }
        $evidenceIndex[$key] = $evidence
    }
    foreach ($row in $rows) {
        if ($OnlyOrigin -and $row.origin -ne $OnlyOrigin) { continue }
        $evidence = $evidenceIndex["$($row.origin)|$($row.path)"]
        if ($null -eq $evidence -or $row.availability -notin @('present','pinned-blob')) { continue }
        if ($row.sha256 -ne $evidence.sha256) {
            $row.review_status = 'stale-review'
            $row.notes = "Source changed since manual review of $($evidence.sha256). Re-read required."
            continue
        }
        $row.review_status = $evidence.review_status
        $row.reuse_target = $evidence.reuse_target
        $row.notes = $evidence.notes
    }
}

$outputDirectory = Split-Path -Parent $OutputPath
New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null
$csv = ($rows | Sort-Object origin, path | ConvertTo-Csv -NoTypeInformation) -join "`n"
$csv += "`n"
if (-not (Test-Path -LiteralPath $OutputPath) -or
    [IO.File]::ReadAllText([IO.Path]::GetFullPath($OutputPath)).Replace("`r`n","`n") -ne $csv) {
    [IO.File]::WriteAllText([IO.Path]::GetFullPath($OutputPath), $csv, [Text.UTF8Encoding]::new($false))
}

$summary = $rows | Group-Object origin | Sort-Object Name | ForEach-Object {
    [pscustomobject]@{ Origin = $_.Name; Files = $_.Count }
}
$summary | Format-Table -AutoSize
Write-Host "Ledger: $OutputPath"
