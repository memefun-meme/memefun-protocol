import { Program, AnchorProvider, web3, BN } from '@project-serum/anchor';
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo } from '@solana/spl-token';
import { expect } from 'chai';

describe('Liquidity Bootstrapping Mechanism (LBM)', () => {
  const provider = AnchorProvider.env();
  const program = new Program(require('../target/idl/solana_memes.json'), require('../target/deploy/solana_memes-keypair.json').publicKey, provider);

  // Test accounts
  let creator: Keypair;
  let participant: Keypair;
  let tokenMint: PublicKey;
  let creatorTokenAccount: PublicKey;
  let participantTokenAccount: PublicKey;
  let lbmPool: PublicKey;
  let creatorProfile: PublicKey;
  let tokenMetadata: PublicKey;
  let treasury: PublicKey;
  let stakingRewards: PublicKey;

  const targetLiquidity = new BN(10_000_000_000); // 10 SOL
  const bootstrapDuration = new BN(86400); // 24 hours
  const priceDiscoveryPeriod = new BN(3600); // 1 hour
  const maxParticipationPerWallet = new BN(1_000_000_000); // 1 SOL
  const minParticipation = new BN(100_000_000); // 0.1 SOL
  const maxParticipation = new BN(10_000_000_000); // 10 SOL

  before(async () => {
    // Setup test accounts
    creator = Keypair.generate();
    participant = Keypair.generate();

    // Airdrop SOL to test accounts
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(creator.publicKey, 20_000_000_000)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(participant.publicKey, 10_000_000_000)
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

    participantTokenAccount = await createAccount(
      provider.connection,
      participant,
      tokenMint,
      participant.publicKey
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

  describe('LBM Pool Creation', () => {
    it('should create LBM pool successfully', async () => {
      try {
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

        // Verify pool was created
        const poolAccount = await program.account.liquidityBootstrappingPool.fetch(lbmPool);
        expect(poolAccount.tokenMint.toString()).to.equal(tokenMint.toString());
        expect(poolAccount.creator.toString()).to.equal(creator.publicKey.toString());
        expect(poolAccount.targetLiquidity.eq(targetLiquidity)).to.be.true;
        expect(poolAccount.isActive).to.be.true;

      } catch (error) {
        console.error('Error creating LBM pool:', error);
        throw error;
      }
    });

    it('should fail with insufficient liquidity requirement', async () => {
      const lowLiquidity = new BN(500_000_000); // 0.5 SOL (below minimum)

      try {
        await program.methods
          .createLbmPool(
            lowLiquidity,
            bootstrapDuration,
            priceDiscoveryPeriod,
            maxParticipationPerWallet,
            minParticipation,
            maxParticipation,
            true
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

        expect.fail('Should have thrown insufficient liquidity error');
      } catch (error) {
        expect(error.message).to.include('InsufficientLiquidity');
      }
    });
  });

  describe('LBM Participation', () => {
    it('should allow valid participation', async () => {
      const participationAmount = new BN(500_000_000); // 0.5 SOL

      try {
        await program.methods
          .participateLbm(participationAmount)
          .accounts({
            participant: participant.publicKey,
            lbmPool,
            tokenMint,
            participantTokenAccount,
            treasury,
            stakingRewards,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([participant])
          .rpc();

        // Verify participation was recorded
        const poolAccount = await program.account.liquidityBootstrappingPool.fetch(lbmPool);
        expect(poolAccount.totalParticipants).to.equal(2); // Creator + participant
        expect(poolAccount.currentLiquidity.gt(new BN(0))).to.be.true;

      } catch (error) {
        console.error('Error participating in LBM:', error);
        throw error;
      }
    });

    it('should fail with insufficient participation amount', async () => {
      const lowAmount = new BN(50_000_000); // 0.05 SOL (below minimum)

      try {
        await program.methods
          .participateLbm(lowAmount)
          .accounts({
            participant: participant.publicKey,
            lbmPool,
            tokenMint,
            participantTokenAccount,
            treasury,
            stakingRewards,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([participant])
          .rpc();

        expect.fail('Should have thrown insufficient participation amount error');
      } catch (error) {
        expect(error.message).to.include('InsufficientParticipationAmount');
      }
    });
  });

  describe('LBM Finalization', () => {
    it('should finalize LBM pool successfully', async () => {
      try {
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

        // Verify pool was finalized
        const poolAccount = await program.account.liquidityBootstrappingPool.fetch(lbmPool);
        expect(poolAccount.isActive).to.be.false;
        expect(poolAccount.priceDiscoveryComplete).to.be.true;
        expect(poolAccount.finalPrice.gt(new BN(0))).to.be.true;

      } catch (error) {
        console.error('Error finalizing LBM pool:', error);
        throw error;
      }
    });
  });

  describe('LBM Security Features', () => {
    it('should enforce participation limits', async () => {
      const poolAccount = await program.account.liquidityBootstrappingPool.fetch(lbmPool);
      
      expect(poolAccount.minParticipation.eq(minParticipation)).to.be.true;
      expect(poolAccount.maxParticipationPerWallet.eq(maxParticipationPerWallet)).to.be.true;
    });

    it('should handle anti-bot protection', async () => {
      const poolAccount = await program.account.liquidityBootstrappingPool.fetch(lbmPool);
      expect(poolAccount.antiBotEnabled).to.be.true;
    });
  });
});
