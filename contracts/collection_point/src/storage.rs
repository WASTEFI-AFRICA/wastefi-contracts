use common::CollectionPoint;
use soroban_sdk::{Address, Env};

/// Storage keys
const POINT_COUNT: &str = "PointCount";

/// Write collection point data
pub fn write_point(env: &Env, point_id: u64, point: &CollectionPoint) {
    let key = ("Point", point_id);
    env.storage().persistent().set(&key, point);

    // Bump storage TTL
    let storage_key = common::StorageKey::CollectionPoint(point_id);
    common::bump_persistent(env, &storage_key);
}

/// Read collection point data
pub fn read_point(env: &Env, point_id: u64) -> Option<CollectionPoint> {
    let key = ("Point", point_id);
    env.storage().persistent().get(&key)
}

/// Check if collection point exists
pub fn has_point(env: &Env, point_id: u64) -> bool {
    let key = ("Point", point_id);
    env.storage().persistent().has(&key)
}

/// Write collection point ID by owner
pub fn write_point_by_owner(env: &Env, owner: &Address, point_id: u64) {
    let key = ("PointOwner", owner);
    env.storage().persistent().set(&key, &point_id);

    // Bump storage TTL
    let storage_key = common::StorageKey::CollectionPointByOwner(owner.clone());
    common::bump_persistent(env, &storage_key);
}

/// Read collection point ID by owner
pub fn read_point_by_owner(env: &Env, owner: &Address) -> Option<u64> {
    let key = ("PointOwner", owner);
    env.storage().persistent().get(&key)
}

/// Write collection point count
pub fn write_point_count(env: &Env, count: u64) {
    env.storage().instance().set(&POINT_COUNT, &count);
}

/// Read collection point count
pub fn read_point_count(env: &Env) -> u64 {
    env.storage().instance().get(&POINT_COUNT).unwrap_or(0)
}
