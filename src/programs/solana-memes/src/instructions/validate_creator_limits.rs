use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct ValidateCreatorLimits<'info> {
    #[account(mut)]
    pub creator_profile: Account<'info, CreatorProfile>,
    
    pub creator: Signer<'info>,
}

pub fn handler(ctx: Context<ValidateCreatorLimits>) -> Result<()> {
    let creator_profile = &mut ctx.accounts.creator_profile;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if creator is registered and not banned
    require!(creator_profile.is_registered, SolanaMemesError::CreatorNotRegistered);
    require!(!creator_profile.is_banned, SolanaMemesError::CreatorBanned);
    
    // Reset weekly count if a week has passed
    if current_time - creator_profile.last_week_reset >= WEEK_IN_SECONDS {
        creator_profile.weekly_creation_count = 0;
        creator_profile.last_week_reset = current_time;
    }
    
    // Check weekly creation limit
    require!(
        creator_profile.weekly_creation_count < MAX_WEEKLY_CREATIONS,
        SolanaMemesError::WeeklyCreationLimitExceeded
    );
    
    // Validate maximum allocation percentage
    require!(
        creator_profile.max_allocation_percent <= MAX_CREATOR_ALLOCATION_PERCENT,
        SolanaMemesError::AllocationExceedsMaximum
    );
    
    // Increment weekly creation count
    creator_profile.weekly_creation_count += 1;
    creator_profile.last_creation_ts = current_time;
    
    Ok(())
}
