'use client'

import { useQuery, useMutation, useInfiniteQuery, useQueryClient } from '@tanstack/react-query'
import { userApi, inventoryApi, Transaction, ServerNotification, InventoryItem } from '../api/endpoints'

// ============================================
// Transactions
// ============================================

export function useTransactions(params: {
  type?: Transaction['type']
  page?: number
  pageSize?: number
} = {}) {
  const { type, page = 1, pageSize = 20 } = params
  
  return useQuery({
    queryKey: ['transactions', { type, page, pageSize }],
    queryFn: () => userApi.getTransactions({ type, page, pageSize }),
    staleTime: 1000 * 60, // 1 minute
  })
}

export function useInfiniteTransactions(type?: Transaction['type']) {
  return useInfiniteQuery({
    queryKey: ['transactions', 'infinite', { type }],
    queryFn: ({ pageParam = 1 }) => userApi.getTransactions({ type, page: pageParam, pageSize: 20 }),
    getNextPageParam: (lastPage) => 
      lastPage.page < lastPage.totalPages ? lastPage.page + 1 : undefined,
    initialPageParam: 1,
  })
}

// ============================================
// Notifications
// ============================================

export function useNotifications(params: {
  type?: ServerNotification['type']
  unreadOnly?: boolean
  page?: number
  pageSize?: number
} = {}) {
  const { type, unreadOnly, page = 1, pageSize = 20 } = params
  
  return useQuery({
    queryKey: ['notifications', { type, unreadOnly, page, pageSize }],
    queryFn: () => userApi.getNotifications({ type, unreadOnly, page, pageSize }),
    staleTime: 1000 * 30, // 30 seconds
    refetchInterval: 1000 * 60, // Auto-refresh every minute
  })
}

export function useUnreadNotificationCount() {
  const { data } = useNotifications({ unreadOnly: true, pageSize: 1 })
  return data?.total || 0
}

export function useMarkNotificationRead() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (id: string) => userApi.markNotificationRead(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['notifications'] })
    },
  })
}

export function useMarkAllNotificationsRead() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: () => userApi.markAllNotificationsRead(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['notifications'] })
    },
  })
}

export function useDeleteNotification() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (id: string) => userApi.deleteNotification(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['notifications'] })
    },
  })
}

// ============================================
// Premium
// ============================================

export function usePremiumStatus() {
  return useQuery({
    queryKey: ['premium', 'status'],
    queryFn: () => userApi.getPremiumStatus(),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export function usePremiumHistory() {
  return useQuery({
    queryKey: ['premium', 'history'],
    queryFn: () => userApi.getPremiumHistory(),
    staleTime: 1000 * 60, // 1 minute
  })
}

export function usePurchasePremium() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (plan: 'monthly' | 'quarterly' | 'yearly') => userApi.purchasePremium(plan),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['premium'] })
      queryClient.invalidateQueries({ queryKey: ['user'] })
    },
  })
}

export function usePurchaseCoins() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (packageId: number) => userApi.purchaseCoins(packageId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['premium'] })
      queryClient.invalidateQueries({ queryKey: ['user'] })
    },
  })
}

// ============================================
// Inventory
// ============================================

export function useInventory(params: {
  characterId?: string
  category?: string
  search?: string
  page?: number
  pageSize?: number
} = {}) {
  const { characterId, category, search, page = 1, pageSize = 50 } = params
  
  return useQuery({
    queryKey: ['inventory', { characterId, category, search, page, pageSize }],
    queryFn: () => inventoryApi.getItems({ characterId, category, search, page, pageSize }),
    staleTime: 1000 * 60, // 1 minute
  })
}

export function useInventoryItem(id: string) {
  return useQuery({
    queryKey: ['inventory', id],
    queryFn: () => inventoryApi.getItem(id),
    enabled: !!id,
  })
}

export function useTransferItem() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: ({ itemId, toCharacterId }: { itemId: string; toCharacterId: string }) =>
      inventoryApi.transferItem(itemId, toCharacterId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['inventory'] })
    },
  })
}

export function useListItemOnMarket() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: ({ itemId, price }: { itemId: string; price: number }) =>
      inventoryApi.listOnMarket(itemId, price),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['inventory'] })
      queryClient.invalidateQueries({ queryKey: ['market'] })
    },
  })
}

// ============================================
// Support Tickets
// ============================================

import { supportApi, auctionApi, houseApi, achievementApi } from '../api/endpoints'
import type { SupportTicket, CharacterAuction, ItemAuction, House, PlayerAchievement } from '../api/endpoints'

export function useSupportTickets(params: {
  status?: SupportTicket['status']
  page?: number
  pageSize?: number
} = {}) {
  return useQuery({
    queryKey: ['support', 'tickets', params],
    queryFn: () => supportApi.getTickets(params),
    staleTime: 1000 * 60,
  })
}

export function useSupportTicket(id: string) {
  return useQuery({
    queryKey: ['support', 'ticket', id],
    queryFn: () => supportApi.getTicket(id),
    enabled: !!id,
  })
}

export function useCreateTicket() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (data: { subject: string; category: SupportTicket['category']; message: string }) =>
      supportApi.createTicket(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['support', 'tickets'] })
    },
  })
}

export function useReplyToTicket() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: ({ id, message }: { id: string; message: string }) =>
      supportApi.replyToTicket(id, message),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['support', 'ticket', id] })
      queryClient.invalidateQueries({ queryKey: ['support', 'tickets'] })
    },
  })
}

export function useFAQ() {
  return useQuery({
    queryKey: ['support', 'faq'],
    queryFn: () => supportApi.getFAQ(),
    staleTime: 1000 * 60 * 60, // 1 hour
  })
}

// ============================================
// Auctions
// ============================================

export function useCharacterAuctions(params: Parameters<typeof auctionApi.getCharacterAuctions>[0] = {}) {
  return useQuery({
    queryKey: ['auctions', 'characters', params],
    queryFn: () => auctionApi.getCharacterAuctions(params),
    staleTime: 1000 * 30, // 30 seconds
    refetchInterval: 1000 * 60, // Refresh every minute
  })
}

export function useItemAuctions(params: Parameters<typeof auctionApi.getItemAuctions>[0] = {}) {
  return useQuery({
    queryKey: ['auctions', 'items', params],
    queryFn: () => auctionApi.getItemAuctions(params),
    staleTime: 1000 * 30,
    refetchInterval: 1000 * 60,
  })
}

export function useMyBids() {
  return useQuery({
    queryKey: ['auctions', 'my-bids'],
    queryFn: () => auctionApi.getMyBids(),
    staleTime: 1000 * 30,
    refetchInterval: 1000 * 60,
  })
}

export function usePlaceBid() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: ({ id, amount }: { id: string; amount: number }) =>
      auctionApi.placeBid(id, amount),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['auctions'] })
    },
  })
}

export function useBuyout() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (id: string) => auctionApi.buyout(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['auctions'] })
    },
  })
}

// ============================================
// Houses
// ============================================

export function useHouses(params: Parameters<typeof houseApi.getHouses>[0] = {}) {
  return useQuery({
    queryKey: ['houses', params],
    queryFn: () => houseApi.getHouses(params),
    staleTime: 1000 * 60,
  })
}

export function useHouse(id: string) {
  return useQuery({
    queryKey: ['houses', id],
    queryFn: () => houseApi.getHouse(id),
    enabled: !!id,
  })
}

export function useMyHouses() {
  return useQuery({
    queryKey: ['houses', 'mine'],
    queryFn: () => houseApi.getMyHouses(),
    staleTime: 1000 * 60,
  })
}

export function useBidOnHouse() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: ({ id, amount }: { id: string; amount: number }) =>
      houseApi.bidOnHouse(id, amount),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['houses'] })
    },
  })
}

export function useLeaveHouse() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (id: string) => houseApi.leaveHouse(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['houses'] })
    },
  })
}

// ============================================
// Achievements
// ============================================

export function useAchievements(characterId?: string) {
  return useQuery({
    queryKey: ['achievements', 'player', characterId],
    queryFn: () => achievementApi.getPlayerAchievements(characterId),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export function useAchievementLeaderboard(params: { page?: number; pageSize?: number } = {}) {
  return useQuery({
    queryKey: ['achievements', 'leaderboard', params],
    queryFn: () => achievementApi.getLeaderboard(params),
    staleTime: 1000 * 60,
  })
}

