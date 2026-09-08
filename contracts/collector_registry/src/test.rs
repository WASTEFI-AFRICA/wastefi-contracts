#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn create_registry_contract<'a>(env: &Env) -> (Address, CollectorRegistryClient<'a>) {
    let contract_id = env.register_contract(None, CollectorRegistry);
    let client = CollectorRegistryClient::new(env, &contract_id);
    (contract_id, client)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);

    assert_eq!(client.admin(), admin);
    assert_eq!(client.get_collector_count(), 0);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_cannot_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);
    client.initialize(&admin); // Should panic
}

#[test]
fn test_register_collector() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);

    client.register(
        &collector,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "+1234567890"),
    );

    let collector_data = client.get_collector(&collector);
    assert_eq!(collector_data.address, collector);
    assert_eq!(collector_data.name, String::from_str(&env, "John Doe"));
    assert_eq!(collector_data.status, common::CollectorStatus::Pending);
    assert_eq!(collector_data.reputation_score, 500);
    assert_eq!(client.get_collector_count(), 1);
}

#[test]
#[should_panic(expected = "Collector already registered")]
fn test_cannot_register_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);

    client.register(
        &collector,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "+1234567890"),
    );

    // Try to register again - should panic
    client.register(
        &collector,
        &String::from_str(&env, "Jane Doe"),
        &String::from_str(&env, "+0987654321"),
    );
}

#[test]
fn test_update_status() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);
    client.register(
        &collector,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "+1234567890"),
    );

    // Update to Active
    client.update_status(&collector, &common::CollectorStatus::Active);

    let collector_data = client.get_collector(&collector);
    assert_eq!(collector_data.status, common::CollectorStatus::Active);
}

#[test]
fn test_update_profile() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);
    client.register(
        &collector,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "+1234567890"),
    );

    // Update profile
    client.update_profile(
        &collector,
        &String::from_str(&env, "John Smith"),
        &String::from_str(&env, "+1111111111"),
    );

    let collector_data = client.get_collector(&collector);
    assert_eq!(collector_data.name, String::from_str(&env, "John Smith"));
    assert_eq!(collector_data.phone, String::from_str(&env, "+1111111111"));
}

#[test]
fn test_update_metrics() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);
    client.register(
        &collector,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "+1234567890"),
    );

    // Update metrics
    client.update_metrics(&collector, &5000);

    let collector_data = client.get_collector(&collector);
    assert_eq!(collector_data.total_collections, 1);
    assert_eq!(collector_data.total_weight, 5000);

    // Update again
    client.update_metrics(&collector, &3000);

    let collector_data = client.get_collector(&collector);
    assert_eq!(collector_data.total_collections, 2);
    assert_eq!(collector_data.total_weight, 8000);
}

#[test]
fn test_update_reputation() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);
    client.register(
        &collector,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "+1234567890"),
    );

    // Update reputation
    client.update_reputation(&collector, &750);

    let collector_data = client.get_collector(&collector);
    assert_eq!(collector_data.reputation_score, 750);
}

#[test]
fn test_is_active() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);
    client.register(
        &collector,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "+1234567890"),
    );

    // Initially not active (Pending)
    assert!(!client.is_active(&collector));

    // Update to Active
    client.update_status(&collector, &common::CollectorStatus::Active);
    assert!(client.is_active(&collector));
}

#[test]
fn test_exists() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let non_collector = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);

    assert!(!client.exists(&collector));

    client.register(
        &collector,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "+1234567890"),
    );

    assert!(client.exists(&collector));
    assert!(!client.exists(&non_collector));
}

#[test]
fn test_get_all_collectors() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector1 = Address::generate(&env);
    let collector2 = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);

    client.register(
        &collector1,
        &String::from_str(&env, "John Doe"),
        &String::from_str(&env, "+1234567890"),
    );
    client.register(
        &collector2,
        &String::from_str(&env, "Jane Doe"),
        &String::from_str(&env, "+0987654321"),
    );

    let collectors = client.get_all_collectors(&0, &10);
    assert_eq!(collectors.len(), 2);
}

#[test]
fn test_multiple_collectors() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_registry_contract(&env);

    client.initialize(&admin);

    // Register multiple collectors
    for i in 0..5 {
        let collector = Address::generate(&env);
        client.register(
            &collector,
            &String::from_str(&env, &format!("Collector {}", i)),
            &String::from_str(&env, "+1234567890"),
        );
    }

    assert_eq!(client.get_collector_count(), 5);
}
