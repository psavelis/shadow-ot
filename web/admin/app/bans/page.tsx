'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Ban, Search, Filter, Clock, User, AlertTriangle, CheckCircle, XCircle, Plus,
  Loader2
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'
import { useBans, useBanPlayer, useUnbanPlayer } from '@/shared/hooks/useAdmin'
import type { BanRecord } from '@/shared/types'

function formatDate(date: string) {
  return new Date(date).toLocaleString('en-US', { 
    year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' 
  })
}

export default function BansPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [filterType, setFilterType] = useState<'all' | 'permanent' | 'temporary'>('all')
  const [showBanModal, setShowBanModal] = useState(false)
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 20
  
  // New ban form state
  const [banForm, setBanForm] = useState({
    accountId: '',
    characterId: '',
    reason: '',
    permanent: true,
    duration: 7,
  })
  
  // Hooks
  const { data: bansData, isLoading, error } = useBans({ page: currentPage, pageSize })
  const banPlayer = useBanPlayer()
  const unbanPlayer = useUnbanPlayer()

  const bans = bansData?.data || []

  const filteredBans = bans.filter(b => {
    const matchesSearch = b.accountId.toLowerCase().includes(searchQuery.toLowerCase()) ||
                          (b.characterId || '').toLowerCase().includes(searchQuery.toLowerCase())
    const matchesType = filterType === 'all' || 
                        (filterType === 'permanent' && b.permanent) ||
                        (filterType === 'temporary' && !b.permanent)
    return matchesSearch && matchesType
  })

  const handleBan = async () => {
    const success = await banPlayer.mutateAsync({
      accountId: banForm.accountId,
      characterId: banForm.characterId || undefined,
      reason: banForm.reason,
      duration: banForm.permanent ? undefined : banForm.duration * 24 * 60, // Convert days to minutes
    })
    if (success) {
      setShowBanModal(false)
      setBanForm({ accountId: '', characterId: '', reason: '', permanent: true, duration: 7 })
    }
  }

  const handleUnban = async (banId: string) => {
    await unbanPlayer.mutateAsync(banId)
  }

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-bold text-white mb-1">Ban Management</h1>
            <p className="text-slate-400">Manage player bans and appeals</p>
          </div>
          <button 
            onClick={() => setShowBanModal(true)} 
            className="flex items-center gap-2 px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition"
          >
            <Plus className="w-4 h-4" />
            Issue Ban
          </button>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.1 }} 
        className="grid sm:grid-cols-4 gap-4"
      >
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Ban className="w-5 h-5 text-red-400 mb-2" />
          <p className="text-2xl font-bold text-white">{bans.length}</p>
          <p className="text-slate-400 text-sm">Total Bans</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <AlertTriangle className="w-5 h-5 text-orange-400 mb-2" />
          <p className="text-2xl font-bold text-white">{bans.filter(b => b.permanent).length}</p>
          <p className="text-slate-400 text-sm">Permanent</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Clock className="w-5 h-5 text-blue-400 mb-2" />
          <p className="text-2xl font-bold text-white">{bans.filter(b => !b.permanent).length}</p>
          <p className="text-slate-400 text-sm">Temporary</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <CheckCircle className="w-5 h-5 text-amber-400 mb-2" />
          <p className="text-2xl font-bold text-white">{bans.filter(b => b.appealStatus === 'pending').length}</p>
          <p className="text-slate-400 text-sm">Pending Appeals</p>
        </div>
      </motion.div>

      {/* Filters */}
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
            placeholder="Search by account or character..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full bg-slate-900/50 border border-slate-700 rounded-lg pl-10 pr-4 py-2 text-white text-sm placeholder-slate-500 focus:outline-none focus:border-red-500/50"
          />
        </div>
        <select
          value={filterType}
          onChange={(e) => setFilterType(e.target.value as typeof filterType)}
          className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 text-white text-sm focus:outline-none focus:border-red-500/50"
        >
          <option value="all">All Bans</option>
          <option value="permanent">Permanent</option>
          <option value="temporary">Temporary</option>
        </select>
      </motion.div>

      {/* Bans Table */}
      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.3 }}
        className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
      >
        {isLoading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-red-400" />
          </div>
        ) : error ? (
          <div className="flex items-center justify-center py-12 text-red-400">
            <AlertTriangle className="w-5 h-5 mr-2" />
            Failed to load bans
          </div>
        ) : filteredBans.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 text-slate-400">
            <Ban className="w-12 h-12 mb-4 opacity-50" />
            <p>No bans found</p>
          </div>
        ) : (
          <table className="w-full">
            <thead className="bg-slate-900/50">
              <tr className="text-slate-400 text-sm">
                <th className="text-left p-4">Account</th>
                <th className="text-left p-4">Character</th>
                <th className="text-left p-4">Reason</th>
                <th className="text-left p-4">Banned By</th>
                <th className="text-left p-4">Date</th>
                <th className="text-left p-4">Type</th>
                <th className="text-left p-4">Appeal</th>
                <th className="text-left p-4">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-700/50">
              {filteredBans.map(ban => (
                <tr key={ban.id} className="hover:bg-slate-700/30 transition">
                  <td className="p-4 text-white font-medium">{ban.accountId}</td>
                  <td className="p-4 text-slate-300">{ban.characterId || '-'}</td>
                  <td className="p-4 text-slate-400 max-w-xs truncate">{ban.reason}</td>
                  <td className="p-4 text-slate-400">{ban.bannedBy}</td>
                  <td className="p-4 text-slate-400 text-sm">{formatDate(ban.bannedAt)}</td>
                  <td className="p-4">
                    <span className={`px-2 py-1 text-xs rounded ${ban.permanent ? 'bg-red-500/20 text-red-400' : 'bg-amber-500/20 text-amber-400'}`}>
                      {ban.permanent ? 'Permanent' : `Until ${formatDate(ban.expiresAt!)}`}
                    </span>
                  </td>
                  <td className="p-4">
                    {ban.appealStatus === 'pending' && (
                      <span className="px-2 py-1 bg-amber-500/20 text-amber-400 text-xs rounded">Pending</span>
                    )}
                    {ban.appealStatus === 'approved' && (
                      <span className="px-2 py-1 bg-emerald-500/20 text-emerald-400 text-xs rounded">Approved</span>
                    )}
                    {ban.appealStatus === 'denied' && (
                      <span className="px-2 py-1 bg-red-500/20 text-red-400 text-xs rounded">Denied</span>
                    )}
                    {!ban.appealStatus && (
                      <span className="text-slate-500 text-xs">None</span>
                    )}
                  </td>
                  <td className="p-4">
                    <button
                      onClick={() => handleUnban(ban.id)}
                      disabled={unbanPlayer.isPending}
                      className="px-3 py-1 bg-emerald-500/20 text-emerald-400 text-xs rounded hover:bg-emerald-500/30 transition disabled:opacity-50"
                    >
                      Unban
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}

        {/* Pagination */}
        {bansData && bansData.totalPages > 1 && (
          <div className="p-4 border-t border-slate-700 flex items-center justify-between">
            <p className="text-slate-500 text-sm">
              Showing {((currentPage - 1) * pageSize) + 1} to {Math.min(currentPage * pageSize, bansData.total)} of {bansData.total}
            </p>
            <div className="flex items-center gap-2">
              <button
                onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
                disabled={currentPage === 1}
                className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded hover:bg-slate-600/50 transition disabled:opacity-50"
              >
                Previous
              </button>
              <span className="text-slate-400 text-sm">
                Page {currentPage} of {bansData.totalPages}
              </span>
              <button
                onClick={() => setCurrentPage(p => Math.min(bansData.totalPages, p + 1))}
                disabled={currentPage === bansData.totalPages}
                className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded hover:bg-slate-600/50 transition disabled:opacity-50"
              >
                Next
              </button>
            </div>
          </div>
        )}
      </motion.div>

      {/* Ban Modal */}
      <Dialog.Root open={showBanModal} onOpenChange={setShowBanModal}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-md bg-slate-900 border border-slate-700 rounded-xl p-6 z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-4 flex items-center gap-2">
              <Ban className="w-6 h-6 text-red-400" />
              Issue Ban
            </Dialog.Title>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-slate-400 mb-2">Account ID *</label>
                <input
                  type="text"
                  value={banForm.accountId}
                  onChange={(e) => setBanForm(f => ({ ...f, accountId: e.target.value }))}
                  placeholder="Enter account ID"
                  className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-red-500/50"
                />
              </div>
              
              <div>
                <label className="block text-sm text-slate-400 mb-2">Character ID (optional)</label>
                <input
                  type="text"
                  value={banForm.characterId}
                  onChange={(e) => setBanForm(f => ({ ...f, characterId: e.target.value }))}
                  placeholder="Enter character ID for character-specific ban"
                  className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-red-500/50"
                />
              </div>
              
              <div>
                <label className="block text-sm text-slate-400 mb-2">Reason *</label>
                <textarea
                  value={banForm.reason}
                  onChange={(e) => setBanForm(f => ({ ...f, reason: e.target.value }))}
                  placeholder="Enter ban reason"
                  rows={3}
                  className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-red-500/50 resize-none"
                />
              </div>
              
              <div className="flex items-center gap-4">
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={banForm.permanent}
                    onChange={(e) => setBanForm(f => ({ ...f, permanent: e.target.checked }))}
                    className="w-4 h-4 rounded border-slate-600 bg-slate-800 text-red-500 focus:ring-red-500/50"
                  />
                  <span className="text-slate-300">Permanent ban</span>
                </label>
              </div>
              
              {!banForm.permanent && (
                <div>
                  <label className="block text-sm text-slate-400 mb-2">Duration (days)</label>
                  <input
                    type="number"
                    value={banForm.duration}
                    onChange={(e) => setBanForm(f => ({ ...f, duration: parseInt(e.target.value) || 1 }))}
                    min={1}
                    max={365}
                    className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-red-500/50"
                  />
                </div>
              )}
            </div>
            
            <div className="flex gap-3 mt-6">
              <Dialog.Close asChild>
                <button className="flex-1 px-4 py-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-600/50 transition">
                  Cancel
                </button>
              </Dialog.Close>
              <button
                onClick={handleBan}
                disabled={!banForm.accountId || !banForm.reason || banPlayer.isPending}
                className="flex-1 px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition disabled:opacity-50 flex items-center justify-center gap-2"
              >
                {banPlayer.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    Banning...
                  </>
                ) : (
                  'Issue Ban'
                )}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}
