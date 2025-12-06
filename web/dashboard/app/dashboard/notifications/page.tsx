'use client'

import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import {
  Bell, Check, CheckCheck, Trash2, Settings, Zap,
  Shield, Coins, Users, MessageSquare, Trophy, AlertTriangle,
  Package, Crown, X, Clock, Loader2
} from 'lucide-react'
import * as Tabs from '@radix-ui/react-tabs'
import { 
  useNotifications, 
  useMarkNotificationRead, 
  useMarkAllNotificationsRead, 
  useDeleteNotification 
} from '@/shared/hooks/useDashboard'
import type { Notification } from '@/shared/api/endpoints'

const typeFilters = [
  { id: 'all', label: 'All' },
  { id: 'levelup', label: 'Level Ups' },
  { id: 'trade', label: 'Trading' },
  { id: 'achievement', label: 'Achievements' },
  { id: 'guild', label: 'Guild' },
  { id: 'system', label: 'System' },
]

const typeIcons: Record<string, React.ElementType> = {
  levelup: Zap,
  trade: Coins,
  achievement: Trophy,
  guild: Shield,
  system: AlertTriangle,
}

const typeColors: Record<string, string> = {
  levelup: 'amber',
  trade: 'emerald',
  achievement: 'purple',
  guild: 'blue',
  system: 'red',
}

function formatTimeAgo(timestamp: string) {
  const date = new Date(timestamp)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  const diffHours = Math.floor(diffMins / 60)
  const diffDays = Math.floor(diffHours / 24)
  
  if (diffMins < 1) return 'Just now'
  if (diffMins < 60) return `${diffMins} minute${diffMins > 1 ? 's' : ''} ago`
  if (diffHours < 24) return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago`
  return `${diffDays} day${diffDays > 1 ? 's' : ''} ago`
}

export default function NotificationsPage() {
  const [selectedType, setSelectedType] = useState<Notification['type'] | 'all'>('all')
  const [showSettings, setShowSettings] = useState(false)
  const [currentPage, setCurrentPage] = useState(1)
  const pageSize = 20
  
  // Real API hooks
  const { data: notificationsData, isLoading, error } = useNotifications({
    type: selectedType === 'all' ? undefined : selectedType,
    page: currentPage,
    pageSize,
  })
  const markAsRead = useMarkNotificationRead()
  const markAllAsRead = useMarkAllNotificationsRead()
  const deleteNotification = useDeleteNotification()

  const notifications = notificationsData?.data || []
  const unreadCount = notifications.filter(n => !n.read).length

  const filteredNotifications = notifications

  const handleMarkAsRead = (id: string) => {
    markAsRead.mutate(id)
  }

  const handleMarkAllAsRead = () => {
    markAllAsRead.mutate()
  }

  const handleDelete = (id: string) => {
    deleteNotification.mutate(id)
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
          <h1 className="text-2xl font-bold text-white mb-1 flex items-center gap-2">
            Notifications
            {unreadCount > 0 && (
              <span className="px-2 py-0.5 bg-amber-500 text-black text-sm font-medium rounded-full">
                {unreadCount}
              </span>
            )}
          </h1>
          <p className="text-slate-400">Stay updated with your game activity</p>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={handleMarkAllAsRead}
            disabled={markAllAsRead.isPending || unreadCount === 0}
            className="flex items-center gap-2 px-4 py-2 bg-slate-800/50 text-slate-300 rounded-lg hover:bg-slate-700 transition disabled:opacity-50"
          >
            <CheckCheck className="w-4 h-4" />
            Mark all read
          </button>
          <button
            onClick={() => setShowSettings(!showSettings)}
            className="p-2 bg-slate-800/50 text-slate-300 rounded-lg hover:bg-slate-700 transition"
          >
            <Settings className="w-5 h-5" />
          </button>
        </div>
      </motion.div>

      {/* Type Filters */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="flex flex-wrap gap-2"
      >
        {typeFilters.map(filter => (
          <button
            key={filter.id}
            onClick={() => setSelectedType(filter.id as typeof selectedType)}
            className={`px-4 py-2 rounded-lg text-sm transition ${
              selectedType === filter.id
                ? 'bg-amber-500/20 text-amber-400 border border-amber-500/30'
                : 'bg-slate-800/50 text-slate-400 hover:bg-slate-700/50'
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
        className="bg-slate-800/50 border border-slate-700/50 rounded-xl overflow-hidden"
      >
        {isLoading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-amber-400" />
          </div>
        ) : error ? (
          <div className="flex items-center justify-center py-12 text-red-400">
            <AlertTriangle className="w-5 h-5 mr-2" />
            Failed to load notifications
          </div>
        ) : filteredNotifications.length === 0 ? (
          <div className="text-center py-16">
            <Bell className="w-16 h-16 text-slate-700 mx-auto mb-4" />
            <h3 className="text-xl font-semibold text-white mb-2">No notifications</h3>
            <p className="text-slate-500">You're all caught up!</p>
          </div>
        ) : (
          <AnimatePresence>
            <div className="divide-y divide-slate-700/50">
              {filteredNotifications.map((notification, idx) => {
                const Icon = typeIcons[notification.type] || Bell
                const color = typeColors[notification.type] || 'slate'
                
                return (
                  <motion.div
                    key={notification.id}
                    initial={{ opacity: 0, x: -10 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, x: 10 }}
                    transition={{ delay: idx * 0.03 }}
                    className={`p-4 flex items-start gap-4 hover:bg-slate-700/30 transition ${
                      !notification.read ? 'bg-slate-700/20' : ''
                    }`}
                  >
                    <div className={`w-10 h-10 rounded-lg flex items-center justify-center bg-${color}-500/20`}>
                      <Icon className={`w-5 h-5 text-${color}-400`} />
                    </div>
                    
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-1">
                        <h4 className="text-white font-medium">{notification.title}</h4>
                        {!notification.read && (
                          <span className="w-2 h-2 rounded-full bg-amber-500" />
                        )}
                      </div>
                      <p className="text-slate-400 text-sm">{notification.message}</p>
                      <div className="flex items-center gap-2 mt-2 text-slate-500 text-xs">
                        <Clock className="w-3 h-3" />
                        {formatTimeAgo(notification.timestamp)}
                      </div>
                    </div>
                    
                    <div className="flex items-center gap-1">
                      {!notification.read && (
                        <button
                          onClick={() => handleMarkAsRead(notification.id)}
                          disabled={markAsRead.isPending}
                          className="p-2 text-slate-400 hover:text-emerald-400 hover:bg-slate-700/50 rounded transition"
                          title="Mark as read"
                        >
                          <Check className="w-4 h-4" />
                        </button>
                      )}
                      <button
                        onClick={() => handleDelete(notification.id)}
                        disabled={deleteNotification.isPending}
                        className="p-2 text-slate-400 hover:text-red-400 hover:bg-slate-700/50 rounded transition"
                        title="Delete"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>
                  </motion.div>
                )
              })}
            </div>
          </AnimatePresence>
        )}

        {/* Pagination */}
        {notificationsData && notificationsData.totalPages > 1 && (
          <div className="p-4 border-t border-slate-700 flex items-center justify-between">
            <p className="text-slate-500 text-sm">
              Page {currentPage} of {notificationsData.totalPages}
            </p>
            <div className="flex items-center gap-2">
              <button
                onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
                disabled={currentPage === 1}
                className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded hover:bg-slate-600/50 transition disabled:opacity-50"
              >
                Previous
              </button>
              <button
                onClick={() => setCurrentPage(p => Math.min(notificationsData.totalPages, p + 1))}
                disabled={currentPage === notificationsData.totalPages}
                className="px-3 py-1 bg-slate-700/50 text-slate-300 rounded hover:bg-slate-600/50 transition disabled:opacity-50"
              >
                Next
              </button>
            </div>
          </div>
        )}
      </motion.div>

      {/* Settings Panel */}
      {showSettings && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6"
        >
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">Notification Settings</h3>
            <button
              onClick={() => setShowSettings(false)}
              className="text-slate-400 hover:text-white transition"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
          
          <div className="space-y-4">
            {typeFilters.slice(1).map(filter => (
              <label key={filter.id} className="flex items-center justify-between">
                <span className="text-slate-300">{filter.label}</span>
                <input
                  type="checkbox"
                  defaultChecked
                  className="w-4 h-4 rounded border-slate-600 bg-slate-800 text-amber-500 focus:ring-amber-500/50"
                />
              </label>
            ))}
          </div>
          
          <div className="mt-6 pt-4 border-t border-slate-700">
            <h4 className="text-white font-medium mb-3">Delivery</h4>
            <div className="space-y-3">
              <label className="flex items-center justify-between">
                <span className="text-slate-300">Push notifications</span>
                <input
                  type="checkbox"
                  defaultChecked
                  className="w-4 h-4 rounded border-slate-600 bg-slate-800 text-amber-500 focus:ring-amber-500/50"
                />
              </label>
              <label className="flex items-center justify-between">
                <span className="text-slate-300">Email digest</span>
                <input
                  type="checkbox"
                  className="w-4 h-4 rounded border-slate-600 bg-slate-800 text-amber-500 focus:ring-amber-500/50"
                />
              </label>
            </div>
          </div>
        </motion.div>
      )}
    </div>
  )
}
