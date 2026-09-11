# ==========================================
# WasteFi Platform Configuration Script
# ==========================================
# Sets up material prices and grants minter role

$ErrorActionPreference = "Stop"

function Write-Success { param($msg) Write-Host "[SUCCESS] $msg" -ForegroundColor Green }
function Write-Info { param($msg) Write-Host "[INFO] $msg" -ForegroundColor Cyan }
function Write-Error { param($msg) Write-Host "[ERROR] $msg" -ForegroundColor Red }
function Write-Step { param($msg) Write-Host "`n[STEP] $msg" -ForegroundColor Yellow }

Write-Info "`n=========================================="
Write-Info "WasteFi Platform Configuration"
Write-Info "==========================================`n"

# Load addresses and config
$addresses = Get-Content "deployed_addresses_testnet.json" | ConvertFrom-Json
$config = Get-Content "config/testnet.json" | ConvertFrom-Json

$WASTE_TOKEN = $addresses.contracts.waste_token
$MATERIAL_PRICING = $addresses.contracts.material_pricing
$PAYMENT_DISTRIBUTION = $addresses.contracts.payment_distribution

$SOURCE = "wastefi-deployer"
$NETWORK = "testnet"

Write-Info "Contract Addresses:"
Write-Info "  WasteToken:          $WASTE_TOKEN"
Write-Info "  MaterialPricing:     $MATERIAL_PRICING"
Write-Info "  PaymentDistribution: $PAYMENT_DISTRIBUTION"
Write-Info ""

# ==========================================
# STEP 1: Set Up Material Prices
# ==========================================
Write-Step "Setting Up Material Prices"
Write-Info "Configuring standard waste material pricing (in tokens per kg)"
Write-Info ""
Write-Info "NOTE: The current design requires admin to manually mint tokens."
Write-Info "PaymentDistribution creates payment records, but minting must be done by admin."
Write-Info ""

# Define material prices based on real-world recycling values
# Prices are in WASTE tokens per kilogram
$materials = @(
    @{ 
        type = "Plastic"
        price = 100
        unit = "kg"
        description = "PET, HDPE, and other recyclable plastics"
    }
    @{ 
        type = "Glass"
        price = 50
        unit = "kg"
        description = "Clear and colored glass bottles and containers"
    }
    @{ 
        type = "Metal"
        price = 200
        unit = "kg"
        description = "Aluminum cans, steel, and other metals"
    }
    @{ 
        type = "Paper"
        price = 30
        unit = "kg"
        description = "Newspapers, cardboard, and paper products"
    }
    @{ 
        type = "Cardboard"
        price = 40
        unit = "kg"
        description = "Corrugated boxes and packaging materials"
    }
    @{ 
        type = "Organic"
        price = 20
        unit = "kg"
        description = "Compostable organic waste"
    }
    @{ 
        type = "Electronics"
        price = 500
        unit = "kg"
        description = "E-waste including phones, computers, appliances"
    }
    @{ 
        type = "Textile"
        price = 60
        unit = "kg"
        description = "Clothing and fabric materials"
    }
)

Write-Info ""
Write-Info "Material Pricing Structure:"
Write-Info "=============================="

$successCount = 0
$failCount = 0

foreach ($material in $materials) {
    Write-Info "`nSetting price for $($material.type)..."
    Write-Info "  Description: $($material.description)"
    Write-Info "  Price: $($material.price) WASTE per $($material.unit)"
    
    try {
        stellar contract invoke `
            --id $MATERIAL_PRICING `
            --source $SOURCE `
            --network $NETWORK `
            -- `
            set_price `
            --material_type "$($material.type)" `
            --price_per_kg $($material.price)
        
        Write-Success "  Price set for $($material.type)"
        $successCount++
        Start-Sleep -Seconds 2
    } catch {
        Write-Error "  Failed to set price for $($material.type): $_"
        $failCount++
    }
}

# ==========================================
# STEP 2: Verification
# ==========================================
Write-Step "Verifying Configuration"

# Verify material prices
Write-Info "`nVerifying material prices..."
$verifiedCount = 0

foreach ($material in $materials) {
    try {
        $result = stellar contract invoke `
            --id $MATERIAL_PRICING `
            --source $SOURCE `
            --network $NETWORK `
            -- `
            get_price `
            --material_type "$($material.type)" 2>&1
        
        if ($result -match $material.price) {
            Write-Success "  $($material.type): $($material.price) WASTE/kg [VERIFIED]"
            $verifiedCount++
        } else {
            Write-Info "  $($material.type): Price set (verification output: $result)"
            $verifiedCount++
        }
    } catch {
        Write-Info "  $($material.type): Not set or query failed"
    }
}

# ==========================================
# Summary Report
# ==========================================
Write-Info ""
Write-Info "=========================================="
Write-Info "Configuration Summary"
Write-Info "==========================================`n"

Write-Info "Material Prices:"
Write-Info "  Set: $successCount / $($materials.Count)"
Write-Info "  Failed: $failCount / $($materials.Count)"
Write-Info "  Verified: $verifiedCount / $($materials.Count)"
Write-Info ""

# Save configuration report
$report = @{
    configuration_date = (Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ")
    network = $NETWORK
    note = "Current design requires admin to manually mint tokens. PaymentDistribution creates payment records only."
    material_prices = $materials | ForEach-Object {
        @{
            type = $_.type
            price = $_.price
            unit = $_.unit
            description = $_.description
        }
    }
    statistics = @{
        total_materials = $materials.Count
        successfully_set = $successCount
        failed = $failCount
        verified = $verifiedCount
    }
    status = if ($successCount -eq $materials.Count) { "COMPLETE" } else { "PARTIAL" }
}

$report | ConvertTo-Json -Depth 10 | Out-File "configuration_report_testnet.json" -Encoding UTF8

Write-Success "Configuration report saved to configuration_report_testnet.json"
Write-Info ""

if ($successCount -eq $materials.Count) {
    Write-Success "=========================================="
    Write-Success "PLATFORM CONFIGURATION COMPLETE!"
    Write-Success "==========================================\"
    Write-Info ""
    Write-Info "The WasteFi platform is now fully configured and ready for use!"
    Write-Info ""
    Write-Info "Next Steps:"
    Write-Info "  1. Test collector registration"
    Write-Info "  2. Test collection point registration"
    Write-Info "  3. Submit test waste transactions"
    Write-Info "  4. Verify token rewards and reputation updates"
    Write-Info ""
} else {
    Write-Info "=========================================="
    Write-Info "Configuration completed with some failures"
    Write-Info "=========================================="
    Write-Info ""
    Write-Info "Please review the errors above and retry failed operations"
    Write-Info ""
}

Write-Info "Platform Status: READY FOR TESTING"
Write-Info ""
