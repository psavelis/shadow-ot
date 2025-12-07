import { useQuery, useMutation, useQueryClient, useInfiniteQuery } from '@tanstack/react-query'
import { newsApi } from '../api/endpoints'
import { NewsCategory } from '../types'

// Query keys
export const newsKeys = {
  all: ['news'] as const,
  lists: () => [...newsKeys.all, 'list'] as const,
  list: (filters: { category?: NewsCategory }) => [...newsKeys.lists(), filters] as const,
  featured: () => [...newsKeys.all, 'featured'] as const,
  detail: (slug: string) => [...newsKeys.all, 'detail', slug] as const,
}

// Fetch news articles with pagination
export function useNews(category?: NewsCategory, page = 1, pageSize = 10) {
  return useQuery({
    queryKey: [...newsKeys.list({ category }), page, pageSize],
    queryFn: () => newsApi.getAll({ category, page, pageSize }),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

// Infinite scroll news
export function useInfiniteNews(category?: NewsCategory, pageSize = 10) {
  return useInfiniteQuery({
    queryKey: newsKeys.list({ category }),
    queryFn: ({ pageParam = 1 }) => newsApi.getAll({ category, page: pageParam, pageSize }),
    initialPageParam: 1,
    getNextPageParam: (lastPage) => {
      if (lastPage.page < lastPage.totalPages) {
        return lastPage.page + 1
      }
      return undefined
    },
    staleTime: 1000 * 60 * 5,
  })
}

// Fetch featured news
export function useFeaturedNews() {
  return useQuery({
    queryKey: newsKeys.featured(),
    queryFn: () => newsApi.getFeatured(),
    staleTime: 1000 * 60 * 10, // 10 minutes
  })
}

// Fetch single news article
export function useNewsArticle(slug: string) {
  return useQuery({
    queryKey: newsKeys.detail(slug),
    queryFn: () => newsApi.getBySlug(slug),
    enabled: !!slug,
  })
}

// React to news article
export function useReactToNews() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, type }: { id: string; type: 'like' }) => newsApi.react(id, type),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: newsKeys.lists() })
    },
  })
}

// News category configuration
export const NEWS_CATEGORIES: { value: NewsCategory; label: string; color: string }[] = [
  { value: 'announcement', label: 'Announcements', color: 'bg-yellow-500' },
  { value: 'update', label: 'Updates', color: 'bg-blue-500' },
  { value: 'event', label: 'Events', color: 'bg-purple-500' },
  { value: 'community', label: 'Community', color: 'bg-green-500' },
  { value: 'maintenance', label: 'Maintenance', color: 'bg-orange-500' },
  { value: 'development', label: 'Development', color: 'bg-cyan-500' },
]

export function getCategoryConfig(category: NewsCategory) {
  return NEWS_CATEGORIES.find(c => c.value === category)
}


