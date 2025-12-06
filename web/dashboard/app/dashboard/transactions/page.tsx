'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import {
  History, ArrowUpRight, ArrowDownRight, Search,
  Download, Coins, ShoppingCart, Gift, Wallet,
  Loader2, AlertTriangle, ExternalLink
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'
import { useTransactions } from '@/shared/hooks/useDashboard'
import type { Transaction } from '@/shared/api/endpoints'

const transactionTypes = [
  { id: 'all', label: 'All Transactions' },
  { id: 'market', label: 'Market', icon: ShoppingCart },
  { id: 'transfer', label: 'Transfers', icon: ArrowUpRight },
  { id: 'nft', label: 'NFT', icon: Wallet },
  { id: 'premium', label: 'Premium', icon: Gift },
]

const typeIcons: Record<string, React.ElementType> = {
  market: ShoppingCart,
  transfer: ArrowUpRight,
  nft: Wallet,
  premium: Gift,
}

export default function TransactionsPage() {
  const [selectedType, setSelectedType] = useState<Transaction['type'] | 'all'>('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [dateRange, setDateRange] = useState('all')
  const [currentPage, setCurrentPage] = useState(1)
  const [selectedTx, setSelectedTx] = useState<Transaction | null>(null)
  const pageSize = 20
  
  // Real API hook
  const { data: transactionsData, isLoading, error } = useTransactions({
    type: selectedType === 'all' ? undefined : selectedType,
    page: currentPage,
    pageSize,
  })
  
  const transactions = transactionsData?.data || []

  const filteredTransactions = transactions.filter(tx => {
    const matchesSearch = tx.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      tx.description.toLowerCase().includes(searchQuery.toLowerCase())
    return matchesSearch
  })
  
  // Calculate stats from real data
  const stats = {
    totalIn: transactions.filter(t => t.currency === 'gold' && t.status === 'completed' && t.to).reduce((sum, t) => sum + t.amount, 0),
    totalOut: transactions.filter(t => t.currency === 'gold' && t.status === 'completed' && t.from).reduce((sum, t) => sum + t.amount, 0),
    nftVolume: transactions.filter(t => t.type === 'nft' && t.status === 'completed').reduce((sum, t) => sum + t.amount, 0),
    premiumSpent: transactions.filter(t => t.type === 'premium' && t.status === 'completed').reduce((sum, t) => sum + t.amount, 0),
  }

  const formatAmount = (amount: number, currency: string) => {
    if (currency === 'gold') {
      if (amount >= 1000000) return `${(amount / 1000000).toFixed(2)}kk`
      if (amount >= 1000) return `${(amount / 1000).toFixed(1)}k`
      return amount.toLocaleString()
    }
    if (currency === 'coins') return `${amount} coins`
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

  const getDirection = (tx: Transaction) => {
    // Determine direction based on from/to fields
    return tx.to ? 'in' : 'out'
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
          <p className="text-2xl font-bold text-white">{formatAmount(stats.nftVolume, 'usd')}</p>
          <p className="text-xs text-slate-500">NFT Volume</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center justify-between mb-2">
            <Gift className="w-5 h-5 text-amber-400" />
          </div>
          <p className="text-2xl font-bold text-white">{formatAmount(stats.premiumSpent, 'usd')}</p>
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
          onChange={e => setSelectedType(e.target.value as typeof selectedType)}
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
        {isLoading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
          </div>
        ) : error ? (
          <div className="flex items-center justify-center py-12 text-red-400">
            <AlertTriangle className="w-5 h-5 mr-2" />
            Failed to load transactions
          </div>
        ) : filteredTransactions.length === 0 ? (
          <div className="text-center py-16">
            <History className="w-16 h-16 text-slate-700 mx-auto mb-4" />
            <h3 className="text-xl font-semibold text-white mb-2">No transactions found</h3>
            <p className="text-slate-500">Try adjusting your filters</p>
          </div>
        ) : (
          <div className="divide-y divide-slate-700/50">
            {filteredTransactions.map((tx, idx) => {
              const Icon = typeIcons[tx.type] || History
              const direction = getDirection(tx)
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
                    direction === 'in' ? 'bg-emerald-500/20' : 'bg-slate-700/50'
                  }`}>
                    <Icon className={`w-5 h-5 ${direction === 'in' ? 'text-emerald-400' : 'text-slate-400'}`} />
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
                    <p className={`font-medium ${direction === 'in' ? 'text-emerald-400' : 'text-white'}`}>
                      {direction === 'in' ? '+' : '-'}{formatAmount(tx.amount, tx.currency)}
                    </p>
                    <p className="text-slate-500 text-xs">{formatDate(tx.timestamp)}</p>
                  </div>
                </motion.button>
              )
            })}
          </div>
        )}

        {/* Pagination */}
        {transactionsData && transactionsData.totalPages > 1 && (
          <div className="p-4 border-t border-slate-700 flex items-center justify-between">
            <p className="text-slate-500 text-sm">
              Page {currentPage} of {transactionsData.totalPages}
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
                onClick={() => setCurrentPage(p => Math.min(transactionsData.totalPages, p + 1))}
                disabled={currentPage === transactionsData.totalPages}
                className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded hover:bg-slate-600/50 transition disabled:opacity-50"
              >
                Next
              </button>
            </div>
          </div>
        )}
      </motion.div>

      {/* Transaction Detail Dialog */}
      <Dialog.Root open={!!selectedTx} onOpenChange={() => setSelectedTx(null)}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            {selectedTx && (
              <>
                <div className="flex items-center gap-4 mb-6">
                  <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${
                    getDirection(selectedTx) === 'in' ? 'bg-emerald-500/20' : 'bg-slate-700/50'
                  }`}>
                    {(() => {
                      const Icon = typeIcons[selectedTx.type] || History
                      return <Icon className={`w-6 h-6 ${getDirection(selectedTx) === 'in' ? 'text-emerald-400' : 'text-slate-400'}`} />
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
                      <span className={`text-xl font-bold ${getDirection(selectedTx) === 'in' ? 'text-emerald-400' : 'text-white'}`}>
                        {getDirection(selectedTx) === 'in' ? '+' : '-'}{formatAmount(selectedTx.amount, selectedTx.currency)}
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-slate-400 text-sm">Status</span>
                      <span className={`px-2 py-0.5 rounded text-xs ${
                        selectedTx.status === 'completed' ? 'bg-emerald-500/20 text-emerald-400' : 
                        selectedTx.status === 'pending' ? 'bg-amber-500/20 text-amber-400' :
                        'bg-red-500/20 text-red-400'
                      }`}>
                        {selectedTx.status}
                      </span>
                    </div>
                  </div>

                  <div className="bg-slate-900/50 rounded-lg p-4 space-y-2">
                    <div className="flex justify-between text-sm">
                      <span className="text-slate-400">Date</span>
                      <span className="text-white">{new Date(selectedTx.timestamp).toLocaleString()}</span>
                    </div>
                    <div className="flex justify-between text-sm">
                      <span className="text-slate-400">Type</span>
                      <span className="text-white capitalize">{selectedTx.type}</span>
                    </div>
                    {selectedTx.from && (
                      <div className="flex justify-between text-sm">
                        <span className="text-slate-400">From</span>
                        <span className="text-white">{selectedTx.from}</span>
                      </div>
                    )}
                    {selectedTx.to && (
                      <div className="flex justify-between text-sm">
                        <span className="text-slate-400">To</span>
                        <span className="text-white">{selectedTx.to}</span>
                      </div>
                    )}
                  </div>
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
