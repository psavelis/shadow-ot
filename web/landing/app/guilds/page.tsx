'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import Link from 'next/link'
import { 
  Search, Users, Globe, Crown, Swords, Shield, Trophy, ChevronRight,
  Loader2, AlertCircle
} from 'lucide-react'
import { useGuilds } from '@shadow-ot/shared'
import type { RealmId } from '@shadow-ot/shared'

const realmOptions: { id: RealmId | 'all'; label: string }[] = [
  { id: 'all' as any, label: 'All Realms' },
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

export default function GuildsPage() {
  const [selectedRealm, setSelectedRealm] = useState<RealmId | 'all'>('all' as any)
  const [searchQuery, setSearchQuery] = useState('')
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 20

  // Fetch guilds from API
  const { data, isLoading, error } = useGuilds(
    selectedRealm === 'all' ? undefined : selectedRealm,
    currentPage,
    pageSize
  )

  // Filter by search query client-side
  const filteredGuilds = data?.data?.filter(guild => 
    !searchQuery || guild.name.toLowerCase().includes(searchQuery.toLowerCase())
  ) || []

  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 py-20">
      <div className="container mx-auto px-4">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-purple-500/10 border border-purple-500/30 rounded-full text-purple-400 text-sm mb-6">
            <Shield className="w-4 h-4" />
            Community
          </div>
          <h1 className="text-4xl md:text-5xl font-bold text-white mb-4">
            <span className="text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-500">Guilds</span>
          </h1>
          <p className="text-slate-400 text-lg max-w-2xl mx-auto">
            Find and join powerful guilds across all realms
          </p>
        </motion.div>

        {/* Filters */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="flex flex-wrap items-center gap-4 bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 mb-8"
        >
          {/* Search */}
          <div className="relative flex-1 min-w-[250px]">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
            <input
              type="text"
              placeholder="Search guilds..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-slate-900/50 border border-slate-700 rounded-lg pl-10 pr-4 py-2 text-white text-sm placeholder-slate-500 focus:outline-none focus:border-purple-500/50"
            />
          </div>

          {/* Realm Filter */}
          <select
            value={selectedRealm}
            onChange={(e) => {
              setSelectedRealm(e.target.value as RealmId | 'all')
              setCurrentPage(1)
            }}
            className="appearance-none bg-slate-900/50 border border-slate-700 rounded-lg px-4 py-2 pr-8 text-white text-sm focus:outline-none focus:border-purple-500/50"
          >
            {realmOptions.map(realm => (
              <option key={realm.id} value={realm.id}>{realm.label}</option>
            ))}
          </select>

          {data && (
            <span className="text-slate-500 text-sm">
              {data.total} guilds found
            </span>
          )}
        </motion.div>

        {/* Guilds Grid */}
        {isLoading ? (
          <div className="flex items-center justify-center py-20">
            <Loader2 className="w-12 h-12 animate-spin text-purple-400" />
          </div>
        ) : error ? (
          <div className="flex flex-col items-center justify-center py-20 text-red-400">
            <AlertCircle className="w-12 h-12 mb-4" />
            <p>Failed to load guilds. Please try again.</p>
          </div>
        ) : filteredGuilds.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-20 text-slate-400">
            <Shield className="w-12 h-12 mb-4 opacity-50" />
            <p>No guilds found</p>
          </div>
        ) : (
          <>
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              {filteredGuilds.map((guild, idx) => (
                <motion.div
                  key={guild.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: Math.min(idx * 0.05, 0.3) }}
                  className="group bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden hover:border-purple-500/50 transition-all hover:shadow-lg hover:shadow-purple-500/10"
                >
                  {/* Guild Header */}
                  <div className="p-6 border-b border-slate-700/50">
                    <div className="flex items-start justify-between mb-4">
                      <div className="flex items-center gap-3">
                        <div className="w-12 h-12 bg-gradient-to-br from-purple-500 to-pink-500 rounded-xl flex items-center justify-center">
                          {guild.logo ? (
                            <img src={guild.logo} alt={guild.name} className="w-10 h-10 object-contain" />
                          ) : (
                            <Shield className="w-6 h-6 text-white" />
                          )}
                        </div>
                        <div>
                          <h3 className="text-white font-bold text-lg group-hover:text-purple-400 transition">
                            {guild.name}
                          </h3>
                          <span className={`text-xs px-2 py-0.5 rounded ${realmColors[guild.realm] || 'text-slate-400 bg-slate-500/20'}`}>
                            {guild.realm}
                          </span>
                        </div>
                      </div>
                      <div className="text-right">
                        <p className="text-purple-400 font-bold text-lg">Lvl {guild.level}</p>
                      </div>
                    </div>

                    <p className="text-slate-400 text-sm line-clamp-2">
                      {guild.description || 'No description available'}
                    </p>
                  </div>

                  {/* Guild Stats */}
                  <div className="p-4 grid grid-cols-3 gap-4">
                    <div className="text-center">
                      <div className="flex items-center justify-center gap-1 text-slate-400 mb-1">
                        <Users className="w-4 h-4" />
                      </div>
                      <p className="text-white font-bold">{guild.members.length}</p>
                      <p className="text-slate-500 text-xs">Members</p>
                    </div>
                    <div className="text-center">
                      <div className="flex items-center justify-center gap-1 text-slate-400 mb-1">
                        <Swords className="w-4 h-4" />
                      </div>
                      <p className="text-white font-bold">{guild.wars.filter(w => w.status === 'active').length}</p>
                      <p className="text-slate-500 text-xs">Active Wars</p>
                    </div>
                    <div className="text-center">
                      <div className="flex items-center justify-center gap-1 text-slate-400 mb-1">
                        <Crown className="w-4 h-4" />
                      </div>
                      <p className="text-white font-bold truncate text-sm">
                        {guild.members.find(m => m.rank === 'Leader')?.characterName || 'Unknown'}
                      </p>
                      <p className="text-slate-500 text-xs">Leader</p>
                    </div>
                  </div>

                  {/* View Guild */}
                  <div className="p-4 border-t border-slate-700/50">
                    <Link
                      href={`/guilds/${guild.id}`}
                      className="flex items-center justify-center gap-2 w-full py-2 bg-purple-500/20 text-purple-400 rounded-lg hover:bg-purple-500/30 transition"
                    >
                      View Guild
                      <ChevronRight className="w-4 h-4" />
                    </Link>
                  </div>
                </motion.div>
              ))}
            </div>

            {/* Pagination */}
            {data && data.totalPages > 1 && (
              <div className="mt-8 flex items-center justify-center gap-4">
                <button
                  onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
                  disabled={currentPage === 1}
                  className="px-4 py-2 bg-slate-800/50 border border-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-700/50 transition disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Previous
                </button>
                <span className="text-slate-400">
                  Page {currentPage} of {data.totalPages}
                </span>
                <button
                  onClick={() => setCurrentPage(p => Math.min(data.totalPages, p + 1))}
                  disabled={currentPage === data.totalPages}
                  className="px-4 py-2 bg-slate-800/50 border border-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-700/50 transition disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Next
                </button>
              </div>
            )}
          </>
        )}
      </div>
    </main>
  )
}
