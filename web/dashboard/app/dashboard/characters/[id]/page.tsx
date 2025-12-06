'use client'

import { useState } from 'react'
import { useParams, useRouter } from 'next/navigation'
import Link from 'next/link'
import { motion } from 'framer-motion'
import {
  ArrowLeft, Shield, Sword, Heart, Zap, Star, Trophy, Skull,
  Clock, MapPin, Users, Crown, ChevronRight, AlertTriangle,
  ArrowRightLeft, Trash2, Edit3, History, Award, Target
} from 'lucide-react'
import * as Dialog from '@radix-ui/react-dialog'
import * as Tabs from '@radix-ui/react-tabs'
import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, AreaChart, Area } from 'recharts'

// Mock character data
const character = {
  id: '1',
  name: 'ShadowKnight',
  level: 342,
  vocation: 'Elite Knight',
  sex: 'Male',
  realm: 'Shadowlands',
  realmColor: '#f59e0b',
  guild: { name: 'Dark Legion', rank: 'Vice Leader' },
  house: { name: 'Thais Manor', town: 'Thais', size: 'Large' },
  premium: true,
  online: true,
  lastLogin: '5 minutes ago',
  created: 'March 15, 2023',
  playtime: '1,245 hours',
  outfit: { looktype: 128, head: 78, body: 68, legs: 58, feet: 76 },
  stats: {
    health: 4520,
    maxHealth: 4520,
    mana: 1850,
    maxMana: 1850,
    capacity: 3420,
    experience: 1847293847,
    magicLevel: 12,
    fist: 45,
    club: 110,
    sword: 115,
    axe: 108,
    distance: 32,
    shielding: 102,
    fishing: 28,
  },
  achievements: {
    points: 1250,
    count: 85,
    recent: [
      { name: 'Dragon Slayer', icon: '🐉', date: '2024-11-28' },
      { name: 'Master of War', icon: '⚔️', date: '2024-11-15' },
      { name: 'Treasure Hunter', icon: '💎', date: '2024-11-01' },
    ]
  },
  deaths: [
    { killer: 'Demon', level: 340, date: '2024-12-01 15:32', assisted: ['Dragon Lord', 'Fire Elemental'] },
    { killer: 'PlayerKiller (342)', level: 338, date: '2024-11-28 22:15', assisted: [] },
    { killer: 'Orshabaal', level: 335, date: '2024-11-20 19:45', assisted: [] },
  ],
  kills: [
    { victim: 'EnemyPlayer', level: 298, date: '2024-12-02 14:20' },
    { victim: 'RivalKnight', level: 312, date: '2024-11-30 18:45' },
  ],
  balance: { gold: 15847293, bank: 284729384 },
}

const expHistory = [
  { date: 'Nov 27', exp: 1820000000, level: 338 },
  { date: 'Nov 28', exp: 1828000000, level: 339 },
  { date: 'Nov 29', exp: 1835000000, level: 340 },
  { date: 'Nov 30', exp: 1840000000, level: 341 },
  { date: 'Dec 1', exp: 1843000000, level: 341 },
  { date: 'Dec 2', exp: 1847000000, level: 342 },
]

const realms = [
  { id: 'shadowlands', name: 'Shadowlands', type: 'PvP-E', available: true },
  { id: 'mythara', name: 'Mythara', type: 'Classic', available: true },
  { id: 'aetheria', name: 'Aetheria', type: 'Mythic', available: false },
  { id: 'voidborne', name: 'Voidborne', type: 'Seasonal', available: true },
]

export default function CharacterDetailPage() {
  const params = useParams()
  const router = useRouter()
  const [showTransferDialog, setShowTransferDialog] = useState(false)
  const [showDeleteDialog, setShowDeleteDialog] = useState(false)
  const [selectedRealm, setSelectedRealm] = useState('')
  const [deleteConfirm, setDeleteConfirm] = useState('')

  const formatNumber = (n: number) => n.toLocaleString()
  const formatGold = (n: number) => {
    if (n >= 1000000000) return `${(n / 1000000000).toFixed(2)}kkk`
    if (n >= 1000000) return `${(n / 1000000).toFixed(2)}kk`
    if (n >= 1000) return `${(n / 1000).toFixed(1)}k`
    return n.toString()
  }

  return (
    <div className="space-y-6">
      {/* Back Button */}
      <Link
        href="/dashboard/characters"
        className="inline-flex items-center gap-2 text-slate-400 hover:text-white transition"
      >
        <ArrowLeft className="w-4 h-4" />
        Back to Characters
      </Link>

      {/* Character Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="bg-slate-800/50 border border-slate-700/50 rounded-2xl overflow-hidden"
      >
        <div className="h-24 bg-gradient-to-r from-amber-500/20 via-orange-500/20 to-red-500/20" />
        <div className="px-6 pb-6">
          <div className="flex items-end gap-6 -mt-10">
            {/* Avatar */}
            <div className="w-20 h-20 bg-gradient-to-br from-amber-500 to-orange-600 rounded-xl flex items-center justify-center border-4 border-slate-800 shadow-xl">
              <Shield className="w-10 h-10 text-white" />
            </div>

            {/* Info */}
            <div className="flex-1 pb-1">
              <div className="flex items-center gap-3">
                <h1 className="text-2xl font-bold text-white">{character.name}</h1>
                {character.online && (
                  <span className="flex items-center gap-1 px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs rounded-full">
                    <span className="w-1.5 h-1.5 bg-emerald-400 rounded-full animate-pulse" />
                    Online
                  </span>
                )}
                {character.premium && (
                  <Crown className="w-5 h-5 text-amber-400" />
                )}
              </div>
              <p className="text-slate-400">
                Level {character.level} - {character.vocation} • {character.sex}
              </p>
            </div>

            {/* Actions */}
            <div className="flex items-center gap-2 pb-1">
              <button
                onClick={() => setShowTransferDialog(true)}
                className="flex items-center gap-2 px-3 py-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-700 transition text-sm"
              >
                <ArrowRightLeft className="w-4 h-4" />
                Transfer
              </button>
              <button className="flex items-center gap-2 px-3 py-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-700 transition text-sm">
                <Edit3 className="w-4 h-4" />
                Edit
              </button>
              <button
                onClick={() => setShowDeleteDialog(true)}
                className="flex items-center gap-2 px-3 py-2 bg-red-500/20 text-red-400 rounded-lg hover:bg-red-500/30 transition text-sm"
              >
                <Trash2 className="w-4 h-4" />
              </button>
            </div>
          </div>

          {/* Quick Stats */}
          <div className="grid grid-cols-6 gap-4 mt-6">
            <div className="text-center">
              <p className="text-2xl font-bold text-white">{character.level}</p>
              <p className="text-xs text-slate-500">Level</p>
            </div>
            <div className="text-center">
              <p className="text-2xl font-bold text-white">{formatNumber(character.stats.experience)}</p>
              <p className="text-xs text-slate-500">Experience</p>
            </div>
            <div className="text-center">
              <p className="text-2xl font-bold text-white">{character.stats.magicLevel}</p>
              <p className="text-xs text-slate-500">Magic Level</p>
            </div>
            <div className="text-center">
              <p className="text-2xl font-bold text-white">{character.achievements.points}</p>
              <p className="text-xs text-slate-500">Achievement Points</p>
            </div>
            <div className="text-center">
              <p className="text-2xl font-bold text-white">{formatGold(character.balance.gold + character.balance.bank)}</p>
              <p className="text-xs text-slate-500">Total Gold</p>
            </div>
            <div className="text-center">
              <p className="text-2xl font-bold text-white">{character.playtime}</p>
              <p className="text-xs text-slate-500">Playtime</p>
            </div>
          </div>
        </div>
      </motion.div>

      {/* Content Tabs */}
      <Tabs.Root defaultValue="stats">
        <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1 mb-6">
          {[
            { id: 'stats', label: 'Statistics', icon: Target },
            { id: 'skills', label: 'Skills', icon: Sword },
            { id: 'history', label: 'Deaths & Kills', icon: Skull },
            { id: 'achievements', label: 'Achievements', icon: Trophy },
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

        {/* Stats Tab */}
        <Tabs.Content value="stats">
          <div className="grid lg:grid-cols-3 gap-6">
            {/* Combat Stats */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
            >
              <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                <Heart className="w-4 h-4 text-red-400" />
                Combat Stats
              </h3>
              <div className="space-y-4">
                <div>
                  <div className="flex justify-between text-sm mb-1">
                    <span className="text-slate-400">Health</span>
                    <span className="text-white">{character.stats.health} / {character.stats.maxHealth}</span>
                  </div>
                  <div className="h-2 bg-slate-700 rounded-full overflow-hidden">
                    <div className="h-full bg-red-500 rounded-full" style={{ width: '100%' }} />
                  </div>
                </div>
                <div>
                  <div className="flex justify-between text-sm mb-1">
                    <span className="text-slate-400">Mana</span>
                    <span className="text-white">{character.stats.mana} / {character.stats.maxMana}</span>
                  </div>
                  <div className="h-2 bg-slate-700 rounded-full overflow-hidden">
                    <div className="h-full bg-blue-500 rounded-full" style={{ width: '100%' }} />
                  </div>
                </div>
                <div className="flex justify-between text-sm pt-2 border-t border-slate-700">
                  <span className="text-slate-400">Capacity</span>
                  <span className="text-white">{character.stats.capacity} oz</span>
                </div>
              </div>
            </motion.div>

            {/* Experience Chart */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.1 }}
              className="lg:col-span-2 bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
            >
              <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                <Zap className="w-4 h-4 text-amber-400" />
                Experience Progress (7 days)
              </h3>
              <div className="h-48">
                <ResponsiveContainer width="100%" height="100%">
                  <AreaChart data={expHistory}>
                    <defs>
                      <linearGradient id="expGradient" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="5%" stopColor="#f59e0b" stopOpacity={0.3}/>
                        <stop offset="95%" stopColor="#f59e0b" stopOpacity={0}/>
                      </linearGradient>
                    </defs>
                    <XAxis dataKey="date" stroke="#64748b" fontSize={12} />
                    <YAxis stroke="#64748b" fontSize={12} tickFormatter={(v) => `${(v / 1000000000).toFixed(1)}B`} />
                    <Tooltip
                      contentStyle={{ backgroundColor: '#1e293b', border: '1px solid #334155', borderRadius: '8px' }}
                      labelStyle={{ color: '#f8fafc' }}
                    />
                    <Area type="monotone" dataKey="exp" stroke="#f59e0b" fill="url(#expGradient)" strokeWidth={2} />
                  </AreaChart>
                </ResponsiveContainer>
              </div>
            </motion.div>

            {/* Info Cards */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.2 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
            >
              <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                <MapPin className="w-4 h-4 text-emerald-400" />
                Location
              </h3>
              <div className="space-y-3 text-sm">
                <div className="flex justify-between">
                  <span className="text-slate-400">Realm</span>
                  <span className="text-amber-400 font-medium">{character.realm}</span>
                </div>
                {character.house && (
                  <div className="flex justify-between">
                    <span className="text-slate-400">House</span>
                    <span className="text-white">{character.house.name}</span>
                  </div>
                )}
                <div className="flex justify-between">
                  <span className="text-slate-400">Last Login</span>
                  <span className="text-white">{character.lastLogin}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-slate-400">Created</span>
                  <span className="text-white">{character.created}</span>
                </div>
              </div>
            </motion.div>

            {/* Guild */}
            {character.guild && (
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.3 }}
                className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
              >
                <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                  <Users className="w-4 h-4 text-purple-400" />
                  Guild
                </h3>
                <Link
                  href={`/guilds/${character.guild.name}`}
                  className="flex items-center justify-between p-3 bg-slate-900/50 rounded-lg hover:bg-slate-800/50 transition group"
                >
                  <div>
                    <p className="text-white font-medium group-hover:text-amber-400 transition">{character.guild.name}</p>
                    <p className="text-slate-500 text-sm">{character.guild.rank}</p>
                  </div>
                  <ChevronRight className="w-5 h-5 text-slate-600 group-hover:text-amber-400 transition" />
                </Link>
              </motion.div>
            )}

            {/* Balance */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.4 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
            >
              <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                <Star className="w-4 h-4 text-yellow-400" />
                Balance
              </h3>
              <div className="space-y-3 text-sm">
                <div className="flex justify-between">
                  <span className="text-slate-400">Gold (Inventory)</span>
                  <span className="text-yellow-400 font-medium">{formatNumber(character.balance.gold)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-slate-400">Bank Balance</span>
                  <span className="text-yellow-400 font-medium">{formatNumber(character.balance.bank)}</span>
                </div>
                <div className="flex justify-between pt-2 border-t border-slate-700">
                  <span className="text-slate-400">Total</span>
                  <span className="text-white font-bold">{formatGold(character.balance.gold + character.balance.bank)}</span>
                </div>
              </div>
            </motion.div>
          </div>
        </Tabs.Content>

        {/* Skills Tab */}
        <Tabs.Content value="skills">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
          >
            <h3 className="text-white font-medium mb-6">Combat Skills</h3>
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              {[
                { name: 'Sword Fighting', value: character.stats.sword, max: 130, color: 'amber' },
                { name: 'Club Fighting', value: character.stats.club, max: 130, color: 'orange' },
                { name: 'Axe Fighting', value: character.stats.axe, max: 130, color: 'red' },
                { name: 'Distance Fighting', value: character.stats.distance, max: 130, color: 'green' },
                { name: 'Shielding', value: character.stats.shielding, max: 130, color: 'blue' },
                { name: 'Fist Fighting', value: character.stats.fist, max: 130, color: 'purple' },
                { name: 'Magic Level', value: character.stats.magicLevel, max: 130, color: 'cyan' },
                { name: 'Fishing', value: character.stats.fishing, max: 130, color: 'slate' },
              ].map(skill => (
                <div key={skill.name} className="bg-slate-900/50 rounded-lg p-4">
                  <div className="flex justify-between mb-2">
                    <span className="text-slate-300">{skill.name}</span>
                    <span className="text-white font-bold">{skill.value}</span>
                  </div>
                  <div className="h-2 bg-slate-700 rounded-full overflow-hidden">
                    <div
                      className={`h-full rounded-full bg-${skill.color}-500`}
                      style={{ width: `${(skill.value / skill.max) * 100}%`, backgroundColor: skill.color === 'amber' ? '#f59e0b' : skill.color === 'orange' ? '#f97316' : skill.color === 'red' ? '#ef4444' : skill.color === 'green' ? '#22c55e' : skill.color === 'blue' ? '#3b82f6' : skill.color === 'purple' ? '#a855f7' : skill.color === 'cyan' ? '#06b6d4' : '#64748b' }}
                    />
                  </div>
                  <p className="text-xs text-slate-500 mt-1">{((skill.value / skill.max) * 100).toFixed(1)}% to max</p>
                </div>
              ))}
            </div>
          </motion.div>
        </Tabs.Content>

        {/* Deaths & Kills Tab */}
        <Tabs.Content value="history">
          <div className="grid md:grid-cols-2 gap-6">
            {/* Deaths */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
            >
              <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                <Skull className="w-4 h-4 text-red-400" />
                Recent Deaths ({character.deaths.length})
              </h3>
              <div className="space-y-3">
                {character.deaths.map((death, idx) => (
                  <div key={idx} className="bg-slate-900/50 rounded-lg p-3">
                    <div className="flex justify-between items-start mb-1">
                      <p className="text-white">Killed by <span className="text-red-400">{death.killer}</span></p>
                      <span className="text-slate-500 text-xs">{death.date}</span>
                    </div>
                    <p className="text-slate-400 text-sm">At level {death.level}</p>
                    {death.assisted.length > 0 && (
                      <p className="text-slate-500 text-xs mt-1">
                        Assisted by: {death.assisted.join(', ')}
                      </p>
                    )}
                  </div>
                ))}
              </div>
            </motion.div>

            {/* Kills */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.1 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
            >
              <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                <Sword className="w-4 h-4 text-emerald-400" />
                Recent PvP Kills ({character.kills.length})
              </h3>
              <div className="space-y-3">
                {character.kills.map((kill, idx) => (
                  <div key={idx} className="bg-slate-900/50 rounded-lg p-3">
                    <div className="flex justify-between items-start mb-1">
                      <p className="text-white">Killed <span className="text-emerald-400">{kill.victim}</span></p>
                      <span className="text-slate-500 text-xs">{kill.date}</span>
                    </div>
                    <p className="text-slate-400 text-sm">Victim level {kill.level}</p>
                  </div>
                ))}
              </div>
            </motion.div>
          </div>
        </Tabs.Content>

        {/* Achievements Tab */}
        <Tabs.Content value="achievements">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
          >
            <div className="flex items-center justify-between mb-6">
              <h3 className="text-white font-medium flex items-center gap-2">
                <Trophy className="w-4 h-4 text-amber-400" />
                Achievements
              </h3>
              <div className="flex items-center gap-4 text-sm">
                <span className="text-slate-400">{character.achievements.count} unlocked</span>
                <span className="text-amber-400 font-bold">{character.achievements.points} points</span>
              </div>
            </div>
            <div className="grid md:grid-cols-3 gap-4">
              {character.achievements.recent.map((achievement, idx) => (
                <div key={idx} className="bg-slate-900/50 rounded-lg p-4 flex items-center gap-4">
                  <div className="w-12 h-12 bg-amber-500/20 rounded-lg flex items-center justify-center text-2xl">
                    {achievement.icon}
                  </div>
                  <div>
                    <p className="text-white font-medium">{achievement.name}</p>
                    <p className="text-slate-500 text-xs">{achievement.date}</p>
                  </div>
                </div>
              ))}
            </div>
            <Link
              href="/dashboard/achievements"
              className="flex items-center justify-center gap-2 w-full mt-4 py-2 text-amber-400 hover:text-amber-300 text-sm transition"
            >
              View All Achievements
              <ChevronRight className="w-4 h-4" />
            </Link>
          </motion.div>
        </Tabs.Content>
      </Tabs.Root>

      {/* Transfer Dialog */}
      <Dialog.Root open={showTransferDialog} onOpenChange={setShowTransferDialog}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md">
            <Dialog.Title className="text-xl font-bold text-white mb-2">Transfer Character</Dialog.Title>
            <Dialog.Description className="text-slate-400 text-sm mb-6">
              Transfer {character.name} to another realm. There is a 30-day cooldown between transfers.
            </Dialog.Description>

            <div className="space-y-3 mb-6">
              {realms.filter(r => r.id !== 'shadowlands').map(realm => (
                <button
                  key={realm.id}
                  onClick={() => setSelectedRealm(realm.id)}
                  disabled={!realm.available}
                  className={`w-full flex items-center justify-between p-4 rounded-lg border transition ${
                    selectedRealm === realm.id
                      ? 'border-amber-500 bg-amber-500/10'
                      : realm.available
                        ? 'border-slate-700 hover:border-slate-600'
                        : 'border-slate-800 opacity-50 cursor-not-allowed'
                  }`}
                >
                  <div className="text-left">
                    <p className="text-white font-medium">{realm.name}</p>
                    <p className="text-slate-500 text-sm">{realm.type}</p>
                  </div>
                  {!realm.available && <span className="text-red-400 text-xs">Full</span>}
                </button>
              ))}
            </div>

            <div className="bg-amber-500/10 border border-amber-500/30 rounded-lg p-3 mb-6">
              <div className="flex items-start gap-2">
                <AlertTriangle className="w-4 h-4 text-amber-400 flex-shrink-0 mt-0.5" />
                <p className="text-amber-200 text-xs">
                  Transfer will reset your house ownership and guild membership. Gold and items will be preserved.
                </p>
              </div>
            </div>

            <div className="flex gap-3">
              <Dialog.Close className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                Cancel
              </Dialog.Close>
              <button
                disabled={!selectedRealm}
                className="flex-1 px-4 py-2 bg-amber-500 text-white rounded-lg hover:bg-amber-400 transition disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Transfer
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>

      {/* Delete Dialog */}
      <Dialog.Root open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <Dialog.Portal>
          <Dialog.Overlay className="fixed inset-0 bg-black/70 backdrop-blur-sm" />
          <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-slate-800 border border-slate-700 rounded-2xl p-6 w-full max-w-md">
            <Dialog.Title className="text-xl font-bold text-white mb-2">Delete Character</Dialog.Title>
            <Dialog.Description className="text-slate-400 text-sm mb-6">
              This action cannot be undone. The character will be permanently deleted after a 7-day recovery period.
            </Dialog.Description>

            <div className="bg-red-500/10 border border-red-500/30 rounded-lg p-4 mb-6">
              <p className="text-red-200 text-sm mb-2">You will lose:</p>
              <ul className="text-red-300 text-xs space-y-1 ml-4 list-disc">
                <li>All experience and levels</li>
                <li>All items and equipment</li>
                <li>All achievements and progress</li>
                <li>House ownership</li>
                <li>Guild membership</li>
              </ul>
            </div>

            <div className="mb-6">
              <label className="text-slate-300 text-sm mb-2 block">
                Type <span className="text-red-400 font-mono">DELETE</span> to confirm
              </label>
              <input
                type="text"
                value={deleteConfirm}
                onChange={e => setDeleteConfirm(e.target.value)}
                className="w-full bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-red-500/50"
                placeholder="DELETE"
              />
            </div>

            <div className="flex gap-3">
              <Dialog.Close className="flex-1 px-4 py-2 bg-slate-700 text-slate-300 rounded-lg hover:bg-slate-600 transition">
                Cancel
              </Dialog.Close>
              <button
                disabled={deleteConfirm !== 'DELETE'}
                className="flex-1 px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-400 transition disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Delete Character
              </button>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </div>
  )
}

