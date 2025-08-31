import React from 'react';
import { Flame, ExternalLink, Calendar, DollarSign } from 'lucide-react';
import { formatSOL, formatToken, formatDate, getSolscanUrl } from '../lib/format.js';

const BuybackTable = ({ buybackData, loading = false }) => {
  if (loading) {
    return (
      <div className="card p-6">
        <div className="animate-pulse">
          <div className="h-6 bg-gray-200 rounded w-1/4 mb-4"></div>
          <div className="space-y-3">
            {[1, 2, 3, 4, 5].map((i) => (
              <div key={i} className="h-12 bg-gray-200 rounded"></div>
            ))}
          </div>
        </div>
      </div>
    );
  }

  const buybacks = buybackData || [];

  return (
    <div className="card p-6">
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-2">
          <Flame className="w-5 h-5 text-orange-500" />
          <h3 className="text-lg font-semibold">Buyback Activity</h3>
        </div>
        <div className="text-sm text-gray-500">
          {buybacks.length} buybacks
        </div>
      </div>

      {buybacks.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          <Flame className="w-12 h-12 mx-auto mb-4 text-gray-300" />
          <p>No buyback activity yet</p>
          <p className="text-sm">Buybacks will appear here when executed</p>
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-gray-200">
                <th className="text-left py-3 px-2 text-sm font-medium text-gray-600">Date</th>
                <th className="text-left py-3 px-2 text-sm font-medium text-gray-600">SOL Spent</th>
                <th className="text-left py-3 px-2 text-sm font-medium text-gray-600">Tokens Bought</th>
                <th className="text-left py-3 px-2 text-sm font-medium text-gray-600">Supply Reduced</th>
                <th className="text-left py-3 px-2 text-sm font-medium text-gray-600">Transaction</th>
              </tr>
            </thead>
            <tbody>
              {buybacks.map((buyback, index) => (
                <tr key={index} className="border-b border-gray-100 hover:bg-gray-50">
                  <td className="py-3 px-2">
                    <div className="flex items-center gap-2">
                      <Calendar className="w-4 h-4 text-gray-400" />
                      <span className="text-sm">{formatDate(buyback.timestamp)}</span>
                    </div>
                  </td>
                  <td className="py-3 px-2">
                    <div className="flex items-center gap-2">
                      <DollarSign className="w-4 h-4 text-green-500" />
                      <span className="text-sm font-medium text-green-600">
                        {formatSOL(buyback.solSpent)}
                      </span>
                    </div>
                  </td>
                  <td className="py-3 px-2">
                    <span className="text-sm">
                      {formatToken(buyback.tokensBought, 'SMEME')}
                    </span>
                  </td>
                  <td className="py-3 px-2">
                    <span className="text-sm text-orange-600 font-medium">
                      {buyback.supplyReduced}%
                    </span>
                  </td>
                  <td className="py-3 px-2">
                    <a
                      href={getSolscanUrl(buyback.signature)}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="flex items-center gap-1 text-sm text-primary-600 hover:text-primary-700"
                    >
                      <span>View</span>
                      <ExternalLink className="w-3 h-3" />
                    </a>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {/* Summary Stats */}
      {buybacks.length > 0 && (
        <div className="mt-6 pt-4 border-t border-gray-200">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="text-center">
              <div className="text-2xl font-bold text-green-600">
                {formatSOL(buybacks.reduce((sum, b) => sum + b.solSpent, 0))}
              </div>
              <div className="text-sm text-gray-600">Total SOL Spent</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-orange-600">
                {formatToken(buybacks.reduce((sum, b) => sum + b.tokensBought, 0), 'SMEME')}
              </div>
              <div className="text-sm text-gray-600">Total Tokens Bought</div>
            </div>
            <div className="text-center">
              <div className="text-2xl font-bold text-red-600">
                {buybacks.reduce((sum, b) => sum + b.supplyReduced, 0).toFixed(2)}%
              </div>
              <div className="text-sm text-gray-600">Total Supply Reduced</div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default BuybackTable;
