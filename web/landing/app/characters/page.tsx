'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { 
  Search, Shield, Target, Wand2, Leaf, User, Globe, Trophy, Skull, Clock, 
  Loader2, AlertCircle
} from 'lucide-react'
import { useCharacterByName, useCharacterDeaths, getOutfitSprite } from '@shadow-ot/shared'
import type { Character, Vocation, RealmId } from '@shadow-ot/shared'

const realmOptions: { id: RealmId | 'all'; label: string }[] = [
  { id: 'all' as any, label: 'All Realms' },
  { id: 'shadowveil', label: 'Shadowveil' },
  { id: 'aetheria', label: 'Aetheria' },
  { id: 'warbound', label: 'Warbound' },
  { id: 'mythara', label: 'Mythara' },
  { id: 'voidborne', label: 'Voidborne' },
  { id: 'grimhollow', label: 'Grimhollow' },
]

const vocationIcons: Record<string, typeof Shield> = {
  'Knight': Shield,
  'Elite Knight': Shield,
  'Paladin': Target,
  'Royal Paladin': Target,
  'Sorcerer': Wand2,
  'Master Sorcerer': Wand2,
  'Druid': Leaf,
  'Elder Druid': Leaf,
  'None': User,
}

const vocationColors: Record<string, string> = {
  'Knight': 'text-red-400',
  'Elite Knight': 'text-red-500',
  'Paladin': 'text-yellow-400',
  'Royal Paladin': 'text-yellow-500',
  'Sorcerer': 'text-blue-400',
  'Master Sorcerer': 'text-blue-500',
  'Druid': 'text-green-400',
  'Elder Druid': 'text-green-500',
  'None': 'text-gray-400',
}

const realmColors: Record<string, string> = {
  shadowveil: 'text-red-400',
  aetheria: 'text-blue-400',
  warbound: 'text-orange-400',
  mythara: 'text-green-400',
  voidborne: 'text-purple-400',
  grimhollow: 'text-slate-400',
}

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function formatNumber(num: number) {
  return num.toLocaleString()
}

export default function CharacterLookupPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [searchName, setSearchName] = useState('')

  // Fetch character data from API
  const { 
    data: character, 
    isLoading, 
    error, 
    refetch 
  } = useCharacterByName(searchName)

  // Fetch deaths separately if character exists
  const { data: deaths } = useCharacterDeaths(character?.id || '')

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault()
    if (!searchQuery.trim()) return
    setSearchName(searchQuery.trim())
  }

  const VocationIcon = character ? vocationIcons[character.vocation] || User : User

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 py-20">
      <div className="max-w-5xl mx-auto px-4">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-purple-500/10 border border-purple-500/30 rounded-full text-purple-400 text-sm mb-6">
            <User className="w-4 h-4" />
            Character Lookup
          </div>
          <h1 className="text-4xl md:text-5xl font-bold text-white mb-4">
            Find <span className="text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-500">Characters</span>
          </h1>
          <p className="text-slate-400 text-lg max-w-2xl mx-auto">
            Search for any character across all realms
          </p>
        </motion.div>

        {/* Search Form */}
        <motion.form
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          onSubmit={handleSearch}
          className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6 mb-8"
        >
          <div className="flex flex-col sm:flex-row gap-4">
            <div className="relative flex-1">
              <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-500" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Enter character name..."
                className="w-full pl-12 pr-4 py-3 bg-slate-900/50 border border-slate-700 rounded-lg text-white placeholder:text-slate-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50"
              />
            </div>
            <button
              type="submit"
              disabled={isLoading || !searchQuery.trim()}
              className="px-8 py-3 bg-purple-500 text-white rounded-lg font-medium hover:bg-purple-600 transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              {isLoading ? (
                <>
                  <Loader2 className="w-5 h-5 animate-spin" />
                  Searching...
                </>
              ) : (
                'Search'
              )}
            </button>
          </div>
        </motion.form>

        {/* Error State */}
        {error && searchName && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="bg-red-500/10 border border-red-500/30 rounded-xl p-6 text-center"
          >
            <AlertCircle className="w-12 h-12 text-red-400 mx-auto mb-4" />
            <h3 className="text-white font-bold text-lg mb-2">Character Not Found</h3>
            <p className="text-slate-400">
              No character named "{searchName}" was found. Please check the spelling and try again.
            </p>
          </motion.div>
        )}

        {/* Character Profile */}
        {character && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-6"
          >
            {/* Main Info */}
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <div className="flex flex-col md:flex-row md:items-start gap-6">
                <div className="flex-shrink-0">
                  <div className="w-24 h-24 rounded-xl bg-slate-700/50 border border-slate-600/50 flex items-center justify-center overflow-hidden">
                    {character.outfit ? (
                      <img 
                        src={getOutfitSprite(character.outfit)} 
                        alt={character.name}
                        className="w-16 h-16 object-contain"
                        onError={(e) => {
                          (e.target as HTMLImageElement).style.display = 'none'
                          ;(e.target as HTMLImageElement).nextElementSibling?.classList.remove('hidden')
                        }}
                      />
                    ) : null}
                    <VocationIcon className={`w-12 h-12 ${vocationColors[character.vocation]} ${character.outfit ? 'hidden' : ''}`} />
                  </div>
                </div>
                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-2">
                    <h2 className="text-2xl font-bold text-white">{character.name}</h2>
                    {character.online && (
                      <span className="flex items-center gap-1 text-green-400 text-sm">
                        <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
                        Online
                      </span>
                    )}
                  </div>
                  <p className="text-slate-400 mb-4">
                    Level {character.level} - <span className={vocationColors[character.vocation]}>{character.vocation}</span>
                  </p>
                  <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4 text-sm">
                    <div className="flex items-center gap-2">
                      <Globe className={`w-4 h-4 ${realmColors[character.realm]}`} />
                      <span className="text-slate-400">Realm:</span>
                      <span className={`font-medium capitalize ${realmColors[character.realm]}`}>{character.realm}</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <User className="w-4 h-4 text-slate-500" />
                      <span className="text-slate-400">Sex:</span>
                      <span className="text-white capitalize">{character.sex}</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Globe className="w-4 h-4 text-slate-500" />
                      <span className="text-slate-400">Residence:</span>
                      <span className="text-white">{character.residence}</span>
                    </div>
                    {character.guild && (
                      <div className="flex items-center gap-2">
                        <Shield className="w-4 h-4 text-slate-500" />
                        <span className="text-slate-400">Guild:</span>
                        <span className="text-white">{character.guild.guildName}</span>
                        <span className="text-slate-500">({character.guild.rank})</span>
                      </div>
                    )}
                    <div className="flex items-center gap-2">
                      <Trophy className="w-4 h-4 text-yellow-500" />
                      <span className="text-slate-400">Achievements:</span>
                      <span className="text-white">{formatNumber(character.achievementPoints || 0)} pts</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Clock className="w-4 h-4 text-slate-500" />
                      <span className="text-slate-400">Last Login:</span>
                      <span className="text-white">{character.lastLogin ? formatDate(character.lastLogin) : 'Unknown'}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Skills */}
            {character.skills && (
              <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
                <h3 className="text-lg font-bold text-white mb-4">Skills</h3>
                <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
                  {Object.entries(character.skills).map(([skill, data]) => (
                    <div key={skill} className="bg-slate-900/50 rounded-lg p-3">
                      <div className="flex justify-between items-center mb-2">
                        <span className="text-slate-400 text-sm capitalize">{skill.replace(/([A-Z])/g, ' $1').trim()}</span>
                        <span className="text-white font-bold">{data.level}</span>
                      </div>
                      <div className="h-2 bg-slate-700 rounded-full overflow-hidden">
                        <div 
                          className="h-full bg-gradient-to-r from-purple-500 to-pink-500 rounded-full transition-all"
                          style={{ width: `${data.percent}%` }}
                        />
                      </div>
                      <p className="text-slate-500 text-xs mt-1 text-right">{data.percent}%</p>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Deaths */}
            {deaths && deaths.length > 0 && (
              <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
                <h3 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
                  <Skull className="w-5 h-5 text-red-400" />
                  Recent Deaths
                </h3>
                <div className="space-y-3">
                  {deaths.slice(0, 5).map((death, idx) => (
                    <div key={idx} className="flex items-center justify-between p-3 bg-slate-900/50 rounded-lg">
                      <div>
                        <p className="text-white">
                          Killed by <span className="text-red-400 font-medium">{death.killerName}</span>
                        </p>
                        <p className="text-slate-500 text-sm">at level {death.levelAtDeath}</p>
                      </div>
                      <div className="text-right">
                        <p className="text-slate-400 text-sm">{formatDate(death.timestamp)}</p>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Experience */}
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6">
              <h3 className="text-lg font-bold text-white mb-4">Experience</h3>
              <div className="grid sm:grid-cols-2 gap-6">
                <div>
                  <p className="text-slate-400 text-sm mb-1">Total Experience</p>
                  <p className="text-2xl font-bold text-amber-400">{formatNumber(character.experience)}</p>
                </div>
                <div>
                  <p className="text-slate-400 text-sm mb-1">Experience to Next Level</p>
                  <div className="h-4 bg-slate-700 rounded-full overflow-hidden mt-2">
                    <div 
                      className="h-full bg-gradient-to-r from-amber-500 to-yellow-500 rounded-full"
                      style={{ width: `${((character.experience % 1000000) / 1000000) * 100}%` }}
                    />
                  </div>
                </div>
              </div>
            </div>
          </motion.div>
        )}

        {/* No Search Yet */}
        {!character && !error && !isLoading && !searchName && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="text-center py-16"
          >
            <User className="w-16 h-16 text-slate-600 mx-auto mb-4" />
            <h3 className="text-white font-bold text-xl mb-2">Search for a Character</h3>
            <p className="text-slate-400">
              Enter a character name above to view their profile, skills, and recent deaths.
            </p>
          </motion.div>
        )}
      </div>
    </div>
  )
}
