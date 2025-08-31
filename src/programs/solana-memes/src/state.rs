use anchor_lang::prelude::*;

/// Vesting schedule for creator tokens with distribution options
#[account]
pub struct Vesting {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub cliff_time: i64,
    pub end_time: i64,
    pub released: u64,
    pub is_revocable: bool,
    pub revoked: bool,
    pub revoke_time: Option<i64>,
    pub distribution_choice: Option<VestingOption>,
    pub choice_deadline: i64,
    pub choice_made: bool,
}

/// Post-vesting distribution options
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VestingOption {
    Withdraw,      // Take all tokens
    Burn,          // Burn 50%, keep 50%
    Distribute,    // Distribute 50% to holders, keep 50%
}

        /// Enhanced creator profile with reputation-based allocation
        #[account]
        pub struct CreatorProfile {
            pub is_registered: bool,
            pub owner: Pubkey,
            pub stake_amount: u64,
            pub last_creation_ts: i64,
            pub reputation_score: i32,
            pub total_tokens_created: u32,
            pub successful_tokens: u32,
            pub failed_tokens: u32,
            pub total_volume: u64,
            pub is_banned: bool,
            pub ban_reason: String,
            pub launch_pass_required: bool,
            pub launch_pass_mint: Option<Pubkey>,
            pub max_allocation_percent: u8,  // Based on reputation (max 7%)
            pub total_profit_shared: u64,    // Total profits shared with community
            pub weekly_creation_count: u8,   // Track weekly creations (max 2 per week)
            pub last_week_reset: i64,        // Track when weekly count resets
        }

/// Token metadata and configuration
#[account]
pub struct TokenMetadata {
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub creator_percent: u8,
    pub vesting_seconds: i64,
    pub created_at: i64,
    pub is_verified: bool,
    pub risk_score: u8,
    pub liquidity_locked: bool,
    pub liquidity_lock_amount: u64,
    pub liquidity_lock_end: i64,
    pub anti_bot_enabled: bool,
    pub anti_bot_window: i64,
    pub max_transaction_size: u64,
    pub min_transaction_size: u64,
}

/// Staking rewards configuration
#[account]
pub struct StakingRewards {
    pub total_rewards_distributed: u64,
    pub platform_fees_collected: u64,
    pub buyback_rewards_pool: u64,
    pub success_rewards_pool: u64,
    pub reward_rate: u64,
    pub last_distribution_time: i64,
    pub distribution_period: i64,
}

/// Platform treasury for fee collection
#[account]
pub struct PlatformTreasury {
    pub authority: Pubkey,
    pub sol_balance: u64,
    pub token_balances: Vec<TokenBalance>,
    pub fee_collection_stats: FeeStats,
    pub last_updated: i64,
}

/// Token balance in treasury
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenBalance {
    pub mint: Pubkey,
    pub amount: u64,
    pub last_updated: i64,
}

/// Fee collection statistics
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FeeStats {
    pub total_creation_fees: u64,      // 0.03 SOL per token creation
    pub total_trading_fees: u64,       // 1% of trading volume
    pub total_buyback_fees: u64,       // 0.05% of buyback amount
    pub total_listing_fees: u64,       // 0.01 SOL per listing
    pub fees_distributed_to_stakers: u64,
    pub fees_retained_for_development: u64,
    pub fees_for_governance: u64,
}

/// Fee constants
pub const TOKEN_CREATION_FEE: u64 = 30_000_000;  // 0.03 SOL
pub const TRADING_FEE_PERCENTAGE: u8 = 12;       // 1.2% (updated from 1%)
pub const BUYBACK_FEE_PERCENTAGE: u8 = 5;        // 0.05%
pub const LISTING_FEE: u64 = 10_000_000;         // 0.01 SOL

/// Adjustable fee configuration
pub const INITIAL_TRADING_FEE: u8 = 12;          // 1.2%
pub const MIN_TRADING_FEE: u8 = 5;               // 0.5%
pub const MAX_TRADING_FEE: u8 = 20;              // 2.0%
pub const FEE_CHANGE_COOLDOWN: i64 = 604800;     // 7 days
pub const FEE_IMPLEMENTATION_DELAY: i64 = 86400; // 24 hours

/// Creator limits and allocation constants
pub const MAX_CREATOR_ALLOCATION_PERCENT: u8 = 7;  // Maximum 7% allocation for creators
pub const MAX_WEEKLY_CREATIONS: u8 = 2;            // Maximum 2 tokens per week
pub const WEEK_IN_SECONDS: i64 = 604800;           // 7 days in seconds

/// Fee distribution percentages (55/35/10 split)
pub const STAKER_REWARD_PERCENTAGE: u8 = 55;       // 55% to stakers
pub const DEVELOPMENT_PERCENTAGE: u8 = 35;         // 35% to development
pub const GOVERNANCE_PERCENTAGE: u8 = 10;          // 10% to governance

/// Platform configuration for adjustable fees
#[account]
pub struct PlatformConfig {
    pub authority: Pubkey,
    pub trading_fee_percentage: u8,    // Current trading fee (in basis points)
    pub min_trading_fee: u8,           // Minimum allowed trading fee
    pub max_trading_fee: u8,           // Maximum allowed trading fee
    pub fee_change_cooldown: i64,      // Cooldown between fee changes
    pub last_fee_change: i64,          // Timestamp of last fee change
    pub pending_fee_change: Option<PendingFeeChange>,
    pub emergency_pause: bool,         // Emergency pause for fee changes
    pub governance_quorum: u8,         // Required quorum for fee changes
    pub created_at: i64,
    pub updated_at: i64,
}

/// Pending fee change waiting for implementation
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PendingFeeChange {
    pub proposed_fee: u8,
    pub proposed_by: Pubkey,
    pub proposal_time: i64,
    pub implementation_time: i64,
    pub reason: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub total_votes: u64,
}

/// Staking pool for tokens
#[account]
pub struct StakingPool {
    pub mint: Pubkey,
    pub total_staked: u64,
    pub total_rewards: u64,
    pub reward_rate: u64,
    pub last_update_time: i64,
    pub min_stake_amount: u64,
    pub max_stake_amount: u64,
    pub lock_period: i64,
    pub early_withdrawal_fee: u8,
    pub is_active: bool,
}

/// User staking position
#[account]
pub struct StakingPosition {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub rewards_claimed: u64,
    pub pending_rewards: u64,
    pub is_locked: bool,
}

/// Governance proposal
#[account]
pub struct Proposal {
    pub id: u64,
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub start_time: i64,
    pub end_time: i64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub total_votes: u64,
    pub quorum: u64,
    pub executed: bool,
    pub executed_at: Option<i64>,
    pub executed_by: Option<Pubkey>,
}

/// User vote on proposal
#[account]
pub struct Vote {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub vote_type: VoteType,
    pub amount: u64,
    pub timestamp: i64,
}

/// Launch pass NFT (required for token creation)
#[account]
pub struct LaunchPass {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub creator: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub is_used: bool,
    pub used_for_token: Option<Pubkey>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

/// Liquidity pool information
#[account]
pub struct LiquidityPool {
    pub token_mint: Pubkey,
    pub token_reserve: u64,
    pub sol_reserve: u64,
    pub lp_supply: u64,
    pub fee_rate: u16,
    pub is_active: bool,
    pub created_at: i64,
    pub last_swap_time: i64,
    pub total_volume: u64,
}

/// Anti-bot protection configuration
#[account]
pub struct AntiBotConfig {
    pub token_mint: Pubkey,
    pub enabled: bool,
    pub max_transaction_size: u64,
    pub min_transaction_size: u64,
    pub cooldown_period: i64,
    pub blacklisted_addresses: Vec<Pubkey>,
    pub whitelisted_addresses: Vec<Pubkey>,
    pub max_wallet_percentage: u8,
    pub max_transaction_percentage: u8,
}

/// Suspicious activity report
#[account]
pub struct ActivityReport {
    pub reporter: Pubkey,
    pub token_mint: Pubkey,
    pub activity_type: ActivityType,
    pub description: String,
    pub evidence: String,
    pub timestamp: i64,
    pub severity: u8,
    pub is_resolved: bool,
    pub resolved_by: Option<Pubkey>,
    pub resolution: Option<String>,
}

/// Treasury management
#[account]
pub struct Treasury {
    pub authority: Pubkey,
    pub reserve_usdc: Pubkey,
    pub buyback_vault: Pubkey,
    pub lp_vault: Pubkey,
    pub buyback_config: Pubkey,
    pub total_usdc_spent: u128,
    pub total_tokens_bought: u128,
    pub total_tokens_burned: u128,
    pub total_tokens_lp: u128,
    pub bump: u8,
    pub created_at: i64,
}

impl Treasury {
    pub const LEN: usize = 32*4 + 16*4 + 1 + 8;
}

/// Analytics data
#[account]
pub struct Analytics {
    pub token_mint: Pubkey,
    pub total_transactions: u64,
    pub total_volume: u64,
    pub unique_holders: u32,
    pub price_change_24h: i64,
    pub price_change_7d: i64,
    pub market_cap: u64,
    pub liquidity: u64,
    pub last_update: i64,
}

/// Risk assessment
#[account]
pub struct RiskAssessment {
    pub token_mint: Pubkey,
    pub risk_score: u8,
    pub rug_pull_risk: u8,
    pub liquidity_risk: u8,
    pub volatility_risk: u8,
    pub concentration_risk: u8,
    pub bot_activity_risk: u8,
    pub assessment_date: i64,
    pub factors: Vec<String>,
}

/// Enums
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalType {
    TokenCreation,
    FeeChange,           // Trading fee adjustment
    BuybackConfig,
    TreasuryAllocation,
    GovernanceRule,
    EmergencyAction,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoteType {
    Yes,
    No,
    Abstain,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ActivityType {
    RugPull,
    WashTrading,
    BotActivity,
    PriceManipulation,
    LiquidityRemoval,
    UnauthorizedMint,
    Other,
}

/// Constants
pub const MIN_STAKE_AMOUNT: u64 = 500_000_000; // 0.5 SOL
pub const MAX_CREATOR_PERCENT: u8 = 20; // 20% max for creator
pub const MIN_VESTING_PERIOD: i64 = 30 * 24 * 60 * 60; // 30 days
pub const MAX_VESTING_PERIOD: i64 = 365 * 24 * 60 * 60; // 1 year
pub const RATE_LIMIT_PERIOD: i64 = 30 * 24 * 60 * 60; // 30 days
pub const MIN_REPUTATION_SCORE: i32 = -100;
pub const MAX_REPUTATION_SCORE: i32 = 1000;
pub const QUORUM_PERCENTAGE: u8 = 10; // 10% of total supply
pub const VOTING_PERIOD: i64 = 7 * 24 * 60 * 60; // 7 days
pub const ANTI_BOT_WINDOW: i64 = 300; // 5 minutes
pub const MAX_TRANSACTION_SIZE_PERCENT: u8 = 5; // 5% of supply
pub const MIN_TRANSACTION_SIZE: u64 = 1_000; // 0.001 tokens
pub const LIQUIDITY_LOCK_PERIOD: i64 = 30 * 24 * 60 * 60; // 30 days
pub const EARLY_WITHDRAWAL_FEE: u8 = 10; // 10%
pub const REWARD_RATE_PER_DAY: u64 = 1_000_000; // 0.1% per day

/// Liquidity bootstrapping pool for fair token launches
#[account]
pub struct LiquidityBootstrappingPool {
    pub token_mint: Pubkey,
    pub creator: Pubkey,
    pub initial_liquidity: u64,
    pub target_liquidity: u64,
    pub current_liquidity: u64,
    pub bootstrap_start_time: i64,
    pub bootstrap_end_time: i64,
    pub price_discovery_period: i64,
    pub liquidity_release_schedule: Vec<LiquidityRelease>,
    pub is_active: bool,
    pub total_participants: u32,
    pub total_volume: u64,
    pub current_price: u64,
    pub initial_price: u64,
    pub final_price: u64,
    pub price_discovery_complete: bool,
    pub liquidity_providers: Vec<LiquidityProvider>,
    pub anti_bot_enabled: bool,
    pub max_participation_per_wallet: u64,
    pub min_participation: u64,
    pub max_participation: u64,
}

/// Liquidity release schedule
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquidityRelease {
    pub release_time: i64,
    pub release_amount: u64,
    pub release_percentage: u8,
    pub is_executed: bool,
}

/// Liquidity provider information
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquidityProvider {
    pub wallet: Pubkey,
    pub provided_amount: u64,
    pub participation_time: i64,
    pub rewards_claimed: u64,
    pub is_whitelisted: bool,
}

/// LBM participation status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum LBMStatus {
    NotStarted,
    Active,
    PriceDiscovery,
    Completed,
    Failed,
}

/// LBM configuration constants
pub const DEFAULT_BOOTSTRAP_DURATION: i64 = 86400;  // 24 hours
pub const DEFAULT_PRICE_DISCOVERY_PERIOD: i64 = 3600;  // 1 hour
pub const MIN_LIQUIDITY_REQUIREMENT: u64 = 1_000_000_000;  // 1 SOL worth
pub const MAX_LIQUIDITY_REQUIREMENT: u64 = 100_000_000_000;  // 100 SOL worth
pub const DEFAULT_MAX_PARTICIPATION: u64 = 10_000_000_000;  // 10 SOL per wallet
pub const DEFAULT_MIN_PARTICIPATION: u64 = 100_000_000;  // 0.1 SOL per wallet

/// Governance token configuration
pub const GOVERNANCE_TOKEN_DECIMALS: u8 = 9;
pub const INITIAL_SUPPLY: u64 = 500_000_000; // 500 million tokens
pub const MIN_VOTING_POWER: u64 = 1_000; // 1K tokens minimum to vote
pub const PROPOSAL_QUORUM_PERCENTAGE: u8 = 10; // 10% of total supply
pub const VOTING_PERIOD: i64 = 604800; // 7 days

/// Security configuration constants
pub const MIN_TRADE_INTERVAL: i64 = 60; // 1 minute minimum between trades
pub const MAX_TRADE_AMOUNT: u64 = 100_000_000_000; // 100 SOL maximum per trade
pub const MAX_DAILY_VOLUME: u64 = 10_000_000_000_000; // 10,000 SOL maximum daily volume
pub const MAX_PRICE_CHANGE_PERCENT: u8 = 50; // 50% maximum price change
pub const MAX_VOLUME_PER_PERIOD: u64 = 1_000_000_000_000; // 1,000 SOL per hour
pub const CIRCUIT_BREAKER_COOLDOWN: i64 = 3600; // 1 hour cooldown
pub const MULTISIG_REQUIRED_SIGNATURES: u8 = 3; // 3 of 5 signatures required
pub const DISTRIBUTION_THRESHOLD: u64 = 1_000_000; // 1M tokens maximum per distribution
pub const VESTING_DURATION: i64 = 63072000; // 2 years in seconds
pub const SUSPICIOUS_PATTERN_THRESHOLD: u64 = 1_000_000_000_000; // 1,000 SOL threshold

/// Governance token mint account
#[account]
pub struct GovernanceToken {
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Governance token holder
#[account]
pub struct TokenHolder {
    pub holder: Pubkey,
    pub balance: u64,
    pub voting_power: u64,
    pub last_vote: i64,
    pub is_delegated: bool,
    pub delegate: Option<Pubkey>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Governance delegation
#[account]
pub struct Delegation {
    pub delegator: Pubkey,
    pub delegate: Pubkey,
    pub amount: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Governance proposal with token-based voting
#[account]
pub struct GovernanceProposal {
    pub id: u64,
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub start_time: i64,
    pub end_time: i64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub total_votes: u64,
    pub quorum_required: u64,
    pub quorum_met: bool,
    pub executed: bool,
    pub executed_at: Option<i64>,
    pub executed_by: Option<Pubkey>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Individual vote on a proposal
#[account]
pub struct Vote {
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub vote_type: VoteType,
    pub voting_power: u64,
    pub voted_at: i64,
}

/// Vote types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoteType {
    Yes,
    No,
    Abstain,
}

/// Governance configuration
#[account]
pub struct GovernanceConfig {
    pub authority: Pubkey,
    pub governance_token: Pubkey,
    pub min_voting_power: u64,
    pub proposal_quorum_percentage: u8,
    pub voting_period: i64,
    pub execution_delay: i64,
    pub emergency_threshold: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Circuit breaker for extreme price movements and volume spikes
#[account]
pub struct CircuitBreaker {
    pub authority: Pubkey,
    pub max_price_change_percent: u8, // 50% max change
    pub max_volume_per_period: u64, // 1000 SOL per hour
    pub cooldown_period: i64, // 3600 seconds (1 hour)
    pub last_trigger_time: i64,
    pub is_triggered: bool,
    pub trigger_count: u32,
    pub last_price: u64,
    pub last_volume_check: i64,
    pub volume_in_period: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Flash loan protection and trade validation
#[account]
pub struct TradeProtection {
    pub authority: Pubkey,
    pub min_trade_interval: i64, // 60 seconds minimum between trades
    pub max_trade_amount: u64, // 100 SOL maximum per trade
    pub max_daily_volume: u64, // 10000 SOL maximum daily volume per wallet
    pub suspicious_pattern_threshold: u64, // Threshold for suspicious activity
    pub last_trade_times: Vec<TradeTimeRecord>,
    pub daily_volumes: Vec<DailyVolumeRecord>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Trade time record for flash loan detection
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TradeTimeRecord {
    pub trader: Pubkey,
    pub last_trade_time: i64,
    pub trade_amount: u64,
}

/// Daily volume record for volume limits
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DailyVolumeRecord {
    pub trader: Pubkey,
    pub date: i64, // Unix timestamp for the day
    pub volume: u64,
}

/// Multi-signature governance for critical operations
#[account]
pub struct MultiSigGovernance {
    pub authority: Pubkey,
    pub required_signatures: u8, // 3 of 5 signatures required
    pub signers: Vec<Pubkey>,
    pub distribution_threshold: u64, // 1M tokens maximum per distribution
    pub vesting_enabled: bool,
    pub vesting_duration: i64, // 2 years in seconds
    pub pending_distributions: Vec<PendingDistribution>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Pending distribution requiring multi-signature approval
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PendingDistribution {
    pub id: u64,
    pub recipient: Pubkey,
    pub amount: u64,
    pub distribution_type: DistributionType,
    pub signatures: Vec<Signature>,
    pub created_at: i64,
    pub expires_at: i64,
}

/// Enhanced emergency controls
#[account]
pub struct EmergencyControls {
    pub authority: Pubkey,
    pub emergency_pause: bool,
    pub emergency_threshold: u64,
    pub pause_reason: String,
    pub pause_initiated_by: Option<Pubkey>,
    pub pause_time: Option<i64>,
    pub auto_resume_time: Option<i64>,
    pub circuit_breaker_active: bool,
    pub flash_loan_protection_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

// Buyback System Structures
#[account]
pub struct BuybackConfig {
    pub burn_percent: u8,   // 0..=100
    pub lp_percent: u8,     // 0..=100
    pub authority: Pubkey,  // who can update the config
    pub enabled: bool,
    pub buyback_threshold: u64, // Minimum USDC amount to trigger buyback
    pub buyback_frequency: i64, // How often buybacks can occur (seconds)
    pub last_buyback_time: i64,
    pub total_buybacks_executed: u64,
    pub total_usdc_spent: u128,
    pub total_tokens_bought: u128,
    pub total_tokens_burned: u128,
    pub total_tokens_lp: u128,
}

impl BuybackConfig {
    pub const LEN: usize = 1 + 1 + 32 + 1 + 8 + 8 + 8 + 8 + 16 + 16 + 16 + 16;
}

#[account]
pub struct Treasury {
    pub authority: Pubkey,
    pub reserve_usdc: Pubkey,
    pub buyback_vault: Pubkey,
    pub lp_vault: Pubkey,
    pub buyback_config: Pubkey,
    pub total_usdc_spent: u128,
    pub total_tokens_bought: u128,
    pub total_tokens_burned: u128,
    pub total_tokens_lp: u128,
    pub bump: u8,
    pub created_at: i64,
}

impl Treasury {
    pub const LEN: usize = 32*4 + 16*4 + 1 + 8;
}

// Buyback System Constants
pub const DEFAULT_BUYBACK_BURN_PERCENT: u8 = 60; // 60% burn
pub const DEFAULT_BUYBACK_LP_PERCENT: u8 = 40;   // 40% to LP
pub const MIN_BUYBACK_AMOUNT: u64 = 1_000_000;   // 1 USDC minimum
pub const MAX_BUYBACK_AMOUNT: u64 = 1_000_000_000; // 1000 USDC maximum
pub const DEFAULT_BUYBACK_FREQUENCY: i64 = 3600; // 1 hour between buybacks
pub const BUYBACK_SLIPPAGE_TOLERANCE: f64 = 0.05; // 5% slippage tolerance

/// Fair voting safeguards and multi-factor voting power
#[account]
pub struct FairVotingSafeguards {
    pub authority: Pubkey,
    
    // Multi-factor voting weights
    pub staked_amount_weight: u8,        // 40% weight for staked amount
    pub staking_duration_weight: u8,     // 25% weight for staking duration
    pub community_contribution_weight: u8, // 20% weight for community contribution
    pub token_holding_weight: u8,        // 15% weight for token holding
    
    // Anti-whale mechanisms
    pub max_voting_power_per_wallet: u64, // Maximum voting power per wallet
    pub whale_voting_discount: u8,        // 50% discount for large holders
    pub max_concentration_percent: u8,    // Maximum 5% of total voting power
    pub whale_cooldown_period: i64,       // Cooldown for large holders
    
    // Time-based voting requirements
    pub minimum_staking_duration: i64,   // 30 days minimum
    pub minimum_staked_amount: u64,      // Minimum stake requirement
    pub lock_period_during_voting: i64,  // Tokens locked during voting
    
    // Duration multipliers
    pub short_term_multiplier: u8,       // 0.5x for <3 months
    pub medium_term_multiplier: u8,      // 1.0x for 3-6 months
    pub long_term_multiplier: u8,        // 1.5x for 6-12 months
    pub very_long_multiplier: u8,        // 2.0x for >12 months
    
    // Detection and monitoring
    pub suspicious_activity_threshold: u64, // Threshold for suspicious activity
    pub manipulation_detection_enabled: bool, // Enable manipulation detection
    pub automated_monitoring_enabled: bool,   // Enable automated monitoring
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Enhanced token holder with fair voting power
#[account]
pub struct EnhancedTokenHolder {
    pub holder: Pubkey,
    pub balance: u64,
    pub voting_power: u64,
    
    // Multi-factor voting components
    pub staked_amount: u64,
    pub staking_duration: i64,           // Duration in seconds
    pub community_contribution_score: u64, // Community contribution score
    pub token_holding_amount: u64,
    
    // Anti-manipulation factors
    pub consistency_score: u64,          // Voting consistency
    pub reputation_score: i32,           // Community reputation (-100 to 1000)
    pub participation_history: u64,      // Historical participation
    pub contribution_quality: u64,       // Quality of contributions
    
    // Time-based factors
    pub first_stake_time: i64,           // When they first staked
    pub last_stake_time: i64,            // Last stake time
    pub total_staking_time: i64,         // Total time staked
    
    // Whale detection
    pub is_whale: bool,                  // Flagged as whale
    pub whale_discount_applied: bool,    // Whale discount applied
    pub concentration_percentage: u8,    // Percentage of total voting power
    
    // Voting restrictions
    pub voting_restricted: bool,         // Voting restrictions applied
    pub restriction_reason: String,      // Reason for restriction
    pub restriction_until: i64,          // Restriction until time
    
    pub last_vote: i64,
    pub is_delegated: bool,
    pub delegate: Option<Pubkey>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Creator performance tracking for fair assessment
#[account]
pub struct CreatorPerformance {
    pub creator: Pubkey,
    pub token_mint: Pubkey,
    
    // Quantitative metrics (70% of decision)
    pub token_price_performance: u64,    // Measurable price growth
    pub trading_volume: u64,             // Measurable volume
    pub community_growth: u64,           // Measurable community size
    pub staking_participation: u64,      // Measurable staking
    
    // Qualitative metrics (30% of decision)
    pub community_satisfaction: u64,     // Survey-based satisfaction
    pub marketing_efforts: u64,          // Measurable marketing
    pub community_engagement: u64,       // Measurable engagement
    pub transparency_score: u64,         // Transparency rating
    
    // Performance tracking
    pub performance_score: u64,          // Overall performance score (0-100)
    pub target_metrics: HashMap<String, u64>, // Target vs actual metrics
    pub performance_history: Vec<u64>,   // Historical performance scores
    
    // Assessment periods
    pub assessment_start_time: i64,      // Assessment period start
    pub assessment_end_time: i64,        // Assessment period end
    pub last_assessment_time: i64,       // Last assessment time
    
    // Community feedback
    pub community_feedback_count: u64,   // Number of feedback responses
    pub positive_feedback_percentage: u8, // Percentage positive feedback
    pub negative_feedback_percentage: u8, // Percentage negative feedback
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Appeal and oversight system
#[account]
pub struct AppealSystem {
    pub authority: Pubkey,
    
    // Appeal mechanisms
    pub appeal_threshold: u8,            // Threshold for appeals (e.g., 15%)
    pub appeal_period: i64,              // Time for appeals (e.g., 7 days)
    pub appeal_review_panel_size: u8,    // Number of review panel members
    pub appeal_fee: u64,                 // Fee to submit appeal
    
    // Oversight mechanisms
    pub governance_oversight_enabled: bool, // Governance committee oversight
    pub external_auditors_enabled: bool,    // External audit
    pub community_oversight_enabled: bool,  // Community oversight committee
    pub emergency_intervention_enabled: bool, // Emergency intervention powers
    
    // Appeal tracking
    pub total_appeals: u64,              // Total appeals submitted
    pub successful_appeals: u64,         // Successful appeals
    pub rejected_appeals: u64,           // Rejected appeals
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Individual appeal record
#[account]
pub struct Appeal {
    pub appeal_id: u64,
    pub appellant: Pubkey,               // Person submitting appeal
    pub original_decision: Pubkey,       // Original decision being appealed
    pub appeal_reason: String,           // Reason for appeal
    pub evidence_provided: String,       // Evidence provided
    
    // Appeal status
    pub status: AppealStatus,
    pub submitted_at: i64,
    pub review_started_at: Option<i64>,
    pub decision_at: Option<i64>,
    pub decision: Option<AppealDecision>,
    pub decision_reason: Option<String>,
    
    // Review panel
    pub review_panel: Vec<Pubkey>,       // Review panel members
    pub panel_votes: Vec<AppealVote>,    // Panel votes
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Appeal status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AppealStatus {
    Pending,        // Appeal submitted, waiting for review
    UnderReview,    // Appeal under review by panel
    Approved,       // Appeal approved
    Rejected,       // Appeal rejected
    Expired,        // Appeal expired
}

/// Appeal decision
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AppealDecision {
    UpholdOriginal,     // Original decision upheld
    OverturnOriginal,   // Original decision overturned
    ModifyOriginal,     // Original decision modified
    RemandForReview,    // Remand for additional review
}

/// Appeal vote by review panel member
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AppealVote {
    pub panel_member: Pubkey,
    pub vote: AppealDecision,
    pub reasoning: String,
    pub voted_at: i64,
}

/// Detection and monitoring system
#[account]
pub struct DetectionSystem {
    pub authority: Pubkey,
    
    // Whale detection
    pub whale_detection_enabled: bool,   // Enable whale detection
    pub whale_threshold: u64,            // Threshold for whale detection
    pub whale_coordination_threshold: u64, // Threshold for coordination detection
    
    // Suspicious activity detection
    pub suspicious_activity_enabled: bool, // Enable suspicious activity detection
    pub suspicious_pattern_threshold: u64, // Threshold for suspicious patterns
    pub vote_pattern_analysis_enabled: bool, // Enable vote pattern analysis
    
    // Collusion detection
    pub collusion_detection_enabled: bool, // Enable collusion detection
    pub bribery_detection_enabled: bool,    // Enable bribery detection
    pub vote_selling_detection_enabled: bool, // Enable vote selling detection
    
    // Monitoring results
    pub total_alerts: u64,               // Total alerts generated
    pub confirmed_manipulations: u64,    // Confirmed manipulations
    pub false_positives: u64,            // False positive alerts
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Suspicious activity alert
#[account]
pub struct SuspiciousActivityAlert {
    pub alert_id: u64,
    pub alert_type: AlertType,
    pub target_wallet: Pubkey,
    pub suspicious_activity: String,     // Description of suspicious activity
    pub evidence: String,                // Evidence of suspicious activity
    pub risk_score: u8,                  // Risk score (0-100)
    
    // Alert status
    pub status: AlertStatus,
    pub created_at: i64,
    pub reviewed_at: Option<i64>,
    pub resolved_at: Option<i64>,
    pub resolution: Option<AlertResolution>,
    
    // Investigation
    pub investigator: Option<Pubkey>,    // Assigned investigator
    pub investigation_notes: String,     // Investigation notes
    pub action_taken: String,            // Action taken
    
    pub updated_at: i64,
}

/// Alert types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AlertType {
    WhaleManipulation,      // Whale manipulation detected
    VoteCoordination,       // Coordinated voting detected
    SuspiciousPattern,      // Suspicious voting pattern
    Collusion,              // Collusion detected
    Bribery,                // Bribery attempt detected
    VoteSelling,            // Vote selling detected
    SybilAttack,            // Sybil attack detected
    FlashLoanAttack,        // Flash loan attack detected
}

/// Alert status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AlertStatus {
    Open,           // Alert created, waiting for review
    UnderInvestigation, // Alert under investigation
    Resolved,       // Alert resolved
    FalsePositive,  // Alert was false positive
    Dismissed,      // Alert dismissed
}

/// Alert resolution
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum AlertResolution {
    NoAction,       // No action needed
    Warning,        // Warning issued
    Restriction,    // Voting restrictions applied
    Penalty,        // Penalty applied
    Ban,            // User banned
    LegalAction,    // Legal action taken
}

/// Penalty system for violations
#[account]
pub struct PenaltySystem {
    pub authority: Pubkey,
    
    // Penalty types and amounts
    pub warning_penalty: u64,            // Warning penalty
    pub restriction_penalty: u64,        // Restriction penalty
    pub voting_ban_penalty: u64,         // Voting ban penalty
    pub token_confiscation_penalty: u64, // Token confiscation penalty
    
    // Penalty tracking
    pub total_penalties_issued: u64,     // Total penalties issued
    pub total_penalty_amount: u64,       // Total penalty amount
    pub active_penalties: u64,           // Active penalties
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Individual penalty record
#[account]
pub struct Penalty {
    pub penalty_id: u64,
    pub offender: Pubkey,                // Person receiving penalty
    pub penalty_type: PenaltyType,
    pub penalty_amount: u64,             // Amount of penalty
    pub reason: String,                  // Reason for penalty
    pub evidence: String,                // Evidence of violation
    
    // Penalty status
    pub status: PenaltyStatus,
    pub issued_at: i64,
    pub expires_at: Option<i64>,
    pub paid_at: Option<i64>,
    
    // Appeal information
    pub appealed: bool,                  // Whether penalty was appealed
    pub appeal_id: Option<u64>,          // Appeal ID if appealed
    
    pub created_at: i64,
    pub updated_at: i64,
}

/// Penalty types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PenaltyType {
    Warning,            // Warning issued
    VotingRestriction,  // Voting restrictions applied
    VotingBan,          // Voting banned
    TokenConfiscation,  // Tokens confiscated
    TemporaryBan,       // Temporary platform ban
    PermanentBan,       // Permanent platform ban
}

/// Penalty status
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PenaltyStatus {
    Active,         // Penalty is active
    Paid,           // Penalty has been paid
    Expired,        // Penalty has expired
    Appealed,       // Penalty is under appeal
    Overturned,     // Penalty overturned on appeal
    Reduced,        // Penalty reduced on appeal
}

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

/// Recent user activity
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecentActivity {
    pub activity_type: ActivityType,
    pub description: String,
    pub amount: Option<u64>,
    pub timestamp: i64,
    pub success: bool,
}

/// User notification
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

/// Dashboard preferences
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DashboardPreferences {
    pub theme: DashboardTheme,
    pub default_view: DashboardView,
    pub auto_refresh: bool,
    pub refresh_interval: i64,
    pub show_advanced_metrics: bool,
    pub language: String,
}

/// Notification settings
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

/// Activity types
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

/// Notification types
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

/// Notification priority
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Dashboard theme
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum DashboardTheme {
    Light,
    Dark,
    Auto,
}

/// Dashboard view
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum DashboardView {
    Overview,
    Tokens,
    Staking,
    Governance,
    Analytics,
    Community,
}
