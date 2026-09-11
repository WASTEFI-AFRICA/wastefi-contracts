# WasteFi Smart Contracts

**Financial Inclusion Through Waste Collection - Powered by Stellar Soroban**

[![Build Status](https://github.com/wastefi-africa/wastefi-contracts/workflows/CI/badge.svg)](https://github.com/wastefi-africa/wastefi-contracts/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Soroban](https://img.shields.io/badge/Soroban-v20.0.0+-blue)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/rust-1.74.0%2B-orange)](https://www.rust-lang.org)

## Overview

WasteFi is a blockchain-powered waste management platform that incentivizes proper waste collection and recycling through tokenized rewards. Built on Stellar Soroban, these smart contracts enable transparent, secure, and automated waste collection transactions across emerging markets.

**Key Features**:
- 🌍 **Sustainable Impact**: Turn waste into rewards while helping the environment
- 💰 **Automated Payments**: Instant token rewards for verified collections
- 📊 **Reputation System**: Build reputation, earn more rewards
- 🔒 **Secure & Transparent**: Blockchain-based audit trail
- 🚀 **Production-Ready**: Comprehensive security features and testing

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

## Quick Start

### Prerequisites

- **Rust**: 1.74.0 or higher
- **Soroban CLI**: v20.0.0 or higher
- **wasm32 target**: For contract compilation

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Soroban CLI
cargo install --locked soroban-cli --features opt

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Clone repository
git clone https://github.com/wastefi-africa/wastefi-contracts.git
cd wastefi-contracts
```

### Build

```bash
# Build all contracts
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test --workspace

# Check code quality
cargo clippy --workspace -- -D warnings
cargo fmt --all --check
```

### Deploy to Testnet

```bash
# Configure testnet
soroban network add \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015" \
  testnet

# Generate keypair and fund account
soroban keys generate deployer --network testnet
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"

# Run deployment script
./scripts/deploy.sh testnet config/testnet.json
```

For detailed deployment instructions, see [DEPLOYMENT.md](docs/DEPLOYMENT.md).

## Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────┐
│                 WasteFi Platform                         │
└────────────────────┬─────────────────────────────────────┘
                     │
      ┌──────────────┼──────────────┐
      │              │              │
┌─────▼─────┐  ┌────▼────┐  ┌─────▼──────┐
│ Collector │  │Collection│  │   Waste    │
│ Registry  │  │  Point   │  │Transaction │
└─────┬─────┘  └────┬────┘  └─────┬──────┘
      │              │              │
      └──────────────┼──────────────┘
                     │
      ┌──────────────┼──────────────┐
      │              │              │
┌─────▼─────┐  ┌────▼────┐  ┌─────▼──────┐
│  Payment  │  │Material │  │ Reputation │
│Distribution│ │ Pricing │  │   System   │
└─────┬─────┘  └────┬────┘  └─────┬──────┘
      │              │              │
      └──────────────┼──────────────┘
                     │
              ┌──────▼──────┐
              │ Waste Token │
              │  (Rewards)  │
              └─────────────┘
```

### Smart Contracts

| Contract | Purpose | Lines of Code |
|----------|---------|---------------|
| **CollectorRegistry** | Manages collector profiles and status | ~600 |
| **CollectionPoint** | Collection point registry and verification | ~500 |
| **WasteTransaction** | Records waste collection transactions | ~800 |
| **PaymentDistribution** | Calculates and distributes rewards | ~700 |
| **MaterialPricing** | Manages material pricing oracle | ~500 |
| **Reputation** | Tracks collector reputation scores | ~600 |
| **WasteToken** | ERC-20 style reward token | ~400 |
| **Common Library** | Shared utilities and security features | ~4,000 |
| **Total** | | **~8,100** |

### Data Flow

1. **Registration**: Collector registers via `CollectorRegistry`
2. **Collection**: Waste collected and recorded via `WasteTransaction`
3. **Pricing**: System queries `MaterialPricing` for current rates
4. **Verification**: Collection point verifies transaction
5. **Payment**: `PaymentDistribution` mints tokens via `WasteToken`
6. **Reputation**: `Reputation` scores updated based on performance

For detailed architecture, see [DEVELOPER.md](docs/DEVELOPER.md).

## Security Features

✅ **Access Control**: Multi-role authorization (admin, operator, user)  
✅ **Emergency Response**: 4-level emergency system with circuit breakers  
✅ **Fraud Detection**: Multi-factor risk scoring (0-1000 scale)  
✅ **Rate Limiting**: Multi-tier rate limits (per-minute, per-hour, per-day)  
✅ **Duplicate Prevention**: Transaction deduplication with configurable tolerance  
✅ **Contract Upgradeability**: Version management with data migration  
✅ **Gas Optimization**: Efficient storage and batch operations  
✅ **Audit Trail**: Comprehensive event logging for all actions  

**Security Documentation**:
- [Security Audit Guide](docs/SECURITY_AUDIT.md)
- [Threat Model](docs/THREAT_MODEL.md)
- [Security Checklist](docs/SECURITY_CHECKLIST.md)
- [Incident Response Plan](docs/INCIDENT_RESPONSE.md)

**Test Coverage**: >85% (135+ unit tests, 55+ integration tests)

## Documentation

### For Users
- 📖 [User Guide](docs/USER_GUIDE.md) - How to use WasteFi as a collector or collection point

### For Developers
- 🔧 [Developer Guide](docs/DEVELOPER.md) - Architecture, setup, and contribution guidelines
- 📚 [API Reference](docs/API.md) - Complete API documentation for all contracts
- 🧪 [Testing Guide](docs/TESTING.md) - Testing strategy and coverage

### For Operators
- 🚀 [Deployment Guide](docs/DEPLOYMENT.md) - Step-by-step deployment instructions
- 🔄 [Operations Runbook](docs/OPERATIONS.md) - Daily operations and maintenance
- 🚨 [Incident Response](docs/INCIDENT_RESPONSE.md) - Emergency procedures

### Security Documentation
- 🔒 [Security Audit](docs/SECURITY_AUDIT.md) - Audit preparation guide
- 🛡️ [Threat Model](docs/THREAT_MODEL.md) - Security threat analysis
- ✅ [Security Checklist](docs/SECURITY_CHECKLIST.md) - Pre-deployment verification
- 📋 [Security Considerations](docs/SECURITY_CONSIDERATIONS.md) - Per-contract security analysis

## Contract Addresses

### Testnet
*Deployment in progress - addresses will be published here*

### Mainnet
*Mainnet deployment pending security audit completion*

## Project Status

**Current Phase**: Phase 5 (Testing & Deployment) - 86% Complete

**Completed**:
- ✅ Phase 1: Project Setup (Commits 1-5)
- ✅ Phase 2: Core Contracts (Commits 6-12)
- ✅ Phase 3: Advanced Features (Commits 13-18)
- ✅ Phase 4: Security & Optimization (Commits 19-23)
- ✅ Phase 5: Testing Infrastructure (Commit 24)
- 🔄 Phase 5: Deployment & Documentation (Commit 25) - In Progress

**Next Steps**:
1. Complete testnet deployment
2. Security audit
3. Mainnet deployment

See [PROJECT_STATUS.md](PROJECT_STATUS.md) for detailed progress.

## Contributing

We welcome contributions from the community! Please read our [Contributing Guidelines](docs/DEVELOPER.md#7-contributing-guidelines) before submitting pull requests.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes and add tests
4. Run tests: `cargo test --workspace`
5. Run linters: `cargo clippy && cargo fmt`
6. Commit with descriptive message: `git commit -m "feat: Add feature description"`
7. Push and create a Pull Request

### Code Standards

- Follow Rust conventions (enforced by `rustfmt` and `clippy`)
- Write tests for new features
- Update documentation
- Ensure CI/CD pipeline passes

## Testing

### Run All Tests
```bash
cargo test --workspace
```

### Run Specific Test Suites
```bash
# Unit tests
cargo test -p collector_registry

# Integration tests
cargo test --test integration_e2e

# Stress tests
cargo test --test stress_tests

# Security tests
cargo test --test security_tests
```

### Generate Coverage Report
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --workspace --out Html
```

**Current Test Coverage**: >85% (135+ unit tests, 55+ integration tests)

See [TESTING.md](docs/TESTING.md) for detailed testing guidelines.

## Support & Community

### Get Help
- 📧 **Email**: support@wastefi.io
- 💬 **Discord**: [Join our community](#) (link TBD)
- 🐛 **Issues**: [GitHub Issues](https://github.com/wastefi-africa/wastefi-contracts/issues)
- 📖 **Documentation**: [docs.wastefi.io](#) (link TBD)

### Stay Updated
- 🐦 **Twitter**: [@WasteFiAfrica](#) (link TBD)
- 📘 **Facebook**: [WasteFi Africa](#) (link TBD)
- 💼 **LinkedIn**: [WasteFi](#) (link TBD)

## Known Issues

**Critical** (Must fix before mainnet):
1. **No double-payment prevention** in PaymentDistribution
2. **No token supply cap** in WasteToken
3. **Single admin key** (multi-sig recommended)

See [SECURITY_CONSIDERATIONS.md](docs/SECURITY_CONSIDERATIONS.md) for complete list.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Stellar Foundation** for the Soroban platform
- **Security Auditors** (pending)
- **Community Contributors**
- **WasteFi Development Team**

## About WasteFi

WasteFi is building financial inclusion infrastructure through waste management. By rewarding proper waste collection and recycling with blockchain-based incentives, we're creating sustainable livelihoods while addressing environmental challenges in emerging markets.

**Website**: [www.wastefi.io](#) (link TBD)  
**GitHub**: [github.com/wastefi-africa](https://github.com/wastefi-africa)

---

**Made with ♻️ by the WasteFi Team**

*For questions or partnership inquiries: info@wastefi.io*
