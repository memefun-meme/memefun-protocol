use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;
use crate::errors::CustomError;

// Initialize leaderboard system
#[derive(Accounts)]
pub struct InitializeLeaderboard<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<LeaderboardSystem>(),
        seeds = [b"leaderboard_system"],
        bump
    )]
    pub leaderboard: Account<'info, LeaderboardSystem>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_leaderboard(
    ctx: Context<InitializeLeaderboard>,
    update_frequency: i64,
    minimum_qualification: u64,
) -> Result<()> {
    let leaderboard = &mut ctx.accounts.leaderboard;
    let clock = Clock::get()?;
    
    leaderboard.authority = ctx.accounts.authority.key();
    leaderboard.update_frequency = update_frequency;
    leaderboard.minimum_qualification = minimum_qualification;
    leaderboard.last_update_time = clock.unix_timestamp;
    leaderboard.created_at = clock.unix_timestamp;
    leaderboard.updated_at = clock.unix_timestamp;
    
    // Initialize empty rankings
    leaderboard.top_creators_by_volume = Vec::new();
    leaderboard.top_creators_by_holders = Vec::new();
    leaderboard.top_creators_by_success_rate = Vec::new();
    leaderboard.top_creators_by_revenue = Vec::new();
    
    leaderboard.top_traders_by_roi = Vec::new();
    leaderboard.top_traders_by_volume = Vec::new();
    leaderboard.top_traders_by_profit = Vec::new();
    leaderboard.top_traders_by_accuracy = Vec::new();
    
    leaderboard.top_tokens_by_market_cap = Vec::new();
    leaderboard.top_tokens_by_volume = Vec::new();
    leaderboard.top_tokens_by_growth = Vec::new();
    leaderboard.top_tokens_by_holders = Vec::new();
    
    leaderboard.weekly_hall_of_fame = Vec::new();
    leaderboard.historical_winners = Vec::new();
    
    Ok(())
}

// Update creator rankings
#[derive(Accounts)]
pub struct UpdateCreatorRankings<'info> {
    #[account(mut, has_one = authority)]
    pub leaderboard: Account<'info, LeaderboardSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub creator_profile: Account<'info, CreatorProfile>,
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
}

pub fn update_creator_rankings(ctx: Context<UpdateCreatorRankings>) -> Result<()> {
    let leaderboard = &mut ctx.accounts.leaderboard;
    let creator_profile = &ctx.accounts.creator_profile;
    let token_metadata = &ctx.accounts.token_metadata;
    let clock = Clock::get()?;
    
    // Check if it's time to update rankings
    require!(
        clock.unix_timestamp - leaderboard.last_update_time >= leaderboard.update_frequency,
        CustomError::UpdateTooFrequent
    );
    
    // Create creator ranking
    let creator_ranking = CreatorRanking {
        creator: creator_profile.owner,
        rank: 0, // Will be calculated after sorting
        total_volume: creator_profile.total_volume,
        total_holders: creator_profile.total_tokens_created * 100, // Estimate
        success_rate: if creator_profile.total_tokens_created > 0 {
            creator_profile.successful_tokens as f64 / creator_profile.total_tokens_created as f64
        } else {
            0.0
        },
        total_revenue: creator_profile.total_volume * 12 / 100, // 1.2% fee estimate
        tokens_created: creator_profile.total_tokens_created,
        average_token_performance: 0.0, // Calculate from token metadata
        reputation_score: creator_profile.reputation_score,
        last_activity: clock.unix_timestamp,
    };
    
    // Update volume rankings
    update_ranking_list(&mut leaderboard.top_creators_by_volume, creator_ranking.clone(), |a, b| {
        a.total_volume.cmp(&b.total_volume)
    });
    
    // Update holder rankings
    update_ranking_list(&mut leaderboard.top_creators_by_holders, creator_ranking.clone(), |a, b| {
        a.total_holders.cmp(&b.total_holders)
    });
    
    // Update success rate rankings
    update_ranking_list(&mut leaderboard.top_creators_by_success_rate, creator_ranking.clone(), |a, b| {
        a.success_rate.partial_cmp(&b.success_rate).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Update revenue rankings
    update_ranking_list(&mut leaderboard.top_creators_by_revenue, creator_ranking, |a, b| {
        a.total_revenue.cmp(&b.total_revenue)
    });
    
    leaderboard.last_update_time = clock.unix_timestamp;
    leaderboard.updated_at = clock.unix_timestamp;
    
    Ok(())
}

// Update trader rankings
#[derive(Accounts)]
pub struct UpdateTraderRankings<'info> {
    #[account(mut, has_one = authority)]
    pub leaderboard: Account<'info, LeaderboardSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub user_dashboard: Account<'info, UserDashboard>,
}

pub fn update_trader_rankings(ctx: Context<UpdateTraderRankings>) -> Result<()> {
    let leaderboard = &mut ctx.accounts.leaderboard;
    let user_dashboard = &ctx.accounts.user_dashboard;
    let clock = Clock::get()?;
    
    // Create trader ranking
    let trader_ranking = TraderRanking {
        trader: user_dashboard.user,
        rank: 0,
        total_roi: user_dashboard.average_token_performance,
        total_volume: user_dashboard.total_volume_traded,
        total_profit: user_dashboard.total_rewards_earned,
        trading_accuracy: user_dashboard.success_rate,
        total_trades: user_dashboard.total_transactions,
        winning_trades: (user_dashboard.total_transactions as f64 * user_dashboard.success_rate) as u32,
        average_trade_size: if user_dashboard.total_transactions > 0 {
            user_dashboard.total_volume_traded / user_dashboard.total_transactions
        } else {
            0
        },
        best_trade_roi: user_dashboard.average_token_performance * 2.0, // Estimate
        last_activity: user_dashboard.last_activity,
    };
    
    // Update ROI rankings
    update_ranking_list(&mut leaderboard.top_traders_by_roi, trader_ranking.clone(), |a, b| {
        a.total_roi.partial_cmp(&b.total_roi).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Update volume rankings
    update_ranking_list(&mut leaderboard.top_traders_by_volume, trader_ranking.clone(), |a, b| {
        a.total_volume.cmp(&b.total_volume)
    });
    
    // Update profit rankings
    update_ranking_list(&mut leaderboard.top_traders_by_profit, trader_ranking.clone(), |a, b| {
        a.total_profit.cmp(&b.total_profit)
    });
    
    // Update accuracy rankings
    update_ranking_list(&mut leaderboard.top_traders_by_accuracy, trader_ranking, |a, b| {
        a.trading_accuracy.partial_cmp(&b.trading_accuracy).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    leaderboard.updated_at = clock.unix_timestamp;
    
    Ok(())
}

// Update token rankings
#[derive(Accounts)]
pub struct UpdateTokenRankings<'info> {
    #[account(mut, has_one = authority)]
    pub leaderboard: Account<'info, LeaderboardSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
}

pub fn update_token_rankings(ctx: Context<UpdateTokenRankings>) -> Result<()> {
    let leaderboard = &mut ctx.accounts.leaderboard;
    let token_metadata = &ctx.accounts.token_metadata;
    let liquidity_pool = &ctx.accounts.liquidity_pool;
    let clock = Clock::get()?;
    
    // Calculate market cap (simplified)
    let market_cap = token_metadata.circulating_supply * 1000; // Estimate price
    
    // Create token ranking
    let token_ranking = TokenRanking {
        token_mint: token_metadata.mint,
        rank: 0,
        market_cap,
        volume_24h: liquidity_pool.total_volume,
        growth_percentage: 0.0, // Calculate from historical data
        holder_count: 1000, // Estimate
        price_change_24h: 0.0, // Calculate from price data
        liquidity: liquidity_pool.sol_reserve + liquidity_pool.token_reserve,
        creator: token_metadata.creator,
        created_at: token_metadata.created_at,
    };
    
    // Update market cap rankings
    update_ranking_list(&mut leaderboard.top_tokens_by_market_cap, token_ranking.clone(), |a, b| {
        a.market_cap.cmp(&b.market_cap)
    });
    
    // Update volume rankings
    update_ranking_list(&mut leaderboard.top_tokens_by_volume, token_ranking.clone(), |a, b| {
        a.volume_24h.cmp(&b.volume_24h)
    });
    
    // Update growth rankings
    update_ranking_list(&mut leaderboard.top_tokens_by_growth, token_ranking.clone(), |a, b| {
        a.growth_percentage.partial_cmp(&b.growth_percentage).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    // Update holder rankings
    update_ranking_list(&mut leaderboard.top_tokens_by_holders, token_ranking, |a, b| {
        a.holder_count.cmp(&b.holder_count)
    });
    
    leaderboard.updated_at = clock.unix_timestamp;
    
    Ok(())
}

// Start weekly competition
#[derive(Accounts)]
pub struct StartWeeklyCompetition<'info> {
    #[account(mut, has_one = authority)]
    pub leaderboard: Account<'info, LeaderboardSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
}

pub fn start_weekly_competition(
    ctx: Context<StartWeeklyCompetition>,
    week_number: u32,
    rewards_pool: u64,
) -> Result<()> {
    let leaderboard = &mut ctx.accounts.leaderboard;
    let clock = Clock::get()?;
    
    // End current competition if active
    if leaderboard.current_week_competition.is_active {
        end_weekly_competition(leaderboard)?;
    }
    
    // Start new competition
    leaderboard.current_week_competition = WeeklyCompetition {
        week_number,
        start_time: clock.unix_timestamp,
        end_time: clock.unix_timestamp + 7 * 24 * 60 * 60, // 7 days
        total_participants: 0,
        total_volume: 0,
        categories: vec![
            CompetitionCategory::BestCreator,
            CompetitionCategory::BestTrader,
            CompetitionCategory::BestToken,
            CompetitionCategory::MostInnovative,
            CompetitionCategory::CommunityChoice,
        ],
        rewards_pool,
        is_active: true,
    };
    
    leaderboard.updated_at = clock.unix_timestamp;
    
    msg!("Weekly competition {} started with {} SOL rewards pool", week_number, rewards_pool);
    
    Ok(())
}

// End weekly competition and declare winners
#[derive(Accounts)]
pub struct EndWeeklyCompetition<'info> {
    #[account(mut, has_one = authority)]
    pub leaderboard: Account<'info, LeaderboardSystem>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
}

pub fn end_weekly_competition_handler(ctx: Context<EndWeeklyCompetition>) -> Result<()> {
    let leaderboard = &mut ctx.accounts.leaderboard;
    let clock = Clock::get()?;
    
    require!(
        leaderboard.current_week_competition.is_active,
        CustomError::CompetitionNotActive
    );
    
    require!(
        clock.unix_timestamp >= leaderboard.current_week_competition.end_time,
        CustomError::CompetitionNotEnded
    );
    
    end_weekly_competition(leaderboard)?;
    
    Ok(())
}

// Helper function to end competition
fn end_weekly_competition(leaderboard: &mut Account<LeaderboardSystem>) -> Result<()> {
    let clock = Clock::get()?;
    let competition = &leaderboard.current_week_competition;
    
    // Declare winners for each category
    for category in &competition.categories {
        let winner = determine_winner(leaderboard, category)?;
        
        let weekly_winner = WeeklyWinner {
            week_number: competition.week_number,
            category: category.clone(),
            winner: winner.0,
            score: winner.1,
            reward_amount: competition.rewards_pool / 5, // Split equally among 5 categories
            timestamp: clock.unix_timestamp,
        };
        
        leaderboard.weekly_hall_of_fame.push(weekly_winner.clone());
        leaderboard.historical_winners.push(HistoricalWinner {
            week_number: weekly_winner.week_number,
            category: weekly_winner.category,
            winner: weekly_winner.winner,
            score: weekly_winner.score,
            reward_amount: weekly_winner.reward_amount,
            timestamp: weekly_winner.timestamp,
        });
    }
    
    // Mark competition as inactive
    leaderboard.current_week_competition.is_active = false;
    leaderboard.updated_at = clock.unix_timestamp;
    
    msg!("Weekly competition {} ended. Winners declared!", competition.week_number);
    
    Ok(())
}

// Helper function to determine winner for a category
fn determine_winner(
    leaderboard: &LeaderboardSystem,
    category: &CompetitionCategory,
) -> Result<(Pubkey, u64)> {
    match category {
        CompetitionCategory::BestCreator => {
            if let Some(top_creator) = leaderboard.top_creators_by_success_rate.first() {
                Ok((top_creator.creator, top_creator.total_revenue))
            } else {
                Err(CustomError::NoParticipants.into())
            }
        }
        CompetitionCategory::BestTrader => {
            if let Some(top_trader) = leaderboard.top_traders_by_roi.first() {
                Ok((top_trader.trader, top_trader.total_profit))
            } else {
                Err(CustomError::NoParticipants.into())
            }
        }
        CompetitionCategory::BestToken => {
            if let Some(top_token) = leaderboard.top_tokens_by_growth.first() {
                Ok((top_token.creator, top_token.market_cap))
            } else {
                Err(CustomError::NoParticipants.into())
            }
        }
        CompetitionCategory::MostInnovative => {
            // For now, use best creator as most innovative
            if let Some(top_creator) = leaderboard.top_creators_by_success_rate.first() {
                Ok((top_creator.creator, top_creator.total_revenue))
            } else {
                Err(CustomError::NoParticipants.into())
            }
        }
        CompetitionCategory::CommunityChoice => {
            // For now, use token with most holders
            if let Some(top_token) = leaderboard.top_tokens_by_holders.first() {
                Ok((top_token.creator, top_token.holder_count as u64))
            } else {
                Err(CustomError::NoParticipants.into())
            }
        }
    }
}

// Helper function to update ranking lists
fn update_ranking_list<T, F>(
    rankings: &mut Vec<T>,
    new_entry: T,
    compare_fn: F,
) where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    // Remove existing entry if exists
    rankings.retain(|entry| {
        // This is a simplified comparison - in real implementation, you'd have a unique identifier
        true
    });
    
    // Add new entry
    rankings.push(new_entry);
    
    // Sort by comparison function
    rankings.sort_by(compare_fn);
    rankings.reverse(); // Highest first
    
    // Keep only top 100
    if rankings.len() > 100 {
        rankings.truncate(100);
    }
    
    // Update ranks
    for (index, entry) in rankings.iter_mut().enumerate() {
        // In a real implementation, you'd update the rank field
        // For now, we'll just keep the sorted order
    }
}
