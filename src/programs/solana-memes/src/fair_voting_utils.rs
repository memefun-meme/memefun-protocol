use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::CustomError;

/// Fair voting utilities for implementing comprehensive safeguards
pub struct FairVotingUtils;

impl FairVotingUtils {
    
    /// Calculate multi-factor voting power with anti-whale protections
    pub fn calculate_fair_voting_power(
        staked_amount: u64,
        staking_duration: i64,
        community_contribution: u64,
        token_holding: u64,
        consistency_score: u64,
        reputation_score: i32,
        participation_history: u64,
        contribution_quality: u64,
        safeguards: &FairVotingSafeguards,
    ) -> Result<u64> {
        // Validate input parameters
        require!(staked_amount > 0, CustomError::STAKED_AMOUNT_INSUFFICIENT);
        require!(staking_duration >= 0, CustomError::STAKING_DURATION_INSUFFICIENT);
        require!(reputation_score >= -100 && reputation_score <= 1000, CustomError::INVALID_REPUTATION_SCORE);
        
        // Calculate base voting power components
        let staked_amount_power = Self::calculate_staked_amount_power(staked_amount, safeguards)?;
        let duration_power = Self::calculate_duration_power(staking_duration, safeguards)?;
        let contribution_power = Self::calculate_contribution_power(community_contribution, safeguards)?;
        let holding_power = Self::calculate_holding_power(token_holding, safeguards)?;
        
        // Calculate anti-manipulation factors
        let consistency_power = Self::calculate_consistency_power(consistency_score)?;
        let reputation_power = Self::calculate_reputation_power(reputation_score)?;
        let participation_power = Self::calculate_participation_power(participation_history)?;
        let quality_power = Self::calculate_quality_power(contribution_quality)?;
        
        // Apply weighted voting power calculation
        let weighted_power = (
            staked_amount_power * safeguards.staked_amount_weight as u64 +
            duration_power * safeguards.staking_duration_weight as u64 +
            contribution_power * safeguards.community_contribution_weight as u64 +
            holding_power * safeguards.token_holding_weight as u64
        ) / 100;
        
        // Apply anti-manipulation bonuses
        let anti_manipulation_bonus = (
            consistency_power + reputation_power + participation_power + quality_power
        ) / 4;
        
        let total_power = weighted_power + anti_manipulation_bonus;
        
        // Apply anti-whale protections
        let final_power = Self::apply_anti_whale_protections(total_power, staked_amount, safeguards)?;
        
        Ok(final_power)
    }
    
    /// Calculate staked amount voting power component
    fn calculate_staked_amount_power(staked_amount: u64, safeguards: &FairVotingSafeguards) -> Result<u64> {
        require!(staked_amount >= safeguards.minimum_staked_amount, CustomError::STAKED_AMOUNT_INSUFFICIENT);
        
        // Apply logarithmic scaling to prevent whale domination
        let log_amount = (staked_amount as f64).ln();
        let scaled_power = (log_amount * 1_000_000.0) as u64;
        
        Ok(scaled_power)
    }
    
    /// Calculate duration-based voting power component
    fn calculate_duration_power(staking_duration: i64, safeguards: &FairVotingSafeguards) -> Result<u64> {
        require!(staking_duration >= safeguards.minimum_staking_duration, CustomError::STAKING_DURATION_INSUFFICIENT);
        
        let duration_months = staking_duration / (30 * 24 * 60 * 60); // Convert to months
        
        let multiplier = match duration_months {
            0..=2 => safeguards.short_term_multiplier,
            3..=5 => safeguards.medium_term_multiplier,
            6..=11 => safeguards.long_term_multiplier,
            _ => safeguards.very_long_multiplier,
        };
        
        let base_power = 1_000_000; // Base power for duration
        let duration_power = base_power * multiplier as u64;
        
        Ok(duration_power)
    }
    
    /// Calculate community contribution voting power component
    fn calculate_contribution_power(community_contribution: u64, _safeguards: &FairVotingSafeguards) -> Result<u64> {
        require!(community_contribution > 0, CustomError::COMMUNITY_CONTRIBUTION_INSUFFICIENT);
        
        // Contribution power scales with contribution amount
        let contribution_power = community_contribution * 2; // 2x multiplier for contributions
        
        Ok(contribution_power)
    }
    
    /// Calculate token holding voting power component
    fn calculate_holding_power(token_holding: u64, _safeguards: &FairVotingSafeguards) -> Result<u64> {
        require!(token_holding > 0, CustomError::TOKEN_HOLDING_INSUFFICIENT);
        
        // Holding power with diminishing returns
        let holding_power = (token_holding as f64).sqrt() as u64 * 1_000;
        
        Ok(holding_power)
    }
    
    /// Calculate consistency score voting power component
    fn calculate_consistency_power(consistency_score: u64) -> Result<u64> {
        require!(consistency_score > 0, CustomError::CONSISTENCY_SCORE_INSUFFICIENT);
        
        // Consistency power with bonus for high consistency
        let consistency_power = if consistency_score >= 80 {
            consistency_score * 3 // 3x bonus for high consistency
        } else {
            consistency_score * 2 // 2x for moderate consistency
        };
        
        Ok(consistency_power)
    }
    
    /// Calculate reputation score voting power component
    fn calculate_reputation_power(reputation_score: i32) -> Result<u64> {
        require!(reputation_score >= -100 && reputation_score <= 1000, CustomError::INVALID_REPUTATION_SCORE);
        
        // Convert negative reputation to positive scale
        let adjusted_score = if reputation_score < 0 {
            0
        } else {
            reputation_score as u64
        };
        
        // Reputation power with bonus for high reputation
        let reputation_power = if adjusted_score >= 800 {
            adjusted_score * 4 // 4x bonus for high reputation
        } else if adjusted_score >= 500 {
            adjusted_score * 3 // 3x bonus for good reputation
        } else {
            adjusted_score * 2 // 2x for moderate reputation
        };
        
        Ok(reputation_power)
    }
    
    /// Calculate participation history voting power component
    fn calculate_participation_power(participation_history: u64) -> Result<u64> {
        require!(participation_history > 0, CustomError::PARTICIPATION_HISTORY_INSUFFICIENT);
        
        // Participation power with bonus for long-term participation
        let participation_power = if participation_history >= 100 {
            participation_history * 3 // 3x bonus for high participation
        } else {
            participation_history * 2 // 2x for moderate participation
        };
        
        Ok(participation_power)
    }
    
    /// Calculate contribution quality voting power component
    fn calculate_quality_power(contribution_quality: u64) -> Result<u64> {
        require!(contribution_quality > 0, CustomError::CONTRIBUTION_QUALITY_INSUFFICIENT);
        
        // Quality power with bonus for high quality contributions
        let quality_power = if contribution_quality >= 80 {
            contribution_quality * 3 // 3x bonus for high quality
        } else {
            contribution_quality * 2 // 2x for moderate quality
        };
        
        Ok(quality_power)
    }
    
    /// Apply anti-whale protections to voting power
    fn apply_anti_whale_protections(
        voting_power: u64,
        staked_amount: u64,
        safeguards: &FairVotingSafeguards,
    ) -> Result<u64> {
        // Check if wallet exceeds maximum voting power
        if voting_power > safeguards.max_voting_power_per_wallet {
            return Ok(safeguards.max_voting_power_per_wallet);
        }
        
        // Apply whale discount if applicable
        let whale_threshold = safeguards.max_voting_power_per_wallet / 2; // 50% of max
        if voting_power > whale_threshold {
            let discount = safeguards.whale_voting_discount;
            let discounted_power = voting_power * (100 - discount) as u64 / 100;
            return Ok(discounted_power);
        }
        
        Ok(voting_power)
    }
    
    /// Check if wallet is flagged as whale
    pub fn is_whale_wallet(
        staked_amount: u64,
        total_supply: u64,
        safeguards: &FairVotingSafeguards,
    ) -> Result<bool> {
        let concentration_percentage = (staked_amount * 100) / total_supply;
        let is_whale = concentration_percentage > safeguards.max_concentration_percent as u64;
        
        Ok(is_whale)
    }
    
    /// Check if wallet is under cooldown period
    pub fn is_under_cooldown(
        last_vote_time: i64,
        current_time: i64,
        safeguards: &FairVotingSafeguards,
    ) -> Result<bool> {
        let time_since_last_vote = current_time - last_vote_time;
        let under_cooldown = time_since_last_vote < safeguards.whale_cooldown_period;
        
        Ok(under_cooldown)
    }
    
    /// Calculate creator performance score
    pub fn calculate_performance_score(performance: &CreatorPerformance) -> Result<u64> {
        // Validate performance metrics
        require!(performance.token_price_performance > 0, CustomError::TOKEN_PRICE_PERFORMANCE_INVALID);
        require!(performance.trading_volume > 0, CustomError::TRADING_VOLUME_INVALID);
        require!(performance.community_growth > 0, CustomError::COMMUNITY_GROWTH_INVALID);
        require!(performance.staking_participation > 0, CustomError::STAKING_PARTICIPATION_INVALID);
        require!(performance.community_satisfaction <= 100, CustomError::COMMUNITY_SATISFACTION_INVALID);
        require!(performance.marketing_efforts <= 100, CustomError::MARKETING_EFFORTS_INVALID);
        require!(performance.community_engagement <= 100, CustomError::COMMUNITY_ENGAGEMENT_INVALID);
        require!(performance.transparency_score <= 100, CustomError::TRANSPARENCY_SCORE_INVALID);
        
        // Calculate quantitative score (70% weight)
        let quantitative_score = (
            performance.token_price_performance * 25 +
            performance.trading_volume * 20 +
            performance.community_growth * 15 +
            performance.staking_participation * 10
        ) / 70;
        
        // Calculate qualitative score (30% weight)
        let qualitative_score = (
            performance.community_satisfaction * 10 +
            performance.marketing_efforts * 8 +
            performance.community_engagement * 7 +
            performance.transparency_score * 5
        ) / 30;
        
        // Calculate overall performance score
        let overall_score = (quantitative_score * 70 + qualitative_score * 30) / 100;
        
        // Ensure score is within valid range (0-100)
        let final_score = std::cmp::min(overall_score, 100);
        
        Ok(final_score)
    }
    
    /// Determine creator release percentage based on performance
    pub fn determine_creator_release_percentage(performance_score: u64) -> Result<u8> {
        require!(performance_score <= 100, CustomError::INVALID_PERFORMANCE_SCORE);
        
        let release_percentage = match performance_score {
            90..=100 => 100, // Full release for excellent performance
            80..=89 => 85,   // 85% release for very good performance
            70..=79 => 70,   // 70% release for good performance
            60..=69 => 55,   // 55% release for moderate performance
            50..=59 => 40,   // 40% release for below average performance
            40..=49 => 25,   // 25% release for poor performance
            30..=39 => 15,   // 15% release for very poor performance
            _ => 10,         // 10% release for failed performance
        };
        
        Ok(release_percentage)
    }
    
    /// Check if appeal threshold is met
    pub fn is_appeal_threshold_met(
        votes_against: u64,
        total_votes: u64,
        appeal_threshold: u8,
    ) -> Result<bool> {
        require!(total_votes > 0, CustomError::APPEAL_THRESHOLD_NOT_MET);
        
        let against_percentage = (votes_against * 100) / total_votes;
        let threshold_met = against_percentage >= appeal_threshold as u64;
        
        Ok(threshold_met)
    }
    
    /// Check if appeal period is still active
    pub fn is_appeal_period_active(
        decision_time: i64,
        current_time: i64,
        appeal_period: i64,
    ) -> Result<bool> {
        let time_since_decision = current_time - decision_time;
        let period_active = time_since_decision <= appeal_period;
        
        Ok(period_active)
    }
    
    /// Calculate risk score for suspicious activity
    pub fn calculate_risk_score(
        alert_type: &AlertType,
        evidence_strength: u8,
        historical_violations: u64,
    ) -> Result<u8> {
        require!(evidence_strength <= 100, CustomError::RISK_SCORE_INVALID);
        
        let base_risk = match alert_type {
            AlertType::WhaleManipulation => 70,
            AlertType::VoteCoordination => 80,
            AlertType::SuspiciousPattern => 60,
            AlertType::Collusion => 90,
            AlertType::Bribery => 95,
            AlertType::VoteSelling => 85,
            AlertType::SybilAttack => 75,
            AlertType::FlashLoanAttack => 90,
        };
        
        // Adjust risk based on evidence strength
        let evidence_adjustment = (evidence_strength as i32 - 50) * 2; // -100 to +100
        
        // Adjust risk based on historical violations
        let violation_adjustment = (historical_violations as i32 * 5).min(50); // +5 per violation, max +50
        
        let adjusted_risk = (base_risk as i32 + evidence_adjustment + violation_adjustment)
            .max(0)
            .min(100) as u8;
        
        Ok(adjusted_risk)
    }
    
    /// Determine penalty type based on risk score
    pub fn determine_penalty_type(risk_score: u8) -> Result<PenaltyType> {
        require!(risk_score <= 100, CustomError::RISK_SCORE_INVALID);
        
        let penalty_type = match risk_score {
            0..=30 => PenaltyType::Warning,
            31..=50 => PenaltyType::VotingRestriction,
            51..=70 => PenaltyType::VotingBan,
            71..=85 => PenaltyType::TokenConfiscation,
            86..=95 => PenaltyType::TemporaryBan,
            _ => PenaltyType::PermanentBan,
        };
        
        Ok(penalty_type)
    }
    
    /// Validate fair voting safeguards configuration
    pub fn validate_safeguards_config(safeguards: &FairVotingSafeguards) -> Result<()> {
        // Validate voting power weights sum to 100
        let total_weight = safeguards.staked_amount_weight as u16 +
                          safeguards.staking_duration_weight as u16 +
                          safeguards.community_contribution_weight as u16 +
                          safeguards.token_holding_weight as u16;
        require!(total_weight == 100, CustomError::INVALID_VOTING_POWER_WEIGHTS);
        
        // Validate duration multipliers
        require!(safeguards.short_term_multiplier > 0, CustomError::INVALID_DURATION_MULTIPLIERS);
        require!(safeguards.medium_term_multiplier > 0, CustomError::INVALID_DURATION_MULTIPLIERS);
        require!(safeguards.long_term_multiplier > 0, CustomError::INVALID_DURATION_MULTIPLIERS);
        require!(safeguards.very_long_multiplier > 0, CustomError::INVALID_DURATION_MULTIPLIERS);
        
        // Validate whale thresholds
        require!(safeguards.max_voting_power_per_wallet > 0, CustomError::INVALID_WHALE_THRESHOLDS);
        require!(safeguards.whale_voting_discount <= 100, CustomError::INVALID_WHALE_THRESHOLDS);
        require!(safeguards.max_concentration_percent <= 100, CustomError::INVALID_WHALE_THRESHOLDS);
        
        // Validate time-based requirements
        require!(safeguards.minimum_staking_duration > 0, CustomError::MINIMUM_STAKING_DURATION_INVALID);
        require!(safeguards.minimum_staked_amount > 0, CustomError::MINIMUM_STAKE_AMOUNT_INVALID);
        require!(safeguards.lock_period_during_voting >= 0, CustomError::LOCK_PERIOD_INVALID);
        require!(safeguards.whale_cooldown_period >= 0, CustomError::COOLDOWN_PERIOD_INVALID);
        
        // Validate thresholds
        require!(safeguards.suspicious_activity_threshold > 0, CustomError::SUSPICIOUS_ACTIVITY_THRESHOLD_INVALID);
        
        Ok(())
    }
}
