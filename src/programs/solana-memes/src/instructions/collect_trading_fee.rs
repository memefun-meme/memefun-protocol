use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;

#[derive(Accounts)]
pub struct CollectTradingFee<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,
    
    #[account(mut)]
    pub trader_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    
    #[account(mut)]
    pub staking_rewards: Account<'info, StakingRewards>,
    
    /// Platform configuration for dynamic fee
    pub platform_config: Account<'info, PlatformConfig>,
    
    /// The token being traded
    pub token_mint: Account<'info, Mint>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CollectTradingFee>, trade_amount: u64) -> Result<()> {
    let platform_config = &ctx.accounts.platform_config;
    
    // Use dynamic trading fee from platform config
    let trading_fee = (trade_amount * platform_config.trading_fee_percentage as u64) / 1000; // Basis points
    
    // Transfer trading fee to treasury
    let cpi_accounts = Transfer {
        from: ctx.accounts.trader_token_account.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.trader.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::transfer(cpi_ctx, trading_fee)?;
    
    // Update treasury stats
    let treasury = &mut ctx.accounts.treasury;
    treasury.fee_collection_stats.total_trading_fees += trading_fee;
    treasury.last_updated = Clock::get()?.unix_timestamp;
    
    // Update staking rewards pool
    let staking_rewards = &mut ctx.accounts.staking_rewards;
    staking_rewards.platform_fees_collected += trading_fee;
    
    // Calculate distribution (55% to stakers, 35% to development, 10% to governance)
    let staker_portion = (trading_fee * STAKER_REWARD_PERCENTAGE as u64) / 100;
    let development_portion = (trading_fee * DEVELOPMENT_PERCENTAGE as u64) / 100;
    let governance_portion = (trading_fee * GOVERNANCE_PERCENTAGE as u64) / 100;
    
    staking_rewards.total_rewards_distributed += staker_portion;
    treasury.fee_collection_stats.fees_distributed_to_stakers += staker_portion;
    treasury.fee_collection_stats.fees_retained_for_development += development_portion;
    treasury.fee_collection_stats.fees_for_governance += governance_portion;
    
    Ok(())
}
