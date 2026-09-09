# Commit 11: Reputation Contract

## Overview
Implemented the Reputation contract, which manages and calculates reputation scores for collectors based on their transaction history.

## Features Implemented

### Core Functionality
1. **Score Management**
   - Track reputation scores (0-1000 scale)
   - Default neutral score of 500 for new collectors
   - Automatic score updates based on transaction outcomes
   - Manual score setting (admin only)
   - Score reset functionality (admin only)

2. **Transaction Tracking**
   - Count total transactions
   - Track successful transactions
   - Track disputed transactions
   - Last updated timestamp

3. **Score Calculation Algorithm**
   - Base score: 500 (neutral)
   - +5 points per successful transaction
   - -10 points per disputed transaction
   - Success rate bonus: up to +100 points (for collectors with 10+ transactions)
   - Scores clamped between 0 (minimum) and 1000 (maximum)

4. **Queries**
   - Get collector reputation score
   - Calculate score without updating
   - View transaction history stats

5. **Access Control**
   - Admin-only score updates
   - Admin-only manual score setting
   - Admin-only score reset
   - Pause/unpause functionality
   - Initialization protection

### Data Structures
- **ReputationScore**: Complete reputation record with:
  - Collector address
  - Score value (0-1000)
  - Total transactions count
  - Successful transactions count
  - Disputed transactions count
  - Last updated timestamp

### Scoring Algorithm Details

```rust
Score Calculation:
1. Start with base: 500 (neutral)
2. Add: successful_transactions × 5
3. Subtract: disputed_transactions × 10
4. Bonus (if ≥10 transactions): success_rate × 100
5. Clamp result to [0, 1000]

Examples:
- New collector: 500 (default)
- 10 successful, 0 disputed: 500 + 50 + 100 = 650
- 8 successful, 2 disputed: 500 + 40 - 20 + 80 = 600
- 100 successful, 0 disputed: 1000 (capped)
- 0 successful, 100 disputed: 0 (capped)
```

### Storage Layer
- Persistent storage for reputation scores
- Indexed by collector address
- Storage TTL management with automatic bumping

### Events
- Score updated event (with old and new score)
- Admin events (pause/unpause)

## Contract Functions

### Public Functions
- `initialize(admin)` - Initialize contract
- `update_score(collector, transaction_successful)` - Update score after transaction (admin)
- `get_score(collector)` - Get collector's reputation score
- `calculate_score(collector)` - Calculate score without updating
- `set_score(collector, new_score)` - Manually set score (admin)
- `reset_score(collector)` - Reset collector to default (admin)
- `pause()` / `unpause()` - Contract pause control (admin)
- `is_paused()` - Check pause status
- `admin()` - Get admin address

## Tests
Implemented 18 comprehensive tests covering:
- ✅ Initialization and re-initialization prevention
- ✅ Default score for new collectors
- ✅ Score updates for successful transactions
- ✅ Score updates for disputed transactions
- ✅ Multiple successful transactions
- ✅ Mixed transaction outcomes
- ✅ Score calculation accuracy
- ✅ Manual score setting
- ✅ Invalid score validation
- ✅ Score reset functionality
- ✅ Score clamping at maximum (1000)
- ✅ Score clamping at minimum (0)
- ✅ Pause/unpause functionality
- ✅ Paused state restrictions
- ✅ Success rate bonus threshold (10 transactions)

## Integration Points
- **WasteTransaction Contract**: Transaction outcomes trigger score updates
- **CollectorRegistry Contract**: Reputation scores influence collector status
- Ready for integration with:
  - Automatic score updates on transaction completion
  - Collector verification workflows
  - Quality control systems
  - Incentive mechanisms

## Reputation Tiers (Suggested)
Based on score ranges:
- **0-300**: Poor (⭐)
- **301-500**: Fair (⭐⭐)
- **501-700**: Good (⭐⭐⭐)
- **701-900**: Excellent (⭐⭐⭐⭐)
- **901-1000**: Outstanding (⭐⭐⭐⭐⭐)

## Use Cases
1. **Collector Verification**: Higher reputation = faster approval
2. **Payment Prioritization**: High reputation collectors paid first
3. **Dispute Resolution**: Reputation considered in conflict resolution
4. **Incentives**: Bonus rewards for high reputation
5. **Quality Control**: Low reputation triggers review

## Files Modified/Created
- `contracts/reputation/src/lib.rs` - Main contract implementation (263 lines)
- `contracts/reputation/src/test.rs` - Comprehensive tests (394 lines)

## Build Verification
✅ `cargo check --workspace` - PASSED
✅ `cargo clippy --workspace -- -D warnings` - PASSED  
✅ `cargo build --target wasm32-unknown-unknown --release` - PASSED

## Next Steps
- Commit 12: MaterialPricing contract (final Phase 2 contract!)
- Phase 3: Advanced features and cross-contract integration

## Technical Notes
- Score calculation uses floating-point for success rate but returns integer
- Success rate bonus only applies after 10+ transactions to ensure statistical significance
- Manual score setting bypasses calculation for special cases (e.g., manual review)
- Reset functionality useful for testing and dispute resolution
- Scores persist across contract upgrades via persistent storage
- Event emission includes both old and new scores for change tracking
