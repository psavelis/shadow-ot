'use client'

import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import {
  History, ArrowUpRight, ArrowDownRight, Filter, Search,
  Download, Calendar, Coins, ShoppingCart, Gift, Wallet,
  RefreshCw, ChevronDown, ExternalLink
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'

const transactionTypes = [
  { id: 'all', label: 'All Transactions' },
  { id: 'market', label: 'Market', icon: ShoppingCart },
  { id: 'transfer', label: 'Transfers', icon: ArrowUpRight },
  { id: 'nft', label: 'NFT', icon: Wallet },
  { id: 'premium', label: 'Premium', icon: Gift },
  { id: 'coins', label: 'Coins', icon: Coins },
]

const transactions = [
  {
    id: '1',
    type: 'market',
    title: 'Sold Demon Helmet',
    description: 'Market sale to DragonSlayer',
    amount: 500000,
    currency: 'gold',
    status: 'completed',
    date: '2024-12-04T14:32:00Z',
    direction: 'in',
    details: { buyer: 'DragonSlayer', item: 'Demon Helmet', quantity: 1 },
  },
  {
    id: '2',
    type: 'transfer',
    title: 'Transfer to MysticDruid',
    description: 'Character transfer',
    amount: 100000,
    currency: 'gold',
    status: 'completed',
    date: '2024-12-04T10:15:00Z',
    direction: 'out',
    details: { recipient: 'MysticDruid', character: 'ShadowKnight' },
  },
  {
    id: '3',
    type: 'nft',
    title: 'Minted Dragon Scale Armor',
    description: 'NFT minting fee',
    amount: 0.05,
    currency: 'ETH',
    status: 'completed',
    date: '2024-12-03T18:45:00Z',
    direction: 'out',
    txHash: '0x1234...5678',
    details: { item: 'Dragon Scale Armor', tokenId: '1234' },
  },
  {
    id: '4',
    type: 'premium',
    title: 'Premium Subscription',
    description: '90 days subscription',
    amount: 29.99,
    currency: 'USD',
    status: 'completed',
    date: '2024-12-01T09:00:00Z',
    direction: 'out',
    details: { period: '90 days', validUntil: '2025-03-01' },
  },
  {
    id: '5',
    type: 'coins',
    title: 'Purchased 1000 Coins',
    description: 'In-game currency purchase',
    amount: 9.99,
    currency: 'USD',
    status: 'completed',
    date: '2024-11-28T15:20:00Z',
    direction: 'out',
    details: { coins: 1000, bonus: 50 },
  },
  {
    id: '6',
    type: 'market',
    title: 'Bought Magic Sword',
    description: 'Market purchase from StormMage',
    amount: 250000,
    currency: 'gold',
    status: 'completed',
    date: '2024-11-25T12:00:00Z',
    direction: 'out',
    details: { seller: 'StormMage', item: 'Magic Sword', quantity: 1 },
  },
  {
    id: '7',
    type: 'nft',
    title: 'NFT Sale',
    description: 'Sold Golden Armor NFT',
    amount: 0.8,
    currency: 'ETH',
    status: 'completed',
    date: '2024-11-20T08:30:00Z',
    direction: 'in',
    txHash: '0xabcd...efgh',
    details: { item: 'Golden Armor', buyer: '0x9876...5432', tokenId: '0892' },
  },
  {
    id: '8',
    type: 'transfer',
    title: 'Received from SwiftArrow',
    description: 'Gift transfer',
    amount: 50000,
    currency: 'gold',
    status: 'completed',
    date: '2024-11-15T20:45:00Z',
    direction: 'in',
    details: { sender: 'SwiftArrow', note: 'Thanks for the help!' },
  },
]

const stats = {
  totalIn: 1384293,
  totalOut: 892847,
  nftVolume: 0.85,
  premiumSpent: 59.98,
}

export default function TransactionsPage() {
  const [selectedType, setSelectedType] = useState('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [dateRange, setDateRange] = useState('all')
  const [selectedTx, setSelectedTx] = useState<typeof transactions[0] | null>(null)

  const filteredTransactions = transactions.filter(tx => {
    const matchesType = selectedType === 'all' || tx.type === selectedType
    const matchesSearch = tx.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      tx.description.toLowerCase().includes(searchQuery.toLowerCase())
    return matchesType && matchesSearch
  })

  const formatAmount = (amount: number, currency: string) => {
    if (currency === 'gold') {
      if (amount >= 1000000) return `${(amount / 1000000).toFixed(2)}kk`
      if (amount >= 1000) return `${(amount / 1000).toFixed(1)}k`
      return amount.toLocaleString()
    }
    if (currency === 'ETH') return `${amount} ETH`
    return `$${amount.toFixed(2)}`
  }

  const formatDate = (date: string) => {
    return new Date(date).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  }

  const typeIcons: Record<string, any> = {
    market: ShoppingCart,
    transfer: ArrowUpRight,
    nft: Wallet,
    premium: Gift,
    coins: Coins,
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
          <h1 className="text-2xl font-bold text-white mb-1">Transaction History</h1>
          <p className="text-slate-400">View all your transactions and activity</p>
        </div>
        <button className="flex items-center gap-2 px-4 py-2 bg-slate-800/50 text-slate-300 rounded-lg hover:bg-slate-700 transition">
          <Download className="w-4 h-4" />
          Export
        </button>
      </motion.div>

      {/* Stats */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4"
      >
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center justify-between mb-2">
            <ArrowDownRight className="w-5 h-5 text-emerald-400" />
          </div>
          <p className="text-2xl font-bold text-white">{formatAmount(stats.totalIn, 'gold')}</p>
          <p className="text-xs text-slate-500">Total Received (Gold)</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center justify-between mb-2">
            <ArrowUpRight className="w-5 h-5 text-red-400" />
          </div>
          <p className="text-2xl font-bold text-white">{formatAmount(stats.totalOut, 'gold')}</p>
          <p className="text-xs text-slate-500">Total Spent (Gold)</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center justify-between mb-2">
            <Wallet className="w-5 h-5 text-purple-400" />
          </div>
          <p className="text-2xl font-bold text-white">{formatAmount(stats.nftVolume, 'ETH')}</p>
          <p className="text-xs text-slate-500">NFT Volume</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center justify-between mb-2">
            <Gift className="w-5 h-5 text-amber-400" />
          </div>
          <p className="text-2xl font-bold text-white">{formatAmount(stats.premiumSpent, 'USD')}</p>
          <p className="text-xs text-slate-500">Premium Spent</p>
        </div>
      </motion.div>

      {/* Filters */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
        className="flex flex-wrap gap-4"
      >
        <div className="flex-1 min-w-[250px] relative">
          <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
          <input
            type="text"
            placeholder="Search transactions..."
            value={searchQuery}
            onChange={e => setSearchQuery(e.target.value)}
            className="w-full bg-slate-800/50 border border-slate-700/50 rounded-lg pl-10 pr-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
          />
        </div>
        <select
          value={selectedType}
          onChange={e => setSelectedType(e.target.value)}
          className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
        >
          {transactionTypes.map(type => (
            <option key={type.id} value={type.id}>{type.label}</option>
          ))}
        </select>
        <select
          value={dateRange}
          onChange={e => setDateRange(e.target.value)}
          className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
        >
          <option value="all">All Time</option>
          <option value="7d">Last 7 Days</option>
          <option value="30d">Last 30 Days</option>
          <option value="90d">Last 90 Days</option>
        </select>
      </motion.div>

      {/* Transactions List */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.3 }}
        className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
      >
        <div className="divide-y divide-slate-700/50">
          {filteredTransactions.map((tx, idx) => {
            const Icon = typeIcons[tx.type] || History
            return (
              <motion.button
                key={tx.id}
                initial={{ opacity: 0, x: -10 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: idx * 0.03 }}
                onClick={() => setSelectedTx(tx)}
                className="w-full p-4 flex items-center gap-4 hover:bg-slate-700/30 transition text-left"
              >
                <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                  tx.direction === 'in' ? 'bg-emerald-500/20' : 'bg-slate-700/50'
                }`}>
                  <Icon className={`w-5 h-5 ${tx.direction === 'in' ? 'text-emerald-400' : 'text-slate-400'}`} />
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <p className="text-white font-medium truncate">{tx.title}</p>
                    <span className={`px-2 py-0.5 rounded text-xs ${
                      tx.status === 'completed' ? 'bg-emerald-500/20 text-emerald-400' :
                      tx.status === 'pending' ? 'bg-amber-500/20 text-amber-400' :
                      'bg-red-500/20 text-red-400'
                    }`}>
                      {tx.status}
                    </span>
                  </div>
                  <p className="text-slate-500 text-sm truncate">{tx.description}</p>
                </div>
                <div className="text-right">
                  <p className={`font-medium ${tx.direction === 'in' ? 'text-emerald-400' : 'text-white'}`}>
                    {tx.direction === 'in' ? '+' : '-'}{formatAmount(tx.amount, tx.currency)}
                  </p>
                  <p className="text-slate-500 text-xs">{formatDate(tx.date)}</p>
                </div>
              </motion.button>
            )
          })}
        </div>
      </motion.div>

      {filteredTransactions.length === 0 && (
        <div className="text-center py-16">
          <History className="w-16 h-16 text-slate-700 mx-auto mb-4" />
          <h3 className="text-xl font-semibold text-white mb-2">No transactions found</h3>
          <p className="text-slate-500">Try adjusting your filters</p>
        </div>
      )}

      {/* Transaction Detail Dialog */}
      <Dialog.Root open={!!selectedTx} onOpenChange={() => setSelectedTx(null)}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            {selectedTx && (
              <>
                <div className="flex items-center gap-4 mb-6">
                  <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${
                    selectedTx.direction === 'in' ? 'bg-emerald-500/20' : 'bg-slate-700/50'
                  }`}>
                    {(() => {
                      const Icon = typeIcons[selectedTx.type] || History
                      return <Icon className={`w-6 h-6 ${selectedTx.direction === 'in' ? 'text-emerald-400' : 'text-slate-400'}`} />
                    })()}
                  </div>
                  <div>
                    <Dialog.Title className="text-lg font-bold text-white">{selectedTx.title}</Dialog.Title>
                    <p className="text-slate-400 text-sm">{selectedTx.description}</p>
                  </div>
                </div>

                <div className="space-y-4 mb-6">
                  <div className="bg-slate-900/50 rounded-lg p-4">
                    <div className="flex items-center justify-between mb-3">
                      <span className="text-slate-400 text-sm">Amount</span>
                      <span className={`text-xl font-bold ${selectedTx.direction === 'in' ? 'text-emerald-400' : 'text-white'}`}>
                        {selectedTx.direction === 'in' ? '+' : '-'}{formatAmount(selectedTx.amount, selectedTx.currency)}
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-slate-400 text-sm">Status</span>
                      <span className={`px-2 py-0.5 rounded text-xs ${
                        selectedTx.status === 'completed' ? 'bg-emerald-500/20 text-emerald-400' : 'bg-amber-500/20 text-amber-400'
                      }`}>
                        {selectedTx.status}
                      </span>
                    </div>
                  </div>

                  <div className="bg-slate-900/50 rounded-lg p-4 space-y-2">
                    <div className="flex justify-between text-sm">
                      <span className="text-slate-400">Date</span>
                      <span className="text-white">{new Date(selectedTx.date).toLocaleString()}</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span className="text-slate-400">Type</span>
                      <span className="text-white capitalize">{selectedTx.type}</span>
                    </div>
                    {selectedTx.txHash && (
                      <div className="flex justify-between text-sm">
                        <span className="text-slate-400">Tx Hash</span>
                        <a href="#" className="text-amber-400 hover:text-amber-300 flex items-center gap-1">
                          {selectedTx.txHash}
                          <ExternalLink className="w-3 h-3" />
                        </a>
                      </div>
                    )}
                  </div>

                  {selectedTx.details && (
                    <div className="bg-slate-900/50 rounded-lg p-4">
                      <p className="text-slate-400 text-xs mb-2">Details</p>
                      {Object.entries(selectedTx.details).map(([key, value]) => (
                        <div key={key} className="flex justify-between text-sm py-1">
                          <span className="text-slate-400 capitalize">{key.replace(/([A-Z])/g, ' $1')}</span>
                          <span className="text-white">{String(value)}</span>
                        </div>
                      ))}
                    </div>
                  )}
                </div>

                <Dialog.Close className="w-full px-4 py-2.5 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                  Close
                </Dialog.Close>
              </>
            )}
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}

