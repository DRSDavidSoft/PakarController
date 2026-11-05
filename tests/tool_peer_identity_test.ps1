$ErrorActionPreference='Stop'
. (Join-Path $PSScriptRoot '../tools/peer-identity.ps1')
. (Join-Path $PSScriptRoot '../tools/serial-wire.ps1')
function Assert($Condition,[string]$Message){if(-not $Condition){throw $Message}}
$headers=Get-PakarToolHeaders
Assert ($headers['X-Pakar-Client'] -eq 'pakarcontroller-tool') 'Tool application header absent'
Assert ($headers['X-Pakar-Version'] -eq '0.2.8') 'Installed tool version absent'
Assert ($headers['User-Agent'] -like 'PakarController/0.2.8 *') 'Actual tool user agent absent'
Assert ($headers['X-Pakar-Client-Id'] -eq (Get-PakarToolHeaders)['X-Pakar-Client-Id']) 'Process session rotated'
Assert ((Get-PakarToolSsdpAgent) -match '^PowerShell/\d+ UPnP/1.1 PakarController/0.2.8$') 'SSDP standard product agent absent'
foreach($row in @(@('0.2.7','0.2.7',0),@('0.2.8','0.2.7',1),@('0.2.7-rc.1','0.2.7',-1),@('0.2.7+local','0.2.7',0),
  @('1.0.0-alpha.9','1.0.0-alpha.10',-1),@('1.0.0-alpha','1.0.0-alpha.1',-1),@('01.2.7','0.2.7',$null),@('0.2.7-01','0.2.7',$null))){
  $actual=Compare-PakarToolVersion $row[0] $row[1]
  if($null -eq $row[2]){Assert ($null -eq $actual) 'Malformed semantic version accepted'}else{Assert ($actual -eq $row[2]) 'Semantic version order differs'}
}
Assert-PakarToolPeerCompatible @{version='0.2.8';compatibility=@{enforced=$true;minimum_app='0.2.8';maximum_app=$null}}
foreach($peer in @(@{version='0.2.7'},@{version='0.2.8-rc.1'},@{version='0.2.8';compatibility=@{enforced=$true;minimum_app='0.2.9'}},
  @{version='0.2.8';compatibility=@{enforced=$true;minimum_app='0.2.8';maximum_app='0.2.8'}})){
  $rejected=$false;try{Assert-PakarToolPeerCompatible $peer}catch{$rejected=$true};Assert $rejected 'Unsupported peer admitted'
}
Assert-PakarToolPeerCompatible @{version='9.0.0'}
Assert-PakarToolPeerCompatible @{version='0.2.9-alpha'}
Assert ($null -eq (ConvertTo-PakarToolSemver '4294967296.0.0')) 'Out-of-range uint32 core accepted'
Assert ($null -ne (ConvertTo-PakarToolSemver '4294967295.0.0')) 'Valid uint32 core rejected'
Assert ($null -eq (ConvertTo-PakarToolSemver ('0.2.8+'+('x'*58)))) 'Version longer than 63 bytes accepted'
$priorPolicyOverride=[Environment]::GetEnvironmentVariable('PAKAR_ENFORCE_PEER_VERSIONS')
try{
  foreach($value in @('0','false')){[Environment]::SetEnvironmentVariable('PAKAR_ENFORCE_PEER_VERSIONS',$value);Assert-PakarToolPeerCompatible @{version='incompatible'}}
}finally{[Environment]::SetEnvironmentVariable('PAKAR_ENFORCE_PEER_VERSIONS',$priorPolicyOverride)}
[byte[]]$hello=New-PakarToolUsbHello
Assert ([BitConverter]::ToString($hello) -eq '03-00-02-08') 'USB HELLO must carry the actual controller release'
[byte[]]$reply=0xa5,0,0x81,1,9,0,2,0,0x20,80,9,0,2,8,0
Assert-PakarToolUsbHello $reply
$reply[13]=7;$rejected=$false;try{Assert-PakarToolUsbHello $reply}catch{$rejected=$true};Assert $rejected 'USB old board release admitted'
foreach($path in @('set-device-name.ps1','set-device-access.ps1','set-device-network.ps1')){
  $source=Get-Content -LiteralPath (Join-Path $PSScriptRoot ('../tools/'+$path)) -Raw
  Assert ($source -match 'Send-PakarNetworkRequest \$serial \(\[byte\]\[PakarOpcode\]::Hello\).*New-PakarToolUsbHello') 'Provisioner lacks controller release introduction'
  $check=$source.IndexOf('Assert-PakarToolUsbHello $hello')
  $firstMutation=if($path -eq 'set-device-name.ps1'){$source.IndexOf('Send-NameChunk ([byte[]] @(0))')}elseif($path -eq 'set-device-access.ps1'){$source.IndexOf('Send-AccessChunk ([byte[]] @(0))')}else{$source.IndexOf('$null=Send-PakarNetworkRequest $serial ([byte][PakarOpcode]::NetworkConfig)')}
  Assert ($check -ge 0 -and $firstMutation -gt $check) 'Provisioner writes before reciprocal compatibility check'
}
# Execute the actual provisioning request functions against a byte queue. Never
# invoke their entrypoints or open a System.IO.Ports.SerialPort.
class PeerToolFixturePort {
  [Collections.Generic.Queue[int]]$Input=[Collections.Generic.Queue[int]]::new()
  [Collections.Generic.List[byte[]]]$Writes=[Collections.Generic.List[byte[]]]::new()
  [int]ReadByte(){if(-not $this.Input.Count){throw [TimeoutException]::new()};return $this.Input.Dequeue()}
  [void]Write([byte[]]$Bytes,[int]$Offset,[int]$Length){$copy=[byte[]]::new($Length);[Array]::Copy($Bytes,$Offset,$copy,0,$Length);$this.Writes.Add($copy)}
}
function Add-PeerToolReply($Port,[byte]$Opcode,[byte]$Sequence,[byte[]]$Payload){
  [byte[]]$raw=@(0xa5,0,$Opcode,$Sequence,$Payload.Length)+$Payload;$raw+=Get-PakarNetworkCrc8 $raw
  foreach($value in (ConvertTo-PakarNetworkCobs $raw)){$Port.Input.Enqueue($value)}
}
foreach($kind in @('name','access','network')){
  if($kind -ne 'network'){
    $tokens=$null;$errors=$null;$ast=[Management.Automation.Language.Parser]::ParseFile((Join-Path $PSScriptRoot ('../tools/set-device-'+$kind+'.ps1')),[ref]$tokens,[ref]$errors)
    Assert (-not $errors.Count) 'Provisioning syntax invalid'
    foreach($function in $ast.FindAll({param($node)$node -is [Management.Automation.Language.FunctionDefinitionAst]},$false)){. ([ScriptBlock]::Create($function.Extent.Text))}
  }
  foreach($scenario in @('supported','old-board','rejected-app')){
    $serial=[PeerToolFixturePort]::new();$opcode=if($kind -eq 'name'){0x55}elseif($kind -eq 'access'){0x54}else{0x53}
    if($scenario -eq 'rejected-app'){Add-PeerToolReply $serial 0x82 1 ([byte[]]@(0x01,0x02))}
    else{$patch=if($scenario -eq 'old-board'){7}else{8};Add-PeerToolReply $serial 0x81 1 ([byte[]]@(0,2,0,0x20,80,9,0,2,$patch))}
    Add-PeerToolReply $serial 0x80 2 ([byte[]]@($opcode,0))
    $failed=$false
    try{
      $response=Send-PakarNetworkRequest $serial 0x01 1 (New-PakarToolUsbHello) 0x81 100
      Assert-PakarToolUsbHello $response
      if($kind -eq 'name'){$script:nameSequence=1;Send-NameChunk ([byte[]]@(0))}
      elseif($kind -eq 'access'){$script:accessSequence=1;Send-AccessChunk ([byte[]]@(0))}
      else{$null=Send-PakarNetworkRequest $serial 0x53 2 ([byte[]]@(0)) 0x80 100}
    }catch{$failed=$true}
    Assert ($failed -eq ($scenario -ne 'supported')) 'USB reciprocal version decision differs'
    Assert ($serial.Writes.Count -eq $(if($scenario -eq 'supported'){2}else{1})) 'Rejected USB introduction reached settings mutation'
    $first=$serial.Writes[0];[byte[]]$decoded=ConvertFrom-PakarNetworkCobs $first[0..($first.Length-2)]
    Assert ($decoded[2] -eq 0x01 -and $decoded[3] -eq 1 -and [BitConverter]::ToString($decoded[5..8]) -eq '03-00-02-08') 'Wire introduction lost actual app release'
  }
}
'Tool HTTP/SSDP identification and strict semantic compatibility checks passed; no network or board contact.'
