# WasteFi Testnet Deployment Script (Stellar CLI v28)
# Simplified deployment for manual execution

$ErrorActionPreference = "Stop"

Write-Host "================================" -ForegroundColor Cyan
Write-Host "WasteFi Testnet Deployment" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

# Configuration
$Network = "testnet"
$Identity = "wastefi-deployer"
$DeployerAddress = "GBKKMOEBKLQU3YARAASL7CB7YWAEXOZ47AXW6AIEZ5UAWFP5SIS4NJNM"

Write-Host "Network: $Network" -ForegroundColor Yellow
Write-Host "Identity: $Identity" -ForegroundColor Yellow
Write-Host "Deployer: $DeployerAddress" -ForegroundColor Yellow
Write-Host ""

# Check contracts are built
Write-Host "[1/8] Checking contracts..." -ForegroundColor Blue
$WasmPath = "target\wasm32-unknown-unknown\release"

if (-not (Test-Path "$WasmPath\waste_token.wasm")) {
    Write-Host "❌ Contracts not built. Building now..." -ForegroundColor Red
    cargo build --target wasm32-unknown-unknown --release
} else {
    Write-Host "✅ Contracts found" -ForegroundColor Green
}

Write-Host ""
Write-Host "[2/8] Deploying WasteToken..." -ForegroundColor Blue
try {
    $WASTE_TOKEN = stellar contract deploy `
        --wasm "$WasmPath\waste_token.wasm" `
        --source $Identity `
        --network $Network 2>&1 | Select-Object -Last 1
    
    Write-Host "✅ WasteToken deployed: $WASTE_TOKEN" -ForegroundColor Green
    
    # Initialize
    stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $Identity `
        --network $Network `
        -- initialize `
        --admin $DeployerAddress
    
    Write-Host "✅ WasteToken initialized" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to deploy WasteToken: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[3/8] Deploying CollectorRegistry..." -ForegroundColor Blue
try {
    $COLLECTOR_REGISTRY = stellar contract deploy `
        --wasm "$WasmPath\collector_registry.wasm" `
        --source $Identity `
        --network $Network 2>&1 | Select-Object -Last 1
    
    Write-Host "✅ CollectorRegistry deployed: $COLLECTOR_REGISTRY" -ForegroundColor Green
    
    # Initialize
    stellar contract invoke `
        --id $COLLECTOR_REGISTRY `
        --source $Identity `
        --network $Network `
        -- initialize `
        --admin $DeployerAddress
    
    Write-Host "✅ CollectorRegistry initialized" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to deploy CollectorRegistry: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[4/8] Deploying CollectionPoint..." -ForegroundColor Blue
try {
    $COLLECTION_POINT = stellar contract deploy `
        --wasm "$WasmPath\collection_point.wasm" `
        --source $Identity `
        --network $Network 2>&1 | Select-Object -Last 1
    
    Write-Host "✅ CollectionPoint deployed: $COLLECTION_POINT" -ForegroundColor Green
    
    # Initialize
    stellar contract invoke `
        --id $COLLECTION_POINT `
        --source $Identity `
        --network $Network `
        -- initialize `
        --admin $DeployerAddress
    
    Write-Host "✅ CollectionPoint initialized" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to deploy CollectionPoint: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[5/8] Deploying MaterialPricing..." -ForegroundColor Blue
try {
    $MATERIAL_PRICING = stellar contract deploy `
        --wasm "$WasmPath\material_pricing.wasm" `
        --source $Identity `
        --network $Network 2>&1 | Select-Object -Last 1
    
    Write-Host "✅ MaterialPricing deployed: $MATERIAL_PRICING" -ForegroundColor Green
    
    # Initialize
    stellar contract invoke `
        --id $MATERIAL_PRICING `
        --source $Identity `
        --network $Network `
        -- initialize `
        --admin $DeployerAddress
    
    Write-Host "✅ MaterialPricing initialized" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to deploy MaterialPricing: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[6/8] Deploying Reputation..." -ForegroundColor Blue
try {
    $REPUTATION = stellar contract deploy `
        --wasm "$WasmPath\reputation.wasm" `
        --source $Identity `
        --network $Network 2>&1 | Select-Object -Last 1
    
    Write-Host "✅ Reputation deployed: $REPUTATION" -ForegroundColor Green
    
    # Initialize
    stellar contract invoke `
        --id $REPUTATION `
        --source $Identity `
        --network $Network `
        -- initialize `
        --admin $DeployerAddress
    
    Write-Host "✅ Reputation initialized" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to deploy Reputation: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[7/8] Deploying WasteTransaction..." -ForegroundColor Blue
try {
    $WASTE_TRANSACTION = stellar contract deploy `
        --wasm "$WasmPath\waste_transaction.wasm" `
        --source $Identity `
        --network $Network 2>&1 | Select-Object -Last 1
    
    Write-Host "✅ WasteTransaction deployed: $WASTE_TRANSACTION" -ForegroundColor Green
    
    # Initialize
    stellar contract invoke `
        --id $WASTE_TRANSACTION `
        --source $Identity `
        --network $Network `
        -- initialize `
        --admin $DeployerAddress
    
    Write-Host "✅ WasteTransaction initialized" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to deploy WasteTransaction: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[8/8] Deploying PaymentDistribution..." -ForegroundColor Blue
try {
    $PAYMENT_DISTRIBUTION = stellar contract deploy `
        --wasm "$WasmPath\payment_distribution.wasm" `
        --source $Identity `
        --network $Network 2>&1 | Select-Object -Last 1
    
    Write-Host "✅ PaymentDistribution deployed: $PAYMENT_DISTRIBUTION" -ForegroundColor Green
    
    # Initialize
    stellar contract invoke `
        --id $PAYMENT_DISTRIBUTION `
        --source $Identity `
        --network $Network `
        -- initialize `
        --admin $DeployerAddress
    
    Write-Host "✅ PaymentDistribution initialized" -ForegroundColor Green
} catch {
    Write-Host "❌ Failed to deploy PaymentDistribution: $_" -ForegroundColor Red
    exit 1
}

# Save addresses to JSON
Write-Host ""
Write-Host "Saving contract addresses..." -ForegroundColor Blue

$Addresses = @{
    network = "testnet"
    deployment_date = (Get-Date -Format "yyyy-MM-ddTHH:mm:ss")
    deployer = $DeployerAddress
    contracts = @{
        waste_token = $WASTE_TOKEN
        collector_registry = $COLLECTOR_REGISTRY
        collection_point = $COLLECTION_POINT
        material_pricing = $MATERIAL_PRICING
        reputation = $REPUTATION
        waste_transaction = $WASTE_TRANSACTION
        payment_distribution = $PAYMENT_DISTRIBUTION
    }
}

$Addresses | ConvertTo-Json -Depth 10 | Set-Content "deployed_addresses_testnet.json"

Write-Host "✅ Addresses saved to deployed_addresses_testnet.json" -ForegroundColor Green

Write-Host ""
Write-Host "================================" -ForegroundColor Cyan
Write-Host "✅ DEPLOYMENT SUCCESSFUL!" -ForegroundColor Green
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Contract Addresses:" -ForegroundColor Yellow
Write-Host "  WasteToken:          $WASTE_TOKEN" -ForegroundColor White
Write-Host "  CollectorRegistry:   $COLLECTOR_REGISTRY" -ForegroundColor White
Write-Host "  CollectionPoint:     $COLLECTION_POINT" -ForegroundColor White
Write-Host "  MaterialPricing:     $MATERIAL_PRICING" -ForegroundColor White
Write-Host "  Reputation:          $REPUTATION" -ForegroundColor White
Write-Host "  WasteTransaction:    $WASTE_TRANSACTION" -ForegroundColor White
Write-Host "  PaymentDistribution: $PAYMENT_DISTRIBUTION" -ForegroundColor White
Write-Host ""
