import React from 'react';
import { Activity, ExternalLink, Clock, CheckCircle, XCircle } from 'lucide-react';
import { formatSOL, formatTimeAgo, formatSignature, getSolscanUrl } from '../lib/format.js';

const TxFeed = ({ transactions, loading = false }) => {
  if (loading) {
    return (
      <div className="card p-6">
        <div className="animate-pulse">
          <div className="h-6 bg-gray-200 rounded w-1/4 mb-4"></div>
          <div className="space-y-3">
            {[1, 2, 3, 4, 5].map((i) => (
              <div key={i} className="h-16 bg-gray-200 rounded"></div>
            ))}
          </div>
        </div>
      </div>
    );
  }

  const txs = transactions || [];

  const getTransactionType = (tx) => {
    // This would be enhanced with actual transaction parsing
    if (tx.logs?.some(log => log.includes('buyback'))) return 'buyback';
    if (tx.logs?.some(log => log.includes('fee'))) return 'fee_collection';
    if (tx.logs?.some(log => log.includes('burn'))) return 'burn';
    return 'transfer';
  };

  const getTransactionIcon = (type) => {
    switch (type) {
      case 'buyback':
        return <div className="w-3 h-3 bg-orange-500 rounded-full"></div>;
      case 'fee_collection':
        return <div className="w-3 h-3 bg-green-500 rounded-full"></div>;
      case 'burn':
        return <div className="w-3 h-3 bg-red-500 rounded-full"></div>;
      default:
        return <div className="w-3 h-3 bg-blue-500 rounded-full"></div>;
    }
  };

  const getTransactionColor = (type) => {
    switch (type) {
      case 'buyback':
        return 'text-orange-600';
      case 'fee_collection':
        return 'text-green-600';
      case 'burn':
        return 'text-red-600';
      default:
        return 'text-blue-600';
    }
  };

  return (
    <div className="card p-6">
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-2">
          <Activity className="w-5 h-5 text-primary-600" />
          <h3 className="text-lg font-semibold">Live Transaction Feed</h3>
        </div>
        <div className="text-sm text-gray-500">
          Last {txs.length} transactions
        </div>
      </div>

      {txs.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          <Activity className="w-12 h-12 mx-auto mb-4 text-gray-300" />
          <p>No recent transactions</p>
          <p className="text-sm">Treasury transactions will appear here</p>
        </div>
      ) : (
        <div className="space-y-3">
          {txs.map((tx, index) => {
            const type = getTransactionType(tx);
            const icon = getTransactionIcon(type);
            const color = getTransactionColor(type);
            
            return (
              <div key={index} className="flex items-center gap-3 p-3 border border-gray-100 rounded-lg hover:bg-gray-50">
                <div className="flex items-center gap-2">
                  {icon}
                  <div className="flex items-center gap-1">
                    {tx.success ? (
                      <CheckCircle className="w-4 h-4 text-green-500" />
                    ) : (
                      <XCircle className="w-4 h-4 text-red-500" />
                    )}
                  </div>
                </div>

                <div className="flex-1 min-w-0">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <span className={`text-sm font-medium ${color}`}>
                        {type.replace('_', ' ').toUpperCase()}
                      </span>
                      <span className="text-xs text-gray-500">
                        {tx.instructions} instructions
                      </span>
                    </div>
                    <div className="flex items-center gap-2 text-xs text-gray-500">
                      <Clock className="w-3 h-3" />
                      <span>{formatTimeAgo(tx.timestamp)}</span>
                    </div>
                  </div>
                  
                  <div className="flex items-center justify-between mt-1">
                    <span className="text-xs text-gray-600 font-mono">
                      {formatSignature(tx.signature)}
                    </span>
                    <span className="text-xs text-gray-500">
                      Fee: {formatSOL(tx.fee / 1e9)}
                    </span>
                  </div>
                </div>

                <a
                  href={getSolscanUrl(tx.signature)}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center gap-1 text-sm text-primary-600 hover:text-primary-700"
                >
                  <span>View</span>
                  <ExternalLink className="w-3 h-3" />
                </a>
              </div>
            );
          })}
        </div>
      )}

      {/* Transaction Stats */}
      {txs.length > 0 && (
        <div className="mt-6 pt-4 border-t border-gray-200">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-center">
            <div>
              <div className="text-lg font-bold text-green-600">
                {txs.filter(tx => tx.success).length}
              </div>
              <div className="text-sm text-gray-600">Successful</div>
            </div>
            <div>
              <div className="text-lg font-bold text-red-600">
                {txs.filter(tx => !tx.success).length}
              </div>
              <div className="text-sm text-gray-600">Failed</div>
            </div>
            <div>
              <div className="text-lg font-bold text-blue-600">
                {formatSOL(txs.reduce((sum, tx) => sum + (tx.fee / 1e9), 0))}
              </div>
              <div className="text-sm text-gray-600">Total Fees</div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default TxFeed;
