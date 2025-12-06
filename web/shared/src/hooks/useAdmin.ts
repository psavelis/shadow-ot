'use client'

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { adminApi } from '../api/endpoints'
import type { AdminLog, BanRecord, AdminStats, AdminAlert, GameEvent, RealmId } from '../types'

// ============================================
// Admin Stats
// ============================================

export function useAdminStats() {
  return useQuery({
    queryKey: ['admin', 'stats'],
    queryFn: () => adminApi.getStats(),
    staleTime: 1000 * 30, // 30 seconds
    refetchInterval: 1000 * 30, // Auto-refresh every 30 seconds
  })
}

// ============================================
// Admin Logs
// ============================================

export function useAdminLogs(params: {
  type?: AdminLog['type']
  page?: number
  pageSize?: number
} = {}) {
  const { type, page = 1, pageSize = 50 } = params
  
  return useQuery({
    queryKey: ['admin', 'logs', { type, page, pageSize }],
    queryFn: () => adminApi.getLogs({ type, page, pageSize }),
    staleTime: 1000 * 10, // 10 seconds
    refetchInterval: 1000 * 15, // Auto-refresh every 15 seconds
  })
}

// ============================================
// Admin Alerts
// ============================================

export function useAdminAlerts() {
  const queryClient = useQueryClient()
  
  const alertsQuery = useQuery({
    queryKey: ['admin', 'alerts'],
    queryFn: () => adminApi.getAlerts(),
    staleTime: 1000 * 10,
    refetchInterval: 1000 * 10,
  })
  
  const acknowledgeMutation = useMutation({
    mutationFn: (id: string) => adminApi.acknowledgeAlert(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'alerts'] })
    },
  })
  
  return {
    alerts: alertsQuery.data || [],
    isLoading: alertsQuery.isLoading,
    error: alertsQuery.error,
    acknowledgeAlert: acknowledgeMutation.mutate,
    isAcknowledging: acknowledgeMutation.isPending,
  }
}

// ============================================
// Ban Management
// ============================================

export function useBans(params: { page?: number; pageSize?: number } = {}) {
  const { page = 1, pageSize = 20 } = params
  
  return useQuery({
    queryKey: ['admin', 'bans', { page, pageSize }],
    queryFn: () => adminApi.getBans({ page, pageSize }),
    staleTime: 1000 * 60, // 1 minute
  })
}

export function useBanPlayer() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (data: {
      accountId: string
      characterId?: string
      reason: string
      duration?: number
    }) => adminApi.banPlayer(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'bans'] })
      queryClient.invalidateQueries({ queryKey: ['admin', 'stats'] })
    },
  })
}

export function useUnbanPlayer() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (banId: string) => adminApi.unbanPlayer(banId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'bans'] })
      queryClient.invalidateQueries({ queryKey: ['admin', 'stats'] })
    },
  })
}

export function useWarnPlayer() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: ({ accountId, reason }: { accountId: string; reason: string }) => 
      adminApi.warnPlayer(accountId, reason),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'players'] })
    },
  })
}

// ============================================
// Player Management
// ============================================

export function usePlayerSearch(query: string) {
  return useQuery({
    queryKey: ['admin', 'players', 'search', query],
    queryFn: () => adminApi.searchPlayers(query),
    enabled: query.length >= 2,
    staleTime: 1000 * 60,
  })
}

export function usePlayerDetails(id: string) {
  return useQuery({
    queryKey: ['admin', 'players', id],
    queryFn: () => adminApi.getPlayerDetails(id),
    enabled: !!id,
  })
}

// ============================================
// Realm Management
// ============================================

export function useRealmStatus(realmId: RealmId) {
  return useQuery({
    queryKey: ['admin', 'realms', realmId, 'status'],
    queryFn: () => adminApi.getRealmStatus(realmId),
    staleTime: 1000 * 10,
    refetchInterval: 1000 * 15,
    enabled: !!realmId,
  })
}

export function useRestartRealm() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (realmId: RealmId) => adminApi.restartRealm(realmId),
    onSuccess: (_, realmId) => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'realms', realmId] })
      queryClient.invalidateQueries({ queryKey: ['realms'] })
    },
  })
}

export function useBroadcast() {
  return useMutation({
    mutationFn: ({ message, realm }: { message: string; realm?: RealmId }) => 
      adminApi.broadcast(message, realm),
  })
}

// ============================================
// Event Management
// ============================================

export function useCreateEvent() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (data: Omit<GameEvent, 'id' | 'status'>) => adminApi.createEvent(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['events'] })
      queryClient.invalidateQueries({ queryKey: ['admin', 'events'] })
    },
  })
}

export function useUpdateEvent() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: Partial<GameEvent> }) => 
      adminApi.updateEvent(id, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['events'] })
      queryClient.invalidateQueries({ queryKey: ['admin', 'events'] })
    },
  })
}

export function useDeleteEvent() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (id: string) => adminApi.deleteEvent(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['events'] })
      queryClient.invalidateQueries({ queryKey: ['admin', 'events'] })
    },
  })
}

// ============================================
// Events Query
// ============================================

export function useAdminEvents(params: { 
  status?: 'active' | 'scheduled' | 'ended'
  realm?: RealmId
} = {}) {
  const { status, realm } = params
  
  return useQuery({
    queryKey: ['events', { status, realm }],
    queryFn: () => {
      // Use eventApi from endpoints
      const { eventApi } = require('../api/endpoints')
      return eventApi.getAll({ status, realm })
    },
    staleTime: 1000 * 60, // 1 minute
  })
}

// ============================================
// Admin Online Players
// ============================================

export function useOnlinePlayersAdmin() {
  return useQuery({
    queryKey: ['admin', 'players', 'online'],
    queryFn: async () => {
      const { realmApi } = require('../api/endpoints')
      const realms = await realmApi.getAll()
      return realms.reduce((total: number, realm: { playersOnline: number }) => total + realm.playersOnline, 0)
    },
    staleTime: 1000 * 30,
    refetchInterval: 1000 * 30,
  })
}

