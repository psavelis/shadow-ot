'use client'

import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import {
  Package, Search, Grid3X3, List, Sword, Shield, Wand2,
  Heart, Star, Sparkles, ChevronDown, X, Eye, Send, Tag,
  Layers, Box, Loader2, AlertTriangle
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'
import { useInventory, useTransferItem, useListItemOnMarket } from '@/shared/hooks/useDashboard'
import { useCharacters } from '@/shared/hooks/useCharacters'
import type { InventoryItem } from '@/shared/api/endpoints'
import { getItemSprite } from '@/shared/utils/assets'

const categories = [
  { id: 'all', name: 'All Items', icon: Package },
  { id: 'weapon', name: 'Weapons', icon: Sword },
  { id: 'armor', name: 'Armor', icon: Shield },
  { id: 'helmet', name: 'Helmets', icon: Shield },
  { id: 'legs', name: 'Legs', icon: Shield },
  { id: 'boots', name: 'Boots', icon: Shield },
  { id: 'shield', name: 'Shields', icon: Shield },
  { id: 'ring', name: 'Rings', icon: Sparkles },
  { id: 'amulet', name: 'Amulets', icon: Sparkles },
  { id: 'consumable', name: 'Consumables', icon: Heart },
  { id: 'tool', name: 'Tools', icon: Wand2 },
  { id: 'quest', name: 'Quest Items', icon: Star },
]

const rarityColors: Record<string, string> = {
  common: 'border-slate-500/30 bg-slate-800/30',
  uncommon: 'border-green-500/30 bg-green-900/20',
  rare: 'border-blue-500/30 bg-blue-900/20',
  epic: 'border-purple-500/30 bg-purple-900/20',
  legendary: 'border-amber-500/30 bg-amber-900/20',
}

const rarityLabels: Record<string, { color: string; label: string }> = {
  common: { color: 'text-slate-400', label: 'Common' },
  uncommon: { color: 'text-green-400', label: 'Uncommon' },
  rare: { color: 'text-blue-400', label: 'Rare' },
  epic: { color: 'text-purple-400', label: 'Epic' },
  legendary: { color: 'text-amber-400', label: 'Legendary' },
}

export default function InventoryPage() {
  const [selectedCategory, setSelectedCategory] = useState('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid')
  const [selectedItem, setSelectedItem] = useState<InventoryItem | null>(null)
  const [showTransfer, setShowTransfer] = useState(false)
  const [showSell, setShowSell] = useState(false)
  const [transferTarget, setTransferTarget] = useState('')
  const [sellPrice, setSellPrice] = useState('')
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 50

  // Real API hooks
  const { data: inventoryData, isLoading, error } = useInventory({
    category: selectedCategory === 'all' ? undefined : selectedCategory,
    search: searchQuery || undefined,
    page: currentPage,
    pageSize,
  })
  const { data: characters = [] } = useCharacters()
  const transferItem = useTransferItem()
  const listOnMarket = useListItemOnMarket()

  const items = inventoryData?.data || []

  const handleTransfer = async () => {
    if (!selectedItem || !transferTarget) return
    await transferItem.mutateAsync({ itemId: selectedItem.id, toCharacterId: transferTarget })
    setShowTransfer(false)
    setSelectedItem(null)
    setTransferTarget('')
  }

  const handleSell = async () => {
    if (!selectedItem || !sellPrice) return
    await listOnMarket.mutateAsync({ itemId: selectedItem.id, price: parseInt(sellPrice) })
    setShowSell(false)
    setSelectedItem(null)
    setSellPrice('')
  }

  const formatValue = (value: number) => {
    if (value >= 1000000) return `${(value / 1000000).toFixed(2)}kk`
    if (value >= 1000) return `${(value / 1000).toFixed(1)}k`
    return value.toLocaleString()
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex items-center justify-between"
      >
        <div>
          <h1 className="text-2xl font-bold text-white mb-1">Inventory</h1>
          <p className="text-slate-400">
            Manage your items across all characters
          </p>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => setViewMode('grid')}
            className={`p-2 rounded-lg transition ${
              viewMode === 'grid' ? 'bg-amber-500/20 text-amber-400' : 'bg-slate-800/50 text-slate-400'
            }`}
          >
            <Grid3X3 className="w-5 h-5" />
          </button>
          <button
            onClick={() => setViewMode('list')}
            className={`p-2 rounded-lg transition ${
              viewMode === 'list' ? 'bg-amber-500/20 text-amber-400' : 'bg-slate-800/50 text-slate-400'
            }`}
          >
            <List className="w-5 h-5" />
          </button>
        </div>
      </motion.div>

      <div className="flex gap-6">
        {/* Categories Sidebar */}
        <motion.div
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ delay: 0.1 }}
          className="w-56 shrink-0"
        >
          <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
            <h3 className="text-sm font-semibold text-slate-400 mb-3">Categories</h3>
            <div className="space-y-1">
              {categories.map(cat => {
                const Icon = cat.icon
                return (
                  <button
                    key={cat.id}
                    onClick={() => setSelectedCategory(cat.id)}
                    className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition ${
                      selectedCategory === cat.id
                        ? 'bg-amber-500/20 text-amber-400'
                        : 'text-slate-400 hover:bg-slate-700/50'
                    }`}
                  >
                    <Icon className="w-4 h-4" />
                    <span className="flex-1 text-sm">{cat.name}</span>
                  </button>
                )
              })}
            </div>
          </div>
        </motion.div>

        {/* Main Content */}
        <div className="flex-1 space-y-4">
          {/* Search */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
            className="relative"
          >
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
            <input
              type="text"
              placeholder="Search items..."
              value={searchQuery}
              onChange={e => setSearchQuery(e.target.value)}
              className="w-full bg-slate-800/50 border border-slate-700/50 rounded-lg pl-10 pr-4 py-2.5 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
            />
          </motion.div>

          {/* Items Grid/List */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
          >
            {isLoading ? (
              <div className="flex items-center justify-center py-12">
                <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
              </div>
            ) : error ? (
              <div className="flex items-center justify-center py-12 text-red-400">
                <AlertTriangle className="w-5 h-5 mr-2" />
                Failed to load inventory
              </div>
            ) : items.length === 0 ? (
              <div className="text-center py-16 bg-slate-800/50 border border-slate-700/50 rounded-xl">
                <Package className="w-16 h-16 text-slate-700 mx-auto mb-4" />
                <h3 className="text-xl font-semibold text-white mb-2">No items found</h3>
                <p className="text-slate-500">Try adjusting your search or category filter</p>
              </div>
            ) : viewMode === 'grid' ? (
              <div className="grid sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                {items.map((item, idx) => {
                  const rarity = rarityColors[item.rarity] || rarityColors.common
                  const rarityLabel = rarityLabels[item.rarity] || rarityLabels.common
                  return (
                    <motion.button
                      key={item.id}
                      initial={{ opacity: 0, scale: 0.95 }}
                      animate={{ opacity: 1, scale: 1 }}
                      transition={{ delay: idx * 0.02 }}
                      onClick={() => setSelectedItem(item)}
                      className={`relative border ${rarity} rounded-xl p-4 text-left hover:scale-[1.02] transition`}
                    >
                      {item.equipped && (
                        <span className="absolute top-2 right-2 px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded">
                          Equipped
                        </span>
                      )}
                      <div className="w-12 h-12 rounded-lg bg-slate-900/50 flex items-center justify-center mb-3">
                        <img
                          src={getItemSprite(item.spriteId)}
                          alt={item.name}
                          className="w-10 h-10 object-contain"
                          onError={(e) => { (e.target as HTMLImageElement).src = '/items/default.png' }}
                        />
                      </div>
                      <h4 className="text-white font-medium truncate">{item.name}</h4>
                      <p className={`text-xs ${rarityLabel.color}`}>{rarityLabel.label}</p>
                      {item.quantity > 1 && (
                        <span className="absolute bottom-4 right-4 px-2 py-0.5 bg-slate-900/80 text-white text-xs rounded">
                          x{item.quantity}
                        </span>
                      )}
                    </motion.button>
                  )
                })}
              </div>
            ) : (
              <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
                <table className="w-full">
                  <thead className="bg-slate-900/50">
                    <tr className="text-slate-400 text-sm">
                      <th className="text-left p-4">Item</th>
                      <th className="text-left p-4">Type</th>
                      <th className="text-left p-4">Rarity</th>
                      <th className="text-left p-4">Qty</th>
                      <th className="text-left p-4">Status</th>
                      <th className="text-left p-4"></th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-700/50">
                    {items.map(item => {
                      const rarityLabel = rarityLabels[item.rarity] || rarityLabels.common
                      return (
                        <tr key={item.id} className="hover:bg-slate-700/30 transition">
                          <td className="p-4">
                            <div className="flex items-center gap-3">
                              <img
                                src={getItemSprite(item.spriteId)}
                                alt={item.name}
                                className="w-8 h-8 object-contain"
                                onError={(e) => { (e.target as HTMLImageElement).src = '/items/default.png' }}
                              />
                              <span className="text-white font-medium">{item.name}</span>
                            </div>
                          </td>
                          <td className="p-4 text-slate-400 capitalize">{item.type}</td>
                          <td className="p-4">
                            <span className={rarityLabel.color}>{rarityLabel.label}</span>
                          </td>
                          <td className="p-4 text-slate-400">{item.quantity}</td>
                          <td className="p-4">
                            {item.equipped ? (
                              <span className="px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded">
                                Equipped
                              </span>
                            ) : (
                              <span className="text-slate-500">-</span>
                            )}
                          </td>
                          <td className="p-4">
                            <button
                              onClick={() => setSelectedItem(item)}
                              className="p-2 text-slate-400 hover:text-white hover:bg-slate-700/50 rounded transition"
                            >
                              <Eye className="w-4 h-4" />
                            </button>
                          </td>
                        </tr>
                      )
                    })}
                  </tbody>
                </table>
              </div>
            )}

            {/* Pagination */}
            {inventoryData && inventoryData.totalPages > 1 && (
              <div className="flex items-center justify-between pt-4">
                <p className="text-slate-500 text-sm">
                  Page {currentPage} of {inventoryData.totalPages}
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
                    onClick={() => setCurrentPage(p => Math.min(inventoryData.totalPages, p + 1))}
                    disabled={currentPage === inventoryData.totalPages}
                    className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded hover:bg-slate-600/50 transition disabled:opacity-50"
                  >
                    Next
                  </button>
                </div>
              </div>
            )}
          </motion.div>
        </div>
      </div>

      {/* Item Detail Dialog */}
      <Dialog.Root open={!!selectedItem && !showTransfer && !showSell} onOpenChange={() => setSelectedItem(null)}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            {selectedItem && (
              <>
                <div className="flex items-start gap-4 mb-6">
                  <div className={`w-16 h-16 rounded-xl flex items-center justify-center ${rarityColors[selectedItem.rarity] || rarityColors.common}`}>
                    <img
                      src={getItemSprite(selectedItem.spriteId)}
                      alt={selectedItem.name}
                      className="w-12 h-12 object-contain"
                    />
                  </div>
                  <div className="flex-1">
                    <Dialog.Title className="text-lg font-bold text-white">{selectedItem.name}</Dialog.Title>
                    <p className={`text-sm ${(rarityLabels[selectedItem.rarity] || rarityLabels.common).color}`}>
                      {(rarityLabels[selectedItem.rarity] || rarityLabels.common).label} {selectedItem.type}
                    </p>
                    {selectedItem.equipped && (
                      <span className="inline-block mt-1 px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded">
                        Equipped
                      </span>
                    )}
                  </div>
                </div>

                {/* Attributes */}
                {Object.keys(selectedItem.attributes).length > 0 && (
                  <div className="bg-slate-900/50 rounded-lg p-4 mb-4">
                    <h4 className="text-slate-400 text-xs mb-2">Attributes</h4>
                    <div className="space-y-1">
                      {Object.entries(selectedItem.attributes).map(([key, value]) => (
                        <div key={key} className="flex justify-between text-sm">
                          <span className="text-slate-400 capitalize">{key}</span>
                          <span className="text-white">{value}</span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {/* Requirements */}
                {selectedItem.requirements && (
                  <div className="bg-slate-900/50 rounded-lg p-4 mb-4">
                    <h4 className="text-slate-400 text-xs mb-2">Requirements</h4>
                    {selectedItem.requirements.level && (
                      <div className="flex justify-between text-sm">
                        <span className="text-slate-400">Level</span>
                        <span className="text-white">{selectedItem.requirements.level}</span>
                      </div>
                    )}
                    {selectedItem.requirements.vocation && (
                      <div className="flex justify-between text-sm">
                        <span className="text-slate-400">Vocation</span>
                        <span className="text-white">{selectedItem.requirements.vocation.join(', ')}</span>
                      </div>
                    )}
                  </div>
                )}

                <div className="flex gap-3">
                  <button
                    onClick={() => setShowTransfer(true)}
                    disabled={selectedItem.equipped}
                    className="flex-1 flex items-center justify-center gap-2 px-4 py-2.5 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition disabled:opacity-50"
                  >
                    <Send className="w-4 h-4" />
                    Transfer
                  </button>
                  <button
                    onClick={() => setShowSell(true)}
                    disabled={selectedItem.equipped}
                    className="flex-1 flex items-center justify-center gap-2 px-4 py-2.5 bg-amber-500 text-black rounded-lg hover:bg-amber-400 transition disabled:opacity-50"
                  >
                    <Tag className="w-4 h-4" />
                    Sell
                  </button>
                </div>
              </>
            )}
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>

      {/* Transfer Dialog */}
      <Dialog.Root open={showTransfer} onOpenChange={setShowTransfer}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            <Dialog.Title className="text-lg font-bold text-white mb-4">Transfer Item</Dialog.Title>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-slate-400 mb-2">Transfer to character</label>
                <select
                  value={transferTarget}
                  onChange={e => setTransferTarget(e.target.value)}
                  className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
                >
                  <option value="">Select character...</option>
                  {characters.filter(c => c.id !== selectedItem?.characterId).map(char => (
                    <option key={char.id} value={char.id}>{char.name}</option>
                  ))}
                </select>
              </div>
            </div>
            
            <div className="flex gap-3 mt-6">
              <button
                onClick={() => setShowTransfer(false)}
                className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition"
              >
                Cancel
              </button>
              <button
                onClick={handleTransfer}
                disabled={!transferTarget || transferItem.isPending}
                className="flex-1 px-4 py-2 bg-amber-500 text-black rounded-lg hover:bg-amber-400 transition disabled:opacity-50 flex items-center justify-center gap-2"
              >
                {transferItem.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    Transferring...
                  </>
                ) : (
                  'Transfer'
                )}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>

      {/* Sell Dialog */}
      <Dialog.Root open={showSell} onOpenChange={setShowSell}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            <Dialog.Title className="text-lg font-bold text-white mb-4">List on Market</Dialog.Title>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-slate-400 mb-2">Price (gold)</label>
                <input
                  type="number"
                  value={sellPrice}
                  onChange={e => setSellPrice(e.target.value)}
                  placeholder="Enter price..."
                  min={1}
                  className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
                />
              </div>
            </div>
            
            <div className="flex gap-3 mt-6">
              <button
                onClick={() => setShowSell(false)}
                className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition"
              >
                Cancel
              </button>
              <button
                onClick={handleSell}
                disabled={!sellPrice || listOnMarket.isPending}
                className="flex-1 px-4 py-2 bg-amber-500 text-black rounded-lg hover:bg-amber-400 transition disabled:opacity-50 flex items-center justify-center gap-2"
              >
                {listOnMarket.isPending ? (
                  <>
                    <Loader2 className="w-4 h-4 animate-spin" />
                    Listing...
                  </>
                ) : (
                  'List for Sale'
                )}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}
