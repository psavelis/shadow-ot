'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import { motion, AnimatePresence } from 'framer-motion'
import {
  Users, Skull, Crown, Swords, Activity, Globe, Clock,
  ChevronRight, Search, Filter, Eye, Zap, Shield, Target,
  Flame, Timer, MapPin, Loader2, AlertCircle
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'
import { 
  usePlayerEvents, 
  useLevelUpFeed, 
  useDeathFeed, 
  useOnlinePlayersCount,
  useServerStats,
  useRealmStatus
} from '@/shared/hooks/useRealtime'
import { useRealms } from '@/shared/hooks/useRealms'
import type { RealmId } from '@/shared/types'

const vocationColors: Record<string, string> = {
  'Elite Knight': 'text-amber-400',
  'Knight': 'text-amber-400',
  'Royal Paladin': 'text-emerald-400',
  'Paladin': 'text-emerald-400',
  'Master Sorcerer': 'text-blue-400',
  'Sorcerer': 'text-blue-400',
  'Elder Druid': 'text-purple-400',
  'Druid': 'text-purple-400',
  'None': 'text-slate-400',
}

const realmColors: Record<string, string> = {
  'shadowveil': 'bg-purple-500/20 text-purple-400',
  'aetheria': 'bg-blue-500/20 text-blue-400',
  'warbound': 'bg-red-500/20 text-red-400',
  'mythara': 'bg-green-500/20 text-green-400',
  'voidborne': 'bg-indigo-500/20 text-indigo-400',
  'grimhollow': 'bg-slate-500/20 text-slate-400',
}

export default function LivePage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedRealm, setSelectedRealm] = useState<RealmId | 'all'>('all')
  const [currentTime, setCurrentTime] = useState(new Date())

  // Real-time hooks
  const { events: playerEvents, isConnected } = usePlayerEvents(50)
  const { levelUps } = useLevelUpFeed(10)
  const { deaths } = useDeathFeed(20)
  const { count: onlineCount } = useOnlinePlayersCount()
  const { stats: serverStats } = useServerStats()
  const { data: realms, isLoading: realmsLoading } = useRealms()

  // Update time every second for real-time feel
  useEffect(() => {
    const interval = setInterval(() => setCurrentTime(new Date()), 1000)
    return () => clearInterval(interval)
  }, [])

  // Filter deaths (kill feed) by realm
  const filteredDeaths = deaths.filter(death => {
    const matchesRealm = selectedRealm === 'all' || death.realm === selectedRealm
    return matchesRealm
  })

  // Filter recent logins by search and realm
  const filteredLogins = playerEvents
    .filter(e => e.type === 'login')
    .filter(event => {
      const matchesSearch = event.characterName.toLowerCase().includes(searchQuery.toLowerCase())
      const matchesRealm = selectedRealm === 'all' || event.realm === selectedRealm
      return matchesSearch && matchesRealm
    })

  const formatTimeAgo = (timestamp: number | string) => {
    const time = typeof timestamp === 'string' ? new Date(timestamp).getTime() : timestamp
    const diff = Date.now() - time
    const minutes = Math.floor(diff / 60000)
    if (minutes < 1) return 'Just now'
    if (minutes < 60) return `${minutes} min ago`
    const hours = Math.floor(minutes / 60)
    if (hours < 24) return `${hours}h ago`
    return `${Math.floor(hours / 24)}d ago`
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
          <h1 className="text-2xl font-bold text-white mb-1 flex items-center gap-3">
            <Activity className="w-6 h-6 text-emerald-400" />
            Live Activity
          </h1>
          <p className="text-slate-400">Real-time updates from all realms</p>
        </div>
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2">
            <span className={`w-2 h-2 rounded-full ${isConnected ? 'bg-emerald-400 animate-pulse' : 'bg-red-400'}`} />
            <span className="text-slate-400 text-sm">
              {isConnected ? 'Connected' : 'Connecting...'}
            </span>
          </div>
          <div className="px-4 py-2 bg-slate-800/50 rounded-lg border border-slate-700/50">
            <p className="text-white font-mono text-lg">
              {currentTime.toLocaleTimeString()}
            </p>
          </div>
        </div>
      </motion.div>

      {/* Stats Bar */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid grid-cols-2 md:grid-cols-4 gap-4"
      >
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center gap-2 text-emerald-400 mb-2">
            <Users className="w-5 h-5" />
            <span className="text-sm">Online Players</span>
          </div>
          <p className="text-2xl font-bold text-white">{onlineCount.toLocaleString()}</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center gap-2 text-blue-400 mb-2">
            <Globe className="w-5 h-5" />
            <span className="text-sm">Active Realms</span>
          </div>
          <p className="text-2xl font-bold text-white">{serverStats?.realmsOnline || realms?.filter(r => r.status === 'online').length || 0}</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center gap-2 text-amber-400 mb-2">
            <Zap className="w-5 h-5" />
            <span className="text-sm">Level Ups Today</span>
          </div>
          <p className="text-2xl font-bold text-white">{levelUps.length}</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center gap-2 text-red-400 mb-2">
            <Skull className="w-5 h-5" />
            <span className="text-sm">Deaths Today</span>
          </div>
          <p className="text-2xl font-bold text-white">{deaths.length}</p>
        </div>
      </motion.div>

      {/* Filters */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
        className="flex flex-wrap items-center gap-4 bg-slate-800/50 border border-slate-700/50 rounded-xl p-4"
      >
        <div className="relative flex-1 min-w-[200px]">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
          <input
            type="text"
            placeholder="Search players..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full bg-slate-900/50 border border-slate-700 rounded-lg pl-10 pr-4 py-2 text-white text-sm placeholder-slate-500 focus:outline-none focus:border-emerald-500/50"
          />
        </div>
        <select
          value={selectedRealm}
          onChange={(e) => setSelectedRealm(e.target.value as RealmId | 'all')}
          className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-emerald-500/50"
        >
          <option value="all">All Realms</option>
          {realms?.map(realm => (
            <option key={realm.id} value={realm.id}>{realm.name}</option>
          ))}
        </select>
      </motion.div>

      {/* Main Content */}
      <Tabs.Root defaultValue="feed" className="space-y-6">
        <Tabs.List className="flex gap-2 border-b border-slate-700 pb-2">
          <Tabs.Trigger
            value="feed"
            className="px-4 py-2 text-sm font-medium text-slate-400 hover:text-white transition data-[state=active]:text-white data-[state=active]:border-b-2 data-[state=active]:border-emerald-500"
          >
            Kill Feed
          </Tabs.Trigger>
          <Tabs.Trigger
            value="levels"
            className="px-4 py-2 text-sm font-medium text-slate-400 hover:text-white transition data-[state=active]:text-white data-[state=active]:border-b-2 data-[state=active]:border-emerald-500"
          >
            Level Ups
          </Tabs.Trigger>
          <Tabs.Trigger
            value="logins"
            className="px-4 py-2 text-sm font-medium text-slate-400 hover:text-white transition data-[state=active]:text-white data-[state=active]:border-b-2 data-[state=active]:border-emerald-500"
          >
            Recent Logins
          </Tabs.Trigger>
        </Tabs.List>

        {/* Kill Feed Tab */}
        <Tabs.Content value="feed">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
          >
            <div className="p-4 border-b border-slate-700 flex items-center justify-between">
              <h2 className="text-white font-bold flex items-center gap-2">
                <Skull className="w-5 h-5 text-red-400" />
                Live Kill Feed
              </h2>
              <span className="text-slate-500 text-sm">{filteredDeaths.length} deaths</span>
            </div>
            {filteredDeaths.length > 0 ? (
              <div className="divide-y divide-slate-700/50">
                <AnimatePresence mode="popLayout">
                  {filteredDeaths.map((death, idx) => (
                    <motion.div
                      key={`${death.characterId}-${death.timestamp}`}
                      initial={{ opacity: 0, x: -20 }}
                      animate={{ opacity: 1, x: 0 }}
                      exit={{ opacity: 0, x: 20 }}
                      transition={{ delay: idx * 0.02 }}
                      className="p-4 hover:bg-slate-700/30 transition"
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex-1">
                          <p className="text-sm">
                            <span className="text-red-400 font-medium">{death.killedBy}</span>
                            {death.isPlayer && <span className="text-slate-500 text-xs"> (Player)</span>}
                            <span className="text-slate-400"> killed </span>
                            <span className="text-white font-medium">{death.characterName}</span>
                            <span className="text-slate-500 text-xs"> (Level {death.level})</span>
                          </p>
                          <div className="flex items-center gap-2 mt-1">
                            <span className={`text-xs px-2 py-0.5 rounded ${realmColors[death.realm] || 'bg-slate-500/20 text-slate-400'}`}>
                              {death.realm}
                            </span>
                            <span className="text-xs text-slate-500 flex items-center gap-1">
                              <Clock className="w-3 h-3" />
                              {formatTimeAgo(death.timestamp)}
                            </span>
                          </div>
                        </div>
                        <span className={`px-2 py-1 text-xs rounded ${death.isPlayer ? 'bg-red-500/20 text-red-400' : 'bg-amber-500/20 text-amber-400'}`}>
                          {death.isPlayer ? 'PvP' : 'PvE'}
                        </span>
                      </div>
                    </motion.div>
                  ))}
                </AnimatePresence>
              </div>
            ) : (
              <div className="p-12 text-center text-slate-400">
                <Skull className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>No deaths recorded</p>
              </div>
            )}
          </motion.div>
        </Tabs.Content>

        {/* Level Ups Tab */}
        <Tabs.Content value="levels">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
          >
            <div className="p-4 border-b border-slate-700 flex items-center justify-between">
              <h2 className="text-white font-bold flex items-center gap-2">
                <Zap className="w-5 h-5 text-amber-400" />
                Recent Level Ups
              </h2>
              <span className="text-slate-500 text-sm">{levelUps.length} level ups</span>
            </div>
            {levelUps.length > 0 ? (
              <div className="divide-y divide-slate-700/50">
                <AnimatePresence mode="popLayout">
                  {levelUps.map((levelUp, idx) => (
                    <motion.div
                      key={`${levelUp.characterId}-${levelUp.newLevel}`}
                      initial={{ opacity: 0, x: -20 }}
                      animate={{ opacity: 1, x: 0 }}
                      exit={{ opacity: 0, x: 20 }}
                      transition={{ delay: idx * 0.02 }}
                      className="p-4 hover:bg-slate-700/30 transition"
                    >
                      <div className="flex items-center justify-between">
                        <div>
                          <p className="text-sm">
                            <span className="text-white font-medium">{levelUp.characterName}</span>
                            <span className="text-slate-400"> advanced to level </span>
                            <span className="text-amber-400 font-bold">{levelUp.newLevel}</span>
                          </p>
                          <div className="flex items-center gap-2 mt-1">
                            <span className={`text-xs px-2 py-0.5 rounded ${realmColors[levelUp.realm] || 'bg-slate-500/20 text-slate-400'}`}>
                              {levelUp.realm}
                            </span>
                            <span className="text-xs text-slate-500 flex items-center gap-1">
                              <Clock className="w-3 h-3" />
                              {formatTimeAgo(levelUp.timestamp)}
                            </span>
                          </div>
                        </div>
                        <div className="text-right">
                          <p className="text-slate-400 text-sm">
                            {levelUp.oldLevel} → {levelUp.newLevel}
                          </p>
                          <p className="text-emerald-400 text-xs">+1 Level</p>
                        </div>
                      </div>
                    </motion.div>
                  ))}
                </AnimatePresence>
              </div>
            ) : (
              <div className="p-12 text-center text-slate-400">
                <Zap className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>No level ups yet</p>
              </div>
            )}
          </motion.div>
        </Tabs.Content>

        {/* Recent Logins Tab */}
        <Tabs.Content value="logins">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
          >
            <div className="p-4 border-b border-slate-700 flex items-center justify-between">
              <h2 className="text-white font-bold flex items-center gap-2">
                <Users className="w-5 h-5 text-emerald-400" />
                Recent Logins
              </h2>
              <span className="text-slate-500 text-sm">{filteredLogins.length} logins</span>
            </div>
            {filteredLogins.length > 0 ? (
              <div className="divide-y divide-slate-700/50">
                <AnimatePresence mode="popLayout">
                  {filteredLogins.map((login, idx) => (
                    <motion.div
                      key={`${login.characterId}-${login.timestamp}`}
                      initial={{ opacity: 0, x: -20 }}
                      animate={{ opacity: 1, x: 0 }}
                      exit={{ opacity: 0, x: 20 }}
                      transition={{ delay: idx * 0.02 }}
                      className="p-4 hover:bg-slate-700/30 transition"
                    >
                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-4">
                          <div className="w-10 h-10 bg-slate-700/50 rounded-lg flex items-center justify-center">
                            <Users className="w-5 h-5 text-slate-400" />
                          </div>
                          <div>
                            <p className="text-white font-medium">{login.characterName}</p>
                            <p className="text-sm">
                              <span className={vocationColors[login.vocation] || 'text-slate-400'}>
                                {login.vocation}
                              </span>
                              <span className="text-slate-500"> • Level {login.level}</span>
                            </p>
                          </div>
                        </div>
                        <div className="text-right">
                          <span className={`text-xs px-2 py-0.5 rounded ${realmColors[login.realm] || 'bg-slate-500/20 text-slate-400'}`}>
                            {login.realm}
                          </span>
                          <p className="text-xs text-slate-500 mt-1">
                            {formatTimeAgo(login.timestamp)}
                          </p>
                        </div>
                      </div>
                    </motion.div>
                  ))}
                </AnimatePresence>
              </div>
            ) : (
              <div className="p-12 text-center text-slate-400">
                <Users className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>No recent logins</p>
              </div>
            )}
          </motion.div>
        </Tabs.Content>
      </Tabs.Root>
    </div>
  )
}
