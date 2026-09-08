# WasteFi Development Guide

Complete guide for developing and testing WasteFi smart contracts.

## Table of Contents

1. [Environment Setup](#environment-setup)
2. [Project Structure](#project-structure)
3. [Building Contracts](#building-contracts)
4. [Testing](#testing)
5. [Deployment](#deployment)
6. [Contract Interactions](#contract-interactions)
7. [Troubleshooting](#troubleshooting)

## Environment Setup

### Prerequisites

- **Rust**: Version 1.79.0 or higher
- **Soroban CLI**: Latest version
- **Git**: For version control
- **Code Editor**: VS Code recommended with Rust extensions

### Quick Setup

#### Linux/macOS
```bash
./scripts/setup.sh
```

#### Windows
```powershell
.\scripts\setup.ps1
```

### Manual Setup

1. **Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install Soroban CLI**
```bash
cargo install --locked soroban-cli
```

3. **Add wasm32 target**
```bash
rustup target add wasm32-unknown-unknown
```

4. **Configure Stellar testnet**
```bash
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

## Project Structure

```
wastefi-contracts/
├── contracts/           # Smart contracts
│   ├── common/         # Shared types and utilities
│   ├── waste_token/    # Token contract
│   ├── collector_registry/
│   ├── collection_point/
│   ├── waste_transaction/
│   ├── payment_distribution/
│   ├── reputation/
│   └── material_pricing/
├── scripts/            # Automation scripts
├── tests/             # Integration tests
└── docs/              # Documentation
```

## Building Contracts

### Build all contracts
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Build specific contract
```bash
cargo build -p waste_token --target wasm32-unknown-unknown --release
```

### Optimized build
```bash
make optimize
```

### Check compilation without building
```bash
cargo check --workspace
```

## Testing

### Run all tests
```bash
cargo test
```

### Run specific contract tests
```bash
cargo test -p waste_token
```

### Run integration tests
```bash
cargo test --test integration_test
```

### Run with output
```bash
cargo test -- --nocapture
```

### Test coverage (requires cargo-tarpaulin)
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --workspace
```

## Deployment

### Generate Deployer Identity

#### Linux/macOS
```bash
./scripts/generate-identity.sh
```

#### Windows
```powershell
.\scripts\generate-identity.ps1
```

### Deploy to Testnet

#### Linux/macOS
```bash
./scripts/deploy.sh
```

#### Windows
```powershell
.\scripts\deploy.ps1
```

### Deploy Single Contract
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/waste_token.wasm \
  --source deployer \
  --network testnet
```

## Contract Interactions

### Initialize WasteToken
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --name "WasteFi Token" \
  --symbol "WASTE" \
  --decimals 7
```

### Register Collector
```bash
soroban contract invoke \
  --id <COLLECTOR_REGISTRY_ID> \
  --source deployer \
  --network testnet \
  -- register \
  --collector <COLLECTOR_ADDRESS> \
  --name "John Doe" \
  --phone "+1234567890"
```

### Record Waste Collection
```bash
soroban contract invoke \
  --id <WASTE_TRANSACTION_ID> \
  --source deployer \
  --network testnet \
  -- record_collection \
  --collector <COLLECTOR_ADDRESS> \
  --collection_point <POINT_ADDRESS> \
  --material_type 0 \
  --weight 5000
```

## Code Quality

### Format code
```bash
cargo fmt --all
```

### Run linter
```bash
cargo clippy --all-targets -- -D warnings
```

### Check all quality gates
```bash
make check
```

## Debugging

### Enable contract logging
```bash
# Set environment variable
export RUST_LOG=debug

# Run tests with logs
cargo test -- --nocapture
```

### View contract state
```bash
soroban contract read \
  --id <CONTRACT_ID> \
  --network testnet
```

## Troubleshooting

### Build Errors

**Error: linker not found**
```bash
rustup target add wasm32-unknown-unknown
```

**Error: cargo-audit failed**
```bash
cargo install cargo-audit
cargo audit fix
```

### Network Errors

**Error: Connection refused**
- Check internet connection
- Verify Stellar testnet is operational
- Try different RPC endpoint

**Error: Account not found**
```bash
# Fund account with testnet XLM
soroban keys fund deployer --network testnet
```

### Contract Errors

**Error: Contract not initialized**
- Call initialize function first
- Check initialization parameters

**Error: Unauthorized**
- Verify deployer identity
- Check admin permissions

## Best Practices

### Development
- Write tests before implementation
- Use descriptive variable names
- Add comments for complex logic
- Follow Rust naming conventions

### Security
- Never commit private keys
- Use environment variables for secrets
- Validate all inputs
- Implement access controls

### Testing
- Test happy path and error cases
- Use realistic test data
- Test edge cases
- Mock external dependencies

### Deployment
- Test on testnet thoroughly
- Document contract addresses
- Keep deployment scripts updated
- Monitor gas usage

## Useful Commands

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check outdated dependencies
cargo outdated

# Run security audit
cargo audit

# Generate documentation
cargo doc --no-deps --open

# Profile build time
cargo build --timings
```

## Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Developer Portal](https://developers.stellar.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WasteFi Project](https://github.com/wastefi)

## Getting Help

- Open an issue on GitHub
- Join our Discord community
- Check existing documentation
- Review test examples

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines.
