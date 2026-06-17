#![no_std]

mod errors;
mod storage;

pub use errors::ContractError;
use storage::DataKey;

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct MaizeReceiptContract;

#[contractimpl]
impl MaizeReceiptContract {
    pub fn init(env: Env, admin: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenCounter, &0u64);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_init_sets_admin() {
        let env = Env::default();
        let contract_id = env.register(MaizeReceiptContract, ());
        let client = MaizeReceiptContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        let stored: Address = env
            .as_contract(&contract_id, || {
                env.storage().instance().get(&DataKey::Admin).unwrap()
            });
        assert_eq!(stored, admin);
    }

    #[test]
    fn test_init_sets_counter_to_zero() {
        let env = Env::default();
        let contract_id = env.register(MaizeReceiptContract, ());
        let client = MaizeReceiptContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        let counter: u64 = env
            .as_contract(&contract_id, || {
                env.storage().instance().get(&DataKey::TokenCounter).unwrap()
            });
        assert_eq!(counter, 0u64);
    }

    #[test]
    fn test_init_rejects_double_call() {
        let env = Env::default();
        let contract_id = env.register(MaizeReceiptContract, ());
        let client = MaizeReceiptContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.init(&admin);

        let result = client.try_init(&admin);
        assert_eq!(
            result,
            Err(Ok(ContractError::AlreadyInitialized))
        );
    }
}
