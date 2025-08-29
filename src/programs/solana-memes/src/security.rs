use anchor_lang::prelude::*;

/// Reentrancy guard for preventing reentrant calls
#[account]
pub struct ReentrancyGuard {
    pub locked: bool,
    pub caller: Pubkey,
    pub timestamp: i64,
}

/// Emergency configuration for pausing the program
#[account]
pub struct EmergencyConfig {
    pub paused: bool,
    pub pause_authority: Pubkey,
    pub pause_reason: String,
    pub pause_timestamp: i64,
    pub unpause_timestamp: Option<i64>,
}

/// Access control for different roles
#[account]
pub struct AccessControl {
    pub admin: Pubkey,
    pub moderators: Vec<Pubkey>,
    pub emergency_authority: Pubkey,
    pub treasury_authority: Pubkey,
    pub governance_authority: Pubkey,
}

/// Rate limiting for operations
#[account]
pub struct RateLimit {
    pub operation: String,
    pub last_execution: i64,
    pub cooldown_period: i64,
    pub max_attempts: u32,
    pub current_attempts: u32,
    pub window_start: i64,
}

/// Security constants
pub const EMERGENCY_AUTHORITY: &str = "EmergencyAuthority111111111111111111111111111111";
pub const ADMIN_AUTHORITY: &str = "AdminAuthority111111111111111111111111111111111111";
pub const TREASURY_AUTHORITY: &str = "TreasuryAuthority11111111111111111111111111111111";

/// Security error codes
#[error_code]
pub enum SecurityError {
    #[msg("Program is currently paused")]
    ProgramPaused,
    
    #[msg("Reentrant call detected")]
    ReentrantCall,
    
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[msg("Invalid authority")]
    InvalidAuthority,
    
    #[msg("Emergency pause not allowed")]
    EmergencyPauseNotAllowed,
    
    #[msg("Cooldown period not elapsed")]
    CooldownNotElapsed,
}

/// Security utilities
pub struct SecurityUtils;

impl SecurityUtils {
    /// Check if program is paused
    pub fn check_paused(emergency_config: &EmergencyConfig) -> Result<()> {
        require!(!emergency_config.paused, SecurityError::ProgramPaused);
        Ok(())
    }

    /// Check reentrancy guard
    pub fn check_reentrancy(guard: &ReentrancyGuard, caller: &Pubkey) -> Result<()> {
        require!(!guard.locked, SecurityError::ReentrantCall);
        Ok(())
    }

    /// Set reentrancy guard
    pub fn set_reentrancy_guard(guard: &mut ReentrancyGuard, caller: &Pubkey) -> Result<()> {
        guard.locked = true;
        guard.caller = *caller;
        guard.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// Clear reentrancy guard
    pub fn clear_reentrancy_guard(guard: &mut ReentrancyGuard) -> Result<()> {
        guard.locked = false;
        Ok(())
    }

    /// Check rate limiting
    pub fn check_rate_limit(rate_limit: &mut RateLimit) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        // Reset window if needed
        if current_time - rate_limit.window_start > rate_limit.cooldown_period {
            rate_limit.current_attempts = 0;
            rate_limit.window_start = current_time;
        }

        // Check if rate limit exceeded
        require!(
            rate_limit.current_attempts < rate_limit.max_attempts,
            SecurityError::RateLimitExceeded
        );

        // Update attempts
        rate_limit.current_attempts += 1;
        rate_limit.last_execution = current_time;

        Ok(())
    }

    /// Validate authority
    pub fn validate_authority(
        authority: &Pubkey,
        expected_authority: &Pubkey,
    ) -> Result<()> {
        require_keys_eq!(*authority, *expected_authority, SecurityError::InvalidAuthority);
        Ok(())
    }

    /// Check if address is blacklisted
    pub fn is_blacklisted(address: &Pubkey, blacklist: &[Pubkey]) -> bool {
        blacklist.contains(address)
    }

    /// Check if address is whitelisted
    pub fn is_whitelisted(address: &Pubkey, whitelist: &[Pubkey]) -> bool {
        whitelist.contains(address)
    }

    /// Validate transaction size limits
    pub fn validate_transaction_size(
        amount: u64,
        max_size: u64,
        min_size: u64,
    ) -> Result<()> {
        require!(amount <= max_size, SecurityError::RateLimitExceeded);
        require!(amount >= min_size, SecurityError::RateLimitExceeded);
        Ok(())
    }

    /// Check cooldown period
    pub fn check_cooldown(last_execution: i64, cooldown_period: i64) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;
        let time_since_last = current_time - last_execution;
        
        require!(
            time_since_last >= cooldown_period,
            SecurityError::CooldownNotElapsed
        );
        
        Ok(())
    }

    /// Validate percentage bounds
    pub fn validate_percentage(percentage: u8, min: u8, max: u8) -> Result<()> {
        require!(percentage >= min, SecurityError::InvalidAuthority);
        require!(percentage <= max, SecurityError::InvalidAuthority);
        Ok(())
    }

    /// Validate time bounds
    pub fn validate_time_bounds(
        start_time: i64,
        end_time: i64,
        min_duration: i64,
        max_duration: i64,
    ) -> Result<()> {
        let duration = end_time - start_time;
        require!(duration >= min_duration, SecurityError::InvalidAuthority);
        require!(duration <= max_duration, SecurityError::InvalidAuthority);
        require!(end_time > start_time, SecurityError::InvalidAuthority);
        Ok(())
    }

    /// Generate secure random seed
    pub fn generate_secure_seed() -> [u8; 32] {
        let clock = Clock::get().unwrap();
        let timestamp = clock.unix_timestamp.to_le_bytes();
        let slot = clock.slot.to_le_bytes();
        
        let mut seed = [0u8; 32];
        seed[0..8].copy_from_slice(&timestamp);
        seed[8..16].copy_from_slice(&slot);
        
        seed
    }

    /// Validate string length
    pub fn validate_string_length(s: &str, max_length: usize) -> Result<()> {
        require!(s.len() <= max_length, SecurityError::InvalidAuthority);
        Ok(())
    }

    /// Validate numeric bounds
    pub fn validate_numeric_bounds<T: PartialOrd>(
        value: T,
        min: T,
        max: T,
    ) -> Result<()> {
        require!(value >= min, SecurityError::InvalidAuthority);
        require!(value <= max, SecurityError::InvalidAuthority);
        Ok(())
    }
}

/// Security context for operations
pub struct SecurityContext<'info> {
    pub emergency_config: &'info Account<'info, EmergencyConfig>,
    pub reentrancy_guard: &'info mut Account<'info, ReentrancyGuard>,
    pub access_control: &'info Account<'info, AccessControl>,
    pub rate_limit: &'info mut Account<'info, RateLimit>,
}

impl<'info> SecurityContext<'info> {
    /// Perform security checks
    pub fn perform_checks(&mut self, caller: &Pubkey) -> Result<()> {
        // Check if program is paused
        SecurityUtils::check_paused(self.emergency_config)?;
        
        // Check reentrancy
        SecurityUtils::check_reentrancy(self.reentrancy_guard, caller)?;
        
        // Set reentrancy guard
        SecurityUtils::set_reentrancy_guard(self.reentrancy_guard, caller)?;
        
        // Check rate limiting
        SecurityUtils::check_rate_limit(self.rate_limit)?;
        
        Ok(())
    }

    /// Clear security state
    pub fn clear_state(&mut self) -> Result<()> {
        SecurityUtils::clear_reentrancy_guard(self.reentrancy_guard)?;
        Ok(())
    }

    /// Validate admin access
    pub fn validate_admin(&self, authority: &Pubkey) -> Result<()> {
        SecurityUtils::validate_authority(authority, &self.access_control.admin)?;
        Ok(())
    }

    /// Validate emergency authority
    pub fn validate_emergency_authority(&self, authority: &Pubkey) -> Result<()> {
        SecurityUtils::validate_authority(authority, &self.access_control.emergency_authority)?;
        Ok(())
    }

    /// Validate moderator access
    pub fn validate_moderator(&self, authority: &Pubkey) -> Result<()> {
        let is_admin = authority == &self.access_control.admin;
        let is_moderator = self.access_control.moderators.contains(authority);
        
        require!(is_admin || is_moderator, SecurityError::Unauthorized);
        Ok(())
    }
}

/// Emergency pause instruction
pub fn emergency_pause(
    ctx: Context<EmergencyPause>,
    reason: String,
) -> Result<()> {
    let emergency_config = &mut ctx.accounts.emergency_config;
    let clock = Clock::get()?;
    
    // Validate emergency authority
    SecurityUtils::validate_authority(
        &ctx.accounts.authority.key(),
        &emergency_config.pause_authority,
    )?;
    
    // Set pause state
    emergency_config.paused = true;
    emergency_config.pause_reason = reason;
    emergency_config.pause_timestamp = clock.unix_timestamp;
    emergency_config.unpause_timestamp = None;
    
    Ok(())
}

/// Emergency unpause instruction
pub fn emergency_unpause(ctx: Context<EmergencyPause>) -> Result<()> {
    let emergency_config = &mut ctx.accounts.emergency_config;
    let clock = Clock::get()?;
    
    // Validate emergency authority
    SecurityUtils::validate_authority(
        &ctx.accounts.authority.key(),
        &emergency_config.pause_authority,
    )?;
    
    // Clear pause state
    emergency_config.paused = false;
    emergency_config.pause_reason = "".to_string();
    emergency_config.unpause_timestamp = Some(clock.unix_timestamp);
    
    Ok(())
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
    let access_control = &mut ctx.accounts.access_control;
    
    // Validate current admin
    SecurityUtils::validate_authority(
        &ctx.accounts.authority.key(),
        &access_control.admin,
    )?;
    
    // Update authorities
    if let Some(admin) = new_admin {
        access_control.admin = admin;
    }
    
    if let Some(emergency_authority) = new_emergency_authority {
        access_control.emergency_authority = emergency_authority;
    }
    
    if let Some(treasury_authority) = new_treasury_authority {
        access_control.treasury_authority = treasury_authority;
    }
    
    // Update moderators
    if let Some(moderator) = add_moderator {
        if !access_control.moderators.contains(&moderator) {
            access_control.moderators.push(moderator);
        }
    }
    
    if let Some(moderator) = remove_moderator {
        access_control.moderators.retain(|&x| x != moderator);
    }
    
    Ok(())
}

/// Account structures for security instructions
#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(mut)]
    pub emergency_config: Account<'info, EmergencyConfig>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAccessControl<'info> {
    #[account(mut)]
    pub access_control: Account<'info, AccessControl>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecurityCheck<'info> {
    #[account(mut)]
    pub emergency_config: Account<'info, EmergencyConfig>,
    #[account(mut)]
    pub reentrancy_guard: Account<'info, ReentrancyGuard>,
    pub access_control: Account<'info, AccessControl>,
    #[account(mut)]
    pub rate_limit: Account<'info, RateLimit>,
}
