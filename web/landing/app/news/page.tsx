'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import Link from 'next/link'
import { 
  Calendar, User, Tag, ChevronRight, MessageSquare, Heart, 
  Loader2, AlertCircle, Newspaper 
} from 'lucide-react'
import { useNews, useFeaturedNews, NEWS_CATEGORIES } from '@/shared/hooks/useNews'
import type { NewsCategory, NewsArticle } from '@/shared/types'

const categoryColors: Record<string, string> = {
  announcement: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
  update: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
  event: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
  community: 'bg-green-500/20 text-green-400 border-green-500/30',
  maintenance: 'bg-orange-500/20 text-orange-400 border-orange-500/30',
  development: 'bg-cyan-500/20 text-cyan-400 border-cyan-500/30',
}

function formatDate(dateString: string) {
  const date = new Date(dateString)
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
}

export default function NewsPage() {
  const [selectedCategory, setSelectedCategory] = useState<NewsCategory | undefined>(undefined)
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 10

  // Fetch news from API
  const { data: newsData, isLoading, error } = useNews(selectedCategory, currentPage, pageSize)
  const { data: featuredNews, isLoading: featuredLoading } = useFeaturedNews()

  const articles = newsData?.data || []

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 py-20">
      <div className="container mx-auto px-4">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center mb-12"
        >
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-blue-500/10 border border-blue-500/30 rounded-full text-blue-400 text-sm mb-6">
            <Newspaper className="w-4 h-4" />
            News & Updates
          </div>
          <h1 className="text-4xl md:text-5xl font-bold text-white mb-4">
            Shadow OT <span className="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-cyan-500">News</span>
          </h1>
          <p className="text-slate-400 text-lg max-w-2xl mx-auto">
            Stay updated with the latest announcements, patches, events, and community highlights
          </p>
        </motion.div>

        {/* Featured News */}
        {!featuredLoading && featuredNews && featuredNews.length > 0 && (
          <motion.section
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            className="mb-12"
          >
            <h2 className="text-xl font-bold text-white mb-6 flex items-center gap-2">
              <span className="w-2 h-2 bg-amber-400 rounded-full animate-pulse" />
              Featured
            </h2>
            <div className="grid md:grid-cols-2 gap-6">
              {featuredNews.slice(0, 2).map((article, idx) => (
                <Link href={`/news/${article.slug}`} key={article.id}>
                  <div className="group relative h-64 rounded-2xl overflow-hidden bg-slate-800/50 border border-slate-700/50 hover:border-blue-500/50 transition-all">
                    {article.image && (
                      <img 
                        src={article.image} 
                        alt={article.title}
                        className="absolute inset-0 w-full h-full object-cover opacity-40 group-hover:opacity-60 transition-opacity"
                      />
                    )}
                    <div className="absolute inset-0 bg-gradient-to-t from-slate-900 via-slate-900/80 to-transparent" />
                    <div className="absolute inset-0 p-6 flex flex-col justify-end">
                      <span className={`inline-flex items-center w-fit px-2 py-1 text-xs rounded border mb-3 ${categoryColors[article.category] || 'bg-slate-500/20 text-slate-400 border-slate-500/30'}`}>
                        {article.category}
                      </span>
                      <h3 className="text-xl font-bold text-white mb-2 group-hover:text-blue-400 transition">
                        {article.title}
                      </h3>
                      <p className="text-slate-400 text-sm line-clamp-2">{article.excerpt}</p>
                      <div className="flex items-center gap-4 mt-4 text-xs text-slate-500">
                        <span className="flex items-center gap-1">
                          <Calendar className="w-3 h-3" />
                          {formatDate(article.publishedAt)}
                        </span>
                        <span className="flex items-center gap-1">
                          <User className="w-3 h-3" />
                          {article.author}
                        </span>
                        <span className="flex items-center gap-1">
                          <Heart className="w-3 h-3" />
                          {article.reactions?.likes || 0}
                        </span>
                        <span className="flex items-center gap-1">
                          <MessageSquare className="w-3 h-3" />
                          {article.reactions?.comments || 0}
                        </span>
                      </div>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
          </motion.section>
        )}

        {/* Category Filter */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="flex flex-wrap gap-2 mb-8"
        >
          <button
            onClick={() => {
              setSelectedCategory(undefined)
              setCurrentPage(1)
            }}
            className={`px-4 py-2 rounded-lg text-sm font-medium transition ${
              !selectedCategory 
                ? 'bg-blue-500 text-white' 
                : 'bg-slate-800/50 border border-slate-700/50 text-slate-400 hover:border-slate-600'
            }`}
          >
            All News
          </button>
          {NEWS_CATEGORIES.map((cat) => (
            <button
              key={cat.value}
              onClick={() => {
                setSelectedCategory(cat.value)
                setCurrentPage(1)
              }}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition ${
                selectedCategory === cat.value
                  ? 'bg-blue-500 text-white'
                  : 'bg-slate-800/50 border border-slate-700/50 text-slate-400 hover:border-slate-600'
              }`}
            >
              {cat.label}
            </button>
          ))}
        </motion.div>

        {/* News Grid */}
        {isLoading ? (
          <div className="flex items-center justify-center py-20">
            <Loader2 className="w-12 h-12 animate-spin text-blue-400" />
          </div>
        ) : error ? (
          <div className="flex flex-col items-center justify-center py-20 text-red-400">
            <AlertCircle className="w-12 h-12 mb-4" />
            <p>Failed to load news. Please try again.</p>
          </div>
        ) : articles.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-20 text-slate-400">
            <Newspaper className="w-12 h-12 mb-4 opacity-50" />
            <p>No news articles found</p>
          </div>
        ) : (
          <>
            <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
              {articles.map((article, idx) => (
                <motion.div
                  key={article.id}
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: Math.min(idx * 0.05, 0.3) }}
                >
                  <Link href={`/news/${article.slug}`}>
                    <div className="group bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden hover:border-blue-500/50 transition-all h-full flex flex-col">
                      {article.image && (
                        <div className="h-40 overflow-hidden">
                          <img 
                            src={article.image} 
                            alt={article.title}
                            className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                          />
                        </div>
                      )}
                      <div className="p-5 flex-1 flex flex-col">
                        <div className="flex items-center gap-2 mb-3">
                          <span className={`px-2 py-1 text-xs rounded border ${categoryColors[article.category] || 'bg-slate-500/20 text-slate-400 border-slate-500/30'}`}>
                            {article.category}
                          </span>
                          <span className="text-slate-500 text-xs">
                            {formatDate(article.publishedAt)}
                          </span>
                        </div>
                        <h3 className="text-lg font-bold text-white mb-2 group-hover:text-blue-400 transition line-clamp-2">
                          {article.title}
                        </h3>
                        <p className="text-slate-400 text-sm flex-1 line-clamp-3">
                          {article.excerpt}
                        </p>
                        <div className="flex items-center justify-between mt-4 pt-4 border-t border-slate-700/50">
                          <div className="flex items-center gap-3 text-xs text-slate-500">
                            <span className="flex items-center gap-1">
                              <Heart className="w-3 h-3" />
                              {article.reactions?.likes || 0}
                            </span>
                            <span className="flex items-center gap-1">
                              <MessageSquare className="w-3 h-3" />
                              {article.reactions?.comments || 0}
                            </span>
                          </div>
                          <span className="text-blue-400 text-sm flex items-center gap-1 group-hover:gap-2 transition-all">
                            Read more
                            <ChevronRight className="w-4 h-4" />
                          </span>
                        </div>
                      </div>
                    </div>
                  </Link>
                </motion.div>
              ))}
            </div>

            {/* Pagination */}
            {newsData && newsData.totalPages > 1 && (
              <div className="mt-8 flex items-center justify-center gap-4">
                <button
                  onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
                  disabled={currentPage === 1}
                  className="px-4 py-2 bg-slate-800/50 border border-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-700/50 transition disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Previous
                </button>
                <span className="text-slate-400">
                  Page {currentPage} of {newsData.totalPages}
                </span>
                <button
                  onClick={() => setCurrentPage(p => Math.min(newsData.totalPages, p + 1))}
                  disabled={currentPage === newsData.totalPages}
                  className="px-4 py-2 bg-slate-800/50 border border-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-700/50 transition disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Next
                </button>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  )
}
