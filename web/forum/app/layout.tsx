import type { Metadata } from 'next'
import { Inter, Cinzel } from 'next/font/google'
import './globals.css'
import Link from 'next/link'

const inter = Inter({ subsets: ['latin'], variable: '--font-inter' })
const cinzel = Cinzel({ subsets: ['latin'], variable: '--font-cinzel' })

export const metadata: Metadata = {
  title: 'Forums - Shadow OT',
  description: 'Shadow OT Community Forums',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className="dark">
      <body className={`${inter.variable} ${cinzel.variable} font-sans bg-shadow-950 text-white antialiased`}>
        <header className="h-16 bg-shadow-900/50 border-b border-shadow-800 flex items-center justify-between px-6">
          <Link href="/" className="flex items-center space-x-3">
            <div className="w-9 h-9 bg-gradient-to-br from-accent-500 to-accent-700 rounded-lg flex items-center justify-center">
              <span className="font-display font-bold text-lg text-white">S</span>
            </div>
            <span className="font-display font-bold text-lg">Shadow OT <span className="text-accent-500">Forums</span></span>
          </Link>
          <nav className="flex items-center space-x-6">
            <Link href="/" className="text-shadow-300 hover:text-white text-sm">Home</Link>
            <Link href="/categories" className="text-shadow-300 hover:text-white text-sm">Categories</Link>
            <Link href="/recent" className="text-shadow-300 hover:text-white text-sm">Recent</Link>
            <Link href="/search" className="text-shadow-300 hover:text-white text-sm">Search</Link>
            <button className="btn-primary text-sm py-2">Sign In</button>
          </nav>
        </header>
        <main className="container mx-auto px-4 py-8">{children}</main>
      </body>
    </html>
  )
}

