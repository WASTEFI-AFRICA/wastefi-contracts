# Build the contract wasm artifacts. See build-wasm.sh for why each contract is
# built with an explicit --crate-type cdylib rather than a plain `cargo build`.
#
# Usage: .\scripts\build-wasm.ps1 [crate ...]

$ErrorActionPreference = "Stop"

$Contracts = @(
    "collector_registry", "collection_point", "waste_token", "waste_transaction",
    "payment_distribution", "reputation", "material_pricing"
)

Set-Location (Join-Path $PSScriptRoot "..")

$Targets = if ($args.Count -gt 0) { $args } else { $Contracts }

foreach ($crate in $Targets) {
    Write-Host "Building $crate"
    cargo rustc --quiet --package $crate --release --target wasm32v1-none --crate-type cdylib
    if ($LASTEXITCODE -ne 0) { throw "Build failed for $crate" }
}

Write-Host "`nBuilt contracts:"
foreach ($crate in $Targets) {
    $wasm = "target\wasm32v1-none\release\$crate.wasm"
    "{0,-26} {1,7:N1} KB" -f "$crate.wasm", ((Get-Item $wasm).Length / 1KB)
}
