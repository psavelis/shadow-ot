'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Skull, Swords, Target, Trophy, TrendingUp, TrendingDown,
  Filter, Calendar, Globe, Users, Crown, Flame, Shield,
  ChevronDown, Search, Zap, Clock
} from 'lucide-react'

const timeRanges = ['Today', 'This Week', 'This Month', 'All Time']
const killTypes = ['All', 'PvP Only', 'PvE Only', 'Boss Kills']

// Mock data for kill statistics
const topKillers = [
  { rank: 1, name: 'Shadowbane', vocation: 'Elite Knight', kills: 1247, deaths: 89, kd: 14.01, realm: 'Nova', level: 892 },
  { rank: 2, name: 'DarkMage', vocation: 'Master Sorcerer', kills: 1156, deaths: 134, kd: 8.63, realm: 'Eldoria', level: 756 },
  { rank: 3, name: 'ArrowStorm', vocation: 'Royal Paladin', kills: 1089, deaths: 67, kd: 16.25, realm: 'Nova', level: 834 },
  { rank: 4, name: 'HealingLight', vocation: 'Elder Druid', kills: 987, deaths: 45, kd: 21.93, realm: 'Shadowlands', level: 901 },
  { rank: 5, name: 'BladeDancer', vocation: 'Elite Knight', kills: 923, deaths: 156, kd: 5.92, realm: 'Nova', level: 678 },
  { rank: 6, name: 'FrostMage', vocation: 'Master Sorcerer', kills: 867, deaths: 98, kd: 8.85, realm: 'Ancient', level: 712 },
  { rank: 7, name: 'SwiftArrow', vocation: 'Royal Paladin', kills: 812, deaths: 78, kd: 10.41, realm: 'Eldoria', level: 645 },
  { rank: 8, name: 'NatureWard', vocation: 'Elder Druid', kills: 756, deaths: 56, kd: 13.50, realm: 'Nova', level: 789 },
]

const recentDeaths = [
  { victim: 'LoneWolf', killer: 'Shadowbane', level: 234, killerLevel: 892, time: '2 min ago', type: 'pvp', realm: 'Nova' },
  { victim: 'DragonSlayer', killer: 'Demon Lord', level: 456, killerLevel: 0, time: '5 min ago', type: 'pve', realm: 'Eldoria' },
  { victim: 'MagicUser', killer: 'DarkMage', level: 189, killerLevel: 756, time: '8 min ago', type: 'pvp', realm: 'Eldoria' },
  { victim: 'TankMaster', killer: 'Orshabaal', level: 567, killerLevel: 0, time: '12 min ago', type: 'boss', realm: 'Shadowlands' },
  { victim: 'SpeedRunner', killer: 'ArrowStorm', level: 123, killerLevel: 834, time: '15 min ago', type: 'pvp', realm: 'Nova' },
  { victim: 'NightBlade', killer: 'Ghazbaran', level: 678, killerLevel: 0, time: '18 min ago', type: 'boss', realm: 'Ancient' },
  { victim: 'HolyWarrior', killer: 'BladeDancer', level: 345, killerLevel: 678, time: '22 min ago', type: 'pvp', realm: 'Nova' },
  { victim: 'ShadowMage', killer: 'Dragon Lord', level: 234, killerLevel: 0, time: '25 min ago', type: 'pve', realm: 'Eldoria' },
]

const serverStats = {
  totalKills: 1567234,
  totalDeaths: 892456,
  pvpKills: 456789,
  bossKills: 234567,
  avgKillsPerDay: 12456,
  mostDangerousRealm: 'Nova',
  safestRealm: 'Ancient',
}

const bossKillLeaders = [
  { name: 'BossHunter', kills: 156, bosses: ['Ferumbras', 'Morgaroth', 'Ghazbaran'], realm: 'Shadowlands' },
  { name: 'DragonSlayer', kills: 134, bosses: ['Orshabaal', 'The Pale Count', 'Zugurosh'], realm: 'Nova' },
  { name: 'DemonBane', kills: 128, bosses: ['Ferumbras', 'The Welter', 'Mazoran'], realm: 'Eldoria' },
]

export default function KillStatisticsPage() {
  const [timeRange, setTimeRange] = useState('This Week')
  const [killType, setKillType] = useState('All')
  const [selectedRealm, setSelectedRealm] = useState('All Realms')
  const [searchQuery, setSearchQuery] = useState('')

  const getKillTypeColor = (type: string) => {
    switch (type) {
      case 'pvp': return 'text-red-400 bg-red-500/20'
      case 'pve': return 'text-amber-400 bg-amber-500/20'
      case 'boss': return 'text-purple-400 bg-purple-500/20'
      default: return 'text-slate-400 bg-slate-500/20'
    }
  }

  const getVocationColor = (vocation: string) => {
    if (vocation.includes('Knight')) return 'text-red-400'
    if (vocation.includes('Sorcerer')) return 'text-blue-400'
    if (vocation.includes('Druid')) return 'text-green-400'
    if (vocation.includes('Paladin')) return 'text-amber-400'
    return 'text-slate-400'
  }

  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      {/* Hero Section */}
      <section className="relative py-20 overflow-hidden">
        <div className="absolute inset-0 bg-[url('/grid.svg')] opacity-5" />
        <div className="absolute inset-0 bg-gradient-to-b from-red-500/5 via-transparent to-transparent" />
        
        <div className="container mx-auto px-4 relative z-10">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-center max-w-3xl mx-auto"
          >
            <div className="inline-flex items-center gap-2 px-4 py-2 bg-red-500/10 border border-red-500/30 rounded-full text-red-400 text-sm mb-6">
              <Skull className="w-4 h-4" />
              Live Kill Statistics
            </div>
            <h1 className="text-4xl md:text-5xl font-bold text-white mb-4">
              Battle <span className="text-red-500">Statistics</span>
            </h1>
            <p className="text-slate-400 text-lg">
              Track PvP battles, monster kills, and boss encounters across all Shadow OT realms
            </p>
          </motion.div>
        </div>
      </section>

      {/* Stats Overview */}
      <section className="container mx-auto px-4 -mt-8">
        <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-7 gap-4">
          {[
            { label: 'Total Kills', value: serverStats.totalKills.toLocaleString(), icon: Swords, color: 'red' },
            { label: 'Total Deaths', value: serverStats.totalDeaths.toLocaleString(), icon: Skull, color: 'slate' },
            { label: 'PvP Kills', value: serverStats.pvpKills.toLocaleString(), icon: Target, color: 'orange' },
            { label: 'Boss Kills', value: serverStats.bossKills.toLocaleString(), icon: Crown, color: 'purple' },
            { label: 'Kills/Day', value: serverStats.avgKillsPerDay.toLocaleString(), icon: TrendingUp, color: 'green' },
            { label: 'Most Dangerous', value: serverStats.mostDangerousRealm, icon: Flame, color: 'red' },
            { label: 'Safest Realm', value: serverStats.safestRealm, icon: Shield, color: 'cyan' },
          ].map((stat, idx) => (
            <motion.div
              key={stat.label}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: idx * 0.05 }}
              className={`bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 text-center`}
            >
              <stat.icon className={`w-5 h-5 text-${stat.color}-400 mx-auto mb-2`} />
              <p className="text-white font-bold text-lg">{stat.value}</p>
              <p className="text-slate-500 text-xs">{stat.label}</p>
            </motion.div>
          ))}
        </div>
      </section>

      {/* Filters */}
      <section className="container mx-auto px-4 py-8">
        <div className="flex flex-wrap items-center gap-4 bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <div className="flex items-center gap-2 text-slate-400">
            <Filter className="w-4 h-4" />
            <span className="text-sm font-medium">Filters:</span>
          </div>
          
          {/* Time Range */}
          <div className="relative">
            <select
              value={timeRange}
              onChange={(e) => setTimeRange(e.target.value)}
              className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-red-500/50"
            >
              {timeRanges.map(range => (
                <option key={range} value={range}>{range}</option>
              ))}
            </select>
            <Calendar className="absolute right-2 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500 pointer-events-none" />
          </div>

          {/* Kill Type */}
          <div className="relative">
            <select
              value={killType}
              onChange={(e) => setKillType(e.target.value)}
              className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-red-500/50"
            >
              {killTypes.map(type => (
                <option key={type} value={type}>{type}</option>
              ))}
            </select>
            <Swords className="absolute right-2 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500 pointer-events-none" />
          </div>

          {/* Realm */}
          <div className="relative">
            <select
              value={selectedRealm}
              onChange={(e) => setSelectedRealm(e.target.value)}
              className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-red-500/50"
            >
              <option>All Realms</option>
              <option>Nova</option>
              <option>Eldoria</option>
              <option>Shadowlands</option>
              <option>Ancient Kingdoms</option>
            </select>
            <Globe className="absolute right-2 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500 pointer-events-none" />
          </div>

          {/* Search */}
          <div className="relative flex-1 min-w-[200px]">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
            <input
              type="text"
              placeholder="Search player..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-slate-900/50 border border-slate-700 rounded-lg pl-10 pr-4 py-2 text-white text-sm placeholder-slate-500 focus:outline-none focus:border-red-500/50"
            />
          </div>
        </div>
      </section>

      {/* Main Content */}
      <section className="container mx-auto px-4 pb-20">
        <div className="grid lg:grid-cols-3 gap-6">
          {/* Top Killers Leaderboard */}
          <div className="lg:col-span-2">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
            >
              <div className="p-4 border-b border-slate-700 flex items-center justify-between">
                <h2 className="text-white font-bold flex items-center gap-2">
                  <Trophy className="w-5 h-5 text-amber-400" />
                  Top Killers
                </h2>
                <span className="text-slate-500 text-sm">{timeRange}</span>
              </div>
              
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead className="bg-slate-900/50">
                    <tr className="text-slate-400 text-sm">
                      <th className="text-left p-4">#</th>
                      <th className="text-left p-4">Player</th>
                      <th className="text-left p-4">Vocation</th>
                      <th className="text-right p-4">Kills</th>
                      <th className="text-right p-4">Deaths</th>
                      <th className="text-right p-4">K/D</th>
                      <th className="text-left p-4">Realm</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-700/50">
                    {topKillers.map((player) => (
                      <tr key={player.rank} className="hover:bg-slate-700/30 transition">
                        <td className="p-4">
                          {player.rank <= 3 ? (
                            <div className={`w-8 h-8 rounded-lg flex items-center justify-center font-bold ${
                              player.rank === 1 ? 'bg-amber-500/20 text-amber-400' :
                              player.rank === 2 ? 'bg-slate-400/20 text-slate-300' :
                              'bg-orange-500/20 text-orange-400'
                            }`}>
                              {player.rank}
                            </div>
                          ) : (
                            <span className="text-slate-500 font-medium pl-2">{player.rank}</span>
                          )}
                        </td>
                        <td className="p-4">
                          <div>
                            <p className="text-white font-medium hover:text-red-400 cursor-pointer transition">
                              {player.name}
                            </p>
                            <p className="text-slate-500 text-xs">Level {player.level}</p>
                          </div>
                        </td>
                        <td className="p-4">
                          <span className={`text-sm ${getVocationColor(player.vocation)}`}>
                            {player.vocation}
                          </span>
                        </td>
                        <td className="p-4 text-right">
                          <span className="text-red-400 font-bold">{player.kills.toLocaleString()}</span>
                        </td>
                        <td className="p-4 text-right">
                          <span className="text-slate-400">{player.deaths.toLocaleString()}</span>
                        </td>
                        <td className="p-4 text-right">
                          <span className={`font-medium ${player.kd >= 10 ? 'text-emerald-400' : player.kd >= 5 ? 'text-amber-400' : 'text-slate-400'}`}>
                            {player.kd.toFixed(2)}
                          </span>
                        </td>
                        <td className="p-4">
                          <span className="px-2 py-1 bg-slate-700/50 text-slate-300 text-xs rounded">
                            {player.realm}
                          </span>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
              
              <div className="p-4 border-t border-slate-700 text-center">
                <button className="text-red-400 text-sm hover:text-red-300 transition">
                  View Full Leaderboard →
                </button>
              </div>
            </motion.div>
          </div>

          {/* Sidebar */}
          <div className="space-y-6">
            {/* Live Kill Feed */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.1 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
            >
              <div className="p-4 border-b border-slate-700 flex items-center justify-between">
                <h3 className="text-white font-bold flex items-center gap-2">
                  <Zap className="w-4 h-4 text-amber-400" />
                  Live Kill Feed
                </h3>
                <div className="flex items-center gap-1 text-emerald-400 text-xs">
                  <span className="w-2 h-2 bg-emerald-400 rounded-full animate-pulse" />
                  Live
                </div>
              </div>
              
              <div className="divide-y divide-slate-700/50 max-h-[400px] overflow-y-auto">
                {recentDeaths.map((death, idx) => (
                  <div key={idx} className="p-3 hover:bg-slate-700/30 transition">
                    <div className="flex items-center justify-between mb-1">
                      <span className={`px-2 py-0.5 text-xs rounded ${getKillTypeColor(death.type)}`}>
                        {death.type.toUpperCase()}
                      </span>
                      <span className="text-slate-500 text-xs flex items-center gap-1">
                        <Clock className="w-3 h-3" />
                        {death.time}
                      </span>
                    </div>
                    <p className="text-sm">
                      <span className="text-red-400 font-medium">{death.killer}</span>
                      {death.killerLevel > 0 && <span className="text-slate-500 text-xs"> (Lvl {death.killerLevel})</span>}
                      <span className="text-slate-400"> killed </span>
                      <span className="text-white font-medium">{death.victim}</span>
                      <span className="text-slate-500 text-xs"> (Lvl {death.level})</span>
                    </p>
                    <p className="text-slate-600 text-xs mt-1">{death.realm}</p>
                  </div>
                ))}
              </div>
            </motion.div>

            {/* Boss Kill Leaders */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.2 }}
              className="bg-gradient-to-br from-purple-500/10 to-pink-500/5 border border-purple-500/30 rounded-xl p-4"
            >
              <h3 className="text-white font-bold flex items-center gap-2 mb-4">
                <Crown className="w-5 h-5 text-purple-400" />
                Boss Hunters
              </h3>
              
              <div className="space-y-3">
                {bossKillLeaders.map((hunter, idx) => (
                  <div key={idx} className="bg-slate-900/50 rounded-lg p-3">
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-white font-medium">{hunter.name}</span>
                      <span className="text-purple-400 font-bold">{hunter.kills} kills</span>
                    </div>
                    <div className="flex flex-wrap gap-1">
                      {hunter.bosses.map((boss, i) => (
                        <span key={i} className="px-2 py-0.5 bg-purple-500/20 text-purple-300 text-xs rounded">
                          {boss}
                        </span>
                      ))}
                    </div>
                    <p className="text-slate-500 text-xs mt-2">{hunter.realm}</p>
                  </div>
                ))}
              </div>
            </motion.div>
          </div>
        </div>
      </section>
    </main>
  )
}

