use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, MintTo, InitializeMint, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod buyback;
pub mod security;
pub mod security_utils;
pub mod fair_voting_utils;
pub mod fair_voting_management;
pub mod phase_1_3_structures;

use instructions::*;
use state::*;
use buyback::*;
use security::*;

#[program]
pub mod solana_memes {
    use super::*;

    /// Initialize the memecoin program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Register a creator with required stake/fee
    pub fn register_creator(ctx: Context<RegisterCreator>, stake_amount: u64) -> Result<()> {
        instructions::register_creator::handler(ctx, stake_amount)
    }

    /// Create a new memecoin with vesting and anti-rug protection
    pub fn create_token(
        ctx: Context<CreateToken>,
        name: String,
        symbol: String,
        uri: String,
        decimals: u8,
        total_supply: u64,
        creator_percent: u8,
        vesting_seconds: i64,
    ) -> Result<()> {
        instructions::create_token::handler(
            ctx,
            name,
            symbol,
            uri,
            decimals,
            total_supply,
            creator_percent,
            vesting_seconds,
        )
    }

    /// Transfer memecoin tokens
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::transfer_tokens::handler(ctx, amount)
    }

    /// Stake tokens to earn rewards
    pub fn stake_tokens(
        ctx: Context<StakeTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::stake_tokens::handler(ctx, amount)
    }

    /// Unstake tokens
    pub fn unstake_tokens(
        ctx: Context<UnstakeTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::unstake_tokens::handler(ctx, amount)
    }

    /// Claim staking rewards
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards::handler(ctx)
    }

    /// Claim vested tokens
    pub fn claim_vested(ctx: Context<ClaimVested>) -> Result<()> {
        instructions::claim_vested::handler(ctx)
    }

    /// Create a governance proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        proposal_type: ProposalType,
    ) -> Result<()> {
        instructions::create_proposal::handler(ctx, title, description, proposal_type)
    }

    /// Vote on a proposal
    pub fn vote(
        ctx: Context<Vote>,
        proposal_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        instructions::vote::handler(ctx, proposal_id, vote_type)
    }

    /// Execute a passed proposal
    pub fn execute_proposal(
        ctx: Context<ExecuteProposal>,
        proposal_id: u64,
    ) -> Result<()> {
        instructions::execute_proposal::handler(ctx, proposal_id)
    }

    /// Mint additional tokens (only for token creator with restrictions)
    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::mint_tokens::handler(ctx, amount)
    }

    /// Burn tokens
    pub fn burn_tokens(
        ctx: Context<BurnTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::burn_tokens::handler(ctx, amount)
    }

    /// Initialize buyback configuration
    pub fn initialize_buyback_config(
        ctx: Context<buyback_management::InitializeBuybackConfig>,
        burn_percent: u8,
        lp_percent: u8,
        buyback_threshold: u64,
        buyback_frequency: i64,
    ) -> Result<()> {
        buyback_management::initialize_buyback_config(ctx, burn_percent, lp_percent, buyback_threshold, buyback_frequency)
    }

    pub fn initialize_treasury(
        ctx: Context<buyback_management::InitializeTreasury>,
    ) -> Result<()> {
        buyback_management::initialize_treasury(ctx)
    }

    pub fn record_and_finalize_buyback(
        ctx: Context<buyback_management::RecordAndFinalizeBuyback>,
        tx_signature: String,
        amount_in_usdc: u64,
        tokens_received: u64,
    ) -> Result<()> {
        buyback_management::record_and_finalize_buyback(ctx, tx_signature, amount_in_usdc, tokens_received)
    }

    /// Update buyback configuration
    pub fn update_buyback_config(
        ctx: Context<buyback_management::UpdateBuybackConfig>,
        burn_percent: Option<u8>,
        lp_percent: Option<u8>,
        buyback_threshold: Option<u64>,
        buyback_frequency: Option<i64>,
        enabled: Option<bool>,
    ) -> Result<()> {
        buyback_management::update_buyback_config(ctx, burn_percent, lp_percent, buyback_threshold, buyback_frequency, enabled)
    }

    /// Burn tokens from buyback vault
    pub fn burn_from_buyback_vault(
        ctx: Context<buyback_management::BurnFromBuybackVault>,
        amount: u64,
    ) -> Result<()> {
        buyback_management::burn_from_buyback_vault(ctx, amount)
    }

    /// Execute buyback and burn
    pub fn execute_buyback(
        ctx: Context<ExecuteBuyback>,
        amount: u64,
    ) -> Result<()> {
        buyback::execute_buyback(ctx, amount)
    }

    /// Transfer buyback authority
    pub fn transfer_buyback_authority(
        ctx: Context<UpdateBuybackConfig>,
        new_authority: Pubkey,
    ) -> Result<()> {
        buyback::transfer_authority(ctx, new_authority)
    }

    /// Create launch pass NFT (required for token creation)
    pub fn create_launch_pass(
        ctx: Context<CreateLaunchPass>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::create_launch_pass::handler(ctx, name, symbol, uri)
    }

    /// Add liquidity to DEX
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        token_amount: u64,
        sol_amount: u64,
    ) -> Result<()> {
        instructions::add_liquidity::handler(ctx, token_amount, sol_amount)
    }

    /// Remove liquidity from DEX
    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        lp_amount: u64,
    ) -> Result<()> {
        instructions::remove_liquidity::handler(ctx, lp_amount)
    }

    /// Report suspicious activity
    pub fn report_activity(
        ctx: Context<ReportActivity>,
        token_mint: Pubkey,
        activity_type: ActivityType,
        description: String,
    ) -> Result<()> {
        instructions::report_activity::handler(ctx, token_mint, activity_type, description)
    }

    /// Update creator reputation
    pub fn update_reputation(
        ctx: Context<UpdateReputation>,
        creator: Pubkey,
        reputation_change: i32,
    ) -> Result<()> {
        instructions::update_reputation::handler(ctx, creator, reputation_change)
    }

    /// Choose vesting distribution option
    pub fn choose_vesting_option(
        ctx: Context<ChooseVestingOption>,
        option: VestingOption,
    ) -> Result<()> {
        instructions::choose_vesting_option::handler(ctx, option)
    }

    /// Collect trading fee
    pub fn collect_trading_fee(
        ctx: Context<CollectTradingFee>,
        trade_amount: u64,
    ) -> Result<()> {
        instructions::collect_trading_fee::handler(ctx, trade_amount)
    }
    
    /// Validate creator limits
    pub fn validate_creator_limits(
        ctx: Context<ValidateCreatorLimits>,
    ) -> Result<()> {
        instructions::validate_creator_limits::handler(ctx)
    }
    
    /// Create liquidity bootstrapping pool
    pub fn create_lbm_pool(
        ctx: Context<CreateLBMPool>,
        target_liquidity: u64,
        bootstrap_duration: i64,
        price_discovery_period: i64,
        max_participation_per_wallet: u64,
        min_participation: u64,
        max_participation: u64,
        anti_bot_enabled: bool,
    ) -> Result<()> {
        instructions::create_lbm_pool::handler(
            ctx,
            target_liquidity,
            bootstrap_duration,
            price_discovery_period,
            max_participation_per_wallet,
            min_participation,
            max_participation,
            anti_bot_enabled,
        )
    }
    
    /// Participate in liquidity bootstrapping pool
    pub fn participate_lbm(
        ctx: Context<ParticipateLBM>,
        participation_amount: u64,
    ) -> Result<()> {
        instructions::participate_lbm::handler(ctx, participation_amount)
    }
    
    /// Finalize liquidity bootstrapping pool
    pub fn finalize_lbm(
        ctx: Context<FinalizeLBM>,
    ) -> Result<()> {
        instructions::finalize_lbm::handler(ctx)
    }

    /// Update trading fee (governance controlled)
    pub fn update_trading_fee(
        ctx: Context<UpdateTradingFee>,
        new_fee: u8,
        reason: String,
    ) -> Result<()> {
        instructions::update_trading_fee::handler(ctx, new_fee, reason)
    }

    /// Execute fee change after governance approval
    pub fn execute_fee_change(
        ctx: Context<ExecuteFeeChange>,
    ) -> Result<()> {
        instructions::update_trading_fee::execute_fee_change(ctx)
    }

    /// Cancel pending fee change
    pub fn cancel_fee_change(
        ctx: Context<CancelFeeChange>,
    ) -> Result<()> {
        instructions::update_trading_fee::cancel_fee_change(ctx)
    }

    /// Emergency pause the program
    pub fn emergency_pause(
        ctx: Context<EmergencyPause>,
        reason: String,
    ) -> Result<()> {
        security::emergency_pause(ctx, reason)
    }

    /// Emergency unpause the program
    pub fn emergency_unpause(ctx: Context<EmergencyPause>) -> Result<()> {
        security::emergency_unpause(ctx)
    }

    /// Update access control
    pub fn update_access_control(
        ctx: Context<UpdateAccessControl>,
        new_admin: Option<Pubkey>,
        new_emergency_authority: Option<Pubkey>,
        new_treasury_authority: Option<Pubkey>,
        add_moderator: Option<Pubkey>,
        remove_moderator: Option<Pubkey>,
    ) -> Result<()> {
        security::update_access_control(
            ctx,
            new_admin,
            new_emergency_authority,
            new_treasury_authority,
            add_moderator,
            remove_moderator,
        )
    }

    // Fair voting management instructions
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
        instructions::fair_voting_management::initialize_fair_voting_safeguards(
            ctx,
            staked_amount_weight,
            staking_duration_weight,
            community_contribution_weight,
            token_holding_weight,
            max_voting_power_per_wallet,
            whale_voting_discount,
            max_concentration_percent,
            whale_cooldown_period,
            minimum_staking_duration,
            minimum_staked_amount,
            lock_period_during_voting,
            short_term_multiplier,
            medium_term_multiplier,
            long_term_multiplier,
            very_long_multiplier,
            suspicious_activity_threshold,
        )
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
        instructions::fair_voting_management::initialize_enhanced_token_holder(
            ctx,
            staked_amount,
            staking_duration,
            community_contribution,
            token_holding,
            consistency_score,
            reputation_score,
            participation_history,
            contribution_quality,
        )
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
        instructions::fair_voting_management::update_voting_power(
            ctx,
            new_staked_amount,
            new_staking_duration,
            new_community_contribution,
            new_token_holding,
            new_consistency_score,
            new_reputation_score,
            new_participation_history,
            new_contribution_quality,
        )
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
        instructions::fair_voting_management::initialize_creator_performance(
            ctx,
            token_price_performance,
            trading_volume,
            community_growth,
            staking_participation,
            community_satisfaction,
            marketing_efforts,
            community_engagement,
            transparency_score,
            assessment_start_time,
            assessment_end_time,
        )
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
        instructions::fair_voting_management::update_creator_performance(
            ctx,
            token_price_performance,
            trading_volume,
            community_growth,
            staking_participation,
            community_satisfaction,
            marketing_efforts,
            community_engagement,
            transparency_score,
        )
    }

    pub fn initialize_appeal_system(
        ctx: Context<InitializeAppealSystem>,
        appeal_threshold: u8,
        appeal_period: i64,
        appeal_review_panel_size: u8,
        appeal_fee: u64,
    ) -> Result<()> {
        instructions::fair_voting_management::initialize_appeal_system(
            ctx,
            appeal_threshold,
            appeal_period,
            appeal_review_panel_size,
            appeal_fee,
        )
    }

    pub fn submit_appeal(
        ctx: Context<SubmitAppeal>,
        original_decision: Pubkey,
        appeal_reason: String,
        evidence_provided: String,
    ) -> Result<()> {
        instructions::fair_voting_management::submit_appeal(
            ctx,
            original_decision,
            appeal_reason,
            evidence_provided,
        )
    }

    pub fn initialize_detection_system(
        ctx: Context<InitializeDetectionSystem>,
        whale_threshold: u64,
        whale_coordination_threshold: u64,
        suspicious_pattern_threshold: u64,
    ) -> Result<()> {
        instructions::fair_voting_management::initialize_detection_system(
            ctx,
            whale_threshold,
            whale_coordination_threshold,
            suspicious_pattern_threshold,
        )
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
        instructions::fair_voting_management::create_suspicious_activity_alert(
            ctx,
            alert_type,
            target_wallet,
            suspicious_activity,
            evidence,
            evidence_strength,
            historical_violations,
        )
    }

    pub fn initialize_penalty_system(
        ctx: Context<InitializePenaltySystem>,
        warning_penalty: u64,
        restriction_penalty: u64,
        voting_ban_penalty: u64,
        token_confiscation_penalty: u64,
    ) -> Result<()> {
        instructions::fair_voting_management::initialize_penalty_system(
            ctx,
            warning_penalty,
            restriction_penalty,
            voting_ban_penalty,
            token_confiscation_penalty,
        )
    }

    pub fn issue_penalty(
        ctx: Context<IssuePenalty>,
        offender: Pubkey,
        penalty_type: PenaltyType,
        reason: String,
        evidence: String,
        risk_score: u8,
    ) -> Result<()> {
        instructions::fair_voting_management::issue_penalty(
            ctx,
            offender,
            penalty_type,
            reason,
            evidence,
            risk_score,
        )
    }

    // Phase 1-3 Management Instructions
    pub fn initialize_user_dashboard(ctx: Context<InitializeUserDashboard>) -> Result<()> {
        instructions::phase_1_3_management::initialize_user_dashboard(ctx)
    }
    
    pub fn initialize_multi_sig_creator(
        ctx: Context<InitializeMultiSigCreator>,
        required_signatures: u8,
        signers: Vec<Pubkey>,
        creator_threshold: u64,
        time_lock_duration: i64,
        max_transaction_amount: u64,
    ) -> Result<()> {
        instructions::phase_1_3_management::initialize_multi_sig_creator(
            ctx,
            required_signatures,
            signers,
            creator_threshold,
            time_lock_duration,
            max_transaction_amount,
        )
    }
    
    pub fn initialize_social_features(
        ctx: Context<InitializeSocialFeatures>,
        username: String,
        display_name: String,
        bio: String,
        avatar_uri: String,
        banner_uri: String,
    ) -> Result<()> {
        instructions::phase_1_3_management::initialize_social_features(
            ctx,
            username,
            display_name,
            bio,
            avatar_uri,
            banner_uri,
        )
    }
    
    pub fn initialize_gamification_system(ctx: Context<InitializeGamificationSystem>) -> Result<()> {
        instructions::phase_1_3_management::initialize_gamification_system(ctx)
    }
    
    pub fn initialize_advanced_staking(ctx: Context<InitializeAdvancedStaking>) -> Result<()> {
        instructions::phase_1_3_management::initialize_advanced_staking(ctx)
    }
}
