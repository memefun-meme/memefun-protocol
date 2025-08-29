use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use crate::state::*;

#[derive(Accounts)]
pub struct ParticipateLBM<'info> {
    #[account(mut)]
    pub participant: Signer<'info>,
    
    #[account(mut)]
    pub lbm_pool: Account<'info, LiquidityBootstrappingPool>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub participant_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    
    #[account(mut)]
    pub staking_rewards: Account<'info, StakingRewards>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ParticipateLBM>, participation_amount: u64) -> Result<()> {
    let lbm_pool = &mut ctx.accounts.lbm_pool;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate LBM pool is active
    require!(lbm_pool.is_active, SolanaMemesError::LBMNotActive);
    require!(
        current_time >= lbm_pool.bootstrap_start_time,
        SolanaMemesError::LBMNotStarted
    );
    require!(
        current_time <= lbm_pool.bootstrap_end_time,
        SolanaMemesError::LBMEnded
    );
    
    // Validate participation amount
    require!(
        participation_amount >= lbm_pool.min_participation,
        SolanaMemesError::InsufficientParticipationAmount
    );
    require!(
        participation_amount <= lbm_pool.max_participation_per_wallet,
        SolanaMemesError::ExcessiveParticipationAmount
    );
    
    // Check if target liquidity would be exceeded
    require!(
        lbm_pool.current_liquidity + participation_amount <= lbm_pool.target_liquidity,
        SolanaMemesError::TargetLiquidityExceeded
    );
    
    // Check if participant already exists
    let participant_key = ctx.accounts.participant.key();
    let existing_provider = lbm_pool.liquidity_providers.iter_mut()
        .find(|provider| provider.wallet == participant_key);
    
    if let Some(provider) = existing_provider {
        // Update existing provider
        let new_total = provider.provided_amount + participation_amount;
        require!(
            new_total <= lbm_pool.max_participation_per_wallet,
            SolanaMemesError::ExcessiveParticipationAmount
        );
        provider.provided_amount = new_total;
    } else {
        // Add new provider
        lbm_pool.liquidity_providers.push(LiquidityProvider {
            wallet: participant_key,
            provided_amount: participation_amount,
            participation_time: current_time,
            rewards_claimed: 0,
            is_whitelisted: false,
        });
        lbm_pool.total_participants += 1;
    }
    
    // Transfer tokens from participant to treasury
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.participant_token_account.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.participant.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::transfer(cpi_ctx, participation_amount)?;
    
    // Update pool statistics
    lbm_pool.current_liquidity += participation_amount;
    lbm_pool.total_volume += participation_amount;
    
    // Calculate new price based on current liquidity
    let new_price = (lbm_pool.current_liquidity * 1_000_000) / lbm_pool.target_liquidity;
    lbm_pool.current_price = new_price;
    
    // Add participation rewards to staking pool (5% of participation goes to stakers)
    let staking_reward = (participation_amount * 5) / 100;
    let staking_rewards = &mut ctx.accounts.staking_rewards;
    staking_rewards.total_rewards_distributed += staking_reward;
    
    // Update treasury
    let treasury = &mut ctx.accounts.treasury;
    treasury.fee_collection_stats.fees_distributed_to_stakers += staking_reward;
    
    msg!("Participation successful: {} lamports", participation_amount);
    msg!("Current liquidity: {} / {} lamports", lbm_pool.current_liquidity, lbm_pool.target_liquidity);
    msg!("Current price: {} lamports", lbm_pool.current_price);
    msg!("Total participants: {}", lbm_pool.total_participants);
    
    Ok(())
}
