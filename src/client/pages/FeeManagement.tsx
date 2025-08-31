import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { toast } from 'react-hot-toast';

interface PlatformConfig {
  tradingFeePercentage: number;
  minTradingFee: number;
  maxTradingFee: number;
  feeChangeCooldown: number;
  lastFeeChange: number;
  pendingFeeChange?: {
    proposedFee: number;
    proposedBy: string;
    proposalTime: number;
    implementationTime: number;
    reason: string;
    votesFor: number;
    votesAgainst: number;
    totalVotes: number;
  };
  emergencyPause: boolean;
  governanceQuorum: number;
}

interface FeeProposal {
  id: string;
  title: string;
  description: string;
  currentFee: number;
  proposedFee: number;
  reason: string;
  status: 'pending' | 'approved' | 'rejected' | 'executed';
  votesFor: number;
  votesAgainst: number;
  totalVotes: number;
  startTime: number;
  endTime: number;
  implementationTime: number;
}

const FeeManagement: React.FC = () => {
  const { publicKey, connected } = useWallet();
  const [platformConfig, setPlatformConfig] = useState<PlatformConfig | null>(null);
  const [feeProposals, setFeeProposals] = useState<FeeProposal[]>([]);
  const [loading, setLoading] = useState(true);
  const [proposing, setProposing] = useState(false);
  const [executing, setExecuting] = useState<string | null>(null);

  // Form state
  const [newFee, setNewFee] = useState<number>(12); // 1.2%
  const [reason, setReason] = useState<string>('');

  useEffect(() => {
    if (connected) {
      loadPlatformConfig();
      loadFeeProposals();
    }
  }, [connected]);

  const loadPlatformConfig = async () => {
    try {
      // Mock data - replace with actual API call
      const mockConfig: PlatformConfig = {
        tradingFeePercentage: 12, // 1.2%
        minTradingFee: 5, // 0.5%
        maxTradingFee: 20, // 2.0%
        feeChangeCooldown: 604800, // 7 days
        lastFeeChange: Date.now() - 86400000, // 1 day ago
        emergencyPause: false,
        governanceQuorum: 10,
        pendingFeeChange: {
          proposedFee: 15, // 1.5%
          proposedBy: 'Admin123...',
          proposalTime: Date.now() - 3600000, // 1 hour ago
          implementationTime: Date.now() + 86400000, // 1 day from now
          reason: 'Increase revenue to fund platform development',
          votesFor: 1500,
          votesAgainst: 200,
          totalVotes: 1700,
        },
      };
      setPlatformConfig(mockConfig);
    } catch (error) {
      console.error('Error loading platform config:', error);
      toast.error('Failed to load platform configuration');
    }
  };

  const loadFeeProposals = async () => {
    try {
      // Mock data - replace with actual API call
      const mockProposals: FeeProposal[] = [
        {
          id: '1',
          title: 'Increase Trading Fee to 1.5%',
          description: 'Proposal to increase trading fee from 1.2% to 1.5% to fund platform development',
          currentFee: 12,
          proposedFee: 15,
          reason: 'Increase revenue to fund platform development and improve staker rewards',
          status: 'pending',
          votesFor: 1500,
          votesAgainst: 200,
          totalVotes: 1700,
          startTime: Date.now() - 3600000,
          endTime: Date.now() + 604800000, // 7 days
          implementationTime: Date.now() + 86400000,
        },
        {
          id: '2',
          title: 'Decrease Trading Fee to 1.0%',
          description: 'Proposal to decrease trading fee from 1.2% to 1.0% to increase trading volume',
          currentFee: 12,
          proposedFee: 10,
          reason: 'Lower fees to attract more traders and increase platform volume',
          status: 'approved',
          votesFor: 2000,
          votesAgainst: 500,
          totalVotes: 2500,
          startTime: Date.now() - 86400000,
          endTime: Date.now() - 86400000 + 604800000,
          implementationTime: Date.now() - 86400000 + 86400000,
        },
      ];
      setFeeProposals(mockProposals);
      setLoading(false);
    } catch (error) {
      console.error('Error loading fee proposals:', error);
      toast.error('Failed to load fee proposals');
    }
  };

  const handleProposeFeeChange = async () => {
    if (!platformConfig) return;

    if (newFee < platformConfig.minTradingFee || newFee > platformConfig.maxTradingFee) {
      toast.error(`Fee must be between ${platformConfig.minTradingFee / 10}% and ${platformConfig.maxTradingFee / 10}%`);
      return;
    }

    if (newFee === platformConfig.tradingFeePercentage) {
      toast.error('New fee must be different from current fee');
      return;
    }

    if (reason.length < 10) {
      toast.error('Please provide a reason (minimum 10 characters)');
      return;
    }

    setProposing(true);
    try {
      // Mock API call - replace with actual implementation
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      toast.success('Fee change proposal submitted successfully!');
      setNewFee(12);
      setReason('');
      loadFeeProposals();
    } catch (error) {
      console.error('Error proposing fee change:', error);
      toast.error('Failed to submit fee change proposal');
    } finally {
      setProposing(false);
    }
  };

  const handleExecuteFeeChange = async (proposalId: string) => {
    setExecuting(proposalId);
    try {
      // Mock API call - replace with actual implementation
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      toast.success('Fee change executed successfully!');
      loadPlatformConfig();
      loadFeeProposals();
    } catch (error) {
      console.error('Error executing fee change:', error);
      toast.error('Failed to execute fee change');
    } finally {
      setExecuting(null);
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pending': return 'text-yellow-600 bg-yellow-100';
      case 'approved': return 'text-green-600 bg-green-100';
      case 'rejected': return 'text-red-600 bg-red-100';
      case 'executed': return 'text-blue-600 bg-blue-100';
      default: return 'text-gray-600 bg-gray-100';
    }
  };

  const formatPercentage = (fee: number) => `${(fee / 10).toFixed(1)}%`;
  const formatTime = (timestamp: number) => new Date(timestamp).toLocaleDateString();

  if (!connected) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-4xl font-bold text-white mb-8">Fee Management</h1>
          <p className="text-gray-300 mb-8">Connect your wallet to manage trading fees</p>
          <WalletMultiButton className="bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-6 rounded-lg" />
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-white mx-auto mb-4"></div>
          <p className="text-white">Loading fee management...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 p-6">
      <div className="max-w-7xl mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-4xl font-bold text-white">Fee Management</h1>
          <WalletMultiButton className="bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-4 rounded-lg" />
        </div>

        {/* Current Fee Status */}
        {platformConfig && (
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 mb-8">
            <h2 className="text-2xl font-bold text-white mb-4">Current Trading Fee</h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="bg-white/5 rounded-lg p-4">
                <p className="text-gray-300 text-sm">Current Fee</p>
                <p className="text-3xl font-bold text-white">{formatPercentage(platformConfig.tradingFeePercentage)}</p>
              </div>
              <div className="bg-white/5 rounded-lg p-4">
                <p className="text-gray-300 text-sm">Fee Range</p>
                <p className="text-xl font-bold text-white">
                  {formatPercentage(platformConfig.minTradingFee)} - {formatPercentage(platformConfig.maxTradingFee)}
                </p>
              </div>
              <div className="bg-white/5 rounded-lg p-4">
                <p className="text-gray-300 text-sm">Last Changed</p>
                <p className="text-lg font-bold text-white">{formatTime(platformConfig.lastFeeChange)}</p>
              </div>
            </div>
          </div>
        )}

        {/* Propose Fee Change */}
        <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 mb-8">
          <h2 className="text-2xl font-bold text-white mb-4">Propose Fee Change</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label className="block text-gray-300 text-sm font-medium mb-2">
                New Fee Percentage
              </label>
              <input
                type="number"
                min={platformConfig?.minTradingFee || 5}
                max={platformConfig?.maxTradingFee || 20}
                step="0.1"
                value={newFee}
                onChange={(e) => setNewFee(Number(e.target.value))}
                className="w-full bg-white/10 border border-gray-600 rounded-lg px-4 py-3 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500"
                placeholder="Enter fee (0.5 - 2.0)"
              />
              <p className="text-sm text-gray-400 mt-1">
                Current: {formatPercentage(platformConfig?.tradingFeePercentage || 12)} | 
                Proposed: {formatPercentage(newFee)}
              </p>
            </div>
            <div>
              <label className="block text-gray-300 text-sm font-medium mb-2">
                Reason for Change
              </label>
              <textarea
                value={reason}
                onChange={(e) => setReason(e.target.value)}
                rows={3}
                className="w-full bg-white/10 border border-gray-600 rounded-lg px-4 py-3 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500"
                placeholder="Explain why this fee change is needed..."
              />
            </div>
          </div>
          <button
            onClick={handleProposeFeeChange}
            disabled={proposing}
            className="mt-6 bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 text-white font-bold py-3 px-6 rounded-lg transition-colors"
          >
            {proposing ? 'Submitting...' : 'Submit Proposal'}
          </button>
        </div>

        {/* Pending Fee Change */}
        {platformConfig?.pendingFeeChange && (
          <div className="bg-yellow-500/10 backdrop-blur-lg rounded-xl p-6 mb-8 border border-yellow-500/30">
            <h2 className="text-2xl font-bold text-yellow-400 mb-4">Pending Fee Change</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <p className="text-gray-300 text-sm">Proposed Fee</p>
                <p className="text-2xl font-bold text-yellow-400">
                  {formatPercentage(platformConfig.pendingFeeChange.proposedFee)}
                </p>
                <p className="text-gray-400 text-sm mt-1">
                  From {formatPercentage(platformConfig.tradingFeePercentage)}
                </p>
              </div>
              <div>
                <p className="text-gray-300 text-sm">Implementation Time</p>
                <p className="text-lg font-bold text-yellow-400">
                  {formatTime(platformConfig.pendingFeeChange.implementationTime)}
                </p>
                <p className="text-gray-400 text-sm mt-1">
                  {platformConfig.pendingFeeChange.reason}
                </p>
              </div>
            </div>
            <div className="mt-4">
              <p className="text-gray-300 text-sm">Voting Results</p>
              <div className="flex items-center gap-4 mt-2">
                <span className="text-green-400">For: {platformConfig.pendingFeeChange.votesFor}</span>
                <span className="text-red-400">Against: {platformConfig.pendingFeeChange.votesAgainst}</span>
                <span className="text-gray-400">Total: {platformConfig.pendingFeeChange.totalVotes}</span>
              </div>
            </div>
          </div>
        )}

        {/* Fee Proposals History */}
        <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6">
          <h2 className="text-2xl font-bold text-white mb-4">Fee Change History</h2>
          <div className="space-y-4">
            {feeProposals.map((proposal) => (
              <div key={proposal.id} className="bg-white/5 rounded-lg p-4">
                <div className="flex justify-between items-start mb-3">
                  <div>
                    <h3 className="text-lg font-bold text-white">{proposal.title}</h3>
                    <p className="text-gray-400 text-sm">{proposal.description}</p>
                  </div>
                  <span className={`px-3 py-1 rounded-full text-sm font-medium ${getStatusColor(proposal.status)}`}>
                    {proposal.status.toUpperCase()}
                  </span>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-3">
                  <div>
                    <p className="text-gray-300 text-sm">Fee Change</p>
                    <p className="text-white font-medium">
                      {formatPercentage(proposal.currentFee)} → {formatPercentage(proposal.proposedFee)}
                    </p>
                  </div>
                  <div>
                    <p className="text-gray-300 text-sm">Voting Results</p>
                    <p className="text-white font-medium">
                      {proposal.votesFor} For / {proposal.votesAgainst} Against
                    </p>
                  </div>
                  <div>
                    <p className="text-gray-300 text-sm">End Date</p>
                    <p className="text-white font-medium">{formatTime(proposal.endTime)}</p>
                  </div>
                </div>
                <p className="text-gray-400 text-sm mb-3">{proposal.reason}</p>
                {proposal.status === 'approved' && (
                  <button
                    onClick={() => handleExecuteFeeChange(proposal.id)}
                    disabled={executing === proposal.id}
                    className="bg-green-600 hover:bg-green-700 disabled:bg-gray-600 text-white font-bold py-2 px-4 rounded-lg transition-colors"
                  >
                    {executing === proposal.id ? 'Executing...' : 'Execute Fee Change'}
                  </button>
                )}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default FeeManagement;
