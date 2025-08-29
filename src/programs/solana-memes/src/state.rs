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
        pub const TRADING_FEE_PERCENTAGE: u8 = 1;        // 1%
        pub const BUYBACK_FEE_PERCENTAGE: u8 = 5;        // 0.05%
        pub const LISTING_FEE: u64 = 10_000_000;         // 0.01 SOL
        
        /// Creator limits and allocation constants
        pub const MAX_CREATOR_ALLOCATION_PERCENT: u8 = 7;  // Maximum 7% allocation for creators
        pub const MAX_WEEKLY_CREATIONS: u8 = 2;            // Maximum 2 tokens per week
        pub const WEEK_IN_SECONDS: i64 = 604800;           // 7 days in seconds
        
        /// Fee distribution percentages (55/35/10 split)
        pub const STAKER_REWARD_PERCENTAGE: u8 = 55;       // 55% to stakers
        pub const DEVELOPMENT_PERCENTAGE: u8 = 35;         // 35% to development
        pub const GOVERNANCE_PERCENTAGE: u8 = 10;          // 10% to governance

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
    pub sol_balance: u64,
    pub total_fees_collected: u64,
    pub buyback_funds: u64,
    pub liquidity_funds: u64,
    pub governance_funds: u64,
    pub last_update: i64,
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
    FeeChange,
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
