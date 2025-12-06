'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import {
  Home, Search, MapPin, Bed, Square, Coins, Clock, User,
  Gavel, ChevronRight, Loader2, AlertTriangle
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'
import * as Dialog from '@radix-ui/react-dialog'
import { useHouses, useMyHouses, useBidOnHouse, useLeaveHouse } from '@/shared/hooks/useDashboard'
import type { House } from '@/shared/api/endpoints'

const towns = ['All Towns', 'Thais', 'Venore', 'Edron', 'Carlin', "Ab'Dendriel", 'Kazordoon', 'Darashia', 'Ankrahmun', 'Port Hope', 'Liberty Bay', 'Yalahar']
const sizes = ['All Sizes', 'small', 'medium', 'large', 'extra_large']
const statuses = ['All Status', 'available', 'auction', 'rented']
const types = ['All Types', 'house', 'guildhall']

const sizeColors: Record<string, string> = {
  'small': 'bg-slate-500/20 text-slate-400',
  'medium': 'bg-blue-500/20 text-blue-400',
  'large': 'bg-purple-500/20 text-purple-400',
  'extra_large': 'bg-amber-500/20 text-amber-400',
}

const statusColors: Record<string, string> = {
  'available': 'bg-emerald-500/20 text-emerald-400',
  'auction': 'bg-amber-500/20 text-amber-400',
  'rented': 'bg-slate-500/20 text-slate-400',
}

function formatGold(amount: number) {
  if (amount >= 1000000) return `${(amount / 1000000).toFixed(2)}kk`
  if (amount >= 1000) return `${(amount / 1000).toFixed(1)}k`
  return amount.toLocaleString()
}

export default function HousesPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedTown, setSelectedTown] = useState('All Towns')
  const [selectedSize, setSelectedSize] = useState('All Sizes')
  const [selectedStatus, setSelectedStatus] = useState('All Status')
  const [selectedType, setSelectedType] = useState('All Types')
  const [selectedHouse, setSelectedHouse] = useState<House | null>(null)
  const [bidAmount, setBidAmount] = useState('')
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 12

  // Real API hooks
  const { data: housesData, isLoading, error } = useHouses({
    town: selectedTown === 'All Towns' ? undefined : selectedTown,
    size: selectedSize === 'All Sizes' ? undefined : selectedSize as House['size'],
    status: selectedStatus === 'All Status' ? undefined : selectedStatus as House['status'],
    type: selectedType === 'All Types' ? undefined : selectedType as House['type'],
    page: currentPage,
    pageSize,
  })
  const { data: myHousesData, isLoading: myHousesLoading } = useMyHouses()
  const bidOnHouse = useBidOnHouse()
  const leaveHouse = useLeaveHouse()

  const houses = housesData?.data || []
  const myHouses = myHousesData || []

  const filteredHouses = houses.filter(house =>
    house.name.toLowerCase().includes(searchQuery.toLowerCase())
  )

  const handleBid = async () => {
    if (!selectedHouse || !bidAmount) return
    await bidOnHouse.mutateAsync({ id: selectedHouse.id, amount: parseInt(bidAmount) })
    setSelectedHouse(null)
    setBidAmount('')
  }

  const handleLeave = async (houseId: string) => {
    if (confirm('Are you sure you want to leave this house?')) {
      await leaveHouse.mutateAsync(houseId)
    }
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
          <h1 className="text-2xl font-bold text-white mb-1">Houses</h1>
          <p className="text-slate-400">Browse and manage your houses</p>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid grid-cols-4 gap-4"
      >
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Home className="w-5 h-5 text-amber-400 mb-2" />
          <p className="text-2xl font-bold text-white">{houses.length}</p>
          <p className="text-xs text-slate-500">Total Houses</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Gavel className="w-5 h-5 text-blue-400 mb-2" />
          <p className="text-2xl font-bold text-white">{houses.filter(h => h.status === 'auction').length}</p>
          <p className="text-xs text-slate-500">On Auction</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <MapPin className="w-5 h-5 text-emerald-400 mb-2" />
          <p className="text-2xl font-bold text-white">{houses.filter(h => h.status === 'available').length}</p>
          <p className="text-xs text-slate-500">Available</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <User className="w-5 h-5 text-purple-400 mb-2" />
          <p className="text-2xl font-bold text-white">{myHouses.length}</p>
          <p className="text-xs text-slate-500">Owned</p>
        </div>
      </motion.div>

      {/* Tabs */}
      <Tabs.Root defaultValue="browse">
        <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1 mb-6">
          <Tabs.Trigger
            value="browse"
            className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400"
          >
            <Home className="w-4 h-4" />
            Browse Houses
          </Tabs.Trigger>
          <Tabs.Trigger
            value="mine"
            className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400"
          >
            <User className="w-4 h-4" />
            My Houses
          </Tabs.Trigger>
        </Tabs.List>

        {/* Browse Tab */}
        <Tabs.Content value="browse">
          {/* Filters */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
            className="flex flex-wrap gap-4 mb-6"
          >
            <div className="relative flex-1 min-w-[200px]">
              <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
              <input
                type="text"
                placeholder="Search houses..."
                value={searchQuery}
                onChange={e => setSearchQuery(e.target.value)}
                className="w-full bg-slate-800/50 border border-slate-700/50 rounded-lg pl-10 pr-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
              />
            </div>
            <select
              value={selectedTown}
              onChange={e => setSelectedTown(e.target.value)}
              className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none"
            >
              {towns.map(town => <option key={town} value={town}>{town}</option>)}
            </select>
            <select
              value={selectedSize}
              onChange={e => setSelectedSize(e.target.value)}
              className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none capitalize"
            >
              {sizes.map(size => <option key={size} value={size}>{size.replace('_', ' ')}</option>)}
            </select>
            <select
              value={selectedStatus}
              onChange={e => setSelectedStatus(e.target.value)}
              className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none capitalize"
            >
              {statuses.map(status => <option key={status} value={status}>{status}</option>)}
            </select>
          </motion.div>

          {/* Houses Grid */}
          {isLoading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
            </div>
          ) : error ? (
            <div className="flex items-center justify-center py-12 text-red-400">
              <AlertTriangle className="w-5 h-5 mr-2" />
              Failed to load houses
            </div>
          ) : filteredHouses.length === 0 ? (
            <div className="text-center py-16 bg-slate-800/50 border border-slate-700/50 rounded-xl">
              <Home className="w-16 h-16 text-slate-700 mx-auto mb-4" />
              <h3 className="text-xl font-semibold text-white mb-2">No houses found</h3>
              <p className="text-slate-500">Try adjusting your filters</p>
            </div>
          ) : (
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredHouses.map((house, idx) => (
                <motion.div
                  key={house.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: idx * 0.05 }}
                  className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 hover:border-amber-500/30 transition cursor-pointer"
                  onClick={() => setSelectedHouse(house)}
                >
                  <div className="flex items-start justify-between mb-3">
                    <div>
                      <h3 className="text-white font-medium">{house.name}</h3>
                      <div className="flex items-center gap-1 text-slate-400 text-sm">
                        <MapPin className="w-3 h-3" />
                        {house.town}
                      </div>
                    </div>
                    <div className="flex flex-col gap-1 items-end">
                      <span className={`px-2 py-0.5 rounded text-xs capitalize ${sizeColors[house.size] || 'bg-slate-500/20 text-slate-400'}`}>
                        {house.size.replace('_', ' ')}
                      </span>
                      <span className={`px-2 py-0.5 rounded text-xs capitalize ${statusColors[house.status] || 'bg-slate-500/20 text-slate-400'}`}>
                        {house.status}
                      </span>
                    </div>
                  </div>

                  <div className="grid grid-cols-3 gap-2 mb-3 text-sm">
                    <div className="flex items-center gap-1 text-slate-400">
                      <Square className="w-4 h-4" />
                      {house.sqm} sqm
                    </div>
                    <div className="flex items-center gap-1 text-slate-400">
                      <Bed className="w-4 h-4" />
                      {house.beds} beds
                    </div>
                    <div className="flex items-center gap-1 text-slate-400">
                      <Coins className="w-4 h-4" />
                      {formatGold(house.rent)}/mo
                    </div>
                  </div>

                  {house.status === 'auction' && house.auction && (
                    <div className="bg-slate-900/50 rounded-lg p-3">
                      <div className="flex items-center justify-between">
                        <div>
                          <p className="text-slate-500 text-xs">Current Bid</p>
                          <p className="text-amber-400 font-bold">{formatGold(house.auction.currentBid)}</p>
                        </div>
                        <div className="flex items-center gap-1 text-slate-400 text-sm">
                          <Clock className="w-4 h-4" />
                          Ends soon
                        </div>
                      </div>
                    </div>
                  )}

                  {house.status === 'rented' && house.owner && (
                    <div className="text-slate-400 text-sm">
                      Owner: <span className="text-white">{house.owner.name}</span>
                    </div>
                  )}
                </motion.div>
              ))}
            </div>
          )}
        </Tabs.Content>

        {/* My Houses Tab */}
        <Tabs.Content value="mine">
          {myHousesLoading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
            </div>
          ) : myHouses.length === 0 ? (
            <div className="text-center py-16 bg-slate-800/50 border border-slate-700/50 rounded-xl">
              <Home className="w-16 h-16 text-slate-700 mx-auto mb-4" />
              <h3 className="text-xl font-semibold text-white mb-2">No houses owned</h3>
              <p className="text-slate-500">Browse available houses to find your perfect home</p>
            </div>
          ) : (
            <div className="grid md:grid-cols-2 gap-4">
              {myHouses.map((house, idx) => (
                <motion.div
                  key={house.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: idx * 0.1 }}
                  className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4"
                >
                  <div className="flex items-start justify-between mb-3">
                    <div>
                      <h3 className="text-white font-medium">{house.name}</h3>
                      <p className="text-slate-400 text-sm">
                        {house.town} • Character: {house.character}
                      </p>
                    </div>
                    <span className={`px-2 py-0.5 rounded text-xs capitalize ${sizeColors[house.size]}`}>
                      {house.size.replace('_', ' ')}
                    </span>
                  </div>

                  <div className="grid grid-cols-3 gap-2 mb-4 text-sm">
                    <div className="flex items-center gap-1 text-slate-400">
                      <Square className="w-4 h-4" />
                      {house.sqm} sqm
                    </div>
                    <div className="flex items-center gap-1 text-slate-400">
                      <Bed className="w-4 h-4" />
                      {house.beds} beds
                    </div>
                    <div className="flex items-center gap-1 text-slate-400">
                      <Coins className="w-4 h-4" />
                      {formatGold(house.rent)}/mo
                    </div>
                  </div>

                  <div className="bg-slate-900/50 rounded-lg p-3 mb-4">
                    <div className="flex items-center justify-between">
                      <span className="text-slate-400 text-sm">Paid Until</span>
                      <span className="text-white">{house.paidUntil}</span>
                    </div>
                  </div>

                  <div className="flex gap-2">
                    <button className="flex-1 px-4 py-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-600/50 transition">
                      Pay Rent
                    </button>
                    <button
                      onClick={() => handleLeave(house.id)}
                      disabled={leaveHouse.isPending}
                      className="px-4 py-2 bg-red-500/20 text-red-400 rounded-lg hover:bg-red-500/30 transition disabled:opacity-50"
                    >
                      Leave
                    </button>
                  </div>
                </motion.div>
              ))}
            </div>
          )}
        </Tabs.Content>
      </Tabs.Root>

      {/* House Detail / Bid Dialog */}
      <Dialog.Root open={!!selectedHouse} onOpenChange={() => setSelectedHouse(null)}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            {selectedHouse && (
              <>
                <Dialog.Title className="text-xl font-bold text-white mb-2">
                  {selectedHouse.name}
                </Dialog.Title>
                <p className="text-slate-400 text-sm mb-4">{selectedHouse.town}</p>

                <div className="grid grid-cols-3 gap-3 mb-4">
                  <div className="bg-slate-900/50 rounded-lg p-3 text-center">
                    <Square className="w-5 h-5 text-slate-400 mx-auto mb-1" />
                    <p className="text-white font-medium">{selectedHouse.sqm}</p>
                    <p className="text-slate-500 text-xs">sqm</p>
                  </div>
                  <div className="bg-slate-900/50 rounded-lg p-3 text-center">
                    <Bed className="w-5 h-5 text-slate-400 mx-auto mb-1" />
                    <p className="text-white font-medium">{selectedHouse.beds}</p>
                    <p className="text-slate-500 text-xs">beds</p>
                  </div>
                  <div className="bg-slate-900/50 rounded-lg p-3 text-center">
                    <Coins className="w-5 h-5 text-slate-400 mx-auto mb-1" />
                    <p className="text-white font-medium">{formatGold(selectedHouse.rent)}</p>
                    <p className="text-slate-500 text-xs">rent/mo</p>
                  </div>
                </div>

                {selectedHouse.status === 'auction' && selectedHouse.auction && (
                  <>
                    <div className="bg-slate-900/50 rounded-lg p-4 mb-4">
                      <p className="text-slate-400 text-sm mb-2">Current Bid</p>
                      <p className="text-2xl font-bold text-amber-400">
                        {formatGold(selectedHouse.auction.currentBid)}
                      </p>
                      {selectedHouse.auction.highestBidder && (
                        <p className="text-slate-500 text-sm">
                          by {selectedHouse.auction.highestBidder.name}
                        </p>
                      )}
                    </div>

                    <div className="mb-4">
                      <label className="block text-sm text-slate-400 mb-2">Your Bid</label>
                      <input
                        type="number"
                        value={bidAmount}
                        onChange={e => setBidAmount(e.target.value)}
                        placeholder="Enter amount..."
                        className="w-full bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
                      />
                    </div>
                  </>
                )}

                <div className="flex gap-3">
                  <Dialog.Close className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                    Close
                  </Dialog.Close>
                  {selectedHouse.status === 'auction' && (
                    <button
                      onClick={handleBid}
                      disabled={!bidAmount || bidOnHouse.isPending}
                      className="flex-1 px-4 py-2 bg-amber-500 text-black rounded-lg hover:bg-amber-400 transition disabled:opacity-50 flex items-center justify-center gap-2"
                    >
                      {bidOnHouse.isPending ? (
                        <Loader2 className="w-4 h-4 animate-spin" />
                      ) : (
                        'Place Bid'
                      )}
                    </button>
                  )}
                </div>
              </>
            )}
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}
