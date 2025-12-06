'use client'

import { useState } from 'react'
import Link from 'next/link'
import { motion, AnimatePresence } from 'framer-motion'
import { 
  Search, Filter, X, MessageSquare, User, Calendar, 
  Clock, Tag, ChevronDown, Loader2, FileText, Users
} from 'lucide-react'

const categories = [
  { id: 'all', name: 'All Categories' },
  { id: 'announcements', name: 'Announcements' },
  { id: 'general', name: 'General Discussion' },
  { id: 'guides', name: 'Guides & Tutorials' },
  { id: 'support', name: 'Support' },
  { id: 'market', name: 'Market' },
  { id: 'guild', name: 'Guild Recruitment' },
  { id: 'off-topic', name: 'Off-Topic' },
]

const sortOptions = [
  { id: 'relevance', name: 'Most Relevant' },
  { id: 'recent', name: 'Most Recent' },
  { id: 'popular', name: 'Most Popular' },
  { id: 'replies', name: 'Most Replies' },
]

const searchResults = [
  {
    id: 1,
    type: 'thread',
    title: 'Complete Hunting Guide for Level 100-200',
    excerpt: 'This comprehensive guide covers all the best hunting spots for players between levels 100 and 200. Whether you\'re a knight, paladin, druid, or sorcerer...',
    category: 'Guides & Tutorials',
    author: { name: 'GuideMaster', avatar: null },
    replies: 47,
    views: 3240,
    lastActivity: '2 hours ago',
    matchedTerms: ['hunting', 'guide', 'level'],
  },
  {
    id: 2,
    type: 'thread',
    title: 'Server Maintenance - December Update',
    excerpt: 'We will be performing scheduled maintenance to deploy the December content update. Expected downtime is approximately 2 hours...',
    category: 'Announcements',
    author: { name: 'Admin', avatar: null },
    replies: 23,
    views: 8521,
    lastActivity: '1 day ago',
    matchedTerms: ['update', 'server'],
  },
  {
    id: 3,
    type: 'post',
    title: 'Re: Best PvP strategies for mages',
    excerpt: '...the key to winning as a mage in PvP is positioning. Always maintain distance and use terrain to your advantage. For hunting grounds...',
    category: 'General Discussion',
    author: { name: 'PvPKing', avatar: null },
    threadTitle: 'Best PvP strategies for mages',
    replies: 156,
    views: 12400,
    lastActivity: '5 hours ago',
    matchedTerms: ['pvp', 'mage', 'hunting'],
  },
  {
    id: 4,
    type: 'thread',
    title: 'WTB Demon Armor - Offering 50kk',
    excerpt: 'Looking to buy Demon Armor. Paying well above market price. Contact me in-game or reply here with your offer.',
    category: 'Market',
    author: { name: 'RichPlayer', avatar: null },
    replies: 12,
    views: 890,
    lastActivity: '30 minutes ago',
    matchedTerms: ['demon armor', 'buy'],
  },
  {
    id: 5,
    type: 'thread',
    title: 'Guild Wars - Tactical Guide',
    excerpt: 'After participating in dozens of guild wars, I\'ve compiled my knowledge into this tactical guide. Topics covered include team composition...',
    category: 'Guides & Tutorials',
    author: { name: 'WarChief', avatar: null },
    replies: 89,
    views: 5670,
    lastActivity: '3 days ago',
    matchedTerms: ['guide', 'guild'],
  },
]

export default function SearchPage() {
  const [query, setQuery] = useState('')
  const [category, setCategory] = useState('all')
  const [sortBy, setSortBy] = useState('relevance')
  const [showFilters, setShowFilters] = useState(false)
  const [searching, setSearching] = useState(false)
  const [hasSearched, setHasSearched] = useState(false)
  const [dateRange, setDateRange] = useState('all')
  const [contentType, setContentType] = useState('all')

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!query.trim()) return
    
    setSearching(true)
    // Simulate search delay
    await new Promise(resolve => setTimeout(resolve, 800))
    setSearching(false)
    setHasSearched(true)
  }

  const clearFilters = () => {
    setCategory('all')
    setSortBy('relevance')
    setDateRange('all')
    setContentType('all')
  }

  const activeFilterCount = [category, dateRange, contentType].filter(f => f !== 'all').length

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      <div className="max-w-5xl mx-auto px-4 py-8">
        {/* Search Header */}
        <motion.div
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-8"
        >
          <h1 className="text-3xl font-bold text-white mb-2">Search Forums</h1>
          <p className="text-slate-400">Find threads, posts, and discussions</p>
        </motion.div>

        {/* Search Form */}
        <motion.form
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          onSubmit={handleSearch}
          className="mb-6"
        >
          <div className="relative">
            <Search className="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-400" />
            <input
              type="text"
              value={query}
              onChange={e => setQuery(e.target.value)}
              placeholder="Search for threads, posts, or users..."
              className="w-full bg-slate-800/50 border border-slate-700/50 rounded-xl pl-12 pr-32 py-4 text-white placeholder-slate-500 focus:outline-none focus:border-amber-500/50 text-lg"
            />
            <div className="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-2">
              <button
                type="button"
                onClick={() => setShowFilters(!showFilters)}
                className={`flex items-center gap-2 px-3 py-2 rounded-lg transition ${
                  showFilters || activeFilterCount > 0
                    ? 'bg-amber-500/20 text-amber-400'
                    : 'bg-slate-700/50 text-slate-300 hover:bg-slate-700'
                }`}
              >
                <Filter className="w-4 h-4" />
                {activeFilterCount > 0 && (
                  <span className="text-xs bg-amber-500 text-black px-1.5 rounded-full">
                    {activeFilterCount}
                  </span>
                )}
              </button>
              <button
                type="submit"
                disabled={!query.trim() || searching}
                className="px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-lg hover:from-amber-400 hover:to-orange-500 transition disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
              >
                {searching ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  <Search className="w-4 h-4" />
                )}
                Search
              </button>
            </div>
          </div>
        </motion.form>

        {/* Filters Panel */}
        <AnimatePresence>
          {showFilters && (
            <motion.div
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              exit={{ opacity: 0, height: 0 }}
              className="overflow-hidden mb-6"
            >
              <div className="bg-slate-800/30 border border-slate-700/50 rounded-xl p-5">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-white font-medium">Filters</h3>
                  {activeFilterCount > 0 && (
                    <button
                      onClick={clearFilters}
                      className="text-sm text-amber-400 hover:text-amber-300 transition flex items-center gap-1"
                    >
                      <X className="w-3 h-3" />
                      Clear all
                    </button>
                  )}
                </div>

                <div className="grid grid-cols-4 gap-4">
                  {/* Category */}
                  <div>
                    <label className="text-slate-400 text-sm mb-2 block">Category</label>
                    <select
                      value={category}
                      onChange={e => setCategory(e.target.value)}
                      className="w-full bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:border-amber-500/50"
                    >
                      {categories.map(cat => (
                        <option key={cat.id} value={cat.id}>{cat.name}</option>
                      ))}
                    </select>
                  </div>

                  {/* Sort By */}
                  <div>
                    <label className="text-slate-400 text-sm mb-2 block">Sort By</label>
                    <select
                      value={sortBy}
                      onChange={e => setSortBy(e.target.value)}
                      className="w-full bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:border-amber-500/50"
                    >
                      {sortOptions.map(opt => (
                        <option key={opt.id} value={opt.id}>{opt.name}</option>
                      ))}
                    </select>
                  </div>

                  {/* Date Range */}
                  <div>
                    <label className="text-slate-400 text-sm mb-2 block">Date Range</label>
                    <select
                      value={dateRange}
                      onChange={e => setDateRange(e.target.value)}
                      className="w-full bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:border-amber-500/50"
                    >
                      <option value="all">All Time</option>
                      <option value="day">Last 24 Hours</option>
                      <option value="week">Last Week</option>
                      <option value="month">Last Month</option>
                      <option value="year">Last Year</option>
                    </select>
                  </div>

                  {/* Content Type */}
                  <div>
                    <label className="text-slate-400 text-sm mb-2 block">Content Type</label>
                    <select
                      value={contentType}
                      onChange={e => setContentType(e.target.value)}
                      className="w-full bg-slate-900 border border-slate-700 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:border-amber-500/50"
                    >
                      <option value="all">All Content</option>
                      <option value="threads">Threads Only</option>
                      <option value="posts">Posts Only</option>
                    </select>
                  </div>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        {/* Search Results */}
        {hasSearched && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="space-y-4"
          >
            {/* Results Header */}
            <div className="flex items-center justify-between">
              <p className="text-slate-400">
                Found <span className="text-white font-medium">{searchResults.length}</span> results for "{query}"
              </p>
            </div>

            {/* Results List */}
            {searchResults.map((result, idx) => (
              <motion.div
                key={result.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: idx * 0.05 }}
                className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5 hover:border-slate-600/50 transition group"
              >
                <div className="flex items-start gap-4">
                  {/* Type Icon */}
                  <div className={`w-10 h-10 rounded-lg flex items-center justify-center flex-shrink-0 ${
                    result.type === 'thread' ? 'bg-blue-500/20' : 'bg-purple-500/20'
                  }`}>
                    {result.type === 'thread' ? (
                      <FileText className={`w-5 h-5 ${result.type === 'thread' ? 'text-blue-400' : 'text-purple-400'}`} />
                    ) : (
                      <MessageSquare className="w-5 h-5 text-purple-400" />
                    )}
                  </div>

                  {/* Content */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="text-xs px-2 py-0.5 rounded bg-slate-700/50 text-slate-400">
                        {result.category}
                      </span>
                      {result.type === 'post' && (
                        <span className="text-xs text-slate-500">
                          in: {result.threadTitle}
                        </span>
                      )}
                    </div>

                    <Link
                      href={`/t/${result.id}`}
                      className="text-lg font-medium text-white group-hover:text-amber-400 transition block mb-2"
                    >
                      {result.title}
                    </Link>

                    <p className="text-slate-400 text-sm mb-3 line-clamp-2">
                      {result.excerpt}
                    </p>

                    {/* Matched Terms */}
                    <div className="flex items-center gap-2 mb-3">
                      <Tag className="w-3 h-3 text-slate-500" />
                      {result.matchedTerms.map(term => (
                        <span
                          key={term}
                          className="text-xs px-2 py-0.5 rounded bg-amber-500/10 text-amber-400 border border-amber-500/20"
                        >
                          {term}
                        </span>
                      ))}
                    </div>

                    {/* Meta */}
                    <div className="flex items-center gap-4 text-xs text-slate-500">
                      <span className="flex items-center gap-1">
                        <User className="w-3 h-3" />
                        {result.author.name}
                      </span>
                      <span className="flex items-center gap-1">
                        <MessageSquare className="w-3 h-3" />
                        {result.replies} replies
                      </span>
                      <span className="flex items-center gap-1">
                        <Clock className="w-3 h-3" />
                        {result.lastActivity}
                      </span>
                    </div>
                  </div>
                </div>
              </motion.div>
            ))}
          </motion.div>
        )}

        {/* Empty State */}
        {!hasSearched && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="text-center py-20"
          >
            <div className="w-20 h-20 bg-slate-800/50 rounded-full flex items-center justify-center mx-auto mb-6">
              <Search className="w-10 h-10 text-slate-600" />
            </div>
            <h2 className="text-xl font-medium text-white mb-2">Search the Forums</h2>
            <p className="text-slate-400 max-w-md mx-auto">
              Enter keywords to find threads, posts, and discussions across all forum categories.
            </p>

            {/* Popular Searches */}
            <div className="mt-8">
              <p className="text-slate-500 text-sm mb-3">Popular searches:</p>
              <div className="flex flex-wrap justify-center gap-2">
                {['hunting guide', 'exp spots', 'pvp build', 'guild recruitment', 'market prices'].map(term => (
                  <button
                    key={term}
                    onClick={() => setQuery(term)}
                    className="px-3 py-1.5 bg-slate-800/50 border border-slate-700/50 rounded-full text-slate-300 text-sm hover:border-amber-500/30 hover:text-amber-400 transition"
                  >
                    {term}
                  </button>
                ))}
              </div>
            </div>
          </motion.div>
        )}
      </div>
    </div>
  )
}

