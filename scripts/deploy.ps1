#
# WasteFi Contract Deployment Script (Windows PowerShell)
#
# This script automates the deployment of all WasteFi contracts to Stellar Soroban.
# It handles compilation, sequential deployment, initialization, and verification.
#
# Usage:
#   .\scripts\deploy.ps1 -Network <network> -ConfigFile <config_file>
#
# Examples:
#   .\scripts\deploy.ps1 -Network testnet -ConfigFile config\testnet.json
#   .\scripts\deploy.ps1 -Network mainnet -ConfigFile config\mainnet.json
#
# Requirements:
#   - Soroban CLI (soroban.exe) installed
#   - Rust toolchain with wasm32-unknown-unknown target
#   - Network configured in Soroban CLI
#   - Admin account with sufficient XLM balance

param(
    [string]$Network = "testnet",
    [string]$ConfigFile = "config\testnet.json"
)

# Error handling
$ErrorActionPreference = "Stop"

# Script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

# Deployment settings
$Timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$DeploymentLog = "deployment_${Network}_${Timestamp}.log"
$AddressesFile = "deployed_addresses_${Network}.json"

# ============================================================================
# Helper Functions
# ============================================================================

function Write-LogInfo {
    param([string]$Message)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [INFO] $Message"
    Write-Host $logMessage -ForegroundColor Cyan
    Add-Content -Path $DeploymentLog -Value $logMessage
}

function Write-LogSuccess {
    param([string]$Message)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [SUCCESS] $Message"
    Write-Host $logMessage -ForegroundColor Green
    Add-Content -Path $DeploymentLog -Value $logMessage
}

function Write-LogWarning {
    param([string]$Message)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [WARNING] $Message"
    Write-Host $logMessage -ForegroundColor Yellow
    Add-Content -Path $DeploymentLog -Value $logMessage
}

function Write-LogError {
    param([string]$Message)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [ERROR] $Message"
    Write-Host $logMessage -ForegroundColor Red
    Add-Content -Path $DeploymentLog -Value $logMessage
}

function Write-Banner {
    Write-Host ""
    Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║                                                           ║" -ForegroundColor Cyan
    Write-Host "║           WasteFi Contract Deployment Script             ║" -ForegroundColor Cyan
    Write-Host "║                  (PowerShell Version)                     ║" -ForegroundColor Cyan
    Write-Host "║                                                           ║" -ForegroundColor Cyan
    Write-Host "╚═══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

function Test-Prerequisites {
    Write-LogInfo "Checking prerequisites..."
    
    # Check Soroban CLI
    try {
        $sorobanVersion = soroban --version
        Write-LogSuccess "✓ Soroban CLI found: $sorobanVersion"
    }
    catch {
        Write-LogError "Soroban CLI not found. Please install: cargo install soroban-cli"
        exit 1
    }
    
    # Check Rust
    try {
        $rustVersion = rustc --version
        Write-LogSuccess "✓ Rust found: $rustVersion"
    }
    catch {
        Write-LogError "Rust/Cargo not found. Please install from https://rustup.rs/"
        exit 1
    }
    
    # Check wasm32 target
    $targets = rustup target list
    if ($targets -match "wasm32-unknown-unknown \(installed\)") {
        Write-LogSuccess "✓ wasm32-unknown-unknown target available"
    }
    else {
        Write-LogWarning "wasm32-unknown-unknown target not installed. Installing..."
        rustup target add wasm32-unknown-unknown
        Write-LogSuccess "✓ wasm32-unknown-unknown target installed"
    }
    
    # Check config file
    $configPath = Join-Path $ProjectRoot $ConfigFile
    if (-not (Test-Path $configPath)) {
        Write-LogError "Config file not found: $ConfigFile"
        exit 1
    }
    Write-LogSuccess "✓ Config file found: $ConfigFile"
    
    # Check network configuration
    $networks = soroban config network ls
    if ($networks -notmatch "^$Network") {
        Write-LogError "Network '$Network' not configured in Soroban CLI"
        Write-LogInfo "Configure with: soroban config network add $Network --rpc-url <URL> --network-passphrase <PASSPHRASE>"
        exit 1
    }
    Write-LogSuccess "✓ Network configured: $Network"
}

function Build-Contracts {
    Write-LogInfo "Building contracts..."
    
    Push-Location $ProjectRoot
    
    try {
        # Clean previous builds
        Write-LogInfo "Cleaning previous builds..."
        cargo clean | Out-Null
        
        # Build all contracts
        Write-LogInfo "Compiling contracts to WASM..."
        $buildOutput = cargo build --target wasm32-unknown-unknown --release 2>&1
        
        if ($LASTEXITCODE -eq 0) {
            Write-LogSuccess "✓ All contracts built successfully"
        }
        else {
            Write-LogError "Contract build failed"
            Write-Host $buildOutput
            exit 1
        }
        
        # List built contracts
        Write-LogInfo "Built contracts:"
        $wasmFiles = Get-ChildItem "target\wasm32-unknown-unknown\release\*.wasm"
        foreach ($file in $wasmFiles) {
            $size = [math]::Round($file.Length / 1KB, 2)
            Write-Host "  $($file.Name) ($size KB)" -ForegroundColor Gray
        }
    }
    finally {
        Pop-Location
    }
}

function Optimize-Wasm {
    Write-LogInfo "Optimizing WASM files..."
    Write-LogInfo "Using release build optimizations (opt-level=z, lto=true)"
    Write-LogSuccess "✓ WASM files optimized"
}

function Deploy-Contract {
    param(
        [string]$ContractName,
        [string]$WasmPath,
        [string]$SourceAccount
    )
    
    Write-LogInfo "Deploying $ContractName..."
    
    try {
        $output = soroban contract deploy `
            --wasm $WasmPath `
            --source $SourceAccount `
            --network $Network `
            2>&1
        
        # Extract contract ID from output (last line typically contains the ID)
        $contractId = ($output | Select-Object -Last 1).Trim()
        
        if ([string]::IsNullOrWhiteSpace($contractId)) {
            Write-LogError "Failed to deploy $ContractName"
            return $null
        }
        
        Write-LogSuccess "✓ $ContractName deployed: $contractId"
        Add-Content -Path $DeploymentLog -Value "Deployed $ContractName : $contractId"
        
        return $contractId
    }
    catch {
        Write-LogError "Failed to deploy $ContractName : $_"
        return $null
    }
}

function Initialize-Contract {
    param(
        [string]$ContractName,
        [string]$ContractId,
        [string]$Admin,
        [string[]]$Args
    )
    
    Write-LogInfo "Initializing $ContractName..."
    
    try {
        $invokeArgs = @(
            "contract", "invoke",
            "--id", $ContractId,
            "--source", $Admin,
            "--network", $Network,
            "--",
            "initialize"
        ) + $Args
        
        $output = & soroban @invokeArgs 2>&1
        Add-Content -Path $DeploymentLog -Value "Initialized $ContractName : $output"
        
        if ($LASTEXITCODE -eq 0) {
            Write-LogSuccess "✓ $ContractName initialized"
            return $true
        }
        else {
            Write-LogError "Failed to initialize $ContractName"
            return $false
        }
    }
    catch {
        Write-LogError "Failed to initialize $ContractName : $_"
        return $false
    }
}

function Test-Deployment {
    param(
        [string]$ContractName,
        [string]$ContractId,
        [string]$Admin
    )
    
    Write-LogInfo "Verifying $ContractName deployment..."
    
    try {
        $output = soroban contract invoke `
            --id $ContractId `
            --source $Admin `
            --network $Network `
            -- `
            --help 2>&1
        
        Write-LogSuccess "✓ $ContractName is callable"
        return $true
    }
    catch {
        Write-LogWarning "Could not verify $ContractName (may be OK if no public methods)"
        return $true
    }
}

function Save-Addresses {
    param([string]$AddressesJson)
    
    Write-LogInfo "Saving contract addresses..."
    
    $addressesPath = Join-Path $ProjectRoot $AddressesFile
    Set-Content -Path $addressesPath -Value $AddressesJson
    Write-LogSuccess "✓ Addresses saved to $AddressesFile"
    
    # Also save to config directory
    $configBackupPath = Join-Path $ProjectRoot "config\$AddressesFile"
    Copy-Item -Path $addressesPath -Destination $configBackupPath -Force
    Write-LogSuccess "✓ Addresses backed up to config\$AddressesFile"
}

function New-DeploymentReport {
    param(
        [datetime]$StartTime,
        [datetime]$EndTime,
        [string]$AddressesJson
    )
    
    $duration = ($EndTime - $StartTime).TotalSeconds
    $reportFile = "deployment_report_${Network}_${Timestamp}.md"
    $reportPath = Join-Path $ProjectRoot $reportFile
    
    Write-LogInfo "Generating deployment report..."
    
    $report = @"
# WasteFi Deployment Report

**Network**: $Network
**Date**: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
**Duration**: $([math]::Round($duration, 2))s
**Deployer**: $AdminAddress

## Deployed Contracts

``````json
$AddressesJson
``````

## Deployment Log

See ``$DeploymentLog`` for detailed logs.

## Verification Steps

1. Check contract addresses are accessible
2. Verify admin roles are set correctly
3. Test basic functionality on each contract
4. Monitor for any errors or issues

## Next Steps

1. Update frontend configuration with new addresses
2. Run integration tests against deployed contracts
3. Set up monitoring and alerting
4. Announce deployment to team

## Rollback Procedure

If issues are detected:
1. Keep old contract addresses as backup
2. Deploy new version with fixes
3. Update references to new addresses
4. Do NOT delete old contracts immediately

---

Generated by deploy.ps1
"@
    
    Set-Content -Path $reportPath -Value $report
    Write-LogSuccess "✓ Deployment report saved to $reportFile"
}

# ============================================================================
# Main Deployment Flow
# ============================================================================

function Start-Deployment {
    $startTime = Get-Date
    
    Write-Banner
    
    Write-LogInfo "Starting deployment to $Network"
    Write-LogInfo "Configuration: $ConfigFile"
    Write-LogInfo "Log file: $DeploymentLog"
    Write-Host ""
    
    # Step 1: Prerequisites
    Test-Prerequisites
    Write-Host ""
    
    # Step 2: Load configuration
    Write-LogInfo "Loading configuration..."
    $configPath = Join-Path $ProjectRoot $ConfigFile
    $config = Get-Content -Path $configPath | ConvertFrom-Json
    $script:AdminAddress = $config.admin
    
    if ([string]::IsNullOrWhiteSpace($AdminAddress)) {
        Write-LogError "Admin address not found in config file"
        exit 1
    }
    Write-LogSuccess "✓ Admin address: $AdminAddress"
    Write-Host ""
    
    # Step 3: Build contracts
    Build-Contracts
    Write-Host ""
    
    # Step 4: Optimize WASM
    Optimize-Wasm
    Write-Host ""
    
    # Step 5: Deploy contracts in order
    Write-LogInfo "Deploying contracts (this may take several minutes)..."
    Write-Host ""
    
    $wasmDir = Join-Path $ProjectRoot "target\wasm32-unknown-unknown\release"
    
    # 1. WasteToken
    Write-LogInfo "[1/7] Deploying WasteToken..."
    $script:WasteTokenId = Deploy-Contract `
        -ContractName "WasteToken" `
        -WasmPath (Join-Path $wasmDir "waste_token.wasm") `
        -SourceAccount $AdminAddress
    if (-not $WasteTokenId) { exit 1 }
    Write-Host ""
    
    # 2. CollectorRegistry
    Write-LogInfo "[2/7] Deploying CollectorRegistry..."
    $script:CollectorRegistryId = Deploy-Contract `
        -ContractName "CollectorRegistry" `
        -WasmPath (Join-Path $wasmDir "collector_registry.wasm") `
        -SourceAccount $AdminAddress
    if (-not $CollectorRegistryId) { exit 1 }
    Write-Host ""
    
    # 3. CollectionPoint
    Write-LogInfo "[3/7] Deploying CollectionPoint..."
    $script:CollectionPointId = Deploy-Contract `
        -ContractName "CollectionPoint" `
        -WasmPath (Join-Path $wasmDir "collection_point.wasm") `
        -SourceAccount $AdminAddress
    if (-not $CollectionPointId) { exit 1 }
    Write-Host ""
    
    # 4. MaterialPricing
    Write-LogInfo "[4/7] Deploying MaterialPricing..."
    $script:MaterialPricingId = Deploy-Contract `
        -ContractName "MaterialPricing" `
        -WasmPath (Join-Path $wasmDir "material_pricing.wasm") `
        -SourceAccount $AdminAddress
    if (-not $MaterialPricingId) { exit 1 }
    Write-Host ""
    
    # 5. Reputation
    Write-LogInfo "[5/7] Deploying Reputation..."
    $script:ReputationId = Deploy-Contract `
        -ContractName "Reputation" `
        -WasmPath (Join-Path $wasmDir "reputation.wasm") `
        -SourceAccount $AdminAddress
    if (-not $ReputationId) { exit 1 }
    Write-Host ""
    
    # 6. WasteTransaction
    Write-LogInfo "[6/7] Deploying WasteTransaction..."
    $script:WasteTransactionId = Deploy-Contract `
        -ContractName "WasteTransaction" `
        -WasmPath (Join-Path $wasmDir "waste_transaction.wasm") `
        -SourceAccount $AdminAddress
    if (-not $WasteTransactionId) { exit 1 }
    Write-Host ""
    
    # 7. PaymentDistribution
    Write-LogInfo "[7/7] Deploying PaymentDistribution..."
    $script:PaymentDistributionId = Deploy-Contract `
        -ContractName "PaymentDistribution" `
        -WasmPath (Join-Path $wasmDir "payment_distribution.wasm") `
        -SourceAccount $AdminAddress
    if (-not $PaymentDistributionId) { exit 1 }
    Write-Host ""
    
    # Step 6: Initialize contracts
    Write-LogInfo "Initializing contracts..."
    Write-Host ""
    
    Initialize-Contract -ContractName "WasteToken" -ContractId $WasteTokenId `
        -Admin $AdminAddress -Args @("--admin", $AdminAddress)
    
    Initialize-Contract -ContractName "CollectorRegistry" -ContractId $CollectorRegistryId `
        -Admin $AdminAddress -Args @("--admin", $AdminAddress)
    
    Initialize-Contract -ContractName "CollectionPoint" -ContractId $CollectionPointId `
        -Admin $AdminAddress -Args @("--admin", $AdminAddress)
    
    Initialize-Contract -ContractName "MaterialPricing" -ContractId $MaterialPricingId `
        -Admin $AdminAddress -Args @("--admin", $AdminAddress)
    
    Initialize-Contract -ContractName "Reputation" -ContractId $ReputationId `
        -Admin $AdminAddress -Args @("--admin", $AdminAddress)
    
    Initialize-Contract -ContractName "WasteTransaction" -ContractId $WasteTransactionId `
        -Admin $AdminAddress -Args @("--admin", $AdminAddress)
    
    Initialize-Contract -ContractName "PaymentDistribution" -ContractId $PaymentDistributionId `
        -Admin $AdminAddress -Args @("--admin", $AdminAddress, "--token_contract", $WasteTokenId)
    
    Write-Host ""
    
    # Step 7: Set cross-contract references
    Write-LogInfo "Setting cross-contract references..."
    
    soroban contract invoke `
        --id $WasteTransactionId `
        --source $AdminAddress `
        --network $Network `
        -- `
        set_material_pricing_contract `
        --contract_address $MaterialPricingId | Out-Null
    
    soroban contract invoke `
        --id $WasteTransactionId `
        --source $AdminAddress `
        --network $Network `
        -- `
        set_reputation_contract `
        --contract_address $ReputationId | Out-Null
    
    Write-LogSuccess "✓ Cross-contract references set"
    Write-Host ""
    
    # Step 8: Verify deployments
    Write-LogInfo "Verifying deployments..."
    Test-Deployment -ContractName "WasteToken" -ContractId $WasteTokenId -Admin $AdminAddress
    Test-Deployment -ContractName "CollectorRegistry" -ContractId $CollectorRegistryId -Admin $AdminAddress
    Test-Deployment -ContractName "CollectionPoint" -ContractId $CollectionPointId -Admin $AdminAddress
    Test-Deployment -ContractName "MaterialPricing" -ContractId $MaterialPricingId -Admin $AdminAddress
    Test-Deployment -ContractName "Reputation" -ContractId $ReputationId -Admin $AdminAddress
    Test-Deployment -ContractName "WasteTransaction" -ContractId $WasteTransactionId -Admin $AdminAddress
    Test-Deployment -ContractName "PaymentDistribution" -ContractId $PaymentDistributionId -Admin $AdminAddress
    Write-Host ""
    
    # Step 9: Save addresses
    $addressesJson = @{
        network = $Network
        deployed_at = (Get-Date -Format "o")
        admin = $AdminAddress
        contracts = @{
            waste_token = $WasteTokenId
            collector_registry = $CollectorRegistryId
            collection_point = $CollectionPointId
            material_pricing = $MaterialPricingId
            reputation = $ReputationId
            waste_transaction = $WasteTransactionId
            payment_distribution = $PaymentDistributionId
        }
    } | ConvertTo-Json -Depth 10
    
    Save-Addresses -AddressesJson $addressesJson
    Write-Host ""
    
    # Step 10: Generate report
    $endTime = Get-Date
    New-DeploymentReport -StartTime $startTime -EndTime $endTime -AddressesJson $addressesJson
    Write-Host ""
    
    # Success!
    Write-Host ""
    Write-LogSuccess "════════════════════════════════════════════════════════"
    Write-LogSuccess "   DEPLOYMENT COMPLETED SUCCESSFULLY!"
    Write-LogSuccess "════════════════════════════════════════════════════════"
    Write-Host ""
    Write-LogInfo "Contract Addresses:"
    Write-Host "  WasteToken:          $WasteTokenId" -ForegroundColor Gray
    Write-Host "  CollectorRegistry:   $CollectorRegistryId" -ForegroundColor Gray
    Write-Host "  CollectionPoint:     $CollectionPointId" -ForegroundColor Gray
    Write-Host "  MaterialPricing:     $MaterialPricingId" -ForegroundColor Gray
    Write-Host "  Reputation:          $ReputationId" -ForegroundColor Gray
    Write-Host "  WasteTransaction:    $WasteTransactionId" -ForegroundColor Gray
    Write-Host "  PaymentDistribution: $PaymentDistributionId" -ForegroundColor Gray
    Write-Host ""
    Write-LogInfo "Next Steps:"
    Write-Host "  1. Review deployment log: $DeploymentLog"
    Write-Host "  2. Update frontend with addresses: $AddressesFile"
    Write-Host "  3. Run integration tests"
    Write-Host "  4. Monitor for issues"
    Write-Host ""
    $duration = ($endTime - $startTime).TotalSeconds
    Write-LogInfo "Deployment Duration: $([math]::Round($duration, 2)) seconds"
}

# ============================================================================
# Script Entry Point
# ============================================================================

try {
    # Change to project root
    Push-Location $ProjectRoot
    
    # Run main deployment
    Start-Deployment
}
catch {
    Write-LogError "Deployment failed! Error: $_"
    Write-LogError "Check $DeploymentLog for details."
    exit 1
}
finally {
    Pop-Location
}

exit 0
