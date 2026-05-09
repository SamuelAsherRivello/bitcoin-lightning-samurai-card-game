param(
    [switch]$Release,
    [switch]$UseSccache,
    [switch]$NoFastLinker,
    [switch]$NoFastDevFeature,
    [string]$TargetTriple,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$PackageName = "bevy-card-game"

& (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -Quiet

$env:CARGO_TARGET_DIR = Join-Path $RepositoryRoot "target"
$env:WGPU_BACKEND = "dx12"

if ($UseSccache) {
    $Sccache = Get-Command "sccache" -ErrorAction SilentlyContinue
    if ($Sccache) {
        $env:CARGO_INCREMENTAL = "0"
        $env:RUSTC_WRAPPER = $Sccache.Source
        $env:SCCACHE_DIR = Join-Path $env:CARGO_TARGET_DIR "sccache"
        Write-Host "Using sccache: $($Sccache.Source)"
    } else {
        throw "sccache was requested but was not found on PATH."
    }
} else {
    $env:CARGO_INCREMENTAL = "1"
    if ($env:RUSTC_WRAPPER -like "*sccache*") {
        Remove-Item Env:\RUSTC_WRAPPER
    }
    Write-Host "Using Cargo incremental compilation. Pass -UseSccache to use sccache instead."
}

if (-not $NoFastLinker) {
    $RustLld = Get-Command "rust-lld" -ErrorAction SilentlyContinue
    if ($RustLld) {
        $env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER = $RustLld.Source
        Write-Host "Using fast linker: $($RustLld.Source)"
    } else {
        Write-Host "rust-lld not found; using the default Windows linker."
    }
}

$CargoCommand = @(
    "run",
    "--package", $PackageName
)

if ($TargetTriple) {
    $CargoCommand += @("--target", $TargetTriple)
}

if ($Release) {
    $CargoCommand += "--release"
} elseif (-not $NoFastDevFeature) {
    $CargoCommand += @("--features", "fast-dev")
}

if ($CargoArgs) {
    $CargoCommand += $CargoArgs
}

if ($TargetTriple) {
    Write-Host "Target: $TargetTriple"
} else {
    Write-Host "Target: host default (shared target/debug cache)"
}
Write-Host "WGPU backend: $env:WGPU_BACKEND"
Write-Host "Incremental builds: $env:CARGO_INCREMENTAL"
if (-not $Release -and -not $NoFastDevFeature) {
    Write-Host "Fast dev feature: enabled"
}
Write-Host "Cargo target dir: $env:CARGO_TARGET_DIR"

Push-Location $RepositoryRoot
try {
    cargo @CargoCommand
} finally {
    Pop-Location
}
