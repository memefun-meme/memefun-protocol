use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use crate::state::*;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct UpdateTradingFee<'info> {
    #[account(mut)]
    pub platform_config: Account<'info, PlatformConfig>,
    
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<UpdateTradingFee>, new_fee: u8, reason: String) -> Result<()> {
    let platform_config = &mut ctx.accounts.platform_config;
    let proposal = &mut ctx.accounts.proposal;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate authority
    require!(
        platform_config.authority == ctx.accounts.authority.key(),
        CustomError::UnauthorizedFeeChange
    );
    
    // Check if platform is paused
    require!(!platform_config.emergency_pause, CustomError::EmergencyPause);
    
    // Validate fee range
    require!(new_fee >= platform_config.min_trading_fee, CustomError::FeeTooLow);
    require!(new_fee <= platform_config.max_trading_fee, CustomError::FeeTooHigh);
    
    // Check if fee is actually changing
    require!(
        new_fee != platform_config.trading_fee_percentage,
        CustomError::InvalidFeeProposal
    );
    
    // Check cooldown period
    require!(
        current_time >= platform_config.last_fee_change + platform_config.fee_change_cooldown,
        CustomError::FeeChangeTooSoon
    );
    
    // Validate reason length
    require!(reason.len() <= 500, CustomError::InvalidFeeProposal);
    
    // Create pending fee change
    let implementation_time = current_time + FEE_IMPLEMENTATION_DELAY;
    
    let pending_change = PendingFeeChange {
        proposed_fee: new_fee,
        proposed_by: ctx.accounts.authority.key(),
        proposal_time: current_time,
        implementation_time,
        reason,
        votes_for: 0,
        votes_against: 0,
        total_votes: 0,
    };
    
    platform_config.pending_fee_change = Some(pending_change);
    platform_config.updated_at = current_time;
    
    // Create governance proposal for fee change
    proposal.id = proposal.id + 1; // Increment proposal ID
    proposal.creator = ctx.accounts.authority.key();
    proposal.title = format!("Trading Fee Change to {}%", new_fee as f64 / 10.0);
    proposal.description = format!(
        "Proposal to change trading fee from {}% to {}%",
        platform_config.trading_fee_percentage as f64 / 10.0,
        new_fee as f64 / 10.0
    );
    proposal.proposal_type = ProposalType::FeeChange;
    proposal.start_time = current_time;
    proposal.end_time = current_time + VOTING_PERIOD;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.total_votes = 0;
    proposal.quorum = platform_config.governance_quorum as u64;
    proposal.executed = false;
    
    msg!(
        "Trading fee change proposed: {}% -> {}%",
        platform_config.trading_fee_percentage as f64 / 10.0,
        new_fee as f64 / 10.0
    );
    msg!("Implementation time: {}", implementation_time);
    msg!("Voting period: {} days", VOTING_PERIOD / 86400);
    
    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteFeeChange<'info> {
    #[account(mut)]
    pub platform_config: Account<'info, PlatformConfig>,
    
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn execute_fee_change(ctx: Context<ExecuteFeeChange>) -> Result<()> {
    let platform_config = &mut ctx.accounts.platform_config;
    let proposal = &mut ctx.accounts.proposal;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate authority
    require!(
        platform_config.authority == ctx.accounts.authority.key(),
        CustomError::UnauthorizedFeeChange
    );
    
    // Check if there's a pending fee change
    require!(
        platform_config.pending_fee_change.is_some(),
        CustomError::FeeChangeNotPending
    );
    
    let pending_change = platform_config.pending_fee_change.as_ref().unwrap();
    
    // Check if implementation time has been reached
    require!(
        current_time >= pending_change.implementation_time,
        CustomError::FeeImplementationTimeNotReached
    );
    
    // Check if proposal was successful
    require!(
        proposal.executed && proposal.yes_votes > proposal.no_votes,
        CustomError::FeeChangeQuorumNotMet
    );
    
    // Execute the fee change
    let old_fee = platform_config.trading_fee_percentage;
    platform_config.trading_fee_percentage = pending_change.proposed_fee;
    platform_config.last_fee_change = current_time;
    platform_config.pending_fee_change = None;
    platform_config.updated_at = current_time;
    
    msg!(
        "Trading fee updated: {}% -> {}%",
        old_fee as f64 / 10.0,
        platform_config.trading_fee_percentage as f64 / 10.0
    );
    msg!("Fee change executed successfully");
    
    Ok(())
}

#[derive(Accounts)]
pub struct CancelFeeChange<'info> {
    #[account(mut)]
    pub platform_config: Account<'info, PlatformConfig>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn cancel_fee_change(ctx: Context<CancelFeeChange>) -> Result<()> {
    let platform_config = &mut ctx.accounts.platform_config;
    
    // Validate authority
    require!(
        platform_config.authority == ctx.accounts.authority.key(),
        CustomError::UnauthorizedFeeChange
    );
    
    // Check if there's a pending fee change
    require!(
        platform_config.pending_fee_change.is_some(),
        CustomError::FeeChangeNotPending
    );
    
    // Cancel the pending fee change
    platform_config.pending_fee_change = None;
    platform_config.updated_at = Clock::get()?.unix_timestamp;
    
    msg!("Pending fee change cancelled");
    
    Ok(())
}
