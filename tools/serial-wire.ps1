# Shared bounded USB wire transport extracted verbatim from set-device-network.ps1.
# Loading this file defines generated types/functions; never opens a port or sends frames.
# Existing PakarNetwork function names remain stable for reviewed callers/tests.
if (-not ('PakarOpcode' -as [type])) { . (Join-Path $PSScriptRoot 'ProtocolOpcodes.ps1') }
function Get-PakarNetworkCrc8([byte[]]$Bytes,[int]$Count=-1) {
    [int]$crc=0
    if($Count -lt 0){$Count=$Bytes.Length}
    for($index=0;$index -lt $Count;$index++) {
        $value=$Bytes[$index]
        $crc=$crc -bxor $value
        for($bit=0;$bit -lt 8;$bit++) {
            $crc=if($crc -band 128){(($crc -shl 1) -bxor 7) -band 255}else{($crc -shl 1) -band 255}
        }
    }
    return [byte]$crc
}
function ConvertTo-PakarNetworkCobs([byte[]]$Bytes) {
    $encoded=[Collections.Generic.List[byte]]::new();$encoded.Add(0)
    $position=0;$code=1
    foreach($value in $Bytes) {
        if($value -eq 0){$encoded[$position]=$code;$position=$encoded.Count;$encoded.Add(0);$code=1}
        else{$encoded.Add($value);$code++}
    }
    $encoded[$position]=$code;$encoded.Add(0)
    [byte[]]$result=$encoded.ToArray()
    for($index=0;$index -lt $encoded.Count;$index++){$encoded[$index]=0}
    return ,$result
}
function ConvertFrom-PakarNetworkCobs([byte[]]$Bytes) {
    if(-not $Bytes.Length -or $Bytes.Length -gt 55){return $null}
    $decoded=[Collections.Generic.List[byte]]::new();$at=0
    while($at -lt $Bytes.Length) {
        $code=[int]$Bytes[$at++]
        if($code -eq 0 -or $at+$code-1 -gt $Bytes.Length){return $null}
        for($index=1;$index -lt $code;$index++){
            if($Bytes[$at] -eq 0){return $null}
            $decoded.Add($Bytes[$at++])
        }
        if($code -ne 255 -and $at -lt $Bytes.Length){$decoded.Add(0)}
    }
    return ,$decoded.ToArray()
}
function Receive-PakarNetworkReply($Serial,[byte]$Sequence,[byte]$ResponseOpcode,
    [byte]$RequestOpcode,[int]$DeadlineMs) {
    $received=[Collections.Generic.List[byte]]::new()
    $dropping=$false;$timer=[Diagnostics.Stopwatch]::StartNew()
    while($timer.ElapsedMilliseconds -lt $DeadlineMs) {
        try{$value=$Serial.ReadByte()}catch [TimeoutException]{continue}
        if($value -lt 0){throw 'USB connection closed; outcome is uncertain. Do not automatically repeat provisioning.'}
        if($value -ne 0) {
            if($dropping){continue}
            if($received.Count -ge 55){$received.Clear();$dropping=$true;continue}
            $received.Add([byte]$value);continue
        }
        if($dropping){$dropping=$false;$received.Clear();continue}
        [byte[]]$reply=ConvertFrom-PakarNetworkCobs $received.ToArray();$received.Clear()
        if(-not $reply -or $reply.Length -lt 6 -or $reply[0] -ne ([byte][PakarEnvelope]::Magic) -or
            $reply[1] -ne ([byte][PakarEnvelope]::Reserved) -or $reply[3] -ne $Sequence -or $reply[4] -gt 48){continue}
        if($reply.Length -ne $reply[4]+6 -or
            (Get-PakarNetworkCrc8 $reply[0..($reply.Length-2)]) -ne $reply[-1]){continue}
        if($reply[2] -eq ([byte][PakarOpcode]::ErrorResponse) -and $reply[4] -eq 2 -and $reply[5] -eq $RequestOpcode) {
            throw "ESP rejected USB request (error code $($reply[6])); no automatic mutation retry was attempted."
        }
        if($reply[2] -ne $ResponseOpcode){continue}
        if($ResponseOpcode -eq ([byte][PakarOpcode]::Ack) -and
            ($reply[4] -ne 2 -or $reply[5] -ne $RequestOpcode -or $reply[6] -ne 0)){continue}
        return ,$reply
    }
    throw 'No matching USB acknowledgment before the deadline. Outcome may be uncertain; stop other serial clients and inspect the board before repeating any mutation.'
}
function Send-PakarNetworkRequest($Serial,[byte]$Opcode,[byte]$Sequence,
    [byte[]]$Payload,[byte]$ResponseOpcode,[int]$DeadlineMs) {
    if($Payload.Length -gt 48){throw 'USB payload exceeds its bounded contract'}
    [byte[]]$raw=[byte[]]::new($Payload.Length+6)
    $raw[0]=([byte][PakarEnvelope]::Magic);$raw[1]=([byte][PakarEnvelope]::Reserved);$raw[2]=$Opcode;$raw[3]=$Sequence;$raw[4]=$Payload.Length
    [Array]::Copy($Payload,0,$raw,5,$Payload.Length)
    [byte[]]$wire=@()
    try {
        $raw[$raw.Length-1]=Get-PakarNetworkCrc8 $raw ($raw.Length-1)
        $wire=ConvertTo-PakarNetworkCobs $raw
        $Serial.Write($wire,0,$wire.Length) # Exactly once: uncertainty never replays a mutation.
        return ,(Receive-PakarNetworkReply $Serial $Sequence $ResponseOpcode $Opcode $DeadlineMs)
    } finally {
        [Array]::Clear($raw,0,$raw.Length)
        [Array]::Clear($wire,0,$wire.Length)
    }
}
function Assert-PakarNetworkEspHello([byte[]]$Reply) {
    if($Reply.Length -lt 15 -or $Reply[2] -ne ([byte][PakarOpcode]::HelloResponse) -or
        $Reply[5] -ne 0 -or $Reply[6] -ne 2 -or
        (($Reply[7] -bor ([int]$Reply[8] -shl 8)) -band 0x2000) -eq 0) {
        throw 'USB peer did not acknowledge ESP role and host-serial capability; no network settings were changed.'
    }
}
