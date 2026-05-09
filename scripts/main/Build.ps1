$ErrorActionPreference = "Stop"

$RepositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")

& (Join-Path $PSScriptRoot "..\other\StopApp.ps1") -Quiet

Push-Location $RepositoryRoot
try {
    cargo build --workspace
} finally {
    Pop-Location
}
