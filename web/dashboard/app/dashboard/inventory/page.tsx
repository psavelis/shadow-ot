'use client'

import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import {
  Package, Search, Filter, Grid3X3, List, Sword, Shield, Wand2,
  Heart, Star, Sparkles, ChevronDown, X, Eye, Send, Tag,
  ArrowUpDown, Layers, Box
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'
import * as Dialog from '@radix-ui/react-dialog'
import * as Tooltip from '@radix-ui/react-tooltip'

const categories = [
  { id: 'all', name: 'All Items', count: 247 },
  { id: 'weapons', name: 'Weapons', count: 34, icon: Sword },
  { id: 'armor', name: 'Armor', count: 28, icon: Shield },
  { id: 'helmets', name: 'Helmets', count: 15, icon: Shield },
  { id: 'legs', name: 'Legs', count: 12, icon: Shield },
  { id: 'boots', name: 'Boots', count: 18, icon: Shield },
  { id: 'shields', name: 'Shields', count: 8, icon: Shield },
  { id: 'rings', name: 'Rings', count: 23, icon: Sparkles },
  { id: 'amulets', name: 'Amulets', count: 19, icon: Sparkles },
  { id: 'runes', name: 'Runes', count: 45, icon: Wand2 },
  { id: 'consumables', name: 'Consumables', count: 32, icon: Heart },
  { id: 'misc', name: 'Misc', count: 13, icon: Box },
]

const items = [
  {
    id: 1,
    name: 'Demon Armor',
    type: 'armor',
    rarity: 'legendary',
    level: 130,
    equipped: true,
    character: 'ShadowKnight',
    attributes: { armor: 16, protection: 'physical +5%' },
    count: 1,
    value: 50000000,
    mintable: true,
    image: '🛡️',
  },
  {
    id: 2,
    name: 'Magic Sword',
    type: 'weapons',
    rarity: 'epic',
    level: 80,
    equipped: true,
    character: 'ShadowKnight',
    attributes: { attack: 51, element: 'fire' },
    count: 1,
    value: 8500000,
    mintable: true,
    image: '⚔️',
  },
  {
    id: 3,
    name: 'Golden Helmet',
    type: 'helmets',
    rarity: 'rare',
    level: 75,
    equipped: false,
    character: null,
    attributes: { armor: 12 },
    count: 1,
    value: 3200000,
    mintable: true,
    image: '👑',
  },
  {
    id: 4,
    name: 'Great Mana Potion',
    type: 'consumables',
    rarity: 'common',
    level: 80,
    equipped: false,
    character: null,
    attributes: { mana: 500 },
    count: 847,
    value: 150,
    mintable: false,
    image: '🧪',
  },
  {
    id: 5,
    name: 'Sudden Death',
    type: 'runes',
    rarity: 'uncommon',
    level: 45,
    equipped: false,
    character: null,
    attributes: { damage: '180-220' },
    count: 234,
    value: 800,
    mintable: false,
    image: '💀',
  },
  {
    id: 6,
    name: 'Boots of Haste',
    type: 'boots',
    rarity: 'rare',
    level: 0,
    equipped: true,
    character: 'MysticDruid',
    attributes: { speed: '+20' },
    count: 1,
    value: 2500000,
    mintable: true,
    image: '👟',
  },
  {
    id: 7,
    name: 'Ring of Healing',
    type: 'rings',
    rarity: 'uncommon',
    level: 0,
    equipped: false,
    character: null,
    attributes: { healing: '+5%' },
    count: 3,
    value: 450000,
    mintable: false,
    image: '💍',
  },
  {
    id: 8,
    name: 'Dragon Scale Legs',
    type: 'legs',
    rarity: 'epic',
    level: 100,
    equipped: true,
    character: 'ShadowKnight',
    attributes: { armor: 14, protection: 'fire +8%' },
    count: 1,
    value: 12000000,
    mintable: true,
    image: '🦿',
  },
]

const rarityColors: Record<string, { bg: string; text: string; border: string }> = {
  common: { bg: 'bg-slate-500/20', text: 'text-slate-400', border: 'border-slate-500/30' },
  uncommon: { bg: 'bg-green-500/20', text: 'text-green-400', border: 'border-green-500/30' },
  rare: { bg: 'bg-blue-500/20', text: 'text-blue-400', border: 'border-blue-500/30' },
  epic: { bg: 'bg-purple-500/20', text: 'text-purple-400', border: 'border-purple-500/30' },
  legendary: { bg: 'bg-orange-500/20', text: 'text-orange-400', border: 'border-orange-500/30' },
}

export default function InventoryPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedCategory, setSelectedCategory] = useState('all')
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid')
  const [sortBy, setSortBy] = useState('name')
  const [selectedItem, setSelectedItem] = useState<typeof items[0] | null>(null)
  const [showFilters, setShowFilters] = useState(false)
  const [rarityFilter, setRarityFilter] = useState<string[]>([])

  const filteredItems = items.filter(item => {
    const matchesSearch = item.name.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesCategory = selectedCategory === 'all' || item.type === selectedCategory
    const matchesRarity = rarityFilter.length === 0 || rarityFilter.includes(item.rarity)
    return matchesSearch && matchesCategory && matchesRarity
  })

  const formatGold = (n: number) => {
    if (n >= 1000000) return `${(n / 1000000).toFixed(2)}kk`
    if (n >= 1000) return `${(n / 1000).toFixed(1)}k`
    return n.toLocaleString()
  }

  const totalValue = filteredItems.reduce((sum, item) => sum + (item.value * item.count), 0)

  return (
    <Tooltip.Provider delayDuration={200}>
      <div className="space-y-6">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex items-center justify-between"
        >
          <div>
            <h1 className="text-2xl font-bold text-white mb-1">Inventory</h1>
            <p className="text-slate-400">Manage items across all your characters</p>
          </div>
          <div className="flex items-center gap-3">
            <div className="text-right">
              <p className="text-slate-500 text-xs">Total Value</p>
              <p className="text-xl font-bold text-amber-400">{formatGold(totalValue)}</p>
            </div>
          </div>
        </motion.div>

        {/* Stats */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="grid grid-cols-4 gap-4"
        >
          {[
            { label: 'Total Items', value: items.reduce((s, i) => s + i.count, 0).toLocaleString(), icon: Package, color: 'amber' },
            { label: 'Unique Items', value: items.length.toString(), icon: Layers, color: 'blue' },
            { label: 'Equipped', value: items.filter(i => i.equipped).length.toString(), icon: Shield, color: 'emerald' },
            { label: 'Mintable NFTs', value: items.filter(i => i.mintable).length.toString(), icon: Sparkles, color: 'purple' },
          ].map((stat, idx) => (
            <div key={idx} className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
              <div className="flex items-center justify-between mb-2">
                <stat.icon className={`w-5 h-5 text-${stat.color}-400`} />
              </div>
              <p className="text-2xl font-bold text-white">{stat.value}</p>
              <p className="text-xs text-slate-500">{stat.label}</p>
            </div>
          ))}
        </motion.div>

        {/* Filters & Search */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="flex flex-wrap gap-4"
        >
          {/* Search */}
          <div className="flex-1 min-w-[250px] relative">
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
            <input
              type="text"
              placeholder="Search items..."
              value={searchQuery}
              onChange={e => setSearchQuery(e.target.value)}
              className="w-full bg-slate-800/50 border border-slate-700/50 rounded-lg pl-10 pr-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
            />
          </div>

          {/* Category Filter */}
          <select
            value={selectedCategory}
            onChange={e => setSelectedCategory(e.target.value)}
            className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
          >
            {categories.map(cat => (
              <option key={cat.id} value={cat.id}>{cat.name} ({cat.count})</option>
            ))}
          </select>

          {/* Sort */}
          <select
            value={sortBy}
            onChange={e => setSortBy(e.target.value)}
            className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
          >
            <option value="name">Sort by Name</option>
            <option value="value">Sort by Value</option>
            <option value="rarity">Sort by Rarity</option>
            <option value="level">Sort by Level</option>
          </select>

          {/* View Toggle */}
          <div className="flex rounded-lg overflow-hidden border border-slate-700/50">
            <button
              onClick={() => setViewMode('grid')}
              className={`p-2 ${viewMode === 'grid' ? 'bg-amber-500 text-white' : 'bg-slate-800/50 text-slate-400'}`}
            >
              <Grid3X3 className="w-5 h-5" />
            </button>
            <button
              onClick={() => setViewMode('list')}
              className={`p-2 ${viewMode === 'list' ? 'bg-amber-500 text-white' : 'bg-slate-800/50 text-slate-400'}`}
            >
              <List className="w-5 h-5" />
            </button>
          </div>
        </motion.div>

        {/* Rarity Filter Pills */}
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.25 }}
          className="flex gap-2"
        >
          {Object.entries(rarityColors).map(([rarity, colors]) => (
            <button
              key={rarity}
              onClick={() => {
                setRarityFilter(prev => 
                  prev.includes(rarity) 
                    ? prev.filter(r => r !== rarity)
                    : [...prev, rarity]
                )
              }}
              className={`px-3 py-1 rounded-full text-xs font-medium capitalize transition ${
                rarityFilter.includes(rarity)
                  ? `${colors.bg} ${colors.text} border ${colors.border}`
                  : 'bg-slate-800/50 text-slate-500 hover:text-slate-300'
              }`}
            >
              {rarity}
            </button>
          ))}
          {rarityFilter.length > 0 && (
            <button
              onClick={() => setRarityFilter([])}
              className="px-3 py-1 text-slate-500 hover:text-slate-300 text-xs"
            >
              Clear filters
            </button>
          )}
        </motion.div>

        {/* Items Grid/List */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
        >
          {viewMode === 'grid' ? (
            <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-3">
              {filteredItems.map((item, idx) => (
                <Tooltip.Root key={item.id}>
                  <Tooltip.Trigger asChild>
                    <motion.button
                      initial={{ opacity: 0, scale: 0.9 }}
                      animate={{ opacity: 1, scale: 1 }}
                      transition={{ delay: idx * 0.02 }}
                      onClick={() => setSelectedItem(item)}
                      className={`relative bg-slate-800/50 border rounded-xl p-4 text-center hover:border-amber-500/50 transition group ${rarityColors[item.rarity].border}`}
                    >
                      {item.equipped && (
                        <div className="absolute top-2 right-2 w-2 h-2 bg-emerald-500 rounded-full" />
                      )}
                      {item.mintable && (
                        <Sparkles className="absolute top-2 left-2 w-3 h-3 text-purple-400" />
                      )}
                      <div className="text-4xl mb-2">{item.image}</div>
                      <p className="text-white text-sm font-medium truncate">{item.name}</p>
                      <p className="text-slate-500 text-xs">
                        {item.count > 1 ? `x${item.count}` : `Lvl ${item.level}`}
                      </p>
                      <p className={`text-xs mt-1 ${rarityColors[item.rarity].text}`}>
                        {formatGold(item.value * item.count)}
                      </p>
                    </motion.button>
                  </Tooltip.Trigger>
                  <Tooltip.Portal>
                    <Tooltip.Content
                      className="bg-slate-900 border border-slate-700 rounded-lg p-3 shadow-xl z-50"
                      side="top"
                    >
                      <p className="text-white font-medium mb-1">{item.name}</p>
                      <p className={`text-xs capitalize ${rarityColors[item.rarity].text}`}>{item.rarity}</p>
                      {Object.entries(item.attributes).map(([key, val]) => (
                        <p key={key} className="text-slate-400 text-xs capitalize">
                          {key}: {val}
                        </p>
                      ))}
                      <Tooltip.Arrow className="fill-slate-900" />
                    </Tooltip.Content>
                  </Tooltip.Portal>
                </Tooltip.Root>
              ))}
            </div>
          ) : (
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-slate-700/50">
                    <th className="text-left py-3 px-4 text-slate-500 font-medium text-xs">Item</th>
                    <th className="text-left py-3 px-4 text-slate-500 font-medium text-xs">Rarity</th>
                    <th className="text-right py-3 px-4 text-slate-500 font-medium text-xs">Count</th>
                    <th className="text-right py-3 px-4 text-slate-500 font-medium text-xs">Value</th>
                    <th className="text-left py-3 px-4 text-slate-500 font-medium text-xs">Location</th>
                    <th className="text-right py-3 px-4 text-slate-500 font-medium text-xs"></th>
                  </tr>
                </thead>
                <tbody>
                  {filteredItems.map((item, idx) => (
                    <motion.tr
                      key={item.id}
                      initial={{ opacity: 0 }}
                      animate={{ opacity: 1 }}
                      transition={{ delay: idx * 0.02 }}
                      className="border-b border-slate-700/50 last:border-0 hover:bg-slate-700/30 transition"
                    >
                      <td className="py-3 px-4">
                        <div className="flex items-center gap-3">
                          <span className="text-2xl">{item.image}</span>
                          <div>
                            <p className="text-white font-medium">{item.name}</p>
                            <p className="text-slate-500 text-xs">Level {item.level}</p>
                          </div>
                        </div>
                      </td>
                      <td className="py-3 px-4">
                        <span className={`px-2 py-0.5 rounded text-xs capitalize ${rarityColors[item.rarity].bg} ${rarityColors[item.rarity].text}`}>
                          {item.rarity}
                        </span>
                      </td>
                      <td className="py-3 px-4 text-right text-white">{item.count.toLocaleString()}</td>
                      <td className="py-3 px-4 text-right text-amber-400">{formatGold(item.value * item.count)}</td>
                      <td className="py-3 px-4">
                        {item.equipped ? (
                          <span className="text-emerald-400 text-xs flex items-center gap-1">
                            <Shield className="w-3 h-3" />
                            {item.character}
                          </span>
                        ) : (
                          <span className="text-slate-500 text-xs">Depot</span>
                        )}
                      </td>
                      <td className="py-3 px-4 text-right">
                        <button
                          onClick={() => setSelectedItem(item)}
                          className="p-1 text-slate-500 hover:text-white transition"
                        >
                          <Eye className="w-4 h-4" />
                        </button>
                      </td>
                    </motion.tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </motion.div>

        {/* Item Detail Dialog */}
        <Dialog.Root open={!!selectedItem} onOpenChange={() => setSelectedItem(null)}>
          <Dialog.Portal>
            <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm" />
            <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md">
              {selectedItem && (
                <>
                  <div className="text-center mb-6">
                    <div className="text-6xl mb-4">{selectedItem.image}</div>
                    <Dialog.Title className="text-xl font-bold text-white">{selectedItem.name}</Dialog.Title>
                    <span className={`px-3 py-1 rounded-full text-xs capitalize ${rarityColors[selectedItem.rarity].bg} ${rarityColors[selectedItem.rarity].text}`}>
                      {selectedItem.rarity}
                    </span>
                  </div>

                  <div className="space-y-3 mb-6">
                    <div className="bg-slate-900/50 rounded-lg p-4">
                      <p className="text-slate-500 text-xs mb-2">Attributes</p>
                      {Object.entries(selectedItem.attributes).map(([key, val]) => (
                        <div key={key} className="flex justify-between text-sm">
                          <span className="text-slate-400 capitalize">{key}</span>
                          <span className="text-white">{val}</span>
                        </div>
                      ))}
                    </div>

                    <div className="grid grid-cols-2 gap-3">
                      <div className="bg-slate-900/50 rounded-lg p-3">
                        <p className="text-slate-500 text-xs">Quantity</p>
                        <p className="text-white font-bold">{selectedItem.count.toLocaleString()}</p>
                      </div>
                      <div className="bg-slate-900/50 rounded-lg p-3">
                        <p className="text-slate-500 text-xs">Total Value</p>
                        <p className="text-amber-400 font-bold">{formatGold(selectedItem.value * selectedItem.count)}</p>
                      </div>
                    </div>

                    {selectedItem.equipped && (
                      <div className="bg-emerald-500/10 border border-emerald-500/30 rounded-lg p-3 flex items-center gap-2">
                        <Shield className="w-4 h-4 text-emerald-400" />
                        <span className="text-emerald-400 text-sm">Equipped by {selectedItem.character}</span>
                      </div>
                    )}
                  </div>

                  <div className="grid grid-cols-2 gap-3">
                    <button className="flex items-center justify-center gap-2 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                      <Send className="w-4 h-4" />
                      Transfer
                    </button>
                    <button className="flex items-center justify-center gap-2 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                      <Tag className="w-4 h-4" />
                      Sell
                    </button>
                    {selectedItem.mintable && (
                      <button className="col-span-2 flex items-center justify-center gap-2 px-4 py-2 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-lg hover:from-purple-400 hover:to-pink-400 transition">
                        <Sparkles className="w-4 h-4" />
                        Mint as NFT
                      </button>
                    )}
                  </div>
                </>
              )}
            </Dialog.Content>
          </Dialog.Portal>
        </Dialog.Root>
      </div>
    </Tooltip.Provider>
  )
}

