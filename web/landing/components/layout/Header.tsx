'use client'

import { useState, useEffect } from 'react'
import Link from 'next/link'
import { motion, AnimatePresence } from 'framer-motion'
import { Menu, X, ChevronDown, User, LogOut, Settings, Wallet } from 'lucide-react'
import { ConnectButton } from '@rainbow-me/rainbowkit'

const navigation = [
  { name: 'Home', href: '/' },
  {
    name: 'Realms',
    href: '#',
    children: [
      { name: 'Shadowveil', href: '/realms/shadowveil', description: 'Dark & Mysterious' },
      { name: 'Aetheria', href: '/realms/aetheria', description: 'Mythic & Epic' },
      { name: 'Warbound', href: '/realms/warbound', description: 'PvP Focused' },
      { name: 'View All Realms', href: '/realms', description: '' },
    ],
  },
  { name: 'Community', href: '/community' },
  { name: 'Highscores', href: '/highscores' },
  { name: 'Market', href: '/market' },
  { name: 'Wiki', href: '/wiki' },
  { name: 'Download', href: '/download' },
]

export function Header() {
  const [isScrolled, setIsScrolled] = useState(false)
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)
  const [activeDropdown, setActiveDropdown] = useState<string | null>(null)

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 20)
    }
    window.addEventListener('scroll', handleScroll)
    return () => window.removeEventListener('scroll', handleScroll)
  }, [])

  return (
    <header
      className={`fixed top-0 left-0 right-0 z-50 transition-all duration-300 ${
        isScrolled
          ? 'bg-shadow-950/95 backdrop-blur-lg border-b border-shadow-800'
          : 'bg-transparent'
      }`}
    >
      <nav className="container mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16 lg:h-20">
          {/* Logo */}
          <Link href="/" className="flex items-center space-x-3">
            <div className="w-10 h-10 bg-gradient-to-br from-accent-500 to-accent-700 rounded-lg flex items-center justify-center">
              <span className="font-display font-bold text-xl text-white">S</span>
            </div>
            <span className="font-display font-bold text-xl hidden sm:block">
              Shadow <span className="text-accent-500">OT</span>
            </span>
          </Link>

          {/* Desktop Navigation */}
          <div className="hidden lg:flex items-center space-x-1">
            {navigation.map((item) => (
              <div
                key={item.name}
                className="relative"
                onMouseEnter={() => item.children && setActiveDropdown(item.name)}
                onMouseLeave={() => setActiveDropdown(null)}
              >
                <Link
                  href={item.href}
                  className={`px-4 py-2 text-sm font-medium transition-colors rounded-lg flex items-center ${
                    activeDropdown === item.name
                      ? 'text-accent-500 bg-shadow-800'
                      : 'text-shadow-300 hover:text-white hover:bg-shadow-800/50'
                  }`}
                >
                  {item.name}
                  {item.children && (
                    <ChevronDown className="ml-1 w-4 h-4" />
                  )}
                </Link>

                {/* Dropdown */}
                <AnimatePresence>
                  {item.children && activeDropdown === item.name && (
                    <motion.div
                      initial={{ opacity: 0, y: 10 }}
                      animate={{ opacity: 1, y: 0 }}
                      exit={{ opacity: 0, y: 10 }}
                      transition={{ duration: 0.2 }}
                      className="absolute top-full left-0 mt-2 w-64 bg-shadow-900 border border-shadow-700 rounded-xl shadow-xl overflow-hidden"
                    >
                      {item.children.map((child) => (
                        <Link
                          key={child.name}
                          href={child.href}
                          className="block px-4 py-3 hover:bg-shadow-800 transition-colors"
                        >
                          <span className="font-medium text-white">{child.name}</span>
                          {child.description && (
                            <span className="block text-sm text-shadow-400">{child.description}</span>
                          )}
                        </Link>
                      ))}
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            ))}
          </div>

          {/* Right side - Auth & Wallet */}
          <div className="hidden lg:flex items-center space-x-4">
            <ConnectButton.Custom>
              {({ account, chain, openConnectModal, openAccountModal, mounted }) => {
                const connected = mounted && account && chain
                return (
                  <button
                    onClick={connected ? openAccountModal : openConnectModal}
                    className="flex items-center space-x-2 px-4 py-2 bg-shadow-800 hover:bg-shadow-700 rounded-lg border border-shadow-600 transition-all"
                  >
                    <Wallet className="w-4 h-4" />
                    <span className="text-sm font-medium">
                      {connected ? `${account.displayName}` : 'Connect Wallet'}
                    </span>
                  </button>
                )
              }}
            </ConnectButton.Custom>

            <Link
              href="/login"
              className="px-4 py-2 text-sm font-medium text-shadow-300 hover:text-white transition-colors"
            >
              Sign In
            </Link>
            <Link
              href="/register"
              className="btn-primary text-sm"
            >
              Create Account
            </Link>
          </div>

          {/* Mobile menu button */}
          <button
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            className="lg:hidden p-2 text-shadow-300 hover:text-white"
          >
            {mobileMenuOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
          </button>
        </div>

        {/* Mobile Navigation */}
        <AnimatePresence>
          {mobileMenuOpen && (
            <motion.div
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              exit={{ opacity: 0, height: 0 }}
              className="lg:hidden border-t border-shadow-800"
            >
              <div className="py-4 space-y-2">
                {navigation.map((item) => (
                  <div key={item.name}>
                    <Link
                      href={item.href}
                      className="block px-4 py-2 text-shadow-300 hover:text-white hover:bg-shadow-800 rounded-lg"
                      onClick={() => setMobileMenuOpen(false)}
                    >
                      {item.name}
                    </Link>
                    {item.children && (
                      <div className="pl-4 space-y-1">
                        {item.children.map((child) => (
                          <Link
                            key={child.name}
                            href={child.href}
                            className="block px-4 py-2 text-sm text-shadow-400 hover:text-white hover:bg-shadow-800 rounded-lg"
                            onClick={() => setMobileMenuOpen(false)}
                          >
                            {child.name}
                          </Link>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
                <div className="pt-4 border-t border-shadow-800 space-y-2">
                  <Link
                    href="/login"
                    className="block px-4 py-2 text-shadow-300 hover:text-white"
                    onClick={() => setMobileMenuOpen(false)}
                  >
                    Sign In
                  </Link>
                  <Link
                    href="/register"
                    className="block mx-4 btn-primary text-center"
                    onClick={() => setMobileMenuOpen(false)}
                  >
                    Create Account
                  </Link>
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </nav>
    </header>
  )
}
