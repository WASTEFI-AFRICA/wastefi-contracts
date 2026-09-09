use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WasteFiError {
    // General errors (1-9)
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    InvalidInput = 4,
    NotFound = 5,
    AlreadyExists = 6,

    // Collector errors (10-19)
    CollectorNotFound = 10,
    CollectorAlreadyRegistered = 11,
    CollectorNotActive = 12,
    CollectorSuspended = 13,
    InvalidCollectorStatus = 14,

    // Collection point errors (20-29)
    CollectionPointNotFound = 20,
    CollectionPointNotVerified = 21,
    MaterialNotAccepted = 22,

    // Transaction errors (30-39)
    TransactionNotFound = 30,
    InvalidTransactionStatus = 31,
    InvalidWeight = 32,
    InvalidAmount = 33,

    // Payment errors (40-49)
    PaymentNotFound = 40,
    PaymentAlreadyProcessed = 41,
    InsufficientBalance = 42,
    PaymentFailed = 43,

    // Reputation errors (50-59)
    ReputationNotFound = 50,
    InvalidReputationScore = 51,

    // Material pricing errors (60-69)
    PriceNotSet = 60,
    InvalidPrice = 61,
    PriceUpdateTooFrequent = 62,

    // Token errors (70-79)
    InvalidTokenAmount = 70,
    TransferFailed = 71,
    MintingDisabled = 72,

    // Access control errors (80-89)
    NotAdmin = 80,
    NotOwner = 81,
    ContractPaused = 82,

    // Emergency and security errors (90-99)
    EmergencyActive = 90,
    EmergencyShutdown = 91,
    CircuitBreakerTripped = 92,
    OperationThrottled = 93,
    EmergencyWithdrawalNotEnabled = 94,
    FraudDetected = 95,
    DuplicateTransaction = 96,

    // Upgrade errors (85-89)
    IncompatibleVersion = 85,
    VersionNotSet = 86,
    UpgradeInProgress = 87,

    // RecycleGraph errors (100-109)
    InvalidPassportId = 100,
    PassportNotFound = 101,
}
