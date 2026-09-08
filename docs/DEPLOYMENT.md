# WasteFi Deployment Guide

Complete guide for deploying WasteFi contracts to Stellar Soroban.

## Table of Contents

1. [Pre-Deployment](#pre-deployment)
2. [Testnet Deployment](#testnet-deployment)
3. [Mainnet Deployment](#mainnet-deployment)
4. [Post-Deployment](#post-deployment)
5. [Contract Initialization](#contract-initialization)
6. [Verification](#verification)

## Pre-Deployment

### Prerequisites Checklist

- [ ] All contracts compile without errors
- [ ] All tests passing
- [ ] Code reviewed and audited
- [ ] Documentation complete
- [ ] Environment configured
- [ ] Deployer account funded

### Build Contracts

```bash
# Build all contracts
cargo build --target wasm32-unknown-unknown --release

# Verify WASM files generated
ls target/wasm32-unknown-unknown/release/*.wasm
```

Expected output:
```
collector_registry.wasm
collection_point.wasm
material_pricing.wasm
payment_distribution.wasm
reputation.wasm
waste_token.wasm
waste_transaction.wasm
```

### Generate Deployer Identity

```bash
# Linux/macOS
./scripts/generate-identity.sh

# Windows
.\scripts\generate-identity.ps1
```

This will:
1. Generate a new Stellar keypair
2. Save it as "deployer" identity
3. Fund the account with testnet XLM
4. Display the public key and secret

**⚠️ IMPORTANT**: Save the secret key securely!

## Testnet Deployment

### Automated Deployment

```bash
# Linux/macOS
./scripts/deploy.sh

# Windows
.\scripts\deploy.ps1
```

### Manual Deployment

Deploy each contract individually:

#### 1. Deploy WasteToken
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/waste_token.wasm \
  --source deployer \
  --network testnet
```

Save the returned contract ID to `.env`:
```bash
WASTE_TOKEN_ADDRESS=<CONTRACT_ID>
```

#### 2. Deploy CollectorRegistry
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/collector_registry.wasm \
  --source deployer \
  --network testnet
```

#### 3. Deploy CollectionPoint
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/collection_point.wasm \
  --source deployer \
  --network testnet
```

#### 4. Deploy WasteTransaction
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/waste_transaction.wasm \
  --source deployer \
  --network testnet
```

#### 5. Deploy PaymentDistribution
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/payment_distribution.wasm \
  --source deployer \
  --network testnet
```

#### 6. Deploy Reputation
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/reputation.wasm \
  --source deployer \
  --network testnet
```

#### 7. Deploy MaterialPricing
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/material_pricing.wasm \
  --source deployer \
  --network testnet
```

### Update .env File

After deployment, update your `.env` file with all contract addresses:

```env
WASTE_TOKEN_ADDRESS=<TOKEN_CONTRACT_ID>
COLLECTOR_REGISTRY_ADDRESS=<REGISTRY_CONTRACT_ID>
COLLECTION_POINT_ADDRESS=<COLLECTION_POINT_CONTRACT_ID>
WASTE_TRANSACTION_ADDRESS=<TRANSACTION_CONTRACT_ID>
PAYMENT_DISTRIBUTION_ADDRESS=<PAYMENT_CONTRACT_ID>
REPUTATION_ADDRESS=<REPUTATION_CONTRACT_ID>
MATERIAL_PRICING_ADDRESS=<PRICING_CONTRACT_ID>
```

## Mainnet Deployment

### Prerequisites

- [ ] Thoroughly tested on testnet
- [ ] Security audit completed
- [ ] Mainnet deployer account created and funded
- [ ] All documentation finalized
- [ ] Emergency procedures documented

### Configure Mainnet Network

```bash
soroban network add mainnet \
  --rpc-url https://soroban-mainnet.stellar.org:443 \
  --network-passphrase "Public Global Stellar Network ; September 2015"
```

### Generate Mainnet Identity

```bash
# Generate mainnet identity
soroban keys generate mainnet-deployer --network mainnet

# Fund with real XLM (not testnet!)
# Transfer XLM from exchange or wallet to deployer address
```

### Deploy to Mainnet

**⚠️ WARNING**: This uses real XLM and deploys to production!

```bash
# Deploy each contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/waste_token.wasm \
  --source mainnet-deployer \
  --network mainnet
```

Repeat for all 7 contracts.

### Cost Estimation

Estimated costs (approximate):
- Contract deployment: ~0.1 XLM per contract
- Initialization: ~0.05 XLM per contract
- Total for 7 contracts: ~1 XLM

**Note**: Keep extra XLM for operations and upgrades.

## Post-Deployment

### Save Contract Addresses

Document all deployed contract addresses:

```bash
# Create deployment record
cat > deployment-$(date +%Y%m%d).txt << EOF
Deployment Date: $(date)
Network: testnet/mainnet

WasteToken: <ADDRESS>
CollectorRegistry: <ADDRESS>
CollectionPoint: <ADDRESS>
WasteTransaction: <ADDRESS>
PaymentDistribution: <ADDRESS>
Reputation: <ADDRESS>
MaterialPricing: <ADDRESS>
EOF
```

### Backup Keys

```bash
# Export deployer key (KEEP SECURE!)
soroban keys show deployer > deployer-backup.txt

# Encrypt backup
gpg --encrypt --recipient your@email.com deployer-backup.txt

# Store encrypted file securely
# DELETE unencrypted file
rm deployer-backup.txt
```

## Contract Initialization

Initialize each contract with proper configuration:

### 1. Initialize WasteToken
```bash
soroban contract invoke \
  --id $WASTE_TOKEN_ADDRESS \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin "$(soroban keys address deployer)" \
  --name "WasteFi Token" \
  --symbol "WASTE" \
  --decimals 7
```

### 2. Initialize CollectorRegistry
```bash
soroban contract invoke \
  --id $COLLECTOR_REGISTRY_ADDRESS \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin "$(soroban keys address deployer)"
```

### 3. Initialize CollectionPoint
```bash
soroban contract invoke \
  --id $COLLECTION_POINT_ADDRESS \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin "$(soroban keys address deployer)"
```

### 4. Initialize WasteTransaction
```bash
soroban contract invoke \
  --id $WASTE_TRANSACTION_ADDRESS \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin "$(soroban keys address deployer)"
```

### 5. Initialize PaymentDistribution
```bash
soroban contract invoke \
  --id $PAYMENT_DISTRIBUTION_ADDRESS \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin "$(soroban keys address deployer)" \
  --token_contract "$WASTE_TOKEN_ADDRESS"
```

### 6. Initialize Reputation
```bash
soroban contract invoke \
  --id $REPUTATION_ADDRESS \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin "$(soroban keys address deployer)"
```

### 7. Initialize MaterialPricing
```bash
soroban contract invoke \
  --id $MATERIAL_PRICING_ADDRESS \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin "$(soroban keys address deployer)"
```

### Set Initial Material Prices
```bash
# Set prices for each material type (in stroops per kg)
soroban contract invoke \
  --id $MATERIAL_PRICING_ADDRESS \
  --source deployer \
  --network testnet \
  -- set_price \
  --material_type 0 \
  --price_per_kg 10000000  # Plastic: 1 XLM/kg
```

Repeat for all material types (0-9).

## Verification

### Test Contract Calls

```bash
# Check token balance (should be 0 initially)
soroban contract invoke \
  --id $WASTE_TOKEN_ADDRESS \
  --network testnet \
  -- balance \
  --account "$(soroban keys address deployer)"

# Verify collector registry is initialized
soroban contract invoke \
  --id $COLLECTOR_REGISTRY_ADDRESS \
  --network testnet \
  -- is_active \
  --collector "$(soroban keys address deployer)"
```

### Verify on Stellar Expert

Visit [Stellar Expert](https://stellar.expert/) and search for your contract addresses to verify deployment.

### Integration Test

Run end-to-end test:
```bash
# Register collector
# Record collection
# Verify payment
# Check reputation
```

## Troubleshooting

### Deployment Fails

**Error: Insufficient balance**
```bash
# Fund account
soroban keys fund deployer --network testnet
```

**Error: Contract already exists**
- Check if contract was deployed previously
- Use existing contract ID
- Or deploy with different identity

### Initialization Fails

**Error: Already initialized**
- Contract can only be initialized once
- Check if already initialized
- Deploy new instance if needed

**Error: Unauthorized**
- Ensure using deployer identity
- Check admin address is correct

## Best Practices

### Security
- [ ] Use hardware wallet for mainnet deployer key
- [ ] Never share private keys
- [ ] Implement multi-sig for critical operations
- [ ] Regular security audits

### Operations
- [ ] Document all contract addresses
- [ ] Keep deployment scripts updated
- [ ] Monitor contract usage
- [ ] Plan for upgrades

### Monitoring
- [ ] Set up alerting for critical errors
- [ ] Monitor transaction volume
- [ ] Track gas usage
- [ ] Watch for unusual patterns

## Emergency Procedures

### Pause Contracts
If critical bug discovered:
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- pause
```

### Upgrade Contract
Deploy new version and migrate state (requires upgrade mechanism).

## Support

For deployment issues:
- Check logs: `soroban contract read --id <ID>`
- Join Discord: [link]
- Open GitHub issue
- Email: support@wastefi.org
