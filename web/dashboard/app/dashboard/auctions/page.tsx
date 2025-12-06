'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import {
  Gavel, Clock, TrendingUp, User, Shield, Sword,
  Search, ChevronRight, Eye, Loader2, AlertTriangle
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'
import * as Dialog from '@radix-ui/react-dialog'
import { 
  useCharacterAuctions, useItemAuctions, useMyBids, usePlaceBid, useBuyout 
} from '@/shared/hooks/useDashboard'
import type { CharacterAuction, ItemAuction, Vocation } from '@/shared/api/endpoints'
import { getItemSprite, getOutfitSprite } from '@/shared/utils/assets'

const vocationColors: Record<string, string> = {
  'Elite Knight': 'text-amber-400 bg-amber-500/20',
  'Royal Paladin': 'text-emerald-400 bg-emerald-500/20',
  'Master Sorcerer': 'text-blue-400 bg-blue-500/20',
  'Elder Druid': 'text-purple-400 bg-purple-500/20',
  'Knight': 'text-amber-400 bg-amber-500/20',
  'Paladin': 'text-emerald-400 bg-emerald-500/20',
  'Sorcerer': 'text-blue-400 bg-blue-500/20',
  'Druid': 'text-purple-400 bg-purple-500/20',
}

const rarityColors: Record<string, string> = {
  'common': 'text-slate-400 bg-slate-500/20',
  'uncommon': 'text-green-400 bg-green-500/20',
  'rare': 'text-blue-400 bg-blue-500/20',
  'epic': 'text-purple-400 bg-purple-500/20',
  'legendary': 'text-orange-400 bg-orange-500/20',
}

function formatGold(amount: number) {
  if (amount >= 1000000) return `${(amount / 1000000).toFixed(2)}kk`
  if (amount >= 1000) return `${(amount / 1000).toFixed(1)}k`
  return amount.toLocaleString()
}

function formatCoins(amount: number) {
  return amount.toLocaleString()
}

function formatTimeLeft(endTime: string) {
  const end = new Date(endTime)
  const now = new Date()
  const diffMs = end.getTime() - now.getTime()
  if (diffMs <= 0) return 'Ended'
  
  const hours = Math.floor(diffMs / (1000 * 60 * 60))
  const mins = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60))
  
  if (hours >= 24) return `${Math.floor(hours / 24)}d ${hours % 24}h`
  return `${hours}h ${mins}m`
}

export default function AuctionsPage() {
  const [activeTab, setActiveTab] = useState('characters')
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedCharAuction, setSelectedCharAuction] = useState<CharacterAuction | null>(null)
  const [selectedItemAuction, setSelectedItemAuction] = useState<ItemAuction | null>(null)
  const [bidAmount, setBidAmount] = useState('')
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 12

  // Real API hooks
  const { data: charAuctionsData, isLoading: charLoading, error: charError } = useCharacterAuctions({
    page: currentPage,
    pageSize,
  })
  const { data: itemAuctionsData, isLoading: itemLoading, error: itemError } = useItemAuctions({
    page: currentPage,
    pageSize,
  })
  const { data: myBidsData, isLoading: bidsLoading } = useMyBids()
  const placeBid = usePlaceBid()
  const buyout = useBuyout()

  const characterAuctions = charAuctionsData?.data || []
  const itemAuctions = itemAuctionsData?.data || []
  const myBids = myBidsData || []

  const filteredCharAuctions = characterAuctions.filter(a =>
    a.characterName.toLowerCase().includes(searchQuery.toLowerCase())
  )

  const filteredItemAuctions = itemAuctions.filter(a =>
    a.itemName.toLowerCase().includes(searchQuery.toLowerCase())
  )

  const handlePlaceBid = async () => {
    const auction = selectedCharAuction || selectedItemAuction
    if (!auction || !bidAmount) return
    
    await placeBid.mutateAsync({ id: auction.id, amount: parseInt(bidAmount) })
    setSelectedCharAuction(null)
    setSelectedItemAuction(null)
    setBidAmount('')
  }

  const handleBuyout = async (auctionId: string) => {
    await buyout.mutateAsync(auctionId)
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
          <h1 className="text-2xl font-bold text-white mb-1">Auction House</h1>
          <p className="text-slate-400">Buy and sell characters & items</p>
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
          <Gavel className="w-5 h-5 text-amber-400 mb-2" />
          <p className="text-2xl font-bold text-white">{characterAuctions.length + itemAuctions.length}</p>
          <p className="text-xs text-slate-500">Active Auctions</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <User className="w-5 h-5 text-blue-400 mb-2" />
          <p className="text-2xl font-bold text-white">{characterAuctions.length}</p>
          <p className="text-xs text-slate-500">Characters</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Sword className="w-5 h-5 text-purple-400 mb-2" />
          <p className="text-2xl font-bold text-white">{itemAuctions.length}</p>
          <p className="text-xs text-slate-500">Items</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <TrendingUp className="w-5 h-5 text-emerald-400 mb-2" />
          <p className="text-2xl font-bold text-white">{myBids.filter(b => b.status === 'winning').length}</p>
          <p className="text-xs text-slate-500">Winning Bids</p>
        </div>
      </motion.div>

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
          placeholder="Search auctions..."
          value={searchQuery}
          onChange={e => setSearchQuery(e.target.value)}
          className="w-full bg-slate-800/50 border border-slate-700/50 rounded-lg pl-10 pr-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
        />
      </motion.div>

      {/* Tabs */}
      <Tabs.Root value={activeTab} onValueChange={setActiveTab}>
        <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1 mb-6">
          <Tabs.Trigger
            value="characters"
            className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400"
          >
            <User className="w-4 h-4" />
            Characters
          </Tabs.Trigger>
          <Tabs.Trigger
            value="items"
            className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400"
          >
            <Sword className="w-4 h-4" />
            Items
          </Tabs.Trigger>
          <Tabs.Trigger
            value="mybids"
            className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400"
          >
            <TrendingUp className="w-4 h-4" />
            My Bids
            {myBids.filter(b => b.status === 'outbid').length > 0 && (
              <span className="px-1.5 py-0.5 bg-red-500 text-white text-xs rounded-full">
                {myBids.filter(b => b.status === 'outbid').length}
              </span>
            )}
          </Tabs.Trigger>
        </Tabs.List>

        {/* Character Auctions */}
        <Tabs.Content value="characters">
          {charLoading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
            </div>
          ) : charError ? (
            <div className="flex items-center justify-center py-12 text-red-400">
              <AlertTriangle className="w-5 h-5 mr-2" />
              Failed to load auctions
            </div>
          ) : filteredCharAuctions.length === 0 ? (
            <div className="text-center py-16 bg-slate-800/50 border border-slate-700/50 rounded-xl">
              <Gavel className="w-16 h-16 text-slate-700 mx-auto mb-4" />
              <h3 className="text-xl font-semibold text-white mb-2">No character auctions</h3>
              <p className="text-slate-500">Check back later for new listings</p>
            </div>
          ) : (
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredCharAuctions.map((auction, idx) => (
                <motion.div
                  key={auction.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: idx * 0.05 }}
                  className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 hover:border-amber-500/30 transition cursor-pointer"
                  onClick={() => setSelectedCharAuction(auction)}
                >
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center gap-3">
                      <div className="w-12 h-12 bg-slate-700/50 rounded-lg overflow-hidden">
                        <img
                          src={getOutfitSprite({ lookType: 128 })}
                          alt={auction.characterName}
                          className="w-full h-full object-cover"
                        />
                      </div>
                      <div>
                        <h3 className="text-white font-medium">{auction.characterName}</h3>
                        <p className="text-slate-400 text-sm">Level {auction.level}</p>
                      </div>
                    </div>
                    <span className={`px-2 py-0.5 rounded text-xs ${vocationColors[auction.vocation] || 'text-slate-400 bg-slate-500/20'}`}>
                      {auction.vocation}
                    </span>
                  </div>

                  <div className="flex items-center justify-between mb-3 text-sm">
                    <div className="flex items-center gap-1 text-slate-400">
                      <Clock className="w-4 h-4" />
                      {formatTimeLeft(auction.endTime)}
                    </div>
                    <div className="text-slate-400">
                      {auction.bidCount} bids
                    </div>
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <p className="text-slate-500 text-xs">Current Bid</p>
                      <p className="text-amber-400 font-bold">{formatCoins(auction.currentBid)} TC</p>
                    </div>
                    {auction.buyoutPrice && (
                      <div className="text-right">
                        <p className="text-slate-500 text-xs">Buyout</p>
                        <p className="text-emerald-400 font-bold">{formatCoins(auction.buyoutPrice)} TC</p>
                      </div>
                    )}
                  </div>
                </motion.div>
              ))}
            </div>
          )}
        </Tabs.Content>

        {/* Item Auctions */}
        <Tabs.Content value="items">
          {itemLoading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
            </div>
          ) : itemError ? (
            <div className="flex items-center justify-center py-12 text-red-400">
              <AlertTriangle className="w-5 h-5 mr-2" />
              Failed to load auctions
            </div>
          ) : filteredItemAuctions.length === 0 ? (
            <div className="text-center py-16 bg-slate-800/50 border border-slate-700/50 rounded-xl">
              <Gavel className="w-16 h-16 text-slate-700 mx-auto mb-4" />
              <h3 className="text-xl font-semibold text-white mb-2">No item auctions</h3>
              <p className="text-slate-500">Check back later for new listings</p>
            </div>
          ) : (
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredItemAuctions.map((auction, idx) => (
                <motion.div
                  key={auction.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: idx * 0.05 }}
                  className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 hover:border-amber-500/30 transition cursor-pointer"
                  onClick={() => setSelectedItemAuction(auction)}
                >
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center gap-3">
                      <div className="w-12 h-12 bg-slate-700/50 rounded-lg flex items-center justify-center">
                        <img
                          src={getItemSprite(auction.spriteId)}
                          alt={auction.itemName}
                          className="w-10 h-10 object-contain"
                        />
                      </div>
                      <div>
                        <h3 className="text-white font-medium">{auction.itemName}</h3>
                        <p className="text-slate-400 text-sm capitalize">{auction.itemType}</p>
                      </div>
                    </div>
                    <span className={`px-2 py-0.5 rounded text-xs capitalize ${rarityColors[auction.rarity] || 'text-slate-400 bg-slate-500/20'}`}>
                      {auction.rarity}
                    </span>
                  </div>

                  <div className="flex items-center justify-between mb-3 text-sm">
                    <div className="flex items-center gap-1 text-slate-400">
                      <Clock className="w-4 h-4" />
                      {formatTimeLeft(auction.endTime)}
                    </div>
                    <div className="text-slate-400">
                      {auction.bidCount} bids
                    </div>
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <p className="text-slate-500 text-xs">Current Bid</p>
                      <p className="text-amber-400 font-bold">{formatGold(auction.currentBid)}</p>
                    </div>
                    {auction.buyoutPrice && (
                      <div className="text-right">
                        <p className="text-slate-500 text-xs">Buyout</p>
                        <p className="text-emerald-400 font-bold">{formatGold(auction.buyoutPrice)}</p>
                      </div>
                    )}
                  </div>
                </motion.div>
              ))}
            </div>
          )}
        </Tabs.Content>

        {/* My Bids */}
        <Tabs.Content value="mybids">
          {bidsLoading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
            </div>
          ) : myBids.length === 0 ? (
            <div className="text-center py-16 bg-slate-800/50 border border-slate-700/50 rounded-xl">
              <TrendingUp className="w-16 h-16 text-slate-700 mx-auto mb-4" />
              <h3 className="text-xl font-semibold text-white mb-2">No active bids</h3>
              <p className="text-slate-500">Start bidding on auctions to see them here</p>
            </div>
          ) : (
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
              <table className="w-full">
                <thead className="bg-slate-900/50">
                  <tr className="text-slate-400 text-sm">
                    <th className="text-left p-4">Auction</th>
                    <th className="text-left p-4">Type</th>
                    <th className="text-left p-4">My Bid</th>
                    <th className="text-left p-4">Current</th>
                    <th className="text-left p-4">Status</th>
                    <th className="text-left p-4">Time Left</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-slate-700/50">
                  {myBids.map(bid => (
                    <tr key={bid.auctionId} className="hover:bg-slate-700/30 transition">
                      <td className="p-4 text-white font-medium">{bid.name}</td>
                      <td className="p-4 text-slate-400 capitalize">{bid.type}</td>
                      <td className="p-4 text-white">{formatGold(bid.myBid)}</td>
                      <td className="p-4 text-amber-400">{formatGold(bid.currentBid)}</td>
                      <td className="p-4">
                        <span className={`px-2 py-0.5 rounded text-xs ${
                          bid.status === 'winning' ? 'bg-emerald-500/20 text-emerald-400' :
                          bid.status === 'outbid' ? 'bg-red-500/20 text-red-400' :
                          bid.status === 'won' ? 'bg-blue-500/20 text-blue-400' :
                          'bg-slate-500/20 text-slate-400'
                        }`}>
                          {bid.status}
                        </span>
                      </td>
                      <td className="p-4 text-slate-400">{bid.endTime}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </Tabs.Content>
      </Tabs.Root>

      {/* Bid Dialog */}
      <Dialog.Root 
        open={!!(selectedCharAuction || selectedItemAuction)} 
        onOpenChange={() => { setSelectedCharAuction(null); setSelectedItemAuction(null) }}
      >
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm z-50" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md z-50">
            {(selectedCharAuction || selectedItemAuction) && (
              <>
                <Dialog.Title className="text-xl font-bold text-white mb-4">
                  Place Bid
                </Dialog.Title>
                
                <div className="bg-slate-900/50 rounded-lg p-4 mb-4">
                  <h3 className="text-white font-medium">
                    {selectedCharAuction?.characterName || selectedItemAuction?.itemName}
                  </h3>
                  <p className="text-slate-400 text-sm">
                    Current bid: {formatGold((selectedCharAuction?.currentBid || selectedItemAuction?.currentBid) || 0)}
                  </p>
                  <p className="text-slate-500 text-xs mt-1">
                    Minimum: {formatGold(((selectedCharAuction?.currentBid || selectedItemAuction?.currentBid) || 0) + 1000)}
                  </p>
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

                <div className="flex gap-3">
                  <Dialog.Close className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                    Cancel
                  </Dialog.Close>
                  {(selectedCharAuction?.buyoutPrice || selectedItemAuction?.buyoutPrice) && (
                    <button
                      onClick={() => handleBuyout(selectedCharAuction?.id || selectedItemAuction?.id || '')}
                      disabled={buyout.isPending}
                      className="flex-1 px-4 py-2 bg-emerald-500 text-white rounded-lg hover:bg-emerald-400 transition disabled:opacity-50"
                    >
                      Buyout
                    </button>
                  )}
                  <button
                    onClick={handlePlaceBid}
                    disabled={!bidAmount || placeBid.isPending}
                    className="flex-1 px-4 py-2 bg-amber-500 text-black rounded-lg hover:bg-amber-400 transition disabled:opacity-50 flex items-center justify-center gap-2"
                  >
                    {placeBid.isPending ? (
                      <Loader2 className="w-4 h-4 animate-spin" />
                    ) : (
                      'Place Bid'
                    )}
                  </button>
                </div>
              </>
            )}
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}
