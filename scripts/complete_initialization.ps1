# Complete initialization of remaining contracts
$ErrorActionPreference = "Stop"

# Load addresses and config
$addresses = Get-Content "deployed_addresses_testnet.json" | ConvertFrom-Json
$config = Get-Content "config/testnet.json" | ConvertFrom-Json

$ADMIN = $config.admin
$SOURCE = "wastefi-deployer"
$NETWORK = "testnet"

Write-Host "`n=== Completing Contract Initialization ===`n" -ForegroundColor Cyan

# Initialize CollectionPoint
Write-Host "1. Initializing CollectionPoint..." -ForegroundColor Yellow
try {
    stellar contract invoke `
        --id $addresses.contracts.collection_point `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN
    
    Write-Host "[SUCCESS] CollectionPoint initialized`n" -ForegroundColor Green
    Start-Sleep -Seconds 3
} catch {
    Write-Host "[FAILED] CollectionPoint: $_`n" -ForegroundColor Red
}

# Initialize MaterialPricing
Write-Host "2. Initializing MaterialPricing..." -ForegroundColor Yellow
try {
    stellar contract invoke `
        --id $addresses.contracts.material_pricing `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN
    
    Write-Host "[SUCCESS] MaterialPricing initialized`n" -ForegroundColor Green
    Start-Sleep -Seconds 3
} catch {
    Write-Host "[FAILED] MaterialPricing: $_`n" -ForegroundColor Red
}

# Initialize Reputation
Write-Host "3. Initializing Reputation..." -ForegroundColor Yellow
try {
    stellar contract invoke `
        --id $addresses.contracts.reputation `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN
    
    Write-Host "[SUCCESS] Reputation initialized`n" -ForegroundColor Green
    Start-Sleep -Seconds 3
} catch {
    Write-Host "[FAILED] Reputation: $_`n" -ForegroundColor Red
}

# Initialize PaymentDistribution
Write-Host "4. Initializing PaymentDistribution..." -ForegroundColor Yellow
try {
    stellar contract invoke `
        --id $addresses.contracts.payment_distribution `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN `
        --token_contract $addresses.contracts.waste_token
    
    Write-Host "[SUCCESS] PaymentDistribution initialized`n" -ForegroundColor Green
    Start-Sleep -Seconds 3
} catch {
    Write-Host "[FAILED] PaymentDistribution: $_`n" -ForegroundColor Red
}

Write-Host "=== Initialization Complete! ===`n" -ForegroundColor Green
