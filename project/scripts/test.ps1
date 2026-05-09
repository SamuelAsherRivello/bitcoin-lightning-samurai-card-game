$ErrorActionPreference = "Stop"

& (Join-Path $PSScriptRoot "RunTests.ps1") @args
