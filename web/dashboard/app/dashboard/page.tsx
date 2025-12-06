'use client'

import { motion } from 'framer-motion'
import Link from 'next/link'
import { 
  Users, 
  Wallet, 
  Trophy, 
  TrendingUp, 
  Clock,
  ArrowUpRight,
  ArrowDownRight,
  Swords,
  Shield,
  Wand2,
  Target,
  Sparkles,
  Crown,
  Activity
} from 'lucide-react'
import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts'

const stats = [
  { name: 'Characters', value: '4', change: '+1', changeType: 'positive', icon: Users },
  { name: 'Total Level', value: '847', change: '+23', changeType: 'positive', icon: TrendingUp },
  { name: 'NFTs Owned', value: '12', change: '+3', changeType: 'positive', icon: Wallet },
  { name: 'Achievements', value: '156', change: '+8', changeType: 'positive', icon: Trophy },
]

const characters = [
  { 
    name: 'ShadowKnight', 
    level: 312, 
    vocation: 'Elite Knight',
    realm: 'Shadowveil',
    online: true,
    lastLogin: 'Now',
    icon: Shield,
    color: 'text-red-400',
  },
  { 
    name: 'MysticDruid', 
    level: 245, 
    vocation: 'Elder Druid',
    realm: 'Aetheria',
    online: false,
    lastLogin: '2 hours ago',
    icon: Wand2,
    color: 'text-green-400',
  },
  { 
    name: 'SwiftArrow', 
    level: 198, 
    vocation: 'Royal Paladin',
    realm: 'Warbound',
    online: false,
    lastLogin: '1 day ago',
    icon: Target,
    color: 'text-yellow-400',
  },
  { 
    name: 'StormMage', 
    level: 92, 
    vocation: 'Master Sorcerer',
    realm: 'Mythara',
    online: false,
    lastLogin: '3 days ago',
    icon: Wand2,
    color: 'text-blue-400',
  },
]

const recentActivity = [
  { type: 'levelup', message: 'ShadowKnight advanced to level 312', time: '15 minutes ago' },
  { type: 'achievement', message: 'Earned "Dragon Slayer" achievement', time: '1 hour ago' },
  { type: 'nft', message: 'Minted "Demon Armor" as NFT', time: '3 hours ago' },
  { type: 'trade', message: 'Sold Magic Sword for 250,000 gold', time: '5 hours ago' },
  { type: 'quest', message: 'Completed "The Annihilator" quest', time: '1 day ago' },
]

const experienceData = [
  { name: 'Mon', exp: 2400000 },
  { name: 'Tue', exp: 1398000 },
  { name: 'Wed', exp: 9800000 },
  { name: 'Thu', exp: 3908000 },
  { name: 'Fri', exp: 4800000 },
  { name: 'Sat', exp: 12800000 },
  { name: 'Sun', exp: 8300000 },
]

const formatExp = (value: number) => {
  if (value >= 1000000) return `${(value / 1000000).toFixed(1)}M`
  if (value >= 1000) return `${(value / 1000).toFixed(0)}K`
  return value.toString()
}

export default function DashboardPage() {
  return (
    <div className="space-y-6">
      {/* Welcome */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <h1 className="text-2xl font-display font-bold mb-1">Welcome back, ShadowMaster</h1>
        <p className="text-shadow-400">Here's what's happening with your account today.</p>
      </motion.div>

      {/* Stats Grid */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4"
      >
        {stats.map((stat, index) => (
          <div key={stat.name} className="stat-card">
            <div className="flex items-center justify-between mb-3">
              <div className="w-10 h-10 bg-accent-500/10 rounded-lg flex items-center justify-center">
                <stat.icon className="w-5 h-5 text-accent-500" />
              </div>
              <span className={`flex items-center text-sm font-medium ${
                stat.changeType === 'positive' ? 'text-green-500' : 'text-red-500'
              }`}>
                {stat.changeType === 'positive' ? (
                  <ArrowUpRight className="w-4 h-4" />
                ) : (
                  <ArrowDownRight className="w-4 h-4" />
                )}
                {stat.change}
              </span>
            </div>
            <h3 className="text-2xl font-bold text-white">{stat.value}</h3>
            <p className="text-shadow-400 text-sm">{stat.name}</p>
          </div>
        ))}
      </motion.div>

      <div className="grid lg:grid-cols-3 gap-6">
        {/* Characters */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="lg:col-span-2"
        >
          <div className="card">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-semibold text-white">Your Characters</h2>
              <Link href="/dashboard/characters" className="text-sm text-accent-500 hover:text-accent-400">
                View all →
              </Link>
            </div>
            <div className="space-y-3">
              {characters.map((char) => (
                <div
                  key={char.name}
                  className="flex items-center justify-between p-3 bg-shadow-800/50 rounded-lg hover:bg-shadow-800 transition-colors"
                >
                  <div className="flex items-center space-x-4">
                    <div className={`w-10 h-10 rounded-lg bg-shadow-700 flex items-center justify-center`}>
                      <char.icon className={`w-5 h-5 ${char.color}`} />
                    </div>
                    <div>
                      <div className="flex items-center space-x-2">
                        <span className="font-medium text-white">{char.name}</span>
                        {char.online && (
                          <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
                        )}
                      </div>
                      <p className="text-sm text-shadow-400">
                        Level {char.level} {char.vocation}
                      </p>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="text-sm text-white">{char.realm}</div>
                    <div className="text-xs text-shadow-500">{char.lastLogin}</div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </motion.div>

        {/* Recent Activity */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
        >
          <div className="card h-full">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-semibold text-white">Recent Activity</h2>
              <Activity className="w-5 h-5 text-shadow-500" />
            </div>
            <div className="space-y-4">
              {recentActivity.map((activity, index) => (
                <div key={index} className="flex items-start space-x-3">
                  <div className={`w-2 h-2 rounded-full mt-2 ${
                    activity.type === 'levelup' ? 'bg-green-500' :
                    activity.type === 'achievement' ? 'bg-yellow-500' :
                    activity.type === 'nft' ? 'bg-purple-500' :
                    activity.type === 'trade' ? 'bg-blue-500' :
                    'bg-accent-500'
                  }`} />
                  <div className="flex-1 min-w-0">
                    <p className="text-sm text-white">{activity.message}</p>
                    <p className="text-xs text-shadow-500">{activity.time}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </motion.div>
      </div>

      {/* Experience Chart */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.4 }}
        className="card"
      >
        <div className="flex items-center justify-between mb-6">
          <div>
            <h2 className="text-lg font-semibold text-white">Experience Gained</h2>
            <p className="text-shadow-400 text-sm">Total experience earned this week</p>
          </div>
          <div className="text-right">
            <div className="text-2xl font-bold text-white">43.4M</div>
            <div className="flex items-center text-green-500 text-sm">
              <ArrowUpRight className="w-4 h-4" />
              <span>+12.5% from last week</span>
            </div>
          </div>
        </div>
        <div className="h-64">
          <ResponsiveContainer width="100%" height="100%">
            <AreaChart data={experienceData}>
              <defs>
                <linearGradient id="expGradient" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#e94560" stopOpacity={0.3}/>
                  <stop offset="95%" stopColor="#e94560" stopOpacity={0}/>
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="#3b3c40" />
              <XAxis dataKey="name" stroke="#696a74" fontSize={12} />
              <YAxis stroke="#696a74" fontSize={12} tickFormatter={formatExp} />
              <Tooltip 
                contentStyle={{ 
                  backgroundColor: '#1a1a2e', 
                  border: '1px solid #3b3c40',
                  borderRadius: '8px',
                }}
                labelStyle={{ color: '#fff' }}
                formatter={(value: number) => [formatExp(value) + ' exp', 'Experience']}
              />
              <Area
                type="monotone"
                dataKey="exp"
                stroke="#e94560"
                strokeWidth={2}
                fill="url(#expGradient)"
              />
            </AreaChart>
          </ResponsiveContainer>
        </div>
      </motion.div>

      {/* Quick Actions */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.5 }}
        className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4"
      >
        <Link 
          href="/dashboard/characters/create"
          className="p-4 bg-gradient-to-br from-accent-500/20 to-purple-500/20 border border-accent-500/30 rounded-xl hover:border-accent-500/50 transition-all group"
        >
          <Users className="w-8 h-8 text-accent-500 mb-3 group-hover:scale-110 transition-transform" />
          <h3 className="font-medium text-white mb-1">Create Character</h3>
          <p className="text-sm text-shadow-400">Start a new adventure</p>
        </Link>
        <Link 
          href="/dashboard/wallet"
          className="p-4 bg-gradient-to-br from-purple-500/20 to-blue-500/20 border border-purple-500/30 rounded-xl hover:border-purple-500/50 transition-all group"
        >
          <Wallet className="w-8 h-8 text-purple-500 mb-3 group-hover:scale-110 transition-transform" />
          <h3 className="font-medium text-white mb-1">Mint NFT</h3>
          <p className="text-sm text-shadow-400">Convert items to NFTs</p>
        </Link>
        <Link 
          href="/download"
          className="p-4 bg-gradient-to-br from-blue-500/20 to-cyan-500/20 border border-blue-500/30 rounded-xl hover:border-blue-500/50 transition-all group"
        >
          <Swords className="w-8 h-8 text-blue-500 mb-3 group-hover:scale-110 transition-transform" />
          <h3 className="font-medium text-white mb-1">Play Now</h3>
          <p className="text-sm text-shadow-400">Launch game client</p>
        </Link>
        <Link 
          href="/dashboard/settings"
          className="p-4 bg-gradient-to-br from-green-500/20 to-emerald-500/20 border border-green-500/30 rounded-xl hover:border-green-500/50 transition-all group"
        >
          <Trophy className="w-8 h-8 text-green-500 mb-3 group-hover:scale-110 transition-transform" />
          <h3 className="font-medium text-white mb-1">View Stats</h3>
          <p className="text-sm text-shadow-400">Track your progress</p>
        </Link>
      </motion.div>
    </div>
  )
}

