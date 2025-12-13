import type { Metadata } from 'next'
import { Inter, Cinzel } from 'next/font/google'
import './globals.css'
import { AdminSidebar } from '@/components/AdminSidebar'
import { AdminTopBar } from '@/components/AdminTopBar'
import { Providers } from './providers'

const inter = Inter({ subsets: ['latin'], variable: '--font-inter' })
const cinzel = Cinzel({ subsets: ['latin'], variable: '--font-cinzel' })

export const metadata: Metadata = {
  title: 'Admin Panel - Shadow OT',
  description: 'Shadow OT Administration Dashboard',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className="dark">
      <body className={`${inter.variable} ${cinzel.variable} font-sans bg-shadow-950 text-white antialiased`}>
        <Providers>
          <div className="flex h-screen overflow-hidden">
            <AdminSidebar />
            <div className="flex flex-col flex-1 overflow-hidden">
              <AdminTopBar />
              <main className="flex-1 overflow-y-auto p-6">{children}</main>
            </div>
          </div>
        </Providers>
      </body>
    </html>
  )
}

