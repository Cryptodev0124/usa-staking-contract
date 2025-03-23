use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token::{Token};

use crate::rewards::update_rewards;
use crate::state::{StakeEntry, StakePool, STAKE_ENTRY_PREFIX, STAKE_POOL_PREFIX};

pub fn handler(ctx: Context<Compound>) -> ProgramResult {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let stake_entry = &mut ctx.accounts.stake_entry;
    update_rewards(stake_pool, stake_entry)?;

    // Claim rewards
    let rewards_amount = stake_entry.rewards;
    stake_entry.rewards = 0;

    // Add rewards to the user's stake
    stake_entry.balance += rewards_amount;

    // Update the stake pool's total staked amount
    stake_pool.balance += rewards_amount;

    Ok(())
}

#[derive(Accounts)]
pub struct Compound<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        mut,
        seeds = [STAKE_POOL_PREFIX.as_bytes(), &stake_pool.id.to_le_bytes()],
        bump = stake_pool.bump
    )]
    pub stake_pool: Account<'info, StakePool>,

    #[account(
        mut,
        seeds = [STAKE_ENTRY_PREFIX.as_bytes(), &stake_pool.key().to_bytes(), &staker.key().to_bytes()],
        bump = stake_entry.bump
    )]
    pub stake_entry: Account<'info, StakeEntry>,

    pub token_program: Program<'info, Token>,
}
