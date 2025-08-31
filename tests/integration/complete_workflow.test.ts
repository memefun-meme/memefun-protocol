import { Program, AnchorProvider, web3, BN } from '@project-serum/anchor';
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo } from '@solana/spl-token';
import { expect } from 'chai';

describe('Complete Token Creation and LBM Workflow', () => {
    const provider = AnchorProvider.env();
    const program = new Program(require('../target/idl/solana_memes.json'), require('../target/deploy/solana_memes-keypair.json').publicKey, provider);

    let creator: Keypair;
    let participant1: Keypair;
    let participant2: Keypair;
    let tokenMint: PublicKey;
    let creatorTokenAccount: PublicKey;
    let participant1TokenAccount: PublicKey;
    let participant2TokenAccount: PublicKey;
    let creatorProfile: PublicKey;
    let tokenMetadata: PublicKey;
    let vesting: PublicKey;
    let lbmPool: PublicKey;
    let treasury: PublicKey;
    let stakingRewards: PublicKey;

    before(async () => {
        // Setup test accounts
        creator = Keypair.generate();
        participant1 = Keypair.generate();
        participant2 = Keypair.generate();

        // Airdrop SOL to test accounts
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(creator.publicKey, 20_000_000_000)
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(participant1.publicKey, 10_000_000_000)
        );
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(participant2.publicKey, 10_000_000_000)
        );

        // Create token mint
        tokenMint = await createMint(
            provider.connection,
            creator,
            creator.publicKey,
            null,
            9
        );

        // Create token accounts
        creatorTokenAccount = await createAccount(
            provider.connection,
            creator,
            tokenMint,
            creator.publicKey
        );

        participant1TokenAccount = await createAccount(
            provider.connection,
            participant1,
            tokenMint,
            participant1.publicKey
        );

        participant2TokenAccount = await createAccount(
            provider.connection,
            participant2,
            tokenMint,
            participant2.publicKey
        );

        // Mint tokens to creator
        await mintTo(
            provider.connection,
            creator,
            tokenMint,
            creatorTokenAccount,
            creator,
            1_000_000_000_000 // 1 billion tokens
        );

        // Derive PDAs
        [creatorProfile] = PublicKey.findProgramAddressSync(
            [Buffer.from('creator_profile'), creator.publicKey.toBuffer()],
            program.programId
        );

        [tokenMetadata] = PublicKey.findProgramAddressSync(
            [Buffer.from('token_metadata'), tokenMint.toBuffer()],
            program.programId
        );

        [vesting] = PublicKey.findProgramAddressSync(
            [Buffer.from('vesting'), tokenMint.toBuffer(), creator.publicKey.toBuffer()],
            program.programId
        );

        [lbmPool] = PublicKey.findProgramAddressSync(
            [Buffer.from('lbm_pool'), tokenMint.toBuffer()],
            program.programId
        );

        [treasury] = PublicKey.findProgramAddressSync(
            [Buffer.from('treasury')],
            program.programId
        );

        [stakingRewards] = PublicKey.findProgramAddressSync(
            [Buffer.from('staking_rewards')],
            program.programId
        );
    });

    describe('Complete Workflow', () => {
        it('should execute complete token creation and LBM workflow', async () => {
            // Step 1: Register creator
            console.log('Step 1: Registering creator...');
            await program.methods
                .registerCreator(new BN(30_000_000)) // 0.03 SOL stake
                .accounts({
                    creator: creator.publicKey,
                    creatorProfile,
                    treasury,
                    systemProgram: SystemProgram.programId,
                })
                .signers([creator])
                .rpc();

            // Verify creator registration
            const creatorAccount = await program.account.creatorProfile.fetch(creatorProfile);
            expect(creatorAccount.isRegistered).to.be.true;
            expect(creatorAccount.owner.toString()).to.equal(creator.publicKey.toString());

            // Step 2: Create token
            console.log('Step 2: Creating token...');
            const supply = new BN(1_000_000_000_000); // 1 billion tokens
            const creatorPercent = 7; // 7% to creator
            const vestingSeconds = new BN(2592000); // 30 days

            await program.methods
                .createToken(9, supply, creatorPercent, vestingSeconds)
                .accounts({
                    creatorProfile,
                    creator: creator.publicKey,
                    tokenMint,
                    creatorTokenAccount,
                    vesting,
                    tokenMetadata,
                    treasury,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                    rent: SYSVAR_RENT_PUBKEY,
                })
                .signers([creator])
                .rpc();

            // Verify token creation
            const metadataAccount = await program.account.tokenMetadata.fetch(tokenMetadata);
            expect(metadataAccount.tokenMint.toString()).to.equal(tokenMint.toString());
            expect(metadataAccount.creator.toString()).to.equal(creator.publicKey.toString());

            // Step 3: Create LBM pool
            console.log('Step 3: Creating LBM pool...');
            const targetLiquidity = new BN(10_000_000_000); // 10 SOL
            const bootstrapDuration = new BN(86400); // 24 hours
            const priceDiscoveryPeriod = new BN(3600); // 1 hour
            const maxParticipationPerWallet = new BN(1_000_000_000); // 1 SOL
            const minParticipation = new BN(100_000_000); // 0.1 SOL
            const maxParticipation = new BN(10_000_000_000); // 10 SOL

            await program.methods
                .createLbmPool(
                    targetLiquidity,
                    bootstrapDuration,
                    priceDiscoveryPeriod,
                    maxParticipationPerWallet,
                    minParticipation,
                    maxParticipation,
                    true // anti_bot_enabled
                )
                .accounts({
                    creator: creator.publicKey,
                    creatorProfile,
                    lbmPool,
                    tokenMint,
                    tokenMetadata,
                    creatorTokenAccount,
                    treasury,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                    rent: SYSVAR_RENT_PUBKEY,
                })
                .signers([creator])
                .rpc();

            // Verify LBM pool creation
            const poolAccount = await program.account.liquidityBootstrappingPool.fetch(lbmPool);
            expect(poolAccount.isActive).to.be.true;
            expect(poolAccount.tokenMint.toString()).to.equal(tokenMint.toString());

            // Step 4: Participants join LBM
            console.log('Step 4: Participants joining LBM...');
            const participation1 = new BN(500_000_000); // 0.5 SOL
            const participation2 = new BN(300_000_000); // 0.3 SOL

            await program.methods
                .participateLbm(participation1)
                .accounts({
                    participant: participant1.publicKey,
                    lbmPool,
                    tokenMint,
                    participantTokenAccount: participant1TokenAccount,
                    treasury,
                    stakingRewards,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([participant1])
                .rpc();

            await program.methods
                .participateLbm(participation2)
                .accounts({
                    participant: participant2.publicKey,
                    lbmPool,
                    tokenMint,
                    participantTokenAccount: participant2TokenAccount,
                    treasury,
                    stakingRewards,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([participant2])
                .rpc();

            // Verify participation
            const updatedPool = await program.account.liquidityBootstrappingPool.fetch(lbmPool);
            expect(updatedPool.totalParticipants).to.equal(3); // Creator + 2 participants

            // Step 5: Finalize LBM
            console.log('Step 5: Finalizing LBM...');
            await program.methods
                .finalizeLbm()
                .accounts({
                    creator: creator.publicKey,
                    lbmPool,
                    tokenMint,
                    tokenMetadata,
                    treasury,
                    stakingRewards,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([creator])
                .rpc();

            // Verify LBM finalization
            const finalizedPool = await program.account.liquidityBootstrappingPool.fetch(lbmPool);
            expect(finalizedPool.isActive).to.be.false;
            expect(finalizedPool.priceDiscoveryComplete).to.be.true;

            // Step 6: Creator chooses vesting option
            console.log('Step 6: Creator choosing vesting option...');
            await program.methods
                .chooseVestingOption(1) // Distribute option
                .accounts({
                    creator: creator.publicKey,
                    vesting,
                    tokenMint,
                    tokenMetadata,
                    treasury,
                    stakingRewards,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .signers([creator])
                .rpc();

            // Verify vesting choice
            const vestingAccount = await program.account.vesting.fetch(vesting);
            expect(vestingAccount.choiceMade).to.be.true;
            expect(vestingAccount.distributionChoice).to.equal(1);

            console.log('✅ Complete workflow executed successfully!');
        });

        it('should handle trading fee collection', async () => {
            console.log('Testing trading fee collection...');

            const tradeAmount = new BN(1_000_000_000); // 1 SOL trade

            await program.methods
                .collectTradingFee(tradeAmount)
                .accounts({
                    treasury,
                    stakingRewards,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: SystemProgram.programId,
                })
                .rpc();

            // Verify fee distribution
            const treasuryAccount = await program.account.platformTreasury.fetch(treasury);
            const stakingRewardsAccount = await program.account.stakingRewards.fetch(stakingRewards);

            expect(treasuryAccount.feeCollectionStats.totalTradingFees.gt(new BN(0))).to.be.true;
            expect(stakingRewardsAccount.totalRewardsDistributed.gt(new BN(0))).to.be.true;

            console.log('✅ Trading fee collection working correctly!');
        });

        it('should validate creator limits', async () => {
            console.log('Testing creator limit validation...');

            await program.methods
                .validateCreatorLimits()
                .accounts({
                    creator: creator.publicKey,
                    creatorProfile,
                    systemProgram: SystemProgram.programId,
                })
                .signers([creator])
                .rpc();

            // Verify limits are enforced
            const creatorAccount = await program.account.creatorProfile.fetch(creatorProfile);
            expect(creatorAccount.weeklyCreations).to.be.greaterThan(0);
            expect(creatorAccount.weeklyCreations).to.be.lessThanOrEqual(2); // Max 2 per week

            console.log('✅ Creator limit validation working correctly!');
        });
    });

    describe('Error Handling', () => {
        it('should handle insufficient funds gracefully', async () => {
            const poorCreator = Keypair.generate();

            try {
                await program.methods
                    .registerCreator(new BN(30_000_000))
                    .accounts({
                        creator: poorCreator.publicKey,
                        creatorProfile,
                        treasury,
                        systemProgram: SystemProgram.programId,
                    })
                    .signers([poorCreator])
                    .rpc();

                expect.fail('Should have thrown insufficient funds error');
            } catch (error) {
                expect(error.message).to.include('InsufficientFunds');
            }
        });

        it('should handle invalid parameters gracefully', async () => {
            try {
                await program.methods
                    .createToken(9, new BN(0), 0, new BN(0)) // Invalid parameters
                    .accounts({
                        creatorProfile,
                        creator: creator.publicKey,
                        tokenMint,
                        creatorTokenAccount,
                        vesting,
                        tokenMetadata,
                        treasury,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId,
                        rent: SYSVAR_RENT_PUBKEY,
                    })
                    .signers([creator])
                    .rpc();

                expect.fail('Should have thrown invalid parameters error');
            } catch (error) {
                expect(error.message).to.include('InvalidParameters');
            }
        });
    });
});

