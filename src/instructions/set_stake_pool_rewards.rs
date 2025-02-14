use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};

use crate::state::StakePool;

pub fn handler(
    ctx: Context<SetStakePoolRewards>,
    rewards_rate: u64,
    deposit_fee: u64,
    withdraw_fee: u64,
) -> ProgramResult {
    let stake_pool = &mut ctx.accounts.stake_pool;

    stake_pool.rewards_rate = rewards_rate;
    stake_pool.deposit_fee = deposit_fee;
    stake_pool.withdraw_fee = withdraw_fee;

    Ok(())
}

#[derive(Accounts)]
pub struct SetStakePoolRewards<'info> {
    #[account(
        mut,
        constraint = admin.key() == stake_pool.creator.key()
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,
}
