use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    // Creator registration errors
    #[msg("Creator already registered")]
    AlreadyRegistered,
    #[msg("Creator not registered")]
    NotRegistered,
    #[msg("Insufficient stake amount")]
    InsufficientStake,
    #[msg("Creator is banned")]
    CreatorBanned,
    #[msg("Launch pass required but not provided")]
    LaunchPassRequired,
    #[msg("Launch pass already used")]
    LaunchPassUsed,
    #[msg("Launch pass expired")]
    LaunchPassExpired,

    // Token creation errors
    #[msg("Rate limit not passed")]
    RateLimit,
    #[msg("Invalid creator percentage")]
    InvalidCreatorPercent,
    #[msg("Invalid vesting period")]
    InvalidVestingPeriod,
    #[msg("Invalid token supply")]
    InvalidTokenSupply,
    #[msg("Invalid token decimals")]
    InvalidTokenDecimals,
    #[msg("Token name too long")]
    TokenNameTooLong,
    #[msg("Token symbol too long")]
    TokenSymbolTooLong,
    #[msg("Token URI too long")]
    TokenUriTooLong,

    // Vesting errors
    #[msg("Vesting cliff not reached")]
    NotVested,
    #[msg("No claimable tokens")]
    NothingToClaim,
    #[msg("Vesting revoked")]
    VestingRevoked,
    #[msg("Vesting not revocable")]
    VestingNotRevocable,

    // Staking errors
    #[msg("Staking pool not active")]
    StakingPoolInactive,
    #[msg("Insufficient stake amount")]
    InsufficientStakeAmount,
    #[msg("Exceeds max stake amount")]
    ExceedsMaxStakeAmount,
    #[msg("Staking position locked")]
    StakingPositionLocked,
    #[msg("No rewards to claim")]
    NoRewardsToClaim,
    #[msg("Early withdrawal fee too high")]
    EarlyWithdrawalFeeTooHigh,

    // Governance errors
    #[msg("Proposal not found")]
    ProposalNotFound,
    #[msg("Proposal not active")]
    ProposalNotActive,
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    #[msg("Insufficient voting power")]
    InsufficientVotingPower,
    #[msg("Already voted")]
    AlreadyVoted,
    #[msg("Voting period ended")]
    VotingPeriodEnded,
    #[msg("Quorum not met")]
    QuorumNotMet,
    #[msg("Unauthorized execution")]
    UnauthorizedExecution,

    // Anti-bot errors
    #[msg("Anti-bot protection active")]
    AntiBotProtectionActive,
    #[msg("Transaction too large")]
    TransactionTooLarge,
    #[msg("Transaction too small")]
    TransactionTooSmall,
    #[msg("Address blacklisted")]
    AddressBlacklisted,
    #[msg("Cooldown period not met")]
    CooldownPeriodNotMet,
    #[msg("Max wallet percentage exceeded")]
    MaxWalletPercentageExceeded,
    #[msg("Max transaction percentage exceeded")]
    MaxTransactionPercentageExceeded,

    // Fee management errors
    #[msg("Fee too low")]
    FeeTooLow,
    #[msg("Fee too high")]
    FeeTooHigh,
    #[msg("Fee change too soon")]
    FeeChangeTooSoon,
    #[msg("Fee change not pending")]
    FeeChangeNotPending,
    #[msg("Fee implementation time not reached")]
    FeeImplementationTimeNotReached,
    #[msg("Invalid fee proposal")]
    InvalidFeeProposal,
    #[msg("Fee change quorum not met")]
    FeeChangeQuorumNotMet,
    #[msg("Platform config not initialized")]
    PlatformConfigNotInitialized,
    #[msg("Unauthorized fee change")]
    UnauthorizedFeeChange,

    // Liquidity errors
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Liquidity pool not active")]
    LiquidityPoolInactive,
    #[msg("Liquidity locked")]
    LiquidityLocked,
    #[msg("Liquidity lock period not met")]
    LiquidityLockPeriodNotMet,
    #[msg("Invalid liquidity amount")]
    InvalidLiquidityAmount,

    // Buyback errors
    #[msg("Buyback not configured")]
    BuybackNotConfigured,
    #[msg("Insufficient buyback funds")]
    InsufficientBuybackFunds,
    #[msg("Buyback amount too small")]
    BuybackAmountTooSmall,
    #[msg("Buyback amount too large")]
    BuybackAmountTooLarge,
    #[msg("Buyback cooldown not met")]
    BuybackCooldownNotMet,
    #[msg("Buyback system is disabled")]
    BuybackDisabled,
    #[msg("Buyback amount too small")]
    BuybackAmountTooSmall,
    #[msg("Buyback amount too large")]
    BuybackAmountTooLarge,
    #[msg("Buyback too frequent")]
    BuybackTooFrequent,
    #[msg("Insufficient treasury balance")]
    InsufficientTreasuryBalance,
    #[msg("Invalid buyback percentages")]
    InvalidBuybackPercentages,
    #[msg("Buyback math overflow")]
    BuybackMathOverflow,
    #[msg("No tokens to process")]
    NoTokensToProcess,
    #[msg("Unauthorized buyback operation")]
    UnauthorizedBuyback,
    #[msg("Invalid buyback vault")]
    InvalidBuybackVault,
    #[msg("Invalid LP vault")]
    InvalidLPVault,
    #[msg("Buyback transaction not found")]
    BuybackTransactionNotFound,
    #[msg("Invalid buyback transaction signature")]
    InvalidBuybackTransactionSignature,

    // Treasury errors
    #[msg("Insufficient treasury balance")]
    InsufficientTreasuryBalance,
    #[msg("Unauthorized treasury access")]
    UnauthorizedTreasuryAccess,
    #[msg("Invalid treasury allocation")]
    InvalidTreasuryAllocation,

    // Analytics errors
    #[msg("Analytics not available")]
    AnalyticsNotAvailable,
    #[msg("Invalid analytics data")]
    InvalidAnalyticsData,
    #[msg("Analytics update failed")]
    AnalyticsUpdateFailed,

    // Risk assessment errors
    #[msg("Risk assessment not available")]
    RiskAssessmentNotAvailable,
    #[msg("Invalid risk score")]
    InvalidRiskScore,
    #[msg("Risk assessment update failed")]
    RiskAssessmentUpdateFailed,

    // Activity reporting errors
    #[msg("Invalid activity type")]
    InvalidActivityType,
    #[msg("Activity report not found")]
    ActivityReportNotFound,
    #[msg("Activity already resolved")]
    ActivityAlreadyResolved,
    #[msg("Unauthorized resolution")]
    UnauthorizedResolution,

    // Reputation errors
    #[msg("Invalid reputation change")]
    InvalidReputationChange,
    #[msg("Reputation score out of bounds")]
    ReputationScoreOutOfBounds,
    #[msg("Reputation update failed")]
    ReputationUpdateFailed,

    // Vesting choice errors
    #[msg("Choice deadline has passed")]
    ChoiceDeadlinePassed,
    #[msg("Choice already made")]
    ChoiceAlreadyMade,
    #[msg("Invalid vesting option")]
    InvalidVestingOption,

    // Trading fee errors
    #[msg("Trading fee calculation error")]
    TradingFeeError,
    #[msg("Invalid trade amount")]
    InvalidTradeAmount,
    
    // Creator limit errors
    #[msg("Weekly creation limit exceeded")]
    WeeklyCreationLimitExceeded,
    #[msg("Allocation exceeds maximum allowed")]
    AllocationExceedsMaximum,
    
    // LBM errors
    #[msg("LBM pool not active")]
    LBMNotActive,
    #[msg("LBM not started yet")]
    LBMNotStarted,
    #[msg("LBM period has ended")]
    LBMEnded,
    #[msg("LBM still active")]
    LBMStillActive,
    #[msg("LBM already finalized")]
    LBMAlreadyFinalized,
    #[msg("Insufficient liquidity requirement")]
    InsufficientLiquidity,
    #[msg("Excessive liquidity requirement")]
    ExcessiveLiquidity,
    #[msg("Invalid participation amount")]
    InvalidParticipationAmount,
    #[msg("Insufficient participation amount")]
    InsufficientParticipationAmount,
    #[msg("Excessive participation amount")]
    ExcessiveParticipationAmount,
    #[msg("Excessive participation limit")]
    ExcessiveParticipationLimit,
    #[msg("Target liquidity exceeded")]
    TargetLiquidityExceeded,

    // General errors
    #[msg("Invalid input")]
    InvalidInput,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Operation not allowed")]
    OperationNotAllowed,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Account not found")]
    AccountNotFound,
    #[msg("Invalid account")]
    InvalidAccount,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    #[msg("Invalid pubkey")]
    InvalidPubkey,
    #[msg("Invalid signature")]
    InvalidSignature,
    #[msg("Invalid instruction")]
    InvalidInstruction,
    #[msg("Invalid program")]
    InvalidProgram,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Invalid token amount")]
    InvalidTokenAmount,
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    #[msg("Invalid token authority")]
    InvalidTokenAuthority,
    #[msg("Invalid token program")]
    InvalidTokenProgram,
    #[msg("Invalid system program")]
    InvalidSystemProgram,
    #[msg("Invalid rent")]
    InvalidRent,
    #[msg("Invalid clock")]
    InvalidClock,
    #[msg("Invalid associated token account")]
    InvalidAssociatedTokenAccount,
    #[msg("Invalid metadata")]
    InvalidMetadata,
    #[msg("Invalid metadata program")]
    InvalidMetadataProgram,
    #[msg("Invalid metadata account")]
    InvalidMetadataAccount,
    #[msg("Invalid metadata authority")]
    InvalidMetadataAuthority,
    #[msg("Invalid metadata mint")]
    InvalidMetadataMint,
    #[msg("Invalid metadata uri")]
    InvalidMetadataUri,
    #[msg("Invalid metadata name")]
    InvalidMetadataName,
    #[msg("Invalid metadata symbol")]
    InvalidMetadataSymbol,
    #[msg("Invalid metadata decimals")]
    InvalidMetadataDecimals,
    #[msg("Invalid metadata supply")]
    InvalidMetadataSupply,
    #[msg("Invalid metadata creator")]
    InvalidMetadataCreator,
    #[msg("Invalid metadata collection")]
    InvalidMetadataCollection,
    #[msg("Invalid metadata collection details")]
    InvalidMetadataCollectionDetails,
    #[msg("Invalid metadata collection family")]
    InvalidMetadataCollectionFamily,
    #[msg("Invalid metadata collection key")]
    InvalidMetadataCollectionKey,
    #[msg("Invalid metadata collection verified")]
    InvalidMetadataCollectionVerified,
    #[msg("Invalid metadata collection size")]
    InvalidMetadataCollectionSize,
    #[msg("Invalid metadata collection max supply")]
    InvalidMetadataCollectionMaxSupply,
    #[msg("Invalid metadata collection seller fee basis points")]
    InvalidMetadataCollectionSellerFeeBasisPoints,
    #[msg("Invalid metadata collection creators")]
    InvalidMetadataCollectionCreators,
    #[msg("Invalid metadata collection creators verified")]
    InvalidMetadataCollectionCreatorsVerified,
    #[msg("Invalid metadata collection creators share")]
    InvalidMetadataCollectionCreatorsShare,
    #[msg("Invalid metadata collection creators address")]
    InvalidMetadataCollectionCreatorsAddress,
    #[msg("Invalid metadata collection creators authority")]
    InvalidMetadataCollectionCreatorsAuthority,
    #[msg("Invalid metadata collection creators authority signer")]
    InvalidMetadataCollectionCreatorsAuthoritySigner,
    #[msg("Invalid metadata collection creators authority delegate")]
    InvalidMetadataCollectionCreatorsAuthorityDelegate,
    #[msg("Invalid metadata collection creators authority close")]
    InvalidMetadataCollectionCreatorsAuthorityClose,
    #[msg("Invalid metadata collection creators authority freeze")]
    InvalidMetadataCollectionCreatorsAuthorityFreeze,
    #[msg("Invalid metadata collection creators authority thaw")]
    InvalidMetadataCollectionCreatorsAuthorityThaw,
    #[msg("Invalid metadata collection creators authority burn")]
    InvalidMetadataCollectionCreatorsAuthorityBurn,
    #[msg("Invalid metadata collection creators authority transfer")]
    InvalidMetadataCollectionCreatorsAuthorityTransfer,
    #[msg("Invalid metadata collection creators authority approve")]
    InvalidMetadataCollectionCreatorsAuthorityApprove,
    #[msg("Invalid metadata collection creators authority revoke")]
    InvalidMetadataCollectionCreatorsAuthorityRevoke,
    #[msg("Invalid metadata collection creators authority close account")]
    InvalidMetadataCollectionCreatorsAuthorityCloseAccount,
    #[msg("Invalid metadata collection creators authority freeze account")]
    InvalidMetadataCollectionCreatorsAuthorityFreezeAccount,
    #[msg("Invalid metadata collection creators authority thaw account")]
    InvalidMetadataCollectionCreatorsAuthorityThawAccount,
    #[msg("Invalid metadata collection creators authority burn account")]
    InvalidMetadataCollectionCreatorsAuthorityBurnAccount,
    #[msg("Invalid metadata collection creators authority transfer account")]
    InvalidMetadataCollectionCreatorsAuthorityTransferAccount,
    #[msg("Invalid metadata collection creators authority approve account")]
    InvalidMetadataCollectionCreatorsAuthorityApproveAccount,
    #[msg("Invalid metadata collection creators authority revoke account")]
    InvalidMetadataCollectionCreatorsAuthorityRevokeAccount,

    // Security and protection errors
    #[msg("Circuit breaker triggered")]
    CircuitBreakerTriggered,
    #[msg("Price change too large")]
    PriceChangeTooLarge,
    #[msg("Volume limit exceeded")]
    VolumeLimitExceeded,
    #[msg("Trade too frequent")]
    TradeTooFrequent,
    #[msg("Trade too large")]
    TradeTooLarge,
    #[msg("Daily volume limit exceeded")]
    DailyVolumeLimitExceeded,
    #[msg("Flash loan attack detected")]
    FlashLoanAttack,
    #[msg("Suspicious trading pattern detected")]
    SuspiciousTradingPattern,
    #[msg("Insufficient signatures for multi-sig")]
    InsufficientSignatures,
    #[msg("Distribution too large")]
    DistributionTooLarge,
    #[msg("Multi-sig distribution not found")]
    MultiSigDistributionNotFound,
    #[msg("Multi-sig distribution expired")]
    MultiSigDistributionExpired,
    #[msg("Emergency pause active")]
    EmergencyPauseActive,
    #[msg("Unauthorized emergency action")]
    UnauthorizedEmergencyAction,
    #[msg("Circuit breaker not initialized")]
    CircuitBreakerNotInitialized,
    #[msg("Trade protection not initialized")]
    TradeProtectionNotInitialized,
    #[msg("Multi-sig governance not initialized")]
    MultiSigGovernanceNotInitialized,
    #[msg("Emergency controls not initialized")]
    EmergencyControlsNotInitialized,
}

// Fair voting safeguard errors
pub const INSUFFICIENT_VOTING_POWER: u32 = 2000;
pub const WHALE_VOTING_RESTRICTED: u32 = 2001;
pub const MINIMUM_STAKING_DURATION_NOT_MET: u32 = 2002;
pub const VOTING_RESTRICTED: u32 = 2003;
pub const SUSPICIOUS_ACTIVITY_DETECTED: u32 = 2004;
pub const MANIPULATION_DETECTED: u32 = 2005;
pub const APPEAL_THRESHOLD_NOT_MET: u32 = 2006;
pub const APPEAL_PERIOD_EXPIRED: u32 = 2007;
pub const APPEAL_FEE_NOT_PAID: u32 = 2008;
pub const PENALTY_ACTIVE: u32 = 2009;
pub const PERFORMANCE_ASSESSMENT_REQUIRED: u32 = 2010;
pub const COMMUNITY_FEEDBACK_INSUFFICIENT: u32 = 2011;
pub const VOTING_POWER_CALCULATION_ERROR: u32 = 2012;
pub const WHALE_DISCOUNT_APPLIED: u32 = 2013;
pub const CONCENTRATION_LIMIT_EXCEEDED: u32 = 2014;
pub const COOLDOWN_PERIOD_ACTIVE: u32 = 2015;
pub const DETECTION_SYSTEM_DISABLED: u32 = 2016;
pub const APPEAL_SYSTEM_DISABLED: u32 = 2017;
pub const PENALTY_SYSTEM_DISABLED: u32 = 2018;
pub const FAIR_VOTING_SAFEGUARDS_NOT_INITIALIZED: u32 = 2019;
pub const ENHANCED_TOKEN_HOLDER_NOT_FOUND: u32 = 2020;
pub const CREATOR_PERFORMANCE_NOT_FOUND: u32 = 2021;
pub const APPEAL_NOT_FOUND: u32 = 2022;
pub const PENALTY_NOT_FOUND: u32 = 2023;
pub const ALERT_NOT_FOUND: u32 = 2024;
pub const INVALID_PERFORMANCE_SCORE: u32 = 2025;
pub const INVALID_REPUTATION_SCORE: u32 = 2026;
pub const INVALID_VOTING_POWER_WEIGHTS: u32 = 2027;
pub const INVALID_DURATION_MULTIPLIERS: u32 = 2028;
pub const INVALID_WHALE_THRESHOLDS: u32 = 2029;
pub const INVALID_APPEAL_THRESHOLDS: u32 = 2030;
pub const INVALID_PENALTY_AMOUNTS: u32 = 2031;
pub const REVIEW_PANEL_INSUFFICIENT: u32 = 2032;
pub const INVESTIGATION_REQUIRED: u32 = 2033;
pub const EVIDENCE_INSUFFICIENT: u32 = 2034;
pub const APPEAL_DECISION_INVALID: u32 = 2035;
pub const PENALTY_TYPE_INVALID: u32 = 2036;
pub const ALERT_TYPE_INVALID: u32 = 2037;
pub const RISK_SCORE_INVALID: u32 = 2038;
pub const MANIPULATION_CONFIRMED: u32 = 2039;
pub const FALSE_POSITIVE_ALERT: u32 = 2040;
pub const INVESTIGATION_IN_PROGRESS: u32 = 2041;
pub const APPEAL_UNDER_REVIEW: u32 = 2042;
pub const PENALTY_UNDER_APPEAL: u32 = 2043;
pub const ALERT_UNDER_INVESTIGATION: u32 = 2044;
pub const VOTING_POWER_LOCKED: u32 = 2045;
pub const COMMUNITY_CONTRIBUTION_INSUFFICIENT: u32 = 2046;
pub const PARTICIPATION_HISTORY_INSUFFICIENT: u32 = 2047;
pub const CONTRIBUTION_QUALITY_INSUFFICIENT: u32 = 2048;
pub const CONSISTENCY_SCORE_INSUFFICIENT: u32 = 2049;
pub const REPUTATION_SCORE_INSUFFICIENT: u32 = 2050;
pub const STAKING_DURATION_INSUFFICIENT: u32 = 2051;
pub const TOKEN_HOLDING_INSUFFICIENT: u32 = 2052;
pub const STAKED_AMOUNT_INSUFFICIENT: u32 = 2053;
pub const VOTING_POWER_WEIGHT_INVALID: u32 = 2054;
pub const DURATION_MULTIPLIER_INVALID: u32 = 2055;
pub const WHALE_DISCOUNT_INVALID: u32 = 2056;
pub const CONCENTRATION_PERCENT_INVALID: u32 = 2057;
pub const COOLDOWN_PERIOD_INVALID: u32 = 2058;
pub const LOCK_PERIOD_INVALID: u32 = 2059;
pub const MINIMUM_STAKE_AMOUNT_INVALID: u32 = 2060;
pub const MINIMUM_STAKING_DURATION_INVALID: u32 = 2061;
pub const SUSPICIOUS_ACTIVITY_THRESHOLD_INVALID: u32 = 2062;
pub const MANIPULATION_DETECTION_DISABLED: u32 = 2063;
pub const AUTOMATED_MONITORING_DISABLED: u32 = 2064;
pub const APPEAL_THRESHOLD_INVALID: u32 = 2065;
pub const APPEAL_PERIOD_INVALID: u32 = 2066;
pub const APPEAL_REVIEW_PANEL_SIZE_INVALID: u32 = 2067;
pub const APPEAL_FEE_INVALID: u32 = 2068;
pub const GOVERNANCE_OVERSIGHT_DISABLED: u32 = 2069;
pub const EXTERNAL_AUDITORS_DISABLED: u32 = 2070;
pub const COMMUNITY_OVERSIGHT_DISABLED: u32 = 2071;
pub const EMERGENCY_INTERVENTION_DISABLED: u32 = 2072;
pub const WHALE_DETECTION_DISABLED: u32 = 2073;
pub const WHALE_THRESHOLD_INVALID: u32 = 2074;
pub const WHALE_COORDINATION_THRESHOLD_INVALID: u32 = 2075;
pub const SUSPICIOUS_ACTIVITY_DETECTION_DISABLED: u32 = 2076;
pub const SUSPICIOUS_PATTERN_THRESHOLD_INVALID: u32 = 2077;
pub const VOTE_PATTERN_ANALYSIS_DISABLED: u32 = 2078;
pub const COLLUSION_DETECTION_DISABLED: u32 = 2079;
pub const BRIBERY_DETECTION_DISABLED: u32 = 2080;
pub const VOTE_SELLING_DETECTION_DISABLED: u32 = 2081;
pub const WARNING_PENALTY_INVALID: u32 = 2082;
pub const RESTRICTION_PENALTY_INVALID: u32 = 2083;
pub const VOTING_BAN_PENALTY_INVALID: u32 = 2084;
pub const TOKEN_CONFISCATION_PENALTY_INVALID: u32 = 2085;
pub const PERFORMANCE_METRICS_INSUFFICIENT: u32 = 2086;
pub const TARGET_METRICS_NOT_SET: u32 = 2087;
pub const ASSESSMENT_PERIOD_NOT_COMPLETE: u32 = 2088;
pub const COMMUNITY_FEEDBACK_INSUFFICIENT_FOR_ASSESSMENT: u32 = 2089;
pub const POSITIVE_FEEDBACK_PERCENTAGE_INVALID: u32 = 2090;
pub const NEGATIVE_FEEDBACK_PERCENTAGE_INVALID: u32 = 2091;
pub const PERFORMANCE_HISTORY_INSUFFICIENT: u32 = 2092;
pub const TOKEN_PRICE_PERFORMANCE_INVALID: u32 = 2093;
pub const TRADING_VOLUME_INVALID: u32 = 2094;
pub const COMMUNITY_GROWTH_INVALID: u32 = 2095;
pub const STAKING_PARTICIPATION_INVALID: u32 = 2096;
pub const COMMUNITY_SATISFACTION_INVALID: u32 = 2097;
pub const MARKETING_EFFORTS_INVALID: u32 = 2098;
pub const COMMUNITY_ENGAGEMENT_INVALID: u32 = 2099;
pub const TRANSPARENCY_SCORE_INVALID: u32 = 2100;

// Phase 1-3 Feature Errors (2101-2200)
pub const INVALID_USERNAME: u32 = 2101;
pub const DISPLAY_NAME_TOO_LONG: u32 = 2102;
pub const BIO_TOO_LONG: u32 = 2103;
pub const INVALID_MULTI_SIG_CONFIGURATION: u32 = 2104;
pub const TOO_MANY_SIGNERS: u32 = 2105;
pub const INSUFFICIENT_SIGNATURES: u32 = 2106;
pub const TRANSACTION_EXPIRED: u32 = 2107;
pub const EMERGENCY_APPROVAL_REQUIRED: u32 = 2108;
pub const INVALID_SOCIAL_ACTION: u32 = 2109;
pub const USER_NOT_FOUND: u32 = 2110;
pub const ALREADY_FOLLOWING: u32 = 2111;
pub const NOT_FOLLOWING: u32 = 2112;
pub const EVENT_FULL: u32 = 2113;
pub const EVENT_NOT_FOUND: u32 = 2114;
pub const EVENT_ALREADY_JOINED: u32 = 2115;
pub const INVALID_ACHIEVEMENT: u32 = 2116;
pub const ACHIEVEMENT_ALREADY_UNLOCKED: u32 = 2117;
pub const INSUFFICIENT_EXPERIENCE: u32 = 2118;
pub const CHALLENGE_NOT_FOUND: u32 = 2119;
pub const CHALLENGE_ALREADY_COMPLETED: u32 = 2120;
pub const CHALLENGE_EXPIRED: u32 = 2121;
pub const INVALID_STAKE_TYPE: u32 = 2122;
pub const INSUFFICIENT_STAKE_AMOUNT: u32 = 2123;
pub const STAKE_LOCKED: u32 = 2124;
pub const EARLY_WITHDRAWAL_FEE_TOO_HIGH: u32 = 2125;
pub const YIELD_FARMING_POOL_FULL: u32 = 2126;
pub const STAKING_POOL_NOT_FOUND: u32 = 2127;
pub const AUTO_COMPOUNDING_DISABLED: u32 = 2128;
pub const INVALID_COMPOUNDING_FREQUENCY: u32 = 2129;
pub const DASHBOARD_NOT_INITIALIZED: u32 = 2130;
pub const INVALID_NOTIFICATION_SETTINGS: u32 = 2131;
pub const INVALID_DASHBOARD_PREFERENCES: u32 = 2132;
pub const LEADERBOARD_UPDATE_FAILED: u32 = 2133;
pub const MILESTONE_ALREADY_ACHIEVED: u32 = 2134;
pub const INVALID_REWARD_TYPE: u32 = 2135;
pub const REWARD_DISTRIBUTION_FAILED: u32 = 2136;
pub const STREAK_BROKEN: u32 = 2137;
pub const INVALID_LEVEL_UP: u32 = 2138;
pub const EXPERIENCE_CALCULATION_ERROR: u32 = 2139;
pub const INVALID_ENGAGEMENT_METRIC: u32 = 2140;
pub const SOCIAL_VERIFICATION_FAILED: u32 = 2141;
pub const INFLUENCER_VERIFICATION_PENDING: u32 = 2142;
pub const COMMUNITY_CHAT_DISABLED: u32 = 2143;
pub const TOKEN_SHOWCASE_DISABLED: u32 = 2144;
pub const EVENT_CREATION_FAILED: u32 = 2145;
pub const EVENT_MODIFICATION_FAILED: u32 = 2146;
pub const PARTICIPANT_LIMIT_EXCEEDED: u32 = 2147;
pub const REWARDS_POOL_INSUFFICIENT: u32 = 2148;
pub const GAMIFICATION_SYSTEM_ERROR: u32 = 2149;
pub const ADVANCED_STAKING_ERROR: u32 = 2150;
pub const MULTI_SIG_TRANSACTION_FAILED: u32 = 2151;
pub const SIGNATURE_VERIFICATION_FAILED: u32 = 2152;
pub const TIME_LOCK_NOT_EXPIRED: u32 = 2153;
pub const EMERGENCY_CONTACT_INVALID: u32 = 2154;
pub const MAX_TRANSACTION_AMOUNT_EXCEEDED: u32 = 2155;
pub const USER_DASHBOARD_UPDATE_FAILED: u32 = 2156;
pub const SOCIAL_FEATURES_UPDATE_FAILED: u32 = 2157;
pub const GAMIFICATION_UPDATE_FAILED: u32 = 2158;
pub const ADVANCED_STAKING_UPDATE_FAILED: u32 = 2159;
pub const MULTI_SIG_UPDATE_FAILED: u32 = 2160;
pub const PHASE_1_3_FEATURE_DISABLED: u32 = 2161;
pub const INVALID_PHASE_1_3_CONFIGURATION: u32 = 2162;
pub const PHASE_1_3_INITIALIZATION_FAILED: u32 = 2163;
pub const PHASE_1_3_UPGRADE_REQUIRED: u32 = 2164;
pub const PHASE_1_3_MIGRATION_FAILED: u32 = 2165;
pub const PHASE_1_3_ROLLBACK_REQUIRED: u32 = 2166;
pub const PHASE_1_3_MAINTENANCE_MODE: u32 = 2167;
pub const PHASE_1_3_BETA_FEATURE: u32 = 2168;
pub const PHASE_1_3_EXPERIMENTAL_MODE: u32 = 2169;
pub const PHASE_1_3_FEATURE_DEPRECATED: u32 = 2170;
pub const PHASE_1_3_COMPATIBILITY_ERROR: u32 = 2171;
pub const PHASE_1_3_VERSION_MISMATCH: u32 = 2172;
pub const PHASE_1_3_DEPENDENCY_MISSING: u32 = 2173;
pub const PHASE_1_3_INTEGRATION_FAILED: u32 = 2174;
pub const PHASE_1_3_SYNC_ERROR: u32 = 2175;
pub const PHASE_1_3_CACHE_ERROR: u32 = 2176;
pub const PHASE_1_3_DATABASE_ERROR: u32 = 2177;
pub const PHASE_1_3_NETWORK_ERROR: u32 = 2178;
pub const PHASE_1_3_TIMEOUT_ERROR: u32 = 2179;
pub const PHASE_1_3_RATE_LIMIT_EXCEEDED: u32 = 2180;
pub const PHASE_1_3_QUOTA_EXCEEDED: u32 = 2181;
pub const PHASE_1_3_RESOURCE_UNAVAILABLE: u32 = 2182;
pub const PHASE_1_3_SERVICE_UNAVAILABLE: u32 = 2183;
pub const PHASE_1_3_GATEWAY_ERROR: u32 = 2184;
pub const PHASE_1_3_PROXY_ERROR: u32 = 2185;
pub const PHASE_1_3_LOAD_BALANCER_ERROR: u32 = 2186;
pub const PHASE_1_3_CDN_ERROR: u32 = 2187;
pub const PHASE_1_3_DNS_ERROR: u32 = 2188;
pub const PHASE_1_3_SSL_ERROR: u32 = 2189;
pub const PHASE_1_3_CERTIFICATE_ERROR: u32 = 2190;
pub const PHASE_1_3_AUTHENTICATION_ERROR: u32 = 2191;
pub const PHASE_1_3_AUTHORIZATION_ERROR: u32 = 2192;
pub const PHASE_1_3_SESSION_EXPIRED: u32 = 2193;
pub const PHASE_1_3_TOKEN_INVALID: u32 = 2194;
pub const PHASE_1_3_TOKEN_EXPIRED: u32 = 2195;
pub const PHASE_1_3_REFRESH_TOKEN_INVALID: u32 = 2196;
pub const PHASE_1_3_ACCESS_DENIED: u32 = 2197;
pub const PHASE_1_3_PERMISSION_DENIED: u32 = 2198;
pub const PHASE_1_3_FORBIDDEN: u32 = 2199;
pub const PHASE_1_3_UNAUTHORIZED: u32 = 2200;
