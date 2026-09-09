#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

use common::types::*;

#[cfg(test)]
mod test;

const TOKEN_CONTRACT: &str = "TokenContract";
const PAYMENT_COUNT: &str = "PaymentCount";

#[contract]
pub struct PaymentDistribution;

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

    /// Process payment for a completed transaction
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
    /// This is a simplified version. Full implementation would call WasteTransaction
    /// contract to fetch transaction details and validate completion status.
    pub fn process_payment(env: Env, transaction_id: u64, recipient: Address, amount: i128) -> u64 {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        // Validate amount is positive
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Get next payment ID
        let payment_count: u64 = env.storage().instance().get(&PAYMENT_COUNT).unwrap_or(0);
        let payment_id = payment_count + 1;
        env.storage().instance().set(&PAYMENT_COUNT, &payment_id);

        // Create payment record
        let payment = Payment {
            id: payment_id,
            recipient: recipient.clone(),
            amount,
            status: PaymentStatus::Pending,
            transaction_id,
            created_at: env.ledger().timestamp(),
            processed_at: 0,
        };

        // Store payment
        let key = common::StorageKey::Payment(payment_id);
        env.storage().persistent().set(&key, &payment);
        common::bump_persistent(&env, &key);

        // Index by recipient
        let recipient_key = ("PaymentsByRecipient", recipient.clone());
        let mut recipient_payments: Vec<u64> = env
            .storage()
            .persistent()
            .get(&recipient_key)
            .unwrap_or(Vec::new(&env));
        recipient_payments.push_back(payment_id);
        env.storage()
            .persistent()
            .set(&recipient_key, &recipient_payments);

        // Emit event
        common::PaymentEvents::created(&env, payment_id, recipient, amount);

        // Bump storage
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

    /// Batch process payments for multiple transactions
    ///
    /// # Arguments
    /// * `payments` - Vector of (transaction_id, recipient, amount) tuples
    ///
    /// # Returns
    /// Vector of payment IDs
    pub fn batch_process(env: Env, payments: Vec<(u64, Address, i128)>) -> Vec<u64> {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        let mut payment_ids = Vec::new(&env);

        for i in 0..payments.len() {
            if let Some((tx_id, recipient, amount)) = payments.get(i) {
                let payment_id = Self::process_payment(env.clone(), tx_id, recipient, amount);
                payment_ids.push_back(payment_id);
            }
        }

        payment_ids
    }

    /// Update payment status (admin only)
    ///
    /// # Arguments
    /// * `payment_id` - Payment ID
    /// * `status` - New status
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

        // Update status
        let status_clone = status.clone();
        payment.status = status;
        if status_clone == PaymentStatus::Completed {
            payment.processed_at = env.ledger().timestamp();
        }

        // Save updated payment
        env.storage().persistent().set(&key, &payment);
        common::bump_persistent(&env, &key);

        // Bump storage
        common::bump_instance(&env);
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
}
