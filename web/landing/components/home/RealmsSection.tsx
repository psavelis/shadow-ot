'use client'

import { motion } from 'framer-motion'
import Link from 'next/link'
import { Users, Swords, Shield, Sparkles, Crown, Flame } from 'lucide-react'

const realms = [
  {
    id: 'shadowveil',
    name: 'Shadowveil',
    theme: 'Dark & Mysterious',
    description: 'A realm shrouded in eternal darkness, where only the brave survive. Open PvP with high rewards.',
    players: 1847,
    pvpType: 'Open PvP',
    rates: '5x Exp',
    color: 'from-purple-900 to-slate-900',
    accent: '#e94560',
    icon: Sparkles,
    featured: true,
  },
  {
    id: 'aetheria',
    name: 'Aetheria',
    theme: 'Mythic & Epic',
    description: 'A land of gods, heroes, and ancient magic. Perfect for those seeking adventure and lore.',
    players: 2103,
    pvpType: 'Optional PvP',
    rates: '3x Exp',
    color: 'from-blue-900 to-indigo-900',
    accent: '#4A90D9',
    icon: Crown,
    featured: true,
  },
  {
    id: 'warbound',
    name: 'Warbound',
    theme: 'PvP Focused',
    description: 'Non-stop PvP action with instant respawns. Battle arenas and guild wars await.',
    players: 892,
    pvpType: 'Hardcore PvP',
    rates: '10x Exp',
    color: 'from-red-900 to-orange-900',
    accent: '#FF4500',
    icon: Swords,
    featured: true,
  },
  {
    id: 'mythara',
    name: 'Mythara',
    theme: 'Classic Experience',
    description: 'Relive the golden age of Tibia with authentic gameplay and nostalgic features.',
    players: 654,
    pvpType: 'Retro Open',
    rates: '1x Exp',
    color: 'from-amber-900 to-yellow-900',
    accent: '#FFD700',
    icon: Shield,
  },
  {
    id: 'voidborne',
    name: 'Voidborne',
    theme: 'Seasonal Realm',
    description: 'Limited-time seasonal realm with unique mechanics and exclusive rewards.',
    players: 421,
    pvpType: 'Mixed',
    rates: '7x Exp',
    color: 'from-violet-900 to-purple-900',
    accent: '#9F7AEA',
    icon: Flame,
    seasonal: true,
  },
]

export function RealmsSection() {
  return (
    <section className="py-20 lg:py-32 bg-shadow-950">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5 }}
          className="text-center mb-16"
        >
          <h2 className="section-title">
            Choose Your <span className="gradient-text">Realm</span>
          </h2>
          <p className="section-subtitle">
            Each realm offers a unique experience with different rates, PvP styles, and communities.
            Your account works across all realms.
          </p>
        </motion.div>

        {/* Featured Realms Grid */}
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
          {realms.filter(r => r.featured).map((realm, index) => (
            <motion.div
              key={realm.id}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: index * 0.1 }}
            >
              <Link href={`/realms/${realm.id}`}>
                <div className={`relative h-full bg-gradient-to-br ${realm.color} rounded-2xl p-6 border border-white/10 hover:border-white/30 transition-all duration-300 group overflow-hidden`}>
                  {/* Glow effect */}
                  <div
                    className="absolute inset-0 opacity-0 group-hover:opacity-20 transition-opacity duration-300"
                    style={{ background: `radial-gradient(circle at center, ${realm.accent}, transparent)` }}
                  />

                  {/* Content */}
                  <div className="relative z-10">
                    <div className="flex items-start justify-between mb-4">
                      <div>
                        <div className="flex items-center space-x-2 mb-1">
                          <realm.icon className="w-5 h-5" style={{ color: realm.accent }} />
                          <span className="text-sm font-medium text-white/60">{realm.theme}</span>
                        </div>
                        <h3 className="text-2xl font-display font-bold text-white">{realm.name}</h3>
                      </div>
                      {realm.seasonal && (
                        <span className="px-2 py-1 bg-yellow-500/20 text-yellow-400 text-xs font-medium rounded-full">
                          Seasonal
                        </span>
                      )}
                    </div>

                    <p className="text-white/70 text-sm mb-6 line-clamp-2">
                      {realm.description}
                    </p>

                    <div className="flex items-center justify-between text-sm">
                      <div className="flex items-center space-x-4">
                        <div className="flex items-center space-x-1">
                          <Users className="w-4 h-4 text-white/50" />
                          <span className="text-white font-medium">{realm.players.toLocaleString()}</span>
                        </div>
                        <span className="text-white/50">|</span>
                        <span className="text-white/70">{realm.pvpType}</span>
                      </div>
                      <span
                        className="font-medium px-2 py-1 rounded-md"
                        style={{ backgroundColor: `${realm.accent}20`, color: realm.accent }}
                      >
                        {realm.rates}
                      </span>
                    </div>
                  </div>
                </div>
              </Link>
            </motion.div>
          ))}
        </div>

        {/* Other Realms */}
        <div className="grid md:grid-cols-2 gap-4">
          {realms.filter(r => !r.featured).map((realm, index) => (
            <motion.div
              key={realm.id}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: 0.3 + index * 0.1 }}
            >
              <Link href={`/realms/${realm.id}`}>
                <div className="flex items-center p-4 bg-shadow-900/50 rounded-xl border border-shadow-700 hover:border-accent-500/50 transition-all duration-300 group">
                  <div
                    className="w-12 h-12 rounded-lg flex items-center justify-center mr-4"
                    style={{ backgroundColor: `${realm.accent}20` }}
                  >
                    <realm.icon className="w-6 h-6" style={{ color: realm.accent }} />
                  </div>
                  <div className="flex-grow">
                    <div className="flex items-center space-x-2">
                      <h3 className="font-semibold text-white">{realm.name}</h3>
                      {realm.seasonal && (
                        <span className="px-2 py-0.5 bg-yellow-500/20 text-yellow-400 text-xs font-medium rounded-full">
                          Seasonal
                        </span>
                      )}
                    </div>
                    <p className="text-shadow-400 text-sm">{realm.theme}</p>
                  </div>
                  <div className="text-right">
                    <div className="flex items-center space-x-1 text-white">
                      <Users className="w-4 h-4 text-shadow-500" />
                      <span className="font-medium">{realm.players}</span>
                    </div>
                    <span className="text-xs text-shadow-500">{realm.pvpType}</span>
                  </div>
                </div>
              </Link>
            </motion.div>
          ))}
        </div>

        {/* View All */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5, delay: 0.5 }}
          className="text-center mt-8"
        >
          <Link href="/realms" className="btn-outline">
            View All Realms
          </Link>
        </motion.div>
      </div>
    </section>
  )
}
