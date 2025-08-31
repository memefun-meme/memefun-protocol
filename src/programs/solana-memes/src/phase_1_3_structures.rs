use anchor_lang::prelude::*;

/// Enhanced user dashboard with real-time metrics
#[account]
pub struct UserDashboard {
    pub user: Pubkey,
    
    // Token creation metrics
    pub total_tokens_created: u64,
    pub successful_tokens: u64,
    pub failed_tokens: u64,
    pub total_volume_traded: u64,
    
    // Staking and rewards
    pub total_rewards_earned: u64,
    pub active_stakes: u64,
    pub total_staked_amount: u64,
    pub staking_apy: f64,
    
    // Governance participation
    pub governance_participation: u64,
    pub proposals_created: u64,
    pub proposals_voted_on: u64,
    pub voting_power: u64,
    
    // Community metrics
    pub community_rank: String,
    pub reputation_score: i32,
    pub community_contribution_score: u64,
    pub followers_count: u64,
    pub following_count: u64,
    
    // Performance tracking
    pub total_transactions: u64,
    pub success_rate: f64,
    pub average_token_performance: f64,
    pub best_performing_token: Option<Pubkey>,
    
    // Recent activity
    pub last_activity: i64,
    pub recent_transactions: Vec<RecentActivity>,
    pub notifications: Vec<Notification>,
    
    // Preferences
    pub dashboard_preferences: DashboardPreferences,
    pub notification_settings: NotificationSettings,
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Multi-signature creator wallet for enhanced security
#[account]
pub struct MultiSigCreator {
    pub creator: Pubkey,
    pub required_signatures: u8,
    pub total_signers: u8,
    pub signers: Vec<Pubkey>,
    pub creator_threshold: u64, // Above this amount requires multi-sig
    pub is_active: bool,
    
    // Multi-sig transactions
    pub pending_transactions: Vec<MultiSigTransaction>,
    pub completed_transactions: Vec<MultiSigTransaction>,
    
    // Security settings
    pub time_lock_duration: i64,
    pub max_transaction_amount: u64,
    pub emergency_contacts: Vec<Pubkey>,
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Social features for community building
#[account]
pub struct SocialFeatures {
    pub user: Pubkey,
    
    // Profile information
    pub username: String,
    pub display_name: String,
    pub bio: String,
    pub avatar_uri: String,
    pub banner_uri: String,
    
    // Social connections
    pub followers: Vec<Pubkey>,
    pub following: Vec<Pubkey>,
    pub followers_count: u64,
    pub following_count: u64,
    
    // Community features
    pub community_chat_enabled: bool,
    pub token_showcase_enabled: bool,
    pub influencer_verification: bool,
    pub verification_badge: bool,
    
    // Community events
    pub hosted_events: Vec<CommunityEvent>,
    pub attended_events: Vec<CommunityEvent>,
    
    // Social metrics
    pub total_likes: u64,
    pub total_shares: u64,
    pub total_comments: u64,
    pub engagement_rate: f64,
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Gamification system for user engagement
#[account]
pub struct GamificationSystem {
    pub user: Pubkey,
    
    // User progression
    pub user_level: u8,
    pub experience_points: u64,
    pub experience_to_next_level: u64,
    pub total_experience_earned: u64,
    
    // Achievements
    pub achievements: Vec<Achievement>,
    pub total_achievements: u64,
    pub achievement_score: u64,
    
    // Leaderboards
    pub leaderboard_rank: u64,
    pub leaderboard_score: u64,
    pub leaderboard_category: LeaderboardCategory,
    
    // Rewards
    pub rewards_multiplier: f64,
    pub total_rewards_earned: u64,
    pub pending_rewards: u64,
    
    // Daily/weekly challenges
    pub daily_challenges: Vec<Challenge>,
    pub weekly_challenges: Vec<Challenge>,
    pub completed_challenges: Vec<Challenge>,
    
    // Streaks and milestones
    pub current_streak: u64,
    pub longest_streak: u64,
    pub milestones_reached: Vec<Milestone>,
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Advanced staking system with flexible options
#[account]
pub struct AdvancedStaking {
    pub user: Pubkey,
    
    // Flexible staking
    pub flexible_stakes: Vec<FlexibleStake>,
    pub total_flexible_staked: u64,
    pub flexible_rewards: u64,
    
    // Lock period options
    pub lock_period_stakes: Vec<LockPeriodStake>,
    pub total_locked_staked: u64,
    pub lock_period_rewards: u64,
    
    // Yield farming
    pub yield_farming_positions: Vec<YieldFarmingPosition>,
    pub total_yield_farmed: u64,
    pub current_yield_rate: f64,
    
    // Staking pools
    pub staking_pool_positions: Vec<StakingPoolPosition>,
    pub total_pool_staked: u64,
    pub pool_rewards: u64,
    
    // Auto-compounding
    pub auto_compounding_enabled: bool,
    pub auto_compounding_frequency: AutoCompoundingFrequency,
    pub total_compounded: u64,
    
    // Performance tracking
    pub total_staking_rewards: u64,
    pub average_apy: f64,
    pub best_performing_stake: Option<Pubkey>,
    pub staking_history: Vec<StakingHistory>,
    
    pub created_at: i64,
    pub updated_at: i64,
}

// Supporting structures
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecentActivity {
    pub activity_type: ActivityType,
    pub description: String,
    pub amount: Option<u64>,
    pub timestamp: i64,
    pub success: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Notification {
    pub notification_id: u64,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub read: bool,
    pub priority: NotificationPriority,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DashboardPreferences {
    pub theme: DashboardTheme,
    pub default_view: DashboardView,
    pub auto_refresh: bool,
    pub refresh_interval: i64,
    pub show_advanced_metrics: bool,
    pub language: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NotificationSettings {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub sms_notifications: bool,
    pub governance_alerts: bool,
    pub price_alerts: bool,
    pub security_alerts: bool,
    pub marketing_notifications: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MultiSigTransaction {
    pub transaction_id: u64,
    pub creator: Pubkey,
    pub transaction_type: MultiSigTransactionType,
    pub amount: u64,
    pub recipient: Option<Pubkey>,
    pub description: String,
    
    // Signatures
    pub required_signatures: u8,
    pub signatures: Vec<MultiSigSignature>,
    pub status: MultiSigTransactionStatus,
    
    // Timestamps
    pub created_at: i64,
    pub expires_at: i64,
    pub executed_at: Option<i64>,
    
    // Security
    pub time_lock_until: i64,
    pub emergency_approved: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MultiSigSignature {
    pub signer: Pubkey,
    pub signature: Vec<u8>,
    pub signed_at: i64,
    pub signature_type: SignatureType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CommunityEvent {
    pub event_id: u64,
    pub host: Pubkey,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub start_time: i64,
    pub end_time: i64,
    pub max_participants: u64,
    pub current_participants: u64,
    pub participants: Vec<Pubkey>,
    pub event_status: EventStatus,
    pub rewards_pool: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Achievement {
    pub achievement_id: u64,
    pub title: String,
    pub description: String,
    pub achievement_type: AchievementType,
    pub difficulty: AchievementDifficulty,
    pub reward_amount: u64,
    pub reward_type: RewardType,
    pub unlocked_at: Option<i64>,
    pub progress: u64,
    pub target: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Challenge {
    pub challenge_id: u64,
    pub title: String,
    pub description: String,
    pub challenge_type: ChallengeType,
    pub duration: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub reward_amount: u64,
    pub participants: Vec<Pubkey>,
    pub completed_by: Vec<Pubkey>,
    pub challenge_status: ChallengeStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Milestone {
    pub milestone_id: u64,
    pub title: String,
    pub description: String,
    pub milestone_type: MilestoneType,
    pub threshold: u64,
    pub reward_amount: u64,
    pub achieved_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FlexibleStake {
    pub stake_id: u64,
    pub amount: u64,
    pub start_time: i64,
    pub current_rewards: u64,
    pub apy: f64,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LockPeriodStake {
    pub stake_id: u64,
    pub amount: u64,
    pub lock_period: i64,
    pub start_time: i64,
    pub end_time: i64,
    pub current_rewards: u64,
    pub apy: f64,
    pub early_withdrawal_fee: u8,
    pub is_locked: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct YieldFarmingPosition {
    pub position_id: u64,
    pub pool: Pubkey,
    pub staked_amount: u64,
    pub reward_tokens: Vec<Pubkey>,
    pub start_time: i64,
    pub current_rewards: Vec<u64>,
    pub apy: f64,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct StakingPoolPosition {
    pub position_id: u64,
    pub pool: Pubkey,
    pub staked_amount: u64,
    pub pool_share: f64,
    pub start_time: i64,
    pub current_rewards: u64,
    pub apy: f64,
    pub is_active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct StakingHistory {
    pub entry_id: u64,
    pub stake_type: StakeType,
    pub amount: u64,
    pub rewards: u64,
    pub apy: f64,
    pub start_time: i64,
    pub end_time: i64,
    pub duration: i64,
}

// Enums
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ActivityType {
    TokenCreated,
    TokenTraded,
    Staked,
    Unstaked,
    RewardsClaimed,
    ProposalCreated,
    ProposalVoted,
    GovernanceParticipated,
    CommunityContribution,
    AchievementUnlocked,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum NotificationType {
    Governance,
    Security,
    Price,
    Rewards,
    Community,
    Achievement,
    System,
    Marketing,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum DashboardTheme {
    Light,
    Dark,
    Auto,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum DashboardView {
    Overview,
    Tokens,
    Staking,
    Governance,
    Analytics,
    Community,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MultiSigTransactionType {
    TokenCreation,
    TokenTransfer,
    FeeChange,
    GovernanceProposal,
    EmergencyAction,
    SecurityUpdate,
    TreasurySpend,
    PartnershipApproval,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MultiSigTransactionStatus {
    Pending,
    PartiallySigned,
    FullySigned,
    Executed,
    Expired,
    Cancelled,
    EmergencyExecuted,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SignatureType {
    Standard,
    Emergency,
    TimeLocked,
    Delegated,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EventType {
    TokenLaunch,
    GovernanceMeeting,
    CommunityCall,
    TradingCompetition,
    EducationalWorkshop,
    NetworkingEvent,
    CharityEvent,
    PartnershipAnnouncement,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EventStatus {
    Upcoming,
    Active,
    Completed,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AchievementType {
    TokenCreation,
    TradingVolume,
    StakingDuration,
    GovernanceParticipation,
    CommunityContribution,
    SocialEngagement,
    StreakMaintenance,
    SpecialEvent,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AchievementDifficulty {
    Easy,
    Medium,
    Hard,
    Legendary,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RewardType {
    Tokens,
    Experience,
    Badge,
    SpecialAccess,
    GovernancePower,
    StakingBonus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ChallengeType {
    Daily,
    Weekly,
    Monthly,
    Special,
    Community,
    Competitive,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ChallengeStatus {
    Active,
    Completed,
    Failed,
    Expired,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MilestoneType {
    TokenCreation,
    TradingVolume,
    StakingAmount,
    GovernanceParticipation,
    CommunityGrowth,
    SocialFollowers,
    ExperiencePoints,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum LeaderboardCategory {
    TokenCreators,
    Traders,
    Stakers,
    GovernanceParticipants,
    CommunityContributors,
    SocialInfluencers,
    Overall,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AutoCompoundingFrequency {
    Daily,
    Weekly,
    Monthly,
    Custom,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum StakeType {
    Flexible,
    Locked,
    YieldFarming,
    Pool,
}
