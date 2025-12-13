'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Search, MoreVertical, Ban, MessageSquare, Eye,
  Shield, Wand2, Target, AlertTriangle, Loader2, Users, UserX
} from 'lucide-react'
import * as DropdownMenu from '@radix-ui/react-dropdown-menu'
import * as Dialog from '@radix-ui/react-dialog'
import { usePlayerSearch, useBanPlayer, useWarnPlayer, useOnlinePlayersAdmin, useAdminStats } from '@shadow-ot/shared'
import type { User, Character } from '@shadow-ot/shared'

const vocationIcons: Record<string, React.ElementType> = {
  'Elite Knight': Shield,
  'Royal Paladin': Target,
  'Elder Druid': Wand2,
  'Master Sorcerer': Wand2,
  'Knight': Shield,
  'Paladin': Target,
  'Druid': Wand2,
  'Sorcerer': Wand2,
}

export default function PlayersPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedPlayer, setSelectedPlayer] = useState<(User & { characters?: Character[] }) | null>(null)
  const [banDialogOpen, setBanDialogOpen] = useState(false)
  const [banReason, setBanReason] = useState('')
  const [banDuration, setBanDuration] = useState(7)
  const [isPermanent, setIsPermanent] = useState(false)
  
  // Real hooks
  const { data: searchResults = [], isLoading: isSearching } = usePlayerSearch(searchQuery)
  const { data: onlinePlayers = 0 } = useOnlinePlayersAdmin()
  const { data: adminStats } = useAdminStats()
  const banPlayer = useBanPlayer()
  const warnPlayer = useWarnPlayer()

  const handleBan = async () => {
    if (!selectedPlayer) return
    await banPlayer.mutateAsync({
      accountId: selectedPlayer.id,
      reason: banReason,
      duration: isPermanent ? undefined : banDuration * 24 * 60,
    })
    setBanDialogOpen(false)
    setBanReason('')
    setSelectedPlayer(null)
  }

  const handleWarn = async (player: User) => {
    const reason = prompt('Enter warning reason:')
    if (reason) {
      await warnPlayer.mutateAsync({ accountId: player.id, reason })
    }
  }

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <h1 className="text-2xl font-bold text-white mb-1">Player Management</h1>
        <p className="text-slate-400">Search, view, and manage player accounts</p>
      </motion.div>

      {/* Stats */}
      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.1 }} 
        className="grid sm:grid-cols-4 gap-4"
      >
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Users className="w-5 h-5 text-purple-400 mb-2" />
          <p className="text-2xl font-bold text-white">{adminStats?.totalAccounts?.toLocaleString() || '-'}</p>
          <p className="text-slate-400 text-sm">Total Players</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="w-3 h-3 rounded-full bg-green-500 mb-2 animate-pulse" />
          <p className="text-2xl font-bold text-green-400">{onlinePlayers.toLocaleString()}</p>
          <p className="text-slate-400 text-sm">Online Now</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Ban className="w-5 h-5 text-red-400 mb-2" />
          <p className="text-2xl font-bold text-white">{adminStats?.bannedAccounts || '-'}</p>
          <p className="text-slate-400 text-sm">Banned</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <AlertTriangle className="w-5 h-5 text-amber-400 mb-2" />
          <p className="text-2xl font-bold text-white">{adminStats?.pendingReports || '-'}</p>
          <p className="text-slate-400 text-sm">Reports</p>
        </div>
      </motion.div>

      {/* Search */}
      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.2 }}
        className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4"
      >
        <div className="relative">
          <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-500" />
          <input
            type="text"
            placeholder="Search players by name, account, or email..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full bg-slate-900/50 border border-slate-700 rounded-lg pl-12 pr-4 py-3 text-white placeholder-slate-500 focus:outline-none focus:border-purple-500/50"
          />
          {isSearching && (
            <Loader2 className="absolute right-4 top-1/2 -translate-y-1/2 w-5 h-5 text-purple-400 animate-spin" />
          )}
        </div>
      </motion.div>

      {/* Results */}
      <motion.div 
        initial={{ opacity: 0, y: 20 }} 
        animate={{ opacity: 1, y: 0 }} 
        transition={{ delay: 0.3 }}
        className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
      >
        {searchQuery.length < 2 ? (
          <div className="flex flex-col items-center justify-center py-12 text-slate-400">
            <Search className="w-12 h-12 mb-4 opacity-50" />
            <p>Enter at least 2 characters to search</p>
          </div>
        ) : isSearching ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-purple-400" />
          </div>
        ) : searchResults.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 text-slate-400">
            <UserX className="w-12 h-12 mb-4 opacity-50" />
            <p>No players found</p>
          </div>
        ) : (
          <table className="w-full">
            <thead className="bg-slate-900/50">
              <tr className="text-slate-400 text-sm">
                <th className="text-left p-4">Player</th>
                <th className="text-left p-4">Account</th>
                <th className="text-left p-4">Email</th>
                <th className="text-left p-4">Status</th>
                <th className="text-left p-4">Created</th>
                <th className="text-left p-4">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-slate-700/50">
              {searchResults.map((player: User) => (
                <tr key={player.id} className="hover:bg-slate-700/30 transition">
                  <td className="p-4">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center font-bold text-white">
                        {player.username?.charAt(0).toUpperCase() || '?'}
                      </div>
                      <div>
                        <p className="text-white font-medium">{player.username}</p>
                        <p className="text-slate-500 text-sm">{player.email}</p>
                      </div>
                    </div>
                  </td>
                  <td className="p-4 text-slate-400">{player.username}</td>
                  <td className="p-4 text-slate-400">{player.email}</td>
                  <td className="p-4">
                    <span className={`px-2 py-1 text-xs rounded ${
                      player.banned
                        ? 'bg-red-500/20 text-red-400'
                        : player.emailVerified
                          ? 'bg-green-500/20 text-green-400'
                          : 'bg-amber-500/20 text-amber-400'
                    }`}>
                      {player.banned ? 'Banned' : player.emailVerified ? 'Active' : 'Unverified'}
                    </span>
                  </td>
                  <td className="p-4 text-slate-500 text-sm">
                    {new Date(player.createdAt).toLocaleDateString()}
                  </td>
                  <td className="p-4">
                    <DropdownMenu.Root>
                      <DropdownMenu.Trigger asChild>
                        <button className="p-2 rounded-lg hover:bg-slate-700/50 transition">
                          <MoreVertical className="w-4 h-4 text-slate-400" />
                        </button>
                      </DropdownMenu.Trigger>
                      <DropdownMenu.Portal>
                        <DropdownMenu.Content
                          className="bg-slate-800 border border-slate-700 rounded-lg py-1 shadow-xl z-50 min-w-[160px]"
                          sideOffset={5}
                        >
                          <DropdownMenu.Item
                            className="flex items-center gap-2 px-3 py-2 text-sm text-slate-300 hover:bg-slate-700/50 cursor-pointer"
                            onSelect={() => setSelectedPlayer(player)}
                          >
                            <Eye className="w-4 h-4" />
                            View Details
                          </DropdownMenu.Item>
                          <DropdownMenu.Item
                            className="flex items-center gap-2 px-3 py-2 text-sm text-amber-400 hover:bg-slate-700/50 cursor-pointer"
                            onSelect={() => handleWarn(player)}
                          >
                            <MessageSquare className="w-4 h-4" />
                            Send Warning
                          </DropdownMenu.Item>
                          <DropdownMenu.Separator className="h-px bg-slate-700 my-1" />
                          <DropdownMenu.Item
                            className="flex items-center gap-2 px-3 py-2 text-sm text-red-400 hover:bg-slate-700/50 cursor-pointer"
                            onSelect={() => {
                              setSelectedPlayer(player)
                              setBanDialogOpen(true)
                            }}
                          >
                            <Ban className="w-4 h-4" />
                            Ban Player
                          </DropdownMenu.Item>
                        </DropdownMenu.Content>
                      </DropdownMenu.Portal>
                    </DropdownMenu.Root>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </motion.div>

      {/* Ban Dialog */}
      <Dialog.Root open={banDialogOpen} onOpenChange={setBanDialogOpen}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-md bg-slate-900 border border-slate-700 rounded-xl p-6 z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-4 flex items-center gap-2">
              <Ban className="w-6 h-6 text-red-400" />
              Ban Player
            </Dialog.Title>
            
            {selectedPlayer && (
              <div className="mb-4 p-3 bg-slate-800/50 rounded-lg">
                <p className="text-slate-400 text-sm">Banning account:</p>
                <p className="text-white font-medium">{selectedPlayer.username}</p>
              </div>
            )}
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-slate-400 mb-2">Reason</label>
                <textarea
                  value={banReason}
                  onChange={(e) => setBanReason(e.target.value)}
                  placeholder="Enter ban reason..."
                  rows={3}
                  className="w-full bg-slate-800/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-red-500/50 resize-none"
                />
              </div>
              
              <div className="flex items-center gap-4">
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={isPermanent}
                    onChange={(e) => setIsPermanent(e.target.checked)}
                    className="w-4 h-4 rounded border-slate-600 bg-slate-800 text-red-500 focus:ring-red-500/50"
                  />
                  <span className="text-slate-300">Permanent ban</span>
                </label>
              </div>
              
              {!isPermanent && (
                <div>
                  <label className="block text-sm text-slate-400 mb-2">Duration (days)</label>
                  <input
                    type="number"
                    value={banDuration}
                    onChange={(e) => setBanDuration(parseInt(e.target.value) || 1)}
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
                disabled={!banReason || banPlayer.isPending}
                className="flex-1 px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition disabled:opacity-50 flex items-center justify-center gap-2"
              >
                {banPlayer.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    Banning...
                  </>
                ) : (
                  'Ban Player'
                )}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}
