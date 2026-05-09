param(
    [switch]$Quiet
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$PackageName = "bevy-card-game"
$ProjectPathPattern = [regex]::Escape($RepositoryRoot)
$StoppedCount = 0

$Processes = Get-CimInstance Win32_Process |
    Where-Object {
        $_.Name -eq "$PackageName.exe" -or
        (
            $_.CommandLine -and
            $_.CommandLine -match $ProjectPathPattern -and
            ($_.Name -in @("cargo.exe", "rustc.exe", "rust-lld.exe", "link.exe", "$PackageName.exe"))
        )
    }

foreach ($Process in $Processes) {
    if ($Process.ProcessId -eq $PID) {
        continue
    }

    try {
        Stop-Process -Id $Process.ProcessId -Force -ErrorAction Stop
        $StoppedCount += 1
        if (-not $Quiet) {
            Write-Host "Stopped $($Process.Name) ($($Process.ProcessId))."
        }
    } catch {
        if (-not $Quiet) {
            Write-Warning "Could not stop $($Process.Name) ($($Process.ProcessId)): $($_.Exception.Message)"
        }
    }
}

if (-not $Quiet) {
    if ($StoppedCount -eq 0) {
        Write-Host "No running project processes found."
    } else {
        Write-Host "Stopped $StoppedCount project process(es)."
    }
}
