import React from 'react'
import { Menu, Moon, Sun } from 'lucide-react'
import { Link } from 'react-router-dom'

function Navbar({ theme, setTheme }) {
  const [isOpen, setIsOpen] = React.useState(false)

  return (
    <nav className="bg-slate-800/80 backdrop-blur border-b border-slate-700 sticky top-0 z-50">
      <div className="container mx-auto px-4 py-4">
        <div className="flex justify-between items-center">
          <Link to="/" className="flex items-center gap-2 text-2xl font-bold bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent hover:scale-105 transition">
            <span>🚀</span>
            Market Maker
          </Link>

          <div className="hidden md:flex items-center gap-8">
            <Link to="/" className="hover:text-blue-400 transition font-medium">Dashboard</Link>
            <Link to="/patterns" className="hover:text-blue-400 transition font-medium">Patterns</Link>
            <Link to="/signals" className="hover:text-blue-400 transition font-medium">Signals</Link>
            <Link to="/backtest" className="hover:text-blue-400 transition font-medium">Backtest</Link>

            <button
              onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
              className="p-2 hover:bg-slate-700 rounded-lg transition"
            >
              {theme === 'dark' ? <Sun size={20} /> : <Moon size={20} />}
            </button>
          </div>

          <button className="md:hidden p-2" onClick={() => setIsOpen(!isOpen)}>
            <Menu size={24} />
          </button>
        </div>

        {isOpen && (
          <div className="md:hidden mt-4 space-y-2 pb-4">
            <Link to="/" className="block py-2 hover:text-blue-400 transition">Dashboard</Link>
            <Link to="/patterns" className="block py-2 hover:text-blue-400 transition">Patterns</Link>
            <Link to="/signals" className="block py-2 hover:text-blue-400 transition">Signals</Link>
            <Link to="/backtest" className="block py-2 hover:text-blue-400 transition">Backtest</Link>
          </div>
        )}
      </div>
    </nav>
  )
}

export default Navbar
