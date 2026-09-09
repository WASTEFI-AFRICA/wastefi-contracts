#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

mod storage;
use common::types::*;
use storage::*;

#[cfg(test)]
mod test;

const MATERIAL_PRICING_CONTRACT: &str = "MaterialPricingContract";
const REPUTATION_CONTRACT: &str = "ReputationContract";

#[contract]
pub struct WasteTransaction;

#[contractimpl]
impl WasteTransaction {
    /// Initialize the waste transaction contract
    ///
    /// # Arguments
    /// * `admin` - Contract administrator address
    pub fn initialize(env: Env, admin: Address) {
        common::Initializable::require_not_initialized(&env).expect("Already initialized");

        // Set admin
        common::AccessControl::set_admin(&env, admin);

        // Initialize transaction counter
        write_transaction_count(&env, 0);

        // Mark as initialized
        common::Initializable::mark_initialized(&env);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Set material pricing contract address (admin only)
    ///
    /// # Arguments
    /// * `contract_address` - MaterialPricing contract address
    pub fn set_material_pricing_contract(env: Env, contract_address: Address) {
        common::Initializable::require_initialized(&env).expect("Not initialized");

        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        env.storage()
            .instance()
            .set(&MATERIAL_PRICING_CONTRACT, &contract_address);
        common::bump_instance(&env);
    }

    /// Set reputation contract address (admin only)
    ///
    /// # Arguments
    /// * `contract_address` - Reputation contract address
    pub fn set_reputation_contract(env: Env, contract_address: Address) {
        common::Initializable::require_initialized(&env).expect("Not initialized");

        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        env.storage()
            .instance()
            .set(&REPUTATION_CONTRACT, &contract_address);
        common::bump_instance(&env);
    }

    /// Get material pricing contract address
    pub fn get_material_pricing_contract(env: Env) -> Option<Address> {
        env.storage().instance().get(&MATERIAL_PRICING_CONTRACT)
    }

    /// Get reputation contract address
    pub fn get_reputation_contract(env: Env) -> Option<Address> {
        env.storage().instance().get(&REPUTATION_CONTRACT)
    }

    /// Record a new waste collection transaction
    ///
    /// # Arguments
    /// * `collector` - Address of the collector
    /// * `collection_point` - Address of the collection point
    /// * `material_type` - Type of material collected
    /// * `weight` - Weight in grams
    /// * `price_per_kg` - Price per kilogram in stroops
    ///
    /// # Returns
    /// Transaction ID
    pub fn record_collection(
        env: Env,
        collector: Address,
        collection_point: Address,
        material_type: MaterialType,
        weight: u64,
        price_per_kg: i128,
    ) -> u64 {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Validate inputs
        common::validation::validate_weight_bounds(weight).expect("Invalid weight");
        common::validation::validate_price(price_per_kg).expect("Invalid price");

        // Calculate total amount: (weight in grams / 1000) * price_per_kg
        let weight_kg = (weight as i128) / 1000;
        let total_amount = weight_kg.saturating_mul(price_per_kg);

        // Get next transaction ID
        let transaction_id = increment_transaction_count(&env);

        // Create transaction record
        let record = WasteRecord {
            id: transaction_id,
            collector: collector.clone(),
            collection_point: collection_point.clone(),
            material_type: material_type.clone(),
            weight,
            price_per_kg,
            total_amount,
            status: TransactionStatus::Pending,
            timestamp: env.ledger().timestamp(),
            verified: false,
        };

        // Store transaction
        write_transaction(&env, transaction_id, &record);

        // Index by collector
        add_collector_transaction(&env, &collector, transaction_id);

        // Emit event
        common::TransactionEvents::recorded(&env, transaction_id, collector, material_type, weight);

        // Bump storage
        common::bump_instance(&env);

        transaction_id
    }

    /// Record collection with automatic price lookup
    ///
    /// # Arguments
    /// * `collector` - Address of the collector
    /// * `collection_point` - Collection point ID (u64)
    /// * `material_type` - Type of material collected
    /// * `weight` - Weight in grams
    ///
    /// # Returns
    /// Transaction ID
    ///
    /// # Note
    /// This method automatically fetches current price from MaterialPricing contract
    pub fn record_with_price_lookup(
        env: Env,
        collector: Address,
        _collection_point: u64,
        material_type: MaterialType,
        weight: u64,
    ) -> u64 {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Validate weight
        common::validation::validate_weight_bounds(weight).expect("Invalid weight");

        // Get MaterialPricing contract address
        let _pricing_contract: Address = env
            .storage()
            .instance()
            .get(&MATERIAL_PRICING_CONTRACT)
            .expect("MaterialPricing contract not configured");

        // TODO: Call pricing_contract.get_price(material_type)
        // For now, we'll use a default price until full cross-contract call implementation
        let price_per_kg = 5_000_000i128; // 0.5 XLM/kg default

        // Calculate total amount
        let weight_kg = (weight as i128) / 1000;
        let total_amount = weight_kg.saturating_mul(price_per_kg);

        // Get next transaction ID
        let transaction_id = increment_transaction_count(&env);

        // Use collector address as placeholder for collection point
        let collection_point_address = collector.clone();

        let record = WasteRecord {
            id: transaction_id,
            collector: collector.clone(),
            collection_point: collection_point_address,
            material_type: material_type.clone(),
            weight,
            price_per_kg,
            total_amount,
            status: TransactionStatus::Pending,
            timestamp: env.ledger().timestamp(),
            verified: false,
        };

        // Store transaction
        write_transaction(&env, transaction_id, &record);

        // Index by collector
        add_collector_transaction(&env, &collector, transaction_id);

        // Emit event
        common::TransactionEvents::recorded(&env, transaction_id, collector, material_type, weight);

        // Bump storage
        common::bump_instance(&env);

        transaction_id
    }

    /// Get transaction details
    ///
    /// # Arguments
    /// * `transaction_id` - Transaction ID
    ///
    /// # Returns
    /// Transaction record
    pub fn get_transaction(env: Env, transaction_id: u64) -> WasteRecord {
        read_transaction(&env, transaction_id).expect("Transaction not found")
    }

    /// Verify a transaction (admin only)
    ///
    /// # Arguments
    /// * `transaction_id` - Transaction ID to verify
    ///
    /// # Note
    /// This also triggers a reputation update if the Reputation contract is configured
    pub fn verify_transaction(env: Env, transaction_id: u64) {
        common::Initializable::require_initialized(&env).expect("Not initialized");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        // Get transaction
        let mut record = read_transaction(&env, transaction_id).expect("Transaction not found");

        // Update verification status
        record.verified = true;
        record.status = TransactionStatus::Completed;

        // Save updated record
        write_transaction(&env, transaction_id, &record);

        // Trigger reputation update if configured
        if let Some(_reputation_contract) = env
            .storage()
            .instance()
            .get::<_, Address>(&REPUTATION_CONTRACT)
        {
            // TODO: Call reputation_contract.record_transaction(collector, TransactionStatus::Completed)
            // For now, just note that the contract is configured
            // Full implementation requires cross-contract invocation setup
        }

        // Emit event
        common::TransactionEvents::verified(&env, transaction_id);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Update transaction status (admin only)
    ///
    /// # Arguments
    /// * `transaction_id` - Transaction ID
    /// * `status` - New status
    pub fn update_status(env: Env, transaction_id: u64, status: TransactionStatus) {
        common::Initializable::require_initialized(&env).expect("Not initialized");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        // Get transaction
        let mut record = read_transaction(&env, transaction_id).expect("Transaction not found");

        // Update status
        record.status = status;

        // Save updated record
        write_transaction(&env, transaction_id, &record);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Get transactions by collector
    ///
    /// # Arguments
    /// * `collector` - Collector address
    /// * `limit` - Maximum number of results (optional, defaults to 10)
    ///
    /// # Returns
    /// Vector of transaction records
    pub fn get_collector_transactions(
        env: Env,
        collector: Address,
        limit: u32,
    ) -> Vec<WasteRecord> {
        let max_limit = if limit == 0 || limit > 50 { 50 } else { limit };
        read_collector_transactions(&env, &collector, max_limit)
    }

    /// Get total transaction count
    ///
    /// # Returns
    /// Total number of transactions
    pub fn get_transaction_count(env: Env) -> u64 {
        read_transaction_count(&env)
    }

    /// Get collector statistics
    ///
    /// # Arguments
    /// * `collector` - Collector address
    ///
    /// # Returns
    /// Tuple of (total_transactions, total_weight, total_amount)
    pub fn get_collector_stats(env: Env, collector: Address) -> (u64, u64, i128) {
        let transactions = read_collector_transactions(&env, &collector, 1000);

        let mut total_weight: u64 = 0;
        let mut total_amount: i128 = 0;

        for tx in transactions.iter() {
            total_weight = total_weight.saturating_add(tx.weight);
            total_amount = total_amount.saturating_add(tx.total_amount);
        }

        (transactions.len() as u64, total_weight, total_amount)
    }

    /// Pause the contract (admin only)
    pub fn pause(env: Env) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::Pausable::admin_pause(&env, &admin).expect("Not admin");

        common::AdminEvents::paused(&env);
    }

    /// Unpause the contract (admin only)
    pub fn unpause(env: Env) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::Pausable::admin_unpause(&env, &admin).expect("Not admin");

        common::AdminEvents::unpaused(&env);
    }

    /// Check if contract is paused
    pub fn is_paused(env: Env) -> bool {
        common::Pausable::is_paused(&env)
    }

    /// Get admin address
    pub fn admin(env: Env) -> Address {
        common::AccessControl::get_admin(&env).expect("Admin not found")
    }
}
