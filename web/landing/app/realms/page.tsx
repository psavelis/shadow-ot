'use client'

import { motion } from 'framer-motion'
import Link from 'next/link'
import { 
  Users, 
  Swords, 
  Shield, 
  Sparkles, 
  Crown, 
  Flame,
  Clock,
  TrendingUp,
  Globe,
  Zap,
  Target,
  Award
} from 'lucide-react'

const realms = [
  {
    id: 'shadowveil',
    name: 'Shadowveil',
    theme: 'Dark & Mysterious',
    tagline: 'Where shadows conceal untold riches',
    description: 'A realm shrouded in eternal darkness, where the brave face deadly creatures and reap tremendous rewards. Open PvP with enhanced loot drops and a thriving competitive scene.',
    players: 1847,
    peak: 2103,
    pvpType: 'Open PvP',
    rates: { exp: 5, skill: 4, loot: 2.5 },
    color: 'from-purple-900 via-slate-900 to-purple-950',
    accent: '#e94560',
    icon: Sparkles,
    featured: true,
    online: true,
    features: ['Enhanced Loot', 'Boss Rush Events', 'Guild Territories', 'NFT Exclusive Items'],
  },
  {
    id: 'aetheria',
    name: 'Aetheria',
    theme: 'Mythic & Epic',
    tagline: 'Legends are born here',
    description: 'A land where gods walk among mortals and ancient magic flows through every stone. Perfect for adventurers seeking epic questlines and rich lore.',
    players: 2103,
    peak: 2847,
    pvpType: 'Optional PvP',
    rates: { exp: 3, skill: 3, loot: 1.5 },
    color: 'from-blue-900 via-indigo-900 to-blue-950',
    accent: '#4A90D9',
    icon: Crown,
    featured: true,
    online: true,
    features: ['Deep Storyline', 'Mythic Bosses', 'Quest Chains', 'Peaceful Zones'],
  },
  {
    id: 'warbound',
    name: 'Warbound',
    theme: 'PvP Focused',
    tagline: 'No mercy. No surrender.',
    description: 'Non-stop PvP action with instant respawns and battle arenas. Guild wars, territory control, and ranked seasons. Only the strongest survive.',
    players: 892,
    peak: 1456,
    pvpType: 'Hardcore PvP',
    rates: { exp: 10, skill: 6, loot: 3 },
    color: 'from-red-900 via-orange-900 to-red-950',
    accent: '#FF4500',
    icon: Swords,
    featured: true,
    online: true,
    features: ['Ranked Seasons', 'Battle Royale', 'Territory Wars', 'No Blessing Loss'],
  },
  {
    id: 'mythara',
    name: 'Mythara',
    theme: 'Classic Experience',
    tagline: 'Return to the golden age',
    description: 'Relive the golden age of Tibia with authentic gameplay, original mechanics, and nostalgic features. 1x rates for the true classic experience.',
    players: 654,
    peak: 987,
    pvpType: 'Retro Open',
    rates: { exp: 1, skill: 1, loot: 1 },
    color: 'from-amber-900 via-yellow-900 to-amber-950',
    accent: '#FFD700',
    icon: Shield,
    featured: false,
    online: true,
    features: ['Original Mechanics', '7.x Protocol', 'Classic Spawns', 'True Nostalgia'],
  },
  {
    id: 'voidborne',
    name: 'Voidborne',
    theme: 'Seasonal Realm',
    tagline: 'Limited time. Unlimited glory.',
    description: 'Limited-time seasonal realm with unique mechanics, exclusive rewards, and fresh starts. New season every 3 months with different rulesets.',
    players: 421,
    peak: 1234,
    pvpType: 'Mixed',
    rates: { exp: 7, skill: 5, loot: 2 },
    color: 'from-violet-900 via-purple-900 to-violet-950',
    accent: '#9F7AEA',
    icon: Flame,
    featured: false,
    online: true,
    seasonal: true,
    seasonEnd: '2025-02-28',
    features: ['Seasonal Rewards', 'Unique Ruleset', 'Fresh Economy', 'Exclusive Titles'],
  },
  {
    id: 'grimhollow',
    name: 'Grimhollow',
    theme: 'Horror & Survival',
    tagline: 'Face your fears',
    description: 'A survival horror experience where death means something. Limited lives, permadeath zones, and terrifying creatures lurking in the dark.',
    players: 0,
    peak: 0,
    pvpType: 'Survival PvP',
    rates: { exp: 2, skill: 2, loot: 1.5 },
    color: 'from-gray-900 via-zinc-900 to-gray-950',
    accent: '#71717A',
    icon: Target,
    featured: false,
    online: false,
    comingSoon: true,
    launchDate: '2025-03-15',
    features: ['Permadeath Zones', 'Survival Mode', 'Horror Events', 'Limited Lives'],
  },
]

export default function RealmsPage() {
  const onlineRealms = realms.filter(r => r.online)
  const totalOnline = onlineRealms.reduce((sum, r) => sum + r.players, 0)

  return (
    <div className="min-h-screen bg-shadow-950 pt-24 pb-16">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <h1 className="font-display text-4xl md:text-5xl font-bold mb-4">
            Choose Your <span className="gradient-text">Realm</span>
          </h1>
          <p className="text-shadow-300 text-lg max-w-2xl mx-auto mb-8">
            Each realm offers a unique experience with different rates, PvP styles, and communities.
            Your account works across all realms seamlessly.
          </p>
          
          {/* Global Stats */}
          <div className="inline-flex items-center space-x-6 bg-shadow-900/50 rounded-full px-8 py-3 border border-shadow-700">
            <div className="flex items-center space-x-2">
              <Users className="w-5 h-5 text-accent-500" />
              <span className="text-white font-bold">{totalOnline.toLocaleString()}</span>
              <span className="text-shadow-400">Online</span>
            </div>
            <div className="w-px h-6 bg-shadow-600" />
            <div className="flex items-center space-x-2">
              <Globe className="w-5 h-5 text-accent-500" />
              <span className="text-white font-bold">{onlineRealms.length}</span>
              <span className="text-shadow-400">Active Realms</span>
            </div>
          </div>
        </motion.div>

        {/* Featured Realms */}
        <div className="grid lg:grid-cols-3 gap-6 mb-12">
          {realms.filter(r => r.featured).map((realm, index) => (
            <motion.div
              key={realm.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.1 }}
            >
              <Link href={`/realms/${realm.id}`}>
                <div className={`relative h-full bg-gradient-to-br ${realm.color} rounded-2xl overflow-hidden border border-white/10 hover:border-white/30 transition-all duration-500 group`}>
                  {/* Animated glow */}
                  <div
                    className="absolute inset-0 opacity-0 group-hover:opacity-30 transition-opacity duration-500"
                    style={{ background: `radial-gradient(circle at 50% 50%, ${realm.accent}, transparent 70%)` }}
                  />
                  
                  {/* Content */}
                  <div className="relative z-10 p-6">
                    {/* Header */}
                    <div className="flex items-start justify-between mb-4">
                      <div>
                        <div className="flex items-center space-x-2 mb-2">
                          <realm.icon className="w-5 h-5" style={{ color: realm.accent }} />
                          <span className="text-sm font-medium text-white/70">{realm.theme}</span>
                        </div>
                        <h3 className="text-2xl font-display font-bold text-white">{realm.name}</h3>
                        <p className="text-white/50 text-sm italic">{realm.tagline}</p>
                      </div>
                      {realm.seasonal && (
                        <div className="flex items-center space-x-1 px-2 py-1 bg-yellow-500/20 rounded-full">
                          <Clock className="w-3 h-3 text-yellow-400" />
                          <span className="text-yellow-400 text-xs font-medium">Seasonal</span>
                        </div>
                      )}
                    </div>

                    <p className="text-white/70 text-sm mb-6 line-clamp-3">
                      {realm.description}
                    </p>

                    {/* Stats */}
                    <div className="grid grid-cols-2 gap-4 mb-6">
                      <div className="bg-black/20 rounded-lg p-3">
                        <div className="flex items-center space-x-2 mb-1">
                          <Users className="w-4 h-4 text-white/50" />
                          <span className="text-white font-bold">{realm.players.toLocaleString()}</span>
                        </div>
                        <span className="text-white/50 text-xs">Online Now</span>
                      </div>
                      <div className="bg-black/20 rounded-lg p-3">
                        <div className="flex items-center space-x-2 mb-1">
                          <TrendingUp className="w-4 h-4 text-white/50" />
                          <span className="text-white font-bold">{realm.peak.toLocaleString()}</span>
                        </div>
                        <span className="text-white/50 text-xs">Peak Players</span>
                      </div>
                    </div>

                    {/* Rates */}
                    <div className="flex items-center space-x-4 mb-6">
                      <div className="text-center">
                        <div className="text-lg font-bold" style={{ color: realm.accent }}>{realm.rates.exp}x</div>
                        <div className="text-white/50 text-xs">EXP</div>
                      </div>
                      <div className="w-px h-8 bg-white/20" />
                      <div className="text-center">
                        <div className="text-lg font-bold" style={{ color: realm.accent }}>{realm.rates.skill}x</div>
                        <div className="text-white/50 text-xs">Skill</div>
                      </div>
                      <div className="w-px h-8 bg-white/20" />
                      <div className="text-center">
                        <div className="text-lg font-bold" style={{ color: realm.accent }}>{realm.rates.loot}x</div>
                        <div className="text-white/50 text-xs">Loot</div>
                      </div>
                      <div className="w-px h-8 bg-white/20" />
                      <div className="text-center px-2 py-1 bg-white/10 rounded">
                        <div className="text-white text-xs font-medium">{realm.pvpType}</div>
                      </div>
                    </div>

                    {/* Features */}
                    <div className="flex flex-wrap gap-2">
                      {realm.features.slice(0, 4).map((feature) => (
                        <span
                          key={feature}
                          className="text-xs bg-white/10 text-white/80 px-2 py-1 rounded"
                        >
                          {feature}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>
              </Link>
            </motion.div>
          ))}
        </div>

        {/* Other Realms */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
        >
          <h2 className="font-display text-2xl font-bold mb-6">All Realms</h2>
          <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
            {realms.filter(r => !r.featured).map((realm, index) => (
              <motion.div
                key={realm.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.4 + index * 0.1 }}
              >
                <Link href={realm.comingSoon ? '#' : `/realms/${realm.id}`}>
                  <div className={`relative p-5 rounded-xl border transition-all duration-300 ${
                    realm.comingSoon 
                      ? 'bg-shadow-900/30 border-shadow-700 cursor-default' 
                      : 'bg-shadow-900/50 border-shadow-700 hover:border-accent-500/50 group'
                  }`}>
                    {realm.comingSoon && (
                      <div className="absolute inset-0 bg-shadow-950/60 rounded-xl flex items-center justify-center">
                        <div className="text-center">
                          <span className="text-accent-500 font-medium">Coming Soon</span>
                          <p className="text-shadow-400 text-sm mt-1">Launches {realm.launchDate}</p>
                        </div>
                      </div>
                    )}
                    
                    <div className="flex items-start space-x-4">
                      <div
                        className="w-12 h-12 rounded-xl flex items-center justify-center flex-shrink-0"
                        style={{ backgroundColor: `${realm.accent}20` }}
                      >
                        <realm.icon className="w-6 h-6" style={{ color: realm.accent }} />
                      </div>
                      <div className="flex-grow min-w-0">
                        <div className="flex items-center space-x-2 mb-1">
                          <h3 className="font-semibold text-white truncate">{realm.name}</h3>
                          {realm.seasonal && (
                            <span className="flex-shrink-0 px-2 py-0.5 bg-yellow-500/20 text-yellow-400 text-xs font-medium rounded-full">
                              Seasonal
                            </span>
                          )}
                        </div>
                        <p className="text-shadow-400 text-sm mb-2">{realm.theme}</p>
                        <div className="flex items-center space-x-4 text-sm">
                          <span className="text-white font-medium">{realm.players} online</span>
                          <span className="text-shadow-500">|</span>
                          <span className="text-shadow-400">{realm.pvpType}</span>
                          <span className="text-shadow-500">|</span>
                          <span style={{ color: realm.accent }}>{realm.rates.exp}x</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </Link>
              </motion.div>
            ))}
          </div>
        </motion.div>

        {/* Cross-Realm Features */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5 }}
          className="mt-16"
        >
          <div className="bg-gradient-to-r from-accent-500/10 via-purple-500/10 to-blue-500/10 rounded-2xl border border-accent-500/20 p-8">
            <h2 className="font-display text-2xl font-bold text-center mb-8">Cross-Realm Features</h2>
            <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6">
              <div className="text-center">
                <div className="w-14 h-14 bg-accent-500/20 rounded-xl flex items-center justify-center mx-auto mb-4">
                  <Users className="w-7 h-7 text-accent-500" />
                </div>
                <h3 className="font-semibold text-white mb-2">Unified Account</h3>
                <p className="text-shadow-400 text-sm">One account, all realms. Switch freely.</p>
              </div>
              <div className="text-center">
                <div className="w-14 h-14 bg-purple-500/20 rounded-xl flex items-center justify-center mx-auto mb-4">
                  <Award className="w-7 h-7 text-purple-500" />
                </div>
                <h3 className="font-semibold text-white mb-2">Shared Achievements</h3>
                <p className="text-shadow-400 text-sm">Achievements unlock account-wide.</p>
              </div>
              <div className="text-center">
                <div className="w-14 h-14 bg-blue-500/20 rounded-xl flex items-center justify-center mx-auto mb-4">
                  <Globe className="w-7 h-7 text-blue-500" />
                </div>
                <h3 className="font-semibold text-white mb-2">Global Events</h3>
                <p className="text-shadow-400 text-sm">Cross-realm tournaments & events.</p>
              </div>
              <div className="text-center">
                <div className="w-14 h-14 bg-green-500/20 rounded-xl flex items-center justify-center mx-auto mb-4">
                  <Zap className="w-7 h-7 text-green-500" />
                </div>
                <h3 className="font-semibold text-white mb-2">NFT Transfer</h3>
                <p className="text-shadow-400 text-sm">Move NFT items between realms.</p>
              </div>
            </div>
          </div>
        </motion.div>
      </div>
    </div>
  )
}

