import React, { useState, useEffect } from 'react'
import { TrendingUp, TrendingDown, Zap } from 'lucide-react'
import api from '../services/api'

function Signals() {
  const [signals, setSignals] = useState([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    fetchSignals()
    const interval = setInterval(fetchSignals, 10000)
    return () => clearInterval(interval)
  }, [])

  const fetchSignals = async () => {
    try {
      const response = await api.get('/signals')
      setSignals(response.data.data.signals)
      setLoading(false)
    } catch (error) {
      console.error('Error fetching signals:', error)
    }
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-4xl font-bold">⚡ Trading Signals</h1>
        <p className="text-slate-400 mt-2">Real-time buy/sell signals with risk management</p>
      </div>

      {loading ? (
        <div className="text-center py-12 text-slate-400">Loading signals...</div>
      ) : (
        <div className="space-y-4">
          {signals.map((signal) => (
            <div
              key={signal.id}
              className={`p-6 rounded-lg border transition ${
                signal.signal_type === 'BUY'
                  ? 'bg-green-900/20 border-green-600/50 hover:border-green-500'
                  : 'bg-red-900/20 border-red-600/50 hover:border-red-500'
              }`}
            >
              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-4">
                  {signal.signal_type === 'BUY' ? (
                    <TrendingUp size={40} className="text-green-400" />
                  ) : (
                    <TrendingDown size={40} className="text-red-400" />
                  )}
                  <div>
                    <h3 className="text-2xl font-bold">{signal.symbol}</h3>
                    <p className={`text-lg font-bold ${signal.signal_type === 'BUY' ? 'text-green-400' : 'text-red-400'}`}>
                      {signal.signal_type}
                    </p>
                  </div>
                </div>
                <div className="text-right">
                  <p className="text-2xl font-bold">${signal.entry_price.toFixed(2)}</p>
                  <p className="text-slate-400 text-sm">Entry Price</p>
                </div>
              </div>

              <div className="grid grid-cols-2 md:grid-cols-4 gap-4 pt-4 border-t border-slate-600">
                <div>
                  <p className="text-slate-400 text-sm">Stop Loss</p>
                  <p className="text-red-400 font-bold">${signal.stop_loss.toFixed(2)}</p>
                </div>
                <div>
                  <p className="text-slate-400 text-sm">Take Profit</p>
                  <p className="text-green-400 font-bold">${signal.take_profit.toFixed(2)}</p>
                </div>
                <div>
                  <p className="text-slate-400 text-sm">Confidence</p>
                  <p className="text-blue-400 font-bold">{(signal.confidence * 100).toFixed(1)}%</p>
                </div>
                <div>
                  <p className="text-slate-400 text-sm">Risk/Reward</p>
                  <p className="text-purple-400 font-bold">{signal.risk_reward_ratio.toFixed(2)}</p>
                </div>
              </div>

              <div className="mt-3 flex items-center gap-2">
                <Zap size={16} className="text-yellow-400" />
                <span className="text-sm text-slate-400">Pattern: {signal.pattern}</span>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}

export default Signals
