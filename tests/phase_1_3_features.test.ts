import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaMemes } from "../target/types/solana_memes";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

describe("Phase 1-3 Features", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.SolanaMemes as Program<SolanaMemes>;

    // Test accounts
    const user = Keypair.generate();
    const creator = Keypair.generate();
    const signer1 = Keypair.generate();
    const signer2 = Keypair.generate();
    const signer3 = Keypair.generate();

    // PDAs
    let userDashboardPda: PublicKey;
    let multiSigCreatorPda: PublicKey;
    let socialFeaturesPda: PublicKey;
    let gamificationSystemPda: PublicKey;
    let advancedStakingPda: PublicKey;

    before(async () => {
        // Airdrop SOL to test accounts
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(user.publicKey, 10 * LAMPORTS_PER_SOL)
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(creator.publicKey, 10 * LAMPORTS_PER_SOL)
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(signer1.publicKey, 5 * LAMPORTS_PER_SOL)
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(signer2.publicKey, 5 * LAMPORTS_PER_SOL)
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(signer3.publicKey, 5 * LAMPORTS_PER_SOL)
        );

        // Find PDAs
        [userDashboardPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("user_dashboard"), user.publicKey.toBuffer()],
            program.programId
        );

        [multiSigCreatorPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("multi_sig_creator"), creator.publicKey.toBuffer()],
            program.programId
        );

        [socialFeaturesPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("social_features"), user.publicKey.toBuffer()],
            program.programId
        );

        [gamificationSystemPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("gamification_system"), user.publicKey.toBuffer()],
            program.programId
        );

        [advancedStakingPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("advanced_staking"), user.publicKey.toBuffer()],
            program.programId
        );
    });

    describe("User Dashboard", () => {
        it("Should initialize user dashboard with default values", async () => {
            const tx = await program.methods
                .initializeUserDashboard()
                .accounts({
                    userDashboard: userDashboardPda,
                    user: user.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user])
                .rpc();

            const dashboard = await program.account.userDashboard.fetch(userDashboardPda);

            expect(dashboard.user.toString()).to.equal(user.publicKey.toString());
            expect(dashboard.totalTokensCreated).to.equal(0);
            expect(dashboard.totalRewardsEarned).to.equal(0);
            expect(dashboard.communityRank).to.equal("Newcomer");
            expect(dashboard.reputationScore).to.equal(0);
            expect(dashboard.dashboardPreferences.theme).to.deep.equal({ auto: {} });
            expect(dashboard.notificationSettings.pushNotifications).to.be.true;
        });
    });

    describe("Multi-Signature Creator", () => {
        it("Should initialize multi-sig creator with valid configuration", async () => {
            const signers = [signer1.publicKey, signer2.publicKey, signer3.publicKey];

            const tx = await program.methods
                .initializeMultiSigCreator(
                    2, // required_signatures
                    signers,
                    new anchor.BN(1000000), // creator_threshold
                    new anchor.BN(86400), // time_lock_duration (1 day)
                    new anchor.BN(10000000) // max_transaction_amount
                )
                .accounts({
                    multiSigCreator: multiSigCreatorPda,
                    creator: creator.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([creator])
                .rpc();

            const multiSig = await program.account.multiSigCreator.fetch(multiSigCreatorPda);

            expect(multiSig.creator.toString()).to.equal(creator.publicKey.toString());
            expect(multiSig.requiredSignatures).to.equal(2);
            expect(multiSig.totalSigners).to.equal(3);
            expect(multiSig.signers.length).to.equal(3);
            expect(multiSig.creatorThreshold.toNumber()).to.equal(1000000);
            expect(multiSig.isActive).to.be.true;
        });

        it("Should reject invalid multi-sig configuration", async () => {
            const signers = [signer1.publicKey, signer2.publicKey];

            try {
                await program.methods
                    .initializeMultiSigCreator(
                        3, // required_signatures > signers.length
                        signers,
                        new anchor.BN(1000000),
                        new anchor.BN(86400),
                        new anchor.BN(10000000)
                    )
                    .accounts({
                        multiSigCreator: multiSigCreatorPda,
                        creator: creator.publicKey,
                        systemProgram: anchor.web3.SystemProgram.programId,
                    })
                    .signers([creator])
                    .rpc();

                expect.fail("Should have thrown an error");
            } catch (error) {
                expect(error.toString()).to.include("InvalidMultiSigConfiguration");
            }
        });
    });

    describe("Social Features", () => {
        it("Should initialize social features with valid profile", async () => {
            const tx = await program.methods
                .initializeSocialFeatures(
                    "testuser", // username
                    "Test User", // display_name
                    "A test user profile", // bio
                    "https://example.com/avatar.jpg", // avatar_uri
                    "https://example.com/banner.jpg" // banner_uri
                )
                .accounts({
                    socialFeatures: socialFeaturesPda,
                    user: user.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user])
                .rpc();

            const social = await program.account.socialFeatures.fetch(socialFeaturesPda);

            expect(social.user.toString()).to.equal(user.publicKey.toString());
            expect(social.username).to.equal("testuser");
            expect(social.displayName).to.equal("Test User");
            expect(social.bio).to.equal("A test user profile");
            expect(social.avatarUri).to.equal("https://example.com/avatar.jpg");
            expect(social.bannerUri).to.equal("https://example.com/banner.jpg");
            expect(social.followersCount).to.equal(0);
            expect(social.followingCount).to.equal(0);
            expect(social.communityChatEnabled).to.be.true;
            expect(social.tokenShowcaseEnabled).to.be.true;
            expect(social.influencerVerification).to.be.false;
            expect(social.verificationBadge).to.be.false;
        });

        it("Should reject invalid username", async () => {
            try {
                await program.methods
                    .initializeSocialFeatures(
                        "ab", // username too short
                        "Test User",
                        "A test user profile",
                        "https://example.com/avatar.jpg",
                        "https://example.com/banner.jpg"
                    )
                    .accounts({
                        socialFeatures: socialFeaturesPda,
                        user: user.publicKey,
                        systemProgram: anchor.web3.SystemProgram.programId,
                    })
                    .signers([user])
                    .rpc();

                expect.fail("Should have thrown an error");
            } catch (error) {
                expect(error.toString()).to.include("InvalidUsername");
            }
        });
    });

    describe("Gamification System", () => {
        it("Should initialize gamification system with default values", async () => {
            const tx = await program.methods
                .initializeGamificationSystem()
                .accounts({
                    gamificationSystem: gamificationSystemPda,
                    user: user.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user])
                .rpc();

            const gamification = await program.account.gamificationSystem.fetch(gamificationSystemPda);

            expect(gamification.user.toString()).to.equal(user.publicKey.toString());
            expect(gamification.userLevel).to.equal(1);
            expect(gamification.experiencePoints).to.equal(0);
            expect(gamification.experienceToNextLevel).to.equal(100);
            expect(gamification.totalExperienceEarned).to.equal(0);
            expect(gamification.totalAchievements).to.equal(0);
            expect(gamification.achievementScore).to.equal(0);
            expect(gamification.leaderboardRank).to.equal(0);
            expect(gamification.leaderboardScore).to.equal(0);
            expect(gamification.rewardsMultiplier).to.equal(1.0);
            expect(gamification.totalRewardsEarned).to.equal(0);
            expect(gamification.pendingRewards).to.equal(0);
            expect(gamification.currentStreak).to.equal(0);
            expect(gamification.longestStreak).to.equal(0);
        });
    });

    describe("Advanced Staking", () => {
        it("Should initialize advanced staking with default values", async () => {
            const tx = await program.methods
                .initializeAdvancedStaking()
                .accounts({
                    advancedStaking: advancedStakingPda,
                    user: user.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .signers([user])
                .rpc();

            const staking = await program.account.advancedStaking.fetch(advancedStakingPda);

            expect(staking.user.toString()).to.equal(user.publicKey.toString());
            expect(staking.totalFlexibleStaked).to.equal(0);
            expect(staking.flexibleRewards).to.equal(0);
            expect(staking.totalLockedStaked).to.equal(0);
            expect(staking.lockPeriodRewards).to.equal(0);
            expect(staking.totalYieldFarmed).to.equal(0);
            expect(staking.currentYieldRate).to.equal(0.0);
            expect(staking.totalPoolStaked).to.equal(0);
            expect(staking.poolRewards).to.equal(0);
            expect(staking.autoCompoundingEnabled).to.be.false;
            expect(staking.totalCompounded).to.equal(0);
            expect(staking.totalStakingRewards).to.equal(0);
            expect(staking.averageApy).to.equal(0.0);
        });
    });

    describe("Integration Tests", () => {
        it("Should have all Phase 1-3 features working together", async () => {
            // Verify all accounts exist
            const dashboard = await program.account.userDashboard.fetch(userDashboardPda);
            const multiSig = await program.account.multiSigCreator.fetch(multiSigCreatorPda);
            const social = await program.account.socialFeatures.fetch(socialFeaturesPda);
            const gamification = await program.account.gamificationSystem.fetch(gamificationSystemPda);
            const staking = await program.account.advancedStaking.fetch(advancedStakingPda);

            // All should be initialized for the same user
            expect(dashboard.user.toString()).to.equal(user.publicKey.toString());
            expect(social.user.toString()).to.equal(user.publicKey.toString());
            expect(gamification.user.toString()).to.equal(user.publicKey.toString());
            expect(staking.user.toString()).to.equal(user.publicKey.toString());

            // Multi-sig should be for the creator
            expect(multiSig.creator.toString()).to.equal(creator.publicKey.toString());

            // Verify default states
            expect(dashboard.communityRank).to.equal("Newcomer");
            expect(gamification.userLevel).to.equal(1);
            expect(staking.autoCompoundingEnabled).to.be.false;
            expect(social.influencerVerification).to.be.false;
        });
    });
});

