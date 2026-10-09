#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events as _},
    Address, Env, String, Symbol, TryFromVal,
};

fn create_token_contract<'a>(env: &Env) -> (Address, WasteTokenClient<'a>) {
    let contract_id = env.register_contract(None, WasteToken);
    let client = WasteTokenClient::new(env, &contract_id);
    (contract_id, client)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );

    assert_eq!(client.name(), String::from_str(&env, "WasteFi Token"));
    assert_eq!(client.symbol(), String::from_str(&env, "WASTE"));
    assert_eq!(client.decimals(), 7);
    assert_eq!(client.total_supply(), 0);
    assert_eq!(client.admin(), admin);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_cannot_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );

    // Try to initialize again - should panic
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );

    // Mint tokens
    client.mint(&user, &1_000_000);

    assert_eq!(client.balance(&user), 1_000_000);
    assert_eq!(client.total_supply(), 1_000_000);
}

#[test]
fn test_mint_multiple() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );

    // Mint to multiple users
    client.mint(&user1, &500_000);
    client.mint(&user2, &300_000);

    assert_eq!(client.balance(&user1), 500_000);
    assert_eq!(client.balance(&user2), 300_000);
    assert_eq!(client.total_supply(), 800_000);
}

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize and mint
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );
    client.mint(&user, &1_000_000);

    // Burn tokens
    client.burn(&user, &400_000);

    assert_eq!(client.balance(&user), 600_000);
    assert_eq!(client.total_supply(), 600_000);
}

#[test]
#[should_panic(expected = "Insufficient balance")]
fn test_burn_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize and mint
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );
    client.mint(&user, &100_000);

    // Try to burn more than balance - should panic
    client.burn(&user, &200_000);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize and mint
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );
    client.mint(&user1, &1_000_000);

    // Transfer tokens
    client.transfer(&user1, &user2, &300_000);

    assert_eq!(client.balance(&user1), 700_000);
    assert_eq!(client.balance(&user2), 300_000);
    assert_eq!(client.total_supply(), 1_000_000);
}

#[test]
#[should_panic(expected = "Insufficient balance")]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize and mint
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );
    client.mint(&user1, &100_000);

    // Try to transfer more than balance - should panic
    client.transfer(&user1, &user2, &200_000);
}

#[test]
fn test_pause_unpause() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );

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
fn test_cannot_mint_when_paused() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize and pause
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );
    client.pause();

    // Try to mint - should panic
    client.mint(&user, &1_000_000);
}

#[test]
fn test_zero_balance_default() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );

    // Check balance is 0 for address that never received tokens
    assert_eq!(client.balance(&user), 0);
}

#[test]
fn test_batch_burn_success() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize and mint to multiple users
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );
    client.mint(&user1, &1_000_000);
    client.mint(&user2, &500_000);
    client.mint(&user3, &800_000);

    // Create batch burn vector
    let mut burns = soroban_sdk::Vec::new(&env);
    burns.push_back((user1.clone(), 300_000i128));
    burns.push_back((user2.clone(), 200_000i128));
    burns.push_back((user3.clone(), 100_000i128));

    // Execute batch burn
    let burn_count = client.batch_burn(&burns);

    // Verify all burns succeeded
    assert_eq!(burn_count, 3);
    assert_eq!(client.balance(&user1), 700_000);
    assert_eq!(client.balance(&user2), 300_000);
    assert_eq!(client.balance(&user3), 700_000);
    assert_eq!(client.total_supply(), 1_700_000);
}

#[test]
fn test_batch_burn_partial_failure() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize and mint
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );
    client.mint(&user1, &1_000_000);
    client.mint(&user2, &200_000);
    client.mint(&user3, &800_000);

    // Create batch burn vector with one insufficient balance
    let mut burns = soroban_sdk::Vec::new(&env);
    burns.push_back((user1.clone(), 300_000i128));
    burns.push_back((user2.clone(), 500_000i128)); // This should fail - insufficient balance
    burns.push_back((user3.clone(), 100_000i128));

    // Execute batch burn
    let burn_count = client.batch_burn(&burns);

    // Verify only 2 burns succeeded
    assert_eq!(burn_count, 2);
    assert_eq!(client.balance(&user1), 700_000);
    assert_eq!(client.balance(&user2), 200_000); // Unchanged
    assert_eq!(client.balance(&user3), 700_000);
    assert_eq!(client.total_supply(), 1_600_000);
}

#[test]
fn test_batch_burn_empty_vector() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );

    // Create empty batch burn vector
    let burns = soroban_sdk::Vec::new(&env);

    // Execute batch burn
    let burn_count = client.batch_burn(&burns);

    // Verify no burns occurred
    assert_eq!(burn_count, 0);
}

#[test]
fn test_batch_burn_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let (_, client) = create_token_contract(&env);

    // Initialize and mint
    client.initialize(
        &admin,
        &String::from_str(&env, "WasteFi Token"),
        &String::from_str(&env, "WASTE"),
        &7,
    );
    client.mint(&user1, &1_000_000);
    client.mint(&user2, &500_000);

    // Create batch burn vector with invalid amount
    let mut burns = soroban_sdk::Vec::new(&env);
    burns.push_back((user1.clone(), 300_000i128));
    burns.push_back((user2.clone(), -100_000i128)); // Invalid negative amount

    // Execute batch burn
    let burn_count = client.batch_burn(&burns);

    // Verify only valid burn succeeded
    assert_eq!(burn_count, 1);
    assert_eq!(client.balance(&user1), 700_000);
    assert_eq!(client.balance(&user2), 500_000); // Unchanged
}

// ============================================================================
// Event emission
// ============================================================================

/// Number of contract events published with `topic` as their first topic.
fn count_events(env: &Env, topic: &str) -> usize {
    let wanted = Symbol::new(env, topic);
    env.events()
        .all()
        .iter()
        .filter(|(_, topics, _)| {
            topics
                .get(0)
                .and_then(|t| Symbol::try_from_val(env, &t).ok())
                .map_or(false, |t| t == wanted)
        })
        .count()
}

#[test]
fn test_mint_emits_one_event() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let user = Address::generate(&env);

    client.mint(&user, &500_000);

    assert_eq!(count_events(&env, "mint"), 1);
    assert_eq!(client.balance(&user), 500_000);
}

#[test]
fn test_burn_emits_one_event() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let user = Address::generate(&env);

    client.mint(&user, &1_000);
    client.burn(&user, &400);

    assert_eq!(count_events(&env, "burn"), 1);
    assert_eq!(client.balance(&user), 600);
}

#[test]
fn test_transfer_emits_one_event() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    client.mint(&from, &1_000);
    client.transfer(&from, &to, &300);

    assert_eq!(count_events(&env, "transfer"), 1);
    assert_eq!(client.balance(&to), 300);
}

#[test]
fn test_each_transfer_emits_its_own_event() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let from = Address::generate(&env);
    let to = Address::generate(&env);

    client.mint(&from, &1_000);
    // `events().all()` reports only the most recent invocation, so check after
    // every call that it published exactly one transfer event.
    for _ in 0..3 {
        client.transfer(&from, &to, &100);
        assert_eq!(count_events(&env, "transfer"), 1);
    }

    assert_eq!(client.balance(&to), 300);
}

#[test]
fn test_approve_emits_one_event() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);

    client.approve(&owner, &spender, &100);

    assert_eq!(count_events(&env, "approve"), 1);
}

#[test]
fn test_rejected_mint_emits_no_event() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let user = Address::generate(&env);

    assert!(client
        .try_mint(&user, &(client.get_max_supply() + 1))
        .is_err());

    assert_eq!(count_events(&env, "mint"), 0);
}

// ============================================================================
// Supply cap
// ============================================================================

fn setup<'a>(env: &Env) -> (Address, WasteTokenClient<'a>) {
    env.mock_all_auths();
    let admin = Address::generate(env);
    let (_, client) = create_token_contract(env);
    client.initialize(
        &admin,
        &String::from_str(env, "WasteFi Token"),
        &String::from_str(env, "WASTE"),
        &7,
    );
    (admin, client)
}

#[test]
fn test_total_supply_tracks_mints() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let user = Address::generate(&env);

    client.mint(&user, &1_000);
    client.mint(&user, &2_500);

    assert_eq!(client.total_supply(), 3_500);
    assert_eq!(client.balance(&user), 3_500);
}

#[test]
fn test_max_supply_getter() {
    let env = Env::default();
    let (_, client) = setup(&env);

    assert_eq!(client.get_max_supply(), 10_000_000_000_000_000);
    assert_eq!(client.get_remaining_supply(), 10_000_000_000_000_000);
}

#[test]
fn test_mint_up_to_exactly_the_cap() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let user = Address::generate(&env);

    client.mint(&user, &client.get_max_supply());

    assert_eq!(client.total_supply(), client.get_max_supply());
    assert_eq!(client.get_remaining_supply(), 0);
}

#[test]
#[should_panic(expected = "Minting would exceed maximum supply cap")]
fn test_mint_beyond_cap_is_rejected() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let user = Address::generate(&env);

    client.mint(&user, &client.get_max_supply());
    client.mint(&user, &1);
}

#[test]
#[should_panic(expected = "Minting would exceed maximum supply cap")]
fn test_single_mint_above_cap_is_rejected() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let user = Address::generate(&env);

    client.mint(&user, &(client.get_max_supply() + 1));
}

#[test]
fn test_rejected_mint_leaves_state_unchanged() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let user = Address::generate(&env);

    client.mint(&user, &1_000);
    let result = client.try_mint(&user, &client.get_max_supply());

    assert!(result.is_err());
    assert_eq!(client.total_supply(), 1_000);
    assert_eq!(client.balance(&user), 1_000);
}

#[test]
fn test_burn_frees_headroom_under_the_cap() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let user = Address::generate(&env);

    client.mint(&user, &client.get_max_supply());
    client.burn(&user, &500);

    assert_eq!(client.get_remaining_supply(), 500);
    client.mint(&user, &500);
    assert_eq!(client.get_remaining_supply(), 0);
}

// ============================================================================
// Allowances
// ============================================================================

#[test]
fn test_approve_sets_allowance() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);

    assert_eq!(client.allowance(&owner, &spender), 0);
    client.approve(&owner, &spender, &750);
    assert_eq!(client.allowance(&owner, &spender), 750);
}

#[test]
fn test_transfer_from_moves_tokens_and_spends_allowance() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.mint(&owner, &1_000);
    client.approve(&owner, &spender, &600);
    client.transfer_from(&spender, &owner, &recipient, &250);

    assert_eq!(client.balance(&owner), 750);
    assert_eq!(client.balance(&recipient), 250);
    assert_eq!(client.allowance(&owner, &spender), 350);
    // Moving tokens never changes supply.
    assert_eq!(client.total_supply(), 1_000);
}

#[test]
#[should_panic(expected = "Insufficient allowance")]
fn test_transfer_from_without_allowance_fails() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.mint(&owner, &1_000);
    client.transfer_from(&spender, &owner, &recipient, &1);
}

#[test]
#[should_panic(expected = "Insufficient allowance")]
fn test_transfer_from_over_allowance_fails() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.mint(&owner, &1_000);
    client.approve(&owner, &spender, &100);
    client.transfer_from(&spender, &owner, &recipient, &101);
}

#[test]
#[should_panic(expected = "Insufficient balance")]
fn test_transfer_from_over_balance_fails() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);

    client.mint(&owner, &50);
    client.approve(&owner, &spender, &1_000);
    client.transfer_from(&spender, &owner, &recipient, &51);
}

#[test]
#[should_panic(expected = "Invalid amount")]
fn test_approve_negative_amount_fails() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);

    client.approve(&owner, &spender, &-1);
}

#[test]
fn test_allowance_persists_across_calls_and_is_per_spender() {
    let env = Env::default();
    let (_, client) = setup(&env);
    let owner = Address::generate(&env);
    let spender_a = Address::generate(&env);
    let spender_b = Address::generate(&env);

    client.approve(&owner, &spender_a, &100);
    client.approve(&owner, &spender_b, &200);

    assert_eq!(client.allowance(&owner, &spender_a), 100);
    assert_eq!(client.allowance(&owner, &spender_b), 200);
}
