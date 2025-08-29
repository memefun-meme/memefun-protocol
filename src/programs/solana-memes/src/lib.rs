use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, MintTo, InitializeMint, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod buyback;
pub mod security;

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
        ctx: Context<InitializeBuybackConfig>,
        burn_percent: u8,
        lp_percent: u8,
    ) -> Result<()> {
        buyback::initialize_config(ctx, burn_percent, lp_percent)
    }

    /// Update buyback configuration
    pub fn update_buyback_config(
        ctx: Context<UpdateBuybackConfig>,
        burn_percent: u8,
        lp_percent: u8,
    ) -> Result<()> {
        buyback::update_config(ctx, burn_percent, lp_percent)
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
}
