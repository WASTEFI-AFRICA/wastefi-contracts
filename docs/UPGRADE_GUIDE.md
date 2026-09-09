# Contract Upgrade Guide

## Overview

This guide provides detailed instructions for safely upgrading WasteFi smart contracts while preserving data integrity and maintaining backward compatibility.

## Table of Contents

1. [Upgrade Process](#upgrade-process)
2. [Version Management](#version-management)
3. [Data Migration](#data-migration)
4. [Backward Compatibility](#backward-compatibility)
5. [Testing Upgrades](#testing-upgrades)
6. [Rollback Procedures](#rollback-procedures)
7. [Best Practices](#best-practices)

## Upgrade Process

### Step 1: Preparation

```bash
# 1. Build new contract version
cargo build --release --target wasm32-unknown-unknown

# 2. Optimize WASM
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/collector_registry.wasm

# 3. Deploy to testnet first
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/collector_registry.optimized.wasm \
  --source deployer \
  --network testnet
```

### Step 2: Pre-Upgrade Checks

```rust
// Check current version
let current_version = registry.get_version();
println!("Current version: {}.{}.{}", 
    current_version.0, current_version.1, current_version.2);

// Check if upgrade is safe
assert!(!registry.is_upgrading(), "Upgrade already in progress");

// Verify no critical operations in progress
assert!(!registry.is_paused(), "Contract is paused");

// Backup critical data (if needed)
```

### Step 3: Perform Upgrade

```rust
// Mark upgrade as in progress
registry.mark_upgrade_in_progress();

// Upload new WASM
let new_wasm_hash = upload_wasm(...);

// Perform upgrade (admin only)
registry.upgrade(new_wasm_hash);

// Complete upgrade with new version
registry.complete_upgrade(1, 1, 0); // v1.1.0

// Verify upgrade
let new_version = registry.get_version();
assert_eq!(new_version, (1, 1, 0));
```

### Step 4: Post-Upgrade Validation

```rust
// Run data migration if needed
if registry.get_schema_version() < 2 {
    migrate_storage_to_v2();
}

// Verify critical functionality
test_collector_registration();
test_transaction_recording();
test_payment_processing();

// Enable new features
registry.enable_feature("batch_operations");
registry.enable_feature("advanced_queries");
```

## Version Management

### Semantic Versioning

WasteFi contracts follow semantic versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Incompatible API changes
- **MINOR**: Backward-compatible functionality additions
- **PATCH**: Backward-compatible bug fixes

### Version Compatibility

```rust
// Check version compatibility
let current = ContractVersion::new(1, 2, 3);
let required = ContractVersion::new(1, 0, 0);

assert!(current.is_compatible_with(&required)); // true (same major)

let incompatible = ContractVersion::new(2, 0, 0);
assert!(!current.is_compatible_with(&incompatible)); // false (different major)
```

### Setting and Querying Versions

```rust
// Set version during initialization
Upgrade::set_version(&env, ContractVersion::new(1, 0, 0));

// Query current version
if let Some(version) = Upgrade::get_version(&env) {
    println!("Version: {}.{}.{}", version.major, version.minor, version.patch);
}

// Require minimum version
Upgrade::require_min_version(&env, ContractVersion::new(1, 1, 0))?;
```

## Data Migration

### Storage Schema Versioning

```rust
// Check storage schema version
let current_schema = StorageSchema::get_schema_version(&env);

// Perform migration if needed
if StorageSchema::needs_migration(&env, 2) {
    migrate_storage_v1_to_v2(&env);
    StorageSchema::mark_schema_migrated(&env, 2);
}
```

### Migration Tracking

```rust
// Record migration steps
Migration::record_migration_step(&env, "migrate_collectors", true);
Migration::record_migration_step(&env, "migrate_transactions", true);
Migration::record_migration_step(&env, "update_indexes", true);

// Mark migration complete
Migration::mark_migration_complete(&env, ContractVersion::new(2, 0, 0));

// Query migration history
let (version, timestamp) = Migration::get_last_migration(&env).unwrap();
let log = Migration::get_migration_log(&env);
```

### Example: Migrating Collector Data

```rust
pub fn migrate_collectors_v1_to_v2(env: &Env) {
    Migration::record_migration_step(env, "start_collector_migration", true);
    
    let collector_count = read_collector_count(env);
    let mut migrated = 0;
    
    for i in 0..collector_count {
        if let Some(address) = get_collector_address(env, i) {
            // Read old format
            let old_data = read_collector_v1(env, &address);
            
            // Convert to new format
            let new_data = Collector {
                address: old_data.address,
                name: old_data.name,
                phone: old_data.phone,
                status: old_data.status,
                reputation_score: old_data.reputation_score,
                total_collections: old_data.total_collections,
                total_weight: old_data.total_weight,
                registration_time: old_data.registration_time,
                last_active: old_data.last_active,
                // New field in v2
                verification_level: VerificationLevel::Basic,
            };
            
            // Write new format
            write_collector(env, &address, &new_data);
            migrated += 1;
        }
    }
    
    Migration::record_migration_step(
        env,
        format!("migrated_{}_collectors", migrated),
        true
    );
}
```

## Backward Compatibility

### Feature Flags

```rust
// Enable features progressively
Compatibility::enable_feature(&env, "batch_operations");
Compatibility::enable_feature(&env, "cross_contract_calls");

// Check if feature is supported before using
if Compatibility::is_feature_supported(&env, "batch_operations") {
    // Use batch operations
    batch_register_collectors(...);
} else {
    // Fall back to single operations
    for collector in collectors {
        register_collector(collector);
    }
}

// List all enabled features
let features = Compatibility::get_enabled_features(&env);
```

### Deprecation Management

```rust
// Mark old function as deprecated
Deprecation::mark_deprecated(
    &env,
    "old_register_method",
    ContractVersion::new(2, 0, 0) // Will be removed in v2.0.0
);

// Check if function is deprecated
if Deprecation::is_deprecated(&env, "old_register_method") {
    if let Some(removal_version) = Deprecation::get_deprecation_version(&env, "old_register_method") {
        println!("Warning: This function will be removed in v{}.{}.{}", 
            removal_version.major, removal_version.minor, removal_version.patch);
    }
}

// Remove deprecation if function is restored
Deprecation::remove_deprecation(&env, "old_register_method");
```

### Maintaining Old APIs

```rust
// Old API (deprecated but still working)
pub fn register_collector_v1(
    env: Env,
    collector: Address,
    name: String,
) -> Result<(), WasteFiError> {
    // Check if deprecated
    if Deprecation::is_deprecated(&env, "register_collector_v1") {
        // Log deprecation warning
    }
    
    // Call new API with default values
    register_collector_v2(env, collector, name, String::default(), CollectorType::Individual)
}

// New API
pub fn register_collector_v2(
    env: Env,
    collector: Address,
    name: String,
    phone: String,
    collector_type: CollectorType,
) -> Result<(), WasteFiError> {
    // New implementation
}
```

## Testing Upgrades

### Test Upgrade on Testnet

```bash
# 1. Deploy current version
soroban contract deploy \
  --wasm current.wasm \
  --source deployer \
  --network testnet

# 2. Initialize and populate with test data
soroban contract invoke \
  --id CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- initialize --admin ADMIN_ADDRESS

# 3. Add test data
soroban contract invoke \
  --id CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- register --collector COLLECTOR_ADDRESS --name "Test"

# 4. Upgrade to new version
soroban contract invoke \
  --id CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- upgrade --new_wasm_hash NEW_WASM_HASH

# 5. Verify data integrity
soroban contract invoke \
  --id CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- get_collector --collector COLLECTOR_ADDRESS
```

### Automated Upgrade Tests

```rust
#[test]
fn test_upgrade_from_v1_to_v2() {
    let env = Env::default();
    let admin = Address::generate(&env);
    
    // Deploy v1
    let contract_v1 = deploy_collector_registry_v1(&env, &admin);
    
    // Add data
    contract_v1.register(collector.clone(), name.clone(), phone.clone());
    
    // Verify v1 data
    let collector_v1 = contract_v1.get_collector(collector.clone());
    assert_eq!(collector_v1.name, name);
    
    // Upgrade to v2
    contract_v1.upgrade(new_wasm_hash);
    contract_v1.complete_upgrade(2, 0, 0);
    
    // Verify version
    let version = contract_v1.get_version();
    assert_eq!(version, (2, 0, 0));
    
    // Verify data preserved
    let collector_v2 = contract_v1.get_collector(collector.clone());
    assert_eq!(collector_v2.name, name);
    
    // Test new v2 features
    contract_v1.enable_feature("new_v2_feature");
    assert!(contract_v1.is_feature_supported("new_v2_feature"));
}
```

## Rollback Procedures

### Preparing for Rollback

```rust
// Before upgrade: Save current state
let pre_upgrade_snapshot = {
    version: registry.get_version(),
    schema: registry.get_schema_version(),
    collector_count: registry.get_collector_count(),
    features: registry.get_enabled_features(),
};

// After failed upgrade: Restore previous version
if upgrade_failed {
    registry.upgrade(previous_wasm_hash);
    registry.complete_upgrade(
        pre_upgrade_snapshot.version.0,
        pre_upgrade_snapshot.version.1,
        pre_upgrade_snapshot.version.2,
    );
}
```

### Emergency Rollback

```rust
// If upgrade causes critical issues:

// 1. Trigger emergency mode
registry.trigger_emergency(EmergencyLevel::Critical, "Upgrade failed");

// 2. Revert to previous WASM
registry.upgrade(previous_wasm_hash);

// 3. Restore version
registry.complete_upgrade(1, 0, 0); // Previous version

// 4. Resolve emergency
registry.resolve_emergency();

// 5. Verify rollback
assert_eq!(registry.get_version(), (1, 0, 0));
assert!(registry.is_feature_supported("core_functionality"));
```

## Best Practices

### 1. Version Increment Guidelines

**Patch (x.x.PATCH)**
- Bug fixes
- Performance improvements
- Documentation updates
- No API changes

**Minor (x.MINOR.x)**
- New features (backward compatible)
- New functions/methods
- Optional new parameters
- Deprecation warnings

**Major (MAJOR.x.x)**
- Breaking API changes
- Removed deprecated functions
- Changed function signatures
- Storage schema changes

### 2. Pre-Upgrade Checklist

- [ ] All tests pass
- [ ] Upgrade tested on testnet
- [ ] Migration script prepared (if needed)
- [ ] Rollback plan documented
- [ ] Admin keys accessible
- [ ] Monitoring in place
- [ ] User notification sent
- [ ] Emergency procedures reviewed

### 3. During Upgrade

- [ ] Mark upgrade in progress
- [ ] Pause non-critical operations
- [ ] Upload new WASM
- [ ] Verify WASM hash
- [ ] Execute upgrade
- [ ] Run migration (if needed)
- [ ] Verify critical data
- [ ] Enable new features
- [ ] Mark upgrade complete

### 4. Post-Upgrade

- [ ] Verify version updated
- [ ] Test all critical functions
- [ ] Check data integrity
- [ ] Monitor for errors
- [ ] Update documentation
- [ ] Notify users of completion
- [ ] Remove upgrade flags
- [ ] Document lessons learned

### 5. Upgrade Safety

**DO:**
- ✅ Test thoroughly on testnet
- ✅ Have rollback plan ready
- ✅ Maintain backward compatibility
- ✅ Use feature flags for new features
- ✅ Document breaking changes
- ✅ Provide migration scripts
- ✅ Monitor post-upgrade

**DON'T:**
- ❌ Upgrade directly on mainnet
- ❌ Skip testing migrations
- ❌ Change storage formats without migration
- ❌ Remove features without deprecation period
- ❌ Upgrade during high traffic
- ❌ Forget to backup data
- ❌ Ignore warning signs

### 6. Communication

**Before Upgrade:**
- Announce upgrade date/time
- Explain new features
- List breaking changes
- Provide migration guide
- Set expectations for downtime

**During Upgrade:**
- Real-time status updates
- Estimated completion time
- Issue notifications
- Contact information

**After Upgrade:**
- Completion announcement
- New feature highlights
- Known issues (if any)
- Support resources
- Feedback channels

## Example Upgrade Scenarios

### Scenario 1: Simple Feature Addition

```rust
// v1.0.0 -> v1.1.0
// Adding batch operations (backward compatible)

// 1. Add new functions
pub fn batch_register(env: Env, collectors: Vec<...>) -> u32 {
    // Implementation
}

// 2. Update version
registry.complete_upgrade(1, 1, 0);

// 3. Enable feature
registry.enable_feature("batch_operations");

// 4. Old single operations still work
registry.register(collector, name, phone); // Still works
```

### Scenario 2: Breaking Change

```rust
// v1.x.x -> v2.0.0
// Changing collector registration to require verification

// Phase 1: v1.5.0 - Add deprecation
Deprecation::mark_deprecated(
    "register",
    ContractVersion::new(2, 0, 0)
);

// Add new method
pub fn register_with_verification(
    env: Env,
    collector: Address,
    name: String,
    phone: String,
    verification_doc: String,
) { /* ... */ }

// Phase 2: v2.0.0 - Remove old method, make new method default
// Old 'register' is removed
// 'register_with_verification' becomes 'register'
```

### Scenario 3: Storage Migration

```rust
// v1.0.0 -> v2.0.0
// Adding new field to Collector struct

pub fn migrate_to_v2(env: &Env) {
    let current_schema = StorageSchema::get_schema_version(env);
    
    if current_schema == 1 {
        // Migrate each collector
        let count = read_collector_count(env);
        for i in 0..count {
            if let Some(addr) = get_collector_address(env, i) {
                let mut collector = read_collector(env, &addr);
                // Add default value for new field
                collector.verification_level = VerificationLevel::Basic;
                write_collector(env, &addr, &collector);
            }
        }
        
        StorageSchema::mark_schema_migrated(env, 2);
        Migration::mark_migration_complete(env, ContractVersion::new(2, 0, 0));
    }
}
```

## Troubleshooting

### Issue: Upgrade Fails Midway

**Solution:**
```rust
// Check upgrade status
if registry.is_upgrading() {
    // Reset upgrade flag
    registry.mark_upgrade_complete(previous_version);
    
    // Rollback if needed
    registry.upgrade(previous_wasm_hash);
}
```

### Issue: Data Corruption After Migration

**Solution:**
```rust
// Check migration log
let log = Migration::get_migration_log(&env);
for (step, success, timestamp) in log {
    if !success {
        println!("Failed step: {}", step);
        // Re-run failed migration step
    }
}
```

### Issue: Incompatible Version Error

**Solution:**
```rust
// Check version compatibility
let current = Upgrade::get_version(&env).unwrap();
let required = ContractVersion::new(2, 0, 0);

if !current.is_compatible_with(&required) {
    println!("Need to upgrade from v{}.{}.{} to v{}.{}.{}",
        current.major, current.minor, current.patch,
        required.major, required.minor, required.patch);
}
```

## Conclusion

Successful contract upgrades require:
1. **Thorough planning** and testing
2. **Clear communication** with users
3. **Robust rollback** procedures
4. **Careful version** management
5. **Comprehensive testing** on testnet
6. **Monitoring** during and after upgrade

Always prioritize data integrity and user experience over speed of deployment.
