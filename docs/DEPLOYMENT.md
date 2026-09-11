# WasteFi Deployment Guide

## Document Purpose

This guide provides step-by-step instructions for deploying the WasteFi smart contracts to Stellar Soroban networks (testnet and mainnet). It covers prerequisites, deployment procedures, verification steps, configuration management, and troubleshooting.

**Target Audience**: DevOps engineers, system administrators, deployment engineers  
**Prerequisites Knowledge**: Stellar basics, command-line proficiency, contract deployment concepts  
**Estimated Deployment Time**: 30-45 minutes (testnet), 60-90 minutes (mainnet)

---

## Table of Contents

1. [Prerequisites](#1-prerequisites)
2. [Environment Setup](#2-environment-setup)
3. [Pre-Deployment Checklist](#3-pre-deployment-checklist)
4. [Testnet Deployment](#4-testnet-deployment)
5. [Mainnet Deployment](#5-mainnet-deployment)
6. [Post-Deployment Verification](#6-post-deployment-verification)
7. [Configuration Management](#7-configuration-management)
8. [Rollback Procedures](#8-rollback-procedures)
9. [Troubleshooting](#9-troubleshooting)
10. [Emergency Procedures](#10-emergency-procedures)

---

## 1. Prerequisites

### 1.1 Required Software

#### Soroban CLI
```bash
# Install Soroban CLI
cargo install --locked soroban-cli --features opt

# Verify installation
soroban --version
# Required: v20.0.0 or higher
```

#### Rust Toolchain
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
# Required: 1.74.0 or higher

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Verify target
rustup target list | grep wasm32-unknown-unknown
# Should show: wasm32-unknown-unknown (installed)
```

#### Additional Tools
```bash
# jq (for JSON processing)
# Ubuntu/Debian
sudo apt-get install jq

# macOS
brew install jq

# Windows (via Chocolatey)
choco install jq

# Git (for version control)
git --version
# Required: 2.0.0 or higher
```

---

### 1.2 Network Accounts

#### Testnet Account Setup
```bash
# Generate a new keypair for testnet admin
soroban keys generate deployer --network testnet

# Get the public key
soroban keys address deployer

# Fund the account from Stellar Friendbot
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"

# Verify balance (should have 10,000 XLM)
soroban contract invoke \
  --network testnet \
  --source deployer \
  -- \
  balance \
  --id $(soroban keys address deployer)
```

#### Mainnet Account Setup
```bash
# Generate keypair (SECURE ENVIRONMENT REQUIRED)
soroban keys generate mainnet-admin --network mainnet

# CRITICAL: Back up the secret key immediately
soroban keys show mainnet-admin

# Store secret key in secure vault (e.g., AWS Secrets Manager, HashiCorp Vault)
# NEVER commit secret keys to version control

# Fund account with XLM (minimum 100 XLM recommended)
# Transfer from exchange or existing funded account
```

**⚠️ SECURITY WARNING**: 
- **NEVER** share or commit private keys
- Use hardware wallets for mainnet admin keys
- Enable multi-signature for mainnet (strongly recommended)
- Store backups in multiple secure locations

---

### 1.3 System Requirements

| Component | Testnet | Mainnet |
|-----------|---------|---------|
| **RAM** | 4 GB | 8 GB |
| **Storage** | 10 GB | 20 GB |
| **Network** | Stable internet | High-speed, redundant |
| **OS** | Linux/macOS/Windows | Linux (Ubuntu 20.04+ LTS) |
| **CPU** | 2 cores | 4+ cores |

---

### 1.4 Access Requirements

**For Testnet**:
- ✅ Testnet account with 10,000+ XLM
- ✅ Internet access to https://soroban-testnet.stellar.org

**For Mainnet**:
- ✅ Mainnet account with 100+ XLM (deployment costs ~10-20 XLM)
- ✅ Security audit completed and approved
- ✅ Legal/compliance clearance
- ✅ Multi-sig admin setup (strongly recommended)
- ✅ Monitoring infrastructure ready
- ✅ Emergency response team on standby
- ✅ Insurance coverage (optional but recommended)

---

## 2. Environment Setup

### 2.1 Clone Repository

```bash
# Clone the repository
git clone https://github.com/wastefi-africa/wastefi-contracts.git
cd wastefi-contracts

# Verify branch (use main for production)
git branch

# For specific release
git checkout tags/v1.0.0
```

---

### 2.2 Build Contracts

```bash
# Clean previous builds
cargo clean

# Build all contracts for production
cargo build --target wasm32-unknown-unknown --release

# Verify all WASM files generated
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# Expected output (7 contracts):
# collector_registry.wasm
# collection_point.wasm
# material_pricing.wasm
# payment_distribution.wasm
# reputation.wasm
# waste_token.wasm
# waste_transaction.wasm
```

**Build Verification**:
```bash
# Run workspace check
cargo check --workspace

# Run all tests
cargo test --workspace

# Check for warnings
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt --all --check
```

---

### 2.3 Configure Network

#### Testnet Configuration
```bash
# Add testnet network to Soroban CLI
soroban network add \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015" \
  testnet

# Verify network
soroban network ls
```

#### Mainnet Configuration
```bash
# Add mainnet network to Soroban CLI
soroban network add \
  --rpc-url https://soroban-mainnet.stellar.org:443 \
  --network-passphrase "Public Global Stellar Network ; September 2015" \
  mainnet

# Verify network
soroban network ls
```

---

### 2.4 Prepare Configuration File

#### For Testnet
```bash
# Copy testnet template
cp config/testnet.json config/testnet-deploy.json

# Edit configuration
nano config/testnet-deploy.json

# Update the following fields:
# - "admin": "<YOUR_TESTNET_ADMIN_ADDRESS>"
# - "operator": "<YOUR_OPERATOR_ADDRESS>"
# - Verify all other settings
```

#### For Mainnet
```bash
# Copy mainnet template
cp config/mainnet.template.json config/mainnet.json

# Edit configuration (CAREFULLY!)
nano config/mainnet.json

# Replace ALL "REPLACE_WITH_*" placeholders:
# - REPLACE_WITH_ACTUAL_ADMIN_ADDRESS
# - REPLACE_WITH_ACTUAL_OPERATOR_ADDRESS
# - REPLACE_WITH_ACTUAL_SECURITY_EMAIL
# - REPLACE_WITH_ACTUAL_SECURITY_PHONE
# - REPLACE_WITH_ACTUAL_WEBHOOK_URL
# - REPLACE_WITH_ACTUAL_METRICS_ENDPOINT

# Complete deployment checklist in config file
```

**⚠️ CRITICAL**: Never commit `mainnet.json` with real addresses/keys to git!

```bash
# Add to .gitignore (already done in template)
echo "config/mainnet.json" >> .gitignore
echo "config/testnet-deploy.json" >> .gitignore
```

---

## 3. Pre-Deployment Checklist

### 3.1 Testnet Checklist

- [ ] **Build**: All contracts built successfully (`cargo build --release`)
- [ ] **Tests**: All tests passing (`cargo test --workspace`)
- [ ] **Account**: Testnet account funded with 10,000+ XLM
- [ ] **Config**: `testnet-deploy.json` configured with correct admin address
- [ ] **Network**: Soroban CLI configured for testnet
- [ ] **Scripts**: Deployment script is executable (`chmod +x scripts/deploy.sh`)

### 3.2 Mainnet Checklist

- [ ] **Security Audit**: Professional audit completed, all Critical/High issues resolved
- [ ] **Testnet**: Successful testnet deployment and 1+ week of testing
- [ ] **Tests**: All test suites passing (unit, integration, stress, security, chaos)
- [ ] **Coverage**: Code coverage >85%
- [ ] **Account**: Mainnet account funded with 100+ XLM
- [ ] **Multi-Sig**: Multi-signature admin setup (STRONGLY RECOMMENDED)
- [ ] **Config**: `mainnet.json` fully configured and validated
- [ ] **Monitoring**: Monitoring infrastructure deployed and tested
- [ ] **Alerts**: Alert webhooks configured and tested
- [ ] **Emergency**: Emergency response team briefed and on standby
- [ ] **Documentation**: All documentation complete and reviewed
- [ ] **Legal**: Legal/compliance review completed
- [ ] **Insurance**: Insurance coverage in place (optional)
- [ ] **Backup**: Admin keys backed up in secure vault (multiple locations)
- [ ] **Rollback**: Rollback plan documented and rehearsed
- [ ] **Budget**: Operations budget allocated (monitoring, maintenance)
- [ ] **Launch Plan**: Marketing/communication plan ready

---

## 4. Testnet Deployment

### 4.1 Automated Deployment (Recommended)

#### Unix/Linux/macOS
```bash
# Make script executable
chmod +x scripts/deploy.sh

# Run deployment
./scripts/deploy.sh testnet config/testnet-deploy.json

# Monitor progress (watch logs)
tail -f deployment_testnet_*.log
```

#### Windows (PowerShell)
```powershell
# Run deployment
.\scripts\deploy.ps1 -Network testnet -ConfigFile config\testnet-deploy.json

# Monitor progress
Get-Content deployment_testnet_*.log -Wait
```

**Deployment Duration**: 15-30 minutes

---

### 4.2 Manual Deployment (Step-by-Step)

If automated script fails or for learning purposes, follow manual steps:

#### Step 1: Deploy WasteToken
```bash
# Deploy contract
WASTE_TOKEN_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/waste_token.wasm \
  --source deployer \
  --network testnet)

echo "WasteToken deployed: $WASTE_TOKEN_ID"

# Initialize contract
soroban contract invoke \
  --id $WASTE_TOKEN_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin $(soroban keys address deployer) \
  --name "WasteFi Token Testnet" \
  --symbol "WASTE-TEST" \
  --decimals 7
```

#### Step 2: Deploy CollectorRegistry
```bash
COLLECTOR_REGISTRY_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/collector_registry.wasm \
  --source deployer \
  --network testnet)

echo "CollectorRegistry deployed: $COLLECTOR_REGISTRY_ID"

# Initialize
soroban contract invoke \
  --id $COLLECTOR_REGISTRY_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin $(soroban keys address deployer)
```

#### Step 3: Deploy CollectionPoint
```bash
COLLECTION_POINT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/collection_point.wasm \
  --source deployer \
  --network testnet)

echo "CollectionPoint deployed: $COLLECTION_POINT_ID"

# Initialize
soroban contract invoke \
  --id $COLLECTION_POINT_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin $(soroban keys address deployer)
```

#### Step 4: Deploy MaterialPricing
```bash
MATERIAL_PRICING_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/material_pricing.wasm \
  --source deployer \
  --network testnet)

echo "MaterialPricing deployed: $MATERIAL_PRICING_ID"

# Initialize with default prices
soroban contract invoke \
  --id $MATERIAL_PRICING_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin $(soroban keys address deployer)
```

#### Step 5: Deploy Reputation
```bash
REPUTATION_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/reputation.wasm \
  --source deployer \
  --network testnet)

echo "Reputation deployed: $REPUTATION_ID"

# Initialize
soroban contract invoke \
  --id $REPUTATION_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin $(soroban keys address deployer) \
  --initial_score 500
```

#### Step 6: Deploy WasteTransaction
```bash
WASTE_TRANSACTION_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/waste_transaction.wasm \
  --source deployer \
  --network testnet)

echo "WasteTransaction deployed: $WASTE_TRANSACTION_ID"

# Initialize
soroban contract invoke \
  --id $WASTE_TRANSACTION_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin $(soroban keys address deployer)
```

#### Step 7: Deploy PaymentDistribution
```bash
PAYMENT_DISTRIBUTION_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/payment_distribution.wasm \
  --source deployer \
  --network testnet)

echo "PaymentDistribution deployed: $PAYMENT_DISTRIBUTION_ID"

# Initialize
soroban contract invoke \
  --id $PAYMENT_DISTRIBUTION_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin $(soroban keys address deployer)
```

#### Step 8: Set Cross-Contract References
```bash
# Set WasteToken address in PaymentDistribution
soroban contract invoke \
  --id $PAYMENT_DISTRIBUTION_ID \
  --source deployer \
  --network testnet \
  -- \
  set_token_contract \
  --token_address $WASTE_TOKEN_ID

# Set MaterialPricing address in WasteTransaction
soroban contract invoke \
  --id $WASTE_TRANSACTION_ID \
  --source deployer \
  --network testnet \
  -- \
  set_pricing_contract \
  --pricing_address $MATERIAL_PRICING_ID

# Set Reputation address in WasteTransaction
soroban contract invoke \
  --id $WASTE_TRANSACTION_ID \
  --source deployer \
  --network testnet \
  -- \
  set_reputation_contract \
  --reputation_address $REPUTATION_ID

# Set PaymentDistribution address in WasteTransaction
soroban contract invoke \
  --id $WASTE_TRANSACTION_ID \
  --source deployer \
  --network testnet \
  -- \
  set_payment_contract \
  --payment_address $PAYMENT_DISTRIBUTION_ID
```

#### Step 9: Record Addresses
```bash
# Create addresses file
cat > deployed_addresses_testnet.json <<EOF
{
  "network": "testnet",
  "deployment_date": "$(date -Iseconds)",
  "deployer": "$(soroban keys address deployer)",
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

# Backup addresses file
cp deployed_addresses_testnet.json deployed_addresses_testnet_$(date +%Y%m%d_%H%M%S).json
```

---

### 4.3 Deployment Output

After successful deployment, you should see:

```
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║           WasteFi Contract Deployment Script             ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝

[INFO] Checking prerequisites...
[SUCCESS] ✓ Soroban CLI found: soroban 20.0.0
[SUCCESS] ✓ Rust found: rustc 1.74.0
[SUCCESS] ✓ wasm32-unknown-unknown target available
[SUCCESS] ✓ Config file found: config/testnet-deploy.json

[INFO] Building contracts...
[SUCCESS] ✓ All contracts built successfully

[INFO] Deploying contracts to testnet...
[SUCCESS] ✓ WasteToken deployed: CCXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
[SUCCESS] ✓ CollectorRegistry deployed: CDXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
[SUCCESS] ✓ CollectionPoint deployed: CEXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
[SUCCESS] ✓ MaterialPricing deployed: CFXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
[SUCCESS] ✓ Reputation deployed: CGXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
[SUCCESS] ✓ WasteTransaction deployed: CHXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
[SUCCESS] ✓ PaymentDistribution deployed: CIXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

[INFO] Initializing contracts...
[SUCCESS] ✓ All contracts initialized

[INFO] Setting cross-contract references...
[SUCCESS] ✓ Cross-contract references set

[INFO] Verifying deployment...
[SUCCESS] ✓ All contracts verified

[SUCCESS] Deployment completed successfully!
[INFO] Contract addresses saved to: deployed_addresses_testnet.json
[INFO] Deployment report saved to: deployment_report_testnet_20260911_143022.md
[INFO] Total deployment time: 18m 34s
```

---

## 5. Mainnet Deployment

### 5.1 Final Pre-Deployment Review

**⚠️ CRITICAL REVIEW POINTS**:

1. **Security Audit**: All findings resolved (especially Critical/High)
2. **Testnet Testing**: Minimum 1 week of continuous testing
3. **Known Issues**: All critical issues fixed (see `SECURITY_CONSIDERATIONS.md`)
4. **Multi-Sig**: Multi-signature admin implemented
5. **Monitoring**: Full monitoring stack deployed and tested
6. **Emergency**: Response team briefed and available 24/7
7. **Backups**: Admin keys backed up in 3+ secure locations
8. **Budget**: Operations budget (monitoring, gas, maintenance) allocated
9. **Legal**: All legal/regulatory requirements met
10. **Insurance**: Coverage for smart contract vulnerabilities (optional)

**⚠️ STOP**: If ANY critical item is incomplete, DO NOT proceed with mainnet deployment!

---

### 5.2 Mainnet Deployment Execution

#### Pre-Flight Checks
```bash
# 1. Verify mainnet config
cat config/mainnet.json | jq .

# 2. Check for placeholder values
grep -i "REPLACE_WITH" config/mainnet.json
# Should return EMPTY (no matches)

# 3. Verify admin account balance
soroban contract invoke \
  --network mainnet \
  --source mainnet-admin \
  -- \
  balance \
  --id $(soroban keys address mainnet-admin)
# Should show 100+ XLM

# 4. Verify build is clean
cargo clean
cargo build --target wasm32-unknown-unknown --release
cargo test --workspace

# 5. Create deployment backup
mkdir -p backups
cp -r . backups/pre-mainnet-deploy-$(date +%Y%m%d_%H%M%S)
```

#### Execute Deployment
```bash
# Unix/Linux/macOS
./scripts/deploy.sh mainnet config/mainnet.json

# Windows PowerShell
.\scripts\deploy.ps1 -Network mainnet -ConfigFile config\mainnet.json
```

**Deployment Duration**: 20-40 minutes

**⚠️ MONITORING**: Have your entire team monitoring during deployment!

---

### 5.3 Mainnet Post-Deployment

#### Immediate Actions (Within 1 Hour)
```bash
# 1. Verify all contracts deployed
cat deployed_addresses_mainnet.json

# 2. Run post-deployment verification (see Section 6)
# 3. Enable monitoring alerts
# 4. Notify stakeholders
# 5. Update public documentation with contract addresses
# 6. Test critical user workflows (registration, transaction, payment)
```

#### First 24 Hours
- Monitor all contract interactions
- Check for anomalous patterns
- Verify payment calculations
- Monitor fraud detection alerts
- Check emergency system responsiveness

#### First Week
- Daily monitoring reviews
- User feedback collection
- Performance analysis
- Gas usage optimization if needed
- Gradual scaling of user onboarding

---

## 6. Post-Deployment Verification

### 6.1 Contract Existence Check

```bash
# Verify all 7 contracts are deployed
for contract in waste_token collector_registry collection_point material_pricing reputation waste_transaction payment_distribution; do
  CONTRACT_ID=$(jq -r ".contracts.$contract" deployed_addresses_testnet.json)
  echo "Checking $contract: $CONTRACT_ID"
  
  soroban contract invoke \
    --id $CONTRACT_ID \
    --source deployer \
    --network testnet \
    -- \
    version || echo "ERROR: $contract not responding"
done
```

---

### 6.2 Admin Verification

```bash
# Check admin for each contract
for contract in waste_token collector_registry collection_point material_pricing reputation waste_transaction payment_distribution; do
  CONTRACT_ID=$(jq -r ".contracts.$contract" deployed_addresses_testnet.json)
  ADMIN=$(soroban contract invoke \
    --id $CONTRACT_ID \
    --source deployer \
    --network testnet \
    -- \
    get_admin)
  
  echo "$contract admin: $ADMIN"
  
  # Verify admin matches expected
  EXPECTED_ADMIN=$(soroban keys address deployer)
  if [ "$ADMIN" != "$EXPECTED_ADMIN" ]; then
    echo "ERROR: Admin mismatch for $contract!"
  fi
done
```

---

### 6.3 Initialization Verification

```bash
# Test WasteToken
WASTE_TOKEN_ID=$(jq -r ".contracts.waste_token" deployed_addresses_testnet.json)

soroban contract invoke \
  --id $WASTE_TOKEN_ID \
  --source deployer \
  --network testnet \
  -- \
  name
# Expected: "WasteFi Token Testnet" (or mainnet variant)

soroban contract invoke \
  --id $WASTE_TOKEN_ID \
  --source deployer \
  --network testnet \
  -- \
  symbol
# Expected: "WASTE-TEST" (or mainnet variant)

# Test Reputation initial score
REPUTATION_ID=$(jq -r ".contracts.reputation" deployed_addresses_testnet.json)

soroban contract invoke \
  --id $REPUTATION_ID \
  --source deployer \
  --network testnet \
  -- \
  get_initial_score
# Expected: 500
```

---

### 6.4 Cross-Contract Reference Verification

```bash
# Check WasteTransaction references
WASTE_TRANSACTION_ID=$(jq -r ".contracts.waste_transaction" deployed_addresses_testnet.json)

# Verify pricing contract
soroban contract invoke \
  --id $WASTE_TRANSACTION_ID \
  --source deployer \
  --network testnet \
  -- \
  get_pricing_contract
# Expected: MaterialPricing contract ID

# Verify reputation contract
soroban contract invoke \
  --id $WASTE_TRANSACTION_ID \
  --source deployer \
  --network testnet \
  -- \
  get_reputation_contract
# Expected: Reputation contract ID

# Verify payment contract
soroban contract invoke \
  --id $WASTE_TRANSACTION_ID \
  --source deployer \
  --network testnet \
  -- \
  get_payment_contract
# Expected: PaymentDistribution contract ID

# Check PaymentDistribution token reference
PAYMENT_DISTRIBUTION_ID=$(jq -r ".contracts.payment_distribution" deployed_addresses_testnet.json)

soroban contract invoke \
  --id $PAYMENT_DISTRIBUTION_ID \
  --source deployer \
  --network testnet \
  -- \
  get_token_contract
# Expected: WasteToken contract ID
```

---

### 6.5 Functional Test (End-to-End)

```bash
#!/bin/bash
# e2e_verification.sh - Quick functional test

# Load contract addresses
COLLECTOR_REGISTRY=$(jq -r ".contracts.collector_registry" deployed_addresses_testnet.json)
WASTE_TRANSACTION=$(jq -r ".contracts.waste_transaction" deployed_addresses_testnet.json)
PAYMENT_DISTRIBUTION=$(jq -r ".contracts.payment_distribution" deployed_addresses_testnet.json)
REPUTATION=$(jq -r ".contracts.reputation" deployed_addresses_testnet.json)

# Test 1: Register a collector
echo "Test 1: Registering collector..."
soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --source deployer \
  --network testnet \
  -- \
  register \
  --collector $(soroban keys address deployer) \
  --name "Test Collector" \
  --location "Test Location" \
  --contact "test@example.com"

# Test 2: Check collector status
echo "Test 2: Checking collector status..."
STATUS=$(soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --source deployer \
  --network testnet \
  -- \
  get_status \
  --collector $(soroban keys address deployer))

echo "Collector status: $STATUS"
# Expected: "Active" or equivalent

# Test 3: Check initial reputation
echo "Test 3: Checking reputation..."
REP_SCORE=$(soroban contract invoke \
  --id $REPUTATION \
  --source deployer \
  --network testnet \
  -- \
  get_score \
  --user $(soroban keys address deployer))

echo "Reputation score: $REP_SCORE"
# Expected: 500 (initial score)

echo "✓ All functional tests passed!"
```

---

## 7. Configuration Management

### 7.1 Configuration Files

**Structure**:
```
config/
├── testnet.json              # Testnet template (committed to git)
├── testnet-deploy.json       # Your testnet config (gitignored)
├── mainnet.template.json     # Mainnet template (committed to git)
├── mainnet.json              # Your mainnet config (gitignored)
└── README.md                 # Configuration documentation
```

**Security Rules**:
- ✅ Commit templates to git
- ❌ NEVER commit files with real addresses/keys
- ✅ Use `.gitignore` to exclude deployment configs
- ✅ Store mainnet configs in secure vault only

---

### 7.2 Configuration Validation

```bash
# Validate JSON syntax
jq . config/testnet-deploy.json

# Check for required fields
jq '.network, .admin, .contracts' config/testnet-deploy.json

# Verify no placeholder values (for mainnet)
grep -i "REPLACE_WITH" config/mainnet.json || echo "✓ No placeholders found"

# Validate Stellar addresses
ADMIN=$(jq -r '.admin' config/testnet-deploy.json)
if [[ $ADMIN =~ ^G[A-Z0-9]{55}$ ]]; then
  echo "✓ Admin address format valid"
else
  echo "ERROR: Invalid admin address format"
fi
```

---

### 7.3 Configuration Updates

**To update configuration after deployment**:

```bash
# 1. Update local config file
nano config/testnet-deploy.json

# 2. Validate changes
jq . config/testnet-deploy.json

# 3. Apply changes to deployed contracts (admin only)
# Example: Update price in MaterialPricing
MATERIAL_PRICING=$(jq -r ".contracts.material_pricing" deployed_addresses_testnet.json)

soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source deployer \
  --network testnet \
  -- \
  update_price \
  --material "plastic" \
  --price 120

# 4. Verify update
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source deployer \
  --network testnet \
  -- \
  get_price \
  --material "plastic"
# Expected: 120
```

---

## 8. Rollback Procedures

### 8.1 Rollback Scenarios

| Scenario | Severity | Rollback Method | Recovery Time |
|----------|----------|-----------------|---------------|
| Contract bug found post-deployment | CRITICAL | Emergency pause + contract upgrade | 2-4 hours |
| Wrong initialization parameters | HIGH | Redeploy affected contract | 1-2 hours |
| Cross-contract reference error | HIGH | Update references via admin function | 30 mins |
| Configuration error | MEDIUM | Update config via admin functions | 15 mins |
| Deployment script failure mid-deployment | HIGH | Complete deployment or redeploy | 1-2 hours |

---

### 8.2 Emergency Pause (Critical Bug Found)

```bash
# Activate emergency mode for affected contract
CONTRACT_ID="<AFFECTED_CONTRACT_ID>"

soroban contract invoke \
  --id $CONTRACT_ID \
  --source mainnet-admin \
  --network mainnet \
  -- \
  set_emergency_level \
  --level "Shutdown"

# Verify emergency mode active
soroban contract invoke \
  --id $CONTRACT_ID \
  --source mainnet-admin \
  --network mainnet \
  -- \
  get_emergency_level
# Expected: "Shutdown"

# Notify all stakeholders immediately
# Begin incident response procedures (see INCIDENT_RESPONSE.md)
```

---

### 8.3 Contract Upgrade (Bug Fix)

```bash
# 1. Build new contract version
cargo build --target wasm32-unknown-unknown --release

# 2. Deploy new version
NEW_CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/<contract>.wasm \
  --source mainnet-admin \
  --network mainnet)

# 3. Initialize new contract with same parameters
soroban contract invoke \
  --id $NEW_CONTRACT_ID \
  --source mainnet-admin \
  --network mainnet \
  -- \
  initialize \
  --admin $(soroban keys address mainnet-admin)

# 4. Migrate data (if needed - contract-specific)
# See UPGRADE_GUIDE.md for data migration procedures

# 5. Update cross-contract references
# Update all contracts that reference the old contract

# 6. Deactivate old contract
soroban contract invoke \
  --id $OLD_CONTRACT_ID \
  --source mainnet-admin \
  --network mainnet \
  -- \
  set_emergency_level \
  --level "Shutdown"

# 7. Verify new contract operational
# Run post-deployment verification (Section 6)

# 8. Update address registry
jq ".contracts.<contract_name> = \"$NEW_CONTRACT_ID\"" \
  deployed_addresses_mainnet.json > tmp.json && \
  mv tmp.json deployed_addresses_mainnet.json

# 9. Notify users of upgrade
```

---

### 8.4 Rollback from Failed Deployment

```bash
# If deployment script fails mid-deployment

# Option 1: Complete deployment manually
# - Identify which contracts deployed successfully (check logs)
# - Continue from Step N in Section 4.2 Manual Deployment

# Option 2: Redeploy from scratch (if early failure)
# - Clean up any partially deployed contracts
# - Start fresh deployment

# Option 3: Emergency rollback (mainnet only)
# - If critical issue discovered during deployment
# - Pause all deployed contracts immediately
# - Investigate issue
# - Either complete deployment with fix or full rollback
```

---

## 9. Troubleshooting

### 9.1 Common Issues

#### Issue: "Insufficient balance" error during deployment

**Symptoms**:
```
Error: Account has insufficient balance
```

**Solution**:
```bash
# Check account balance
soroban contract invoke \
  --network testnet \
  --source deployer \
  -- \
  balance \
  --id $(soroban keys address deployer)

# For testnet: Request more XLM from friendbot
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"

# For mainnet: Transfer more XLM to admin account (min 100 XLM)
```

---

#### Issue: Contract deployment times out

**Symptoms**:
```
Error: Transaction submission timed out
```

**Solutions**:
1. **Network congestion**: Wait 5-10 minutes and retry
2. **RPC issues**: Try alternative RPC endpoint
3. **Increase timeout**: Add `--timeout 120` flag to soroban commands

```bash
# Retry with increased timeout
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/<contract>.wasm \
  --source deployer \
  --network testnet \
  --timeout 120
```

---

#### Issue: "Contract already initialized" error

**Symptoms**:
```
Error: Contract is already initialized
```

**Explanation**: Contract initialization can only be called once.

**Solution**:
- If initialization was successful: Continue to next step
- If initialization failed: Deploy new instance of contract

---

#### Issue: Build fails with "target not found"

**Symptoms**:
```
Error: target 'wasm32-unknown-unknown' not found
```

**Solution**:
```bash
# Install wasm32 target
rustup target add wasm32-unknown-unknown

# Verify installation
rustup target list | grep wasm32-unknown-unknown
```

---

#### Issue: Cross-contract call fails

**Symptoms**:
```
Error: Contract not found or invalid address
```

**Solutions**:
1. **Verify contract address**: Check `deployed_addresses_<network>.json`
2. **Verify network**: Ensure using correct network (testnet vs mainnet)
3. **Check contract deployed**: Run existence check (Section 6.1)

```bash
# Verify contract exists
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- \
  version
```

---

#### Issue: Admin operation unauthorized

**Symptoms**:
```
Error: Unauthorized - admin only
```

**Solutions**:
```bash
# Check who is admin
soroban contract invoke \
  --id $CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  get_admin

# Ensure using correct source account
# For admin operations, use: --source <admin_key_name>
```

---

### 9.2 Diagnostic Commands

```bash
# Check Soroban CLI version
soroban --version

# Check Rust version
rustc --version

# List all Soroban networks
soroban network ls

# Check account balance
soroban contract invoke \
  --network testnet \
  --source deployer \
  -- \
  balance \
  --id $(soroban keys address deployer)

# View deployment logs
tail -n 100 deployment_testnet_*.log

# Check contract WASM size
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# Verify contract bytecode
wasm-opt --version  # If available
```

---

### 9.3 Getting Help

**Resources**:
- **Stellar Soroban Docs**: https://soroban.stellar.org/docs
- **Soroban Discord**: https://discord.gg/stellar
- **GitHub Issues**: https://github.com/wastefi-africa/wastefi-contracts/issues
- **Security Issues**: security@wastefi.io (DO NOT publicly disclose)

**When Reporting Issues**:
1. Include Soroban CLI version
2. Include deployment logs (redact private keys!)
3. Include error messages (full stack trace)
4. Include network (testnet/mainnet)
5. Include steps to reproduce

---

## 10. Emergency Procedures

### 10.1 Emergency Contacts

| Role | Contact | Response Time |
|------|---------|---------------|
| **Security Lead** | security@wastefi.io | < 15 minutes |
| **DevOps Lead** | devops@wastefi.io | < 30 minutes |
| **CTO** | cto@wastefi.io | < 1 hour |
| **Emergency Hotline** | +1-XXX-XXX-XXXX | 24/7 |

---

### 10.2 Emergency Response Levels

| Level | Trigger | Response | Activation |
|-------|---------|----------|------------|
| **P0 - Critical** | Fund loss risk, contract exploit | Emergency shutdown, all hands | Immediate |
| **P1 - High** | Contract malfunction, data corruption | Emergency pause, investigation | < 15 mins |
| **P2 - Medium** | Performance degradation, minor bugs | Monitoring, planned fix | < 2 hours |
| **P3 - Low** | Non-critical issues | Normal process | Next business day |

**See `INCIDENT_RESPONSE.md` for detailed procedures.**

---

### 10.3 Emergency Shutdown

```bash
# CRITICAL: Only use in P0/P1 incidents

# Shutdown all contracts immediately
for contract in waste_token collector_registry collection_point material_pricing reputation waste_transaction payment_distribution; do
  CONTRACT_ID=$(jq -r ".contracts.$contract" deployed_addresses_mainnet.json)
  
  echo "Shutting down $contract..."
  soroban contract invoke \
    --id $CONTRACT_ID \
    --source mainnet-admin \
    --network mainnet \
    -- \
    set_emergency_level \
    --level "Shutdown"
done

# Verify all contracts in shutdown mode
for contract in waste_token collector_registry collection_point material_pricing reputation waste_transaction payment_distribution; do
  CONTRACT_ID=$(jq -r ".contracts.$contract" deployed_addresses_mainnet.json)
  LEVEL=$(soroban contract invoke \
    --id $CONTRACT_ID \
    --source mainnet-admin \
    --network mainnet \
    -- \
    get_emergency_level)
  
  echo "$contract: $LEVEL"
done

# Notify all stakeholders immediately
# Begin post-incident analysis
```

---

### 10.4 Emergency Withdrawal (Fund Recovery)

```bash
# If critical vulnerability found and funds at risk

# Withdraw all funds from affected contract to safe address
SAFE_ADDRESS="<COLD_WALLET_ADDRESS>"
CONTRACT_ID="<AFFECTED_CONTRACT_ID>"

soroban contract invoke \
  --id $CONTRACT_ID \
  --source mainnet-admin \
  --network mainnet \
  -- \
  emergency_withdraw \
  --destination $SAFE_ADDRESS

# Verify withdrawal
soroban contract invoke \
  --network mainnet \
  --source mainnet-admin \
  -- \
  balance \
  --id $SAFE_ADDRESS

# Pause contract
soroban contract invoke \
  --id $CONTRACT_ID \
  --source mainnet-admin \
  --network mainnet \
  -- \
  set_emergency_level \
  --level "Shutdown"
```

---

## Appendix A: Deployment Checklist (Printable)

### Pre-Deployment
- [ ] Security audit complete (mainnet only)
- [ ] All tests passing
- [ ] Code coverage >85%
- [ ] Contracts built successfully
- [ ] Admin account funded
- [ ] Configuration file prepared
- [ ] Network configured in Soroban CLI
- [ ] Deployment script executable
- [ ] Monitoring infrastructure ready (mainnet only)
- [ ] Emergency response team briefed (mainnet only)

### During Deployment
- [ ] Run deployment script
- [ ] Monitor deployment logs
- [ ] Verify each contract deploys successfully
- [ ] Verify initialization succeeds
- [ ] Verify cross-contract references set
- [ ] Record all contract addresses
- [ ] Backup addresses file

### Post-Deployment
- [ ] Run existence checks
- [ ] Verify admin addresses
- [ ] Verify initialization parameters
- [ ] Verify cross-contract references
- [ ] Run functional tests
- [ ] Enable monitoring alerts (mainnet only)
- [ ] Update documentation with addresses
- [ ] Notify stakeholders
- [ ] Monitor for first 24 hours

### Emergency Preparedness
- [ ] Emergency procedures documented
- [ ] Emergency contacts available 24/7
- [ ] Rollback plan tested
- [ ] Backup admin keys in secure vault (3+ locations)
- [ ] Insurance coverage active (optional, mainnet)

---

## Appendix B: Network Information

### Testnet
- **RPC URL**: https://soroban-testnet.stellar.org:443
- **Network Passphrase**: "Test SDF Network ; September 2015"
- **Friendbot**: https://friendbot.stellar.org
- **Explorer**: https://stellar.expert/explorer/testnet
- **Status**: https://status.stellar.org

### Mainnet
- **RPC URL**: https://soroban-mainnet.stellar.org:443
- **Network Passphrase**: "Public Global Stellar Network ; September 2015"
- **Explorer**: https://stellar.expert/explorer/public
- **Status**: https://status.stellar.org

---

## Appendix C: Estimated Costs

### Testnet (Free)
- All operations free via Friendbot funding

### Mainnet
| Operation | Cost (XLM) | Notes |
|-----------|------------|-------|
| Contract Deployment (each) | ~1-2 XLM | Variable based on size |
| Contract Initialization | ~0.1 XLM | Per contract |
| Total Deployment (7 contracts) | ~10-20 XLM | One-time cost |
| Monthly Operations | ~5-10 XLM | Variable based on usage |
| **Recommended Initial Funding** | **100 XLM** | Includes buffer |

---

## Document Version

**Version**: 1.0.0  
**Last Updated**: September 11, 2026  
**Authors**: WasteFi Development Team  
**Next Review**: Before mainnet deployment

---

## Related Documentation

- `SECURITY_AUDIT.md` - Security audit preparation
- `INCIDENT_RESPONSE.md` - Emergency procedures
- `OPERATIONS.md` - Daily operations guide
- `UPGRADE_GUIDE.md` - Contract upgrade procedures
- `API.md` - Complete API reference
- `TESTING.md` - Testing documentation

---

**END OF DEPLOYMENT GUIDE**

For questions or issues, contact: devops@wastefi.io
