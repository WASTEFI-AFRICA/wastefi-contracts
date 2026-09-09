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

- [x] Phase 1: Project Setup (Commits 1-5) ✅
  - [x] Commit 1: Initial project setup
  - [x] Commit 2: Core contract structure and interfaces
  - [x] Commit 3: Testing framework and CI/CD
  - [x] Commit 4: Development scripts and documentation
  - [x] Commit 5: Error handling and utilities
- [x] Phase 2: Core Contracts (Commits 6-12) ✅
  - [x] Commit 6: WasteToken contract
  - [x] Commit 7: Collector registry contract
  - [x] Commit 8: Collection point contract
  - [x] Commit 9: Waste transaction contract
  - [x] Commit 10: Payment distribution contract
  - [x] Commit 11: Reputation contract
  - [x] Commit 12: Material pricing contract
- [x] Phase 3: Advanced Features (Commits 13-18) ✅
  - [x] Commit 13: Integration testing suite
  - [x] Commit 14: Cross-contract interactions
  - [x] Commit 15: Batch operations and optimizations
  - [x] Commit 16: Advanced query functions
  - [x] Commit 17: Event indexing utilities
  - [x] Commit 18: Admin management improvements
- [x] Phase 4: Security & Optimization (Commits 19-23) ✅
  - [x] Commit 19: Emergency response mechanisms
  - [x] Commit 20: Rate limiting and anti-fraud
  - [x] Commit 21: Contract upgradeability patterns
  - [x] Commit 22: Gas optimization and storage efficiency
  - [x] Commit 23: Security audit preparation
- [ ] Phase 5: Testing & Deployment (Commits 24-25)
  - [ ] Commit 24: End-to-end testing and stress tests
  - [ ] Commit 25: Testnet deployment and documentation

## License

MIT License - See LICENSE file for details

## Contributing

See CONTRIBUTING.md for development guidelines

## Support

For issues and questions:
- GitHub Issues: [repository-url]
- Documentation: [docs-url]
- Discord: [discord-invite]
