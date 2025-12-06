import type { Metadata } from 'next'
import { Inter, Cinzel } from 'next/font/google'
import './globals.css'
import { Providers } from './providers'
import { Header } from '@/components/layout/Header'
import { Footer } from '@/components/layout/Footer'

const inter = Inter({ subsets: ['latin'], variable: '--font-inter' })
const cinzel = Cinzel({ subsets: ['latin'], variable: '--font-cinzel' })

export const metadata: Metadata = {
  title: 'Shadow OT - The Ultimate Open Tibia Experience',
  description: 'Join Shadow OT, the most advanced Open Tibia server with multiple realms, blockchain integration, and unparalleled features. Choose your realm: Shadowveil, Aetheria, Warbound, and more!',
  keywords: ['Open Tibia', 'OT Server', 'MMORPG', 'Tibia', 'Gaming', 'NFT', 'Blockchain Gaming'],
  authors: [{ name: 'Shadow OT Team' }],
  openGraph: {
    title: 'Shadow OT - The Ultimate Open Tibia Experience',
    description: 'Multiple realms, blockchain-native assets, and the most complete OT server ever built.',
    url: 'https://shadow-ot.com',
    siteName: 'Shadow OT',
    images: [
      {
        url: '/images/og-image.jpg',
        width: 1200,
        height: 630,
        alt: 'Shadow OT',
      },
    ],
    locale: 'en_US',
    type: 'website',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Shadow OT - The Ultimate Open Tibia Experience',
    description: 'Multiple realms, blockchain-native assets, and the most complete OT server ever built.',
    images: ['/images/twitter-image.jpg'],
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="dark">
      <body className={`${inter.variable} ${cinzel.variable} font-sans bg-shadow-950 text-white antialiased`}>
        <Providers>
          <div className="flex flex-col min-h-screen">
            <Header />
            <main className="flex-grow">
              {children}
            </main>
            <Footer />
          </div>
        </Providers>
      </body>
    </html>
  )
}
