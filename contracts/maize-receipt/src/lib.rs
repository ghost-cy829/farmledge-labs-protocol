#![no_std]

use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct TokenMetadata {
    pub token_id: String,
    pub commodity: String,
    pub grade: String,
    pub bag_count: u32,
    pub weight_per_bag_kg: u32,
    pub total_weight_kg: u32,
    pub warehouse_id: String,
    pub custodian: Address,
    pub deposit_ts: u64,
    pub is_locked: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_token_metadata_fields() {
        let env = Env::default();
        let custodian = Address::generate(&env);
        let token_id = String::from_str(&env, "TKN-001");
        let commodity = String::from_str(&env, "MAIZE_WHITE");
        let grade = String::from_str(&env, "Grade A");
        let warehouse_id = String::from_str(&env, "WH-001");

        let metadata = TokenMetadata {
            token_id: token_id.clone(),
            commodity: commodity.clone(),
            grade: grade.clone(),
            bag_count: 100,
            weight_per_bag_kg: 50,
            total_weight_kg: 5000,
            warehouse_id: warehouse_id.clone(),
            custodian: custodian.clone(),
            deposit_ts: 1234567890,
            is_locked: false,
        };

        assert_eq!(metadata.token_id, token_id);
        assert_eq!(metadata.commodity, commodity);
        assert_eq!(metadata.grade, grade);
        assert_eq!(metadata.bag_count, 100);
        assert_eq!(metadata.weight_per_bag_kg, 50);
        assert_eq!(metadata.total_weight_kg, 5000);
        assert_eq!(metadata.warehouse_id, warehouse_id);
        assert_eq!(metadata.custodian, custodian);
        assert_eq!(metadata.deposit_ts, 1234567890);
        assert_eq!(metadata.is_locked, false);
    }

    #[test]
    fn test_token_metadata_roundtrip() {
        let env = Env::default();
        let custodian = Address::generate(&env);
        let token_id = String::from_str(&env, "TKN-002");
        let commodity = String::from_str(&env, "MAIZE_YELLOW");
        let grade = String::from_str(&env, "Grade B");
        let warehouse_id = String::from_str(&env, "WH-002");

        let original = TokenMetadata {
            token_id: token_id.clone(),
            commodity: commodity.clone(),
            grade: grade.clone(),
            bag_count: 200,
            weight_per_bag_kg: 60,
            total_weight_kg: 12000,
            warehouse_id: warehouse_id.clone(),
            custodian: custodian.clone(),
            deposit_ts: 9876543210,
            is_locked: true,
        };

        // Simulate roundtrip by cloning and comparing
        let cloned = original.clone();

        assert_eq!(original.token_id, cloned.token_id);
        assert_eq!(original.commodity, cloned.commodity);
        assert_eq!(original.grade, cloned.grade);
        assert_eq!(original.bag_count, cloned.bag_count);
        assert_eq!(original.weight_per_bag_kg, cloned.weight_per_bag_kg);
        assert_eq!(original.total_weight_kg, cloned.total_weight_kg);
        assert_eq!(original.warehouse_id, cloned.warehouse_id);
        assert_eq!(original.custodian, cloned.custodian);
        assert_eq!(original.deposit_ts, cloned.deposit_ts);
        assert_eq!(original.is_locked, cloned.is_locked);
    }
}
