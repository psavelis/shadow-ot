import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { characterApi } from '../api/endpoints'
import { Character, Vocation, RealmId } from '../types'
import { useNotificationStore } from '../stores/notificationStore'

// Query keys
export const characterKeys = {
  all: ['characters'] as const,
  lists: () => [...characterKeys.all, 'list'] as const,
  list: () => [...characterKeys.lists()] as const,
  details: () => [...characterKeys.all, 'detail'] as const,
  detail: (id: string) => [...characterKeys.details(), id] as const,
  byName: (name: string) => [...characterKeys.all, 'name', name] as const,
  deaths: (id: string) => [...characterKeys.detail(id), 'deaths'] as const,
  kills: (id: string) => [...characterKeys.detail(id), 'kills'] as const,
  achievements: (id: string) => [...characterKeys.detail(id), 'achievements'] as const,
}

// Fetch all characters for current user
export function useCharacters() {
  return useQuery({
    queryKey: characterKeys.list(),
    queryFn: () => characterApi.getAll(),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

// Fetch single character by ID
export function useCharacter(id: string) {
  return useQuery({
    queryKey: characterKeys.detail(id),
    queryFn: () => characterApi.getById(id),
    enabled: !!id,
  })
}

// Fetch character by name (public lookup)
export function useCharacterByName(name: string) {
  return useQuery({
    queryKey: characterKeys.byName(name),
    queryFn: () => characterApi.getByName(name),
    enabled: !!name && name.length >= 2,
    retry: false,
  })
}

// Fetch character deaths
export function useCharacterDeaths(id: string, page = 1) {
  return useQuery({
    queryKey: [...characterKeys.deaths(id), page],
    queryFn: () => characterApi.getDeaths(id, page),
    enabled: !!id,
  })
}

// Fetch character kills
export function useCharacterKills(id: string, page = 1) {
  return useQuery({
    queryKey: [...characterKeys.kills(id), page],
    queryFn: () => characterApi.getKills(id, page),
    enabled: !!id,
  })
}

// Fetch character achievements
export function useCharacterAchievements(id: string) {
  return useQuery({
    queryKey: characterKeys.achievements(id),
    queryFn: () => characterApi.getAchievements(id),
    enabled: !!id,
  })
}

// Create character mutation
export function useCreateCharacter() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: (data: { 
      name: string
      vocation: Vocation
      gender: 'male' | 'female'
      realm: RealmId 
    }) => characterApi.create(data),
    onSuccess: (newCharacter) => {
      queryClient.invalidateQueries({ queryKey: characterKeys.lists() })
      success('Character Created', `${newCharacter.name} has been created successfully!`)
    },
    onError: (err: Error) => {
      error('Failed to Create Character', err.message)
    },
  })
}

// Delete character mutation
export function useDeleteCharacter() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: (id: string) => characterApi.delete(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: characterKeys.lists() })
      queryClient.removeQueries({ queryKey: characterKeys.detail(id) })
      success('Character Deleted', 'Your character has been deleted.')
    },
    onError: (err: Error) => {
      error('Failed to Delete Character', err.message)
    },
  })
}

// Rename character mutation
export function useRenameCharacter() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ id, newName }: { id: string; newName: string }) => 
      characterApi.rename(id, newName),
    onSuccess: (updatedCharacter) => {
      queryClient.invalidateQueries({ queryKey: characterKeys.lists() })
      queryClient.setQueryData(
        characterKeys.detail(updatedCharacter.id), 
        updatedCharacter
      )
      success('Character Renamed', `Your character is now known as ${updatedCharacter.name}!`)
    },
    onError: (err: Error) => {
      error('Failed to Rename Character', err.message)
    },
  })
}

// Transfer character mutation
export function useTransferCharacter() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ id, targetRealm }: { id: string; targetRealm: RealmId }) => 
      characterApi.transfer(id, targetRealm),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: characterKeys.all })
      success('Character Transferred', 'Your character has been transferred to the new realm!')
    },
    onError: (err: Error) => {
      error('Failed to Transfer Character', err.message)
    },
  })
}

// Update outfit mutation
export function useUpdateOutfit() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ id, outfit }: { id: string; outfit: Character['outfit'] }) => 
      characterApi.updateOutfit(id, outfit),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: characterKeys.detail(id) })
      success('Outfit Updated', 'Your character\'s appearance has been updated!')
    },
    onError: (err: Error) => {
      error('Failed to Update Outfit', err.message)
    },
  })
}


