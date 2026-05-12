param(
    [switch]$EnableFastDevFeature,
    [string]$DioxusCliVersion = "0.7.9",
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$DxArgs
)

$ErrorActionPreference = "Stop"

$ScriptPath = Join-Path $PSScriptRoot "..\main\RunAppDesktopHotReload.ps1"
$ScriptArgs = @("-AiRuntime")

if ($EnableFastDevFeature) {
    $ScriptArgs += "-EnableFastDevFeature"
}

if ($DioxusCliVersion) {
    $ScriptArgs += @("-DioxusCliVersion", $DioxusCliVersion)
}

if ($DxArgs) {
    $ScriptArgs += $DxArgs
}

& $ScriptPath @ScriptArgs
