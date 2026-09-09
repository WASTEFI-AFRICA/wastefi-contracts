#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env,
};

fn create_contract(env: &Env) -> (Address, ReputationClient) {
    let contract_id = env.register_contract(None, Reputation);
    let client = ReputationClient::new(env, &contract_id);
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
fn test_default_score_for_new_collector() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // New collector should have default neutral score
    let score = client.get_score(&collector);
    assert_eq!(score.score, 500);
    assert_eq!(score.total_transactions, 0);
    assert_eq!(score.successful_transactions, 0);
    assert_eq!(score.disputed_transactions, 0);
}

#[test]
fn test_update_score_successful_transaction() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000);

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Update with successful transaction
    client.update_score(&collector, &true);

    let score = client.get_score(&collector);
    assert_eq!(score.total_transactions, 1);
    assert_eq!(score.successful_transactions, 1);
    assert_eq!(score.disputed_transactions, 0);
    assert_eq!(score.score, 505); // 500 + (1 * 5)
    assert_eq!(score.last_updated, 1000);
}

#[test]
fn test_update_score_disputed_transaction() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Update with disputed transaction
    client.update_score(&collector, &false);

    let score = client.get_score(&collector);
    assert_eq!(score.total_transactions, 1);
    assert_eq!(score.successful_transactions, 0);
    assert_eq!(score.disputed_transactions, 1);
    assert_eq!(score.score, 490); // 500 - (1 * 10)
}

#[test]
fn test_multiple_successful_transactions() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // 10 successful transactions
    for _ in 0..10 {
        client.update_score(&collector, &true);
    }

    let score = client.get_score(&collector);
    assert_eq!(score.total_transactions, 10);
    assert_eq!(score.successful_transactions, 10);
    assert_eq!(score.disputed_transactions, 0);
    // 500 + (10 * 5) + 100 (success rate bonus) = 650
    assert_eq!(score.score, 650);
}

#[test]
fn test_mixed_transactions() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // 8 successful, 2 disputed
    for _ in 0..8 {
        client.update_score(&collector, &true);
    }
    for _ in 0..2 {
        client.update_score(&collector, &false);
    }

    let score = client.get_score(&collector);
    assert_eq!(score.total_transactions, 10);
    assert_eq!(score.successful_transactions, 8);
    assert_eq!(score.disputed_transactions, 2);
    // 500 + (8 * 5) - (2 * 10) + 80 (success rate bonus for 80%) = 600
    assert_eq!(score.score, 600);
}

#[test]
fn test_calculate_score() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Add some transactions
    for _ in 0..5 {
        client.update_score(&collector, &true);
    }

    let calculated = client.calculate_score(&collector);
    let stored = client.get_score(&collector);

    assert_eq!(calculated, stored.score);
}

#[test]
fn test_set_score_manually() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Manually set score
    client.set_score(&collector, &750);

    let score = client.get_score(&collector);
    assert_eq!(score.score, 750);
}

#[test]
#[should_panic(expected = "Invalid score")]
fn test_set_score_invalid() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Try to set invalid score (> 1000)
    client.set_score(&collector, &1500);
}

#[test]
fn test_reset_score() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Build up some reputation
    for _ in 0..5 {
        client.update_score(&collector, &true);
    }

    let score_before = client.get_score(&collector);
    assert!(score_before.score > 500);

    // Reset
    client.reset_score(&collector);

    let score_after = client.get_score(&collector);
    assert_eq!(score_after.score, 500);
    assert_eq!(score_after.total_transactions, 0);
    assert_eq!(score_after.successful_transactions, 0);
    assert_eq!(score_after.disputed_transactions, 0);
}

#[test]
fn test_score_clamped_at_maximum() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Many successful transactions to exceed 1000
    for _ in 0..200 {
        client.update_score(&collector, &true);
    }

    let score = client.get_score(&collector);
    assert_eq!(score.score, 1000); // Clamped at max
}

#[test]
fn test_score_clamped_at_minimum() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // Many disputed transactions to go below 0
    for _ in 0..100 {
        client.update_score(&collector, &false);
    }

    let score = client.get_score(&collector);
    assert_eq!(score.score, 0); // Clamped at min
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
fn test_cannot_update_when_paused() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);
    client.pause();

    // Try to update score while paused
    client.update_score(&collector, &true);
}

#[test]
fn test_success_rate_bonus_threshold() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let collector = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin);

    // 9 transactions (below 10 threshold) - no bonus
    for _ in 0..9 {
        client.update_score(&collector, &true);
    }

    let score_before = client.get_score(&collector);
    // 500 + (9 * 5) = 545 (no bonus yet)
    assert_eq!(score_before.score, 545);

    // 10th transaction - should now get bonus
    client.update_score(&collector, &true);

    let score_after = client.get_score(&collector);
    // 500 + (10 * 5) + 100 (success rate bonus) = 650
    assert_eq!(score_after.score, 650);
}
