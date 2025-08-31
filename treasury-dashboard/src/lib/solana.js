import { Connection, PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, getAccount, getMint } from '@solana/spl-token';

// Configuration
const RPC_ENDPOINT = 'https://api.mainnet-beta.solana.com';
const HELIUS_RPC = 'https://rpc.helius.xyz/?api-key=YOUR_API_KEY'; // Optional: Replace with your Helius API key

export class SolanaService {
  constructor() {
    this.connection = new Connection(RPC_ENDPOINT, 'confirmed');
    this.heliusConnection = HELIUS_RPC ? new Connection(HELIUS_RPC, 'confirmed') : null;
  }

  // Get SOL balance for treasury wallet
  async getTreasuryBalance(treasuryAddress) {
    try {
      const balance = await this.connection.getBalance(new PublicKey(treasuryAddress));
      return balance / LAMPORTS_PER_SOL;
    } catch (error) {
      console.error('Error fetching treasury balance:', error);
      return 0;
    }
  }

  // Get SPL token accounts for treasury wallet
  async getTokenAccounts(treasuryAddress) {
    try {
      const accounts = await this.connection.getTokenAccountsByOwner(
        new PublicKey(treasuryAddress),
        { programId: TOKEN_PROGRAM_ID }
      );

      const tokenAccounts = [];
      
      for (const account of accounts.value) {
        try {
          const accountInfo = await getAccount(this.connection, account.pubkey);
          const mintInfo = await getMint(this.connection, accountInfo.mint);
          
          tokenAccounts.push({
            mint: accountInfo.mint.toString(),
            balance: Number(accountInfo.amount) / Math.pow(10, mintInfo.decimals),
            decimals: mintInfo.decimals,
            symbol: mintInfo.symbol || 'Unknown',
          });
        } catch (error) {
          console.error('Error fetching token account info:', error);
        }
      }

      return tokenAccounts;
    } catch (error) {
      console.error('Error fetching token accounts:', error);
      return [];
    }
  }

  // Get token supply information
  async getTokenSupply(mintAddress) {
    try {
      const mint = await getMint(this.connection, new PublicKey(mintAddress));
      return {
        totalSupply: Number(mint.supply) / Math.pow(10, mint.decimals),
        decimals: mint.decimals,
        isInitialized: mint.isInitialized,
      };
    } catch (error) {
      console.error('Error fetching token supply:', error);
      return { totalSupply: 0, decimals: 0, isInitialized: false };
    }
  }

  // Get recent transactions for treasury wallet
  async getRecentTransactions(treasuryAddress, limit = 10) {
    try {
      const signatures = await this.connection.getSignaturesForAddress(
        new PublicKey(treasuryAddress),
        { limit }
      );

      const transactions = [];
      
      for (const sig of signatures) {
        try {
          const tx = await this.connection.getTransaction(sig.signature, {
            maxSupportedTransactionVersion: 0,
          });

          if (tx) {
            transactions.push({
              signature: sig.signature,
              timestamp: tx.blockTime,
              fee: tx.meta?.fee || 0,
              success: tx.meta?.err === null,
              instructions: tx.transaction.message.instructions.length,
            });
          }
        } catch (error) {
          console.error('Error fetching transaction:', error);
        }
      }

      return transactions;
    } catch (error) {
      console.error('Error fetching recent transactions:', error);
      return [];
    }
  }

  // Get token holders count (using Helius if available)
  async getTokenHolders(mintAddress) {
    try {
      if (this.heliusConnection) {
        // Use Helius API for better performance
        const response = await fetch(`${HELIUS_RPC.replace('/?api-key=', '')}/v0/token-accounts`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            mintAccounts: [mintAddress],
            includeOffChain: false,
            encoding: 'jsonParsed',
          }),
        });

        const data = await response.json();
        return data.length;
      } else {
        // Fallback to RPC (limited to 1000 accounts)
        const accounts = await this.connection.getTokenLargestAccounts(new PublicKey(mintAddress));
        return accounts.value.length;
      }
    } catch (error) {
      console.error('Error fetching token holders:', error);
      return 0;
    }
  }

  // Subscribe to treasury wallet for real-time updates
  subscribeToTreasury(treasuryAddress, callback) {
    const subscriptionId = this.connection.onAccountChange(
      new PublicKey(treasuryAddress),
      (accountInfo) => {
        callback({
          type: 'balance_change',
          balance: accountInfo.lamports / LAMPORTS_PER_SOL,
          timestamp: Date.now(),
        });
      },
      'confirmed'
    );

    return () => {
      this.connection.removeAccountChangeListener(subscriptionId);
    };
  }

  // Subscribe to transaction logs
  subscribeToLogs(treasuryAddress, callback) {
    const subscriptionId = this.connection.onLogs(
      new PublicKey(treasuryAddress),
      (logs) => {
        callback({
          type: 'transaction',
          signature: logs.signature,
          timestamp: Date.now(),
          logs: logs.logs,
        });
      },
      'confirmed'
    );

    return () => {
      this.connection.removeOnLogsListener(subscriptionId);
    };
  }
}

// Export singleton instance
export const solanaService = new SolanaService();
