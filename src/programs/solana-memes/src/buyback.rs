use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

/// Buyback configuration account
#[account]
pub struct BuybackConfig {
    pub burn_percent: u8,    // % for burning (0–100)
    pub lp_percent: u8,      // % for LP (0–100)
    pub authority: Pubkey,   // the team/DAO that can update it
    pub treasury: Pubkey,    // treasury account for buyback funds
    pub min_buyback_amount: u64, // minimum amount for buyback
    pub max_buyback_amount: u64, // maximum amount for buyback
    pub cooldown_period: i64,    // cooldown between buybacks
    pub last_buyback_time: i64,  // timestamp of last buyback
}

/// Buyback execution account
#[account]
pub struct BuybackExecution {
    pub config: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub executed_at: i64,
    pub burn_amount: u64,
    pub lp_amount: u64,
}

/// Initialize buyback configuration
#[derive(Accounts)]
pub struct InitializeBuybackConfig<'info> {
    #[account(init, payer = authority, space = 8 + 1 + 1 + 32 + 32 + 8 + 8 + 8 + 8)]
    pub config: Account<'info, BuybackConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: Treasury account
    pub treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

/// Update buyback configuration
#[derive(Accounts)]
pub struct UpdateBuybackConfig<'info> {
    #[account(mut, has_one = authority)]
    pub config: Account<'info, BuybackConfig>,
    pub authority: Signer<'info>,
}

/// Execute buyback
#[derive(Accounts)]
pub struct ExecuteBuyback<'info> {
    #[account(mut)]
    pub config: Account<'info, BuybackConfig>,
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub dex_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_pool: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + 32 + 32 + 8 + 8 + 8 + 8)]
    pub execution: Account<'info, BuybackExecution>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Burn tokens from treasury
#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// Transfer tokens to DEX for buyback
#[derive(Accounts)]
pub struct BuybackTokens<'info> {
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dex_vault: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

/// Add tokens to liquidity pool
#[derive(Accounts)]
pub struct AddToLiquidity<'info> {
    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,
    #[account(mut)]
    pub lp_pool: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn initialize_config(
    ctx: Context<InitializeBuybackConfig>,
    burn_percent: u8,
    lp_percent: u8,
) -> Result<()> {
    require!(burn_percent + lp_percent == 100, CustomError::InvalidPercentages);
    require!(burn_percent > 0, CustomError::InvalidBurnPercent);
    require!(lp_percent > 0, CustomError::InvalidLpPercent);

    let config = &mut ctx.accounts.config;
    config.burn_percent = burn_percent;
    config.lp_percent = lp_percent;
    config.authority = ctx.accounts.authority.key();
    config.treasury = ctx.accounts.treasury.key();
    config.min_buyback_amount = 1_000_000; // 1 token with 6 decimals
    config.max_buyback_amount = 1_000_000_000; // 1000 tokens with 6 decimals
    config.cooldown_period = 3600; // 1 hour
    config.last_buyback_time = 0;

    msg!("Buyback config initialized: {}% burn, {}% LP", burn_percent, lp_percent);
    Ok(())
}

pub fn update_config(
    ctx: Context<UpdateBuybackConfig>,
    burn_percent: u8,
    lp_percent: u8,
) -> Result<()> {
    require!(burn_percent + lp_percent == 100, CustomError::InvalidPercentages);
    require!(burn_percent > 0, CustomError::InvalidBurnPercent);
    require!(lp_percent > 0, CustomError::InvalidLpPercent);

    let config = &mut ctx.accounts.config;
    config.burn_percent = burn_percent;
    config.lp_percent = lp_percent;

    msg!("Buyback config updated: {}% burn, {}% LP", burn_percent, lp_percent);
    Ok(())
}

pub fn execute_buyback(
    ctx: Context<ExecuteBuyback>,
    amount: u64,
) -> Result<()> {
    let config = &ctx.accounts.config;
    let clock = Clock::get()?;

    // Check cooldown period
    require!(
        clock.unix_timestamp - config.last_buyback_time >= config.cooldown_period,
        CustomError::BuybackCooldown
    );

    // Check amount limits
    require!(amount >= config.min_buyback_amount, CustomError::AmountTooSmall);
    require!(amount <= config.max_buyback_amount, CustomError::AmountTooLarge);

    // Check treasury balance
    require!(
        ctx.accounts.treasury.amount >= amount,
        CustomError::InsufficientTreasuryBalance
    );

    // Calculate burn and LP amounts
    let burn_amount = amount * config.burn_percent as u64 / 100;
    let lp_amount = amount - burn_amount;

    // Execute buyback from DEX (simplified - in production would use Jupiter)
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury.to_account_info(),
                to: ctx.accounts.dex_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    // Burn portion
    if burn_amount > 0 {
        token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.token_mint.to_account_info(),
                    from: ctx.accounts.dex_vault.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            burn_amount,
        )?;
    }

    // Add to LP portion
    if lp_amount > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.dex_vault.to_account_info(),
                    to: ctx.accounts.lp_pool.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            lp_amount,
        )?;
    }

    // Record execution
    let execution = &mut ctx.accounts.execution;
    execution.config = config.key();
    execution.token_mint = ctx.accounts.token_mint.key();
    execution.amount = amount;
    execution.executed_at = clock.unix_timestamp;
    execution.burn_amount = burn_amount;
    execution.lp_amount = lp_amount;

    // Update last buyback time
    let config_mut = &mut ctx.accounts.config;
    config_mut.last_buyback_time = clock.unix_timestamp;

    msg!(
        "Buyback executed: {} tokens ({} burned, {} to LP)",
        amount,
        burn_amount,
        lp_amount
    );

    Ok(())
}

pub fn transfer_authority(
    ctx: Context<UpdateBuybackConfig>,
    new_authority: Pubkey,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require_keys_eq!(
        ctx.accounts.authority.key(),
        config.authority,
        CustomError::Unauthorized
    );
    config.authority = new_authority;
    msg!("Buyback authority transferred to: {}", new_authority);
    Ok(())
}

// Helper function to burn tokens directly
pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.treasury.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;
    msg!("Burned {} tokens", amount);
    Ok(())
}

// Helper function to transfer tokens to DEX
pub fn buyback_tokens(ctx: Context<BuybackTokens>, amount: u64) -> Result<()> {
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury.to_account_info(),
                to: ctx.accounts.dex_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;
    msg!("Transferred {} tokens to DEX for buyback", amount);
    Ok(())
}

// Helper function to add tokens to liquidity pool
pub fn add_to_liquidity(ctx: Context<AddToLiquidity>, amount: u64) -> Result<()> {
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury.to_account_info(),
                to: ctx.accounts.lp_pool.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;
    msg!("Added {} tokens to liquidity pool", amount);
    Ok(())
}

#[error_code]
pub enum CustomError {
    #[msg("Invalid percentages - must sum to 100")]
    InvalidPercentages,
    #[msg("Invalid burn percentage - must be greater than 0")]
    InvalidBurnPercent,
    #[msg("Invalid LP percentage - must be greater than 0")]
    InvalidLpPercent,
    #[msg("Buyback cooldown period not met")]
    BuybackCooldown,
    #[msg("Amount too small for buyback")]
    AmountTooSmall,
    #[msg("Amount too large for buyback")]
    AmountTooLarge,
    #[msg("Insufficient treasury balance")]
    InsufficientTreasuryBalance,
    #[msg("Unauthorized operation")]
    Unauthorized,
}
