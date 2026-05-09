$ErrorActionPreference = "Stop"

$RepositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")

& (Join-Path $PSScriptRoot "StopApp.ps1") -Quiet

Push-Location $RepositoryRoot
try {
    cargo test --workspace
} finally {
    Pop-Location
}
