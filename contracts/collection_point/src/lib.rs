#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

mod storage;
use storage::*;

#[cfg(test)]
mod test;

#[contract]
pub struct CollectionPoint;

#[contractimpl]
impl CollectionPoint {
    /// Initialize the collection point registry
    /// 
    /// # Arguments
    /// * `admin` - Contract administrator address
    pub fn initialize(env: Env, admin: Address) {
        common::Initializable::require_not_initialized(&env)
            .expect("Already initialized");
        
        // Set admin
        common::AccessControl::set_admin(&env, admin);
        
        // Initialize point count to 0
        write_point_count(&env, 0);
        
        // Mark as initialized
        common::Initializable::mark_initialized(&env);
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Register a new collection point
    /// 
    /// # Arguments
    /// * `owner` - Collection point owner address
    /// * `name` - Point name
    /// * `location` - Location description
    /// * `accepted_materials` - List of accepted material types
    /// 
    /// # Returns
    /// Collection point ID
    pub fn register_point(
        env: Env,
        owner: Address,
        name: String,
        location: String,
        accepted_materials: Vec<common::MaterialType>,
    ) -> u64 {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        common::Pausable::require_not_paused(&env)
            .expect("Contract paused");
        
        // Require authorization from owner
        owner.require_auth();
        
        // Validate inputs
        common::validation::validate_collection_point_name(&name)
            .expect("Invalid name");
        common::validation::validate_location(&location)
            .expect("Invalid location");
        
        if accepted_materials.is_empty() {
            panic!("Must accept at least one material type");
        }
        
        // Generate new point ID
        let point_id = read_point_count(&env) + 1;
        
        // Create collection point
        let point_data = common::CollectionPoint {
            id: point_id,
            owner: owner.clone(),
            name: name.clone(),
            location,
            verification_status: common::VerificationStatus::Unverified,
            accepted_materials,
            total_processed: 0,
            created_at: common::get_timestamp(&env),
        };
        
        // Save point
        write_point(&env, point_id, &point_data);
        write_point_by_owner(&env, &owner, point_id);
        
        // Update count
        write_point_count(&env, point_id);
        
        // Emit event
        common::CollectionPointEvents::registered(&env, &point_id, &owner);
        
        // Bump storage
        common::bump_instance(&env);
        
        point_id
    }

    /// Get collection point details
    /// 
    /// # Arguments
    /// * `point_id` - Collection point ID
    /// 
    /// # Returns
    /// Collection point data
    pub fn get_point(env: Env, point_id: u64) -> common::CollectionPoint {
        read_point(&env, point_id)
            .expect("Collection point not found")
    }

    /// Get collection point by owner
    /// 
    /// # Arguments
    /// * `owner` - Owner address
    /// 
    /// # Returns
    /// Collection point ID
    pub fn get_point_by_owner(env: Env, owner: Address) -> u64 {
        read_point_by_owner(&env, &owner)
            .expect("No collection point found for owner")
    }

    /// Verify a collection point (admin only)
    /// 
    /// # Arguments
    /// * `point_id` - Collection point ID
    pub fn verify_point(env: Env, point_id: u64) {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        
        // Verify admin
        let admin = common::AccessControl::get_admin(&env)
            .expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin)
            .expect("Not admin");
        
        // Get point
        let mut point_data = read_point(&env, point_id)
            .expect("Collection point not found");
        
        // Update to verified
        point_data.verification_status = common::VerificationStatus::Verified;
        
        // Save updated point
        write_point(&env, point_id, &point_data);
        
        // Emit event
        common::CollectionPointEvents::verified(&env, &point_id);
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Update verification status (admin only)
    /// 
    /// # Arguments
    /// * `point_id` - Collection point ID
    /// * `status` - New verification status
    pub fn update_verification_status(
        env: Env,
        point_id: u64,
        status: common::VerificationStatus,
    ) {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        
        // Verify admin
        let admin = common::AccessControl::get_admin(&env)
            .expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin)
            .expect("Not admin");
        
        // Get point
        let mut point_data = read_point(&env, point_id)
            .expect("Collection point not found");
        
        // Validate transition
        common::validation::validate_verification_transition(
            &point_data.verification_status,
            &status,
        )
        .expect("Invalid verification transition");
        
        // Update status
        point_data.verification_status = status;
        
        // Save updated point
        write_point(&env, point_id, &point_data);
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Update collection point info (owner only)
    /// 
    /// # Arguments
    /// * `point_id` - Collection point ID
    /// * `name` - New name
    /// * `location` - New location
    /// * `accepted_materials` - New accepted materials list
    pub fn update_point(
        env: Env,
        point_id: u64,
        name: String,
        location: String,
        accepted_materials: Vec<common::MaterialType>,
    ) {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        common::Pausable::require_not_paused(&env)
            .expect("Contract paused");
        
        // Get point
        let mut point_data = read_point(&env, point_id)
            .expect("Collection point not found");
        
        // Require authorization from owner
        point_data.owner.require_auth();
        
        // Validate inputs
        common::validation::validate_collection_point_name(&name)
            .expect("Invalid name");
        common::validation::validate_location(&location)
            .expect("Invalid location");
        
        if accepted_materials.is_empty() {
            panic!("Must accept at least one material type");
        }
        
        // Update point
        point_data.name = name;
        point_data.location = location;
        point_data.accepted_materials = accepted_materials;
        
        // Save updated point
        write_point(&env, point_id, &point_data);
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Update processed weight (called by transaction contract)
    /// 
    /// # Arguments
    /// * `point_id` - Collection point ID
    /// * `weight` - Weight to add (in grams)
    pub fn update_processed(env: Env, point_id: u64, weight: u64) {
        common::Initializable::require_initialized(&env)
            .expect("Not initialized");
        
        // Get point
        let mut point_data = read_point(&env, point_id)
            .expect("Collection point not found");
        
        // Update total processed
        point_data.total_processed = point_data.total_processed.saturating_add(weight);
        
        // Save updated point
        write_point(&env, point_id, &point_data);
        
        // Bump storage
        common::bump_instance(&env);
    }

    /// Check if collection point is verified
    /// 
    /// # Arguments
    /// * `point_id` - Collection point ID
    /// 
    /// # Returns
    /// True if verified
    pub fn is_verified(env: Env, point_id: u64) -> bool {
        if let Some(point_data) = read_point(&env, point_id) {
            point_data.verification_status == common::VerificationStatus::Verified
        } else {
            false
        }
    }

    /// Check if material type is accepted
    /// 
    /// # Arguments
    /// * `point_id` - Collection point ID
    /// * `material_type` - Material type to check
    /// 
    /// # Returns
    /// True if material is accepted
    pub fn accepts_material(env: Env, point_id: u64, material_type: common::MaterialType) -> bool {
        if let Some(point_data) = read_point(&env, point_id) {
            for material in point_data.accepted_materials.iter() {
                if material == material_type {
                    return true;
                }
            }
        }
        false
    }

    /// Check if collection point exists
    /// 
    /// # Arguments
    /// * `point_id` - Collection point ID
    /// 
    /// # Returns
    /// True if exists
    pub fn exists(env: Env, point_id: u64) -> bool {
        has_point(&env, point_id)
    }

    /// Get total number of collection points
    /// 
    /// # Returns
    /// Total point count
    pub fn get_point_count(env: Env) -> u64 {
        read_point_count(&env)
    }

    /// Get all collection points (paginated)
    /// 
    /// # Arguments
    /// * `start` - Start index (1-based)
    /// * `limit` - Maximum number to return
    /// 
    /// # Returns
    /// List of collection point IDs
    pub fn get_all_points(env: Env, start: u64, limit: u64) -> Vec<u64> {
        let mut result = Vec::new(&env);
        let total = read_point_count(&env);
        let end = (start + limit - 1).min(total);
        
        for id in start..=end {
            if has_point(&env, id) {
                result.push_back(id);
            }
        }
        
        result
    }

    /// Get verified collection points (paginated)
    /// 
    /// # Arguments
    /// * `start` - Start index (1-based)
    /// * `limit` - Maximum number to return
    /// 
    /// # Returns
    /// List of verified collection point IDs
    pub fn get_verified_points(env: Env, start: u64, limit: u64) -> Vec<u64> {
        let mut result = Vec::new(&env);
        let total = read_point_count(&env);
        let mut found = 0u64;
        let mut skipped = 0u64;
        
        for id in 1..=total {
            if let Some(point) = read_point(&env, id) {
                if point.verification_status == common::VerificationStatus::Verified {
                    if skipped >= start - 1 {
                        result.push_back(id);
                        found += 1;
                        if found >= limit {
                            break;
                        }
                    } else {
                        skipped += 1;
                    }
                }
            }
        }
        
        result
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
