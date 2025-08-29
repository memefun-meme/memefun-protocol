import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

interface Token {
  id: string;
  name: string;
  symbol: string;
  mint: string;
  creator: string;
  totalSupply: number;
  circulatingSupply: number;
  price: number;
  marketCap: number;
  volume24h: number;
  priceChange24h: number;
  riskScore: number;
  isVerified: boolean;
  liquidityLocked: boolean;
  createdAt: string;
  holderCount: number;
}

const TokenExplorer: React.FC = () => {
  const { connected } = useWallet();
  const [tokens, setTokens] = useState<Token[]>([]);
  const [filteredTokens, setFilteredTokens] = useState<Token[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'marketCap' | 'volume24h' | 'priceChange24h' | 'riskScore'>('marketCap');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');
  const [filterVerified, setFilterVerified] = useState(false);
  const [filterLiquidityLocked, setFilterLiquidityLocked] = useState(false);

  // Mock data - replace with actual API call
  useEffect(() => {
    const mockTokens: Token[] = [
      {
        id: '1',
        name: 'DogeMoon',
        symbol: 'DOGE',
        mint: '11111111111111111111111111111111',
        creator: 'Creator1',
        totalSupply: 1000000000,
        circulatingSupply: 850000000,
        price: 0.00000123,
        marketCap: 1045500,
        volume24h: 125000,
        priceChange24h: 15.5,
        riskScore: 25,
        isVerified: true,
        liquidityLocked: true,
        createdAt: '2024-01-15',
        holderCount: 1250
      },
      {
        id: '2',
        name: 'CatCoin',
        symbol: 'CAT',
        mint: '22222222222222222222222222222222',
        creator: 'Creator2',
        totalSupply: 500000000,
        circulatingSupply: 400000000,
        price: 0.00000245,
        marketCap: 980000,
        volume24h: 89000,
        priceChange24h: -5.2,
        riskScore: 45,
        isVerified: false,
        liquidityLocked: false,
        createdAt: '2024-01-20',
        holderCount: 890
      },
      {
        id: '3',
        name: 'PepeToken',
        symbol: 'PEPE',
        mint: '33333333333333333333333333333333',
        creator: 'Creator3',
        totalSupply: 2000000000,
        circulatingSupply: 1800000000,
        price: 0.00000089,
        marketCap: 1602000,
        volume24h: 210000,
        priceChange24h: 8.7,
        riskScore: 15,
        isVerified: true,
        liquidityLocked: true,
        createdAt: '2024-01-10',
        holderCount: 2100
      }
    ];

    setTokens(mockTokens);
    setFilteredTokens(mockTokens);
    setLoading(false);
  }, []);

  // Filter and sort tokens
  useEffect(() => {
    let filtered = tokens.filter(token => {
      const matchesSearch = token.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                           token.symbol.toLowerCase().includes(searchTerm.toLowerCase());
      const matchesVerified = !filterVerified || token.isVerified;
      const matchesLiquidity = !filterLiquidityLocked || token.liquidityLocked;
      
      return matchesSearch && matchesVerified && matchesLiquidity;
    });

    // Sort tokens
    filtered.sort((a, b) => {
      let aValue = a[sortBy];
      let bValue = b[sortBy];
      
      if (sortOrder === 'asc') {
        return aValue - bValue;
      } else {
        return bValue - aValue;
      }
    });

    setFilteredTokens(filtered);
  }, [tokens, searchTerm, sortBy, sortOrder, filterVerified, filterLiquidityLocked]);

  const getRiskColor = (riskScore: number) => {
    if (riskScore <= 20) return 'text-green-400';
    if (riskScore <= 40) return 'text-yellow-400';
    if (riskScore <= 60) return 'text-orange-400';
    return 'text-red-400';
  };

  const getRiskLabel = (riskScore: number) => {
    if (riskScore <= 20) return 'Low Risk';
    if (riskScore <= 40) return 'Medium Risk';
    if (riskScore <= 60) return 'High Risk';
    return 'Very High Risk';
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

  const formatPrice = (price: number) => {
    if (price < 0.000001) {
      return price.toExponential(2);
    }
    return price.toFixed(8);
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 flex items-center justify-center">
        <div className="spinner"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 py-8">
      <div className="container mx-auto px-4">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-white mb-4">
            Token Explorer
          </h1>
          <p className="text-gray-300 text-lg">
            Discover and analyze memecoins with comprehensive risk assessment
          </p>
        </div>

        {/* Controls */}
        <div className="card mb-8">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            {/* Search */}
            <div>
              <label className="block text-gray-300 mb-2">Search Tokens</label>
              <input
                type="text"
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="input-field w-full"
                placeholder="Search by name or symbol..."
              />
            </div>

            {/* Sort By */}
            <div>
              <label className="block text-gray-300 mb-2">Sort By</label>
              <select
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value as any)}
                className="input-field w-full"
              >
                <option value="marketCap">Market Cap</option>
                <option value="volume24h">24h Volume</option>
                <option value="priceChange24h">24h Change</option>
                <option value="riskScore">Risk Score</option>
              </select>
            </div>

            {/* Sort Order */}
            <div>
              <label className="block text-gray-300 mb-2">Order</label>
              <select
                value={sortOrder}
                onChange={(e) => setSortOrder(e.target.value as 'asc' | 'desc')}
                className="input-field w-full"
              >
                <option value="desc">Descending</option>
                <option value="asc">Ascending</option>
              </select>
            </div>

            {/* Filters */}
            <div>
              <label className="block text-gray-300 mb-2">Filters</label>
              <div className="space-y-2">
                <label className="flex items-center text-gray-300">
                  <input
                    type="checkbox"
                    checked={filterVerified}
                    onChange={(e) => setFilterVerified(e.target.checked)}
                    className="mr-2"
                  />
                  Verified Only
                </label>
                <label className="flex items-center text-gray-300">
                  <input
                    type="checkbox"
                    checked={filterLiquidityLocked}
                    onChange={(e) => setFilterLiquidityLocked(e.target.checked)}
                    className="mr-2"
                  />
                  Liquidity Locked
                </label>
              </div>
            </div>
          </div>
        </div>

        {/* Token List */}
        <div className="space-y-4">
          {filteredTokens.map((token) => (
            <div key={token.id} className="card hover-glow">
              <div className="grid grid-cols-1 lg:grid-cols-12 gap-4 items-center">
                {/* Token Info */}
                <div className="lg:col-span-3">
                  <div className="flex items-center space-x-3">
                    <div className="w-10 h-10 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full flex items-center justify-center">
                      <span className="text-white font-bold text-sm">
                        {token.symbol.charAt(0)}
                      </span>
                    </div>
                    <div>
                      <div className="flex items-center space-x-2">
                        <h3 className="text-white font-semibold">{token.name}</h3>
                        {token.isVerified && (
                          <svg className="w-4 h-4 text-blue-400" fill="currentColor" viewBox="0 0 20 20">
                            <path fillRule="evenodd" d="M6.267 3.455a3.066 3.066 0 001.745-.723 3.066 3.066 0 013.976 0 3.066 3.066 0 001.745.723 3.066 3.066 0 012.812 2.812c.051.643.304 1.254.723 1.745a3.066 3.066 0 010 3.976 3.066 3.066 0 00-.723 1.745 3.066 3.066 0 01-2.812 2.812 3.066 3.066 0 00-1.745.723 3.066 3.066 0 01-3.976 0 3.066 3.066 0 00-1.745-.723 3.066 3.066 0 01-2.812-2.812 3.066 3.066 0 00-.723-1.745 3.066 3.066 0 010-3.976 3.066 3.066 0 00.723-1.745 3.066 3.066 0 012.812-2.812zm7.44 5.252a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                          </svg>
                        )}
                        {token.liquidityLocked && (
                          <svg className="w-4 h-4 text-green-400" fill="currentColor" viewBox="0 0 20 20">
                            <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd" />
                          </svg>
                        )}
                      </div>
                      <p className="text-gray-400 text-sm">{token.symbol}</p>
                    </div>
                  </div>
                </div>

                {/* Price */}
                <div className="lg:col-span-2">
                  <div className="text-right">
                    <p className="text-white font-semibold">${formatPrice(token.price)}</p>
                    <p className={`text-sm ${token.priceChange24h >= 0 ? 'text-green-400' : 'text-red-400'}`}>
                      {token.priceChange24h >= 0 ? '+' : ''}{token.priceChange24h.toFixed(2)}%
                    </p>
                  </div>
                </div>

                {/* Market Cap */}
                <div className="lg:col-span-2">
                  <div className="text-right">
                    <p className="text-white font-semibold">${formatNumber(token.marketCap)}</p>
                    <p className="text-gray-400 text-sm">
                      {((token.circulatingSupply / token.totalSupply) * 100).toFixed(1)}% circulating
                    </p>
                  </div>
                </div>

                {/* Volume */}
                <div className="lg:col-span-2">
                  <div className="text-right">
                    <p className="text-white font-semibold">${formatNumber(token.volume24h)}</p>
                    <p className="text-gray-400 text-sm">24h volume</p>
                  </div>
                </div>

                {/* Risk Score */}
                <div className="lg:col-span-2">
                  <div className="text-right">
                    <p className={`font-semibold ${getRiskColor(token.riskScore)}`}>
                      {token.riskScore}/100
                    </p>
                    <p className={`text-sm ${getRiskColor(token.riskScore)}`}>
                      {getRiskLabel(token.riskScore)}
                    </p>
                  </div>
                </div>

                {/* Actions */}
                <div className="lg:col-span-1">
                  <div className="flex justify-end space-x-2">
                    <button className="btn-primary px-3 py-1 text-sm">
                      Trade
                    </button>
                    <button className="btn-secondary px-3 py-1 text-sm">
                      Details
                    </button>
                  </div>
                </div>
              </div>

              {/* Additional Info */}
              <div className="mt-4 pt-4 border-t border-gray-700">
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                  <div>
                    <span className="text-gray-400">Holders:</span>
                    <span className="text-white ml-2">{formatNumber(token.holderCount)}</span>
                  </div>
                  <div>
                    <span className="text-gray-400">Supply:</span>
                    <span className="text-white ml-2">{formatNumber(token.totalSupply)}</span>
                  </div>
                  <div>
                    <span className="text-gray-400">Created:</span>
                    <span className="text-white ml-2">{new Date(token.createdAt).toLocaleDateString()}</span>
                  </div>
                  <div>
                    <span className="text-gray-400">Creator:</span>
                    <span className="text-white ml-2">{token.creator}</span>
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Empty State */}
        {filteredTokens.length === 0 && (
          <div className="card text-center py-12">
            <svg className="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <h3 className="text-xl font-semibold text-white mb-2">No tokens found</h3>
            <p className="text-gray-400">Try adjusting your search or filters</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default TokenExplorer;
