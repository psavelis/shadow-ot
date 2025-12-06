'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Search, 
  ChevronDown, 
  Crown, 
  Sword, 
  Shield, 
  Wand2, 
  Target,
  Trophy,
  Medal,
  TrendingUp,
  TrendingDown,
  Minus
} from 'lucide-react'
import * as Select from '@radix-ui/react-select'

const vocations = ['All', 'Knight', 'Paladin', 'Sorcerer', 'Druid']
const realms = ['All Realms', 'Shadowveil', 'Aetheria', 'Warbound', 'Mythara', 'Voidborne']
const categories = [
  { id: 'experience', name: 'Experience', icon: Trophy },
  { id: 'magic', name: 'Magic Level', icon: Wand2 },
  { id: 'fist', name: 'Fist Fighting', icon: Target },
  { id: 'club', name: 'Club Fighting', icon: Sword },
  { id: 'sword', name: 'Sword Fighting', icon: Sword },
  { id: 'axe', name: 'Axe Fighting', icon: Sword },
  { id: 'distance', name: 'Distance', icon: Target },
  { id: 'shielding', name: 'Shielding', icon: Shield },
  { id: 'achievements', name: 'Achievements', icon: Medal },
]

const vocationIcons: Record<string, typeof Sword> = {
  Knight: Shield,
  Paladin: Target,
  Sorcerer: Wand2,
  Druid: Wand2,
}

const vocationColors: Record<string, string> = {
  Knight: 'text-red-400',
  Paladin: 'text-yellow-400',
  Sorcerer: 'text-blue-400',
  Druid: 'text-green-400',
}

// Mock data
const generatePlayers = (count: number) => {
  const names = ['ShadowLord', 'DragonSlayer', 'MysticMage', 'IronKnight', 'SwiftArrow', 'StormBringer', 'DarkHunter', 'LightBringer', 'FrostWizard', 'FlameKnight']
  const vocationsArr = ['Knight', 'Paladin', 'Sorcerer', 'Druid']
  const realmsArr = ['Shadowveil', 'Aetheria', 'Warbound', 'Mythara']
  
  return Array.from({ length: count }, (_, i) => ({
    rank: i + 1,
    name: `${names[i % names.length]}${i > 9 ? i : ''}`,
    level: Math.floor(1500 - (i * 15) + Math.random() * 20),
    experience: Math.floor(500000000000 - (i * 5000000000) + Math.random() * 1000000000),
    vocation: vocationsArr[Math.floor(Math.random() * vocationsArr.length)],
    realm: realmsArr[Math.floor(Math.random() * realmsArr.length)],
    change: i < 3 ? 0 : Math.floor(Math.random() * 6) - 2,
    online: Math.random() > 0.7,
  }))
}

const players = generatePlayers(100)

function formatNumber(num: number): string {
  if (num >= 1e12) return `${(num / 1e12).toFixed(2)}T`
  if (num >= 1e9) return `${(num / 1e9).toFixed(2)}B`
  if (num >= 1e6) return `${(num / 1e6).toFixed(2)}M`
  if (num >= 1e3) return `${(num / 1e3).toFixed(2)}K`
  return num.toLocaleString()
}

export default function HighscoresPage() {
  const [selectedCategory, setSelectedCategory] = useState('experience')
  const [selectedVocation, setSelectedVocation] = useState('All')
  const [selectedRealm, setSelectedRealm] = useState('All Realms')
  const [searchQuery, setSearchQuery] = useState('')
  const [currentPage, setCurrentPage] = useState(1)
  const playersPerPage = 20

  const filteredPlayers = players.filter(player => {
    if (selectedVocation !== 'All' && player.vocation !== selectedVocation) return false
    if (selectedRealm !== 'All Realms' && player.realm !== selectedRealm) return false
    if (searchQuery && !player.name.toLowerCase().includes(searchQuery.toLowerCase())) return false
    return true
  })

  const paginatedPlayers = filteredPlayers.slice(
    (currentPage - 1) * playersPerPage,
    currentPage * playersPerPage
  )

  const totalPages = Math.ceil(filteredPlayers.length / playersPerPage)

  return (
    <div className="min-h-screen bg-shadow-950 pt-24 pb-16">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <h1 className="font-display text-4xl md:text-5xl font-bold mb-4">
            <span className="gradient-text">Highscores</span>
          </h1>
          <p className="text-shadow-300 text-lg max-w-2xl mx-auto">
            The most powerful warriors across all Shadow Realms. Will you rise to the top?
          </p>
        </motion.div>

        {/* Top 3 Showcase */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="grid md:grid-cols-3 gap-6 mb-12"
        >
          {players.slice(0, 3).map((player, index) => {
            const VocIcon = vocationIcons[player.vocation] || Sword
            const positions = [1, 0, 2]
            const realIndex = positions[index]
            const sizes = ['md:scale-110', 'md:scale-100', 'md:scale-100']
            const order = ['md:order-2', 'md:order-1', 'md:order-3']
            const crowns = ['text-yellow-400', 'text-gray-400', 'text-amber-600']
            
            return (
              <div 
                key={player.name}
                className={`${order[index]} ${sizes[index]} relative group`}
              >
                <div className={`relative bg-gradient-to-br ${
                  realIndex === 0 ? 'from-yellow-900/30 to-amber-900/30 border-yellow-500/30' :
                  realIndex === 1 ? 'from-gray-800/50 to-gray-900/50 border-gray-500/30' :
                  'from-amber-900/20 to-orange-900/20 border-amber-700/30'
                } rounded-2xl p-6 border transition-all duration-300 hover:scale-105`}>
                  {/* Crown/Medal */}
                  <div className="absolute -top-4 left-1/2 -translate-x-1/2">
                    <div className={`w-10 h-10 rounded-full flex items-center justify-center ${
                      realIndex === 0 ? 'bg-yellow-500/20' :
                      realIndex === 1 ? 'bg-gray-500/20' :
                      'bg-amber-600/20'
                    }`}>
                      {realIndex === 0 ? (
                        <Crown className={`w-6 h-6 ${crowns[realIndex]}`} />
                      ) : (
                        <Medal className={`w-6 h-6 ${crowns[realIndex]}`} />
                      )}
                    </div>
                  </div>

                  {/* Rank */}
                  <div className={`text-6xl font-display font-bold mb-4 text-center opacity-20 ${
                    realIndex === 0 ? 'text-yellow-400' :
                    realIndex === 1 ? 'text-gray-400' :
                    'text-amber-600'
                  }`}>
                    #{player.rank}
                  </div>

                  {/* Avatar */}
                  <div className="w-20 h-20 mx-auto mb-4 rounded-full bg-gradient-to-br from-accent-500/20 to-purple-500/20 border-2 border-accent-500/30 flex items-center justify-center">
                    <VocIcon className={`w-10 h-10 ${vocationColors[player.vocation]}`} />
                  </div>

                  {/* Info */}
                  <div className="text-center">
                    <div className="flex items-center justify-center space-x-2 mb-1">
                      <h3 className="font-display font-bold text-xl text-white">{player.name}</h3>
                      {player.online && (
                        <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
                      )}
                    </div>
                    <p className="text-shadow-400 text-sm mb-2">Level {player.level} {player.vocation}</p>
                    <p className="text-xs text-shadow-500 mb-4">{player.realm}</p>
                    <div className="text-2xl font-bold text-white">
                      {formatNumber(player.experience)}
                      <span className="text-shadow-500 text-sm font-normal ml-1">exp</span>
                    </div>
                  </div>
                </div>
              </div>
            )
          })}
        </motion.div>

        {/* Filters */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="bg-shadow-900/50 rounded-2xl border border-shadow-700 p-6 mb-8"
        >
          {/* Categories */}
          <div className="flex flex-wrap gap-2 mb-6">
            {categories.map((category) => (
              <button
                key={category.id}
                onClick={() => setSelectedCategory(category.id)}
                className={`flex items-center space-x-2 px-4 py-2 rounded-lg text-sm font-medium transition-all ${
                  selectedCategory === category.id
                    ? 'bg-accent-500 text-white'
                    : 'bg-shadow-800 text-shadow-300 hover:bg-shadow-700 hover:text-white'
                }`}
              >
                <category.icon className="w-4 h-4" />
                <span>{category.name}</span>
              </button>
            ))}
          </div>

          {/* Filters Row */}
          <div className="flex flex-col sm:flex-row gap-4">
            {/* Search */}
            <div className="flex-1 relative">
              <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
              <input
                type="text"
                placeholder="Search player name..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full pl-12 pr-4 py-2.5 bg-shadow-800 border border-shadow-600 rounded-xl text-white placeholder-shadow-500 focus:outline-none focus:border-accent-500 transition-all"
              />
            </div>

            {/* Vocation Filter */}
            <Select.Root value={selectedVocation} onValueChange={setSelectedVocation}>
              <Select.Trigger className="flex items-center justify-between space-x-2 px-4 py-2.5 bg-shadow-800 border border-shadow-600 rounded-xl text-white min-w-[140px] focus:outline-none focus:border-accent-500">
                <Select.Value />
                <ChevronDown className="w-4 h-4 text-shadow-400" />
              </Select.Trigger>
              <Select.Portal>
                <Select.Content className="bg-shadow-800 border border-shadow-600 rounded-xl shadow-xl overflow-hidden z-50">
                  <Select.Viewport>
                    {vocations.map((voc) => (
                      <Select.Item
                        key={voc}
                        value={voc}
                        className="px-4 py-2.5 text-white hover:bg-shadow-700 cursor-pointer focus:outline-none focus:bg-shadow-700"
                      >
                        <Select.ItemText>{voc}</Select.ItemText>
                      </Select.Item>
                    ))}
                  </Select.Viewport>
                </Select.Content>
              </Select.Portal>
            </Select.Root>

            {/* Realm Filter */}
            <Select.Root value={selectedRealm} onValueChange={setSelectedRealm}>
              <Select.Trigger className="flex items-center justify-between space-x-2 px-4 py-2.5 bg-shadow-800 border border-shadow-600 rounded-xl text-white min-w-[160px] focus:outline-none focus:border-accent-500">
                <Select.Value />
                <ChevronDown className="w-4 h-4 text-shadow-400" />
              </Select.Trigger>
              <Select.Portal>
                <Select.Content className="bg-shadow-800 border border-shadow-600 rounded-xl shadow-xl overflow-hidden z-50">
                  <Select.Viewport>
                    {realms.map((realm) => (
                      <Select.Item
                        key={realm}
                        value={realm}
                        className="px-4 py-2.5 text-white hover:bg-shadow-700 cursor-pointer focus:outline-none focus:bg-shadow-700"
                      >
                        <Select.ItemText>{realm}</Select.ItemText>
                      </Select.Item>
                    ))}
                  </Select.Viewport>
                </Select.Content>
              </Select.Portal>
            </Select.Root>
          </div>
        </motion.div>

        {/* Table */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
          className="bg-shadow-900/50 rounded-2xl border border-shadow-700 overflow-hidden"
        >
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-shadow-700">
                  <th className="text-left py-4 px-6 text-shadow-400 font-medium text-sm">Rank</th>
                  <th className="text-left py-4 px-6 text-shadow-400 font-medium text-sm">Player</th>
                  <th className="text-left py-4 px-6 text-shadow-400 font-medium text-sm">Vocation</th>
                  <th className="text-left py-4 px-6 text-shadow-400 font-medium text-sm">Realm</th>
                  <th className="text-right py-4 px-6 text-shadow-400 font-medium text-sm">Level</th>
                  <th className="text-right py-4 px-6 text-shadow-400 font-medium text-sm">Experience</th>
                </tr>
              </thead>
              <tbody>
                {paginatedPlayers.map((player) => {
                  const VocIcon = vocationIcons[player.vocation] || Sword
                  return (
                    <tr 
                      key={player.name}
                      className="border-b border-shadow-800 last:border-0 hover:bg-shadow-800/50 transition-colors"
                    >
                      <td className="py-4 px-6">
                        <div className="flex items-center space-x-2">
                          <span className={`font-bold ${
                            player.rank === 1 ? 'text-yellow-400' :
                            player.rank === 2 ? 'text-gray-400' :
                            player.rank === 3 ? 'text-amber-600' :
                            'text-white'
                          }`}>
                            #{player.rank}
                          </span>
                          {player.change !== 0 && (
                            <span className={`flex items-center text-xs ${
                              player.change > 0 ? 'text-green-500' : 'text-red-500'
                            }`}>
                              {player.change > 0 ? (
                                <TrendingUp className="w-3 h-3" />
                              ) : (
                                <TrendingDown className="w-3 h-3" />
                              )}
                              <span className="ml-0.5">{Math.abs(player.change)}</span>
                            </span>
                          )}
                        </div>
                      </td>
                      <td className="py-4 px-6">
                        <div className="flex items-center space-x-3">
                          <div className="w-10 h-10 rounded-lg bg-shadow-800 flex items-center justify-center">
                            <VocIcon className={`w-5 h-5 ${vocationColors[player.vocation]}`} />
                          </div>
                          <div>
                            <div className="flex items-center space-x-2">
                              <span className="font-medium text-white">{player.name}</span>
                              {player.online && (
                                <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
                              )}
                            </div>
                          </div>
                        </div>
                      </td>
                      <td className="py-4 px-6">
                        <span className={`${vocationColors[player.vocation]}`}>{player.vocation}</span>
                      </td>
                      <td className="py-4 px-6">
                        <span className="text-shadow-300">{player.realm}</span>
                      </td>
                      <td className="py-4 px-6 text-right">
                        <span className="font-bold text-white">{player.level}</span>
                      </td>
                      <td className="py-4 px-6 text-right">
                        <span className="text-shadow-300">{formatNumber(player.experience)}</span>
                      </td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
          </div>

          {/* Pagination */}
          <div className="flex items-center justify-between px-6 py-4 border-t border-shadow-700">
            <p className="text-shadow-400 text-sm">
              Showing {((currentPage - 1) * playersPerPage) + 1} to {Math.min(currentPage * playersPerPage, filteredPlayers.length)} of {filteredPlayers.length} players
            </p>
            <div className="flex items-center space-x-2">
              <button
                onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
                disabled={currentPage === 1}
                className="px-3 py-1.5 bg-shadow-800 rounded-lg text-white text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-shadow-700 transition-colors"
              >
                Previous
              </button>
              {Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
                const page = currentPage <= 3 ? i + 1 : currentPage - 2 + i
                if (page > totalPages) return null
                return (
                  <button
                    key={page}
                    onClick={() => setCurrentPage(page)}
                    className={`w-8 h-8 rounded-lg text-sm font-medium transition-colors ${
                      currentPage === page
                        ? 'bg-accent-500 text-white'
                        : 'bg-shadow-800 text-shadow-300 hover:bg-shadow-700'
                    }`}
                  >
                    {page}
                  </button>
                )
              })}
              <button
                onClick={() => setCurrentPage(p => Math.min(totalPages, p + 1))}
                disabled={currentPage === totalPages}
                className="px-3 py-1.5 bg-shadow-800 rounded-lg text-white text-sm disabled:opacity-50 disabled:cursor-not-allowed hover:bg-shadow-700 transition-colors"
              >
                Next
              </button>
            </div>
          </div>
        </motion.div>
      </div>
    </div>
  )
}

