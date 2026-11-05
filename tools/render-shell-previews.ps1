[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$PortableShellRoot)
$ErrorActionPreference='Stop'
$taskRoot=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$taskShell=[IO.Path]::GetFullPath($PortableShellRoot)
$taskOutput=Join-Path $taskRoot 'docs/assets/terminal'
$taskBuild=Join-Path $taskRoot 'build/native'
$taskFont=Join-Path $taskShell 'build/previews/dependencies/Agave.ttf'
$taskRaster=Get-ChildItem (Join-Path $taskShell 'build/previews/dependencies') -Recurse -Filter resvg.exe | Select-Object -First 1
if(-not (Test-Path $taskFont) -or -not $taskRaster){
    throw 'Prepare the pinned Agave/resvg dependencies using PortableShell: cmake -DMODE=UPDATE -P tools/rebuild-previews.cmake'
}
$taskPin=(Get-Content (Join-Path $taskRoot 'third_party/portable-shell/source.json') -Raw | ConvertFrom-Json).component_revision
$taskHead=(& git -C $taskShell rev-parse HEAD).Trim()
if($taskHead -ne $taskPin){throw "PortableShell checkout must match the firmware component pin: $taskPin"}
Push-Location $taskRoot
try {
    # Use the normal guarded build/capture wrapper. Never bypass a donor gate.
    & (Join-Path $PSScriptRoot 'test-shell.ps1') -WorkspaceOnly -CapturePrefix "$taskBuild/after-"
    $taskRenderer=Join-Path $taskBuild 'shell-preview-render.exe'
    & gcc -std=c99 -O2 -DPS_REPLAY_ONLY -DPS_RICH_REPLAY -DW=96 -DH=24 -DPS_REPLAY_SVG_CAPACITY=4000000 "-I$taskShell/include" "$taskShell/tests/ansi_render.c" -o $taskRenderer
    if($LASTEXITCODE){throw 'Terminal renderer compilation failed'}
    New-Item -ItemType Directory -Force $taskOutput | Out-Null
    foreach($taskScene in @('inside','vertical','inline','dialog','theme-menu','page-5')){
        $taskStem=Join-Path $taskOutput "after-$taskScene"
        $taskSvg=Join-Path $taskBuild "after-$taskScene.svg"
        Copy-Item "$taskBuild/after-$taskScene.ansi" "$taskStem.ansi"
        & $taskRenderer "$taskStem.ansi" $taskSvg
        if($LASTEXITCODE){throw "Terminal replay failed: $taskScene"}
        & $taskRaster.FullName --skip-system-fonts --use-font-file $taskFont $taskSvg "$taskStem.png"
        if($LASTEXITCODE){throw "PNG rendering failed: $taskScene"}
    }
} finally { Pop-Location }
Write-Output 'Refreshed actual firmware-binding replay previews; simulated observations, no board connection.'
