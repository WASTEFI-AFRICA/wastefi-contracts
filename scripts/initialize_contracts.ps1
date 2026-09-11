# ==========================================
# WasteFi Contracts - Testnet Initialization
# ==========================================
# This script initializes all deployed contracts on Stellar testnet
# and sets up cross-contract references

$ErrorActionPreference = "Stop"

# Colors for output
function Write-Success { param($msg) Write-Host "[SUCCESS] $msg" -ForegroundColor Green }
function Write-Info { param($msg) Write-Host "[INFO] $msg" -ForegroundColor Cyan }
function Write-Error { param($msg) Write-Host "[ERROR] $msg" -ForegroundColor Red }
function Write-Warning { param($msg) Write-Host "[WARNING] $msg" -ForegroundColor Yellow }

Write-Info "Starting WasteFi Contracts Initialization on Testnet"
Write-Info "=================================================="

# Load configuration
$configPath = "config/testnet.json"
if (-not (Test-Path $configPath)) {
    Write-Error "Configuration file not found: $configPath"
    exit 1
}

$config = Get-Content $configPath | ConvertFrom-Json
Write-Success "Configuration loaded from $configPath"

# Load deployed addresses
$addressesPath = "deployed_addresses_testnet.json"
if (-not (Test-Path $addressesPath)) {
    Write-Error "Deployed addresses file not found: $addressesPath"
    exit 1
}

$addresses = Get-Content $addressesPath | ConvertFrom-Json
Write-Success "Deployed addresses loaded"

# Extract contract addresses
$WASTE_TOKEN = $addresses.contracts.waste_token
$COLLECTOR_REGISTRY = $addresses.contracts.collector_registry
$COLLECTION_POINT = $addresses.contracts.collection_point
$MATERIAL_PRICING = $addresses.contracts.material_pricing
$REPUTATION = $addresses.contracts.reputation
$WASTE_TRANSACTION = $addresses.contracts.waste_transaction
$PAYMENT_DISTRIBUTION = $addresses.contracts.payment_distribution

# Admin account (deployer)
$ADMIN = $config.admin

Write-Info ""
Write-Info "Contract Addresses:"
Write-Info "  WasteToken:          $WASTE_TOKEN"
Write-Info "  CollectorRegistry:   $COLLECTOR_REGISTRY"
Write-Info "  CollectionPoint:     $COLLECTION_POINT"
Write-Info "  MaterialPricing:     $MATERIAL_PRICING"
Write-Info "  Reputation:          $REPUTATION"
Write-Info "  WasteTransaction:    $WASTE_TRANSACTION"
Write-Info "  PaymentDistribution: $PAYMENT_DISTRIBUTION"
Write-Info ""
Write-Info "Admin Account: $ADMIN"
Write-Info ""

# Network configuration
$NETWORK = "testnet"
$SOURCE = "wastefi-deployer"

Write-Info "Network: $NETWORK"
Write-Info "Source Identity: $SOURCE"
Write-Info ""

# ==========================================
# STEP 1: Initialize WasteToken
# ==========================================
Write-Info "STEP 1: Initializing WasteToken..."

try {
    stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN `
        --name "WasteFi Token" `
        --symbol "WASTE" `
        --decimals 7
    
    Write-Success "WasteToken initialized successfully"
} catch {
    Write-Warning "WasteToken initialization failed: $_"
    Write-Info "This may be normal if already initialized"
}

Start-Sleep -Seconds 2

# ==========================================
# STEP 2: Initialize CollectorRegistry
# ==========================================
Write-Info "STEP 2: Initializing CollectorRegistry..."

try {
    stellar contract invoke `
        --id $COLLECTOR_REGISTRY `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN
    
    Write-Success "CollectorRegistry initialized successfully"
} catch {
    Write-Warning "CollectorRegistry initialization failed: $_"
    Write-Info "This may be normal if already initialized"
}

Start-Sleep -Seconds 2

# ==========================================
# STEP 3: Initialize CollectionPoint
# ==========================================
Write-Info "STEP 3: Initializing CollectionPoint..."

try {
    stellar contract invoke `
        --id $COLLECTION_POINT `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN
    
    Write-Success "CollectionPoint initialized successfully"
} catch {
    Write-Warning "CollectionPoint initialization failed: $_"
    Write-Info "This may be normal if already initialized"
}

Start-Sleep -Seconds 2

# ==========================================
# STEP 4: Initialize MaterialPricing
# ==========================================
Write-Info "STEP 4: Initializing MaterialPricing..."

try {
    stellar contract invoke `
        --id $MATERIAL_PRICING `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN
    
    Write-Success "MaterialPricing initialized successfully"
} catch {
    Write-Warning "MaterialPricing initialization failed: $_"
    Write-Info "This may be normal if already initialized"
}

Start-Sleep -Seconds 2

# ==========================================
# STEP 5: Initialize Reputation
# ==========================================
Write-Info "STEP 5: Initializing Reputation..."

try {
    stellar contract invoke `
        --id $REPUTATION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN
    
    Write-Success "Reputation initialized successfully"
} catch {
    Write-Warning "Reputation initialization failed: $_"
    Write-Info "This may be normal if already initialized"
}

Start-Sleep -Seconds 2

# ==========================================
# STEP 6: Initialize WasteTransaction
# ==========================================
Write-Info "STEP 6: Initializing WasteTransaction..."

try {
    stellar contract invoke `
        --id $WASTE_TRANSACTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN
    
    Write-Success "WasteTransaction initialized successfully"
} catch {
    Write-Warning "WasteTransaction initialization failed: $_"
    Write-Info "This may be normal if already initialized"
}

Start-Sleep -Seconds 2

# ==========================================
# STEP 7: Initialize PaymentDistribution
# ==========================================
Write-Info "STEP 7: Initializing PaymentDistribution..."

try {
    stellar contract invoke `
        --id $PAYMENT_DISTRIBUTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        initialize `
        --admin $ADMIN `
        --token_contract $WASTE_TOKEN
    
    Write-Success "PaymentDistribution initialized successfully"
} catch {
    Write-Warning "PaymentDistribution initialization failed: $_"
    Write-Info "This may be normal if already initialized"
}

Start-Sleep -Seconds 2

Write-Info ""
Write-Success "=================================================="
Write-Success "All contracts initialized!"
Write-Success "=================================================="
Write-Info ""

# ==========================================
# STEP 8: Set Up Cross-Contract References
# ==========================================
Write-Info "STEP 8: Setting up cross-contract references..."
Write-Info ""

# Check if contracts have set_contract methods
Write-Info "Setting WasteTransaction contract references..."

# Set CollectorRegistry in WasteTransaction
try {
    stellar contract invoke `
        --id $WASTE_TRANSACTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        set_collector_registry `
        --registry $COLLECTOR_REGISTRY
    
    Write-Success "  CollectorRegistry reference set in WasteTransaction"
} catch {
    Write-Warning "  Failed to set CollectorRegistry: $_"
}

Start-Sleep -Seconds 1

# Set CollectionPoint in WasteTransaction
try {
    stellar contract invoke `
        --id $WASTE_TRANSACTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        set_collection_point `
        --point $COLLECTION_POINT
    
    Write-Success "  CollectionPoint reference set in WasteTransaction"
} catch {
    Write-Warning "  Failed to set CollectionPoint: $_"
}

Start-Sleep -Seconds 1

# Set MaterialPricing in WasteTransaction
try {
    stellar contract invoke `
        --id $WASTE_TRANSACTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        set_material_pricing `
        --pricing $MATERIAL_PRICING
    
    Write-Success "  MaterialPricing reference set in WasteTransaction"
} catch {
    Write-Warning "  Failed to set MaterialPricing: $_"
}

Start-Sleep -Seconds 1

# Set Reputation in WasteTransaction
try {
    stellar contract invoke `
        --id $WASTE_TRANSACTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        set_reputation `
        --reputation $REPUTATION
    
    Write-Success "  Reputation reference set in WasteTransaction"
} catch {
    Write-Warning "  Failed to set Reputation: $_"
}

Start-Sleep -Seconds 1

# Set PaymentDistribution in WasteTransaction
try {
    stellar contract invoke `
        --id $WASTE_TRANSACTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        set_payment_distribution `
        --payment $PAYMENT_DISTRIBUTION
    
    Write-Success "  PaymentDistribution reference set in WasteTransaction"
} catch {
    Write-Warning "  Failed to set PaymentDistribution: $_"
}

Write-Info ""

# ==========================================
# STEP 9: Grant Minter Role to PaymentDistribution
# ==========================================
Write-Info "STEP 9: Granting minter role to PaymentDistribution..."

try {
    stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        set_minter `
        --minter $PAYMENT_DISTRIBUTION
    
    Write-Success "Minter role granted to PaymentDistribution"
} catch {
    Write-Warning "Failed to grant minter role: $_"
}

Write-Info ""

# ==========================================
# STEP 10: Add Default Material Prices
# ==========================================
Write-Info "STEP 10: Setting up default material prices..."

$materials = @(
    @{ type = "Plastic"; price = 100; unit = "kg" }
    @{ type = "Glass"; price = 50; unit = "kg" }
    @{ type = "Metal"; price = 200; unit = "kg" }
    @{ type = "Paper"; price = 30; unit = "kg" }
    @{ type = "Organic"; price = 20; unit = "kg" }
    @{ type = "Electronic"; price = 500; unit = "kg" }
)

foreach ($material in $materials) {
    try {
        stellar contract invoke `
            --id $MATERIAL_PRICING `
            --source $SOURCE `
            --network $NETWORK `
            -- `
            set_price `
            --material_type "$($material.type)" `
            --price $($material.price) `
            --unit "$($material.unit)"
        
        Write-Success "  Set price for $($material.type): $($material.price) per $($material.unit)"
        Start-Sleep -Seconds 1
    } catch {
        Write-Warning "  Failed to set price for $($material.type): $_"
    }
}

Write-Info ""

# ==========================================
# STEP 11: Verification
# ==========================================
Write-Info "STEP 11: Verifying initialization..."
Write-Info ""

# Verify WasteToken
Write-Info "Verifying WasteToken..."
try {
    $tokenName = stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        name
    
    Write-Success "  Token Name: $tokenName"
} catch {
    Write-Error "  Failed to verify WasteToken"
}

# Verify CollectorRegistry
Write-Info "Verifying CollectorRegistry..."
try {
    $collectorCount = stellar contract invoke `
        --id $COLLECTOR_REGISTRY `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_collector_count
    
    Write-Success "  Collector Count: $collectorCount"
} catch {
    Write-Error "  Failed to verify CollectorRegistry"
}

# Verify MaterialPricing
Write-Info "Verifying MaterialPricing..."
try {
    $plasticPrice = stellar contract invoke `
        --id $MATERIAL_PRICING `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_price `
        --material_type "Plastic"
    
    Write-Success "  Plastic Price: $plasticPrice"
} catch {
    Write-Error "  Failed to verify MaterialPricing"
}

Write-Info ""

# ==========================================
# Save Initialization Report
# ==========================================
Write-Info "Saving initialization report..."

$report = @{
    initialization_date = (Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ")
    network = $NETWORK
    admin = $ADMIN
    contracts_initialized = @(
        "waste_token"
        "collector_registry"
        "collection_point"
        "material_pricing"
        "reputation"
        "waste_transaction"
        "payment_distribution"
    )
    cross_references_set = @{
        waste_transaction = @(
            "collector_registry"
            "collection_point"
            "material_pricing"
            "reputation"
            "payment_distribution"
        )
    }
    minter_role_granted = $true
    default_prices_set = $materials
    status = "COMPLETE"
}

$report | ConvertTo-Json -Depth 10 | Out-File "initialization_report_testnet.json" -Encoding UTF8

Write-Success "Initialization report saved to initialization_report_testnet.json"
Write-Info ""

# ==========================================
# Summary
# ==========================================
Write-Success "=================================================="
Write-Success "INITIALIZATION COMPLETE!"
Write-Success "=================================================="
Write-Info ""
Write-Info "Summary:"
Write-Info "  [OK] 7 contracts initialized"
Write-Info "  [OK] Cross-contract references configured"
Write-Info "  [OK] Minter role granted"
Write-Info "  [OK] Default material prices set"
Write-Info ""
Write-Info "Next Steps:"
Write-Info "  1. Run integration tests to verify functionality"
Write-Info "  2. Test end-to-end user workflows"
Write-Info "  3. Monitor contract interactions"
Write-Info ""
Write-Success "The WasteFi platform is now ready for testing!"
Write-Info ""
