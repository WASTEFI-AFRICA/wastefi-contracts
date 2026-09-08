use soroban_sdk::String;
use crate::errors::WasteFiError;
use crate::types::*;

/// Input validation utilities

/// Validate string is not empty
pub fn validate_non_empty_string(s: &String) -> Result<(), WasteFiError> {
    if s.len() == 0 {
        return Err(WasteFiError::InvalidInput);
    }
    Ok(())
}

/// Validate string length
pub fn validate_string_length(s: &String, min: u32, max: u32) -> Result<(), WasteFiError> {
    let len = s.len();
    if len < min || len > max {
        return Err(WasteFiError::InvalidInput);
    }
    Ok(())
}

/// Validate phone number format (basic check)
pub fn validate_phone_number(phone: &String) -> Result<(), WasteFiError> {
    if phone.len() < 10 || phone.len() > 20 {
        return Err(WasteFiError::InvalidInput);
    }
    Ok(())
}

/// Validate collector name
pub fn validate_collector_name(name: &String) -> Result<(), WasteFiError> {
    validate_string_length(name, 2, 100)
}

/// Validate collection point name
pub fn validate_collection_point_name(name: &String) -> Result<(), WasteFiError> {
    validate_string_length(name, 3, 100)
}

/// Validate location string
pub fn validate_location(location: &String) -> Result<(), WasteFiError> {
    validate_string_length(location, 5, 200)
}

/// Validate material type is valid
pub fn validate_material_type(material_type: &MaterialType) -> Result<(), WasteFiError> {
    // All enum variants are valid
    Ok(())
}

/// Validate weight is within reasonable bounds
pub fn validate_weight_bounds(weight_grams: u64) -> Result<(), WasteFiError> {
    const MIN_WEIGHT: u64 = 10; // 10 grams minimum
    const MAX_WEIGHT: u64 = 1_000_000_000; // 1 million kg maximum
    
    if weight_grams < MIN_WEIGHT || weight_grams > MAX_WEIGHT {
        return Err(WasteFiError::InvalidWeight);
    }
    Ok(())
}

/// Validate price is positive and within reasonable bounds
pub fn validate_price(price_per_kg: i128) -> Result<(), WasteFiError> {
    const MIN_PRICE: i128 = 1; // At least 1 stroop
    const MAX_PRICE: i128 = 100_000_000_000; // 10,000 XLM max
    
    if price_per_kg < MIN_PRICE || price_per_kg > MAX_PRICE {
        return Err(WasteFiError::InvalidPrice);
    }
    Ok(())
}

/// Validate reputation score
pub fn validate_reputation_bounds(score: u32) -> Result<(), WasteFiError> {
    const MAX_SCORE: u32 = 1000;
    
    if score > MAX_SCORE {
        return Err(WasteFiError::InvalidReputationScore);
    }
    Ok(())
}

/// Validate payment amount
pub fn validate_payment_amount(amount: i128) -> Result<(), WasteFiError> {
    if amount <= 0 {
        return Err(WasteFiError::InvalidAmount);
    }
    Ok(())
}

/// Validate collector status transition
pub fn validate_status_transition(
    current: &CollectorStatus,
    new: &CollectorStatus,
) -> Result<(), WasteFiError> {
    match (current, new) {
        // Valid transitions
        (CollectorStatus::Pending, CollectorStatus::Active) => Ok(()),
        (CollectorStatus::Active, CollectorStatus::Suspended) => Ok(()),
        (CollectorStatus::Suspended, CollectorStatus::Active) => Ok(()),
        (CollectorStatus::Active, CollectorStatus::Banned) => Ok(()),
        (CollectorStatus::Suspended, CollectorStatus::Banned) => Ok(()),
        
        // Invalid transitions
        (CollectorStatus::Banned, _) => Err(WasteFiError::CollectorBanned),
        (current_status, new_status) if current_status == new_status => {
            Err(WasteFiError::InvalidCollectorStatus)
        }
        _ => Err(WasteFiError::InvalidCollectorStatus),
    }
}

/// Validate verification status transition
pub fn validate_verification_transition(
    current: &VerificationStatus,
    new: &VerificationStatus,
) -> Result<(), WasteFiError> {
    match (current, new) {
        // Valid transitions
        (VerificationStatus::Unverified, VerificationStatus::Pending) => Ok(()),
        (VerificationStatus::Pending, VerificationStatus::Verified) => Ok(()),
        (VerificationStatus::Pending, VerificationStatus::Rejected) => Ok(()),
        (VerificationStatus::Rejected, VerificationStatus::Pending) => Ok(()),
        
        // Invalid transitions
        (current_status, new_status) if current_status == new_status => {
            Err(WasteFiError::InvalidVerificationStatus)
        }
        _ => Err(WasteFiError::InvalidVerificationStatus),
    }
}

/// Validate transaction can be processed
pub fn validate_transaction_processable(status: &TransactionStatus) -> Result<(), WasteFiError> {
    match status {
        TransactionStatus::Pending => Ok(()),
        TransactionStatus::Completed => Err(WasteFiError::TransactionAlreadyCompleted),
        TransactionStatus::Cancelled => Err(WasteFiError::TransactionCancelled),
        TransactionStatus::Disputed => Err(WasteFiError::InvalidTransactionStatus),
    }
}

/// Validate payment can be processed
pub fn validate_payment_processable(status: &PaymentStatus) -> Result<(), WasteFiError> {
    match status {
        PaymentStatus::Pending => Ok(()),
        PaymentStatus::Processing => Ok(()),
        PaymentStatus::Completed => Err(WasteFiError::PaymentAlreadyProcessed),
        PaymentStatus::Failed => Err(WasteFiError::PaymentFailed),
        PaymentStatus::Refunded => Err(WasteFiError::InvalidPaymentStatus),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_validate_string_length() {
        let env = Env::default();
        let short = String::from_str(&env, "ab");
        let valid = String::from_str(&env, "valid name");
        
        assert!(validate_string_length(&valid, 3, 50).is_ok());
        assert!(validate_string_length(&short, 3, 50).is_err());
    }

    #[test]
    fn test_validate_weight_bounds() {
        assert!(validate_weight_bounds(1000).is_ok());
        assert!(validate_weight_bounds(5).is_err()); // Too small
        assert!(validate_weight_bounds(1_000_000_001).is_err()); // Too large
    }

    #[test]
    fn test_validate_price() {
        assert!(validate_price(10000000).is_ok());
        assert!(validate_price(0).is_err());
        assert!(validate_price(-100).is_err());
    }

    #[test]
    fn test_validate_reputation_bounds() {
        assert!(validate_reputation_bounds(500).is_ok());
        assert!(validate_reputation_bounds(1000).is_ok());
        assert!(validate_reputation_bounds(1001).is_err());
    }

    #[test]
    fn test_status_transitions() {
        // Valid transition
        assert!(validate_status_transition(
            &CollectorStatus::Pending,
            &CollectorStatus::Active
        ).is_ok());
        
        // Invalid transition
        assert!(validate_status_transition(
            &CollectorStatus::Banned,
            &CollectorStatus::Active
        ).is_err());
    }

    #[test]
    fn test_transaction_processable() {
        assert!(validate_transaction_processable(&TransactionStatus::Pending).is_ok());
        assert!(validate_transaction_processable(&TransactionStatus::Completed).is_err());
    }
}
