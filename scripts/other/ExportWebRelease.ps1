param(
    [string]$OutputPath = "target/run-app-web/site"
)

$ErrorActionPreference = "Stop"

$RepositoryRoot = (Resolve-Path (Join-Path (Join-Path $PSScriptRoot "..") "..")).Path
$ResolvedOutputPath = [System.IO.Path]::GetFullPath((Join-Path $RepositoryRoot $OutputPath))
$DefaultWebRoot = [System.IO.Path]::GetFullPath((Join-Path (Join-Path (Join-Path $RepositoryRoot "target") "run-app-web") "site"))

& (Join-Path $PSScriptRoot "RunAppWeb.ps1") -Release -NoOpen -ExportOnly
if ($LASTEXITCODE -ne 0) {
    throw "RunAppWeb.ps1 failed with exit code $LASTEXITCODE."
}

if ($ResolvedOutputPath -ne $DefaultWebRoot) {
    if (Test-Path $ResolvedOutputPath) {
        Remove-Item -Path $ResolvedOutputPath -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path $ResolvedOutputPath | Out-Null
    Copy-Item -Path (Join-Path $DefaultWebRoot "*") -Destination $ResolvedOutputPath -Recurse -Force
}

$IndexPath = Join-Path $ResolvedOutputPath "index.html"
$FallbackPath = Join-Path $ResolvedOutputPath "404.html"
$SkyBoltTitlePath = Join-Path (Join-Path (Join-Path (Join-Path (Join-Path $ResolvedOutputPath "assets") "cards") "card_types") "card_type_skybolt") "title_skybolt.png"
$TarTitlePath = Join-Path (Join-Path (Join-Path (Join-Path (Join-Path $ResolvedOutputPath "assets") "cards") "card_types") "card_type_tar") "title_tar.png"

if (-not (Test-Path $IndexPath -PathType Leaf)) {
    throw "Expected release index.html was not found: $IndexPath"
}

$IndexContent = Get-Content -Raw -Path $IndexPath
if (-not $IndexContent.Contains("Bevy Card Game")) {
    throw "Expected release index.html to contain the application title."
}

$WasmFile = Get-ChildItem -Path $ResolvedOutputPath -Filter "bevy_card_game_bg.wasm" -File | Select-Object -First 1
if (-not $WasmFile) {
    throw "Expected bevy_card_game_bg.wasm was not found in $ResolvedOutputPath."
}

if (-not (Test-Path $SkyBoltTitlePath -PathType Leaf)) {
    throw "Expected SkyBolt title asset was not found: $SkyBoltTitlePath"
}

if (-not (Test-Path $TarTitlePath -PathType Leaf)) {
    throw "Expected Tar title asset was not found: $TarTitlePath"
}

Copy-Item -Path $IndexPath -Destination $FallbackPath -Force

Write-Host "Exported release web app: $ResolvedOutputPath"
if ($env:GITHUB_OUTPUT) {
    "web_release_output=$OutputPath" | Add-Content -Path $env:GITHUB_OUTPUT -Encoding UTF8
}
