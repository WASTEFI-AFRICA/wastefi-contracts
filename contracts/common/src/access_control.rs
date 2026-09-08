use soroban_sdk::{Address, Env};
use crate::errors::WasteFiError;
use crate::storage::StorageKey;

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
    pub fn transfer_admin(env: &Env, current_admin: &Address, new_admin: Address) -> Result<(), WasteFiError> {
        Self::require_admin(env, current_admin)?;
        Self::set_admin(env, new_admin);
        Ok(())
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
        Self::pause(env);
        Ok(())
    }

    /// Unpause contract (admin only)
    pub fn admin_unpause(env: &Env, admin: &Address) -> Result<(), WasteFiError> {
        AccessControl::require_admin(env, admin)?;
        Self::unpause(env);
        Ok(())
    }
}

/// Initialization control
pub struct Initializable;

impl Initializable {
    /// Mark contract as initialized
    pub fn mark_initialized(env: &Env) {
        env.storage().instance().set(&StorageKey::Initialized, &true);
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
