import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { User, AuthTokens } from '../types'
import { apiClient } from '../api/client'
import { authApi, userApi } from '../api/endpoints'

interface AuthState {
  user: User | null
  isAuthenticated: boolean
  isLoading: boolean
  error: string | null
  
  // Actions
  login: (email: string, password: string, twoFactorCode?: string) => Promise<void>
  register: (email: string, username: string, password: string) => Promise<void>
  logout: () => Promise<void>
  refreshUser: () => Promise<void>
  clearError: () => void
  setUser: (user: User | null) => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      user: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,

      login: async (email, password, twoFactorCode) => {
        set({ isLoading: true, error: null })
        try {
          const response = await authApi.login({ email, password, twoFactorCode })
          apiClient.setTokens(response.tokens)
          set({ 
            user: response.user, 
            isAuthenticated: true, 
            isLoading: false,
            error: null
          })
        } catch (err: unknown) {
          const error = err as { message?: string }
          set({ 
            isLoading: false, 
            error: error.message || 'Login failed',
            isAuthenticated: false,
            user: null
          })
          throw err
        }
      },

      register: async (email, username, password) => {
        set({ isLoading: true, error: null })
        try {
          const response = await authApi.register({ 
            email, 
            username, 
            password, 
            confirmPassword: password 
          })
          apiClient.setTokens(response.tokens)
          set({ 
            user: response.user, 
            isAuthenticated: true, 
            isLoading: false,
            error: null
          })
        } catch (err: unknown) {
          const error = err as { message?: string }
          set({ 
            isLoading: false, 
            error: error.message || 'Registration failed',
            isAuthenticated: false,
            user: null
          })
          throw err
        }
      },

      logout: async () => {
        try {
          await authApi.logout()
        } catch {
          // Ignore logout errors
        } finally {
          apiClient.clearTokens()
          set({ user: null, isAuthenticated: false, error: null })
        }
      },

      refreshUser: async () => {
        const token = apiClient.getAccessToken()
        if (!token) {
          set({ user: null, isAuthenticated: false })
          return
        }

        set({ isLoading: true })
        try {
          const user = await userApi.getProfile()
          set({ user, isAuthenticated: true, isLoading: false })
        } catch {
          apiClient.clearTokens()
          set({ user: null, isAuthenticated: false, isLoading: false })
        }
      },

      clearError: () => set({ error: null }),
      
      setUser: (user) => set({ user, isAuthenticated: !!user }),
    }),
    {
      name: 'shadow-auth',
      partialize: (state) => ({ 
        user: state.user, 
        isAuthenticated: state.isAuthenticated 
      }),
    }
  )
)


