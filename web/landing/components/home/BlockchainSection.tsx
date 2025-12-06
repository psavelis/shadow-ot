'use client'

import { motion } from 'framer-motion'
import { Wallet, ArrowLeftRight, Shield, Coins, ExternalLink } from 'lucide-react'

const chains = [
  { name: 'Ethereum', logo: '/images/chains/ethereum.svg' },
  { name: 'Polygon', logo: '/images/chains/polygon.svg' },
  { name: 'Starknet', logo: '/images/chains/starknet.svg' },
  { name: 'Bitcoin', logo: '/images/chains/bitcoin.svg' },
  { name: 'Base', logo: '/images/chains/base.svg' },
  { name: 'Arbitrum', logo: '/images/chains/arbitrum.svg' },
]

const benefits = [
  {
    icon: Shield,
    title: 'True Ownership',
    description: 'Your items, houses, and achievements are NFTs that you truly own.',
  },
  {
    icon: ArrowLeftRight,
    title: 'Cross-Chain Bridge',
    description: 'Transfer your assets between supported blockchains seamlessly.',
  },
  {
    icon: Coins,
    title: 'Trade Anywhere',
    description: 'Sell your NFT items on any marketplace, not just in-game.',
  },
  {
    icon: Wallet,
    title: 'Wallet Login',
    description: 'Connect your wallet for secure authentication and asset management.',
  },
]

export function BlockchainSection() {
  return (
    <section className="py-20 lg:py-32 bg-shadow-950 overflow-hidden">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid lg:grid-cols-2 gap-12 lg:gap-20 items-center">
          {/* Left - Content */}
          <motion.div
            initial={{ opacity: 0, x: -50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.5 }}
          >
            <div className="inline-flex items-center space-x-2 bg-gradient-to-r from-purple-500/10 to-blue-500/10 border border-purple-500/30 rounded-full px-4 py-2 mb-6">
              <span className="text-purple-400 text-sm font-medium">Web3 Native</span>
            </div>

            <h2 className="font-display text-4xl lg:text-5xl font-bold mb-6">
              Blockchain-Native{' '}
              <span className="bg-gradient-to-r from-purple-400 to-blue-400 bg-clip-text text-transparent">
                Game Assets
              </span>
            </h2>

            <p className="text-shadow-300 text-lg mb-8">
              Shadow OT is the first Open Tibia server with native blockchain integration.
              Your items, houses, and achievements can be minted as NFTs across multiple chains,
              giving you true ownership and the ability to trade anywhere.
            </p>

            {/* Benefits */}
            <div className="grid sm:grid-cols-2 gap-4 mb-8">
              {benefits.map((benefit, index) => (
                <motion.div
                  key={benefit.title}
                  initial={{ opacity: 0, y: 20 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ duration: 0.5, delay: index * 0.1 }}
                  className="flex items-start space-x-3"
                >
                  <div className="w-10 h-10 bg-gradient-to-br from-purple-500/20 to-blue-500/20 rounded-lg flex items-center justify-center flex-shrink-0">
                    <benefit.icon className="w-5 h-5 text-purple-400" />
                  </div>
                  <div>
                    <h3 className="font-semibold text-white mb-1">{benefit.title}</h3>
                    <p className="text-shadow-400 text-sm">{benefit.description}</p>
                  </div>
                </motion.div>
              ))}
            </div>

            <a
              href="/docs/blockchain"
              className="inline-flex items-center text-purple-400 hover:text-purple-300 transition-colors"
            >
              Learn more about our blockchain integration
              <ExternalLink className="w-4 h-4 ml-2" />
            </a>
          </motion.div>

          {/* Right - Visual */}
          <motion.div
            initial={{ opacity: 0, x: 50 }}
            whileInView={{ opacity: 1, x: 0 }}
            viewport={{ once: true }}
            transition={{ duration: 0.5 }}
            className="relative"
          >
            {/* NFT Card Preview */}
            <div className="relative mx-auto max-w-sm">
              {/* Glow effect */}
              <div className="absolute inset-0 bg-gradient-to-r from-purple-500/30 to-blue-500/30 blur-3xl" />

              {/* Card */}
              <div className="relative bg-gradient-to-br from-shadow-800 to-shadow-900 rounded-2xl border border-white/10 overflow-hidden">
                {/* NFT Image */}
                <div className="aspect-square bg-gradient-to-br from-purple-900/50 to-blue-900/50 flex items-center justify-center">
                  <div className="text-center">
                    <div className="w-24 h-24 bg-gradient-to-br from-yellow-500 to-orange-500 rounded-full mx-auto mb-4 flex items-center justify-center shadow-lg shadow-orange-500/30">
                      <span className="text-4xl">⚔️</span>
                    </div>
                    <span className="text-white/60 text-sm">Legendary Item NFT</span>
                  </div>
                </div>

                {/* Card Info */}
                <div className="p-6">
                  <div className="flex items-start justify-between mb-4">
                    <div>
                      <h3 className="font-display font-bold text-xl text-white">
                        Sword of Shadows
                      </h3>
                      <p className="text-shadow-400 text-sm">Legendary Weapon</p>
                    </div>
                    <span className="px-2 py-1 bg-purple-500/20 text-purple-400 text-xs font-medium rounded-full">
                      #1234
                    </span>
                  </div>

                  <div className="flex items-center justify-between text-sm mb-4">
                    <div>
                      <span className="text-shadow-500">Attack</span>
                      <span className="text-white font-medium ml-2">+58</span>
                    </div>
                    <div>
                      <span className="text-shadow-500">Rarity</span>
                      <span className="text-orange-400 font-medium ml-2">Legendary</span>
                    </div>
                  </div>

                  <div className="flex items-center justify-between pt-4 border-t border-shadow-700">
                    <div>
                      <span className="text-shadow-500 text-xs">Current Price</span>
                      <p className="text-white font-bold">0.5 ETH</p>
                    </div>
                    <button className="btn-primary text-sm py-2 px-4">
                      View on OpenSea
                    </button>
                  </div>
                </div>
              </div>
            </div>

            {/* Supported Chains */}
            <div className="mt-8">
              <p className="text-shadow-500 text-sm text-center mb-4">Supported Chains</p>
              <div className="flex items-center justify-center space-x-4">
                {chains.map((chain) => (
                  <div
                    key={chain.name}
                    className="w-10 h-10 bg-shadow-800 rounded-full flex items-center justify-center border border-shadow-700 hover:border-shadow-500 transition-colors"
                    title={chain.name}
                  >
                    <span className="text-lg">⛓️</span>
                  </div>
                ))}
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </section>
  )
}
