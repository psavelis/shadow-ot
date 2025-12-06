'use client'

import { useState } from 'react'
import Link from 'next/link'
import { motion } from 'framer-motion'
import {
  Home, Search, Filter, MapPin, Users, Clock, Coins,
  ChevronRight, Star, Heart, Eye, Key, Calendar, Crown
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'
import * as Dialog from '@radix-ui/react-dialog'

const houses = [
  {
    id: 1,
    name: 'Thais Manor',
    town: 'Thais',
    type: 'house',
    size: 'Large',
    sqm: 156,
    beds: 8,
    rent: 15000,
    status: 'available',
    auctionEnd: null,
    currentBid: null,
    owner: null,
    realm: 'Shadowlands',
    position: { x: 32369, y: 32241, z: 7 },
  },
  {
    id: 2,
    name: 'Venore Shop',
    town: 'Venore',
    type: 'guildhall',
    size: 'Extra Large',
    sqm: 324,
    beds: 15,
    rent: 45000,
    status: 'auction',
    auctionEnd: '2h 15m',
    currentBid: 125000,
    owner: null,
    realm: 'Shadowlands',
    position: { x: 32954, y: 32076, z: 6 },
  },
  {
    id: 3,
    name: 'Edron Villa',
    town: 'Edron',
    type: 'house',
    size: 'Medium',
    sqm: 89,
    beds: 4,
    rent: 8500,
    status: 'rented',
    auctionEnd: null,
    currentBid: null,
    owner: 'ShadowKnight',
    realm: 'Shadowlands',
    position: { x: 33217, y: 31814, z: 8 },
  },
  {
    id: 4,
    name: 'Carlin Cottage',
    town: 'Carlin',
    type: 'house',
    size: 'Small',
    sqm: 42,
    beds: 2,
    rent: 3500,
    status: 'available',
    auctionEnd: null,
    currentBid: null,
    owner: null,
    realm: 'Mythara',
    position: { x: 32360, y: 31782, z: 7 },
  },
  {
    id: 5,
    name: 'Ab\'Dendriel Treehouse',
    town: 'Ab\'Dendriel',
    type: 'house',
    size: 'Medium',
    sqm: 78,
    beds: 3,
    rent: 7200,
    status: 'auction',
    auctionEnd: '5d 12h',
    currentBid: 85000,
    owner: null,
    realm: 'Aetheria',
    position: { x: 32732, y: 31634, z: 7 },
  },
]

const myHouses = [
  {
    id: 3,
    name: 'Edron Villa',
    town: 'Edron',
    character: 'ShadowKnight',
    rent: 8500,
    nextPayment: '12 days',
    paidUntil: '2025-01-15',
  },
]

const towns = ['All Towns', 'Thais', 'Venore', 'Edron', 'Carlin', 'Ab\'Dendriel', 'Kazordoon', 'Darashia', 'Ankrahmun', 'Port Hope', 'Liberty Bay', 'Yalahar']
const sizes = ['All Sizes', 'Small', 'Medium', 'Large', 'Extra Large']
const statuses = ['All Status', 'Available', 'Auction', 'Rented']
const types = ['All Types', 'House', 'Guildhall']

const sizeColors: Record<string, string> = {
  'Small': 'bg-slate-500/20 text-slate-400',
  'Medium': 'bg-blue-500/20 text-blue-400',
  'Large': 'bg-purple-500/20 text-purple-400',
  'Extra Large': 'bg-amber-500/20 text-amber-400',
}

const statusColors: Record<string, { bg: string; text: string }> = {
  'available': { bg: 'bg-emerald-500/20', text: 'text-emerald-400' },
  'auction': { bg: 'bg-amber-500/20', text: 'text-amber-400' },
  'rented': { bg: 'bg-slate-500/20', text: 'text-slate-400' },
}

export default function HousesPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedTown, setSelectedTown] = useState('All Towns')
  const [selectedSize, setSelectedSize] = useState('All Sizes')
  const [selectedStatus, setSelectedStatus] = useState('All Status')
  const [selectedType, setSelectedType] = useState('All Types')
  const [showBidDialog, setShowBidDialog] = useState(false)
  const [selectedHouse, setSelectedHouse] = useState<any>(null)

  const filteredHouses = houses.filter(house => {
    const matchesSearch = house.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         house.town.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesTown = selectedTown === 'All Towns' || house.town === selectedTown
    const matchesSize = selectedSize === 'All Sizes' || house.size === selectedSize
    const matchesStatus = selectedStatus === 'All Status' || house.status === selectedStatus.toLowerCase()
    const matchesType = selectedType === 'All Types' || house.type === selectedType.toLowerCase()
    return matchesSearch && matchesTown && matchesSize && matchesStatus && matchesType
  })

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <h1 className="text-2xl font-bold text-white mb-1">House Browser</h1>
        <p className="text-slate-400">Find and rent houses across all realms</p>
      </motion.div>

      {/* Stats */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid grid-cols-4 gap-4"
      >
        {[
          { label: 'Available Houses', value: '142', icon: Home, color: 'emerald' },
          { label: 'Active Auctions', value: '23', icon: Clock, color: 'amber' },
          { label: 'Your Houses', value: myHouses.length.toString(), icon: Key, color: 'blue' },
          { label: 'Total Rent/Month', value: `${(myHouses.reduce((sum, h) => sum + h.rent, 0) / 1000).toFixed(1)}k`, icon: Coins, color: 'purple' },
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
      <Tabs.Root defaultValue="browse">
        <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1 mb-6">
          <Tabs.Trigger
            value="browse"
            className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400 data-[state=inactive]:hover:text-white"
          >
            <Search className="w-4 h-4" />
            Browse Houses
          </Tabs.Trigger>
          <Tabs.Trigger
            value="my"
            className="flex-1 flex items-center justify-center gap-2 px-4 py-2 rounded-lg text-sm font-medium transition data-[state=active]:bg-amber-500 data-[state=active]:text-white data-[state=inactive]:text-slate-400 data-[state=inactive]:hover:text-white"
          >
            <Key className="w-4 h-4" />
            My Houses
          </Tabs.Trigger>
        </Tabs.List>

        {/* Browse Tab */}
        <Tabs.Content value="browse">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-4"
          >
            {/* Filters */}
            <div className="flex flex-wrap gap-3">
              <div className="flex-1 min-w-[200px] relative">
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
                className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
              >
                {towns.map(town => <option key={town} value={town}>{town}</option>)}
              </select>
              <select
                value={selectedSize}
                onChange={e => setSelectedSize(e.target.value)}
                className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
              >
                {sizes.map(size => <option key={size} value={size}>{size}</option>)}
              </select>
              <select
                value={selectedStatus}
                onChange={e => setSelectedStatus(e.target.value)}
                className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
              >
                {statuses.map(status => <option key={status} value={status}>{status}</option>)}
              </select>
              <select
                value={selectedType}
                onChange={e => setSelectedType(e.target.value)}
                className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
              >
                {types.map(type => <option key={type} value={type}>{type}</option>)}
              </select>
            </div>

            {/* House Grid */}
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredHouses.map((house, idx) => (
                <motion.div
                  key={house.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: idx * 0.05 }}
                  className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden hover:border-amber-500/30 transition group"
                >
                  {/* House Image Placeholder */}
                  <div className="h-32 bg-gradient-to-br from-slate-700/50 to-slate-800/50 flex items-center justify-center relative">
                    <Home className="w-12 h-12 text-slate-600" />
                    <div className="absolute top-3 left-3 flex gap-2">
                      <span className={`px-2 py-0.5 rounded text-xs ${statusColors[house.status].bg} ${statusColors[house.status].text}`}>
                        {house.status}
                      </span>
                      {house.type === 'guildhall' && (
                        <span className="px-2 py-0.5 bg-purple-500/20 text-purple-400 rounded text-xs">
                          Guildhall
                        </span>
                      )}
                    </div>
                    <span className={`absolute top-3 right-3 px-2 py-0.5 rounded text-xs ${sizeColors[house.size]}`}>
                      {house.size}
                    </span>
                  </div>

                  <div className="p-4">
                    <div className="flex items-start justify-between mb-2">
                      <div>
                        <h3 className="text-white font-bold group-hover:text-amber-400 transition">{house.name}</h3>
                        <p className="text-slate-500 text-sm flex items-center gap-1">
                          <MapPin className="w-3 h-3" />
                          {house.town}, {house.realm}
                        </p>
                      </div>
                    </div>

                    {/* Stats */}
                    <div className="grid grid-cols-3 gap-2 mb-4">
                      <div className="bg-slate-900/50 rounded-lg p-2 text-center">
                        <p className="text-white font-bold text-sm">{house.sqm}</p>
                        <p className="text-slate-500 text-xs">SQM</p>
                      </div>
                      <div className="bg-slate-900/50 rounded-lg p-2 text-center">
                        <p className="text-white font-bold text-sm">{house.beds}</p>
                        <p className="text-slate-500 text-xs">Beds</p>
                      </div>
                      <div className="bg-slate-900/50 rounded-lg p-2 text-center">
                        <p className="text-amber-400 font-bold text-sm">{(house.rent / 1000).toFixed(1)}k</p>
                        <p className="text-slate-500 text-xs">Rent</p>
                      </div>
                    </div>

                    {/* Actions */}
                    {house.status === 'available' && (
                      <button
                        onClick={() => {
                          setSelectedHouse(house)
                          setShowBidDialog(true)
                        }}
                        className="w-full py-2 bg-emerald-500 text-white rounded-lg text-sm font-medium hover:bg-emerald-400 transition"
                      >
                        Rent Now
                      </button>
                    )}
                    {house.status === 'auction' && (
                      <div className="space-y-2">
                        <div className="flex justify-between text-sm">
                          <span className="text-slate-400">Current Bid:</span>
                          <span className="text-white font-bold">{house.currentBid?.toLocaleString()} gold</span>
                        </div>
                        <div className="flex justify-between text-sm">
                          <span className="text-slate-400">Ends in:</span>
                          <span className="text-red-400">{house.auctionEnd}</span>
                        </div>
                        <button
                          onClick={() => {
                            setSelectedHouse(house)
                            setShowBidDialog(true)
                          }}
                          className="w-full py-2 bg-amber-500 text-white rounded-lg text-sm font-medium hover:bg-amber-400 transition"
                        >
                          Place Bid
                        </button>
                      </div>
                    )}
                    {house.status === 'rented' && (
                      <div className="flex items-center justify-between text-sm">
                        <span className="text-slate-400">Owner:</span>
                        <Link href={`/characters/${house.owner}`} className="text-amber-400 hover:text-amber-300">
                          {house.owner}
                        </Link>
                      </div>
                    )}
                  </div>
                </motion.div>
              ))}
            </div>
          </motion.div>
        </Tabs.Content>

        {/* My Houses Tab */}
        <Tabs.Content value="my">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
          >
            {myHouses.length > 0 ? (
              <div className="space-y-4">
                {myHouses.map((house, idx) => (
                  <div
                    key={house.id}
                    className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-4">
                        <div className="w-16 h-16 bg-gradient-to-br from-amber-500/20 to-orange-500/20 rounded-xl flex items-center justify-center">
                          <Home className="w-8 h-8 text-amber-400" />
                        </div>
                        <div>
                          <h3 className="text-white font-bold text-lg">{house.name}</h3>
                          <p className="text-slate-400 text-sm flex items-center gap-1">
                            <MapPin className="w-3 h-3" />
                            {house.town}
                          </p>
                          <p className="text-slate-500 text-sm">Owned by: {house.character}</p>
                        </div>
                      </div>
                      <div className="text-right">
                        <p className="text-slate-400 text-sm">Monthly Rent</p>
                        <p className="text-2xl font-bold text-amber-400">{house.rent.toLocaleString()} <span className="text-sm">gold</span></p>
                      </div>
                    </div>

                    <div className="mt-6 pt-4 border-t border-slate-700/50 grid grid-cols-3 gap-4">
                      <div>
                        <p className="text-slate-500 text-sm">Next Payment</p>
                        <p className="text-white font-medium flex items-center gap-1">
                          <Calendar className="w-4 h-4 text-amber-400" />
                          {house.nextPayment}
                        </p>
                      </div>
                      <div>
                        <p className="text-slate-500 text-sm">Paid Until</p>
                        <p className="text-white font-medium">{house.paidUntil}</p>
                      </div>
                      <div className="flex justify-end gap-2">
                        <button className="px-4 py-2 bg-slate-700/50 text-slate-300 rounded-lg text-sm hover:bg-slate-700 transition">
                          Pay Rent
                        </button>
                        <button className="px-4 py-2 bg-red-500/20 text-red-400 rounded-lg text-sm hover:bg-red-500/30 transition">
                          Leave House
                        </button>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            ) : (
              <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-12 text-center">
                <Home className="w-16 h-16 text-slate-600 mx-auto mb-4" />
                <h3 className="text-white font-bold text-lg mb-2">No Houses Yet</h3>
                <p className="text-slate-400 text-sm mb-6">
                  You don't own any houses. Browse available houses to find your perfect home!
                </p>
                <button className="px-6 py-2 bg-amber-500 text-white rounded-lg font-medium hover:bg-amber-400 transition">
                  Browse Houses
                </button>
              </div>
            )}
          </motion.div>
        </Tabs.Content>
      </Tabs.Root>

      {/* Bid/Rent Dialog */}
      <Dialog.Root open={showBidDialog} onOpenChange={setShowBidDialog}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md">
            <Dialog.Title className="text-xl font-bold text-white mb-2">
              {selectedHouse?.status === 'available' ? 'Rent House' : 'Place Bid'}
            </Dialog.Title>
            <Dialog.Description className="text-slate-400 text-sm mb-6">
              {selectedHouse?.name} in {selectedHouse?.town}
            </Dialog.Description>

            {selectedHouse && (
              <div className="space-y-4 mb-6">
                <div className="bg-slate-900/50 rounded-lg p-4">
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <p className="text-slate-500 text-sm">Size</p>
                      <p className="text-white font-medium">{selectedHouse.sqm} SQM</p>
                    </div>
                    <div>
                      <p className="text-slate-500 text-sm">Beds</p>
                      <p className="text-white font-medium">{selectedHouse.beds}</p>
                    </div>
                    <div>
                      <p className="text-slate-500 text-sm">Monthly Rent</p>
                      <p className="text-amber-400 font-medium">{selectedHouse.rent.toLocaleString()} gold</p>
                    </div>
                    {selectedHouse.status === 'auction' && (
                      <div>
                        <p className="text-slate-500 text-sm">Current Bid</p>
                        <p className="text-white font-medium">{selectedHouse.currentBid?.toLocaleString()} gold</p>
                      </div>
                    )}
                  </div>
                </div>

                <div>
                  <label className="text-slate-300 text-sm mb-2 block">Select Character</label>
                  <select className="w-full bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50">
                    <option value="1">ShadowKnight (342 EK)</option>
                    <option value="2">MysticDruid (298 ED)</option>
                  </select>
                </div>

                {selectedHouse.status === 'auction' && (
                  <div>
                    <label className="text-slate-300 text-sm mb-2 block">Your Bid</label>
                    <input
                      type="number"
                      placeholder="Enter bid amount"
                      className="w-full bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
                    />
                    <p className="text-slate-500 text-xs mt-1">Minimum bid: {((selectedHouse.currentBid || 0) + 1000).toLocaleString()} gold</p>
                  </div>
                )}
              </div>
            )}

            <div className="flex gap-3">
              <Dialog.Close className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                Cancel
              </Dialog.Close>
              <button className="flex-1 px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-lg hover:from-amber-400 hover:to-orange-500 transition">
                {selectedHouse?.status === 'available' ? 'Rent Now' : 'Place Bid'}
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}

