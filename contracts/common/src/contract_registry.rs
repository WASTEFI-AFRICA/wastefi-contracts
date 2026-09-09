// Contract registry utilities for managing cross-contract interactions
use soroban_sdk::{Address, Env};

/// Storage keys for contract addresses
pub const WASTE_TOKEN_CONTRACT: &str = "WasteTokenContract";
pub const COLLECTOR_REGISTRY_CONTRACT: &str = "CollectorRegistryContract";
pub const COLLECTION_POINT_CONTRACT: &str = "CollectionPointContract";
pub const WASTE_TRANSACTION_CONTRACT: &str = "WasteTransactionContract";
pub const PAYMENT_DISTRIBUTION_CONTRACT: &str = "PaymentDistributionContract";
pub const REPUTATION_CONTRACT: &str = "ReputationContract";
pub const MATERIAL_PRICING_CONTRACT: &str = "MaterialPricingContract";

/// Contract registry for managing inter-contract dependencies
pub struct ContractRegistry;

impl ContractRegistry {
    /// Register a contract address in storage
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `key` - Storage key for the contract
    /// * `address` - Contract address
    pub fn register(env: &Env, key: &str, address: &Address) {
        env.storage().instance().set(&key, address);
        crate::bump_instance(env);
    }

    /// Get a registered contract address
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `key` - Storage key for the contract
    ///
    /// # Returns
    /// Optional contract address
    pub fn get(env: &Env, key: &str) -> Option<Address> {
        env.storage().instance().get(&key)
    }

    /// Check if a contract is registered
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `key` - Storage key for the contract
    ///
    /// # Returns
    /// True if contract is registered
    pub fn is_registered(env: &Env, key: &str) -> bool {
        env.storage().instance().has(&key)
    }

    /// Remove a contract registration
    ///
    /// # Arguments
    /// * `env` - Contract environment
    /// * `key` - Storage key for the contract
    pub fn unregister(env: &Env, key: &str) {
        env.storage().instance().remove(&key);
    }

    /// Get all registered contract names (for debugging/admin)
    ///
    /// # Returns
    /// Vector of contract type names that are commonly used
    pub fn get_all_contract_keys() -> [&'static str; 7] {
        [
            WASTE_TOKEN_CONTRACT,
            COLLECTOR_REGISTRY_CONTRACT,
            COLLECTION_POINT_CONTRACT,
            WASTE_TRANSACTION_CONTRACT,
            PAYMENT_DISTRIBUTION_CONTRACT,
            REPUTATION_CONTRACT,
            MATERIAL_PRICING_CONTRACT,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_register_and_get_contract() {
        let env = Env::default();
        let address = Address::generate(&env);

        ContractRegistry::register(&env, WASTE_TOKEN_CONTRACT, &address);

        let retrieved = ContractRegistry::get(&env, WASTE_TOKEN_CONTRACT);
        assert_eq!(retrieved, Some(address));
    }

    #[test]
    fn test_is_registered() {
        let env = Env::default();
        let address = Address::generate(&env);

        assert!(!ContractRegistry::is_registered(&env, WASTE_TOKEN_CONTRACT));

        ContractRegistry::register(&env, WASTE_TOKEN_CONTRACT, &address);

        assert!(ContractRegistry::is_registered(&env, WASTE_TOKEN_CONTRACT));
    }

    #[test]
    fn test_unregister_contract() {
        let env = Env::default();
        let address = Address::generate(&env);

        ContractRegistry::register(&env, WASTE_TOKEN_CONTRACT, &address);
        assert!(ContractRegistry::is_registered(&env, WASTE_TOKEN_CONTRACT));

        ContractRegistry::unregister(&env, WASTE_TOKEN_CONTRACT);
        assert!(!ContractRegistry::is_registered(&env, WASTE_TOKEN_CONTRACT));
    }

    #[test]
    fn test_get_all_contract_keys() {
        let keys = ContractRegistry::get_all_contract_keys();
        assert_eq!(keys.len(), 7);
        assert_eq!(keys[0], WASTE_TOKEN_CONTRACT);
        assert_eq!(keys[6], MATERIAL_PRICING_CONTRACT);
    }
}
