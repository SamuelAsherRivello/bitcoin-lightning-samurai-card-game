$ErrorActionPreference = "Stop"

$RepositoryRoot = Resolve-Path (Join-Path $PSScriptRoot "../..")

Push-Location $RepositoryRoot
try {
    cargo test --workspace
} finally {
    Pop-Location
}
