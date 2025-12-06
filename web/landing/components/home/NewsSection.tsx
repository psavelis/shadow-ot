'use client'

import { motion } from 'framer-motion'
import Link from 'next/link'
import { Calendar, ArrowRight, Tag } from 'lucide-react'

const news = [
  {
    id: 1,
    title: 'New Seasonal Realm: Voidborne Launches Next Week',
    excerpt: 'Get ready for our most ambitious seasonal realm yet, featuring unique mechanics and exclusive rewards.',
    category: 'Announcement',
    date: '2024-01-15',
    image: '/images/news/voidborne.jpg',
    featured: true,
  },
  {
    id: 2,
    title: 'Winter Event 2024: Ice and Fire',
    excerpt: 'The annual winter event brings new bosses, quests, and limited-time rewards.',
    category: 'Event',
    date: '2024-01-12',
    image: '/images/news/winter-event.jpg',
  },
  {
    id: 3,
    title: 'Server Update v2.5: New Anti-Cheat System',
    excerpt: 'Major update introduces AI-powered anti-cheat detection for fairer gameplay.',
    category: 'Update',
    date: '2024-01-10',
    image: '/images/news/update.jpg',
  },
  {
    id: 4,
    title: 'NFT Marketplace Now Live on Polygon',
    excerpt: 'Trade your in-game assets as NFTs with low gas fees on Polygon network.',
    category: 'Feature',
    date: '2024-01-08',
    image: '/images/news/nft.jpg',
  },
]

const categoryColors: Record<string, string> = {
  Announcement: 'bg-accent-500/20 text-accent-400',
  Event: 'bg-purple-500/20 text-purple-400',
  Update: 'bg-blue-500/20 text-blue-400',
  Feature: 'bg-green-500/20 text-green-400',
}

export function NewsSection() {
  return (
    <section className="py-20 lg:py-32 bg-shadow-950">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.5 }}
          className="flex items-end justify-between mb-12"
        >
          <div>
            <h2 className="font-display text-4xl lg:text-5xl font-bold mb-4">
              Latest <span className="gradient-text">News</span>
            </h2>
            <p className="text-shadow-300 text-lg max-w-xl">
              Stay updated with the latest announcements, events, and updates.
            </p>
          </div>
          <Link
            href="/news"
            className="hidden md:flex items-center text-accent-500 hover:text-accent-400 transition-colors"
          >
            View All News
            <ArrowRight className="w-4 h-4 ml-2" />
          </Link>
        </motion.div>

        {/* News Grid */}
        <div className="grid lg:grid-cols-2 gap-6">
          {/* Featured Article */}
          {news.filter(n => n.featured).map((article) => (
            <motion.article
              key={article.id}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5 }}
              className="lg:row-span-2"
            >
              <Link href={`/news/${article.id}`} className="block group h-full">
                <div className="relative h-full bg-shadow-900/50 rounded-2xl border border-shadow-700 overflow-hidden hover:border-accent-500/50 transition-all duration-300">
                  {/* Image */}
                  <div className="aspect-video bg-gradient-to-br from-accent-900/50 to-shadow-900 flex items-center justify-center">
                    <span className="text-6xl">🎮</span>
                  </div>

                  {/* Content */}
                  <div className="p-6">
                    <div className="flex items-center space-x-3 mb-4">
                      <span className={`px-3 py-1 rounded-full text-xs font-medium ${categoryColors[article.category]}`}>
                        {article.category}
                      </span>
                      <div className="flex items-center text-shadow-500 text-sm">
                        <Calendar className="w-4 h-4 mr-1" />
                        {new Date(article.date).toLocaleDateString('en-US', {
                          month: 'short',
                          day: 'numeric',
                          year: 'numeric'
                        })}
                      </div>
                    </div>

                    <h3 className="font-display text-2xl font-bold text-white mb-3 group-hover:text-accent-500 transition-colors">
                      {article.title}
                    </h3>
                    <p className="text-shadow-400 line-clamp-3">
                      {article.excerpt}
                    </p>
                  </div>
                </div>
              </Link>
            </motion.article>
          ))}

          {/* Other Articles */}
          <div className="space-y-4">
            {news.filter(n => !n.featured).map((article, index) => (
              <motion.article
                key={article.id}
                initial={{ opacity: 0, y: 20 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.5, delay: index * 0.1 }}
              >
                <Link href={`/news/${article.id}`} className="block group">
                  <div className="flex items-start space-x-4 p-4 bg-shadow-900/50 rounded-xl border border-shadow-700 hover:border-accent-500/50 transition-all duration-300">
                    {/* Thumbnail */}
                    <div className="w-20 h-20 flex-shrink-0 bg-gradient-to-br from-shadow-700 to-shadow-800 rounded-lg flex items-center justify-center">
                      <span className="text-2xl">📰</span>
                    </div>

                    {/* Content */}
                    <div className="flex-grow min-w-0">
                      <div className="flex items-center space-x-2 mb-2">
                        <span className={`px-2 py-0.5 rounded-full text-xs font-medium ${categoryColors[article.category]}`}>
                          {article.category}
                        </span>
                        <span className="text-shadow-500 text-xs">
                          {new Date(article.date).toLocaleDateString('en-US', {
                            month: 'short',
                            day: 'numeric'
                          })}
                        </span>
                      </div>
                      <h3 className="font-semibold text-white group-hover:text-accent-500 transition-colors line-clamp-2">
                        {article.title}
                      </h3>
                    </div>
                  </div>
                </Link>
              </motion.article>
            ))}
          </div>
        </div>

        {/* Mobile View All */}
        <div className="mt-8 text-center md:hidden">
          <Link href="/news" className="btn-outline">
            View All News
          </Link>
        </div>
      </div>
    </section>
  )
}
