# Contract Upgradeability Patterns

## Overview
Implemented comprehensive contract upgrade infrastructure with version management, data migration utilities, backward compatibility features, and complete upgrade/rollback procedures.

## New Features

### 1. Version Management System

#### Semantic Versioning Support
```rust
pub struct ContractVersion {
    pub major: u32,  // Breaking changes
    pub minor: u32,  // New features (backward compatible)
    pub patch: u32,  // Bug fixes
}
```

#### Version Operations
- **`set_version()`** - Set contract version
- **`get_version()`** - Query current version
- **`is_compatible_with()`** - Check version compatibility
- **`require_min_version()`** - Enforce minimum version requirement

#### Version Compatibility Rules
- Compatible if major versions match
- Current version must be >= required version
- Example: v1.2.3 compatible with v1.0.0, not with v2.0.0

### 2. Contract Upgrade Management

#### Upgrade Methods
```rust
// Perform WASM upgrade
Upgrade::upgrade_contract(env, admin, new_wasm_hash);

// Mark upgrade in progress
Upgrade::mark_upgrade_in_progress(env);

// Complete upgrade
Upgrade::mark_upgrade_complete(env, new_version);

// Check upgrade status
Upgrade::is_upgrade_in_progress(env);
```

#### Upgrade Safety
- Admin-only operation
- Automatic admin action logging
- Upgrade-in-progress flag prevents concurrent upgrades
- Version tracking for audit trail

### 3. Data Migration Utilities

#### Migration Tracking
```rust
// Check if migration needed
Migration::is_migration_needed(env, target_version);

// Record migration steps
Migration::record_migration_step(env, "migrate_collectors", true);

// Mark migration complete
Migration::mark_migration_complete(env, version);

// Query migration history
Migration::get_last_migration(env);
Migration::get_migration_log(env);
```

#### Migration Log
- Records up to 50 migration steps
- Tracks success/failure per step
- Includes timestamps for auditing
- Helps diagnose migration issues

### 4. Storage Schema Versioning

#### Schema Management
```rust
// Set schema version
StorageSchema::set_schema_version(env, 2);

// Check current schema
StorageSchema::get_schema_version(env);

// Check if migration needed
StorageSchema::needs_migration(env, target_schema);

// Mark schema migrated
StorageSchema::mark_schema_migrated(env, new_schema);
```

#### Use Cases
- Track storage format changes
- Trigger data migrations
- Ensure compatibility
- Version control for data structures

### 5. Backward Compatibility Features

#### Feature Flags
```rust
// Enable/disable features
Compatibility::enable_feature(env, "batch_operations");
Compatibility::disable_feature(env, "old_feature");

// Check feature support
Compatibility::is_feature_supported(env, "batch_operations");

// Get all enabled features
Compatibility::get_enabled_features(env);
```

#### Benefits
- Gradual feature rollout
- A/B testing support
- Emergency feature disable
- Backward compatibility control

### 6. Deprecation Management

#### Deprecation System
```rust
// Mark function deprecated
Deprecation::mark_deprecated(env, "old_method", removal_version);

// Check deprecation status
Deprecation::is_deprecated(env, "old_method");

// Get removal version
Deprecation::get_deprecation_version(env, "old_method");

// Remove deprecation
Deprecation::remove_deprecation(env, "old_method");
```

#### Deprecation Strategy
- Warn users before removal
- Specify removal version
- Track deprecated functions
- Graceful migration period

## Implementation Details

### New Module: `contracts/common/src/upgrade.rs`

#### Storage Patterns

**Contract Version**
```rust
("ContractVersion",) -> (u32, u32, u32) // (major, minor, patch)
```

**Upgrade Status**
```rust
("UpgradeInProgress",) -> bool
```

**Last Migration**
```rust
("LastMigration",) -> ((u32, u32, u32), u64) // (version, timestamp)
```

**Migration Log**
```rust
("MigrationLog",) -> Vec<(String, bool, u64)> // (step, success, timestamp)
```

**Storage Schema**
```rust
("StorageSchemaVersion",) -> u32
("SchemaUpgradedAt",) -> u64
```

**Feature Flags**
```rust
("SupportedFeatures", feature: String) -> bool
("EnabledFeaturesList",) -> Vec<String>
```

**Deprecations**
```rust
("Deprecated", function: String) -> (u32, u32, u32) // removal version
```

### Error Types Added

```rust
IncompatibleVersion = 85,  // Version incompatibility
VersionNotSet = 86,        // No version set
UpgradeInProgress = 87,    // Upgrade currently running
```

### CollectorRegistry Integration

#### New Upgrade Methods

```rust
// Get version
get_version() -> (u32, u32, u32)

// Perform upgrade (admin)
upgrade(new_wasm_hash)

// Complete upgrade (admin)
complete_upgrade(major, minor, patch)

// Check upgrade status
is_upgrading() -> bool

// Get storage schema
get_schema_version() -> u32

// Feature management (admin)
enable_feature(feature)
is_feature_supported(feature) -> bool
```

#### Enhanced Initialization

```rust
pub fn initialize(env: Env, admin: Address) {
    // ... existing initialization ...
    
    // Set initial version
    let initial_version = ContractVersion::new(1, 0, 0);
    Upgrade::set_version(&env, initial_version);
    
    // Set initial storage schema
    StorageSchema::set_schema_version(&env, 1);
    
    // ... complete initialization ...
}
```

## Documentation

### Comprehensive Upgrade Guide

Created `docs/UPGRADE_GUIDE.md` with:
- **Upgrade Process**: Step-by-step procedures
- **Version Management**: Semantic versioning guide
- **Data Migration**: Migration patterns and examples
- **Backward Compatibility**: Feature flags and deprecation
- **Testing Upgrades**: Testnet testing procedures
- **Rollback Procedures**: Emergency rollback steps
- **Best Practices**: Upgrade safety guidelines
- **Example Scenarios**: Real-world upgrade examples
- **Troubleshooting**: Common issues and solutions

## Use Cases

### Use Case 1: Simple Version Update

```rust
// Deploy v1.0.0
registry.initialize(admin);
// Version: 1.0.0, Schema: 1

// Upgrade to v1.1.0 (new features, no breaking changes)
registry.upgrade(new_wasm_hash);
registry.complete_upgrade(1, 1, 0);

// Version: 1.1.0, Schema: 1 (no migration needed)
```

### Use Case 2: Breaking Change with Migration

```rust
// Current: v1.5.0, Schema: 1

// Upgrade to v2.0.0 with schema migration
registry.mark_upgrade_in_progress();
registry.upgrade(new_wasm_hash);

// Migrate storage schema
if StorageSchema::needs_migration(&env, 2) {
    migrate_collectors_to_v2(&env);
    StorageSchema::mark_schema_migrated(&env, 2);
}

registry.complete_upgrade(2, 0, 0);
// Version: 2.0.0, Schema: 2
```

### Use Case 3: Feature Flag Usage

```rust
// Enable new feature
registry.enable_feature("advanced_queries");

// Use conditionally
if registry.is_feature_supported("advanced_queries") {
    // Use new advanced query methods
    registry.get_collectors_by_reg_time(...);
} else {
    // Fall back to basic queries
    registry.get_all_collectors(...);
}
```

### Use Case 4: Deprecation Warning

```rust
// Mark old method as deprecated in v1.5.0
Deprecation::mark_deprecated(
    &env,
    "old_register",
    ContractVersion::new(2, 0, 0) // Remove in v2.0.0
);

// Old method still works but warns
pub fn old_register(env: Env, ...) {
    if Deprecation::is_deprecated(&env, "old_register") {
        // Log warning to users
    }
    // Delegate to new method
    new_register(env, ...)
}
```

### Use Case 5: Rollback After Failed Upgrade

```rust
// Attempt upgrade
registry.mark_upgrade_in_progress();
registry.upgrade(new_wasm_hash);

// Migration fails
if migration_failed {
    // Rollback to previous version
    registry.upgrade(previous_wasm_hash);
    registry.complete_upgrade(1, 0, 0); // Previous version
    
    // Check rollback success
    assert!(!registry.is_upgrading());
    assert_eq!(registry.get_version(), (1, 0, 0));
}
```

## Benefits

### 1. Safe Upgrades
- Version tracking prevents accidental downgrades
- Upgrade-in-progress flag prevents concurrent upgrades
- Admin-only access with audit logging
- Rollback capability for failed upgrades

### 2. Data Integrity
- Schema versioning tracks storage changes
- Migration utilities preserve data
- Migration logging for debugging
- Incremental migration support

### 3. Backward Compatibility
- Feature flags for gradual rollout
- Deprecation warnings before removal
- Old APIs can delegate to new ones
- Version compatibility checks

### 4. Operational Excellence
- Clear upgrade procedures
- Comprehensive documentation
- Testing guidelines
- Troubleshooting support

## Best Practices

### Version Increment Rules

**Patch (x.x.PATCH)** - Bug fixes only
- No API changes
- No new features
- Performance improvements

**Minor (x.MINOR.x)** - Backward compatible
- New features
- New methods
- Optional parameters
- Deprecation warnings

**Major (MAJOR.x.x)** - Breaking changes
- Removed deprecated features
- Changed method signatures
- Storage schema changes
- Incompatible changes

### Upgrade Checklist

**Pre-Upgrade:**
- [ ] Test on testnet
- [ ] Prepare migration scripts
- [ ] Document breaking changes
- [ ] Plan rollback procedure
- [ ] Notify users

**During Upgrade:**
- [ ] Mark upgrade in progress
- [ ] Upload new WASM
- [ ] Run migrations
- [ ] Verify data integrity
- [ ] Enable new features

**Post-Upgrade:**
- [ ] Verify version
- [ ] Test critical functions
- [ ] Monitor for errors
- [ ] Update documentation
- [ ] Mark upgrade complete

### Testing Strategy

1. **Unit Tests**: Test upgrade utilities
2. **Integration Tests**: Full upgrade flow
3. **Testnet Tests**: Real-world conditions
4. **Rollback Tests**: Emergency procedures
5. **Performance Tests**: Migration impact

## Testing

### New Test Coverage

```rust
#[test]
fn test_version_compatibility() {
    let v1_0_0 = ContractVersion::new(1, 0, 0);
    let v1_1_0 = ContractVersion::new(1, 1, 0);
    let v2_0_0 = ContractVersion::new(2, 0, 0);
    
    assert!(v1_1_0.is_compatible_with(&v1_0_0)); // true
    assert!(!v2_0_0.is_compatible_with(&v1_0_0)); // false
}

#[test]
fn test_upgrade_flag() {
    assert!(!Upgrade::is_upgrade_in_progress(&env));
    
    Upgrade::mark_upgrade_in_progress(&env);
    assert!(Upgrade::is_upgrade_in_progress(&env));
    
    Upgrade::mark_upgrade_complete(&env, ContractVersion::new(2, 0, 0));
    assert!(!Upgrade::is_upgrade_in_progress(&env));
}

#[test]
fn test_migration_tracking() {
    Migration::mark_migration_complete(&env, ContractVersion::new(2, 0, 0));
    let (version, _) = Migration::get_last_migration(&env).unwrap();
    assert_eq!(version.major, 2);
}
```

8 comprehensive tests covering all upgrade utilities.

## Files Modified

### New Files
- **`contracts/common/src/upgrade.rs`** - Complete upgrade system (400+ lines)
- **`docs/UPGRADE_GUIDE.md`** - Comprehensive upgrade documentation (500+ lines)

### Modified Files
- **`contracts/common/src/lib.rs`** - Export upgrade module
- **`contracts/common/src/errors.rs`** - Add 3 upgrade error types
- **`contracts/collector_registry/src/lib.rs`** - Add upgrade methods, version initialization

## Build & Verification
- ✅ cargo check --workspace
- ✅ cargo clippy --workspace --lib (no warnings)
- ✅ cargo build --target wasm32-unknown-unknown --release
- ✅ cargo fmt --all
- ✅ 8 comprehensive tests in upgrade module

## Summary

This commit delivers **enterprise-grade contract upgradeability**:

### Features
- **Version management**: Semantic versioning with compatibility checks
- **Upgrade infrastructure**: Safe WASM upgrade with admin controls
- **Data migration**: Complete migration tracking and utilities
- **Storage schema versioning**: Track and manage storage changes
- **Feature flags**: Gradual feature rollout and A/B testing
- **Deprecation system**: Graceful removal of old features
- **Comprehensive documentation**: Complete upgrade guide with examples

### Benefits
- **Safe upgrades**: Version tracking, upgrade flags, admin-only
- **Data preservation**: Migration utilities maintain data integrity
- **Backward compatibility**: Feature flags, deprecation warnings
- **Operational excellence**: Clear procedures, testing guidelines
- **Production ready**: 400+ lines of upgrade code, 8 tests, full documentation

Provides complete infrastructure for safely evolving WasteFi contracts over time while maintaining data integrity and backward compatibility.
