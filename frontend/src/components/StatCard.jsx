import React from 'react'

function StatCard({ title, value, icon, color }) {
  const colorClasses = {
    green: 'from-green-900/30 to-green-800/10 border-green-700/50',
    blue: 'from-blue-900/30 to-blue-800/10 border-blue-700/50',
    purple: 'from-purple-900/30 to-purple-800/10 border-purple-700/50',
    orange: 'from-orange-900/30 to-orange-800/10 border-orange-700/50'
  }

  const iconColors = {
    green: 'text-green-400',
    blue: 'text-blue-400',
    purple: 'text-purple-400',
    orange: 'text-orange-400'
  }

  return (
    <div className={`bg-gradient-to-br ${colorClasses[color]} p-6 rounded-lg border ${colorClasses[color].split(' ')[1]} hover:scale-105 transition`}>
      <div className="flex items-center justify-between">
        <div>
          <p className="text-slate-400 text-sm font-medium">{title}</p>
          <p className="text-3xl font-bold mt-2">{value}</p>
        </div>
        <div className={`opacity-50 group-hover:opacity-100 transition ${iconColors[color]}`}>
          {icon}
        </div>
      </div>
    </div>
  )
}

export default StatCard
