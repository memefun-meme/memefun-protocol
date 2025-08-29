use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub anti_bot_config: Account<'info, AntiBotConfig>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    // Anti-bot protection checks
    let anti_bot = &ctx.accounts.anti_bot_config;
    
    if anti_bot.enabled {
        // Check transaction size limits
        require!(
            amount <= anti_bot.max_transaction_size,
            CustomError::TransactionTooLarge
        );
        require!(
            amount >= anti_bot.min_transaction_size,
            CustomError::TransactionTooSmall
        );
        
        // Check if sender is blacklisted
        require!(
            !anti_bot.blacklisted_addresses.contains(&ctx.accounts.authority.key()),
            CustomError::AddressBlacklisted
        );
    }
    
    // Perform the transfer
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;
    
    msg!("Transferred {} tokens", amount);
    Ok(())
}
