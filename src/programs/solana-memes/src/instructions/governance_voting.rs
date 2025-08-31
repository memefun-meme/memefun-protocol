use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct CreateGovernanceProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, GovernanceProposal>,
    
    #[account(mut)]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    #[account(mut)]
    pub creator: Account<'info, TokenHolder>,
    
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_proposal(
    ctx: Context<CreateGovernanceProposal>,
    title: String,
    description: String,
    proposal_type: ProposalType,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let governance_config = &ctx.accounts.governance_config;
    let creator = &ctx.accounts.creator;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if governance is active
    require!(governance_config.is_active, CustomError::ProposalNotActive);
    
    // Validate creator has minimum voting power
    require!(
        creator.voting_power >= governance_config.min_voting_power,
        CustomError::InsufficientVotingPower
    );
    
    // Validate proposal data
    require!(title.len() <= 200, CustomError::InvalidFeeProposal);
    require!(description.len() <= 2000, CustomError::InvalidFeeProposal);
    
    // Initialize proposal
    proposal.id = proposal.id + 1; // Increment proposal ID
    proposal.creator = ctx.accounts.authority.key();
    proposal.title = title;
    proposal.description = description;
    proposal.proposal_type = proposal_type;
    proposal.start_time = current_time;
    proposal.end_time = current_time + governance_config.voting_period;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.total_votes = 0;
    proposal.quorum_required = (governance_config.proposal_quorum_percentage as u64 * INITIAL_SUPPLY) / 100; // 10% of total supply
    proposal.quorum_met = false;
    proposal.executed = false;
    proposal.executed_at = None;
    proposal.executed_by = None;
    proposal.created_at = current_time;
    proposal.updated_at = current_time;
    
    msg!("Governance proposal created successfully!");
    msg!("Proposal ID: {}", proposal.id);
    msg!("Voting Period: {} days", governance_config.voting_period / 86400);
    msg!("Quorum Required: {} tokens", proposal.quorum_required);
    
    Ok(())
}

#[derive(Accounts)]
pub struct VoteOnProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, GovernanceProposal>,
    
    #[account(mut)]
    pub vote: Account<'info, Vote>,
    
    #[account(mut)]
    pub voter: Account<'info, TokenHolder>,
    
    #[account(mut)]
    pub voter_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn vote(
    ctx: Context<VoteOnProposal>,
    vote_type: VoteType,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let vote = &mut ctx.accounts.vote;
    let voter = &ctx.accounts.voter;
    let governance_config = &ctx.accounts.governance_config;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if governance is active
    require!(governance_config.is_active, CustomError::ProposalNotActive);
    
    // Check if proposal is active
    require!(
        current_time >= proposal.start_time && current_time <= proposal.end_time,
        CustomError::VotingPeriodEnded
    );
    
    // Check if voter has minimum voting power
    require!(
        voter.voting_power >= governance_config.min_voting_power,
        CustomError::InsufficientVotingPower
    );
    
    // Check if voter hasn't already voted
    require!(vote.voter == Pubkey::default(), CustomError::AlreadyVoted);
    
    // Record the vote
    vote.proposal_id = proposal.id;
    vote.voter = ctx.accounts.authority.key();
    vote.vote_type = vote_type;
    vote.voting_power = voter.voting_power;
    vote.voted_at = current_time;
    
    // Update proposal vote counts
    match vote_type {
        VoteType::Yes => {
            proposal.yes_votes += voter.voting_power;
        },
        VoteType::No => {
            proposal.no_votes += voter.voting_power;
        },
        VoteType::Abstain => {
            // Abstain votes don't count towards quorum but are recorded
        },
    }
    
    proposal.total_votes += voter.voting_power;
    proposal.updated_at = current_time;
    
    // Check if quorum is met
    if proposal.total_votes >= proposal.quorum_required {
        proposal.quorum_met = true;
    }
    
    // Update voter's last vote time
    // Note: In a real implementation, you'd update the voter's last_vote field
    
    msg!("Vote recorded successfully!");
    msg!("Vote Type: {:?}", vote_type);
    msg!("Voting Power: {}", voter.voting_power);
    msg!("Total Votes: {}", proposal.total_votes);
    msg!("Quorum Met: {}", proposal.quorum_met);
    
    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub proposal: Account<'info, GovernanceProposal>,
    
    #[account(mut)]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    #[account(mut)]
    pub executor: Account<'info, TokenHolder>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let governance_config = &ctx.accounts.governance_config;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if governance is active
    require!(governance_config.is_active, CustomError::ProposalNotActive);
    
    // Check if proposal voting has ended
    require!(
        current_time > proposal.end_time,
        CustomError::VotingPeriodEnded
    );
    
    // Check if proposal hasn't been executed
    require!(!proposal.executed, CustomError::ProposalAlreadyExecuted);
    
    // Check if quorum was met
    require!(proposal.quorum_met, CustomError::QuorumNotMet);
    
    // Check if proposal passed (yes votes > no votes)
    require!(
        proposal.yes_votes > proposal.no_votes,
        CustomError::QuorumNotMet
    );
    
    // Check if execution delay has passed
    require!(
        current_time >= proposal.end_time + governance_config.execution_delay,
        CustomError::VotingPeriodEnded
    );
    
    // Execute the proposal based on type
    match proposal.proposal_type {
        ProposalType::FeeChange => {
            // Fee change execution logic would be here
            msg!("Executing fee change proposal");
        },
        ProposalType::TokenCreation => {
            msg!("Executing token creation proposal");
        },
        ProposalType::BuybackConfig => {
            msg!("Executing buyback config proposal");
        },
        ProposalType::TreasuryAllocation => {
            msg!("Executing treasury allocation proposal");
        },
        ProposalType::GovernanceRule => {
            msg!("Executing governance rule proposal");
        },
        ProposalType::EmergencyAction => {
            msg!("Executing emergency action proposal");
        },
    }
    
    // Mark proposal as executed
    proposal.executed = true;
    proposal.executed_at = Some(current_time);
    proposal.executed_by = Some(ctx.accounts.authority.key());
    proposal.updated_at = current_time;
    
    msg!("Proposal executed successfully!");
    msg!("Executed by: {}", ctx.accounts.authority.key());
    msg!("Execution time: {}", current_time);
    
    Ok(())
}

#[derive(Accounts)]
pub struct DelegateVotingPower<'info> {
    #[account(mut)]
    pub delegator: Account<'info, TokenHolder>,
    
    #[account(mut)]
    pub delegate: Account<'info, TokenHolder>,
    
    #[account(mut)]
    pub delegation: Account<'info, Delegation>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn delegate_voting_power(
    ctx: Context<DelegateVotingPower>,
    amount: u64,
) -> Result<()> {
    let delegator = &mut ctx.accounts.delegator;
    let delegate = &mut ctx.accounts.delegate;
    let delegation = &mut ctx.accounts.delegation;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate delegator
    require!(
        delegator.holder == ctx.accounts.authority.key(),
        CustomError::UnauthorizedFeeChange
    );
    
    // Check if delegator has enough tokens
    require!(
        delegator.balance >= amount,
        CustomError::InsufficientStakeAmount
    );
    
    // Check if delegation doesn't already exist
    require!(
        delegation.delegator == Pubkey::default(),
        CustomError::AlreadyVoted
    );
    
    // Create delegation
    delegation.delegator = ctx.accounts.authority.key();
    delegation.delegate = delegate.holder;
    delegation.amount = amount;
    delegation.is_active = true;
    delegation.created_at = current_time;
    delegation.updated_at = current_time;
    
    // Update delegator
    delegator.is_delegated = true;
    delegator.delegate = Some(delegate.holder);
    delegator.voting_power = delegator.balance - amount;
    delegator.updated_at = current_time;
    
    // Update delegate
    delegate.voting_power += amount;
    delegate.updated_at = current_time;
    
    msg!("Voting power delegated successfully!");
    msg!("Delegated Amount: {}", amount);
    msg!("Delegator Voting Power: {}", delegator.voting_power);
    msg!("Delegate Voting Power: {}", delegate.voting_power);
    
    Ok(())
}
