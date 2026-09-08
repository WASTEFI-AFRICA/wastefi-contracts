#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String, Vec};

fn create_collection_point_contract<'a>(env: &Env) -> (Address, CollectionPointClient<'a>) {
    let contract_id = env.register_contract(None, CollectionPoint);
    let client = CollectionPointClient::new(env, &contract_id);
    (contract_id, client)
}

fn create_materials_vec(env: &Env) -> Vec<common::MaterialType> {
    vec![
        env,
        common::MaterialType::Plastic,
        common::MaterialType::Glass,
        common::MaterialType::Metal,
    ]
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    assert_eq!(client.admin(), admin);
    assert_eq!(client.get_point_count(), 0);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_cannot_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    client.initialize(&admin); // Should panic
}

#[test]
fn test_register_point() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    let materials = create_materials_vec(&env);
    let point_id = client.register_point(
        &owner,
        &String::from_str(&env, "Downtown Station"),
        &String::from_str(&env, "123 Main Street"),
        &materials,
    );
    
    assert_eq!(point_id, 1);
    assert_eq!(client.get_point_count(), 1);
    
    let point = client.get_point(&point_id);
    assert_eq!(point.id, 1);
    assert_eq!(point.owner, owner);
    assert_eq!(point.name, String::from_str(&env, "Downtown Station"));
    assert_eq!(point.verification_status, common::VerificationStatus::Unverified);
}

#[test]
fn test_verify_point() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    let materials = create_materials_vec(&env);
    let point_id = client.register_point(
        &owner,
        &String::from_str(&env, "Downtown Station"),
        &String::from_str(&env, "123 Main Street"),
        &materials,
    );
    
    // Initially unverified
    assert!(!client.is_verified(&point_id));
    
    // Verify
    client.verify_point(&point_id);
    
    assert!(client.is_verified(&point_id));
    
    let point = client.get_point(&point_id);
    assert_eq!(point.verification_status, common::VerificationStatus::Verified);
}

#[test]
fn test_update_verification_status() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    let materials = create_materials_vec(&env);
    let point_id = client.register_point(
        &owner,
        &String::from_str(&env, "Downtown Station"),
        &String::from_str(&env, "123 Main Street"),
        &materials,
    );
    
    // Update to Pending
    client.update_verification_status(&point_id, &common::VerificationStatus::Pending);
    
    let point = client.get_point(&point_id);
    assert_eq!(point.verification_status, common::VerificationStatus::Pending);
}

#[test]
fn test_update_point() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    let materials = create_materials_vec(&env);
    let point_id = client.register_point(
        &owner,
        &String::from_str(&env, "Downtown Station"),
        &String::from_str(&env, "123 Main Street"),
        &materials,
    );
    
    // Update point
    let new_materials = vec![&env, common::MaterialType::Paper];
    client.update_point(
        &point_id,
        &String::from_str(&env, "Updated Station"),
        &String::from_str(&env, "456 New Street"),
        &new_materials,
    );
    
    let point = client.get_point(&point_id);
    assert_eq!(point.name, String::from_str(&env, "Updated Station"));
    assert_eq!(point.location, String::from_str(&env, "456 New Street"));
}

#[test]
fn test_update_processed() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    let materials = create_materials_vec(&env);
    let point_id = client.register_point(
        &owner,
        &String::from_str(&env, "Downtown Station"),
        &String::from_str(&env, "123 Main Street"),
        &materials,
    );
    
    // Initially 0
    let point = client.get_point(&point_id);
    assert_eq!(point.total_processed, 0);
    
    // Update processed
    client.update_processed(&point_id, &5000);
    
    let point = client.get_point(&point_id);
    assert_eq!(point.total_processed, 5000);
    
    // Update again
    client.update_processed(&point_id, &3000);
    
    let point = client.get_point(&point_id);
    assert_eq!(point.total_processed, 8000);
}

#[test]
fn test_accepts_material() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    let materials = vec![&env, common::MaterialType::Plastic, common::MaterialType::Glass];
    let point_id = client.register_point(
        &owner,
        &String::from_str(&env, "Downtown Station"),
        &String::from_str(&env, "123 Main Street"),
        &materials,
    );
    
    // Check accepted materials
    assert!(client.accepts_material(&point_id, &common::MaterialType::Plastic));
    assert!(client.accepts_material(&point_id, &common::MaterialType::Glass));
    assert!(!client.accepts_material(&point_id, &common::MaterialType::Paper));
}

#[test]
fn test_exists() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    assert!(!client.exists(&1));
    
    let materials = create_materials_vec(&env);
    let point_id = client.register_point(
        &owner,
        &String::from_str(&env, "Downtown Station"),
        &String::from_str(&env, "123 Main Street"),
        &materials,
    );
    
    assert!(client.exists(&point_id));
    assert!(!client.exists(&999));
}

#[test]
fn test_get_point_by_owner() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    let materials = create_materials_vec(&env);
    let point_id = client.register_point(
        &owner,
        &String::from_str(&env, "Downtown Station"),
        &String::from_str(&env, "123 Main Street"),
        &materials,
    );
    
    assert_eq!(client.get_point_by_owner(&owner), point_id);
}

#[test]
fn test_get_all_points() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    // Register multiple points
    let materials = create_materials_vec(&env);
    for i in 0..3 {
        let owner = Address::generate(&env);
        client.register_point(
            &owner,
            &String::from_str(&env, &format!("Station {}", i)),
            &String::from_str(&env, "Location"),
            &materials,
        );
    }
    
    let points = client.get_all_points(&1, &10);
    assert_eq!(points.len(), 3);
}

#[test]
fn test_get_verified_points() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let (_, client) = create_collection_point_contract(&env);
    
    client.initialize(&admin);
    
    // Register and verify some points
    let materials = create_materials_vec(&env);
    let point1 = client.register_point(
        &Address::generate(&env),
        &String::from_str(&env, "Station 1"),
        &String::from_str(&env, "Location"),
        &materials,
    );
    let point2 = client.register_point(
        &Address::generate(&env),
        &String::from_str(&env, "Station 2"),
        &String::from_str(&env, "Location"),
        &materials,
    );
    let _point3 = client.register_point(
        &Address::generate(&env),
        &String::from_str(&env, "Station 3"),
        &String::from_str(&env, "Location"),
        &materials,
    );
    
    // Verify only first two
    client.verify_point(&point1);
    client.verify_point(&point2);
    
    let verified = client.get_verified_points(&1, &10);
    assert_eq!(verified.len(), 2);
}
