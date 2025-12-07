'use client'

import { useQuery } from '@tanstack/react-query'
import { creatureApi } from '../api/endpoints'

export function useCreatures(params?: {
  race?: string
  difficulty?: string
  search?: string
  page?: number
  pageSize?: number
}) {
  return useQuery({
    queryKey: ['creatures', params],
    queryFn: () => creatureApi.getAll(params),
    staleTime: 1000 * 60 * 30, // 30 minutes - creatures don't change often
  })
}

export function useCreature(id: string) {
  return useQuery({
    queryKey: ['creature', id],
    queryFn: () => creatureApi.getById(id),
    enabled: !!id,
    staleTime: 1000 * 60 * 60, // 1 hour
  })
}

export function useCreatureByName(name: string) {
  return useQuery({
    queryKey: ['creature', 'name', name],
    queryFn: () => creatureApi.getByName(name),
    enabled: !!name,
    staleTime: 1000 * 60 * 60,
  })
}

export function useBestiaryProgress(characterId: string) {
  return useQuery({
    queryKey: ['bestiary', characterId],
    queryFn: () => creatureApi.getBestiaryProgress(characterId),
    enabled: !!characterId,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export function useBestiaryEntry(characterId: string, creatureId: string) {
  return useQuery({
    queryKey: ['bestiary', characterId, creatureId],
    queryFn: () => creatureApi.getBestiaryEntry(characterId, creatureId),
    enabled: !!characterId && !!creatureId,
    staleTime: 1000 * 60 * 5,
  })
}


