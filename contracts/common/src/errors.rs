use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WasteFiError {
    // General errors (100-199)
    NotInitialized = 100,
    AlreadyInitialized = 101,
    Unauthorized = 102,
    InvalidInput = 103,
    NotFound = 104,
    AlreadyExists = 105,

    // Collector errors (200-299)
    CollectorNotFound = 200,
    CollectorAlreadyRegistered = 201,
    CollectorNotActive = 202,
    CollectorSuspended = 203,
    CollectorBanned = 204,
    InvalidCollectorStatus = 205,

    // Collection point errors (300-399)
    CollectionPointNotFound = 300,
    CollectionPointNotVerified = 301,
    InvalidVerificationStatus = 302,
    MaterialNotAccepted = 303,

    // Transaction errors (400-499)
    TransactionNotFound = 400,
    TransactionAlreadyCompleted = 401,
    TransactionCancelled = 402,
    InvalidTransactionStatus = 403,
    InvalidWeight = 404,
    InvalidAmount = 405,

    // Payment errors (500-599)
    PaymentNotFound = 500,
    PaymentAlreadyProcessed = 501,
    InsufficientBalance = 502,
    PaymentFailed = 503,
    InvalidPaymentStatus = 504,

    // Reputation errors (600-699)
    ReputationNotFound = 600,
    InvalidReputationScore = 601,

    // Material pricing errors (700-799)
    PriceNotSet = 700,
    InvalidPrice = 701,
    PriceUpdateTooFrequent = 702,

    // Token errors (800-899)
    InvalidTokenAmount = 800,
    TransferFailed = 801,
    MintingDisabled = 802,
    BurningDisabled = 803,

    // Access control errors (900-999)
    NotAdmin = 900,
    NotOwner = 901,
    NotAuthorizedOperator = 902,
    ContractPaused = 903,

    // RecycleGraph errors (1000-1099)
    InvalidPassportId = 1000,
    PassportNotFound = 1001,
    InvalidChainOfCustody = 1002,
}
