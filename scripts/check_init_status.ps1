# Check initialization status of all contracts
$ErrorActionPreference = "Continue"

# Load addresses
$addresses = Get-Content "deployed_addresses_testnet.json" | ConvertFrom-Json

$WASTE_TOKEN = $addresses.contracts.waste_token
$COLLECTOR_REGISTRY = $addresses.contracts.collector_registry
$COLLECTION_POINT = $addresses.contracts.collection_point
$MATERIAL_PRICING = $addresses.contracts.material_pricing
$REPUTATION = $addresses.contracts.reputation
$WASTE_TRANSACTION = $addresses.contracts.waste_transaction
$PAYMENT_DISTRIBUTION = $addresses.contracts.payment_distribution

Write-Host "`n=== Checking Initialization Status ===`n" -ForegroundColor Cyan

# Check each contract
Write-Host "1. WasteToken..." -NoNewline
try {
    $result = stellar contract invoke --id $WASTE_TOKEN --source wastefi-deployer --network testnet -- name 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " [INITIALIZED]" -ForegroundColor Green
        Write-Host "   Name: $result"
    } else {
        Write-Host " [NOT INITIALIZED]" -ForegroundColor Yellow
    }
} catch {
    Write-Host " [ERROR]" -ForegroundColor Red
}

Write-Host "`n2. CollectorRegistry..." -NoNewline
try {
    $result = stellar contract invoke --id $COLLECTOR_REGISTRY --source wastefi-deployer --network testnet -- get_collector_count 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " [INITIALIZED]" -ForegroundColor Green
        Write-Host "   Collector Count: $result"
    } else {
        Write-Host " [NOT INITIALIZED]" -ForegroundColor Yellow
    }
} catch {
    Write-Host " [ERROR]" -ForegroundColor Red
}

Write-Host "`n3. CollectionPoint..." -NoNewline
try {
    $result = stellar contract invoke --id $COLLECTION_POINT --source wastefi-deployer --network testnet -- get_collection_point_count 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " [INITIALIZED]" -ForegroundColor Green
        Write-Host "   Point Count: $result"
    } else {
        Write-Host " [NOT INITIALIZED]" -ForegroundColor Yellow
    }
} catch {
    Write-Host " [ERROR]" -ForegroundColor Red
}

Write-Host "`n4. MaterialPricing..." -NoNewline
try {
    $result = stellar contract invoke --id $MATERIAL_PRICING --source wastefi-deployer --network testnet -- get_material_type_count 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " [INITIALIZED]" -ForegroundColor Green
        Write-Host "   Material Types: $result"
    } else {
        Write-Host " [NOT INITIALIZED]" -ForegroundColor Yellow
    }
} catch {
    Write-Host " [ERROR]" -ForegroundColor Red
}

Write-Host "`n5. Reputation..." -NoNewline
try {
    $result = stellar contract invoke --id $REPUTATION --source wastefi-deployer --network testnet -- get_total_collectors 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " [INITIALIZED]" -ForegroundColor Green
        Write-Host "   Total Collectors: $result"
    } else {
        Write-Host " [NOT INITIALIZED]" -ForegroundColor Yellow
    }
} catch {
    Write-Host " [ERROR]" -ForegroundColor Red
}

Write-Host "`n6. WasteTransaction..." -NoNewline
try {
    $result = stellar contract invoke --id $WASTE_TRANSACTION --source wastefi-deployer --network testnet -- get_transaction_count 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " [INITIALIZED]" -ForegroundColor Green
        Write-Host "   Transaction Count: $result"
    } else {
        Write-Host " [NOT INITIALIZED]" -ForegroundColor Yellow
    }
} catch {
    Write-Host " [ERROR]" -ForegroundColor Red
}

Write-Host "`n7. PaymentDistribution..." -NoNewline
try {
    $result = stellar contract invoke --id $PAYMENT_DISTRIBUTION --source wastefi-deployer --network testnet -- get_total_distributed 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host " [INITIALIZED]" -ForegroundColor Green
        Write-Host "   Total Distributed: $result"
    } else {
        Write-Host " [NOT INITIALIZED]" -ForegroundColor Yellow
    }
} catch {
    Write-Host " [ERROR]" -ForegroundColor Red
}

Write-Host "`n=== Status Check Complete ===`n" -ForegroundColor Cyan
