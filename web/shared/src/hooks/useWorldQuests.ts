'use client'

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { worldQuestApi } from '../api/endpoints'
import type { RealmId } from '../types'

export function useWorldQuests(params?: {
  realm?: RealmId
  status?: 'active' | 'completed' | 'failed'
}) {
  return useQuery({
    queryKey: ['world-quests', params],
    queryFn: () => worldQuestApi.getAll(params),
    staleTime: 1000 * 60 * 2, // 2 minutes
    refetchInterval: 1000 * 60 * 2, // Refresh for progress updates
  })
}

export function useActiveWorldQuests(realm?: RealmId) {
  return useQuery({
    queryKey: ['world-quests', 'active', realm],
    queryFn: () => worldQuestApi.getActive(realm),
    staleTime: 1000 * 60, // 1 minute
    refetchInterval: 1000 * 60, // Refresh every minute for live progress
  })
}

export function useWorldQuest(id: string) {
  return useQuery({
    queryKey: ['world-quest', id],
    queryFn: () => worldQuestApi.getById(id),
    enabled: !!id,
    staleTime: 1000 * 30, // 30 seconds
    refetchInterval: 1000 * 30,
  })
}

export function useContributeToWorldQuest() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, amount }: { id: string; amount: number }) =>
      worldQuestApi.contribute(id, amount),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['world-quest', id] })
      queryClient.invalidateQueries({ queryKey: ['world-quests'] })
    },
  })
}


