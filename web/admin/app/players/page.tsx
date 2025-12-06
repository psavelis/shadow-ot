'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Search, 
  Filter, 
  MoreVertical, 
  Ban, 
  MessageSquare, 
  Eye,
  Shield,
  Wand2,
  Target,
  ChevronDown,
  AlertTriangle,
  Clock
} from 'lucide-react'
import * as DropdownMenu from '@radix-ui/react-dropdown-menu'
import * as Dialog from '@radix-ui/react-dialog'

const vocationIcons: Record<string, any> = {
  'Elite Knight': Shield,
  'Royal Paladin': Target,
  'Elder Druid': Wand2,
  'Master Sorcerer': Wand2,
}

const players = [
  { id: 1, name: 'ShadowKnight', account: 'shadowmaster', level: 312, vocation: 'Elite Knight', realm: 'Shadowveil', status: 'online', ip: '192.168.1.1', lastLogin: 'Now', warnings: 0 },
  { id: 2, name: 'DragonSlayer', account: 'dragonlord', level: 456, vocation: 'Royal Paladin', realm: 'Aetheria', status: 'online', ip: '192.168.1.2', lastLogin: 'Now', warnings: 1 },
  { id: 3, name: 'MysticMage', account: 'mysticone', level: 287, vocation: 'Master Sorcerer', realm: 'Warbound', status: 'offline', ip: '192.168.1.3', lastLogin: '2h ago', warnings: 0 },
  { id: 4, name: 'NatureDruid', account: 'naturelover', level: 198, vocation: 'Elder Druid', realm: 'Mythara', status: 'online', ip: '192.168.1.4', lastLogin: 'Now', warnings: 2 },
  { id: 5, name: 'IronWarrior', account: 'ironman', level: 523, vocation: 'Elite Knight', realm: 'Shadowveil', status: 'banned', ip: '192.168.1.5', lastLogin: '3d ago', warnings: 5 },
]

export default function PlayersPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedPlayer, setSelectedPlayer] = useState<typeof players[0] | null>(null)
  const [banDialogOpen, setBanDialogOpen] = useState(false)

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <h1 className="text-2xl font-display font-bold mb-1">Player Management</h1>
        <p className="text-shadow-400">Search, view, and manage player accounts</p>
      </motion.div>

      {/* Stats */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="grid sm:grid-cols-4 gap-4">
        <div className="stat-card">
          <p className="text-shadow-400 text-sm mb-1">Total Players</p>
          <p className="text-2xl font-bold text-white">52,847</p>
        </div>
        <div className="stat-card">
          <p className="text-shadow-400 text-sm mb-1">Online Now</p>
          <p className="text-2xl font-bold text-green-400">5,247</p>
        </div>
        <div className="stat-card">
          <p className="text-shadow-400 text-sm mb-1">Banned</p>
          <p className="text-2xl font-bold text-red-400">128</p>
        </div>
        <div className="stat-card">
          <p className="text-shadow-400 text-sm mb-1">New Today</p>
          <p className="text-2xl font-bold text-blue-400">234</p>
        </div>
      </motion.div>

      {/* Search & Filters */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="card">
        <div className="flex flex-col sm:flex-row gap-4">
          <div className="flex-1 relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
            <input
              type="text"
              placeholder="Search by name, account, or IP..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="input pl-10"
            />
          </div>
          <div className="flex space-x-2">
            <select className="input w-40">
              <option>All Realms</option>
              <option>Shadowveil</option>
              <option>Aetheria</option>
              <option>Warbound</option>
            </select>
            <select className="input w-40">
              <option>All Status</option>
              <option>Online</option>
              <option>Offline</option>
              <option>Banned</option>
            </select>
            <button className="btn-secondary flex items-center space-x-2">
              <Filter className="w-4 h-4" />
              <span>More Filters</span>
            </button>
          </div>
        </div>
      </motion.div>

      {/* Players Table */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.3 }} className="card">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-shadow-700">
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Character</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Account</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Level</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Realm</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Status</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">IP Address</th>
                <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Warnings</th>
                <th className="text-right py-3 px-4 text-shadow-400 font-medium text-sm">Actions</th>
              </tr>
            </thead>
            <tbody>
              {players.map((player) => {
                const VocIcon = vocationIcons[player.vocation] || Shield
                return (
                  <tr key={player.id} className="border-b border-shadow-800 last:border-0 hover:bg-shadow-800/50">
                    <td className="py-3 px-4">
                      <div className="flex items-center space-x-3">
                        <div className="w-8 h-8 bg-shadow-700 rounded-lg flex items-center justify-center">
                          <VocIcon className="w-4 h-4 text-accent-500" />
                        </div>
                        <div>
                          <p className="font-medium text-white">{player.name}</p>
                          <p className="text-xs text-shadow-500">{player.vocation}</p>
                        </div>
                      </div>
                    </td>
                    <td className="py-3 px-4 text-shadow-300">{player.account}</td>
                    <td className="py-3 px-4 text-white font-medium">{player.level}</td>
                    <td className="py-3 px-4 text-shadow-300">{player.realm}</td>
                    <td className="py-3 px-4">
                      <span className={`inline-flex items-center space-x-1.5 px-2 py-1 rounded-full text-xs font-medium ${
                        player.status === 'online' ? 'bg-green-500/20 text-green-400' :
                        player.status === 'banned' ? 'bg-red-500/20 text-red-400' :
                        'bg-shadow-700 text-shadow-400'
                      }`}>
                        <span className={`w-1.5 h-1.5 rounded-full ${
                          player.status === 'online' ? 'bg-green-400' :
                          player.status === 'banned' ? 'bg-red-400' : 'bg-shadow-500'
                        }`} />
                        <span className="capitalize">{player.status}</span>
                      </span>
                    </td>
                    <td className="py-3 px-4 font-mono text-sm text-shadow-400">{player.ip}</td>
                    <td className="py-3 px-4">
                      {player.warnings > 0 && (
                        <span className={`inline-flex items-center space-x-1 px-2 py-1 rounded-full text-xs ${
                          player.warnings >= 3 ? 'bg-red-500/20 text-red-400' : 'bg-yellow-500/20 text-yellow-400'
                        }`}>
                          <AlertTriangle className="w-3 h-3" />
                          <span>{player.warnings}</span>
                        </span>
                      )}
                    </td>
                    <td className="py-3 px-4 text-right">
                      <DropdownMenu.Root>
                        <DropdownMenu.Trigger asChild>
                          <button className="p-1.5 hover:bg-shadow-700 rounded transition-colors">
                            <MoreVertical className="w-4 h-4 text-shadow-400" />
                          </button>
                        </DropdownMenu.Trigger>
                        <DropdownMenu.Portal>
                          <DropdownMenu.Content className="w-48 bg-shadow-900 border border-shadow-700 rounded-xl shadow-xl overflow-hidden z-50" sideOffset={5}>
                            <DropdownMenu.Item className="flex items-center space-x-2 px-4 py-2.5 text-shadow-300 hover:text-white hover:bg-shadow-800 cursor-pointer focus:outline-none">
                              <Eye className="w-4 h-4" /><span>View Details</span>
                            </DropdownMenu.Item>
                            <DropdownMenu.Item className="flex items-center space-x-2 px-4 py-2.5 text-shadow-300 hover:text-white hover:bg-shadow-800 cursor-pointer focus:outline-none">
                              <MessageSquare className="w-4 h-4" /><span>Send Warning</span>
                            </DropdownMenu.Item>
                            <DropdownMenu.Separator className="h-px bg-shadow-700 my-1" />
                            <DropdownMenu.Item 
                              onClick={() => { setSelectedPlayer(player); setBanDialogOpen(true) }}
                              className="flex items-center space-x-2 px-4 py-2.5 text-red-400 hover:text-red-300 hover:bg-shadow-800 cursor-pointer focus:outline-none"
                            >
                              <Ban className="w-4 h-4" /><span>Ban Player</span>
                            </DropdownMenu.Item>
                          </DropdownMenu.Content>
                        </DropdownMenu.Portal>
                      </DropdownMenu.Root>
                    </td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        </div>

        {/* Pagination */}
        <div className="flex items-center justify-between mt-4 pt-4 border-t border-shadow-700">
          <p className="text-shadow-400 text-sm">Showing 1-5 of 52,847 players</p>
          <div className="flex space-x-2">
            <button className="btn-secondary text-sm py-1.5 px-3">Previous</button>
            <button className="btn-primary text-sm py-1.5 px-3">Next</button>
          </div>
        </div>
      </motion.div>

      {/* Ban Dialog */}
      <Dialog.Root open={banDialogOpen} onOpenChange={setBanDialogOpen}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/60 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-md bg-shadow-900 border border-shadow-700 rounded-2xl p-6 z-50">
            <Dialog.Title className="text-xl font-bold text-white mb-2">Ban Player</Dialog.Title>
            <Dialog.Description className="text-shadow-400 mb-6">
              Ban <span className="text-white font-medium">{selectedPlayer?.name}</span> ({selectedPlayer?.account})
            </Dialog.Description>
            <div className="space-y-4 mb-6">
              <div>
                <label className="block text-sm font-medium text-shadow-300 mb-2">Ban Duration</label>
                <select className="input">
                  <option>1 Day</option>
                  <option>3 Days</option>
                  <option>7 Days</option>
                  <option>30 Days</option>
                  <option>Permanent</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-shadow-300 mb-2">Reason</label>
                <textarea rows={3} className="input resize-none" placeholder="Enter ban reason..." />
              </div>
            </div>
            <div className="flex space-x-3">
              <Dialog.Close asChild>
                <button className="flex-1 btn-secondary">Cancel</button>
              </Dialog.Close>
              <button onClick={() => setBanDialogOpen(false)} className="flex-1 btn-danger">
                Ban Player
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}

