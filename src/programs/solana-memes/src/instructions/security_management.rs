use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use crate::state::*;
use crate::errors::CustomError;
use crate::security_utils::SecurityUtils;

/// Initialize circuit breaker for price and volume protection
#[derive(Accounts)]
pub struct InitializeCircuitBreaker<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<CircuitBreaker>(),
        seeds = [b"circuit_breaker"],
        bump
    )]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_circuit_breaker(ctx: Context<InitializeCircuitBreaker>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    *ctx.accounts.circuit_breaker = SecurityUtils::initialize_circuit_breaker(
        ctx.accounts.authority.key(),
        current_time,
    );
    
    Ok(())
}

/// Initialize trade protection for flash loan and suspicious activity detection
#[derive(Accounts)]
pub struct InitializeTradeProtection<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<TradeProtection>(),
        seeds = [b"trade_protection"],
        bump
    )]
    pub trade_protection: Account<'info, TradeProtection>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_trade_protection(ctx: Context<InitializeTradeProtection>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    *ctx.accounts.trade_protection = SecurityUtils::initialize_trade_protection(
        ctx.accounts.authority.key(),
        current_time,
    );
    
    Ok(())
}

/// Initialize multi-signature governance for critical operations
#[derive(Accounts)]
pub struct InitializeMultiSigGovernance<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<MultiSigGovernance>(),
        seeds = [b"multi_sig_governance"],
        bump
    )]
    pub multi_sig_governance: Account<'info, MultiSigGovernance>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_multi_sig_governance(
    ctx: Context<InitializeMultiSigGovernance>,
    signers: Vec<Pubkey>,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    require!(
        signers.len() >= 3 && signers.len() <= 10,
        CustomError::InvalidTokenAmount
    );
    
    *ctx.accounts.multi_sig_governance = SecurityUtils::initialize_multi_sig_governance(
        ctx.accounts.authority.key(),
        signers,
        current_time,
    );
    
    Ok(())
}

/// Initialize emergency controls
#[derive(Accounts)]
pub struct InitializeEmergencyControls<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<EmergencyControls>(),
        seeds = [b"emergency_controls"],
        bump
    )]
    pub emergency_controls: Account<'info, EmergencyControls>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_emergency_controls(ctx: Context<InitializeEmergencyControls>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    *ctx.accounts.emergency_controls = SecurityUtils::initialize_emergency_controls(
        ctx.accounts.authority.key(),
        current_time,
    );
    
    Ok(())
}

/// Trigger emergency pause
#[derive(Accounts)]
pub struct TriggerEmergencyPause<'info> {
    #[account(
        mut,
        seeds = [b"emergency_controls"],
        bump,
        has_one = authority
    )]
    pub emergency_controls: Account<'info, EmergencyControls>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn trigger_emergency_pause(
    ctx: Context<TriggerEmergencyPause>,
    reason: String,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    ctx.accounts.emergency_controls.emergency_pause = true;
    ctx.accounts.emergency_controls.pause_reason = reason;
    ctx.accounts.emergency_controls.pause_initiated_by = Some(ctx.accounts.authority.key());
    ctx.accounts.emergency_controls.pause_time = Some(current_time);
    ctx.accounts.emergency_controls.updated_at = current_time;
    
    Ok(())
}

/// Resume from emergency pause
#[derive(Accounts)]
pub struct ResumeFromEmergencyPause<'info> {
    #[account(
        mut,
        seeds = [b"emergency_controls"],
        bump,
        has_one = authority
    )]
    pub emergency_controls: Account<'info, EmergencyControls>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn resume_from_emergency_pause(ctx: Context<ResumeFromEmergencyPause>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    ctx.accounts.emergency_controls.emergency_pause = false;
    ctx.accounts.emergency_controls.pause_reason = String::new();
    ctx.accounts.emergency_controls.pause_initiated_by = None;
    ctx.accounts.emergency_controls.pause_time = None;
    ctx.accounts.emergency_controls.auto_resume_time = None;
    ctx.accounts.emergency_controls.updated_at = current_time;
    
    Ok(())
}

/// Update circuit breaker parameters
#[derive(Accounts)]
pub struct UpdateCircuitBreaker<'info> {
    #[account(
        mut,
        seeds = [b"circuit_breaker"],
        bump,
        has_one = authority
    )]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn update_circuit_breaker(
    ctx: Context<UpdateCircuitBreaker>,
    max_price_change_percent: Option<u8>,
    max_volume_per_period: Option<u64>,
    cooldown_period: Option<i64>,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    if let Some(percent) = max_price_change_percent {
        require!(percent <= 100, CustomError::InvalidTokenAmount);
        ctx.accounts.circuit_breaker.max_price_change_percent = percent;
    }
    
    if let Some(volume) = max_volume_per_period {
        require!(volume > 0, CustomError::InvalidTokenAmount);
        ctx.accounts.circuit_breaker.max_volume_per_period = volume;
    }
    
    if let Some(cooldown) = cooldown_period {
        require!(cooldown > 0, CustomError::InvalidTokenAmount);
        ctx.accounts.circuit_breaker.cooldown_period = cooldown;
    }
    
    ctx.accounts.circuit_breaker.updated_at = current_time;
    
    Ok(())
}

/// Update trade protection parameters
#[derive(Accounts)]
pub struct UpdateTradeProtection<'info> {
    #[account(
        mut,
        seeds = [b"trade_protection"],
        bump,
        has_one = authority
    )]
    pub trade_protection: Account<'info, TradeProtection>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn update_trade_protection(
    ctx: Context<UpdateTradeProtection>,
    min_trade_interval: Option<i64>,
    max_trade_amount: Option<u64>,
    max_daily_volume: Option<u64>,
    suspicious_pattern_threshold: Option<u64>,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    if let Some(interval) = min_trade_interval {
        require!(interval > 0, CustomError::InvalidTokenAmount);
        ctx.accounts.trade_protection.min_trade_interval = interval;
    }
    
    if let Some(amount) = max_trade_amount {
        require!(amount > 0, CustomError::InvalidTokenAmount);
        ctx.accounts.trade_protection.max_trade_amount = amount;
    }
    
    if let Some(volume) = max_daily_volume {
        require!(volume > 0, CustomError::InvalidTokenAmount);
        ctx.accounts.trade_protection.max_daily_volume = volume;
    }
    
    if let Some(threshold) = suspicious_pattern_threshold {
        require!(threshold > 0, CustomError::InvalidTokenAmount);
        ctx.accounts.trade_protection.suspicious_pattern_threshold = threshold;
    }
    
    ctx.accounts.trade_protection.updated_at = current_time;
    
    Ok(())
}

/// Manually trigger circuit breaker
#[derive(Accounts)]
pub struct ManualTriggerCircuitBreaker<'info> {
    #[account(
        mut,
        seeds = [b"circuit_breaker"],
        bump,
        has_one = authority
    )]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn manual_trigger_circuit_breaker(
    ctx: Context<ManualTriggerCircuitBreaker>,
    current_price: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    SecurityUtils::trigger_circuit_breaker(
        &mut ctx.accounts.circuit_breaker,
        current_time,
        current_price,
    )?;
    
    ctx.accounts.circuit_breaker.updated_at = current_time;
    
    Ok(())
}

/// Reset circuit breaker
#[derive(Accounts)]
pub struct ResetCircuitBreaker<'info> {
    #[account(
        mut,
        seeds = [b"circuit_breaker"],
        bump,
        has_one = authority
    )]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn reset_circuit_breaker(ctx: Context<ResetCircuitBreaker>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    SecurityUtils::reset_circuit_breaker(
        &mut ctx.accounts.circuit_breaker,
        current_time,
    )?;
    
    ctx.accounts.circuit_breaker.updated_at = current_time;
    
    Ok(())
}
