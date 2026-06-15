import React, { useState } from 'react'
import { BarChart3, Play } from 'lucide-react'
import api from '../services/api'

function Backtest() {
  const [config, setConfig] = useState({
    symbol: 'BTC/USDT',
    startDate: '2023-01-01',
    endDate: '2024-01-01',
    strategy: 'dtw-based'
  })
  const [results, setResults] = useState(null)
  const [loading, setLoading] = useState(false)

  const handleSubmit = async (e) => {
    e.preventDefault()
    setLoading(true)
    try {
      const response = await api.post('/backtest', config)
      setResults(response.data.data)
    } catch (error) {
      console.error('Error running backtest:', error)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-4xl font-bold">🔄 Backtest Engine</h1>
        <p className="text-slate-400 mt-2">Test strategies on historical market data</p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Configuration */}
        <div className="bg-gradient-to-br from-slate-800 to-slate-800/50 p-8 rounded-lg border border-slate-700">
          <h2 className="text-2xl font-bold mb-6 flex items-center gap-2">
            <BarChart3 size={28} />
            Configure Backtest
          </h2>
          <form onSubmit={handleSubmit} className="space-y-6">
            <div>
              <label className="block text-sm font-medium mb-2 text-slate-300">Symbol</label>
              <input
                type="text"
                value={config.symbol}
                onChange={(e) => setConfig({...config, symbol: e.target.value})}
                className="w-full bg-slate-700 border border-slate-600 rounded px-4 py-2 text-white focus:border-blue-500 focus:outline-none"
              />
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium mb-2 text-slate-300">Start Date</label>
                <input
                  type="date"
                  value={config.startDate}
                  onChange={(e) => setConfig({...config, startDate: e.target.value})}
                  className="w-full bg-slate-700 border border-slate-600 rounded px-4 py-2 text-white focus:border-blue-500 focus:outline-none"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-2 text-slate-300">End Date</label>
                <input
                  type="date"
                  value={config.endDate}
                  onChange={(e) => setConfig({...config, endDate: e.target.value})}
                  className="w-full bg-slate-700 border border-slate-600 rounded px-4 py-2 text-white focus:border-blue-500 focus:outline-none"
                />
              </div>
            </div>

            <button
              type="submit"
              disabled={loading}
              className="w-full bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 disabled:from-slate-600 disabled:to-slate-600 py-3 rounded font-bold transition flex items-center justify-center gap-2"
            >
              <Play size={20} />
              {loading ? 'Running...' : 'Run Backtest'}
            </button>
          </form>
        </div>

        {/* Results */}
        {results && (
          <div className="bg-gradient-to-br from-slate-800 to-slate-800/50 p-8 rounded-lg border border-slate-700">
            <h2 className="text-2xl font-bold mb-6 flex items-center gap-2">
              <BarChart3 size={28} />
              Results
            </h2>
            <div className="space-y-4">
              <ResultItem label="Total Trades" value={results.total_trades} />
              <ResultItem label="Winning Trades" value={results.winning_trades} color="green" />
              <ResultItem label="Losing Trades" value={results.losing_trades} color="red" />
              <ResultItem label="Win Rate" value={`${(results.win_rate * 100).toFixed(2)}%`} color="blue" />
              <ResultItem label="Total Profit" value={`$${results.total_profit.toFixed(2)}`} color="green" />
              <ResultItem label="ROI" value={`${(results.roi * 100).toFixed(2)}%`} color="purple" />
              <ResultItem label="Max Drawdown" value={`${(results.max_drawdown * 100).toFixed(2)}%`} color="red" />
              <ResultItem label="Sharpe Ratio" value={results.sharpe_ratio.toFixed(2)} color="orange" />
              <ResultItem label="Profit Factor" value={results.profit_factor.toFixed(2)} />
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

function ResultItem({ label, value, color = 'white' }) {
  const colorClasses = {
    white: 'text-white',
    green: 'text-green-400',
    red: 'text-red-400',
    blue: 'text-blue-400',
    purple: 'text-purple-400',
    orange: 'text-orange-400'
  }

  return (
    <div className="flex justify-between items-center py-3 border-b border-slate-700">
      <span className="text-slate-300">{label}</span>
      <span className={`font-bold ${colorClasses[color]}`}>{value}</span>
    </div>
  )
}

export default Backtest
