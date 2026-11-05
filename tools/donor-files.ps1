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
    $paths = [Collections.Generic.List[string]]::new()
    # Retain absent tracked files AND unpacked source (notably vendor code).
    # Gaining a Git repository must not silently drop untracked source files.
    if (Test-Path -LiteralPath (Join-Path $Root '.git')) {
        $tracked = & git -C $Root ls-files
        if ($LASTEXITCODE) { throw 'Cannot enumerate tracked RayanLamp source' }
        foreach ($path in $tracked) { $paths.Add($path) }
    }
    # Prune generated trees before traversal; retain vendor source and licenses.
    $pending = [Collections.Generic.Stack[string]]::new()
    $pending.Push($Root)
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
