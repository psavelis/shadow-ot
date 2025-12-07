'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import Link from 'next/link'
import { Search, Book, Sword, Shield, Wand2, Map, Users, Trophy, Sparkles, ChevronRight, Star } from 'lucide-react'

const categories = [
  { id: 'items', label: 'Items', icon: Sword, count: 2456, color: 'text-red-400 bg-red-500/20' },
  { id: 'creatures', label: 'Creatures', icon: Sparkles, count: 1234, color: 'text-purple-400 bg-purple-500/20' },
  { id: 'spells', label: 'Spells', icon: Wand2, count: 687, color: 'text-blue-400 bg-blue-500/20' },
  { id: 'quests', label: 'Quests', icon: Map, count: 345, color: 'text-green-400 bg-green-500/20' },
  { id: 'npcs', label: 'NPCs', icon: Users, count: 567, color: 'text-yellow-400 bg-yellow-500/20' },
  { id: 'achievements', label: 'Achievements', icon: Trophy, count: 892, color: 'text-orange-400 bg-orange-500/20' },
  { id: 'locations', label: 'Locations', icon: Map, count: 234, color: 'text-cyan-400 bg-cyan-500/20' },
  { id: 'guides', label: 'Guides', icon: Book, count: 156, color: 'text-pink-400 bg-pink-500/20' },
]

const popularArticles = [
  { id: '1', title: 'Demon Helmet', category: 'items', views: 12453, rating: 4.8 },
  { id: '2', title: 'Dragon Lord', category: 'creatures', views: 9876, rating: 4.9 },
  { id: '3', title: 'The Annihilator Quest', category: 'quests', views: 8765, rating: 4.7 },
  { id: '4', title: 'Best Hunting Spots Level 100-200', category: 'guides', views: 15234, rating: 4.9 },
  { id: '5', title: 'Ultimate Heal (exura vita)', category: 'spells', views: 7654, rating: 4.6 },
]

const recentArticles = [
  { id: '6', title: 'Frost Dragon', category: 'creatures', updated: '2 hours ago' },
  { id: '7', title: 'Winter Event Guide 2024', category: 'guides', updated: '5 hours ago' },
  { id: '8', title: 'Ice Rapier', category: 'items', updated: '1 day ago' },
  { id: '9', title: 'Frozen Citadel', category: 'locations', updated: '1 day ago' },
  { id: '10', title: 'Frost King', category: 'creatures', updated: '2 days ago' },
]

const categoryColors: Record<string, string> = {
  items: 'text-red-400',
  creatures: 'text-purple-400',
  spells: 'text-blue-400',
  quests: 'text-green-400',
  npcs: 'text-yellow-400',
  achievements: 'text-orange-400',
  locations: 'text-cyan-400',
  guides: 'text-pink-400',
}

export default function WikiPage() {
  const [searchQuery, setSearchQuery] = useState('')

  return (
    <div className="min-h-screen bg-shadow-950 py-12">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <Book className="w-16 h-16 text-accent-500 mx-auto mb-4" />
          <h1 className="font-display text-4xl md:text-5xl font-bold mb-4">Shadow OT Wiki</h1>
          <p className="text-shadow-400 text-lg max-w-2xl mx-auto">
            Your comprehensive guide to everything in Shadow OT
          </p>
        </motion.div>

        {/* Search */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="max-w-2xl mx-auto mb-12"
        >
          <div className="relative">
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-shadow-500" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search the wiki..."
              className="w-full pl-12 pr-4 py-4 bg-shadow-800 border border-shadow-600 rounded-xl text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50 text-lg"
            />
          </div>
          {searchQuery && (
            <div className="mt-2 p-4 bg-shadow-800 border border-shadow-600 rounded-xl">
              <p className="text-shadow-400 text-sm">Press Enter to search for "{searchQuery}"</p>
            </div>
          )}
        </motion.div>

        {/* Categories Grid */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="mb-12"
        >
          <h2 className="font-display text-2xl font-bold mb-6">Browse by Category</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            {categories.map((cat) => (
              <Link key={cat.id} href={`/wiki/${cat.id}`}>
                <div className="card text-center hover:border-accent-500/50 transition-all group py-6">
                  <div className={`w-12 h-12 rounded-xl mx-auto mb-3 flex items-center justify-center ${cat.color.split(' ')[1]}`}>
                    <cat.icon className={`w-6 h-6 ${cat.color.split(' ')[0]}`} />
                  </div>
                  <h3 className="font-semibold text-white group-hover:text-accent-400 transition-colors">
                    {cat.label}
                  </h3>
                  <p className="text-shadow-500 text-sm">{cat.count.toLocaleString()} entries</p>
                </div>
              </Link>
            ))}
          </div>
        </motion.div>

        <div className="grid lg:grid-cols-2 gap-8">
          {/* Popular Articles */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
          >
            <div className="card">
              <h2 className="font-display text-xl font-bold mb-4 flex items-center gap-2">
                <Star className="w-5 h-5 text-yellow-500" />
                Popular Articles
              </h2>
              <div className="space-y-3">
                {popularArticles.map((article, index) => (
                  <Link key={article.id} href={`/wiki/${article.category}/${article.id}`}>
                    <div className="flex items-center gap-4 p-3 rounded-lg hover:bg-shadow-800 transition-colors group">
                      <span className="text-2xl font-bold text-shadow-600 w-8">{index + 1}</span>
                      <div className="flex-1 min-w-0">
                        <h3 className="font-medium text-white group-hover:text-accent-400 transition-colors">
                          {article.title}
                        </h3>
                        <p className={`text-sm capitalize ${categoryColors[article.category]}`}>
                          {article.category}
                        </p>
                      </div>
                      <div className="text-right text-sm">
                        <div className="flex items-center gap-1 text-yellow-400">
                          <Star className="w-4 h-4 fill-current" />
                          {article.rating}
                        </div>
                        <p className="text-shadow-500">{article.views.toLocaleString()} views</p>
                      </div>
                      <ChevronRight className="w-5 h-5 text-shadow-600 group-hover:text-accent-500 transition-colors" />
                    </div>
                  </Link>
                ))}
              </div>
            </div>
          </motion.div>

          {/* Recently Updated */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4 }}
          >
            <div className="card">
              <h2 className="font-display text-xl font-bold mb-4 flex items-center gap-2">
                <Sparkles className="w-5 h-5 text-accent-500" />
                Recently Updated
              </h2>
              <div className="space-y-3">
                {recentArticles.map((article) => (
                  <Link key={article.id} href={`/wiki/${article.category}/${article.id}`}>
                    <div className="flex items-center gap-4 p-3 rounded-lg hover:bg-shadow-800 transition-colors group">
                      <div className="flex-1 min-w-0">
                        <h3 className="font-medium text-white group-hover:text-accent-400 transition-colors">
                          {article.title}
                        </h3>
                        <p className={`text-sm capitalize ${categoryColors[article.category]}`}>
                          {article.category}
                        </p>
                      </div>
                      <span className="text-sm text-shadow-500">{article.updated}</span>
                      <ChevronRight className="w-5 h-5 text-shadow-600 group-hover:text-accent-500 transition-colors" />
                    </div>
                  </Link>
                ))}
              </div>
            </div>
          </motion.div>
        </div>

        {/* Quick Links */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5 }}
          className="mt-12"
        >
          <h2 className="font-display text-2xl font-bold mb-6">Quick Links</h2>
          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <Link href="/wiki/guides/getting-started" className="card hover:border-accent-500/50 transition-all group">
              <h3 className="font-semibold text-white group-hover:text-accent-400 mb-2">Getting Started</h3>
              <p className="text-shadow-400 text-sm">New to Shadow OT? Start here!</p>
            </Link>
            <Link href="/wiki/guides/vocations" className="card hover:border-accent-500/50 transition-all group">
              <h3 className="font-semibold text-white group-hover:text-accent-400 mb-2">Vocation Guide</h3>
              <p className="text-shadow-400 text-sm">Choose your path wisely</p>
            </Link>
            <Link href="/wiki/guides/hunting" className="card hover:border-accent-500/50 transition-all group">
              <h3 className="font-semibold text-white group-hover:text-accent-400 mb-2">Hunting Spots</h3>
              <p className="text-shadow-400 text-sm">Best places to level up</p>
            </Link>
            <Link href="/wiki/guides/economy" className="card hover:border-accent-500/50 transition-all group">
              <h3 className="font-semibold text-white group-hover:text-accent-400 mb-2">Economy Guide</h3>
              <p className="text-shadow-400 text-sm">Make gold efficiently</p>
            </Link>
          </div>
        </motion.div>

        {/* Contribute CTA */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.6 }}
          className="mt-12 text-center"
        >
          <div className="card bg-gradient-to-br from-accent-500/20 to-purple-500/20 border-accent-500/30">
            <h2 className="font-display text-2xl font-bold mb-2">Help Improve the Wiki</h2>
            <p className="text-shadow-400 mb-4">
              Share your knowledge! Create or edit articles to help other players.
            </p>
            <Link href="/wiki/contribute" className="btn-primary">
              Start Contributing
            </Link>
          </div>
        </motion.div>
      </div>
    </div>
  )
}


