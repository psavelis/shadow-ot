'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { Trophy, Star, Lock, CheckCircle, Filter, Search, Sparkles } from 'lucide-react'

const categories = [
  { id: 'all', label: 'All', count: 892 },
  { id: 'exploration', label: 'Exploration', count: 156 },
  { id: 'combat', label: 'Combat', count: 234 },
  { id: 'social', label: 'Social', count: 89 },
  { id: 'economy', label: 'Economy', count: 67 },
  { id: 'collection', label: 'Collection', count: 145 },
  { id: 'skill', label: 'Skill', count: 123 },
  { id: 'quest', label: 'Quest', count: 78 },
]

const rarityColors: Record<string, { bg: string; text: string; border: string }> = {
  common: { bg: 'bg-gray-500/20', text: 'text-gray-400', border: 'border-gray-500' },
  uncommon: { bg: 'bg-green-500/20', text: 'text-green-400', border: 'border-green-500' },
  rare: { bg: 'bg-blue-500/20', text: 'text-blue-400', border: 'border-blue-500' },
  epic: { bg: 'bg-purple-500/20', text: 'text-purple-400', border: 'border-purple-500' },
  legendary: { bg: 'bg-yellow-500/20', text: 'text-yellow-400', border: 'border-yellow-500' },
}

const achievements = [
  { id: '1', name: 'First Steps', description: 'Reach level 10', category: 'exploration', points: 10, rarity: 'common', unlocked: true, unlockedAt: '2024-01-15' },
  { id: '2', name: 'Dragon Slayer', description: 'Kill 100 dragons', category: 'combat', points: 50, rarity: 'rare', unlocked: true, unlockedAt: '2024-06-20', progress: { current: 100, required: 100 } },
  { id: '3', name: 'Social Butterfly', description: 'Add 50 friends', category: 'social', points: 25, rarity: 'uncommon', unlocked: false, progress: { current: 32, required: 50 } },
  { id: '4', name: 'Millionaire', description: 'Have 1,000,000 gold', category: 'economy', points: 100, rarity: 'epic', unlocked: true, unlockedAt: '2024-09-01' },
  { id: '5', name: 'Master Collector', description: 'Collect all rare items', category: 'collection', points: 200, rarity: 'legendary', unlocked: false, progress: { current: 45, required: 100 }, secret: true },
  { id: '6', name: 'Sword Master', description: 'Reach sword fighting 100', category: 'skill', points: 75, rarity: 'rare', unlocked: false, progress: { current: 85, required: 100 } },
  { id: '7', name: 'Quest Champion', description: 'Complete 50 quests', category: 'quest', points: 50, rarity: 'uncommon', unlocked: true, unlockedAt: '2024-11-10', progress: { current: 50, required: 50 } },
  { id: '8', name: 'World Explorer', description: 'Discover all areas', category: 'exploration', points: 150, rarity: 'epic', unlocked: false, progress: { current: 78, required: 120 } },
]

const stats = {
  total: 892,
  unlocked: 156,
  points: 1250,
  rank: 'Gold',
}

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
}

export default function AchievementsPage() {
  const [selectedCategory, setSelectedCategory] = useState('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [showUnlockedOnly, setShowUnlockedOnly] = useState(false)

  const filteredAchievements = achievements.filter(a => {
    const matchesCategory = selectedCategory === 'all' || a.category === selectedCategory
    const matchesSearch = a.name.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesUnlocked = !showUnlockedOnly || a.unlocked
    return matchesCategory && matchesSearch && matchesUnlocked
  })

  return (
    <div className="space-y-6">
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }}>
        <h1 className="text-2xl font-display font-bold mb-1">Achievements</h1>
        <p className="text-shadow-400">Track your progress and earn rewards</p>
      </motion.div>

      {/* Stats */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="stat-card">
          <div className="flex items-center justify-between mb-3">
            <div className="w-10 h-10 bg-accent-500/10 rounded-lg flex items-center justify-center">
              <Trophy className="w-5 h-5 text-accent-500" />
            </div>
          </div>
          <h3 className="text-2xl font-bold text-white">{stats.unlocked}/{stats.total}</h3>
          <p className="text-shadow-400 text-sm">Achievements Unlocked</p>
        </div>
        <div className="stat-card">
          <div className="flex items-center justify-between mb-3">
            <div className="w-10 h-10 bg-yellow-500/10 rounded-lg flex items-center justify-center">
              <Star className="w-5 h-5 text-yellow-500" />
            </div>
          </div>
          <h3 className="text-2xl font-bold text-white">{stats.points.toLocaleString()}</h3>
          <p className="text-shadow-400 text-sm">Achievement Points</p>
        </div>
        <div className="stat-card">
          <div className="flex items-center justify-between mb-3">
            <div className="w-10 h-10 bg-purple-500/10 rounded-lg flex items-center justify-center">
              <Sparkles className="w-5 h-5 text-purple-500" />
            </div>
          </div>
          <h3 className="text-2xl font-bold text-white">{Math.round((stats.unlocked / stats.total) * 100)}%</h3>
          <p className="text-shadow-400 text-sm">Completion Rate</p>
        </div>
        <div className="stat-card">
          <div className="flex items-center justify-between mb-3">
            <div className="w-10 h-10 bg-green-500/10 rounded-lg flex items-center justify-center">
              <Trophy className="w-5 h-5 text-green-500" />
            </div>
          </div>
          <h3 className="text-2xl font-bold text-yellow-400">{stats.rank}</h3>
          <p className="text-shadow-400 text-sm">Current Rank</p>
        </div>
      </motion.div>

      {/* Filters */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="card">
        <div className="flex flex-col md:flex-row gap-4">
          <div className="relative flex-1">
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search achievements..."
              className="w-full pl-12 pr-4 py-2.5 bg-shadow-800 border border-shadow-600 rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50"
            />
          </div>
          <div className="flex gap-2 overflow-x-auto pb-2 md:pb-0">
            {categories.map((cat) => (
              <button
                key={cat.id}
                onClick={() => setSelectedCategory(cat.id)}
                className={`px-4 py-2 rounded-lg text-sm whitespace-nowrap transition-all ${
                  selectedCategory === cat.id
                    ? 'bg-accent-500 text-white'
                    : 'bg-shadow-800 text-shadow-400 hover:text-white'
                }`}
              >
                {cat.label} ({cat.count})
              </button>
            ))}
          </div>
        </div>
        <div className="mt-4 flex items-center gap-2">
          <input
            type="checkbox"
            id="unlocked-only"
            checked={showUnlockedOnly}
            onChange={(e) => setShowUnlockedOnly(e.target.checked)}
            className="w-4 h-4 rounded border-shadow-600 bg-shadow-800 text-accent-500 focus:ring-accent-500/50"
          />
          <label htmlFor="unlocked-only" className="text-sm text-shadow-400">
            Show unlocked only
          </label>
        </div>
      </motion.div>

      {/* Achievements Grid */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.3 }} className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {filteredAchievements.map((achievement) => {
          const rarity = rarityColors[achievement.rarity]
          return (
            <div
              key={achievement.id}
              className={`card relative overflow-hidden ${!achievement.unlocked && 'opacity-70'}`}
            >
              {/* Rarity indicator */}
              <div className={`absolute top-0 right-0 w-16 h-16 ${rarity.bg} transform rotate-45 translate-x-8 -translate-y-8`} />
              
              <div className="flex items-start gap-4">
                <div className={`w-12 h-12 rounded-xl flex items-center justify-center ${rarity.bg} border ${rarity.border}`}>
                  {achievement.unlocked ? (
                    <CheckCircle className={`w-6 h-6 ${rarity.text}`} />
                  ) : achievement.secret ? (
                    <Lock className="w-6 h-6 text-shadow-500" />
                  ) : (
                    <Trophy className={`w-6 h-6 ${rarity.text}`} />
                  )}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <h3 className="font-semibold text-white truncate">{achievement.name}</h3>
                    <span className={`px-1.5 py-0.5 rounded text-xs ${rarity.bg} ${rarity.text}`}>
                      {achievement.rarity}
                    </span>
                  </div>
                  <p className="text-shadow-400 text-sm mb-2">{achievement.description}</p>
                  
                  {achievement.progress && !achievement.unlocked && (
                    <div className="mb-2">
                      <div className="flex justify-between text-xs mb-1">
                        <span className="text-shadow-500">Progress</span>
                        <span className="text-white">{achievement.progress.current}/{achievement.progress.required}</span>
                      </div>
                      <div className="h-1.5 bg-shadow-700 rounded-full overflow-hidden">
                        <div
                          className="h-full bg-accent-500 rounded-full"
                          style={{ width: `${(achievement.progress.current / achievement.progress.required) * 100}%` }}
                        />
                      </div>
                    </div>
                  )}
                  
                  <div className="flex items-center justify-between text-sm">
                    <span className="flex items-center gap-1 text-yellow-400">
                      <Star className="w-4 h-4" />
                      {achievement.points} pts
                    </span>
                    {achievement.unlocked && achievement.unlockedAt && (
                      <span className="text-shadow-500">{formatDate(achievement.unlockedAt)}</span>
                    )}
                  </div>
                </div>
              </div>
            </div>
          )
        })}
      </motion.div>

      {filteredAchievements.length === 0 && (
        <div className="text-center py-16">
          <Trophy className="w-16 h-16 text-shadow-700 mx-auto mb-4" />
          <h3 className="text-xl font-semibold text-white mb-2">No achievements found</h3>
          <p className="text-shadow-500">Try adjusting your filters</p>
        </div>
      )}
    </div>
  )
}

