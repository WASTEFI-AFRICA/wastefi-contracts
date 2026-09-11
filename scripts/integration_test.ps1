# ==========================================
# WasteFi Platform - Integration Testing
# ==========================================
# Tests end-to-end workflows on testnet

$ErrorActionPreference = "Continue"

function Write-Success { param($msg) Write-Host "[SUCCESS] $msg" -ForegroundColor Green }
function Write-Info { param($msg) Write-Host "[INFO] $msg" -ForegroundColor Cyan }
function Write-Error { param($msg) Write-Host "[ERROR] $msg" -ForegroundColor Red }
function Write-Test { param($msg) Write-Host "`n[TEST] $msg" -ForegroundColor Yellow }
function Write-Step { param($step, $msg) Write-Host "  [$step] $msg" -ForegroundColor White }

Write-Info "`n=========================================="
Write-Info "WasteFi Platform - Integration Testing"
Write-Info "==========================================`n"

# Load addresses and config
$addresses = Get-Content "deployed_addresses_testnet.json" | ConvertFrom-Json
$config = Get-Content "config/testnet.json" | ConvertFrom-Json

$WASTE_TOKEN = $addresses.contracts.waste_token
$COLLECTOR_REGISTRY = $addresses.contracts.collector_registry
$COLLECTION_POINT = $addresses.contracts.collection_point
$MATERIAL_PRICING = $addresses.contracts.material_pricing
$REPUTATION = $addresses.contracts.reputation
$WASTE_TRANSACTION = $addresses.contracts.waste_transaction
$PAYMENT_DISTRIBUTION = $addresses.contracts.payment_distribution

$ADMIN = $config.admin
$SOURCE = "wastefi-deployer"
$NETWORK = "testnet"

# Test accounts (using admin account for simplicity)
$TEST_COLLECTOR = $ADMIN
$TEST_COLLECTION_POINT = $ADMIN

Write-Info "Test Configuration:"
Write-Info "  Network: $NETWORK"
Write-Info "  Admin: $ADMIN"
Write-Info "  Test Collector: $TEST_COLLECTOR"
Write-Info "  Test Collection Point: $TEST_COLLECTION_POINT"
Write-Info ""

# Test results tracking
$script:testsRun = 0
$script:testsPassed = 0
$script:testsFailed = 0

function Run-Test {
    param(
        [string]$TestName,
        [scriptblock]$TestBlock
    )
    
    $script:testsRun++
    Write-Test "$TestName"
    
    try {
        & $TestBlock
        $script:testsPassed++
        Write-Success "PASSED: $TestName`n"
        return $true
    } catch {
        $script:testsFailed++
        Write-Error "FAILED: $TestName"
        Write-Error "Error: $_`n"
        return $false
    }
}

# ==========================================
# TEST 1: Verify Token Contract
# ==========================================
Run-Test "Token Contract - Basic Info" {
    Write-Step "1.1" "Get token name..."
    $name = stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- name 2>&1 | Select-String -Pattern '"([^"]+)"' | ForEach-Object { $_.Matches.Groups[1].Value }
    
    Write-Step "1.2" "Token name: $name"
    if ($name -notlike "*WasteFi*") {
        throw "Invalid token name: $name"
    }
    
    Write-Step "1.3" "Get token symbol..."
    $symbol = stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- symbol 2>&1 | Select-String -Pattern '"([^"]+)"' | ForEach-Object { $_.Matches.Groups[1].Value }
    
    Write-Step "1.4" "Token symbol: $symbol"
    if ($symbol -notlike "*WASTE*") {
        throw "Invalid token symbol: $symbol"
    }
    
    Write-Step "1.5" "Get decimals..."
    $decimals = stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- decimals 2>&1 | Select-String -Pattern '\d+' | ForEach-Object { $_.Matches.Value }
    
    Write-Step "1.6" "Token decimals: $decimals"
}

# ==========================================
# TEST 2: Register Collector
# ==========================================
Run-Test "Collector Registration" {
    Write-Step "2.1" "Registering test collector..."
    
    stellar contract invoke `
        --id $COLLECTOR_REGISTRY `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        register_collector `
        --collector $TEST_COLLECTOR `
        --name "Test Collector Alpha" `
        --location "Nairobi-Test" `
        --phone "+254700123456" 2>&1 | Out-Null
    
    Write-Step "2.2" "Collector registered"
    
    Write-Step "2.3" "Verifying registration..."
    $collectorInfo = stellar contract invoke `
        --id $COLLECTOR_REGISTRY `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_collector `
        --collector $TEST_COLLECTOR 2>&1
    
    Write-Step "2.4" "Collector info retrieved"
    if ($collectorInfo -notmatch "Test Collector Alpha") {
        throw "Collector registration verification failed"
    }
}

# ==========================================
# TEST 3: Check Initial Reputation
# ==========================================
Run-Test "Initial Reputation Score" {
    Write-Step "3.1" "Getting initial reputation score..."
    
    $reputationInfo = stellar contract invoke `
        --id $REPUTATION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_score `
        --collector $TEST_COLLECTOR 2>&1
    
    Write-Step "3.2" "Initial reputation retrieved"
    Write-Step "3.3" "Score info: $($reputationInfo -join ' ')"
}

# ==========================================
# TEST 4: Register Collection Point
# ==========================================
Run-Test "Collection Point Registration" {
    Write-Step "4.1" "Registering test collection point..."
    
    stellar contract invoke `
        --id $COLLECTION_POINT `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        register_point `
        --owner $TEST_COLLECTION_POINT `
        --name "Test Recycling Center" `
        --location "Nairobi CBD Test" `
        --capacity 5000 2>&1 | Out-Null
    
    Write-Step "4.2" "Collection point registered"
    
    Write-Step "4.3" "Verifying registration..."
    $pointInfo = stellar contract invoke `
        --id $COLLECTION_POINT `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_point `
        --point_id 1 2>&1
    
    Write-Step "4.4" "Collection point info retrieved"
    if ($pointInfo -notmatch "Test Recycling Center") {
        throw "Collection point registration verification failed"
    }
}

# ==========================================
# TEST 5: Verify Material Prices
# ==========================================
Run-Test "Material Pricing Verification" {
    $materials = @("Plastic", "Glass", "Metal", "Paper", "Electronics")
    $expectedPrices = @{
        "Plastic" = "100"
        "Glass" = "50"
        "Metal" = "200"
        "Paper" = "30"
        "Electronics" = "500"
    }
    
    $stepNum = 1
    foreach ($material in $materials) {
        Write-Step "5.$stepNum" "Checking $material price..."
        
        $price = stellar contract invoke `
            --id $MATERIAL_PRICING `
            --source $SOURCE `
            --network $NETWORK `
            -- `
            get_price `
            --material_type "$material" 2>&1 | Select-String -Pattern '"(\d+)"' | ForEach-Object { $_.Matches.Groups[1].Value }
        
        if ($price -eq $expectedPrices[$material]) {
            Write-Step "5.$stepNum" "$material - $price WASTE/kg [OK]"
        } else {
            throw "$material price mismatch - expected $($expectedPrices[$material]), got $price"
        }
        $stepNum++
    }
}

# ==========================================
# TEST 6: Token Balance Check
# ==========================================
Run-Test "Token Balance - Initial State" {
    Write-Step "6.1" "Checking collector token balance..."
    
    $balance = stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        balance `
        --account $TEST_COLLECTOR 2>&1 | Select-String -Pattern '"?(-?\d+)"?' | ForEach-Object { $_.Matches.Groups[1].Value }
    
    Write-Step "6.2" "Current balance: $balance WASTE"
    
    Write-Step "6.3" "Checking total supply..."
    $totalSupply = stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        total_supply 2>&1 | Select-String -Pattern '"?(-?\d+)"?' | ForEach-Object { $_.Matches.Groups[1].Value }
    
    Write-Step "6.4" "Total supply: $totalSupply WASTE"
}

# ==========================================
# TEST 7: Mint Test Tokens
# ==========================================
Run-Test "Token Minting" {
    Write-Step "7.1" "Minting 10,000 test tokens to collector..."
    
    stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        mint `
        --to $TEST_COLLECTOR `
        --amount 100000000000 2>&1 | Out-Null
    
    Write-Step "7.2" "Tokens minted (10,000.0000000 with 7 decimals)"
    
    Write-Step "7.3" "Verifying new balance..."
    $newBalance = stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        balance `
        --account $TEST_COLLECTOR 2>&1 | Select-String -Pattern '"?(-?\d+)"?' | ForEach-Object { $_.Matches.Groups[1].Value }
    
    Write-Step "7.4" "New balance: $newBalance"
    
    if ([int64]$newBalance -lt 100000000000) {
        throw "Balance not updated correctly"
    }
}

# ==========================================
# TEST 8: Payment Distribution - Process Payment
# ==========================================
Run-Test "Payment Distribution - Create Payment Record" {
    Write-Step "8.1" "Creating payment record for waste transaction..."
    
    # Transaction: 5kg Plastic = 500 WASTE = 5000000000 (with 7 decimals)
    $paymentResult = stellar contract invoke `
        --id $PAYMENT_DISTRIBUTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        process_payment `
        --transaction_id 1 `
        --recipient $TEST_COLLECTOR `
        --amount 5000000000 2>&1
    
    Write-Step "8.2" "Payment record created"
    
    Write-Step "8.3" "Getting payment details..."
    $payment = stellar contract invoke `
        --id $PAYMENT_DISTRIBUTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_payment `
        --payment_id 1 2>&1
    
    Write-Step "8.4" "Payment info retrieved"
    if ($payment -notmatch "5000000000") {
        throw "Payment amount mismatch"
    }
}

# ==========================================
# TEST 9: Query Payment Statistics
# ==========================================
Run-Test "Payment Distribution - Statistics" {
    Write-Step "9.1" "Getting payment count..."
    
    $count = stellar contract invoke `
        --id $PAYMENT_DISTRIBUTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_payment_count 2>&1 | Select-String -Pattern '\d+' | ForEach-Object { $_.Matches.Value }
    
    Write-Step "9.2" "Total payments: $count"
    
    if ([int]$count -lt 1) {
        throw "Payment count should be at least 1"
    }
    
    Write-Step "9.3" "Getting recipient statistics..."
    $stats = stellar contract invoke `
        --id $PAYMENT_DISTRIBUTION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_recipient_statistics `
        --recipient $TEST_COLLECTOR 2>&1
    
    Write-Step "9.4" "Recipient statistics retrieved"
}

# ==========================================
# TEST 10: Reputation System Update
# ==========================================
Run-Test "Reputation System - Score Update" {
    Write-Step "10.1" "Recording successful transaction..."
    
    stellar contract invoke `
        --id $REPUTATION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        update_score `
        --collector $TEST_COLLECTOR `
        --transaction_successful true 2>&1 | Out-Null
    
    Write-Step "10.2" "Transaction recorded"
    
    Write-Step "10.3" "Getting updated reputation score..."
    $updatedScore = stellar contract invoke `
        --id $REPUTATION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_score `
        --collector $TEST_COLLECTOR 2>&1
    
    Write-Step "10.4" "Updated reputation retrieved"
    
    Write-Step "10.5" "Getting statistics..."
    $stats = stellar contract invoke `
        --id $REPUTATION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_statistics `
        --collector $TEST_COLLECTOR 2>&1
    
    Write-Step "10.6" "Reputation statistics: $($stats -join ' ')"
}

# ==========================================
# TEST 11: Collector Query Operations
# ==========================================
Run-Test "Collector Registry - Query Operations" {
    Write-Step "11.1" "Getting total collector count..."
    
    $count = stellar contract invoke `
        --id $COLLECTOR_REGISTRY `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_collector_count 2>&1 | Select-String -Pattern '\d+' | ForEach-Object { $_.Matches.Value }
    
    Write-Step "11.2" "Total collectors: $count"
    
    if ([int]$count -lt 1) {
        throw "Should have at least 1 collector"
    }
    
    Write-Step "11.3" "Checking collector active status..."
    $isActive = stellar contract invoke `
        --id $COLLECTOR_REGISTRY `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        is_collector_active `
        --collector $TEST_COLLECTOR 2>&1
    
    Write-Step "11.4" "Collector active: $($isActive -match 'true')"
}

# ==========================================
# TEST 12: Collection Point Query Operations
# ==========================================
Run-Test "Collection Point - Query Operations" {
    Write-Step "12.1" "Getting collection point count..."
    
    $count = stellar contract invoke `
        --id $COLLECTION_POINT `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_collection_point_count 2>&1 | Select-String -Pattern '\d+' | ForEach-Object { $_.Matches.Value }
    
    Write-Step "12.2" "Total collection points: $count"
    
    if ([int]$count -lt 1) {
        throw "Should have at least 1 collection point"
    }
    
    Write-Step "12.3" "Checking point status..."
    $pointStatus = stellar contract invoke `
        --id $COLLECTION_POINT `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        is_point_active `
        --point_id 1 2>&1
    
    Write-Step "12.4" "Collection point active: $($pointStatus -match 'true')"
}

# ==========================================
# TEST 13: Pause/Unpause Functionality
# ==========================================
Run-Test "Admin Controls - Pause/Unpause" {
    Write-Step "13.1" "Pausing WasteToken contract..."
    
    stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        pause 2>&1 | Out-Null
    
    Write-Step "13.2" "Contract paused"
    
    Write-Step "13.3" "Checking paused status..."
    $isPaused = stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        is_paused 2>&1
    
    if ($isPaused -notmatch 'true') {
        throw "Contract should be paused"
    }
    
    Write-Step "13.4" "Unpausing contract..."
    stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        unpause 2>&1 | Out-Null
    
    Write-Step "13.5" "Contract unpaused"
    
    Write-Step "13.6" "Verifying unpause..."
    $isPaused = stellar contract invoke `
        --id $WASTE_TOKEN `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        is_paused 2>&1
    
    if ($isPaused -notmatch 'false') {
        throw "Contract should be unpaused"
    }
}

# ==========================================
# TEST 14: Reputation Tier System
# ==========================================
Run-Test "Reputation System - Tier Classification" {
    Write-Step "14.1" "Getting reputation tier..."
    
    $tier = stellar contract invoke `
        --id $REPUTATION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_reputation_tier `
        --collector $TEST_COLLECTOR 2>&1 | Select-String -Pattern '\d+' | ForEach-Object { $_.Matches.Value }
    
    Write-Step "14.2" "Collector tier: $tier"
    Write-Step "14.3" "Tiers: 1=Bronze, 2=Silver, 3=Gold, 4=Platinum"
    
    Write-Step "14.4" "Getting reputation breakdown..."
    $breakdown = stellar contract invoke `
        --id $REPUTATION `
        --source $SOURCE `
        --network $NETWORK `
        -- `
        get_reputation_breakdown `
        --collector $TEST_COLLECTOR 2>&1
    
    Write-Step "14.5" "Breakdown: $($breakdown -join ' ')"
}

# ==========================================
# TEST RESULTS SUMMARY
# ==========================================
Write-Info ""
Write-Info "=========================================="
Write-Info "Integration Test Results"
Write-Info "==========================================`n"

Write-Info "Tests Run: $script:testsRun"
Write-Success "Tests Passed: $script:testsPassed"
if ($script:testsFailed -gt 0) {
    Write-Error "Tests Failed: $script:testsFailed"
} else {
    Write-Info "Tests Failed: $script:testsFailed"
}

$successRate = if ($script:testsRun -gt 0) { 
    [math]::Round(($script:testsPassed / $script:testsRun) * 100, 2) 
} else { 
    0 
}

Write-Info "Success Rate: $successRate%"
Write-Info ""

# Save test report
$report = @{
    test_date = (Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ")
    network = $NETWORK
    total_tests = $script:testsRun
    passed = $script:testsPassed
    failed = $script:testsFailed
    success_rate = "$successRate%"
    test_collector = $TEST_COLLECTOR
    test_collection_point = $TEST_COLLECTION_POINT
    status = if ($script:testsFailed -eq 0) { "ALL_PASSED" } else { "SOME_FAILED" }
}

$report | ConvertTo-Json -Depth 10 | Out-File "integration_test_report.json" -Encoding UTF8

Write-Success "Test report saved to integration_test_report.json"
Write-Info ""

if ($script:testsFailed -eq 0) {
    Write-Success "=========================================="
    Write-Success "ALL INTEGRATION TESTS PASSED!"
    Write-Success "==========================================\"
    Write-Info ""
    Write-Info "The WasteFi platform is functioning correctly!"
    Write-Info "All core workflows have been validated."
    Write-Info ""
    exit 0
} else {
    Write-Info "=========================================="
    Write-Info "Some tests failed - review errors above"
    Write-Info "==========================================`n"
    exit 1
}
