use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Burn, Mint};
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct InitializeBuybackConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = BuybackConfig::LEN,
        seeds = [b"buyback_config", authority.key().as_ref()],
        bump
    )]
    pub buyback_config: Account<'info, BuybackConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        init,
        payer = authority,
        space = Treasury::LEN,
        seeds = [b"treasury", authority.key().as_ref()],
        bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    /// CHECK: This is the USDC reserve account
    pub reserve_usdc: AccountInfo<'info>,
    
    /// CHECK: This is the buyback vault token account
    pub buyback_vault: AccountInfo<'info>,
    
    /// CHECK: This is the LP vault token account
    pub lp_vault: AccountInfo<'info>,
    
    pub buyback_config: Account<'info, BuybackConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordAndFinalizeBuyback<'info> {
    /// Treasury PDA (holds config + pointers)
    #[account(
        mut,
        seeds = [b"treasury", treasury.authority.as_ref()],
        bump = treasury.bump,
        has_one = buyback_config
    )]
    pub treasury: Account<'info, Treasury>,

    /// Vault where bought MEME tokens were deposited (token account)
    #[account(
        mut,
        token::mint = meme_mint,
        constraint = buyback_vault.key() == treasury.buyback_vault @ CustomError::InvalidBuybackVault
    )]
    pub buyback_vault: Account<'info, TokenAccount>,

    /// The mint of the MEME token being bought
    pub meme_mint: Account<'info, Mint>,

    /// Destination LP vault (a token account under treasury or LP manager)
    #[account(
        mut,
        constraint = lp_vault.key() == treasury.lp_vault @ CustomError::InvalidLPVault
    )]
    pub lp_vault: Account<'info, TokenAccount>,

    /// The config PDA that stores burn%/lp% and authority
    #[account(
        mut,
        seeds = [b"buyback_config", authority.key().as_ref()],
        bump
    )]
    pub buyback_config: Account<'info, BuybackConfig>,

    /// Authority that signs the finalization (must match treasury.authority)
    #[account(
        constraint = authority.key() == treasury.authority @ CustomError::UnauthorizedBuyback
    )]
    pub authority: Signer<'info>,

    /// Token program
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnFromBuybackVault<'info> {
    #[account(
        mut,
        token::mint = meme_mint,
        constraint = buyback_vault.key() == treasury.buyback_vault @ CustomError::InvalidBuybackVault
    )]
    pub buyback_vault: Account<'info, TokenAccount>,
    
    pub meme_mint: Account<'info, Mint>,
    
    #[account(
        constraint = authority.key() == treasury.authority @ CustomError::UnauthorizedBuyback
    )]
    pub authority: Signer<'info>,
    
    /// Treasury account for validation
    #[account(
        seeds = [b"treasury", authority.key().as_ref()],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateBuybackConfig<'info> {
    #[account(
        mut,
        seeds = [b"buyback_config", authority.key().as_ref()],
        bump,
        constraint = authority.key() == buyback_config.authority @ CustomError::UnauthorizedBuyback
    )]
    pub buyback_config: Account<'info, BuybackConfig>,
    
    pub authority: Signer<'info>,
}

#[event]
pub struct BuybackFinalized {
    pub timestamp: i64,
    pub tx_signature: String,
    pub amount_in_usdc: u64,
    pub tokens_received: u64,
    pub burned: u64,
    pub lp_sent: u64,
    pub triggered_by: Pubkey,
}

#[event]
pub struct BuybackConfigUpdated {
    pub timestamp: i64,
    pub burn_percent: u8,
    pub lp_percent: u8,
    pub buyback_threshold: u64,
    pub buyback_frequency: i64,
    pub enabled: bool,
    pub updated_by: Pubkey,
}

/// Initialize buyback configuration
pub fn initialize_buyback_config(
    ctx: Context<InitializeBuybackConfig>,
    burn_percent: u8,
    lp_percent: u8,
    buyback_threshold: u64,
    buyback_frequency: i64,
) -> Result<()> {
    let buyback_config = &mut ctx.accounts.buyback_config;
    
    // Validate percentages
    require!(
        burn_percent.checked_add(lp_percent).unwrap_or(255) == 100,
        CustomError::InvalidBuybackPercentages
    );
    
    // Validate amounts
    require!(
        buyback_threshold >= MIN_BUYBACK_AMOUNT,
        CustomError::BuybackAmountTooSmall
    );
    require!(
        buyback_threshold <= MAX_BUYBACK_AMOUNT,
        CustomError::BuybackAmountTooLarge
    );
    require!(
        buyback_frequency > 0,
        CustomError::InvalidParameter
    );
    
    buyback_config.burn_percent = burn_percent;
    buyback_config.lp_percent = lp_percent;
    buyback_config.authority = ctx.accounts.authority.key();
    buyback_config.enabled = true;
    buyback_config.buyback_threshold = buyback_threshold;
    buyback_config.buyback_frequency = buyback_frequency;
    buyback_config.last_buyback_time = 0;
    buyback_config.total_buybacks_executed = 0;
    buyback_config.total_usdc_spent = 0;
    buyback_config.total_tokens_bought = 0;
    buyback_config.total_tokens_burned = 0;
    buyback_config.total_tokens_lp = 0;
    
    Ok(())
}

/// Initialize treasury
pub fn initialize_treasury(
    ctx: Context<InitializeTreasury>,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    
    treasury.authority = ctx.accounts.authority.key();
    treasury.reserve_usdc = ctx.accounts.reserve_usdc.key();
    treasury.buyback_vault = ctx.accounts.buyback_vault.key();
    treasury.lp_vault = ctx.accounts.lp_vault.key();
    treasury.buyback_config = ctx.accounts.buyback_config.key();
    treasury.total_usdc_spent = 0;
    treasury.total_tokens_bought = 0;
    treasury.total_tokens_burned = 0;
    treasury.total_tokens_lp = 0;
    treasury.bump = *ctx.bumps.get("treasury").unwrap();
    treasury.created_at = Clock::get()?.unix_timestamp;
    
    Ok(())
}

/// Main instruction: called AFTER off-chain executor has swapped and transferred `tokens_received` to `buyback_vault`.
/// This instruction will:
/// 1) Verify caller is treasury authority
/// 2) Compute burn_amount and lp_amount using BuybackConfig
/// 3) Burn burn_amount from buyback_vault
/// 4) Transfer lp_amount into lp_vault for later LP insertion
/// 5) Update treasury totals and emit BuybackFinalized
pub fn record_and_finalize_buyback(
    ctx: Context<RecordAndFinalizeBuyback>,
    tx_signature: String,
    amount_in_usdc: u64,
    tokens_received: u64,
) -> Result<()> {
    let treasury = &mut ctx.accounts.treasury;
    let config = &mut ctx.accounts.buyback_config;
    let current_time = Clock::get()?.unix_timestamp;

    // Authority check
    require_keys_eq!(
        ctx.accounts.authority.key(),
        treasury.authority,
        CustomError::UnauthorizedBuyback
    );

    // Check if buyback is enabled
    require!(config.enabled, CustomError::BuybackDisabled);

    // Check buyback frequency
    require!(
        current_time - config.last_buyback_time >= config.buyback_frequency,
        CustomError::BuybackTooFrequent
    );

    // Validate config
    require!(
        config.burn_percent.checked_add(config.lp_percent).unwrap_or(255) == 100,
        CustomError::InvalidBuybackPercentages
    );

    // Validate amounts
    require!(amount_in_usdc >= config.buyback_threshold, CustomError::BuybackAmountTooSmall);
    require!(amount_in_usdc <= MAX_BUYBACK_AMOUNT, CustomError::BuybackAmountTooLarge);
    require!(tokens_received > 0, CustomError::NoTokensToProcess);

    // Compute splits (u128 intermediates to prevent overflow)
    let tokens_u128 = tokens_received as u128;
    let burn_amount = tokens_u128
        .checked_mul(config.burn_percent as u128)
        .ok_or(CustomError::BuybackMathOverflow)?
        .checked_div(100u128)
        .ok_or(CustomError::BuybackMathOverflow)?;

    let lp_amount = tokens_u128.checked_sub(burn_amount).ok_or(CustomError::BuybackMathOverflow)?;

    // Burn tokens: CPI to token::burn
    if burn_amount > 0 {
        let burn_amt_u64 = burn_amount as u64;
        let cpi_accounts = Burn {
            mint: ctx.accounts.meme_mint.to_account_info(),
            from: ctx.accounts.buyback_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::burn(CpiContext::new(cpi_program, cpi_accounts), burn_amt_u64)?;
    }

    // Transfer LP portion to lp_vault
    if lp_amount > 0 {
        let lp_amt_u64 = lp_amount as u64;
        let cpi_accounts = Transfer {
            from: ctx.accounts.buyback_vault.to_account_info(),
            to: ctx.accounts.lp_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), lp_amt_u64)?;
    }

    // Update treasury totals
    treasury.total_usdc_spent = treasury
        .total_usdc_spent
        .checked_add(amount_in_usdc as u128)
        .ok_or(CustomError::BuybackMathOverflow)?;
    treasury.total_tokens_bought = treasury
        .total_tokens_bought
        .checked_add(tokens_received as u128)
        .ok_or(CustomError::BuybackMathOverflow)?;
    treasury.total_tokens_burned = treasury
        .total_tokens_burned
        .checked_add(burn_amount)
        .ok_or(CustomError::BuybackMathOverflow)?;
    treasury.total_tokens_lp = treasury
        .total_tokens_lp
        .checked_add(lp_amount)
        .ok_or(CustomError::BuybackMathOverflow)?;

    // Update buyback config totals
    config.last_buyback_time = current_time;
    config.total_buybacks_executed = config.total_buybacks_executed.checked_add(1)
        .ok_or(CustomError::BuybackMathOverflow)?;
    config.total_usdc_spent = config.total_usdc_spent
        .checked_add(amount_in_usdc as u128)
        .ok_or(CustomError::BuybackMathOverflow)?;
    config.total_tokens_bought = config.total_tokens_bought
        .checked_add(tokens_received as u128)
        .ok_or(CustomError::BuybackMathOverflow)?;
    config.total_tokens_burned = config.total_tokens_burned
        .checked_add(burn_amount)
        .ok_or(CustomError::BuybackMathOverflow)?;
    config.total_tokens_lp = config.total_tokens_lp
        .checked_add(lp_amount)
        .ok_or(CustomError::BuybackMathOverflow)?;

    // Emit event for indexers
    emit!(BuybackFinalized {
        timestamp: current_time,
        tx_signature,
        amount_in_usdc,
        tokens_received,
        burned: burn_amount as u64,
        lp_sent: lp_amount as u64,
        triggered_by: ctx.accounts.authority.key(),
    });

    Ok(())
}

/// Update buyback configuration
pub fn update_buyback_config(
    ctx: Context<UpdateBuybackConfig>,
    burn_percent: Option<u8>,
    lp_percent: Option<u8>,
    buyback_threshold: Option<u64>,
    buyback_frequency: Option<i64>,
    enabled: Option<bool>,
) -> Result<()> {
    let config = &mut ctx.accounts.buyback_config;
    
    // Update burn and LP percentages if provided
    if let (Some(burn), Some(lp)) = (burn_percent, lp_percent) {
        require!(
            burn.checked_add(lp).unwrap_or(255) == 100,
            CustomError::InvalidBuybackPercentages
        );
        config.burn_percent = burn;
        config.lp_percent = lp;
    }
    
    // Update threshold if provided
    if let Some(threshold) = buyback_threshold {
        require!(
            threshold >= MIN_BUYBACK_AMOUNT,
            CustomError::BuybackAmountTooSmall
        );
        require!(
            threshold <= MAX_BUYBACK_AMOUNT,
            CustomError::BuybackAmountTooLarge
        );
        config.buyback_threshold = threshold;
    }
    
    // Update frequency if provided
    if let Some(frequency) = buyback_frequency {
        require!(frequency > 0, CustomError::InvalidParameter);
        config.buyback_frequency = frequency;
    }
    
    // Update enabled status if provided
    if let Some(enabled_status) = enabled {
        config.enabled = enabled_status;
    }
    
    // Emit event
    emit!(BuybackConfigUpdated {
        timestamp: Clock::get()?.unix_timestamp,
        burn_percent: config.burn_percent,
        lp_percent: config.lp_percent,
        buyback_threshold: config.buyback_threshold,
        buyback_frequency: config.buyback_frequency,
        enabled: config.enabled,
        updated_by: ctx.accounts.authority.key(),
    });
    
    Ok(())
}

/// Emergency burn function for manual token burning
pub fn burn_from_buyback_vault(
    ctx: Context<BurnFromBuybackVault>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, CustomError::NoTokensToProcess);
    
    let cpi_accounts = Burn {
        mint: ctx.accounts.meme_mint.to_account_info(),
        from: ctx.accounts.buyback_vault.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::burn(CpiContext::new(cpi_program, cpi_accounts), amount)?;
    
    Ok(())
}
