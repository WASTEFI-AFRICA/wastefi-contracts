#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env,
};

fn create_contract(env: &Env) -> (Address, PaymentDistributionClient) {
    let contract_id = env.register_contract(None, PaymentDistribution);
    let client = PaymentDistributionClient::new(env, &contract_id);
    (contract_id, client)
}

/// Deploys a real WASTE token with the payment contract approved as a minter.
fn create_token<'a>(
    env: &Env,
    admin: &Address,
    payment_contract: &Address,
) -> waste_token::WasteTokenClient<'a> {
    let token_id = env.register_contract(None, waste_token::WasteToken);
    let token = waste_token::WasteTokenClient::new(env, &token_id);
    token.initialize(
        admin,
        &soroban_sdk::String::from_str(env, "WasteFi Token"),
        &soroban_sdk::String::from_str(env, "WASTE"),
        &7,
    );
    token.add_minter(payment_contract);
    token
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    assert_eq!(client.admin(), admin);
    assert_eq!(client.get_token_contract(), token_contract);
    assert_eq!(client.get_payment_count(), 0);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_cannot_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);
    client.initialize(&admin, &token_contract); // Should panic
}

#[test]
fn test_process_payment() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Process payment for transaction
    let payment_id = client.process_payment(&1, &recipient, &1_000);

    assert_eq!(payment_id, 1);
    assert_eq!(client.get_payment_count(), 1);

    // Check payment details
    let payment = client.get_payment(&payment_id);
    assert_eq!(payment.id, 1);
    assert_eq!(payment.transaction_id, 1);
    assert_eq!(payment.recipient, recipient);
    assert_eq!(payment.amount, 1_000);
    assert_eq!(payment.status, PaymentStatus::Pending);
    assert_eq!(payment.created_at, 1000);
}

#[test]
fn test_process_multiple_payments() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Process multiple payments
    let payment1 = client.process_payment(&1, &recipient, &1_000);
    let payment2 = client.process_payment(&2, &recipient, &1_000);
    let payment3 = client.process_payment(&3, &recipient, &1_000);

    assert_eq!(payment1, 1);
    assert_eq!(payment2, 2);
    assert_eq!(payment3, 3);
    assert_eq!(client.get_payment_count(), 3);
}

#[test]
fn test_update_payment_status() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000);

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (contract_id, client) = create_contract(&env);
    let token = create_token(&env, &admin, &contract_id);

    client.initialize(&admin, &token.address);

    let payment_id = client.process_payment(&1, &recipient, &1_000);

    // Update status to completed
    env.ledger().with_mut(|li| li.timestamp = 2000);
    client.update_payment_status(&payment_id, &PaymentStatus::Completed);

    let payment = client.get_payment(&payment_id);
    assert_eq!(payment.status, PaymentStatus::Completed);
    assert_eq!(payment.processed_at, 2000);
}

#[test]
fn test_batch_process() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);
    let recipient3 = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Batch process 3 payments with different recipients
    let mut payments = Vec::new(&env);
    payments.push_back((1u64, recipient1.clone(), 100_000i128));
    payments.push_back((2u64, recipient2.clone(), 200_000i128));
    payments.push_back((3u64, recipient3.clone(), 300_000i128));

    let payment_ids = client.batch_process(&payments);

    assert_eq!(payment_ids.len(), 3);
    assert_eq!(client.get_payment_count(), 3);

    // Verify all payments were created with correct data
    for i in 0..payment_ids.len() {
        if let Some(payment_id) = payment_ids.get(i) {
            let payment = client.get_payment(&payment_id);
            assert_eq!(payment.id, payment_id);
            assert_eq!(payment.status, PaymentStatus::Pending);
        }
    }

    // Verify each payment has correct recipient and amount
    let payment1 = client.get_payment(&1);
    assert_eq!(payment1.recipient, recipient1);
    assert_eq!(payment1.amount, 100_000);

    let payment2 = client.get_payment(&2);
    assert_eq!(payment2.recipient, recipient2);
    assert_eq!(payment2.amount, 200_000);

    let payment3 = client.get_payment(&3);
    assert_eq!(payment3.recipient, recipient3);
    assert_eq!(payment3.amount, 300_000);
}

#[test]
fn test_get_recipient_payments() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Process payments for one recipient
    client.process_payment(&1, &recipient, &1_000);
    client.process_payment(&2, &recipient, &1_000);
    client.process_payment(&3, &recipient, &1_000);

    // Get the recipient's payments
    let payments = client.get_recipient_payments(&recipient, &10);
    assert_eq!(payments.len(), 3);
}

#[test]
fn test_limit_recipient_payments() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Process 10 payments
    for i in 1..=10 {
        client.process_payment(&i, &recipient, &1_000);
    }

    // Request only 5
    let payments = client.get_recipient_payments(&recipient, &5);
    assert_eq!(payments.len(), 5);

    // Request all (0 means use default limit)
    let all_payments = client.get_recipient_payments(&recipient, &0);
    assert_eq!(all_payments.len(), 10);
}

#[test]
fn test_pause_unpause() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

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
fn test_cannot_process_when_paused() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);
    client.pause();

    // Try to process payment while paused
    client.process_payment(&1, &recipient, &1_000);
}

#[test]
fn test_payment_status_transitions() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (contract_id, client) = create_contract(&env);
    let token = create_token(&env, &admin, &contract_id);

    client.initialize(&admin, &token.address);

    let payment_id = client.process_payment(&1, &recipient, &1_000);

    // Check initial status
    let payment = client.get_payment(&payment_id);
    assert_eq!(payment.status, PaymentStatus::Pending);

    // Update to processing
    client.update_payment_status(&payment_id, &PaymentStatus::Processing);
    let payment = client.get_payment(&payment_id);
    assert_eq!(payment.status, PaymentStatus::Processing);

    // Update to completed
    client.update_payment_status(&payment_id, &PaymentStatus::Completed);
    let payment = client.get_payment(&payment_id);
    assert_eq!(payment.status, PaymentStatus::Completed);
}

#[test]
fn test_payment_status_failed() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    let payment_id = client.process_payment(&1, &recipient, &1_000);

    // Mark payment as failed
    client.update_payment_status(&payment_id, &PaymentStatus::Failed);

    let payment = client.get_payment(&payment_id);
    assert_eq!(payment.status, PaymentStatus::Failed);
}

#[test]
#[should_panic(expected = "Payment not found")]
fn test_get_nonexistent_payment() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Try to get payment that doesn't exist
    client.get_payment(&999);
}

#[test]
fn test_batch_process_gas_optimization() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Test batch processing with larger batch size
    let mut payments = Vec::new(&env);
    for i in 1..=10 {
        let recipient = Address::generate(&env);
        payments.push_back((i, recipient, 50_000i128 * i as i128));
    }

    // Record budget before batch operation
    env.budget().reset_unlimited();

    let payment_ids = client.batch_process(&payments);

    // Verify all payments processed correctly
    assert_eq!(payment_ids.len(), 10);
    assert_eq!(client.get_payment_count(), 10);

    // Verify payment data integrity
    for i in 0..payment_ids.len() {
        if let Some(payment_id) = payment_ids.get(i) {
            let payment = client.get_payment(&payment_id);
            assert_eq!(payment.status, PaymentStatus::Pending);
            assert_eq!(payment.transaction_id, payment_id);
        }
    }
}

#[test]
fn test_batch_vs_individual_processing() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);

    // Test individual processing
    let (_, client1) = create_contract(&env);
    client1.initialize(&admin, &token_contract);

    env.budget().reset_unlimited();

    // Process 5 payments individually
    for i in 1..=5 {
        let recipient = Address::generate(&env);
        client1.process_payment(&(i as u64), &recipient, &100_000i128);
    }

    // Test batch processing
    let (_, client2) = create_contract(&env);
    client2.initialize(&admin, &token_contract);

    env.budget().reset_unlimited();

    // Process 5 payments in batch
    let mut payments = Vec::new(&env);
    for i in 1..=5 {
        let recipient = Address::generate(&env);
        payments.push_back((i as u64, recipient, 100_000i128));
    }

    client2.batch_process(&payments);

    // Both should produce same result
    assert_eq!(client1.get_payment_count(), 5);
    assert_eq!(client2.get_payment_count(), 5);
}

#[test]
fn test_batch_process_empty_list() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Process empty batch
    let payments = Vec::new(&env);
    let payment_ids = client.batch_process(&payments);

    assert_eq!(payment_ids.len(), 0);
    assert_eq!(client.get_payment_count(), 0);
}

#[test]
fn test_batch_process_single_payment() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Process single payment via batch
    let mut payments = Vec::new(&env);
    payments.push_back((1u64, recipient.clone(), 100_000i128));

    let payment_ids = client.batch_process(&payments);

    assert_eq!(payment_ids.len(), 1);
    assert_eq!(client.get_payment_count(), 1);

    let payment = client.get_payment(&1);
    assert_eq!(payment.recipient, recipient);
    assert_eq!(payment.amount, 100_000);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_batch_process_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Try to batch process with negative amount
    let mut payments = Vec::new(&env);
    payments.push_back((1u64, recipient.clone(), -100_000i128));

    client.batch_process(&payments);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_batch_process_zero_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Try to batch process with zero amount
    let mut payments = Vec::new(&env);
    payments.push_back((1u64, recipient.clone(), 0i128));

    client.batch_process(&payments);
}

#[test]
fn test_batch_process_same_recipient_multiple_times() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Batch process multiple payments to same recipient
    let mut payments = Vec::new(&env);
    payments.push_back((1u64, recipient.clone(), 100_000i128));
    payments.push_back((2u64, recipient.clone(), 200_000i128));
    payments.push_back((3u64, recipient.clone(), 300_000i128));

    let payment_ids = client.batch_process(&payments);

    assert_eq!(payment_ids.len(), 3);
    assert_eq!(client.get_payment_count(), 3);

    // Verify recipient has all payments indexed
    let recipient_payments = client.get_recipient_payments(&recipient, &10);
    assert_eq!(recipient_payments.len(), 3);

    // Verify total statistics for recipient
    let (total_payments, total_amount, completed, pending) =
        client.get_recipient_statistics(&recipient);
    assert_eq!(total_payments, 3);
    assert_eq!(total_amount, 600_000);
    assert_eq!(pending, 3);
    assert_eq!(completed, 0);
}

#[test]
fn test_batch_process_payment_counter_consistency() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // First batch
    let mut payments1 = Vec::new(&env);
    for i in 1..=3 {
        let recipient = Address::generate(&env);
        payments1.push_back((i, recipient, 100_000i128));
    }
    let ids1 = client.batch_process(&payments1);

    // Second batch
    let mut payments2 = Vec::new(&env);
    for i in 4..=7 {
        let recipient = Address::generate(&env);
        payments2.push_back((i, recipient, 200_000i128));
    }
    let ids2 = client.batch_process(&payments2);

    // Verify payment IDs are sequential across batches
    assert_eq!(ids1.get(0).unwrap(), 1);
    assert_eq!(ids1.get(2).unwrap(), 3);
    assert_eq!(ids2.get(0).unwrap(), 4);
    assert_eq!(ids2.get(3).unwrap(), 7);

    assert_eq!(client.get_payment_count(), 7);
}

#[test]
fn test_batch_process_maintains_timestamp_consistency() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 5000);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let (_, client) = create_contract(&env);

    client.initialize(&admin, &token_contract);

    // Batch process multiple payments
    let mut payments = Vec::new(&env);
    for i in 1..=5 {
        let recipient = Address::generate(&env);
        payments.push_back((i, recipient, 100_000i128));
    }

    let payment_ids = client.batch_process(&payments);

    // All payments in batch should have same timestamp
    let first_payment = client.get_payment(&payment_ids.get(0).unwrap());
    for i in 1..payment_ids.len() {
        let payment = client.get_payment(&payment_ids.get(i).unwrap());
        assert_eq!(payment.created_at, first_payment.created_at);
        assert_eq!(payment.created_at, 5000);
    }
}

#[test]
fn test_completing_a_payment_mints_to_the_recipient() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (contract_id, client) = create_contract(&env);
    let token = create_token(&env, &admin, &contract_id);
    client.initialize(&admin, &token.address);

    let payment_id = client.process_payment(&7, &recipient, &16_000_000);
    assert_eq!(token.balance(&recipient), 0);

    client.update_payment_status(&payment_id, &PaymentStatus::Completed);
    assert_eq!(token.balance(&recipient), 16_000_000);
    assert_eq!(token.total_supply(), 16_000_000);
}

#[test]
fn test_completion_fails_if_not_a_minter() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (contract_id, client) = create_contract(&env);
    let token = create_token(&env, &admin, &contract_id);
    token.remove_minter(&contract_id);
    client.initialize(&admin, &token.address);

    let payment_id = client.process_payment(&7, &recipient, &1_000);
    let result = client.try_update_payment_status(&payment_id, &PaymentStatus::Completed);

    assert!(result.is_err());
    assert_eq!(
        client.get_payment(&payment_id).status,
        PaymentStatus::Pending
    );
    assert_eq!(token.balance(&recipient), 0);
}

#[test]
#[should_panic(expected = "Payment already exists for this transaction")]
fn test_transaction_cannot_be_paid_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);
    client.initialize(&admin, &token_contract);

    client.process_payment(&1, &recipient, &1_000);
    client.process_payment(&1, &recipient, &1_000);
}

#[test]
fn test_payment_is_indexed_by_transaction() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);
    client.initialize(&admin, &token_contract);

    let payment_id = client.process_payment(&42, &recipient, &1_000);
    assert_eq!(client.get_payment_for_transaction(&42), Some(payment_id));
    assert_eq!(client.get_payment_for_transaction(&43), None);
}

#[test]
#[should_panic(expected = "Payment already exists for this transaction")]
fn test_batch_rejects_an_already_paid_transaction() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);
    client.initialize(&admin, &token_contract);

    client.process_payment(&1, &recipient, &1_000);

    let mut batch = Vec::new(&env);
    batch.push_back((2u64, recipient.clone(), 1_000i128));
    batch.push_back((1u64, recipient.clone(), 1_000i128));
    client.batch_process(&batch);
}

#[test]
#[should_panic(expected = "Payment already exists for this transaction")]
fn test_batch_rejects_a_transaction_repeated_within_the_batch() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (_, client) = create_contract(&env);
    client.initialize(&admin, &token_contract);

    let mut batch = Vec::new(&env);
    batch.push_back((5u64, recipient.clone(), 1_000i128));
    batch.push_back((5u64, recipient.clone(), 1_000i128));
    client.batch_process(&batch);
}

// ---------------------------------------------------------------------------
// Payouts derived from the verified transaction
// ---------------------------------------------------------------------------

use common::MaterialType;
use waste_transaction::{WasteTransaction, WasteTransactionClient};

struct Derived {
    env: Env,
    collector: Address,
    payments_id: Address,
    transactions_id: Address,
    token_id: Address,
}

impl Derived {
    /// A payment contract bound to a real transaction contract, with minting approved.
    fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().with_mut(|li| li.timestamp = 1_000);

        let admin = Address::generate(&env);
        let collector = Address::generate(&env);

        let transactions_id = env.register_contract(None, WasteTransaction);
        WasteTransactionClient::new(&env, &transactions_id).initialize(&admin);

        let (payments_id, payments) = create_contract(&env);
        let token = create_token(&env, &admin, &payments_id);
        payments.initialize(&admin, &token.address);
        payments.set_transaction_contract(&transactions_id);

        Self {
            collector,
            payments_id,
            transactions_id,
            token_id: token.address,
            env,
        }
    }

    fn payments(&self) -> PaymentDistributionClient<'_> {
        PaymentDistributionClient::new(&self.env, &self.payments_id)
    }

    fn transactions(&self) -> WasteTransactionClient<'_> {
        WasteTransactionClient::new(&self.env, &self.transactions_id)
    }

    fn token(&self) -> waste_token::WasteTokenClient<'_> {
        waste_token::WasteTokenClient::new(&self.env, &self.token_id)
    }

    /// Record a delivery; `weight` is in grams.
    fn record(&self, weight: u64, price_per_kg: i128) -> u64 {
        let point = Address::generate(&self.env);
        self.transactions().record_collection(
            &self.collector,
            &point,
            &MaterialType::Plastic,
            &weight,
            &price_per_kg,
        )
    }

    fn verified(&self, weight: u64, price_per_kg: i128) -> u64 {
        let id = self.record(weight, price_per_kg);
        self.transactions().verify_transaction(&id);
        id
    }
}

#[test]
fn test_payment_is_derived_from_the_verified_transaction() {
    let f = Derived::new();
    let tx_id = f.verified(2_500, 8_000_000); // 2.5 kg at 0.8 per kg

    let payment_id = f.payments().process_verified_payment(&tx_id);
    let payment = f.payments().get_payment(&payment_id);
    assert_eq!(payment.recipient, f.collector);
    assert_eq!(payment.amount, 20_000_000);
    assert_eq!(payment.transaction_id, tx_id);
    assert_eq!(payment.status, PaymentStatus::Pending);
    assert_eq!(
        f.payments().get_payment_for_transaction(&tx_id),
        Some(payment_id)
    );

    f.payments()
        .update_payment_status(&payment_id, &PaymentStatus::Completed);
    assert_eq!(f.token().balance(&f.collector), 20_000_000);
}

#[test]
fn test_derived_payment_follows_the_price_on_the_record() {
    let f = Derived::new();
    // Different weights: the transaction contract rejects an identical repeat.
    let cheap = f.verified(1_000, 4_000_000);
    let dear = f.verified(2_000, 25_000_000);

    let a = f.payments().process_verified_payment(&cheap);
    let b = f.payments().process_verified_payment(&dear);

    assert_eq!(f.payments().get_payment(&a).amount, 4_000_000);
    assert_eq!(f.payments().get_payment(&b).amount, 50_000_000);
}

#[test]
#[should_panic(expected = "Transaction is not verified")]
fn test_unverified_transaction_cannot_be_paid() {
    let f = Derived::new();
    let tx_id = f.record(2_000, 8_000_000);
    f.payments().process_verified_payment(&tx_id);
}

#[test]
#[should_panic(expected = "Transaction is not verified")]
fn test_disputed_transaction_cannot_be_paid() {
    let f = Derived::new();
    let tx_id = f.verified(2_000, 8_000_000);
    f.transactions()
        .update_status(&tx_id, &common::TransactionStatus::Disputed);
    f.payments().process_verified_payment(&tx_id);
}

#[test]
#[should_panic]
fn test_unknown_transaction_cannot_be_paid() {
    let f = Derived::new();
    f.payments().process_verified_payment(&999);
}

#[test]
#[should_panic(expected = "Payment already exists for this transaction")]
fn test_derived_payment_is_made_once() {
    let f = Derived::new();
    let tx_id = f.verified(2_000, 8_000_000);
    f.payments().process_verified_payment(&tx_id);
    f.payments().process_verified_payment(&tx_id);
}

#[test]
#[should_panic(expected = "Payment already exists for this transaction")]
fn test_override_cannot_pay_a_transaction_that_was_already_paid() {
    let f = Derived::new();
    let tx_id = f.verified(2_000, 8_000_000);
    f.payments().process_verified_payment(&tx_id);
    f.payments().process_payment(&tx_id, &f.collector, &1_000);
}

#[test]
#[should_panic(expected = "Transaction contract not set")]
fn test_derived_payment_needs_a_bound_transaction_contract() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let (_, client) = create_contract(&env);
    client.initialize(&admin, &token);

    client.process_verified_payment(&1);
}

#[test]
fn test_transaction_contract_is_set_once() {
    let f = Derived::new();
    assert_eq!(
        f.payments().get_transaction_contract(),
        Some(f.transactions_id.clone())
    );
    let other = Address::generate(&f.env);
    assert!(f.payments().try_set_transaction_contract(&other).is_err());
    assert_eq!(
        f.payments().get_transaction_contract(),
        Some(f.transactions_id.clone())
    );
}

#[test]
fn test_only_the_admin_binds_the_transaction_contract() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let (_, client) = create_contract(&env);
    client.initialize(&admin, &token);

    env.set_auths(&[]);
    let result = client.try_set_transaction_contract(&Address::generate(&env));
    assert!(result.is_err());
    assert_eq!(client.get_transaction_contract(), None);
}

#[test]
fn test_only_the_admin_can_trigger_a_derived_payment() {
    let f = Derived::new();
    let tx_id = f.verified(2_000, 8_000_000);

    f.env.set_auths(&[]);
    assert!(f.payments().try_process_verified_payment(&tx_id).is_err());
    assert!(f
        .payments()
        .try_batch_process_verified(&soroban_sdk::vec![&f.env, tx_id])
        .is_err());
    assert_eq!(f.payments().get_payment_for_transaction(&tx_id), None);
}

#[test]
#[should_panic(expected = "Contract paused")]
fn test_paused_contract_makes_no_derived_payment() {
    let f = Derived::new();
    let tx_id = f.verified(2_000, 8_000_000);
    f.payments().pause();
    f.payments().process_verified_payment(&tx_id);
}

#[test]
fn test_batch_derives_every_payment() {
    let f = Derived::new();
    let first = f.verified(1_000, 8_000_000);
    let second = f.verified(2_000, 8_000_000);

    let ids = f
        .payments()
        .batch_process_verified(&soroban_sdk::vec![&f.env, first, second]);

    assert_eq!(ids.len(), 2);
    assert_eq!(
        f.payments().get_payment(&ids.get(0).unwrap()).amount,
        8_000_000
    );
    assert_eq!(
        f.payments().get_payment(&ids.get(1).unwrap()).amount,
        16_000_000
    );
    assert_eq!(f.payments().get_payment_count(), 2);
}

#[test]
fn test_batch_with_one_unverified_transaction_pays_nothing() {
    let f = Derived::new();
    let good = f.verified(1_000, 8_000_000);
    let pending = f.record(2_000, 8_000_000);

    let result = f
        .payments()
        .try_batch_process_verified(&soroban_sdk::vec![&f.env, good, pending]);

    assert!(result.is_err());
    assert_eq!(f.payments().get_payment_for_transaction(&good), None);
    assert_eq!(f.payments().get_payment_count(), 0);
}

#[test]
fn test_batch_rejects_a_transaction_listed_twice() {
    let f = Derived::new();
    let tx_id = f.verified(1_000, 8_000_000);

    let result = f
        .payments()
        .try_batch_process_verified(&soroban_sdk::vec![&f.env, tx_id, tx_id]);

    assert!(result.is_err());
    assert_eq!(f.payments().get_payment_count(), 0);
}

#[test]
fn test_overrides_are_logged_and_derived_payments_are_not() {
    use soroban_sdk::testutils::Events;

    let f = Derived::new();
    let derived = f.verified(1_000, 8_000_000);
    let corrected = f.record(1_100, 8_000_000);
    let first_batched = f.record(1_200, 8_000_000);
    let second_batched = f.record(1_300, 8_000_000);

    // The test host reports the events of the latest invocation, so check each call.
    let override_events = |f: &Derived| {
        let topic = soroban_sdk::symbol_short!("pay_ovr");
        f.env
            .events()
            .all()
            .iter()
            .filter(|(contract, topics, _)| {
                *contract == f.payments_id
                    && topics == &soroban_sdk::vec![&f.env, topic.into_val(&f.env)]
            })
            .count()
    };

    f.payments().process_verified_payment(&derived);
    assert_eq!(override_events(&f), 0, "derived payments are not overrides");

    f.payments().process_payment(&corrected, &f.collector, &123);
    assert_eq!(override_events(&f), 1);

    f.payments().batch_process(&soroban_sdk::vec![
        &f.env,
        (first_batched, f.collector.clone(), 456i128),
        (second_batched, f.collector.clone(), 789i128),
    ]);
    assert_eq!(override_events(&f), 2, "one event per overridden payment");
}
