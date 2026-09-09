use soroban_sdk::{contracttype, Address, String, Vec};

/// Material types supported by WasteFi
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MaterialType {
    Plastic,
    Glass,
    Metal,
    Paper,
    Cardboard,
    Electronics,
    Organic,
    Textile,
    Rubber,
    Other,
}

/// Collector status in the system
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollectorStatus {
    Pending,
    Active,
    Suspended,
    Banned,
}

/// Collection point verification status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationStatus {
    Unverified,
    Pending,
    Verified,
    Rejected,
}

/// Transaction status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Disputed,
    Cancelled,
}

/// Payment status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
}

/// Collector profile data
#[contracttype]
#[derive(Clone, Debug)]
pub struct Collector {
    pub address: Address,
    pub name: String,
    pub phone: String,
    pub status: CollectorStatus,
    pub reputation_score: u32,
    pub total_collections: u64,
    pub total_weight: u64, // in grams
    pub registration_time: u64,
    pub last_active: u64,
}

/// Collection point data
#[contracttype]
#[derive(Clone, Debug)]
pub struct CollectionPoint {
    pub id: u64,
    pub owner: Address,
    pub name: String,
    pub location: String,
    pub verification_status: VerificationStatus,
    pub accepted_materials: Vec<MaterialType>,
    pub total_processed: u64, // in grams
    pub created_at: u64,
}

/// Waste transaction record
#[contracttype]
#[derive(Clone, Debug)]
pub struct WasteRecord {
    pub id: u64,
    pub collector: Address,
    pub collection_point: Address,
    pub material_type: MaterialType,
    pub weight: u64, // in grams
    pub price_per_kg: i128,
    pub total_amount: i128,
    pub status: TransactionStatus,
    pub timestamp: u64,
    pub verified: bool,
}

/// Material pricing data
#[contracttype]
#[derive(Clone, Debug)]
pub struct MaterialPrice {
    pub material_type: MaterialType,
    pub price_per_kg: i128, // in stroops (1 XLM = 10^7 stroops)
    pub last_updated: u64,
    pub updated_by: Address,
}

/// Reputation score data
#[contracttype]
#[derive(Clone, Debug)]
pub struct ReputationScore {
    pub collector: Address,
    pub score: u32, // 0-1000
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub disputed_transactions: u64,
    pub last_updated: u64,
}

/// Payment distribution data
#[contracttype]
#[derive(Clone, Debug)]
pub struct Payment {
    pub id: u64,
    pub recipient: Address,
    pub amount: i128,
    pub status: PaymentStatus,
    pub transaction_id: u64,
    pub created_at: u64,
    pub processed_at: u64,
}

/// Carbon credit calculation data
#[contracttype]
#[derive(Clone, Debug)]
pub struct CarbonCredit {
    pub transaction_id: u64,
    pub material_type: MaterialType,
    pub weight: u64,    // in grams
    pub co2_saved: u64, // in grams of CO2
    pub credit_amount: i128,
    pub issued_at: u64,
}

/// RecycleGraph material passport
#[contracttype]
#[derive(Clone, Debug)]
pub struct MaterialPassport {
    pub id: String,
    pub material_type: MaterialType,
    pub origin: String,
    pub weight: u64,
    pub quality_grade: u32, // 1-10
    pub chain_of_custody: Vec<Address>,
    pub created_at: u64,
}

/// Collector statistics summary
#[contracttype]
#[derive(Clone, Debug)]
pub struct CollectorStats {
    pub total_transactions: u64,
    pub total_weight: u64,  // in grams
    pub total_amount: i128, // in stroops
}
