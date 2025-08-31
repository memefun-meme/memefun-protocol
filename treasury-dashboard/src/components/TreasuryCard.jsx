import React from 'react';
import { Wallet, Coins, TrendingUp, TrendingDown } from 'lucide-react';
import { formatSOL, formatToken, formatPercentage } from '../lib/format.js';

const TreasuryCard = ({ treasuryData, loading = false }) => {
  const { solBalance, tokenAccounts, totalValue, change24h } = treasuryData || {};

  if (loading) {
    return (
      <div className="card p-6">
        <div className="animate-pulse">
          <div className="h-4 bg-gray-200 rounded w-1/3 mb-4"></div>
          <div className="h-8 bg-gray-200 rounded w-1/2 mb-2"></div>
          <div className="h-4 bg-gray-200 rounded w-1/4"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="card p-6">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <Wallet className="w-5 h-5 text-primary-600" />
          <h3 className="text-lg font-semibold">Treasury Balance</h3>
        </div>
        <div className={`flex items-center gap-1 text-sm ${
          change24h >= 0 ? 'text-success-600' : 'text-red-600'
        }`}>
          {change24h >= 0 ? (
            <TrendingUp className="w-4 h-4" />
          ) : (
            <TrendingDown className="w-4 h-4" />
          )}
          <span>{formatPercentage(Math.abs(change24h || 0) / 100)}</span>
        </div>
      </div>

      {/* SOL Balance */}
      <div className="mb-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-primary-500 rounded-full"></div>
            <span className="text-sm text-gray-600">SOL</span>
          </div>
          <span className="text-2xl font-bold">{formatSOL(solBalance || 0)}</span>
        </div>
      </div>

      {/* Token Balances */}
      {tokenAccounts && tokenAccounts.length > 0 && (
        <div className="space-y-2">
          <div className="flex items-center gap-2 mb-2">
            <Coins className="w-4 h-4 text-gray-500" />
            <span className="text-sm font-medium text-gray-700">Token Holdings</span>
          </div>
          
          {tokenAccounts.slice(0, 3).map((token, index) => (
            <div key={index} className="flex items-center justify-between py-1">
              <div className="flex items-center gap-2">
                <div className="w-2 h-2 bg-success-500 rounded-full"></div>
                <span className="text-sm text-gray-600">{token.symbol}</span>
              </div>
              <span className="text-sm font-medium">
                {formatToken(token.balance, token.symbol)}
              </span>
            </div>
          ))}
          
          {tokenAccounts.length > 3 && (
            <div className="text-xs text-gray-500 text-center pt-1">
              +{tokenAccounts.length - 3} more tokens
            </div>
          )}
        </div>
      )}

      {/* Total Value */}
      {totalValue && (
        <div className="mt-4 pt-4 border-t border-gray-100">
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium text-gray-700">Total Value</span>
            <span className="text-lg font-bold text-primary-600">
              {formatSOL(totalValue)}
            </span>
          </div>
        </div>
      )}
    </div>
  );
};

export default TreasuryCard;
