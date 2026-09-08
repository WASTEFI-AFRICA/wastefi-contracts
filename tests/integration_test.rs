// Integration tests for WasteFi contracts
#![cfg(test)]

use soroban_sdk::{Env, testutils::Address as _};

#[test]
fn test_workspace_setup() {
    // Verify test environment works
    let env = Env::default();
    assert!(env.ledger().timestamp() > 0);
}

#[test]
fn test_contract_addresses() {
    // Test that we can generate addresses
    let env = Env::default();
    let addr1 = soroban_sdk::Address::generate(&env);
    let addr2 = soroban_sdk::Address::generate(&env);
    
    assert_ne!(addr1, addr2);
}

#[test]
fn test_time_travel() {
    // Test ledger time manipulation
    let env = Env::default();
    let initial_time = env.ledger().timestamp();
    
    env.ledger().with_mut(|li| {
        li.timestamp += 1000;
    });
    
    let new_time = env.ledger().timestamp();
    assert_eq!(new_time, initial_time + 1000);
}
