import axios, { AxiosInstance, AxiosError, InternalAxiosRequestConfig } from 'axios'
import { AuthTokens, ApiError } from '../types'

// API Configuration
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1'
const WS_BASE_URL = process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8080/ws'

// Token storage keys
const ACCESS_TOKEN_KEY = 'shadow_access_token'
const REFRESH_TOKEN_KEY = 'shadow_refresh_token'

class ApiClient {
  private client: AxiosInstance
  private refreshPromise: Promise<AuthTokens> | null = null

  constructor() {
    this.client = axios.create({
      baseURL: API_BASE_URL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    })

    this.setupInterceptors()
  }

  private setupInterceptors() {
    // Request interceptor - add auth token
    this.client.interceptors.request.use(
      (config: InternalAxiosRequestConfig) => {
        const token = this.getAccessToken()
        if (token && config.headers) {
          config.headers.Authorization = `Bearer ${token}`
        }
        return config
      },
      (error) => Promise.reject(error)
    )

    // Response interceptor - handle token refresh
    this.client.interceptors.response.use(
      (response) => response,
      async (error: AxiosError<ApiError>) => {
        const originalRequest = error.config as InternalAxiosRequestConfig & { _retry?: boolean }

        // Handle 401 and attempt token refresh
        if (error.response?.status === 401 && !originalRequest._retry) {
          originalRequest._retry = true

          try {
            const tokens = await this.refreshTokens()
            if (tokens && originalRequest.headers) {
              originalRequest.headers.Authorization = `Bearer ${tokens.accessToken}`
              return this.client(originalRequest)
            }
          } catch (refreshError) {
            this.clearTokens()
            window.location.href = '/login'
            return Promise.reject(refreshError)
          }
        }

        return Promise.reject(this.formatError(error))
      }
    )
  }

  private formatError(error: AxiosError<ApiError>): ApiError {
    if (error.response?.data) {
      return error.response.data
    }
    return {
      code: 'NETWORK_ERROR',
      message: error.message || 'An unexpected error occurred',
    }
  }

  // Token management
  getAccessToken(): string | null {
    if (typeof window === 'undefined') return null
    return localStorage.getItem(ACCESS_TOKEN_KEY)
  }

  getRefreshToken(): string | null {
    if (typeof window === 'undefined') return null
    return localStorage.getItem(REFRESH_TOKEN_KEY)
  }

  setTokens(tokens: AuthTokens): void {
    if (typeof window === 'undefined') return
    localStorage.setItem(ACCESS_TOKEN_KEY, tokens.accessToken)
    localStorage.setItem(REFRESH_TOKEN_KEY, tokens.refreshToken)
  }

  clearTokens(): void {
    if (typeof window === 'undefined') return
    localStorage.removeItem(ACCESS_TOKEN_KEY)
    localStorage.removeItem(REFRESH_TOKEN_KEY)
  }

  private async refreshTokens(): Promise<AuthTokens | null> {
    // Prevent multiple simultaneous refresh requests
    if (this.refreshPromise) {
      return this.refreshPromise
    }

    const refreshToken = this.getRefreshToken()
    if (!refreshToken) {
      return null
    }

    this.refreshPromise = (async () => {
      try {
        const response = await axios.post<AuthTokens>(`${API_BASE_URL}/auth/refresh`, {
          refreshToken,
        })
        this.setTokens(response.data)
        return response.data
      } finally {
        this.refreshPromise = null
      }
    })()

    return this.refreshPromise
  }

  // HTTP methods
  async get<T>(url: string, params?: Record<string, unknown>): Promise<T> {
    const response = await this.client.get<T>(url, { params })
    return response.data
  }

  async post<T>(url: string, data?: unknown): Promise<T> {
    const response = await this.client.post<T>(url, data)
    return response.data
  }

  async put<T>(url: string, data?: unknown): Promise<T> {
    const response = await this.client.put<T>(url, data)
    return response.data
  }

  async patch<T>(url: string, data?: unknown): Promise<T> {
    const response = await this.client.patch<T>(url, data)
    return response.data
  }

  async delete<T>(url: string): Promise<T> {
    const response = await this.client.delete<T>(url)
    return response.data
  }

  // WebSocket URL
  getWebSocketUrl(): string {
    const token = this.getAccessToken()
    return token ? `${WS_BASE_URL}?token=${token}` : WS_BASE_URL
  }
}

// Singleton instance
export const apiClient = new ApiClient()

// Export for testing
export { ApiClient, API_BASE_URL, WS_BASE_URL }


