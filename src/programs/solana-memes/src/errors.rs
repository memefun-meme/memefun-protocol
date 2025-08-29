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
}
