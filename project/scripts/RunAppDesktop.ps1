param(
    [switch]$Release,
    [switch]$UseSccache,
    [switch]$NoFastLinker,
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "../..")
$TargetTriple = "x86_64-pc-windows-msvc"
$PackageName = "bevy-card-game"

$env:CARGO_TARGET_DIR = Join-Path $RepositoryRoot "target"

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
    "--package", $PackageName,
    "--target", $TargetTriple
)

if ($Release) {
    $CargoCommand += "--release"
}

if ($CargoArgs) {
    $CargoCommand += $CargoArgs
}

Write-Host "Target: $TargetTriple"
Write-Host "Incremental builds: $env:CARGO_INCREMENTAL"
Write-Host "Cargo target dir: $env:CARGO_TARGET_DIR"

Push-Location $RepositoryRoot
try {
    cargo @CargoCommand
} finally {
    Pop-Location
}
