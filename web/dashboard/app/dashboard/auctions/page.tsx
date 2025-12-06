'use client'

import { useState } from 'react'
import Link from 'next/link'
import { motion } from 'framer-motion'
import {
  Gavel, Clock, TrendingUp, User, Shield, Sword, Heart,
  Search, Filter, ChevronRight, Star, Eye, ArrowUpRight,
  Timer, Users, Crown, Zap
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'
import * as Dialog from '@radix-ui/react-dialog'

const characterAuctions = [
  {
    id: 1,
    name: 'DragonSlayer',
    level: 487,
    vocation: 'Elite Knight',
    realm: 'Shadowlands',
    currentBid: 15000,
    buyout: 25000,
    bidders: 12,
    endTime: '2h 34m',
    skills: { sword: 118, shielding: 112, magic: 8 },
    items: ['Demon Armor', 'Magic Sword', 'Golden Boots'],
    seller: 'Anonymous',
  },
  {
    id: 2,
    name: 'MysticLord',
    level: 412,
    vocation: 'Master Sorcerer',
    realm: 'Mythara',
    currentBid: 8500,
    buyout: 15000,
    bidders: 8,
    endTime: '5h 12m',
    skills: { magic: 98, distance: 32, shielding: 45 },
    items: ['Spellbook of Mind Control', 'Blue Robe', 'Crown'],
    seller: 'MageTrader',
  },
  {
    id: 3,
    name: 'HolyPaladin',
    level: 356,
    vocation: 'Royal Paladin',
    realm: 'Aetheria',
    currentBid: 5200,
    buyout: null,
    bidders: 15,
    endTime: '12h 45m',
    skills: { distance: 108, shielding: 95, magic: 25 },
    items: ['Royal Crossbow', 'Paladin Armor', 'Boots of Haste'],
    seller: 'Anonymous',
  },
  {
    id: 4,
    name: 'NatureDruid',
    level: 298,
    vocation: 'Elder Druid',
    realm: 'Voidborne',
    currentBid: 3800,
    buyout: 6000,
    bidders: 6,
    endTime: '1d 3h',
    skills: { magic: 82, distance: 28, shielding: 52 },
    items: ['Druid Staff', 'Focus Cape', 'Gill Legs'],
    seller: 'DruidMaster',
  },
]

const itemAuctions = [
  {
    id: 1,
    name: 'Demon Armor',
    type: 'Armor',
    rarity: 'Legendary',
    currentBid: 45000000,
    buyout: 65000000,
    bidders: 23,
    endTime: '4h 15m',
    attributes: { armor: 16, level: 130 },
    seller: 'ItemHunter',
  },
  {
    id: 2,
    name: 'Magic Sword',
    type: 'Weapon',
    rarity: 'Epic',
    currentBid: 8500000,
    buyout: 12000000,
    bidders: 11,
    endTime: '8h 30m',
    attributes: { attack: 51, level: 80 },
    seller: 'Anonymous',
  },
  {
    id: 3,
    name: 'Golden Helmet',
    type: 'Helmet',
    rarity: 'Rare',
    currentBid: 3200000,
    buyout: 5000000,
    bidders: 8,
    endTime: '16h 20m',
    attributes: { armor: 12, level: 75 },
    seller: 'TreasureHunter',
  },
]

const myBids = [
  { id: 1, auctionId: 1, type: 'character', name: 'DragonSlayer', myBid: 14500, currentBid: 15000, status: 'outbid', endTime: '2h 34m' },
  { id: 2, auctionId: 2, type: 'item', name: 'Magic Sword', myBid: 8500000, currentBid: 8500000, status: 'winning', endTime: '8h 30m' },
]

const vocationColors: Record<string, string> = {
  'Elite Knight': 'text-amber-400 bg-amber-500/20',
  'Royal Paladin': 'text-emerald-400 bg-emerald-500/20',
  'Master Sorcerer': 'text-blue-400 bg-blue-500/20',
  'Elder Druid': 'text-purple-400 bg-purple-500/20',
}

const rarityColors: Record<string, string> = {
  'Common': 'text-slate-400 bg-slate-500/20',
  'Uncommon': 'text-green-400 bg-green-500/20',
  'Rare': 'text-blue-400 bg-blue-500/20',
  'Epic': 'text-purple-400 bg-purple-500/20',
  'Legendary': 'text-orange-400 bg-orange-500/20',
}

export default function AuctionsPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [showBidDialog, setShowBidDialog] = useState(false)
  const [selectedAuction, setSelectedAuction] = useState<any>(null)
  const [bidAmount, setBidAmount] = useState('')

  const formatGold = (n: number) => {
    if (n >= 1000000000) return `${(n / 1000000000).toFixed(2)}kkk`
    if (n >= 1000000) return `${(n / 1000000).toFixed(2)}kk`
    if (n >= 1000) return `${(n / 1000).toFixed(1)}k`
    return n.toLocaleString()
  }

  const handleBid = (auction: any) => {
    setSelectedAuction(auction)
    setBidAmount('')
    setShowBidDialog(true)
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
          <p className="text-slate-400">Bid on characters and rare items</p>
        </div>
        <button className="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-lg hover:from-amber-400 hover:to-orange-500 transition">
          <Gavel className="w-4 h-4" />
          Create Auction
        </button>
      </motion.div>

      {/* Stats */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid grid-cols-4 gap-4"
      >
        {[
          { label: 'Active Auctions', value: '247', icon: Gavel, color: 'amber' },
          { label: 'Your Active Bids', value: '2', icon: TrendingUp, color: 'blue' },
          { label: 'Ending Soon', value: '12', icon: Timer, color: 'red' },
          { label: 'Your Won Auctions', value: '5', icon: Crown, color: 'emerald' },
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

      {/* Main Content */}
      <Tabs.Root defaultValue="characters">
        <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1 mb-6">
          {[
            { id: 'characters', label: 'Character Bazaar', icon: User },
            { id: 'items', label: 'Item Auctions', icon: Sword },
            { id: 'mybids', label: 'My Bids', icon: TrendingUp },
          ].map(tab => (
            <Tabs.Trigger
              key={tab.id}
              value={tab.id}
              className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400 data-[state=inactive]:hover:text-white"
            >
              <tab.icon className="w-4 h-4" />
              {tab.label}
            </Tabs.Trigger>
          ))}
        </Tabs.List>

        {/* Search Bar */}
        <div className="flex gap-4 mb-6">
          <div className="flex-1 relative">
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
            <input
              type="text"
              placeholder="Search auctions..."
              value={searchQuery}
              onChange={e => setSearchQuery(e.target.value)}
              className="w-full bg-slate-800/50 border border-slate-700/50 rounded-lg pl-10 pr-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
            />
          </div>
          <button className="flex items-center gap-2 px-4 py-2 bg-slate-800/50 border border-slate-700/50 rounded-lg text-slate-300 hover:border-slate-600 transition">
            <Filter className="w-4 h-4" />
            Filters
          </button>
        </div>

        {/* Character Bazaar Tab */}
        <Tabs.Content value="characters">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="grid md:grid-cols-2 gap-4"
          >
            {characterAuctions.map((auction, idx) => (
              <motion.div
                key={auction.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: idx * 0.05 }}
                className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5 hover:border-amber-500/30 transition"
              >
                <div className="flex items-start justify-between mb-4">
                  <div className="flex items-center gap-4">
                    <div className="w-14 h-14 bg-gradient-to-br from-amber-500/20 to-orange-500/20 rounded-xl flex items-center justify-center">
                      <Shield className="w-7 h-7 text-amber-400" />
                    </div>
                    <div>
                      <h3 className="text-white font-bold text-lg">{auction.name}</h3>
                      <p className="text-slate-400 text-sm">Level {auction.level}</p>
                      <span className={`inline-block px-2 py-0.5 rounded text-xs mt-1 ${vocationColors[auction.vocation]}`}>
                        {auction.vocation}
                      </span>
                    </div>
                  </div>
                  <div className="flex items-center gap-1 text-red-400 text-sm">
                    <Clock className="w-4 h-4" />
                    {auction.endTime}
                  </div>
                </div>

                {/* Skills */}
                <div className="grid grid-cols-3 gap-2 mb-4">
                  {Object.entries(auction.skills).map(([skill, value]) => (
                    <div key={skill} className="bg-slate-900/50 rounded-lg p-2 text-center">
                      <p className="text-white font-bold">{value}</p>
                      <p className="text-slate-500 text-xs capitalize">{skill}</p>
                    </div>
                  ))}
                </div>

                {/* Items */}
                <div className="flex flex-wrap gap-1 mb-4">
                  {auction.items.slice(0, 3).map((item, i) => (
                    <span key={i} className="px-2 py-0.5 bg-slate-700/50 text-slate-300 text-xs rounded">
                      {item}
                    </span>
                  ))}
                </div>

                {/* Bidding Info */}
                <div className="flex items-end justify-between pt-4 border-t border-slate-700/50">
                  <div>
                    <p className="text-slate-500 text-xs mb-1">Current Bid ({auction.bidders} bidders)</p>
                    <p className="text-2xl font-bold text-white">{auction.currentBid.toLocaleString()} <span className="text-amber-400 text-sm">coins</span></p>
                  </div>
                  <div className="flex gap-2">
                    {auction.buyout && (
                      <button className="px-3 py-2 bg-emerald-500/20 text-emerald-400 rounded-lg text-sm hover:bg-emerald-500/30 transition">
                        Buyout: {auction.buyout.toLocaleString()}
                      </button>
                    )}
                    <button
                      onClick={() => handleBid(auction)}
                      className="px-4 py-2 bg-amber-500 text-white rounded-lg text-sm font-medium hover:bg-amber-400 transition"
                    >
                      Place Bid
                    </button>
                  </div>
                </div>
              </motion.div>
            ))}
          </motion.div>
        </Tabs.Content>

        {/* Item Auctions Tab */}
        <Tabs.Content value="items">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
          >
            <div className="grid grid-cols-12 gap-4 px-5 py-3 bg-slate-900/50 border-b border-slate-700/50 text-xs font-medium text-slate-500">
              <div className="col-span-4">Item</div>
              <div className="col-span-2">Current Bid</div>
              <div className="col-span-2">Buyout</div>
              <div className="col-span-2">Time Left</div>
              <div className="col-span-2"></div>
            </div>
            <div className="divide-y divide-slate-700/50">
              {itemAuctions.map((auction, idx) => (
                <motion.div
                  key={auction.id}
                  initial={{ opacity: 0, x: -10 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: idx * 0.05 }}
                  className="grid grid-cols-12 gap-4 px-5 py-4 hover:bg-slate-700/30 transition items-center"
                >
                  <div className="col-span-4 flex items-center gap-3">
                    <div className="w-10 h-10 bg-slate-700/50 rounded-lg flex items-center justify-center text-xl">
                      🛡️
                    </div>
                    <div>
                      <p className="text-white font-medium">{auction.name}</p>
                      <div className="flex items-center gap-2">
                        <span className="text-slate-500 text-xs">{auction.type}</span>
                        <span className={`px-1.5 py-0.5 rounded text-xs ${rarityColors[auction.rarity]}`}>
                          {auction.rarity}
                        </span>
                      </div>
                    </div>
                  </div>
                  <div className="col-span-2">
                    <p className="text-white font-medium">{formatGold(auction.currentBid)}</p>
                    <p className="text-slate-500 text-xs">{auction.bidders} bids</p>
                  </div>
                  <div className="col-span-2">
                    {auction.buyout ? (
                      <p className="text-emerald-400">{formatGold(auction.buyout)}</p>
                    ) : (
                      <p className="text-slate-500">-</p>
                    )}
                  </div>
                  <div className="col-span-2 flex items-center gap-1 text-red-400">
                    <Clock className="w-4 h-4" />
                    {auction.endTime}
                  </div>
                  <div className="col-span-2 flex justify-end gap-2">
                    <button
                      onClick={() => handleBid(auction)}
                      className="px-3 py-1.5 bg-amber-500 text-white rounded-lg text-sm hover:bg-amber-400 transition"
                    >
                      Bid
                    </button>
                  </div>
                </motion.div>
              ))}
            </div>
          </motion.div>
        </Tabs.Content>

        {/* My Bids Tab */}
        <Tabs.Content value="mybids">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
          >
            {myBids.length > 0 ? (
              <div className="divide-y divide-slate-700/50">
                {myBids.map((bid, idx) => (
                  <div key={bid.id} className="flex items-center justify-between px-5 py-4 hover:bg-slate-700/30 transition">
                    <div className="flex items-center gap-4">
                      <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                        bid.status === 'winning' ? 'bg-emerald-500/20' : 'bg-red-500/20'
                      }`}>
                        {bid.status === 'winning' ? (
                          <TrendingUp className="w-5 h-5 text-emerald-400" />
                        ) : (
                          <ArrowUpRight className="w-5 h-5 text-red-400" />
                        )}
                      </div>
                      <div>
                        <p className="text-white font-medium">{bid.name}</p>
                        <p className="text-slate-500 text-sm capitalize">{bid.type} auction</p>
                      </div>
                    </div>
                    <div className="text-right">
                      <p className="text-white">
                        Your bid: <span className={bid.status === 'winning' ? 'text-emerald-400' : 'text-slate-400'}>
                          {bid.type === 'character' ? bid.myBid.toLocaleString() : formatGold(bid.myBid)}
                        </span>
                      </p>
                      <p className="text-slate-500 text-sm">
                        {bid.status === 'outbid' && (
                          <span className="text-red-400">Outbid! Current: {bid.type === 'character' ? bid.currentBid.toLocaleString() : formatGold(bid.currentBid)}</span>
                        )}
                        {bid.status === 'winning' && (
                          <span className="text-emerald-400">Winning! Ends in {bid.endTime}</span>
                        )}
                      </p>
                    </div>
                    <button className="px-3 py-1.5 bg-amber-500 text-white rounded-lg text-sm hover:bg-amber-400 transition ml-4">
                      Increase Bid
                    </button>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-center py-12">
                <Gavel className="w-12 h-12 text-slate-600 mx-auto mb-4" />
                <p className="text-white font-medium mb-2">No Active Bids</p>
                <p className="text-slate-400 text-sm">Browse auctions to place your first bid</p>
              </div>
            )}
          </motion.div>
        </Tabs.Content>
      </Tabs.Root>

      {/* Bid Dialog */}
      <Dialog.Root open={showBidDialog} onOpenChange={setShowBidDialog}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md">
            <Dialog.Title className="text-xl font-bold text-white mb-2">
              Place Bid
            </Dialog.Title>
            <Dialog.Description className="text-slate-400 text-sm mb-6">
              {selectedAuction && `Bidding on: ${selectedAuction.name}`}
            </Dialog.Description>

            {selectedAuction && (
              <div className="space-y-4 mb-6">
                <div className="bg-slate-900/50 rounded-lg p-4">
                  <div className="flex justify-between mb-2">
                    <span className="text-slate-400">Current Bid</span>
                    <span className="text-white font-bold">
                      {selectedAuction.currentBid?.toLocaleString() || formatGold(selectedAuction.currentBid)}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-slate-400">Minimum Bid</span>
                    <span className="text-amber-400">
                      {((selectedAuction.currentBid || 0) * 1.05).toLocaleString()}
                    </span>
                  </div>
                </div>

                <div>
                  <label className="text-slate-300 text-sm mb-2 block">Your Bid</label>
                  <input
                    type="number"
                    value={bidAmount}
                    onChange={e => setBidAmount(e.target.value)}
                    placeholder="Enter bid amount"
                    className="w-full bg-slate-900 border border-slate-700 rounded-lg px-4 py-3 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
                  />
                </div>

                <p className="text-slate-500 text-xs">
                  By placing a bid, you agree to pay the amount if you win the auction.
                </p>
              </div>
            )}

            <div className="flex gap-3">
              <Dialog.Close className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                Cancel
              </Dialog.Close>
              <button
                className="flex-1 px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-lg hover:from-amber-400 hover:to-orange-500 transition"
              >
                Place Bid
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}

