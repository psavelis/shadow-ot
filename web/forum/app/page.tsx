'use client'

import { motion } from 'framer-motion'
import Link from 'next/link'
import { 
  MessageSquare, 
  Users, 
  Pin, 
  Clock, 
  ChevronRight,
  Megaphone,
  Flame,
  HelpCircle,
  Palette,
  ShoppingCart,
  Bug,
  Swords,
  Crown,
  Sparkles
} from 'lucide-react'

const categories = [
  {
    id: 'announcements',
    name: 'News & Announcements',
    description: 'Official news, updates, and announcements from the team',
    icon: Megaphone,
    color: 'text-yellow-500 bg-yellow-500/20',
    threads: 145,
    posts: 892,
    lastPost: { title: 'Winter Event 2024 is here!', author: 'Admin', time: '2 hours ago' },
  },
  {
    id: 'general',
    name: 'General Discussion',
    description: 'Talk about anything related to Shadow OT',
    icon: MessageSquare,
    color: 'text-blue-500 bg-blue-500/20',
    threads: 3247,
    posts: 28492,
    lastPost: { title: 'Best hunting spot for level 300?', author: 'DragonSlayer', time: '5 min ago' },
  },
  {
    id: 'guides',
    name: 'Guides & Tutorials',
    description: 'Player-created guides, tutorials, and strategies',
    icon: HelpCircle,
    color: 'text-green-500 bg-green-500/20',
    threads: 567,
    posts: 4521,
    lastPost: { title: '[Guide] Complete Bestiary Farm 2024', author: 'MasterGuide', time: '1 hour ago' },
  },
  {
    id: 'trading',
    name: 'Trading',
    description: 'Buy, sell, and trade items with other players',
    icon: ShoppingCart,
    color: 'text-amber-500 bg-amber-500/20',
    threads: 8923,
    posts: 45231,
    lastPost: { title: 'WTS Demon Armor - 500k', author: 'Merchant', time: '3 min ago' },
  },
  {
    id: 'support',
    name: 'Support',
    description: 'Get help with technical issues and account problems',
    icon: Bug,
    color: 'text-red-500 bg-red-500/20',
    threads: 1234,
    posts: 5678,
    lastPost: { title: 'Client crashing on startup', author: 'NewPlayer', time: '30 min ago' },
  },
  {
    id: 'creative',
    name: 'Creative Corner',
    description: 'Fan art, stories, videos, and creative content',
    icon: Palette,
    color: 'text-purple-500 bg-purple-500/20',
    threads: 456,
    posts: 2341,
    lastPost: { title: 'My Knight fanart collection', author: 'ArtistPro', time: '4 hours ago' },
  },
]

const realmForums = [
  { id: 'shadowveil', name: 'Shadowveil', icon: Sparkles, color: 'text-accent-500', threads: 1234, posts: 8765 },
  { id: 'aetheria', name: 'Aetheria', icon: Crown, color: 'text-blue-400', threads: 2345, posts: 12345 },
  { id: 'warbound', name: 'Warbound', icon: Swords, color: 'text-orange-500', threads: 876, posts: 5432 },
]

const recentThreads = [
  { id: 1, title: 'Winter Event 2024 is here!', category: 'Announcements', author: 'Admin', replies: 156, views: 4521, pinned: true, hot: true },
  { id: 2, title: 'Best hunting spot for level 300?', category: 'General', author: 'DragonSlayer', replies: 23, views: 342, pinned: false, hot: true },
  { id: 3, title: '[Guide] Complete Bestiary Farm 2024', category: 'Guides', author: 'MasterGuide', replies: 45, views: 1234, pinned: true, hot: false },
  { id: 4, title: 'WTS Demon Armor - 500k', category: 'Trading', author: 'Merchant', replies: 8, views: 156, pinned: false, hot: false },
  { id: 5, title: 'Guild recruitment - Dark Legion', category: 'Shadowveil', author: 'GuildMaster', replies: 34, views: 567, pinned: false, hot: false },
]

const stats = { users: 52847, threads: 15678, posts: 234567, online: 847 }

export default function ForumHomePage() {
  return (
    <div className="space-y-8">
      {/* Hero */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} className="text-center py-8">
        <h1 className="font-display text-4xl font-bold mb-2">Community Forums</h1>
        <p className="text-shadow-400">Join the discussion with thousands of Shadow OT players</p>
      </motion.div>

      {/* Stats */}
      <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.1 }} className="grid grid-cols-4 gap-4">
        <div className="card text-center">
          <Users className="w-6 h-6 text-accent-500 mx-auto mb-2" />
          <p className="text-2xl font-bold text-white">{stats.users.toLocaleString()}</p>
          <p className="text-shadow-400 text-sm">Members</p>
        </div>
        <div className="card text-center">
          <MessageSquare className="w-6 h-6 text-accent-500 mx-auto mb-2" />
          <p className="text-2xl font-bold text-white">{stats.threads.toLocaleString()}</p>
          <p className="text-shadow-400 text-sm">Threads</p>
        </div>
        <div className="card text-center">
          <MessageSquare className="w-6 h-6 text-accent-500 mx-auto mb-2" />
          <p className="text-2xl font-bold text-white">{stats.posts.toLocaleString()}</p>
          <p className="text-shadow-400 text-sm">Posts</p>
        </div>
        <div className="card text-center">
          <span className="w-2 h-2 bg-green-500 rounded-full mx-auto mb-2 block animate-pulse" />
          <p className="text-2xl font-bold text-green-400">{stats.online}</p>
          <p className="text-shadow-400 text-sm">Online Now</p>
        </div>
      </motion.div>

      <div className="grid lg:grid-cols-3 gap-6">
        {/* Categories */}
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.2 }} className="lg:col-span-2 space-y-4">
          <h2 className="font-display text-xl font-bold">Categories</h2>
          {categories.map((category) => (
            <Link key={category.id} href={`/c/${category.id}`}>
              <div className="card flex items-center space-x-4 hover:border-accent-500/50 transition-all group">
                <div className={`w-12 h-12 rounded-lg flex items-center justify-center ${category.color}`}>
                  <category.icon className="w-6 h-6" />
                </div>
                <div className="flex-1 min-w-0">
                  <h3 className="font-semibold text-white group-hover:text-accent-500 transition-colors">{category.name}</h3>
                  <p className="text-shadow-400 text-sm truncate">{category.description}</p>
                </div>
                <div className="text-right text-sm hidden sm:block">
                  <p className="text-white">{category.threads.toLocaleString()} threads</p>
                  <p className="text-shadow-500">{category.posts.toLocaleString()} posts</p>
                </div>
                <ChevronRight className="w-5 h-5 text-shadow-500 group-hover:text-accent-500 transition-colors" />
              </div>
            </Link>
          ))}

          {/* Realm Forums */}
          <h2 className="font-display text-xl font-bold mt-8">Realm Forums</h2>
          <div className="grid sm:grid-cols-3 gap-4">
            {realmForums.map((realm) => (
              <Link key={realm.id} href={`/r/${realm.id}`}>
                <div className="card text-center hover:border-accent-500/50 transition-all group py-6">
                  <realm.icon className={`w-8 h-8 ${realm.color} mx-auto mb-2`} />
                  <h3 className="font-semibold text-white group-hover:text-accent-500">{realm.name}</h3>
                  <p className="text-shadow-500 text-sm">{realm.threads} threads</p>
                </div>
              </Link>
            ))}
          </div>
        </motion.div>

        {/* Sidebar */}
        <motion.div initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: 0.3 }} className="space-y-6">
          <div className="card">
            <h2 className="font-display text-lg font-bold mb-4">Recent Threads</h2>
            <div className="space-y-3">
              {recentThreads.map((thread) => (
                <Link key={thread.id} href={`/t/${thread.id}`} className="block group">
                  <div className="flex items-start space-x-2">
                    {thread.pinned && <Pin className="w-4 h-4 text-yellow-500 flex-shrink-0 mt-0.5" />}
                    {thread.hot && !thread.pinned && <Flame className="w-4 h-4 text-orange-500 flex-shrink-0 mt-0.5" />}
                    <div className="min-w-0">
                      <p className="text-white text-sm font-medium group-hover:text-accent-500 truncate">{thread.title}</p>
                      <p className="text-shadow-500 text-xs">{thread.author} • {thread.replies} replies</p>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
          </div>

          <div className="card">
            <h2 className="font-display text-lg font-bold mb-4">Online Users</h2>
            <div className="flex flex-wrap gap-2">
              {['Admin', 'DragonSlayer', 'MysticMage', 'IronKnight', 'SwiftArrow'].map((user) => (
                <span key={user} className="px-2 py-1 bg-shadow-800 rounded text-xs text-shadow-300">{user}</span>
              ))}
              <span className="px-2 py-1 text-shadow-500 text-xs">+842 more</span>
            </div>
          </div>
        </motion.div>
      </div>
    </div>
  )
}

