# WasteFi Platform API Reference

## Document Purpose

This document provides complete API reference documentation for all WasteFi smart contracts. It includes method signatures, parameters, return values, error codes, usage examples, and integration patterns.

**Target Audience**: Developers, integrators, frontend developers  
**Version**: 1.0.0  
**Last Updated**: September 11, 2026  
**Platform**: Stellar Soroban

---

## Table of Contents

1. [Overview](#1-overview)
2. [Common Types and Errors](#2-common-types-and-errors)
3. [CollectorRegistry Contract](#3-collectorregistry-contract)
4. [CollectionPoint Contract](#4-collectionpoint-contract)
5. [WasteTransaction Contract](#5-wastetransaction-contract)
6. [PaymentDistribution Contract](#6-paymentdistribution-contract)
7. [MaterialPricing Contract](#7-materialpricing-contract)
8. [Reputation Contract](#8-reputation-contract)
9. [WasteToken Contract](#9-wastetoken-contract)
10. [Integration Patterns](#10-integration-patterns)
11. [Error Handling](#11-error-handling)

---

## 1. Overview

### 1.1 Contract Addresses

Contract addresses vary by network. Use the appropriate addresses from your deployment:

```json
{
  "network": "testnet",
  "contracts": {
    "collector_registry": "CC...",
    "collection_point": "CD...",
    "waste_transaction": "CE...",
    "payment_distribution": "CF...",
    "material_pricing": "CG...",
    "reputation": "CH...",
    "waste_token": "CI..."
  }
}
```

### 1.2 Authentication

All write operations require authentication via Stellar signatures. Read operations are public.

```typescript
// Example: Authenticated call
const result = await contract.invoke({
  method: 'register',
  args: [collector, name, phone],
  source: userKeypair,  // Signs the transaction
  network: 'testnet'
});
```

### 1.3 Rate Limits

Default rate limits (configurable per deployment):
- **Registration**: 3 per day per address
- **Transaction submission**: 20 per hour per collector
- **Queries**: 60 per minute per address

---

## 2. Common Types and Errors

### 2.1 Common Types

#### Address
```rust
pub type Address = stellar_sdk::Address;
// Example: "GABC...XYZ" (56 characters, starts with G)
```

#### MaterialType
```rust
pub enum MaterialType {
    Plastic,
    Paper,
    Metal,
    Glass,
    Organic,
}
```

#### CollectorStatus
```rust
pub enum CollectorStatus {
    Active,      // Can submit transactions
    Suspended,   // Temporarily disabled
    Banned,      // Permanently disabled
    Pending,     // Awaiting verification
}
```

#### TransactionStatus
```rust
pub enum TransactionStatus {
    Pending,     // Awaiting verification
    Verified,    // Verified by collection point
    Rejected,    // Rejected
    Paid,        // Payment processed
}
```

#### EmergencyLevel
```rust
pub enum EmergencyLevel {
    Normal = 0,      // No restrictions
    Warning = 1,     // Monitoring active
    Critical = 2,    // Some operations restricted
    Shutdown = 3,    // All operations halted
}
```

---

### 2.2 Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 1 | `NotInitialized` | Contract not initialized |
| 2 | `AlreadyInitialized` | Contract already initialized |
| 3 | `Unauthorized` | Caller not authorized |
| 4 | `InvalidInput` | Invalid input parameter |
| 5 | `NotFound` | Resource not found |
| 6 | `AlreadyExists` | Resource already exists |
| 10 | `CollectorNotFound` | Collector not registered |
| 11 | `CollectorAlreadyRegistered` | Collector already exists |
| 12 | `CollectorNotActive` | Collector not in active status |
| 13 | `CollectorSuspended` | Collector is suspended |
| 14 | `InvalidCollectorStatus` | Invalid status value |
| 20 | `CollectionPointNotFound` | Collection point not found |
| 21 | `CollectionPointNotVerified` | Point not verified |
| 22 | `MaterialNotAccepted` | Material type not accepted |
| 30 | `TransactionNotFound` | Transaction not found |
| 31 | `InvalidTransactionStatus` | Invalid status |
| 32 | `InvalidWeight` | Weight out of bounds |
| 33 | `InvalidAmount` | Amount out of bounds |
| 40 | `PaymentNotFound` | Payment record not found |
| 41 | `PaymentAlreadyProcessed` | Payment already made |
| 42 | `InsufficientBalance` | Not enough balance |
| 43 | `PaymentFailed` | Payment processing failed |
| 50 | `ReputationNotFound` | No reputation record |
| 51 | `InvalidReputationScore` | Score out of range |
| 60 | `PriceNotSet` | Price not configured |
| 61 | `InvalidPrice` | Price out of bounds |
| 62 | `PriceUpdateTooFrequent` | Update too soon |
| 70 | `InvalidTokenAmount` | Token amount invalid |
| 71 | `TransferFailed` | Token transfer failed |
| 72 | `MintingDisabled` | Minting not allowed |
| 80 | `NotAdmin` | Admin privileges required |
| 81 | `NotOwner` | Owner privileges required |
| 82 | `ContractPaused` | Contract is paused |
| 90 | `EmergencyActive` | Emergency mode active |
| 91 | `EmergencyShutdown` | System in shutdown |
| 92 | `CircuitBreakerTripped` | Circuit breaker active |
| 93 | `OperationThrottled` | Rate limit exceeded |
| 95 | `FraudDetected` | Fraudulent activity detected |
| 96 | `DuplicateTransaction` | Duplicate submission |

---

## 3. CollectorRegistry Contract

**Purpose**: Manages collector registration, profiles, and status

**Contract ID**: `<COLLECTOR_REGISTRY_CONTRACT_ID>`

---

### 3.1 Initialization

#### `initialize`

Initialize the contract with an admin.

**Signature**:
```rust
pub fn initialize(env: Env, admin: Address)
```

**Parameters**:
- `admin` (Address): Admin account address

**Returns**: `()`

**Errors**:
- `AlreadyInitialized` (2): Contract already initialized

**Authorization**: None (callable once)

**Example**:
```bash
soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin GABC...XYZ
```

---

### 3.2 Collector Management

#### `register`

Register a new collector.

**Signature**:
```rust
pub fn register(env: Env, collector: Address, name: String, phone: String)
```

**Parameters**:
- `collector` (Address): Collector's address
- `name` (String): Collector's full name
- `phone` (String): Contact phone number

**Returns**: `()`

**Errors**:
- `CollectorAlreadyRegistered` (11): Collector already exists
- `InvalidInput` (4): Name or phone invalid
- `OperationThrottled` (93): Rate limit exceeded

**Authorization**: `collector` must sign

**Rate Limit**: 3 per day per address

**Events**:
```rust
CollectorRegistered {
    collector: Address,
    name: String,
    timestamp: u64
}
```

**Example**:
```bash
soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --source collector-key \
  --network testnet \
  -- \
  register \
  --collector GCOL...123 \
  --name "John Doe" \
  --phone "+1234567890"
```

**TypeScript Example**:
```typescript
const result = await collectorRegistry.register({
  collector: userAddress,
  name: "John Doe",
  phone: "+1234567890"
}, {
  source: userKeypair
});
```

---

#### `get_collector`

Retrieve collector information.

**Signature**:
```rust
pub fn get_collector(env: Env, collector: Address) -> Collector
```

**Parameters**:
- `collector` (Address): Collector's address

**Returns**: `Collector` object
```rust
pub struct Collector {
    pub address: Address,
    pub name: String,
    pub phone: String,
    pub status: CollectorStatus,
    pub total_weight: u64,         // kg
    pub total_transactions: u64,
    pub reputation_score: u32,      // 0-1000
    pub registration_time: u64,     // Unix timestamp
}
```

**Errors**:
- `CollectorNotFound` (10): Collector not registered

**Authorization**: Public (no signature required)

**Example**:
```bash
soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --network testnet \
  -- \
  get_collector \
  --collector GCOL...123
```

---

#### `update_status`

Update collector status (admin only).

**Signature**:
```rust
pub fn update_status(env: Env, collector: Address, status: CollectorStatus)
```

**Parameters**:
- `collector` (Address): Collector's address
- `status` (CollectorStatus): New status (Active, Suspended, Banned, Pending)

**Returns**: `()`

**Errors**:
- `CollectorNotFound` (10): Collector not registered
- `NotAdmin` (80): Caller is not admin
- `InvalidCollectorStatus` (14): Invalid status value

**Authorization**: Admin only

**Events**:
```rust
CollectorStatusUpdated {
    collector: Address,
    old_status: CollectorStatus,
    new_status: CollectorStatus,
    timestamp: u64
}
```

**Example**:
```bash
soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --source admin \
  --network mainnet \
  -- \
  update_status \
  --collector GCOL...123 \
  --status Suspended
```

---

#### `update_profile`

Update collector profile information.

**Signature**:
```rust
pub fn update_profile(env: Env, collector: Address, name: String, phone: String)
```

**Parameters**:
- `collector` (Address): Collector's address
- `name` (String): Updated name
- `phone` (String): Updated phone number

**Returns**: `()`

**Errors**:
- `CollectorNotFound` (10): Collector not registered
- `Unauthorized` (3): Caller is not the collector
- `InvalidInput` (4): Name or phone invalid

**Authorization**: `collector` must sign

**Example**:
```bash
soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --source collector-key \
  --network testnet \
  -- \
  update_profile \
  --collector GCOL...123 \
  --name "Jane Doe" \
  --phone "+9876543210"
```

---

#### `is_active`

Check if collector is active.

**Signature**:
```rust
pub fn is_active(env: Env, collector: Address) -> bool
```

**Parameters**:
- `collector` (Address): Collector's address

**Returns**: `bool` (true if status is Active)

**Errors**: None

**Authorization**: Public

**Example**:
```bash
soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --network testnet \
  -- \
  is_active \
  --collector GCOL...123
```

---

### 3.3 Batch Operations

#### `batch_register`

Register multiple collectors in one transaction.

**Signature**:
```rust
pub fn batch_register(
    env: Env,
    collectors: Vec<Address>,
    names: Vec<String>,
    phones: Vec<String>
) -> u32
```

**Parameters**:
- `collectors` (Vec<Address>): List of collector addresses
- `names` (Vec<String>): List of names (same length as collectors)
- `phones` (Vec<String>): List of phones (same length as collectors)

**Returns**: `u32` - Number of successfully registered collectors

**Errors**:
- `InvalidInput` (4): Array lengths don't match
- `OperationThrottled` (93): Rate limit exceeded

**Authorization**: Admin only

**Example**:
```bash
soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --source admin \
  --network testnet \
  -- \
  batch_register \
  --collectors '["GCOL1...123", "GCOL2...456"]' \
  --names '["John Doe", "Jane Smith"]' \
  --phones '["+1234567890", "+9876543210"]'
```

---

### 3.4 Query Operations

#### `get_collector_count`

Get total number of registered collectors.

**Signature**:
```rust
pub fn get_collector_count(env: Env) -> u64
```

**Returns**: `u64` - Total count

**Authorization**: Public

---

#### `get_all_collectors`

Get paginated list of collector addresses.

**Signature**:
```rust
pub fn get_all_collectors(env: Env, start: u64, limit: u64) -> Vec<Address>
```

**Parameters**:
- `start` (u64): Starting index (0-based)
- `limit` (u64): Maximum number to return (max 100)

**Returns**: `Vec<Address>` - List of collector addresses

**Authorization**: Public

**Example**:
```bash
# Get first 50 collectors
soroban contract invoke \
  --id $COLLECTOR_REGISTRY \
  --network testnet \
  -- \
  get_all_collectors \
  --start 0 \
  --limit 50
```

---

#### `get_collectors_by_status`

Get collectors with specific status.

**Signature**:
```rust
pub fn get_collectors_by_status(
    env: Env,
    status: CollectorStatus,
    start: u64,
    limit: u64
) -> Vec<Address>
```

**Parameters**:
- `status` (CollectorStatus): Status to filter by
- `start` (u64): Starting index
- `limit` (u64): Maximum results (max 100)

**Returns**: `Vec<Address>`

**Authorization**: Public

---

#### `get_top_collectors_by_weight`

Get top collectors by total weight collected.

**Signature**:
```rust
pub fn get_top_collectors_by_weight(env: Env, limit: u64) -> Vec<Address>
```

**Parameters**:
- `limit` (u64): Number of top collectors (max 100)

**Returns**: `Vec<Address>` - Sorted by weight (descending)

**Authorization**: Public

---

#### `search_collectors_by_name`

Search collectors by name (fuzzy match).

**Signature**:
```rust
pub fn search_collectors_by_name(
    env: Env,
    query: String,
    limit: u64
) -> Vec<Address>
```

**Parameters**:
- `query` (String): Search query
- `limit` (u64): Maximum results (max 50)

**Returns**: `Vec<Address>` - Matching collectors

**Authorization**: Public

---

### 3.5 Admin Operations

#### `admin`

Get admin address.

**Signature**:
```rust
pub fn admin(env: Env) -> Address
```

**Returns**: `Address` - Admin address

**Authorization**: Public

---

#### `pause` / `unpause`

Pause or unpause contract operations.

**Signature**:
```rust
pub fn pause(env: Env)
pub fn unpause(env: Env)
```

**Authorization**: Admin only

**Effects**: When paused, only query operations allowed

---

#### `add_operator` / `remove_operator`

Manage operator addresses (can perform some admin actions).

**Signature**:
```rust
pub fn add_operator(env: Env, operator: Address)
pub fn remove_operator(env: Env, operator: Address)
```

**Authorization**: Admin only

---

### 3.6 Emergency Operations

#### `trigger_emergency`

Activate emergency mode.

**Signature**:
```rust
pub fn trigger_emergency(env: Env, level: u32, reason: String)
```

**Parameters**:
- `level` (u32): Emergency level (0=Normal, 1=Warning, 2=Critical, 3=Shutdown)
- `reason` (String): Reason for emergency

**Authorization**: Admin only

---

## 4. CollectionPoint Contract

**Purpose**: Manages collection point registry and waste collection verification

**Contract ID**: `<COLLECTION_POINT_CONTRACT_ID>`

### 4.1 Key Methods

#### `register_point`

Register a new collection point.

**Signature**:
```rust
pub fn register_point(
    env: Env,
    point: Address,
    name: String,
    location: String,
    accepted_materials: Vec<MaterialType>
)
```

**Parameters**:
- `point` (Address): Collection point address
- `name` (String): Point name
- `location` (String): Physical location
- `accepted_materials` (Vec<MaterialType>): Materials accepted

**Authorization**: `point` must sign

**Example**:
```bash
soroban contract invoke \
  --id $COLLECTION_POINT \
  --source point-key \
  --network testnet \
  -- \
  register_point \
  --point GPOINT...789 \
  --name "Downtown Collection" \
  --location "123 Main St" \
  --accepted_materials '["Plastic", "Paper", "Metal"]'
```

---

#### `verify_collection`

Verify a waste collection transaction.

**Signature**:
```rust
pub fn verify_collection(
    env: Env,
    point: Address,
    transaction_id: u64,
    verified: bool
)
```

**Parameters**:
- `point` (Address): Collection point address
- `transaction_id` (u64): Transaction to verify
- `verified` (bool): true to verify, false to reject

**Authorization**: `point` must sign

**Effects**: Updates transaction status in WasteTransaction contract

---

## 5. WasteTransaction Contract

**Purpose**: Records and manages waste collection transactions

**Contract ID**: `<WASTE_TRANSACTION_CONTRACT_ID>`

---

### 5.1 Configuration

#### `set_material_pricing_contract`

Set the MaterialPricing contract address (admin only).

**Signature**:
```rust
pub fn set_material_pricing_contract(env: Env, contract_address: Address)
```

**Authorization**: Admin only

---

#### `set_reputation_contract`

Set the Reputation contract address (admin only).

**Signature**:
```rust
pub fn set_reputation_contract(env: Env, contract_address: Address)
```

**Authorization**: Admin only

---

### 5.2 Transaction Management

#### `record_collection`

Record a new waste collection.

**Signature**:
```rust
pub fn record_collection(
    env: Env,
    collector: Address,
    collection_point: Address,
    material_type: MaterialType,
    weight: u64,
    unit_price: i128
) -> u64
```

**Parameters**:
- `collector` (Address): Collector address
- `collection_point` (Address): Collection point address
- `material_type` (MaterialType): Type of waste (Plastic, Paper, etc.)
- `weight` (u64): Weight in grams
- `unit_price` (i128): Price per kg in stroops

**Returns**: `u64` - Transaction ID

**Errors**:
- `CollectorNotActive` (12): Collector not active
- `InvalidWeight` (32): Weight is 0 or too large
- `InvalidAmount` (33): Price out of bounds
- `DuplicateTransaction` (96): Duplicate detected
- `FraudDetected` (95): Fraud risk too high
- `OperationThrottled` (93): Rate limit exceeded

**Authorization**: `collector` must sign

**Rate Limit**: 20 per hour per collector

**Events**:
```rust
CollectionRecorded {
    transaction_id: u64,
    collector: Address,
    collection_point: Address,
    material_type: MaterialType,
    weight: u64,
    amount: i128,
    timestamp: u64
}
```

**Example**:
```bash
soroban contract invoke \
  --id $WASTE_TRANSACTION \
  --source collector-key \
  --network testnet \
  -- \
  record_collection \
  --collector GCOL...123 \
  --collection_point GPOINT...789 \
  --material_type Plastic \
  --weight 5000 \
  --unit_price 100
```

**TypeScript Example**:
```typescript
const txId = await wasteTransaction.recordCollection({
  collector: userAddress,
  collectionPoint: pointAddress,
  materialType: "Plastic",
  weight: 5000,  // 5 kg in grams
  unitPrice: 100  // stroops per kg
}, {
  source: userKeypair
});

console.log(`Transaction recorded: ${txId}`);
```

---

#### `record_with_price_lookup`

Record collection with automatic price lookup.

**Signature**:
```rust
pub fn record_with_price_lookup(
    env: Env,
    collector: Address,
    collection_point: Address,
    material_type: MaterialType,
    weight: u64
) -> u64
```

**Parameters**:
- `collector` (Address): Collector address
- `collection_point` (Address): Collection point address
- `material_type` (MaterialType): Material type
- `weight` (u64): Weight in grams

**Returns**: `u64` - Transaction ID

**Note**: Automatically fetches current price from MaterialPricing contract

**Authorization**: `collector` must sign

**Example**:
```bash
soroban contract invoke \
  --id $WASTE_TRANSACTION \
  --source collector-key \
  --network testnet \
  -- \
  record_with_price_lookup \
  --collector GCOL...123 \
  --collection_point GPOINT...789 \
  --material_type Paper \
  --weight 3000
```

---

#### `get_transaction`

Retrieve transaction details.

**Signature**:
```rust
pub fn get_transaction(env: Env, transaction_id: u64) -> WasteRecord
```

**Parameters**:
- `transaction_id` (u64): Transaction ID

**Returns**: `WasteRecord`
```rust
pub struct WasteRecord {
    pub transaction_id: u64,
    pub collector: Address,
    pub collection_point: Address,
    pub material_type: MaterialType,
    pub weight: u64,              // grams
    pub unit_price: i128,         // stroops per kg
    pub total_amount: i128,       // stroops
    pub status: TransactionStatus,
    pub timestamp: u64,           // Unix timestamp
    pub verification_time: Option<u64>,
    pub risk_score: u32,          // 0-1000
}
```

**Errors**:
- `TransactionNotFound` (30): Transaction doesn't exist

**Authorization**: Public

---

#### `verify_transaction`

Verify a transaction (collection point only).

**Signature**:
```rust
pub fn verify_transaction(env: Env, transaction_id: u64)
```

**Parameters**:
- `transaction_id` (u64): Transaction to verify

**Effects**:
- Updates status to `Verified`
- Updates reputation scores
- Enables payment processing

**Authorization**: Collection point associated with transaction

---

### 5.3 Batch Operations

#### `batch_record_collections`

Record multiple collections in one transaction.

**Signature**:
```rust
pub fn batch_record_collections(
    env: Env,
    collector: Address,
    collection_points: Vec<Address>,
    material_types: Vec<MaterialType>,
    weights: Vec<u64>,
    unit_prices: Vec<i128>
) -> Vec<u64>
```

**Returns**: `Vec<u64>` - List of transaction IDs

**Authorization**: `collector` must sign

---

### 5.4 Statistics and Queries

#### `get_collector_stats`

Get collector's transaction statistics.

**Signature**:
```rust
pub fn get_collector_stats(env: Env, collector: Address) -> (u64, u64, i128)
```

**Returns**: `(total_transactions, total_weight, total_earnings)`

**Authorization**: Public

---

#### `get_collector_transactions`

Get collector's transaction history (paginated).

**Signature**:
```rust
pub fn get_collector_transactions(
    env: Env,
    collector: Address,
    start: u64,
    limit: u64
) -> Vec<u64>
```

**Returns**: `Vec<u64>` - List of transaction IDs

**Authorization**: Public

---

#### `get_transactions_by_status`

Get transactions with specific status.

**Signature**:
```rust
pub fn get_transactions_by_status(
    env: Env,
    status: TransactionStatus,
    start: u64,
    limit: u64
) -> Vec<u64>
```

**Authorization**: Public

---

#### `get_material_statistics`

Get statistics for a specific material type.

**Signature**:
```rust
pub fn get_material_statistics(
    env: Env,
    material_type: MaterialType
) -> (u64, u64, i128)
```

**Returns**: `(transaction_count, total_weight_kg, total_value)`

**Authorization**: Public

---

### 5.5 Fraud Detection

#### `get_risk_score`

Get fraud risk score for a collector.

**Signature**:
```rust
pub fn get_risk_score(env: Env, collector: Address) -> u32
```

**Returns**: `u32` - Risk score (0-1000)
- 0-300: Low risk
- 301-600: Medium risk
- 601-800: High risk
- 801-1000: Critical risk

**Authorization**: Public

---

#### `flag_for_review`

Flag a collector for manual review (admin/operator only).

**Signature**:
```rust
pub fn flag_for_review(env: Env, collector: Address, reason: String)
```

**Authorization**: Admin or operator

---

## 6. PaymentDistribution Contract

**Purpose**: Calculates and distributes token rewards for verified collections

**Contract ID**: `<PAYMENT_DISTRIBUTION_CONTRACT_ID>`

---

### 6.1 Configuration

#### `set_token_contract`

Set the WasteToken contract address (admin only).

**Signature**:
```rust
pub fn set_token_contract(env: Env, token_address: Address)
```

**Authorization**: Admin only

---

### 6.2 Payment Processing

#### `process_payment`

Process payment for a verified transaction.

**Signature**:
```rust
pub fn process_payment(env: Env, transaction_id: u64) -> u64
```

**Parameters**:
- `transaction_id` (u64): Verified transaction ID

**Returns**: `u64` - Payment ID

**Errors**:
- `TransactionNotFound` (30): Transaction doesn't exist
- `InvalidTransactionStatus` (31): Transaction not verified
- `PaymentAlreadyProcessed` (41): Payment already made
- `InsufficientBalance` (42): Contract has insufficient tokens

**Effects**:
- Mints tokens to collector
- Records payment
- Updates transaction status to `Paid`

**Authorization**: Admin or automated process

**Events**:
```rust
PaymentProcessed {
    payment_id: u64,
    transaction_id: u64,
    collector: Address,
    amount: i128,
    timestamp: u64
}
```

**Example**:
```bash
soroban contract invoke \
  --id $PAYMENT_DISTRIBUTION \
  --source admin \
  --network testnet \
  -- \
  process_payment \
  --transaction_id 123
```

---

#### `batch_process_payments`

Process multiple payments in one transaction.

**Signature**:
```rust
pub fn batch_process_payments(env: Env, transaction_ids: Vec<u64>) -> u32
```

**Returns**: `u32` - Number of successful payments

**Authorization**: Admin only

---

#### `get_payment`

Retrieve payment details.

**Signature**:
```rust
pub fn get_payment(env: Env, payment_id: u64) -> Payment
```

**Returns**: `Payment` object
```rust
pub struct Payment {
    pub payment_id: u64,
    pub transaction_id: u64,
    pub collector: Address,
    pub amount: i128,
    pub timestamp: u64,
    pub status: PaymentStatus,
}
```

**Authorization**: Public

---

### 6.3 Statistics

#### `get_total_payments`

Get total number of payments processed.

**Signature**:
```rust
pub fn get_total_payments(env: Env) -> u64
```

**Authorization**: Public

---

#### `get_collector_payments`

Get collector's payment history.

**Signature**:
```rust
pub fn get_collector_payments(
    env: Env,
    collector: Address,
    start: u64,
    limit: u64
) -> Vec<u64>
```

**Returns**: `Vec<u64>` - List of payment IDs

**Authorization**: Public

---

## 7. MaterialPricing Contract

**Purpose**: Manages pricing for different waste material types

**Contract ID**: `<MATERIAL_PRICING_CONTRACT_ID>`

---

### 7.1 Price Management

#### `update_price`

Update price for a material type (admin only).

**Signature**:
```rust
pub fn update_price(env: Env, material: MaterialType, price: i128)
```

**Parameters**:
- `material` (MaterialType): Material type
- `price` (i128): Price per kg in stroops

**Errors**:
- `InvalidPrice` (61): Price out of bounds (0 to max_price)
- `PriceUpdateTooFrequent` (62): Updated too recently (min 1 hour)

**Authorization**: Admin only

**Events**:
```rust
PriceUpdated {
    material: MaterialType,
    old_price: i128,
    new_price: i128,
    timestamp: u64
}
```

**Example**:
```bash
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source admin \
  --network testnet \
  -- \
  update_price \
  --material Plastic \
  --price 120
```

---

#### `get_price`

Get current price for a material.

**Signature**:
```rust
pub fn get_price(env: Env, material: MaterialType) -> i128
```

**Returns**: `i128` - Price per kg in stroops

**Errors**:
- `PriceNotSet` (60): Price not configured for material

**Authorization**: Public

**Example**:
```bash
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --network testnet \
  -- \
  get_price \
  --material Paper
```

---

#### `get_all_prices`

Get prices for all materials.

**Signature**:
```rust
pub fn get_all_prices(env: Env) -> Map<MaterialType, i128>
```

**Returns**: Map of material types to prices

**Authorization**: Public

---

#### `get_last_update`

Get timestamp of last price update for a material.

**Signature**:
```rust
pub fn get_last_update(env: Env, material: MaterialType) -> u64
```

**Returns**: `u64` - Unix timestamp

**Authorization**: Public

---

### 7.2 Batch Operations

#### `batch_update_prices`

Update multiple prices at once.

**Signature**:
```rust
pub fn batch_update_prices(
    env: Env,
    materials: Vec<MaterialType>,
    prices: Vec<i128>
)
```

**Authorization**: Admin only

**Example**:
```bash
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source admin \
  --network testnet \
  -- \
  batch_update_prices \
  --materials '["Plastic", "Paper", "Metal"]' \
  --prices '[120, 85, 160]'
```

---

## 8. Reputation Contract

**Purpose**: Tracks and manages reputation scores for collectors

**Contract ID**: `<REPUTATION_CONTRACT_ID>`

---

### 8.1 Score Management

#### `get_score`

Get reputation score for a collector.

**Signature**:
```rust
pub fn get_score(env: Env, user: Address) -> u32
```

**Returns**: `u32` - Reputation score (0-1000)
- Initial score: 500
- Good behavior increases score
- Bad behavior decreases score

**Authorization**: Public

**Example**:
```bash
soroban contract invoke \
  --id $REPUTATION \
  --network testnet \
  -- \
  get_score \
  --user GCOL...123
```

---

#### `update_score`

Update reputation score (authorized contracts only).

**Signature**:
```rust
pub fn update_score(env: Env, user: Address, delta: i32)
```

**Parameters**:
- `user` (Address): User address
- `delta` (i32): Change in score (positive or negative)

**Effects**: Score clamped to 0-1000 range

**Authorization**: Authorized contracts (WasteTransaction, PaymentDistribution) or admin

---

#### `get_reputation_level`

Get reputation level category.

**Signature**:
```rust
pub fn get_reputation_level(env: Env, user: Address) -> String
```

**Returns**: String
- "Excellent" (800-1000)
- "Good" (600-799)
- "Fair" (400-599)
- "Poor" (200-399)
- "Bad" (0-199)

**Authorization**: Public

---

### 8.2 History and Statistics

#### `get_score_history`

Get score change history for a user.

**Signature**:
```rust
pub fn get_score_history(
    env: Env,
    user: Address,
    limit: u32
) -> Vec<ScoreChange>
```

**Returns**: `Vec<ScoreChange>`
```rust
pub struct ScoreChange {
    pub old_score: u32,
    pub new_score: u32,
    pub reason: String,
    pub timestamp: u64,
}
```

**Authorization**: Public

---

## 9. WasteToken Contract

**Purpose**: ERC-20 style reward token for the platform

**Contract ID**: `<WASTE_TOKEN_CONTRACT_ID>`

---

### 9.1 Token Information

#### `name`

Get token name.

**Signature**:
```rust
pub fn name(env: Env) -> String
```

**Returns**: "WasteFi Token" (or "WasteFi Token Testnet")

---

#### `symbol`

Get token symbol.

**Signature**:
```rust
pub fn symbol(env: Env) -> String
```

**Returns**: "WASTE" (or "WASTE-TEST")

---

#### `decimals`

Get token decimals.

**Signature**:
```rust
pub fn decimals(env: Env) -> u32
```

**Returns**: `7` (Stellar standard)

---

### 9.2 Balance Operations

#### `balance`

Get token balance for an address.

**Signature**:
```rust
pub fn balance(env: Env, id: Address) -> i128
```

**Returns**: `i128` - Balance in stroops (10^-7 tokens)

**Example**:
```bash
soroban contract invoke \
  --id $WASTE_TOKEN \
  --network testnet \
  -- \
  balance \
  --id GCOL...123
```

---

#### `transfer`

Transfer tokens to another address.

**Signature**:
```rust
pub fn transfer(env: Env, from: Address, to: Address, amount: i128)
```

**Parameters**:
- `from` (Address): Sender address
- `to` (Address): Recipient address
- `amount` (i128): Amount in stroops

**Errors**:
- `InsufficientBalance` (42): Sender balance too low
- `InvalidTokenAmount` (70): Amount is negative or zero

**Authorization**: `from` must sign

---

### 9.3 Admin Operations

#### `mint`

Mint new tokens (admin only).

**Signature**:
```rust
pub fn mint(env: Env, to: Address, amount: i128)
```

**Parameters**:
- `to` (Address): Recipient address
- `amount` (i128): Amount to mint in stroops

**Errors**:
- `MintingDisabled` (72): Minting disabled
- `NotAdmin` (80): Caller is not admin

**Authorization**: Admin only

**Note**: Called automatically by PaymentDistribution contract

---

#### `burn`

Burn tokens from circulation.

**Signature**:
```rust
pub fn burn(env: Env, from: Address, amount: i128)
```

**Authorization**: Admin or `from` address

---

## 10. Integration Patterns

### 10.1 Complete Collection Workflow

```typescript
// 1. Check if collector is registered
const isActive = await collectorRegistry.isActive(collectorAddress);

if (!isActive) {
  // 2. Register collector
  await collectorRegistry.register({
    collector: collectorAddress,
    name: "John Doe",
    phone: "+1234567890"
  }, { source: collectorKeypair });
}

// 3. Record collection with automatic price lookup
const txId = await wasteTransaction.recordWithPriceLookup({
  collector: collectorAddress,
  collectionPoint: pointAddress,
  materialType: "Plastic",
  weight: 5000  // 5 kg
}, { source: collectorKeypair });

console.log(`Collection recorded: Transaction ${txId}`);

// 4. Collection point verifies (separate action)
// await collectionPoint.verifyCollection({
//   point: pointAddress,
//   transactionId: txId,
//   verified: true
// }, { source: pointKeypair });

// 5. Payment processed automatically (by backend service)
// await paymentDistribution.processPayment(txId);

// 6. Check token balance
const balance = await wasteToken.balance(collectorAddress);
console.log(`Current balance: ${balance / 10000000} WASTE`);
```

---

### 10.2 Admin Price Update Workflow

```bash
#!/bin/bash
# update_prices.sh - Weekly price update

# Get current prices for reference
echo "Current prices:"
for material in Plastic Paper Metal Glass Organic; do
  PRICE=$(soroban contract invoke \
    --id $MATERIAL_PRICING \
    --network mainnet \
    -- \
    get_price \
    --material $material)
  echo "  $material: $PRICE stroops/kg"
done

# Update prices based on market rates
soroban contract invoke \
  --id $MATERIAL_PRICING \
  --source mainnet-admin \
  --network mainnet \
  -- \
  batch_update_prices \
  --materials '["Plastic", "Paper", "Metal", "Glass", "Organic"]' \
  --prices '[120, 85, 160, 65, 45]'

echo "✓ Prices updated"
```

---

### 10.3 Monitoring Dashboard Integration

```javascript
// Dashboard data fetching
async function fetchDashboardData() {
  // Global statistics
  const totalCollectors = await collectorRegistry.getCollectorCount();
  const [totalTx, totalWeight, totalValue] = 
    await wasteTransaction.getGlobalTxStatistics();
  
  // Material breakdown
  const materials = ['Plastic', 'Paper', 'Metal', 'Glass', 'Organic'];
  const materialStats = await Promise.all(
    materials.map(m => wasteTransaction.getMaterialStatistics(m))
  );
  
  // Recent transactions
  const recentTx = await wasteTransaction.getRecentTransactions(10);
  
  // Top collectors
  const topCollectors = await collectorRegistry.getTopCollectorsByWeight(10);
  
  return {
    totalCollectors,
    totalTransactions: totalTx,
    totalWeight,
    totalValue,
    materialStats,
    recentTransactions: recentTx,
    topCollectors
  };
}
```

---

## 11. Error Handling

### 11.1 Error Handling Pattern

```typescript
import { WasteFiError } from './errors';

async function recordCollection(params) {
  try {
    const txId = await wasteTransaction.recordCollection(params);
    return { success: true, transactionId: txId };
  } catch (error) {
    // Parse Soroban error
    const errorCode = parseErrorCode(error);
    
    switch (errorCode) {
      case 12: // CollectorNotActive
        return {
          success: false,
          error: 'Collector account is not active. Please contact support.'
        };
      
      case 93: // OperationThrottled
        return {
          success: false,
          error: 'Rate limit exceeded. Please wait before submitting more transactions.'
        };
      
      case 95: // FraudDetected
        return {
          success: false,
          error: 'Transaction flagged for fraud review. Please contact support.'
        };
      
      case 96: // DuplicateTransaction
        return {
          success: false,
          error: 'This appears to be a duplicate submission. Please check your transaction history.'
        };
      
      default:
        return {
          success: false,
          error: `Transaction failed with error code ${errorCode}`
        };
    }
  }
}
```

---

### 11.2 Retry Logic

```typescript
async function recordWithRetry(params, maxRetries = 3) {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await wasteTransaction.recordCollection(params);
    } catch (error) {
      const errorCode = parseErrorCode(error);
      
      // Don't retry on client errors
      if ([4, 12, 95, 96].includes(errorCode)) {
        throw error;
      }
      
      // Retry on network/server errors
      if (attempt === maxRetries) {
        throw error;
      }
      
      // Exponential backoff
      await sleep(Math.pow(2, attempt) * 1000);
    }
  }
}
```

---

## Appendix: Quick Reference

### Contract Addresses (Example - Testnet)

```json
{
  "network": "testnet",
  "contracts": {
    "collector_registry": "CCABC...XYZ",
    "collection_point": "CDDEF...UVW",
    "waste_transaction": "CEGHI...RST",
    "payment_distribution": "CFJKL...OPQ",
    "material_pricing": "CGMNO...LMN",
    "reputation": "CHPQR...IJK",
    "waste_token": "CISTU...GHI"
  }
}
```

### Common Material Prices (Example)

| Material | Price (stroops/kg) | Price (WASTE/kg) |
|----------|-------------------|------------------|
| Plastic | 120 | 0.0000120 |
| Paper | 85 | 0.0000085 |
| Metal | 160 | 0.0000160 |
| Glass | 65 | 0.0000065 |
| Organic | 45 | 0.0000045 |

---

## Document Maintenance

**Version**: 1.0.0  
**Last Updated**: September 11, 2026  
**Maintainer**: WasteFi Development Team  
**Review Cycle**: After each contract upgrade

---

## Related Documentation

- `DEPLOYMENT.md` - Deployment procedures
- `OPERATIONS.md` - Operations runbook
- `DEVELOPER.md` - Developer guide
- `USER_GUIDE.md` - End-user documentation
- `SECURITY_AUDIT.md` - Security documentation

---

**END OF API REFERENCE**

For support: dev@wastefi.io
