'use client'

import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import {
  Bell, Check, CheckCheck, Trash2, Filter, Settings, Zap,
  Shield, Coins, Users, MessageSquare, Trophy, AlertTriangle,
  Package, Crown, X, Clock
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'

const notifications = [
  {
    id: 1,
    type: 'levelup',
    title: 'Level Up!',
    message: 'ShadowKnight advanced to level 342! Keep up the great work.',
    time: '5 minutes ago',
    read: false,
    icon: Zap,
    color: 'amber',
  },
  {
    id: 2,
    type: 'trade',
    title: 'Trade Completed',
    message: 'Your Demon Helmet sold for 500,000 gold on the market.',
    time: '15 minutes ago',
    read: false,
    icon: Coins,
    color: 'emerald',
  },
  {
    id: 3,
    type: 'achievement',
    title: 'Achievement Unlocked',
    message: 'You earned the "Dragon Slayer" achievement (+100 points).',
    time: '1 hour ago',
    read: false,
    icon: Trophy,
    color: 'purple',
  },
  {
    id: 4,
    type: 'guild',
    title: 'Guild Message',
    message: 'Dark Legion: Double XP event this weekend! Be online at 20:00 server time.',
    time: '2 hours ago',
    read: true,
    icon: Shield,
    color: 'blue',
  },
  {
    id: 5,
    type: 'security',
    title: 'New Login Detected',
    message: 'Your account was accessed from a new device in San Francisco, US.',
    time: '3 hours ago',
    read: true,
    icon: AlertTriangle,
    color: 'red',
  },
  {
    id: 6,
    type: 'premium',
    title: 'Premium Expiring',
    message: 'Your premium subscription will expire in 7 days. Renew now to keep your benefits!',
    time: '1 day ago',
    read: true,
    icon: Crown,
    color: 'amber',
  },
  {
    id: 7,
    type: 'social',
    title: 'Friend Request',
    message: 'MysticMage wants to add you to their friend list.',
    time: '1 day ago',
    read: true,
    icon: Users,
    color: 'cyan',
  },
  {
    id: 8,
    type: 'market',
    title: 'Offer Expired',
    message: 'Your buy offer for Magic Sword has expired. Create a new offer to continue.',
    time: '2 days ago',
    read: true,
    icon: Package,
    color: 'slate',
  },
  {
    id: 9,
    type: 'support',
    title: 'Support Reply',
    message: 'Your support ticket #TKT-2024-001 has been updated. Check the response.',
    time: '3 days ago',
    read: true,
    icon: MessageSquare,
    color: 'indigo',
  },
]

const typeFilters = [
  { id: 'all', label: 'All' },
  { id: 'levelup', label: 'Level Ups' },
  { id: 'trade', label: 'Trading' },
  { id: 'achievement', label: 'Achievements' },
  { id: 'guild', label: 'Guild' },
  { id: 'security', label: 'Security' },
  { id: 'social', label: 'Social' },
]

export default function NotificationsPage() {
  const [notificationList, setNotificationList] = useState(notifications)
  const [selectedType, setSelectedType] = useState('all')
  const [showSettings, setShowSettings] = useState(false)

  const unreadCount = notificationList.filter(n => !n.read).length
  const filteredNotifications = selectedType === 'all' 
    ? notificationList 
    : notificationList.filter(n => n.type === selectedType)

  const markAsRead = (id: number) => {
    setNotificationList(prev => 
      prev.map(n => n.id === id ? { ...n, read: true } : n)
    )
  }

  const markAllAsRead = () => {
    setNotificationList(prev => 
      prev.map(n => ({ ...n, read: true }))
    )
  }

  const deleteNotification = (id: number) => {
    setNotificationList(prev => prev.filter(n => n.id !== id))
  }

  const clearAll = () => {
    setNotificationList([])
  }

  return (
    <div className="space-y-6 max-w-4xl">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex items-center justify-between"
      >
        <div>
          <h1 className="text-2xl font-bold text-white mb-1">Notifications</h1>
          <p className="text-slate-400">
            {unreadCount > 0 
              ? `You have ${unreadCount} unread notification${unreadCount > 1 ? 's' : ''}`
              : 'All caught up!'}
          </p>
        </div>
        <div className="flex items-center gap-3">
          {unreadCount > 0 && (
            <button
              onClick={markAllAsRead}
              className="flex items-center gap-2 px-3 py-2 bg-slate-800/50 text-slate-300 rounded-lg hover:bg-slate-700 transition text-sm"
            >
              <CheckCheck className="w-4 h-4" />
              Mark all read
            </button>
          )}
          <button
            onClick={() => setShowSettings(!showSettings)}
            className="p-2 bg-slate-800/50 text-slate-400 rounded-lg hover:bg-slate-700 hover:text-white transition"
          >
            <Settings className="w-5 h-5" />
          </button>
        </div>
      </motion.div>

      {/* Settings Panel */}
      <AnimatePresence>
        {showSettings && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: 'auto' }}
            exit={{ opacity: 0, height: 0 }}
            className="overflow-hidden"
          >
            <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-5">
              <h3 className="text-white font-medium mb-4">Notification Preferences</h3>
              <div className="grid md:grid-cols-2 gap-3">
                {[
                  { label: 'Level Ups', desc: 'When your characters level up', enabled: true },
                  { label: 'Trade Activity', desc: 'When items are bought or sold', enabled: true },
                  { label: 'Achievements', desc: 'When you unlock achievements', enabled: true },
                  { label: 'Guild Updates', desc: 'Messages from your guild', enabled: true },
                  { label: 'Security Alerts', desc: 'Login activity and warnings', enabled: true },
                  { label: 'Social Updates', desc: 'Friend requests and messages', enabled: false },
                ].map((pref, idx) => (
                  <div key={idx} className="flex items-center justify-between p-3 bg-slate-900/50 rounded-lg">
                    <div>
                      <p className="text-white text-sm">{pref.label}</p>
                      <p className="text-slate-500 text-xs">{pref.desc}</p>
                    </div>
                    <button
                      className={`relative w-10 h-5 rounded-full transition ${
                        pref.enabled ? 'bg-amber-500' : 'bg-slate-700'
                      }`}
                    >
                      <div className={`absolute top-0.5 w-4 h-4 bg-white rounded-full transition-transform ${
                        pref.enabled ? 'translate-x-5' : 'translate-x-0.5'
                      }`} />
                    </button>
                  </div>
                ))}
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Type Filter */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="flex gap-2 overflow-x-auto pb-2"
      >
        {typeFilters.map(filter => (
          <button
            key={filter.id}
            onClick={() => setSelectedType(filter.id)}
            className={`px-4 py-2 rounded-lg text-sm font-medium whitespace-nowrap transition ${
              selectedType === filter.id
                ? 'bg-amber-500 text-white'
                : 'bg-slate-800/50 text-slate-400 hover:text-white'
            }`}
          >
            {filter.label}
          </button>
        ))}
      </motion.div>

      {/* Notifications List */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
        className="space-y-2"
      >
        <AnimatePresence mode="popLayout">
          {filteredNotifications.length > 0 ? (
            filteredNotifications.map((notification, idx) => (
              <motion.div
                key={notification.id}
                layout
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: 20, height: 0 }}
                transition={{ delay: idx * 0.03 }}
                className={`bg-slate-800/50 border rounded-xl p-4 transition group ${
                  notification.read 
                    ? 'border-slate-700/50' 
                    : 'border-amber-500/30 bg-amber-500/5'
                }`}
              >
                <div className="flex items-start gap-4">
                  {/* Icon */}
                  <div className={`w-10 h-10 rounded-lg flex items-center justify-center flex-shrink-0 ${
                    notification.color === 'amber' ? 'bg-amber-500/20' :
                    notification.color === 'emerald' ? 'bg-emerald-500/20' :
                    notification.color === 'purple' ? 'bg-purple-500/20' :
                    notification.color === 'blue' ? 'bg-blue-500/20' :
                    notification.color === 'red' ? 'bg-red-500/20' :
                    notification.color === 'cyan' ? 'bg-cyan-500/20' :
                    notification.color === 'indigo' ? 'bg-indigo-500/20' :
                    'bg-slate-700/50'
                  }`}>
                    <notification.icon className={`w-5 h-5 ${
                      notification.color === 'amber' ? 'text-amber-400' :
                      notification.color === 'emerald' ? 'text-emerald-400' :
                      notification.color === 'purple' ? 'text-purple-400' :
                      notification.color === 'blue' ? 'text-blue-400' :
                      notification.color === 'red' ? 'text-red-400' :
                      notification.color === 'cyan' ? 'text-cyan-400' :
                      notification.color === 'indigo' ? 'text-indigo-400' :
                      'text-slate-400'
                    }`} />
                  </div>

                  {/* Content */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-start justify-between gap-4">
                      <div>
                        <h3 className="text-white font-medium flex items-center gap-2">
                          {notification.title}
                          {!notification.read && (
                            <span className="w-2 h-2 bg-amber-500 rounded-full" />
                          )}
                        </h3>
                        <p className="text-slate-400 text-sm mt-0.5">{notification.message}</p>
                        <p className="text-slate-500 text-xs mt-2 flex items-center gap-1">
                          <Clock className="w-3 h-3" />
                          {notification.time}
                        </p>
                      </div>

                      {/* Actions */}
                      <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition">
                        {!notification.read && (
                          <button
                            onClick={() => markAsRead(notification.id)}
                            className="p-1.5 text-slate-500 hover:text-emerald-400 hover:bg-emerald-500/20 rounded-lg transition"
                            title="Mark as read"
                          >
                            <Check className="w-4 h-4" />
                          </button>
                        )}
                        <button
                          onClick={() => deleteNotification(notification.id)}
                          className="p-1.5 text-slate-500 hover:text-red-400 hover:bg-red-500/20 rounded-lg transition"
                          title="Delete"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </motion.div>
            ))
          ) : (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="text-center py-16"
            >
              <div className="w-20 h-20 bg-slate-800/50 rounded-full flex items-center justify-center mx-auto mb-6">
                <Bell className="w-10 h-10 text-slate-600" />
              </div>
              <h2 className="text-xl font-medium text-white mb-2">No Notifications</h2>
              <p className="text-slate-400">
                {selectedType !== 'all' 
                  ? `No ${typeFilters.find(t => t.id === selectedType)?.label.toLowerCase()} notifications`
                  : "You're all caught up! Check back later."}
              </p>
            </motion.div>
          )}
        </AnimatePresence>
      </motion.div>

      {/* Clear All */}
      {filteredNotifications.length > 0 && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="text-center pt-4"
        >
          <button
            onClick={clearAll}
            className="text-slate-500 hover:text-red-400 text-sm transition"
          >
            Clear all notifications
          </button>
        </motion.div>
      )}
    </div>
  )
}

