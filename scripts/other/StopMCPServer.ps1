param(
    [switch]$Quiet
)

$ErrorActionPreference = "Stop"

$ControlCommand = Get-Command "bevy-debugger-control" -ErrorAction SilentlyContinue

if ($ControlCommand) {
    if (-not $Quiet) {
        Write-Host "Stopping Bevy Debugger MCP via bevy-debugger-control."
    }
    & $ControlCommand.Source stop
    if (-not $Quiet) {
        & $ControlCommand.Source status
    }
    return
}

$Processes = Get-CimInstance Win32_Process |
    Where-Object {
        $_.Name -in @("bevy-debugger-mcp.exe", "bevy-debugger-mcp") -or
        ($_.CommandLine -and $_.CommandLine -match "bevy-debugger-mcp")
    }

if (-not $Processes) {
    if (-not $Quiet) {
        Write-Host "No Bevy Debugger MCP process found."
    }
    return
}

foreach ($Process in $Processes) {
    try {
        Stop-Process -Id $Process.ProcessId -Force -ErrorAction Stop
        if (-not $Quiet) {
            Write-Host "Stopped $($Process.Name) ($($Process.ProcessId))."
        }
    } catch {
        if (-not $Quiet) {
            Write-Warning "Could not stop $($Process.Name) ($($Process.ProcessId)): $($_.Exception.Message)"
        }
    }
}
