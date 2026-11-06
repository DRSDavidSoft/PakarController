[CmdletBinding()]param([switch]$Plan)
$ErrorActionPreference='Stop'
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern orbit-firmware | Out-Null
& (Join-Path $PSScriptRoot 'consult-reuse.ps1') -Concern build-storage -Gate | Out-Null
$taskRoot=Split-Path -Parent $PSScriptRoot
$taskOutput=Join-Path $taskRoot 'build/native/runtime_cpu_test.exe'
$taskArguments=@('-std=c++17','-O2','-g0','-Wall','-Wextra','-Werror',(Join-Path $taskRoot 'tests/runtime_cpu_test.cpp'),'-o',$taskOutput)
if($Plan){return [pscustomobject]@{Executable='g++';Arguments=$taskArguments;Scope='CPU runtime window arithmetic; not ESP hardware sampling'}}
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $taskOutput) | Out-Null
& g++ @taskArguments
if($LASTEXITCODE){throw 'CPU policy compilation failed'}
& $taskOutput
if($LASTEXITCODE){throw 'CPU policy assertions failed'}
