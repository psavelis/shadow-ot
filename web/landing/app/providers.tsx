'use client'

import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { WagmiConfig, createConfig, configureChains, mainnet } from 'wagmi'
import { polygon, arbitrum, base } from 'wagmi/chains'
import { publicProvider } from 'wagmi/providers/public'
import { RainbowKitProvider, darkTheme, getDefaultWallets } from '@rainbow-me/rainbowkit'
import '@rainbow-me/rainbowkit/styles.css'
import { useState, ReactNode } from 'react'

const { chains, publicClient } = configureChains(
  [mainnet, polygon, arbitrum, base],
  [publicProvider()]
)

const { connectors } = getDefaultWallets({
  appName: 'Shadow OT',
  projectId: process.env.NEXT_PUBLIC_WALLET_CONNECT_ID || 'shadow-ot',
  chains,
})

const wagmiConfig = createConfig({
  autoConnect: true,
  connectors,
  publicClient,
})

export function Providers({ children }: { children: ReactNode }) {
  const [queryClient] = useState(() => new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 60 * 1000,
        refetchOnWindowFocus: false,
      },
    },
  }))

  return (
    <WagmiConfig config={wagmiConfig}>
      <RainbowKitProvider
        chains={chains}
        theme={darkTheme({
          accentColor: '#e94560',
          accentColorForeground: 'white',
          borderRadius: 'medium',
          fontStack: 'system',
        })}
      >
        <QueryClientProvider client={queryClient}>
          {children}
        </QueryClientProvider>
      </RainbowKitProvider>
    </WagmiConfig>
  )
}
