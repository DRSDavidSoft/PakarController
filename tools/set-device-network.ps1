[CmdletBinding()]
param(
    [Parameter(Mandatory)][ValidateNotNullOrEmpty()][string]$Port,
    [Parameter(Mandatory)][AllowEmptyString()][string]$Ssid,
    [Parameter(Mandatory)][Security.SecureString]$Password,
    [Parameter(Mandatory)][ValidateSet('ap','station','both')][string]$Mode,
    [ValidateRange(100,30000)][int]$TimeoutMs=5000
)
# Physical ESP USB commissioning only. Actual staging/COBS/CRC/ACK code adapted
# from set-device-name.ps1 and set-device-access.ps1, fully read before reuse.
# No reset/fuse/flash, no automatic mutation replay, no secret log or defaults.
$ErrorActionPreference='Stop'

. (Join-Path $PSScriptRoot 'serial-wire.ps1')
. (Join-Path $PSScriptRoot 'peer-identity.ps1')
function Read-PakarNetworkStatus([byte[]]$Reply,[bool]$StationEnabled,[bool]$ApEnabled) {
    if($Reply[2] -ne ([byte][PakarOpcode]::StatusResponse) -or $Reply[4] -lt 19 -or $Reply[20] -gt 1 -or $Reply[21] -gt 1) {
        throw 'ESP returned an invalid status observation after save; do not automatically repeat provisioning.'
    }
    # Payload15 is station CONNECTED; payload16 is AP started. Neither is a
    # substitute for persisted flags. New optional payload38/39/40 are config.
    if($Reply[4] -lt 41) {
        throw 'ESP acknowledged saving, but this firmware lacks configured-mode readback. Do not repeat provisioning automatically; inspect or update firmware.'
    }
    if($Reply[43] -gt 1 -or $Reply[44] -gt 1 -or $Reply[45] -gt 1 -or
        [bool]$Reply[43] -ne $StationEnabled -or [bool]$Reply[44] -ne $ApEnabled) {
        throw 'ESP acknowledged saving but configured-mode readback disagrees; stop and inspect without replaying provisioning.'
    }
    [pscustomobject]@{
        SaveAcknowledged=$true;ConfiguredModeConfirmed=$true
        StationEnabled=[bool]$Reply[43];AccessPointEnabled=[bool]$Reply[44]
        StationConnected=[bool]$Reply[20];AccessPointStarted=[bool]$Reply[21]
        DhcpEnabled=[bool]$Reply[45]
        Meaning='Persist/save ACK plus configured-mode readback; not Wi-Fi association, LAN/Web or physical acceptance.'
    }
}

$serial=$null;$secretPointer=[IntPtr]::Zero
[byte[]]$ssidBytes=@();[byte[]]$passwordBytes=@();[byte[]]$chunk=@()
try {
    $stationEnabled=$Mode -ne 'ap';$apEnabled=$Mode -ne 'station'
    if($Ssid.IndexOf([char]0) -ge 0){throw 'SSID cannot contain NUL'}
    $ssidBytes=[Text.Encoding]::UTF8.GetBytes($Ssid)
    if($ssidBytes.Length -gt 32 -or ($stationEnabled -and -not $ssidBytes.Length)){
        throw 'Enabled station requires an SSID of1..32 UTF-8 bytes'
    }
    if($Password.Length -ne 0 -and ($Password.Length -lt 8 -or $Password.Length -gt 63)){
        throw 'Wi-Fi password must be empty for an explicitly open network or8..63 printable ASCII characters'
    }
    $passwordBytes=[byte[]]::new($Password.Length)
    $secretPointer=[Runtime.InteropServices.Marshal]::SecureStringToBSTR($Password)
    for($index=0;$index -lt $Password.Length;$index++){
        $code=[Runtime.InteropServices.Marshal]::ReadInt16($secretPointer,$index*2)
        if($code -lt 32 -or $code -gt 126){throw 'Wi-Fi password must contain printable ASCII characters only'}
        $passwordBytes[$index]=[byte]$code
    }
    [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($secretPointer);$secretPointer=[IntPtr]::Zero
    $serial=[IO.Ports.SerialPort]::new($Port,115200,'None',8,'One')
    $serial.DtrEnable=$false;$serial.RtsEnable=$false;$serial.ReadTimeout=100
    $serial.Open();Start-Sleep -Milliseconds 250;$serial.DiscardInBuffer()
    [byte]$sequence=1
    $hello=Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::Hello) $sequence (New-PakarToolUsbHello) ([byte][PakarOpcode]::HelloResponse) $TimeoutMs
    Assert-PakarNetworkEspHello $hello # Premutation role/capability/read integrity gate.
    Assert-PakarToolUsbHello $hello # Mutual release policy before any staged setting.
    $sequence++
    $null=Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::NetworkConfig) $sequence ([byte[]]@(0)) ([byte][PakarOpcode]::Ack) $TimeoutMs
    foreach($entry in @(@{Kind=1;Bytes=$ssidBytes},@{Kind=2;Bytes=$passwordBytes})){
        for($offset=0;$offset -lt $entry.Bytes.Length;$offset+=47){
            $length=[Math]::Min(47,$entry.Bytes.Length-$offset)
            $chunk=[byte[]]::new($length+1);$chunk[0]=$entry.Kind
            [Array]::Copy($entry.Bytes,$offset,$chunk,1,$length)
            $sequence++
            try{$null=Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::NetworkConfig) $sequence $chunk ([byte][PakarOpcode]::Ack) $TimeoutMs}
            finally{[Array]::Clear($chunk,0,$chunk.Length)}
        }
    }
    $sequence++
    $null=Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::NetworkConfig) $sequence ([byte[]]@(3,[byte]$stationEnabled,[byte]$apEnabled)) ([byte][PakarOpcode]::Ack) $TimeoutMs
    # Commit ACK is not connected. Give the scheduled interface apply time, then
    # perform one read-only query; the script does not retry any stage/commit.
    Start-Sleep -Milliseconds 750
    $sequence++
    $status=Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::GetStatus) $sequence ([byte[]]@()) ([byte][PakarOpcode]::StatusResponse) $TimeoutMs
    Read-PakarNetworkStatus $status $stationEnabled $apEnabled
} finally {
    if($secretPointer -ne [IntPtr]::Zero){[Runtime.InteropServices.Marshal]::ZeroFreeBSTR($secretPointer)}
    [Array]::Clear($ssidBytes,0,$ssidBytes.Length)
    [Array]::Clear($passwordBytes,0,$passwordBytes.Length)
    [Array]::Clear($chunk,0,$chunk.Length)
    $entry=$null
    if($serial){if($serial.IsOpen){$serial.Close()};$serial.Dispose()}
}
