use crate::errors::WasteFiError;
use crate::storage::StorageKey;
use soroban_sdk::{Address, Env, String, Vec};

/// Emergency status levels
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EmergencyLevel {
    Normal = 0,   // No emergency
    Warning = 1,  // Potential issue detected
    Critical = 2, // Major issue, limited functionality
    Shutdown = 3, // Complete shutdown
}

/// Emergency event record
#[derive(Clone, Debug)]
pub struct EmergencyEvent {
    pub level: EmergencyLevel,
    pub reason: String,
    pub triggered_by: Address,
    pub timestamp: u64,
    pub resolved: bool,
}

/// Emergency response system for critical situations
pub struct Emergency;

impl Emergency {
    /// Set emergency level
    pub fn set_level(env: &Env, level: EmergencyLevel) {
        let key = StorageKey::EmergencyLevel;
        env.storage().instance().set(&key, &(level as u32));
    }

    /// Get current emergency level
    pub fn get_level(env: &Env) -> EmergencyLevel {
        let key = StorageKey::EmergencyLevel;
        let level_u32: u32 = env.storage().instance().get(&key).unwrap_or(0);

        match level_u32 {
            0 => EmergencyLevel::Normal,
            1 => EmergencyLevel::Warning,
            2 => EmergencyLevel::Critical,
            3 => EmergencyLevel::Shutdown,
            _ => EmergencyLevel::Normal,
        }
    }

    /// Trigger emergency (admin only)
    pub fn trigger(
        env: &Env,
        admin: &Address,
        level: EmergencyLevel,
        reason: String,
    ) -> Result<(), WasteFiError> {
        crate::AccessControl::require_admin(env, admin)?;

        // Set emergency level
        Self::set_level(env, level);

        // Log emergency event
        Self::log_event(env, level, reason, admin.clone(), false);

        // If critical or shutdown, also pause the contract
        if matches!(level, EmergencyLevel::Critical | EmergencyLevel::Shutdown) {
            crate::Pausable::pause(env);
        }

        // Log admin action
        crate::AccessControl::log_admin_action(
            env,
            admin,
            String::from_str(env, "trigger_emergency"),
            Some(String::from_str(env, "level_set")),
        );

        Ok(())
    }

    /// Resolve emergency (admin only)
    pub fn resolve(env: &Env, admin: &Address) -> Result<(), WasteFiError> {
        crate::AccessControl::require_admin(env, admin)?;

        let current_level = Self::get_level(env);

        // Set back to normal
        Self::set_level(env, EmergencyLevel::Normal);

        // Mark last event as resolved
        Self::mark_resolved(env);

        // If contract was paused, unpause it
        if matches!(
            current_level,
            EmergencyLevel::Critical | EmergencyLevel::Shutdown
        ) {
            crate::Pausable::unpause(env);
        }

        // Log admin action
        crate::AccessControl::log_admin_action(
            env,
            admin,
            String::from_str(env, "resolve_emergency"),
            None,
        );

        Ok(())
    }

    /// Check if emergency is active
    pub fn is_active(env: &Env) -> bool {
        !matches!(Self::get_level(env), EmergencyLevel::Normal)
    }

    /// Require normal operation (no emergency)
    pub fn require_normal(env: &Env) -> Result<(), WasteFiError> {
        if Self::is_active(env) {
            return Err(WasteFiError::EmergencyActive);
        }
        Ok(())
    }

    /// Require not in shutdown
    pub fn require_not_shutdown(env: &Env) -> Result<(), WasteFiError> {
        if matches!(Self::get_level(env), EmergencyLevel::Shutdown) {
            return Err(WasteFiError::EmergencyShutdown);
        }
        Ok(())
    }

    /// Log emergency event
    fn log_event(
        env: &Env,
        level: EmergencyLevel,
        reason: String,
        triggered_by: Address,
        resolved: bool,
    ) {
        let log_key = ("EmergencyLog",);
        let mut events: Vec<(u32, String, Address, u64, bool)> = env
            .storage()
            .instance()
            .get(&log_key)
            .unwrap_or(Vec::new(env));

        // Keep only last 50 events
        if events.len() >= 50 {
            events.remove(0);
        }

        events.push_back((
            level as u32,
            reason,
            triggered_by,
            env.ledger().timestamp(),
            resolved,
        ));

        env.storage().instance().set(&log_key, &events);
    }

    /// Mark last emergency event as resolved
    fn mark_resolved(env: &Env) {
        let log_key = ("EmergencyLog",);
        let mut events: Vec<(u32, String, Address, u64, bool)> = env
            .storage()
            .instance()
            .get(&log_key)
            .unwrap_or(Vec::new(env));

        if !events.is_empty() {
            let last_idx = events.len() - 1;
            if let Some((level, reason, triggered_by, timestamp, _)) = events.get(last_idx) {
                events.set(last_idx, (level, reason, triggered_by, timestamp, true));
                env.storage().instance().set(&log_key, &events);
            }
        }
    }

    /// Get emergency event history
    pub fn get_event_history(env: &Env, limit: u32) -> Vec<(u32, String, Address, u64, bool)> {
        let log_key = ("EmergencyLog",);
        let events: Vec<(u32, String, Address, u64, bool)> = env
            .storage()
            .instance()
            .get(&log_key)
            .unwrap_or(Vec::new(env));

        let max_limit = if limit > 50 { 50 } else { limit };
        let start = if events.len() > max_limit {
            events.len() - max_limit
        } else {
            0
        };

        let mut result = Vec::new(env);
        for i in start..events.len() {
            if let Some(event) = events.get(i) {
                result.push_back(event);
            }
        }

        result
    }
}

/// Circuit breaker pattern for critical operations
pub struct CircuitBreaker;

impl CircuitBreaker {
    /// Trip the circuit breaker for a specific operation
    pub fn trip(env: &Env, operation: String) {
        let key = ("CircuitBreaker", operation.clone());
        let trip_time = env.ledger().timestamp();
        env.storage().instance().set(&key, &trip_time);
        crate::bump_instance(env);
    }

    /// Reset the circuit breaker for an operation
    pub fn reset(env: &Env, operation: String) {
        let key = ("CircuitBreaker", operation);
        env.storage().instance().remove(&key);
    }

    /// Check if circuit breaker is tripped
    pub fn is_tripped(env: &Env, operation: String) -> bool {
        let key = ("CircuitBreaker", operation);
        env.storage().instance().has(&key)
    }

    /// Check if circuit breaker has cooled down
    pub fn can_retry(env: &Env, operation: String, cooldown_seconds: u64) -> bool {
        let key = ("CircuitBreaker", operation);

        if let Some(trip_time) = env.storage().instance().get::<_, u64>(&key) {
            let current_time = env.ledger().timestamp();
            let elapsed = current_time.saturating_sub(trip_time);
            elapsed >= cooldown_seconds
        } else {
            true // Not tripped, can proceed
        }
    }

    /// Require circuit not tripped
    pub fn require_not_tripped(env: &Env, operation: String) -> Result<(), WasteFiError> {
        if Self::is_tripped(env, operation) {
            return Err(WasteFiError::CircuitBreakerTripped);
        }
        Ok(())
    }

    /// Auto-reset if cooldown period has passed
    pub fn auto_reset_if_ready(env: &Env, operation: String, cooldown_seconds: u64) {
        if Self::can_retry(env, operation.clone(), cooldown_seconds) {
            Self::reset(env, operation);
        }
    }
}

/// Emergency withdrawal system for stuck funds
pub struct EmergencyWithdrawal;

impl EmergencyWithdrawal {
    /// Enable emergency withdrawal mode (admin only)
    pub fn enable(env: &Env, admin: &Address) -> Result<(), WasteFiError> {
        crate::AccessControl::require_admin(env, admin)?;

        let key = ("EmergencyWithdrawalEnabled",);
        env.storage().instance().set(&key, &true);

        // Log action
        crate::AccessControl::log_admin_action(
            env,
            admin,
            String::from_str(env, "enable_emergency_withdrawal"),
            None,
        );

        crate::bump_instance(env);
        Ok(())
    }

    /// Disable emergency withdrawal mode (admin only)
    pub fn disable(env: &Env, admin: &Address) -> Result<(), WasteFiError> {
        crate::AccessControl::require_admin(env, admin)?;

        let key = ("EmergencyWithdrawalEnabled",);
        env.storage().instance().set(&key, &false);

        // Log action
        crate::AccessControl::log_admin_action(
            env,
            admin,
            String::from_str(env, "disable_emergency_withdrawal"),
            None,
        );

        crate::bump_instance(env);
        Ok(())
    }

    /// Check if emergency withdrawal is enabled
    pub fn is_enabled(env: &Env) -> bool {
        let key = ("EmergencyWithdrawalEnabled",);
        env.storage().instance().get(&key).unwrap_or(false)
    }

    /// Require emergency withdrawal mode is enabled
    pub fn require_enabled(env: &Env) -> Result<(), WasteFiError> {
        if !Self::is_enabled(env) {
            return Err(WasteFiError::EmergencyWithdrawalNotEnabled);
        }
        Ok(())
    }

    /// Record emergency withdrawal
    pub fn record_withdrawal(env: &Env, admin: &Address, amount: i128, recipient: Address) {
        let log_key = ("EmergencyWithdrawals",);
        let mut withdrawals: Vec<(Address, i128, Address, u64)> = env
            .storage()
            .instance()
            .get(&log_key)
            .unwrap_or(Vec::new(env));

        withdrawals.push_back((admin.clone(), amount, recipient, env.ledger().timestamp()));

        env.storage().instance().set(&log_key, &withdrawals);
    }

    /// Get withdrawal history
    pub fn get_withdrawal_history(env: &Env) -> Vec<(Address, i128, Address, u64)> {
        let log_key = ("EmergencyWithdrawals",);
        env.storage()
            .instance()
            .get(&log_key)
            .unwrap_or(Vec::new(env))
    }
}

/// Operation throttling for rate limiting
pub struct OperationThrottle;

impl OperationThrottle {
    /// Record an operation attempt
    pub fn record_attempt(env: &Env, operation: String, caller: &Address) {
        let key = ("Throttle", operation, caller.clone());
        let current_time = env.ledger().timestamp();

        let mut attempts: Vec<u64> = env.storage().temporary().get(&key).unwrap_or(Vec::new(env));

        attempts.push_back(current_time);

        // Keep only last 100 attempts
        while attempts.len() > 100 {
            attempts.remove(0);
        }

        env.storage().temporary().set(&key, &attempts);
        env.storage().temporary().extend_ttl(&key, 0, 86400); // 24 hours
    }

    /// Check if operation is throttled
    pub fn is_throttled(
        env: &Env,
        operation: String,
        caller: &Address,
        max_attempts: u32,
        window_seconds: u64,
    ) -> bool {
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

            count >= max_attempts
        } else {
            false
        }
    }

    /// Require not throttled
    pub fn require_not_throttled(
        env: &Env,
        operation: String,
        caller: &Address,
        max_attempts: u32,
        window_seconds: u64,
    ) -> Result<(), WasteFiError> {
        if Self::is_throttled(env, operation, caller, max_attempts, window_seconds) {
            return Err(WasteFiError::OperationThrottled);
        }
        Ok(())
    }

    /// Clear throttle records for an address (admin only)
    pub fn clear_for_address(env: &Env, operation: String, address: &Address) {
        let key = ("Throttle", operation, address.clone());
        env.storage().temporary().remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_emergency_levels() {
        let env = Env::default();

        assert_eq!(Emergency::get_level(&env), EmergencyLevel::Normal);

        Emergency::set_level(&env, EmergencyLevel::Warning);
        assert_eq!(Emergency::get_level(&env), EmergencyLevel::Warning);
        assert!(Emergency::is_active(&env));

        Emergency::set_level(&env, EmergencyLevel::Normal);
        assert!(!Emergency::is_active(&env));
    }

    #[test]
    fn test_circuit_breaker() {
        let env = Env::default();
        let operation = String::from_str(&env, "test_operation");

        assert!(!CircuitBreaker::is_tripped(&env, operation.clone()));

        CircuitBreaker::trip(&env, operation.clone());
        assert!(CircuitBreaker::is_tripped(&env, operation.clone()));

        CircuitBreaker::reset(&env, operation.clone());
        assert!(!CircuitBreaker::is_tripped(&env, operation));
    }

    #[test]
    fn test_emergency_withdrawal() {
        let env = Env::default();

        assert!(!EmergencyWithdrawal::is_enabled(&env));

        let admin = Address::generate(&env);
        crate::AccessControl::set_admin(&env, admin.clone());

        EmergencyWithdrawal::enable(&env, &admin).unwrap();
        assert!(EmergencyWithdrawal::is_enabled(&env));

        EmergencyWithdrawal::disable(&env, &admin).unwrap();
        assert!(!EmergencyWithdrawal::is_enabled(&env));
    }

    #[test]
    fn test_operation_throttle() {
        let env = Env::default();
        let operation = String::from_str(&env, "test_op");
        let caller = Address::generate(&env);

        assert!(!OperationThrottle::is_throttled(
            &env,
            operation.clone(),
            &caller,
            3,
            60
        ));

        OperationThrottle::record_attempt(&env, operation.clone(), &caller);
        OperationThrottle::record_attempt(&env, operation.clone(), &caller);
        OperationThrottle::record_attempt(&env, operation.clone(), &caller);

        assert!(OperationThrottle::is_throttled(
            &env, operation, &caller, 3, 60
        ));
    }

    #[test]
    fn test_emergency_event_history() {
        let env = Env::default();
        let admin = Address::generate(&env);
        crate::AccessControl::set_admin(&env, admin.clone());

        Emergency::trigger(
            &env,
            &admin,
            EmergencyLevel::Warning,
            String::from_str(&env, "Test alert"),
        )
        .unwrap();

        let history = Emergency::get_event_history(&env, 10);
        assert_eq!(history.len(), 1);
    }
}
