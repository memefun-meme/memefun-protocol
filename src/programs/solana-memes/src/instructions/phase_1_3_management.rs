use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;
use crate::phase_1_3_structures::*;

// User Dashboard Management
#[derive(Accounts)]
pub struct InitializeUserDashboard<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<UserDashboard>(),
        seeds = [b"user_dashboard", user.key().as_ref()],
        bump
    )]
    pub user_dashboard: Account<'info, UserDashboard>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_user_dashboard(ctx: Context<InitializeUserDashboard>) -> Result<()> {
    let user_dashboard = &mut ctx.accounts.user_dashboard;
    let clock = Clock::get()?;
    
    user_dashboard.user = ctx.accounts.user.key();
    user_dashboard.total_tokens_created = 0;
    user_dashboard.successful_tokens = 0;
    user_dashboard.failed_tokens = 0;
    user_dashboard.total_volume_traded = 0;
    user_dashboard.total_rewards_earned = 0;
    user_dashboard.active_stakes = 0;
    user_dashboard.total_staked_amount = 0;
    user_dashboard.staking_apy = 0.0;
    user_dashboard.governance_participation = 0;
    user_dashboard.proposals_created = 0;
    user_dashboard.proposals_voted_on = 0;
    user_dashboard.voting_power = 0;
    user_dashboard.community_rank = "Newcomer".to_string();
    user_dashboard.reputation_score = 0;
    user_dashboard.community_contribution_score = 0;
    user_dashboard.followers_count = 0;
    user_dashboard.following_count = 0;
    user_dashboard.total_transactions = 0;
    user_dashboard.success_rate = 0.0;
    user_dashboard.average_token_performance = 0.0;
    user_dashboard.best_performing_token = None;
    user_dashboard.last_activity = clock.unix_timestamp;
    user_dashboard.recent_transactions = Vec::new();
    user_dashboard.notifications = Vec::new();
    
    // Default preferences
    user_dashboard.dashboard_preferences = DashboardPreferences {
        theme: DashboardTheme::Auto,
        default_view: DashboardView::Overview,
        auto_refresh: true,
        refresh_interval: 300, // 5 minutes
        show_advanced_metrics: false,
        language: "en".to_string(),
    };
    
    // Default notification settings
    user_dashboard.notification_settings = NotificationSettings {
        email_notifications: true,
        push_notifications: true,
        sms_notifications: false,
        governance_alerts: true,
        price_alerts: true,
        security_alerts: true,
        marketing_notifications: false,
    };
    
    user_dashboard.created_at = clock.unix_timestamp;
    user_dashboard.updated_at = clock.unix_timestamp;
    
    Ok(())
}

// Multi-Signature Creator Management
#[derive(Accounts)]
pub struct InitializeMultiSigCreator<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + std::mem::size_of::<MultiSigCreator>(),
        seeds = [b"multi_sig_creator", creator.key().as_ref()],
        bump
    )]
    pub multi_sig_creator: Account<'info, MultiSigCreator>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_multi_sig_creator(
    ctx: Context<InitializeMultiSigCreator>,
    required_signatures: u8,
    signers: Vec<Pubkey>,
    creator_threshold: u64,
    time_lock_duration: i64,
    max_transaction_amount: u64,
) -> Result<()> {
    let multi_sig_creator = &mut ctx.accounts.multi_sig_creator;
    let clock = Clock::get()?;
    
    require!(
        required_signatures > 0 && required_signatures <= signers.len() as u8,
        CustomError::InvalidMultiSigConfiguration
    );
    
    require!(
        signers.len() <= 10, // Maximum 10 signers
        CustomError::TooManySigners
    );
    
    multi_sig_creator.creator = ctx.accounts.creator.key();
    multi_sig_creator.required_signatures = required_signatures;
    multi_sig_creator.total_signers = signers.len() as u8;
    multi_sig_creator.signers = signers;
    multi_sig_creator.creator_threshold = creator_threshold;
    multi_sig_creator.is_active = true;
    multi_sig_creator.pending_transactions = Vec::new();
    multi_sig_creator.completed_transactions = Vec::new();
    multi_sig_creator.time_lock_duration = time_lock_duration;
    multi_sig_creator.max_transaction_amount = max_transaction_amount;
    multi_sig_creator.emergency_contacts = Vec::new();
    multi_sig_creator.created_at = clock.unix_timestamp;
    multi_sig_creator.updated_at = clock.unix_timestamp;
    
    Ok(())
}

// Social Features Management
#[derive(Accounts)]
pub struct InitializeSocialFeatures<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<SocialFeatures>(),
        seeds = [b"social_features", user.key().as_ref()],
        bump
    )]
    pub social_features: Account<'info, SocialFeatures>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_social_features(
    ctx: Context<InitializeSocialFeatures>,
    username: String,
    display_name: String,
    bio: String,
    avatar_uri: String,
    banner_uri: String,
) -> Result<()> {
    let social_features = &mut ctx.accounts.social_features;
    let clock = Clock::get()?;
    
    require!(
        username.len() >= 3 && username.len() <= 20,
        CustomError::InvalidUsername
    );
    
    require!(
        display_name.len() <= 50,
        CustomError::DisplayNameTooLong
    );
    
    require!(
        bio.len() <= 500,
        CustomError::BioTooLong
    );
    
    social_features.user = ctx.accounts.user.key();
    social_features.username = username;
    social_features.display_name = display_name;
    social_features.bio = bio;
    social_features.avatar_uri = avatar_uri;
    social_features.banner_uri = banner_uri;
    social_features.followers = Vec::new();
    social_features.following = Vec::new();
    social_features.followers_count = 0;
    social_features.following_count = 0;
    social_features.community_chat_enabled = true;
    social_features.token_showcase_enabled = true;
    social_features.influencer_verification = false;
    social_features.verification_badge = false;
    social_features.hosted_events = Vec::new();
    social_features.attended_events = Vec::new();
    social_features.total_likes = 0;
    social_features.total_shares = 0;
    social_features.total_comments = 0;
    social_features.engagement_rate = 0.0;
    social_features.created_at = clock.unix_timestamp;
    social_features.updated_at = clock.unix_timestamp;
    
    Ok(())
}

// Gamification System Management
#[derive(Accounts)]
pub struct InitializeGamificationSystem<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<GamificationSystem>(),
        seeds = [b"gamification_system", user.key().as_ref()],
        bump
    )]
    pub gamification_system: Account<'info, GamificationSystem>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_gamification_system(ctx: Context<InitializeGamificationSystem>) -> Result<()> {
    let gamification_system = &mut ctx.accounts.gamification_system;
    let clock = Clock::get()?;
    
    gamification_system.user = ctx.accounts.user.key();
    gamification_system.user_level = 1;
    gamification_system.experience_points = 0;
    gamification_system.experience_to_next_level = 100;
    gamification_system.total_experience_earned = 0;
    gamification_system.achievements = Vec::new();
    gamification_system.total_achievements = 0;
    gamification_system.achievement_score = 0;
    gamification_system.leaderboard_rank = 0;
    gamification_system.leaderboard_score = 0;
    gamification_system.leaderboard_category = LeaderboardCategory::Overall;
    gamification_system.rewards_multiplier = 1.0;
    gamification_system.total_rewards_earned = 0;
    gamification_system.pending_rewards = 0;
    gamification_system.daily_challenges = Vec::new();
    gamification_system.weekly_challenges = Vec::new();
    gamification_system.completed_challenges = Vec::new();
    gamification_system.current_streak = 0;
    gamification_system.longest_streak = 0;
    gamification_system.milestones_reached = Vec::new();
    gamification_system.created_at = clock.unix_timestamp;
    gamification_system.updated_at = clock.unix_timestamp;
    
    Ok(())
}

// Advanced Staking Management
#[derive(Accounts)]
pub struct InitializeAdvancedStaking<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<AdvancedStaking>(),
        seeds = [b"advanced_staking", user.key().as_ref()],
        bump
    )]
    pub advanced_staking: Account<'info, AdvancedStaking>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_advanced_staking(ctx: Context<InitializeAdvancedStaking>) -> Result<()> {
    let advanced_staking = &mut ctx.accounts.advanced_staking;
    let clock = Clock::get()?;
    
    advanced_staking.user = ctx.accounts.user.key();
    advanced_staking.flexible_stakes = Vec::new();
    advanced_staking.total_flexible_staked = 0;
    advanced_staking.flexible_rewards = 0;
    advanced_staking.lock_period_stakes = Vec::new();
    advanced_staking.total_locked_staked = 0;
    advanced_staking.lock_period_rewards = 0;
    advanced_staking.yield_farming_positions = Vec::new();
    advanced_staking.total_yield_farmed = 0;
    advanced_staking.current_yield_rate = 0.0;
    advanced_staking.staking_pool_positions = Vec::new();
    advanced_staking.total_pool_staked = 0;
    advanced_staking.pool_rewards = 0;
    advanced_staking.auto_compounding_enabled = false;
    advanced_staking.auto_compounding_frequency = AutoCompoundingFrequency::Daily;
    advanced_staking.total_compounded = 0;
    advanced_staking.total_staking_rewards = 0;
    advanced_staking.average_apy = 0.0;
    advanced_staking.best_performing_stake = None;
    advanced_staking.staking_history = Vec::new();
    advanced_staking.created_at = clock.unix_timestamp;
    advanced_staking.updated_at = clock.unix_timestamp;
    
    Ok(())
}
