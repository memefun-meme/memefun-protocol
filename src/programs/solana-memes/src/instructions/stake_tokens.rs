use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1
    )]
    pub staking_position: Account<'info, StakingPosition>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub pool_token_account: Account<'info, TokenAccount>,
    
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
    let staking_pool = &mut ctx.accounts.staking_pool;
    let staking_position = &mut ctx.accounts.staking_position;
    let clock = Clock::get()?;

    // Check if staking pool is active
    require!(staking_pool.is_active, CustomError::StakingPoolInactive);

    // Check amount limits
    require!(amount >= staking_pool.min_stake_amount, CustomError::InsufficientStakeAmount);
    require!(amount <= staking_pool.max_stake_amount, CustomError::ExceedsMaxStakeAmount);

    // Initialize staking position if new
    if staking_position.owner == Pubkey::default() {
        staking_position.owner = ctx.accounts.user.key();
        staking_position.pool = staking_pool.key();
        staking_position.amount = 0;
        staking_position.start_time = clock.unix_timestamp;
        staking_position.end_time = clock.unix_timestamp + staking_pool.lock_period;
        staking_position.rewards_claimed = 0;
        staking_position.pending_rewards = 0;
        staking_position.is_locked = true;
    }

    // Transfer tokens to pool
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.pool_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update staking position
    staking_position.amount += amount;
    staking_position.end_time = clock.unix_timestamp + staking_pool.lock_period;

    // Update pool
    staking_pool.total_staked += amount;
    staking_pool.last_update_time = clock.unix_timestamp;

    msg!(
        "Staked {} tokens. Total staked: {}",
        amount,
        staking_position.amount
    );

    Ok(())
}
