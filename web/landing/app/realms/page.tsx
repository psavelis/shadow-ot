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
  Award,
  Loader2,
  AlertCircle
} from 'lucide-react'
import { useRealms } from '@shadow-ot/shared'
import type { Realm, RealmId } from '@shadow-ot/shared'

const realmIcons: Record<string, React.ComponentType<{ className?: string }>> = {
  shadowveil: Sparkles,
  aetheria: Crown,
  warbound: Swords,
  mythara: Shield,
  voidborne: Zap,
  grimhollow: Flame,
}

const realmGradients: Record<string, string> = {
  shadowveil: 'from-purple-900 via-slate-900 to-purple-950',
  aetheria: 'from-blue-900 via-indigo-900 to-blue-950',
  warbound: 'from-red-900 via-orange-900 to-red-950',
  mythara: 'from-green-900 via-emerald-900 to-green-950',
  voidborne: 'from-indigo-900 via-purple-900 to-violet-950',
  grimhollow: 'from-slate-900 via-zinc-900 to-slate-950',
}

const realmAccents: Record<string, string> = {
  shadowveil: '#e94560',
  aetheria: '#4A90D9',
  warbound: '#FF4500',
  mythara: '#10b981',
  voidborne: '#8b5cf6',
  grimhollow: '#64748b',
}

export default function RealmsPage() {
  const { data: realms, isLoading, error } = useRealms()
  
  const onlineRealms = realms?.filter(r => r.status === 'online') || []
  const totalOnline = onlineRealms.reduce((sum, r) => sum + r.playersOnline, 0)

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 py-20">
      <div className="container mx-auto px-4">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-cyan-500/10 border border-cyan-500/30 rounded-full text-cyan-400 text-sm mb-6">
            <Globe className="w-4 h-4" />
            Game Worlds
          </div>
          <h1 className="text-4xl md:text-5xl font-bold text-white mb-4">
            Choose Your <span className="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-blue-500">Realm</span>
          </h1>
          <p className="text-slate-400 text-lg max-w-2xl mx-auto mb-8">
            Each realm offers a unique experience with different rates, PvP styles, and communities.
            Your account works across all realms seamlessly.
          </p>
          
          {/* Global Stats */}
          {realms && (
            <div className="inline-flex items-center gap-6 bg-slate-800/50 border border-slate-700/50 rounded-full px-8 py-3">
              <div className="flex items-center gap-2">
                <Users className="w-5 h-5 text-cyan-400" />
                <span className="text-white font-bold">{totalOnline.toLocaleString()}</span>
                <span className="text-slate-400">Online</span>
              </div>
              <div className="w-px h-6 bg-slate-600" />
              <div className="flex items-center gap-2">
                <Globe className="w-5 h-5 text-cyan-400" />
                <span className="text-white font-bold">{onlineRealms.length}</span>
                <span className="text-slate-400">Active Realms</span>
              </div>
            </div>
          )}
        </motion.div>

        {/* Loading State */}
        {isLoading && (
          <div className="flex items-center justify-center py-20">
            <Loader2 className="w-12 h-12 animate-spin text-cyan-400" />
          </div>
        )}

        {/* Error State */}
        {error && (
          <div className="flex flex-col items-center justify-center py-20 text-red-400">
            <AlertCircle className="w-12 h-12 mb-4" />
            <p>Failed to load realms. Please try again.</p>
          </div>
        )}

        {/* Realms Grid */}
        {realms && realms.length > 0 && (
          <>
            {/* Featured Realms */}
            <div className="grid lg:grid-cols-3 gap-6 mb-12">
              {realms.filter(r => r.featured).map((realm, index) => {
                const Icon = realmIcons[realm.id] || Globe
                const gradient = realmGradients[realm.id] || 'from-slate-900 to-slate-950'
                const accent = realmAccents[realm.id] || '#64748b'
                
                return (
                  <motion.div
                    key={realm.id}
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ delay: index * 0.1 }}
                  >
                    <Link href={`/realms/${realm.id}`}>
                      <div className={`relative h-full bg-gradient-to-br ${gradient} rounded-2xl overflow-hidden border border-white/10 hover:border-white/30 transition-all duration-500 group`}>
                        {/* Animated glow */}
                        <div
                          className="absolute inset-0 opacity-0 group-hover:opacity-30 transition-opacity duration-500"
                          style={{ background: `radial-gradient(circle at 50% 50%, ${accent}, transparent 70%)` }}
                        />
                        
                        {/* Content */}
                        <div className="relative z-10 p-6">
                          {/* Header */}
                          <div className="flex items-start justify-between mb-4">
                            <div>
                              <div className="flex items-center gap-2 mb-2">
                                <span style={{ color: accent }}>
                                  <Icon className="w-5 h-5" />
                                </span>
                                <span className="text-sm font-medium text-white/70">{realm.theme}</span>
                              </div>
                              <h3 className="text-2xl font-bold text-white">{realm.name}</h3>
                              <p className="text-white/50 text-sm italic">{realm.tagline}</p>
                            </div>
                            {realm.seasonal && (
                              <div className="flex items-center gap-1 px-2 py-1 bg-yellow-500/20 rounded-full">
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
                              <div className="flex items-center gap-2 mb-1">
                                <Users className="w-4 h-4 text-white/50" />
                                <span className="text-white font-bold">{realm.playersOnline.toLocaleString()}</span>
                              </div>
                              <span className="text-white/50 text-xs">Online Now</span>
                            </div>
                            <div className="bg-black/20 rounded-lg p-3">
                              <div className="flex items-center gap-2 mb-1">
                                <TrendingUp className="w-4 h-4 text-white/50" />
                                <span className="text-white font-bold">{realm.peakPlayers.toLocaleString()}</span>
                              </div>
                              <span className="text-white/50 text-xs">Peak Players</span>
                            </div>
                          </div>

                          {/* Rates */}
                          <div className="flex items-center gap-4 mb-6">
                            <div className="text-center">
                              <div className="text-lg font-bold" style={{ color: accent }}>{realm.rates.experience}x</div>
                              <div className="text-white/50 text-xs">EXP</div>
                            </div>
                            <div className="w-px h-8 bg-white/20" />
                            <div className="text-center">
                              <div className="text-lg font-bold" style={{ color: accent }}>{realm.rates.skill}x</div>
                              <div className="text-white/50 text-xs">Skill</div>
                            </div>
                            <div className="w-px h-8 bg-white/20" />
                            <div className="text-center">
                              <div className="text-lg font-bold" style={{ color: accent }}>{realm.rates.loot}x</div>
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
                )
              })}
            </div>

            {/* Other Realms */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.3 }}
            >
              <h2 className="text-2xl font-bold text-white mb-6">All Realms</h2>
              <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
                {realms.filter(r => !r.featured).map((realm, index) => {
                  const Icon = realmIcons[realm.id] || Globe
                  const accent = realmAccents[realm.id] || '#64748b'
                  
                  return (
                    <motion.div
                      key={realm.id}
                      initial={{ opacity: 0, y: 20 }}
                      animate={{ opacity: 1, y: 0 }}
                      transition={{ delay: 0.4 + index * 0.1 }}
                    >
                      <Link href={realm.status === 'coming_soon' ? '#' : `/realms/${realm.id}`}>
                        <div className={`relative p-5 rounded-xl border transition-all duration-300 ${
                          realm.status === 'coming_soon'
                            ? 'bg-slate-900/30 border-slate-700 cursor-default' 
                            : 'bg-slate-800/50 border-slate-700 hover:border-cyan-500/50 group'
                        }`}>
                          {realm.status === 'coming_soon' && (
                            <div className="absolute inset-0 bg-slate-950/60 rounded-xl flex items-center justify-center">
                              <div className="text-center">
                                <span className="text-cyan-400 font-medium">Coming Soon</span>
                                {realm.launchDate && (
                                  <p className="text-slate-400 text-sm mt-1">Launches {new Date(realm.launchDate).toLocaleDateString()}</p>
                                )}
                              </div>
                            </div>
                          )}
                          
                          <div className="flex items-start gap-4">
                            <div
                              className="w-12 h-12 rounded-xl flex items-center justify-center flex-shrink-0"
                              style={{ backgroundColor: `${accent}20` }}
                            >
                              <span style={{ color: accent }}>
                                <Icon className="w-6 h-6" />
                              </span>
                            </div>
                            <div className="flex-grow min-w-0">
                              <div className="flex items-center gap-2 mb-1">
                                <h3 className="font-semibold text-white truncate">{realm.name}</h3>
                                {realm.seasonal && (
                                  <span className="flex-shrink-0 px-2 py-0.5 bg-yellow-500/20 text-yellow-400 text-xs font-medium rounded-full">
                                    Seasonal
                                  </span>
                                )}
                              </div>
                              <p className="text-slate-400 text-sm mb-2">{realm.theme}</p>
                              <div className="flex items-center gap-4 text-sm">
                                <span className="text-white font-medium">{realm.playersOnline} online</span>
                                <span className="text-slate-500">|</span>
                                <span className="text-slate-400">{realm.pvpType}</span>
                                <span className="text-slate-500">|</span>
                                <span style={{ color: accent }}>{realm.rates.experience}x</span>
                              </div>
                            </div>
                          </div>
                        </div>
                      </Link>
                    </motion.div>
                  )
                })}
              </div>
            </motion.div>

            {/* Cross-Realm Features */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.5 }}
              className="mt-16"
            >
              <div className="bg-gradient-to-r from-cyan-500/10 via-purple-500/10 to-blue-500/10 rounded-2xl border border-cyan-500/20 p-8">
                <h2 className="text-2xl font-bold text-white text-center mb-8">Cross-Realm Features</h2>
                <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6">
                  <div className="text-center">
                    <div className="w-14 h-14 bg-cyan-500/20 rounded-xl flex items-center justify-center mx-auto mb-4">
                      <Users className="w-7 h-7 text-cyan-400" />
                    </div>
                    <h3 className="font-semibold text-white mb-2">Unified Account</h3>
                    <p className="text-slate-400 text-sm">One account, all realms. Switch freely.</p>
                  </div>
                  <div className="text-center">
                    <div className="w-14 h-14 bg-purple-500/20 rounded-xl flex items-center justify-center mx-auto mb-4">
                      <Award className="w-7 h-7 text-purple-400" />
                    </div>
                    <h3 className="font-semibold text-white mb-2">Shared Achievements</h3>
                    <p className="text-slate-400 text-sm">Achievements unlock account-wide.</p>
                  </div>
                  <div className="text-center">
                    <div className="w-14 h-14 bg-blue-500/20 rounded-xl flex items-center justify-center mx-auto mb-4">
                      <Globe className="w-7 h-7 text-blue-400" />
                    </div>
                    <h3 className="font-semibold text-white mb-2">Global Events</h3>
                    <p className="text-slate-400 text-sm">Cross-realm tournaments & events.</p>
                  </div>
                  <div className="text-center">
                    <div className="w-14 h-14 bg-green-500/20 rounded-xl flex items-center justify-center mx-auto mb-4">
                      <Zap className="w-7 h-7 text-green-400" />
                    </div>
                    <h3 className="font-semibold text-white mb-2">NFT Transfer</h3>
                    <p className="text-slate-400 text-sm">Move NFT items between realms.</p>
                  </div>
                </div>
              </div>
            </motion.div>
          </>
        )}
      </div>
    </div>
  )
}
