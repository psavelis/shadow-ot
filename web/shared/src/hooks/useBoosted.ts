'use client'

import { useQuery } from '@tanstack/react-query'
import { boostedApi } from '../api/endpoints'

export function useBoostedCreature() {
  return useQuery({
    queryKey: ['boosted', 'creature'],
    queryFn: () => boostedApi.getCreature(),
    staleTime: 1000 * 60 * 15, // 15 minutes
    refetchInterval: 1000 * 60 * 15, // Refresh every 15 minutes
  })
}

export function useBoostedBoss() {
  return useQuery({
    queryKey: ['boosted', 'boss'],
    queryFn: () => boostedApi.getBoss(),
    staleTime: 1000 * 60 * 15,
    refetchInterval: 1000 * 60 * 15,
  })
}

export function useBoostedHistory(type: 'creature' | 'boss', days = 7) {
  return useQuery({
    queryKey: ['boosted', type, 'history', days],
    queryFn: () => boostedApi.getHistory(type, days),
    staleTime: 1000 * 60 * 60, // 1 hour
  })
}

// Combined hook for both creature and boss
export function useDailyBoosted() {
  const creature = useBoostedCreature()
  const boss = useBoostedBoss()

  return {
    creature: creature.data,
    boss: boss.data,
    isLoading: creature.isLoading || boss.isLoading,
    error: creature.error || boss.error,
    refetch: () => {
      creature.refetch()
      boss.refetch()
    },
  }
}


