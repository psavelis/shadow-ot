'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { Ban, Search, Filter, Clock, User, AlertTriangle, CheckCircle, XCircle, Plus } from 'lucide-react'

const bans = [
  { id: '1', account: 'cheater123', character: 'SpeedHacker', reason: 'Using speed hacks', bannedBy: 'Admin', bannedAt: '2024-12-04T10:00:00Z', expiresAt: null, permanent: true, appealStatus: 'denied' },
  { id: '2', account: 'toxic_player', character: 'FlameWarrior', reason: 'Harassment and hate speech', bannedBy: 'Moderator', bannedAt: '2024-12-03T15:30:00Z', expiresAt: '2024-12-10T15:30:00Z', permanent: false, appealStatus: 'pending' },
  { id: '3', account: 'bot_farmer', character: 'GoldBot99', reason: 'Unauthorized botting in regular zones', bannedBy: 'Anti-cheat', bannedAt: '2024-12-02T08:45:00Z', expiresAt: '2024-12-09T08:45:00Z', permanent: false, appealStatus: null },
  { id: '4', account: 'scammer_account', character: 'TrustMe', reason: 'Scamming other players', bannedBy: 'Admin', bannedAt: '2024-12-01T20:00:00Z', expiresAt: null, permanent: true, appealStatus: 'approved' },
]

function formatDate(date: string) {
  return new Date(date).toLocaleString('en-US', { 
    year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' 
  })
}

export default function BansPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [filterType, setFilterType] = useState<'all' | 'permanent' | 'temporary'>('all')
  const [showBanModal, setShowBanModal] = useState(false)

  const filteredBans = bans.filter(b => {
    const matchesSearch = b.account.toLowerCase().includes(searchQuery.toLowerCase()) ||
                          b.character.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesType = filterType === 'all' || 
                        (filterType === 'permanent' && b.permanent) ||
                        (filterType === 'temporary' && !b.permanent)
    return matchesSearch && matchesType
  })

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-display font-bold mb-1">Ban Management</h1>
            <p className="text-shadow-400">Manage player bans and appeals</p>
          </div>
          <button onClick={() => setShowBanModal(true)} className="btn-primary flex items-center gap-2">
            <Plus className="w-4 h-4" />
            Issue Ban
          </button>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="grid sm:grid-cols-4 gap-4">
        <div className="stat-card">
          <Ban className="w-5 h-5 text-red-500 mb-2" />
          <h3 className="text-2xl font-bold text-white">{bans.length}</h3>
          <p className="text-shadow-400 text-sm">Total Bans</p>
        </div>
        <div className="stat-card">
          <AlertTriangle className="w-5 h-5 text-yellow-500 mb-2" />
          <h3 className="text-2xl font-bold text-white">{bans.filter(b => b.appealStatus === 'pending').length}</h3>
          <p className="text-shadow-400 text-sm">Pending Appeals</p>
        </div>
        <div className="stat-card">
          <Clock className="w-5 h-5 text-blue-500 mb-2" />
          <h3 className="text-2xl font-bold text-white">{bans.filter(b => !b.permanent).length}</h3>
          <p className="text-shadow-400 text-sm">Temporary Bans</p>
        </div>
        <div className="stat-card">
          <XCircle className="w-5 h-5 text-red-500 mb-2" />
          <h3 className="text-2xl font-bold text-white">{bans.filter(b => b.permanent).length}</h3>
          <p className="text-shadow-400 text-sm">Permanent Bans</p>
        </div>
      </motion.div>

      {/* Filters */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.15 }} className="card">
        <div className="flex flex-col sm:flex-row gap-4">
          <div className="relative flex-1">
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search by account or character..."
              className="w-full pl-12 pr-4 py-2.5 bg-shadow-800 border border-shadow-600 rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50"
            />
          </div>
          <div className="flex rounded-lg overflow-hidden border border-shadow-600">
            {(['all', 'permanent', 'temporary'] as const).map((type) => (
              <button
                key={type}
                onClick={() => setFilterType(type)}
                className={`px-4 py-2.5 text-sm capitalize transition-colors ${
                  filterType === type
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

      {/* Bans Table */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="card">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-shadow-700">
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Account</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Character</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Reason</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Type</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Banned At</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Expires</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Appeal</th>
                <th className="text-right py-3 px-4 text-shadow-400 font-medium text-sm">Actions</th>
              </tr>
            </thead>
            <tbody>
              {filteredBans.map((ban) => (
                <tr key={ban.id} className="border-b border-shadow-800 last:border-0 hover:bg-shadow-800/50">
                  <td className="py-3 px-4 font-medium text-white">{ban.account}</td>
                  <td className="py-3 px-4 text-shadow-300">{ban.character}</td>
                  <td className="py-3 px-4 text-shadow-400 max-w-xs truncate">{ban.reason}</td>
                  <td className="py-3 px-4">
                    <span className={`px-2 py-1 rounded text-xs font-medium ${
                      ban.permanent ? 'bg-red-500/20 text-red-400' : 'bg-yellow-500/20 text-yellow-400'
                    }`}>
                      {ban.permanent ? 'Permanent' : 'Temporary'}
                    </span>
                  </td>
                  <td className="py-3 px-4 text-shadow-500 text-sm">{formatDate(ban.bannedAt)}</td>
                  <td className="py-3 px-4 text-shadow-500 text-sm">
                    {ban.expiresAt ? formatDate(ban.expiresAt) : 'Never'}
                  </td>
                  <td className="py-3 px-4">
                    {ban.appealStatus === 'pending' && (
                      <span className="px-2 py-1 rounded text-xs font-medium bg-yellow-500/20 text-yellow-400">Pending</span>
                    )}
                    {ban.appealStatus === 'approved' && (
                      <span className="px-2 py-1 rounded text-xs font-medium bg-green-500/20 text-green-400">Approved</span>
                    )}
                    {ban.appealStatus === 'denied' && (
                      <span className="px-2 py-1 rounded text-xs font-medium bg-red-500/20 text-red-400">Denied</span>
                    )}
                    {!ban.appealStatus && (
                      <span className="text-shadow-500 text-sm">None</span>
                    )}
                  </td>
                  <td className="py-3 px-4 text-right">
                    <button className="text-sm text-red-400 hover:text-red-300">Unban</button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </motion.div>
    </div>
  )
}

