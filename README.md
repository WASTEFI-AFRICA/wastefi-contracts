# WasteFi Smart Contracts

Financial Inclusion Through Waste Collection - Powered by Stellar Soroban

## Overview

WasteFi is a mobile-first waste banking platform built on Stellar blockchain. These smart contracts power the waste collection, verification, and payment distribution system for emerging markets.

## Project Structure

```
wastefi-contracts/
├── contracts/
│   ├── common/               # Shared types, errors, and interfaces
│   ├── waste_token/          # Reward tokenomics
│   ├── collector_registry/   # Collector identity management
│   ├── collection_point/     # Collection point verification
│   ├── waste_transaction/    # Waste transaction recording
│   ├── payment_distribution/ # Payment and escrow
│   ├── reputation/           # Reputation scoring
│   └── material_pricing/     # Material pricing oracle
└── Cargo.toml
```

## Prerequisites

- Rust 1.74.0 or higher
- Soroban CLI
- Stellar account (testnet/mainnet)

## Installation

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Soroban CLI
```bash
cargo install --locked soroban-cli
```

### Configure Soroban for Stellar Testnet
```bash
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

## Build

Build all contracts:
```bash
cargo build --release --target wasm32-unknown-unknown
```

## Testing

Run all tests:
```bash
cargo test
```

Run tests for a specific contract:
```bash
cargo test -p waste_token
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run integration tests:
```bash
cargo test --test integration_test
```

## Deployment

Deploy to testnet:
```bash
# Generate identity
soroban keys generate deployer --network testnet

# Fund account
soroban keys fund deployer --network testnet

# Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/waste_token.wasm \
  --source deployer \
  --network testnet
```

## Development

### Format code
```bash
cargo fmt --all
```

### Run linter
```bash
cargo clippy --all-targets -- -D warnings
```

### Build optimized
```bash
cargo build --release --target wasm32-unknown-unknown
```

## Architecture

### Layer 1: WasteFi Application Layer
- Mobile app integration
- Collection point management
- Mobile money integration
- Stellar-based payments

### Layer 2: RecycleGraph Protocol Layer
- Open material identification standards
- Cross-chain interoperability
- Digital product passport API
- Verification protocol

## Contract Interactions

```
Collector → CollectorRegistry (register)
       ↓
Collector → WasteTransaction (record collection)
       ↓
WasteTransaction → MaterialPricing (get price)
       ↓
PaymentDistribution → WasteToken (mint rewards)
       ↓
Reputation → Update collector score
```

## Security

- Multi-signature treasury management
- Access control and permissions
- Rate limiting and anti-fraud mechanisms
- Emergency pause mechanisms
- Comprehensive audit trail

## Roadmap

- [x] Phase 1: Project Setup (Commits 1-5)
  - [x] Commit 1: Initial project setup
  - [x] Commit 2: Core contract structure and interfaces
  - [x] Commit 3: Testing framework and CI/CD
  - [x] Commit 4: Development scripts and documentation
  - [ ] Commit 5: Error handling and utilities
- [ ] Phase 2: Core Contracts (Commits 6-12)
- [ ] Phase 3: Advanced Features (Commits 13-18)
- [ ] Phase 4: Security & Optimization (Commits 19-23)
- [ ] Phase 5: Testing & Deployment (Commits 24-25)

## License

MIT License - See LICENSE file for details

## Contributing

See CONTRIBUTING.md for development guidelines

## Support

For issues and questions:
- GitHub Issues: [repository-url]
- Documentation: [docs-url]
- Discord: [discord-invite]
