use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, InitializeMint};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::*;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut, has_one = owner)]
    pub creator_profile: Account<'info, CreatorProfile>,
    
    pub creator: Signer<'info>,
    
    #[account(
        init,
        payer = creator,
        mint::decimals = 9,
        mint::authority = creator
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = creator
    )]
    pub creator_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = vesting
    )]
    pub vesting_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 8
    )]
    pub vesting: Account<'info, Vesting>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 32 + 200 + 200 + 200 + 1 + 8 + 8 + 8 + 1 + 8 + 8 + 8 + 1 + 8 + 8 + 8 + 8
    )]
    pub token_metadata: Account<'info, TokenMetadata>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8
    )]
    pub staking_pool: Account<'info, StakingPool>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8
    )]
    pub anti_bot_config: Account<'info, AntiBotConfig>,
    
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 8 + 8 + 8 + 8 + 16 + 1 + 8 + 8 + 8
    )]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    
    /// CHECK: Launch pass verification
    pub launch_pass: Option<AccountInfo<'info>>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreateToken>,
    name: String,
    symbol: String,
    uri: String,
    decimals: u8,
    total_supply: u64,
    creator_percent: u8,
    vesting_seconds: i64,
) -> Result<()> {
    let creator = &mut ctx.accounts.creator_profile;
    let clock = Clock::get()?;

    // Ensure creator is registered
    require!(creator.is_registered, CustomError::NotRegistered);
    require!(!creator.is_banned, CustomError::CreatorBanned);

    // Enforce rate-limit: 1 token per 30 days
    require!(
        clock.unix_timestamp - creator.last_creation_ts > RATE_LIMIT_PERIOD,
        CustomError::RateLimit
    );

    // Validate creator percentage
    require!(creator_percent <= MAX_CREATOR_PERCENT, CustomError::InvalidCreatorPercent);
    require!(creator_percent > 0, CustomError::InvalidCreatorPercent);

    // Validate vesting period
    require!(
        vesting_seconds >= MIN_VESTING_PERIOD && vesting_seconds <= MAX_VESTING_PERIOD,
        CustomError::InvalidVestingPeriod
    );

    // Validate token parameters
    require!(total_supply > 0, CustomError::InvalidTokenSupply);
    require!(decimals <= 9, CustomError::InvalidTokenDecimals);
    require!(name.len() <= 200, CustomError::TokenNameTooLong);
    require!(symbol.len() <= 200, CustomError::TokenSymbolTooLong);
    require!(uri.len() <= 200, CustomError::TokenUriTooLong);

    // Check launch pass if required
    if creator.launch_pass_required {
        require!(ctx.accounts.launch_pass.is_some(), CustomError::LaunchPassRequired);
        // In production, verify the launch pass NFT ownership and validity
    }

    // Initialize the token mint
    token::initialize_mint(
        ctx.accounts.init_mint_ctx(),
        decimals,
        &ctx.accounts.creator.key(),
        Some(&ctx.accounts.creator.key()),
    )?;

    // Mint total supply to creator's account
    token::mint_to(
        ctx.accounts.mint_to_ctx(),
        total_supply,
    )?;

    // Calculate creator allocation
    let creator_amount = (total_supply as u128 * creator_percent as u128 / 100) as u64;
    let public_amount = total_supply - creator_amount;

    // Transfer creator's portion to vesting account
    token::transfer(
        ctx.accounts.transfer_to_vesting_ctx(),
        creator_amount,
    )?;

    // Initialize vesting schedule
    let vesting = &mut ctx.accounts.vesting;
    vesting.owner = ctx.accounts.creator.key();
    vesting.mint = ctx.accounts.mint.key();
    vesting.amount = creator_amount;
    vesting.start_time = clock.unix_timestamp;
    vesting.cliff_time = clock.unix_timestamp + vesting_seconds;
    vesting.end_time = clock.unix_timestamp + vesting_seconds;
    vesting.released = 0;
    vesting.is_revocable = false;
    vesting.revoked = false;
    vesting.revoke_time = None;

    // Initialize token metadata
    let token_metadata = &mut ctx.accounts.token_metadata;
    token_metadata.mint = ctx.accounts.mint.key();
    token_metadata.creator = ctx.accounts.creator.key();
    token_metadata.name = name.clone();
    token_metadata.symbol = symbol.clone();
    token_metadata.uri = uri.clone();
    token_metadata.decimals = decimals;
    token_metadata.total_supply = total_supply;
    token_metadata.circulating_supply = public_amount;
    token_metadata.creator_percent = creator_percent;
    token_metadata.vesting_seconds = vesting_seconds;
    token_metadata.created_at = clock.unix_timestamp;
    token_metadata.is_verified = false;
    token_metadata.risk_score = 50; // Medium risk initially
    token_metadata.liquidity_locked = false;
    token_metadata.liquidity_lock_amount = 0;
    token_metadata.liquidity_lock_end = 0;
    token_metadata.anti_bot_enabled = true;
    token_metadata.anti_bot_window = ANTI_BOT_WINDOW;
    token_metadata.max_transaction_size = total_supply * MAX_TRANSACTION_SIZE_PERCENT as u64 / 100;
    token_metadata.min_transaction_size = MIN_TRANSACTION_SIZE;

    // Initialize staking pool
    let staking_pool = &mut ctx.accounts.staking_pool;
    staking_pool.mint = ctx.accounts.mint.key();
    staking_pool.total_staked = 0;
    staking_pool.total_rewards = 0;
    staking_pool.reward_rate = REWARD_RATE_PER_DAY;
    staking_pool.last_update_time = clock.unix_timestamp;
    staking_pool.min_stake_amount = MIN_TRANSACTION_SIZE;
    staking_pool.max_stake_amount = total_supply / 10; // Max 10% of supply
    staking_pool.lock_period = LIQUIDITY_LOCK_PERIOD;
    staking_pool.early_withdrawal_fee = EARLY_WITHDRAWAL_FEE;
    staking_pool.is_active = true;

    // Initialize anti-bot configuration
    let anti_bot_config = &mut ctx.accounts.anti_bot_config;
    anti_bot_config.token_mint = ctx.accounts.mint.key();
    anti_bot_config.enabled = true;
    anti_bot_config.max_transaction_size = total_supply * MAX_TRANSACTION_SIZE_PERCENT as u64 / 100;
    anti_bot_config.min_transaction_size = MIN_TRANSACTION_SIZE;
    anti_bot_config.cooldown_period = ANTI_BOT_WINDOW;
    anti_bot_config.blacklisted_addresses = Vec::new();
    anti_bot_config.whitelisted_addresses = Vec::new();
    anti_bot_config.max_wallet_percentage = 5; // 5% max per wallet
    anti_bot_config.max_transaction_percentage = 2; // 2% max per transaction

    // Initialize liquidity pool
    let liquidity_pool = &mut ctx.accounts.liquidity_pool;
    liquidity_pool.token_mint = ctx.accounts.mint.key();
    liquidity_pool.token_reserve = 0;
    liquidity_pool.sol_reserve = 0;
    liquidity_pool.lp_supply = 0;
    liquidity_pool.fee_rate = 300; // 0.3%
    liquidity_pool.is_active = false; // Will be activated when liquidity is added
    liquidity_pool.created_at = clock.unix_timestamp;
    liquidity_pool.last_swap_time = 0;
    liquidity_pool.total_volume = 0;

    // Update creator profile
    creator.last_creation_ts = clock.unix_timestamp;
    creator.total_tokens_created += 1;

    msg!(
        "Token '{}' created successfully! Supply: {}, Creator: {}% (vested), Public: {}",
        name,
        total_supply,
        creator_percent,
        public_amount
    );

    Ok(())
}

impl<'info> CreateToken<'info> {
    pub fn init_mint_ctx(&self) -> CpiContext<'_, '_, '_, 'info, InitializeMint<'info>> {
        let cpi_accounts = InitializeMint {
            mint: self.mint.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn mint_to_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.creator_token_account.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn transfer_to_vesting_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let cpi_accounts = token::Transfer {
            from: self.creator_token_account.to_account_info(),
            to: self.vesting_token_account.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
