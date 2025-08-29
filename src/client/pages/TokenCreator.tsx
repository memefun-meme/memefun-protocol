import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { toast } from 'react-hot-toast';

interface TokenFormData {
  name: string;
  symbol: string;
  uri: string;
  decimals: number;
  totalSupply: number;
  creatorPercent: number;
  vestingDays: number;
}

const TokenCreator: React.FC = () => {
  const { publicKey, connected } = useWallet();
  const [isLoading, setIsLoading] = useState(false);
  const [formData, setFormData] = useState<TokenFormData>({
    name: '',
    symbol: '',
    uri: '',
    decimals: 9,
    totalSupply: 1000000000,
    creatorPercent: 15,
    vestingDays: 30
  });

  const [errors, setErrors] = useState<Partial<TokenFormData>>({});

  const validateForm = (): boolean => {
    const newErrors: Partial<TokenFormData> = {};

    // Name validation
    if (!formData.name.trim()) {
      newErrors.name = 'Token name is required';
    } else if (formData.name.length > 200) {
      newErrors.name = 'Token name must be 200 characters or less';
    }

    // Symbol validation
    if (!formData.symbol.trim()) {
      newErrors.symbol = 'Token symbol is required';
    } else if (formData.symbol.length > 200) {
      newErrors.symbol = 'Token symbol must be 200 characters or less';
    }

    // URI validation
    if (!formData.uri.trim()) {
      newErrors.uri = 'Metadata URI is required';
    } else if (!isValidUrl(formData.uri)) {
      newErrors.uri = 'Please enter a valid URL';
    }

    // Decimals validation
    if (formData.decimals < 0 || formData.decimals > 9) {
      newErrors.decimals = 'Decimals must be between 0 and 9';
    }

    // Supply validation
    if (formData.totalSupply <= 0) {
      newErrors.totalSupply = 'Total supply must be greater than 0';
    }

    // Creator percentage validation
    if (formData.creatorPercent < 1 || formData.creatorPercent > 20) {
      newErrors.creatorPercent = 'Creator percentage must be between 1% and 20%';
    }

    // Vesting validation
    if (formData.vestingDays < 30 || formData.vestingDays > 365) {
      newErrors.vestingDays = 'Vesting period must be between 30 and 365 days';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const isValidUrl = (string: string): boolean => {
    try {
      new URL(string);
      return true;
    } catch (_) {
      return false;
    }
  };

  const handleInputChange = (field: keyof TokenFormData, value: string | number) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => ({ ...prev, [field]: undefined }));
    }
  };

  const handleCreateToken = async () => {
    if (!connected) {
      toast.error('Please connect your wallet first');
      return;
    }

    if (!validateForm()) {
      toast.error('Please fix the errors in the form');
      return;
    }

    setIsLoading(true);
    try {
      // Mock API call - replace with actual implementation
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      toast.success('Token created successfully!');
      // Reset form
      setFormData({
        name: '',
        symbol: '',
        uri: '',
        decimals: 9,
        totalSupply: 1000000000,
        creatorPercent: 15,
        vestingDays: 30
      });
    } catch (error) {
      toast.error('Failed to create token. Please try again.');
      console.error('Token creation error:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const calculateAllocation = () => {
    const creatorAmount = (formData.totalSupply * formData.creatorPercent) / 100;
    const publicAmount = formData.totalSupply - creatorAmount;
    return { creatorAmount, publicAmount };
  };

  const { creatorAmount, publicAmount } = calculateAllocation();

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 py-8">
      <div className="container mx-auto px-4">
        <div className="max-w-4xl mx-auto">
          {/* Header */}
          <div className="text-center mb-8">
            <h1 className="text-4xl font-bold text-white mb-4">
              Create Your Memecoin
            </h1>
            <p className="text-gray-300 text-lg">
              Launch your memecoin with built-in security and anti-rug protection
            </p>
          </div>

          {/* Wallet Connection */}
          <div className="flex justify-center mb-8">
            <WalletMultiButton className="btn-primary" />
          </div>

          {/* Main Form */}
          <div className="card">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
              {/* Left Column - Form */}
              <div>
                <h2 className="text-2xl font-semibold text-white mb-6">
                  Token Details
                </h2>

                {/* Token Name */}
                <div className="mb-6">
                  <label className="block text-gray-300 mb-2">Token Name</label>
                  <input
                    type="text"
                    value={formData.name}
                    onChange={(e) => handleInputChange('name', e.target.value)}
                    className={`input-field w-full ${errors.name ? 'border-red-500' : ''}`}
                    placeholder="e.g., DogeMoon"
                  />
                  {errors.name && (
                    <p className="text-red-400 text-sm mt-1">{errors.name}</p>
                  )}
                </div>

                {/* Token Symbol */}
                <div className="mb-6">
                  <label className="block text-gray-300 mb-2">Token Symbol</label>
                  <input
                    type="text"
                    value={formData.symbol}
                    onChange={(e) => handleInputChange('symbol', e.target.value)}
                    className={`input-field w-full ${errors.symbol ? 'border-red-500' : ''}`}
                    placeholder="e.g., DOGE"
                  />
                  {errors.symbol && (
                    <p className="text-red-400 text-sm mt-1">{errors.symbol}</p>
                  )}
                </div>

                {/* Metadata URI */}
                <div className="mb-6">
                  <label className="block text-gray-300 mb-2">Metadata URI</label>
                  <input
                    type="url"
                    value={formData.uri}
                    onChange={(e) => handleInputChange('uri', e.target.value)}
                    className={`input-field w-full ${errors.uri ? 'border-red-500' : ''}`}
                    placeholder="https://example.com/metadata.json"
                  />
                  {errors.uri && (
                    <p className="text-red-400 text-sm mt-1">{errors.uri}</p>
                  )}
                </div>

                {/* Decimals */}
                <div className="mb-6">
                  <label className="block text-gray-300 mb-2">Decimals</label>
                  <input
                    type="number"
                    value={formData.decimals}
                    onChange={(e) => handleInputChange('decimals', parseInt(e.target.value))}
                    className={`input-field w-full ${errors.decimals ? 'border-red-500' : ''}`}
                    min="0"
                    max="9"
                  />
                  {errors.decimals && (
                    <p className="text-red-400 text-sm mt-1">{errors.decimals}</p>
                  )}
                </div>

                {/* Total Supply */}
                <div className="mb-6">
                  <label className="block text-gray-300 mb-2">Total Supply</label>
                  <input
                    type="number"
                    value={formData.totalSupply}
                    onChange={(e) => handleInputChange('totalSupply', parseInt(e.target.value))}
                    className={`input-field w-full ${errors.totalSupply ? 'border-red-500' : ''}`}
                    min="1"
                  />
                  {errors.totalSupply && (
                    <p className="text-red-400 text-sm mt-1">{errors.totalSupply}</p>
                  )}
                </div>

                {/* Creator Percentage */}
                <div className="mb-6">
                  <label className="block text-gray-300 mb-2">Creator Percentage (%)</label>
                  <input
                    type="number"
                    value={formData.creatorPercent}
                    onChange={(e) => handleInputChange('creatorPercent', parseInt(e.target.value))}
                    className={`input-field w-full ${errors.creatorPercent ? 'border-red-500' : ''}`}
                    min="1"
                    max="20"
                  />
                  {errors.creatorPercent && (
                    <p className="text-red-400 text-sm mt-1">{errors.creatorPercent}</p>
                  )}
                </div>

                {/* Vesting Period */}
                <div className="mb-6">
                  <label className="block text-gray-300 mb-2">Vesting Period (Days)</label>
                  <input
                    type="number"
                    value={formData.vestingDays}
                    onChange={(e) => handleInputChange('vestingDays', parseInt(e.target.value))}
                    className={`input-field w-full ${errors.vestingDays ? 'border-red-500' : ''}`}
                    min="30"
                    max="365"
                  />
                  {errors.vestingDays && (
                    <p className="text-red-400 text-sm mt-1">{errors.vestingDays}</p>
                  )}
                </div>

                {/* Create Button */}
                <button
                  onClick={handleCreateToken}
                  disabled={!connected || isLoading}
                  className="btn-primary w-full py-4 text-lg font-semibold disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {isLoading ? (
                    <div className="flex items-center justify-center">
                      <div className="spinner mr-2"></div>
                      Creating Token...
                    </div>
                  ) : (
                    'Create Token'
                  )}
                </button>
              </div>

              {/* Right Column - Preview */}
              <div>
                <h2 className="text-2xl font-semibold text-white mb-6">
                  Token Preview
                </h2>

                <div className="space-y-6">
                  {/* Token Info */}
                  <div className="card">
                    <h3 className="text-lg font-semibold text-white mb-4">Token Information</h3>
                    <div className="space-y-3">
                      <div className="flex justify-between">
                        <span className="text-gray-300">Name:</span>
                        <span className="text-white font-medium">{formData.name || 'N/A'}</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-gray-300">Symbol:</span>
                        <span className="text-white font-medium">{formData.symbol || 'N/A'}</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-gray-300">Decimals:</span>
                        <span className="text-white font-medium">{formData.decimals}</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-gray-300">Total Supply:</span>
                        <span className="text-white font-medium">
                          {formData.totalSupply.toLocaleString()}
                        </span>
                      </div>
                    </div>
                  </div>

                  {/* Allocation */}
                  <div className="card">
                    <h3 className="text-lg font-semibold text-white mb-4">Token Allocation</h3>
                    <div className="space-y-3">
                      <div className="flex justify-between">
                        <span className="text-gray-300">Creator (Vested):</span>
                        <span className="text-blue-400 font-medium">
                          {creatorAmount.toLocaleString()} ({formData.creatorPercent}%)
                        </span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-gray-300">Public:</span>
                        <span className="text-green-400 font-medium">
                          {publicAmount.toLocaleString()} ({100 - formData.creatorPercent}%)
                        </span>
                      </div>
                    </div>
                  </div>

                  {/* Security Features */}
                  <div className="card">
                    <h3 className="text-lg font-semibold text-white mb-4">Security Features</h3>
                    <div className="space-y-2">
                      <div className="flex items-center text-green-400">
                        <svg className="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                        </svg>
                        Anti-rug vesting ({formData.vestingDays} days)
                      </div>
                      <div className="flex items-center text-green-400">
                        <svg className="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                        </svg>
                        Anti-bot protection
                      </div>
                      <div className="flex items-center text-green-400">
                        <svg className="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                        </svg>
                        Rate limiting (1 token per 30 days)
                      </div>
                      <div className="flex items-center text-green-400">
                        <svg className="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd" />
                        </svg>
                        Liquidity protection
                      </div>
                    </div>
                  </div>

                  {/* Requirements */}
                  <div className="card">
                    <h3 className="text-lg font-semibold text-white mb-4">Requirements</h3>
                    <div className="space-y-2">
                      <div className="flex items-center text-yellow-400">
                        <svg className="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                        </svg>
                        0.5 SOL stake required
                      </div>
                      <div className="flex items-center text-yellow-400">
                        <svg className="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                          <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                        </svg>
                        Creator registration required
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TokenCreator;
