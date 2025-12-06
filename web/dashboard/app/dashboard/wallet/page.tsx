'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Wallet, 
  Plus, 
  ArrowUpRight, 
  ArrowDownRight,
  ExternalLink,
  Copy,
  Check,
  RefreshCw,
  Shield,
  Sword,
  Crown,
  Sparkles,
  Package
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'
import { ConnectButton } from '@rainbow-me/rainbowkit'

const nfts = [
  {
    id: 1,
    name: 'Demon Armor',
    type: 'Equipment',
    rarity: 'Legendary',
    image: '🛡️',
    price: '0.5 ETH',
    chain: 'Ethereum',
    character: 'ShadowKnight',
    attributes: { armor: 16, level: 130 },
  },
  {
    id: 2,
    name: 'Sword of Valor',
    type: 'Weapon',
    rarity: 'Epic',
    image: '⚔️',
    price: '0.25 ETH',
    chain: 'Polygon',
    character: 'ShadowKnight',
    attributes: { attack: 51, level: 120 },
  },
  {
    id: 3,
    name: 'Golden Helmet',
    type: 'Equipment',
    rarity: 'Rare',
    image: '👑',
    price: '0.15 ETH',
    chain: 'Polygon',
    character: 'MysticDruid',
    attributes: { armor: 12, level: 100 },
  },
  {
    id: 4,
    name: 'Dragon Slayer',
    type: 'Achievement',
    rarity: 'Legendary',
    image: '🏆',
    price: '0.8 ETH',
    chain: 'Ethereum',
    character: null,
    attributes: { date: '2024-10-15' },
  },
]

const transactions = [
  { id: 1, type: 'mint', item: 'Demon Armor', amount: '0.01 ETH', date: '2024-12-01', status: 'completed', hash: '0x1234...5678' },
  { id: 2, type: 'sale', item: 'Magic Sword', amount: '0.3 ETH', date: '2024-11-28', status: 'completed', hash: '0x8765...4321' },
  { id: 3, type: 'purchase', item: 'Blessed Shield', amount: '0.2 ETH', date: '2024-11-25', status: 'completed', hash: '0xabcd...efgh' },
  { id: 4, type: 'bridge', item: 'Golden Boots', amount: '0.005 ETH', date: '2024-11-20', status: 'completed', hash: '0xijkl...mnop' },
]

const rarityColors: Record<string, string> = {
  'Common': 'text-gray-400 bg-gray-500/20',
  'Uncommon': 'text-green-400 bg-green-500/20',
  'Rare': 'text-blue-400 bg-blue-500/20',
  'Epic': 'text-purple-400 bg-purple-500/20',
  'Legendary': 'text-orange-400 bg-orange-500/20',
}

export default function WalletPage() {
  const [copied, setCopied] = useState(false)
  const walletAddress = '0x1234...5678'

  const copyAddress = () => {
    navigator.clipboard.writeText('0x1234567890abcdef1234567890abcdef12345678')
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <h1 className="text-2xl font-display font-bold mb-1">Wallet & NFTs</h1>
        <p className="text-shadow-400">Manage your blockchain assets and NFT collection</p>
      </motion.div>

      {/* Wallet Overview */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid lg:grid-cols-3 gap-6"
      >
        {/* Connected Wallet */}
        <div className="lg:col-span-2 card">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-lg font-semibold text-white">Connected Wallet</h2>
            <ConnectButton.Custom>
              {({ account, chain, openConnectModal, openAccountModal, mounted }) => {
                const connected = mounted && account && chain
                return (
                  <button
                    onClick={connected ? openAccountModal : openConnectModal}
                    className="btn-secondary text-sm py-2"
                  >
                    {connected ? 'Manage Wallet' : 'Connect Wallet'}
                  </button>
                )
              }}
            </ConnectButton.Custom>
          </div>

          <div className="bg-gradient-to-r from-purple-500/10 via-blue-500/10 to-purple-500/10 rounded-xl p-6 border border-purple-500/20 mb-6">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center space-x-3">
                <div className="w-12 h-12 bg-purple-500/20 rounded-xl flex items-center justify-center">
                  <Wallet className="w-6 h-6 text-purple-400" />
                </div>
                <div>
                  <p className="text-sm text-shadow-400">Wallet Address</p>
                  <div className="flex items-center space-x-2">
                    <span className="font-mono text-white">{walletAddress}</span>
                    <button onClick={copyAddress} className="p-1 hover:bg-white/10 rounded transition-colors">
                      {copied ? (
                        <Check className="w-4 h-4 text-green-500" />
                      ) : (
                        <Copy className="w-4 h-4 text-shadow-400" />
                      )}
                    </button>
                  </div>
                </div>
              </div>
              <a href="#" className="text-purple-400 hover:text-purple-300 flex items-center space-x-1">
                <span className="text-sm">View on Etherscan</span>
                <ExternalLink className="w-4 h-4" />
              </a>
            </div>

            <div className="grid grid-cols-3 gap-4">
              <div className="bg-black/20 rounded-lg p-3">
                <p className="text-xs text-shadow-400 mb-1">ETH Balance</p>
                <p className="text-xl font-bold text-white">2.45 ETH</p>
                <p className="text-xs text-green-500">≈ $4,312</p>
              </div>
              <div className="bg-black/20 rounded-lg p-3">
                <p className="text-xs text-shadow-400 mb-1">MATIC Balance</p>
                <p className="text-xl font-bold text-white">1,250</p>
                <p className="text-xs text-green-500">≈ $987</p>
              </div>
              <div className="bg-black/20 rounded-lg p-3">
                <p className="text-xs text-shadow-400 mb-1">NFTs Owned</p>
                <p className="text-xl font-bold text-white">12</p>
                <p className="text-xs text-shadow-500">4 listed</p>
              </div>
            </div>
          </div>

          <div className="flex space-x-3">
            <button className="flex-1 btn-primary flex items-center justify-center space-x-2">
              <Plus className="w-5 h-5" />
              <span>Mint New NFT</span>
            </button>
            <button className="flex-1 btn-secondary flex items-center justify-center space-x-2">
              <RefreshCw className="w-5 h-5" />
              <span>Bridge Assets</span>
            </button>
          </div>
        </div>

        {/* Stats */}
        <div className="space-y-4">
          <div className="stat-card">
            <div className="flex items-center justify-between mb-2">
              <span className="text-shadow-400 text-sm">Portfolio Value</span>
              <ArrowUpRight className="w-4 h-4 text-green-500" />
            </div>
            <p className="text-2xl font-bold text-white">$8,547</p>
            <p className="text-sm text-green-500">+12.5% this month</p>
          </div>
          <div className="stat-card">
            <div className="flex items-center justify-between mb-2">
              <span className="text-shadow-400 text-sm">Total Sales</span>
              <ArrowUpRight className="w-4 h-4 text-green-500" />
            </div>
            <p className="text-2xl font-bold text-white">1.8 ETH</p>
            <p className="text-sm text-shadow-500">≈ $3,168</p>
          </div>
          <div className="stat-card">
            <div className="flex items-center justify-between mb-2">
              <span className="text-shadow-400 text-sm">Pending</span>
              <Package className="w-4 h-4 text-yellow-500" />
            </div>
            <p className="text-2xl font-bold text-white">2 items</p>
            <p className="text-sm text-yellow-500">Awaiting confirmation</p>
          </div>
        </div>
      </motion.div>

      {/* NFTs & Transactions Tabs */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
      >
        <Tabs.Root defaultValue="nfts">
          <Tabs.List className="flex space-x-1 bg-shadow-900/50 rounded-xl p-1 mb-6">
            <Tabs.Trigger
              value="nfts"
              className="flex-1 px-4 py-2.5 rounded-lg text-sm font-medium transition-all data-[state=active]:bg-accent-500 data-[state=active]:text-white data-[state=inactive]:text-shadow-400 data-[state=inactive]:hover:text-white"
            >
              My NFTs ({nfts.length})
            </Tabs.Trigger>
            <Tabs.Trigger
              value="transactions"
              className="flex-1 px-4 py-2.5 rounded-lg text-sm font-medium transition-all data-[state=active]:bg-accent-500 data-[state=active]:text-white data-[state=inactive]:text-shadow-400 data-[state=inactive]:hover:text-white"
            >
              Transactions
            </Tabs.Trigger>
            <Tabs.Trigger
              value="mintable"
              className="flex-1 px-4 py-2.5 rounded-lg text-sm font-medium transition-all data-[state=active]:bg-accent-500 data-[state=active]:text-white data-[state=inactive]:text-shadow-400 data-[state=inactive]:hover:text-white"
            >
              Mintable Items
            </Tabs.Trigger>
          </Tabs.List>

          <Tabs.Content value="nfts">
            <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
              {nfts.map((nft) => (
                <div
                  key={nft.id}
                  className="bg-shadow-900/50 rounded-xl border border-shadow-700 overflow-hidden hover:border-accent-500/50 transition-all group"
                >
                  <div className="aspect-square bg-gradient-to-br from-shadow-800 to-shadow-900 flex items-center justify-center text-6xl">
                    {nft.image}
                  </div>
                  <div className="p-4">
                    <div className="flex items-start justify-between mb-2">
                      <div>
                        <h3 className="font-semibold text-white">{nft.name}</h3>
                        <p className="text-xs text-shadow-400">{nft.type}</p>
                      </div>
                      <span className={`text-xs px-2 py-0.5 rounded-full ${rarityColors[nft.rarity]}`}>
                        {nft.rarity}
                      </span>
                    </div>
                    <div className="flex items-center justify-between text-sm">
                      <span className="text-shadow-400">{nft.chain}</span>
                      <span className="font-medium text-white">{nft.price}</span>
                    </div>
                    {nft.character && (
                      <p className="text-xs text-shadow-500 mt-2">Equipped by {nft.character}</p>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </Tabs.Content>

          <Tabs.Content value="transactions">
            <div className="card">
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead>
                    <tr className="border-b border-shadow-700">
                      <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Type</th>
                      <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Item</th>
                      <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Amount</th>
                      <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Date</th>
                      <th className="text-left py-3 px-4 text-shadow-400 font-medium text-sm">Status</th>
                      <th className="text-right py-3 px-4 text-shadow-400 font-medium text-sm">Hash</th>
                    </tr>
                  </thead>
                  <tbody>
                    {transactions.map((tx) => (
                      <tr key={tx.id} className="border-b border-shadow-800 last:border-0 hover:bg-shadow-800/50">
                        <td className="py-3 px-4">
                          <span className={`px-2 py-1 rounded text-xs font-medium ${
                            tx.type === 'mint' ? 'bg-purple-500/20 text-purple-400' :
                            tx.type === 'sale' ? 'bg-green-500/20 text-green-400' :
                            tx.type === 'purchase' ? 'bg-blue-500/20 text-blue-400' :
                            'bg-yellow-500/20 text-yellow-400'
                          }`}>
                            {tx.type.charAt(0).toUpperCase() + tx.type.slice(1)}
                          </span>
                        </td>
                        <td className="py-3 px-4 text-white">{tx.item}</td>
                        <td className="py-3 px-4 text-white font-medium">{tx.amount}</td>
                        <td className="py-3 px-4 text-shadow-400">{tx.date}</td>
                        <td className="py-3 px-4">
                          <span className="text-green-500 text-sm">{tx.status}</span>
                        </td>
                        <td className="py-3 px-4 text-right">
                          <a href="#" className="text-accent-500 hover:text-accent-400 font-mono text-sm">
                            {tx.hash}
                          </a>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </Tabs.Content>

          <Tabs.Content value="mintable">
            <div className="card text-center py-12">
              <div className="w-16 h-16 bg-shadow-800 rounded-full flex items-center justify-center mx-auto mb-4">
                <Sparkles className="w-8 h-8 text-shadow-500" />
              </div>
              <h3 className="font-semibold text-white mb-2">No Mintable Items</h3>
              <p className="text-shadow-400 text-sm max-w-sm mx-auto mb-6">
                Items must meet certain criteria before they can be minted as NFTs. 
                Keep playing to unlock mintable items!
              </p>
              <button className="btn-secondary">Learn More</button>
            </div>
          </Tabs.Content>
        </Tabs.Root>
      </motion.div>
    </div>
  )
}

