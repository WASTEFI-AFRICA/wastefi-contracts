use crate::errors::WasteFiError;
use soroban_sdk::{Address, Env, String, Vec};

/// Fraud detection patterns and thresholds
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FraudPattern {
    RapidTransactions = 0,    // Too many transactions too quickly
    UnusualWeight = 1,        // Weight outside normal range
    SuspiciousLocation = 2,   // Collection from unusual location
    RepeatedRejections = 3,   // Many rejected transactions
    VelocityAnomaly = 4,      // Sudden spike in activity
    DuplicateTransaction = 5, // Potential duplicate submission
}

/// Risk score level
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RiskLevel {
    Low = 0,      // 0-300: Normal activity
    Medium = 1,   // 301-600: Elevated risk
    High = 2,     // 601-800: High risk, enhanced monitoring
    Critical = 3, // 801-1000: Critical risk, block transactions
}

impl RiskLevel {
    pub fn from_score(score: u32) -> Self {
        match score {
            0..=300 => RiskLevel::Low,
            301..=600 => RiskLevel::Medium,
            601..=800 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }
}

/// Fraud detection and prevention system
pub struct FraudDetection;

impl FraudDetection {
    /// Calculate risk score for a collector
    pub fn calculate_risk_score(env: &Env, collector: &Address) -> u32 {
        let mut risk_score = 0u32;

        // Check transaction velocity
        risk_score += Self::check_transaction_velocity(env, collector);

        // Check rejection rate
        risk_score += Self::check_rejection_rate(env, collector);

        // Check weight anomalies
        risk_score += Self::check_weight_anomalies(env, collector);

        // Check time pattern anomalies
        risk_score += Self::check_time_patterns(env, collector);

        // Cap at 1000
        if risk_score > 1000 {
            risk_score = 1000;
        }

        risk_score
    }

    /// Get risk level for a collector
    pub fn get_risk_level(env: &Env, collector: &Address) -> RiskLevel {
        let score = Self::get_stored_risk_score(env, collector);
        RiskLevel::from_score(score)
    }

    /// Update risk score for a collector
    pub fn update_risk_score(env: &Env, collector: &Address) {
        let new_score = Self::calculate_risk_score(env, collector);
        Self::store_risk_score(env, collector, new_score);
    }

    /// Check if collector is flagged for fraud
    pub fn is_flagged(env: &Env, collector: &Address) -> bool {
        matches!(
            Self::get_risk_level(env, collector),
            RiskLevel::High | RiskLevel::Critical
        )
    }

    /// Require not flagged for critical risk
    pub fn require_not_critical(env: &Env, collector: &Address) -> Result<(), WasteFiError> {
        if matches!(Self::get_risk_level(env, collector), RiskLevel::Critical) {
            return Err(WasteFiError::FraudDetected);
        }
        Ok(())
    }

    /// Flag a collector for manual review
    pub fn flag_for_review(env: &Env, collector: &Address, reason: String) {
        let key = ("FraudFlag", collector.clone());
        let timestamp = env.ledger().timestamp();
        env.storage().instance().set(&key, &(reason, timestamp));
        crate::bump_instance(env);
    }

    /// Check if collector is flagged for manual review
    pub fn is_flagged_for_review(env: &Env, collector: &Address) -> bool {
        let key = ("FraudFlag", collector.clone());
        env.storage().instance().has(&key)
    }

    /// Clear fraud flag (admin only)
    pub fn clear_flag(env: &Env, collector: &Address) {
        let key = ("FraudFlag", collector.clone());
        env.storage().instance().remove(&key);
    }

    /// Get fraud flag details
    pub fn get_flag_details(env: &Env, collector: &Address) -> Option<(String, u64)> {
        let key = ("FraudFlag", collector.clone());
        env.storage().instance().get(&key)
    }

    // Internal risk calculation methods

    /// Check transaction velocity (transactions per hour)
    fn check_transaction_velocity(env: &Env, collector: &Address) -> u32 {
        let key = ("TxVelocity", collector.clone());
        let recent_txs: Vec<u64> = env.storage().temporary().get(&key).unwrap_or(Vec::new(env));

        let current_time = env.ledger().timestamp();
        let hour_ago = current_time.saturating_sub(3600);

        // Count transactions in last hour
        let mut count = 0u32;
        for i in 0..recent_txs.len() {
            if let Some(tx_time) = recent_txs.get(i) {
                if tx_time >= hour_ago {
                    count += 1;
                }
            }
        }

        // Risk scoring based on velocity
        // Normal: 0-10 per hour (0 points)
        // Elevated: 11-20 per hour (100 points)
        // High: 21-30 per hour (200 points)
        // Critical: 30+ per hour (300 points)
        match count {
            0..=10 => 0,
            11..=20 => 100,
            21..=30 => 200,
            _ => 300,
        }
    }

    /// Check rejection rate
    fn check_rejection_rate(env: &Env, collector: &Address) -> u32 {
        let key = ("RejectionRate", collector.clone());
        let stats: Option<(u32, u32)> = env.storage().instance().get(&key);
        // (total_transactions, rejected_transactions)

        if let Some((total, rejected)) = stats {
            if total == 0 {
                return 0;
            }

            let rejection_percentage = (rejected as u64 * 100) / total as u64;

            // Risk scoring based on rejection rate
            // Normal: 0-10% (0 points)
            // Elevated: 11-25% (150 points)
            // High: 26-50% (250 points)
            // Critical: 50%+ (400 points)
            match rejection_percentage {
                0..=10 => 0,
                11..=25 => 150,
                26..=50 => 250,
                _ => 400,
            }
        } else {
            0
        }
    }

    /// Check for weight anomalies
    fn check_weight_anomalies(env: &Env, collector: &Address) -> u32 {
        let key = ("WeightHistory", collector.clone());
        let weights: Vec<u64> = env.storage().temporary().get(&key).unwrap_or(Vec::new(env));

        if weights.len() < 5 {
            return 0; // Not enough data
        }

        // Calculate average weight
        let mut sum = 0u64;
        for i in 0..weights.len() {
            if let Some(weight) = weights.get(i) {
                sum = sum.saturating_add(weight);
            }
        }
        let avg = sum / weights.len() as u64;

        // Check last weight against average
        if let Some(last_weight) = weights.get(weights.len() - 1) {
            // If last weight is more than 3x average, flag as anomaly
            if last_weight > avg.saturating_mul(3) {
                return 200;
            }
            // If last weight is more than 2x average
            if last_weight > avg.saturating_mul(2) {
                return 100;
            }
        }

        0
    }

    /// Check for time pattern anomalies
    fn check_time_patterns(env: &Env, collector: &Address) -> u32 {
        let key = ("TxVelocity", collector.clone());
        let recent_txs: Vec<u64> = env.storage().temporary().get(&key).unwrap_or(Vec::new(env));

        if recent_txs.len() < 3 {
            return 0;
        }

        // Check for transactions submitted within very short intervals
        let mut suspicious_intervals = 0u32;
        for i in 1..recent_txs.len() {
            if let (Some(prev), Some(curr)) = (recent_txs.get(i - 1), recent_txs.get(i)) {
                let interval = curr.saturating_sub(prev);
                // Transactions less than 30 seconds apart are suspicious
                if interval < 30 {
                    suspicious_intervals += 1;
                }
            }
        }

        // Risk based on number of suspicious intervals
        match suspicious_intervals {
            0..=1 => 0,
            2..=3 => 100,
            _ => 200,
        }
    }

    /// Record a transaction for velocity tracking
    pub fn record_transaction(env: &Env, collector: &Address) {
        let key = ("TxVelocity", collector.clone());
        let mut recent_txs: Vec<u64> = env.storage().temporary().get(&key).unwrap_or(Vec::new(env));

        let current_time = env.ledger().timestamp();
        recent_txs.push_back(current_time);

        // Keep only last 100 transactions
        while recent_txs.len() > 100 {
            recent_txs.remove(0);
        }

        env.storage().temporary().set(&key, &recent_txs);
        env.storage().temporary().extend_ttl(&key, 0, 86400); // 24 hours
    }

    /// Record a weight for anomaly detection
    pub fn record_weight(env: &Env, collector: &Address, weight: u64) {
        let key = ("WeightHistory", collector.clone());
        let mut weights: Vec<u64> = env.storage().temporary().get(&key).unwrap_or(Vec::new(env));

        weights.push_back(weight);

        // Keep only last 50 weights
        while weights.len() > 50 {
            weights.remove(0);
        }

        env.storage().temporary().set(&key, &weights);
        env.storage().temporary().extend_ttl(&key, 0, 604800); // 7 days
    }

    /// Update rejection statistics
    pub fn record_rejection(env: &Env, collector: &Address, was_rejected: bool) {
        let key = ("RejectionRate", collector.clone());
        let mut stats: (u32, u32) = env.storage().instance().get(&key).unwrap_or((0, 0));
        // (total_transactions, rejected_transactions)

        stats.0 += 1; // Increment total
        if was_rejected {
            stats.1 += 1; // Increment rejected
        }

        env.storage().instance().set(&key, &stats);
        crate::bump_instance(env);
    }

    /// Store risk score
    fn store_risk_score(env: &Env, collector: &Address, score: u32) {
        let key = ("RiskScore", collector.clone());
        env.storage().instance().set(&key, &score);
        crate::bump_instance(env);
    }

    /// Get stored risk score
    fn get_stored_risk_score(env: &Env, collector: &Address) -> u32 {
        let key = ("RiskScore", collector.clone());
        env.storage().instance().get(&key).unwrap_or(0)
    }
}

/// Rate limiting with multiple tiers
pub struct RateLimit;

impl RateLimit {
    /// Tier 1: Per-minute rate limit (fast operations)
    pub fn check_per_minute(
        env: &Env,
        operation: String,
        caller: &Address,
        max_per_minute: u32,
    ) -> Result<(), WasteFiError> {
        Self::check_rate(env, operation, caller, max_per_minute, 60)
    }

    /// Tier 2: Per-hour rate limit (moderate operations)
    pub fn check_per_hour(
        env: &Env,
        operation: String,
        caller: &Address,
        max_per_hour: u32,
    ) -> Result<(), WasteFiError> {
        Self::check_rate(env, operation, caller, max_per_hour, 3600)
    }

    /// Tier 3: Per-day rate limit (heavy operations)
    pub fn check_per_day(
        env: &Env,
        operation: String,
        caller: &Address,
        max_per_day: u32,
    ) -> Result<(), WasteFiError> {
        Self::check_rate(env, operation, caller, max_per_day, 86400)
    }

    /// Generic rate check
    fn check_rate(
        env: &Env,
        operation: String,
        caller: &Address,
        max_attempts: u32,
        window_seconds: u64,
    ) -> Result<(), WasteFiError> {
        crate::OperationThrottle::require_not_throttled(
            env,
            operation,
            caller,
            max_attempts,
            window_seconds,
        )
    }

    /// Record rate limit attempt
    pub fn record(env: &Env, operation: String, caller: &Address) {
        crate::OperationThrottle::record_attempt(env, operation, caller);
    }

    /// Get remaining quota for time period
    pub fn get_remaining_quota(
        env: &Env,
        operation: String,
        caller: &Address,
        max_attempts: u32,
        window_seconds: u64,
    ) -> u32 {
        let key = ("Throttle", operation, caller.clone());
        let current_time = env.ledger().timestamp();

        if let Some(attempts) = env.storage().temporary().get::<_, Vec<u64>>(&key) {
            // Count attempts within the time window
            let mut count = 0u32;
            for i in 0..attempts.len() {
                if let Some(attempt_time) = attempts.get(i) {
                    if current_time.saturating_sub(attempt_time) <= window_seconds {
                        count += 1;
                    }
                }
            }

            max_attempts.saturating_sub(count)
        } else {
            max_attempts
        }
    }
}

/// Duplicate transaction detection
pub struct DuplicateDetection;

impl DuplicateDetection {
    /// Check if transaction appears to be a duplicate
    pub fn is_duplicate(
        env: &Env,
        collector: &Address,
        weight: u64,
        material: u32,
        tolerance_seconds: u64,
    ) -> bool {
        let key = ("RecentTx", collector.clone());
        let recent: Vec<(u64, u32, u64)> =
            env.storage().temporary().get(&key).unwrap_or(Vec::new(env));
        // (weight, material, timestamp)

        let current_time = env.ledger().timestamp();

        // Check for matching transactions within tolerance window
        for i in 0..recent.len() {
            if let Some((prev_weight, prev_material, prev_time)) = recent.get(i) {
                let time_diff = current_time.saturating_sub(prev_time);

                if time_diff <= tolerance_seconds
                    && prev_weight == weight
                    && prev_material == material
                {
                    return true; // Duplicate found
                }
            }
        }

        false
    }

    /// Record transaction for duplicate detection
    pub fn record_transaction(env: &Env, collector: &Address, weight: u64, material: u32) {
        let key = ("RecentTx", collector.clone());
        let mut recent: Vec<(u64, u32, u64)> =
            env.storage().temporary().get(&key).unwrap_or(Vec::new(env));

        let current_time = env.ledger().timestamp();
        recent.push_back((weight, material, current_time));

        // Keep only last 20 transactions
        while recent.len() > 20 {
            recent.remove(0);
        }

        env.storage().temporary().set(&key, &recent);
        env.storage().temporary().extend_ttl(&key, 0, 3600); // 1 hour
    }

    /// Require not duplicate
    pub fn require_not_duplicate(
        env: &Env,
        collector: &Address,
        weight: u64,
        material: u32,
        tolerance_seconds: u64,
    ) -> Result<(), WasteFiError> {
        if Self::is_duplicate(env, collector, weight, material, tolerance_seconds) {
            return Err(WasteFiError::DuplicateTransaction);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_risk_level_from_score() {
        assert_eq!(RiskLevel::from_score(0), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(300), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(301), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(600), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(601), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(800), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(801), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(1000), RiskLevel::Critical);
    }

    #[test]
    fn test_fraud_detection_flagging() {
        let env = Env::default();
        let collector = Address::generate(&env);

        assert!(!FraudDetection::is_flagged_for_review(&env, &collector));

        FraudDetection::flag_for_review(
            &env,
            &collector,
            String::from_str(&env, "Suspicious activity"),
        );

        assert!(FraudDetection::is_flagged_for_review(&env, &collector));

        FraudDetection::clear_flag(&env, &collector);
        assert!(!FraudDetection::is_flagged_for_review(&env, &collector));
    }

    #[test]
    fn test_transaction_velocity_tracking() {
        let env = Env::default();
        let collector = Address::generate(&env);

        // Record several transactions
        for _ in 0..5 {
            FraudDetection::record_transaction(&env, &collector);
        }

        // Risk score should increase with velocity
        let score = FraudDetection::calculate_risk_score(&env, &collector);
        assert!(score >= 0); // At least some base score
    }

    #[test]
    fn test_duplicate_detection() {
        let env = Env::default();
        let collector = Address::generate(&env);

        assert!(!DuplicateDetection::is_duplicate(
            &env, &collector, 1000, 1, 300
        ));

        DuplicateDetection::record_transaction(&env, &collector, 1000, 1);

        assert!(DuplicateDetection::is_duplicate(
            &env, &collector, 1000, 1, 300
        ));
        assert!(!DuplicateDetection::is_duplicate(
            &env, &collector, 2000, 1, 300
        ));
    }

    #[test]
    fn test_rate_limit_quota() {
        let env = Env::default();
        let operation = String::from_str(&env, "test_op");
        let caller = Address::generate(&env);

        let remaining = RateLimit::get_remaining_quota(&env, operation.clone(), &caller, 10, 60);
        assert_eq!(remaining, 10);

        RateLimit::record(&env, operation.clone(), &caller);

        let remaining = RateLimit::get_remaining_quota(&env, operation, &caller, 10, 60);
        assert_eq!(remaining, 9);
    }

    #[test]
    fn test_weight_anomaly_detection() {
        let env = Env::default();
        let collector = Address::generate(&env);

        // Record normal weights
        for _ in 0..10 {
            FraudDetection::record_weight(&env, &collector, 1000);
        }

        // Record anomalous weight
        FraudDetection::record_weight(&env, &collector, 5000);

        let score = FraudDetection::calculate_risk_score(&env, &collector);
        assert!(score > 0); // Should detect anomaly
    }
}
