import { create } from 'zustand'

export type NotificationType = 'success' | 'error' | 'warning' | 'info'

export interface Notification {
  id: string
  type: NotificationType
  title: string
  message?: string
  duration?: number // ms, 0 for persistent
  action?: {
    label: string
    onClick: () => void
  }
}

interface NotificationState {
  notifications: Notification[]
  
  // Actions
  addNotification: (notification: Omit<Notification, 'id'>) => string
  removeNotification: (id: string) => void
  clearAll: () => void
  
  // Convenience methods
  success: (title: string, message?: string) => string
  error: (title: string, message?: string) => string
  warning: (title: string, message?: string) => string
  info: (title: string, message?: string) => string
}

const generateId = () => Math.random().toString(36).substring(2, 9)

export const useNotificationStore = create<NotificationState>((set, get) => ({
  notifications: [],

  addNotification: (notification) => {
    const id = generateId()
    const newNotification: Notification = {
      ...notification,
      id,
      duration: notification.duration ?? 5000,
    }

    set((state) => ({
      notifications: [...state.notifications, newNotification],
    }))

    // Auto-remove after duration (if not persistent)
    if (newNotification.duration && newNotification.duration > 0) {
      setTimeout(() => {
        get().removeNotification(id)
      }, newNotification.duration)
    }

    return id
  },

  removeNotification: (id) => {
    set((state) => ({
      notifications: state.notifications.filter((n) => n.id !== id),
    }))
  },

  clearAll: () => set({ notifications: [] }),

  success: (title, message) => 
    get().addNotification({ type: 'success', title, message }),
  
  error: (title, message) => 
    get().addNotification({ type: 'error', title, message, duration: 0 }),
  
  warning: (title, message) => 
    get().addNotification({ type: 'warning', title, message, duration: 7000 }),
  
  info: (title, message) => 
    get().addNotification({ type: 'info', title, message }),
}))

