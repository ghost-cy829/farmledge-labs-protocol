use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ContractError {
    AlreadyInitialized = 1,
    Unauthorized = 2,
    TokenNotFound = 3,
    TokenLocked = 4,
    InvalidCommodity = 5,
    InvalidWeight = 6,
}
