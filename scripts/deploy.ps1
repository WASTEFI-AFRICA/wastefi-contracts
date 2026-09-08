# Deploy WasteFi contracts to Stellar testnet (Windows)

Write-Host "🚀 Deploying WasteFi contracts to Stellar testnet...`n" -ForegroundColor Cyan

# Check if soroban is installed
if (!(Get-Command soroban -ErrorAction SilentlyContinue)) {
    Write-Host "Soroban CLI not found. Please run .\scripts\setup.ps1 first" -ForegroundColor Red
    exit 1
}

# Build contracts
Write-Host "Building contracts..." -ForegroundColor Yellow
cargo build --target wasm32-unknown-unknown --release
Write-Host "✓ Contracts built`n" -ForegroundColor Green

# Deploy contracts
$contracts = @("waste_token", "collector_registry", "collection_point", "waste_transaction", "payment_distribution", "reputation", "material_pricing")

Write-Host "Deploying contracts...`n" -ForegroundColor Yellow

$deployedAddresses = @()

foreach ($contract in $contracts) {
    Write-Host "Deploying $contract..." -ForegroundColor Yellow
    
    $contractId = soroban contract deploy `
        --wasm "target/wasm32-unknown-unknown/release/$contract.wasm" `
        --source deployer `
        --network testnet
    
    Write-Host "✓ $contract deployed" -ForegroundColor Green
    Write-Host "Contract ID: $contractId`n" -ForegroundColor Blue
    
    # Save contract ID
    $upperName = $contract.ToUpper()
    $deployedAddresses += "${upperName}_ADDRESS=$contractId"
}

Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Green
Write-Host "✨ All contracts deployed successfully!" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`n" -ForegroundColor Green

# Save to file
$deployedAddresses | Out-File -FilePath ".env.deployed" -Encoding UTF8

Write-Host "Contract addresses saved to .env.deployed" -ForegroundColor Yellow
Write-Host "Copy these addresses to your .env file`n" -ForegroundColor Yellow

Get-Content ".env.deployed"
