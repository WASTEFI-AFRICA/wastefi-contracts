use soroban_sdk::{Address, Env, Vec};
use common::Collector;

/// Storage keys
const COLLECTOR_COUNT: &str = "CollectorCount";
const COLLECTOR_LIST: &str = "CollectorList";

/// Write collector data
pub fn write_collector(env: &Env, address: &Address, collector: &Collector) {
    let key = ("Collector", address);
    env.storage().persistent().set(&key, collector);
    
    // Bump storage TTL
    let storage_key = common::StorageKey::Collector(address.clone());
    common::bump_persistent(env, &storage_key);
    
    // Add to list if not exists
    if !is_in_list(env, address) {
        add_to_list(env, address);
    }
}

/// Read collector data
pub fn read_collector(env: &Env, address: &Address) -> Option<Collector> {
    let key = ("Collector", address);
    env.storage().persistent().get(&key)
}

/// Check if collector exists
pub fn has_collector(env: &Env, address: &Address) -> bool {
    let key = ("Collector", address);
    env.storage().persistent().has(&key)
}

/// Write collector count
pub fn write_collector_count(env: &Env, count: u64) {
    env.storage().instance().set(&COLLECTOR_COUNT, &count);
}

/// Read collector count
pub fn read_collector_count(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&COLLECTOR_COUNT)
        .unwrap_or(0)
}

/// Add collector to list
fn add_to_list(env: &Env, address: &Address) {
    let mut list: Vec<Address> = env.storage()
        .persistent()
        .get(&COLLECTOR_LIST)
        .unwrap_or(Vec::new(env));
    
    list.push_back(address.clone());
    env.storage().persistent().set(&COLLECTOR_LIST, &list);
}

/// Check if address is in list
fn is_in_list(env: &Env, address: &Address) -> bool {
    let list: Vec<Address> = env.storage()
        .persistent()
        .get(&COLLECTOR_LIST)
        .unwrap_or(Vec::new(env));
    
    for item in list.iter() {
        if &item == address {
            return true;
        }
    }
    false
}

/// Read all collectors (paginated)
pub fn read_all_collectors(env: &Env, start: u64, limit: u64) -> Vec<Address> {
    let all_collectors: Vec<Address> = env.storage()
        .persistent()
        .get(&COLLECTOR_LIST)
        .unwrap_or(Vec::new(env));
    
    let mut result = Vec::new(env);
    let end = (start + limit).min(all_collectors.len() as u64);
    
    for i in start..end {
        if let Some(addr) = all_collectors.get(i as u32) {
            result.push_back(addr);
        }
    }
    
    result
}
