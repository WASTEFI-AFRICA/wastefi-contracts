use crate::types::*;
use soroban_sdk::{Address, Env, String, Vec};

/// Waste Token interface
pub trait WasteTokenTrait {
    /// Initialize the token contract
    fn initialize(env: Env, admin: Address, name: String, symbol: String, decimals: u32);

    /// Mint tokens to an address
    fn mint(env: Env, to: Address, amount: i128);

    /// Burn tokens from an address
    fn burn(env: Env, from: Address, amount: i128);

    /// Get token balance
    fn balance(env: Env, account: Address) -> i128;

    /// Transfer tokens
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
}

/// Collector Registry interface
pub trait CollectorRegistryTrait {
    /// Initialize the registry
    fn initialize(env: Env, admin: Address);

    /// Register a new collector
    fn register(env: Env, collector: Address, name: String, phone: String);

    /// Get collector information
    fn get_collector(env: Env, collector: Address) -> Collector;

    /// Update collector status
    fn update_status(env: Env, collector: Address, status: CollectorStatus);

    /// Check if collector is active
    fn is_active(env: Env, collector: Address) -> bool;
}

/// Collection Point interface
pub trait CollectionPointTrait {
    /// Initialize the contract
    fn initialize(env: Env, admin: Address);

    /// Register a new collection point
    fn register_point(
        env: Env,
        owner: Address,
        name: String,
        location: String,
        accepted_materials: Vec<MaterialType>,
    ) -> u64;

    /// Verify a collection point
    fn verify_point(env: Env, point_id: u64);

    /// Get collection point details
    fn get_point(env: Env, point_id: u64) -> CollectionPoint;
}

/// Waste Transaction interface
pub trait WasteTransactionTrait {
    /// Initialize the contract
    fn initialize(env: Env, admin: Address);

    /// Record a new waste collection
    fn record_collection(
        env: Env,
        collector: Address,
        collection_point: Address,
        material_type: MaterialType,
        weight: u64,
    ) -> u64;

    /// Get transaction details
    fn get_transaction(env: Env, transaction_id: u64) -> WasteRecord;

    /// Verify a transaction
    fn verify_transaction(env: Env, transaction_id: u64);

    /// Get transactions by collector
    fn get_collector_transactions(env: Env, collector: Address) -> Vec<WasteRecord>;
}

/// Payment Distribution interface
pub trait PaymentDistributionTrait {
    /// Initialize the contract
    fn initialize(env: Env, admin: Address, token_contract: Address);

    /// Bind the transaction contract that payments are derived from (once)
    fn set_transaction_contract(env: Env, contract_address: Address);

    /// Record the payment owed for a verified transaction. The recipient and amount are
    /// read from the transaction record.
    fn process_verified_payment(env: Env, transaction_id: u64) -> u64;

    /// Record the payments owed for several verified transactions
    fn batch_process_verified(env: Env, transaction_ids: Vec<u64>) -> Vec<u64>;

    /// Admin override: record a payment with a supplied recipient and amount
    fn process_payment(env: Env, transaction_id: u64, recipient: Address, amount: i128) -> u64;

    /// Admin override for several payments
    fn batch_process(env: Env, payments: Vec<(u64, Address, i128)>) -> Vec<u64>;

    /// Complete a payment, which mints it to the recipient
    fn update_payment_status(env: Env, payment_id: u64, status: PaymentStatus);

    /// Get payment details
    fn get_payment(env: Env, payment_id: u64) -> Payment;
}

/// Reputation interface
pub trait ReputationTrait {
    /// Initialize the contract
    fn initialize(env: Env, admin: Address);

    /// Update reputation score
    fn update_score(env: Env, collector: Address, transaction_successful: bool);

    /// Get reputation score
    fn get_score(env: Env, collector: Address) -> ReputationScore;

    /// Calculate new score
    fn calculate_score(env: Env, collector: Address) -> u32;
}

/// Material Pricing interface
pub trait MaterialPricingTrait {
    /// Initialize the contract
    fn initialize(env: Env, admin: Address);

    /// Set price for a material type
    fn set_price(env: Env, material_type: MaterialType, price_per_kg: i128);

    /// Get price for a material type
    fn get_price(env: Env, material_type: MaterialType) -> i128;

    /// Get all prices
    fn get_all_prices(env: Env) -> Vec<MaterialPrice>;
}
