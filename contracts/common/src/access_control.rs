use crate::errors::WasteFiError;
use crate::storage::StorageKey;
use soroban_sdk::{Address, Env, String, Vec};

/// Admin role enumeration
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdminRole {
    SuperAdmin = 0, // Full permissions
    Operator = 1,   // Can verify, update statuses
    Auditor = 2,    // Read-only access
}

/// Admin action log entry
#[derive(Clone, Debug)]
pub struct AdminAction {
    pub admin: Address,
    pub action: String,
    pub timestamp: u64,
    pub target: Option<String>,
}

/// Access control utilities for role-based permissions
/// Admin role management
pub struct AccessControl;

impl AccessControl {
    /// Initialize admin
    pub fn set_admin(env: &Env, admin: Address) {
        env.storage().instance().set(&StorageKey::Admin, &admin);
    }

    /// Get current admin
    pub fn get_admin(env: &Env) -> Result<Address, WasteFiError> {
        env.storage()
            .instance()
            .get(&StorageKey::Admin)
            .ok_or(WasteFiError::NotInitialized)
    }

    /// Check if address is admin
    pub fn is_admin(env: &Env, address: &Address) -> bool {
        if let Ok(admin) = Self::get_admin(env) {
            &admin == address
        } else {
            false
        }
    }

    /// Require caller to be admin
    pub fn require_admin(env: &Env, caller: &Address) -> Result<(), WasteFiError> {
        let admin = Self::get_admin(env)?;
        if caller != &admin {
            return Err(WasteFiError::NotAdmin);
        }
        caller.require_auth();
        Ok(())
    }

    /// Transfer admin role
    pub fn transfer_admin(
        env: &Env,
        current_admin: &Address,
        new_admin: Address,
    ) -> Result<(), WasteFiError> {
        Self::require_admin(env, current_admin)?;

        // Log admin transfer
        Self::log_admin_action(
            env,
            current_admin,
            String::from_str(env, "transfer_admin"),
            Some(new_admin.to_string()),
        );

        Self::set_admin(env, new_admin);
        Ok(())
    }

    /// Add secondary admin (operator role)
    pub fn add_operator(env: &Env, operator: Address) {
        let key = ("Operator", operator.clone());
        env.storage().instance().set(&key, &true);
        crate::bump_instance(env);
    }

    /// Remove operator
    pub fn remove_operator(env: &Env, operator: &Address) {
        let key = ("Operator", operator.clone());
        env.storage().instance().remove(&key);
    }

    /// Check if address is operator
    pub fn is_operator(env: &Env, address: &Address) -> bool {
        let key = ("Operator", address.clone());
        env.storage().instance().get(&key).unwrap_or(false)
    }

    /// Check if address has admin or operator role
    pub fn has_elevated_access(env: &Env, address: &Address) -> bool {
        Self::is_admin(env, address) || Self::is_operator(env, address)
    }

    /// Require admin or operator role
    pub fn require_elevated_access(env: &Env, caller: &Address) -> Result<(), WasteFiError> {
        if !Self::has_elevated_access(env, caller) {
            return Err(WasteFiError::NotAdmin);
        }
        caller.require_auth();
        Ok(())
    }

    /// Log admin action for audit trail
    pub fn log_admin_action(env: &Env, admin: &Address, action: String, target: Option<String>) {
        let log_key = ("AdminActionLog",);
        let mut actions: Vec<(Address, String, u64, Option<String>)> = env
            .storage()
            .instance()
            .get(&log_key)
            .unwrap_or(Vec::new(env));

        // Keep only last 100 actions to prevent unbounded growth
        if actions.len() >= 100 {
            actions.remove(0);
        }

        actions.push_back((admin.clone(), action, env.ledger().timestamp(), target));

        env.storage().instance().set(&log_key, &actions);
    }

    /// Get recent admin actions (last N actions)
    pub fn get_admin_actions(env: &Env, limit: u32) -> Vec<(Address, String, u64, Option<String>)> {
        let log_key = ("AdminActionLog",);
        let actions: Vec<(Address, String, u64, Option<String>)> = env
            .storage()
            .instance()
            .get(&log_key)
            .unwrap_or(Vec::new(env));

        let max_limit = if limit > 50 { 50 } else { limit };
        let start = if actions.len() > max_limit {
            actions.len() - max_limit
        } else {
            0
        };

        let mut result = Vec::new(env);
        for i in start..actions.len() {
            if let Some(action) = actions.get(i) {
                result.push_back(action);
            }
        }

        result
    }

    /// Get all operators
    pub fn get_operators(env: &Env) -> Vec<Address> {
        // Note: This is a simplified version. For production, maintain an operators list.
        Vec::new(env)
    }
}

/// Contract pause functionality
pub struct Pausable;

impl Pausable {
    /// Pause the contract
    pub fn pause(env: &Env) {
        env.storage().instance().set(&StorageKey::Paused, &true);
    }

    /// Unpause the contract
    pub fn unpause(env: &Env) {
        env.storage().instance().set(&StorageKey::Paused, &false);
    }

    /// Check if contract is paused
    pub fn is_paused(env: &Env) -> bool {
        env.storage()
            .instance()
            .get(&StorageKey::Paused)
            .unwrap_or(false)
    }

    /// Require contract is not paused
    pub fn require_not_paused(env: &Env) -> Result<(), WasteFiError> {
        if Self::is_paused(env) {
            return Err(WasteFiError::ContractPaused);
        }
        Ok(())
    }

    /// Pause contract (admin only)
    pub fn admin_pause(env: &Env, admin: &Address) -> Result<(), WasteFiError> {
        AccessControl::require_admin(env, admin)?;

        // Log pause action
        AccessControl::log_admin_action(env, admin, String::from_str(env, "pause_contract"), None);

        Self::pause(env);
        Ok(())
    }

    /// Unpause contract (admin only)
    pub fn admin_unpause(env: &Env, admin: &Address) -> Result<(), WasteFiError> {
        AccessControl::require_admin(env, admin)?;

        // Log unpause action
        AccessControl::log_admin_action(
            env,
            admin,
            String::from_str(env, "unpause_contract"),
            None,
        );

        Self::unpause(env);
        Ok(())
    }

    /// Get pause history (last N pause/unpause events)
    pub fn get_pause_history(env: &Env, limit: u32) -> Vec<(String, u64)> {
        let actions = AccessControl::get_admin_actions(env, limit);
        let mut history = Vec::new(env);

        let pause_str = String::from_str(env, "pause");
        let unpause_str = String::from_str(env, "unpause");

        for i in 0..actions.len() {
            if let Some((_, action, timestamp, _)) = actions.get(i) {
                // Check if action contains "pause" keywords
                // Note: Simple comparison since Soroban String doesn't have contains()
                if action == pause_str || action == unpause_str {
                    history.push_back((action, timestamp));
                }
            }
        }

        history
    }
}

/// Initialization control
pub struct Initializable;

impl Initializable {
    /// Mark contract as initialized
    pub fn mark_initialized(env: &Env) {
        env.storage()
            .instance()
            .set(&StorageKey::Initialized, &true);
    }

    /// Check if contract is initialized
    pub fn is_initialized(env: &Env) -> bool {
        env.storage()
            .instance()
            .get(&StorageKey::Initialized)
            .unwrap_or(false)
    }

    /// Require contract is initialized
    pub fn require_initialized(env: &Env) -> Result<(), WasteFiError> {
        if !Self::is_initialized(env) {
            return Err(WasteFiError::NotInitialized);
        }
        Ok(())
    }

    /// Require contract is not initialized (for initialization)
    pub fn require_not_initialized(env: &Env) -> Result<(), WasteFiError> {
        if Self::is_initialized(env) {
            return Err(WasteFiError::AlreadyInitialized);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_admin_management() {
        let env = Env::default();
        let admin = Address::generate(&env);

        AccessControl::set_admin(&env, admin.clone());
        assert!(AccessControl::is_admin(&env, &admin));

        let non_admin = Address::generate(&env);
        assert!(!AccessControl::is_admin(&env, &non_admin));
    }

    #[test]
    fn test_operator_management() {
        let env = Env::default();
        let operator = Address::generate(&env);

        assert!(!AccessControl::is_operator(&env, &operator));

        AccessControl::add_operator(&env, operator.clone());
        assert!(AccessControl::is_operator(&env, &operator));
        assert!(AccessControl::has_elevated_access(&env, &operator));

        AccessControl::remove_operator(&env, &operator);
        assert!(!AccessControl::is_operator(&env, &operator));
    }

    #[test]
    fn test_admin_action_logging() {
        let env = Env::default();
        let admin = Address::generate(&env);

        AccessControl::log_admin_action(
            &env,
            &admin,
            String::from_str(&env, "test_action"),
            Some(String::from_str(&env, "target1")),
        );

        let actions = AccessControl::get_admin_actions(&env, 10);
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_pausable() {
        let env = Env::default();

        assert!(!Pausable::is_paused(&env));

        Pausable::pause(&env);
        assert!(Pausable::is_paused(&env));

        Pausable::unpause(&env);
        assert!(!Pausable::is_paused(&env));
    }

    #[test]
    fn test_initializable() {
        let env = Env::default();

        assert!(!Initializable::is_initialized(&env));

        Initializable::mark_initialized(&env);
        assert!(Initializable::is_initialized(&env));
    }
}
