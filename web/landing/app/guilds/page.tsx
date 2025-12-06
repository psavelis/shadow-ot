'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import Link from 'next/link'
import { Search, Users, Globe, Crown, Swords, Shield, Trophy, ChevronRight } from 'lucide-react'

const realmOptions = [
  { id: 'all', label: 'All Realms' },
  { id: 'shadowveil', label: 'Shadowveil' },
  { id: 'aetheria', label: 'Aetheria' },
  { id: 'warbound', label: 'Warbound' },
  { id: 'mythara', label: 'Mythara' },
  { id: 'voidborne', label: 'Voidborne' },
  { id: 'grimhollow', label: 'Grimhollow' },
]

const realmColors: Record<string, string> = {
  shadowveil: 'text-red-400 bg-red-500/20',
  aetheria: 'text-blue-400 bg-blue-500/20',
  warbound: 'text-orange-400 bg-orange-500/20',
  mythara: 'text-green-400 bg-green-500/20',
  voidborne: 'text-purple-400 bg-purple-500/20',
  grimhollow: 'text-slate-400 bg-slate-500/20',
}

const guilds = [
  {
    id: '1',
    name: 'Dark Legion',
    realm: 'shadowveil',
    members: 156,
    level: 45,
    leader: 'ShadowMaster',
    description: 'The oldest and most powerful guild in Shadowveil. We dominate PvP and control the best hunting grounds.',
    wars: 3,
    founded: '2022-03-15',
  },
  {
    id: '2',
    name: 'Phoenix Rising',
    realm: 'aetheria',
    members: 234,
    level: 52,
    leader: 'PhoenixLord',
    description: 'A guild focused on teamwork and helping new players reach their full potential.',
    wars: 1,
    founded: '2021-11-20',
  },
  {
    id: '3',
    name: 'Blood Ravens',
    realm: 'warbound',
    members: 89,
    level: 38,
    leader: 'RavenKing',
    description: 'Hardcore PvP guild. Apply only if you\'re ready for constant war.',
    wars: 7,
    founded: '2023-02-08',
  },
  {
    id: '4',
    name: 'Mystic Order',
    realm: 'mythara',
    members: 178,
    level: 41,
    leader: 'MysticSage',
    description: 'Dedicated to exploring all quests and unlocking every achievement in the game.',
    wars: 0,
    founded: '2022-06-30',
  },
  {
    id: '5',
    name: 'Void Walkers',
    realm: 'voidborne',
    members: 67,
    level: 29,
    leader: 'VoidPriest',
    description: 'Seasonal realm guild - competing for the top spot this season!',
    wars: 2,
    founded: '2024-09-01',
  },
]

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

export default function GuildsPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [selectedRealm, setSelectedRealm] = useState('all')
  const [sortBy, setSortBy] = useState<'level' | 'members' | 'wars'>('level')

  const filteredGuilds = guilds
    .filter(guild => {
      const matchesSearch = guild.name.toLowerCase().includes(searchQuery.toLowerCase())
      const matchesRealm = selectedRealm === 'all' || guild.realm === selectedRealm
      return matchesSearch && matchesRealm
    })
    .sort((a, b) => b[sortBy] - a[sortBy])

  return (
    <div className="min-h-screen bg-shadow-950 py-12">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <h1 className="font-display text-4xl md:text-5xl font-bold mb-4">Guilds</h1>
          <p className="text-shadow-400 text-lg max-w-2xl mx-auto">
            Find and join guilds across all realms
          </p>
        </motion.div>

        {/* Search & Filters */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="card mb-8"
        >
          <div className="flex flex-col md:flex-row gap-4">
            <div className="relative flex-1">
              <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search guilds..."
                className="w-full pl-12 pr-4 py-3 bg-shadow-800 border border-shadow-600 rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50"
              />
            </div>
            <select
              value={selectedRealm}
              onChange={(e) => setSelectedRealm(e.target.value)}
              className="px-4 py-3 bg-shadow-800 border border-shadow-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-accent-500/50"
            >
              {realmOptions.map((realm) => (
                <option key={realm.id} value={realm.id}>{realm.label}</option>
              ))}
            </select>
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as 'level' | 'members' | 'wars')}
              className="px-4 py-3 bg-shadow-800 border border-shadow-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-accent-500/50"
            >
              <option value="level">Sort by Level</option>
              <option value="members">Sort by Members</option>
              <option value="wars">Sort by Wars</option>
            </select>
          </div>
        </motion.div>

        {/* Stats */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.15 }}
          className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8"
        >
          <div className="card text-center">
            <Shield className="w-8 h-8 text-accent-500 mx-auto mb-2" />
            <p className="text-2xl font-bold text-white">{guilds.length}</p>
            <p className="text-shadow-400 text-sm">Total Guilds</p>
          </div>
          <div className="card text-center">
            <Users className="w-8 h-8 text-accent-500 mx-auto mb-2" />
            <p className="text-2xl font-bold text-white">{guilds.reduce((a, b) => a + b.members, 0)}</p>
            <p className="text-shadow-400 text-sm">Total Members</p>
          </div>
          <div className="card text-center">
            <Swords className="w-8 h-8 text-accent-500 mx-auto mb-2" />
            <p className="text-2xl font-bold text-white">{guilds.reduce((a, b) => a + b.wars, 0)}</p>
            <p className="text-shadow-400 text-sm">Active Wars</p>
          </div>
          <div className="card text-center">
            <Trophy className="w-8 h-8 text-yellow-500 mx-auto mb-2" />
            <p className="text-2xl font-bold text-white">{Math.max(...guilds.map(g => g.level))}</p>
            <p className="text-shadow-400 text-sm">Highest Level</p>
          </div>
        </motion.div>

        {/* Guild List */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="space-y-4"
        >
          {filteredGuilds.map((guild, index) => (
            <Link key={guild.id} href={`/guilds/${guild.id}`}>
              <div className="card hover:border-accent-500/50 transition-all group">
                <div className="flex items-center gap-6">
                  <div className="hidden sm:flex w-16 h-16 rounded-xl bg-shadow-700 items-center justify-center flex-shrink-0">
                    <Shield className="w-8 h-8 text-accent-500" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-3 mb-1">
                      <h3 className="text-lg font-semibold text-white group-hover:text-accent-400 transition-colors">
                        {guild.name}
                      </h3>
                      <span className={`px-2 py-0.5 rounded text-xs font-medium capitalize ${realmColors[guild.realm]}`}>
                        {guild.realm}
                      </span>
                      {guild.wars > 0 && (
                        <span className="px-2 py-0.5 rounded text-xs font-medium bg-red-500/20 text-red-400 flex items-center gap-1">
                          <Swords className="w-3 h-3" />
                          {guild.wars} wars
                        </span>
                      )}
                    </div>
                    <p className="text-shadow-400 text-sm line-clamp-1 mb-2">{guild.description}</p>
                    <div className="flex flex-wrap items-center gap-4 text-sm text-shadow-500">
                      <span className="flex items-center gap-1">
                        <Crown className="w-4 h-4" />
                        {guild.leader}
                      </span>
                      <span className="flex items-center gap-1">
                        <Users className="w-4 h-4" />
                        {guild.members} members
                      </span>
                      <span className="flex items-center gap-1">
                        <Trophy className="w-4 h-4" />
                        Level {guild.level}
                      </span>
                      <span className="flex items-center gap-1">
                        <Globe className="w-4 h-4" />
                        Founded {formatDate(guild.founded)}
                      </span>
                    </div>
                  </div>
                  <ChevronRight className="w-5 h-5 text-shadow-600 group-hover:text-accent-500 transition-colors flex-shrink-0" />
                </div>
              </div>
            </Link>
          ))}

          {filteredGuilds.length === 0 && (
            <div className="text-center py-16">
              <Shield className="w-16 h-16 text-shadow-700 mx-auto mb-4" />
              <h3 className="text-xl font-semibold text-white mb-2">No guilds found</h3>
              <p className="text-shadow-500">Try adjusting your search or filters</p>
            </div>
          )}
        </motion.div>

        {/* Pagination */}
        {filteredGuilds.length > 0 && (
          <div className="flex items-center justify-center gap-2 pt-8">
            <button className="px-4 py-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">
              Previous
            </button>
            <button className="px-4 py-2 rounded-lg bg-accent-500 text-white">1</button>
            <button className="px-4 py-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">2</button>
            <button className="px-4 py-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">
              Next
            </button>
          </div>
        )}
      </div>
    </div>
  )
}

