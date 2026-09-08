use soroban_sdk::{contracttype, Address, String};

/// Storage keys for WasteFi contracts
#[contracttype]
#[derive(Clone, Debug)]
pub enum StorageKey {
    // Admin and configuration
    Admin,
    Initialized,
    Paused,
    
    // Collector registry keys
    Collector(Address),
    CollectorCount,
    
    // Collection point keys
    CollectionPoint(u64),
    CollectionPointByOwner(Address),
    CollectionPointCount,
    
    // Transaction keys
    WasteRecord(u64),
    TransactionsByCollector(Address, u64),
    TransactionCount,
    
    // Payment keys
    Payment(u64),
    PaymentsByRecipient(Address, u64),
    PaymentCount,
    
    // Reputation keys
    Reputation(Address),
    
    // Material pricing keys
    MaterialPrice(u8), // MaterialType as u8
    
    // Token keys
    TokenName,
    TokenSymbol,
    TokenDecimals,
    TotalSupply,
    Balance(Address),
    Allowance(Address, Address),
    
    // Carbon credit keys
    CarbonCredit(u64),
    CarbonCreditByTransaction(u64),
    
    // RecycleGraph passport keys
    MaterialPassport(String),
    PassportsByMaterial(u8, u64),
}

/// Data bucket identifiers for better storage organization
#[contracttype]
#[derive(Clone, Debug)]
pub enum DataBucket {
    Persistent,  // Long-term storage (admin, config)
    Temporary,   // Short-term storage (pending transactions)
    Instance,    // Per-instance storage
}
