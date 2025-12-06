'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import { motion, AnimatePresence } from 'framer-motion'
import {
  Users, Skull, Crown, Swords, Activity, Globe, Clock,
  ChevronRight, Search, Filter, Eye, Zap, Shield, Target,
  Flame, Timer, MapPin
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'

// Mock real-time data
const onlinePlayers = [
  { id: 1, name: 'ShadowKnight', level: 342, vocation: 'Elite Knight', realm: 'Shadowlands', guild: 'Dark Legion', status: 'hunting' },
  { id: 2, name: 'MysticDruid', level: 298, vocation: 'Elder Druid', realm: 'Shadowlands', guild: 'Mystic Order', status: 'trading' },
  { id: 3, name: 'StormMage', level: 425, vocation: 'Master Sorcerer', realm: 'Mythara', guild: null, status: 'afk' },
  { id: 4, name: 'SwiftArrow', level: 267, vocation: 'Royal Paladin', realm: 'Shadowlands', guild: 'Rangers Guild', status: 'hunting' },
  { id: 5, name: 'DarkBlade', level: 512, vocation: 'Elite Knight', realm: 'Voidborne', guild: 'Shadow Hunters', status: 'pvp' },
  { id: 6, name: 'HealingLight', level: 189, vocation: 'Elder Druid', realm: 'Aetheria', guild: 'Holy Order', status: 'hunting' },
  { id: 7, name: 'ThunderBolt', level: 378, vocation: 'Master Sorcerer', realm: 'Shadowlands', guild: 'Storm Casters', status: 'hunting' },
  { id: 8, name: 'IronFist', level: 445, vocation: 'Elite Knight', realm: 'Warbound', guild: 'Iron Legion', status: 'pvp' },
]

const killFeed = [
  { id: 1, killer: 'DarkBlade', killerLevel: 512, victim: 'EnemyPlayer', victimLevel: 489, time: '2 min ago', realm: 'Voidborne', type: 'pvp' },
  { id: 2, killer: 'Demon', killerLevel: null, victim: 'UnluckyMage', victimLevel: 234, time: '5 min ago', realm: 'Shadowlands', type: 'pve' },
  { id: 3, killer: 'IronFist', killerLevel: 445, victim: 'SwiftKiller', victimLevel: 412, time: '8 min ago', realm: 'Warbound', type: 'pvp' },
  { id: 4, killer: 'Orshabaal', killerLevel: null, victim: 'BraveSoul', victimLevel: 356, time: '12 min ago', realm: 'Mythara', type: 'pve' },
  { id: 5, killer: 'ThunderBolt', killerLevel: 378, victim: 'RivalMage', victimLevel: 345, time: '15 min ago', realm: 'Shadowlands', type: 'pvp' },
  { id: 6, killer: 'Dragon Lord', killerLevel: null, victim: 'DragonHunter', victimLevel: 287, time: '18 min ago', realm: 'Aetheria', type: 'pve' },
]

const bossTimers = [
  { name: 'Orshabaal', realm: 'Mythara', status: 'spawned', location: 'Edron', lastKill: '2h ago', respawnWindow: '5-7 days' },
  { name: 'Morgaroth', realm: 'Shadowlands', status: 'window', location: 'Goroma', lastKill: '5d ago', respawnWindow: '5-7 days', windowStart: '2h ago' },
  { name: 'Ghazbaran', realm: 'Aetheria', status: 'unknown', location: 'Demona', lastKill: '3d ago', respawnWindow: '5-7 days' },
  { name: 'Ferumbras', realm: 'Voidborne', status: 'cooldown', location: 'Citadel', lastKill: '1d ago', respawnWindow: '7-10 days', cooldownRemaining: '4d' },
  { name: 'The Welter', realm: 'Warbound', status: 'window', location: 'Zao', lastKill: '6d ago', respawnWindow: '5-7 days', windowStart: '1d ago' },
]

const levelUps = [
  { name: 'ShadowKnight', oldLevel: 341, newLevel: 342, time: '10 min ago', realm: 'Shadowlands' },
  { name: 'SwiftArrow', oldLevel: 266, newLevel: 267, time: '25 min ago', realm: 'Shadowlands' },
  { name: 'StormMage', oldLevel: 424, newLevel: 425, time: '45 min ago', realm: 'Mythara' },
  { name: 'HealingLight', oldLevel: 188, newLevel: 189, time: '1h ago', realm: 'Aetheria' },
]

const vocationColors: Record<string, string> = {
  'Elite Knight': 'text-amber-400',
  'Royal Paladin': 'text-emerald-400',
  'Master Sorcerer': 'text-blue-400',
  'Elder Druid': 'text-purple-400',
}

const realmColors: Record<string, string> = {
  'Shadowlands': 'bg-amber-500/20 text-amber-400',
  'Mythara': 'bg-emerald-500/20 text-emerald-400',
  'Aetheria': 'bg-purple-500/20 text-purple-400',
  'Voidborne': 'bg-red-500/20 text-red-400',
  'Warbound': 'bg-orange-500/20 text-orange-400',
}

export default function LivePage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedRealm, setSelectedRealm] = useState('all')
  const [currentTime, setCurrentTime] = useState(new Date())

  // Update time every second for real-time feel
  useEffect(() => {
    const interval = setInterval(() => setCurrentTime(new Date()), 1000)
    return () => clearInterval(interval)
  }, [])

  const filteredPlayers = onlinePlayers.filter(player => {
    const matchesSearch = player.name.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesRealm = selectedRealm === 'all' || player.realm === selectedRealm
    return matchesSearch && matchesRealm
  })

  return (
    <div className="space-y-6">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex items-center justify-between"
      >
        <div>
          <h1 className="text-2xl font-bold text-white mb-1">Live Activity</h1>
          <p className="text-slate-400">Real-time game activity and statistics</p>
        </div>
        <div className="flex items-center gap-2 px-4 py-2 bg-slate-800/50 border border-slate-700/50 rounded-lg">
          <Activity className="w-4 h-4 text-emerald-400 animate-pulse" />
          <span className="text-white text-sm">
            {currentTime.toLocaleTimeString()}
          </span>
        </div>
      </motion.div>

      {/* Quick Stats */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid grid-cols-4 gap-4"
      >
        {[
          { label: 'Players Online', value: '1,247', icon: Users, color: 'emerald', change: '+23' },
          { label: 'Active Battles', value: '18', icon: Swords, color: 'red', change: null },
          { label: 'Bosses Spawned', value: '3', icon: Crown, color: 'amber', change: null },
          { label: 'Deaths (24h)', value: '847', icon: Skull, color: 'slate', change: '-12%' },
        ].map((stat, idx) => (
          <div key={idx} className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
            <div className="flex items-center justify-between mb-2">
              <stat.icon className={`w-5 h-5 text-${stat.color}-400`} />
              {stat.change && (
                <span className={`text-xs ${stat.change.startsWith('+') ? 'text-emerald-400' : stat.change.startsWith('-') ? 'text-red-400' : 'text-slate-400'}`}>
                  {stat.change}
                </span>
              )}
            </div>
            <p className="text-2xl font-bold text-white">{stat.value}</p>
            <p className="text-xs text-slate-500">{stat.label}</p>
          </div>
        ))}
      </motion.div>

      {/* Main Content Tabs */}
      <Tabs.Root defaultValue="online">
        <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1 mb-6">
          {[
            { id: 'online', label: 'Online Players', icon: Users },
            { id: 'kills', label: 'Kill Feed', icon: Skull },
            { id: 'bosses', label: 'Boss Tracker', icon: Crown },
            { id: 'levels', label: 'Level Ups', icon: Zap },
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

        {/* Online Players Tab */}
        <Tabs.Content value="online">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-4"
          >
            {/* Filters */}
            <div className="flex gap-4">
              <div className="flex-1 relative">
                <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
                <input
                  type="text"
                  placeholder="Search players..."
                  value={searchQuery}
                  onChange={e => setSearchQuery(e.target.value)}
                  className="w-full bg-slate-800/50 border border-slate-700/50 rounded-lg pl-10 pr-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
                />
              </div>
              <select
                value={selectedRealm}
                onChange={e => setSelectedRealm(e.target.value)}
                className="bg-slate-800/50 border border-slate-700/50 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-amber-500/50"
              >
                <option value="all">All Realms</option>
                <option value="Shadowlands">Shadowlands</option>
                <option value="Mythara">Mythara</option>
                <option value="Aetheria">Aetheria</option>
                <option value="Voidborne">Voidborne</option>
                <option value="Warbound">Warbound</option>
              </select>
            </div>

            {/* Player List */}
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden">
              <div className="grid grid-cols-12 gap-4 px-4 py-3 bg-slate-900/50 border-b border-slate-700/50 text-xs font-medium text-slate-500">
                <div className="col-span-4">Player</div>
                <div className="col-span-2">Level</div>
                <div className="col-span-2">Vocation</div>
                <div className="col-span-2">Realm</div>
                <div className="col-span-2">Status</div>
              </div>
              <div className="divide-y divide-slate-700/50">
                {filteredPlayers.map((player, idx) => (
                  <motion.div
                    key={player.id}
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    transition={{ delay: idx * 0.03 }}
                    className="grid grid-cols-12 gap-4 px-4 py-3 hover:bg-slate-700/30 transition group"
                  >
                    <div className="col-span-4 flex items-center gap-3">
                      <div className="w-2 h-2 bg-emerald-400 rounded-full animate-pulse" />
                      <Link
                        href={`/characters/${player.name}`}
                        className="text-white font-medium group-hover:text-amber-400 transition"
                      >
                        {player.name}
                      </Link>
                      {player.guild && (
                        <span className="text-slate-500 text-xs">({player.guild})</span>
                      )}
                    </div>
                    <div className="col-span-2 text-white">{player.level}</div>
                    <div className={`col-span-2 ${vocationColors[player.vocation]}`}>{player.vocation}</div>
                    <div className="col-span-2">
                      <span className={`px-2 py-0.5 rounded text-xs ${realmColors[player.realm]}`}>
                        {player.realm}
                      </span>
                    </div>
                    <div className="col-span-2">
                      <span className={`text-xs ${
                        player.status === 'hunting' ? 'text-emerald-400' :
                        player.status === 'pvp' ? 'text-red-400' :
                        player.status === 'trading' ? 'text-blue-400' :
                        'text-slate-500'
                      }`}>
                        {player.status}
                      </span>
                    </div>
                  </motion.div>
                ))}
              </div>
            </div>
          </motion.div>
        </Tabs.Content>

        {/* Kill Feed Tab */}
        <Tabs.Content value="kills">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
          >
            <div className="divide-y divide-slate-700/50">
              {killFeed.map((kill, idx) => (
                <motion.div
                  key={kill.id}
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: idx * 0.05 }}
                  className="px-5 py-4 hover:bg-slate-700/30 transition"
                >
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${
                        kill.type === 'pvp' ? 'bg-red-500/20' : 'bg-slate-700/50'
                      }`}>
                        {kill.type === 'pvp' ? (
                          <Swords className="w-5 h-5 text-red-400" />
                        ) : (
                          <Skull className="w-5 h-5 text-slate-400" />
                        )}
                      </div>
                      <div>
                        <p className="text-white">
                          <span className={kill.type === 'pvp' ? 'text-red-400' : 'text-orange-400'}>
                            {kill.killer}
                          </span>
                          {kill.killerLevel && <span className="text-slate-500 text-sm"> ({kill.killerLevel})</span>}
                          <span className="text-slate-400"> killed </span>
                          <span className="text-white">{kill.victim}</span>
                          <span className="text-slate-500 text-sm"> ({kill.victimLevel})</span>
                        </p>
                        <p className="text-slate-500 text-sm flex items-center gap-2">
                          <span className={`px-1.5 py-0.5 rounded text-xs ${realmColors[kill.realm]}`}>
                            {kill.realm}
                          </span>
                          <span>•</span>
                          <span>{kill.time}</span>
                        </p>
                      </div>
                    </div>
                    {kill.type === 'pvp' && (
                      <span className="px-2 py-1 bg-red-500/20 text-red-400 text-xs rounded">PvP</span>
                    )}
                  </div>
                </motion.div>
              ))}
            </div>
          </motion.div>
        </Tabs.Content>

        {/* Boss Tracker Tab */}
        <Tabs.Content value="bosses">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="grid md:grid-cols-2 gap-4"
          >
            {bossTimers.map((boss, idx) => (
              <motion.div
                key={boss.name}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: idx * 0.05 }}
                className={`bg-slate-800/50 border rounded-xl p-5 ${
                  boss.status === 'spawned' ? 'border-emerald-500/50' :
                  boss.status === 'window' ? 'border-amber-500/50' :
                  'border-slate-700/50'
                }`}
              >
                <div className="flex items-start justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${
                      boss.status === 'spawned' ? 'bg-emerald-500/20' :
                      boss.status === 'window' ? 'bg-amber-500/20' :
                      boss.status === 'cooldown' ? 'bg-blue-500/20' :
                      'bg-slate-700/50'
                    }`}>
                      <Crown className={`w-6 h-6 ${
                        boss.status === 'spawned' ? 'text-emerald-400' :
                        boss.status === 'window' ? 'text-amber-400' :
                        boss.status === 'cooldown' ? 'text-blue-400' :
                        'text-slate-400'
                      }`} />
                    </div>
                    <div>
                      <h3 className="text-white font-bold">{boss.name}</h3>
                      <p className="text-slate-500 text-sm flex items-center gap-1">
                        <MapPin className="w-3 h-3" />
                        {boss.location}
                      </p>
                    </div>
                  </div>
                  <span className={`px-2 py-0.5 rounded text-xs ${realmColors[boss.realm]}`}>
                    {boss.realm}
                  </span>
                </div>

                <div className="space-y-2 text-sm">
                  <div className="flex justify-between">
                    <span className="text-slate-400">Status</span>
                    <span className={`font-medium ${
                      boss.status === 'spawned' ? 'text-emerald-400' :
                      boss.status === 'window' ? 'text-amber-400' :
                      boss.status === 'cooldown' ? 'text-blue-400' :
                      'text-slate-400'
                    }`}>
                      {boss.status === 'spawned' ? '🟢 Spawned!' :
                       boss.status === 'window' ? '🟡 In Window' :
                       boss.status === 'cooldown' ? '⏳ Cooldown' :
                       '❓ Unknown'}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-slate-400">Last Kill</span>
                    <span className="text-white">{boss.lastKill}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-slate-400">Respawn Window</span>
                    <span className="text-white">{boss.respawnWindow}</span>
                  </div>
                  {boss.windowStart && (
                    <div className="flex justify-between">
                      <span className="text-slate-400">Window Started</span>
                      <span className="text-amber-400">{boss.windowStart}</span>
                    </div>
                  )}
                  {boss.cooldownRemaining && (
                    <div className="flex justify-between">
                      <span className="text-slate-400">Cooldown Remaining</span>
                      <span className="text-blue-400">{boss.cooldownRemaining}</span>
                    </div>
                  )}
                </div>
              </motion.div>
            ))}
          </motion.div>
        </Tabs.Content>

        {/* Level Ups Tab */}
        <Tabs.Content value="levels">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
          >
            <div className="divide-y divide-slate-700/50">
              {levelUps.map((levelUp, idx) => (
                <motion.div
                  key={idx}
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: idx * 0.05 }}
                  className="px-5 py-4 hover:bg-slate-700/30 transition flex items-center justify-between"
                >
                  <div className="flex items-center gap-4">
                    <div className="w-10 h-10 bg-amber-500/20 rounded-lg flex items-center justify-center">
                      <Zap className="w-5 h-5 text-amber-400" />
                    </div>
                    <div>
                      <p className="text-white">
                        <Link href={`/characters/${levelUp.name}`} className="text-amber-400 hover:text-amber-300 font-medium">
                          {levelUp.name}
                        </Link>
                        <span className="text-slate-400"> advanced to level </span>
                        <span className="text-white font-bold">{levelUp.newLevel}</span>
                      </p>
                      <p className="text-slate-500 text-sm flex items-center gap-2">
                        <span className={`px-1.5 py-0.5 rounded text-xs ${realmColors[levelUp.realm]}`}>
                          {levelUp.realm}
                        </span>
                        <span>•</span>
                        <span>{levelUp.time}</span>
                      </p>
                    </div>
                  </div>
                  <div className="text-slate-400 text-sm">
                    {levelUp.oldLevel} → {levelUp.newLevel}
                  </div>
                </motion.div>
              ))}
            </div>
          </motion.div>
        </Tabs.Content>
      </Tabs.Root>
    </div>
  )
}

