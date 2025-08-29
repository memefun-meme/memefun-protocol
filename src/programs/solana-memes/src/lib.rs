use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::*;

#[program]
pub mod solana_memes {
    use super::*;

    /// Initialize the memecoin program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Create a new memecoin token
    pub fn create_token(
        ctx: Context<CreateToken>,
        name: String,
        symbol: String,
        uri: String,
        decimals: u8,
        total_supply: u64,
    ) -> Result<()> {
        instructions::create_token::handler(ctx, name, symbol, uri, decimals, total_supply)
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

    /// Mint additional tokens (only for token creator)
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
}
