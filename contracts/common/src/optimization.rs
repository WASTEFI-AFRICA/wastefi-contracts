use soroban_sdk::Env;

/// Storage optimization utilities
pub struct StorageOptimization;

impl StorageOptimization {
    /// Get recommended storage type for data category
    pub fn get_storage_recommendation(data_category: StorageCategory) -> StorageType {
        match data_category {
            StorageCategory::Configuration => StorageType::Instance,
            StorageCategory::UserData => StorageType::Persistent,
            StorageCategory::TemporaryCache => StorageType::Temporary,
            StorageCategory::RateLimiting => StorageType::Temporary,
            StorageCategory::SessionData => StorageType::Temporary,
        }
    }

    /// Calculate optimal TTL for persistent storage
    pub fn calculate_optimal_ttl(data_type: DataType) -> (u32, u32) {
        match data_type {
            DataType::CriticalRecord => (518400, 31536000), // 6 days - 1 year
            DataType::StandardRecord => (86400, 2592000),   // 1 day - 30 days
            DataType::CachedData => (3600, 86400),          // 1 hour - 1 day
        }
    }

    /// Check if collection needs pruning
    pub fn needs_pruning(collection_len: u32, max_size: u32) -> bool {
        collection_len > max_size
    }

    /// Get recommended pruning strategy
    /// Returns (keep_recent, start_index) for slicing
    pub fn get_pruning_strategy(current_len: u32, target_size: u32) -> (bool, u32) {
        if current_len <= target_size {
            (true, 0)
        } else {
            // Keep most recent items
            let start_index = current_len - target_size;
            (true, start_index)
        }
    }

    /// Estimate storage cost (relative units)
    pub fn estimate_storage_cost(
        storage_type: StorageType,
        data_size_bytes: u32,
        duration_seconds: u32,
    ) -> u32 {
        let base_cost = match storage_type {
            StorageType::Instance => data_size_bytes,
            StorageType::Persistent => data_size_bytes * 2,
            StorageType::Temporary => data_size_bytes / 2,
        };

        // Factor in duration for persistent/temporary
        match storage_type {
            StorageType::Persistent | StorageType::Temporary => {
                base_cost * (duration_seconds / 86400) // Cost per day
            }
            StorageType::Instance => base_cost,
        }
    }
}

/// Storage category classification
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageCategory {
    Configuration,  // Contract settings
    UserData,       // User profiles, records
    TemporaryCache, // Short-lived computed data
    RateLimiting,   // Throttling counters
    SessionData,    // Active session info
}

/// Storage type recommendation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageType {
    Instance,   // Contract-lifetime
    Persistent, // Long-term with TTL
    Temporary,  // Short-lived with TTL
}

/// Data type for TTL calculation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataType {
    CriticalRecord, // Must persist long-term
    StandardRecord, // Normal persistence
    CachedData,     // Can expire quickly
}

/// Performance monitoring utilities
pub struct PerformanceMonitoring;

impl PerformanceMonitoring {
    /// Check if operation should use batch processing
    pub fn should_use_batch(item_count: u32) -> bool {
        item_count >= 3 // Batch processing efficient for 3+ items
    }

    /// Calculate recommended batch size
    pub fn recommended_batch_size(total_items: u32) -> u32 {
        if total_items < 10 {
            total_items
        } else if total_items < 100 {
            20
        } else {
            50 // Max batch size for safety
        }
    }

    /// Estimate gas savings from batching
    pub fn estimate_batch_savings(individual_cost: u32, batch_overhead: u32, count: u32) -> u32 {
        let individual_total = individual_cost * count;
        let batch_total = batch_overhead + (individual_cost / 2) * count;
        individual_total.saturating_sub(batch_total)
    }
}

/// Caching utilities
pub struct CacheOptimization;

impl CacheOptimization {
    /// Check if value should be cached
    pub fn should_cache(
        computation_cost: ComputationCost,
        access_frequency: AccessFrequency,
    ) -> bool {
        matches!(
            (computation_cost, access_frequency),
            (ComputationCost::High, AccessFrequency::High)
                | (ComputationCost::High, AccessFrequency::Medium)
                | (ComputationCost::Medium, AccessFrequency::High)
        )
    }

    /// Calculate cache TTL based on data volatility
    pub fn calculate_cache_ttl(volatility: DataVolatility) -> u64 {
        match volatility {
            DataVolatility::Static => 86400, // 24 hours
            DataVolatility::Stable => 3600,  // 1 hour
            DataVolatility::Dynamic => 300,  // 5 minutes
            DataVolatility::Volatile => 60,  // 1 minute
        }
    }

    /// Check if cache entry is still valid
    pub fn is_cache_valid(env: &Env, cached_at: u64, ttl: u64) -> bool {
        let current_time = env.ledger().timestamp();
        current_time.saturating_sub(cached_at) < ttl
    }
}

/// Computation cost classification
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComputationCost {
    Low,    // Simple arithmetic
    Medium, // Multiple storage reads
    High,   // Complex calculations or many operations
}

/// Access frequency classification
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessFrequency {
    Low,    // Rarely accessed
    Medium, // Occasionally accessed
    High,   // Frequently accessed
}

/// Data volatility classification
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DataVolatility {
    Static,   // Never changes
    Stable,   // Changes infrequently
    Dynamic,  // Changes regularly
    Volatile, // Changes frequently
}

/// Data structure optimization hints
pub struct StructureOptimization;

impl StructureOptimization {
    /// Check if struct can be packed better
    pub fn check_struct_packing(field_sizes: &[u32]) -> PackingRecommendation {
        let total_size: u32 = field_sizes.iter().sum();
        let field_count = field_sizes.len() as u32;

        // Check for alignment waste
        let has_small_fields = field_sizes.iter().any(|&size| size <= 4);
        let has_large_fields = field_sizes.iter().any(|&size| size > 8);

        if has_small_fields && has_large_fields {
            PackingRecommendation::Reorder
        } else if field_count > 10 {
            PackingRecommendation::Split
        } else if total_size > 256 {
            PackingRecommendation::Compress
        } else {
            PackingRecommendation::Optimal
        }
    }
}

/// Struct packing recommendation
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackingRecommendation {
    Optimal,  // Already well-packed
    Reorder,  // Reorder fields for better alignment
    Split,    // Split into multiple structs
    Compress, // Use more compact types
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_recommendations() {
        assert_eq!(
            StorageOptimization::get_storage_recommendation(StorageCategory::Configuration),
            StorageType::Instance
        );
        assert_eq!(
            StorageOptimization::get_storage_recommendation(StorageCategory::TemporaryCache),
            StorageType::Temporary
        );
    }

    #[test]
    fn test_ttl_calculation() {
        let (min, max) = StorageOptimization::calculate_optimal_ttl(DataType::CriticalRecord);
        assert_eq!(min, 518400); // 6 days
        assert_eq!(max, 31536000); // 1 year
    }

    #[test]
    fn test_pruning_detection() {
        assert!(!StorageOptimization::needs_pruning(50, 100));
        assert!(StorageOptimization::needs_pruning(150, 100));

        let (keep_recent, start) = StorageOptimization::get_pruning_strategy(150, 100);
        assert!(keep_recent);
        assert_eq!(start, 50); // Start at index 50 to keep last 100 items
    }

    #[test]
    fn test_batch_recommendation() {
        assert!(!PerformanceMonitoring::should_use_batch(2));
        assert!(PerformanceMonitoring::should_use_batch(3));
        assert!(PerformanceMonitoring::should_use_batch(10));
    }

    #[test]
    fn test_batch_size_calculation() {
        assert_eq!(PerformanceMonitoring::recommended_batch_size(5), 5);
        assert_eq!(PerformanceMonitoring::recommended_batch_size(50), 20);
        assert_eq!(PerformanceMonitoring::recommended_batch_size(200), 50);
    }

    #[test]
    fn test_cache_decision() {
        assert!(CacheOptimization::should_cache(
            ComputationCost::High,
            AccessFrequency::High
        ));
        assert!(!CacheOptimization::should_cache(
            ComputationCost::Low,
            AccessFrequency::Low
        ));
    }

    #[test]
    fn test_cache_ttl() {
        assert_eq!(
            CacheOptimization::calculate_cache_ttl(DataVolatility::Static),
            86400
        );
        assert_eq!(
            CacheOptimization::calculate_cache_ttl(DataVolatility::Volatile),
            60
        );
    }

    #[test]
    fn test_cache_validity() {
        let env = Env::default();
        let cached_at = env.ledger().timestamp();

        assert!(CacheOptimization::is_cache_valid(&env, cached_at, 3600));
        assert!(!CacheOptimization::is_cache_valid(
            &env,
            cached_at - 7200,
            3600
        ));
    }

    #[test]
    fn test_storage_cost_estimation() {
        let cost_instance =
            StorageOptimization::estimate_storage_cost(StorageType::Instance, 100, 86400);
        let cost_temporary =
            StorageOptimization::estimate_storage_cost(StorageType::Temporary, 100, 86400);

        assert!(cost_instance > cost_temporary);
    }

    #[test]
    fn test_struct_packing_check() {
        // Well-packed struct
        let optimal = vec![8, 8, 8, 4];
        assert_eq!(
            StructureOptimization::check_struct_packing(&optimal),
            PackingRecommendation::Optimal
        );

        // Needs reordering
        let mixed = vec![1, 8, 2, 8, 1];
        assert_eq!(
            StructureOptimization::check_struct_packing(&mixed),
            PackingRecommendation::Reorder
        );
    }
}
