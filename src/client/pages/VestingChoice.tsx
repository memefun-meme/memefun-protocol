import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { toast } from 'react-hot-toast';

interface VestingInfo {
    mint: string;
    amount: number;
    cliffTime: number;
    choiceDeadline: number;
    choiceMade: boolean;
    distributionChoice?: 'withdraw' | 'burn' | 'distribute';
}

const VestingChoice: React.FC = () => {
    const { publicKey, connected } = useWallet();
    const [vestingInfo, setVestingInfo] = useState<VestingInfo | null>(null);
    const [selectedOption, setSelectedOption] = useState<'withdraw' | 'burn' | 'distribute' | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [timeRemaining, setTimeRemaining] = useState<string>('');

    // Mock vesting data - replace with actual API call
    useEffect(() => {
        if (connected) {
            setVestingInfo({
                mint: '11111111111111111111111111111111',
                amount: 150_000_000,
                cliffTime: Date.now() / 1000,
                choiceDeadline: (Date.now() / 1000) + (7 * 24 * 60 * 60), // 7 days
                choiceMade: false
            });
        }
    }, [connected]);

    // Countdown timer
    useEffect(() => {
        if (vestingInfo && !vestingInfo.choiceMade) {
            const timer = setInterval(() => {
                const now = Date.now() / 1000;
                const remaining = vestingInfo.choiceDeadline - now;

                if (remaining <= 0) {
                    setTimeRemaining('Deadline passed');
                } else {
                    const days = Math.floor(remaining / (24 * 60 * 60));
                    const hours = Math.floor((remaining % (24 * 60 * 60)) / (60 * 60));
                    const minutes = Math.floor((remaining % (60 * 60)) / 60);
                    setTimeRemaining(`${days}d ${hours}h ${minutes}m`);
                }
            }, 1000);

            return () => clearInterval(timer);
        }
    }, [vestingInfo]);

    const handleOptionSelect = (option: 'withdraw' | 'burn' | 'distribute') => {
        setSelectedOption(option);
    };

    const handleSubmitChoice = async () => {
        if (!selectedOption) {
            toast.error('Please select a distribution option');
            return;
        }

        setIsLoading(true);
        try {
            // Mock API call - replace with actual implementation
            await new Promise(resolve => setTimeout(resolve, 2000));

            toast.success(`Distribution choice submitted: ${selectedOption}`);
            setVestingInfo(prev => prev ? { ...prev, choiceMade: true, distributionChoice: selectedOption } : null);
        } catch (error) {
            toast.error('Failed to submit choice. Please try again.');
            console.error('Choice submission error:', error);
        } finally {
            setIsLoading(false);
        }
    };

    const getOptionDescription = (option: string) => {
        switch (option) {
            case 'withdraw':
                return {
                    title: 'Withdraw All Tokens',
                    description: 'Take all your vested tokens to your wallet',
                    benefits: ['Full control over your tokens', 'No restrictions on usage'],
                    risks: ['May impact token price if sold', 'No community benefit'],
                    icon: '💰'
                };
            case 'burn':
                return {
                    title: 'Burn 50% for Community',
                    description: 'Burn half your tokens, keep the other half',
                    benefits: ['Reduces total supply', 'Shows commitment to project', 'May increase token value'],
                    risks: ['Lose 50% of tokens permanently'],
                    icon: '🔥'
                };
            case 'distribute':
                return {
                    title: 'Distribute 50% to Holders',
                    description: 'Give half your tokens to current holders',
                    benefits: ['Rewards loyal holders', 'Builds community trust', 'Distributes wealth fairly'],
                    risks: ['Lose 50% of tokens', 'May not benefit you directly'],
                    icon: '🎁'
                };
            default:
                return { title: '', description: '', benefits: [], risks: [], icon: '' };
        }
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

    if (!vestingInfo) {
        return (
            <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 flex items-center justify-center">
                <div className="spinner"></div>
            </div>
        );
    }

    if (vestingInfo.choiceMade) {
        return (
            <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 py-8">
                <div className="container mx-auto px-4">
                    <div className="max-w-2xl mx-auto text-center">
                        <div className="card">
                            <h1 className="text-3xl font-bold text-white mb-4">Choice Already Made</h1>
                            <p className="text-gray-300 mb-6">
                                You have already chosen to: <strong className="text-white">
                                    {getOptionDescription(vestingInfo.distributionChoice || '').title}
                                </strong>
                            </p>
                            <div className="text-green-400">
                                ✅ Your tokens have been distributed according to your choice
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 py-8">
            <div className="container mx-auto px-4">
                <div className="max-w-4xl mx-auto">
                    {/* Header */}
                    <div className="text-center mb-8">
                        <h1 className="text-4xl font-bold text-white mb-4">
                            Choose Your Distribution Option
                        </h1>
                        <p className="text-gray-300 text-lg">
                            Your vesting period has ended. Choose how to distribute your tokens.
                        </p>

                        {/* Countdown */}
                        <div className="mt-4 p-4 bg-red-900/20 border border-red-500/30 rounded-lg">
                            <p className="text-red-400 font-semibold">
                                ⏰ Time Remaining: {timeRemaining}
                            </p>
                            <p className="text-red-300 text-sm">
                                If you don't choose within the deadline, tokens will be automatically distributed to holders.
                            </p>
                        </div>
                    </div>

                    {/* Vesting Info */}
                    <div className="card mb-8">
                        <h2 className="text-2xl font-semibold text-white mb-4">Vesting Summary</h2>
                        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                            <div className="text-center">
                                <p className="text-gray-400">Total Vested</p>
                                <p className="text-white font-bold text-xl">
                                    {vestingInfo.amount.toLocaleString()} tokens
                                </p>
                            </div>
                            <div className="text-center">
                                <p className="text-gray-400">Vesting Period</p>
                                <p className="text-white font-bold">30 days</p>
                            </div>
                            <div className="text-center">
                                <p className="text-gray-400">Choice Deadline</p>
                                <p className="text-white font-bold">
                                    {new Date(vestingInfo.choiceDeadline * 1000).toLocaleDateString()}
                                </p>
                            </div>
                        </div>
                    </div>

                    {/* Distribution Options */}
                    <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
                        {(['withdraw', 'burn', 'distribute'] as const).map((option) => {
                            const optionInfo = getOptionDescription(option);
                            const isSelected = selectedOption === option;

                            return (
                                <div
                                    key={option}
                                    className={`card cursor-pointer transition-all duration-200 ${isSelected
                                            ? 'ring-2 ring-blue-500 bg-blue-900/20'
                                            : 'hover:bg-gray-800/50'
                                        }`}
                                    onClick={() => handleOptionSelect(option)}
                                >
                                    <div className="text-center mb-4">
                                        <div className="text-4xl mb-2">{optionInfo.icon}</div>
                                        <h3 className="text-xl font-semibold text-white mb-2">
                                            {optionInfo.title}
                                        </h3>
                                        <p className="text-gray-300 text-sm">
                                            {optionInfo.description}
                                        </p>
                                    </div>

                                    <div className="space-y-3">
                                        <div>
                                            <h4 className="text-green-400 font-semibold text-sm mb-1">Benefits:</h4>
                                            <ul className="text-xs text-gray-300 space-y-1">
                                                {optionInfo.benefits.map((benefit, index) => (
                                                    <li key={index}>• {benefit}</li>
                                                ))}
                                            </ul>
                                        </div>

                                        <div>
                                            <h4 className="text-red-400 font-semibold text-sm mb-1">Considerations:</h4>
                                            <ul className="text-xs text-gray-300 space-y-1">
                                                {optionInfo.risks.map((risk, index) => (
                                                    <li key={index}>• {risk}</li>
                                                ))}
                                            </ul>
                                        </div>

                                        <div className="pt-2 border-t border-gray-700">
                                            <p className="text-white font-semibold">
                                                You will receive: {
                                                    option === 'withdraw' ? '100%' :
                                                        option === 'burn' ? '50%' : '50%'
                                                } of your tokens
                                            </p>
                                        </div>
                                    </div>

                                    {isSelected && (
                                        <div className="mt-4 p-2 bg-blue-500/20 border border-blue-500/30 rounded">
                                            <p className="text-blue-400 text-sm text-center">✓ Selected</p>
                                        </div>
                                    )}
                                </div>
                            );
                        })}
                    </div>

                    {/* Submit Button */}
                    <div className="text-center">
                        <button
                            onClick={handleSubmitChoice}
                            disabled={!selectedOption || isLoading}
                            className="btn-primary px-8 py-4 text-lg font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {isLoading ? (
                                <div className="flex items-center justify-center">
                                    <div className="spinner mr-2"></div>
                                    Submitting Choice...
                                </div>
                            ) : (
                                'Submit Distribution Choice'
                            )}
                        </button>

                        {selectedOption && (
                            <p className="text-gray-400 text-sm mt-2">
                                This action cannot be undone. Please review your choice carefully.
                            </p>
                        )}
                    </div>

                    {/* Important Notice */}
                    <div className="mt-8 card bg-yellow-900/20 border border-yellow-500/30">
                        <h3 className="text-yellow-400 font-semibold mb-2">⚠️ Important Notice</h3>
                        <ul className="text-gray-300 text-sm space-y-1">
                            <li>• Your choice is final and cannot be changed once submitted</li>
                            <li>• If you don't choose within the deadline, tokens will be automatically distributed to holders</li>
                            <li>• This choice affects your token economics and community perception</li>
                            <li>• Consider the long-term impact on your project's success</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default VestingChoice;
