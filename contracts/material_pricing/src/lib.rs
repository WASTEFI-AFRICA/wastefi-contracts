#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

use common::types::*;

#[cfg(test)]
mod test;

#[contract]
pub struct MaterialPricing;

#[contractimpl]
impl MaterialPricing {
    /// Initialize the material pricing contract
    ///
    /// # Arguments
    /// * `admin` - Contract administrator address
    pub fn initialize(env: Env, admin: Address) {
        common::Initializable::require_not_initialized(&env).expect("Already initialized");

        // Set admin
        common::AccessControl::set_admin(&env, admin.clone());

        // Set default prices for all material types (in stroops, 1 XLM = 10^7 stroops)
        Self::set_default_prices(&env, &admin);

        // Mark as initialized
        common::Initializable::mark_initialized(&env);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Set default prices for all material types
    fn set_default_prices(env: &Env, admin: &Address) {
        let timestamp = env.ledger().timestamp();

        // Default prices per kg in stroops (1 XLM = 10,000,000 stroops)
        let default_prices = [
            (MaterialType::Plastic, 8_000_000),      // 0.8 XLM/kg
            (MaterialType::Glass, 5_000_000),        // 0.5 XLM/kg
            (MaterialType::Metal, 15_000_000),       // 1.5 XLM/kg
            (MaterialType::Paper, 4_000_000),        // 0.4 XLM/kg
            (MaterialType::Cardboard, 3_000_000),    // 0.3 XLM/kg
            (MaterialType::Electronics, 25_000_000), // 2.5 XLM/kg
            (MaterialType::Organic, 2_000_000),      // 0.2 XLM/kg
            (MaterialType::Textile, 6_000_000),      // 0.6 XLM/kg
            (MaterialType::Rubber, 7_000_000),       // 0.7 XLM/kg
            (MaterialType::Other, 3_000_000),        // 0.3 XLM/kg
        ];

        for (material_type, price) in default_prices.iter() {
            let price_record = MaterialPrice {
                material_type: material_type.clone(),
                price_per_kg: *price,
                last_updated: timestamp,
                updated_by: admin.clone(),
            };

            let key = Self::get_price_key(material_type);
            env.storage().persistent().set(&key, &price_record);
            common::bump_persistent(env, &key);
        }
    }

    /// Get storage key for a material type
    fn get_price_key(material_type: &MaterialType) -> common::StorageKey {
        let type_id = Self::material_type_to_u32(material_type);
        common::StorageKey::MaterialPrice(type_id)
    }

    /// Convert MaterialType to u32 for storage key
    fn material_type_to_u32(material_type: &MaterialType) -> u32 {
        match material_type {
            MaterialType::Plastic => 0,
            MaterialType::Glass => 1,
            MaterialType::Metal => 2,
            MaterialType::Paper => 3,
            MaterialType::Cardboard => 4,
            MaterialType::Electronics => 5,
            MaterialType::Organic => 6,
            MaterialType::Textile => 7,
            MaterialType::Rubber => 8,
            MaterialType::Other => 9,
        }
    }

    /// Set price for a material type (admin only)
    ///
    /// # Arguments
    /// * `material_type` - Type of material
    /// * `price_per_kg` - Price per kilogram in stroops
    pub fn set_price(env: Env, material_type: MaterialType, price_per_kg: i128) {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        // Validate price
        common::validation::validate_price(price_per_kg).expect("Invalid price");

        // Create price record
        let price_record = MaterialPrice {
            material_type: material_type.clone(),
            price_per_kg,
            last_updated: env.ledger().timestamp(),
            updated_by: admin.clone(),
        };

        // Store price
        let key = Self::get_price_key(&material_type);
        env.storage().persistent().set(&key, &price_record);
        common::bump_persistent(&env, &key);

        // Emit event
        common::PricingEvents::price_updated(&env, material_type, price_per_kg);

        // Bump storage
        common::bump_instance(&env);
    }

    /// Get price for a material type
    ///
    /// # Arguments
    /// * `material_type` - Type of material
    ///
    /// # Returns
    /// Price per kilogram in stroops
    pub fn get_price(env: Env, material_type: MaterialType) -> i128 {
        let key = Self::get_price_key(&material_type);
        let price_record: MaterialPrice = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Price not found");
        price_record.price_per_kg
    }

    /// Get material price record with metadata
    ///
    /// # Arguments
    /// * `material_type` - Type of material
    ///
    /// # Returns
    /// Full MaterialPrice record
    pub fn get_price_record(env: Env, material_type: MaterialType) -> MaterialPrice {
        let key = Self::get_price_key(&material_type);
        env.storage()
            .persistent()
            .get(&key)
            .expect("Price not found")
    }

    /// Get all material prices
    ///
    /// # Returns
    /// Vector of all material price records
    pub fn get_all_prices(env: Env) -> Vec<MaterialPrice> {
        let mut prices = Vec::new(&env);

        let material_types = [
            MaterialType::Plastic,
            MaterialType::Glass,
            MaterialType::Metal,
            MaterialType::Paper,
            MaterialType::Cardboard,
            MaterialType::Electronics,
            MaterialType::Organic,
            MaterialType::Textile,
            MaterialType::Rubber,
            MaterialType::Other,
        ];

        for material_type in material_types.iter() {
            let key = Self::get_price_key(material_type);
            if let Some(price_record) = env.storage().persistent().get::<_, MaterialPrice>(&key) {
                prices.push_back(price_record);
            }
        }

        prices
    }

    /// Batch update prices (admin only)
    ///
    /// # Arguments
    /// * `prices` - Vector of material types and their new prices
    pub fn batch_update_prices(env: Env, prices: Vec<(MaterialType, i128)>) {
        common::Initializable::require_initialized(&env).expect("Not initialized");
        common::Pausable::require_not_paused(&env).expect("Contract paused");

        // Get admin and verify
        let admin = common::AccessControl::get_admin(&env).expect("Admin not found");
        common::AccessControl::require_admin(&env, &admin).expect("Not admin");

        for i in 0..prices.len() {
            if let Some((material_type, price_per_kg)) = prices.get(i) {
                // Validate price
                common::validation::validate_price(price_per_kg).expect("Invalid price");

                // Create and store price record
                let price_record = MaterialPrice {
                    material_type: material_type.clone(),
                    price_per_kg,
                    last_updated: env.ledger().timestamp(),
                    updated_by: admin.clone(),
                };

                let key = Self::get_price_key(&material_type);
                env.storage().persistent().set(&key, &price_record);
                common::bump_persistent(&env, &key);

                // Emit event
                common::PricingEvents::price_updated(&env, material_type, price_per_kg);
            }
        }

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

    /// Get prices for multiple material types (optimized batch query)
    ///
    /// # Arguments
    /// * `material_types` - Vector of material types
    ///
    /// # Returns
    /// Vector of prices (in same order as input)
    pub fn get_prices_batch(env: Env, material_types: Vec<MaterialType>) -> Vec<i128> {
        let mut prices = Vec::new(&env);

        for i in 0..material_types.len() {
            if let Some(material_type) = material_types.get(i) {
                let price = Self::get_price(env.clone(), material_type);
                prices.push_back(price);
            }
        }

        prices
    }

    /// Get price records for multiple material types (optimized batch query with metadata)
    ///
    /// # Arguments
    /// * `material_types` - Vector of material types
    ///
    /// # Returns
    /// Vector of MaterialPrice records
    pub fn get_price_records_batch(
        env: Env,
        material_types: Vec<MaterialType>,
    ) -> Vec<MaterialPrice> {
        let mut records = Vec::new(&env);

        for i in 0..material_types.len() {
            if let Some(material_type) = material_types.get(i) {
                let record = Self::get_price_record(env.clone(), material_type);
                records.push_back(record);
            }
        }

        records
    }
}
