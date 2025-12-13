'use client'

import { useQuery } from '@tanstack/react-query'
import { killStatsApi } from '../api/endpoints'
import type { RealmId } from '../types'

export function useKillStatistics(realm?: RealmId) {
  return useQuery({
    queryKey: ['kill-statistics', realm],
    queryFn: () => killStatsApi.getStatistics(realm),
    staleTime: 1000 * 60 * 5, // 5 minutes
    refetchInterval: 1000 * 60 * 5, // Refresh every 5 minutes
  })
}

export function useTopKillers(params?: {
  realm?: RealmId
  type?: 'pvp' | 'pve' | 'boss' | 'all'
  timeRange?: 'today' | 'week' | 'month' | 'all'
  limit?: number
}) {
  return useQuery({
    queryKey: ['kill-statistics', 'top-killers', params],
    queryFn: () => killStatsApi.getTopKillers(params),
    staleTime: 1000 * 60 * 2, // 2 minutes
    refetchInterval: 1000 * 60 * 2,
  })
}

export function useRecentDeaths(params?: {
  realm?: RealmId
  type?: 'pvp' | 'pve' | 'boss' | 'all'
  limit?: number
}) {
  return useQuery({
    queryKey: ['kill-statistics', 'recent-deaths', params],
    queryFn: () => killStatsApi.getRecentDeaths(params),
    staleTime: 1000 * 30, // 30 seconds
    refetchInterval: 1000 * 30, // Refresh every 30 seconds for live feed
  })
}

export function useBossHunters(params?: {
  realm?: RealmId
  limit?: number
}) {
  return useQuery({
    queryKey: ['kill-statistics', 'boss-hunters', params],
    queryFn: () => killStatsApi.getBossHunters(params),
    staleTime: 1000 * 60 * 5,
  })
}

export function useCharacterKillStats(
  characterId: string,
  params?: {
    type?: 'pvp' | 'pve' | 'boss' | 'all'
    page?: number
    pageSize?: number
  }
) {
  return useQuery({
    queryKey: ['kill-statistics', 'character', characterId, params],
    queryFn: () => killStatsApi.getCharacterKills(characterId, params),
    enabled: !!characterId,
    staleTime: 1000 * 60 * 2,
  })
}


