param(
    [string]$BevyHost = "localhost",
    [int]$BevyPort = 15702,
    [switch]$Foreground
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$LogDirectory = Join-Path $RepositoryRoot "target\mcp"

New-Item -ItemType Directory -Path $LogDirectory -Force | Out-Null

$env:BEVY_BRP_HOST = $BevyHost
$env:BEVY_BRP_PORT = [string]$BevyPort

$ControlCommand = Get-Command "bevy-debugger-control" -ErrorAction SilentlyContinue
$McpCommand = Get-Command "bevy-debugger-mcp" -ErrorAction SilentlyContinue

if ($ControlCommand) {
    Write-Host "Starting Bevy Debugger MCP via bevy-debugger-control."
    Write-Host "Bevy BRP endpoint: $BevyHost`:$BevyPort"
    Write-Host "Control command: $($ControlCommand.Source)"
    & $ControlCommand.Source start
    & $ControlCommand.Source status
    return
}

if ($McpCommand -and $Foreground) {
    Write-Host "Starting Bevy Debugger MCP in foreground via stdio."
    Write-Host "This mode is intended for an MCP client process; use Ctrl+C to stop it."
    Write-Host "Bevy BRP endpoint: $BevyHost`:$BevyPort"
    & $McpCommand.Source --stdio
    return
}

if ($McpCommand) {
    throw "Found bevy-debugger-mcp at '$($McpCommand.Source)', but no bevy-debugger-control helper is available. Run with -Foreground for stdio mode, or install the upstream control helper before using background lifecycle scripts."
}

throw "bevy_debugger_mcp is not installed. Install it with: cargo install bevy_debugger_mcp"
