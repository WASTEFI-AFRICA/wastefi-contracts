# WasteFi Developer Guide

## Document Purpose

This guide provides developers with comprehensive information about the WasteFi platform architecture, development environment setup, testing guidelines, integration patterns, and contribution guidelines.

**Target Audience**: Developers, contributors, integrators  
**Version**: 1.0.0  
**Last Updated**: September 11, 2026

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Development Environment Setup](#2-development-environment-setup)
3. [Project Structure](#3-project-structure)
4. [Contract Interaction Patterns](#4-contract-interaction-patterns)
5. [Testing Guidelines](#5-testing-guidelines)
6. [Integration Guide](#6-integration-guide)
7. [Contributing Guidelines](#7-contributing-guidelines)
8. [Code Style Guide](#8-code-style-guide)
9. [Security Best Practices](#9-security-best-practices)
10. [Debugging and Troubleshooting](#10-debugging-and-troubleshooting)

---

## 1. Architecture Overview

### 1.1 System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend Layer                        │
│  (Web App, Mobile App, Admin Dashboard)                 │
└────────────────────┬────────────────────────────────────┘
                     │ HTTPS/WebSocket
┌────────────────────▼────────────────────────────────────┐
│                  Backend Services                        │
│  (API Gateway, Payment Processor, Monitoring)           │
└────────────────────┬────────────────────────────────────┘
                     │ Soroban SDK
┌────────────────────▼────────────────────────────────────┐
│              Stellar Soroban Network                     │
│                                                           │
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐  │
│  │  Collector  │  │  Collection  │  │     Waste     │  │
│  │  Registry   │  │    Point     │  │  Transaction  │  │
│  └──────┬──────┘  └──────┬───────┘  └───────┬───────┘  │
│         │                 │                   │           │
│         └─────────────────┼───────────────────┘          │
│                           │                               │
│  ┌─────────────┐  ┌──────▼──────┐  ┌───────────────┐  │
│  │   Payment   │  │  Material   │  │  Reputation   │  │
│  │Distribution │  │   Pricing   │  │    System     │  │
│  └──────┬──────┘  └──────┬──────┘  └───────┬───────┘  │
│         │                 │                   │           │
│         └─────────────────┼───────────────────┘          │
│                           │                               │
│                  ┌────────▼────────┐                     │
│                  │  Waste Token    │                     │
│                  │   (Rewards)     │                     │
│                  └─────────────────┘                     │
│                                                           │
│                  ┌─────────────────┐                     │
│                  │ Common Library  │                     │
│                  │ (Shared Utils)  │                     │
│                  └─────────────────┘                     │
└───────────────────────────────────────────────────────────┘
```

---

### 1.2 Contract Relationships

**CollectorRegistry**:
- Manages collector profiles and status
- Referenced by: WasteTransaction, PaymentDistribution
- No dependencies on other contracts

**CollectionPoint**:
- Manages collection point registry
- Can verify transactions in WasteTransaction
- No dependencies on other contracts

**WasteTransaction**:
- Core transaction management
- Depends on:
  - MaterialPricing (price lookup)
  - Reputation (score queries)
  - CollectorRegistry (status check - optional)
- Integrates with: PaymentDistribution

**PaymentDistribution**:
- Processes payments for verified transactions
- Depends on:
  - WasteToken (for minting rewards)
  - WasteTransaction (for transaction details)

**MaterialPricing**:
- Manages material prices
- No dependencies
- Queried by: WasteTransaction

**Reputation**:
- Tracks user reputation scores
- No dependencies
- Updated by: WasteTransaction, PaymentDistribution

**WasteToken**:
- ERC-20 style reward token
- No dependencies
- Used by: PaymentDistribution

---

### 1.3 Data Flow

**Typical Transaction Flow**:
```
1. User registers
   CollectorRegistry.register()
   → Status: Active
   → Initial reputation: 500

2. User records collection
   WasteTransaction.record_collection()
   → Queries MaterialPricing.get_price()
   → Checks Reputation.get_score()
   → Fraud detection analysis
   → Status: Pending

3. Collection point verifies
   CollectionPoint.verify_collection()
   → Updates WasteTransaction status: Verified
   → Updates Reputation scores

4. Payment processed
   PaymentDistribution.process_payment()
   → Calculates amount with reputation bonus
   → Mints tokens via WasteToken.mint()
   → Status: Paid

5. User receives tokens
   WasteToken.balance()
   → Shows updated balance
```

---

### 1.4 Technology Stack

**Smart Contracts**:
- Language: Rust
- Framework: Soroban SDK v20.0.0+
- Target: wasm32-unknown-unknown
- Network: Stellar Soroban

**Development Tools**:
- Soroban CLI: v20.0.0+
- Rust: 1.74.0+
- Cargo: Latest stable

**Testing**:
- Test Framework: Rust built-in + Soroban test utilities
- Coverage: cargo-tarpaulin
- E2E: Custom integration tests

**CI/CD**:
- GitHub Actions
- Automated testing
- Deployment automation

---

## 2. Development Environment Setup

### 2.1 Prerequisites Installation

#### Install Rust
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version  # Should be 1.74.0 or higher

# Add wasm32 target
rustup target add wasm32-unknown-unknown
```

#### Install Soroban CLI
```bash
# Install Soroban CLI
cargo install --locked soroban-cli --features opt

# Verify installation
soroban --version  # Should be 20.0.0 or higher
```

#### Install Additional Tools
```bash
# Code coverage tool
cargo install cargo-tarpaulin

# Formatting and linting (should be included with Rust)
rustup component add rustfmt clippy

# Optional: cargo-watch for auto-rebuild
cargo install cargo-watch
```

---

### 2.2 Clone and Build

```bash
# Clone repository
git clone https://github.com/wastefi-africa/wastefi-contracts.git
cd wastefi-contracts

# Build all contracts
cargo build --target wasm32-unknown-unknown --release

# Verify build
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# Run tests
cargo test --workspace

# Check code quality
cargo clippy --workspace -- -D warnings
cargo fmt --all --check
```

---

### 2.3 IDE Setup

#### Visual Studio Code (Recommended)
```json
// .vscode/settings.json
{
  "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "rust-analyzer.linkedProjects": [
    "./Cargo.toml"
  ]
}
```

**Recommended Extensions**:
- rust-analyzer
- CodeLLDB (for debugging)
- Better TOML
- Error Lens

#### IntelliJ IDEA / CLion
- Install Rust plugin
- Configure Rust toolchain
- Set target to wasm32-unknown-unknown
- Enable format on save

---

### 2.4 Local Network Setup

#### Configure Soroban Networks
```bash
# Add testnet
soroban network add \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015" \
  testnet

# Add mainnet
soroban network add \
  --rpc-url https://soroban-mainnet.stellar.org:443 \
  --network-passphrase "Public Global Stellar Network ; September 2015" \
  mainnet

# Verify networks
soroban network ls
```

#### Generate Test Keypairs
```bash
# Generate testnet admin key
soroban keys generate testnet-admin --network testnet

# Get address
soroban keys address testnet-admin

# Fund from friendbot
curl "https://friendbot.stellar.org?addr=$(soroban keys address testnet-admin)"

# Verify balance
soroban contract invoke \
  --network testnet \
  --source testnet-admin \
  -- \
  balance \
  --id $(soroban keys address testnet-admin)
```

---

## 3. Project Structure

### 3.1 Directory Layout

```
wastefi-contracts/
├── contracts/                   # Smart contracts
│   ├── collector_registry/      # Collector management
│   │   ├── src/
│   │   │   ├── lib.rs          # Main contract
│   │   │   ├── storage.rs      # Storage layer
│   │   │   └── test.rs         # Unit tests
│   │   └── Cargo.toml
│   ├── collection_point/        # Collection point registry
│   ├── waste_transaction/       # Transaction management
│   ├── payment_distribution/    # Payment processing
│   ├── material_pricing/        # Pricing oracle
│   ├── reputation/              # Reputation system
│   ├── waste_token/             # Reward token
│   └── common/                  # Shared library
│       ├── src/
│       │   ├── lib.rs           # Common exports
│       │   ├── access_control.rs
│       │   ├── anti_fraud.rs
│       │   ├── emergency.rs
│       │   ├── errors.rs
│       │   ├── events.rs
│       │   ├── types.rs
│       │   ├── validation.rs
│       │   └── ...
│       └── Cargo.toml
├── tests/                       # Integration tests
│   ├── integration_e2e.rs       # E2E test suite
│   ├── stress_tests.rs          # Stress/performance tests
│   ├── security_tests.rs        # Security tests
│   └── chaos_tests.rs           # Chaos testing
├── scripts/                     # Deployment & utility scripts
│   ├── deploy.sh                # Unix deployment script
│   ├── deploy.ps1               # Windows deployment script
│   └── ...
├── config/                      # Configuration files
│   ├── testnet.json
│   ├── mainnet.template.json
│   └── README.md
├── docs/                        # Documentation
│   ├── API.md
│   ├── DEPLOYMENT.md
│   ├── DEVELOPER.md (this file)
│   ├── OPERATIONS.md
│   ├── SECURITY_AUDIT.md
│   ├── TESTING.md
│   ├── USER_GUIDE.md
│   └── ...
├── Cargo.toml                   # Workspace manifest
├── Cargo.lock                   # Dependency lock file
├── rust-toolchain.toml          # Rust version specification
├── .gitignore
├── README.md
└── LICENSE
```

---

### 3.2 Contract Structure

**Standard Contract Layout**:
```rust
// contracts/<contract_name>/src/lib.rs

#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address};
use common::*;  // Shared utilities

#[contract]
pub struct ContractName;

#[contractimpl]
impl ContractName {
    // Initialization
    pub fn initialize(env: Env, admin: Address) {
        // Setup logic
    }
    
    // Core functionality
    pub fn some_method(env: Env, param: Type) -> ReturnType {
        // Implementation
    }
    
    // Admin operations
    pub fn admin_method(env: Env) {
        // Admin-only logic
    }
    
    // Query operations
    pub fn get_something(env: Env, id: u64) -> Data {
        // Read-only query
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_something() {
        // Test implementation
    }
}
```

---

## 4. Contract Interaction Patterns

### 4.1 Cross-Contract Calls

**Example: WasteTransaction querying MaterialPricing**:

```rust
// In WasteTransaction contract

use soroban_sdk::{Address, Env, IntoVal};

fn get_material_price(env: &Env, material_type: MaterialType) -> i128 {
    let pricing_contract: Address = /* get from storage */;
    
    // Call MaterialPricing.get_price()
    let price: i128 = env.invoke_contract(
        &pricing_contract,
        &symbol_short!("get_price"),
        vec![
            &env,
            material_type.into_val(env)
        ]
    );
    
    price
}
```

**Best Practices**:
- Always validate contract addresses before calling
- Handle errors from cross-contract calls
- Cache contract addresses in storage
- Consider gas costs of cross-contract calls

---

### 4.2 Event Emission

```rust
use soroban_sdk::{contractevent, symbol_short};

// Define event
#[contractevent]
pub struct CollectionRecorded {
    pub transaction_id: u64,
    pub collector: Address,
    pub weight: u64,
    pub amount: i128,
}

// Emit event
pub fn record_collection(env: Env, /* params */) -> u64 {
    let tx_id = generate_id(&env);
    
    // ... business logic ...
    
    // Emit event
    env.events().publish((
        symbol_short!("rec_coll"),  // Topic
        tx_id,                       // Additional indexing
    ), CollectionRecorded {
        transaction_id: tx_id,
        collector: collector.clone(),
        weight,
        amount,
    });
    
    tx_id
}
```

**Event Best Practices**:
- Use events for important state changes
- Keep event data concise
- Use symbolic topics for filtering
- Don't rely on events for contract logic

---

### 4.3 Storage Patterns

**Efficient Storage Usage**:

```rust
use soroban_sdk::{Env, Address, Map};

// Storage keys
const COLLECTORS: Symbol = symbol_short!("collectors");
const COLLECTOR_COUNT: Symbol = symbol_short!("col_count");

// Store collector data
pub fn store_collector(env: &Env, collector: &Address, data: &Collector) {
    let mut collectors: Map<Address, Collector> = 
        env.storage().persistent().get(&COLLECTORS)
            .unwrap_or(Map::new(env));
    
    collectors.set(collector.clone(), data.clone());
    env.storage().persistent().set(&COLLECTORS, &collectors);
    
    // Update count
    let count: u64 = env.storage().persistent()
        .get(&COLLECTOR_COUNT).unwrap_or(0);
    env.storage().persistent().set(&COLLECTOR_COUNT, &(count + 1));
}

// Retrieve collector data
pub fn get_collector(env: &Env, collector: &Address) -> Option<Collector> {
    let collectors: Map<Address, Collector> = 
        env.storage().persistent().get(&COLLECTORS)?;
    
    collectors.get(collector.clone())
}
```

**Storage Types**:
- **Persistent**: Long-term data (use for core state)
- **Temporary**: Short-term data (use for caches)
- **Instance**: Contract-level data (use for configuration)

---

### 4.4 Authorization Pattern

```rust
use soroban_sdk::auth::{Context, CustomAccountInterface};

pub fn protected_operation(env: Env, caller: Address) {
    // Verify caller authorization
    caller.require_auth();
    
    // ... perform operation ...
}

pub fn admin_only_operation(env: Env) {
    let admin: Address = get_admin(&env);
    
    // Verify admin
    admin.require_auth();
    
    // ... perform admin operation ...
}
```

---

## 5. Testing Guidelines

### 5.1 Unit Testing

**Test Structure**:
```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    
    #[test]
    fn test_collector_registration() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CollectorRegistry);
        let client = CollectorRegistryClient::new(&env, &contract_id);
        
        // Setup
        let admin = Address::generate(&env);
        let collector = Address::generate(&env);
        
        // Initialize
        client.initialize(&admin);
        
        // Test registration
        client.register(&collector, &String::from_str(&env, "John"), 
                       &String::from_str(&env, "+1234567890"));
        
        // Assertions
        let data = client.get_collector(&collector);
        assert_eq!(data.name, String::from_str(&env, "John"));
        assert_eq!(data.status, CollectorStatus::Active);
    }
    
    #[test]
    #[should_panic(expected = "AlreadyRegistered")]
    fn test_duplicate_registration() {
        // Test that duplicate registration fails
    }
}
```

**Run Tests**:
```bash
# Run all tests
cargo test --workspace

# Run specific contract tests
cargo test -p collector_registry

# Run single test
cargo test test_collector_registration

# Run with output
cargo test -- --nocapture

# Run with coverage
cargo tarpaulin --workspace --out Html
```

---

### 5.2 Integration Testing

**Example E2E Test**:
```rust
// tests/integration_e2e.rs

#[test]
fn test_complete_workflow() {
    let env = Env::default();
    
    // Deploy all contracts
    let registry = deploy_collector_registry(&env);
    let transaction = deploy_waste_transaction(&env);
    let payment = deploy_payment_distribution(&env);
    let token = deploy_waste_token(&env);
    let pricing = deploy_material_pricing(&env);
    let reputation = deploy_reputation(&env);
    
    // Setup cross-contract references
    transaction.set_pricing_contract(&pricing.address);
    transaction.set_reputation_contract(&reputation.address);
    payment.set_token_contract(&token.address);
    
    // Test workflow
    let collector = Address::generate(&env);
    let point = Address::generate(&env);
    
    // 1. Register collector
    registry.register(&collector, &"John", &"+1234567890");
    
    // 2. Record collection
    let tx_id = transaction.record_collection(
        &collector, &point, &MaterialType::Plastic, &5000, &100
    );
    
    // 3. Verify transaction
    transaction.verify_transaction(&tx_id);
    
    // 4. Process payment
    payment.process_payment(&tx_id);
    
    // 5. Verify token balance
    let balance = token.balance(&collector);
    assert!(balance > 0);
}
```

**Run Integration Tests**:
```bash
cargo test --test integration_e2e
```

---

### 5.3 Test Coverage

**Generate Coverage Report**:
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate HTML report
cargo tarpaulin --workspace --out Html --output-dir ./coverage

# Open report
open coverage/index.html  # macOS
xdg-open coverage/index.html  # Linux
start coverage/index.html  # Windows
```

**Coverage Targets**:
- Overall: >85%
- Critical paths: >95%
- Error handling: >80%

---

## 6. Integration Guide

### 6.1 Frontend Integration

**Using Soroban SDK (JavaScript)**:

```typescript
import { Contract, SorobanRpc, TransactionBuilder, Networks } from 'soroban-client';

// Initialize contract client
const rpcUrl = 'https://soroban-testnet.stellar.org';
const server = new SorobanRpc.Server(rpcUrl);

const contractId = 'CC...';  // CollectorRegistry contract ID
const contract = new Contract(contractId);

// Call contract method
async function registerCollector(
  userKeypair: Keypair,
  name: string,
  phone: string
) {
  const account = await server.getAccount(userKeypair.publicKey());
  
  const transaction = new TransactionBuilder(account, {
    fee: '100',
    networkPassphrase: Networks.TESTNET
  })
    .addOperation(
      contract.call(
        'register',
        contract.address(userKeypair.publicKey()),
        contract.string(name),
        contract.string(phone)
      )
    )
    .setTimeout(30)
    .build();
  
  transaction.sign(userKeypair);
  
  const result = await server.sendTransaction(transaction);
  return result;
}
```

---

### 6.2 Backend Integration

**Python Example (using stellar-sdk)**:

```python
from stellar_sdk import Soroban, Keypair, Network, TransactionBuilder
from stellar_sdk.soroban_rpc import SorobanServer

# Initialize
rpc_url = 'https://soroban-testnet.stellar.org:443'
server = SorobanServer(rpc_url)
network_passphrase = Network.TESTNET_NETWORK_PASSPHRASE

# Load contract
contract_id = 'CC...'

# Call contract
def get_collector(collector_address: str):
    source = Keypair.from_secret('S...')  # Admin keypair
    
    # Build transaction
    source_account = server.load_account(source.public_key)
    
    tx = (
        TransactionBuilder(
            source_account=source_account,
            network_passphrase=network_passphrase,
            base_fee=100
        )
        .append_invoke_contract_function_op(
            contract_id=contract_id,
            function_name='get_collector',
            parameters=[
                scval.to_address(collector_address)
            ]
        )
        .set_timeout(30)
        .build()
    )
    
    # Simulate first
    sim_response = server.simulate_transaction(tx)
    
    # Send transaction
    tx.sign(source)
    response = server.send_transaction(tx)
    
    return response
```

---

### 6.3 Mobile Integration

**React Native Example**:

```typescript
import { SorobanClient } from '@stellar/soroban-client';

class WasteFiSDK {
  private server: SorobanClient.Server;
  private contractIds: {
    collectorRegistry: string;
    wasteTransaction: string;
    // ...
  };
  
  constructor(network: 'testnet' | 'mainnet') {
    const rpcUrl = network === 'testnet'
      ? 'https://soroban-testnet.stellar.org'
      : 'https://soroban-mainnet.stellar.org';
    
    this.server = new SorobanClient.Server(rpcUrl);
    this.contractIds = loadContractIds(network);
  }
  
  async registerCollector(
    userKeypair: Keypair,
    name: string,
    phone: string
  ): Promise<void> {
    // Implementation
  }
  
  async recordCollection(
    userKeypair: Keypair,
    collectionPoint: string,
    materialType: MaterialType,
    weight: number
  ): Promise<string> {
    // Returns transaction ID
  }
  
  async getBalance(address: string): Promise<number> {
    // Get WASTE token balance
  }
}

// Usage
const sdk = new WasteFiSDK('testnet');
await sdk.registerCollector(keypair, 'John Doe', '+1234567890');
```

---

## 7. Contributing Guidelines

### 7.1 Getting Started

1. **Fork the repository**
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/my-feature
   ```
3. **Make your changes**
4. **Run tests**:
   ```bash
   cargo test --workspace
   cargo clippy --workspace
   cargo fmt --all
   ```
5. **Commit with descriptive message**:
   ```bash
   git commit -m "feat: Add new feature description"
   ```
6. **Push and create Pull Request**

---

### 7.2 Commit Message Convention

**Format**: `<type>(<scope>): <subject>`

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `chore`: Build/tooling changes

**Examples**:
```
feat(collector-registry): Add batch registration support
fix(payment): Resolve double-payment issue
docs(api): Update CollectorRegistry documentation
test(transaction): Add fraud detection tests
```

---

### 7.3 Pull Request Process

**PR Checklist**:
- [ ] Code follows project style guide
- [ ] All tests passing
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] No security vulnerabilities introduced
- [ ] Breaking changes documented
- [ ] Commit messages follow convention

**PR Description Template**:
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- Test scenario 1
- Test scenario 2

## Checklist
- [ ] Tests pass
- [ ] Documentation updated
- [ ] No security issues
```

---

### 7.4 Code Review Guidelines

**As a Reviewer**:
- Check for security issues
- Verify test coverage
- Ensure code clarity
- Check performance implications
- Verify documentation updates

**As an Author**:
- Respond to all comments
- Make requested changes
- Keep PR focused and small
- Update PR description if scope changes

---

## 8. Code Style Guide

### 8.1 Rust Style

**Follow Rust conventions**:
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow naming conventions:
  - `snake_case` for functions and variables
  - `PascalCase` for types and traits
  - `SCREAMING_SNAKE_CASE` for constants

**Example**:
```rust
// Good
const MAX_WEIGHT: u64 = 1_000_000;

pub struct Collector {
    pub address: Address,
    pub total_weight: u64,
}

pub fn get_collector(env: &Env, address: &Address) -> Option<Collector> {
    // Implementation
}

// Bad
const maxWeight: u64 = 1000000;  // Wrong naming

pub struct collector {  // Wrong casing
    Address: Address,  // Wrong casing
}
```

---

### 8.2 Documentation

**Document public APIs**:
```rust
/// Registers a new collector on the platform.
///
/// # Arguments
/// * `env` - The contract environment
/// * `collector` - Address of the collector to register
/// * `name` - Full name of the collector
/// * `phone` - Contact phone number
///
/// # Errors
/// * `CollectorAlreadyRegistered` - Collector already exists
/// * `InvalidInput` - Name or phone is invalid
///
/// # Events
/// Emits `CollectorRegistered` event on success
pub fn register(
    env: Env,
    collector: Address,
    name: String,
    phone: String
) {
    // Implementation
}
```

---

### 8.3 Error Handling

**Use Result types for fallible operations**:
```rust
// Good
pub fn get_collector(env: &Env, address: &Address) -> Result<Collector, WasteFiError> {
    collectors.get(address)
        .ok_or(WasteFiError::CollectorNotFound)
}

// Use panic! for contract-level errors
pub fn admin_only_operation(env: &Env) {
    let admin = get_admin(env);
    if env.invoker() != admin {
        panic_with_error!(env, WasteFiError::NotAdmin);
    }
}
```

---

## 9. Security Best Practices

### 9.1 Input Validation

**Always validate inputs**:
```rust
pub fn record_collection(
    env: Env,
    weight: u64,
    unit_price: i128
) -> u64 {
    // Validate weight
    if weight == 0 || weight > MAX_WEIGHT {
        panic_with_error!(&env, WasteFiError::InvalidWeight);
    }
    
    // Validate price
    if unit_price < MIN_PRICE || unit_price > MAX_PRICE {
        panic_with_error!(&env, WasteFiError::InvalidPrice);
    }
    
    // Proceed with logic
}
```

---

### 9.2 Access Control

**Implement proper authorization**:
```rust
pub fn update_price(env: Env, material: MaterialType, price: i128) {
    // Check admin authorization
    let admin: Address = env.storage().instance().get(&ADMIN)
        .expect("Admin not set");
    
    admin.require_auth();
    
    // Perform update
}
```

---

### 9.3 Reentrancy Protection

**Be aware of cross-contract call risks**:
```rust
pub fn process_payment(env: Env, transaction_id: u64) -> u64 {
    // Check if already processed (state check FIRST)
    if is_payment_processed(&env, transaction_id) {
        panic_with_error!(&env, WasteFiError::PaymentAlreadyProcessed);
    }
    
    // Mark as processing BEFORE external call
    mark_payment_processing(&env, transaction_id);
    
    // External call (to token contract)
    mint_tokens(&env, recipient, amount);
    
    // Finalize
    mark_payment_completed(&env, transaction_id);
    
    payment_id
}
```

---

## 10. Debugging and Troubleshooting

### 10.1 Common Build Issues

**Issue: "target not found"**
```bash
# Solution: Install wasm32 target
rustup target add wasm32-unknown-unknown
```

**Issue: "failed to compile"**
```bash
# Solution: Update dependencies
cargo update
cargo clean
cargo build
```

---

### 10.2 Testing Issues

**Issue: "Test panicked"**
- Check panic message for error code
- Verify test setup (addresses, initialization)
- Check authorization in test

**Issue: "Transaction simulation failed"**
- Verify contract is deployed
- Check network configuration
- Verify account has sufficient balance

---

### 10.3 Deployment Issues

**Issue: "Insufficient balance"**
```bash
# Check balance
soroban contract invoke --network testnet --source deployer -- balance --id $(soroban keys address deployer)

# Fund account (testnet)
curl "https://friendbot.stellar.org?addr=$(soroban keys address deployer)"
```

**Issue: "Contract not found"**
- Verify contract ID is correct
- Check you're on the correct network
- Ensure contract was deployed successfully

---

## Appendix: Useful Commands

### Build Commands
```bash
cargo build --target wasm32-unknown-unknown --release
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all --check
```

### Deployment Commands
```bash
# Deploy contract
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/<contract>.wasm --source deployer --network testnet

# Initialize contract
soroban contract invoke --id <CONTRACT_ID> --source deployer --network testnet -- initialize --admin <ADMIN_ADDRESS>

# Query contract
soroban contract invoke --id <CONTRACT_ID> --network testnet -- <method> --<param> <value>
```

### Testing Commands
```bash
cargo test
cargo test --test integration_e2e
cargo tarpaulin --workspace --out Html
```

---

## Resources

**Official Documentation**:
- Soroban: https://soroban.stellar.org
- Stellar: https://developers.stellar.org
- Rust: https://doc.rust-lang.org

**WasteFi Resources**:
- GitHub: https://github.com/wastefi-africa/wastefi-contracts
- Documentation: https://docs.wastefi.io (TBD)
- Discord: https://discord.gg/wastefi (TBD)

---

## Document Maintenance

**Version**: 1.0.0  
**Last Updated**: September 11, 2026  
**Maintainer**: WasteFi Development Team  
**Next Review**: After major version releases

---

**END OF DEVELOPER GUIDE**

For questions: dev@wastefi.io
