param(
    [switch]$CheckOnly,
    [switch]$Release,
    [switch]$AiRuntime,
    [switch]$UseSccache,
    [switch]$UseFastLinker,
    [switch]$NoFastDevFeature,
    [string]$TargetTriple,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$PackageName = "samurai-card-game"
$TargetDir = Join-Path $RepositoryRoot "target\run-app-desktop"
$AiRuntimeFeature = "ai-runtime"

if (-not $env:RUST_BACKTRACE) {
    $env:RUST_BACKTRACE = "1"
}
if (-not $env:RUST_LIB_BACKTRACE) {
    $env:RUST_LIB_BACKTRACE = "1"
}

if ($CheckOnly) {
    & (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -AppOnly -Quiet
} else {
    & (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -Quiet
}

$CompileParams = @{
    Action = if ($CheckOnly) { "check" } else { "build" }
    PackageName = $PackageName
    TargetDir = $TargetDir
    WgpuBackend = "dx12"
}
if ($TargetTriple) {
    $CompileParams.TargetTriple = $TargetTriple
}
$Features = @()
if ($Release) {
    $CompileParams.Release = $true
} elseif (-not $NoFastDevFeature) {
    $Features += "fast-dev"
} else {
    $Features = @()
}
if ($AiRuntime) {
    $Features += $AiRuntimeFeature
}
if ($Features.Count -gt 0) {
    $CompileParams.Features = $Features
}
if ($UseSccache) {
    $CompileParams.UseSccache = $true
}
if ($UseFastLinker) {
    $CompileParams.UseFastLinker = $true
}

Push-Location $RepositoryRoot
try {
    & (Join-Path $PSScriptRoot "..\other\CompileApp.ps1") @CompileParams @CargoArgs

    if (-not $CheckOnly) {
        $ProfileName = if ($Release) { "release" } else { "debug" }
        $TargetRoot = if ($TargetTriple) {
            Join-Path $TargetDir $TargetTriple
        } else {
            $TargetDir
        }
        $ExecutablePath = Join-Path $TargetRoot (Join-Path $ProfileName "$PackageName.exe")
        $ProfilePath = Join-Path $TargetRoot $ProfileName
        $DependencyPath = Join-Path $TargetRoot (Join-Path $ProfileName "deps")
        $RustSysroot = (rustc --print sysroot).Trim()
        $RustBinPath = Join-Path $RustSysroot "bin"

        if (-not (Test-Path $ExecutablePath)) {
            throw "Expected executable was not found: $ExecutablePath"
        }

        if (Test-Path $RustBinPath) {
            $env:PATH = "$RustBinPath;$env:PATH"
        }
        if (Test-Path $ProfilePath) {
            $env:PATH = "$ProfilePath;$env:PATH"
        }
        if (Test-Path $DependencyPath) {
            $env:PATH = "$DependencyPath;$env:PATH"
        }

        Write-Host "Opening desktop app: $ExecutablePath"
        Write-Host "Rust backtrace: RUST_BACKTRACE=$env:RUST_BACKTRACE, RUST_LIB_BACKTRACE=$env:RUST_LIB_BACKTRACE"
        if ($AiRuntime) {
            Write-Host "AI runtime bridge: Bevy Remote Protocol at http://localhost:15702"
            Write-Host "AI runtime screenshot method: bevy_debugger/screenshot"
        }
        Start-Process -FilePath $ExecutablePath -WorkingDirectory $RepositoryRoot
    }
} finally {
    Pop-Location
    if ($CheckOnly) {
        & (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -AppOnly -Quiet
    }
}
