use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use crate::state::*;

#[derive(Accounts)]
pub struct FinalizeLBM<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(mut)]
    pub lbm_pool: Account<'info, LiquidityBootstrappingPool>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
    
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    
    #[account(mut)]
    pub staking_rewards: Account<'info, StakingRewards>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FinalizeLBM>) -> Result<()> {
    let lbm_pool = &mut ctx.accounts.lbm_pool;
    let token_metadata = &mut ctx.accounts.token_metadata;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate creator permissions
    require!(lbm_pool.creator == ctx.accounts.creator.key(), SolanaMemesError::Unauthorized);
    
    // Check if bootstrap period has ended
    require!(
        current_time >= lbm_pool.bootstrap_end_time,
        SolanaMemesError::LBMStillActive
    );
    
    // Check if LBM is still active
    require!(lbm_pool.is_active, SolanaMemesError::LBMAlreadyFinalized);
    
    // Calculate final price based on total participation
    let final_price = if lbm_pool.current_liquidity > 0 {
        (lbm_pool.current_liquidity * 1_000_000) / lbm_pool.target_liquidity
    } else {
        lbm_pool.initial_price // Fallback to initial price if no participation
    };
    
    lbm_pool.final_price = final_price;
    lbm_pool.price_discovery_complete = true;
    
    // Execute liquidity release schedule
    let mut total_released = 0;
    for release in &mut lbm_pool.liquidity_release_schedule {
        if current_time >= release.release_time && !release.is_executed {
            release.is_executed = true;
            total_released += release.release_amount;
        }
    }
    
    // Calculate success metrics
    let success_rate = if lbm_pool.target_liquidity > 0 {
        (lbm_pool.current_liquidity * 100) / lbm_pool.target_liquidity
    } else {
        0
    };
    
    // Determine if LBM was successful (at least 50% of target liquidity)
    let is_successful = success_rate >= 50;
    
    if is_successful {
        // LBM successful - release remaining liquidity
        let remaining_liquidity = lbm_pool.target_liquidity - lbm_pool.current_liquidity;
        
        // Distribute rewards to participants based on their contribution
        let total_participation = lbm_pool.liquidity_providers.iter()
            .map(|provider| provider.provided_amount)
            .sum::<u64>();
        
        if total_participation > 0 {
            for provider in &mut lbm_pool.liquidity_providers {
                let share = (provider.provided_amount * remaining_liquidity) / total_participation;
                provider.rewards_claimed += share;
            }
        }
        
        // Add success bonus to staking rewards (10% of total liquidity)
        let success_bonus = (lbm_pool.current_liquidity * 10) / 100;
        let staking_rewards = &mut ctx.accounts.staking_rewards;
        staking_rewards.success_rewards_pool += success_bonus;
        staking_rewards.total_rewards_distributed += success_bonus;
        
        msg!("LBM finalized successfully! Success rate: {}%", success_rate);
        msg!("Final price: {} lamports", final_price);
        msg!("Total participants: {}", lbm_pool.total_participants);
        msg!("Success bonus distributed: {} lamports", success_bonus);
    } else {
        // LBM failed - return funds to participants
        msg!("LBM failed to reach target liquidity. Success rate: {}%", success_rate);
        msg!("Funds will be returned to participants");
    }
    
    // Update token metadata
    token_metadata.liquidity_locked = false;
    token_metadata.liquidity_lock_amount = 0;
    token_metadata.liquidity_lock_end = current_time;
    
    // Mark LBM as completed
    lbm_pool.is_active = false;
    
    // Update treasury
    let treasury = &mut ctx.accounts.treasury;
    treasury.fee_collection_stats.fees_distributed_to_stakers += total_released;
    
    msg!("LBM finalization complete");
    msg!("Token is now available for trading");
    
    Ok(())
}
