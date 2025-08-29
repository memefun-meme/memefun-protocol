use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct RegisterCreator<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    /// CHECK: Treasury account (safe, just receives SOL)
    #[account(mut)]
    pub treasury: AccountInfo<'info>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + 1 + 32 + 8 + 8 + 4 + 4 + 4 + 8 + 1 + 200 + 1 + 32
    )]
    pub creator_profile: Account<'info, CreatorProfile>,
    
    /// CHECK: Launch pass mint (optional)
    pub launch_pass_mint: Option<AccountInfo<'info>>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterCreator>, stake_amount: u64) -> Result<()> {
    let creator = &mut ctx.accounts.creator_profile;
    let clock = Clock::get()?;

    // Ensure not already registered
    require!(!creator.is_registered, CustomError::AlreadyRegistered);

    // Check minimum stake amount
    require!(stake_amount >= MIN_STAKE_AMOUNT, CustomError::InsufficientStake);

    // Transfer stake (in SOL lamports)
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.creator.key(),
        &ctx.accounts.treasury.key(),
        stake_amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.creator.to_account_info(),
            ctx.accounts.treasury.to_account_info(),
        ],
    )?;

    // Initialize creator profile
    creator.is_registered = true;
    creator.owner = ctx.accounts.creator.key();
    creator.stake_amount = stake_amount;
    creator.last_creation_ts = 0;
    creator.reputation_score = 0;
    creator.total_tokens_created = 0;
    creator.successful_tokens = 0;
    creator.failed_tokens = 0;
    creator.total_volume = 0;
    creator.is_banned = false;
    creator.ban_reason = "".to_string();
    creator.launch_pass_required = ctx.accounts.launch_pass_mint.is_some();
    creator.launch_pass_mint = ctx.accounts.launch_pass_mint.as_ref().map(|mint| mint.key());

    msg!(
        "Creator registered successfully with {} SOL stake",
        stake_amount as f64 / 1_000_000_000.0
    );

    Ok(())
}
