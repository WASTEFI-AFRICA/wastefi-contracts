use soroban_sdk::{Address, Env};
use crate::errors::WasteFiError;
use crate::storage::StorageKey;

/// Utility functions for WasteFi contracts

/// Check if contract is initialized
pub fn require_initialized(env: &Env) -> Result<(), WasteFiError> {
    if !env.storage().instance().has(&StorageKey::Initialized) {
        return Err(WasteFiError::NotInitialized);
    }
    Ok(())
}

/// Check if contract is not paused
pub fn require_not_paused(env: &Env) -> Result<(), WasteFiError> {
    if env.storage().instance().has(&StorageKey::Paused) {
        let paused: bool = env.storage().instance().get(&StorageKey::Paused).unwrap();
        if paused {
            return Err(WasteFiError::ContractPaused);
        }
    }
    Ok(())
}

/// Check if caller is admin
pub fn require_admin(env: &Env, caller: &Address) -> Result<(), WasteFiError> {
    let admin: Address = env
        .storage()
        .instance()
        .get(&StorageKey::Admin)
        .ok_or(WasteFiError::NotInitialized)?;
    
    if caller != &admin {
        return Err(WasteFiError::NotAdmin);
    }
    Ok(())
}

/// Check if address is authorized
pub fn require_auth(_env: &Env, address: &Address) {
    address.require_auth();
}

/// Validate non-zero amount
pub fn validate_amount(amount: i128) -> Result<(), WasteFiError> {
    if amount <= 0 {
        return Err(WasteFiError::InvalidAmount);
    }
    Ok(())
}

/// Validate weight (must be greater than zero)
pub fn validate_weight(weight: u64) -> Result<(), WasteFiError> {
    if weight == 0 {
        return Err(WasteFiError::InvalidWeight);
    }
    Ok(())
}

/// Validate reputation score (0-1000)
pub fn validate_reputation_score(score: u32) -> Result<(), WasteFiError> {
    if score > 1000 {
        return Err(WasteFiError::InvalidReputationScore);
    }
    Ok(())
}

/// Check if value exists in storage
pub fn require_exists<T>(value: Option<T>) -> Result<T, WasteFiError> {
    value.ok_or(WasteFiError::NotFound)
}

/// Calculate percentage
/// Returns (value * percentage) / 100
pub fn calculate_percentage(value: i128, percentage: u32) -> i128 {
    value.saturating_mul(percentage as i128) / 100
}

/// Convert kg to grams
pub fn kg_to_grams(kg: u64) -> u64 {
    kg.saturating_mul(1000)
}

/// Convert grams to kg
pub fn grams_to_kg(grams: u64) -> u64 {
    grams / 1000
}

/// Convert price per kg to price per gram
pub fn price_per_kg_to_gram(price_per_kg: i128) -> i128 {
    price_per_kg / 1000
}

/// Calculate total amount from weight and price per kg
pub fn calculate_total_amount(weight_grams: u64, price_per_kg: i128) -> i128 {
    let weight_kg = grams_to_kg(weight_grams);
    price_per_kg.saturating_mul(weight_kg as i128)
}

/// Get current timestamp
pub fn get_timestamp(env: &Env) -> u64 {
    env.ledger().timestamp()
}

/// Check if time has elapsed
pub fn has_time_elapsed(env: &Env, start_time: u64, duration: u64) -> bool {
    get_timestamp(env) >= start_time.saturating_add(duration)
}

/// Bump storage expiration (for persistent data)
pub fn bump_instance(env: &Env) {
    const DAY_IN_LEDGERS: u32 = 17280; // approximately
    const INSTANCE_LIFETIME_THRESHOLD: u32 = 30 * DAY_IN_LEDGERS; // 30 days
    const INSTANCE_BUMP_AMOUNT: u32 = 90 * DAY_IN_LEDGERS; // 90 days
    
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

/// Bump temporary storage
pub fn bump_temporary(env: &Env, key: &StorageKey) {
    const DAY_IN_LEDGERS: u32 = 17280;
    const TEMP_LIFETIME_THRESHOLD: u32 = 7 * DAY_IN_LEDGERS; // 7 days
    const TEMP_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS; // 30 days
    
    env.storage()
        .temporary()
        .extend_ttl(key, TEMP_LIFETIME_THRESHOLD, TEMP_BUMP_AMOUNT);
}

/// Bump persistent storage
pub fn bump_persistent(env: &Env, key: &StorageKey) {
    const DAY_IN_LEDGERS: u32 = 17280;
    const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120 * DAY_IN_LEDGERS; // 120 days
    const PERSISTENT_BUMP_AMOUNT: u32 = 365 * DAY_IN_LEDGERS; // 365 days
    
    env.storage()
        .persistent()
        .extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_validate_amount() {
        assert!(validate_amount(100).is_ok());
        assert!(validate_amount(0).is_err());
        assert!(validate_amount(-100).is_err());
    }

    #[test]
    fn test_validate_weight() {
        assert!(validate_weight(1000).is_ok());
        assert!(validate_weight(0).is_err());
    }

    #[test]
    fn test_validate_reputation_score() {
        assert!(validate_reputation_score(500).is_ok());
        assert!(validate_reputation_score(1000).is_ok());
        assert!(validate_reputation_score(1001).is_err());
    }

    #[test]
    fn test_calculate_percentage() {
        assert_eq!(calculate_percentage(1000, 10), 100);
        assert_eq!(calculate_percentage(1000, 50), 500);
        assert_eq!(calculate_percentage(1000, 100), 1000);
    }

    #[test]
    fn test_kg_grams_conversion() {
        assert_eq!(kg_to_grams(5), 5000);
        assert_eq!(grams_to_kg(5000), 5);
    }

    #[test]
    fn test_calculate_total_amount() {
        // 5000 grams (5kg) at 1 XLM per kg = 5 XLM
        assert_eq!(calculate_total_amount(5000, 10000000), 50000000);
    }

    #[test]
    fn test_get_timestamp() {
        let env = Env::default();
        let ts = get_timestamp(&env);
        assert!(ts > 0);
    }
}
