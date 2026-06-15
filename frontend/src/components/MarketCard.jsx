import React from 'react'
import { TrendingUp, TrendingDown } from 'lucide-react'

function MarketCard({ market }) {
  const isPositive = market.close >= market.open
  const change = ((market.close - market.open) / market.open * 100).toFixed(2)

  return (
    <div className="bg-gradient-to-br from-slate-800 to-slate-800/50 p-4 rounded-lg border border-slate-700 hover:border-blue-600 transition cursor-pointer group">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h3 className="font-bold text-lg group-hover:text-blue-400 transition">{market.symbol}</h3>
          <p className="text-xs text-slate-400">{market.exchange}</p>
        </div>
        {isPositive ? (
          <TrendingUp size={24} className="text-green-400" />
        ) : (
          <TrendingDown size={24} className="text-red-400" />
        )}
      </div>

      <div className="space-y-2">
        <p className="text-2xl font-bold">
          {typeof market.close === 'number' ? market.close.toFixed(2) : market.close}
        </p>
        <p className={`text-sm font-bold ${isPositive ? 'text-green-400' : 'text-red-400'}`}>
          {isPositive ? '+' : ''}{change}%
        </p>
      </div>

      <div className="mt-3 pt-3 border-t border-slate-700 text-xs text-slate-400">
        <div className="flex justify-between">
          <span>H: {market.high.toFixed(2)}</span>
          <span>L: {market.low.toFixed(2)}</span>
        </div>
      </div>
    </div>
  )
}

export default MarketCard
