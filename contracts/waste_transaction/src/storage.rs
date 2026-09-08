use soroban_sdk::{Address, Env, Vec};
use common::WasteRecord;

/// Storage keys
const TRANSACTION_COUNT: &str = "TransactionCount";

/// Write waste transaction data
pub fn write_transaction(env: &Env, transaction_id: u64, transaction: &WasteRecord) {
    let key = ("Transaction", transaction_id);
    env.storage().persistent().set(&key, transaction);
    
    // Bump storage TTL
    let storage_key = common::StorageKey::WasteRecord(transaction_id);
    common::bump_persistent(env, &storage_key);
}

/// Read waste transaction data
pub fn read_transaction(env: &Env, transaction_id: u64) -> Option<WasteRecord> {
    let key = ("Transaction", transaction_id);
    env.storage().persistent().get(&key)
}

/// Check if transaction exists
pub fn has_transaction(env: &Env, transaction_id: u64) -> bool {
    let key = ("Transaction", transaction_id);
    env.storage().persistent().has(&key)
}

/// Write transaction count
pub fn write_transaction_count(env: &Env, count: u64) {
    env.storage().instance().set(&TRANSACTION_COUNT, &count);
}

/// Read transaction count
pub fn read_transaction_count(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&TRANSACTION_COUNT)
        .unwrap_or(0)
}

/// Add transaction to collector's list
pub fn add_to_collector_transactions(env: &Env, collector: &Address, transaction_id: u64) {
    let key = ("CollectorTxs", collector);
    let mut txs: Vec<u64> = env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));
    
    txs.push_back(transaction_id);
    env.storage().persistent().set(&key, &txs);
    
    // Bump storage TTL
    let storage_key = common::StorageKey::TransactionsByCollector(collector.clone(), 0);
    common::bump_persistent(env, &storage_key);
}

/// Get collector's transactions
pub fn read_collector_transactions(env: &Env, collector: &Address) -> Vec<u64> {
    let key = ("CollectorTxs", collector);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

/// Add transaction to collection point's list
pub fn add_to_point_transactions(env: &Env, point: &Address, transaction_id: u64) {
    let key = ("PointTxs", point);
    let mut txs: Vec<u64> = env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));
    
    txs.push_back(transaction_id);
    env.storage().persistent().set(&key, &txs);
}

/// Get collection point's transactions
pub fn read_point_transactions(env: &Env, point: &Address) -> Vec<u64> {
    let key = ("PointTxs", point);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}
