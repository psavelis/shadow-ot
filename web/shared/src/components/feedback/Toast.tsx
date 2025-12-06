'use client'

import React from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { X, CheckCircle, AlertCircle, AlertTriangle, Info } from 'lucide-react'
import { cn } from '../../utils/cn'
import { useNotificationStore, Notification } from '../../stores/notificationStore'

const icons = {
  success: CheckCircle,
  error: AlertCircle,
  warning: AlertTriangle,
  info: Info,
}

const colors = {
  success: 'bg-green-500/10 border-green-500/30 text-green-400',
  error: 'bg-red-500/10 border-red-500/30 text-red-400',
  warning: 'bg-yellow-500/10 border-yellow-500/30 text-yellow-400',
  info: 'bg-blue-500/10 border-blue-500/30 text-blue-400',
}

interface ToastProps {
  notification: Notification
  onClose: () => void
}

function Toast({ notification, onClose }: ToastProps) {
  const Icon = icons[notification.type]

  return (
    <motion.div
      initial={{ opacity: 0, y: 50, scale: 0.9 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: 20, scale: 0.9 }}
      className={cn(
        'flex items-start gap-3 p-4 rounded-lg border backdrop-blur-sm',
        'shadow-lg min-w-[300px] max-w-[400px]',
        colors[notification.type]
      )}
    >
      <Icon className="w-5 h-5 flex-shrink-0 mt-0.5" />
      <div className="flex-1 min-w-0">
        <p className="font-medium text-white">{notification.title}</p>
        {notification.message && (
          <p className="text-sm mt-0.5 opacity-80">{notification.message}</p>
        )}
        {notification.action && (
          <button
            onClick={notification.action.onClick}
            className="text-sm font-medium mt-2 hover:underline"
          >
            {notification.action.label}
          </button>
        )}
      </div>
      <button
        onClick={onClose}
        className="text-shadow-400 hover:text-white transition-colors"
      >
        <X className="w-4 h-4" />
      </button>
    </motion.div>
  )
}

interface ToastContainerProps {
  position?: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left' | 'top-center' | 'bottom-center'
}

const positions = {
  'top-right': 'top-4 right-4',
  'top-left': 'top-4 left-4',
  'bottom-right': 'bottom-4 right-4',
  'bottom-left': 'bottom-4 left-4',
  'top-center': 'top-4 left-1/2 -translate-x-1/2',
  'bottom-center': 'bottom-4 left-1/2 -translate-x-1/2',
}

export function ToastContainer({ position = 'bottom-right' }: ToastContainerProps) {
  const { notifications, removeNotification } = useNotificationStore()

  return (
    <div className={cn('fixed z-50 flex flex-col gap-2', positions[position])}>
      <AnimatePresence mode="popLayout">
        {notifications.map((notification) => (
          <Toast
            key={notification.id}
            notification={notification}
            onClose={() => removeNotification(notification.id)}
          />
        ))}
      </AnimatePresence>
    </div>
  )
}

