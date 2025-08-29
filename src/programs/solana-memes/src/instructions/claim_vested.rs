use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct ClaimVested<'info> {
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    
    pub creator: Signer<'info>,
    
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub vesting_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ClaimVested>) -> Result<()> {
    let clock = Clock::get()?;
    let vesting = &mut ctx.accounts.vesting;

    // Verify creator owns the vesting
    require!(
        vesting.owner == ctx.accounts.creator.key(),
        CustomError::Unauthorized
    );

    // Check if vesting is revoked
    require!(!vesting.revoked, CustomError::VestingRevoked);

    // Check if cliff time has been reached
    require!(
        clock.unix_timestamp >= vesting.cliff_time,
        CustomError::NotVested
    );

    // Calculate claimable amount
    let total_vested = if clock.unix_timestamp >= vesting.end_time {
        vesting.amount
    } else {
        let time_elapsed = clock.unix_timestamp - vesting.start_time;
        let total_duration = vesting.end_time - vesting.start_time;
        (vesting.amount as u128 * time_elapsed as u128 / total_duration as u128) as u64
    };

    let claimable = total_vested - vesting.released;
    require!(claimable > 0, CustomError::NothingToClaim);

    // Transfer claimable tokens to creator's wallet
    let seeds: &[&[&[u8]]] = &[];
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.vesting_token_account.to_account_info(),
                to: ctx.accounts.creator_token_account.to_account_info(),
                authority: ctx.accounts.vesting.to_account_info(),
            },
            seeds,
        ),
        claimable,
    )?;

    // Update vesting record
    vesting.released += claimable;

    msg!(
        "Claimed {} vested tokens. Total claimed: {}/{}",
        claimable,
        vesting.released,
        vesting.amount
    );

    Ok(())
}
