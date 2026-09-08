#!/bin/bash
# WasteFi Development Environment Setup Script

set -e

echo "🚀 Setting up WasteFi development environment..."

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if Rust is installed
echo -e "\n${YELLOW}Checking Rust installation...${NC}"
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Rust is not installed. Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust installed successfully${NC}"
else
    echo -e "${GREEN}✓ Rust is already installed ($(rustc --version))${NC}"
fi

# Update Rust
echo -e "\n${YELLOW}Updating Rust toolchain...${NC}"
rustup update
echo -e "${GREEN}✓ Rust updated${NC}"

# Add wasm32 target
echo -e "\n${YELLOW}Adding wasm32-unknown-unknown target...${NC}"
rustup target add wasm32-unknown-unknown
echo -e "${GREEN}✓ wasm32 target added${NC}"

# Install rustfmt and clippy
echo -e "\n${YELLOW}Installing rustfmt and clippy...${NC}"
rustup component add rustfmt clippy
echo -e "${GREEN}✓ rustfmt and clippy installed${NC}"

# Check if Soroban CLI is installed
echo -e "\n${YELLOW}Checking Soroban CLI installation...${NC}"
if ! command -v soroban &> /dev/null; then
    echo -e "${YELLOW}Soroban CLI not found. Installing...${NC}"
    cargo install --locked soroban-cli
    echo -e "${GREEN}✓ Soroban CLI installed successfully${NC}"
else
    echo -e "${GREEN}✓ Soroban CLI is already installed ($(soroban --version))${NC}"
fi

# Configure Soroban for Stellar testnet
echo -e "\n${YELLOW}Configuring Soroban for Stellar testnet...${NC}"
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015" \
  2>/dev/null || echo "Testnet already configured"
echo -e "${GREEN}✓ Soroban configured for testnet${NC}"

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo -e "\n${YELLOW}Creating .env file...${NC}"
    cp .env.example .env
    echo -e "${GREEN}✓ .env file created${NC}"
    echo -e "${YELLOW}⚠️  Please update .env with your configuration${NC}"
else
    echo -e "${GREEN}✓ .env file already exists${NC}"
fi

# Build the project
echo -e "\n${YELLOW}Building WasteFi contracts...${NC}"
cargo build --target wasm32-unknown-unknown --release
echo -e "${GREEN}✓ Contracts built successfully${NC}"

# Run tests
echo -e "\n${YELLOW}Running tests...${NC}"
cargo test
echo -e "${GREEN}✓ All tests passed${NC}"

echo -e "\n${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✨ WasteFi development environment is ready!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

echo -e "\n${YELLOW}Next steps:${NC}"
echo -e "1. Update .env with your deployer secret key"
echo -e "2. Generate a deployer identity: ${GREEN}./scripts/generate-identity.sh${NC}"
echo -e "3. Deploy contracts: ${GREEN}./scripts/deploy.sh${NC}"
echo -e "4. Start developing! 🚀"
