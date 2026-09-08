#![cfg(test)]
use soroban_sdk::{Address, Env, String};
use crate::types::*;

/// Test utilities for WasteFi contracts

/// Create a test environment
pub fn create_test_env() -> Env {
    Env::default()
}

/// Generate a test address from a seed
pub fn generate_address(env: &Env, seed: u32) -> Address {
    Address::generate(env)
}

/// Create a test collector
pub fn create_test_collector(env: &Env, name: &str) -> Collector {
    Collector {
        address: generate_address(env, 1),
        name: String::from_str(env, name),
        phone: String::from_str(env, "+1234567890"),
        status: CollectorStatus::Active,
        reputation_score: 500,
        total_collections: 0,
        total_weight: 0,
        registration_time: env.ledger().timestamp(),
        last_active: env.ledger().timestamp(),
    }
}

/// Create a test collection point
pub fn create_test_collection_point(env: &Env, name: &str) -> CollectionPoint {
    CollectionPoint {
        id: 1,
        owner: generate_address(env, 2),
        name: String::from_str(env, name),
        location: String::from_str(env, "Test Location"),
        verification_status: VerificationStatus::Verified,
        accepted_materials: soroban_sdk::vec![
            env,
            MaterialType::Plastic,
            MaterialType::Glass,
            MaterialType::Metal,
        ],
        total_processed: 0,
        created_at: env.ledger().timestamp(),
    }
}

/// Create a test waste record
pub fn create_test_waste_record(env: &Env) -> WasteRecord {
    WasteRecord {
        id: 1,
        collector: generate_address(env, 1),
        collection_point: generate_address(env, 2),
        material_type: MaterialType::Plastic,
        weight: 5000, // 5kg in grams
        price_per_kg: 1_0000000, // 1 XLM per kg
        total_amount: 5_0000000, // 5 XLM total
        status: TransactionStatus::Completed,
        timestamp: env.ledger().timestamp(),
        verified: true,
    }
}

/// Create a test material price
pub fn create_test_material_price(env: &Env, material: MaterialType, price: i128) -> MaterialPrice {
    MaterialPrice {
        material_type: material,
        price_per_kg: price,
        last_updated: env.ledger().timestamp(),
        updated_by: generate_address(env, 0),
    }
}

/// Create a test payment
pub fn create_test_payment(env: &Env, amount: i128) -> Payment {
    Payment {
        id: 1,
        recipient: generate_address(env, 1),
        amount,
        status: PaymentStatus::Pending,
        transaction_id: 1,
        created_at: env.ledger().timestamp(),
        processed_at: 0,
    }
}

/// Create a test reputation score
pub fn create_test_reputation(env: &Env) -> ReputationScore {
    ReputationScore {
        collector: generate_address(env, 1),
        score: 750,
        total_transactions: 10,
        successful_transactions: 9,
        disputed_transactions: 1,
        last_updated: env.ledger().timestamp(),
    }
}

/// Create a test carbon credit
pub fn create_test_carbon_credit(env: &Env) -> CarbonCredit {
    CarbonCredit {
        transaction_id: 1,
        material_type: MaterialType::Plastic,
        weight: 5000, // 5kg
        co2_saved: 12000, // 12kg CO2
        credit_amount: 12_0000000,
        issued_at: env.ledger().timestamp(),
    }
}

/// Advance ledger time by seconds
pub fn advance_time(env: &Env, seconds: u64) {
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp.saturating_add(seconds);
    });
}

/// Set ledger timestamp
pub fn set_time(env: &Env, timestamp: u64) {
    env.ledger().with_mut(|li| {
        li.timestamp = timestamp;
    });
}
