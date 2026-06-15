import React, { useState, useEffect } from 'react'
import { TrendingUp, TrendingDown, Zap } from 'lucide-react'
import api from '../services/api'

function Patterns() {
  const [patterns, setPatterns] = useState([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    fetchPatterns()
    const interval = setInterval(fetchPatterns, 15000)
    return () => clearInterval(interval)
  }, [])

  const fetchPatterns = async () => {
    try {
      const response = await api.get('/patterns')
      setPatterns(response.data.data.patterns)
      setLoading(false)
    } catch (error) {
      console.error('Error fetching patterns:', error)
    }
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-4xl font-bold">🔍 Pattern Recognition</h1>
        <p className="text-slate-400 mt-2">Historical pattern matching with DTW & LSTM</p>
      </div>

      {loading ? (
        <div className="text-center py-12 text-slate-400">Loading patterns...</div>
      ) : (
        <div className="space-y-4">
          {patterns.map((pattern, idx) => (
            <div key={idx} className="bg-gradient-to-r from-slate-800 to-slate-800/50 p-6 rounded-lg border border-slate-700 hover:border-blue-600 transition group">
              <div className="flex items-start justify-between mb-4">
                <div className="flex items-center gap-4">
                  <Zap className="text-yellow-400 group-hover:scale-110 transition" size={32} />
                  <div>
                    <h3 className="text-2xl font-bold">{pattern.name}</h3>
                    <p className="text-slate-400">{pattern.symbol} • {pattern.timeframe}</p>
                    <p className="text-slate-500 text-sm mt-2">{pattern.description}</p>
                  </div>
                </div>
                <div className="text-right">
                  <p className="text-3xl font-bold text-blue-400">{(pattern.confidence * 100).toFixed(1)}%</p>
                  <p className="text-slate-400 text-sm">Confidence</p>
                </div>
              </div>

              <div className="grid grid-cols-3 gap-4 pt-4 border-t border-slate-700">
                <div>
                  <p className="text-slate-400 text-sm">Pattern Type</p>
                  <p className="text-lg font-bold text-green-400">{pattern.name}</p>
                </div>
                <div>
                  <p className="text-slate-400 text-sm">Similarity Score</p>
                  <p className="text-lg font-bold text-purple-400">{(pattern.similarity * 100).toFixed(1)}%</p>
                </div>
                <div>
                  <p className="text-slate-400 text-sm">Match Count</p>
                  <p className="text-lg font-bold text-blue-400">12</p>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}

export default Patterns
