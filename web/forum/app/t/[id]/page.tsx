'use client'

import { useState } from 'react'
import { motion } from 'framer-motion'
import Link from 'next/link'
import { 
  ArrowLeft, MessageSquare, Eye, Clock, Heart, ThumbsUp, Laugh, Quote, Edit, Trash2, 
  Flag, Pin, Lock, Share2, ChevronLeft, ChevronRight
} from 'lucide-react'

const thread = {
  id: '1',
  title: 'Best hunting spots for level 300 Elite Knight?',
  category: { id: 'general', name: 'General Discussion' },
  author: { id: '1', name: 'DragonSlayer', avatar: null, posts: 234, reputation: 156, joinedAt: '2022-03-15' },
  createdAt: '2024-12-04T14:30:00Z',
  views: 1234,
  replies: 23,
  pinned: false,
  locked: false,
  tags: ['hunting', 'knight', 'guide'],
}

const posts = [
  {
    id: '1',
    author: { id: '1', name: 'DragonSlayer', avatar: null, title: 'Veteran', posts: 234, reputation: 156, joinedAt: '2022-03-15', badges: ['verified', 'helper'] },
    content: `Hey everyone!\n\nI recently hit level 300 on my Elite Knight and I'm looking for some good hunting spots. I've been hunting at Asura Palace but the exp seems to have slowed down.\n\nMy current gear:\n- Cobra Sword (attack 52)\n- Ornate Shield\n- Full Falcon Set\n\nWhat spots would you recommend for solo hunting at this level? Looking for something with good exp/h and decent profit.\n\nThanks in advance!`,
    createdAt: '2024-12-04T14:30:00Z',
    reactions: { likes: 5, helpful: 2, funny: 0 },
    isOriginalPost: true,
  },
  {
    id: '2',
    author: { id: '2', name: 'ProHunter', avatar: null, title: 'Hunting Expert', posts: 567, reputation: 423, joinedAt: '2021-06-20', badges: ['expert', 'guide-writer'] },
    content: `Great question! At level 300 EK, you have several solid options:\n\n**Top Recommendations:**\n1. **Issavi Surface (Sphinx/Lamassu)** - 2.5-3kk exp/h, good profit\n2. **Carnivores (Feyrist)** - 2-2.5kk exp/h, decent charm points\n3. **Catacombs** - 2.8-3.2kk exp/h, needs team ideally\n\n**Pro Tips:**\n- Use imbued items (critical, life leech, mana leech)\n- Bring fire bombs for emergencies\n- Consider hunting during non-peak hours\n\nWith your gear, Issavi should be very doable solo. Just be careful with the Lamassu waves!`,
    createdAt: '2024-12-04T14:45:00Z',
    reactions: { likes: 12, helpful: 8, funny: 0 },
    isOriginalPost: false,
  },
  {
    id: '3',
    author: { id: '3', name: 'CasualPlayer', avatar: null, title: 'Member', posts: 45, reputation: 23, joinedAt: '2024-01-10', badges: [] },
    content: `I'm also level 300 EK and I've been having great success at the Issavi surface. Make sure to bring plenty of mana potions - you'll need them!\n\nAlso, don't forget to check the Wiki for respawn patterns.`,
    createdAt: '2024-12-04T15:10:00Z',
    reactions: { likes: 3, helpful: 1, funny: 0 },
    isOriginalPost: false,
  },
]

const badgeColors: Record<string, string> = {
  verified: 'bg-green-500/20 text-green-400',
  helper: 'bg-blue-500/20 text-blue-400',
  expert: 'bg-purple-500/20 text-purple-400',
  'guide-writer': 'bg-yellow-500/20 text-yellow-400',
}

function formatDate(date: string) {
  return new Date(date).toLocaleString('en-US', { 
    year: 'numeric', month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit'
  })
}

function formatJoinDate(date: string) {
  return new Date(date).toLocaleDateString('en-US', { year: 'numeric', month: 'short' })
}

export default function ThreadPage({ params }: { params: { id: string } }) {
  const [replyContent, setReplyContent] = useState('')

  return (
    <div className="min-h-screen bg-shadow-950 py-8">
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Breadcrumb */}
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} className="mb-6">
          <div className="flex items-center gap-2 text-sm">
            <Link href="/forum" className="text-shadow-400 hover:text-white transition-colors">Forums</Link>
            <span className="text-shadow-600">/</span>
            <Link href={`/c/${thread.category.id}`} className="text-shadow-400 hover:text-white transition-colors">
              {thread.category.name}
            </Link>
            <span className="text-shadow-600">/</span>
            <span className="text-white truncate max-w-xs">{thread.title}</span>
          </div>
        </motion.div>

        {/* Thread Header */}
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="card mb-6">
          <div className="flex items-start justify-between">
            <div>
              <div className="flex items-center gap-2 mb-2">
                {thread.pinned && <Pin className="w-4 h-4 text-yellow-500" />}
                {thread.locked && <Lock className="w-4 h-4 text-red-500" />}
                <h1 className="text-2xl font-display font-bold text-white">{thread.title}</h1>
              </div>
              <div className="flex flex-wrap items-center gap-4 text-sm text-shadow-400">
                <span className="flex items-center gap-1">
                  <Eye className="w-4 h-4" />
                  {thread.views.toLocaleString()} views
                </span>
                <span className="flex items-center gap-1">
                  <MessageSquare className="w-4 h-4" />
                  {thread.replies} replies
                </span>
                <span className="flex items-center gap-1">
                  <Clock className="w-4 h-4" />
                  {formatDate(thread.createdAt)}
                </span>
              </div>
              <div className="flex gap-2 mt-3">
                {thread.tags.map((tag) => (
                  <span key={tag} className="px-2 py-0.5 bg-shadow-800 text-shadow-400 rounded text-xs">
                    #{tag}
                  </span>
                ))}
              </div>
            </div>
            <div className="flex items-center gap-2">
              <button className="p-2 text-shadow-400 hover:text-white hover:bg-shadow-800 rounded-lg transition-colors">
                <Share2 className="w-4 h-4" />
              </button>
              <button className="p-2 text-shadow-400 hover:text-white hover:bg-shadow-800 rounded-lg transition-colors">
                <Flag className="w-4 h-4" />
              </button>
            </div>
          </div>
        </motion.div>

        {/* Posts */}
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="space-y-6">
          {posts.map((post, index) => (
            <div key={post.id} className="card">
              <div className="flex gap-6">
                {/* Author Info */}
                <div className="hidden md:block w-40 flex-shrink-0 text-center">
                  <div className="w-16 h-16 rounded-xl bg-shadow-700 mx-auto mb-2 flex items-center justify-center">
                    <span className="text-2xl font-bold text-accent-500">
                      {post.author.name.charAt(0)}
                    </span>
                  </div>
                  <Link href={`/u/${post.author.id}`} className="font-semibold text-white hover:text-accent-400 transition-colors">
                    {post.author.name}
                  </Link>
                  {post.author.title && (
                    <p className="text-accent-400 text-xs mt-1">{post.author.title}</p>
                  )}
                  <div className="flex justify-center gap-1 mt-2">
                    {post.author.badges.map((badge) => (
                      <span key={badge} className={`px-1.5 py-0.5 rounded text-xs ${badgeColors[badge]}`}>
                        {badge}
                      </span>
                    ))}
                  </div>
                  <div className="mt-3 text-xs text-shadow-500 space-y-1">
                    <p>{post.author.posts} posts</p>
                    <p>Rep: {post.author.reputation}</p>
                    <p>Joined {formatJoinDate(post.author.joinedAt)}</p>
                  </div>
                </div>

                {/* Post Content */}
                <div className="flex-1 min-w-0">
                  {/* Mobile author info */}
                  <div className="md:hidden flex items-center gap-3 mb-4 pb-4 border-b border-shadow-700">
                    <div className="w-10 h-10 rounded-lg bg-shadow-700 flex items-center justify-center">
                      <span className="font-bold text-accent-500">{post.author.name.charAt(0)}</span>
                    </div>
                    <div>
                      <p className="font-semibold text-white">{post.author.name}</p>
                      <p className="text-shadow-500 text-xs">{formatDate(post.createdAt)}</p>
                    </div>
                  </div>

                  {/* Post number and date (desktop) */}
                  <div className="hidden md:flex items-center justify-between mb-4 pb-4 border-b border-shadow-700">
                    <span className="text-shadow-500 text-sm">#{index + 1}</span>
                    <span className="text-shadow-500 text-sm">{formatDate(post.createdAt)}</span>
                  </div>

                  {/* Content */}
                  <div className="prose prose-invert prose-sm max-w-none mb-4">
                    {post.content.split('\n').map((line, i) => (
                      <p key={i} className={line === '' ? 'h-4' : 'text-shadow-200'}>
                        {line}
                      </p>
                    ))}
                  </div>

                  {/* Actions */}
                  <div className="flex items-center justify-between pt-4 border-t border-shadow-700">
                    <div className="flex items-center gap-4">
                      <button className="flex items-center gap-1 text-shadow-400 hover:text-accent-400 transition-colors">
                        <Heart className="w-4 h-4" />
                        <span className="text-sm">{post.reactions.likes}</span>
                      </button>
                      <button className="flex items-center gap-1 text-shadow-400 hover:text-green-400 transition-colors">
                        <ThumbsUp className="w-4 h-4" />
                        <span className="text-sm">{post.reactions.helpful}</span>
                      </button>
                      <button className="flex items-center gap-1 text-shadow-400 hover:text-yellow-400 transition-colors">
                        <Laugh className="w-4 h-4" />
                        <span className="text-sm">{post.reactions.funny}</span>
                      </button>
                    </div>
                    <div className="flex items-center gap-2">
                      <button className="p-1.5 text-shadow-400 hover:text-white hover:bg-shadow-800 rounded transition-colors">
                        <Quote className="w-4 h-4" />
                      </button>
                      <button className="p-1.5 text-shadow-400 hover:text-white hover:bg-shadow-800 rounded transition-colors">
                        <Flag className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          ))}
        </motion.div>

        {/* Reply Box */}
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.3 }} className="card mt-6">
          <h3 className="font-semibold text-white mb-4">Reply to this thread</h3>
          <textarea
            value={replyContent}
            onChange={(e) => setReplyContent(e.target.value)}
            placeholder="Write your reply..."
            className="w-full min-h-[150px] px-4 py-3 bg-shadow-800 border border-shadow-600 rounded-lg text-white placeholder:text-shadow-500 focus:outline-none focus:ring-2 focus:ring-accent-500/50 resize-y"
          />
          <div className="flex justify-between items-center mt-4">
            <p className="text-shadow-500 text-sm">Markdown supported</p>
            <button className="btn-primary" disabled={!replyContent.trim()}>
              Post Reply
            </button>
          </div>
        </motion.div>

        {/* Pagination */}
        <div className="flex items-center justify-center gap-2 mt-8">
          <button className="p-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">
            <ChevronLeft className="w-5 h-5" />
          </button>
          <span className="px-4 py-2 text-shadow-400">Page 1 of 1</span>
          <button className="p-2 rounded-lg bg-shadow-800 text-shadow-400 hover:text-white transition-colors">
            <ChevronRight className="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>
  )
}


