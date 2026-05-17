param(
    [switch]$EnableFastDevFeature,
    [switch]$AiRuntime,
    [switch]$NoAiRuntime,
    [string]$DioxusCliVersion = "0.7.9",
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$DxArgs
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$PackageName = "samurai-card-game"
$FastDevFeature = "fast-dev"
$AssetHotReloadFeature = "asset-hot-reload"
$HotReloadFeature = "desktop-hot-reload"
$AiRuntimeFeature = "ai-runtime"
$IsWindowsHost = $env:OS -eq "Windows_NT"
$UseAiRuntime = $AiRuntime
if ($AiRuntime) {
    $UseAiRuntime = $true
}
if ($NoAiRuntime) {
    $UseAiRuntime = $false
}

function Test-CommandExists {
    param([Parameter(Mandatory = $true)][string]$Name)

    return $null -ne (Get-Command $Name -ErrorAction SilentlyContinue)
}

function Test-DioxusCliVersion {
    param([Parameter(Mandatory = $true)][string]$VersionOutput)

    return $VersionOutput -match "0\.7(\.|-|$)"
}

& (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -Quiet

if (-not (Test-CommandExists "dx")) {
    throw "Dioxus CLI is required for desktop hot reload. Install it with: cargo install dioxus-cli --version $DioxusCliVersion --locked"
}

$DxVersionOutput = (& dx --version | Out-String).Trim()
if (-not (Test-DioxusCliVersion -VersionOutput $DxVersionOutput)) {
    Write-Warning "Detected '$DxVersionOutput'."
    throw "Hot reload requires Dioxus CLI 0.7.x with --hot-patch support. Install a compatible version with: cargo install dioxus-cli --version $DioxusCliVersion --locked --force"
}

$env:CARGO_INCREMENTAL = "1"
$env:CARGO_TARGET_DIR = Join-Path $RepositoryRoot "target\run-app-desktop-hot-reload"
$env:WGPU_BACKEND = "dx12"
$env:BEVY_ASSET_ROOT = $RepositoryRoot
if (-not $env:RUST_BACKTRACE) {
    $env:RUST_BACKTRACE = "1"
}
if (-not $env:RUST_LIB_BACKTRACE) {
    $env:RUST_LIB_BACKTRACE = "1"
}
if (-not $env:CARGO_BUILD_JOBS) {
    $env:CARGO_BUILD_JOBS = [Environment]::ProcessorCount
}

if ($IsWindowsHost -and $EnableFastDevFeature) {
    Write-Warning "Windows hot-patch compatibility mode ignores -EnableFastDevFeature because Bevy dynamic linking can conflict with dx hot patching."
    $EnableFastDevFeature = $false
}

if (Test-CommandExists "sccache") {
    Write-Host "sccache detected but CARGO_INCREMENTAL is set: skipping compiler cache."
} else {
    Write-Host "No sccache detected."
}

Write-Host ""
Write-Host "Starting desktop hot reload with Dioxus CLI."
Write-Host "Package: $PackageName"
Write-Host "Target dir: $env:CARGO_TARGET_DIR"
Write-Host "Dioxus CLI: $DxVersionOutput"
Write-Host "Rust backtrace: RUST_BACKTRACE=$env:RUST_BACKTRACE, RUST_LIB_BACKTRACE=$env:RUST_LIB_BACKTRACE"
Write-Host "Edit hot-reload-enabled Rust systems and save."
$FeatureList = @($HotReloadFeature, $AssetHotReloadFeature)
if ($UseAiRuntime) {
    $FeatureList += $AiRuntimeFeature
}
if ($EnableFastDevFeature) {
    $FeatureList += $FastDevFeature
}

Write-Host "Using features: $($FeatureList -join ',')"
if (-not $EnableFastDevFeature) {
    Write-Host "Running without '$FastDevFeature' for hot-patch compatibility."
}
if ($UseAiRuntime) {
    Write-Host "AI runtime bridge: Bevy Remote Protocol at http://localhost:15702"
    Write-Host "AI runtime screenshot method: bevy_debugger/screenshot"
} else {
    Write-Host "AI runtime bridge disabled. Use -AiRuntime or scripts/other/RunAppDesktopHotReloadMCP.ps1 to enable it."
}
Write-Host "Press Ctrl+C to stop."
Write-Host ""

$CommandArgs = @("serve", "--hot-patch", "--windows", "--package", $PackageName, "--bin", $PackageName)
$CommandArgs += @("--features", ($FeatureList -join " "))
if ($DxArgs) {
    $CommandArgs += $DxArgs
}

Push-Location $RepositoryRoot
try {
    & dx @CommandArgs
    $DxExitCode = $LASTEXITCODE
} finally {
    Pop-Location
}

$InterruptedExitCodes = @(-1, 130, 3221225786)
if ($DxExitCode -eq 0 -or $InterruptedExitCodes -contains $DxExitCode) {
    Write-Host "Desktop hot reload stopped (exit code $DxExitCode)."
    return
}

throw "Desktop hot reload failed with exit code $DxExitCode."

