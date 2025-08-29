import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';

interface LBMPool {
    id: string;
    tokenName: string;
    tokenSymbol: string;
    creator: string;
    targetLiquidity: number;
    currentLiquidity: number;
    bootstrapStartTime: number;
    bootstrapEndTime: number;
    currentPrice: number;
    initialPrice: number;
    totalParticipants: number;
    isActive: boolean;
    minParticipation: number;
    maxParticipationPerWallet: number;
    antiBotEnabled: boolean;
    successRate: number;
    timeRemaining: string;
}

const LiquidityBootstrapping: React.FC = () => {
    const { publicKey } = useWallet();
    const [activePools, setActivePools] = useState<LBMPool[]>([]);
    const [selectedPool, setSelectedPool] = useState<LBMPool | null>(null);
    const [participationAmount, setParticipationAmount] = useState<string>('');
    const [loading, setLoading] = useState(false);

    // Mock data for demonstration
    useEffect(() => {
        const mockPools: LBMPool[] = [
            {
                id: '1',
                tokenName: 'MemeCoin Alpha',
                tokenSymbol: 'MALPHA',
                creator: 'Creator1...ABC123',
                targetLiquidity: 10000000000, // 10 SOL
                currentLiquidity: 6500000000, // 6.5 SOL
                bootstrapStartTime: Date.now() - 3600000, // 1 hour ago
                bootstrapEndTime: Date.now() + 82800000, // 23 hours from now
                currentPrice: 1500000, // 1.5 lamports
                initialPrice: 1000000, // 1 lamport
                totalParticipants: 45,
                isActive: true,
                minParticipation: 100000000, // 0.1 SOL
                maxParticipationPerWallet: 1000000000, // 1 SOL
                antiBotEnabled: true,
                successRate: 65,
                timeRemaining: '23h 45m',
            },
            {
                id: '2',
                tokenName: 'DogeMoon',
                tokenSymbol: 'DOGEM',
                creator: 'Creator2...DEF456',
                targetLiquidity: 5000000000, // 5 SOL
                currentLiquidity: 2000000000, // 2 SOL
                bootstrapStartTime: Date.now() - 7200000, // 2 hours ago
                bootstrapEndTime: Date.now() + 79200000, // 22 hours from now
                currentPrice: 800000, // 0.8 lamports
                initialPrice: 1000000, // 1 lamport
                totalParticipants: 23,
                isActive: true,
                minParticipation: 50000000, // 0.05 SOL
                maxParticipationPerWallet: 500000000, // 0.5 SOL
                antiBotEnabled: false,
                successRate: 40,
                timeRemaining: '22h 15m',
            },
        ];
        setActivePools(mockPools);
    }, []);

    const formatCurrency = (amount: number) => {
        return (amount / 1000000000).toFixed(2) + ' SOL';
    };

    const formatPrice = (price: number) => {
        return (price / 1000000).toFixed(4) + ' lamports';
    };

    const handleParticipation = async () => {
        if (!publicKey || !selectedPool) return;

        const amount = parseFloat(participationAmount);
        if (isNaN(amount) || amount <= 0) {
            alert('Please enter a valid amount');
            return;
        }

        const amountInLamports = amount * 1000000000; // Convert SOL to lamports

        if (amountInLamports < selectedPool.minParticipation) {
            alert(`Minimum participation is ${formatCurrency(selectedPool.minParticipation)}`);
            return;
        }

        if (amountInLamports > selectedPool.maxParticipationPerWallet) {
            alert(`Maximum participation is ${formatCurrency(selectedPool.maxParticipationPerWallet)}`);
            return;
        }

        setLoading(true);
        try {
            // Mock transaction
            await new Promise(resolve => setTimeout(resolve, 2000));
            alert('Participation successful!');
            setParticipationAmount('');
            setSelectedPool(null);
        } catch (error) {
            alert('Participation failed: ' + error);
        } finally {
            setLoading(false);
        }
    };

    const getStatusColor = (successRate: number) => {
        if (successRate >= 80) return 'text-green-400';
        if (successRate >= 60) return 'text-yellow-400';
        return 'text-red-400';
    };

    const getStatusText = (successRate: number) => {
        if (successRate >= 80) return 'Excellent';
        if (successRate >= 60) return 'Good';
        if (successRate >= 40) return 'Fair';
        return 'Poor';
    };

    return (
        <div className="min-h-screen bg-gray-900 text-white p-6">
            <div className="max-w-7xl mx-auto">
                {/* Header */}
                <div className="mb-8">
                    <h1 className="text-4xl font-bold text-white mb-4">Liquidity Bootstrapping</h1>
                    <p className="text-gray-400 text-lg">
                        Participate in fair token launches with automated liquidity provision and price discovery
                    </p>
                </div>

                {/* Active Pools */}
                <div className="mb-8">
                    <h2 className="text-2xl font-semibold text-white mb-6">Active LBM Pools</h2>
                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        {activePools.map((pool) => (
                            <div key={pool.id} className="bg-gray-800 rounded-lg p-6 border border-gray-700">
                                <div className="flex justify-between items-start mb-4">
                                    <div>
                                        <h3 className="text-xl font-semibold text-white">{pool.tokenName}</h3>
                                        <p className="text-gray-400">{pool.tokenSymbol}</p>
                                    </div>
                                    <div className="text-right">
                                        <div className={`text-sm font-semibold ${getStatusColor(pool.successRate)}`}>
                                            {getStatusText(pool.successRate)}
                                        </div>
                                        <div className="text-gray-400 text-sm">{pool.successRate}% Success</div>
                                    </div>
                                </div>

                                {/* Progress Bar */}
                                <div className="mb-4">
                                    <div className="flex justify-between text-sm mb-2">
                                        <span className="text-gray-400">Progress</span>
                                        <span className="text-white">
                                            {formatCurrency(pool.currentLiquidity)} / {formatCurrency(pool.targetLiquidity)}
                                        </span>
                                    </div>
                                    <div className="w-full bg-gray-700 rounded-full h-2">
                                        <div
                                            className="bg-gradient-to-r from-blue-500 to-purple-500 h-2 rounded-full transition-all duration-300"
                                            style={{ width: `${pool.successRate}%` }}
                                        ></div>
                                    </div>
                                </div>

                                {/* Pool Stats */}
                                <div className="grid grid-cols-2 gap-4 mb-4">
                                    <div>
                                        <p className="text-gray-400 text-sm">Current Price</p>
                                        <p className="text-white font-semibold">{formatPrice(pool.currentPrice)}</p>
                                    </div>
                                    <div>
                                        <p className="text-gray-400 text-sm">Initial Price</p>
                                        <p className="text-white font-semibold">{formatPrice(pool.initialPrice)}</p>
                                    </div>
                                    <div>
                                        <p className="text-gray-400 text-sm">Participants</p>
                                        <p className="text-white font-semibold">{pool.totalParticipants}</p>
                                    </div>
                                    <div>
                                        <p className="text-gray-400 text-sm">Time Remaining</p>
                                        <p className="text-white font-semibold">{pool.timeRemaining}</p>
                                    </div>
                                </div>

                                {/* Participation Limits */}
                                <div className="bg-gray-700 rounded-lg p-3 mb-4">
                                    <div className="grid grid-cols-2 gap-4 text-sm">
                                        <div>
                                            <p className="text-gray-400">Min Participation</p>
                                            <p className="text-white">{formatCurrency(pool.minParticipation)}</p>
                                        </div>
                                        <div>
                                            <p className="text-gray-400">Max per Wallet</p>
                                            <p className="text-white">{formatCurrency(pool.maxParticipationPerWallet)}</p>
                                        </div>
                                    </div>
                                </div>

                                {/* Action Button */}
                                <button
                                    onClick={() => setSelectedPool(pool)}
                                    className="w-full bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold py-3 px-4 rounded-lg transition-all duration-200"
                                >
                                    Participate Now
                                </button>
                            </div>
                        ))}
                    </div>
                </div>

                {/* Participation Modal */}
                {selectedPool && (
                    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
                        <div className="bg-gray-800 rounded-lg p-6 max-w-md w-full">
                            <h3 className="text-xl font-semibold text-white mb-4">
                                Participate in {selectedPool.tokenName}
                            </h3>

                            <div className="mb-4">
                                <label className="block text-gray-400 text-sm mb-2">Participation Amount (SOL)</label>
                                <input
                                    type="number"
                                    value={participationAmount}
                                    onChange={(e) => setParticipationAmount(e.target.value)}
                                    placeholder="0.0"
                                    step="0.1"
                                    className="w-full bg-gray-700 border border-gray-600 rounded-lg px-3 py-2 text-white focus:outline-none focus:border-blue-500"
                                />
                                <div className="text-xs text-gray-400 mt-1">
                                    Min: {formatCurrency(selectedPool.minParticipation)} |
                                    Max: {formatCurrency(selectedPool.maxParticipationPerWallet)}
                                </div>
                            </div>

                            <div className="bg-gray-700 rounded-lg p-3 mb-4">
                                <div className="text-sm">
                                    <div className="flex justify-between mb-1">
                                        <span className="text-gray-400">Current Pool Progress:</span>
                                        <span className="text-white">{selectedPool.successRate}%</span>
                                    </div>
                                    <div className="flex justify-between mb-1">
                                        <span className="text-gray-400">Current Price:</span>
                                        <span className="text-white">{formatPrice(selectedPool.currentPrice)}</span>
                                    </div>
                                    <div className="flex justify-between">
                                        <span className="text-gray-400">Time Remaining:</span>
                                        <span className="text-white">{selectedPool.timeRemaining}</span>
                                    </div>
                                </div>
                            </div>

                            <div className="flex space-x-3">
                                <button
                                    onClick={() => setSelectedPool(null)}
                                    className="flex-1 bg-gray-600 hover:bg-gray-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors"
                                >
                                    Cancel
                                </button>
                                <button
                                    onClick={handleParticipation}
                                    disabled={loading}
                                    className="flex-1 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 text-white font-semibold py-2 px-4 rounded-lg transition-all duration-200 disabled:opacity-50"
                                >
                                    {loading ? 'Processing...' : 'Participate'}
                                </button>
                            </div>
                        </div>
                    </div>
                )}

                {/* LBM Benefits */}
                <div className="mt-12">
                    <h2 className="text-2xl font-semibold text-white mb-6">Why Liquidity Bootstrapping?</h2>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
                            <div className="w-12 h-12 bg-blue-600 rounded-lg flex items-center justify-center mb-4">
                                <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                            </div>
                            <h3 className="text-lg font-semibold text-white mb-2">Fair Launch</h3>
                            <p className="text-gray-400">Prevents front-running and bot manipulation with gradual price discovery</p>
                        </div>

                        <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
                            <div className="w-12 h-12 bg-green-600 rounded-lg flex items-center justify-center mb-4">
                                <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
                                </svg>
                            </div>
                            <h3 className="text-lg font-semibold text-white mb-2">Price Stability</h3>
                            <p className="text-gray-400">Automated liquidity provision ensures stable trading from day one</p>
                        </div>

                        <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
                            <div className="w-12 h-12 bg-purple-600 rounded-lg flex items-center justify-center mb-4">
                                <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                                </svg>
                            </div>
                            <h3 className="text-lg font-semibold text-white mb-2">Community Driven</h3>
                            <p className="text-gray-400">Community participation determines token success and fair pricing</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default LiquidityBootstrapping;
