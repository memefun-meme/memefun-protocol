use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct ChooseVestingOption<'info> {
    #[account(mut)]
    pub vesting: Account<'info, Vesting>,
    pub creator: Signer<'info>,
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vesting_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub treasury: Account<'info, PlatformTreasury>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ChooseVestingOption>, option: VestingOption) -> Result<()> {
    let vesting = &mut ctx.accounts.vesting;
    let clock = Clock::get()?;
    
    // Verify creator ownership
    require!(
        vesting.owner == ctx.accounts.creator.key(),
        CustomError::Unauthorized
    );
    
    // Check if vesting period has ended
    require!(
        clock.unix_timestamp >= vesting.cliff_time,
        CustomError::VestingNotAvailable
    );
    
    // Check if choice deadline hasn't passed
    require!(
        clock.unix_timestamp <= vesting.choice_deadline,
        CustomError::ChoiceDeadlinePassed
    );
    
    // Check if choice hasn't already been made
    require!(
        !vesting.choice_made,
        CustomError::ChoiceAlreadyMade
    );
    
    // Set the distribution choice
    vesting.distribution_choice = Some(option);
    vesting.choice_made = true;
    
    // Calculate claimable amount
    let claimable_amount = vesting.amount - vesting.released;
    
    match option {
        VestingOption::Withdraw => {
            // Creator gets all tokens
            transfer_tokens(
                &ctx.accounts.vesting_token_account,
                &ctx.accounts.creator_token_account,
                &ctx.accounts.creator,
                claimable_amount,
                &ctx.accounts.token_program,
            )?;
            
            vesting.released = vesting.amount;
        },
        
        VestingOption::Burn => {
            // Burn 50%, creator gets 50%
            let burn_amount = claimable_amount / 2;
            let creator_amount = claimable_amount - burn_amount;
            
            // Burn tokens
            burn_tokens(
                &ctx.accounts.vesting_token_account,
                burn_amount,
                &ctx.accounts.token_program,
            )?;
            
            // Transfer remaining to creator
            transfer_tokens(
                &ctx.accounts.vesting_token_account,
                &ctx.accounts.creator_token_account,
                &ctx.accounts.creator,
                creator_amount,
                &ctx.accounts.token_program,
            )?;
            
            vesting.released = vesting.amount;
        },
        
        VestingOption::Distribute => {
            // Distribute 50% to holders, creator gets 50%
            let distribute_amount = claimable_amount / 2;
            let creator_amount = claimable_amount - distribute_amount;
            
            // Transfer to treasury for distribution to holders
            transfer_tokens(
                &ctx.accounts.vesting_token_account,
                &ctx.accounts.treasury,
                &ctx.accounts.creator,
                distribute_amount,
                &ctx.accounts.token_program,
            )?;
            
            // Transfer remaining to creator
            transfer_tokens(
                &ctx.accounts.vesting_token_account,
                &ctx.accounts.creator_token_account,
                &ctx.accounts.creator,
                creator_amount,
                &ctx.accounts.token_program,
            )?;
            
            vesting.released = vesting.amount;
        },
    }
    
    Ok(())
}

fn transfer_tokens(
    from: &Account<TokenAccount>,
    to: &Account<TokenAccount>,
    authority: &Signer,
    amount: u64,
    token_program: &Program<Token>,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
    
    let cpi_program = token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

fn burn_tokens(
    token_account: &Account<TokenAccount>,
    amount: u64,
    token_program: &Program<Token>,
) -> Result<()> {
    let cpi_accounts = Burn {
        mint: token_account.to_account_info(),
        from: token_account.to_account_info(),
        authority: token_account.to_account_info(),
    };
    
    let cpi_program = token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::burn(cpi_ctx, amount)?;
    Ok(())
}
