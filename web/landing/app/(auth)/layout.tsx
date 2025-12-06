'use client'

import Link from 'next/link'
import { motion } from 'framer-motion'

export default function AuthLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen flex">
      {/* Left side - Branding */}
      <div className="hidden lg:flex lg:w-1/2 relative overflow-hidden">
        {/* Animated background */}
        <div className="absolute inset-0 bg-gradient-to-br from-shadow-950 via-accent-950 to-shadow-950">
          {/* Floating orbs */}
          <motion.div
            className="absolute w-96 h-96 bg-accent-500/20 rounded-full blur-3xl"
            animate={{
              x: [0, 100, 0],
              y: [0, -50, 0],
            }}
            transition={{ duration: 20, repeat: Infinity, ease: 'easeInOut' }}
            style={{ top: '10%', left: '10%' }}
          />
          <motion.div
            className="absolute w-80 h-80 bg-aetheria-primary/20 rounded-full blur-3xl"
            animate={{
              x: [0, -80, 0],
              y: [0, 80, 0],
            }}
            transition={{ duration: 15, repeat: Infinity, ease: 'easeInOut' }}
            style={{ bottom: '20%', right: '10%' }}
          />
          <motion.div
            className="absolute w-64 h-64 bg-purple-500/20 rounded-full blur-3xl"
            animate={{
              x: [0, 60, 0],
              y: [0, 60, 0],
            }}
            transition={{ duration: 18, repeat: Infinity, ease: 'easeInOut' }}
            style={{ top: '40%', right: '30%' }}
          />
          
          {/* Grid pattern overlay */}
          <div 
            className="absolute inset-0 opacity-10"
            style={{
              backgroundImage: `linear-gradient(rgba(255,255,255,0.05) 1px, transparent 1px),
                               linear-gradient(90deg, rgba(255,255,255,0.05) 1px, transparent 1px)`,
              backgroundSize: '50px 50px',
            }}
          />
        </div>

        {/* Content */}
        <div className="relative z-10 flex flex-col justify-between p-12 w-full">
          {/* Logo */}
          <Link href="/" className="flex items-center space-x-3">
            <div className="w-12 h-12 bg-gradient-to-br from-accent-500 to-accent-700 rounded-xl flex items-center justify-center shadow-lg shadow-accent-500/30">
              <span className="font-display font-bold text-2xl text-white">S</span>
            </div>
            <span className="font-display font-bold text-2xl">
              Shadow <span className="text-accent-500">OT</span>
            </span>
          </Link>

          {/* Center content */}
          <div className="max-w-md">
            <motion.h1 
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: 0.2 }}
              className="font-display text-4xl font-bold mb-4"
            >
              Enter the{' '}
              <span className="bg-gradient-to-r from-accent-400 to-aetheria-primary bg-clip-text text-transparent">
                Shadow Realms
              </span>
            </motion.h1>
            <motion.p
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.5, delay: 0.3 }}
              className="text-shadow-300 text-lg"
            >
              Join thousands of adventurers across multiple realms. 
              True asset ownership, epic battles, and endless possibilities await.
            </motion.p>
          </div>

          {/* Stats */}
          <motion.div 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.4 }}
            className="grid grid-cols-3 gap-6"
          >
            <div className="text-center p-4 bg-white/5 rounded-xl backdrop-blur-sm border border-white/10">
              <div className="text-3xl font-bold text-white mb-1">50K+</div>
              <div className="text-shadow-400 text-sm">Players</div>
            </div>
            <div className="text-center p-4 bg-white/5 rounded-xl backdrop-blur-sm border border-white/10">
              <div className="text-3xl font-bold text-white mb-1">6</div>
              <div className="text-shadow-400 text-sm">Realms</div>
            </div>
            <div className="text-center p-4 bg-white/5 rounded-xl backdrop-blur-sm border border-white/10">
              <div className="text-3xl font-bold text-white mb-1">10K+</div>
              <div className="text-shadow-400 text-sm">NFTs Minted</div>
            </div>
          </motion.div>
        </div>
      </div>

      {/* Right side - Form */}
      <div className="w-full lg:w-1/2 flex items-center justify-center p-8 bg-shadow-950">
        <div className="w-full max-w-md">
          {children}
        </div>
      </div>
    </div>
  )
}

