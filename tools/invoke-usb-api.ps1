[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$Port,
    [Parameter(Mandatory)][string]$ExpectedBoardId,
    [Parameter(Mandatory,ParameterSetName='File')][string]$RequestFile,
    [Parameter(Mandatory,ParameterSetName='Json')][string]$RequestJson,
    [ValidateRange(1000,30000)][int]$TimeoutMs=10000
)
# Repository transport/identity helpers; System.IO.Ports is the .NET serial
# implementation. No esptool, board reset, firmware upload or mutation retries.
$ErrorActionPreference='Stop'
. (Join-Path $PSScriptRoot 'serial-wire.ps1')
. (Join-Path $PSScriptRoot 'peer-identity.ps1')
$serial=$null
$script:apiSequence=0
function Send-ApiFrame([byte]$Opcode,[byte[]]$Payload,[byte]$Reply) {
    $script:apiSequence=($script:apiSequence+1) -band 255
    return ,(Send-PakarNetworkRequest $serial $Opcode ([byte]$script:apiSequence) $Payload $Reply $TimeoutMs)
}
try {
    $request=if($PSCmdlet.ParameterSetName -eq 'File'){Get-Content -LiteralPath $RequestFile -Raw | ConvertFrom-Json}else{$RequestJson | ConvertFrom-Json}
    if(-not $request.method -or -not $request.path){throw 'Request needs method and path'}
    $serial=[IO.Ports.SerialPort]::new($Port,115200,[IO.Ports.Parity]::None,8,[IO.Ports.StopBits]::One)
    $serial.DtrEnable=$false;$serial.RtsEnable=$false;$serial.ReadTimeout=100;$serial.WriteTimeout=$TimeoutMs
    $serial.Open()
    $release=Get-Content -LiteralPath (Join-Path $PSScriptRoot '../interfaces/client-release.json') -Raw | ConvertFrom-Json
    $version=([string]$release.version -split '[-+]')[0] -split '\.'
    $hello=Send-ApiFrame ([byte][PakarOpcode]::Hello) ([byte[]]@(3,[byte]$version[0],[byte]$version[1],[byte]$version[2])) ([byte][PakarOpcode]::HelloResponse)
    Assert-PakarNetworkEspHello $hello
    $identityBytes=[Collections.Generic.List[byte]]::new();$identityTotal=0;$identityRevision=$null
    do {
        $offset=$identityBytes.Count
        $frame=Send-ApiFrame ([byte][PakarOpcode]::GetIdentity) ([byte[]]@(37,($offset -band 255),($offset -shr 8))) ([byte][PakarOpcode]::IdentityResponse)
        if($frame[4] -le 9 -or $frame[5] -ne 37){throw 'Invalid identity chunk'}
        $revision=[BitConverter]::ToUInt32($frame,6);$total=[BitConverter]::ToUInt16($frame,10)
        if($total -gt 8192 -or [BitConverter]::ToUInt16($frame,12) -ne $offset){throw 'Invalid identity range'}
        if($null -eq $identityRevision){$identityRevision=$revision;$identityTotal=$total}
        if($revision -ne $identityRevision -or $total -ne $identityTotal){throw 'Identity changed; no request sent'}
        $identityBytes.AddRange([byte[]]$frame[14..($frame.Length-2)])
        if($identityBytes.Count -gt $total){throw 'Identity length exceeded'}
    } while($identityBytes.Count -lt $identityTotal)
    $identity=[Text.Encoding]::UTF8.GetString($identityBytes.ToArray()) | ConvertFrom-Json
    if($identity.board_id -cne $ExpectedBoardId -or $identity.boot_id -cnotmatch '^[0-9a-f]{32}$'){throw 'Board identity mismatch; no request sent'}
    Assert-PakarToolPeerCompatible $identity
    if('usb-api' -notin $identity.features){throw 'Board does not advertise USB API; update through an existing recovery path first'}
    $request | Add-Member -Force NoteProperty board_id $identity.board_id
    $request | Add-Member -Force NoteProperty boot_id $identity.boot_id
    [byte[]]$bytes=[Text.Encoding]::UTF8.GetBytes(($request | ConvertTo-Json -Depth 32 -Compress))
    if($bytes.Length -gt 4096){throw 'API request exceeds 4096 bytes'}
    [byte[]]$open=[byte[]]::new(19)
    for($i=0;$i -lt 16;$i++){$open[$i+1]=[Convert]::ToByte($identity.boot_id.Substring($i*2,2),16)}
    $open[17]=$bytes.Length -band 255;$open[18]=$bytes.Length -shr 8
    $reply=Send-ApiFrame ([byte][PakarOpcode]::UsbApi) $open ([byte][PakarOpcode]::UsbApiResponse)
    if($reply[4] -ne 9 -or $reply[5] -ne 0){throw 'Invalid API open response'}
    [byte[]]$token=$reply[6..9]
    for($offset=0;$offset -lt $bytes.Length;) {
        $count=[Math]::Min(41,$bytes.Length-$offset)
        [byte[]]$chunk=@(1)+$token+@(($offset -band 255),($offset -shr 8))+$bytes[$offset..($offset+$count-1)]
        $reply=Send-ApiFrame ([byte][PakarOpcode]::UsbApi) $chunk ([byte][PakarOpcode]::UsbApiResponse)
        if($reply[4] -ne 9 -or $reply[5] -ne 1 -or [BitConverter]::ToUInt32($reply,6) -ne [BitConverter]::ToUInt32($token,0)){throw 'Invalid API chunk receipt'}
        $offset+=$count
    }
    $reply=Send-ApiFrame ([byte][PakarOpcode]::UsbApi) ([byte[]](@(2)+$token)) ([byte][PakarOpcode]::UsbApiResponse)
    if($reply[4] -ne 9 -or $reply[5] -ne 2 -or [BitConverter]::ToUInt32($reply,6) -ne [BitConverter]::ToUInt32($token,0)){throw 'Invalid API execution receipt; do not replay'}
    $total=[BitConverter]::ToUInt16($reply,10)
    if(-not $total -or $total -gt 8192){throw 'Invalid response length; outcome uncertain'}
    $result=[Collections.Generic.List[byte]]::new()
    while($result.Count -lt $total) {
        $offset=$result.Count
        $reply=Send-ApiFrame ([byte][PakarOpcode]::UsbApi) ([byte[]](@(3)+$token+@(($offset -band 255),($offset -shr 8)))) ([byte][PakarOpcode]::UsbApiResponse)
        if($reply[4] -le 9 -or $reply[5] -ne 3 -or
            [BitConverter]::ToUInt32($reply,6) -ne [BitConverter]::ToUInt32($token,0) -or
            [BitConverter]::ToUInt16($reply,10) -ne $total -or [BitConverter]::ToUInt16($reply,12) -ne $offset){throw 'Invalid response chunk; do not replay'}
        $result.AddRange([byte[]]$reply[14..($reply.Length-2)])
        if($result.Count -gt $total){throw 'Response length exceeded'}
    }
    [Text.Encoding]::UTF8.GetString($result.ToArray()) | ConvertFrom-Json
    # Closing is optional cleanup, not success evidence and never a mutation retry.
    try {$null=Send-ApiFrame ([byte][PakarOpcode]::UsbApi) ([byte[]](@(4)+$token)) ([byte][PakarOpcode]::Ack)} catch {Write-Warning 'Response received; USB session will expire automatically'}
} finally {
    if($null -ne $bytes){[Array]::Clear($bytes,0,$bytes.Length)}
    if($serial){$serial.Dispose()}
}
