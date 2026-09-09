#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env,
};

fn create_contract(env: &Env) -> (Address, MaterialPricingClient) {
    let contract_id = env.register_contract(None, MaterialPricing);
    let client = MaterialPricingClient::new(env, &contract_id);
    (contract_id, client)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    assert_eq!(client.admin(), admin);

    // Check that default prices were set
    let plastic_price = client.get_price(&MaterialType::Plastic);
    assert_eq!(plastic_price, 8_000_000); // 0.8 XLM/kg
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_cannot_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);
    client.initialize(&admin); // Should panic
}

#[test]
fn test_default_prices() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Verify all default prices
    assert_eq!(client.get_price(&MaterialType::Plastic), 8_000_000);
    assert_eq!(client.get_price(&MaterialType::Glass), 5_000_000);
    assert_eq!(client.get_price(&MaterialType::Metal), 15_000_000);
    assert_eq!(client.get_price(&MaterialType::Paper), 4_000_000);
    assert_eq!(client.get_price(&MaterialType::Cardboard), 3_000_000);
    assert_eq!(client.get_price(&MaterialType::Electronics), 25_000_000);
    assert_eq!(client.get_price(&MaterialType::Organic), 2_000_000);
    assert_eq!(client.get_price(&MaterialType::Textile), 6_000_000);
    assert_eq!(client.get_price(&MaterialType::Rubber), 7_000_000);
    assert_eq!(client.get_price(&MaterialType::Other), 3_000_000);
}

#[test]
fn test_set_price() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000);

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Update plastic price
    let new_price = 10_000_000; // 1 XLM/kg
    client.set_price(&MaterialType::Plastic, &new_price);

    // Verify price was updated
    assert_eq!(client.get_price(&MaterialType::Plastic), new_price);

    // Check metadata
    let record = client.get_price_record(&MaterialType::Plastic);
    assert_eq!(record.price_per_kg, new_price);
    assert_eq!(record.last_updated, 1000);
    assert_eq!(record.updated_by, admin);
}

#[test]
fn test_get_price_record() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 2000);

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    let record = client.get_price_record(&MaterialType::Metal);
    assert_eq!(record.material_type, MaterialType::Metal);
    assert_eq!(record.price_per_kg, 15_000_000);
    assert_eq!(record.updated_by, admin);
}

#[test]
fn test_get_all_prices() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    let all_prices = client.get_all_prices();
    assert_eq!(all_prices.len(), 10); // All 10 material types

    // Verify at least one price
    let first_price = all_prices.get(0).unwrap();
    assert!(first_price.price_per_kg > 0);
}

#[test]
fn test_batch_update_prices() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Prepare batch update
    let mut updates = Vec::new(&env);
    updates.push_back((MaterialType::Plastic, 12_000_000));
    updates.push_back((MaterialType::Glass, 7_000_000));
    updates.push_back((MaterialType::Metal, 20_000_000));

    client.batch_update_prices(&updates);

    // Verify updates
    assert_eq!(client.get_price(&MaterialType::Plastic), 12_000_000);
    assert_eq!(client.get_price(&MaterialType::Glass), 7_000_000);
    assert_eq!(client.get_price(&MaterialType::Metal), 20_000_000);

    // Other prices should remain unchanged
    assert_eq!(client.get_price(&MaterialType::Paper), 4_000_000);
}

#[test]
#[should_panic(expected = "Invalid price")]
fn test_set_invalid_price_zero() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Try to set price to 0
    client.set_price(&MaterialType::Plastic, &0);
}

#[test]
#[should_panic(expected = "Invalid price")]
fn test_set_invalid_price_negative() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Try to set negative price
    client.set_price(&MaterialType::Plastic, &-1000);
}

#[test]
fn test_multiple_price_updates() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Update same material multiple times
    client.set_price(&MaterialType::Plastic, &10_000_000);
    assert_eq!(client.get_price(&MaterialType::Plastic), 10_000_000);

    client.set_price(&MaterialType::Plastic, &12_000_000);
    assert_eq!(client.get_price(&MaterialType::Plastic), 12_000_000);

    client.set_price(&MaterialType::Plastic, &9_000_000);
    assert_eq!(client.get_price(&MaterialType::Plastic), 9_000_000);
}

#[test]
fn test_pause_unpause() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Check not paused initially
    assert!(!client.is_paused());

    // Pause
    client.pause();
    assert!(client.is_paused());

    // Unpause
    client.unpause();
    assert!(!client.is_paused());
}

#[test]
#[should_panic(expected = "Contract paused")]
fn test_cannot_set_price_when_paused() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);
    client.pause();

    // Try to set price while paused
    client.set_price(&MaterialType::Plastic, &10_000_000);
}

#[test]
fn test_all_material_types_have_prices() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Test that all material types can be queried
    let materials = [
        MaterialType::Plastic,
        MaterialType::Glass,
        MaterialType::Metal,
        MaterialType::Paper,
        MaterialType::Cardboard,
        MaterialType::Electronics,
        MaterialType::Organic,
        MaterialType::Textile,
        MaterialType::Rubber,
        MaterialType::Other,
    ];

    for material in materials.iter() {
        let price = client.get_price(material);
        assert!(price > 0, "Price should be set for all material types");
    }
}

#[test]
fn test_price_update_timestamp() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000);

    let admin = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Update price at different time
    env.ledger().with_mut(|li| li.timestamp = 2000);
    client.set_price(&MaterialType::Plastic, &11_000_000);

    let record = client.get_price_record(&MaterialType::Plastic);
    assert_eq!(record.last_updated, 2000);
}

#[test]
#[should_panic(expected = "Price not found")]
fn test_get_price_before_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_contract(&env);

    // Try to get price before initialization
    client.get_price(&MaterialType::Plastic);
}
