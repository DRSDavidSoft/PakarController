[CmdletBinding()]
param(
    [Parameter(Mandatory=$true)][string]$Concern,
    [string]$Origin,
    [string]$SourcePath,
    [string]$LedgerPath=(Join-Path $PSScriptRoot '../docs/legacy-source-ledger.csv'),
    [string]$EvidencePath=(Join-Path $PSScriptRoot '../docs/file-comparison-evidence.json'),
    [string]$IndexPath=(Join-Path $PSScriptRoot '../docs/reuse-capabilities.json'),
    [string]$TargetRoot=(Join-Path $PSScriptRoot '..'),
    [hashtable]$DonorRoots=@{Pealayer='C:\Users\David\AppData\Local\Programs\Pealayer\source\Pealayer'; RayanLamp='C:\Users\David\Documents\Codex\2026-07-23\ba\outputs\RayanLamp'; esp32WS2811=(Join-Path $PSScriptRoot '../../repos/esp32WS2811'); 'DRS-jSnake'='I:\HTML Projects\My Scripts\Test-Cat\JSNake'},
    [switch]$Gate,
    [switch]$Summary,
    [switch]$RequireLive
)
$ErrorActionPreference='Stop'
. (Join-Path $PSScriptRoot 'donor-files.ps1')
$taskIndex=Get-Content -LiteralPath $IndexPath -Raw | ConvertFrom-Json
if($taskIndex.schema -ne 1){throw 'Unsupported reuse index'}
$taskMatches=@($taskIndex.concerns | Where-Object id -eq $Concern)
if($taskMatches.Count -ne 1){throw "Unknown or duplicate concern '$Concern'; consultation cannot be silently skipped."}
$taskRoute=$taskMatches[0]
$taskLedger=@(Import-Csv -LiteralPath $LedgerPath)
$taskEvidence=Get-Content -LiteralPath $EvidencePath -Raw | ConvertFrom-Json
$taskSelected=@($taskLedger | Where-Object {$_.origin -in $taskRoute.origins -and $_.path -match $taskRoute.path_pattern})
if($Origin){$taskSelected=@($taskSelected | Where-Object origin -eq $Origin)}
if($SourcePath){$taskSelected=@($taskSelected | Where-Object path -eq $SourcePath)}
if(-not $taskSelected.Count){throw "No inventoried donors for $Concern"}
$taskSeen=@{}
foreach($taskRow in $taskLedger){
    $taskKey="$($taskRow.origin)|$($taskRow.path)"
    if($taskSeen.ContainsKey($taskKey)){throw "Duplicate inventoried donor: $taskKey"}
    $taskSeen[$taskKey]=$taskRow
}
$taskReviewed=@{}
foreach($taskReview in $taskEvidence.files){
    $taskKey="$($taskReview.origin)|$($taskReview.path)"
    if($taskReviewed.ContainsKey($taskKey)){throw "Duplicate comparison: $taskKey"}
    $taskReviewed[$taskKey]=$taskReview
}
$taskTargetRoot=[IO.Path]::GetFullPath($TargetRoot).TrimEnd('\','/')
# Snapshot gates also run on Linux runners without the donor's Windows drive.
# Probe the literal root before Join-Path: Join-Path resolves PS drives and
# throws on C: even though Test-Path correctly reports an unavailable root.
# Do not suppress hash/read/inventory errors for roots that actually exist.
$taskAvailableRoots=@{}
$taskSelectedOrigins=@($taskSelected.origin | Sort-Object -Unique)
foreach($taskOrigin in $taskSelectedOrigins){
    $taskAvailableRoots[$taskOrigin]=$DonorRoots.ContainsKey($taskOrigin) -and
        (Test-Path -LiteralPath $DonorRoots[$taskOrigin] -PathType Container)
}
foreach($taskOrigin in $taskSelectedOrigins){
    if($taskOrigin -eq 'RayanLamp' -and $taskAvailableRoots[$taskOrigin]){
        $taskLivePaths=@(Get-RayanLampFiles $DonorRoots[$taskOrigin])
        $taskSnapshot=@($taskLedger | Where-Object origin -eq $taskOrigin).path
        if(@(Compare-Object $taskSnapshot $taskLivePaths).Count){
            throw 'Filesystem donor inventory drift: RayanLamp; refresh source paths before consultation.'
        }
    }
    elseif($taskAvailableRoots[$taskOrigin] -and (Test-Path -LiteralPath (Join-Path $DonorRoots[$taskOrigin] '.git'))){
        $taskTracked=@(& git -C $DonorRoots[$taskOrigin] ls-files)
        if($LASTEXITCODE -ne 0){throw 'Cannot verify live donor inventory'}
        $taskSnapshot=@($taskLedger | Where-Object origin -eq $taskOrigin).path
        if(@($taskTracked | Where-Object {$_ -notin $taskSnapshot}).Count -or
            @($taskSnapshot | Where-Object {$_ -notin $taskTracked}).Count){
            throw "Tracked donor inventory drift: $taskOrigin; refresh all tracked paths before consultation."
        }
    }
}
$taskRows=@(foreach($taskRow in $taskSelected){
    $taskKey="$($taskRow.origin)|$($taskRow.path)"
    $taskReview=$taskReviewed[$taskKey]
    $taskSpecialists=@($taskIndex.concerns | Where-Object {
        $_.id -ne 'donor-audit' -and $taskRow.origin -in $_.origins -and $taskRow.path -match $_.path_pattern
    })
    $taskCurrentReview=$taskReview -and $taskReview.sha256 -eq $taskRow.sha256 -and (Test-PakarDonorSnapshot $taskRow $taskReview)
    $taskLive='unavailable'
    if($taskAvailableRoots[$taskRow.origin]){
        $taskSourceRoot=[IO.Path]::GetFullPath($DonorRoots[$taskRow.origin]).TrimEnd('\','/')
        $taskSource=[IO.Path]::GetFullPath((Join-Path $taskSourceRoot $taskRow.path))
        if(-not $taskSource.StartsWith($taskSourceRoot+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){throw 'Escaping donor path'}
        $taskLive=if(-not(Test-Path -LiteralPath $taskSource -PathType Leaf)){'missing'}
            elseif((Get-FileHash -LiteralPath $taskSource -Algorithm SHA256).Hash.ToLowerInvariant() -ne $taskRow.sha256){'changed'}else{'matches-snapshot'}
    }
    [pscustomobject]@{Key=$taskKey;SnapshotRevision=$taskRow.revision;SourceSHA256=$taskRow.sha256;
        Comparison=if($taskCurrentReview){$taskReview.verdict}elseif($taskReview){'stale-comparison'}else{'pending-comparison'};
        SourceRead=if($taskCurrentReview){$taskReview.source_read}elseif($taskReview){'stale'}else{'not-certified'};
        Availability=$taskRow.availability;
        SpecialistConcerns=@($taskSpecialists | ForEach-Object {$_.id});
        Routing=if($taskSpecialists.Count){'specialist-indexed'}else{'audit-only: specialist routing still pending'};
        TrackingIssues=@(@(3)+@($taskSpecialists | ForEach-Object {$_.issue}) | Sort-Object -Unique);
        Live=$taskLive;Targets=if($taskReview){@($taskReview.targets.path)}else{@()};
        Preserved=if($taskReview){@($taskReview.preserved)}else{@()};
        Omissions=if($taskReview){@($taskReview.omissions)}else{@()};
        Proof=if($taskReview){@($taskReview.proof)}else{@()};
        ProofMeaning='Hash-bound recorded test references, not execution/runtime acceptance; lookup does not validate targets unless Gate is requested.';
        Remaining=if($taskReview -and -not $taskCurrentReview){@('Reread changed/absent donor; recorded comparison is stale.')+@($taskReview.remaining)}
            elseif($taskReview){@($taskReview.remaining)}else{@('Read complete donor and map applicable code; no approved exclusion.')}}
})
if($RequireLive -and @($taskRows | Where-Object Live -ne 'matches-snapshot').Count){throw 'Live donor verification unavailable or stale; refresh before using it.'}
if($Gate){
    if(-not @($taskRoute.required_reviews).Count){throw "No reviewed gate configured for $Concern; lookup is routing, not acceptance."}
    foreach($taskKey in $taskRoute.required_reviews){
        $taskSource=$taskSeen[$taskKey]; $taskReview=$taskReviewed[$taskKey]
        if(-not $taskSource -or -not $taskReview -or $taskReview.source_read -ne 'complete' -or
            -not (Test-PakarDonorSnapshot $taskSource $taskReview) -or $taskSource.sha256 -ne $taskReview.sha256 -or
            $taskReview.sha256 -notmatch '^[0-9a-f]{64}$' -or -not @($taskReview.targets).Count){throw "Required donor not fully reviewed/current: $taskKey"}
        $taskLiveRow=@($taskRows | Where-Object Key -eq $taskKey)
        if($taskLiveRow.Count -ne 1 -or $taskLiveRow[0].Live -in @('missing','changed')){throw "Required donor source drift: $taskKey"}
        foreach($taskRef in @($taskReview.targets)+@($taskReview.proof)){
            if([IO.Path]::IsPathRooted($taskRef.path)){throw 'Absolute reuse target rejected'}
            $taskFull=[IO.Path]::GetFullPath((Join-Path $taskTargetRoot $taskRef.path))
            if(-not $taskFull.StartsWith($taskTargetRoot+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)){throw 'Escaping reuse target'}
            $taskText=[IO.File]::ReadAllText($taskFull).Replace("`r`n","`n")
            $taskDigest=[Security.Cryptography.SHA256]::Create()
            try{$taskHash=[BitConverter]::ToString($taskDigest.ComputeHash([Text.Encoding]::UTF8.GetBytes($taskText))).Replace('-','').ToLowerInvariant()}finally{$taskDigest.Dispose()}
            if($taskHash -ne $taskRef.sha256 -or -not $taskRef.anchor -or -not $taskText.Contains($taskRef.anchor)){throw "Reuse target/test changed; rereview: $taskKey -> $($taskRef.path)"}
        }
    }
}
$taskResult=[pscustomobject]@{Concern=$Concern;Issue=$taskRoute.issue;GateChecked=[bool]$Gate;
    Meaning='Consultation/current mapped slice only; gaps and unreviewed files are NOT accepted exclusions or full feature parity.';
    Inventoried=$taskLedger.Count;Matched=$taskRows.Count;
    Pending=@($taskRows | Where-Object Comparison -eq 'pending-comparison').Count;
    Stale=@($taskRows | Where-Object Comparison -eq 'stale-comparison').Count;
    AuditOnly=@($taskRows | Where-Object {-not $_.SpecialistConcerns.Count}).Count;
    OpenActions=@($taskRows | Where-Object {@($_.Remaining).Count -gt 0}).Count;
    Gaps=@($taskRows | Where-Object Comparison -eq 'parity-gap').Count;
    Rows=$taskRows}
if($Summary){$taskResult | Select-Object Concern,Issue,GateChecked,Meaning,Inventoried,Matched,Pending,Stale,AuditOnly,OpenActions,Gaps}
else{$taskResult}
