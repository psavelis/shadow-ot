'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { Search, Filter, TrendingUp, TrendingDown, ShoppingCart, Package, History, Plus, X } from 'lucide-react'
import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts'

const itemTypes = [
  { id: 'all', label: 'All Items' },
  { id: 'weapon', label: 'Weapons' },
  { id: 'armor', label: 'Armor' },
  { id: 'helmet', label: 'Helmets' },
  { id: 'legs', label: 'Legs' },
  { id: 'boots', label: 'Boots' },
  { id: 'shield', label: 'Shields' },
  { id: 'ring', label: 'Rings' },
  { id: 'amulet', label: 'Amulets' },
  { id: 'consumable', label: 'Consumables' },
  { id: 'rune', label: 'Runes' },
]

const offers = [
  { id: '1', type: 'sell', itemName: 'Demon Helmet', amount: 1, price: 500000, seller: 'DragonSlayer', realm: 'shadowveil', createdAt: '2024-12-04T10:00:00Z' },
  { id: '2', type: 'sell', itemName: 'Magic Sword', amount: 1, price: 250000, seller: 'MysticMage', realm: 'aetheria', createdAt: '2024-12-04T09:30:00Z' },
  { id: '3', type: 'buy', itemName: 'Golden Armor', amount: 1, price: 750000, buyer: 'IronKnight', realm: 'shadowveil', createdAt: '2024-12-04T08:15:00Z' },
  { id: '4', type: 'sell', itemName: 'Thunder Hammer', amount: 1, price: 180000, seller: 'StormBringer', realm: 'warbound', createdAt: '2024-12-03T22:00:00Z' },
  { id: '5', type: 'buy', itemName: 'Dragon Scale Legs', amount: 1, price: 450000, buyer: 'ScaleFinder', realm: 'mythara', createdAt: '2024-12-03T18:45:00Z' },
]

const myOffers = [
  { id: '6', type: 'sell', itemName: 'Blue Robe', amount: 2, price: 15000, status: 'active', createdAt: '2024-12-04T08:00:00Z' },
  { id: '7', type: 'buy', itemName: 'Great Mana Potion', amount: 100, price: 150, status: 'active', createdAt: '2024-12-03T20:00:00Z' },
]

const priceHistory = [
  { date: 'Nov 28', price: 480000 },
  { date: 'Nov 29', price: 495000 },
  { date: 'Nov 30', price: 510000 },
  { date: 'Dec 01', price: 505000 },
  { date: 'Dec 02', price: 490000 },
  { date: 'Dec 03', price: 498000 },
  { date: 'Dec 04', price: 500000 },
]

function formatGold(amount: number) {
  if (amount >= 1000000) return `${(amount / 1000000).toFixed(2)}kk`
  if (amount >= 1000) return `${(amount / 1000).toFixed(0)}k`
  return amount.toLocaleString()
}

function formatTime(date: string) {
  const now = new Date()
  const then = new Date(date)
  const diff = now.getTime() - then.getTime()
  const hours = Math.floor(diff / (1000 * 60 * 60))
  if (hours < 1) return 'Just now'
  if (hours < 24) return `${hours}h ago`
  return `${Math.floor(hours / 24)}d ago`
}

export default function MarketPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedType, setSelectedType] = useState('all')
  const [offerType, setOfferType] = useState<'all' | 'buy' | 'sell'>('all')
  const [selectedItem, setSelectedItem] = useState<string | null>('Demon Helmet')
  const [showCreateModal, setShowCreateModal] = useState(false)

  const filteredOffers = offers.filter(o => {
    const matchesSearch = o.itemName.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesType = offerType === 'all' || o.type === offerType
    return matchesSearch && matchesType
  })

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-2xl font-display font-bold mb-1">Market</h1>
            <p className="text-shadow-400">Buy and sell items with other players</p>
          </div>
          <button onClick={() => setShowCreateModal(true)} className="btn-primary flex items-center gap-2">
            <Plus className="w-4 h-4" />
            Create Offer
          </button>
        </div>
      </motion.div>

      <div className="grid lg:grid-cols-3 gap-6">
        {/* Main Content */}
        <div className="lg:col-span-2 space-y-6">
          {/* Filters */}
          <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="card">
            <div className="flex flex-col sm:flex-row gap-4">
              <div className="relative flex-1">
                <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
                <input
                  type="text"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  placeholder="Search items..."
                  className="w-full pl-12 pr-4 py-2.5 bg-shadow-800 border border-shadow-600 rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50"
                />
              </div>
              <select
                value={selectedType}
                onChange={(e) => setSelectedType(e.target.value)}
                className="px-4 py-2.5 bg-shadow-800 border border-shadow-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-accent-500/50"
              >
                {itemTypes.map((type) => (
                  <option key={type.id} value={type.id}>{type.label}</option>
                ))}
              </select>
              <div className="flex rounded-lg overflow-hidden border border-shadow-600">
                {(['all', 'buy', 'sell'] as const).map((type) => (
                  <button
                    key={type}
                    onClick={() => setOfferType(type)}
                    className={`px-4 py-2.5 text-sm capitalize transition-colors ${
                      offerType === type
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

          {/* Offers List */}
          <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="card">
            <h2 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
              <ShoppingCart className="w-5 h-5 text-accent-500" />
              Market Offers
            </h2>
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-shadow-700">
                    <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Item</th>
                    <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Type</th>
                    <th className="text-right py-3 px-4 text-shadow-400 font-medium text-sm">Amount</th>
                    <th className="text-right py-3 px-4 text-shadow-400 font-medium text-sm">Price</th>
                    <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Player</th>
                    <th className="text-right py-3 px-4 text-shadow-400 font-medium text-sm">Time</th>
                    <th className="text-right py-3 px-4 text-shadow-400 font-medium text-sm"></th>
                  </tr>
                </thead>
                <tbody>
                  {filteredOffers.map((offer) => (
                    <tr key={offer.id} className="border-b border-shadow-800 last:border-0 hover:bg-shadow-800/50">
                      <td className="py-3 px-4">
                        <button
                          onClick={() => setSelectedItem(offer.itemName)}
                          className="font-medium text-white hover:text-accent-400 transition-colors"
                        >
                          {offer.itemName}
                        </button>
                      </td>
                      <td className="py-3 px-4">
                        <span className={`px-2 py-1 rounded text-xs font-medium ${
                          offer.type === 'buy' ? 'bg-green-500/20 text-green-400' : 'bg-blue-500/20 text-blue-400'
                        }`}>
                          {offer.type.toUpperCase()}
                        </span>
                      </td>
                      <td className="py-3 px-4 text-right text-white">{offer.amount}x</td>
                      <td className="py-3 px-4 text-right text-yellow-400 font-medium">{formatGold(offer.price)}</td>
                      <td className="py-3 px-4 text-shadow-400">{'seller' in offer ? offer.seller : offer.buyer}</td>
                      <td className="py-3 px-4 text-right text-shadow-500 text-sm">{formatTime(offer.createdAt)}</td>
                      <td className="py-3 px-4 text-right">
                        <button className="btn-secondary text-xs py-1.5 px-3">
                          {offer.type === 'buy' ? 'Sell' : 'Buy'}
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </motion.div>

          {/* My Offers */}
          <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.3 }} className="card">
            <h2 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
              <Package className="w-5 h-5 text-accent-500" />
              My Offers
            </h2>
            {myOffers.length > 0 ? (
              <div className="space-y-3">
                {myOffers.map((offer) => (
                  <div key={offer.id} className="flex items-center justify-between p-3 bg-shadow-800/50 rounded-lg">
                    <div className="flex items-center gap-4">
                      <span className={`px-2 py-1 rounded text-xs font-medium ${
                        offer.type === 'buy' ? 'bg-green-500/20 text-green-400' : 'bg-blue-500/20 text-blue-400'
                      }`}>
                        {offer.type.toUpperCase()}
                      </span>
                      <span className="text-white font-medium">{offer.amount}x {offer.itemName}</span>
                    </div>
                    <div className="flex items-center gap-4">
                      <span className="text-yellow-400">{formatGold(offer.price * offer.amount)}</span>
                      <span className="text-shadow-500 text-sm">{formatTime(offer.createdAt)}</span>
                      <button className="p-1.5 text-red-400 hover:bg-red-500/20 rounded transition-colors">
                        <X className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-shadow-500 text-center py-8">You have no active offers</p>
            )}
          </motion.div>
        </div>

        {/* Sidebar - Price Chart */}
        <motion.div initial={{ opacity: 0, x: 20 }} animate={{ opacity: 1, x: 0 }} transition={{ delay: 0.2 }} className="space-y-6">
          {selectedItem && (
            <div className="card">
              <h2 className="text-lg font-semibold text-white mb-2">{selectedItem}</h2>
              <p className="text-shadow-400 text-sm mb-4">7-day price history</p>
              <div className="flex items-center gap-4 mb-4">
                <div>
                  <p className="text-2xl font-bold text-white">{formatGold(500000)}</p>
                  <p className="text-sm text-shadow-500">Current price</p>
                </div>
                <div className="flex items-center text-green-400 text-sm">
                  <TrendingUp className="w-4 h-4 mr-1" />
                  +4.2%
                </div>
              </div>
              <div className="h-48">
                <ResponsiveContainer width="100%" height="100%">
                  <AreaChart data={priceHistory}>
                    <defs>
                      <linearGradient id="priceGradient" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="5%" stopColor="#e94560" stopOpacity={0.3}/>
                        <stop offset="95%" stopColor="#e94560" stopOpacity={0}/>
                      </linearGradient>
                    </defs>
                    <CartesianGrid strokeDasharray="3 3" stroke="#3b3c40" />
                    <XAxis dataKey="date" stroke="#696a74" fontSize={10} />
                    <YAxis stroke="#696a74" fontSize={10} tickFormatter={(v) => formatGold(v)} />
                    <Tooltip
                      contentStyle={{ backgroundColor: '#1a1a2e', border: '1px solid #3b3c40', borderRadius: '8px' }}
                      formatter={(value: number) => [formatGold(value), 'Price']}
                    />
                    <Area type="monotone" dataKey="price" stroke="#e94560" strokeWidth={2} fill="url(#priceGradient)" />
                  </AreaChart>
                </ResponsiveContainer>
              </div>
              <div className="mt-4 pt-4 border-t border-shadow-700 grid grid-cols-2 gap-4 text-sm">
                <div>
                  <p className="text-shadow-500">7d Low</p>
                  <p className="text-white font-medium">{formatGold(480000)}</p>
                </div>
                <div>
                  <p className="text-shadow-500">7d High</p>
                  <p className="text-white font-medium">{formatGold(510000)}</p>
                </div>
                <div>
                  <p className="text-shadow-500">Avg Price</p>
                  <p className="text-white font-medium">{formatGold(496857)}</p>
                </div>
                <div>
                  <p className="text-shadow-500">Volume (7d)</p>
                  <p className="text-white font-medium">47 sold</p>
                </div>
              </div>
            </div>
          )}

          {/* Recent Transactions */}
          <div className="card">
            <h2 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
              <History className="w-5 h-5 text-accent-500" />
              Recent Transactions
            </h2>
            <div className="space-y-3">
              {[
                { item: 'Demon Helmet', price: 500000, time: '10 min ago', type: 'sold' },
                { item: 'Magic Sword', price: 245000, time: '25 min ago', type: 'bought' },
                { item: 'Great Mana Potion', price: 1500, time: '1 hour ago', type: 'sold' },
              ].map((tx, i) => (
                <div key={i} className="flex items-center justify-between py-2">
                  <div>
                    <p className="text-white text-sm">{tx.item}</p>
                    <p className="text-shadow-500 text-xs">{tx.time}</p>
                  </div>
                  <div className="text-right">
                    <p className={`text-sm font-medium ${tx.type === 'sold' ? 'text-green-400' : 'text-red-400'}`}>
                      {tx.type === 'sold' ? '+' : '-'}{formatGold(tx.price)}
                    </p>
                    <p className="text-shadow-500 text-xs capitalize">{tx.type}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </motion.div>
      </div>
    </div>
  )
}

