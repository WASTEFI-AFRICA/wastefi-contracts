use common::types::WasteRecord;
use soroban_sdk::{Address, Env, Vec};

const TRANSACTION_COUNT: &str = "TxCount";

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

/// Increment and return new transaction count
pub fn increment_transaction_count(env: &Env) -> u64 {
    let count = read_transaction_count(env);
    let new_count = count + 1;
    write_transaction_count(env, new_count);
    new_count
}

/// Write transaction record
pub fn write_transaction(env: &Env, transaction_id: u64, record: &WasteRecord) {
    let key = common::StorageKey::WasteRecord(transaction_id);
    env.storage().persistent().set(&key, record);

    // Bump storage TTL
    common::bump_persistent(env, &key);
}

/// Read transaction record
pub fn read_transaction(env: &Env, transaction_id: u64) -> Option<WasteRecord> {
    let key = common::StorageKey::WasteRecord(transaction_id);
    env.storage().persistent().get(&key)
}

/// Add transaction to collector's list
pub fn add_collector_transaction(env: &Env, collector: &Address, transaction_id: u64) {
    let key = ("CollectorTxs", collector);

    // Get existing transactions or create new list
    let mut transactions: Vec<u64> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));

    // Add new transaction
    transactions.push_back(transaction_id);

    // Store updated list
    env.storage().persistent().set(&key, &transactions);

    // Bump storage TTL
    let storage_key = common::StorageKey::TransactionsByCollector(collector.clone(), 0);
    common::bump_persistent(env, &storage_key);
}

/// Read collector transactions
pub fn read_collector_transactions(
    env: &Env,
    collector: &Address,
    limit: u32,
) -> Vec<WasteRecord> {
    let key = ("CollectorTxs", collector);

    let transaction_ids: Vec<u64> = env.storage().persistent().get(&key).unwrap_or(Vec::new(env));

    let mut records = Vec::new(env);

    // Get the most recent transactions up to limit
    let start_idx = if transaction_ids.len() > limit {
        transaction_ids.len() - limit
    } else {
        0
    };

    for i in start_idx..transaction_ids.len() {
        if let Some(tx_id) = transaction_ids.get(i) {
            if let Some(record) = read_transaction(env, tx_id) {
                records.push_back(record);
            }
        }
    }

    records
}
