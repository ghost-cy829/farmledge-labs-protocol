use soroban_sdk::{Address, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenMetadata {
    pub token_id: String,
    pub commodity: String,   // "MAIZE_WHITE" or "MAIZE_YELLOW"
    pub grade: String,       // "Grade A" | "Grade B" | "Grade C"
    pub bag_count: u32,
    pub weight_per_bag_kg: u32,
    pub total_weight_kg: u32, // always bag_count * weight_per_bag_kg
    pub warehouse_id: String,
    pub custodian: Address,
    pub deposit_ts: u64,
    pub is_locked: bool,
}
