use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;
use crate::fair_voting_utils::FairVotingUtils;

#[derive(Accounts)]
pub struct InitializeFairVotingSafeguards<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<FairVotingSafeguards>(),
        seeds = [b"fair_voting_safeguards"],
        bump
    )]
    pub fair_voting_safeguards: Account<'info, FairVotingSafeguards>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_fair_voting_safeguards(
    ctx: Context<InitializeFairVotingSafeguards>,
    staked_amount_weight: u8,
    staking_duration_weight: u8,
    community_contribution_weight: u8,
    token_holding_weight: u8,
    max_voting_power_per_wallet: u64,
    whale_voting_discount: u8,
    max_concentration_percent: u8,
    whale_cooldown_period: i64,
    minimum_staking_duration: i64,
    minimum_staked_amount: u64,
    lock_period_during_voting: i64,
    short_term_multiplier: u8,
    medium_term_multiplier: u8,
    long_term_multiplier: u8,
    very_long_multiplier: u8,
    suspicious_activity_threshold: u64,
) -> Result<()> {
    let fair_voting_safeguards = &mut ctx.accounts.fair_voting_safeguards;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize fair voting safeguards
    fair_voting_safeguards.authority = ctx.accounts.authority.key();
    
    // Multi-factor voting weights
    fair_voting_safeguards.staked_amount_weight = staked_amount_weight;
    fair_voting_safeguards.staking_duration_weight = staking_duration_weight;
    fair_voting_safeguards.community_contribution_weight = community_contribution_weight;
    fair_voting_safeguards.token_holding_weight = token_holding_weight;
    
    // Anti-whale mechanisms
    fair_voting_safeguards.max_voting_power_per_wallet = max_voting_power_per_wallet;
    fair_voting_safeguards.whale_voting_discount = whale_voting_discount;
    fair_voting_safeguards.max_concentration_percent = max_concentration_percent;
    fair_voting_safeguards.whale_cooldown_period = whale_cooldown_period;
    
    // Time-based voting requirements
    fair_voting_safeguards.minimum_staking_duration = minimum_staking_duration;
    fair_voting_safeguards.minimum_staked_amount = minimum_staked_amount;
    fair_voting_safeguards.lock_period_during_voting = lock_period_during_voting;
    
    // Duration multipliers
    fair_voting_safeguards.short_term_multiplier = short_term_multiplier;
    fair_voting_safeguards.medium_term_multiplier = medium_term_multiplier;
    fair_voting_safeguards.long_term_multiplier = long_term_multiplier;
    fair_voting_safeguards.very_long_multiplier = very_long_multiplier;
    
    // Detection and monitoring
    fair_voting_safeguards.suspicious_activity_threshold = suspicious_activity_threshold;
    fair_voting_safeguards.manipulation_detection_enabled = true;
    fair_voting_safeguards.automated_monitoring_enabled = true;
    
    fair_voting_safeguards.created_at = current_time;
    fair_voting_safeguards.updated_at = current_time;
    
    // Validate configuration
    FairVotingUtils::validate_safeguards_config(fair_voting_safeguards)?;
    
    msg!("Fair voting safeguards initialized successfully!");
    msg!("Authority: {}", ctx.accounts.authority.key());
    msg!("Max voting power per wallet: {}", max_voting_power_per_wallet);
    msg!("Whale voting discount: {}%", whale_voting_discount);
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeEnhancedTokenHolder<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<EnhancedTokenHolder>(),
        seeds = [b"enhanced_token_holder", authority.key().as_ref()],
        bump
    )]
    pub enhanced_token_holder: Account<'info, EnhancedTokenHolder>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_enhanced_token_holder(
    ctx: Context<InitializeEnhancedTokenHolder>,
    staked_amount: u64,
    staking_duration: i64,
    community_contribution: u64,
    token_holding: u64,
    consistency_score: u64,
    reputation_score: i32,
    participation_history: u64,
    contribution_quality: u64,
) -> Result<()> {
    let enhanced_token_holder = &mut ctx.accounts.enhanced_token_holder;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize enhanced token holder
    enhanced_token_holder.holder = ctx.accounts.authority.key();
    enhanced_token_holder.balance = 0;
    enhanced_token_holder.voting_power = 0;
    
    // Multi-factor voting components
    enhanced_token_holder.staked_amount = staked_amount;
    enhanced_token_holder.staking_duration = staking_duration;
    enhanced_token_holder.community_contribution_score = community_contribution;
    enhanced_token_holder.token_holding_amount = token_holding;
    
    // Anti-manipulation factors
    enhanced_token_holder.consistency_score = consistency_score;
    enhanced_token_holder.reputation_score = reputation_score;
    enhanced_token_holder.participation_history = participation_history;
    enhanced_token_holder.contribution_quality = contribution_quality;
    
    // Time-based factors
    enhanced_token_holder.first_stake_time = current_time;
    enhanced_token_holder.last_stake_time = current_time;
    enhanced_token_holder.total_staking_time = 0;
    
    // Whale detection
    enhanced_token_holder.is_whale = false;
    enhanced_token_holder.whale_discount_applied = false;
    enhanced_token_holder.concentration_percentage = 0;
    
    // Voting restrictions
    enhanced_token_holder.voting_restricted = false;
    enhanced_token_holder.restriction_reason = "".to_string();
    enhanced_token_holder.restriction_until = 0;
    
    enhanced_token_holder.last_vote = 0;
    enhanced_token_holder.is_delegated = false;
    enhanced_token_holder.delegate = None;
    enhanced_token_holder.created_at = current_time;
    enhanced_token_holder.updated_at = current_time;
    
    msg!("Enhanced token holder initialized successfully!");
    msg!("Holder: {}", ctx.accounts.authority.key());
    msg!("Staked amount: {}", staked_amount);
    msg!("Reputation score: {}", reputation_score);
    
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateVotingPower<'info> {
    #[account(mut)]
    pub enhanced_token_holder: Account<'info, EnhancedTokenHolder>,
    
    #[account(mut)]
    pub fair_voting_safeguards: Account<'info, FairVotingSafeguards>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn update_voting_power(
    ctx: Context<UpdateVotingPower>,
    new_staked_amount: u64,
    new_staking_duration: i64,
    new_community_contribution: u64,
    new_token_holding: u64,
    new_consistency_score: u64,
    new_reputation_score: i32,
    new_participation_history: u64,
    new_contribution_quality: u64,
) -> Result<()> {
    let enhanced_token_holder = &mut ctx.accounts.enhanced_token_holder;
    let fair_voting_safeguards = &ctx.accounts.fair_voting_safeguards;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate authority
    require!(
        enhanced_token_holder.holder == ctx.accounts.authority.key(),
        CustomError::UnauthorizedFeeChange
    );
    
    // Update token holder data
    enhanced_token_holder.staked_amount = new_staked_amount;
    enhanced_token_holder.staking_duration = new_staking_duration;
    enhanced_token_holder.community_contribution_score = new_community_contribution;
    enhanced_token_holder.token_holding_amount = new_token_holding;
    enhanced_token_holder.consistency_score = new_consistency_score;
    enhanced_token_holder.reputation_score = new_reputation_score;
    enhanced_token_holder.participation_history = new_participation_history;
    enhanced_token_holder.contribution_quality = new_contribution_quality;
    
    // Calculate new voting power
    let new_voting_power = FairVotingUtils::calculate_fair_voting_power(
        new_staked_amount,
        new_staking_duration,
        new_community_contribution,
        new_token_holding,
        new_consistency_score,
        new_reputation_score,
        new_participation_history,
        new_contribution_quality,
        fair_voting_safeguards,
    )?;
    
    enhanced_token_holder.voting_power = new_voting_power;
    enhanced_token_holder.updated_at = current_time;
    
    msg!("Voting power updated successfully!");
    msg!("New voting power: {}", new_voting_power);
    msg!("Staked amount: {}", new_staked_amount);
    msg!("Reputation score: {}", new_reputation_score);
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCreatorPerformance<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<CreatorPerformance>(),
        seeds = [b"creator_performance", creator.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub creator_performance: Account<'info, CreatorPerformance>,
    
    /// CHECK: Creator account
    pub creator: AccountInfo<'info>,
    
    /// CHECK: Token mint account
    pub token_mint: AccountInfo<'info>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_creator_performance(
    ctx: Context<InitializeCreatorPerformance>,
    token_price_performance: u64,
    trading_volume: u64,
    community_growth: u64,
    staking_participation: u64,
    community_satisfaction: u64,
    marketing_efforts: u64,
    community_engagement: u64,
    transparency_score: u64,
    assessment_start_time: i64,
    assessment_end_time: i64,
) -> Result<()> {
    let creator_performance = &mut ctx.accounts.creator_performance;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize creator performance
    creator_performance.creator = ctx.accounts.creator.key();
    creator_performance.token_mint = ctx.accounts.token_mint.key();
    
    // Quantitative metrics (70% of decision)
    creator_performance.token_price_performance = token_price_performance;
    creator_performance.trading_volume = trading_volume;
    creator_performance.community_growth = community_growth;
    creator_performance.staking_participation = staking_participation;
    
    // Qualitative metrics (30% of decision)
    creator_performance.community_satisfaction = community_satisfaction;
    creator_performance.marketing_efforts = marketing_efforts;
    creator_performance.community_engagement = community_engagement;
    creator_performance.transparency_score = transparency_score;
    
    // Performance tracking
    creator_performance.performance_score = 0; // Will be calculated
    creator_performance.target_metrics = HashMap::new();
    creator_performance.performance_history = Vec::new();
    
    // Assessment periods
    creator_performance.assessment_start_time = assessment_start_time;
    creator_performance.assessment_end_time = assessment_end_time;
    creator_performance.last_assessment_time = current_time;
    
    // Community feedback
    creator_performance.community_feedback_count = 0;
    creator_performance.positive_feedback_percentage = 0;
    creator_performance.negative_feedback_percentage = 0;
    
    creator_performance.created_at = current_time;
    creator_performance.updated_at = current_time;
    
    // Calculate initial performance score
    let performance_score = FairVotingUtils::calculate_performance_score(creator_performance)?;
    creator_performance.performance_score = performance_score;
    
    msg!("Creator performance initialized successfully!");
    msg!("Creator: {}", ctx.accounts.creator.key());
    msg!("Token: {}", ctx.accounts.token_mint.key());
    msg!("Performance score: {}", performance_score);
    
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateCreatorPerformance<'info> {
    #[account(mut)]
    pub creator_performance: Account<'info, CreatorPerformance>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn update_creator_performance(
    ctx: Context<UpdateCreatorPerformance>,
    token_price_performance: u64,
    trading_volume: u64,
    community_growth: u64,
    staking_participation: u64,
    community_satisfaction: u64,
    marketing_efforts: u64,
    community_engagement: u64,
    transparency_score: u64,
) -> Result<()> {
    let creator_performance = &mut ctx.accounts.creator_performance;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Update performance metrics
    creator_performance.token_price_performance = token_price_performance;
    creator_performance.trading_volume = trading_volume;
    creator_performance.community_growth = community_growth;
    creator_performance.staking_participation = staking_participation;
    creator_performance.community_satisfaction = community_satisfaction;
    creator_performance.marketing_efforts = marketing_efforts;
    creator_performance.community_engagement = community_engagement;
    creator_performance.transparency_score = transparency_score;
    
    // Calculate new performance score
    let new_performance_score = FairVotingUtils::calculate_performance_score(creator_performance)?;
    
    // Update performance history
    creator_performance.performance_history.push(creator_performance.performance_score);
    creator_performance.performance_score = new_performance_score;
    creator_performance.last_assessment_time = current_time;
    creator_performance.updated_at = current_time;
    
    // Determine release percentage
    let release_percentage = FairVotingUtils::determine_creator_release_percentage(new_performance_score)?;
    
    msg!("Creator performance updated successfully!");
    msg!("New performance score: {}", new_performance_score);
    msg!("Release percentage: {}%", release_percentage);
    msg!("Creator: {}", creator_performance.creator);
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeAppealSystem<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<AppealSystem>(),
        seeds = [b"appeal_system"],
        bump
    )]
    pub appeal_system: Account<'info, AppealSystem>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_appeal_system(
    ctx: Context<InitializeAppealSystem>,
    appeal_threshold: u8,
    appeal_period: i64,
    appeal_review_panel_size: u8,
    appeal_fee: u64,
) -> Result<()> {
    let appeal_system = &mut ctx.accounts.appeal_system;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize appeal system
    appeal_system.authority = ctx.accounts.authority.key();
    
    // Appeal mechanisms
    appeal_system.appeal_threshold = appeal_threshold;
    appeal_system.appeal_period = appeal_period;
    appeal_system.appeal_review_panel_size = appeal_review_panel_size;
    appeal_system.appeal_fee = appeal_fee;
    
    // Oversight mechanisms
    appeal_system.governance_oversight_enabled = true;
    appeal_system.external_auditors_enabled = true;
    appeal_system.community_oversight_enabled = true;
    appeal_system.emergency_intervention_enabled = true;
    
    // Appeal tracking
    appeal_system.total_appeals = 0;
    appeal_system.successful_appeals = 0;
    appeal_system.rejected_appeals = 0;
    
    appeal_system.created_at = current_time;
    appeal_system.updated_at = current_time;
    
    msg!("Appeal system initialized successfully!");
    msg!("Appeal threshold: {}%", appeal_threshold);
    msg!("Appeal period: {} days", appeal_period / 86400);
    msg!("Review panel size: {}", appeal_review_panel_size);
    
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitAppeal<'info> {
    #[account(
        init,
        payer = appellant,
        space = 8 + std::mem::size_of::<Appeal>(),
        seeds = [b"appeal", appeal_system.total_appeals.to_le_bytes().as_ref()],
        bump
    )]
    pub appeal: Account<'info, Appeal>,
    
    #[account(mut)]
    pub appeal_system: Account<'info, AppealSystem>,
    
    pub appellant: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn submit_appeal(
    ctx: Context<SubmitAppeal>,
    original_decision: Pubkey,
    appeal_reason: String,
    evidence_provided: String,
) -> Result<()> {
    let appeal = &mut ctx.accounts.appeal;
    let appeal_system = &mut ctx.accounts.appeal_system;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate appeal data
    require!(appeal_reason.len() <= 1000, CustomError::InvalidFeeProposal);
    require!(evidence_provided.len() <= 2000, CustomError::InvalidFeeProposal);
    
    // Initialize appeal
    appeal.appeal_id = appeal_system.total_appeals + 1;
    appeal.appellant = ctx.accounts.appellant.key();
    appeal.original_decision = original_decision;
    appeal.appeal_reason = appeal_reason;
    appeal.evidence_provided = evidence_provided;
    
    // Appeal status
    appeal.status = AppealStatus::Pending;
    appeal.submitted_at = current_time;
    appeal.review_started_at = None;
    appeal.decision_at = None;
    appeal.decision = None;
    appeal.decision_reason = None;
    
    // Review panel (empty initially)
    appeal.review_panel = Vec::new();
    appeal.panel_votes = Vec::new();
    
    appeal.created_at = current_time;
    appeal.updated_at = current_time;
    
    // Update appeal system
    appeal_system.total_appeals += 1;
    appeal_system.updated_at = current_time;
    
    msg!("Appeal submitted successfully!");
    msg!("Appeal ID: {}", appeal.appeal_id);
    msg!("Appellant: {}", ctx.accounts.appellant.key());
    msg!("Original decision: {}", original_decision);
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeDetectionSystem<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<DetectionSystem>(),
        seeds = [b"detection_system"],
        bump
    )]
    pub detection_system: Account<'info, DetectionSystem>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_detection_system(
    ctx: Context<InitializeDetectionSystem>,
    whale_threshold: u64,
    whale_coordination_threshold: u64,
    suspicious_pattern_threshold: u64,
) -> Result<()> {
    let detection_system = &mut ctx.accounts.detection_system;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize detection system
    detection_system.authority = ctx.accounts.authority.key();
    
    // Whale detection
    detection_system.whale_detection_enabled = true;
    detection_system.whale_threshold = whale_threshold;
    detection_system.whale_coordination_threshold = whale_coordination_threshold;
    
    // Suspicious activity detection
    detection_system.suspicious_activity_enabled = true;
    detection_system.suspicious_pattern_threshold = suspicious_pattern_threshold;
    detection_system.vote_pattern_analysis_enabled = true;
    
    // Collusion detection
    detection_system.collusion_detection_enabled = true;
    detection_system.bribery_detection_enabled = true;
    detection_system.vote_selling_detection_enabled = true;
    
    // Monitoring results
    detection_system.total_alerts = 0;
    detection_system.confirmed_manipulations = 0;
    detection_system.false_positives = 0;
    
    detection_system.created_at = current_time;
    detection_system.updated_at = current_time;
    
    msg!("Detection system initialized successfully!");
    msg!("Whale threshold: {}", whale_threshold);
    msg!("Coordination threshold: {}", whale_coordination_threshold);
    msg!("Pattern threshold: {}", suspicious_pattern_threshold);
    
    Ok(())
}

#[derive(Accounts)]
pub struct CreateSuspiciousActivityAlert<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<SuspiciousActivityAlert>(),
        seeds = [b"suspicious_alert", detection_system.total_alerts.to_le_bytes().as_ref()],
        bump
    )]
    pub alert: Account<'info, SuspiciousActivityAlert>,
    
    #[account(mut)]
    pub detection_system: Account<'info, DetectionSystem>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_suspicious_activity_alert(
    ctx: Context<CreateSuspiciousActivityAlert>,
    alert_type: AlertType,
    target_wallet: Pubkey,
    suspicious_activity: String,
    evidence: String,
    evidence_strength: u8,
    historical_violations: u64,
) -> Result<()> {
    let alert = &mut ctx.accounts.alert;
    let detection_system = &mut ctx.accounts.detection_system;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate alert data
    require!(suspicious_activity.len() <= 500, CustomError::InvalidFeeProposal);
    require!(evidence.len() <= 1000, CustomError::InvalidFeeProposal);
    require!(evidence_strength <= 100, CustomError::RISK_SCORE_INVALID);
    
    // Initialize alert
    alert.alert_id = detection_system.total_alerts + 1;
    alert.alert_type = alert_type;
    alert.target_wallet = target_wallet;
    alert.suspicious_activity = suspicious_activity;
    alert.evidence = evidence;
    
    // Calculate risk score
    let risk_score = FairVotingUtils::calculate_risk_score(
        &alert_type,
        evidence_strength,
        historical_violations,
    )?;
    alert.risk_score = risk_score;
    
    // Alert status
    alert.status = AlertStatus::Open;
    alert.created_at = current_time;
    alert.reviewed_at = None;
    alert.resolved_at = None;
    alert.resolution = None;
    
    // Investigation
    alert.investigator = None;
    alert.investigation_notes = "".to_string();
    alert.action_taken = "".to_string();
    
    alert.updated_at = current_time;
    
    // Update detection system
    detection_system.total_alerts += 1;
    detection_system.updated_at = current_time;
    
    msg!("Suspicious activity alert created successfully!");
    msg!("Alert ID: {}", alert.alert_id);
    msg!("Alert type: {:?}", alert_type);
    msg!("Target wallet: {}", target_wallet);
    msg!("Risk score: {}", risk_score);
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializePenaltySystem<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<PenaltySystem>(),
        seeds = [b"penalty_system"],
        bump
    )]
    pub penalty_system: Account<'info, PenaltySystem>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize_penalty_system(
    ctx: Context<InitializePenaltySystem>,
    warning_penalty: u64,
    restriction_penalty: u64,
    voting_ban_penalty: u64,
    token_confiscation_penalty: u64,
) -> Result<()> {
    let penalty_system = &mut ctx.accounts.penalty_system;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Initialize penalty system
    penalty_system.authority = ctx.accounts.authority.key();
    
    // Penalty types and amounts
    penalty_system.warning_penalty = warning_penalty;
    penalty_system.restriction_penalty = restriction_penalty;
    penalty_system.voting_ban_penalty = voting_ban_penalty;
    penalty_system.token_confiscation_penalty = token_confiscation_penalty;
    
    // Penalty tracking
    penalty_system.total_penalties_issued = 0;
    penalty_system.total_penalty_amount = 0;
    penalty_system.active_penalties = 0;
    
    penalty_system.created_at = current_time;
    penalty_system.updated_at = current_time;
    
    msg!("Penalty system initialized successfully!");
    msg!("Warning penalty: {}", warning_penalty);
    msg!("Restriction penalty: {}", restriction_penalty);
    msg!("Voting ban penalty: {}", voting_ban_penalty);
    msg!("Token confiscation penalty: {}", token_confiscation_penalty);
    
    Ok(())
}

#[derive(Accounts)]
pub struct IssuePenalty<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Penalty>(),
        seeds = [b"penalty", penalty_system.total_penalties_issued.to_le_bytes().as_ref()],
        bump
    )]
    pub penalty: Account<'info, Penalty>,
    
    #[account(mut)]
    pub penalty_system: Account<'info, PenaltySystem>,
    
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn issue_penalty(
    ctx: Context<IssuePenalty>,
    offender: Pubkey,
    penalty_type: PenaltyType,
    reason: String,
    evidence: String,
    risk_score: u8,
) -> Result<()> {
    let penalty = &mut ctx.accounts.penalty;
    let penalty_system = &mut ctx.accounts.penalty_system;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate penalty data
    require!(reason.len() <= 500, CustomError::InvalidFeeProposal);
    require!(evidence.len() <= 1000, CustomError::InvalidFeeProposal);
    require!(risk_score <= 100, CustomError::RISK_SCORE_INVALID);
    
    // Determine penalty amount based on type
    let penalty_amount = match penalty_type {
        PenaltyType::Warning => penalty_system.warning_penalty,
        PenaltyType::VotingRestriction => penalty_system.restriction_penalty,
        PenaltyType::VotingBan => penalty_system.voting_ban_penalty,
        PenaltyType::TokenConfiscation => penalty_system.token_confiscation_penalty,
        PenaltyType::TemporaryBan => penalty_system.voting_ban_penalty * 2,
        PenaltyType::PermanentBan => penalty_system.token_confiscation_penalty * 2,
    };
    
    // Initialize penalty
    penalty.penalty_id = penalty_system.total_penalties_issued + 1;
    penalty.offender = offender;
    penalty.penalty_type = penalty_type;
    penalty.penalty_amount = penalty_amount;
    penalty.reason = reason;
    penalty.evidence = evidence;
    
    // Penalty status
    penalty.status = PenaltyStatus::Active;
    penalty.issued_at = current_time;
    penalty.expires_at = None; // Will be set based on penalty type
    penalty.paid_at = None;
    
    // Appeal information
    penalty.appealed = false;
    penalty.appeal_id = None;
    
    penalty.created_at = current_time;
    penalty.updated_at = current_time;
    
    // Update penalty system
    penalty_system.total_penalties_issued += 1;
    penalty_system.total_penalty_amount += penalty_amount;
    penalty_system.active_penalties += 1;
    penalty_system.updated_at = current_time;
    
    msg!("Penalty issued successfully!");
    msg!("Penalty ID: {}", penalty.penalty_id);
    msg!("Offender: {}", offender);
    msg!("Penalty type: {:?}", penalty_type);
    msg!("Penalty amount: {}", penalty_amount);
    msg!("Risk score: {}", risk_score);
    
    Ok(())
}
