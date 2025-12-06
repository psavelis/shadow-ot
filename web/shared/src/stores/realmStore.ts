import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { Realm, RealmId } from '../types'
import { realmApi } from '../api/endpoints'

interface RealmState {
  realms: Realm[]
  selectedRealm: RealmId | null
  isLoading: boolean
  error: string | null
  
  // Computed
  currentRealm: Realm | null
  
  // Actions
  fetchRealms: () => Promise<void>
  selectRealm: (realmId: RealmId | null) => void
  clearError: () => void
}

export const useRealmStore = create<RealmState>()(
  persist(
    (set, get) => ({
      realms: [],
      selectedRealm: null,
      isLoading: false,
      error: null,

      get currentRealm() {
        const { realms, selectedRealm } = get()
        return realms.find(r => r.id === selectedRealm) || null
      },

      fetchRealms: async () => {
        set({ isLoading: true, error: null })
        try {
          const realms = await realmApi.getAll()
          set({ realms, isLoading: false })
        } catch (err: unknown) {
          const error = err as { message?: string }
          set({ 
            isLoading: false, 
            error: error.message || 'Failed to fetch realms' 
          })
        }
      },

      selectRealm: (realmId) => set({ selectedRealm: realmId }),

      clearError: () => set({ error: null }),
    }),
    {
      name: 'shadow-realm',
      partialize: (state) => ({ selectedRealm: state.selectedRealm }),
    }
  )
)

// Realm theme configuration
export const REALM_THEMES: Record<RealmId, {
  name: string
  primary: string
  secondary: string
  gradient: string
  icon: string
}> = {
  shadowveil: {
    name: 'Shadowveil',
    primary: '#e94560',
    secondary: '#1a1a2e',
    gradient: 'from-red-600 via-purple-700 to-slate-900',
    icon: '🌑',
  },
  aetheria: {
    name: 'Aetheria',
    primary: '#4a90d9',
    secondary: '#1e3a5f',
    gradient: 'from-blue-500 via-cyan-600 to-teal-700',
    icon: '✨',
  },
  warbound: {
    name: 'Warbound',
    primary: '#f97316',
    secondary: '#7c2d12',
    gradient: 'from-orange-500 via-red-600 to-amber-700',
    icon: '⚔️',
  },
  mythara: {
    name: 'Mythara',
    primary: '#22c55e',
    secondary: '#14532d',
    gradient: 'from-green-500 via-emerald-600 to-teal-700',
    icon: '🏰',
  },
  voidborne: {
    name: 'Voidborne',
    primary: '#a855f7',
    secondary: '#4c1d95',
    gradient: 'from-purple-500 via-violet-600 to-indigo-800',
    icon: '🌀',
  },
  grimhollow: {
    name: 'Grimhollow',
    primary: '#64748b',
    secondary: '#1e293b',
    gradient: 'from-slate-500 via-gray-700 to-zinc-900',
    icon: '💀',
  },
}

