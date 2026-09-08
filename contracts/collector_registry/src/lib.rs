#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod storage;
use storage::*;

#[cfg(test)]
mod test;

#[contract]
pub struct CollectorRegistry;

#[contractimpl]
impl CollectorRegistry {
    /// Initialize the collector registry
    /// 
    /// # Arguments
    /// * `admin` - Contract administrator address
    pub fn initialize(env: Env, admin: Address) {
        common::Initializable::require_not_initialized(&env)
            .expect("Already initialized");
        
        // Set admin
        common::AccessControl::set_admin(&env, admin);
        
        // Initialize collector count to 0
        write_collector_count(&env, 0);
        
        // Mark as initialized
        common::Initializable::mark_initialized(&env);
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Register a new collector
    /// 
    /// # Arguments
    /// * `collector` - Collector address
    /// * `name` - Collector full name
    /// * `phone` - Phone number for contact
    pub fn register(env: Env, collector: Address, name: String, phone: String) {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        common::Pausable::require_not_paused(&env)
            .expect("Contract paused");
        
        // Require authorization from collector
        collector.require_auth();
        
        // Check if collector already exists
        if has_collector(&env, &collector) {
            panic!("Collector already registered");
        }
        
        // Validate inputs
        common::validation::validate_collector_name(&name)
            .expect("Invalid name");
        common::validation::validate_phone_number(&phone)
            .expect("Invalid phone");
        
        // Create collector profile
        let collector_data = common::Collector {
            address: collector.clone(),
            name: name.clone(),
            phone,
            status: common::CollectorStatus::Pending,
            reputation_score: 500, // Start with neutral reputation
            total_collections: 0,
            total_weight: 0,
            registration_time: common::get_timestamp(&env),
            last_active: common::get_timestamp(&env),
        };
        
        // Save collector
        write_collector(&env, &collector, &collector_data);
        
        // Increment count
        let count = read_collector_count(&env);
        write_collector_count(&env, count + 1);
        
        // Emit event
        common::CollectorEvents::registered(&env, collector.clone(), name.clone());
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Get collector information
    /// 
    /// # Arguments
    /// * `collector` - Collector address
    /// 
    /// # Returns
    /// Collector data
    pub fn get_collector(env: Env, collector: Address) -> common::Collector {
        read_collector(&env, &collector)
            .expect("Collector not found")
    }

    /// Update collector status (admin only)
    /// 
    /// # Arguments
    /// * `collector` - Collector address
    /// * `status` - New status
    pub fn update_status(env: Env, collector: Address, status: common::CollectorStatus) {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        
        // Verify admin
        let admin = common::AccessControl::get_admin(&env)
            .expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin)
            .expect("Not admin");
        
        // Get collector
        let mut collector_data = read_collector(&env, &collector)
            .expect("Collector not found");
        
        // Validate status transition
        common::validation::validate_status_transition(&collector_data.status, &status)
            .expect("Invalid status transition");
        
        // Update status
        collector_data.status = status.clone();
        collector_data.last_active = common::get_timestamp(&env);
        
        // Save updated collector
        write_collector(&env, &collector, &collector_data);
        
        // Emit event
        common::CollectorEvents::status_updated(&env, collector.clone(), status.clone());
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Update collector profile (self only)
    /// 
    /// # Arguments
    /// * `collector` - Collector address
    /// * `name` - New name (optional)
    /// * `phone` - New phone (optional)
    pub fn update_profile(env: Env, collector: Address, name: String, phone: String) {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        common::Pausable::require_not_paused(&env)
            .expect("Contract paused");
        
        // Require authorization from collector
        collector.require_auth();
        
        // Get collector
        let mut collector_data = read_collector(&env, &collector)
            .expect("Collector not found");
        
        // Validate inputs
        common::validation::validate_collector_name(&name)
            .expect("Invalid name");
        common::validation::validate_phone_number(&phone)
            .expect("Invalid phone");
        
        // Update profile
        collector_data.name = name;
        collector_data.phone = phone;
        collector_data.last_active = common::get_timestamp(&env);
        
        // Save updated collector
        write_collector(&env, &collector, &collector_data);
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Update collector metrics (called by other contracts)
    /// 
    /// # Arguments
    /// * `collector` - Collector address
    /// * `weight` - Weight to add (in grams)
    pub fn update_metrics(env: Env, collector: Address, weight: u64) {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        
        // Get collector
        let mut collector_data = read_collector(&env, &collector)
            .expect("Collector not found");
        
        // Update metrics
        collector_data.total_collections = collector_data.total_collections.saturating_add(1);
        collector_data.total_weight = collector_data.total_weight.saturating_add(weight);
        collector_data.last_active = common::get_timestamp(&env);
        
        // Save updated collector
        write_collector(&env, &collector, &collector_data);
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Update collector reputation score (called by reputation contract)
    /// 
    /// # Arguments
    /// * `collector` - Collector address
    /// * `score` - New reputation score (0-1000)
    pub fn update_reputation(env: Env, collector: Address, score: u32) {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        
        // Validate score
        common::validation::validate_reputation_bounds(score)
            .expect("Invalid reputation score");
        
        // Get collector
        let mut collector_data = read_collector(&env, &collector)
            .expect("Collector not found");
        
        // Update reputation
        collector_data.reputation_score = score;
        collector_data.last_active = common::get_timestamp(&env);
        
        // Save updated collector
        write_collector(&env, &collector, &collector_data);
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Check if collector is active
    /// 
    /// # Arguments
    /// * `collector` - Collector address
    /// 
    /// # Returns
    /// True if collector is active
    pub fn is_active(env: Env, collector: Address) -> bool {
        if let Some(collector_data) = read_collector(&env, &collector) {
            collector_data.status == common::CollectorStatus::Active
        } else {
            false
        }
    }

    /// Check if collector exists
    /// 
    /// # Arguments
    /// * `collector` - Collector address
    /// 
    /// # Returns
    /// True if collector exists
    pub fn exists(env: Env, collector: Address) -> bool {
        has_collector(&env, &collector)
    }

    /// Get total number of registered collectors
    /// 
    /// # Returns
    /// Total collector count
    pub fn get_collector_count(env: Env) -> u64 {
        read_collector_count(&env)
    }

    /// Get all collectors (paginated)
    /// 
    /// # Arguments
    /// * `start` - Start index
    /// * `limit` - Maximum number to return
    /// 
    /// # Returns
    /// List of collector addresses
    pub fn get_all_collectors(env: Env, start: u64, limit: u64) -> Vec<Address> {
        read_all_collectors(&env, start, limit)
    }

    /// Pause the contract (admin only)
    pub fn pause(env: Env) {
        let admin = common::AccessControl::get_admin(&env)
            .expect("Admin not found");
        common::Pausable::admin_pause(&env, &admin)
            .expect("Not admin");
        
        common::AdminEvents::paused(&env);
    }

    /// Unpause the contract (admin only)
    pub fn unpause(env: Env) {
        let admin = common::AccessControl::get_admin(&env)
            .expect("Admin not found");
        common::Pausable::admin_unpause(&env, &admin)
            .expect("Not admin");
        
        common::AdminEvents::unpaused(&env);
    }

    /// Get admin address
    pub fn admin(env: Env) -> Address {
        common::AccessControl::get_admin(&env)
            .expect("Admin not found")
    }
}
