use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8
    )]
    pub treasury: Account<'info, Treasury>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    
    // Initialize treasury
    treasury.authority = ctx.accounts.authority.key();
    treasury.sol_balance = 0;
    treasury.total_fees_collected = 0;
    treasury.buyback_funds = 0;
    treasury.liquidity_funds = 0;
    treasury.governance_funds = 0;
    treasury.last_update = Clock::get()?.unix_timestamp;
    
    msg!("Solana Memes program initialized successfully!");
    msg!("Treasury authority: {}", treasury.authority);
    
    Ok(())
}
