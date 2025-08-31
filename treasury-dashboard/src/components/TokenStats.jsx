import React from 'react';
import { BarChart3, Users, Coins, TrendingUp } from 'lucide-react';
import { formatToken, formatNumber, formatPercentage } from '../lib/format.js';

const TokenStats = ({ tokenData, loading = false }) => {
  const { totalSupply, circulatingSupply, burnedSupply, holders, marketCap, price } = tokenData || {};

  if (loading) {
    return (
      <div className="card p-6">
        <div className="animate-pulse">
          <div className="h-6 bg-gray-200 rounded w-1/3 mb-4"></div>
          <div className="grid grid-cols-2 gap-4">
            {[1, 2, 3, 4].map((i) => (
              <div key={i} className="h-16 bg-gray-200 rounded"></div>
            ))}
          </div>
        </div>
      </div>
    );
  }

  const burnedPercentage = totalSupply > 0 ? (burnedSupply / totalSupply) * 100 : 0;
  const circulatingPercentage = totalSupply > 0 ? (circulatingSupply / totalSupply) * 100 : 0;

  return (
    <div className="card p-6">
      <div className="flex items-center gap-2 mb-6">
        <BarChart3 className="w-5 h-5 text-primary-600" />
        <h3 className="text-lg font-semibold">Token Statistics</h3>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Supply Metrics */}
        <div className="space-y-4">
          <div>
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-gray-600">Total Supply</span>
              <span className="text-lg font-bold">
                {formatToken(totalSupply || 0, 'SMEME')}
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div 
                className="bg-primary-600 h-2 rounded-full" 
                style={{ width: '100%' }}
              ></div>
            </div>
          </div>

          <div>
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-gray-600">Circulating Supply</span>
              <span className="text-lg font-bold text-green-600">
                {formatToken(circulatingSupply || 0, 'SMEME')}
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div 
                className="bg-green-500 h-2 rounded-full" 
                style={{ width: `${circulatingPercentage}%` }}
              ></div>
            </div>
            <div className="text-xs text-gray-500 mt-1">
              {formatPercentage(circulatingPercentage / 100)} of total supply
            </div>
          </div>

          <div>
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium text-gray-600">Burned Supply</span>
              <span className="text-lg font-bold text-red-600">
                {formatToken(burnedSupply || 0, 'SMEME')}
              </span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div 
                className="bg-red-500 h-2 rounded-full" 
                style={{ width: `${burnedPercentage}%` }}
              ></div>
            </div>
            <div className="text-xs text-gray-500 mt-1">
              {formatPercentage(burnedPercentage / 100)} of total supply
            </div>
          </div>
        </div>

        {/* Market Metrics */}
        <div className="space-y-4">
          <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <div className="flex items-center gap-2">
              <Users className="w-4 h-4 text-blue-500" />
              <span className="text-sm font-medium text-gray-600">Holders</span>
            </div>
            <span className="text-lg font-bold text-blue-600">
              {formatNumber(holders || 0)}
            </span>
          </div>

          <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <div className="flex items-center gap-2">
              <Coins className="w-4 h-4 text-green-500" />
              <span className="text-sm font-medium text-gray-600">Market Cap</span>
            </div>
            <span className="text-lg font-bold text-green-600">
              {marketCap ? `$${formatNumber(marketCap)}` : 'N/A'}
            </span>
          </div>

          <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <div className="flex items-center gap-2">
              <TrendingUp className="w-4 h-4 text-purple-500" />
              <span className="text-sm font-medium text-gray-600">Price</span>
            </div>
            <span className="text-lg font-bold text-purple-600">
              {price ? `$${price.toFixed(6)}` : 'N/A'}
            </span>
          </div>

          <div className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
            <div className="flex items-center gap-2">
              <BarChart3 className="w-4 h-4 text-orange-500" />
              <span className="text-sm font-medium text-gray-600">Circulating Ratio</span>
            </div>
            <span className="text-lg font-bold text-orange-600">
              {formatPercentage(circulatingPercentage / 100)}
            </span>
          </div>
        </div>
      </div>

      {/* Supply Distribution Chart */}
      <div className="mt-6 pt-4 border-t border-gray-200">
        <h4 className="text-sm font-medium text-gray-700 mb-3">Supply Distribution</h4>
        <div className="flex h-4 rounded-full overflow-hidden">
          <div 
            className="bg-green-500" 
            style={{ width: `${circulatingPercentage}%` }}
            title={`Circulating: ${formatPercentage(circulatingPercentage / 100)}`}
          ></div>
          <div 
            className="bg-red-500" 
            style={{ width: `${burnedPercentage}%` }}
            title={`Burned: ${formatPercentage(burnedPercentage / 100)}`}
          ></div>
          <div 
            className="bg-gray-300" 
            style={{ width: `${100 - circulatingPercentage - burnedPercentage}%` }}
            title={`Other: ${formatPercentage((100 - circulatingPercentage - burnedPercentage) / 100)}`}
          ></div>
        </div>
        <div className="flex justify-between text-xs text-gray-500 mt-2">
          <span>Circulating</span>
          <span>Burned</span>
          <span>Other</span>
        </div>
      </div>
    </div>
  );
};

export default TokenStats;
