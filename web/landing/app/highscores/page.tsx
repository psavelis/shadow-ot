'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Search, 
  Crown, 
  Sword, 
  Shield, 
  Wand2, 
  Target,
  Trophy,
  Medal,
  TrendingUp,
  TrendingDown,
  Minus,
  Loader2,
  AlertCircle
} from 'lucide-react'
import { useHighscores, HIGHSCORE_CATEGORIES } from '@/shared/hooks/useHighscores'
import { getOutfitSprite, getVocationIcon } from '@/shared/utils/assets'
import type { HighscoreCategory, RealmId, Vocation } from '@/shared/types'

const vocations: (Vocation | 'All')[] = ['All' as any, 'Knight', 'Elite Knight', 'Paladin', 'Royal Paladin', 'Sorcerer', 'Master Sorcerer', 'Druid', 'Elder Druid']
const realms: (RealmId | 'all')[] = ['all' as any, 'shadowveil', 'aetheria', 'warbound', 'mythara', 'voidborne', 'grimhollow']

const categoryIcons: Record<string, React.ComponentType<{ className?: string }>> = {
  experience: Trophy,
  magic: Wand2,
  fist: Target,
  club: Sword,
  sword: Sword,
  axe: Sword,
  distance: Target,
  shielding: Shield,
  fishing: Target,
  achievements: Medal,
  loyalty: Crown,
  charm: Medal,
}

const vocationColors: Record<string, string> = {
  'Knight': 'text-red-400',
  'Elite Knight': 'text-red-400',
  'Paladin': 'text-yellow-400',
  'Royal Paladin': 'text-yellow-400',
  'Sorcerer': 'text-blue-400',
  'Master Sorcerer': 'text-blue-400',
  'Druid': 'text-green-400',
  'Elder Druid': 'text-green-400',
  'None': 'text-slate-400',
}

function formatNumber(num: number): string {
  if (num >= 1e12) return `${(num / 1e12).toFixed(2)}T`
  if (num >= 1e9) return `${(num / 1e9).toFixed(2)}B`
  if (num >= 1e6) return `${(num / 1e6).toFixed(2)}M`
  if (num >= 1e3) return `${(num / 1e3).toFixed(2)}K`
  return num.toLocaleString()
}

export default function HighscoresPage() {
  const [selectedCategory, setSelectedCategory] = useState<HighscoreCategory>('experience')
  const [selectedVocation, setSelectedVocation] = useState<Vocation | 'All'>('All' as any)
  const [selectedRealm, setSelectedRealm] = useState<RealmId | 'all'>('all' as any)
  const [searchQuery, setSearchQuery] = useState('')
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 50

  // Fetch highscores from API
  const { data, isLoading, error } = useHighscores(
    {
      category: selectedCategory,
      realm: selectedRealm === 'all' ? undefined : selectedRealm,
      vocation: selectedVocation === 'All' ? undefined : selectedVocation,
    },
    currentPage,
    pageSize
  )

  // Filter by search query client-side
  const filteredEntries = data?.data?.filter(entry => 
    !searchQuery || entry.character.name.toLowerCase().includes(searchQuery.toLowerCase())
  ) || []

  const CategoryIcon = categoryIcons[selectedCategory] || Trophy

  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 py-20">
      <div className="container mx-auto px-4">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-amber-500/10 border border-amber-500/30 rounded-full text-amber-400 text-sm mb-6">
            <Trophy className="w-4 h-4" />
            Rankings & Leaderboards
          </div>
          <h1 className="text-4xl md:text-5xl font-bold text-white mb-4">
            <span className="text-transparent bg-clip-text bg-gradient-to-r from-amber-400 to-yellow-500">Highscores</span>
          </h1>
          <p className="text-slate-400 text-lg max-w-2xl mx-auto">
            View the top players across all realms and categories
          </p>
        </motion.div>

        {/* Categories */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="flex flex-wrap justify-center gap-2 mb-8"
        >
          {HIGHSCORE_CATEGORIES.map((cat) => {
            const Icon = categoryIcons[cat.value] || Trophy
            const isActive = selectedCategory === cat.value
            return (
              <button
                key={cat.value}
                onClick={() => {
                  setSelectedCategory(cat.value)
                  setCurrentPage(1)
                }}
                className={`flex items-center gap-2 px-4 py-2 rounded-xl border transition ${
                  isActive
                    ? 'bg-amber-500/20 border-amber-500/50 text-amber-400'
                    : 'bg-slate-800/50 border-slate-700/50 text-slate-400 hover:border-slate-600'
                }`}
              >
                <Icon className="w-4 h-4" />
                <span className="text-sm font-medium">{cat.label}</span>
              </button>
            )
          })}
        </motion.div>

        {/* Filters */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="flex flex-wrap items-center gap-4 bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 mb-8"
        >
          {/* Search */}
          <div className="relative flex-1 min-w-[200px]">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
            <input
              type="text"
              placeholder="Search player..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-slate-900/50 border border-slate-700 rounded-lg pl-10 pr-4 py-2 text-white text-sm placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
            />
          </div>

          {/* Vocation Filter */}
          <select
            value={selectedVocation}
            onChange={(e) => {
              setSelectedVocation(e.target.value as Vocation | 'All')
              setCurrentPage(1)
            }}
            className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-amber-500/50"
          >
            <option value="All">All Vocations</option>
            <option value="Knight">Knight</option>
            <option value="Elite Knight">Elite Knight</option>
            <option value="Paladin">Paladin</option>
            <option value="Royal Paladin">Royal Paladin</option>
            <option value="Sorcerer">Sorcerer</option>
            <option value="Master Sorcerer">Master Sorcerer</option>
            <option value="Druid">Druid</option>
            <option value="Elder Druid">Elder Druid</option>
          </select>

          {/* Realm Filter */}
          <select
            value={selectedRealm}
            onChange={(e) => {
              setSelectedRealm(e.target.value as RealmId | 'all')
              setCurrentPage(1)
            }}
            className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-amber-500/50"
          >
            <option value="all">All Realms</option>
            <option value="shadowveil">Shadowveil</option>
            <option value="aetheria">Aetheria</option>
            <option value="warbound">Warbound</option>
            <option value="mythara">Mythara</option>
            <option value="voidborne">Voidborne</option>
            <option value="grimhollow">Grimhollow</option>
          </select>
        </motion.div>

        {/* Highscores Table */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
          className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
        >
          {isLoading ? (
            <div className="flex items-center justify-center py-20">
              <Loader2 className="w-12 h-12 animate-spin text-amber-400" />
            </div>
          ) : error ? (
            <div className="flex flex-col items-center justify-center py-20 text-red-400">
              <AlertCircle className="w-12 h-12 mb-4" />
              <p>Failed to load highscores. Please try again.</p>
            </div>
          ) : filteredEntries.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-20 text-slate-400">
              <Trophy className="w-12 h-12 mb-4 opacity-50" />
              <p>No players found</p>
            </div>
          ) : (
            <>
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead className="bg-slate-900/50">
                    <tr className="text-slate-400 text-sm">
                      <th className="text-left p-4 w-16">#</th>
                      <th className="text-left p-4">Player</th>
                      <th className="text-left p-4">Vocation</th>
                      <th className="text-right p-4">Level</th>
                      <th className="text-right p-4">
                        {HIGHSCORE_CATEGORIES.find(c => c.value === selectedCategory)?.label || 'Value'}
                      </th>
                      <th className="text-left p-4">Realm</th>
                      <th className="text-center p-4">Status</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-700/50">
                    {filteredEntries.map((entry) => (
                      <tr key={entry.character.id} className="hover:bg-slate-700/30 transition">
                        <td className="p-4">
                          {entry.rank <= 3 ? (
                            <div className={`w-8 h-8 rounded-lg flex items-center justify-center font-bold ${
                              entry.rank === 1 ? 'bg-amber-500/20 text-amber-400' :
                              entry.rank === 2 ? 'bg-slate-400/20 text-slate-300' :
                              'bg-orange-500/20 text-orange-400'
                            }`}>
                              {entry.rank}
                            </div>
                          ) : (
                            <span className="text-slate-500 font-medium pl-2">{entry.rank}</span>
                          )}
                        </td>
                        <td className="p-4">
                          <div className="flex items-center gap-3">
                            <div className="w-8 h-8 bg-slate-700/50 rounded-lg overflow-hidden">
                              {/* Outfit would go here */}
                            </div>
                            <span className="text-white font-medium hover:text-amber-400 cursor-pointer transition">
                              {entry.character.name}
                            </span>
                          </div>
                        </td>
                        <td className="p-4">
                          <span className={`text-sm ${vocationColors[entry.character.vocation] || 'text-slate-400'}`}>
                            {entry.character.vocation}
                          </span>
                        </td>
                        <td className="p-4 text-right">
                          <span className="text-white font-medium">{entry.character.level}</span>
                        </td>
                        <td className="p-4 text-right">
                          <span className="text-amber-400 font-bold">{formatNumber(entry.value)}</span>
                        </td>
                        <td className="p-4">
                          <span className="px-2 py-1 bg-slate-700/50 text-slate-300 text-xs rounded capitalize">
                            {entry.character.realm}
                          </span>
                        </td>
                        <td className="p-4 text-center">
                          {entry.character.online ? (
                            <span className="flex items-center justify-center gap-1 text-emerald-400 text-xs">
                              <span className="w-2 h-2 bg-emerald-400 rounded-full animate-pulse" />
                              Online
                            </span>
                          ) : (
                            <span className="text-slate-500 text-xs">Offline</span>
                          )}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>

              {/* Pagination */}
              {data && data.totalPages > 1 && (
                <div className="p-4 border-t border-slate-700 flex items-center justify-between">
                  <p className="text-slate-500 text-sm">
                    Showing {((currentPage - 1) * pageSize) + 1} to {Math.min(currentPage * pageSize, data.total)} of {data.total} players
                  </p>
                  <div className="flex items-center gap-2">
                    <button
                      onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
                      disabled={currentPage === 1}
                      className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-600/50 transition disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      Previous
                    </button>
                    <span className="text-slate-400 text-sm">
                      Page {currentPage} of {data.totalPages}
                    </span>
                    <button
                      onClick={() => setCurrentPage(p => Math.min(data.totalPages, p + 1))}
                      disabled={currentPage === data.totalPages}
                      className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-600/50 transition disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      Next
                    </button>
                  </div>
                </div>
              )}
            </>
          )}
        </motion.div>
      </div>
    </main>
  )
}
