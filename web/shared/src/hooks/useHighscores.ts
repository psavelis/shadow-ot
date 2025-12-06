import { useQuery, useInfiniteQuery } from '@tanstack/react-query'
import { highscoreApi } from '../api/endpoints'
import { HighscoreCategory, RealmId, Vocation } from '../types'

// Query keys
export const highscoreKeys = {
  all: ['highscores'] as const,
  lists: () => [...highscoreKeys.all, 'list'] as const,
  list: (filters: HighscoreFilters) => [...highscoreKeys.lists(), filters] as const,
  top: (category: HighscoreCategory) => [...highscoreKeys.all, 'top', category] as const,
  rank: (characterId: string, category: HighscoreCategory) => 
    [...highscoreKeys.all, 'rank', characterId, category] as const,
}

export interface HighscoreFilters {
  category?: HighscoreCategory
  realm?: RealmId
  vocation?: Vocation
}

// Fetch highscores with pagination
export function useHighscores(filters: HighscoreFilters = {}, page = 1, pageSize = 50) {
  return useQuery({
    queryKey: [...highscoreKeys.list(filters), page, pageSize],
    queryFn: () => highscoreApi.get({ ...filters, page, pageSize }),
    staleTime: 1000 * 60, // 1 minute
    placeholderData: (previousData) => previousData,
  })
}

// Infinite scroll highscores
export function useInfiniteHighscores(filters: HighscoreFilters = {}, pageSize = 50) {
  return useInfiniteQuery({
    queryKey: highscoreKeys.list(filters),
    queryFn: ({ pageParam = 1 }) => highscoreApi.get({ ...filters, page: pageParam, pageSize }),
    initialPageParam: 1,
    getNextPageParam: (lastPage) => {
      if (lastPage.page < lastPage.totalPages) {
        return lastPage.page + 1
      }
      return undefined
    },
    staleTime: 1000 * 60,
  })
}

// Fetch top entries for a category
export function useTopHighscores(category: HighscoreCategory, limit = 10) {
  return useQuery({
    queryKey: [...highscoreKeys.top(category), limit],
    queryFn: () => highscoreApi.getTop(category, limit),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

// Fetch character rank for a category
export function useCharacterRank(characterId: string, category: HighscoreCategory) {
  return useQuery({
    queryKey: highscoreKeys.rank(characterId, category),
    queryFn: () => highscoreApi.getCharacterRank(characterId, category),
    enabled: !!characterId,
    staleTime: 1000 * 60 * 5,
  })
}

// Available categories with display names
export const HIGHSCORE_CATEGORIES: { value: HighscoreCategory; label: string }[] = [
  { value: 'experience', label: 'Experience' },
  { value: 'magic', label: 'Magic Level' },
  { value: 'fist', label: 'Fist Fighting' },
  { value: 'club', label: 'Club Fighting' },
  { value: 'sword', label: 'Sword Fighting' },
  { value: 'axe', label: 'Axe Fighting' },
  { value: 'distance', label: 'Distance Fighting' },
  { value: 'shielding', label: 'Shielding' },
  { value: 'fishing', label: 'Fishing' },
  { value: 'achievements', label: 'Achievements' },
  { value: 'loyalty', label: 'Loyalty Points' },
  { value: 'charm', label: 'Charm Points' },
]

// Get display name for a category
export function getCategoryLabel(category: HighscoreCategory): string {
  return HIGHSCORE_CATEGORIES.find(c => c.value === category)?.label || category
}

