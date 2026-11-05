param(
    [Parameter(Mandatory)] [string] $Port,
    [Parameter(Mandatory)] [ValidatePattern('^[A-Za-z0-9_.-]{1,32}$')] [string] $Username,
    [Security.SecureString] $Password
)
# Physical ESP USB recovery; does not erase filesystem, Wi-Fi or AVR backups.
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'serial-wire.ps1')
. (Join-Path $PSScriptRoot 'peer-identity.ps1')
if (-not $Password) { $Password = Read-Host 'New device password (8..64 printable ASCII characters)' -AsSecureString }
$secretPointer = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($Password)
try { $secret = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($secretPointer) }
finally { [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($secretPointer) }
if ($secret.Length -lt 8 -or $secret.Length -gt 64 -or $secret -match '[^\x20-\x7e]') {
    $secret = $null; throw 'Password must be 8..64 printable ASCII characters'
}
$serial = [IO.Ports.SerialPort]::new($Port, 115200, 'None', 8, 'One')
$serial.DtrEnable = $false; $serial.RtsEnable = $false; $serial.ReadTimeout = 100
$script:accessSequence = 0
function Send-AccessChunk([byte[]] $Payload) {
    $script:accessSequence++
    $null = Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::AccessConfig) ([byte]$script:accessSequence) $Payload ([byte][PakarOpcode]::Ack) 5000
}
try {
    $serial.Open(); Start-Sleep -Milliseconds 250; $serial.DiscardInBuffer()
    $hello=Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::Hello) 1 (New-PakarToolUsbHello) ([byte][PakarOpcode]::HelloResponse) 5000
    Assert-PakarToolUsbHello $hello
    $script:accessSequence=1
    Send-AccessChunk ([byte[]] @(0))
    foreach ($entry in @(@{Type=1;Value=$Username}, @{Type=2;Value=$secret})) {
        [byte[]] $bytes = [Text.Encoding]::ASCII.GetBytes($entry.Value)
        for ($offset = 0; $offset -lt $bytes.Length; $offset += 47) {
            $end = [Math]::Min($offset + 46, $bytes.Length - 1)
            Send-AccessChunk ([byte[]] (@($entry.Type) + $bytes[$offset..$end]))
        }
        [Array]::Clear($bytes, 0, $bytes.Length)
    }
    Send-AccessChunk ([byte[]] @(3))
    Write-Host 'Device confirmed credentials saved. Sign in with the new credentials; Wi-Fi and backups were preserved.'
} finally {
    $secret = $null
    if ($serial.IsOpen) { $serial.Close() }; $serial.Dispose()
}
