# WasteFi Smart Contracts - Security Considerations

## Document Purpose

This document provides detailed security analysis for each contract, identifying critical functions, state invariants, edge cases, and integration security concerns.

---

## 1. Common Module Security

**Module**: `contracts/common/`  
**Security Level**: Critical (shared by all contracts)

### 1.1 Access Control (`access_control.rs`)

#### Critical Functions
- `require_admin()` - Authorization gate for privileged operations
- `transfer_admin()` - Admin role transfer
- `add_operator()` - Operator role grant
- `admin_pause()` / `admin_unpause()` - Emergency controls

#### Security Features
✅ All admin checks use `require_auth()` for cryptographic verification  
✅ Admin transfers logged to audit trail  
✅ No privilege escalation paths identified  
✅ Operator permissions scoped appropriately

#### State Invariants
- **INV-AC-1**: Only one admin address at any time
- **INV-AC-2**: Admin address cannot be zero
- **INV-AC-3**: Pause state can only be changed by admin
- **INV-AC-4**: Admin action log is append-only

####Edge Cases
- Admin transfers to self (allowed, logged)
- Operator added twice (idempotent)
- Pause when already paused (idempotent)
- Admin action log at capacity (prunes oldest)

#### Security Concerns
⚠️ **Single admin**: No multi-sig at contract level (operational concern)  
✅ **Mitigation**: Recommend multi-sig wallet for admin operations

---

### 1.2 Anti-Fraud (`anti_fraud.rs`)

#### Critical Functions
- `calculate_risk_score()` - Fraud detection algorithm
- `is_duplicate()` - Duplicate transaction detection
- `check_per_hour()` - Rate limiting enforcement
- `flag_for_review()` - Manual fraud flagging

#### Security Features
✅ Multi-factor risk scoring (velocity, rejections, weight, timing)  
✅ Configurable risk thresholds  
✅ Temporary storage for efficiency (auto-expires)  
✅ Admin-only fraud flag management

#### State Invariants
- **INV-AF-1**: Risk score is 0-1000
- **INV-AF-2**: Rate limit counters accurate within TTL
- **INV-AF-3**: Duplicate detection window configurable
- **INV-AF-4**: Fraud flags include timestamp and reason

#### Edge Cases
- Risk score calculation with no history (returns 0)
- Rate limit at exact threshold (blocked correctly)
- Duplicate detection across TTL boundary (may miss edge cases)
- Weight anomaly with <5 transactions (no detection)

#### Security Concerns
✅ **Evasion**: Sophisticated attackers may game scoring (acceptable, requires manual review)  
✅ **Temporary storage**: Data expires after TTL (design decision, reduces storage cost)  
⚠️ **Sybil attack**: Cannot prevent multiple addresses (requires off-chain KYC)

---

### 1.3 Emergency Response (`emergency.rs`)

#### Critical Functions
- `trigger()` - Emergency activation (admin-only)
- `resolve()` - Emergency resolution (admin-only)
- `trip()` / `reset()` - Circuit breaker control
- `enable()` - Emergency withdrawal activation

#### Security Features
✅ Four-level emergency system with auto-pause  
✅ Circuit breakers prevent cascading failures  
✅ Emergency withdrawal disabled by default  
✅ Complete audit trail for all emergency actions

#### State Invariants
- **INV-EM-1**: Emergency level is 0-3
- **INV-EM-2**: Critical/Shutdown auto-pauses contract
- **INV-EM-3**: Emergency events immutable once logged
- **INV-EM-4**: Circuit breaker reset clears trip time

#### Edge Cases
- Emergency trigger when already in emergency (updates level, logs new event)
- Circuit breaker trip when already tripped (updates timestamp)
- Emergency withdrawal with no funds (fails gracefully)
- Cooldown exactly met (allows operation)

#### Security Concerns
✅ **Admin dependency**: Emergency controls require admin (design decision)  
✅ **Manual activation**: Not automatic (requires monitoring and response)  
⚠️ **Emergency withdrawal**: Powerful capability (audit trail provides accountability)

---

### 1.4 Upgrade Management (`upgrade.rs`)

#### Critical Functions
- `upgrade_contract()` - WASM upgrade (admin-only)
- `mark_upgrade_in_progress()` - Upgrade locking
- `require_min_version()` - Version compatibility check
- `record_migration_step()` - Migration tracking

#### Security Features
✅ Semantic versioning with compatibility checks  
✅ Upgrade-in-progress flag prevents concurrent upgrades  
✅ Migration step tracking for debugging  
✅ Feature flags for gradual rollout

#### State Invariants
- **INV-UP-1**: Version never decreases (no downgrades)
- **INV-UP-2**: Only one upgrade at a time
- **INV-UP-3**: Migration log is append-only
- **INV-UP-4**: Schema version monotonically increasing

#### Edge Cases
- Upgrade to same version (allowed, updates timestamp)
- Migration failure mid-upgrade (logged, manual intervention required)
- Version compatibility check with no version set (fails safely)
- Feature flag toggle during active use (takes effect immediately)

#### Security Concerns
⚠️ **Upgrade risk**: Admin can deploy malicious code (inherent to upgradeability)  
✅ **Mitigation**: Audit trail, testnet testing, community review recommended  
✅ **Rollback**: Supported via downgrade WASM, but data may be lost

---

## 2. WasteToken Contract Security

**Contract**: `contracts/waste_token/`  
**Security Level**: Critical (financial asset)

### 2.1 Critical Functions

#### `mint(to: Address, amount: i128)`
**Purpose**: Create new tokens  
**Access**: Admin-only  
**Risks**: Unlimited minting could devalue token

**Security Controls**:
- ✅ `require_admin()` check
- ✅ Amount validation (positive, no overflow)
- ✅ Balance overflow check
- ✅ Minting event emitted
- ⚠️ No hard supply cap (can be added if needed)

**State Invariants**:
- Total supply = sum of all balances
- Balance[to] increases by exactly amount
- Event emitted with correct parameters

#### `transfer(from: Address, to: Address, amount: i128)`
**Purpose**: Move tokens between accounts  
**Access**: From address must authorize  
**Risks**: Unauthorized transfer, balance manipulation

**Security Controls**:
- ✅ `from.require_auth()` - cryptographic authorization
- ✅ Balance check before transfer
- ✅ Checked arithmetic (no overflow/underflow)
- ✅ Transfer event emitted

**State Invariants**:
- Total supply unchanged
- Balance[from] decreases by amount
- Balance[to] increases by amount
- No tokens created or destroyed

#### `burn(from: Address, amount: i128)`
**Purpose**: Destroy tokens  
**Access**: From address must authorize  
**Risks**: Permanent loss

**Security Controls**:
- ✅ `from.require_auth()`
- ✅ Balance validation
- ✅ Checked subtraction
- ✅ Total supply decreases

### 2.2 Edge Cases
- Transfer to self (allowed, no-op effectively)
- Transfer zero amount (allowed, emits event)
- Mint to zero address (should be blocked)
- Burn entire balance (allowed)

### 2.3 Integration Security
- Payment distribution mints tokens (admin access required)
- No external token calls (Stellar native assets only if needed)

---

## 3. CollectorRegistry Contract Security

**Contract**: `contracts/collector_registry/`  
**Security Level**: High (identity foundation)

### 3.1 Critical Functions

#### `register(name: String, contact: String)`
**Purpose**: Onboard new collector  
**Access**: Public  
**Risks**: Spam, Sybil attack

**Security Controls**:
- ✅ Rate limiting (3 registrations/hour per address)
- ✅ String validation (max length 256)
- ✅ Duplicate address check
- ✅ Fraud tracking initialization

**State Invariants**:
- Each address registered once
- Collector ID unique and sequential
- Status initially "Pending" or "Active"
- Registration timestamp recorded

#### `update_status(collector: Address, new_status: CollectorStatus)`
**Purpose**: Change collector verification status  
**Access**: Admin/Operator  
**Risks**: Unauthorized status changes

**Security Controls**:
- ✅ `require_elevated_access()` check
- ✅ Valid status transition enforcement
- ✅ Status change event emitted
- ✅ Fraud integration (flags checked)

**State Invariants**:
- Status transitions follow rules (Pending→Active, Active→Suspended, etc.)
- Status changes logged
- Cannot reactivate banned collector without admin

### 3.2 Edge Cases
- Register with empty name (rejected)
- Register while paused (rejected)
- Status change to current status (idempotent, emits event)
- Suspend already suspended collector (idempotent)

### 3.3 Integration Security
- WasteTransaction validates collector exists and is active
- Fraud detection checks collector status
- Reputation linked to collector ID

---

## 4. WasteTransaction Contract Security

**Contract**: `contracts/waste_transaction/`  
**Security Level**: Critical (fraud prevention core)

### 4.1 Critical Functions

#### `record_collection(collector, point, material, weight, price)`
**Purpose**: Record waste collection transaction  
**Access**: Public (collectors)  
**Risks**: Fraudulent submissions, spam, duplicates

**Security Controls**:
- ✅ Fraud detection: `require_not_critical()` blocks high-risk users
- ✅ Rate limiting: 20 transactions/hour per collector
- ✅ Duplicate detection: 5-minute window for same weight+material
- ✅ Weight validation: positive, reasonable bounds
- ✅ Collector must be active
- ✅ Collection point must be verified
- ✅ Material accepted at point

**State Invariants**:
- Transaction ID unique and sequential
- Initial status "Pending"
- Verification required before payment
- Fraud metrics updated per transaction
- Weight and pricing immutable after creation

**Edge Cases**:
- Exact rate limit boundary (20th transaction in hour blocked)
- Duplicate within tolerance (blocked correctly)
- Collection at unverified point (rejected)
- Weight exactly zero (rejected)
- Extremely large weight (fraud score increases)

#### `verify_transaction(tx_id: u64)`
**Purpose**: Admin verification of transaction  
**Access**: Admin/Operator  
**Risks**: Unauthorized verification, missed fraud

**Security Controls**:
- ✅ `require_elevated_access()` check
- ✅ Transaction must exist
- ✅ Cannot verify already-verified transaction
- ✅ Updates fraud statistics (rejection rate)
- ✅ Status transition to "Completed"
- ✅ Triggers payment calculation

**State Invariants**:
- Once verified, cannot revert to pending
- Verification timestamp recorded
- Verifier address logged
- Payment eligible after verification

#### `update_status(tx_id, status)`
**Purpose**: Change transaction status (reject, dispute)  
**Access**: Admin/Operator  
**Risks**: Improper status changes

**Security Controls**:
- ✅ Admin-only access
- ✅ Valid status transition rules
- ✅ Fraud statistics updated for rejections
- ✅ Status change event emitted

### 4.2 State Machine

```
Pending ──verify()──> Completed ──payment──> Paid
   │
   └──update_status()──> Disputed
   │
   └──update_status()──> Cancelled
```

**Invariants**:
- No reverse transitions (Completed → Pending)
- Only one terminal state (Paid, Cancelled)
- Disputed transactions reviewable by admin

### 4.3 Edge Cases
- Verify non-existent transaction (fails with NotFound)
- Update status of paid transaction (rejected)
- Record collection while collector suspended (rejected)
- Identical transaction from different collector (allowed)

### 4.4 Fraud Detection Integration
- Risk score calculated on submission
- Transaction velocity tracked
- Weight anomaly detection
- Rejection rate impacts future risk
- Critical risk (800+) blocks submission

---

## 5. PaymentDistribution Contract Security

**Contract**: `contracts/payment_distribution/`  
**Security Level**: Critical (financial operations)

### 5.1 Critical Functions

#### `calculate_payment(tx_id: u64) -> i128`
**Purpose**: Compute payment amount  
**Access**: Public (query) / Admin (execution)  
**Risks**: Calculation errors, overflow

**Security Controls**:
- ✅ Checked arithmetic throughout
- ✅ Price validation from MaterialPricing
- ✅ Weight validation from transaction
- ✅ Rounding consistent (rounds down)
- ✅ Maximum payment bounds (if configured)

**Formula**: `payment = weight_kg × price_per_kg`

**State Invariants**:
- Payment >= 0
- Payment calculable from immutable transaction data
- Recalculation yields same result (idempotent)

#### `release_payment(tx_id: u64)`
**Purpose**: Execute payment to collector  
**Access**: Admin-only  
**Risks**: Double payment, unauthorized release

**Security Controls**:
- ✅ `require_admin()` check
- ✅ Transaction must be verified
- ✅ Payment status prevents double-pay
- ✅ Balance check before transfer
- ✅ Payment record immutable after release
- ✅ Event emitted with all details

**State Invariants**:
- Payment released exactly once per transaction
- Escrow balance decreases by payment amount
- Collector balance increases by payment amount
- Payment status set to "Paid"
- Payment timestamp recorded

#### `hold_in_escrow(amount: i128)`
**Purpose**: Move funds into escrow for pending payments  
**Access**: Admin-only  
**Risks**: Escrow manipulation

**Security Controls**:
- ✅ Admin-only access
- ✅ Amount validation (positive)
- ✅ Balance checks
- ✅ Escrow balance tracked

### 5.2 Edge Cases
- Calculate payment for unverified transaction (returns amount but payment blocked)
- Release payment with insufficient escrow (fails with InsufficientBalance)
- Release payment for cancelled transaction (rejected)
- Multiple calculate calls (idempotent, always same result)

### 5.3 Double-Payment Prevention

**Mechanism**:
1. Check payment status on `release_payment()`
2. If already paid, return `AlreadyProcessed` error
3. Mark as paid before external calls
4. Event emission confirms payment

**Attack Scenarios Prevented**:
- Sequential double-pay attempt (status check blocks)
- Concurrent double-pay (atomic state update)
- Reentrancy (no external calls to untrusted contracts)

### 5.4 Integration Security
- MaterialPricing provides prices (validated bounds)
- WasteTransaction provides weight/material (immutable)
- WasteToken minting triggered after payment (if applicable)
- Reputation update after successful payment

---

## 6. MaterialPricing Contract Security

**Contract**: `contracts/material_pricing/`  
**Security Level**: High (economic manipulation risk)

### 6.1 Critical Functions

#### `set_price(material: MaterialType, price: i128)`
**Purpose**: Update material price  
**Access**: Admin-only  
**Risks**: Price manipulation, economic exploit

**Security Controls**:
- ✅ `require_admin()` check
- ✅ Price validation (positive, reasonable bounds)
- ✅ Rate limiting on updates (prevents spam)
- ✅ Price history maintained
- ✅ Price change event emitted

**State Invariants**:
- Price >= 0 (free materials allowed)
- Price < MAX_PRICE (e.g., 1,000,000)
- Last update timestamp accurate
- Price history append-only

#### `get_price(material: MaterialType) -> i128`
**Purpose**: Query current price  
**Access**: Public  
**Risks**: Stale data

**Security Controls**:
- ✅ Returns current price
- ✅ Error if price not set
- ✅ No manipulation possible (read-only)

### 6.2 Price Manipulation Scenarios

**Scenario 1: Admin sets artificially high price**
- Impact: Inflated payments to collectors
- Detection: Price history shows anomaly
- Mitigation: Price bounds, admin monitoring, multi-sig recommended

**Scenario 2: Rapid price updates**
- Impact: Confusion, potential exploit of price timing
- Mitigation: Rate limiting on updates, update event logging

**Scenario 3: Price not set for material**
- Impact: Transactions cannot be priced
- Mitigation: Error propagation, admin must set before accepting material

### 6.3 Edge Cases
- Set price to zero (allowed, indicates free material)
- Set same price twice (idempotent, updates timestamp)
- Get price for material never set (returns error)
- Price update while transactions pending (new price applies to new transactions only)

### 6.4 Integration Security
- PaymentDistribution queries prices (validated)
- No price manipulation from user contracts
- Price immutable per transaction (recorded at submission)

---

## 7. Reputation Contract Security

**Contract**: `contracts/reputation/`  
**Security Level**: Medium (fairness, not financial)

### 7.1 Critical Functions

#### `update_score(collector: Address, delta: i32)`
**Purpose**: Adjust reputation score  
**Access**: Admin/Operator (automatic via transactions)  
**Risks**: Score manipulation

**Security Controls**:
- ✅ Score bounds enforced (0-1000)
- ✅ Delta validation
- ✅ Admin-only manual adjustments
- ✅ Score history maintained

**State Invariants**:
- Score >= 0 and <= 1000
- Score changes logged
- Cannot manipulate own score (user)

#### `adjust_score(collector: Address, new_score: u32, reason: String)`
**Purpose**: Manual score adjustment  
**Access**: Admin-only  
**Risks**: Unfair manipulation

**Security Controls**:
- ✅ `require_admin()` check
- ✅ Reason required (audit trail)
- ✅ Score bounds enforced
- ✅ Adjustment event emitted

### 7.2 Score Calculation
- Based on verified transactions
- Successful verifications increase score
- Rejections decrease score
- Fraud flags impact score
- Cannot be directly manipulated by users

### 7.3 Edge Cases
- Adjust score to current value (allowed, logs reason)
- Score at 0 with negative delta (remains 0)
- Score at 1000 with positive delta (remains 1000)
- New collector with no transactions (score = 0 or default)

### 7.4 Fairness Considerations
- Algorithm transparent
- Historical scores queryable
- Manual adjustments logged with reason
- Appeals process (off-chain governance)

---

## 8. CollectionPoint Contract Security

**Contract**: `contracts/collection_point/`  
**Security Level**: High (verification integrity)

### 8.1 Critical Functions

#### `register_point(name, location, materials)`
**Purpose**: Register new collection point  
**Access**: Admin-only  
**Risks**: Fake collection points

**Security Controls**:
- ✅ Admin-only registration
- ✅ Location validation
- ✅ Material list validation
- ✅ Initial status "Unverified"

#### `verify_point(point_id: u64)`
**Purpose**: Verify collection point as legitimate  
**Access**: Admin-only  
**Risks**: Unverified points used in fraud

**Security Controls**:
- ✅ Admin-only verification
- ✅ Physical verification process (off-chain)
- ✅ Status change to "Verified"
- ✅ Verification event emitted

#### `update_accepted_materials(point_id, materials)`
**Purpose**: Change materials accepted at point  
**Access**: Admin-only  
**Risks**: Material acceptance mismatch

**Security Controls**:
- ✅ Admin-only updates
- ✅ Material list validation
- ✅ Update event emitted

### 8.2 Edge Cases
- Register point with no materials (rejected)
- Verify already-verified point (idempotent)
- Transaction referencing unverified point (rejected)
- Update materials to empty list (rejected)

### 8.3 Integration Security
- WasteTransaction validates point is verified
- WasteTransaction checks material accepted at point
- Prevents fake transactions from unverified points

---

## 9. Cross-Contract Integration Security

### 9.1 Call Flow
```
1. Collector registers (CollectorRegistry)
2. Collection point registered (CollectionPoint)
3. Point verified by admin (CollectionPoint)
4. Collector records transaction (WasteTransaction)
   ├─> Validates collector active (CollectorRegistry)
   ├─> Validates point verified (CollectionPoint)
   ├─> Checks fraud risk (AntiFraud)
   └─> Checks rate limits (RateLimit)
5. Admin verifies transaction (WasteTransaction)
6. Payment calculated (PaymentDistribution)
   └─> Queries price (MaterialPricing)
7. Payment released (PaymentDistribution)
   ├─> Updates token balance (WasteToken)
   └─> Updates reputation (Reputation)
```

### 9.2 Integration Invariants
- All cross-contract addresses validated
- No circular dependencies
- State consistent across contracts
- Events emitted at each step

### 9.3 Attack Vectors
✅ **Reentrancy**: Prevented (no external untrusted calls)  
✅ **Front-running**: Limited impact (prices recorded at submission)  
✅ **State inconsistency**: Prevented (atomic operations, events)  
⚠️ **Admin compromise**: Impacts all contracts (multi-sig recommended)

---

## 10. General Security Principles Applied

### 10.1 Input Validation
✅ All public functions validate inputs  
✅ Address validation (non-zero)  
✅ Amount validation (positive, bounds)  
✅ String validation (max length)  
✅ Enum validation (valid variants)

### 10.2 Access Control
✅ Admin-only functions protected  
✅ Operator permissions scoped  
✅ User authentication via `require_auth()`  
✅ No privilege escalation paths

### 10.3 Arithmetic Safety
✅ Checked arithmetic throughout  
✅ No overflow/underflow possible  
✅ Balance checks before transfers  
✅ Appropriate integer types used

### 10.4 State Management
✅ State transitions validated  
✅ Immutable data where appropriate  
✅ Append-only logs  
✅ State consistency across operations

### 10.5 Event Logging
✅ All state changes emit events  
✅ Admin actions logged  
✅ Financial operations logged  
✅ Events immutable (blockchain)

---

## 11. Residual Risks & Mitigations

### 11.1 Admin Key Compromise (Critical)
**Risk**: Admin can perform any privileged operation  
**Impact**: Full system compromise possible  
**Mitigations**:
- Use hardware wallet for admin key
- Implement multi-sig (2-of-3 or 3-of-5)
- Time-lock critical operations
- Regular key rotation
- Incident response plan ready

### 11.2 Sophisticated Fraud (Medium)
**Risk**: Advanced attackers may evade detection temporarily  
**Impact**: Financial losses, system trust  
**Mitigations**:
- Continuous fraud algorithm improvement
- Manual review of high-risk cases
- Machine learning integration (future)
- Community reporting mechanisms

### 11.3 Sybil Attack (Medium)
**Risk**: Multiple accounts to bypass limits  
**Impact**: Rate limit evasion, reward farming  
**Mitigations**:
- Off-chain KYC integration
- Pattern detection across addresses
- Manual review of suspicious patterns
- Network analysis

### 11.4 Price Manipulation (Low)
**Risk**: Admin sets unfair prices  
**Impact**: Economic imbalance  
**Mitigations**:
- Price bounds enforcement
- Multi-sig for price updates
- Community oversight
- Price change alerts

---

## 12. Security Testing Recommendations

### 12.1 Unit Testing
- [ ] Test all access control gates
- [ ] Test arithmetic edge cases
- [ ] Test state transitions
- [ ] Test error conditions
- [ ] Test fraud detection triggers

### 12.2 Integration Testing
- [ ] Test full transaction workflow
- [ ] Test cross-contract interactions
- [ ] Test emergency scenarios
- [ ] Test concurrent operations
- [ ] Test upgrade procedures

### 12.3 Security Testing
- [ ] Attempt authorization bypass
- [ ] Test fraud detection evasion
- [ ] Test rate limit circumvention
- [ ] Test duplicate detection bypass
- [ ] Test arithmetic vulnerabilities

### 12.4 Stress Testing
- [ ] High-volume transaction submission
- [ ] Concurrent user operations
- [ ] Storage capacity limits
- [ ] Gas consumption at scale
- [ ] Rate limit boundaries

---

## 13. Monitoring & Alerting Recommendations

### 13.1 Real-Time Alerts
- Emergency trigger (immediate)
- Circuit breaker trip (5 minutes)
- High fraud risk detected (15 minutes)
- Large payment (immediate)
- Admin action (immediate)

### 13.2 Daily Monitoring
- Transaction volume
- Fraud detection rate
- Payment processing time
- Error rates
- Storage usage

### 13.3 Weekly Review
- Fraud patterns
- Rate limit hits
- Admin actions
- System health
- Performance metrics

---

**End of Security Considerations**

*This analysis should be used alongside the security audit guide, threat model, and security checklist for comprehensive security review.*
