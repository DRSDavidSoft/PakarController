# Immutable blob evidence is distinct from a live filesystem donor.
function Test-PakarDonorSnapshot {
    param($Row,$Review)
    if($Row.availability -eq 'present'){return $true}
    if($Row.availability -ne 'pinned-blob' -or -not $Review){return $false}
    $blob=$Review.PSObject.Properties['source_blob']
    $repositories=@{'PU850-ESP'='atomicdeploy/pu850-esp';'portable-shell'='atomicdeploy/portable-shell'}
    return $repositories.ContainsKey($Row.origin) -and $blob -and
        $blob.Value.repository -eq $repositories[$Row.origin] -and
        $Row.revision -match '^[a-f0-9]{40}$' -and $blob.Value.revision -eq $Row.revision -and
        $blob.Value.git_sha -match '^[a-f0-9]{40}$' -and $blob.Value.sha256 -eq $Row.sha256
}
function Get-RayanLampFiles {
    param([string]$Root)
    $Root = [IO.Path]::GetFullPath($Root).TrimEnd('\','/')
    # Retain absent tracked files AND unpacked source (notably vendor code).
    # Git ignore rules exclude local build products from untracked inventory;
    # tracked files always remain visible, even when ignored or missing.
    if (Test-Path -LiteralPath (Join-Path $Root '.git')) {
        $tracked = & git -C $Root ls-files
        if ($LASTEXITCODE) { throw 'Cannot enumerate tracked RayanLamp source' }
        $untracked = & git -C $Root ls-files --others --exclude-standard
        if ($LASTEXITCODE) { throw 'Cannot enumerate unpacked RayanLamp source' }
        $untracked = @($untracked | Where-Object { $_ -notmatch '(^|/)(node_modules|\.cache|dist|dist-sdk|work|coverage)/' })
        return @($tracked) + $untracked | Sort-Object -Unique
    }
    # Prune generated trees before traversal; retain vendor source and licenses.
    $pending = [Collections.Generic.Stack[string]]::new()
    $pending.Push($Root)
    $paths = [Collections.Generic.List[string]]::new()
    while ($pending.Count) {
        foreach ($item in (Get-ChildItem -LiteralPath $pending.Pop() -Force)) {
            $relative = $item.FullName.Substring($Root.Length).TrimStart('\','/').Replace('\','/')
            if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint) { throw "Donor symlink requires explicit inventory: $relative" }
            if ($item.PSIsContainer) {
                if ($item.Name -in @('.git','node_modules','.cache','dist','dist-sdk','work','coverage')) { continue }
                $pending.Push($item.FullName)
            } else { $paths.Add($relative) }
        }
    }
    return $paths | Sort-Object -Unique
}
