import React, { useState, useEffect } from 'react';
import { RefreshCw, Settings, Shield } from 'lucide-react';
import TreasuryCard from '../components/TreasuryCard.jsx';
import BuybackTable from '../components/BuybackTable.jsx';
import TokenStats from '../components/TokenStats.jsx';
import TxFeed from '../components/TxFeed.jsx';
import { solanaService } from '../lib/solana.js';

// Configuration - Replace with actual addresses
const TREASURY_ADDRESS = 'YOUR_TREASURY_ADDRESS_HERE';
const TOKEN_MINT = 'YOUR_TOKEN_MINT_HERE';

const Dashboard = () => {
  const [loading, setLoading] = useState(true);
  const [treasuryData, setTreasuryData] = useState(null);
  const [buybackData, setBuybackData] = useState([]);
  const [tokenData, setTokenData] = useState(null);
  const [transactions, setTransactions] = useState([]);
  const [lastUpdate, setLastUpdate] = useState(null);

  const fetchData = async () => {
    try {
      setLoading(true);
      
      // Fetch treasury balance and token accounts
      const solBalance = await solanaService.getTreasuryBalance(TREASURY_ADDRESS);
      const tokenAccounts = await solanaService.getTokenAccounts(TREASURY_ADDRESS);
      
      // Calculate total value (simplified - would include token prices)
      const totalValue = solBalance + tokenAccounts.reduce((sum, token) => sum + token.balance, 0);
      
      setTreasuryData({
        solBalance,
        tokenAccounts,
        totalValue,
        change24h: 2.5, // Mock data - would calculate from historical data
      });

      // Fetch token supply and holder data
      const supplyInfo = await solanaService.getTokenSupply(TOKEN_MINT);
      const holders = await solanaService.getTokenHolders(TOKEN_MINT);
      
      setTokenData({
        totalSupply: supplyInfo.totalSupply,
        circulatingSupply: supplyInfo.totalSupply * 0.7, // Mock - would calculate from locked tokens
        burnedSupply: supplyInfo.totalSupply * 0.15, // Mock - would track burned tokens
        holders,
        marketCap: 5000000, // Mock - would calculate from price * supply
        price: 0.000001, // Mock - would fetch from DEX
      });

      // Fetch recent transactions
      const recentTxs = await solanaService.getRecentTransactions(TREASURY_ADDRESS, 10);
      setTransactions(recentTxs);

      // Mock buyback data - would come from indexed events
      setBuybackData([
        {
          timestamp: Date.now() / 1000 - 3600,
          solSpent: 1000,
          tokensBought: 1000000,
          supplyReduced: 0.1,
          signature: 'mock_signature_1',
        },
        {
          timestamp: Date.now() / 1000 - 7200,
          solSpent: 500,
          tokensBought: 500000,
          supplyReduced: 0.05,
          signature: 'mock_signature_2',
        },
      ]);

      setLastUpdate(new Date());
    } catch (error) {
      console.error('Error fetching data:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
    
    // Set up real-time subscriptions
    const unsubscribeTreasury = solanaService.subscribeToTreasury(TREASURY_ADDRESS, (update) => {
      console.log('Treasury update:', update);
      // Update treasury data in real-time
    });

    const unsubscribeLogs = solanaService.subscribeToLogs(TREASURY_ADDRESS, (update) => {
      console.log('Transaction log:', update);
      // Update transaction feed in real-time
    });

    // Auto-refresh every 30 seconds
    const interval = setInterval(fetchData, 30000);

    return () => {
      unsubscribeTreasury();
      unsubscribeLogs();
      clearInterval(interval);
    };
  }, []);

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center gap-3">
              <Shield className="w-8 h-8 text-primary-600" />
              <div>
                <h1 className="text-xl font-bold text-gray-900">Solana Memes Treasury</h1>
                <p className="text-sm text-gray-500">Real-time treasury monitoring</p>
              </div>
            </div>
            
            <div className="flex items-center gap-4">
              {lastUpdate && (
                <div className="text-sm text-gray-500">
                  Last updated: {lastUpdate.toLocaleTimeString()}
                </div>
              )}
              <button
                onClick={fetchData}
                disabled={loading}
                className="btn btn-secondary flex items-center gap-2"
              >
                <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
                Refresh
              </button>
              <button className="btn btn-secondary flex items-center gap-2">
                <Settings className="w-4 h-4" />
                Settings
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Left Column */}
          <div className="space-y-8">
            <TreasuryCard treasuryData={treasuryData} loading={loading} />
            <TokenStats tokenData={tokenData} loading={loading} />
          </div>

          {/* Right Column */}
          <div className="space-y-8">
            <BuybackTable buybackData={buybackData} loading={loading} />
            <TxFeed transactions={transactions} loading={loading} />
          </div>
        </div>

        {/* Configuration Notice */}
        {TREASURY_ADDRESS === 'YOUR_TREASURY_ADDRESS_HERE' && (
          <div className="mt-8 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
            <div className="flex items-center gap-2">
              <Shield className="w-5 h-5 text-yellow-600" />
              <h3 className="text-sm font-medium text-yellow-800">Configuration Required</h3>
            </div>
            <p className="mt-1 text-sm text-yellow-700">
              Please update the treasury address and token mint in the configuration to see real data.
            </p>
          </div>
        )}
      </div>
    </div>
  );
};

export default Dashboard;
