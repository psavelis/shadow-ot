import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { guildApi } from '../api/endpoints'
import { Guild, RealmId } from '../types'
import { useNotificationStore } from '../stores/notificationStore'

// Query keys
export const guildKeys = {
  all: ['guilds'] as const,
  lists: () => [...guildKeys.all, 'list'] as const,
  list: (filters: { realm?: RealmId }) => [...guildKeys.lists(), filters] as const,
  details: () => [...guildKeys.all, 'detail'] as const,
  detail: (id: string) => [...guildKeys.details(), id] as const,
  byName: (name: string) => [...guildKeys.all, 'name', name] as const,
}

// Fetch guilds with filters
export function useGuilds(realm?: RealmId, page = 1, pageSize = 20) {
  return useQuery({
    queryKey: [...guildKeys.list({ realm }), page, pageSize],
    queryFn: () => guildApi.getAll({ realm, page, pageSize }),
    staleTime: 1000 * 60 * 5,
  })
}

// Fetch single guild by ID
export function useGuild(id: string) {
  return useQuery({
    queryKey: guildKeys.detail(id),
    queryFn: () => guildApi.getById(id),
    enabled: !!id,
  })
}

// Fetch guild by name (public lookup)
export function useGuildByName(name: string) {
  return useQuery({
    queryKey: guildKeys.byName(name),
    queryFn: () => guildApi.getByName(name),
    enabled: !!name && name.length >= 2,
    retry: false,
  })
}

// Create guild mutation
export function useCreateGuild() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: (data: { name: string; realm: RealmId }) => guildApi.create(data),
    onSuccess: (newGuild) => {
      queryClient.invalidateQueries({ queryKey: guildKeys.lists() })
      success('Guild Created', `${newGuild.name} has been founded!`)
    },
    onError: (err: Error) => {
      error('Failed to Create Guild', err.message)
    },
  })
}

// Update guild mutation
export function useUpdateGuild() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: Partial<Pick<Guild, 'description' | 'motd'>> }) => 
      guildApi.update(id, data),
    onSuccess: (updatedGuild) => {
      queryClient.setQueryData(guildKeys.detail(updatedGuild.id), updatedGuild)
      success('Guild Updated', 'Guild information has been updated.')
    },
    onError: (err: Error) => {
      error('Failed to Update Guild', err.message)
    },
  })
}

// Invite member mutation
export function useInviteMember() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ guildId, characterName }: { guildId: string; characterName: string }) => 
      guildApi.invite(guildId, characterName),
    onSuccess: (_, { guildId, characterName }) => {
      queryClient.invalidateQueries({ queryKey: guildKeys.detail(guildId) })
      success('Invitation Sent', `${characterName} has been invited to the guild.`)
    },
    onError: (err: Error) => {
      error('Failed to Invite Member', err.message)
    },
  })
}

// Kick member mutation
export function useKickMember() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ guildId, characterId }: { guildId: string; characterId: string }) => 
      guildApi.kick(guildId, characterId),
    onSuccess: (_, { guildId }) => {
      queryClient.invalidateQueries({ queryKey: guildKeys.detail(guildId) })
      success('Member Kicked', 'The member has been removed from the guild.')
    },
    onError: (err: Error) => {
      error('Failed to Kick Member', err.message)
    },
  })
}

// Leave guild mutation
export function useLeaveGuild() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: (guildId: string) => guildApi.leave(guildId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: guildKeys.all })
      success('Left Guild', 'You have left the guild.')
    },
    onError: (err: Error) => {
      error('Failed to Leave Guild', err.message)
    },
  })
}

// Declare war mutation
export function useDeclareWar() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ guildId, targetGuildId }: { guildId: string; targetGuildId: string }) => 
      guildApi.declareWar(guildId, targetGuildId),
    onSuccess: (_, { guildId }) => {
      queryClient.invalidateQueries({ queryKey: guildKeys.detail(guildId) })
      success('War Declared', 'War has been declared!')
    },
    onError: (err: Error) => {
      error('Failed to Declare War', err.message)
    },
  })
}

