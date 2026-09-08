# WasteFi Smart Contract API Reference

Complete API documentation for all WasteFi smart contracts.

## Table of Contents

1. [WasteToken](#wastetoken)
2. [CollectorRegistry](#collectorregistry)
3. [CollectionPoint](#collectionpoint)
4. [WasteTransaction](#wastetransaction)
5. [PaymentDistribution](#paymentdistribution)
6. [Reputation](#reputation)
7. [MaterialPricing](#materialpricing)
8. [Common Types](#common-types)
9. [Error Codes](#error-codes)

## WasteToken

ERC-20 compatible token for WasteFi rewards.

### Functions

#### `initialize`
Initialize the token contract.

**Parameters:**
- `admin`: Address - Admin address
- `name`: String - Token name
- `symbol`: String - Token symbol
- `decimals`: u32 - Number of decimals

**Returns:** `()`

**Errors:**
- `AlreadyInitialized` (101)
- `InvalidInput` (103)

**Example:**
```bash
soroban contract invoke --id <ID> -- initialize \
  --admin <ADMIN> \
  --name "WasteFi Token" \
  --symbol "WASTE" \
  --decimals 7
```

#### `mint`
Mint new tokens (admin only).

**Parameters:**
- `to`: Address - Recipient address
- `amount`: i128 - Amount to mint

**Returns:** `()`

**Errors:**
- `NotAdmin` (900)
- `InvalidTokenAmount` (800)

#### `burn`
Burn tokens.

**Parameters:**
- `from`: Address - Address to burn from
- `amount`: i128 - Amount to burn

**Returns:** `()`

#### `balance`
Get token balance.

**Parameters:**
- `account`: Address - Account address

**Returns:** `i128` - Token balance

#### `transfer`
Transfer tokens.

**Parameters:**
- `from`: Address - Sender address
- `to`: Address - Recipient address
- `amount`: i128 - Amount to transfer

**Returns:** `()`

---

## CollectorRegistry

Manages collector registration and profiles.

### Functions

#### `initialize`
Initialize the registry.

**Parameters:**
- `admin`: Address - Admin address

**Returns:** `()`

#### `register`
Register a new collector.

**Parameters:**
- `collector`: Address - Collector address
- `name`: String - Collector name
- `phone`: String - Phone number

**Returns:** `()`

**Errors:**
- `CollectorAlreadyRegistered` (201)
- `InvalidInput` (103)

**Example:**
```bash
soroban contract invoke --id <ID> -- register \
  --collector <ADDRESS> \
  --name "John Doe" \
  --phone "+1234567890"
```

#### `get_collector`
Get collector information.

**Parameters:**
- `collector`: Address - Collector address

**Returns:** `Collector` - Collector data

**Errors:**
- `CollectorNotFound` (200)

#### `update_status`
Update collector status (admin only).

**Parameters:**
- `collector`: Address - Collector address
- `status`: CollectorStatus - New status

**Returns:** `()`

#### `is_active`
Check if collector is active.

**Parameters:**
- `collector`: Address - Collector address

**Returns:** `bool` - Active status

---

## CollectionPoint

Manages waste collection points.

### Functions

#### `initialize`
Initialize the contract.

**Parameters:**
- `admin`: Address - Admin address

**Returns:** `()`

#### `register_point`
Register a new collection point.

**Parameters:**
- `owner`: Address - Owner address
- `name`: String - Point name
- `location`: String - Location description
- `accepted_materials`: Vec<MaterialType> - Accepted materials

**Returns:** `u64` - Collection point ID

**Example:**
```bash
soroban contract invoke --id <ID> -- register_point \
  --owner <ADDRESS> \
  --name "Downtown Station" \
  --location "123 Main St" \
  --accepted_materials '[0, 1, 2]'
```

#### `verify_point`
Verify a collection point (admin only).

**Parameters:**
- `point_id`: u64 - Point ID

**Returns:** `()`

#### `get_point`
Get collection point details.

**Parameters:**
- `point_id`: u64 - Point ID

**Returns:** `CollectionPoint` - Point data

---

## WasteTransaction

Records waste collection transactions.

### Functions

#### `initialize`
Initialize the contract.

**Parameters:**
- `admin`: Address - Admin address

**Returns:** `()`

#### `record_collection`
Record a new waste collection.

**Parameters:**
- `collector`: Address - Collector address
- `collection_point`: Address - Point address
- `material_type`: MaterialType - Material type (0-9)
- `weight`: u64 - Weight in grams

**Returns:** `u64` - Transaction ID

**Example:**
```bash
soroban contract invoke --id <ID> -- record_collection \
  --collector <ADDRESS> \
  --collection_point <ADDRESS> \
  --material_type 0 \
  --weight 5000
```

#### `get_transaction`
Get transaction details.

**Parameters:**
- `transaction_id`: u64 - Transaction ID

**Returns:** `WasteRecord` - Transaction data

#### `verify_transaction`
Verify a transaction (admin only).

**Parameters:**
- `transaction_id`: u64 - Transaction ID

**Returns:** `()`

#### `get_collector_transactions`
Get all transactions for a collector.

**Parameters:**
- `collector`: Address - Collector address

**Returns:** `Vec<WasteRecord>` - Transaction list

---

## PaymentDistribution

Handles payment processing and distribution.

### Functions

#### `initialize`
Initialize the contract.

**Parameters:**
- `admin`: Address - Admin address
- `token_contract`: Address - WasteToken address

**Returns:** `()`

#### `process_payment`
Process payment for a transaction.

**Parameters:**
- `transaction_id`: u64 - Transaction ID

**Returns:** `u64` - Payment ID

**Example:**
```bash
soroban contract invoke --id <ID> -- process_payment \
  --transaction_id 1
```

#### `get_payment`
Get payment details.

**Parameters:**
- `payment_id`: u64 - Payment ID

**Returns:** `Payment` - Payment data

#### `batch_process`
Process multiple payments.

**Parameters:**
- `transaction_ids`: Vec<u64> - Transaction IDs

**Returns:** `Vec<u64>` - Payment IDs

---

## Reputation

Manages collector reputation scores.

### Functions

#### `initialize`
Initialize the contract.

**Parameters:**
- `admin`: Address - Admin address

**Returns:** `()`

#### `update_score`
Update reputation score.

**Parameters:**
- `collector`: Address - Collector address
- `transaction_successful`: bool - Transaction outcome

**Returns:** `()`

#### `get_score`
Get reputation score.

**Parameters:**
- `collector`: Address - Collector address

**Returns:** `ReputationScore` - Reputation data

**Example:**
```bash
soroban contract invoke --id <ID> -- get_score \
  --collector <ADDRESS>
```

#### `calculate_score`
Calculate new reputation score.

**Parameters:**
- `collector`: Address - Collector address

**Returns:** `u32` - Calculated score (0-1000)

---

## MaterialPricing

Oracle for material prices.

### Functions

#### `initialize`
Initialize the contract.

**Parameters:**
- `admin`: Address - Admin address

**Returns:** `()`

#### `set_price`
Set price for a material (admin only).

**Parameters:**
- `material_type`: MaterialType - Material type
- `price_per_kg`: i128 - Price in stroops per kg

**Returns:** `()`

**Example:**
```bash
soroban contract invoke --id <ID> -- set_price \
  --material_type 0 \
  --price_per_kg 10000000
```

#### `get_price`
Get price for a material.

**Parameters:**
- `material_type`: MaterialType - Material type

**Returns:** `i128` - Price per kg

#### `get_all_prices`
Get all material prices.

**Parameters:** None

**Returns:** `Vec<MaterialPrice>` - All prices

---

## Common Types

### MaterialType (enum)
```rust
Plastic = 0
Glass = 1
Metal = 2
Paper = 3
Cardboard = 4
Electronics = 5
Organic = 6
Textile = 7
Rubber = 8
Other = 9
```

### CollectorStatus (enum)
```rust
Pending = 0
Active = 1
Suspended = 2
Banned = 3
```

### VerificationStatus (enum)
```rust
Unverified = 0
Pending = 1
Verified = 2
Rejected = 3
```

### TransactionStatus (enum)
```rust
Pending = 0
Completed = 1
Disputed = 2
Cancelled = 3
```

### PaymentStatus (enum)
```rust
Pending = 0
Processing = 1
Completed = 2
Failed = 3
Refunded = 4
```

---

## Error Codes

### General (100-199)
- `100` - NotInitialized
- `101` - AlreadyInitialized
- `102` - Unauthorized
- `103` - InvalidInput
- `104` - NotFound
- `105` - AlreadyExists

### Collector (200-299)
- `200` - CollectorNotFound
- `201` - CollectorAlreadyRegistered
- `202` - CollectorNotActive
- `203` - CollectorSuspended
- `204` - CollectorBanned

### Collection Point (300-399)
- `300` - CollectionPointNotFound
- `301` - CollectionPointNotVerified
- `303` - MaterialNotAccepted

### Transaction (400-499)
- `400` - TransactionNotFound
- `404` - InvalidWeight
- `405` - InvalidAmount

### Payment (500-599)
- `500` - PaymentNotFound
- `502` - InsufficientBalance
- `503` - PaymentFailed

### Access Control (900-999)
- `900` - NotAdmin
- `903` - ContractPaused

---

## Usage Examples

### Complete Workflow

```bash
# 1. Register collector
soroban contract invoke --id $COLLECTOR_REGISTRY -- register \
  --collector $USER \
  --name "Alice" \
  --phone "+1234567890"

# 2. Record waste collection
TX_ID=$(soroban contract invoke --id $WASTE_TRANSACTION -- record_collection \
  --collector $USER \
  --collection_point $POINT \
  --material_type 0 \
  --weight 5000)

# 3. Process payment
soroban contract invoke --id $PAYMENT_DISTRIBUTION -- process_payment \
  --transaction_id $TX_ID

# 4. Check balance
soroban contract invoke --id $WASTE_TOKEN -- balance \
  --account $USER
```

For more examples, see [DEVELOPMENT.md](DEVELOPMENT.md).
