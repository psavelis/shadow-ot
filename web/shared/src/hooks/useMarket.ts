import { useQuery, useMutation, useQueryClient, useInfiniteQuery } from '@tanstack/react-query'
import { marketApi } from '../api/endpoints'
import { RealmId } from '../types'
import { useNotificationStore } from '../stores/notificationStore'

// Query keys
export const marketKeys = {
  all: ['market'] as const,
  offers: () => [...marketKeys.all, 'offers'] as const,
  offerList: (filters: MarketFilters) => [...marketKeys.offers(), filters] as const,
  history: (itemId: number) => [...marketKeys.all, 'history', itemId] as const,
  items: () => [...marketKeys.all, 'items'] as const,
  item: (id: number) => [...marketKeys.items(), id] as const,
}

export interface MarketFilters {
  type?: 'buy' | 'sell'
  itemId?: number
  realm?: RealmId
}

// Fetch market offers
export function useMarketOffers(filters: MarketFilters = {}, page = 1, pageSize = 20) {
  return useQuery({
    queryKey: [...marketKeys.offerList(filters), page, pageSize],
    queryFn: () => marketApi.getOffers({ ...filters, page, pageSize }),
    staleTime: 1000 * 30, // 30 seconds
  })
}

// Infinite scroll market offers
export function useInfiniteMarketOffers(filters: MarketFilters = {}, pageSize = 20) {
  return useInfiniteQuery({
    queryKey: marketKeys.offerList(filters),
    queryFn: ({ pageParam = 1 }) => marketApi.getOffers({ ...filters, page: pageParam, pageSize }),
    initialPageParam: 1,
    getNextPageParam: (lastPage) => {
      if (lastPage.page < lastPage.totalPages) {
        return lastPage.page + 1
      }
      return undefined
    },
    staleTime: 1000 * 30,
  })
}

// Fetch item price history
export function useMarketHistory(itemId: number, days = 30) {
  return useQuery({
    queryKey: [...marketKeys.history(itemId), days],
    queryFn: () => marketApi.getHistory(itemId, days),
    enabled: !!itemId,
    staleTime: 1000 * 60 * 5,
  })
}

// Search items
export function useMarketItems(search?: string, type?: string) {
  return useQuery({
    queryKey: [...marketKeys.items(), { search, type }],
    queryFn: () => marketApi.getItems({ search, type }),
    enabled: !search || search.length >= 2,
    staleTime: 1000 * 60 * 10, // 10 minutes
  })
}

// Get item by ID
export function useMarketItem(id: number) {
  return useQuery({
    queryKey: marketKeys.item(id),
    queryFn: () => marketApi.getItemById(id),
    enabled: !!id,
    staleTime: 1000 * 60 * 60, // 1 hour (item data rarely changes)
  })
}

// Create market offer
export function useCreateOffer() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: (data: {
      type: 'buy' | 'sell'
      itemId: number
      amount: number
      price: number
      anonymous?: boolean
    }) => marketApi.createOffer(data),
    onSuccess: (offer) => {
      queryClient.invalidateQueries({ queryKey: marketKeys.offers() })
      success(
        'Offer Created',
        `Your ${offer.type} offer for ${offer.amount}x ${offer.itemName} has been listed.`
      )
    },
    onError: (err: Error) => {
      error('Failed to Create Offer', err.message)
    },
  })
}

// Cancel market offer
export function useCancelOffer() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: (offerId: string) => marketApi.cancelOffer(offerId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: marketKeys.offers() })
      success('Offer Cancelled', 'Your offer has been cancelled.')
    },
    onError: (err: Error) => {
      error('Failed to Cancel Offer', err.message)
    },
  })
}

// Accept market offer
export function useAcceptOffer() {
  const queryClient = useQueryClient()
  const { success, error } = useNotificationStore()

  return useMutation({
    mutationFn: ({ offerId, amount }: { offerId: string; amount: number }) => 
      marketApi.acceptOffer(offerId, amount),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: marketKeys.offers() })
      success('Trade Complete', 'The trade has been completed successfully!')
    },
    onError: (err: Error) => {
      error('Trade Failed', err.message)
    },
  })
}


