$ErrorActionPreference='Stop'
$taskSource=Join-Path $PSScriptRoot '../tools/set-device-network.ps1'
$taskTokens=$null;$taskErrors=$null
$taskAst=[Management.Automation.Language.Parser]::ParseFile($taskSource,[ref]$taskTokens,[ref]$taskErrors)
if($taskErrors.Count){throw 'Network commissioning script syntax invalid'}
. (Join-Path $PSScriptRoot '../tools/serial-wire.ps1')
# Extract functions ONLY: the mandatory-param entrypoint and serial side effects
# are never dot-sourced/invoked during this host contract test.
foreach($taskFunction in $taskAst.FindAll({param($node)$node -is [Management.Automation.Language.FunctionDefinitionAst]},$false)){
    . ([ScriptBlock]::Create($taskFunction.Extent.Text))
}
function Assert([bool]$Condition,[string]$Message){if(-not $Condition){throw $Message}}
function Must-Fail([scriptblock]$Action,[string]$Reason){
    $failed=$false;try{& $Action}catch{$failed=$true}
    Assert $failed $Reason
}
class NetworkFixturePort {
    [Collections.Generic.Queue[int]]$Input=[Collections.Generic.Queue[int]]::new()
    [int]$Writes=0
    [int]ReadByte(){if(-not $this.Input.Count){throw [TimeoutException]::new()};return $this.Input.Dequeue()}
    [void]Write([byte[]]$Bytes,[int]$Offset,[int]$Length){$this.Writes++}
}
function Make-Reply([byte]$Op,[byte]$Seq,[byte[]]$Payload){
    [byte[]]$raw=@(0xa5,0,$Op,$Seq,$Payload.Length)+$Payload
    $raw+=Get-PakarNetworkCrc8 $raw
    return ,(ConvertTo-PakarNetworkCobs $raw)
}
function Add-Bytes($Port,[byte[]]$Bytes){foreach($byte in $Bytes){$Port.Input.Enqueue($byte)}}
Assert ((Get-PakarNetworkCrc8 ([Text.Encoding]::ASCII.GetBytes('123456789'))) -eq 0xf4) 'CRC-8/ATM golden vector failed'
[byte[]]$fixture=0,1,0,255,2,0,3
[byte[]]$encoded=ConvertTo-PakarNetworkCobs $fixture
[byte[]]$decoded=ConvertFrom-PakarNetworkCobs $encoded[0..($encoded.Length-2)]
Assert ([BitConverter]::ToString($fixture) -eq [BitConverter]::ToString($decoded)) 'COBS roundtrip failed'
Assert ($null -eq (ConvertFrom-PakarNetworkCobs ([byte[]]@(5,1,2)))) 'Truncated COBS accepted'
$port=[NetworkFixturePort]::new()
Add-Bytes $port (Make-Reply 0x80 9 ([byte[]]@(0x53,0))) # Wrong sequence.
Add-Bytes $port (Make-Reply 0x80 7 ([byte[]]@(0x54,0))) # Wrong request opcode.
Add-Bytes $port (Make-Reply 0x80 7 ([byte[]]@(0x53,1))) # ACK error isn't success.
Add-Bytes $port (Make-Reply 0x80 7 ([byte[]]@(0x53,0)))
$reply=Receive-PakarNetworkReply $port 7 0x80 0x53 100
Assert ($reply[6] -eq 0 -and $port.Input.Count -eq 0) 'Strict matching ACK gate failed'
$port=[NetworkFixturePort]::new()
[byte[]]$overlong=[byte[]]::new(60)
for($index=0;$index -lt $overlong.Length;$index++){$overlong[$index]=1}
Add-Bytes $port $overlong
Add-Bytes $port (Make-Reply 0x80 7 ([byte[]]@(0x53,0))) # Valid-looking suffix MUST drop.
Add-Bytes $port (Make-Reply 0x80 7 ([byte[]]@(0x53,0))) # First legitimate whole frame.
$null=Receive-PakarNetworkReply $port 7 0x80 0x53 100
Assert ($port.Input.Count -eq 0) 'Overlength frame suffix was incorrectly parsed'
$port=[NetworkFixturePort]::new()
[byte[]]$badRaw=0xa5,1,0x80,7,2,0x53,0 # Nonzero reserved byte, valid CRC.
$badRaw+=Get-PakarNetworkCrc8 $badRaw;Add-Bytes $port (ConvertTo-PakarNetworkCobs $badRaw)
[byte[]]$badCrc=Make-Reply 0x80 7 ([byte[]]@(0x53,0));$badCrc[$badCrc.Length-2]=$badCrc[$badCrc.Length-2] -bxor 0x08
Add-Bytes $port $badCrc
Add-Bytes $port (Make-Reply 0x80 7 ([byte[]]@(0x53,0)))
$null=Receive-PakarNetworkReply $port 7 0x80 0x53 100
Assert ($port.Input.Count -eq 0) 'Reserved/CRC malformed replies were accepted'
$port=[NetworkFixturePort]::new()
Add-Bytes $port (Make-Reply 0x82 7 ([byte[]]@(0x53,5)))
Must-Fail {Receive-PakarNetworkReply $port 7 0x80 0x53 100} 'Explicit ESP error accepted'
$port=[NetworkFixturePort]::new()
Must-Fail {Send-PakarNetworkRequest $port 0x53 7 ([byte[]]@(3,1,1)) 0x80 10} 'Timeout accepted'
Assert ($port.Writes -eq 1) 'An uncertain mutation was retried'
[byte[]]$hello=0xa5,0,0x81,1,9,0,2,0,0x20,80,9,0,2,7,0
Assert-PakarNetworkEspHello $hello
$hello[6]=1;Must-Fail {Assert-PakarNetworkEspHello $hello} 'AVR accepted as commissioning ESP'
$hello[6]=2;$hello[8]=0;Must-Fail {Assert-PakarNetworkEspHello $hello} 'Missing host-serial capability accepted'
$port=[NetworkFixturePort]::new()
Add-Bytes $port (Make-Reply 0x81 1 ([byte[]]@(0,2,0,0x20,80,9,0,2,7)))
$hello=Send-PakarNetworkRequest $port 0x01 1 ([byte[]]@()) 0x81 100
Assert-PakarNetworkEspHello $hello
Assert ($port.Writes -eq 1) 'Empty HELLO payload send failed'
[byte[]]$status=[byte[]]::new(47);$status[2]=0x90;$status[4]=41
$status[20]=0;$status[21]=1;$status[43]=1;$status[44]=1;$status[45]=1
$state=Read-PakarNetworkStatus $status $true $true
Assert ($state.SaveAcknowledged -and $state.StationEnabled -and -not $state.StationConnected) 'Configuration was confused with association'
Must-Fail {Read-PakarNetworkStatus $status $true $false} 'Mismatched configured mode accepted'
$status[4]=38;Must-Fail {Read-PakarNetworkStatus $status $true $true} 'Old firmware missing config readback accepted'
$taskText=Get-Content -LiteralPath $taskSource -Raw
Assert ($taskText -match 'SecureStringToBSTR' -and $taskText -match 'ZeroFreeBSTR' -and $taskText -notmatch 'PtrToStringBSTR') 'Secret copied into an immutable managed string'
Assert ($taskText -match 'DtrEnable=\$false;\$serial.RtsEnable=\$false') 'DTR/RTS reset safety missing'
Assert ($taskText -match '\[Array\]::Clear\(\$passwordBytes') 'Secret buffer clearing missing'
$taskMutations=@($taskAst.FindAll({param($node)
    $node -is [Management.Automation.Language.CommandAst] -and
    $node.GetCommandName() -eq 'Send-PakarNetworkRequest' -and
    $node.Extent.Text.Contains('[PakarOpcode]::NetworkConfig')
},$true))
Assert ($taskMutations.Count -gt 0) 'Network mutation AST was not found'
foreach($taskGate in @('Assert-PakarNetworkEspHello','Assert-PakarToolUsbHello')){
    $taskChecks=@($taskAst.FindAll({param($node)
        $node -is [Management.Automation.Language.CommandAst] -and $node.GetCommandName() -eq $taskGate
    },$true))
    Assert ($taskChecks.Count -eq 1) 'Missing or ambiguous commissioning peer gate'
    foreach($taskMutation in $taskMutations){
        Assert ($taskChecks[0].Extent.EndOffset -lt $taskMutation.Extent.StartOffset) 'Mutation preceded peer verification'
    }
}
Write-Output 'USB network commissioning host contracts pass: COBS/CRC, role, strict ACK, overlength drop, uncertainty no-replay, mode readback and buffer hygiene; no serial device was opened.'
