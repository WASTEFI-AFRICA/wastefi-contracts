#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal, Symbol, Vec};

use common::types::*;

#[cfg(test)]
mod test;

const TOKEN_CONTRACT: &str = "TokenContract";
const PAYMENT_COUNT: &str = "PaymentCount";
const PAYMENT_FOR_TRANSACTION: &str = "PaymentForTx";
const TRANSACTION_CONTRACT: &str = "TransactionContract";

#[contract]
pub struct PaymentDistribution;

impl PaymentDistribution {
    /// Links a transaction to its payment, refusing a second payment for the same
    /// transaction.
    fn claim_transaction(env: &Env, transaction_id: u64, payment_id: u64) {
        let key = (PAYMENT_FOR_TRANSACTION, transaction_id);
        if env.storage().persistent().has(&key) {
            panic!("Payment already exists for this transaction");
        }
        env.storage().persistent().set(&key, &payment_id);
        common::bump_persistent(env, &key);
    }

    /// Stores a new pending payment and indexes it by recipient. The caller has
    /// already checked authorization and chosen `payment_id`.
    fn store_payment(
        env: &Env,
        payment_id: u64,
        transaction_id: u64,
        recipient: Address,
        amount: i128,
        created_at: u64,
    ) {
        Self::claim_transaction(env, transaction_id, payment_id);

        let payment = Payment {
            id: payment_id,
            recipient: recipient.clone(),
            amount,
            status: PaymentStatus::Pending,
            transaction_id,
            created_at,
            processed_at: 0,
        };
        let key = common::StorageKey::Payment(payment_id);
        env.storage().persistent().set(&key, &payment);
        common::bump_persistent(env, &key);

        let recipient_key = ("PaymentsByRecipient", recipient.clone());
        let mut recipient_payments: Vec<u64> = env
            .storage()
            .persistent()
            .get(&recipient_key)
            .unwrap_or(Vec::new(env));
        recipient_payments.push_back(payment_id);
        env.storage()
            .persistent()
            .set(&recipient_key, &recipient_payments);

        common::PaymentEvents::created(env, payment_id, recipient, amount);
    }

    /// Who is owed what for a transaction, read from the transaction contract that was
    /// bound at setup. The caller supplies nothing but the id, so a payment can never
    /// differ from the verified record.
    fn verified_payout(env: &Env, transaction_id: u64) -> (Address, i128) {
        let contract: Address = env
            .storage()
            .instance()
            .get(&TRANSACTION_CONTRACT)
            .expect("Transaction contract not set");
        // A missing transaction makes the callee panic, which fails this call too.
        let record: WasteRecord = env.invoke_contract(
            &contract,
            &Symbol::new(env, "get_transaction"),
            soroban_sdk::vec![env, transaction_id.into_val(env)],
        );
        if record.id != transaction_id {
            panic!("Transaction record does not match the requested id");
        }
        if !record.verified || record.status != TransactionStatus::Completed {
            panic!("Transaction is not verified");
        }
        if record.total_amount <= 0 {
            panic!("Amount must be positive");
        }
        (record.collector, record.total_amount)
    }

    fn require_admin_and_running(env: &Env) {
        common::Initializable::require_initialized(env).expect("Not initialized");
        common::Pausable::require_not_paused(env).expect("Contract paused");
        let admin = common::AccessControl::get_admin(env).expect("Admin not found");
        common::AccessControl::require_admin(env, &admin).expect("Not admin");
    }

    /// Mints the payment amount to the recipient through the token contract. This
    /// contract must be an approved minter there (`add_minter`).
    fn pay_out(env: &Env, payment: &Payment) {
        let token: Address = env
            .storage()
            .instance()
            .get(&TOKEN_CONTRACT)
            .expect("Token contract not set");
        env.invoke_contract::<()>(
            &token,
            &Symbol::new(env, "mint_from"),
            soroban_sdk::vec![
                env,
                env.current_contract_address().into_val(env),
                payment.recipient.clone().into_val(env),
                payment.amount.into_val(env),
            ],
        );
    }
}

#[contractimpl]
impl PaymentDistribution {
    /// Initialize the payment distribution contract
    ///
    /// # Arguments
    /// * `admin` - Contract administrator address
    /// * `token_contract` - WasteToken contract address for minting rewards
    pub fn initialize(env: Env, admin: Address, token_contract: Address) {
        common::Initializable::require_not_initialized(&env).expect("Already initialized");

        // Set admin
        common::AccessControl::set_admin(&env, admin);

        // Store token contract address
        env.storage()
            .instance()
            .set(&TOKEN_CONTRACT, &token_contract);

        // Initialize payment counter
        env.storage().instance().set(&PAYMENT_COUNT, &0u64);

        // Mark as initialized
        common::Initializable::mark_initialized(&env);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Bind the transaction contract that payments are derived from (admin only, once)
    ///
    /// # Arguments
    /// * `contract_address` - WasteTransaction contract whose verified records are paid
    ///
    /// # Note
    /// It can be set only once, so an admin cannot later point payments at a different
    /// transaction contract and pay for records that were never verified here.
    pub fn set_transaction_contract(env: Env, contract_address: Address) {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        if env.storage().instance().has(&TRANSACTION_CONTRACT) {
            panic!("Transaction contract already set");
        }
        env.storage()
            .instance()
            .set(&TRANSACTION_CONTRACT, &contract_address);
        common::bump_instance(&env);
    }

    /// The transaction contract payments are derived from, if one has been bound
    pub fn get_transaction_contract(env: Env) -> Option<Address> {
        env.storage().instance().get(&TRANSACTION_CONTRACT)
    }

    /// Record the payment owed for a verified transaction (admin only)
    ///
    /// # Arguments
    /// * `transaction_id` - Transaction ID from the bound WasteTransaction contract
    ///
    /// # Returns
    /// Payment ID
    ///
    /// # Note
    /// The recipient is the collector on the transaction record and the amount is its
    /// `total_amount`. Both are read on-chain; the caller cannot choose them. The
    /// transaction must be verified, and each transaction can be paid once.
    pub fn process_verified_payment(env: Env, transaction_id: u64) -> u64 {
        Self::require_admin_and_running(&env);

        let (recipient, amount) = Self::verified_payout(&env, transaction_id);

        let payment_count: u64 = env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0);
        let payment_id = payment_count + 1;
        env.storage().instance().set(&PAYMENT_COUNT, &payment_id);
        Self::store_payment(
            &env,
            payment_id,
            transaction_id,
            recipient,
            amount,
            env.ledger().timestamp(),
        );

        common::bump_instance(&env);
        payment_id
    }

    /// Record the payments owed for several verified transactions (admin only)
    ///
    /// # Arguments
    /// * `transaction_ids` - Transaction IDs from the bound WasteTransaction contract
    ///
    /// # Returns
    /// The new payment IDs, in order
    ///
    /// # Note
    /// Every payment is derived as in `process_verified_payment`. One transaction that
    /// is unverified, unknown, already paid, or repeated in the list aborts the batch.
    pub fn batch_process_verified(env: Env, transaction_ids: Vec<u64>) -> Vec<u64> {
        Self::require_admin_and_running(&env);

        let timestamp = env.ledger().timestamp();
        let mut payment_count: u64 = env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0);
        let mut payment_ids = Vec::new(&env);

        for transaction_id in transaction_ids.iter() {
            let (recipient, amount) = Self::verified_payout(&env, transaction_id);
            payment_count += 1;
            Self::store_payment(
                &env,
                payment_count,
                transaction_id,
                recipient,
                amount,
                timestamp,
            );
            payment_ids.push_back(payment_count);
        }

        env.storage().instance().set(&PAYMENT_COUNT, &payment_count);
        common::bump_instance(&env);
        payment_ids
    }

    /// Admin override: record a payment with a recipient and amount the admin supplies
    ///
    /// # Arguments
    /// * `transaction_id` - Transaction ID from WasteTransaction contract
    /// * `recipient` - Payment recipient address
    /// * `amount` - Payment amount in stroops
    ///
    /// # Returns
    /// Payment ID
    ///
    /// # Note
    /// This does not read or check the transaction, so it can pay any amount. It exists
    /// for corrections and for payments created before payouts were derived on-chain,
    /// and every use emits a `pay_ovr` event. Prefer `process_verified_payment`.
    pub fn process_payment(env: Env, transaction_id: u64, recipient: Address, amount: i128) -> u64 {
        Self::require_admin_and_running(&env);

        // Validate amount is positive
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let payment_count: u64 = env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0);
        let payment_id = payment_count + 1;
        env.storage().instance().set(&PAYMENT_COUNT, &payment_id);
        Self::store_payment(
            &env,
            payment_id,
            transaction_id,
            recipient,
            amount,
            env.ledger().timestamp(),
        );
        common::PaymentEvents::overridden(&env, payment_id, transaction_id, amount);

        common::bump_instance(&env);
        payment_id
    }

    /// Get payment details
    ///
    /// # Arguments
    /// * `payment_id` - Payment ID
    ///
    /// # Returns
    /// Payment record
    pub fn get_payment(env: Env, payment_id: u64) -> Payment {
        let key = common::StorageKey::Payment(payment_id);
        env.storage()
            .persistent()
            .get(&key)
            .expect("Payment not found")
    }

    /// Get payments by recipient
    ///
    /// # Arguments
    /// * `recipient` - Recipient address
    /// * `limit` - Maximum number of results
    ///
    /// # Returns
    /// Vector of payment records
    pub fn get_recipient_payments(env: Env, recipient: Address, limit: u32) -> Vec<Payment> {
        let max_limit = if limit == 0 || limit > 50 { 50 } else { limit };

        let recipient_key = ("PaymentsByRecipient", recipient);
        let payment_ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&recipient_key)
            .unwrap_or(Vec::new(&env));

        let mut payments = Vec::new(&env);

        // Get the most recent payments up to limit
        let start_idx = if payment_ids.len() > max_limit {
            payment_ids.len() - max_limit
        } else {
            0
        };

        for i in start_idx..payment_ids.len() {
            if let Some(payment_id) = payment_ids.get(i) {
                let key = common::StorageKey::Payment(payment_id);
                if let Some(payment) = env.storage().persistent().get(&key) {
                    payments.push_back(payment);
                }
            }
        }

        payments
    }

    /// Admin override: record several payments with supplied recipients and amounts
    ///
    /// # Arguments
    /// * `payments` - Vector of (transaction_id, recipient, amount) tuples
    ///
    /// # Returns
    /// Vector of payment IDs
    ///
    /// # Note
    /// Like `process_payment`, this does not read the transactions and emits a
    /// `pay_ovr` event for each payment. A transaction that already has a payment, or
    /// appears twice in the batch, aborts the whole batch.
    pub fn batch_process(env: Env, payments: Vec<(u64, Address, i128)>) -> Vec<u64> {
        Self::require_admin_and_running(&env);

        let timestamp = env.ledger().timestamp();
        let mut payment_count: u64 = env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0);
        let mut payment_ids = Vec::new(&env);

        for (tx_id, recipient, amount) in payments.iter() {
            if amount <= 0 {
                panic!("Amount must be positive");
            }
            payment_count += 1;
            Self::store_payment(&env, payment_count, tx_id, recipient, amount, timestamp);
            common::PaymentEvents::overridden(&env, payment_count, tx_id, amount);
            payment_ids.push_back(payment_count);
        }

        env.storage().instance().set(&PAYMENT_COUNT, &payment_count);
        common::bump_instance(&env);
        payment_ids
    }

    /// Update payment status (admin only)
    ///
    /// # Arguments
    /// * `payment_id` - Payment ID
    /// * `status` - New status
    ///
    /// # Note
    /// Setting `Completed` mints the payment amount to the recipient, so this contract
    /// must be an approved minter on the token contract.
    pub fn update_payment_status(env: Env, payment_id: u64, status: PaymentStatus) {
        common::Initializable::require_initialized(&env).expect("Not initialized");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        // Get payment
        let key = common::StorageKey::Payment(payment_id);
        let mut payment: Payment = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Payment not found");

        // CRITICAL: Prevent modification of completed or failed payments
        // This protects against double-payment scenarios
        if payment.status == PaymentStatus::Completed {
            panic!("Cannot modify completed payment");
        }
        if payment.status == PaymentStatus::Failed {
            panic!("Cannot modify failed payment");
        }

        // Update status. Completing a payment mints the tokens; if the mint fails the
        // whole call fails and the payment stays as it was.
        let status_clone = status.clone();
        payment.status = status;
        if status_clone == PaymentStatus::Completed {
            payment.processed_at = env.ledger().timestamp();
            Self::pay_out(&env, &payment);
        }

        // Save updated payment
        env.storage().persistent().set(&key, &payment);
        common::bump_persistent(&env, &key);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Get the payment recorded for a transaction, if any
    ///
    /// # Arguments
    /// * `transaction_id` - Transaction ID from WasteTransaction contract
    pub fn get_payment_for_transaction(env: Env, transaction_id: u64) -> Option<u64> {
        env.storage()
            .persistent()
            .get(&(PAYMENT_FOR_TRANSACTION, transaction_id))
    }

    /// Get total payment count
    ///
    /// # Returns
    /// Total number of payments
    pub fn get_payment_count(env: Env) -> u64 {
        env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0)
    }

    /// Get token contract address
    ///
    /// # Returns
    /// Token contract address
    pub fn get_token_contract(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&TOKEN_CONTRACT)
            .expect("Token contract not set")
    }

    /// Pause the contract (admin only)
    pub fn pause(env: Env) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::Pausable::admin_pause(&env, &admin).expect("Not admin");

        common::AdminEvents::paused(&env);
    }

    /// Unpause the contract (admin only)
    pub fn unpause(env: Env) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::Pausable::admin_unpause(&env, &admin).expect("Not admin");

        common::AdminEvents::unpaused(&env);
    }

    /// Check if contract is paused
    pub fn is_paused(env: Env) -> bool {
        common::Pausable::is_paused(&env)
    }

    /// Get admin address
    pub fn admin(env: Env) -> Address {
        common::AccessControl::get_admin(&env).expect("Admin not found")
    }

    /// Get payments by status (filtered query)
    ///
    /// # Arguments
    /// * `status` - Filter by payment status
    /// * `limit` - Maximum results (max 100)
    ///
    /// # Returns
    /// Vector of payments matching the status
    pub fn get_payments_by_status(env: Env, status: PaymentStatus, limit: u64) -> Vec<Payment> {
        let max_limit = if limit > 100 { 100 } else { limit };
        let mut results = Vec::new(&env);
        let total_count = env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0u64);

        for payment_id in 1..=total_count {
            if results.len() >= max_limit as u32 {
                break;
            }

            let key = common::StorageKey::Payment(payment_id);
            if let Some(payment) = env.storage().persistent().get::<_, Payment>(&key) {
                if payment.status == status {
                    results.push_back(payment);
                }
            }
        }

        results
    }

    /// Get recipient payment statistics
    ///
    /// # Arguments
    /// * `recipient` - Recipient address
    ///
    /// # Returns
    /// Tuple of (total_payments, total_amount, completed_count, pending_count)
    pub fn get_recipient_statistics(env: Env, recipient: Address) -> (u64, i128, u64, u64) {
        let recipient_key = ("PaymentsByRecipient", recipient);
        let payment_ids: Vec<u64> = env
            .storage()
            .persistent()
            .get(&recipient_key)
            .unwrap_or(Vec::new(&env));

        let mut total_amount = 0i128;
        let mut completed_count = 0u64;
        let mut pending_count = 0u64;

        for i in 0..payment_ids.len() {
            if let Some(payment_id) = payment_ids.get(i) {
                let key = common::StorageKey::Payment(payment_id);
                if let Some(payment) = env.storage().persistent().get::<_, Payment>(&key) {
                    total_amount = total_amount.saturating_add(payment.amount);

                    match payment.status {
                        PaymentStatus::Completed => completed_count += 1,
                        PaymentStatus::Pending => pending_count += 1,
                        _ => {}
                    }
                }
            }
        }

        let total_payments = payment_ids.len() as u64;
        (total_payments, total_amount, completed_count, pending_count)
    }

    /// Get global payment statistics
    ///
    /// # Returns
    /// Tuple of (total_payments, total_amount, completed_count, pending_count, failed_count)
    pub fn get_global_payment_statistics(env: Env) -> (u64, i128, u64, u64, u64) {
        let total_payments = env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0u64);
        let mut total_amount = 0i128;
        let mut completed_count = 0u64;
        let mut pending_count = 0u64;
        let mut failed_count = 0u64;

        for payment_id in 1..=total_payments {
            let key = common::StorageKey::Payment(payment_id);
            if let Some(payment) = env.storage().persistent().get::<_, Payment>(&key) {
                total_amount = total_amount.saturating_add(payment.amount);

                match payment.status {
                    PaymentStatus::Completed => completed_count += 1,
                    PaymentStatus::Pending => pending_count += 1,
                    PaymentStatus::Failed => failed_count += 1,
                    _ => {}
                }
            }
        }

        (
            total_payments,
            total_amount,
            completed_count,
            pending_count,
            failed_count,
        )
    }

    /// Get recent payments (last N payments)
    ///
    /// # Arguments
    /// * `limit` - Number of recent payments (max 50)
    ///
    /// # Returns
    /// Vector of most recent payments
    pub fn get_recent_payments(env: Env, limit: u64) -> Vec<Payment> {
        let max_limit = if limit > 50 { 50 } else { limit };
        let mut results = Vec::new(&env);
        let total_count = env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0u64);

        let start_id = if total_count > max_limit {
            total_count - max_limit + 1
        } else {
            1
        };

        for payment_id in start_id..=total_count {
            let key = common::StorageKey::Payment(payment_id);
            if let Some(payment) = env.storage().persistent().get(&key) {
                results.push_back(payment);
            }
        }

        results
    }

    /// Get payments in a time range
    ///
    /// # Arguments
    /// * `start_time` - Start timestamp
    /// * `end_time` - End timestamp
    /// * `limit` - Maximum results (max 100)
    ///
    /// # Returns
    /// Vector of payments created within the time range
    pub fn get_payments_by_time_range(
        env: Env,
        start_time: u64,
        end_time: u64,
        limit: u64,
    ) -> Vec<Payment> {
        let max_limit = if limit > 100 { 100 } else { limit };
        let mut results = Vec::new(&env);
        let total_count = env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0u64);

        for payment_id in 1..=total_count {
            if results.len() >= max_limit as u32 {
                break;
            }

            let key = common::StorageKey::Payment(payment_id);
            if let Some(payment) = env.storage().persistent().get::<_, Payment>(&key) {
                if payment.created_at >= start_time && payment.created_at <= end_time {
                    results.push_back(payment);
                }
            }
        }

        results
    }
}
