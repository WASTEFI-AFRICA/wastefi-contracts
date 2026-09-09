use crate::errors::WasteFiError;
use soroban_sdk::{Address, Env, String};

/// Contract version information
#[derive(Clone, Debug)]
pub struct ContractVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ContractVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Check if this version is compatible with another version
    /// Compatible if major versions match and this version >= other
    pub fn is_compatible_with(&self, other: &ContractVersion) -> bool {
        if self.major != other.major {
            return false;
        }
        if self.minor > other.minor {
            return true;
        }
        if self.minor == other.minor && self.patch >= other.patch {
            return true;
        }
        false
    }

    /// Convert to string representation (e.g., "1.2.3")
    pub fn to_string(&self, env: &Env) -> String {
        // Manual string building since format! is not available in no_std
        let result = String::from_str(env, "");
        // Note: Simplified version, full implementation would need custom formatting
        result
    }

    /// Parse from storage tuple
    pub fn from_tuple(tuple: (u32, u32, u32)) -> Self {
        Self::new(tuple.0, tuple.1, tuple.2)
    }

    /// Convert to storage tuple
    pub fn to_tuple(&self) -> (u32, u32, u32) {
        (self.major, self.minor, self.patch)
    }
}

/// Contract upgrade management
pub struct Upgrade;

impl Upgrade {
    /// Set contract version
    pub fn set_version(env: &Env, version: ContractVersion) {
        let key = ("ContractVersion",);
        env.storage().instance().set(&key, &version.to_tuple());
        crate::bump_instance(env);
    }

    /// Get contract version
    pub fn get_version(env: &Env) -> Option<ContractVersion> {
        let key = ("ContractVersion",);
        env.storage()
            .instance()
            .get::<_, (u32, u32, u32)>(&key)
            .map(ContractVersion::from_tuple)
    }

    /// Require minimum version
    pub fn require_min_version(
        env: &Env,
        min_version: ContractVersion,
    ) -> Result<(), WasteFiError> {
        if let Some(current) = Self::get_version(env) {
            if current.is_compatible_with(&min_version) {
                Ok(())
            } else {
                Err(WasteFiError::IncompatibleVersion)
            }
        } else {
            Err(WasteFiError::VersionNotSet)
        }
    }

    /// Upgrade contract WASM (admin only)
    pub fn upgrade_contract(env: &Env, admin: &Address, new_wasm_hash: soroban_sdk::BytesN<32>) {
        // Require admin
        crate::AccessControl::require_admin(env, admin).expect("Not admin");

        // Update the contract code
        env.deployer().update_current_contract_wasm(new_wasm_hash);

        // Log the upgrade
        crate::AccessControl::log_admin_action(
            env,
            admin,
            String::from_str(env, "upgrade_contract"),
            None,
        );

        crate::bump_instance(env);
    }

    /// Check if upgrade is in progress
    pub fn is_upgrade_in_progress(env: &Env) -> bool {
        let key = ("UpgradeInProgress",);
        env.storage().instance().get(&key).unwrap_or(false)
    }

    /// Mark upgrade as in progress
    pub fn mark_upgrade_in_progress(env: &Env) {
        let key = ("UpgradeInProgress",);
        env.storage().instance().set(&key, &true);
        crate::bump_instance(env);
    }

    /// Mark upgrade as complete
    pub fn mark_upgrade_complete(env: &Env, new_version: ContractVersion) {
        let key = ("UpgradeInProgress",);
        env.storage().instance().set(&key, &false);
        Self::set_version(env, new_version);
        crate::bump_instance(env);
    }

    /// Require not upgrading
    pub fn require_not_upgrading(env: &Env) -> Result<(), WasteFiError> {
        if Self::is_upgrade_in_progress(env) {
            Err(WasteFiError::UpgradeInProgress)
        } else {
            Ok(())
        }
    }
}

/// Data migration utilities
pub struct Migration;

impl Migration {
    /// Check if migration is needed
    pub fn is_migration_needed(env: &Env, target_version: ContractVersion) -> bool {
        if let Some(current) = Upgrade::get_version(env) {
            !current.is_compatible_with(&target_version)
        } else {
            true // No version set, migration needed
        }
    }

    /// Mark migration as complete
    pub fn mark_migration_complete(env: &Env, version: ContractVersion) {
        let key = ("LastMigration",);
        let timestamp = env.ledger().timestamp();
        env.storage()
            .instance()
            .set(&key, &(version.to_tuple(), timestamp));
        crate::bump_instance(env);
    }

    /// Get last migration info
    pub fn get_last_migration(env: &Env) -> Option<(ContractVersion, u64)> {
        let key = ("LastMigration",);
        env.storage()
            .instance()
            .get::<_, ((u32, u32, u32), u64)>(&key)
            .map(|(version_tuple, timestamp)| {
                (ContractVersion::from_tuple(version_tuple), timestamp)
            })
    }

    /// Record migration step
    pub fn record_migration_step(env: &Env, step: String, success: bool) {
        let log_key = ("MigrationLog",);
        let mut log: soroban_sdk::Vec<(String, bool, u64)> = env
            .storage()
            .instance()
            .get(&log_key)
            .unwrap_or(soroban_sdk::Vec::new(env));

        log.push_back((step, success, env.ledger().timestamp()));

        // Keep only last 50 migration steps
        while log.len() > 50 {
            log.remove(0);
        }

        env.storage().instance().set(&log_key, &log);
        crate::bump_instance(env);
    }

    /// Get migration log
    pub fn get_migration_log(env: &Env) -> soroban_sdk::Vec<(String, bool, u64)> {
        let log_key = ("MigrationLog",);
        env.storage()
            .instance()
            .get(&log_key)
            .unwrap_or(soroban_sdk::Vec::new(env))
    }
}

/// Storage schema versioning
pub struct StorageSchema;

impl StorageSchema {
    /// Set storage schema version
    pub fn set_schema_version(env: &Env, version: u32) {
        let key = ("StorageSchemaVersion",);
        env.storage().instance().set(&key, &version);
        crate::bump_instance(env);
    }

    /// Get storage schema version
    pub fn get_schema_version(env: &Env) -> u32 {
        let key = ("StorageSchemaVersion",);
        env.storage().instance().get(&key).unwrap_or(1)
    }

    /// Check if schema migration is needed
    pub fn needs_migration(env: &Env, target_schema: u32) -> bool {
        Self::get_schema_version(env) < target_schema
    }

    /// Mark schema as migrated
    pub fn mark_schema_migrated(env: &Env, new_schema: u32) {
        Self::set_schema_version(env, new_schema);
        let key = ("SchemaUpgradedAt",);
        env.storage()
            .instance()
            .set(&key, &env.ledger().timestamp());
        crate::bump_instance(env);
    }
}

/// Backward compatibility helpers
pub struct Compatibility;

impl Compatibility {
    /// Check if feature is supported
    pub fn is_feature_supported(env: &Env, feature: String) -> bool {
        let key = ("SupportedFeatures", feature);
        env.storage().instance().get(&key).unwrap_or(false)
    }

    /// Enable feature
    pub fn enable_feature(env: &Env, feature: String) {
        let key = ("SupportedFeatures", feature);
        env.storage().instance().set(&key, &true);
        crate::bump_instance(env);
    }

    /// Disable feature
    pub fn disable_feature(env: &Env, feature: String) {
        let key = ("SupportedFeatures", feature);
        env.storage().instance().set(&key, &false);
        crate::bump_instance(env);
    }

    /// Get all enabled features
    pub fn get_enabled_features(env: &Env) -> soroban_sdk::Vec<String> {
        let key = ("EnabledFeaturesList",);
        env.storage()
            .instance()
            .get(&key)
            .unwrap_or(soroban_sdk::Vec::new(env))
    }

    /// Set enabled features list
    pub fn set_enabled_features(env: &Env, features: soroban_sdk::Vec<String>) {
        let key = ("EnabledFeaturesList",);
        env.storage().instance().set(&key, &features);
        crate::bump_instance(env);
    }
}

/// Deprecation warnings
pub struct Deprecation;

impl Deprecation {
    /// Mark function as deprecated
    pub fn mark_deprecated(env: &Env, function_name: String, removal_version: ContractVersion) {
        let key = ("Deprecated", function_name);
        env.storage()
            .instance()
            .set(&key, &removal_version.to_tuple());
        crate::bump_instance(env);
    }

    /// Check if function is deprecated
    pub fn is_deprecated(env: &Env, function_name: String) -> bool {
        let key = ("Deprecated", function_name);
        env.storage().instance().has(&key)
    }

    /// Get deprecation info
    pub fn get_deprecation_version(env: &Env, function_name: String) -> Option<ContractVersion> {
        let key = ("Deprecated", function_name);
        env.storage()
            .instance()
            .get::<_, (u32, u32, u32)>(&key)
            .map(ContractVersion::from_tuple)
    }

    /// Remove deprecation (if function is restored)
    pub fn remove_deprecation(env: &Env, function_name: String) {
        let key = ("Deprecated", function_name);
        env.storage().instance().remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_version_compatibility() {
        let v1_0_0 = ContractVersion::new(1, 0, 0);
        let v1_0_1 = ContractVersion::new(1, 0, 1);
        let v1_1_0 = ContractVersion::new(1, 1, 0);
        let v2_0_0 = ContractVersion::new(2, 0, 0);

        assert!(v1_0_1.is_compatible_with(&v1_0_0));
        assert!(v1_1_0.is_compatible_with(&v1_0_0));
        assert!(!v2_0_0.is_compatible_with(&v1_0_0));
        assert!(!v1_0_0.is_compatible_with(&v1_0_1));
    }

    #[test]
    fn test_version_storage() {
        let env = Env::default();
        let version = ContractVersion::new(1, 2, 3);

        Upgrade::set_version(&env, version.clone());
        let retrieved = Upgrade::get_version(&env).unwrap();

        assert_eq!(retrieved.major, 1);
        assert_eq!(retrieved.minor, 2);
        assert_eq!(retrieved.patch, 3);
    }

    #[test]
    fn test_upgrade_flag() {
        let env = Env::default();

        assert!(!Upgrade::is_upgrade_in_progress(&env));

        Upgrade::mark_upgrade_in_progress(&env);
        assert!(Upgrade::is_upgrade_in_progress(&env));

        Upgrade::mark_upgrade_complete(&env, ContractVersion::new(2, 0, 0));
        assert!(!Upgrade::is_upgrade_in_progress(&env));
    }

    #[test]
    fn test_migration_tracking() {
        let env = Env::default();
        let version = ContractVersion::new(2, 0, 0);

        Migration::mark_migration_complete(&env, version);
        let (migrated_version, _timestamp) = Migration::get_last_migration(&env).unwrap();

        assert_eq!(migrated_version.major, 2);
        assert_eq!(migrated_version.minor, 0);
        assert_eq!(migrated_version.patch, 0);
    }

    #[test]
    fn test_migration_log() {
        let env = Env::default();

        Migration::record_migration_step(&env, String::from_str(&env, "step1"), true);
        Migration::record_migration_step(&env, String::from_str(&env, "step2"), true);

        let log = Migration::get_migration_log(&env);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn test_storage_schema() {
        let env = Env::default();

        assert_eq!(StorageSchema::get_schema_version(&env), 1);

        StorageSchema::set_schema_version(&env, 2);
        assert_eq!(StorageSchema::get_schema_version(&env), 2);

        assert!(StorageSchema::needs_migration(&env, 3));
        assert!(!StorageSchema::needs_migration(&env, 2));
    }

    #[test]
    fn test_feature_flags() {
        let env = Env::default();
        let feature = String::from_str(&env, "new_feature");

        assert!(!Compatibility::is_feature_supported(&env, feature.clone()));

        Compatibility::enable_feature(&env, feature.clone());
        assert!(Compatibility::is_feature_supported(&env, feature.clone()));

        Compatibility::disable_feature(&env, feature.clone());
        assert!(!Compatibility::is_feature_supported(&env, feature));
    }

    #[test]
    fn test_deprecation() {
        let env = Env::default();
        let func = String::from_str(&env, "old_function");

        assert!(!Deprecation::is_deprecated(&env, func.clone()));

        Deprecation::mark_deprecated(&env, func.clone(), ContractVersion::new(2, 0, 0));
        assert!(Deprecation::is_deprecated(&env, func.clone()));

        let removal_version = Deprecation::get_deprecation_version(&env, func.clone()).unwrap();
        assert_eq!(removal_version.major, 2);
    }
}
