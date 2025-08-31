// buyback_executor.js (Node.js executor for Solana Memes buyback system)
import { Connection, PublicKey, Keypair, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import fetch from "node-fetch";
import { Program, AnchorProvider, web3, BN } from "@project-serum/anchor";
import * as fs from "fs";

// Configuration
const RPC = process.env.RPC_URL || "https://api.mainnet-beta.solana.com";
const connection = new Connection(RPC, "confirmed");
const AUTH_KEYPAIR = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("/path/to/authority.json", "utf8"))));
const programId = new PublicKey("YOUR_PROGRAM_ID");
const treasuryPDA = new PublicKey("TREASURY_PDA_PUBKEY");
const buybackVault = new PublicKey("BUYBACK_VAULT_TOKEN_ACCOUNT");
const lpVault = new PublicKey("LP_VAULT_TOKEN_ACCOUNT");
const memeMint = new PublicKey("MEME_MINT");
const usdcMint = new PublicKey("USDC_MINT");

// Anchor provider + program client
const provider = new AnchorProvider(connection, new web3.Keypair(AUTH_KEYPAIR.secretKey), {});
const idl = /* your program IDL */;
const program = new Program(idl, programId, provider);

/**
 * Execute buyback with Jupiter integration
 * @param {number} usdcAmountRaw - Amount in USDC minor units (e.g., 1 USDC = 1_000_000)
 * @returns {Promise<string>} Transaction signature
 */
async function doBuyback(usdcAmountRaw) {
    try {
        console.log(`Starting buyback for ${usdcAmountRaw / 1_000_000} USDC...`);

        // 1) Get Jupiter quote for best route
        const jupiterQuoteUrl = `https://quote-api.jup.ag/v1/quote?inputMint=${usdcMint.toBase58()}&outputMint=${memeMint.toBase58()}&amount=${usdcAmountRaw}&slippageBps=500`;
        console.log("Fetching Jupiter quote...");
        
        const quoteRes = await fetch(jupiterQuoteUrl);
        if (!quoteRes.ok) {
            throw new Error(`Jupiter quote failed: ${quoteRes.statusText}`);
        }
        
        const quote = await quoteRes.json();
        if (!quote || quote.data?.length === 0) {
            throw new Error("No Jupiter route available");
        }

        const best = quote.data[0];
        console.log(`Best route found: ${best.outAmount} tokens for ${usdcAmountRaw} USDC`);

        // 2) Build swap transaction
        console.log("Building swap transaction...");
        const swapPayload = await fetch("https://quote-api.jup.ag/v1/swap", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                route: best,
                userPublicKey: AUTH_KEYPAIR.publicKey.toBase58(),
                wrapUnwrapSOL: false,
                destination: buybackVault.toBase58(), // Send tokens to buyback vault
            }),
        });

        if (!swapPayload.ok) {
            throw new Error(`Jupiter swap build failed: ${swapPayload.statusText}`);
        }

        const swapTx = await swapPayload.json();
        console.log("Swap transaction built successfully");

        // 3) Sign and send swap transaction
        console.log("Signing and sending swap transaction...");
        const swapTransaction = Transaction.from(Buffer.from(swapTx.swapTransaction, 'base64'));
        
        // Add our authority as a signer
        swapTransaction.sign(AUTH_KEYPAIR);
        
        const swapSignature = await sendAndConfirmTransaction(
            connection,
            swapTransaction,
            [AUTH_KEYPAIR],
            { commitment: 'confirmed' }
        );

        console.log(`Swap transaction confirmed: ${swapSignature}`);

        // 4) Verify tokens landed in buybackVault
        console.log("Verifying tokens received...");
        const buybackVaultAccount = await connection.getTokenAccountBalance(buybackVault);
        const tokensReceived = buybackVaultAccount.value.amount;
        
        console.log(`Tokens received in buyback vault: ${tokensReceived}`);

        // 5) Call program.record_and_finalize_buyback
        console.log("Finalizing buyback on-chain...");
        const finalizeTx = await program.methods
            .recordAndFinalizeBuyback(
                swapSignature,
                new BN(usdcAmountRaw),
                new BN(tokensReceived)
            )
            .accounts({
                treasury: treasuryPDA,
                buybackVault: buybackVault,
                memeMint: memeMint,
                lpVault: lpVault,
                buybackConfig: /* buyback_config PDA pubkey */,
                authority: AUTH_KEYPAIR.publicKey,
                tokenProgram: web3.TokenProgram.programId,
            })
            .signers([AUTH_KEYPAIR])
            .rpc();

        console.log(`Buyback finalized: ${finalizeTx}`);
        return finalizeTx;

    } catch (error) {
        console.error("Buyback execution failed:", error);
        throw error;
    }
}

/**
 * Check if buyback should be executed based on conditions
 * @returns {Promise<boolean>}
 */
async function shouldExecuteBuyback() {
    try {
        // Get treasury balance
        const treasuryAccount = await program.account.treasury.fetch(treasuryPDA);
        const usdcBalance = await connection.getTokenAccountBalance(treasuryAccount.reserveUsdc);
        
        // Get buyback config
        const buybackConfig = await program.account.buybackConfig.fetch(/* buyback_config PDA */);
        
        // Check if buyback is enabled
        if (!buybackConfig.enabled) {
            console.log("Buyback is disabled");
            return false;
        }
        
        // Check if enough USDC available
        if (usdcBalance.value.amount < buybackConfig.buybackThreshold) {
            console.log(`Insufficient USDC: ${usdcBalance.value.amount} < ${buybackConfig.buybackThreshold}`);
            return false;
        }
        
        // Check buyback frequency
        const currentTime = Math.floor(Date.now() / 1000);
        if (currentTime - buybackConfig.lastBuybackTime < buybackConfig.buybackFrequency) {
            console.log("Buyback frequency not met");
            return false;
        }
        
        return true;
    } catch (error) {
        console.error("Error checking buyback conditions:", error);
        return false;
    }
}

/**
 * Calculate optimal buyback amount
 * @returns {Promise<number>}
 */
async function calculateBuybackAmount() {
    try {
        const treasuryAccount = await program.account.treasury.fetch(treasuryPDA);
        const usdcBalance = await connection.getTokenAccountBalance(treasuryAccount.reserveUsdc);
        const buybackConfig = await program.account.buybackConfig.fetch(/* buyback_config PDA */);
        
        // Use 0.05% of trading volume or available balance, whichever is smaller
        const availableBalance = parseInt(usdcBalance.value.amount);
        const maxBuybackAmount = Math.min(availableBalance, MAX_BUYBACK_AMOUNT);
        
        // Ensure minimum threshold
        if (maxBuybackAmount < buybackConfig.buybackThreshold) {
            return 0;
        }
        
        return maxBuybackAmount;
    } catch (error) {
        console.error("Error calculating buyback amount:", error);
        return 0;
    }
}

/**
 * Main buyback execution loop
 */
async function buybackLoop() {
    console.log("Starting buyback monitoring loop...");
    
    while (true) {
        try {
            // Check if buyback should be executed
            if (await shouldExecuteBuyback()) {
                const buybackAmount = await calculateBuybackAmount();
                
                if (buybackAmount > 0) {
                    console.log(`Executing buyback for ${buybackAmount / 1_000_000} USDC`);
                    await doBuyback(buybackAmount);
                    console.log("Buyback completed successfully");
                }
            }
            
            // Wait for next check (every 5 minutes)
            await new Promise(resolve => setTimeout(resolve, 5 * 60 * 1000));
            
        } catch (error) {
            console.error("Error in buyback loop:", error);
            // Wait before retrying
            await new Promise(resolve => setTimeout(resolve, 60 * 1000));
        }
    }
}

/**
 * Manual buyback execution
 */
async function manualBuyback(usdcAmount) {
    try {
        console.log(`Executing manual buyback for ${usdcAmount} USDC`);
        const usdcAmountRaw = usdcAmount * 1_000_000; // Convert to minor units
        await doBuyback(usdcAmountRaw);
        console.log("Manual buyback completed successfully");
    } catch (error) {
        console.error("Manual buyback failed:", error);
        throw error;
    }
}

// Export functions for use
export {
    doBuyback,
    shouldExecuteBuyback,
    calculateBuybackAmount,
    buybackLoop,
    manualBuyback
};

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
    const args = process.argv.slice(2);
    
    if (args.length > 0 && args[0] === 'manual') {
        const amount = parseFloat(args[1]) || 10;
        manualBuyback(amount).catch(console.error);
    } else {
        buybackLoop().catch(console.error);
    }
}
