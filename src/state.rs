use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const EPOCH_LENGTH: i32 = 432000;
pub enum AccTypesWithVersion {
    YourPoolDataV1 = 2,
    UserDataV1 = 3,
}

pub const YOUR_POOL_STORAGE_TOTAL_BYTES: usize = 142; // Should be 2 bytes less than real size of
#[derive(Clone, BorshDeserialize, BorshSerialize, Copy)]
pub struct YourPool {
    pub acc_type: u8,
    pub owner_wallet: Pubkey,
    pub your_staking_vault: Pubkey,
    pub your_reward_rate: u64,
    pub your_epoch_duration: u64,
    pub user_stake_count: u32,
    pub user_total_stake: u64,
    pub pda_nonce: u8,
    pub reward_duration_end: u64,

    pub rewards_per_slot: u64,
    pub max_reward_rate: u64,
    pub min_reward_rate: u64,

    pub total_weighted_stake: f64,
    pub weighted_epoch_id: i64,
}

pub const USER_STORAGE_TOTAL_BYTES: usize = 114;
#[derive(Clone, BorshDeserialize, BorshSerialize, Copy)]
pub struct User {
    pub acc_type: u8,
    pub user_wallet: Pubkey,
    pub your_pool: Pubkey,

    pub unstake_pending: u64,
    pub unstake_pending_date: i64,

    pub nonce: u8,
    pub claim_timeout_date: i64,

    pub user_weighted_epoch: i64,
    pub user_weighted_stake: f64,
    pub balance_your_staked: u64,
}
