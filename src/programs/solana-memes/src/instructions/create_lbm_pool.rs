use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use crate::state::*;

#[derive(Accounts)]
pub struct CreateLBMPool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    
    #[account(mut)]
    pub creator_profile: Account<'info, CreatorProfile>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + std::mem::size_of::<LiquidityBootstrappingPool>(),
        seeds = [b"lbm_pool", token_mint.key().as_ref()],
        bump
    )]
    pub lbm_pool: Account<'info, LiquidityBootstrappingPool>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
    
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreateLBMPool>,
    target_liquidity: u64,
    bootstrap_duration: i64,
    price_discovery_period: i64,
    max_participation_per_wallet: u64,
    min_participation: u64,
    max_participation: u64,
    anti_bot_enabled: bool,
) -> Result<()> {
    let creator_profile = &mut ctx.accounts.creator_profile;
    let lbm_pool = &mut ctx.accounts.lbm_pool;
    let token_metadata = &mut ctx.accounts.token_metadata;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate creator permissions
    require!(creator_profile.is_registered, SolanaMemesError::CreatorNotRegistered);
    require!(!creator_profile.is_banned, SolanaMemesError::CreatorBanned);
    require!(creator_profile.owner == ctx.accounts.creator.key(), SolanaMemesError::Unauthorized);
    
    // Validate token ownership
    require!(token_metadata.creator == ctx.accounts.creator.key(), SolanaMemesError::Unauthorized);
    
    // Validate liquidity requirements
    require!(
        target_liquidity >= MIN_LIQUIDITY_REQUIREMENT,
        SolanaMemesError::InsufficientLiquidity
    );
    require!(
        target_liquidity <= MAX_LIQUIDITY_REQUIREMENT,
        SolanaMemesError::ExcessiveLiquidity
    );
    
    // Validate participation limits
    require!(
        min_participation >= DEFAULT_MIN_PARTICIPATION,
        SolanaMemesError::InvalidParticipationAmount
    );
    require!(
        max_participation_per_wallet <= DEFAULT_MAX_PARTICIPATION,
        SolanaMemesError::ExcessiveParticipationLimit
    );
    
    // Calculate initial price based on token supply and target liquidity
    let initial_price = (target_liquidity * 1_000_000) / token_metadata.total_supply; // Price in lamports
    
    // Create liquidity release schedule (gradual release over time)
    let mut release_schedule = Vec::new();
    let total_releases = 10; // 10 releases over the bootstrap period
    let release_interval = bootstrap_duration / total_releases;
    
    for i in 1..=total_releases {
        let release_time = current_time + (i as i64 * release_interval);
        let release_percentage = (i * 10) as u8; // 10%, 20%, 30%, etc.
        let release_amount = (target_liquidity * release_percentage as u64) / 100;
        
        release_schedule.push(LiquidityRelease {
            release_time,
            release_amount,
            release_percentage,
            is_executed: false,
        });
    }
    
    // Initialize LBM pool
    lbm_pool.token_mint = ctx.accounts.token_mint.key();
    lbm_pool.creator = ctx.accounts.creator.key();
    lbm_pool.initial_liquidity = 0;
    lbm_pool.target_liquidity = target_liquidity;
    lbm_pool.current_liquidity = 0;
    lbm_pool.bootstrap_start_time = current_time;
    lbm_pool.bootstrap_end_time = current_time + bootstrap_duration;
    lbm_pool.price_discovery_period = price_discovery_period;
    lbm_pool.liquidity_release_schedule = release_schedule;
    lbm_pool.is_active = true;
    lbm_pool.total_participants = 0;
    lbm_pool.total_volume = 0;
    lbm_pool.current_price = initial_price;
    lbm_pool.initial_price = initial_price;
    lbm_pool.final_price = 0;
    lbm_pool.price_discovery_complete = false;
    lbm_pool.liquidity_providers = Vec::new();
    lbm_pool.anti_bot_enabled = anti_bot_enabled;
    lbm_pool.max_participation_per_wallet = max_participation_per_wallet;
    lbm_pool.min_participation = min_participation;
    lbm_pool.max_participation = max_participation;
    
    // Update token metadata
    token_metadata.liquidity_locked = true;
    token_metadata.liquidity_lock_amount = target_liquidity;
    token_metadata.liquidity_lock_end = current_time + bootstrap_duration + price_discovery_period;
    
    // Transfer initial liquidity from creator to treasury
    let initial_liquidity_amount = (target_liquidity * 10) / 100; // 10% initial liquidity
    
    let cpi_accounts = token::Transfer {
        from: ctx.accounts.creator_token_account.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::transfer(cpi_ctx, initial_liquidity_amount)?;
    
    // Update pool with initial liquidity
    lbm_pool.initial_liquidity = initial_liquidity_amount;
    lbm_pool.current_liquidity = initial_liquidity_amount;
    
    // Add creator as first liquidity provider
    lbm_pool.liquidity_providers.push(LiquidityProvider {
        wallet: ctx.accounts.creator.key(),
        provided_amount: initial_liquidity_amount,
        participation_time: current_time,
        rewards_claimed: 0,
        is_whitelisted: true,
    });
    
    lbm_pool.total_participants = 1;
    
    msg!("LBM pool created successfully for token: {}", token_metadata.name);
    msg!("Target liquidity: {} lamports", target_liquidity);
    msg!("Initial price: {} lamports", initial_price);
    msg!("Bootstrap duration: {} seconds", bootstrap_duration);
    
    Ok(())
}
