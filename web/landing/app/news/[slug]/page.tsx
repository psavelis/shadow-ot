'use client'

import { motion } from 'framer-motion'
import Link from 'next/link'
import { 
  Calendar, User, Tag, Heart, MessageSquare, Share2, ArrowLeft, Clock,
  Loader2, AlertCircle
} from 'lucide-react'
import { useNewsArticle } from '@/shared/hooks/useNews'
import ReactMarkdown from 'react-markdown'

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

export default function NewsArticlePage({ params }: { params: { slug: string } }) {
  const { data: article, isLoading, error } = useNewsArticle(params.slug)

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 flex items-center justify-center">
        <Loader2 className="w-12 h-12 animate-spin text-blue-400" />
      </div>
    )
  }

  if (error || !article) {
    return (
      <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950 flex items-center justify-center">
        <div className="text-center">
          <AlertCircle className="w-16 h-16 text-red-400 mx-auto mb-4" />
          <h2 className="text-2xl font-bold text-white mb-2">Article Not Found</h2>
          <p className="text-slate-400 mb-6">The news article you're looking for doesn't exist.</p>
          <Link href="/news" className="text-blue-400 hover:text-blue-300 flex items-center gap-2 justify-center">
            <ArrowLeft className="w-4 h-4" />
            Back to News
          </Link>
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      {/* Hero Image */}
      {article.image && (
        <div className="relative h-[40vh] md:h-[50vh]">
          <img
            src={article.image}
            alt={article.title}
            className="w-full h-full object-cover"
          />
          <div className="absolute inset-0 bg-gradient-to-t from-slate-950 via-slate-950/60 to-transparent" />
        </div>
      )}

      <div className="container mx-auto px-4 py-8">
        {/* Back Button */}
        <motion.div
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          className={article.image ? '-mt-32 relative z-10' : ''}
        >
          <Link 
            href="/news" 
            className="inline-flex items-center gap-2 text-slate-400 hover:text-white transition mb-6"
          >
            <ArrowLeft className="w-4 h-4" />
            Back to News
          </Link>
        </motion.div>

        <div className="max-w-4xl mx-auto">
          {/* Article Header */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            className="mb-8"
          >
            <div className="flex items-center gap-3 mb-4">
              <span className={`px-3 py-1 text-sm rounded-lg ${categoryColors[article.category] || 'bg-slate-500/20 text-slate-400'}`}>
                {article.category}
              </span>
              {article.featured && (
                <span className="px-3 py-1 bg-amber-500/20 text-amber-400 text-sm rounded-lg">
                  Featured
                </span>
              )}
            </div>

            <h1 className="text-3xl md:text-4xl lg:text-5xl font-bold text-white mb-4">
              {article.title}
            </h1>

            <p className="text-xl text-slate-400 mb-6">{article.excerpt}</p>

            {/* Meta */}
            <div className="flex flex-wrap items-center gap-4 text-sm text-slate-500">
              <div className="flex items-center gap-2">
                {article.author?.avatar && (
                  <img 
                    src={article.author.avatar} 
                    alt={article.author.name}
                    className="w-8 h-8 rounded-full"
                  />
                )}
                <span className="text-slate-300">{article.author?.name || article.author}</span>
              </div>
              <div className="flex items-center gap-1">
                <Calendar className="w-4 h-4" />
                {formatDate(article.publishedAt)}
              </div>
              {article.readTime && (
                <div className="flex items-center gap-1">
                  <Clock className="w-4 h-4" />
                  {article.readTime} min read
                </div>
              )}
            </div>
          </motion.div>

          {/* Article Content */}
          <motion.article
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
            className="prose prose-invert prose-lg max-w-none mb-12"
          >
            {article.content ? (
              <ReactMarkdown
                components={{
                  h2: ({ children }) => <h2 className="text-2xl font-bold text-white mt-8 mb-4">{children}</h2>,
                  h3: ({ children }) => <h3 className="text-xl font-bold text-white mt-6 mb-3">{children}</h3>,
                  p: ({ children }) => <p className="text-slate-300 leading-relaxed mb-4">{children}</p>,
                  ul: ({ children }) => <ul className="list-disc list-inside text-slate-300 space-y-2 mb-4">{children}</ul>,
                  ol: ({ children }) => <ol className="list-decimal list-inside text-slate-300 space-y-2 mb-4">{children}</ol>,
                  li: ({ children }) => <li className="text-slate-300">{children}</li>,
                  strong: ({ children }) => <strong className="text-white font-bold">{children}</strong>,
                  em: ({ children }) => <em className="text-slate-200 italic">{children}</em>,
                  blockquote: ({ children }) => (
                    <blockquote className="border-l-4 border-blue-500 pl-4 italic text-slate-400 my-4">
                      {children}
                    </blockquote>
                  ),
                  code: ({ children }) => (
                    <code className="bg-slate-800 text-blue-400 px-2 py-1 rounded text-sm">
                      {children}
                    </code>
                  ),
                  table: ({ children }) => (
                    <div className="overflow-x-auto my-6">
                      <table className="w-full border-collapse">
                        {children}
                      </table>
                    </div>
                  ),
                  th: ({ children }) => (
                    <th className="bg-slate-800 text-white p-3 text-left border border-slate-700">
                      {children}
                    </th>
                  ),
                  td: ({ children }) => (
                    <td className="p-3 border border-slate-700 text-slate-300">
                      {children}
                    </td>
                  ),
                }}
              >
                {article.content}
              </ReactMarkdown>
            ) : (
              <p className="text-slate-300">{article.excerpt}</p>
            )}
          </motion.article>

          {/* Tags */}
          {article.tags && article.tags.length > 0 && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.2 }}
              className="flex flex-wrap items-center gap-2 mb-8 pb-8 border-b border-slate-800"
            >
              <Tag className="w-4 h-4 text-slate-500" />
              {article.tags.map((tag) => (
                <span 
                  key={tag} 
                  className="px-3 py-1 bg-slate-800/50 text-slate-400 text-sm rounded-lg hover:bg-slate-700/50 cursor-pointer transition"
                >
                  #{tag}
                </span>
              ))}
            </motion.div>
          )}

          {/* Reactions */}
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
            className="flex items-center justify-between bg-slate-800/50 border border-slate-700/50 rounded-xl p-4"
          >
            <div className="flex items-center gap-4">
              <button className="flex items-center gap-2 px-4 py-2 bg-slate-700/50 rounded-lg text-slate-300 hover:bg-slate-700 hover:text-red-400 transition">
                <Heart className="w-5 h-5" />
                <span>{article.reactions?.likes || 0}</span>
              </button>
              <button className="flex items-center gap-2 px-4 py-2 bg-slate-700/50 rounded-lg text-slate-300 hover:bg-slate-700 hover:text-blue-400 transition">
                <MessageSquare className="w-5 h-5" />
                <span>{article.reactions?.comments || 0}</span>
              </button>
            </div>
            <button className="flex items-center gap-2 px-4 py-2 bg-slate-700/50 rounded-lg text-slate-300 hover:bg-slate-700 hover:text-green-400 transition">
              <Share2 className="w-5 h-5" />
              Share
            </button>
          </motion.div>
        </div>
      </div>
    </div>
  )
}
