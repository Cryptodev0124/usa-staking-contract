pub mod error;
pub mod instructions;
pub mod rewards;
pub mod state;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use instructions::*;

declare_id!("EoTc7b3Cn4smXnKDrrNFSJQa8VPjgZ53bDti9SbmCyrU");

#[program]
pub mod usa_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        instructions::initialize::handler(ctx)
    }

    pub fn set_stake_pool_rewards(
        ctx: Context<SetStakePoolRewards>,
        rewards_rate: u64,
        deposit_fee: u64,
        withdraw_fee: u64,
    ) -> ProgramResult {
        instructions::set_stake_pool_rewards::handler(ctx, rewards_rate, deposit_fee, withdraw_fee)
    }

    pub fn create_stake_pool(
        ctx: Context<CreateStakePool>,
        deposit_fee: u64,
        withdraw_fee: u64,
    ) -> ProgramResult {
        instructions::create_stake_pool::handler(ctx, deposit_fee, withdraw_fee)
    }

    pub fn create_stake_entry(ctx: Context<CreateStakeEntry>) -> ProgramResult {
        instructions::create_stake_entry::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> ProgramResult {
        instructions::stake::handler(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> ProgramResult {
        instructions::unstake::handler(ctx, amount)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> ProgramResult {
        instructions::claim_rewards::handler(ctx)
    }

    pub fn compound(ctx: Context<Compound>) -> ProgramResult {
        instructions::compound::handler(ctx)
    }
}
