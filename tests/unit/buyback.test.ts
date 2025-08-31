import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount } from "@solana/spl-token";
import { expect } from "chai";
import { SolanaMemes } from "../target/types/solana_memes";

describe("Buyback System", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.SolanaMemes as Program<SolanaMemes>;

    let authority: Keypair;
    let treasury: PublicKey;
    let buybackConfig: PublicKey;
    let buybackVault: PublicKey;
    let lpVault: PublicKey;
    let reserveUsdc: PublicKey;
    let memeMint: PublicKey;
    let usdcMint: PublicKey;
    let treasuryBump: number;
    let buybackConfigBump: number;

    before(async () => {
        authority = Keypair.generate();

        // Airdrop SOL to authority
        const signature = await provider.connection.requestAirdrop(authority.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
        await provider.connection.confirmTransaction(signature);

        // Create token mints
        memeMint = await createMint(
            provider.connection,
            authority,
            authority.publicKey,
            null,
            6
        );

        usdcMint = await createMint(
            provider.connection,
            authority,
            authority.publicKey,
            null,
            6
        );

        // Create token accounts
        buybackVault = await createAccount(
            provider.connection,
            authority,
            memeMint,
            authority.publicKey
        );

        lpVault = await createAccount(
            provider.connection,
            authority,
            memeMint,
            authority.publicKey
        );

        reserveUsdc = await createAccount(
            provider.connection,
            authority,
            usdcMint,
            authority.publicKey
        );

        // Mint some USDC to reserve
        await mintTo(
            provider.connection,
            authority,
            usdcMint,
            reserveUsdc,
            authority,
            1_000_000_000 // 1000 USDC
        );

        // Find PDAs
        [treasury, treasuryBump] = PublicKey.findProgramAddressSync(
            [Buffer.from("treasury"), authority.publicKey.toBuffer()],
            program.programId
        );

        [buybackConfig, buybackConfigBump] = PublicKey.findProgramAddressSync(
            [Buffer.from("buyback_config"), authority.publicKey.toBuffer()],
            program.programId
        );
    });

    describe("Initialize Buyback Config", () => {
        it("Should initialize buyback config successfully", async () => {
            const burnPercent = 60;
            const lpPercent = 40;
            const buybackThreshold = 1_000_000; // 1 USDC
            const buybackFrequency = 3600; // 1 hour

            await program.methods
                .initializeBuybackConfig(burnPercent, lpPercent, buybackThreshold, buybackFrequency)
                .accounts({
                    buybackConfig,
                    authority: authority.publicKey,
                    systemProgram: SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const config = await program.account.buybackConfig.fetch(buybackConfig);
            expect(config.burnPercent).to.equal(burnPercent);
            expect(config.lpPercent).to.equal(lpPercent);
            expect(config.authority).to.eql(authority.publicKey);
            expect(config.enabled).to.be.true;
            expect(config.buybackThreshold).to.equal(buybackThreshold);
            expect(config.buybackFrequency).to.equal(buybackFrequency);
        });

        it("Should fail with invalid percentages", async () => {
            const invalidBurnPercent = 70;
            const invalidLpPercent = 40; // Total = 110%

            try {
                await program.methods
                    .initializeBuybackConfig(invalidBurnPercent, invalidLpPercent, 1_000_000, 3600)
                    .accounts({
                        buybackConfig: Keypair.generate().publicKey,
                        authority: authority.publicKey,
                        systemProgram: SystemProgram.programId,
                    })
                    .signers([authority])
                    .rpc();
                expect.fail("Should have thrown an error");
            } catch (error) {
                expect(error.message).to.include("InvalidBuybackPercentages");
            }
        });

        it("Should fail with amount too small", async () => {
            try {
                await program.methods
                    .initializeBuybackConfig(60, 40, 100_000, 3600) // 0.1 USDC
                    .accounts({
                        buybackConfig: Keypair.generate().publicKey,
                        authority: authority.publicKey,
                        systemProgram: SystemProgram.programId,
                    })
                    .signers([authority])
                    .rpc();
                expect.fail("Should have thrown an error");
            } catch (error) {
                expect(error.message).to.include("BuybackAmountTooSmall");
            }
        });
    });

    describe("Initialize Treasury", () => {
        it("Should initialize treasury successfully", async () => {
            await program.methods
                .initializeTreasury()
                .accounts({
                    treasury,
                    reserveUsdc,
                    buybackVault,
                    lpVault,
                    buybackConfig,
                    authority: authority.publicKey,
                    systemProgram: SystemProgram.programId,
                })
                .signers([authority])
                .rpc();

            const treasuryAccount = await program.account.treasury.fetch(treasury);
            expect(treasuryAccount.authority).to.eql(authority.publicKey);
            expect(treasuryAccount.reserveUsdc).to.eql(reserveUsdc);
            expect(treasuryAccount.buybackVault).to.eql(buybackVault);
            expect(treasuryAccount.lpVault).to.eql(lpVault);
            expect(treasuryAccount.buybackConfig).to.eql(buybackConfig);
            expect(treasuryAccount.totalUsdcSpent).to.equal(0);
            expect(treasuryAccount.totalTokensBought).to.equal(0);
            expect(treasuryAccount.bump).to.equal(treasuryBump);
        });
    });

    describe("Record and Finalize Buyback", () => {
        beforeEach(async () => {
            // Mint some tokens to buyback vault to simulate a swap
            await mintTo(
                provider.connection,
                authority,
                memeMint,
                buybackVault,
                authority,
                1_000_000_000 // 1000 tokens
            );
        });

        it("Should finalize buyback successfully", async () => {
            const txSignature = "test_signature_123";
            const amountInUsdc = 10_000_000; // 10 USDC
            const tokensReceived = 1_000_000_000; // 1000 tokens

            // Get initial balances
            const initialBuybackVaultBalance = await getAccount(provider.connection, buybackVault);
            const initialLpVaultBalance = await getAccount(provider.connection, lpVault);

            await program.methods
                .recordAndFinalizeBuyback(txSignature, new anchor.BN(amountInUsdc), new anchor.BN(tokensReceived))
                .accounts({
                    treasury,
                    buybackVault,
                    memeMint,
                    lpVault,
                    buybackConfig,
                    authority: authority.publicKey,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .signers([authority])
                .rpc();

            // Check final balances
            const finalBuybackVaultBalance = await getAccount(provider.connection, buybackVault);
            const finalLpVaultBalance = await getAccount(provider.connection, lpVault);

            // 60% should be burned (600 tokens), 40% should go to LP vault (400 tokens)
            const expectedBurned = tokensReceived * 0.6;
            const expectedLp = tokensReceived * 0.4;

            expect(parseInt(finalBuybackVaultBalance.amount)).to.equal(0); // All tokens processed
            expect(parseInt(finalLpVaultBalance.amount)).to.equal(expectedLp);

            // Check treasury totals
            const treasuryAccount = await program.account.treasury.fetch(treasury);
            expect(treasuryAccount.totalUsdcSpent).to.equal(amountInUsdc);
            expect(treasuryAccount.totalTokensBought).to.equal(tokensReceived);
            expect(treasuryAccount.totalTokensBurned).to.equal(expectedBurned);
            expect(treasuryAccount.totalTokensLp).to.equal(expectedLp);

            // Check buyback config totals
            const config = await program.account.buybackConfig.fetch(buybackConfig);
            expect(config.totalBuybacksExecuted).to.equal(1);
            expect(config.totalUsdcSpent).to.equal(amountInUsdc);
            expect(config.totalTokensBought).to.equal(tokensReceived);
            expect(config.totalTokensBurned).to.equal(expectedBurned);
            expect(config.totalTokensLp).to.equal(expectedLp);
        });

        it("Should fail if buyback is disabled", async () => {
            // Disable buyback
            await program.methods
                .updateBuybackConfig(null, null, null, null, false)
                .accounts({
                    buybackConfig,
                    authority: authority.publicKey,
                })
                .signers([authority])
                .rpc();

            try {
                await program.methods
                    .recordAndFinalizeBuyback("test", new anchor.BN(10_000_000), new anchor.BN(1_000_000_000))
                    .accounts({
                        treasury,
                        buybackVault,
                        memeMint,
                        lpVault,
                        buybackConfig,
                        authority: authority.publicKey,
                        tokenProgram: TOKEN_PROGRAM_ID,
                    })
                    .signers([authority])
                    .rpc();
                expect.fail("Should have thrown an error");
            } catch (error) {
                expect(error.message).to.include("BuybackDisabled");
            }
        });

        it("Should fail if buyback too frequent", async () => {
            // Set very high frequency requirement
            await program.methods
                .updateBuybackConfig(null, null, null, 86400, null) // 24 hours
                .accounts({
                    buybackConfig,
                    authority: authority.publicKey,
                })
                .signers([authority])
                .rpc();

            try {
                await program.methods
                    .recordAndFinalizeBuyback("test", new anchor.BN(10_000_000), new anchor.BN(1_000_000_000))
                    .accounts({
                        treasury,
                        buybackVault,
                        memeMint,
                        lpVault,
                        buybackConfig,
                        authority: authority.publicKey,
                        tokenProgram: TOKEN_PROGRAM_ID,
                    })
                    .signers([authority])
                    .rpc();
                expect.fail("Should have thrown an error");
            } catch (error) {
                expect(error.message).to.include("BuybackTooFrequent");
            }
        });

        it("Should fail with unauthorized authority", async () => {
            const unauthorizedAuthority = Keypair.generate();

            try {
                await program.methods
                    .recordAndFinalizeBuyback("test", new anchor.BN(10_000_000), new anchor.BN(1_000_000_000))
                    .accounts({
                        treasury,
                        buybackVault,
                        memeMint,
                        lpVault,
                        buybackConfig,
                        authority: unauthorizedAuthority.publicKey,
                        tokenProgram: TOKEN_PROGRAM_ID,
                    })
                    .signers([unauthorizedAuthority])
                    .rpc();
                expect.fail("Should have thrown an error");
            } catch (error) {
                expect(error.message).to.include("UnauthorizedBuyback");
            }
        });
    });

    describe("Update Buyback Config", () => {
        it("Should update burn and LP percentages", async () => {
            const newBurnPercent = 70;
            const newLpPercent = 30;

            await program.methods
                .updateBuybackConfig(newBurnPercent, newLpPercent, null, null, null)
                .accounts({
                    buybackConfig,
                    authority: authority.publicKey,
                })
                .signers([authority])
                .rpc();

            const config = await program.account.buybackConfig.fetch(buybackConfig);
            expect(config.burnPercent).to.equal(newBurnPercent);
            expect(config.lpPercent).to.equal(newLpPercent);
        });

        it("Should update buyback threshold", async () => {
            const newThreshold = 5_000_000; // 5 USDC

            await program.methods
                .updateBuybackConfig(null, null, newThreshold, null, null)
                .accounts({
                    buybackConfig,
                    authority: authority.publicKey,
                })
                .signers([authority])
                .rpc();

            const config = await program.account.buybackConfig.fetch(buybackConfig);
            expect(config.buybackThreshold).to.equal(newThreshold);
        });

        it("Should update buyback frequency", async () => {
            const newFrequency = 7200; // 2 hours

            await program.methods
                .updateBuybackConfig(null, null, null, newFrequency, null)
                .accounts({
                    buybackConfig,
                    authority: authority.publicKey,
                })
                .signers([authority])
                .rpc();

            const config = await program.account.buybackConfig.fetch(buybackConfig);
            expect(config.buybackFrequency).to.equal(newFrequency);
        });

        it("Should toggle enabled status", async () => {
            // Disable
            await program.methods
                .updateBuybackConfig(null, null, null, null, false)
                .accounts({
                    buybackConfig,
                    authority: authority.publicKey,
                })
                .signers([authority])
                .rpc();

            let config = await program.account.buybackConfig.fetch(buybackConfig);
            expect(config.enabled).to.be.false;

            // Enable
            await program.methods
                .updateBuybackConfig(null, null, null, null, true)
                .accounts({
                    buybackConfig,
                    authority: authority.publicKey,
                })
                .signers([authority])
                .rpc();

            config = await program.account.buybackConfig.fetch(buybackConfig);
            expect(config.enabled).to.be.true;
        });
    });

    describe("Burn From Buyback Vault", () => {
        beforeEach(async () => {
            // Mint tokens to buyback vault
            await mintTo(
                provider.connection,
                authority,
                memeMint,
                buybackVault,
                authority,
                1_000_000_000
            );
        });

        it("Should burn tokens successfully", async () => {
            const burnAmount = 500_000_000; // 500 tokens

            const initialBalance = await getAccount(provider.connection, buybackVault);

            await program.methods
                .burnFromBuybackVault(new anchor.BN(burnAmount))
                .accounts({
                    buybackVault,
                    memeMint,
                    authority: authority.publicKey,
                    treasury,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .signers([authority])
                .rpc();

            const finalBalance = await getAccount(provider.connection, buybackVault);
            expect(parseInt(finalBalance.amount)).to.equal(parseInt(initialBalance.amount) - burnAmount);
        });

        it("Should fail with zero amount", async () => {
            try {
                await program.methods
                    .burnFromBuybackVault(new anchor.BN(0))
                    .accounts({
                        buybackVault,
                        memeMint,
                        authority: authority.publicKey,
                        treasury,
                        tokenProgram: TOKEN_PROGRAM_ID,
                    })
                    .signers([authority])
                    .rpc();
                expect.fail("Should have thrown an error");
            } catch (error) {
                expect(error.message).to.include("NoTokensToProcess");
            }
        });
    });
});
