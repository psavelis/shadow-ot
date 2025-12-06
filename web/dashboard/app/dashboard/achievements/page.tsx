'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import { Trophy, Star, Lock, CheckCircle, Search, Loader2, AlertTriangle } from 'lucide-react'
import { useAchievements, useAchievementLeaderboard } from '@/shared/hooks/useDashboard'
import type { PlayerAchievement } from '@/shared/api/endpoints'

const categories = [
  { id: 'all', label: 'All' },
  { id: 'exploration', label: 'Exploration' },
  { id: 'combat', label: 'Combat' },
  { id: 'social', label: 'Social' },
  { id: 'economy', label: 'Economy' },
  { id: 'collection', label: 'Collection' },
  { id: 'special', label: 'Special' },
]

const rarityColors: Record<string, { bg: string; text: string; border: string }> = {
  common: { bg: 'bg-slate-500/20', text: 'text-slate-400', border: 'border-slate-500/30' },
  uncommon: { bg: 'bg-green-500/20', text: 'text-green-400', border: 'border-green-500/30' },
  rare: { bg: 'bg-blue-500/20', text: 'text-blue-400', border: 'border-blue-500/30' },
  epic: { bg: 'bg-purple-500/20', text: 'text-purple-400', border: 'border-purple-500/30' },
  legendary: { bg: 'bg-orange-500/20', text: 'text-orange-400', border: 'border-orange-500/30' },
}

export default function AchievementsPage() {
  const [selectedCategory, setSelectedCategory] = useState('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [showCompleted, setShowCompleted] = useState(true)

  // Real API hooks
  const { data: achievementsData, isLoading, error } = useAchievements()
  const { data: leaderboardData } = useAchievementLeaderboard({ pageSize: 5 })

  const achievements = achievementsData?.achievements || []
  const totalPoints = achievementsData?.totalPoints || 0
  const completedCount = achievementsData?.completedCount || 0
  const totalCount = achievementsData?.totalCount || 0
  const leaderboard = leaderboardData?.data || []

  const filteredAchievements = achievements.filter(ach => {
    const matchesCategory = selectedCategory === 'all' || ach.category === selectedCategory
    const matchesSearch = ach.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                          ach.description.toLowerCase().includes(searchQuery.toLowerCase())
    const matchesCompleted = showCompleted || !ach.unlocked
    return matchesCategory && matchesSearch && matchesCompleted
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
          <h1 className="text-2xl font-bold text-white mb-1">Achievements</h1>
          <p className="text-slate-400">Track your progress and accomplishments</p>
        </div>
      </motion.div>

      {/* Stats */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="grid grid-cols-4 gap-4"
      >
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Trophy className="w-5 h-5 text-amber-400 mb-2" />
          <p className="text-2xl font-bold text-white">{totalPoints}</p>
          <p className="text-xs text-slate-500">Total Points</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <CheckCircle className="w-5 h-5 text-emerald-400 mb-2" />
          <p className="text-2xl font-bold text-white">{completedCount}</p>
          <p className="text-xs text-slate-500">Completed</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Lock className="w-5 h-5 text-slate-400 mb-2" />
          <p className="text-2xl font-bold text-white">{totalCount - completedCount}</p>
          <p className="text-xs text-slate-500">Remaining</p>
        </div>
        <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
          <Star className="w-5 h-5 text-purple-400 mb-2" />
          <p className="text-2xl font-bold text-white">
            {totalCount > 0 ? Math.round((completedCount / totalCount) * 100) : 0}%
          </p>
          <p className="text-xs text-slate-500">Completion</p>
        </div>
      </motion.div>

      <div className="grid lg:grid-cols-4 gap-6">
        {/* Sidebar */}
        <motion.div
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ delay: 0.2 }}
          className="lg:col-span-1 space-y-4"
        >
          {/* Categories */}
          <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
            <h3 className="text-sm font-semibold text-slate-400 mb-3">Categories</h3>
            <div className="space-y-1">
              {categories.map(cat => {
                const count = cat.id === 'all' 
                  ? achievements.length 
                  : achievements.filter(a => a.category === cat.id).length
                return (
                  <button
                    key={cat.id}
                    onClick={() => setSelectedCategory(cat.id)}
                    className={`w-full flex items-center justify-between px-3 py-2 rounded-lg text-left transition ${
                      selectedCategory === cat.id
                        ? 'bg-amber-500/20 text-amber-400'
                        : 'text-slate-400 hover:bg-slate-700/50'
                    }`}
                  >
                    <span className="text-sm">{cat.label}</span>
                    <span className="text-xs bg-slate-700/50 px-2 py-0.5 rounded">{count}</span>
                  </button>
                )
              })}
            </div>
          </div>

          {/* Top Players */}
          <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4">
            <h3 className="text-sm font-semibold text-slate-400 mb-3">Top Achievement Hunters</h3>
            <div className="space-y-2">
              {leaderboard.map((player, idx) => (
                <div key={player.character.id} className="flex items-center gap-3">
                  <span className={`w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold ${
                    idx === 0 ? 'bg-amber-500 text-black' :
                    idx === 1 ? 'bg-slate-400 text-black' :
                    idx === 2 ? 'bg-orange-600 text-white' :
                    'bg-slate-700 text-slate-400'
                  }`}>
                    {player.rank}
                  </span>
                  <div className="flex-1 min-w-0">
                    <p className="text-white text-sm truncate">{player.character.name}</p>
                    <p className="text-slate-500 text-xs">{player.points} pts</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </motion.div>

        {/* Main Content */}
        <div className="lg:col-span-3 space-y-4">
          {/* Search & Filters */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
            className="flex gap-4"
          >
            <div className="relative flex-1">
              <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
              <input
                type="text"
                placeholder="Search achievements..."
                value={searchQuery}
                onChange={e => setSearchQuery(e.target.value)}
                className="w-full bg-slate-800/50 border border-slate-700/50 rounded-lg pl-10 pr-4 py-2 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50"
              />
            </div>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={showCompleted}
                onChange={e => setShowCompleted(e.target.checked)}
                className="w-4 h-4 rounded border-slate-600 bg-slate-800 text-amber-500 focus:ring-amber-500/50"
              />
              <span className="text-slate-400 text-sm">Show completed</span>
            </label>
          </motion.div>

          {/* Achievements Grid */}
          {isLoading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
            </div>
          ) : error ? (
            <div className="flex items-center justify-center py-12 text-red-400">
              <AlertTriangle className="w-5 h-5 mr-2" />
              Failed to load achievements
            </div>
          ) : filteredAchievements.length === 0 ? (
            <div className="text-center py-16 bg-slate-800/50 border border-slate-700/50 rounded-xl">
              <Trophy className="w-16 h-16 text-slate-700 mx-auto mb-4" />
              <h3 className="text-xl font-semibold text-white mb-2">No achievements found</h3>
              <p className="text-slate-500">Try adjusting your filters</p>
            </div>
          ) : (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.4 }}
              className="grid sm:grid-cols-2 gap-4"
            >
              {filteredAchievements.map((achievement, idx) => {
                const rarity = rarityColors[achievement.rarity] || rarityColors.common
                return (
                  <motion.div
                    key={achievement.id}
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    transition={{ delay: idx * 0.02 }}
                    className={`relative bg-slate-800/50 border ${rarity.border} rounded-xl p-4 ${
                      !achievement.unlocked && 'opacity-70'
                    }`}
                  >
                    {/* Secret Badge */}
                    {achievement.secret && !achievement.unlocked && (
                      <div className="absolute top-3 right-3">
                        <span className="px-2 py-0.5 bg-slate-700/50 text-slate-500 text-xs rounded">
                          Secret
                        </span>
                      </div>
                    )}

                    <div className="flex items-start gap-3">
                      <div className={`w-12 h-12 rounded-lg flex items-center justify-center ${rarity.bg}`}>
                        {achievement.unlocked ? (
                          <Trophy className={`w-6 h-6 ${rarity.text}`} />
                        ) : (
                          <Lock className="w-6 h-6 text-slate-500" />
                        )}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2 mb-1">
                          <h3 className="text-white font-medium truncate">{achievement.name}</h3>
                          <span className={`px-2 py-0.5 rounded text-xs ${rarity.bg} ${rarity.text} capitalize`}>
                            {achievement.rarity}
                          </span>
                        </div>
                        <p className="text-slate-400 text-sm mb-2">{achievement.description}</p>
                        
                        {/* Progress Bar */}
                        {achievement.progress && !achievement.unlocked && (
                          <div className="mb-2">
                            <div className="flex items-center justify-between text-xs text-slate-500 mb-1">
                              <span>Progress</span>
                              <span>{achievement.progress.current}/{achievement.progress.required}</span>
                            </div>
                            <div className="h-1.5 bg-slate-700 rounded-full overflow-hidden">
                              <div
                                className={`h-full ${rarity.bg.replace('/20', '')} transition-all`}
                                style={{ width: `${(achievement.progress.current / achievement.progress.required) * 100}%` }}
                              />
                            </div>
                          </div>
                        )}

                        <div className="flex items-center justify-between">
                          <span className="text-amber-400 font-medium text-sm">
                            +{achievement.points} pts
                          </span>
                          {achievement.unlocked && achievement.unlockedAt && (
                            <span className="text-slate-500 text-xs">
                              Unlocked {new Date(achievement.unlockedAt).toLocaleDateString()}
                            </span>
                          )}
                        </div>
                      </div>
                    </div>
                  </motion.div>
                )
              })}
            </motion.div>
          )}
        </div>
      </div>
    </div>
  )
}
