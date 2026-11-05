param(
    [Parameter(Mandatory)] [string] $Port,
    [Parameter(Mandatory)] [string] $Name
)
# Physical ESP USB naming; no Wi-Fi, authentication or AVR storage is erased.
$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'serial-wire.ps1')
. (Join-Path $PSScriptRoot 'peer-identity.ps1')
$label = $Name.ToLowerInvariant()
if ($label.EndsWith('.local')) { $label = $label.Substring(0, $label.Length - 6) }
if ($label -notmatch '^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$' -or $label -in @('local','localhost')) {
    throw 'Use 1..63 letters/digits/hyphens, no leading/trailing hyphen; optional .local suffix'
}
$serial = [IO.Ports.SerialPort]::new($Port, 115200, 'None', 8, 'One')
$serial.DtrEnable = $false; $serial.RtsEnable = $false; $serial.ReadTimeout = 100
$script:nameSequence = 0
function Send-NameChunk([byte[]] $Payload) {
    $script:nameSequence++
    $null = Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::DeviceName) ([byte]$script:nameSequence) $Payload ([byte][PakarOpcode]::Ack) 5000
}
try {
    $serial.Open(); Start-Sleep -Milliseconds 250; $serial.DiscardInBuffer()
    $hello=Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::Hello) 1 (New-PakarToolUsbHello) ([byte][PakarOpcode]::HelloResponse) 5000
    Assert-PakarToolUsbHello $hello
    $script:nameSequence=1
    Send-NameChunk ([byte[]] @(0))
    [byte[]] $bytes = [Text.Encoding]::ASCII.GetBytes($label)
    for ($offset = 0; $offset -lt $bytes.Length; $offset += 47) {
        $end = [Math]::Min($offset + 46, $bytes.Length - 1)
        Send-NameChunk ([byte[]] (@(1) + $bytes[$offset..$end]))
    }
    Send-NameChunk ([byte[]] @(2))
    Write-Host "Device confirmed the name is saved. Reconnect at http://$label.local/ after Wi-Fi reassociates."
} finally {
    if ($serial.IsOpen) { $serial.Close() }; $serial.Dispose()
}
