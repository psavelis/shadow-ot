'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { FileText, Search, Filter, AlertTriangle, AlertCircle, Info, CheckCircle, RefreshCw } from 'lucide-react'

const logs = [
  { id: '1', type: 'error', message: 'Database connection timeout', source: 'shadow-db', timestamp: '2024-12-05T10:45:23Z', details: { query: 'SELECT * FROM characters WHERE online = true', timeout: 30000 } },
  { id: '2', type: 'warning', message: 'High memory usage detected on Aetheria (85%)', source: 'shadow-world', timestamp: '2024-12-05T10:44:12Z', details: { realm: 'aetheria', memory: '85%', threshold: '80%' } },
  { id: '3', type: 'info', message: 'Player DragonSlayer logged in', source: 'shadow-api', timestamp: '2024-12-05T10:43:45Z', details: { playerId: '12345', ip: '192.168.1.1' } },
  { id: '4', type: 'info', message: 'Scheduled backup completed successfully', source: 'shadow-db', timestamp: '2024-12-05T10:30:00Z', details: { size: '2.4GB', duration: '45s' } },
  { id: '5', type: 'critical', message: 'Failed to process payment transaction', source: 'shadow-api', timestamp: '2024-12-05T10:28:15Z', details: { transactionId: 'TXN-123456', error: 'Gateway timeout' } },
  { id: '6', type: 'warning', message: 'Rate limit exceeded for IP 10.0.0.5', source: 'shadow-api', timestamp: '2024-12-05T10:25:00Z', details: { ip: '10.0.0.5', requests: 150, limit: 100 } },
  { id: '7', type: 'info', message: 'New account registered: test_user@email.com', source: 'shadow-api', timestamp: '2024-12-05T10:20:33Z', details: { email: 'test_user@email.com' } },
  { id: '8', type: 'error', message: 'Anti-cheat detection: possible speed hack', source: 'shadow-anticheat', timestamp: '2024-12-05T10:15:00Z', details: { characterId: '67890', speed: 250, maxSpeed: 100 } },
]

const typeConfig = {
  info: { color: 'text-blue-400', bg: 'bg-blue-500/20', icon: Info },
  warning: { color: 'text-yellow-400', bg: 'bg-yellow-500/20', icon: AlertTriangle },
  error: { color: 'text-red-400', bg: 'bg-red-500/20', icon: AlertCircle },
  critical: { color: 'text-red-500', bg: 'bg-red-500/30', icon: AlertCircle },
}

const sources = ['all', 'shadow-api', 'shadow-db', 'shadow-world', 'shadow-anticheat', 'shadow-protocol']

function formatTime(date: string) {
  return new Date(date).toLocaleString('en-US', { 
    month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit', second: '2-digit' 
  })
}

export default function LogsPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedType, setSelectedType] = useState<'all' | 'info' | 'warning' | 'error' | 'critical'>('all')
  const [selectedSource, setSelectedSource] = useState('all')
  const [expandedLog, setExpandedLog] = useState<string | null>(null)

  const filteredLogs = logs.filter(log => {
    const matchesSearch = log.message.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesType = selectedType === 'all' || log.type === selectedType
    const matchesSource = selectedSource === 'all' || log.source === selectedSource
    return matchesSearch && matchesType && matchesSource
  })

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-display font-bold mb-1">System Logs</h1>
            <p className="text-shadow-400">Monitor server activity and errors</p>
          </div>
          <button className="btn-secondary flex items-center gap-2">
            <RefreshCw className="w-4 h-4" />
            Refresh
          </button>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="grid sm:grid-cols-4 gap-4">
        <div className="stat-card">
          <FileText className="w-5 h-5 text-accent-500 mb-2" />
          <h3 className="text-2xl font-bold text-white">{logs.length}</h3>
          <p className="text-shadow-400 text-sm">Total Logs (24h)</p>
        </div>
        <div className="stat-card">
          <AlertCircle className="w-5 h-5 text-red-500 mb-2" />
          <h3 className="text-2xl font-bold text-red-400">{logs.filter(l => l.type === 'error' || l.type === 'critical').length}</h3>
          <p className="text-shadow-400 text-sm">Errors</p>
        </div>
        <div className="stat-card">
          <AlertTriangle className="w-5 h-5 text-yellow-500 mb-2" />
          <h3 className="text-2xl font-bold text-yellow-400">{logs.filter(l => l.type === 'warning').length}</h3>
          <p className="text-shadow-400 text-sm">Warnings</p>
        </div>
        <div className="stat-card">
          <Info className="w-5 h-5 text-blue-500 mb-2" />
          <h3 className="text-2xl font-bold text-blue-400">{logs.filter(l => l.type === 'info').length}</h3>
          <p className="text-shadow-400 text-sm">Info</p>
        </div>
      </motion.div>

      {/* Filters */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.15 }} className="card">
        <div className="flex flex-col md:flex-row gap-4">
          <div className="relative flex-1">
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search logs..."
              className="w-full pl-12 pr-4 py-2.5 bg-shadow-800 border border-shadow-600 rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50"
            />
          </div>
          <select
            value={selectedSource}
            onChange={(e) => setSelectedSource(e.target.value)}
            className="px-4 py-2.5 bg-shadow-800 border border-shadow-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-accent-500/50"
          >
            {sources.map((source) => (
              <option key={source} value={source}>{source === 'all' ? 'All Sources' : source}</option>
            ))}
          </select>
          <div className="flex rounded-lg overflow-hidden border border-shadow-600">
            {(['all', 'info', 'warning', 'error', 'critical'] as const).map((type) => (
              <button
                key={type}
                onClick={() => setSelectedType(type)}
                className={`px-4 py-2.5 text-sm capitalize transition-colors ${
                  selectedType === type
                    ? 'bg-accent-500 text-white'
                    : 'bg-shadow-800 text-shadow-400 hover:text-white'
                }`}
              >
                {type}
              </button>
            ))}
          </div>
        </div>
      </motion.div>

      {/* Logs */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="space-y-2">
        {filteredLogs.map((log) => {
          const config = typeConfig[log.type as keyof typeof typeConfig]
          const Icon = config.icon
          const isExpanded = expandedLog === log.id

          return (
            <div key={log.id} className="card cursor-pointer" onClick={() => setExpandedLog(isExpanded ? null : log.id)}>
              <div className="flex items-start gap-4">
                <div className={`w-8 h-8 rounded-lg flex items-center justify-center ${config.bg}`}>
                  <Icon className={`w-4 h-4 ${config.color}`} />
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <span className={`text-xs font-medium uppercase ${config.color}`}>{log.type}</span>
                    <span className="text-xs text-shadow-500">{log.source}</span>
                  </div>
                  <p className="text-white">{log.message}</p>
                  <p className="text-shadow-500 text-xs mt-1">{formatTime(log.timestamp)}</p>
                </div>
              </div>
              {isExpanded && log.details && (
                <div className="mt-4 pt-4 border-t border-shadow-700">
                  <p className="text-xs text-shadow-500 mb-2">Details:</p>
                  <pre className="bg-shadow-800 rounded-lg p-3 text-sm text-shadow-300 overflow-x-auto">
                    {JSON.stringify(log.details, null, 2)}
                  </pre>
                </div>
              )}
            </div>
          )
        })}
      </motion.div>
    </div>
  )
}

