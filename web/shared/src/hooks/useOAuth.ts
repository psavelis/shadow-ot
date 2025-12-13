import { useState, useCallback, useEffect } from 'react'
import { apiClient } from '../api/client'
import { useAuthStore } from '../stores/authStore'

export type OAuthProvider = 'google' | 'discord' | 'twitch'

interface OAuthConfig {
  google: {
    clientId: string
    scope: string
    redirectUri: string
  }
  discord: {
    clientId: string
    scope: string
    redirectUri: string
  }
  twitch: {
    clientId: string
    scope: string
    redirectUri: string
  }
}

// Default OAuth configuration (populated from env)
const getOAuthConfig = (): OAuthConfig => ({
  google: {
    clientId: process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID || '',
    scope: 'openid email profile',
    redirectUri: `${typeof window !== 'undefined' ? window.location.origin : ''}/auth/callback/google`,
  },
  discord: {
    clientId: process.env.NEXT_PUBLIC_DISCORD_CLIENT_ID || '',
    scope: 'identify email',
    redirectUri: `${typeof window !== 'undefined' ? window.location.origin : ''}/auth/callback/discord`,
  },
  twitch: {
    clientId: process.env.NEXT_PUBLIC_TWITCH_CLIENT_ID || '',
    scope: 'user:read:email',
    redirectUri: `${typeof window !== 'undefined' ? window.location.origin : ''}/auth/callback/twitch`,
  },
})

// OAuth URL builders
const buildOAuthUrl = (provider: OAuthProvider, state: string): string => {
  const config = getOAuthConfig()
  
  switch (provider) {
    case 'google':
      return `https://accounts.google.com/o/oauth2/v2/auth?` +
        `client_id=${config.google.clientId}&` +
        `redirect_uri=${encodeURIComponent(config.google.redirectUri)}&` +
        `response_type=code&` +
        `scope=${encodeURIComponent(config.google.scope)}&` +
        `state=${state}&` +
        `access_type=offline&` +
        `prompt=consent`
    
    case 'discord':
      return `https://discord.com/api/oauth2/authorize?` +
        `client_id=${config.discord.clientId}&` +
        `redirect_uri=${encodeURIComponent(config.discord.redirectUri)}&` +
        `response_type=code&` +
        `scope=${encodeURIComponent(config.discord.scope)}&` +
        `state=${state}`
    
    case 'twitch':
      return `https://id.twitch.tv/oauth2/authorize?` +
        `client_id=${config.twitch.clientId}&` +
        `redirect_uri=${encodeURIComponent(config.twitch.redirectUri)}&` +
        `response_type=code&` +
        `scope=${encodeURIComponent(config.twitch.scope)}&` +
        `state=${state}`
    
    default:
      throw new Error(`Unknown OAuth provider: ${provider}`)
  }
}

// Generate a random state for CSRF protection
const generateState = (): string => {
  const array = new Uint8Array(32)
  if (typeof window !== 'undefined') {
    crypto.getRandomValues(array)
  }
  return Array.from(array, byte => byte.toString(16).padStart(2, '0')).join('')
}

export function useOAuth() {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const { setUser } = useAuthStore()

  // Initiate OAuth flow
  const login = useCallback((provider: OAuthProvider, redirectAfter?: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      // Generate and store state for CSRF protection
      const state = generateState()
      const stateData = {
        provider,
        redirectAfter: redirectAfter || '/dashboard',
        timestamp: Date.now(),
      }
      
      // Store state in sessionStorage for verification
      if (typeof window !== 'undefined') {
        sessionStorage.setItem(`oauth_state_${state}`, JSON.stringify(stateData))
      }
      
      // Build and redirect to OAuth URL
      const oauthUrl = buildOAuthUrl(provider, state)
      window.location.href = oauthUrl
    } catch (err: any) {
      setError(err.message || 'Failed to initiate OAuth flow')
      setIsLoading(false)
    }
  }, [])

  // Handle OAuth callback
  const handleCallback = useCallback(async (
    provider: OAuthProvider,
    code: string,
    state: string
  ): Promise<{ success: boolean; redirectTo?: string }> => {
    setIsLoading(true)
    setError(null)
    
    try {
      // Verify state
      const storedStateJson = sessionStorage.getItem(`oauth_state_${state}`)
      if (!storedStateJson) {
        throw new Error('Invalid OAuth state - possible CSRF attack')
      }
      
      const storedState = JSON.parse(storedStateJson)
      
      // Check state expiration (5 minutes)
      if (Date.now() - storedState.timestamp > 5 * 60 * 1000) {
        throw new Error('OAuth state expired')
      }
      
      // Clean up stored state
      sessionStorage.removeItem(`oauth_state_${state}`)
      
      // Exchange code for tokens via backend
      const response = await apiClient.post<{
        user: any
        tokens: { accessToken: string; refreshToken: string; expiresAt: number }
      }>(`/auth/oauth/${provider}/callback`, { code, state })
      
      // Store tokens
      apiClient.setTokens(response.tokens)
      
      // Update user state
      setUser(response.user)
      
      return { success: true, redirectTo: storedState.redirectAfter }
    } catch (err: any) {
      setError(err.message || 'OAuth authentication failed')
      return { success: false }
    } finally {
      setIsLoading(false)
    }
  }, [setUser])

  // Link OAuth account to existing account
  const linkAccount = useCallback(async (provider: OAuthProvider) => {
    setIsLoading(true)
    setError(null)
    
    try {
      const state = generateState()
      const stateData = {
        provider,
        action: 'link',
        timestamp: Date.now(),
      }
      
      sessionStorage.setItem(`oauth_state_${state}`, JSON.stringify(stateData))
      
      const oauthUrl = buildOAuthUrl(provider, state)
      window.location.href = oauthUrl
    } catch (err: any) {
      setError(err.message || 'Failed to initiate account linking')
      setIsLoading(false)
    }
  }, [])

  // Unlink OAuth account
  const unlinkAccount = useCallback(async (provider: OAuthProvider) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await apiClient.delete(`/auth/oauth/${provider}/unlink`)
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to unlink account')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  return {
    login,
    handleCallback,
    linkAccount,
    unlinkAccount,
    isLoading,
    error,
    clearError: () => setError(null),
  }
}

// Hook for checking linked OAuth accounts
export function useLinkedAccounts() {
  const [accounts, setAccounts] = useState<Record<OAuthProvider, boolean>>({
    google: false,
    discord: false,
    twitch: false,
  })
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    const fetchLinkedAccounts = async () => {
      try {
        const response = await apiClient.get<{ linkedProviders: OAuthProvider[] }>('/auth/oauth/linked')
        const linked: Record<OAuthProvider, boolean> = {
          google: false,
          discord: false,
          twitch: false,
        }
        response.linkedProviders.forEach(provider => {
          linked[provider] = true
        })
        setAccounts(linked)
      } catch {
        // Ignore errors - user might not be authenticated
      } finally {
        setIsLoading(false)
      }
    }

    fetchLinkedAccounts()
  }, [])

  return { accounts, isLoading }
}

// Provider display information
export const oauthProviders: Record<OAuthProvider, {
  name: string
  color: string
  bgColor: string
  hoverBg: string
}> = {
  google: {
    name: 'Google',
    color: '#4285F4',
    bgColor: 'bg-white/5',
    hoverBg: 'hover:bg-white/10',
  },
  discord: {
    name: 'Discord',
    color: '#5865F2',
    bgColor: 'bg-[#5865F2]/10',
    hoverBg: 'hover:bg-[#5865F2]/20',
  },
  twitch: {
    name: 'Twitch',
    color: '#9146FF',
    bgColor: 'bg-[#9146FF]/10',
    hoverBg: 'hover:bg-[#9146FF]/20',
  },
}


