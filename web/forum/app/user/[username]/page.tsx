'use client'

import { useState } from 'react'
import Link from 'next/link'
import { motion } from 'framer-motion'
import * as Tabs from '@radix-ui/react-tabs'
import { 
  User, Calendar, MessageSquare, Heart, Award, Shield, 
  Crown, Star, Clock, Eye, MapPin, Globe, Mail, 
  ExternalLink, ChevronRight, TrendingUp
} from 'lucide-react'

const userData = {
  id: 1,
  username: 'ShadowMaster',
  title: 'Forum Legend',
  avatar: null,
  joinDate: 'March 2020',
  lastSeen: '5 minutes ago',
  location: 'Thais',
  website: 'https://shadowmaster.ot',
  signature: 'May the shadows guide your path...',
  stats: {
    posts: 2847,
    threads: 156,
    reputation: 4532,
    thanks: 1289,
    likes: 3421,
    followers: 234,
    following: 89,
  },
  badges: [
    { id: 1, name: 'Founder', icon: Crown, color: 'amber', description: 'Original forum member' },
    { id: 2, name: 'Guide Writer', icon: Award, color: 'blue', description: '10+ guides published' },
    { id: 3, name: 'Helpful', icon: Heart, color: 'pink', description: '1000+ thanks received' },
    { id: 4, name: 'Veteran', icon: Shield, color: 'purple', description: '3+ years membership' },
    { id: 5, name: 'Rising Star', icon: Star, color: 'emerald', description: 'Top contributor' },
  ],
  characters: [
    { name: 'Shadow Master', level: 523, vocation: 'Elite Knight', world: 'Shadowlands' },
    { name: 'Dark Mage', level: 412, vocation: 'Master Sorcerer', world: 'Shadowlands' },
    { name: 'Swift Arrow', level: 389, vocation: 'Royal Paladin', world: 'Realms of Chaos' },
  ],
}

const recentPosts = [
  { id: 1, thread: 'Best hunting spots for EK 300+', excerpt: 'I highly recommend trying the new spawns...', time: '2 hours ago', likes: 12 },
  { id: 2, thread: 'Server downtime update', excerpt: 'Thanks for the quick fix!', time: '5 hours ago', likes: 3 },
  { id: 3, thread: 'Guild Wars Strategy Guide', excerpt: 'Great guide, but I think you missed...', time: '1 day ago', likes: 28 },
  { id: 4, thread: 'New boss mechanics discussion', excerpt: 'The key is to stack fire resistance...', time: '2 days ago', likes: 45 },
  { id: 5, thread: 'Market price predictions', excerpt: 'Based on recent trends, I expect...', time: '3 days ago', likes: 67 },
]

const recentThreads = [
  { id: 1, title: 'Complete Solo Hunting Guide (All Vocations)', replies: 156, views: 12400, time: '1 week ago' },
  { id: 2, title: 'Optimal Skill Training Routes 2024', replies: 89, views: 8700, time: '2 weeks ago' },
  { id: 3, title: 'Boss Loot Statistics Analysis', replies: 234, views: 18900, time: '1 month ago' },
]

const activityData = [
  { month: 'Jul', posts: 45 },
  { month: 'Aug', posts: 62 },
  { month: 'Sep', posts: 38 },
  { month: 'Oct', posts: 71 },
  { month: 'Nov', posts: 55 },
  { month: 'Dec', posts: 48 },
]

export default function UserProfilePage() {
  const [activeTab, setActiveTab] = useState('posts')
  const maxPosts = Math.max(...activityData.map(d => d.posts))

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      <div className="max-w-6xl mx-auto px-4 py-8">
        {/* Profile Header */}
        <motion.div
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-slate-800/50 border border-slate-700/50 rounded-2xl overflow-hidden mb-6"
        >
          {/* Banner */}
          <div className="h-32 bg-gradient-to-r from-amber-500/20 via-purple-500/20 to-blue-500/20" />
          
          <div className="px-8 pb-8">
            <div className="flex items-end gap-6 -mt-12">
              {/* Avatar */}
              <div className="w-24 h-24 bg-gradient-to-br from-amber-500 to-orange-600 rounded-2xl flex items-center justify-center border-4 border-slate-800 shadow-xl">
                <span className="text-3xl font-bold text-white">
                  {userData.username[0]}
                </span>
              </div>

              {/* Name & Title */}
              <div className="flex-1 pb-2">
                <div className="flex items-center gap-3">
                  <h1 className="text-2xl font-bold text-white">{userData.username}</h1>
                  <span className="px-2 py-0.5 bg-amber-500/20 text-amber-400 text-xs rounded-full border border-amber-500/30">
                    {userData.title}
                  </span>
                </div>
                <p className="text-slate-400 text-sm mt-1">"{userData.signature}"</p>
              </div>

              {/* Actions */}
              <div className="flex items-center gap-3 pb-2">
                <button className="px-4 py-2 bg-slate-700/50 text-slate-300 rounded-lg hover:bg-slate-700 transition text-sm">
                  Follow
                </button>
                <button className="px-4 py-2 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-lg hover:from-amber-400 hover:to-orange-500 transition text-sm">
                  Message
                </button>
              </div>
            </div>

            {/* Info Row */}
            <div className="flex items-center gap-6 mt-6 text-sm text-slate-400">
              <span className="flex items-center gap-1.5">
                <Calendar className="w-4 h-4" />
                Joined {userData.joinDate}
              </span>
              <span className="flex items-center gap-1.5">
                <Clock className="w-4 h-4" />
                Last seen {userData.lastSeen}
              </span>
              <span className="flex items-center gap-1.5">
                <MapPin className="w-4 h-4" />
                {userData.location}
              </span>
              {userData.website && (
                <a 
                  href={userData.website}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center gap-1.5 text-amber-400 hover:text-amber-300 transition"
                >
                  <Globe className="w-4 h-4" />
                  Website
                  <ExternalLink className="w-3 h-3" />
                </a>
              )}
            </div>

            {/* Stats */}
            <div className="grid grid-cols-7 gap-4 mt-6">
              {[
                { label: 'Posts', value: userData.stats.posts },
                { label: 'Threads', value: userData.stats.threads },
                { label: 'Reputation', value: userData.stats.reputation },
                { label: 'Thanks', value: userData.stats.thanks },
                { label: 'Likes', value: userData.stats.likes },
                { label: 'Followers', value: userData.stats.followers },
                { label: 'Following', value: userData.stats.following },
              ].map(stat => (
                <div key={stat.label} className="text-center">
                  <div className="text-xl font-bold text-white">{stat.value.toLocaleString()}</div>
                  <div className="text-xs text-slate-500">{stat.label}</div>
                </div>
              ))}
            </div>

            {/* Badges */}
            <div className="flex items-center gap-3 mt-6">
              {userData.badges.map(badge => (
                <div
                  key={badge.id}
                  className={`flex items-center gap-2 px-3 py-1.5 rounded-full border ${
                    badge.color === 'amber' ? 'bg-amber-500/10 border-amber-500/30 text-amber-400' :
                    badge.color === 'blue' ? 'bg-blue-500/10 border-blue-500/30 text-blue-400' :
                    badge.color === 'pink' ? 'bg-pink-500/10 border-pink-500/30 text-pink-400' :
                    badge.color === 'purple' ? 'bg-purple-500/10 border-purple-500/30 text-purple-400' :
                    'bg-emerald-500/10 border-emerald-500/30 text-emerald-400'
                  }`}
                  title={badge.description}
                >
                  <badge.icon className="w-3.5 h-3.5" />
                  <span className="text-xs font-medium">{badge.name}</span>
                </div>
              ))}
            </div>
          </div>
        </motion.div>

        {/* Content Grid */}
        <div className="grid grid-cols-3 gap-6">
          {/* Main Content */}
          <div className="col-span-2 space-y-6">
            {/* Tabs */}
            <Tabs.Root value={activeTab} onValueChange={setActiveTab}>
              <Tabs.List className="flex gap-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-1">
                {['posts', 'threads', 'activity'].map(tab => (
                  <Tabs.Trigger
                    key={tab}
                    value={tab}
                    className={`flex-1 px-4 py-2 rounded-lg text-sm font-medium transition ${
                      activeTab === tab
                        ? 'bg-gradient-to-r from-amber-500 to-orange-600 text-white'
                        : 'text-slate-400 hover:text-white hover:bg-slate-700/50'
                    }`}
                  >
                    {tab.charAt(0).toUpperCase() + tab.slice(1)}
                  </Tabs.Trigger>
                ))}
              </Tabs.List>

              <Tabs.Content value="posts" className="mt-4">
                <motion.div
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  className="space-y-3"
                >
                  {recentPosts.map((post, idx) => (
                    <motion.div
                      key={post.id}
                      initial={{ opacity: 0, y: 10 }}
                      animate={{ opacity: 1, y: 0 }}
                      transition={{ delay: idx * 0.05 }}
                      className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 hover:border-slate-600/50 transition"
                    >
                      <Link href={`/t/${post.id}`} className="text-white font-medium hover:text-amber-400 transition">
                        {post.thread}
                      </Link>
                      <p className="text-slate-400 text-sm mt-1">{post.excerpt}</p>
                      <div className="flex items-center gap-4 mt-3 text-xs text-slate-500">
                        <span className="flex items-center gap-1">
                          <Clock className="w-3 h-3" />
                          {post.time}
                        </span>
                        <span className="flex items-center gap-1">
                          <Heart className="w-3 h-3" />
                          {post.likes} likes
                        </span>
                      </div>
                    </motion.div>
                  ))}
                </motion.div>
              </Tabs.Content>

              <Tabs.Content value="threads" className="mt-4">
                <motion.div
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  className="space-y-3"
                >
                  {recentThreads.map((thread, idx) => (
                    <motion.div
                      key={thread.id}
                      initial={{ opacity: 0, y: 10 }}
                      animate={{ opacity: 1, y: 0 }}
                      transition={{ delay: idx * 0.05 }}
                      className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 hover:border-slate-600/50 transition"
                    >
                      <Link href={`/t/${thread.id}`} className="text-white font-medium hover:text-amber-400 transition">
                        {thread.title}
                      </Link>
                      <div className="flex items-center gap-4 mt-3 text-xs text-slate-500">
                        <span className="flex items-center gap-1">
                          <MessageSquare className="w-3 h-3" />
                          {thread.replies} replies
                        </span>
                        <span className="flex items-center gap-1">
                          <Eye className="w-3 h-3" />
                          {thread.views.toLocaleString()} views
                        </span>
                        <span className="flex items-center gap-1">
                          <Clock className="w-3 h-3" />
                          {thread.time}
                        </span>
                      </div>
                    </motion.div>
                  ))}
                </motion.div>
              </Tabs.Content>

              <Tabs.Content value="activity" className="mt-4">
                <motion.div
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
                >
                  <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                    <TrendingUp className="w-4 h-4 text-amber-400" />
                    Posting Activity
                  </h3>
                  <div className="flex items-end gap-2 h-32">
                    {activityData.map((data, idx) => (
                      <div key={idx} className="flex-1 flex flex-col items-center gap-2">
                        <div 
                          className="w-full bg-gradient-to-t from-amber-500 to-orange-500 rounded-t"
                          style={{ height: `${(data.posts / maxPosts) * 100}%` }}
                        />
                        <span className="text-xs text-slate-500">{data.month}</span>
                      </div>
                    ))}
                  </div>
                </motion.div>
              </Tabs.Content>
            </Tabs.Root>
          </div>

          {/* Sidebar */}
          <div className="space-y-6">
            {/* Characters */}
            <motion.div
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: 0.1 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
            >
              <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                <User className="w-4 h-4 text-amber-400" />
                Characters
              </h3>
              <div className="space-y-3">
                {userData.characters.map((char, idx) => (
                  <Link
                    key={idx}
                    href={`/characters?name=${char.name}`}
                    className="block bg-slate-900/50 rounded-lg p-3 hover:bg-slate-800/50 transition group"
                  >
                    <div className="flex items-center justify-between">
                      <span className="text-white font-medium group-hover:text-amber-400 transition">
                        {char.name}
                      </span>
                      <ChevronRight className="w-4 h-4 text-slate-600 group-hover:text-amber-400 transition" />
                    </div>
                    <div className="flex items-center gap-2 mt-1 text-xs text-slate-500">
                      <span>Level {char.level}</span>
                      <span>•</span>
                      <span>{char.vocation}</span>
                    </div>
                    <div className="text-xs text-slate-600 mt-1">{char.world}</div>
                  </Link>
                ))}
              </div>
            </motion.div>

            {/* Top Contributions */}
            <motion.div
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: 0.2 }}
              className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5"
            >
              <h3 className="text-white font-medium mb-4 flex items-center gap-2">
                <Award className="w-4 h-4 text-amber-400" />
                Top Contributions
              </h3>
              <div className="space-y-3 text-sm">
                <div className="flex items-center justify-between">
                  <span className="text-slate-400">Guides Written</span>
                  <span className="text-white font-medium">12</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-slate-400">Best Answers</span>
                  <span className="text-white font-medium">89</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-slate-400">Bugs Reported</span>
                  <span className="text-white font-medium">23</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-slate-400">Events Won</span>
                  <span className="text-white font-medium">7</span>
                </div>
              </div>
            </motion.div>

            {/* Report Button */}
            <button className="w-full px-4 py-2 text-slate-500 text-sm hover:text-slate-400 transition">
              Report User
            </button>
          </div>
        </div>
      </div>
    </div>
  )
}

