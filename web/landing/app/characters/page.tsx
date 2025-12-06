'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { Search, Shield, Target, Wand2, Leaf, User, Globe, Trophy, Skull, Clock, Filter } from 'lucide-react'

const realmOptions = [
  { id: 'all', label: 'All Realms' },
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

// Mock character data
const mockCharacter = {
  name: 'Shadow Knight',
  level: 312,
  vocation: 'Elite Knight',
  realm: 'shadowveil',
  online: true,
  sex: 'male',
  residence: 'Thais',
  guild: { name: 'Dark Legion', rank: 'Leader' },
  lastLogin: '2024-12-05T10:30:00Z',
  accountAge: '2 years, 3 months',
  experience: 2847593842,
  achievementPoints: 1250,
  deaths: [
    { killer: 'a demon', level: 310, date: '2024-12-03T14:22:00Z' },
    { killer: 'DragonSlayer (Player)', level: 308, date: '2024-12-01T20:15:00Z' },
    { killer: 'a dragon lord', level: 305, date: '2024-11-28T09:45:00Z' },
  ],
  skills: {
    fist: { level: 32, percent: 45 },
    club: { level: 15, percent: 12 },
    sword: { level: 118, percent: 78 },
    axe: { level: 95, percent: 23 },
    distance: { level: 25, percent: 56 },
    shielding: { level: 112, percent: 34 },
    fishing: { level: 45, percent: 89 },
    magicLevel: { level: 28, percent: 67 },
  },
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
  const [selectedRealm, setSelectedRealm] = useState('all')
  const [character, setCharacter] = useState<typeof mockCharacter | null>(null)
  const [searching, setSearching] = useState(false)

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault()
    if (!searchQuery.trim()) return
    
    setSearching(true)
    // Simulate API call
    setTimeout(() => {
      setCharacter(mockCharacter)
      setSearching(false)
    }, 500)
  }

  const VocationIcon = character ? vocationIcons[character.vocation] : User

  return (
    <div className="min-h-screen bg-shadow-950 py-12">
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <h1 className="font-display text-4xl md:text-5xl font-bold mb-4">Character Lookup</h1>
          <p className="text-shadow-400 text-lg max-w-2xl mx-auto">
            Search for any character across all realms
          </p>
        </motion.div>

        {/* Search Form */}
        <motion.form
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          onSubmit={handleSearch}
          className="card mb-8"
        >
          <div className="flex flex-col sm:flex-row gap-4">
            <div className="relative flex-1">
              <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Enter character name..."
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
            <button
              type="submit"
              disabled={searching}
              className="btn-primary px-8 disabled:opacity-50"
            >
              {searching ? 'Searching...' : 'Search'}
            </button>
          </div>
        </motion.form>

        {/* Character Profile */}
        {character && (
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="space-y-6"
          >
            {/* Main Info */}
            <div className="card">
              <div className="flex flex-col md:flex-row md:items-start gap-6">
                <div className="flex-shrink-0">
                  <div className="w-24 h-24 rounded-xl bg-shadow-700 flex items-center justify-center">
                    <VocationIcon className={`w-12 h-12 ${vocationColors[character.vocation]}`} />
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
                  <p className="text-shadow-400 mb-4">
                    Level {character.level} - {character.vocation}
                  </p>
                  <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4 text-sm">
                    <div className="flex items-center gap-2">
                      <Globe className={`w-4 h-4 ${realmColors[character.realm]}`} />
                      <span className="text-shadow-400">Realm:</span>
                      <span className={`font-medium capitalize ${realmColors[character.realm]}`}>{character.realm}</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <User className="w-4 h-4 text-shadow-500" />
                      <span className="text-shadow-400">Sex:</span>
                      <span className="text-white capitalize">{character.sex}</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Globe className="w-4 h-4 text-shadow-500" />
                      <span className="text-shadow-400">Residence:</span>
                      <span className="text-white">{character.residence}</span>
                    </div>
                    {character.guild && (
                      <div className="flex items-center gap-2">
                        <Shield className="w-4 h-4 text-shadow-500" />
                        <span className="text-shadow-400">Guild:</span>
                        <span className="text-white">{character.guild.name}</span>
                        <span className="text-shadow-500">({character.guild.rank})</span>
                      </div>
                    )}
                    <div className="flex items-center gap-2">
                      <Trophy className="w-4 h-4 text-yellow-500" />
                      <span className="text-shadow-400">Achievements:</span>
                      <span className="text-white">{formatNumber(character.achievementPoints)} pts</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Clock className="w-4 h-4 text-shadow-500" />
                      <span className="text-shadow-400">Account Age:</span>
                      <span className="text-white">{character.accountAge}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Skills */}
            <div className="card">
              <h3 className="text-lg font-semibold text-white mb-4">Skills</h3>
              <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
                {Object.entries(character.skills).map(([skill, data]) => (
                  <div key={skill} className="bg-shadow-800/50 rounded-lg p-3">
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-shadow-400 capitalize">{skill === 'magicLevel' ? 'Magic Level' : skill}</span>
                      <span className="font-semibold text-white">{data.level}</span>
                    </div>
                    <div className="h-1.5 bg-shadow-700 rounded-full overflow-hidden">
                      <div
                        className="h-full bg-accent-500 rounded-full"
                        style={{ width: `${data.percent}%` }}
                      />
                    </div>
                    <div className="text-right text-xs text-shadow-500 mt-1">{data.percent}%</div>
                  </div>
                ))}
              </div>
            </div>

            {/* Deaths */}
            <div className="card">
              <h3 className="text-lg font-semibold text-white mb-4 flex items-center gap-2">
                <Skull className="w-5 h-5 text-red-500" />
                Recent Deaths
              </h3>
              {character.deaths.length > 0 ? (
                <div className="space-y-3">
                  {character.deaths.map((death, index) => (
                    <div key={index} className="flex items-center justify-between py-3 border-b border-shadow-700 last:border-0">
                      <div>
                        <p className="text-white">
                          Killed by <span className="text-red-400">{death.killer}</span>
                        </p>
                        <p className="text-sm text-shadow-500">at level {death.level}</p>
                      </div>
                      <span className="text-shadow-500 text-sm">{formatDate(death.date)}</span>
                    </div>
                  ))}
                </div>
              ) : (
                <p className="text-shadow-500 text-center py-8">No deaths recorded</p>
              )}
            </div>
          </motion.div>
        )}

        {/* Initial State */}
        {!character && !searching && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="text-center py-16"
          >
            <Search className="w-16 h-16 text-shadow-700 mx-auto mb-4" />
            <h3 className="text-xl font-semibold text-white mb-2">Search for a character</h3>
            <p className="text-shadow-500">Enter a character name to view their profile</p>
          </motion.div>
        )}
      </div>
    </div>
  )
}

