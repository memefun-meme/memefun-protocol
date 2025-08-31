use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::CustomError;

/// Security utilities for circuit breakers, flash loan protection, and trade validation
pub struct SecurityUtils;

impl SecurityUtils {
    /// Check circuit breaker for extreme price movements and volume spikes
    pub fn check_circuit_breaker(
        current_price: u64,
        previous_price: u64,
        trade_volume: u64,
        circuit_breaker: &CircuitBreaker,
        current_time: i64,
    ) -> Result<()> {
        // Check if circuit breaker is already triggered
        if circuit_breaker.is_triggered {
            // Check if cooldown period has passed
            if current_time - circuit_breaker.last_trigger_time < circuit_breaker.cooldown_period {
                return err!(CustomError::CircuitBreakerTriggered);
            } else {
                // Reset circuit breaker after cooldown
                return Ok(());
            }
        }

        // Calculate price change percentage
        let price_change = if current_price > previous_price {
            ((current_price - previous_price) * 100) / previous_price
        } else {
            ((previous_price - current_price) * 100) / previous_price
        };

        // Check if price change exceeds maximum allowed
        if price_change > circuit_breaker.max_price_change_percent as u64 {
            return err!(CustomError::PriceChangeTooLarge);
        }

        // Check volume limits
        if trade_volume > circuit_breaker.max_volume_per_period {
            return err!(CustomError::VolumeLimitExceeded);
        }

        Ok(())
    }

    /// Validate trade for flash loan protection
    pub fn validate_trade(
        trader: Pubkey,
        trade_amount: u64,
        current_time: i64,
        trade_protection: &TradeProtection,
    ) -> Result<()> {
        // Check trade amount limits
        if trade_amount > trade_protection.max_trade_amount {
            return err!(CustomError::TradeTooLarge);
        }

        // Find trader's last trade time
        let last_trade_time = trade_protection
            .last_trade_times
            .iter()
            .find(|record| record.trader == trader)
            .map(|record| record.last_trade_time)
            .unwrap_or(0);

        // Check trade frequency
        if current_time - last_trade_time < trade_protection.min_trade_interval {
            return err!(CustomError::TradeTooFrequent);
        }

        // Check daily volume limits
        let today = current_time - (current_time % 86400); // Start of day
        let daily_volume = trade_protection
            .daily_volumes
            .iter()
            .find(|record| record.trader == trader && record.date == today)
            .map(|record| record.volume)
            .unwrap_or(0);

        if daily_volume + trade_amount > trade_protection.max_daily_volume {
            return err!(CustomError::DailyVolumeLimitExceeded);
        }

        // Check for suspicious patterns
        if trade_amount > trade_protection.suspicious_pattern_threshold {
            // Additional checks for large trades
            Self::check_suspicious_patterns(trader, trade_amount, current_time, trade_protection)?;
        }

        Ok(())
    }

    /// Check for suspicious trading patterns
    pub fn check_suspicious_patterns(
        trader: Pubkey,
        trade_amount: u64,
        current_time: i64,
        trade_protection: &TradeProtection,
    ) -> Result<()> {
        // Check for rapid successive trades
        let recent_trades = trade_protection
            .last_trade_times
            .iter()
            .filter(|record| {
                record.trader == trader && 
                current_time - record.last_trade_time < 300 // 5 minutes
            })
            .count();

        if recent_trades > 5 {
            return err!(CustomError::SuspiciousTradingPattern);
        }

        // Check for unusual trade amounts
        let avg_trade_amount = trade_protection
            .last_trade_times
            .iter()
            .filter(|record| record.trader == trader)
            .map(|record| record.trade_amount)
            .sum::<u64>()
            .checked_div(recent_trades as u64)
            .unwrap_or(0);

        // If trade amount is 10x larger than average, flag as suspicious
        if trade_amount > avg_trade_amount * 10 && avg_trade_amount > 0 {
            return err!(CustomError::SuspiciousTradingPattern);
        }

        Ok(())
    }

    /// Validate multi-signature distribution
    pub fn validate_multi_sig_distribution(
        amount: u64,
        signatures: &[Signature],
        multi_sig: &MultiSigGovernance,
        current_time: i64,
    ) -> Result<()> {
        // Check distribution threshold
        if amount > multi_sig.distribution_threshold {
            return err!(CustomError::DistributionTooLarge);
        }

        // Check signature count
        if signatures.len() < multi_sig.required_signatures as usize {
            return err!(CustomError::InsufficientSignatures);
        }

        // Validate signatures (basic check - in real implementation, verify against signers)
        if signatures.len() > multi_sig.signers.len() {
            return err!(CustomError::InsufficientSignatures);
        }

        Ok(())
    }

    /// Check emergency pause status
    pub fn check_emergency_pause(emergency_controls: &EmergencyControls) -> Result<()> {
        if emergency_controls.emergency_pause {
            return err!(CustomError::EmergencyPauseActive);
        }

        if emergency_controls.circuit_breaker_active {
            return err!(CustomError::CircuitBreakerTriggered);
        }

        Ok(())
    }

    /// Update trade protection records
    pub fn update_trade_records(
        trader: Pubkey,
        trade_amount: u64,
        current_time: i64,
        trade_protection: &mut TradeProtection,
    ) -> Result<()> {
        // Update last trade time
        let trade_record = TradeTimeRecord {
            trader,
            last_trade_time: current_time,
            trade_amount,
        };

        // Remove old record for this trader
        trade_protection.last_trade_times.retain(|record| record.trader != trader);
        trade_protection.last_trade_times.push(trade_record);

        // Update daily volume
        let today = current_time - (current_time % 86400);
        let daily_volume_record = trade_protection
            .daily_volumes
            .iter_mut()
            .find(|record| record.trader == trader && record.date == today);

        match daily_volume_record {
            Some(record) => {
                record.volume += trade_amount;
            }
            None => {
                let new_record = DailyVolumeRecord {
                    trader,
                    date: today,
                    volume: trade_amount,
                };
                trade_protection.daily_volumes.push(new_record);
            }
        }

        // Clean up old records (older than 7 days)
        let week_ago = current_time - (7 * 86400);
        trade_protection.last_trade_times.retain(|record| record.last_trade_time > week_ago);
        trade_protection.daily_volumes.retain(|record| record.date > week_ago);

        Ok(())
    }

    /// Trigger circuit breaker
    pub fn trigger_circuit_breaker(
        circuit_breaker: &mut CircuitBreaker,
        current_time: i64,
        current_price: u64,
    ) -> Result<()> {
        circuit_breaker.is_triggered = true;
        circuit_breaker.last_trigger_time = current_time;
        circuit_breaker.trigger_count += 1;
        circuit_breaker.last_price = current_price;

        Ok(())
    }

    /// Reset circuit breaker after cooldown
    pub fn reset_circuit_breaker(
        circuit_breaker: &mut CircuitBreaker,
        current_time: i64,
    ) -> Result<()> {
        if circuit_breaker.is_triggered && 
           current_time - circuit_breaker.last_trigger_time >= circuit_breaker.cooldown_period {
            circuit_breaker.is_triggered = false;
            circuit_breaker.volume_in_period = 0;
        }

        Ok(())
    }

    /// Initialize security structures with default values
    pub fn initialize_circuit_breaker(authority: Pubkey, current_time: i64) -> CircuitBreaker {
        CircuitBreaker {
            authority,
            max_price_change_percent: MAX_PRICE_CHANGE_PERCENT,
            max_volume_per_period: MAX_VOLUME_PER_PERIOD,
            cooldown_period: CIRCUIT_BREAKER_COOLDOWN,
            last_trigger_time: 0,
            is_triggered: false,
            trigger_count: 0,
            last_price: 0,
            last_volume_check: current_time,
            volume_in_period: 0,
            created_at: current_time,
            updated_at: current_time,
        }
    }

    pub fn initialize_trade_protection(authority: Pubkey, current_time: i64) -> TradeProtection {
        TradeProtection {
            authority,
            min_trade_interval: MIN_TRADE_INTERVAL,
            max_trade_amount: MAX_TRADE_AMOUNT,
            max_daily_volume: MAX_DAILY_VOLUME,
            suspicious_pattern_threshold: SUSPICIOUS_PATTERN_THRESHOLD,
            last_trade_times: Vec::new(),
            daily_volumes: Vec::new(),
            created_at: current_time,
            updated_at: current_time,
        }
    }

    pub fn initialize_multi_sig_governance(
        authority: Pubkey,
        signers: Vec<Pubkey>,
        current_time: i64,
    ) -> MultiSigGovernance {
        MultiSigGovernance {
            authority,
            required_signatures: MULTISIG_REQUIRED_SIGNATURES,
            signers,
            distribution_threshold: DISTRIBUTION_THRESHOLD,
            vesting_enabled: true,
            vesting_duration: VESTING_DURATION,
            pending_distributions: Vec::new(),
            created_at: current_time,
            updated_at: current_time,
        }
    }

    pub fn initialize_emergency_controls(authority: Pubkey, current_time: i64) -> EmergencyControls {
        EmergencyControls {
            authority,
            emergency_pause: false,
            emergency_threshold: 1_000_000_000_000, // 1,000 SOL
            pause_reason: String::new(),
            pause_initiated_by: None,
            pause_time: None,
            auto_resume_time: None,
            circuit_breaker_active: true,
            flash_loan_protection_active: true,
            created_at: current_time,
            updated_at: current_time,
        }
    }
}
