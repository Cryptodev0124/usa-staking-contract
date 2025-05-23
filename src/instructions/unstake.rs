use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult, Accounts};
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use crate::error::ErrorCode;
use crate::rewards::update_rewards;
use crate::state::{StakeEntry, StakePool, STAKE_ENTRY_PREFIX, STAKE_POOL_PREFIX, TREASURY};

pub fn handler(ctx: Context<Unstake>, amount: u64) -> ProgramResult {
    let stake_pool_account_info = ctx.accounts.stake_pool.clone().to_account_info();

    let stake_pool = &mut ctx.accounts.stake_pool;
    let stake_entry = &mut ctx.accounts.stake_entry;

    update_rewards(stake_pool, stake_entry)?;

    stake_pool.balance -= amount;
    stake_entry.balance -= amount;

    let accounts = Transfer {
        from: ctx.accounts.escrow_a.to_account_info(),
        to: ctx.accounts.staker_token_a.to_account_info(),
        authority: stake_pool_account_info.clone(),
    };

    let seeds = &[
        STAKE_POOL_PREFIX.as_bytes(),
        &stake_pool.id.to_le_bytes(),
        &[stake_pool.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let transfer_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        accounts,
        signer_seeds,
    );

    let fee_amount = amount * stake_pool.withdraw_fee / 100;

    let fee_accounts = Transfer {
        from: ctx.accounts.escrow_a.to_account_info(),
        to: ctx.accounts.escrow_fee.to_account_info(),
        authority: stake_pool_account_info,
    };

    let fee_transfer_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        fee_accounts,
        signer_seeds,
    );

    transfer(fee_transfer_context, fee_amount)?;

    transfer(transfer_context, amount - fee_amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Unstake<'info> {
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

    #[account(
        mut,
        constraint = 
            staker_token_a.mint == stake_pool.mint_a &&
            staker_token_a.mint == mint_a.key() @ ErrorCode::TokenAMintMismatch,
        constraint = 
            staker_token_a.owner == staker.key() @ ErrorCode::OwnerMismatch
    )]
    pub staker_token_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = 
            escrow_a.mint == stake_pool.mint_a &&
            escrow_a.mint == mint_a.key() @ ErrorCode::TokenAMintMismatch,
        constraint = 
            escrow_a.owner == stake_pool.key() @ ErrorCode::OwnerMismatch
    )]
    pub escrow_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = 
            escrow_fee.mint == stake_pool.mint_a &&
            escrow_fee.mint == mint_a.key() @ ErrorCode::TokenAMintMismatch,
        constraint = 
            escrow_fee.owner == TREASURY.parse::<Pubkey>().unwrap() @ ErrorCode::OwnerMismatch
    )]
    pub escrow_fee: Account<'info, TokenAccount>,

    pub mint_a: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}
