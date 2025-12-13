'use client'

import { useQuery } from '@tanstack/react-query'
import { realmApi } from '../api/endpoints'
import type { RealmId } from '../types'

export function useRealms() {
  return useQuery({
    queryKey: ['realms'],
    queryFn: () => realmApi.getAll(),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export function useRealm(id: RealmId) {
  return useQuery({
    queryKey: ['realm', id],
    queryFn: () => realmApi.getById(id),
    enabled: !!id,
    staleTime: 1000 * 60 * 5,
  })
}

export function useOnlinePlayers(realmId: RealmId) {
  return useQuery({
    queryKey: ['realm', realmId, 'online'],
    queryFn: () => realmApi.getOnlinePlayers(realmId),
    enabled: !!realmId,
    staleTime: 1000 * 30, // 30 seconds
    refetchInterval: 1000 * 30, // Refresh every 30 seconds
  })
}

export function useRealmLiveStatus(realmId: RealmId) {
  return useQuery({
    queryKey: ['realm', realmId, 'status'],
    queryFn: () => realmApi.getStatus(realmId),
    enabled: !!realmId,
    staleTime: 1000 * 15, // 15 seconds
    refetchInterval: 1000 * 15, // Refresh every 15 seconds for live status
  })
}


