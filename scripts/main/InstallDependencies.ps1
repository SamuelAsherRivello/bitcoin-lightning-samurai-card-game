param(
    [switch]$CheckOnly
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$TargetTriple = "x86_64-pc-windows-msvc"
$Toolchain = "stable"

& (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -Quiet

function Test-CommandExists {
    param([Parameter(Mandatory = $true)][string]$Name)

    return $null -ne (Get-Command $Name -ErrorAction SilentlyContinue)
}

function Add-CargoBinToPath {
    $CargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    if ((Test-Path $CargoBin) -and ($env:PATH -notlike "*$CargoBin*")) {
        $env:PATH = "$CargoBin;$env:PATH"
    }
}

function Install-Rustup {
    if ($CheckOnly) {
        throw "Rust was not found. Install rustup from https://rustup.rs/ or run this script without -CheckOnly."
    }

    if (-not (Test-CommandExists "winget")) {
        throw "Rust was not found and winget is unavailable. Install rustup from https://rustup.rs/ and rerun this script."
    }

    Write-Host "Installing rustup with winget..."
    winget install --id Rustlang.Rustup --exact --source winget
    Add-CargoBinToPath
}

Push-Location $RepositoryRoot
try {
    Add-CargoBinToPath

    if (-not (Test-CommandExists "rustup")) {
        Install-Rustup
    }

    if (-not (Test-CommandExists "cargo")) {
        Add-CargoBinToPath
    }

    if (-not (Test-CommandExists "cargo")) {
        throw "Cargo was not found after rustup setup. Restart the terminal and rerun this script."
    }

    if (-not $CheckOnly) {
        Write-Host "Installing Rust toolchain: $Toolchain"
        rustup toolchain install $Toolchain

        Write-Host "Installing Rust target: $TargetTriple"
        rustup target add $TargetTriple --toolchain $Toolchain
    }

    Write-Host "Rust:"
    rustc --version

    Write-Host "Cargo:"
    cargo --version

    $InstalledTargets = rustup target list --installed
    if ($InstalledTargets -notcontains $TargetTriple) {
        throw "Rust target '$TargetTriple' is not installed. Run this script without -CheckOnly to install it."
    }

    Write-Host "Rust targets:"
    $InstalledTargets

    if (-not (Test-CommandExists "link")) {
        Write-Warning "MSVC linker 'link.exe' was not found on PATH. If builds fail, install Visual Studio Build Tools with the Desktop development with C++ workload."
    }

    if (-not (Test-CommandExists "rust-lld")) {
        Write-Host "Optional fast linker rust-lld was not found on PATH; RunAppDesktop.ps1 will use the default Windows linker."
    }

    Write-Host "Checking Cargo workspace metadata..."
    cargo metadata --no-deps --format-version 1 | Out-Null

    Write-Host "Dependencies are ready."
} finally {
    Pop-Location
}
