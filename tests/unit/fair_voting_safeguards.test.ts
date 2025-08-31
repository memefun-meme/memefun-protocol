import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaMemes } from "../target/types/solana_memes";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

describe("Fair Voting Safeguards", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.SolanaMemes as Program<SolanaMemes>;

    // Test accounts
    const authority = Keypair.generate();
    const user1 = Keypair.generate();
    const user2 = Keypair.generate();
    const whale = Keypair.generate();
    const creator = Keypair.generate();
    const tokenMint = Keypair.generate();

    // PDAs
    let fairVotingSafeguardsPda: PublicKey;
    let enhancedTokenHolderPda: PublicKey;
    let creatorPerformancePda: PublicKey;
    let appealSystemPda: PublicKey;
    let detectionSystemPda: PublicKey;
    let penaltySystemPda: PublicKey;

    before(async () => {
        // Airdrop SOL to test accounts
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(authority.publicKey, 10 * LAMPORTS_PER_SOL)
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(user1.publicKey, 5 * LAMPORTS_PER_SOL)
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(user2.publicKey, 5 * LAMPORTS_PER_SOL)
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(whale.publicKey, 100 * LAMPORTS_PER_SOL)
        );

        // Find PDAs
        [fairVotingSafeguardsPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("fair_voting_safeguards")],
            program.programId
        );

        [enhancedTokenHolderPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("enhanced_token_holder"), user1.publicKey.toBuffer()],
            program.programId
        );

        [creatorPerformancePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("creator_performance"), creator.publicKey.toBuffer(), tokenMint.publicKey.toBuffer()],
            program.programId
        );

        [appealSystemPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("appeal_system")],
            program.programId
        );

        [detectionSystemPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("detection_system")],
            program.programId
        );

        [penaltySystemPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("penalty_system")],
            program.programId
        );
    });

    describe("Fair Voting Safeguards Initialization", () => {
        it("Should initialize fair voting safeguards with correct parameters", async () => {
            const tx = await program.methods
                .initializeFairVotingSafeguards(
                    40, // staked_amount_weight
                    25, // staking_duration_weight
                    20, // community_contribution_weight
                    15, // token_holding_weight
                    new anchor.BN(1000000), // max_voting_power_per_wallet
                    50, // whale_voting_discount
                    5, // max_concentration_percent
                    new anchor.BN(2592000), // whale_cooldown_period (30 days)
                    new anchor.BN(2592000), // minimum_staking_duration (30 days)
                    new anchor.BN(1000000), // minimum_staked_amount
                    new anchor.BN(604800), // lock_period_during_voting (7 days)
                    5, // short_term_multiplier
                    10, // medium_term_multiplier
                    15, // long_term_multiplier
                    20, // very_long_multiplier
                    new anchor.BN(1000000) // suspicious_activity_threshold
                )
                .accounts({
                    fairVotingSafeguards: fairVotingSafeguardsPda,
                    authority: authority.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const safeguards = await program.account.fairVotingSafeguards.fetch(fairVotingSafeguardsPda);

            expect(safeguards.stakedAmountWeight).to.equal(40);
            expect(safeguards.stakingDurationWeight).to.equal(25);
            expect(safeguards.communityContributionWeight).to.equal(20);
            expect(safeguards.tokenHoldingWeight).to.equal(15);
            expect(safeguards.maxVotingPowerPerWallet.toNumber()).to.equal(1000000);
            expect(safeguards.whaleVotingDiscount).to.equal(50);
            expect(safeguards.maxConcentrationPercent).to.equal(5);
            expect(safeguards.manipulationDetectionEnabled).to.be.true;
            expect(safeguards.automatedMonitoringEnabled).to.be.true;
        });

        it("Should reject invalid voting power weights", async () => {
            try {
                await program.methods
                    .initializeFairVotingSafeguards(
                        50, // staked_amount_weight (invalid - doesn't sum to 100)
                        25, // staking_duration_weight
                        20, // community_contribution_weight
                        15, // token_holding_weight
                        new anchor.BN(1000000),
                        50,
                        5,
                        new anchor.BN(2592000),
                        new anchor.BN(2592000),
                        new anchor.BN(1000000),
                        new anchor.BN(604800),
                        5,
                        10,
                        15,
                        20,
                        new anchor.BN(1000000)
                    )
                    .accounts({
                        fairVotingSafeguards: fairVotingSafeguardsPda,
                        authority: authority.publicKey,
                        systemProgram: anchor.web3.SystemProgram.programId,
                    })
                    .signers([authority])
                    .rpc();

                expect.fail("Should have thrown an error");
            } catch (error) {
                expect(error.toString()).to.include("InvalidVotingPowerWeights");
            }
        });
    });

    describe("Enhanced Token Holder", () => {
        it("Should initialize enhanced token holder with fair voting power", async () => {
            const tx = await program.methods
                .initializeEnhancedTokenHolder(
                    new anchor.BN(1000000), // staked_amount
                    new anchor.BN(7776000), // staking_duration (90 days)
                    new anchor.BN(50000), // community_contribution
                    new anchor.BN(100000), // token_holding
                    85, // consistency_score
                    750, // reputation_score
                    50, // participation_history
                    80 // contribution_quality
                )
                .accounts({
                    enhancedTokenHolder: enhancedTokenHolderPda,
                    authority: user1.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user1])
                .rpc();

            const tokenHolder = await program.account.enhancedTokenHolder.fetch(enhancedTokenHolderPda);

            expect(tokenHolder.holder.toString()).to.equal(user1.publicKey.toString());
            expect(tokenHolder.stakedAmount.toNumber()).to.equal(1000000);
            expect(tokenHolder.stakingDuration.toNumber()).to.equal(7776000);
            expect(tokenHolder.communityContributionScore.toNumber()).to.equal(50000);
            expect(tokenHolder.reputationScore).to.equal(750);
            expect(tokenHolder.isWhale).to.be.false;
            expect(tokenHolder.votingRestricted).to.be.false;
        });

        it("Should update voting power with new parameters", async () => {
            const tx = await program.methods
                .updateVotingPower(
                    new anchor.BN(2000000), // new_staked_amount
                    new anchor.BN(15552000), // new_staking_duration (180 days)
                    new anchor.BN(100000), // new_community_contribution
                    new anchor.BN(200000), // new_token_holding
                    90, // new_consistency_score
                    800, // new_reputation_score
                    75, // new_participation_history
                    85 // new_contribution_quality
                )
                .accounts({
                    enhancedTokenHolder: enhancedTokenHolderPda,
                    fairVotingSafeguards: fairVotingSafeguardsPda,
                    authority: user1.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user1])
                .rpc();

            const tokenHolder = await program.account.enhancedTokenHolder.fetch(enhancedTokenHolderPda);

            expect(tokenHolder.stakedAmount.toNumber()).to.equal(2000000);
            expect(tokenHolder.stakingDuration.toNumber()).to.equal(15552000);
            expect(tokenHolder.reputationScore).to.equal(800);
            expect(tokenHolder.votingPower.toNumber()).to.be.greaterThan(0);
        });
    });

    describe("Creator Performance Tracking", () => {
        it("Should initialize creator performance tracking", async () => {
            const tx = await program.methods
                .initializeCreatorPerformance(
                    new anchor.BN(500), // token_price_performance (500% growth)
                    new anchor.BN(10000000), // trading_volume
                    new anchor.BN(1000), // community_growth
                    new anchor.BN(500000), // staking_participation
                    85, // community_satisfaction
                    80, // marketing_efforts
                    75, // community_engagement
                    90, // transparency_score
                    new anchor.BN(Date.now() / 1000), // assessment_start_time
                    new anchor.BN(Date.now() / 1000 + 2592000) // assessment_end_time (30 days)
                )
                .accounts({
                    creatorPerformance: creatorPerformancePda,
                    creator: creator.publicKey,
                    tokenMint: tokenMint.publicKey,
                    authority: authority.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const performance = await program.account.creatorPerformance.fetch(creatorPerformancePda);

            expect(performance.creator.toString()).to.equal(creator.publicKey.toString());
            expect(performance.tokenMint.toString()).to.equal(tokenMint.publicKey.toString());
            expect(performance.tokenPricePerformance.toNumber()).to.equal(500);
            expect(performance.tradingVolume.toNumber()).to.equal(10000000);
            expect(performance.communitySatisfaction).to.equal(85);
            expect(performance.transparencyScore).to.equal(90);
            expect(performance.performanceScore).to.be.greaterThan(0);
        });

        it("Should update creator performance and calculate release percentage", async () => {
            const tx = await program.methods
                .updateCreatorPerformance(
                    new anchor.BN(800), // token_price_performance (800% growth)
                    new anchor.BN(20000000), // trading_volume
                    new anchor.BN(2000), // community_growth
                    new anchor.BN(1000000), // staking_participation
                    90, // community_satisfaction
                    85, // marketing_efforts
                    80, // community_engagement
                    95 // transparency_score
                )
                .accounts({
                    creatorPerformance: creatorPerformancePda,
                    authority: authority.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const performance = await program.account.creatorPerformance.fetch(creatorPerformancePda);

            expect(performance.tokenPricePerformance.toNumber()).to.equal(800);
            expect(performance.tradingVolume.toNumber()).to.equal(20000000);
            expect(performance.communitySatisfaction).to.equal(90);
            expect(performance.transparencyScore).to.equal(95);
            expect(performance.performanceScore).to.be.greaterThan(0);
            expect(performance.performanceHistory.length).to.be.greaterThan(0);
        });
    });

    describe("Appeal System", () => {
        it("Should initialize appeal system", async () => {
            const tx = await program.methods
                .initializeAppealSystem(
                    15, // appeal_threshold (15%)
                    new anchor.BN(604800), // appeal_period (7 days)
                    5, // appeal_review_panel_size
                    new anchor.BN(1000000) // appeal_fee
                )
                .accounts({
                    appealSystem: appealSystemPda,
                    authority: authority.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const appealSystem = await program.account.appealSystem.fetch(appealSystemPda);

            expect(appealSystem.appealThreshold).to.equal(15);
            expect(appealSystem.appealPeriod.toNumber()).to.equal(604800);
            expect(appealSystem.appealReviewPanelSize).to.equal(5);
            expect(appealSystem.appealFee.toNumber()).to.equal(1000000);
            expect(appealSystem.governanceOversightEnabled).to.be.true;
            expect(appealSystem.externalAuditorsEnabled).to.be.true;
        });

        it("Should submit an appeal", async () => {
            const originalDecision = Keypair.generate().publicKey;

            const tx = await program.methods
                .submitAppeal(
                    originalDecision,
                    "Unfair voting decision due to whale manipulation",
                    "Evidence of coordinated voting by large holders"
                )
                .accounts({
                    appeal: PublicKey.findProgramAddressSync(
                        [Buffer.from("appeal"), new anchor.BN(0).toArrayLike(Buffer, "le", 8)],
                        program.programId
                    )[0],
                    appealSystem: appealSystemPda,
                    appellant: user1.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user1])
                .rpc();

            const appealSystem = await program.account.appealSystem.fetch(appealSystemPda);
            expect(appealSystem.totalAppeals.toNumber()).to.equal(1);
        });
    });

    describe("Detection System", () => {
        it("Should initialize detection system", async () => {
            const tx = await program.methods
                .initializeDetectionSystem(
                    new anchor.BN(10000000), // whale_threshold
                    new anchor.BN(5000000), // whale_coordination_threshold
                    new anchor.BN(1000000) // suspicious_pattern_threshold
                )
                .accounts({
                    detectionSystem: detectionSystemPda,
                    authority: authority.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const detectionSystem = await program.account.detectionSystem.fetch(detectionSystemPda);

            expect(detectionSystem.whaleThreshold.toNumber()).to.equal(10000000);
            expect(detectionSystem.whaleCoordinationThreshold.toNumber()).to.equal(5000000);
            expect(detectionSystem.suspiciousPatternThreshold.toNumber()).to.equal(1000000);
            expect(detectionSystem.whaleDetectionEnabled).to.be.true;
            expect(detectionSystem.suspiciousActivityEnabled).to.be.true;
            expect(detectionSystem.collusionDetectionEnabled).to.be.true;
        });

        it("Should create suspicious activity alert", async () => {
            const alertType = { whaleManipulation: {} };
            const targetWallet = whale.publicKey;

            const tx = await program.methods
                .createSuspiciousActivityAlert(
                    alertType,
                    targetWallet,
                    "Large coordinated voting detected",
                    "Multiple wallets voting in same pattern within short time",
                    85, // evidence_strength
                    2 // historical_violations
                )
                .accounts({
                    alert: PublicKey.findProgramAddressSync(
                        [Buffer.from("suspicious_alert"), new anchor.BN(0).toArrayLike(Buffer, "le", 8)],
                        program.programId
                    )[0],
                    detectionSystem: detectionSystemPda,
                    authority: authority.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const detectionSystem = await program.account.detectionSystem.fetch(detectionSystemPda);
            expect(detectionSystem.totalAlerts.toNumber()).to.equal(1);
        });
    });

    describe("Penalty System", () => {
        it("Should initialize penalty system", async () => {
            const tx = await program.methods
                .initializePenaltySystem(
                    new anchor.BN(100000), // warning_penalty
                    new anchor.BN(500000), // restriction_penalty
                    new anchor.BN(1000000), // voting_ban_penalty
                    new anchor.BN(2000000) // token_confiscation_penalty
                )
                .accounts({
                    penaltySystem: penaltySystemPda,
                    authority: authority.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const penaltySystem = await program.account.penaltySystem.fetch(penaltySystemPda);

            expect(penaltySystem.warningPenalty.toNumber()).to.equal(100000);
            expect(penaltySystem.restrictionPenalty.toNumber()).to.equal(500000);
            expect(penaltySystem.votingBanPenalty.toNumber()).to.equal(1000000);
            expect(penaltySystem.tokenConfiscationPenalty.toNumber()).to.equal(2000000);
        });

        it("Should issue penalty for manipulation", async () => {
            const penaltyType = { votingRestriction: {} };
            const offender = whale.publicKey;

            const tx = await program.methods
                .issuePenalty(
                    offender,
                    penaltyType,
                    "Coordinated voting manipulation detected",
                    "Evidence of vote buying and coordination",
                    75 // risk_score
                )
                .accounts({
                    penalty: PublicKey.findProgramAddressSync(
                        [Buffer.from("penalty"), new anchor.BN(0).toArrayLike(Buffer, "le", 8)],
                        program.programId
                    )[0],
                    penaltySystem: penaltySystemPda,
                    authority: authority.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const penaltySystem = await program.account.penaltySystem.fetch(penaltySystemPda);
            expect(penaltySystem.totalPenaltiesIssued.toNumber()).to.equal(1);
            expect(penaltySystem.activePenalties.toNumber()).to.equal(1);
        });
    });

    describe("Anti-Whale Protection", () => {
        it("Should detect whale wallet and apply restrictions", async () => {
            // Create a whale token holder with large stake
            const whaleTokenHolderPda = PublicKey.findProgramAddressSync(
                [Buffer.from("enhanced_token_holder"), whale.publicKey.toBuffer()],
                program.programId
            )[0];

            await program.methods
                .initializeEnhancedTokenHolder(
                    new anchor.BN(50000000), // Large staked amount
                    new anchor.BN(7776000), // staking_duration
                    new anchor.BN(100000), // community_contribution
                    new anchor.BN(1000000), // token_holding
                    70, // consistency_score
                    600, // reputation_score
                    30, // participation_history
                    75 // contribution_quality
                )
                .accounts({
                    enhancedTokenHolder: whaleTokenHolderPda,
                    authority: whale.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([whale])
                .rpc();

            const whaleHolder = await program.account.enhancedTokenHolder.fetch(whaleTokenHolderPda);

            // Whale should be detected and voting power should be limited
            expect(whaleHolder.stakedAmount.toNumber()).to.equal(50000000);
            expect(whaleHolder.votingPower.toNumber()).to.be.lessThan(whaleHolder.stakedAmount.toNumber());
        });
    });

    describe("Fair Voting Power Calculation", () => {
        it("Should calculate fair voting power with multiple factors", async () => {
            // Create a well-rounded token holder
            const balancedHolderPda = PublicKey.findProgramAddressSync(
                [Buffer.from("enhanced_token_holder"), user2.publicKey.toBuffer()],
                program.programId
            )[0];

            await program.methods
                .initializeEnhancedTokenHolder(
                    new anchor.BN(5000000), // Moderate staked amount
                    new anchor.BN(15552000), // Long staking duration (180 days)
                    new anchor.BN(200000), // High community contribution
                    new anchor.BN(500000), // Good token holding
                    95, // High consistency score
                    900, // High reputation score
                    100, // High participation history
                    90 // High contribution quality
                )
                .accounts({
                    enhancedTokenHolder: balancedHolderPda,
                    authority: user2.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user2])
                .rpc();

            const balancedHolder = await program.account.enhancedTokenHolder.fetch(balancedHolderPda);

            // Should have good voting power due to balanced factors
            expect(balancedHolder.votingPower.toNumber()).to.be.greaterThan(0);
            expect(balancedHolder.reputationScore).to.equal(900);
            expect(balancedHolder.consistencyScore).to.equal(95);
            expect(balancedHolder.isWhale).to.be.false;
        });
    });
});
