use collection_point::{CollectionPoint, CollectionPointClient};
use common::MaterialType;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn missing_owner_returns_none_without_changing_the_registry() {
    let env = Env::default();
    let id = env.register(CollectionPoint, ());
    let points = CollectionPointClient::new(&env, &id);
    points.initialize(&Address::generate(&env));
    env.set_auths(&[]);
    assert_eq!(points.find_point_by_owner(&Address::generate(&env)), None);
    assert_eq!(points.get_point_count(), 0);
}

#[test]
fn owner_lookup_is_public_and_preserves_the_existing_getter() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(CollectionPoint, ());
    let points = CollectionPointClient::new(&env, &id);
    points.initialize(&Address::generate(&env));
    let owner = Address::generate(&env);
    let point = points.register_point(
        &owner,
        &String::from_str(&env, "Test point"),
        &String::from_str(&env, "Test location"),
        &soroban_sdk::vec![&env, MaterialType::Plastic],
    );
    env.set_auths(&[]);
    assert_eq!(points.find_point_by_owner(&owner), Some(point));
    assert_eq!(points.get_point_by_owner(&owner), point);
    assert_eq!(points.find_point_by_owner(&Address::generate(&env)), None);
    assert_eq!(points.get_point_count(), 1);
}

#[test]
fn lookup_remains_available_while_the_registry_is_paused() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(CollectionPoint, ());
    let points = CollectionPointClient::new(&env, &id);
    points.initialize(&Address::generate(&env));
    let owner = Address::generate(&env);
    let point = points.register_point(
        &owner,
        &String::from_str(&env, "Test point"),
        &String::from_str(&env, "Test location"),
        &soroban_sdk::vec![&env, MaterialType::Plastic],
    );
    points.pause();
    env.set_auths(&[]);
    assert_eq!(points.find_point_by_owner(&owner), Some(point));
}
