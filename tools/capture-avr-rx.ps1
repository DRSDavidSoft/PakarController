[CmdletBinding()]
param(
    [Parameter(Mandatory)][ValidateNotNullOrEmpty()][string]$Port,
    [Parameter(Mandatory)][ValidateSet(38400,115200)][int]$Baud,
    [ValidateRange(100,7500)][int]$WindowMs=3000,
    [ValidateRange(100,30000)][int]$TimeoutMs=5000
)
# Quiet ESP USB firmware only: one explicitly chosen RX-only capture. This sends
# no AVR frames/tones, performs no reset/flash/fuse operation and never sweeps
# baud rates, retries a start, or resumes the ordinary AVR link automatically.
$ErrorActionPreference='Stop'
. (Join-Path $PSScriptRoot 'serial-wire.ps1')

function Read-PakarAvrRxUint32([byte[]]$Bytes,[int]$Offset) {
    [uint64]$value=0
    for($index=0;$index -lt 4;$index++){$value=$value -bor ([uint64]$Bytes[$Offset+$index] -shl ($index*8))}
    return [uint32]$value
}
function Assert-PakarAvrRxPrefix([byte[]]$Reply,[byte]$Action) {
    if($Reply.Length -lt 8 -or $Reply[0] -ne ([byte][PakarEnvelope]::Magic) -or $Reply[1] -ne ([byte][PakarEnvelope]::Reserved) -or
        $Reply[2] -ne ([byte][PakarOpcode]::AvrRxDiagnosticResponse) -or $Reply[4] -gt 48 -or $Reply.Length -ne $Reply[4]+6 -or
        $Reply[5] -ne $Action){throw 'Invalid or mismatched passive-capture response; no diagnostic success is assumed.'}
    if($Reply[6] -ne 0){throw "ESP rejected passive-capture action (error code $($Reply[6])); no automatic start retry was attempted."}
}
function Read-PakarAvrRxSnapshot([byte[]]$Reply,[byte]$Action) {
    Assert-PakarAvrRxPrefix $Reply $Action
    if($Action -gt 2 -or $Reply[4] -ne 27 -or $Reply[7] -gt 1 -or $Reply[9] -gt 1 -or
        $Reply[12] -gt 128 -or $Reply[29] -gt 1 -or $Reply[30] -gt 1 -or
        $Reply[31] -notin @(0,1,255) -or $Reply[8] -notin @(0,1,255) -or
        ($Reply[7] -eq 1 -and $Reply[8] -eq 255)){
        throw 'Passive-capture snapshot has invalid length, bounds or GPIO observations.'
    }
    $received=Read-PakarAvrRxUint32 $Reply 13
    $dropped=Read-PakarAvrRxUint32 $Reply 17
    if($received -lt $Reply[12] -or $dropped -gt $received){throw 'Passive-capture byte counts are inconsistent.'}
    [pscustomobject]@{
        Active=[bool]$Reply[7]
        Baud=$(if($Reply[8] -eq 0){38400}elseif($Reply[8] -eq 1){115200}else{$null})
        PinsSafe=[bool]$Reply[9];CaptureError=[int]$Reply[10];StopReason=[int]$Reply[11]
        Stored=[int]$Reply[12];Received=$received;Dropped=$dropped
        HardwareErrors=(Read-PakarAvrRxUint32 $Reply 21)
        ElapsedMs=(Read-PakarAvrRxUint32 $Reply 25)
        Rx18Level=[int]$Reply[29];Reset10Level=[int]$Reply[30]
        Reset10OutputKnown=($Reply[31] -ne 255)
        Reset10OutputEnabled=$(if($Reply[31] -eq 255){$null}else{[bool]$Reply[31]})
    }
}
function Read-PakarAvrRxChunk([byte[]]$Reply,[byte]$Offset,[int]$Remaining) {
    Assert-PakarAvrRxPrefix $Reply 3
    if($Reply[4] -lt 4 -or $Reply[7] -ne $Offset -or $Reply[8] -gt 32 -or
        $Reply[8] -ne [Math]::Min(32,$Remaining) -or $Reply[4] -ne 4+$Reply[8]){
        throw 'Passive-capture chunk has mismatched offset, count or exact length.'
    }
    [byte[]]$bytes=[byte[]]::new($Reply[8])
    [Array]::Copy($Reply,9,$bytes,0,$bytes.Length)
    return ,$bytes
}
function Assert-PakarAvrRxPaused([byte[]]$Reply) {
    if($Reply.Length -lt 48 -or $Reply[2] -ne ([byte][PakarOpcode]::StatusResponse) -or $Reply[4] -lt 42 -or
        $Reply.Length -ne $Reply[4]+6 -or $Reply[46] -ne 0){
        throw 'Normal AVR link is not explicitly acknowledged paused; passive capture was not started. Use the reviewed quiet commissioning firmware.'
    }
}
function Invoke-PakarAvrRxCapture($Serial,[ValidateSet(38400,115200)][int]$CaptureBaud,
    [ValidateRange(100,7500)][int]$CaptureWindowMs,[int]$DeadlineMs,
    [scriptblock]$Wait={param($Milliseconds)Start-Sleep -Milliseconds $Milliseconds}) {
    [byte]$sequence=1
    $hello=Send-PakarNetworkRequest $Serial ([byte][PakarOpcode]::Hello) $sequence ([byte[]]@()) ([byte][PakarOpcode]::HelloResponse) $DeadlineMs
    Assert-PakarNetworkEspHello $hello
    $sequence++
    $status=Send-PakarNetworkRequest $Serial ([byte][PakarOpcode]::GetStatus) $sequence ([byte[]]@()) ([byte][PakarOpcode]::StatusResponse) $DeadlineMs
    Assert-PakarAvrRxPaused $status
    $sequence++
    $reply=Send-PakarNetworkRequest $Serial ([byte][PakarOpcode]::AvrRxDiagnostic) $sequence ([byte[]]@(0)) ([byte][PakarOpcode]::AvrRxDiagnosticResponse) $DeadlineMs
    $before=Read-PakarAvrRxSnapshot $reply 0
    if($before.Active -or -not $before.PinsSafe -or -not $before.Reset10OutputKnown -or
        $before.Reset10OutputEnabled){throw 'Existing capture or unsafe/unknown pad ownership; no passive capture was started.'}
    $startAttempted=$false;$stopAttempted=$false;$stop=$null
    try {
        $sequence++;$startAttempted=$true
        [byte]$baudCode=if($CaptureBaud -eq 38400){0}else{1}
        $reply=Send-PakarNetworkRequest $Serial ([byte][PakarOpcode]::AvrRxDiagnostic) $sequence ([byte[]]@(1,$baudCode)) ([byte][PakarOpcode]::AvrRxDiagnosticResponse) $DeadlineMs
        $start=Read-PakarAvrRxSnapshot $reply 1
        if(-not $start.Active -or -not $start.PinsSafe -or $start.Baud -ne $CaptureBaud -or
            -not $start.Reset10OutputKnown -or $start.Reset10OutputEnabled){
            throw 'ESP did not confirm the selected RX-only capture and safe pad ownership; no start retry was attempted.'
        }
        & $Wait $CaptureWindowMs
        $sequence++
        $reply=Send-PakarNetworkRequest $Serial ([byte][PakarOpcode]::AvrRxDiagnostic) $sequence ([byte[]]@(0)) ([byte][PakarOpcode]::AvrRxDiagnosticResponse) $DeadlineMs
        $observed=Read-PakarAvrRxSnapshot $reply 0
        $sequence++;$stopAttempted=$true
        $reply=Send-PakarNetworkRequest $Serial ([byte][PakarOpcode]::AvrRxDiagnostic) $sequence ([byte[]]@(2)) ([byte][PakarOpcode]::AvrRxDiagnosticResponse) $DeadlineMs
        $stop=Read-PakarAvrRxSnapshot $reply 2
        if($stop.Active){throw 'Passive capture stop was not confirmed; do not retry blindly. Firmware has an eight-second auto-stop.'}
        [byte[]]$captured=[byte[]]::new($stop.Stored)
        for($offset=0;$offset -lt $captured.Length;$offset+=32){
            $sequence++
            $reply=Send-PakarNetworkRequest $Serial ([byte][PakarOpcode]::AvrRxDiagnostic) $sequence ([byte[]]@(3,[byte]$offset)) ([byte][PakarOpcode]::AvrRxDiagnosticResponse) $DeadlineMs
            [byte[]]$chunk=Read-PakarAvrRxChunk $reply ([byte]$offset) ($captured.Length-$offset)
            [Array]::Copy($chunk,0,$captured,$offset,$chunk.Length)
        }
        [pscustomobject]@{
            Baud=$CaptureBaud;RequestedWindowMs=$CaptureWindowMs
            Capture=$stop;ObservedBeforeStop=$observed
            Hex=[BitConverter]::ToString($captured).Replace('-','');Base64=[Convert]::ToBase64String($captured)
            Meaning=$(if($stop.Received -eq 0){'No bytes observed in this bounded window at the selected baud. This does not prove correct or incorrect wiring, AVR power, reset, baud or firmware.'}
                else{'Raw RX bytes observed; not proof of valid frames, correct baud, electrical safety or successful application communication.'})
            NormalAvrLinkResumed=$false
        }
    } finally {
        if($startAttempted -and -not $stopAttempted){
            # One cleanup stop, including uncertain start ACK. The USB port is
            # exclusively owned by this call. Never replay start or retry stop.
            $stopAttempted=$true;$sequence++
            try {
                $reply=Send-PakarNetworkRequest $Serial ([byte][PakarOpcode]::AvrRxDiagnostic) $sequence ([byte[]]@(2)) ([byte][PakarOpcode]::AvrRxDiagnosticResponse) $DeadlineMs
                $cleanup=Read-PakarAvrRxSnapshot $reply 2
                if($cleanup.Active){Write-Warning 'Capture stop not confirmed; firmware auto-stop is eight seconds. No stop retry was attempted.'}
            } catch {Write-Warning 'Capture cleanup stop outcome is uncertain; firmware auto-stop is eight seconds. No stop retry was attempted.'}
        }
    }
}

$serial=$null
try {
    $serial=[IO.Ports.SerialPort]::new($Port,115200,'None',8,'One')
    $serial.DtrEnable=$false;$serial.RtsEnable=$false;$serial.ReadTimeout=100
    $serial.Open();Start-Sleep -Milliseconds 250;$serial.DiscardInBuffer()
    Invoke-PakarAvrRxCapture $serial $Baud $WindowMs $TimeoutMs
} finally {
    if($serial){if($serial.IsOpen){$serial.Close()};$serial.Dispose()}
}
