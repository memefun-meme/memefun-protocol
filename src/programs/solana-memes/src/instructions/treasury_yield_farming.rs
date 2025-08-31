use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::CustomError;

// Initialize treasury yield farming system
#[derive(Accounts)]
pub struct InitializeTreasuryYieldFarming<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<TreasuryYieldFarming>(),
        seeds = [b"treasury_yield_farming"],
        bump
    )]
    pub treasury_yield: Account<'info, TreasuryYieldFarming>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_treasury_yield_farming(ctx: Context<InitializeTreasuryYieldFarming>) -> Result<()> {
    let treasury_yield = &mut ctx.accounts.treasury_yield;
    let clock = Clock::get()?;
    
    treasury_yield.authority = ctx.accounts.authority.key();
    treasury_yield.total_value_locked = 0;
    treasury_yield.total_yield_earned = 0;
    treasury_yield.staker_rewards = 0;
    treasury_yield.treasury_retention = 0;
    treasury_yield.emergency_reserve = 0;
    
    // Initialize yield distribution (70% stakers, 20% treasury, 10% emergency)
    treasury_yield.yield_distribution = YieldDistribution {
        staker_percentage: 70,
        treasury_percentage: 20,
        emergency_percentage: 10,
        last_distribution_time: clock.unix_timestamp,
        distribution_frequency: 24 * 60 * 60, // Daily
    };
    
    // Initialize risk management
    treasury_yield.risk_management = RiskManagement {
        max_total_risk: 50, // 50% max risk
        max_protocol_risk: 20, // 20% max per protocol
        max_token_allocation: 30, // 30% max per token
        emergency_withdrawal_threshold: 1_000_000_000, // 1 SOL
        circuit_breaker_enabled: true,
        insurance_fund: 0,
    };
    
    // Initialize performance metrics
    treasury_yield.performance_metrics = YieldPerformance {
        total_apy: 0.0,
        daily_yield: 0,
        weekly_yield: 0,
        monthly_yield: 0,
        annual_yield: 0,
        volatility_score: 0.0,
        sharpe_ratio: 0.0,
        max_drawdown: 0.0,
    };
    
    treasury_yield.created_at = clock.unix_timestamp;
    treasury_yield.updated_at = clock.unix_timestamp;
    
    Ok(())
}

// Add DeFi protocol integration
#[derive(Accounts)]
pub struct AddDeFiProtocol<'info> {
    #[account(mut, has_one = authority)]
    pub treasury_yield: Account<'info, TreasuryYieldFarming>,
    pub authority: Signer<'info>,
}

pub fn add_defi_protocol(
    ctx: Context<AddDeFiProtocol>,
    protocol_name: String,
    protocol_address: Pubkey,
    supported_tokens: Vec<Pubkey>,
    min_apy: f64,
    max_apy: f64,
    risk_level: RiskLevel,
) -> Result<()> {
    let treasury_yield = &mut ctx.accounts.treasury_yield;
    
    let defi_protocol = DeFiProtocol {
        protocol_name,
        protocol_address,
        supported_tokens,
        apy_range: (min_apy, max_apy),
        risk_level,
        is_active: true,
        total_deposited: 0,
        total_earned: 0,
    };
    
    treasury_yield.supported_protocols.push(defi_protocol);
    treasury_yield.updated_at = Clock::get()?.unix_timestamp;
    
    Ok(())
}

// Create yield farming position
#[derive(Accounts)]
pub struct CreateYieldPosition<'info> {
    #[account(mut, has_one = authority)]
    pub treasury_yield: Account<'info, TreasuryYieldFarming>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn create_yield_position(
    ctx: Context<CreateYieldPosition>,
    protocol: Pubkey,
    token_mint: Pubkey,
    amount: u64,
    strategy_id: u64,
) -> Result<()> {
    let treasury_yield = &mut ctx.accounts.treasury_yield;
    let clock = Clock::get()?;
    
    // Validate protocol exists and is active
    let protocol_info = treasury_yield.supported_protocols
        .iter()
        .find(|p| p.protocol_address == protocol && p.is_active)
        .ok_or(CustomError::ProtocolNotFound)?;
    
    // Check risk management limits
    let total_risk = calculate_total_risk(treasury_yield)?;
    require!(
        total_risk <= treasury_yield.risk_management.max_total_risk as u64,
        CustomError::RiskLimitExceeded
    );
    
    // Check protocol risk limit
    let protocol_risk = calculate_protocol_risk(treasury_yield, protocol)?;
    require!(
        protocol_risk <= treasury_yield.risk_management.max_protocol_risk as u64,
        CustomError::ProtocolRiskLimitExceeded
    );
    
    // Create yield position
    let position_id = treasury_yield.active_positions.len() as u64 + 1;
    let yield_position = YieldPosition {
        position_id,
        protocol,
        token_mint,
        amount_deposited: amount,
        current_value: amount,
        yield_earned: 0,
        apy: protocol_info.apy_range.0, // Start with min APY
        start_time: clock.unix_timestamp,
        end_time: None,
        is_active: true,
        risk_score: risk_level_to_score(&protocol_info.risk_level),
    };
    
    treasury_yield.active_positions.push(yield_position);
    treasury_yield.total_value_locked += amount;
    treasury_yield.updated_at = clock.unix_timestamp;
    
    // Transfer tokens to protocol (simplified - in real implementation, this would interact with the actual DeFi protocol)
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                to: ctx.accounts.treasury.to_account_info(), // Simplified - would be protocol address
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;
    
    msg!("Yield position created: {} tokens deposited in protocol {}", amount, protocol);
    
    Ok(())
}

// Harvest yield from positions
#[derive(Accounts)]
pub struct HarvestYield<'info> {
    #[account(mut, has_one = authority)]
    pub treasury_yield: Account<'info, TreasuryYieldFarming>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
}

pub fn harvest_yield(ctx: Context<HarvestYield>) -> Result<()> {
    let treasury_yield = &mut ctx.accounts.treasury_yield;
    let clock = Clock::get()?;
    
    let mut total_yield = 0u64;
    
    // Calculate yield from all active positions
    for position in &mut treasury_yield.active_positions {
        if position.is_active {
            let yield_amount = calculate_position_yield(position, clock.unix_timestamp)?;
            position.yield_earned += yield_amount;
            total_yield += yield_amount;
            
            // Update position value
            position.current_value += yield_amount;
        }
    }
    
    treasury_yield.total_yield_earned += total_yield;
    
    // Distribute yield according to allocation
    let staker_share = total_yield * treasury_yield.yield_distribution.staker_percentage as u64 / 100;
    let treasury_share = total_yield * treasury_yield.yield_distribution.treasury_percentage as u64 / 100;
    let emergency_share = total_yield * treasury_yield.yield_distribution.emergency_percentage as u64 / 100;
    
    treasury_yield.staker_rewards += staker_share;
    treasury_yield.treasury_retention += treasury_share;
    treasury_yield.emergency_reserve += emergency_share;
    
    // Update performance metrics
    update_performance_metrics(treasury_yield, total_yield, clock.unix_timestamp)?;
    
    treasury_yield.yield_distribution.last_distribution_time = clock.unix_timestamp;
    treasury_yield.updated_at = clock.unix_timestamp;
    
    msg!("Yield harvested: {} total, {} to stakers, {} to treasury, {} to emergency", 
         total_yield, staker_share, treasury_share, emergency_share);
    
    Ok(())
}

// Close yield position
#[derive(Accounts)]
pub struct CloseYieldPosition<'info> {
    #[account(mut, has_one = authority)]
    pub treasury_yield: Account<'info, TreasuryYieldFarming>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
}

pub fn close_yield_position(
    ctx: Context<CloseYieldPosition>,
    position_id: u64,
) -> Result<()> {
    let treasury_yield = &mut ctx.accounts.treasury_yield;
    let clock = Clock::get()?;
    
    // Find and close position
    let position_index = treasury_yield.active_positions
        .iter()
        .position(|p| p.position_id == position_id && p.is_active)
        .ok_or(CustomError::PositionNotFound)?;
    
    let position = &mut treasury_yield.active_positions[position_index];
    position.is_active = false;
    position.end_time = Some(clock.unix_timestamp);
    
    // Calculate final yield
    let final_yield = calculate_position_yield(position, clock.unix_timestamp)?;
    position.yield_earned += final_yield;
    position.current_value += final_yield;
    
    // Update total values
    treasury_yield.total_value_locked -= position.amount_deposited;
    treasury_yield.total_yield_earned += final_yield;
    
    // Distribute final yield
    let staker_share = final_yield * treasury_yield.yield_distribution.staker_percentage as u64 / 100;
    let treasury_share = final_yield * treasury_yield.yield_distribution.treasury_percentage as u64 / 100;
    let emergency_share = final_yield * treasury_yield.yield_distribution.emergency_percentage as u64 / 100;
    
    treasury_yield.staker_rewards += staker_share;
    treasury_yield.treasury_retention += treasury_share;
    treasury_yield.emergency_reserve += emergency_share;
    
    treasury_yield.updated_at = clock.unix_timestamp;
    
    msg!("Position {} closed. Final yield: {}", position_id, final_yield);
    
    Ok(())
}

// Emergency withdrawal
#[derive(Accounts)]
pub struct EmergencyWithdrawal<'info> {
    #[account(mut, has_one = authority)]
    pub treasury_yield: Account<'info, TreasuryYieldFarming>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
}

pub fn emergency_withdrawal(ctx: Context<EmergencyWithdrawal>) -> Result<()> {
    let treasury_yield = &mut ctx.accounts.treasury_yield;
    let clock = Clock::get()?;
    
    require!(
        treasury_yield.risk_management.circuit_breaker_enabled,
        CustomError::CircuitBreakerDisabled
    );
    
    // Close all active positions
    for position in &mut treasury_yield.active_positions {
        if position.is_active {
            position.is_active = false;
            position.end_time = Some(clock.unix_timestamp);
            
            // Calculate final yield
            let final_yield = calculate_position_yield(position, clock.unix_timestamp)?;
            position.yield_earned += final_yield;
            position.current_value += final_yield;
        }
    }
    
    // Reset total value locked
    treasury_yield.total_value_locked = 0;
    treasury_yield.updated_at = clock.unix_timestamp;
    
    msg!("Emergency withdrawal executed. All positions closed.");
    
    Ok(())
}

// Helper functions

fn calculate_position_yield(position: &YieldPosition, current_time: i64) -> Result<u64> {
    if !position.is_active {
        return Ok(0);
    }
    
    let time_elapsed = current_time - position.start_time;
    let daily_rate = position.apy / 365.0;
    let yield_rate = daily_rate / 86400.0; // Per second
    
    let yield_amount = (position.amount_deposited as f64 * yield_rate * time_elapsed as f64) as u64;
    
    Ok(yield_amount)
}

fn calculate_total_risk(treasury_yield: &TreasuryYieldFarming) -> Result<u64> {
    let mut total_risk = 0u64;
    
    for position in &treasury_yield.active_positions {
        if position.is_active {
            total_risk += position.risk_score as u64;
        }
    }
    
    Ok(total_risk)
}

fn calculate_protocol_risk(treasury_yield: &TreasuryYieldFarming, protocol: Pubkey) -> Result<u64> {
    let mut protocol_risk = 0u64;
    
    for position in &treasury_yield.active_positions {
        if position.is_active && position.protocol == protocol {
            protocol_risk += position.risk_score as u64;
        }
    }
    
    Ok(protocol_risk)
}

fn risk_level_to_score(risk_level: &RiskLevel) -> u8 {
    match risk_level {
        RiskLevel::Low => 1,
        RiskLevel::Medium => 5,
        RiskLevel::High => 8,
        RiskLevel::VeryHigh => 10,
    }
}

fn update_performance_metrics(
    treasury_yield: &mut Account<TreasuryYieldFarming>,
    yield_amount: u64,
    current_time: i64,
) -> Result<()> {
    let time_since_last = current_time - treasury_yield.yield_distribution.last_distribution_time;
    
    // Update daily yield
    if time_since_last >= 24 * 60 * 60 {
        treasury_yield.performance_metrics.daily_yield = yield_amount;
    }
    
    // Update weekly yield
    if time_since_last >= 7 * 24 * 60 * 60 {
        treasury_yield.performance_metrics.weekly_yield = yield_amount;
    }
    
    // Update monthly yield
    if time_since_last >= 30 * 24 * 60 * 60 {
        treasury_yield.performance_metrics.monthly_yield = yield_amount;
    }
    
    // Calculate total APY
    if treasury_yield.total_value_locked > 0 {
        let annual_yield = yield_amount * 365 * 24 * 60 * 60 / time_since_last;
        treasury_yield.performance_metrics.total_apy = 
            (annual_yield as f64 / treasury_yield.total_value_locked as f64) * 100.0;
    }
    
    Ok(())
}
