'use client'

import { useQuery, useMutation, useInfiniteQuery, useQueryClient } from '@tanstack/react-query'
import { userApi, inventoryApi, Transaction, Notification, InventoryItem } from '../api/endpoints'

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
  type?: Notification['type']
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

