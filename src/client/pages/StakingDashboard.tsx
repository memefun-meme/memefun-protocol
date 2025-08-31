import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { toast } from 'react-hot-toast';

interface StakingPosition {
  id: string;
  tokenName: string;
  tokenSymbol: string;
  stakedAmount: number;
  stakedValue: number;
  rewardsEarned: number;
  rewardsValue: number;
  apy: number;
  lockPeriod: number;
  startDate: string;
  endDate: string;
  isLocked: boolean;
  canClaim: boolean;
  totalRewards: number;
}

interface PlatformStats {
  totalStaked: number;
  totalRewardsDistributed: number;
  totalStakers: number;
  averageAPY: number;
  platformFees: number;
  buybackRewards: number;
  successRewards: number;
}

const StakingDashboard: React.FC = () => {
  const { publicKey, connected } = useWallet();
  const [stakingPositions, setStakingPositions] = useState<StakingPosition[]>([]);
  const [platformStats, setPlatformStats] = useState<PlatformStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [claiming, setClaiming] = useState<string | null>(null);
  const [selectedPosition, setSelectedPosition] = useState<StakingPosition | null>(null);

  // Mock data - replace with actual API calls
  useEffect(() => {
    if (connected) {
      // Mock staking positions
      const mockPositions: StakingPosition[] = [
        {
          id: '1',
          tokenName: 'DogeMoon',
          tokenSymbol: 'DOGE',
          stakedAmount: 1000000,
          stakedValue: 1230,
          rewardsEarned: 50000,
          rewardsValue: 61.5,
          apy: 12.5,
          lockPeriod: 30,
          startDate: '2024-01-15',
          endDate: '2024-02-15',
          isLocked: true,
          canClaim: true,
          totalRewards: 50000
        },
        {
          id: '2',
          tokenName: 'CatCoin',
          tokenSymbol: 'CAT',
          stakedAmount: 500000,
          stakedValue: 1225,
          rewardsEarned: 25000,
          rewardsValue: 61.25,
          apy: 15.2,
          lockPeriod: 60,
          startDate: '2024-01-20',
          endDate: '2024-03-20',
          isLocked: true,
          canClaim: false,
          totalRewards: 25000
        }
      ];

      // Mock platform stats
      const mockStats: PlatformStats = {
        totalStaked: 5000000,
        totalRewardsDistributed: 250000,
        totalStakers: 1250,
        averageAPY: 13.8,
        platformFees: 15000,
        buybackRewards: 8000,
        successRewards: 2000
      };

      setStakingPositions(mockPositions);
      setPlatformStats(mockStats);
      setLoading(false);
    }
  }, [connected]);

  const handleClaimRewards = async (positionId: string) => {
    setClaiming(positionId);
    try {
      // Mock API call - replace with actual implementation
      await new Promise(resolve => setTimeout(resolve, 2000));

      toast.success('Rewards claimed successfully!');

      // Update position
      setStakingPositions(prev => prev.map(pos =>
        pos.id === positionId
          ? { ...pos, rewardsEarned: 0, canClaim: false }
          : pos
      ));
    } catch (error) {
      toast.error('Failed to claim rewards. Please try again.');
      console.error('Claim error:', error);
    } finally {
      setClaiming(null);
    }
  };

  const handleUnstake = async (positionId: string) => {
    try {
      // Mock API call - replace with actual implementation
      await new Promise(resolve => setTimeout(resolve, 2000));

      toast.success('Tokens unstaked successfully!');

      // Remove position
      setStakingPositions(prev => prev.filter(pos => pos.id !== positionId));
    } catch (error) {
      toast.error('Failed to unstake tokens. Please try again.');
      console.error('Unstake error:', error);
    }
  };

  const formatNumber = (num: number) => {
    if (num >= 1000000) {
      return (num / 1000000).toFixed(2) + 'M';
    }
    if (num >= 1000) {
      return (num / 1000).toFixed(2) + 'K';
    }
    return num.toLocaleString();
  };

  const formatCurrency = (amount: number) => {
    return `$${amount.toFixed(2)}`;
  };

  if (!connected) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-3xl font-bold text-white mb-4">Connect Your Wallet</h1>
          <WalletMultiButton className="btn-primary" />
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 flex items-center justify-center">
        <div className="spinner"></div>
      </div>
    );
  }

  const totalStakedValue = stakingPositions.reduce((sum, pos) => sum + pos.stakedValue, 0);
  const totalRewardsValue = stakingPositions.reduce((sum, pos) => sum + pos.rewardsValue, 0);
  const totalRewardsEarned = stakingPositions.reduce((sum, pos) => sum + pos.rewardsEarned, 0);

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 py-8">
      <div className="container mx-auto px-4">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-white mb-4">
            Staking Dashboard
          </h1>
          <p className="text-gray-300 text-lg">
            Manage your staked tokens and claim rewards
          </p>
        </div>

        {/* Platform Stats */}
        {platformStats && (
          <div className="card mb-8">
            <h2 className="text-2xl font-semibold text-white mb-6">Platform Statistics</h2>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="text-center">
                <p className="text-gray-400 text-sm">Total Staked</p>
                <p className="text-white font-bold text-xl">
                  {formatNumber(platformStats.totalStaked)} tokens
                </p>
                <p className="text-gray-400 text-xs">
                  {formatCurrency(platformStats.totalStaked * 0.00123)}
                </p>
              </div>
              <div className="text-center">
                <p className="text-gray-400 text-sm">Total Rewards</p>
                <p className="text-white font-bold text-xl">
                  {formatNumber(platformStats.totalRewardsDistributed)} tokens
                </p>
                <p className="text-gray-400 text-xs">
                  {formatCurrency(platformStats.totalRewardsDistributed * 0.00123)}
                </p>
              </div>
              <div className="text-center">
                <p className="text-gray-400 text-sm">Total Stakers</p>
                <p className="text-white font-bold text-xl">
                  {formatNumber(platformStats.totalStakers)}
                </p>
              </div>
              <div className="text-center">
                <p className="text-gray-400 text-sm">Average APY</p>
                <p className="text-white font-bold text-xl">
                  {platformStats.averageAPY.toFixed(1)}%
                </p>
              </div>
            </div>

            {/* Reward Sources */}
            <div className="mt-6 pt-6 border-t border-gray-700">
              <h3 className="text-lg font-semibold text-white mb-4">Reward Sources</h3>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="bg-blue-900/20 p-4 rounded-lg">
                  <h4 className="text-blue-400 font-semibold mb-2">Platform Fees</h4>
                  <p className="text-white font-bold">{formatCurrency(platformStats.platformFees)}</p>
                  <p className="text-gray-400 text-sm">From token creation & trading (55% to stakers, 1.2% trading fee)</p>
                </div>
                <div className="bg-green-900/20 p-4 rounded-lg">
                  <h4 className="text-green-400 font-semibold mb-2">Buyback Rewards</h4>
                  <p className="text-white font-bold">{formatCurrency(platformStats.buybackRewards)}</p>
                  <p className="text-gray-400 text-sm">From buyback operations</p>
                </div>
                <div className="bg-purple-900/20 p-4 rounded-lg">
                  <h4 className="text-purple-400 font-semibold mb-2">Success Rewards</h4>
                  <p className="text-white font-bold">{formatCurrency(platformStats.successRewards)}</p>
                  <p className="text-gray-400 text-sm">From successful projects</p>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Your Staking Summary */}
        <div className="card mb-8">
          <h2 className="text-2xl font-semibold text-white mb-6">Your Staking Summary</h2>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div className="text-center">
              <p className="text-gray-400 text-sm">Total Staked</p>
              <p className="text-white font-bold text-xl">
                {formatNumber(stakingPositions.reduce((sum, pos) => sum + pos.stakedAmount, 0))} tokens
              </p>
              <p className="text-gray-400 text-xs">
                {formatCurrency(totalStakedValue)}
              </p>
            </div>
            <div className="text-center">
              <p className="text-gray-400 text-sm">Total Rewards</p>
              <p className="text-white font-bold text-xl">
                {formatNumber(totalRewardsEarned)} tokens
              </p>
              <p className="text-gray-400 text-xs">
                {formatCurrency(totalRewardsValue)}
              </p>
            </div>
            <div className="text-center">
              <p className="text-gray-400 text-sm">Active Positions</p>
              <p className="text-white font-bold text-xl">
                {stakingPositions.length}
              </p>
            </div>
            <div className="text-center">
              <p className="text-gray-400 text-sm">Average APY</p>
              <p className="text-white font-bold text-xl">
                {stakingPositions.length > 0
                  ? (stakingPositions.reduce((sum, pos) => sum + pos.apy, 0) / stakingPositions.length).toFixed(1)
                  : '0'
                }%
              </p>
            </div>
          </div>
        </div>

        {/* Staking Positions */}
        <div className="space-y-4">
          <h2 className="text-2xl font-semibold text-white">Your Staking Positions</h2>

          {stakingPositions.length === 0 ? (
            <div className="card text-center py-12">
              <svg className="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
              </svg>
              <h3 className="text-xl font-semibold text-white mb-2">No Staking Positions</h3>
              <p className="text-gray-400">Start staking tokens to earn rewards</p>
            </div>
          ) : (
            stakingPositions.map((position) => (
              <div key={position.id} className="card">
                <div className="grid grid-cols-1 lg:grid-cols-12 gap-4 items-center">
                  {/* Token Info */}
                  <div className="lg:col-span-3">
                    <div className="flex items-center space-x-3">
                      <div className="w-10 h-10 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full flex items-center justify-center">
                        <span className="text-white font-bold text-sm">
                          {position.tokenSymbol.charAt(0)}
                        </span>
                      </div>
                      <div>
                        <h3 className="text-white font-semibold">{position.tokenName}</h3>
                        <p className="text-gray-400 text-sm">{position.tokenSymbol}</p>
                      </div>
                    </div>
                  </div>

                  {/* Staked Amount */}
                  <div className="lg:col-span-2">
                    <div className="text-center">
                      <p className="text-gray-400 text-sm">Staked</p>
                      <p className="text-white font-semibold">
                        {formatNumber(position.stakedAmount)} {position.tokenSymbol}
                      </p>
                      <p className="text-gray-400 text-xs">
                        {formatCurrency(position.stakedValue)}
                      </p>
                    </div>
                  </div>

                  {/* Rewards */}
                  <div className="lg:col-span-2">
                    <div className="text-center">
                      <p className="text-gray-400 text-sm">Rewards Earned</p>
                      <p className="text-green-400 font-semibold">
                        {formatNumber(position.rewardsEarned)} {position.tokenSymbol}
                      </p>
                      <p className="text-gray-400 text-xs">
                        {formatCurrency(position.rewardsValue)}
                      </p>
                    </div>
                  </div>

                  {/* APY */}
                  <div className="lg:col-span-2">
                    <div className="text-center">
                      <p className="text-gray-400 text-sm">APY</p>
                      <p className="text-white font-semibold">
                        {position.apy.toFixed(1)}%
                      </p>
                      <p className="text-gray-400 text-xs">
                        Lock: {position.lockPeriod} days
                      </p>
                    </div>
                  </div>

                  {/* Status */}
                  <div className="lg:col-span-2">
                    <div className="text-center">
                      <p className="text-gray-400 text-sm">Status</p>
                      <div className="flex items-center justify-center space-x-2">
                        {position.isLocked ? (
                          <span className="text-yellow-400 text-sm">🔒 Locked</span>
                        ) : (
                          <span className="text-green-400 text-sm">✅ Unlocked</span>
                        )}
                      </div>
                      <p className="text-gray-400 text-xs">
                        {new Date(position.endDate).toLocaleDateString()}
                      </p>
                    </div>
                  </div>

                  {/* Actions */}
                  <div className="lg:col-span-1">
                    <div className="flex flex-col space-y-2">
                      {position.canClaim && (
                        <button
                          onClick={() => handleClaimRewards(position.id)}
                          disabled={claiming === position.id}
                          className="btn-primary px-3 py-1 text-sm disabled:opacity-50"
                        >
                          {claiming === position.id ? 'Claiming...' : 'Claim'}
                        </button>
                      )}
                      {!position.isLocked && (
                        <button
                          onClick={() => handleUnstake(position.id)}
                          className="btn-secondary px-3 py-1 text-sm"
                        >
                          Unstake
                        </button>
                      )}
                    </div>
                  </div>
                </div>

                {/* Progress Bar */}
                <div className="mt-4 pt-4 border-t border-gray-700">
                  <div className="flex justify-between text-sm text-gray-400 mb-2">
                    <span>Staking Progress</span>
                    <span>
                      {Math.floor((Date.now() - new Date(position.startDate).getTime()) /
                        (new Date(position.endDate).getTime() - new Date(position.startDate).getTime()) * 100)}%
                    </span>
                  </div>
                  <div className="w-full bg-gray-700 rounded-full h-2">
                    <div
                      className="bg-gradient-to-r from-blue-500 to-purple-500 h-2 rounded-full"
                      style={{
                        width: `${Math.min(100, Math.floor((Date.now() - new Date(position.startDate).getTime()) /
                          (new Date(position.endDate).getTime() - new Date(position.startDate).getTime()) * 100))}%`
                      }}
                    ></div>
                  </div>
                </div>
              </div>
            ))
          )}
        </div>

        {/* Quick Actions */}
        <div className="mt-8 card">
          <h2 className="text-2xl font-semibold text-white mb-6">Quick Actions</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <button className="btn-primary py-4">
              <div className="text-center">
                <div className="text-2xl mb-2">🔒</div>
                <div className="font-semibold">Stake Tokens</div>
                <div className="text-sm opacity-80">Start earning rewards</div>
              </div>
            </button>
            <button className="btn-secondary py-4">
              <div className="text-center">
                <div className="text-2xl mb-2">📊</div>
                <div className="font-semibold">View Analytics</div>
                <div className="text-sm opacity-80">Detailed performance</div>
              </div>
            </button>
            <button className="btn-secondary py-4">
              <div className="text-center">
                <div className="text-2xl mb-2">📚</div>
                <div className="font-semibold">Staking Guide</div>
                <div className="text-sm opacity-80">Learn how to stake</div>
              </div>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default StakingDashboard;
