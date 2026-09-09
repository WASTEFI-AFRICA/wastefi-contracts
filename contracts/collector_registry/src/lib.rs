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
        common::Initializable::require_not_initialized(&env).expect("Already initialized");

        // Set admin
        common::AccessControl::set_admin(&env, admin);

        // Initialize collector count to 0
        write_collector_count(&env, 0);

        // Set initial version
        let initial_version = common::ContractVersion::new(1, 0, 0);
        common::Upgrade::set_version(&env, initial_version);

        // Set initial storage schema version
        common::StorageSchema::set_schema_version(&env, 1);

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
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Require authorization from collector
        collector.require_auth();

        // Check if collector already exists
        if has_collector(&env, &collector) {
            panic!("Collector already registered");
        }

        // Validate inputs
        common::validation::validate_collector_name(&name).expect("Invalid name");
        common::validation::validate_phone_number(&phone).expect("Invalid phone");

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
        read_collector(&env, &collector).expect("Collector not found")
    }

    /// Update collector status (admin only)
    ///
    /// # Arguments
    /// * `collector` - Collector address
    /// * `status` - New status
    pub fn update_status(env: Env, collector: Address, status: common::CollectorStatus) {
        common::Initializable::require_initialized(&env).expect("Not initialized");

        // Verify admin
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        // Get collector
        let mut collector_data = read_collector(&env, &collector).expect("Collector not found");

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
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Require authorization from collector
        collector.require_auth();

        // Get collector
        let mut collector_data = read_collector(&env, &collector).expect("Collector not found");

        // Validate inputs
        common::validation::validate_collector_name(&name).expect("Invalid name");
        common::validation::validate_phone_number(&phone).expect("Invalid phone");

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
        common::Initializable::require_initialized(&env).expect("Not initialized");

        // Get collector
        let mut collector_data = read_collector(&env, &collector).expect("Collector not found");

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
        common::Initializable::require_initialized(&env).expect("Not initialized");

        // Validate score
        common::validation::validate_reputation_bounds(score).expect("Invalid reputation score");

        // Get collector
        let mut collector_data = read_collector(&env, &collector).expect("Collector not found");

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

    /// Get admin address
    pub fn admin(env: Env) -> Address {
        common::AccessControl::get_admin(&env).expect("Admin not found")
    }

    /// Add operator (admin only)
    ///
    /// # Arguments
    /// * `operator` - Address to grant operator role
    pub fn add_operator(env: Env, operator: Address) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        common::AccessControl::add_operator(&env, operator.clone());

        // Log action
        common::AccessControl::log_admin_action(
            &env,
            &admin,
            soroban_sdk::String::from_str(&env, "add_operator"),
            Some(operator.to_string()),
        );

        common::bump_instance(&env);
    }

    /// Remove operator (admin only)
    ///
    /// # Arguments
    /// * `operator` - Address to revoke operator role
    pub fn remove_operator(env: Env, operator: Address) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        common::AccessControl::remove_operator(&env, &operator);

        // Log action
        common::AccessControl::log_admin_action(
            &env,
            &admin,
            soroban_sdk::String::from_str(&env, "remove_operator"),
            Some(operator.to_string()),
        );

        common::bump_instance(&env);
    }

    /// Check if address is operator
    ///
    /// # Arguments
    /// * `address` - Address to check
    ///
    /// # Returns
    /// True if address has operator role
    pub fn is_operator(env: Env, address: Address) -> bool {
        common::AccessControl::is_operator(&env, &address)
    }

    /// Get recent admin actions (audit log)
    ///
    /// # Arguments
    /// * `limit` - Maximum number of actions to return (max 50)
    ///
    /// # Returns
    /// Vector of (admin_address, action, timestamp, target) tuples
    pub fn get_admin_actions(
        env: Env,
        limit: u32,
    ) -> soroban_sdk::Vec<(
        Address,
        soroban_sdk::String,
        u64,
        Option<soroban_sdk::String>,
    )> {
        common::AccessControl::get_admin_actions(&env, limit)
    }

    /// Get pause history
    ///
    /// # Arguments
    /// * `limit` - Maximum number of events to return
    ///
    /// # Returns
    /// Vector of (action, timestamp) tuples
    pub fn get_pause_history(env: Env, limit: u32) -> soroban_sdk::Vec<(soroban_sdk::String, u64)> {
        common::Pausable::get_pause_history(&env, limit)
    }

    /// Get contract version
    ///
    /// # Returns
    /// Tuple of (major, minor, patch)
    pub fn get_version(env: Env) -> (u32, u32, u32) {
        if let Some(version) = common::Upgrade::get_version(&env) {
            version.to_tuple()
        } else {
            (1, 0, 0) // Default version
        }
    }

    /// Upgrade contract (admin only)
    ///
    /// # Arguments
    /// * `new_wasm_hash` - Hash of new WASM code
    pub fn upgrade(env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");

        // Mark upgrade in progress
        common::Upgrade::mark_upgrade_in_progress(&env);

        // Perform upgrade
        common::Upgrade::upgrade_contract(&env, &admin, new_wasm_hash);

        common::bump_instance(&env);
    }

    /// Complete upgrade (admin only, called after upgrade)
    ///
    /// # Arguments
    /// * `major` - Major version
    /// * `minor` - Minor version
    /// * `patch` - Patch version
    pub fn complete_upgrade(env: Env, major: u32, minor: u32, patch: u32) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        let new_version = common::ContractVersion::new(major, minor, patch);
        common::Upgrade::mark_upgrade_complete(&env, new_version);

        common::bump_instance(&env);
    }

    /// Check if upgrade is in progress
    ///
    /// # Returns
    /// True if upgrade is in progress
    pub fn is_upgrading(env: Env) -> bool {
        common::Upgrade::is_upgrade_in_progress(&env)
    }

    /// Get storage schema version
    ///
    /// # Returns
    /// Current storage schema version
    pub fn get_schema_version(env: Env) -> u32 {
        common::StorageSchema::get_schema_version(&env)
    }

    /// Enable feature (admin only)
    ///
    /// # Arguments
    /// * `feature` - Feature name
    pub fn enable_feature(env: Env, feature: soroban_sdk::String) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        common::Compatibility::enable_feature(&env, feature);
        common::bump_instance(&env);
    }

    /// Check if feature is supported
    ///
    /// # Arguments
    /// * `feature` - Feature name
    ///
    /// # Returns
    /// True if feature is supported
    pub fn is_feature_supported(env: Env, feature: soroban_sdk::String) -> bool {
        common::Compatibility::is_feature_supported(&env, feature)
    }

    /// Trigger emergency (admin only)
    ///
    /// # Arguments
    /// * `level` - Emergency level (0=Normal, 1=Warning, 2=Critical, 3=Shutdown)
    /// * `reason` - Reason for emergency
    pub fn trigger_emergency(env: Env, level: u32, reason: soroban_sdk::String) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");

        let emergency_level = match level {
            0 => common::EmergencyLevel::Normal,
            1 => common::EmergencyLevel::Warning,
            2 => common::EmergencyLevel::Critical,
            3 => common::EmergencyLevel::Shutdown,
            _ => panic!("Invalid emergency level"),
        };

        common::Emergency::trigger(&env, &admin, emergency_level, reason)
            .expect("Failed to trigger emergency");

        common::bump_instance(&env);
    }

    /// Resolve emergency (admin only)
    pub fn resolve_emergency(env: Env) {
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::Emergency::resolve(&env, &admin).expect("Failed to resolve emergency");
        common::bump_instance(&env);
    }

    /// Get current emergency level
    ///
    /// # Returns
    /// Emergency level (0=Normal, 1=Warning, 2=Critical, 3=Shutdown)
    pub fn get_emergency_level(env: Env) -> u32 {
        common::Emergency::get_level(&env) as u32
    }

    /// Get emergency event history
    ///
    /// # Arguments
    /// * `limit` - Maximum number of events to return
    ///
    /// # Returns
    /// Vector of (level, reason, triggered_by, timestamp, resolved) tuples
    pub fn get_emergency_history(
        env: Env,
        limit: u32,
    ) -> soroban_sdk::Vec<(u32, soroban_sdk::String, Address, u64, bool)> {
        common::Emergency::get_event_history(&env, limit)
    }

    /// Batch register multiple collectors (admin only)
    ///
    /// # Arguments
    /// * `collectors` - Vector of (address, name, phone) tuples
    ///
    /// # Returns
    /// Number of collectors successfully registered
    pub fn batch_register(
        env: Env,
        collectors: soroban_sdk::Vec<(Address, String, String)>,
    ) -> u32 {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Verify admin
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        let mut registered_count = 0u32;

        for i in 0..collectors.len() {
            if let Some((collector, name, phone)) = collectors.get(i) {
                // Skip if already registered
                if has_collector(&env, &collector) {
                    continue;
                }

                // Validate inputs
                if common::validation::validate_collector_name(&name).is_err()
                    || common::validation::validate_phone_number(&phone).is_err()
                {
                    continue;
                }

                // Create collector profile
                let collector_data = common::Collector {
                    address: collector.clone(),
                    name: name.clone(),
                    phone,
                    status: common::CollectorStatus::Pending,
                    reputation_score: 500,
                    total_collections: 0,
                    total_weight: 0,
                    registration_time: common::get_timestamp(&env),
                    last_active: common::get_timestamp(&env),
                };

                // Save collector
                write_collector(&env, &collector, &collector_data);
                registered_count += 1;

                // Emit event
                common::CollectorEvents::registered(&env, collector, name);
            }
        }

        // Update total count
        let count = read_collector_count(&env);
        write_collector_count(&env, count + registered_count as u64);

        // Bump storage
        common::bump_instance(&env);

        registered_count
    }

    /// Batch update collector statuses (admin only)
    ///
    /// # Arguments
    /// * `updates` - Vector of (collector_address, new_status) tuples
    ///
    /// # Returns
    /// Number of collectors successfully updated
    pub fn batch_update_status(
        env: Env,
        updates: soroban_sdk::Vec<(Address, common::CollectorStatus)>,
    ) -> u32 {
        common::Initializable::require_initialized(&env).expect("Not initialized");

        // Verify admin
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        let mut updated_count = 0u32;

        for i in 0..updates.len() {
            if let Some((collector, new_status)) = updates.get(i) {
                // Get collector
                if let Some(mut collector_data) = read_collector(&env, &collector) {
                    // Validate status transition
                    if common::validation::validate_status_transition(
                        &collector_data.status,
                        &new_status,
                    )
                    .is_ok()
                    {
                        // Update status
                        collector_data.status = new_status.clone();
                        collector_data.last_active = common::get_timestamp(&env);

                        // Save updated collector
                        write_collector(&env, &collector, &collector_data);
                        updated_count += 1;

                        // Emit event
                        common::CollectorEvents::status_updated(&env, collector, new_status);
                    }
                }
            }
        }

        // Bump storage
        common::bump_instance(&env);

        updated_count
    }

    /// Get multiple collectors at once (optimized batch query)
    ///
    /// # Arguments
    /// * `addresses` - Vector of collector addresses
    ///
    /// # Returns
    /// Vector of collector data (None for non-existent collectors)
    pub fn get_collectors_batch(
        env: Env,
        addresses: soroban_sdk::Vec<Address>,
    ) -> soroban_sdk::Vec<Option<common::Collector>> {
        let mut results = soroban_sdk::Vec::new(&env);

        for i in 0..addresses.len() {
            if let Some(addr) = addresses.get(i) {
                let collector = read_collector(&env, &addr);
                results.push_back(collector);
            }
        }

        results
    }

    /// Get collectors by status (filtered query)
    ///
    /// # Arguments
    /// * `status` - Filter by collector status
    /// * `limit` - Maximum number of results (max 100)
    ///
    /// # Returns
    /// Vector of collectors matching the status
    pub fn get_collectors_by_status(
        env: Env,
        status: common::CollectorStatus,
        limit: u64,
    ) -> soroban_sdk::Vec<common::Collector> {
        let max_limit = if limit > 100 { 100 } else { limit };
        let mut results = soroban_sdk::Vec::new(&env);
        let mut found = 0u64;

        // Get all collectors and filter
        let all_addresses = read_all_collectors(&env, 0, 1000);

        for i in 0..all_addresses.len() {
            if found >= max_limit {
                break;
            }

            if let Some(addr) = all_addresses.get(i) {
                if let Some(collector) = read_collector(&env, &addr) {
                    if collector.status == status {
                        results.push_back(collector);
                        found += 1;
                    }
                }
            }
        }

        results
    }

    /// Get top collectors by total weight (leaderboard)
    ///
    /// # Arguments
    /// * `limit` - Number of top collectors to return (max 50)
    ///
    /// # Returns
    /// Vector of collectors sorted by total weight (descending)
    ///
    /// # Note
    /// This is a simplified implementation. For production, use indexed storage
    pub fn get_top_collectors_by_weight(
        env: Env,
        limit: u64,
    ) -> soroban_sdk::Vec<common::Collector> {
        let max_limit = if limit > 50 { 50 } else { limit };
        let mut collectors = soroban_sdk::Vec::new(&env);

        // Get all active collectors
        let all_addresses = read_all_collectors(&env, 0, 500);

        for i in 0..all_addresses.len() {
            if let Some(addr) = all_addresses.get(i) {
                if let Some(collector) = read_collector(&env, &addr) {
                    if collector.status == common::CollectorStatus::Active {
                        collectors.push_back(collector);
                    }
                }
            }
        }

        // Simple bubble sort by total_weight (descending)
        // Note: For production, consider more efficient sorting or pre-sorted indexes
        for i in 0..collectors.len() {
            for j in (i + 1)..collectors.len() {
                if let (Some(mut a), Some(b)) = (collectors.get(i), collectors.get(j)) {
                    if b.total_weight > a.total_weight {
                        // Swap
                        let temp = a.clone();
                        a = b.clone();
                        collectors.set(i, a);
                        collectors.set(j, temp);
                    }
                }
            }
        }

        // Return top N
        let mut top = soroban_sdk::Vec::new(&env);
        for i in 0..max_limit.min(collectors.len() as u64) {
            if let Some(collector) = collectors.get(i as u32) {
                top.push_back(collector);
            }
        }

        top
    }

    /// Get collectors registered in a time range
    ///
    /// # Arguments
    /// * `start_time` - Start timestamp
    /// * `end_time` - End timestamp
    /// * `limit` - Maximum results (max 100)
    ///
    /// # Returns
    /// Vector of collectors registered within the time range
    pub fn get_collectors_by_reg_time(
        env: Env,
        start_time: u64,
        end_time: u64,
        limit: u64,
    ) -> soroban_sdk::Vec<common::Collector> {
        let max_limit = if limit > 100 { 100 } else { limit };
        let mut results = soroban_sdk::Vec::new(&env);
        let mut found = 0u64;

        let all_addresses = read_all_collectors(&env, 0, 1000);

        for i in 0..all_addresses.len() {
            if found >= max_limit {
                break;
            }

            if let Some(addr) = all_addresses.get(i) {
                if let Some(collector) = read_collector(&env, &addr) {
                    if collector.registration_time >= start_time
                        && collector.registration_time <= end_time
                    {
                        results.push_back(collector);
                        found += 1;
                    }
                }
            }
        }

        results
    }

    /// Get aggregate statistics for all collectors
    ///
    /// # Returns
    /// Tuple of (total_collectors, total_weight, total_collections)
    pub fn get_global_statistics(env: Env) -> (u64, u64, u64) {
        let mut total_weight = 0u64;
        let mut total_collections = 0u64;

        let all_addresses = read_all_collectors(&env, 0, 1000);

        for i in 0..all_addresses.len() {
            if let Some(addr) = all_addresses.get(i) {
                if let Some(collector) = read_collector(&env, &addr) {
                    total_weight = total_weight.saturating_add(collector.total_weight);
                    total_collections =
                        total_collections.saturating_add(collector.total_collections);
                }
            }
        }

        let total_collectors = read_collector_count(&env);

        (total_collectors, total_weight, total_collections)
    }

    /// Search collectors by name (partial match)
    ///
    /// # Arguments
    /// * `_search_term` - Partial name to search for (placeholder)
    /// * `limit` - Maximum results (max 50)
    ///
    /// # Returns
    /// Vector of collectors with matching names
    ///
    /// # Note
    /// This is a placeholder. Soroban String doesn't have substring search yet.
    /// Returns first N collectors as placeholder implementation.
    pub fn search_collectors_by_name(
        env: Env,
        _search_term: String,
        limit: u64,
    ) -> soroban_sdk::Vec<common::Collector> {
        let max_limit = if limit > 50 { 50 } else { limit };
        let mut results = soroban_sdk::Vec::new(&env);
        let mut found = 0u64;

        let all_addresses = read_all_collectors(&env, 0, 500);

        for i in 0..all_addresses.len() {
            if found >= max_limit {
                break;
            }

            if let Some(addr) = all_addresses.get(i) {
                if let Some(collector) = read_collector(&env, &addr) {
                    // Simple contains check (case-sensitive)
                    // Note: Soroban String doesn't have built-in substring search
                    // This is a placeholder for the concept
                    results.push_back(collector);
                    found += 1;
                }
            }
        }

        results
    }
}
