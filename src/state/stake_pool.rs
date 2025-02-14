use std::mem::size_of;

use anchor_lang::prelude::*;

pub const STAKE_POOL_SIZE: usize = size_of::<StakePool>() + 8;
pub const STAKE_POOL_PREFIX: &str = "stake-pool";
pub const REWARD_ESCROW_A_PREFIX: &str = "reward-a-escrow";
pub const REWARD_ESCROW_B_PREFIX: &str = "reward-b-escrow";
pub const REWARD_ESCROW_FEE_PREFIX: &str = "reward-fee-escrow";
pub const TREASURY: &str = "9isyjp6Fu6sEJmV1D9pyorxmuCZGDMJYBuZsjp9sStTe";

#[account]
pub struct StakePool {
    pub bump: u8,
    pub id: u16,
    pub balance: u64,
    pub mint_a: Pubkey,
    pub escrow_a: Pubkey,
    pub escrow_b: Pubkey,
    // pub escrow_fee: Pubkey,
    pub creator: Pubkey,
    pub rewards_rate: u64,
    pub deposit_fee: u64,
    pub withdraw_fee: u64,
    pub rewards_per_token_stored: u64,
    pub last_update_timestamp: i64,
}
