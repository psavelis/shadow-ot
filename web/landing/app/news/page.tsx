'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import Link from 'next/link'
import { Calendar, User, Tag, ChevronRight, MessageSquare, Heart, Filter } from 'lucide-react'

const newsCategories = [
  { id: 'all', label: 'All News', count: 156 },
  { id: 'announcement', label: 'Announcements', count: 23 },
  { id: 'update', label: 'Updates', count: 45 },
  { id: 'event', label: 'Events', count: 34 },
  { id: 'community', label: 'Community', count: 28 },
  { id: 'maintenance', label: 'Maintenance', count: 12 },
  { id: 'development', label: 'Development', count: 14 },
]

const newsArticles = [
  {
    id: '1',
    slug: 'winter-event-2024',
    title: 'Winter Event 2024 - Frozen Realms Await!',
    excerpt: 'The most anticipated event of the year is here! Explore frozen dungeons, defeat ice bosses, and earn exclusive winter-themed rewards.',
    category: 'event',
    author: 'Shadow Team',
    publishedAt: '2024-12-01T10:00:00Z',
    featured: true,
    image: 'https://placehold.co/800x400/1a1a2e/e94560?text=Winter+Event',
    reactions: { likes: 342, comments: 89 },
    tags: ['event', 'seasonal', 'rewards'],
  },
  {
    id: '2',
    slug: 'patch-2-5-combat-rebalance',
    title: 'Patch 2.5 - Major Combat Rebalance',
    excerpt: 'We\'ve made significant changes to the combat system based on community feedback. Check out the full changelog.',
    category: 'update',
    author: 'Dev Team',
    publishedAt: '2024-11-28T14:30:00Z',
    featured: true,
    image: 'https://placehold.co/800x400/1a1a2e/4a90d9?text=Patch+2.5',
    reactions: { likes: 256, comments: 134 },
    tags: ['patch', 'combat', 'balance'],
  },
  {
    id: '3',
    slug: 'new-realm-grimhollow',
    title: 'Introducing Grimhollow - A New Horror-Themed Realm',
    excerpt: 'Dare to enter Grimhollow, our newest realm featuring dark dungeons, terrifying creatures, and hardcore PvP.',
    category: 'announcement',
    author: 'Shadow Team',
    publishedAt: '2024-11-25T09:00:00Z',
    featured: false,
    image: 'https://placehold.co/800x400/1a1a2e/64748b?text=Grimhollow',
    reactions: { likes: 567, comments: 203 },
    tags: ['realm', 'new', 'pvp'],
  },
  {
    id: '4',
    slug: 'community-spotlight-november',
    title: 'Community Spotlight - November 2024',
    excerpt: 'Celebrating our amazing community members who have contributed maps, guides, and more this month.',
    category: 'community',
    author: 'Community Manager',
    publishedAt: '2024-11-20T16:00:00Z',
    featured: false,
    reactions: { likes: 189, comments: 45 },
    tags: ['community', 'spotlight'],
  },
  {
    id: '5',
    slug: 'scheduled-maintenance-dec-5',
    title: 'Scheduled Maintenance - December 5th',
    excerpt: 'All servers will be offline for approximately 2 hours for database optimization and security updates.',
    category: 'maintenance',
    author: 'Tech Team',
    publishedAt: '2024-12-03T08:00:00Z',
    featured: false,
    reactions: { likes: 45, comments: 12 },
    tags: ['maintenance', 'downtime'],
  },
]

const categoryColors: Record<string, string> = {
  announcement: 'bg-yellow-500/20 text-yellow-400',
  update: 'bg-blue-500/20 text-blue-400',
  event: 'bg-purple-500/20 text-purple-400',
  community: 'bg-green-500/20 text-green-400',
  maintenance: 'bg-orange-500/20 text-orange-400',
  development: 'bg-cyan-500/20 text-cyan-400',
}

function formatDate(date: string) {
  return new Date(date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

export default function NewsPage() {
  const [selectedCategory, setSelectedCategory] = useState('all')
  
  const filteredArticles = selectedCategory === 'all' 
    ? newsArticles 
    : newsArticles.filter(a => a.category === selectedCategory)

  const featuredArticles = newsArticles.filter(a => a.featured)

  return (
    <div className="min-h-screen bg-shadow-950 py-12">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <h1 className="font-display text-4xl md:text-5xl font-bold mb-4">News & Updates</h1>
          <p className="text-shadow-400 text-lg max-w-2xl mx-auto">
            Stay informed about the latest happenings in Shadow OT
          </p>
        </motion.div>

        {/* Featured Articles */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="grid md:grid-cols-2 gap-6 mb-12"
        >
          {featuredArticles.map((article, index) => (
            <Link key={article.id} href={`/news/${article.slug}`}>
              <div className="group relative h-80 rounded-2xl overflow-hidden border border-shadow-700 hover:border-accent-500/50 transition-all">
                <div className="absolute inset-0 bg-gradient-to-t from-shadow-950 via-shadow-950/80 to-transparent z-10" />
                <img
                  src={article.image}
                  alt={article.title}
                  className="absolute inset-0 w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                />
                <div className="absolute bottom-0 left-0 right-0 p-6 z-20">
                  <span className={`inline-block px-2 py-1 rounded text-xs font-medium mb-3 ${categoryColors[article.category]}`}>
                    {article.category.charAt(0).toUpperCase() + article.category.slice(1)}
                  </span>
                  <h2 className="text-xl font-bold text-white mb-2 group-hover:text-accent-400 transition-colors">
                    {article.title}
                  </h2>
                  <p className="text-shadow-400 text-sm line-clamp-2 mb-3">{article.excerpt}</p>
                  <div className="flex items-center gap-4 text-sm text-shadow-500">
                    <span className="flex items-center gap-1">
                      <Calendar className="w-4 h-4" />
                      {formatDate(article.publishedAt)}
                    </span>
                    <span className="flex items-center gap-1">
                      <Heart className="w-4 h-4" />
                      {article.reactions.likes}
                    </span>
                    <span className="flex items-center gap-1">
                      <MessageSquare className="w-4 h-4" />
                      {article.reactions.comments}
                    </span>
                  </div>
                </div>
              </div>
            </Link>
          ))}
        </motion.div>

        <div className="grid lg:grid-cols-4 gap-8">
          {/* Categories Sidebar */}
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.2 }}
            className="lg:col-span-1"
          >
            <div className="card sticky top-24">
              <h3 className="font-semibold text-white mb-4 flex items-center gap-2">
                <Filter className="w-5 h-5 text-accent-500" />
                Categories
              </h3>
              <div className="space-y-1">
                {newsCategories.map((cat) => (
                  <button
                    key={cat.id}
                    onClick={() => setSelectedCategory(cat.id)}
                    className={`w-full flex items-center justify-between px-3 py-2 rounded-lg text-sm transition-all ${
                      selectedCategory === cat.id
                        ? 'bg-accent-500/20 text-accent-400'
                        : 'text-shadow-400 hover:bg-shadow-800 hover:text-white'
                    }`}
                  >
                    <span>{cat.label}</span>
                    <span className="text-xs bg-shadow-700 px-2 py-0.5 rounded">{cat.count}</span>
                  </button>
                ))}
              </div>
            </div>
          </motion.div>

          {/* Articles List */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
            className="lg:col-span-3 space-y-6"
          >
            {filteredArticles.map((article) => (
              <Link key={article.id} href={`/news/${article.slug}`}>
                <article className="card flex gap-6 group hover:border-accent-500/50 transition-all">
                  {article.image && (
                    <div className="hidden sm:block w-48 h-32 flex-shrink-0 rounded-lg overflow-hidden">
                      <img
                        src={article.image}
                        alt={article.title}
                        className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                      />
                    </div>
                  )}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-3 mb-2">
                      <span className={`px-2 py-0.5 rounded text-xs font-medium ${categoryColors[article.category]}`}>
                        {article.category.charAt(0).toUpperCase() + article.category.slice(1)}
                      </span>
                      <span className="text-shadow-500 text-sm">{formatDate(article.publishedAt)}</span>
                    </div>
                    <h3 className="text-lg font-semibold text-white group-hover:text-accent-400 transition-colors mb-2">
                      {article.title}
                    </h3>
                    <p className="text-shadow-400 text-sm line-clamp-2 mb-3">{article.excerpt}</p>
                    <div className="flex items-center gap-4 text-sm text-shadow-500">
                      <span className="flex items-center gap-1">
                        <User className="w-4 h-4" />
                        {article.author}
                      </span>
                      <span className="flex items-center gap-1">
                        <Heart className="w-4 h-4" />
                        {article.reactions.likes}
                      </span>
                      <span className="flex items-center gap-1">
                        <MessageSquare className="w-4 h-4" />
                        {article.reactions.comments}
                      </span>
                      {article.tags && (
                        <span className="flex items-center gap-1">
                          <Tag className="w-4 h-4" />
                          {article.tags.length} tags
                        </span>
                      )}
                    </div>
                  </div>
                  <ChevronRight className="w-5 h-5 text-shadow-600 group-hover:text-accent-500 transition-colors self-center" />
                </article>
              </Link>
            ))}

            {/* Pagination */}
            <div className="flex items-center justify-center gap-2 pt-8">
              <button className="px-4 py-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">
                Previous
              </button>
              <button className="px-4 py-2 rounded-lg bg-accent-500 text-white">1</button>
              <button className="px-4 py-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">2</button>
              <button className="px-4 py-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">3</button>
              <span className="text-shadow-600">...</span>
              <button className="px-4 py-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">16</button>
              <button className="px-4 py-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">
                Next
              </button>
            </div>
          </motion.div>
        </div>
      </div>
    </div>
  )
}

