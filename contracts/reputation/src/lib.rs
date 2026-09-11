#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env};

use common::types::*;

#[cfg(test)]
mod test;

#[contract]
pub struct Reputation;

#[contractimpl]
impl Reputation {
    /// Initialize the reputation contract
    ///
    /// # Arguments
    /// * `admin` - Contract administrator address
    pub fn initialize(env: Env, admin: Address) {
        common::Initializable::require_not_initialized(&env).expect("Already initialized");

        // Set admin
        common::AccessControl::set_admin(&env, admin);

        // Mark as initialized
        common::Initializable::mark_initialized(&env);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Update reputation score for a collector after a transaction
    ///
    /// # Arguments
    /// * `collector` - Collector address
    /// * `transaction_successful` - Whether the transaction was successful
    pub fn update_score(env: Env, collector: Address, transaction_successful: bool) {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        // Get or create reputation score
        let key = common::StorageKey::Reputation(collector.clone());
        let mut score: ReputationScore =
            env.storage()
                .persistent()
                .get(&key)
                .unwrap_or(ReputationScore {
                    collector: collector.clone(),
                    score: 500, // Start with neutral score
                    total_transactions: 0,
                    successful_transactions: 0,
                    disputed_transactions: 0,
                    last_updated: env.ledger().timestamp(),
                });

        // Update transaction counts
        score.total_transactions += 1;
        if transaction_successful {
            score.successful_transactions += 1;
        } else {
            score.disputed_transactions += 1;
        }

        // Calculate new score
        let old_score = score.score;
        score.score = Self::calculate_score_internal(&score);
        score.last_updated = env.ledger().timestamp();

        // Save updated score
        env.storage().persistent().set(&key, &score);
        common::bump_persistent(&env, &key);

        // Emit event
        common::ReputationEvents::score_updated(&env, collector, old_score, score.score);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Get reputation score for a collector
    ///
    /// # Arguments
    /// * `collector` - Collector address
    ///
    /// # Returns
    /// Reputation score record
    pub fn get_score(env: Env, collector: Address) -> ReputationScore {
        let key = common::StorageKey::Reputation(collector.clone());
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(ReputationScore {
                collector,
                score: 500, // Default neutral score
                total_transactions: 0,
                successful_transactions: 0,
                disputed_transactions: 0,
                last_updated: 0,
            })
    }

    /// Calculate reputation score based on transaction history
    ///
    /// # Arguments
    /// * `collector` - Collector address
    ///
    /// # Returns
    /// Calculated score (0-1000)
    pub fn calculate_score(env: Env, collector: Address) -> u32 {
        let score_record = Self::get_score(env, collector);
        Self::calculate_score_internal(&score_record)
    }

    /// Internal score calculation logic
    ///
    /// Score calculation:
    /// - Base score: 500 (neutral)
    /// - Each successful transaction: +5 points (up to max 1000)
    /// - Each disputed transaction: -10 points (down to min 0)
    /// - Success rate bonus: up to +100 points for high success rate
    fn calculate_score_internal(score_record: &ReputationScore) -> u32 {
        if score_record.total_transactions == 0 {
            return 500; // Neutral score for new collectors
        }

        let mut score: i32 = 500; // Start with neutral

        // Add points for successful transactions
        score += (score_record.successful_transactions as i32) * 5;

        // Subtract points for disputed transactions
        score -= (score_record.disputed_transactions as i32) * 10;

        // Calculate success rate bonus (using integer math to avoid floating-point)
        // Success rate = (successful * 10000) / total gives basis points (0-10000 = 0%-100%)
        let success_rate_basis_points = (score_record.successful_transactions * 10000)
            .checked_div(score_record.total_transactions)
            .unwrap_or(0);

        // Bonus for high success rate (only if significant transaction history)
        // Convert basis points to bonus: 10000 basis points = 100 bonus
        if score_record.total_transactions >= 10 {
            let bonus = (success_rate_basis_points / 100) as i32; // Convert to 0-100 range
            score += bonus;
        }

        // Clamp score between 0 and 1000
        if score < 0 {
            0
        } else if score > 1000 {
            1000
        } else {
            score as u32
        }
    }

    /// Manually set reputation score (admin only, for special cases)
    ///
    /// # Arguments
    /// * `collector` - Collector address
    /// * `new_score` - New score value (0-1000)
    pub fn set_score(env: Env, collector: Address, new_score: u32) {
        common::Initializable::require_initialized(&env).expect("Not initialized");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        // Validate score
        common::validation::validate_reputation_bounds(new_score).expect("Invalid score");

        // Get existing or create new score record
        let key = common::StorageKey::Reputation(collector.clone());
        let mut score: ReputationScore =
            env.storage()
                .persistent()
                .get(&key)
                .unwrap_or(ReputationScore {
                    collector: collector.clone(),
                    score: 500,
                    total_transactions: 0,
                    successful_transactions: 0,
                    disputed_transactions: 0,
                    last_updated: env.ledger().timestamp(),
                });

        // Update score
        let old_score = score.score;
        score.score = new_score;
        score.last_updated = env.ledger().timestamp();

        // Save updated score
        env.storage().persistent().set(&key, &score);
        common::bump_persistent(&env, &key);

        // Emit event
        common::ReputationEvents::score_updated(&env, collector, old_score, new_score);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Reset reputation score (admin only, for testing or special cases)
    ///
    /// # Arguments
    /// * `collector` - Collector address
    pub fn reset_score(env: Env, collector: Address) {
        common::Initializable::require_initialized(&env).expect("Not initialized");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        let key = common::StorageKey::Reputation(collector.clone());
        let reset_score = ReputationScore {
            collector: collector.clone(),
            score: 500,
            total_transactions: 0,
            successful_transactions: 0,
            disputed_transactions: 0,
            last_updated: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&key, &reset_score);
        common::bump_persistent(&env, &key);

        // Emit event
        common::ReputationEvents::score_updated(&env, collector, 0, 500);

        // Bump storage
        common::bump_instance(&env);
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

    /// Batch update reputation scores for multiple collectors
    ///
    /// # Arguments
    /// * `updates` - Vector of (collector, transaction_successful) tuples
    ///
    /// # Returns
    /// Number of scores successfully updated
    pub fn batch_update_scores(env: Env, updates: soroban_sdk::Vec<(Address, bool)>) -> u32 {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        let mut updated_count = 0u32;

        for i in 0..updates.len() {
            if let Some((collector, transaction_successful)) = updates.get(i) {
                // Get or create reputation score
                let key = common::StorageKey::Reputation(collector.clone());
                let mut score: ReputationScore =
                    env.storage()
                        .persistent()
                        .get(&key)
                        .unwrap_or(ReputationScore {
                            collector: collector.clone(),
                            score: 500,
                            total_transactions: 0,
                            successful_transactions: 0,
                            disputed_transactions: 0,
                            last_updated: env.ledger().timestamp(),
                        });

                // Update transaction counts
                score.total_transactions += 1;
                if transaction_successful {
                    score.successful_transactions += 1;
                } else {
                    score.disputed_transactions += 1;
                }

                // Calculate new score
                let old_score = score.score;
                score.score = Self::calculate_score_internal(&score);
                score.last_updated = env.ledger().timestamp();

                // Save updated score
                env.storage().persistent().set(&key, &score);
                common::bump_persistent(&env, &key);

                // Emit event
                common::ReputationEvents::score_updated(&env, collector, old_score, score.score);

                updated_count += 1;
            }
        }

        // Bump storage
        common::bump_instance(&env);

        updated_count
    }

    /// Get reputation scores for multiple collectors (optimized batch query)
    ///
    /// # Arguments
    /// * `collectors` - Vector of collector addresses
    ///
    /// # Returns
    /// Vector of reputation scores
    pub fn get_scores_batch(
        env: Env,
        collectors: soroban_sdk::Vec<Address>,
    ) -> soroban_sdk::Vec<ReputationScore> {
        let mut results = soroban_sdk::Vec::new(&env);

        for i in 0..collectors.len() {
            if let Some(collector) = collectors.get(i) {
                let score = Self::get_score(env.clone(), collector);
                results.push_back(score);
            }
        }

        results
    }

    /// Record transaction status for reputation tracking
    ///
    /// # Arguments
    /// * `collector` - Collector address
    /// * `status` - Transaction status
    ///
    /// # Note
    /// This is a convenience method that maps TransactionStatus to boolean
    pub fn record_transaction(env: Env, collector: Address, status: TransactionStatus) {
        let transaction_successful = status == TransactionStatus::Completed;
        Self::update_score(env, collector, transaction_successful);
    }

    /// Get collector statistics
    ///
    /// # Arguments
    /// * `collector` - Collector address
    ///
    /// # Returns
    /// Tuple of (score, total_transactions, successful_transactions, disputed_transactions)
    pub fn get_statistics(env: Env, collector: Address) -> (u32, u64, u64, u64) {
        let score_record = Self::get_score(env, collector);
        (
            score_record.score,
            score_record.total_transactions,
            score_record.successful_transactions,
            score_record.disputed_transactions,
        )
    }

    /// Get collectors by score range (leaderboard support)
    ///
    /// # Arguments
    /// * `_min_score` - Minimum score (inclusive) - placeholder
    /// * `_max_score` - Maximum score (inclusive) - placeholder
    /// * `_limit` - Maximum results (max 50) - placeholder
    ///
    /// # Returns
    /// Vector of ReputationScore records within the range
    ///
    /// # Note
    /// This is a placeholder. Real implementation needs indexed storage.
    pub fn get_collectors_by_score_range(
        env: Env,
        _min_score: u32,
        _max_score: u32,
        _limit: u64,
    ) -> soroban_sdk::Vec<ReputationScore> {
        let results = soroban_sdk::Vec::new(&env);

        // Note: This is a placeholder. Real implementation would need
        // a way to iterate through all reputation records in storage.
        // For now, returning empty vector as we don't have an index of all collectors.

        results
    }

    /// Get average reputation score
    ///
    /// # Returns
    /// Average score across all collectors (u32)
    ///
    /// # Note
    /// This is a placeholder. Real implementation would iterate all scores.
    pub fn get_average_score(_env: Env) -> u32 {
        // Placeholder: would need index of all reputation records
        500 // Return neutral score as placeholder
    }

    /// Get success rate for a collector
    ///
    /// # Arguments
    /// * `collector` - Collector address
    ///
    /// # Returns
    /// Success rate as percentage (0-100)
    pub fn get_success_rate(env: Env, collector: Address) -> u32 {
        let score_record = Self::get_score(env, collector);

        if score_record.total_transactions == 0 {
            return 0;
        }

        // Use integer math: (successful * 100) / total
        let rate = (score_record.successful_transactions * 100) / score_record.total_transactions;

        rate as u32
    }

    /// Check if collector meets minimum reputation threshold
    ///
    /// # Arguments
    /// * `collector` - Collector address
    /// * `threshold` - Minimum score required
    ///
    /// # Returns
    /// True if collector meets or exceeds threshold
    pub fn meets_threshold(env: Env, collector: Address, threshold: u32) -> bool {
        let score_record = Self::get_score(env, collector);
        score_record.score >= threshold
    }

    /// Get reputation tier for a collector
    ///
    /// # Arguments
    /// * `collector` - Collector address
    ///
    /// # Returns
    /// Tier number: 1 (Bronze: 0-400), 2 (Silver: 401-700), 3 (Gold: 701-900), 4 (Platinum: 901-1000)
    pub fn get_reputation_tier(env: Env, collector: Address) -> u32 {
        let score_record = Self::get_score(env, collector);

        match score_record.score {
            0..=400 => 1,    // Bronze
            401..=700 => 2,  // Silver
            701..=900 => 3,  // Gold
            901..=1000 => 4, // Platinum
            _ => 1,          // Default to Bronze
        }
    }

    /// Get detailed reputation breakdown
    ///
    /// # Arguments
    /// * `collector` - Collector address
    ///
    /// # Returns
    /// Tuple of (score, tier, success_rate, total_tx, successful_tx, disputed_tx)
    pub fn get_reputation_breakdown(
        env: Env,
        collector: Address,
    ) -> (u32, u32, u32, u64, u64, u64) {
        let score_record = Self::get_score(env.clone(), collector.clone());
        let tier = Self::get_reputation_tier(env.clone(), collector.clone());
        let success_rate = Self::get_success_rate(env, collector);

        (
            score_record.score,
            tier,
            success_rate,
            score_record.total_transactions,
            score_record.successful_transactions,
            score_record.disputed_transactions,
        )
    }
}
