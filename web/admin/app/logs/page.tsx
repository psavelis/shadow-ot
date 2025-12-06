'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  FileText, Search, AlertTriangle, AlertCircle, Info, RefreshCw,
  Loader2, ChevronDown, ChevronUp
} from 'lucide-react'
import { useAdminLogs } from '@/shared/hooks/useAdmin'
import type { AdminLog } from '@/shared/types'

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
  const [selectedType, setSelectedType] = useState<'all' | AdminLog['type']>('all')
  const [selectedSource, setSelectedSource] = useState('all')
  const [expandedLog, setExpandedLog] = useState<string | null>(null)
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 50
  
  const { data: logsData, isLoading, error, refetch, isRefetching } = useAdminLogs({
    type: selectedType === 'all' ? undefined : selectedType,
    page: currentPage,
    pageSize,
  })
  
  const logs = logsData?.data || []

  const filteredLogs = logs.filter(log => {
    const matchesSearch = log.message.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesSource = selectedSource === 'all' || log.source === selectedSource
    return matchesSearch && matchesSource
  })

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-white mb-1">System Logs</h1>
            <p className="text-slate-400">Real-time server logs and alerts</p>
          </div>
          <button
            onClick={() => refetch()}
            disabled={isRefetching}
            className="flex items-center gap-2 px-4 py-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-600/50 transition disabled:opacity-50"
          >
            <RefreshCw className={`w-4 h-4 ${isRefetching ? 'animate-spin' : ''}`} />
            Refresh
          </button>
        </div>
      </motion.div>

      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.1 }}
        className="flex flex-wrap gap-3"
      >
        {(['all', 'info', 'warning', 'error', 'critical'] as const).map((type) => {
          const config = type !== 'all' ? typeConfig[type] : null
          const Icon = config?.icon || FileText
          const count = type === 'all' ? logs.length : logs.filter(l => l.type === type).length
          return (
            <button
              key={type}
              onClick={() => setSelectedType(type)}
              className={`flex items-center gap-2 px-4 py-2 rounded-lg transition ${
                selectedType === type 
                  ? config ? `${config.bg} ${config.color}` : 'bg-slate-700 text-white'
                  : 'bg-slate-800/50 text-slate-400 hover:bg-slate-700/50'
              }`}
            >
              <Icon className="w-4 h-4" />
              <span className="capitalize">{type}</span>
              <span className="px-1.5 py-0.5 text-xs rounded bg-slate-900/50">{count}</span>
            </button>
          )
        })}
      </motion.div>

      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.2 }}
        className="flex flex-wrap items-center gap-4 bg-slate-800/50 border border-slate-700/50 rounded-xl p-4"
      >
        <div className="relative flex-1 min-w-[200px]">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
          <input
            type="text"
            placeholder="Search logs..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full bg-slate-900/50 border border-slate-700 rounded-lg pl-10 pr-4 py-2 text-white text-sm placeholder-slate-500 focus:outline-none focus:border-blue-500/50"
          />
        </div>
        <select
          value={selectedSource}
          onChange={(e) => setSelectedSource(e.target.value)}
          className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 text-white text-sm focus:outline-none focus:border-blue-500/50"
        >
          {sources.map(source => (
            <option key={source} value={source}>{source === 'all' ? 'All Sources' : source}</option>
          ))}
        </select>
      </motion.div>

      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.3 }}
        className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
      >
        {isLoading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-blue-400" />
          </div>
        ) : error ? (
          <div className="flex items-center justify-center py-12 text-red-400">
            <AlertTriangle className="w-5 h-5 mr-2" />
            Failed to load logs
          </div>
        ) : filteredLogs.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 text-slate-400">
            <FileText className="w-12 h-12 mb-4 opacity-50" />
            <p>No logs found</p>
          </div>
        ) : (
          <div className="divide-y divide-slate-700/50">
            {filteredLogs.map(log => {
              const config = typeConfig[log.type]
              const Icon = config.icon
              const isExpanded = expandedLog === log.id
              
              return (
                <div 
                  key={log.id}
                  className="hover:bg-slate-700/20 transition cursor-pointer"
                  onClick={() => setExpandedLog(isExpanded ? null : log.id)}
                >
                  <div className="p-4 flex items-start gap-4">
                    <div className={`p-2 rounded-lg ${config.bg}`}>
                      <Icon className={`w-4 h-4 ${config.color}`} />
                    </div>
                    <div className="flex-1 min-w-0">
                      <p className="text-white font-medium">{log.message}</p>
                      <div className="flex items-center gap-3 mt-1 text-sm text-slate-500">
                        <span className="text-slate-400">{log.source}</span>
                        <span>-</span>
                        <span>{formatTime(log.timestamp)}</span>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      <span className={`px-2 py-1 text-xs rounded ${config.bg} ${config.color} capitalize`}>
                        {log.type}
                      </span>
                      {isExpanded ? (
                        <ChevronUp className="w-4 h-4 text-slate-500" />
                      ) : (
                        <ChevronDown className="w-4 h-4 text-slate-500" />
                      )}
                    </div>
                  </div>
                  
                  {isExpanded && log.details && (
                    <div className="px-4 pb-4">
                      <pre className="bg-slate-900/50 rounded-lg p-4 text-sm font-mono text-slate-300 overflow-x-auto">
                        {JSON.stringify(log.details, null, 2)}
                      </pre>
                    </div>
                  )}
                </div>
              )
            })}
          </div>
        )}

        {logsData && logsData.totalPages > 1 && (
          <div className="p-4 border-t border-slate-700 flex items-center justify-between">
            <p className="text-slate-500 text-sm">
              Page {currentPage} of {logsData.totalPages}
            </p>
            <div className="flex items-center gap-2">
              <button
                onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
                disabled={currentPage === 1}
                className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded hover:bg-slate-600/50 transition disabled:opacity-50"
              >
                Previous
              </button>
              <button
                onClick={() => setCurrentPage(p => Math.min(logsData.totalPages, p + 1))}
                disabled={currentPage === logsData.totalPages}
                className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded hover:bg-slate-600/50 transition disabled:opacity-50"
              >
                Next
              </button>
            </div>
          </div>
        )}
      </motion.div>
    </div>
  )
}
