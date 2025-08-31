use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, MintTo, InitializeMint};
use crate::state::*;
use crate::errors::CustomError;
use crate::security_utils::SecurityUtils;

#[derive(Accounts)]
pub struct DeployGovernanceToken<'info> {
    #[account(mut)]
    pub governance_token: Account<'info, GovernanceToken>,
    
    #[account(mut)]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<DeployGovernanceToken>) -> Result<()> {
    let governance_token = &mut ctx.accounts.governance_token;
    let governance_config = &mut ctx.accounts.governance_config;
    let token_mint = &mut ctx.accounts.token_mint;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize governance token mint
    let cpi_accounts = InitializeMint {
        mint: token_mint.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::initialize_mint(
        cpi_ctx,
        GOVERNANCE_TOKEN_DECIMALS,
        &ctx.accounts.authority.key(),
        Some(&ctx.accounts.authority.key()),
    )?;
    
    // Initialize governance token account
    governance_token.mint = token_mint.key();
    governance_token.authority = ctx.accounts.authority.key();
    governance_token.total_supply = INITIAL_SUPPLY;
    governance_token.circulating_supply = 0;
    governance_token.is_active = false; // Will be activated after initial distribution
    governance_token.created_at = current_time;
    governance_token.updated_at = current_time;
    
    // Initialize governance configuration
    governance_config.authority = ctx.accounts.authority.key();
    governance_config.governance_token = governance_token.key();
    governance_config.min_voting_power = MIN_VOTING_POWER;
    governance_config.proposal_quorum_percentage = PROPOSAL_QUORUM_PERCENTAGE;
    governance_config.voting_period = VOTING_PERIOD;
    governance_config.execution_delay = 86400; // 24 hours
    governance_config.emergency_threshold = INITIAL_SUPPLY / 100; // 1% of total supply (5M tokens)
    governance_config.is_active = false; // Will be activated after token distribution
    governance_config.created_at = current_time;
    governance_config.updated_at = current_time;
    
    msg!("Governance token deployed successfully!");
    msg!("Token Mint: {}", token_mint.key());
    msg!("Total Supply: {}", INITIAL_SUPPLY);
    msg!("Decimals: {}", GOVERNANCE_TOKEN_DECIMALS);
    
    Ok(())
}

#[derive(Accounts)]
pub struct ActivateGovernance<'info> {
    #[account(mut)]
    pub governance_token: Account<'info, GovernanceToken>,
    
    #[account(mut)]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn activate_governance(ctx: Context<ActivateGovernance>) -> Result<()> {
    let governance_token = &mut ctx.accounts.governance_token;
    let governance_config = &mut ctx.accounts.governance_config;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate authority
    require!(
        governance_token.authority == ctx.accounts.authority.key(),
        CustomError::UnauthorizedFeeChange
    );
    
    // Check if minimum distribution has occurred
    require!(
        governance_token.circulating_supply >= INITIAL_SUPPLY / 10, // At least 10% distributed (50M tokens)
        CustomError::InvalidFeeProposal
    );
    
    // Activate governance
    governance_token.is_active = true;
    governance_config.is_active = true;
    governance_token.updated_at = current_time;
    governance_config.updated_at = current_time;
    
    msg!("Governance system activated!");
    msg!("Circulating Supply: {}", governance_token.circulating_supply);
    msg!("Minimum Voting Power: {}", governance_config.min_voting_power);
    msg!("Quorum Percentage: {}%", governance_config.proposal_quorum_percentage);
    
    Ok(())
}

#[derive(Accounts)]
pub struct DistributeGovernanceTokens<'info> {
    #[account(mut)]
    pub governance_token: Account<'info, GovernanceToken>,
    
    #[account(mut)]
    pub recipient: Account<'info, TokenHolder>,
    
    #[account(mut)]
    pub recipient_token_account: Account<'info, token::TokenAccount>,
    
    #[account(mut)]
    pub treasury_token_account: Account<'info, token::TokenAccount>,
    
    /// Multi-signature governance for secure distribution
    #[account(
        seeds = [b"multi_sig_governance"],
        bump
    )]
    pub multi_sig_governance: Account<'info, MultiSigGovernance>,
    
    /// Emergency controls
    #[account(
        seeds = [b"emergency_controls"],
        bump,
        constraint = !emergency_controls.emergency_pause @ CustomError::EmergencyPauseActive
    )]
    pub emergency_controls: Account<'info, EmergencyControls>,
    
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn distribute_tokens(
    ctx: Context<DistributeGovernanceTokens>,
    amount: u64,
    distribution_type: DistributionType,
) -> Result<()> {
    let governance_token = &mut ctx.accounts.governance_token;
    let recipient = &mut ctx.accounts.recipient;
    let current_time = Clock::get()?.unix_timestamp;
    
    // SECURITY CHECKS
    // Check emergency pause
    SecurityUtils::check_emergency_pause(&ctx.accounts.emergency_controls)?;
    
    // Validate multi-signature distribution
    SecurityUtils::validate_multi_sig_distribution(
        amount,
        &[], // Empty signatures for now - in real implementation, pass actual signatures
        &ctx.accounts.multi_sig_governance,
        current_time,
    )?;
    
    // Validate authority
    require!(
        governance_token.authority == ctx.accounts.authority.key(),
        CustomError::UnauthorizedFeeChange
    );
    
    // Check if governance is not yet active
    require!(!governance_token.is_active, CustomError::InvalidFeeProposal);
    
    // Validate distribution amount with security limits
    require!(
        governance_token.circulating_supply + amount <= governance_token.total_supply,
        CustomError::InvalidTokenSupply
    );
    
    // Check distribution threshold
    require!(
        amount <= ctx.accounts.multi_sig_governance.distribution_threshold,
        CustomError::DistributionTooLarge
    );
    
    // Mint tokens to recipient
    let cpi_accounts = MintTo {
        mint: ctx.accounts.governance_token.mint.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::mint_to(cpi_ctx, amount)?;
    
    // Update governance token stats
    governance_token.circulating_supply += amount;
    governance_token.updated_at = current_time;
    
    // Update or create recipient record
    if recipient.holder == Pubkey::default() {
        // New recipient
        recipient.holder = ctx.accounts.recipient.key();
        recipient.balance = amount;
        recipient.voting_power = amount;
        recipient.last_vote = 0;
        recipient.is_delegated = false;
        recipient.delegate = None;
        recipient.created_at = current_time;
        recipient.updated_at = current_time;
    } else {
        // Existing recipient
        recipient.balance += amount;
        recipient.voting_power += amount;
        recipient.updated_at = current_time;
    }
    
    msg!("Distributed {} governance tokens to {}", amount, recipient.holder);
    msg!("Distribution Type: {:?}", distribution_type);
    msg!("Circulating Supply: {}", governance_token.circulating_supply);
    
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum DistributionType {
    Team,
    Community,
    Stakers,
    Treasury,
    Liquidity,
    Marketing,
}
