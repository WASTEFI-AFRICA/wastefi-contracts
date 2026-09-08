#!/bin/bash
# Deploy WasteFi contracts to Stellar testnet

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}🚀 Deploying WasteFi contracts to Stellar testnet...${NC}\n"

# Check if soroban is installed
if ! command -v soroban &> /dev/null; then
    echo -e "${RED}Soroban CLI not found. Please run ./scripts/setup.sh first${NC}"
    exit 1
fi

# Build contracts
echo -e "${YELLOW}Building contracts...${NC}"
cargo build --target wasm32-unknown-unknown --release
echo -e "${GREEN}✓ Contracts built${NC}\n"

# Deploy contracts
CONTRACTS=("waste_token" "collector_registry" "collection_point" "waste_transaction" "payment_distribution" "reputation" "material_pricing")

echo -e "${YELLOW}Deploying contracts...${NC}\n"

for contract in "${CONTRACTS[@]}"; do
    echo -e "${YELLOW}Deploying ${contract}...${NC}"
    
    CONTRACT_ID=$(soroban contract deploy \
        --wasm target/wasm32-unknown-unknown/release/${contract}.wasm \
        --source deployer \
        --network testnet)
    
    echo -e "${GREEN}✓ ${contract} deployed${NC}"
    echo -e "${BLUE}Contract ID: ${CONTRACT_ID}${NC}\n"
    
    # Save contract ID to .env
    UPPER_NAME=$(echo "${contract}" | tr '[:lower:]' '[:upper:]')
    echo "${UPPER_NAME}_ADDRESS=${CONTRACT_ID}" >> .env.deployed
done

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✨ All contracts deployed successfully!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

echo -e "${YELLOW}Contract addresses saved to .env.deployed${NC}"
echo -e "${YELLOW}Copy these addresses to your .env file${NC}\n"

cat .env.deployed
