$ErrorActionPreference = "Stop"

& (Join-Path $PSScriptRoot "..\other\RunTests.ps1") @args
