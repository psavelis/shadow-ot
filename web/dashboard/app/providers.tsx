'use client'

import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { WagmiConfig, createConfig, configureChains, mainnet } from 'wagmi'
import { polygon, arbitrum, base } from 'wagmi/chains'
import { publicProvider } from 'wagmi/providers/public'
import { RainbowKitProvider, darkTheme, getDefaultWallets } from '@rainbow-me/rainbowkit'
import '@rainbow-me/rainbowkit/styles.css'
import { useState, useEffect, ReactNode } from 'react'
import { useAuthStore } from '@shadow-ot/shared/stores/authStore'

const { chains, publicClient } = configureChains(
  [mainnet, polygon, arbitrum, base],
  [publicProvider()]
)

const { connectors } = getDefaultWallets({
  appName: 'Shadow OT Dashboard',
  projectId: process.env.NEXT_PUBLIC_WALLET_CONNECT_ID || 'shadow-ot',
  chains,
})

const wagmiConfig = createConfig({
  autoConnect: true,
  connectors,
  publicClient,
})

// Auth initializer component
function AuthInitializer({ children }: { children: ReactNode }) {
  const refreshUser = useAuthStore(state => state.refreshUser)
  const [initialized, setInitialized] = useState(false)

  useEffect(() => {
    // Try to restore auth state on mount
    const init = async () => {
      try {
        await refreshUser()
      } catch {
        // Ignore errors during initialization
      } finally {
        setInitialized(true)
      }
    }
    init()
  }, [refreshUser])

  // Show minimal loading while initializing auth
  if (!initialized) {
    return (
      <div className="min-h-screen bg-slate-950 flex items-center justify-center">
        <div className="flex flex-col items-center gap-4">
          <div className="w-12 h-12 bg-gradient-to-br from-amber-500 to-orange-600 rounded-xl flex items-center justify-center animate-pulse">
            <span className="font-bold text-lg text-white">S</span>
          </div>
          <div className="w-32 h-1 bg-slate-800 rounded-full overflow-hidden">
            <div className="h-full bg-gradient-to-r from-amber-500 to-orange-500 animate-loading-bar" />
          </div>
        </div>
      </div>
    )
  }

  return <>{children}</>
}

export function Providers({ children }: { children: ReactNode }) {
  const [queryClient] = useState(() => new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 60 * 1000,
        refetchOnWindowFocus: false,
        retry: 1,
      },
    },
  }))

  return (
    <WagmiConfig config={wagmiConfig}>
      <RainbowKitProvider
        chains={chains}
        theme={darkTheme({
          accentColor: '#f59e0b',
          accentColorForeground: 'white',
          borderRadius: 'medium',
          fontStack: 'system',
          overlayBlur: 'small',
        })}
        coolMode
      >
        <QueryClientProvider client={queryClient}>
          <AuthInitializer>
            {children}
          </AuthInitializer>
        </QueryClientProvider>
      </RainbowKitProvider>
    </WagmiConfig>
  )
}
