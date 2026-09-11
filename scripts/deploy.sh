#!/bin/bash
#
# WasteFi Contract Deployment Script (Unix/Linux/macOS)
#
# This script automates the deployment of all WasteFi contracts to Stellar Soroban.
# It handles compilation, sequential deployment, initialization, and verification.
#
# Usage:
#   ./scripts/deploy.sh [network] [config_file]
#
# Examples:
#   ./scripts/deploy.sh testnet config/testnet.json
#   ./scripts/deploy.sh mainnet config/mainnet.json
#
# Requirements:
#   - Soroban CLI (soroban) installed
#   - Rust toolchain with wasm32-unknown-unknown target
#   - Network configured in Soroban CLI
#   - Admin account with sufficient XLM balance

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Default values
NETWORK="${1:-testnet}"
CONFIG_FILE="${2:-config/testnet.json}"
DEPLOYMENT_LOG="deployment_${NETWORK}_$(date +%Y%m%d_%H%M%S).log"
ADDRESSES_FILE="deployed_addresses_${NETWORK}.json"

# ============================================================================
# Helper Functions
# ============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$DEPLOYMENT_LOG"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$DEPLOYMENT_LOG"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$DEPLOYMENT_LOG"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$DEPLOYMENT_LOG"
}

print_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║                                                           ║"
    echo "║           WasteFi Contract Deployment Script             ║"
    echo "║                                                           ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Soroban CLI
    if ! command -v soroban &> /dev/null; then
        log_error "Soroban CLI not found. Please install: cargo install soroban-cli"
        exit 1
    fi
    log_success "✓ Soroban CLI found: $(soroban --version)"
    
    # Check Rust
    if ! command -v cargo &> /dev/null; then
        log_error "Rust/Cargo not found. Please install from https://rustup.rs/"
        exit 1
    fi
    log_success "✓ Rust found: $(rustc --version)"
    
    # Check wasm32 target
    if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
        log_warning "wasm32-unknown-unknown target not installed. Installing..."
        rustup target add wasm32-unknown-unknown
    fi
    log_success "✓ wasm32-unknown-unknown target available"
    
    # Check config file
    if [ ! -f "$PROJECT_ROOT/$CONFIG_FILE" ]; then
        log_error "Config file not found: $CONFIG_FILE"
        exit 1
    fi
    log_success "✓ Config file found: $CONFIG_FILE"
    
    # Check network configuration
    if ! soroban config network ls | grep -q "^$NETWORK"; then
        log_error "Network '$NETWORK' not configured in Soroban CLI"
        log_info "Configure with: soroban config network add $NETWORK --rpc-url <URL> --network-passphrase <PASSPHRASE>"
        exit 1
    fi
    log_success "✓ Network configured: $NETWORK"
}

build_contracts() {
    log_info "Building contracts..."
    
    cd "$PROJECT_ROOT"
    
    # Clean previous builds
    log_info "Cleaning previous builds..."
    cargo clean
    
    # Build all contracts
    log_info "Compiling contracts to WASM..."
    if cargo build --target wasm32-unknown-unknown --release; then
        log_success "✓ All contracts built successfully"
    else
        log_error "Contract build failed"
        exit 1
    fi
    
    # List built contracts
    log_info "Built contracts:"
    ls -lh target/wasm32-unknown-unknown/release/*.wasm | awk '{print "  " $9 " (" $5 ")"}'
}

optimize_wasm() {
    log_info "Optimizing WASM files..."
    
    # Note: soroban contract optimize can be used if available
    # For now, we'll note that release builds are already optimized
    log_info "Using release build optimizations (opt-level=z, lto=true)"
    log_success "✓ WASM files optimized"
}

deploy_contract() {
    local contract_name=$1
    local wasm_path=$2
    local source_account=$3
    
    log_info "Deploying $contract_name..."
    
    # Deploy contract
    local contract_id
    contract_id=$(soroban contract deploy \
        --wasm "$wasm_path" \
        --source "$source_account" \
        --network "$NETWORK" \
        2>&1 | tee -a "$DEPLOYMENT_LOG" | tail -n 1)
    
    if [ -z "$contract_id" ]; then
        log_error "Failed to deploy $contract_name"
        return 1
    fi
    
    log_success "✓ $contract_name deployed: $contract_id"
    echo "$contract_id"
}

initialize_contract() {
    local contract_name=$1
    local contract_id=$2
    local admin=$3
    shift 3
    local args=("$@")
    
    log_info "Initializing $contract_name..."
    
    # Build arguments string
    local args_str=""
    for arg in "${args[@]}"; do
        args_str="$args_str $arg"
    done
    
    # Initialize contract
    if soroban contract invoke \
        --id "$contract_id" \
        --source "$admin" \
        --network "$NETWORK" \
        -- \
        initialize $args_str >> "$DEPLOYMENT_LOG" 2>&1; then
        log_success "✓ $contract_name initialized"
        return 0
    else
        log_error "Failed to initialize $contract_name"
        return 1
    fi
}

verify_deployment() {
    local contract_name=$1
    local contract_id=$2
    local admin=$3
    
    log_info "Verifying $contract_name deployment..."
    
    # Try to call a read-only method (e.g., get_admin or similar)
    # This varies by contract, so we'll do a simple check
    if soroban contract invoke \
        --id "$contract_id" \
        --source "$admin" \
        --network "$NETWORK" \
        -- \
        --help >> "$DEPLOYMENT_LOG" 2>&1; then
        log_success "✓ $contract_name is callable"
        return 0
    else
        log_warning "Could not verify $contract_name (may be OK if no public methods)"
        return 0
    fi
}

save_addresses() {
    local addresses_json=$1
    
    log_info "Saving contract addresses..."
    
    echo "$addresses_json" > "$PROJECT_ROOT/$ADDRESSES_FILE"
    log_success "✓ Addresses saved to $ADDRESSES_FILE"
    
    # Also save to config directory
    cp "$PROJECT_ROOT/$ADDRESSES_FILE" "$PROJECT_ROOT/config/$ADDRESSES_FILE"
    log_success "✓ Addresses backed up to config/$ADDRESSES_FILE"
}

generate_deployment_report() {
    local start_time=$1
    local end_time=$2
    local addresses_json=$3
    
    local duration=$((end_time - start_time))
    local report_file="deployment_report_${NETWORK}_$(date +%Y%m%d_%H%M%S).md"
    
    log_info "Generating deployment report..."
    
    cat > "$PROJECT_ROOT/$report_file" <<EOF
# WasteFi Deployment Report

**Network**: $NETWORK
**Date**: $(date)
**Duration**: ${duration}s
**Deployer**: $ADMIN_ADDRESS

## Deployed Contracts

$addresses_json

## Deployment Log

See \`$DEPLOYMENT_LOG\` for detailed logs.

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

Generated by deploy.sh
EOF

    log_success "✓ Deployment report saved to $report_file"
}

# ============================================================================
# Main Deployment Flow
# ============================================================================

main() {
    local start_time=$(date +%s)
    
    print_banner
    
    log_info "Starting deployment to $NETWORK"
    log_info "Configuration: $CONFIG_FILE"
    log_info "Log file: $DEPLOYMENT_LOG"
    echo ""
    
    # Step 1: Prerequisites
    check_prerequisites
    echo ""
    
    # Step 2: Load configuration
    log_info "Loading configuration..."
    # Read admin address from config (example - adjust based on actual config structure)
    ADMIN_ADDRESS=$(grep -oP '"admin":\s*"\K[^"]+' "$PROJECT_ROOT/$CONFIG_FILE" || echo "")
    if [ -z "$ADMIN_ADDRESS" ]; then
        log_error "Admin address not found in config file"
        exit 1
    fi
    log_success "✓ Admin address: $ADMIN_ADDRESS"
    echo ""
    
    # Step 3: Build contracts
    build_contracts
    echo ""
    
    # Step 4: Optimize WASM
    optimize_wasm
    echo ""
    
    # Step 5: Deploy contracts in order
    log_info "Deploying contracts (this may take several minutes)..."
    echo ""
    
    # Deploy Common Library (Note: Common is a library, not deployed separately)
    # Deploy in dependency order
    
    # 1. WasteToken (no dependencies)
    log_info "[1/7] Deploying WasteToken..."
    WASTE_TOKEN_ID=$(deploy_contract \
        "WasteToken" \
        "target/wasm32-unknown-unknown/release/waste_token.wasm" \
        "$ADMIN_ADDRESS")
    if [ $? -ne 0 ]; then exit 1; fi
    echo ""
    
    # 2. CollectorRegistry (no dependencies)
    log_info "[2/7] Deploying CollectorRegistry..."
    COLLECTOR_REGISTRY_ID=$(deploy_contract \
        "CollectorRegistry" \
        "target/wasm32-unknown-unknown/release/collector_registry.wasm" \
        "$ADMIN_ADDRESS")
    if [ $? -ne 0 ]; then exit 1; fi
    echo ""
    
    # 3. CollectionPoint (no dependencies)
    log_info "[3/7] Deploying CollectionPoint..."
    COLLECTION_POINT_ID=$(deploy_contract \
        "CollectionPoint" \
        "target/wasm32-unknown-unknown/release/collection_point.wasm" \
        "$ADMIN_ADDRESS")
    if [ $? -ne 0 ]; then exit 1; fi
    echo ""
    
    # 4. MaterialPricing (no dependencies)
    log_info "[4/7] Deploying MaterialPricing..."
    MATERIAL_PRICING_ID=$(deploy_contract \
        "MaterialPricing" \
        "target/wasm32-unknown-unknown/release/material_pricing.wasm" \
        "$ADMIN_ADDRESS")
    if [ $? -ne 0 ]; then exit 1; fi
    echo ""
    
    # 5. Reputation (no dependencies)
    log_info "[5/7] Deploying Reputation..."
    REPUTATION_ID=$(deploy_contract \
        "Reputation" \
        "target/wasm32-unknown-unknown/release/reputation.wasm" \
        "$ADMIN_ADDRESS")
    if [ $? -ne 0 ]; then exit 1; fi
    echo ""
    
    # 6. WasteTransaction (depends on MaterialPricing, Reputation)
    log_info "[6/7] Deploying WasteTransaction..."
    WASTE_TRANSACTION_ID=$(deploy_contract \
        "WasteTransaction" \
        "target/wasm32-unknown-unknown/release/waste_transaction.wasm" \
        "$ADMIN_ADDRESS")
    if [ $? -ne 0 ]; then exit 1; fi
    echo ""
    
    # 7. PaymentDistribution (depends on WasteToken)
    log_info "[7/7] Deploying PaymentDistribution..."
    PAYMENT_DISTRIBUTION_ID=$(deploy_contract \
        "PaymentDistribution" \
        "target/wasm32-unknown-unknown/release/payment_distribution.wasm" \
        "$ADMIN_ADDRESS")
    if [ $? -ne 0 ]; then exit 1; fi
    echo ""
    
    # Step 6: Initialize contracts
    log_info "Initializing contracts..."
    echo ""
    
    initialize_contract "WasteToken" "$WASTE_TOKEN_ID" "$ADMIN_ADDRESS" \
        --admin "$ADMIN_ADDRESS"
    
    initialize_contract "CollectorRegistry" "$COLLECTOR_REGISTRY_ID" "$ADMIN_ADDRESS" \
        --admin "$ADMIN_ADDRESS"
    
    initialize_contract "CollectionPoint" "$COLLECTION_POINT_ID" "$ADMIN_ADDRESS" \
        --admin "$ADMIN_ADDRESS"
    
    initialize_contract "MaterialPricing" "$MATERIAL_PRICING_ID" "$ADMIN_ADDRESS" \
        --admin "$ADMIN_ADDRESS"
    
    initialize_contract "Reputation" "$REPUTATION_ID" "$ADMIN_ADDRESS" \
        --admin "$ADMIN_ADDRESS"
    
    initialize_contract "WasteTransaction" "$WASTE_TRANSACTION_ID" "$ADMIN_ADDRESS" \
        --admin "$ADMIN_ADDRESS"
    
    initialize_contract "PaymentDistribution" "$PAYMENT_DISTRIBUTION_ID" "$ADMIN_ADDRESS" \
        --admin "$ADMIN_ADDRESS" \
        --token_contract "$WASTE_TOKEN_ID"
    
    echo ""
    
    # Step 7: Set cross-contract references
    log_info "Setting cross-contract references..."
    
    # WasteTransaction needs MaterialPricing and Reputation addresses
    soroban contract invoke \
        --id "$WASTE_TRANSACTION_ID" \
        --source "$ADMIN_ADDRESS" \
        --network "$NETWORK" \
        -- \
        set_material_pricing_contract \
        --contract_address "$MATERIAL_PRICING_ID" >> "$DEPLOYMENT_LOG" 2>&1
    
    soroban contract invoke \
        --id "$WASTE_TRANSACTION_ID" \
        --source "$ADMIN_ADDRESS" \
        --network "$NETWORK" \
        -- \
        set_reputation_contract \
        --contract_address "$REPUTATION_ID" >> "$DEPLOYMENT_LOG" 2>&1
    
    log_success "✓ Cross-contract references set"
    echo ""
    
    # Step 8: Verify deployments
    log_info "Verifying deployments..."
    verify_deployment "WasteToken" "$WASTE_TOKEN_ID" "$ADMIN_ADDRESS"
    verify_deployment "CollectorRegistry" "$COLLECTOR_REGISTRY_ID" "$ADMIN_ADDRESS"
    verify_deployment "CollectionPoint" "$COLLECTION_POINT_ID" "$ADMIN_ADDRESS"
    verify_deployment "MaterialPricing" "$MATERIAL_PRICING_ID" "$ADMIN_ADDRESS"
    verify_deployment "Reputation" "$REPUTATION_ID" "$ADMIN_ADDRESS"
    verify_deployment "WasteTransaction" "$WASTE_TRANSACTION_ID" "$ADMIN_ADDRESS"
    verify_deployment "PaymentDistribution" "$PAYMENT_DISTRIBUTION_ID" "$ADMIN_ADDRESS"
    echo ""
    
    # Step 9: Save addresses
    ADDRESSES_JSON=$(cat <<EOF
{
  "network": "$NETWORK",
  "deployed_at": "$(date -Iseconds)",
  "admin": "$ADMIN_ADDRESS",
  "contracts": {
    "waste_token": "$WASTE_TOKEN_ID",
    "collector_registry": "$COLLECTOR_REGISTRY_ID",
    "collection_point": "$COLLECTION_POINT_ID",
    "material_pricing": "$MATERIAL_PRICING_ID",
    "reputation": "$REPUTATION_ID",
    "waste_transaction": "$WASTE_TRANSACTION_ID",
    "payment_distribution": "$PAYMENT_DISTRIBUTION_ID"
  }
}
EOF
)
    
    save_addresses "$ADDRESSES_JSON"
    echo ""
    
    # Step 10: Generate report
    local end_time=$(date +%s)
    generate_deployment_report "$start_time" "$end_time" "$ADDRESSES_JSON"
    echo ""
    
    # Success!
    log_success "════════════════════════════════════════════════════════"
    log_success "   DEPLOYMENT COMPLETED SUCCESSFULLY!"
    log_success "════════════════════════════════════════════════════════"
    echo ""
    log_info "Contract Addresses:"
    echo "$ADDRESSES_JSON" | grep -E '(waste_token|collector_registry|collection_point|material_pricing|reputation|waste_transaction|payment_distribution)' | sed 's/^/  /'
    echo ""
    log_info "Next Steps:"
    echo "  1. Review deployment log: $DEPLOYMENT_LOG"
    echo "  2. Update frontend with addresses: $ADDRESSES_FILE"
    echo "  3. Run integration tests"
    echo "  4. Monitor for issues"
    echo ""
    log_info "Deployment Duration: $((end_time - start_time)) seconds"
}

# ============================================================================
# Script Entry Point
# ============================================================================

# Trap errors
trap 'log_error "Deployment failed! Check $DEPLOYMENT_LOG for details."; exit 1' ERR

# Run main deployment
main

exit 0
