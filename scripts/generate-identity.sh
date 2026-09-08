#!/bin/bash
# Generate Stellar identity for deployment

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🔑 Generating Stellar identity for WasteFi deployment...${NC}\n"

# Check if soroban is installed
if ! command -v soroban &> /dev/null; then
    echo -e "${YELLOW}Soroban CLI not found. Please run ./scripts/setup.sh first${NC}"
    exit 1
fi

# Generate identity
echo -e "${YELLOW}Generating deployer identity...${NC}"
soroban keys generate deployer --network testnet

# Get the public key
PUBLIC_KEY=$(soroban keys address deployer)
echo -e "${GREEN}✓ Identity generated${NC}"
echo -e "${GREEN}Public Key: ${BLUE}${PUBLIC_KEY}${NC}\n"

# Fund the account
echo -e "${YELLOW}Funding account with testnet XLM...${NC}"
soroban keys fund deployer --network testnet

echo -e "${GREEN}✓ Account funded with testnet XLM${NC}\n"

# Get secret key for .env
SECRET_KEY=$(soroban keys show deployer)

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✨ Deployer identity ready!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

echo -e "${YELLOW}Important:${NC}"
echo -e "1. Your identity has been saved in Soroban CLI"
echo -e "2. Public Key: ${BLUE}${PUBLIC_KEY}${NC}"
echo -e "3. Update your .env file with:"
echo -e "   ${GREEN}DEPLOYER_SECRET=${SECRET_KEY}${NC}"
echo -e "\n${YELLOW}⚠️  Keep your secret key secure! Never commit it to version control.${NC}"
