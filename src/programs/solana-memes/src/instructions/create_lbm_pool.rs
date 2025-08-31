use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct CreateLBMPool<'info> {
    #[account(mut)]
    pub creator: Account<'info, CreatorProfile>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub lbm_pool: Account<'info, LiquidityBootstrappingPool>,
    
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateLBMPool>,
    target_liquidity: u64,
    bootstrap_duration: i64,
    price_discovery_period: i64,
    max_participation_per_wallet: u64, // Set to 0 for unlimited
    min_participation: u64, // Set to 0 for no minimum
    max_participation: u64, // Set to 0 for unlimited
    anti_bot_enabled: bool, // Set to false for wealth creation
) -> Result<()> {
    let creator = &mut ctx.accounts.creator;
    let lbm_pool = &mut ctx.accounts.lbm_pool;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate creator
    require!(
        creator.creator == ctx.accounts.authority.key(),
        CustomError::NotRegistered
    );
    
    // Check if creator is not banned
    require!(!creator.is_banned, CustomError::CreatorBanned);
    
    // Validate LBM parameters for wealth creation
    require!(target_liquidity > 0, CustomError::InvalidTokenSupply);
    require!(bootstrap_duration > 0, CustomError::InvalidVestingPeriod);
    require!(price_discovery_period > 0, CustomError::InvalidVestingPeriod);
    
    // For wealth creation: Remove all limits
    let unlimited_participation = 0; // 0 means unlimited
    
    // Initialize LBM pool for wealth creation
    lbm_pool.creator = ctx.accounts.authority.key();
    lbm_pool.token_mint = ctx.accounts.token_mint.key();
    lbm_pool.target_liquidity = target_liquidity;
    lbm_pool.current_liquidity = 0;
    lbm_pool.bootstrap_duration = bootstrap_duration;
    lbm_pool.price_discovery_period = price_discovery_period;
    lbm_pool.start_time = current_time;
    lbm_pool.end_time = current_time + bootstrap_duration;
    lbm_pool.price_discovery_end = current_time + bootstrap_duration + price_discovery_period;
    
    // Wealth creation settings - NO LIMITS
    lbm_pool.max_participation_per_wallet = unlimited_participation; // Unlimited
    lbm_pool.min_participation = 0; // No minimum
    lbm_pool.max_participation = unlimited_participation; // Unlimited
    lbm_pool.anti_bot_enabled = false; // Disable anti-bot for wealth creation
    lbm_pool.total_participants = 0;
    lbm_pool.total_participation = 0;
    lbm_pool.current_price = 0;
    lbm_pool.final_price = 0;
    lbm_pool.is_active = true;
    lbm_pool.is_finalized = false;
    lbm_pool.created_at = current_time;
    lbm_pool.updated_at = current_time;
    
    // Bonding curve parameters for wealth creation
    lbm_pool.bonding_curve_enabled = true;
    lbm_pool.curve_exponent = 1.5; // Aggressive price appreciation
    lbm_pool.base_price = 1000; // 0.001 SOL base price
    lbm_pool.initial_supply = ctx.accounts.token_mint.supply;
    
    msg!("LBM Pool created for wealth creation!");
    msg!("Creator: {}", ctx.accounts.authority.key());
    msg!("Target Liquidity: {} SOL", target_liquidity / 1_000_000_000);
    msg!("Duration: {} hours", bootstrap_duration / 3600);
    msg!("Unlimited Participation: {}", unlimited_participation == 0);
    msg!("Anti-Bot: Disabled for wealth creation");
    msg!("Bonding Curve: Enabled with 1.5x exponent");
    
    Ok(())
}

#[derive(Accounts)]
pub struct ParticipateLBMUnlimited<'info> {
    #[account(mut)]
    pub lbm_pool: Account<'info, LiquidityBootstrappingPool>,
    
    #[account(mut)]
    pub participant: Account<'info, LiquidityProvider>,
    
    #[account(mut)]
    pub participant_sol_account: Signer<'info>,
    
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn participate_unlimited(
    ctx: Context<ParticipateLBMUnlimited>,
    participation_amount: u64,
) -> Result<()> {
    let lbm_pool = &mut ctx.accounts.lbm_pool;
    let participant = &mut ctx.accounts.participant;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if LBM is active
    require!(lbm_pool.is_active, CustomError::LiquidityPoolInactive);
    require!(!lbm_pool.is_finalized, CustomError::ProposalAlreadyExecuted);
    
    // Check if participation period is still open
    require!(
        current_time <= lbm_pool.end_time,
        CustomError::VotingPeriodEnded
    );
    
    // For wealth creation: NO LIMITS
    // Remove all participation limits
    require!(participation_amount > 0, CustomError::InsufficientStakeAmount);
    
    // No maximum participation check (unlimited)
    // No minimum participation check (no minimum)
    // No anti-bot checks (disabled for wealth creation)
    
    // Calculate tokens based on current price
    let current_price = lbm_pool.current_price;
    let tokens_to_receive = if current_price > 0 {
        (participation_amount * 1_000_000_000) / current_price // Convert to token decimals
    } else {
        // Initial participation gets bonus tokens
        (participation_amount * 1_000_000_000) / lbm_pool.base_price * 2 // 2x bonus for early participants
    };
    
    // Update LBM pool
    lbm_pool.current_liquidity += participation_amount;
    lbm_pool.total_participation += participation_amount;
    lbm_pool.total_participants += 1;
    
    // Update price based on bonding curve
    let new_price = calculate_bonding_curve_price(
        lbm_pool.base_price,
        lbm_pool.current_liquidity,
        lbm_pool.target_liquidity,
        lbm_pool.curve_exponent,
    );
    lbm_pool.current_price = new_price;
    
    // Update participant record
    if participant.participant == Pubkey::default() {
        // New participant
        participant.participant = ctx.accounts.authority.key();
        participant.participation_amount = participation_amount;
        participant.tokens_received = tokens_to_receive;
        participant.participation_time = current_time;
        participant.created_at = current_time;
        participant.updated_at = current_time;
    } else {
        // Existing participant
        participant.participation_amount += participation_amount;
        participant.tokens_received += tokens_to_receive;
        participant.updated_at = current_time;
    }
    
    // Transfer SOL to treasury
    // Note: In real implementation, this would transfer SOL
    
    msg!("Unlimited LBM participation successful!");
    msg!("Participant: {}", ctx.accounts.authority.key());
    msg!("Amount: {} SOL", participation_amount / 1_000_000_000);
    msg!("Tokens: {}", tokens_to_receive);
    msg!("Current Price: {}", lbm_pool.current_price);
    msg!("Total Liquidity: {} SOL", lbm_pool.current_liquidity / 1_000_000_000);
    
    Ok(())
}

// Bonding curve price calculation for wealth creation
fn calculate_bonding_curve_price(
    base_price: u64,
    current_liquidity: u64,
    target_liquidity: u64,
    curve_exponent: f64,
) -> u64 {
    if current_liquidity == 0 {
        return base_price;
    }
    
    let liquidity_ratio = current_liquidity as f64 / target_liquidity as f64;
    let price_multiplier = liquidity_ratio.powf(curve_exponent);
    let new_price = base_price as f64 * price_multiplier;
    
    new_price as u64
}
