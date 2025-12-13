import { useEffect, useCallback, useState } from 'react'
import { useAuthStore } from '../stores/authStore'
import { apiClient } from '../api/client'
import { authApi, userApi } from '../api/endpoints'

// ====== Main Auth Hook ======

export function useAuth() {
  const store = useAuthStore()

  // Refresh user on mount if we have a token but no user
  useEffect(() => {
    if (!store.user && !store.isLoading) {
      store.refreshUser()
    }
  }, [])

  return {
    user: store.user,
    isAuthenticated: store.isAuthenticated,
    isLoading: store.isLoading,
    error: store.error,
    login: store.login,
    register: store.register,
    logout: store.logout,
    refreshUser: store.refreshUser,
    clearError: store.clearError,
  }
}

// ====== Auth Guards ======

export function useRequireAuth(redirectTo = '/login') {
  const { isAuthenticated, isLoading } = useAuth()

  useEffect(() => {
    if (!isLoading && !isAuthenticated && typeof window !== 'undefined') {
      const currentPath = window.location.pathname
      window.location.href = `${redirectTo}?redirect=${encodeURIComponent(currentPath)}`
    }
  }, [isAuthenticated, isLoading, redirectTo])

  return { isAuthenticated, isLoading }
}

export function useRedirectIfAuthenticated(redirectTo = '/dashboard') {
  const { isAuthenticated, isLoading } = useAuth()

  useEffect(() => {
    if (!isLoading && isAuthenticated && typeof window !== 'undefined') {
      window.location.href = redirectTo
    }
  }, [isAuthenticated, isLoading, redirectTo])

  return { isAuthenticated, isLoading }
}

export function useRequireRole(allowedRoles: string[], redirectTo = '/unauthorized') {
  const { user, isAuthenticated, isLoading } = useAuth()
  const [hasAccess, setHasAccess] = useState(false)

  useEffect(() => {
    if (!isLoading) {
      if (!isAuthenticated) {
        window.location.href = '/login'
        return
      }
      
      if (user && user.role && allowedRoles.includes(user.role)) {
        setHasAccess(true)
      } else {
        window.location.href = redirectTo
      }
    }
  }, [isAuthenticated, isLoading, user, allowedRoles, redirectTo])

  return { hasAccess, isLoading }
}

// ====== Password Management ======

export function usePasswordReset() {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState(false)

  const requestReset = useCallback(async (email: string) => {
    setIsLoading(true)
    setError(null)
    setSuccess(false)
    
    try {
      await authApi.requestPasswordReset({ email })
      setSuccess(true)
    } catch (err: any) {
      setError(err.message || 'Failed to send reset email')
    } finally {
      setIsLoading(false)
    }
  }, [])

  const resetPassword = useCallback(async (token: string, newPassword: string) => {
    setIsLoading(true)
    setError(null)
    setSuccess(false)
    
    try {
      await authApi.resetPassword({ token, newPassword })
      setSuccess(true)
    } catch (err: any) {
      setError(err.message || 'Failed to reset password')
    } finally {
      setIsLoading(false)
    }
  }, [])

  return { requestReset, resetPassword, isLoading, error, success }
}

export function useChangePassword() {
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState(false)

  const changePassword = useCallback(async (currentPassword: string, newPassword: string) => {
    setIsLoading(true)
    setError(null)
    setSuccess(false)
    
    try {
      await userApi.changePassword({ currentPassword, newPassword })
      setSuccess(true)
    } catch (err: any) {
      setError(err.message || 'Failed to change password')
    } finally {
      setIsLoading(false)
    }
  }, [])

  return { changePassword, isLoading, error, success, clearError: () => setError(null) }
}

// ====== Two-Factor Authentication ======

interface TwoFactorSetup {
  secret: string
  qrCode: string
  backupCodes: string[]
}

export function useTwoFactor() {
  const { user, refreshUser } = useAuth()
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [setup, setSetup] = useState<TwoFactorSetup | null>(null)

  const isEnabled = user?.twoFactorEnabled ?? false

  const initSetup = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    
    try {
      const response = await userApi.setup2FA()
      setSetup(response)
    } catch (err: any) {
      setError(err.message || 'Failed to setup 2FA')
    } finally {
      setIsLoading(false)
    }
  }, [])

  const enable = useCallback(async (code: string) => {
    if (!setup) {
      setError('Setup not initialized')
      return false
    }
    
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.enable2FA({ secret: setup.secret, code })
      await refreshUser()
      setSetup(null)
      return true
    } catch (err: any) {
      setError(err.message || 'Invalid code')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [setup, refreshUser])

  const disable = useCallback(async (code: string, password: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.disable2FA({ code, password })
      await refreshUser()
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to disable 2FA')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [refreshUser])

  const regenerateBackupCodes = useCallback(async (code: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      const response = await userApi.regenerateBackupCodes({ code })
      return response.backupCodes
    } catch (err: any) {
      setError(err.message || 'Failed to regenerate backup codes')
      return null
    } finally {
      setIsLoading(false)
    }
  }, [])

  return {
    isEnabled,
    isLoading,
    error,
    setup,
    initSetup,
    enable,
    disable,
    regenerateBackupCodes,
    clearError: () => setError(null),
  }
}

// ====== Email Verification ======

export function useEmailVerification() {
  const { user, refreshUser } = useAuth()
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [success, setSuccess] = useState(false)

  const isVerified = user?.emailVerified ?? false

  const resendVerification = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    setSuccess(false)
    
    try {
      await authApi.resendVerification()
      setSuccess(true)
    } catch (err: any) {
      setError(err.message || 'Failed to send verification email')
    } finally {
      setIsLoading(false)
    }
  }, [])

  const verifyEmail = useCallback(async (token: string) => {
    setIsLoading(true)
    setError(null)
    setSuccess(false)
    
    try {
      await authApi.verifyEmail({ token })
      await refreshUser()
      setSuccess(true)
      return true
    } catch (err: any) {
      setError(err.message || 'Invalid or expired token')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [refreshUser])

  return { isVerified, resendVerification, verifyEmail, isLoading, error, success }
}

// ====== Session Management ======

interface Session {
  id: string
  device: string
  browser: string
  ip: string
  location: string
  lastActive: string
  current: boolean
}

export function useSessions() {
  const [sessions, setSessions] = useState<Session[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const fetchSessions = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    
    try {
      const response = await userApi.getSessions()
      setSessions(response.sessions)
    } catch (err: any) {
      setError(err.message || 'Failed to fetch sessions')
    } finally {
      setIsLoading(false)
    }
  }, [])

  const revokeSession = useCallback(async (sessionId: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.revokeSession(sessionId)
      setSessions(prev => prev.filter(s => s.id !== sessionId))
    } catch (err: any) {
      setError(err.message || 'Failed to revoke session')
    } finally {
      setIsLoading(false)
    }
  }, [])

  const revokeAllOtherSessions = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.revokeAllSessions()
      setSessions(prev => prev.filter(s => s.current))
    } catch (err: any) {
      setError(err.message || 'Failed to revoke sessions')
    } finally {
      setIsLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchSessions()
  }, [fetchSessions])

  return {
    sessions,
    isLoading,
    error,
    refreshSessions: fetchSessions,
    revokeSession,
    revokeAllOtherSessions,
  }
}

// ====== Wallet Connection (Web3) ======

export function useWalletAuth() {
  const { user, refreshUser } = useAuth()
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const linkedWallets = user?.linkedWallets ?? []

  const linkWallet = useCallback(async (address: string, signature: string, chain: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.linkWallet({ address, signature, chain })
      await refreshUser()
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to link wallet')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [refreshUser])

  const unlinkWallet = useCallback(async (address: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.unlinkWallet(address)
      await refreshUser()
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to unlink wallet')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [refreshUser])

  const loginWithWallet = useCallback(async (address: string, signature: string, chain: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      const response = await authApi.loginWithWallet({ address, signature, chain })
      apiClient.setTokens(response.tokens)
      await refreshUser()
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to login with wallet')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [refreshUser])

  const getNonce = useCallback(async (address: string) => {
    try {
      const response = await authApi.getWalletNonce(address)
      return response.nonce
    } catch (err: any) {
      setError(err.message || 'Failed to get nonce')
      return null
    }
  }, [])

  return {
    linkedWallets,
    linkWallet,
    unlinkWallet,
    loginWithWallet,
    getNonce,
    isLoading,
    error,
    clearError: () => setError(null),
  }
}

// ====== JWT Token Utilities ======

export function useTokenRefresh() {
  const { logout } = useAuth()

  useEffect(() => {
    // Set up automatic token refresh
    const checkAndRefreshToken = async () => {
      const token = apiClient.getAccessToken()
      if (!token) return

      try {
        // Decode token to check expiry (JWT structure: header.payload.signature)
        const payload = JSON.parse(atob(token.split('.')[1]))
        const expiresIn = payload.exp * 1000 - Date.now()
        
        // Refresh if less than 5 minutes until expiry
        if (expiresIn < 5 * 60 * 1000) {
          const refreshToken = apiClient.getRefreshToken()
          if (refreshToken) {
            const response = await authApi.refreshToken({ refreshToken })
            apiClient.setTokens(response.tokens)
          }
        }
      } catch {
        // Token parsing failed or refresh failed, logout
        logout()
      }
    }

    // Check immediately and then every minute
    checkAndRefreshToken()
    const interval = setInterval(checkAndRefreshToken, 60 * 1000)

    return () => clearInterval(interval)
  }, [logout])
}

// ====== Account Deletion ======

export function useAccountDeletion() {
  const { logout } = useAuth()
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const requestDeletion = useCallback(async (password: string, reason?: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.requestAccountDeletion({ password, reason })
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to request account deletion')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  const confirmDeletion = useCallback(async (token: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.confirmAccountDeletion({ token })
      logout()
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to delete account')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [logout])

  const cancelDeletion = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.cancelAccountDeletion()
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to cancel deletion')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  return { requestDeletion, confirmDeletion, cancelDeletion, isLoading, error }
}

// ====== Security Keys (FIDO2/WebAuthn) ======

interface SecurityKey {
  id: string
  name: string
  type: 'yubikey' | 'fido2'
  addedAt: string
  lastUsed: string
}

export function useSecurityKeys() {
  const [keys, setKeys] = useState<SecurityKey[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [isRegistering, setIsRegistering] = useState(false)

  const fetchKeys = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    
    try {
      const response = await userApi.getSecurityKeys()
      setKeys(response.keys)
    } catch (err: any) {
      setError(err.message || 'Failed to fetch security keys')
    } finally {
      setIsLoading(false)
    }
  }, [])

  const registerKey = useCallback(async (name: string) => {
    setIsRegistering(true)
    setError(null)
    
    try {
      // Get challenge from server
      const challenge = await userApi.challengeSecurityKey()
      
      // Use WebAuthn to create credential
      const credential = await navigator.credentials.create({
        publicKey: challenge.options
      })
      
      if (!credential) {
        throw new Error('Failed to create credential')
      }
      
      // Register with server
      await userApi.registerSecurityKey({ name, credential })
      await fetchKeys()
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to register security key')
      return false
    } finally {
      setIsRegistering(false)
    }
  }, [fetchKeys])

  const deleteKey = useCallback(async (id: string) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.deleteSecurityKey(id)
      setKeys(prev => prev.filter(k => k.id !== id))
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to delete security key')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchKeys()
  }, [fetchKeys])

  return {
    keys,
    isLoading,
    isRegistering,
    error,
    registerKey,
    deleteKey,
    refreshKeys: fetchKeys,
    clearError: () => setError(null),
  }
}

// ====== Activity Log ======

interface ActivityLogEntry {
  id: string
  action: string
  ip: string
  location: string
  userAgent: string
  timestamp: string
}

export function useActivityLog(limit = 20) {
  const [logs, setLogs] = useState<ActivityLogEntry[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const fetchLogs = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    
    try {
      const response = await userApi.getActivityLog({ limit })
      setLogs(response.logs)
    } catch (err: any) {
      setError(err.message || 'Failed to fetch activity log')
    } finally {
      setIsLoading(false)
    }
  }, [limit])

  useEffect(() => {
    fetchLogs()
  }, [fetchLogs])

  return {
    logs,
    isLoading,
    error,
    refreshLogs: fetchLogs,
  }
}

// ====== SSO Management ======

interface SSORealmStatus {
  realmId: string
  realmName: string
  enabled: boolean
  lastSync: string | null
}

export function useSSO() {
  const [realms, setRealms] = useState<SSORealmStatus[]>([])
  const [isEnabled, setIsEnabled] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const fetchStatus = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    
    try {
      const response = await userApi.getSSOStatus()
      setRealms(response.realms)
      setIsEnabled(response.enabled)
    } catch (err: any) {
      setError(err.message || 'Failed to fetch SSO status')
    } finally {
      setIsLoading(false)
    }
  }, [])

  const toggleRealm = useCallback(async (realmId: string, enabled: boolean) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.toggleSSOForRealm(realmId, enabled)
      setRealms(prev => prev.map(r => 
        r.realmId === realmId ? { ...r, enabled } : r
      ))
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to toggle SSO')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  const toggleGlobal = useCallback(async (enabled: boolean) => {
    setIsLoading(true)
    setError(null)
    
    try {
      await userApi.toggleSSO(enabled)
      setIsEnabled(enabled)
      return true
    } catch (err: any) {
      setError(err.message || 'Failed to toggle SSO')
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchStatus()
  }, [fetchStatus])

  return {
    realms,
    isEnabled,
    isLoading,
    error,
    toggleRealm,
    toggleGlobal,
    refreshStatus: fetchStatus,
    clearError: () => setError(null),
  }
}

